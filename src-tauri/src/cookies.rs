use std::path::PathBuf;
use std::process::Command;

use aes_gcm::{aead::Aead, Aes128Gcm, Aes256Gcm, KeyInit, Nonce};
use cbc::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use rusqlite::Connection;
use serde::Serialize;
use sha1::Sha1;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedCookies {
    pub tool: String,
    pub session_key: Option<String>,
    pub org_id: Option<String>,
    pub browser: Option<String>,
}

#[tauri::command]
pub fn detect_browser_cookies() -> Vec<DetectedCookies> {
    let mut results = Vec::new();

    results.push(detect_claude_cookies());
    results.push(detect_cursor_cookies());
    results.push(detect_windsurf_cookies());

    results
}

fn detect_claude_cookies() -> DetectedCookies {
    let mut result = DetectedCookies {
        tool: "claude".to_string(),
        session_key: None,
        org_id: None,
        browser: None,
    };

    let claude_hosts = [".claude.ai", "claude.ai", "www.claude.ai"];

    for (browser_name, cookie_path) in get_chromium_cookie_paths() {
        if !cookie_path.exists() {
            continue;
        }

        let key = match get_chromium_decryption_key(&browser_name) {
            Some(k) => k,
            None => continue,
        };

        // Copy DB to temp to avoid SQLite lock from running browser
        let tmp_path = std::env::temp_dir().join("burnr_cookies_tmp.db");
        if std::fs::copy(&cookie_path, &tmp_path).is_err() {
            continue;
        }

        if let Ok(conn) = Connection::open(&tmp_path) {
            for host in &claude_hosts {
                if result.session_key.is_none() {
                    if let Some(val) = query_cookie_chromium(&conn, host, "sessionKey", &key) {
                        result.session_key = Some(val);
                        result.browser = Some(browser_name.clone());
                    }
                }
                if result.org_id.is_none() {
                    if let Some(val) = query_cookie_chromium(&conn, host, "lastActiveOrg", &key) {
                        result.org_id = Some(val);
                    }
                }
            }
            if result.session_key.is_some() {
                return result;
            }
        }
    }

    if let Some(firefox_path) = get_firefox_cookie_path() {
        if let Ok(conn) = Connection::open(&firefox_path) {
            for host in &claude_hosts {
                if result.session_key.is_none() {
                    if let Some(val) = query_cookie_firefox(&conn, host, "sessionKey") {
                        result.session_key = Some(val);
                        result.browser = Some("Firefox".to_string());
                    }
                }
                if result.org_id.is_none() {
                    if let Some(val) = query_cookie_firefox(&conn, host, "lastActiveOrg") {
                        result.org_id = Some(val);
                    }
                }
            }
        }
    }


    result
}

fn detect_cursor_cookies() -> DetectedCookies {
    let mut result = DetectedCookies {
        tool: "cursor".to_string(),
        session_key: None,
        org_id: None,
        browser: None,
    };

    for (browser_name, cookie_path) in get_chromium_cookie_paths() {
        if !cookie_path.exists() {
            continue;
        }
        let key = match get_chromium_decryption_key(&browser_name) {
            Some(k) => k,
            None => continue,
        };
        let tmp_path = std::env::temp_dir().join("burnr_cookies_cursor_tmp.db");
        if std::fs::copy(&cookie_path, &tmp_path).is_err() {
            continue;
        }
        if let Ok(conn) = Connection::open(&tmp_path) {
            if let Some(val) =
                query_cookie_chromium(&conn, ".cursor.com", "WorkosCursorSessionToken", &key)
            {
                result.session_key = Some(val);
                result.browser = Some(browser_name.clone());
                return result;
            }
        }
    }

    if let Some(firefox_path) = get_firefox_cookie_path() {
        if let Ok(conn) = Connection::open(&firefox_path) {
            if let Some(val) =
                query_cookie_firefox(&conn, ".cursor.com", "WorkosCursorSessionToken")
            {
                result.session_key = Some(val);
                result.browser = Some("Firefox".to_string());
            }
        }
    }

    result
}

fn detect_windsurf_cookies() -> DetectedCookies {
    let mut result = DetectedCookies {
        tool: "windsurf".to_string(),
        session_key: None,
        org_id: None,
        browser: None,
    };

    for (browser_name, cookie_path) in get_chromium_cookie_paths() {
        if !cookie_path.exists() {
            continue;
        }
        let key = match get_chromium_decryption_key(&browser_name) {
            Some(k) => k,
            None => continue,
        };
        let tmp_path = std::env::temp_dir().join("burnr_cookies_windsurf_tmp.db");
        if std::fs::copy(&cookie_path, &tmp_path).is_err() {
            continue;
        }
        if let Ok(conn) = Connection::open(&tmp_path) {
            if let Some(val) = query_cookie_chromium(&conn, ".windsurf.com", "session", &key) {
                result.session_key = Some(val);
                result.browser = Some(browser_name.clone());
                return result;
            }
        }
    }

    if let Some(firefox_path) = get_firefox_cookie_path() {
        if let Ok(conn) = Connection::open(&firefox_path) {
            if let Some(val) = query_cookie_firefox(&conn, ".windsurf.com", "session") {
                result.session_key = Some(val);
                result.browser = Some("Firefox".to_string());
            }
        }
    }

    result
}

fn get_chromium_cookie_paths() -> Vec<(String, PathBuf)> {
    let mut paths = Vec::new();
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return paths,
    };

    let config = match dirs::config_dir() {
        Some(c) => c,
        None => home.join(".config"),
    };

    let candidates = [
        ("Chrome", config.join("google-chrome/Default/Cookies")),
        ("Chrome", config.join("google-chrome/Profile 1/Cookies")),
        ("Chromium", config.join("chromium/Default/Cookies")),
        ("Brave", config.join("BraveSoftware/Brave-Browser/Default/Cookies")),
        ("Edge", config.join("microsoft-edge/Default/Cookies")),
        ("Vivaldi", config.join("vivaldi/Default/Cookies")),
    ];

    for (name, path) in candidates {
        if path.exists() {
            paths.push((name.to_string(), path));
        }
    }

    paths
}

fn get_firefox_cookie_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let mozilla_dir = home.join(".mozilla/firefox");
    if !mozilla_dir.exists() {
        return None;
    }

    let profiles_ini = mozilla_dir.join("profiles.ini");
    if profiles_ini.exists() {
        if let Ok(content) = std::fs::read_to_string(&profiles_ini) {
            for line in content.lines() {
                if line.starts_with("Path=") {
                    let profile_path = line.trim_start_matches("Path=");
                    let cookie_file = if profile_path.starts_with('/') {
                        PathBuf::from(profile_path).join("cookies.sqlite")
                    } else {
                        mozilla_dir.join(profile_path).join("cookies.sqlite")
                    };
                    if cookie_file.exists() {
                        return Some(cookie_file);
                    }
                }
            }
        }
    }

    if let Ok(entries) = std::fs::read_dir(&mozilla_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let cookie_file = path.join("cookies.sqlite");
                if cookie_file.exists() {
                    return Some(cookie_file);
                }
            }
        }
    }

    None
}

struct ChromiumKeys {
    v10_key: Vec<u8>,      // 16 bytes for AES-128-CBC (PBKDF2 derived)
    v11_key: Vec<u8>,      // 16 bytes for AES-128-GCM (PBKDF2 derived)
    raw_key: Option<Vec<u8>>, // raw bytes from base64 password (for direct use)
}

fn get_chromium_decryption_key(browser: &str) -> Option<ChromiumKeys> {
    let safe_storage_name = match browser {
        "Chrome" => "Chrome Safe Storage",
        "Chromium" => "Chromium Safe Storage",
        "Brave" => "Brave Safe Storage",
        "Edge" => "Microsoft Edge Safe Storage",
        "Vivaldi" => "Chrome Safe Storage",
        _ => "Chrome Safe Storage",
    };

    let password = get_password_from_keyring(safe_storage_name)
        .unwrap_or_else(|| "peanuts".to_string());

    // v10 always uses PBKDF2 with the password
    let mut v10_key = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.as_bytes(), b"saltysalt", 1, &mut v10_key);

    // v11 on Linux: same PBKDF2 derivation as v10
    let v11_key = v10_key.to_vec();

    // Also try base64-decoding the password as a raw key
    let raw_key = base64_decode(&password).ok().filter(|k| k.len() == 16 || k.len() == 32);


    Some(ChromiumKeys {
        v10_key: v10_key.to_vec(),
        v11_key,
        raw_key,
    })
}

fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    // Simple base64 decoder
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input = input.trim().trim_end_matches('=');
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for &b in input.as_bytes() {
        let val = TABLE.iter().position(|&c| c == b).ok_or(())? as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

fn get_password_from_keyring(label: &str) -> Option<String> {
    // Try GNOME secret-tool first
    let output = Command::new("secret-tool")
        .args(["lookup", "xdg:schema", "chrome_libsecret_os_crypt_password_v2", "application", label])
        .output()
        .ok();

    if let Some(ref out) = output {
        if out.status.success() && !out.stdout.is_empty() {
            let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
            return Some(val);
        }
    }

    let output = Command::new("secret-tool")
        .args(["lookup", "application", label])
        .output()
        .ok();

    if let Some(ref out) = output {
        if out.status.success() && !out.stdout.is_empty() {
            let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
            return Some(val);
        }
    }

    // Try KWallet (KDE)
    let folder = match label {
        "Chrome Safe Storage" => "Chrome Keys",
        "Chromium Safe Storage" => "Chromium Keys",
        "Brave Safe Storage" => "Chromium Keys",
        "Microsoft Edge Safe Storage" => "Microsoft Edge Keys",
        _ => "Chrome Keys",
    };
    let entry_name = label;

    let output = Command::new("kwallet-query")
        .args(["-f", folder, "-r", entry_name, "kdewallet"])
        .output()
        .ok();

    if let Some(ref out) = output {
        if out.status.success() && !out.stdout.is_empty() {
            let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !val.contains("n'existe pas") && !val.is_empty() {
                return Some(val);
            }
        }
    }

    // Also try the generic "Chromium Safe Storage" entry name for Brave
    if label == "Brave Safe Storage" {
        let output = Command::new("kwallet-query")
            .args(["-f", "Chromium Keys", "-r", "Chromium Safe Storage", "kdewallet"])
            .output()
            .ok();

        if let Some(ref out) = output {
            if out.status.success() && !out.stdout.is_empty() {
                let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !val.contains("n'existe pas") && !val.is_empty() {
                    return Some(val);
                }
            }
        }
    }

    None
}

fn query_cookie_chromium(
    conn: &Connection,
    host: &str,
    name: &str,
    keys: &ChromiumKeys,
) -> Option<String> {
    let mut stmt = conn
        .prepare(
            "SELECT encrypted_value, value FROM cookies WHERE host_key = ?1 AND name = ?2 ORDER BY last_access_utc DESC LIMIT 1",
        )
        .ok()?;

    let result: Option<(Vec<u8>, String)> = stmt
        .query_row(rusqlite::params![host, name], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .ok();

    let (encrypted, plaintext) = result?;

    if !plaintext.is_empty() {
        return Some(plaintext);
    }

    if encrypted.is_empty() {
        return None;
    }

    decrypt_chromium_cookie(&encrypted, keys)
}

fn decrypt_chromium_cookie(encrypted: &[u8], keys: &ChromiumKeys) -> Option<String> {
    if encrypted.len() < 3 {
        return None;
    }

    // v11: AES-GCM with 12-byte nonce
    if encrypted[0..3] == [0x76, 0x31, 0x31] {
        let payload = &encrypted[3..];
        if payload.len() < 12 + 16 {
            return None;
        }
        let nonce_bytes = &payload[..12];
        let ciphertext = &payload[12..];
        let nonce = Nonce::from_slice(nonce_bytes);

        // Try AES-128-GCM with raw 16-byte key
        if keys.v11_key.len() >= 16 {
            if let Ok(cipher) = Aes128Gcm::new_from_slice(&keys.v11_key[..16]) {
                if let Ok(decrypted) = cipher.decrypt(nonce, ciphertext) {
                        return String::from_utf8(decrypted).ok();
                }
            }
        }

        // Try AES-256-GCM with full 32-byte key
        if keys.v11_key.len() >= 32 {
            if let Ok(cipher) = Aes256Gcm::new_from_slice(&keys.v11_key[..32]) {
                if let Ok(decrypted) = cipher.decrypt(nonce, ciphertext) {
                        return String::from_utf8(decrypted).ok();
                }
            }
        }

        // Try PBKDF2-derived 16-byte key with AES-128-GCM (same as v10 key)
        if let Ok(cipher) = Aes128Gcm::new_from_slice(&keys.v10_key) {
            if let Ok(decrypted) = cipher.decrypt(nonce, ciphertext) {
                return String::from_utf8(decrypted).ok();
            }
        }

        // Try raw base64-decoded key directly (for os_crypt_async)
        if let Some(ref raw) = keys.raw_key {
            if raw.len() == 16 {
                if let Ok(cipher) = Aes128Gcm::new_from_slice(raw) {
                    if let Ok(decrypted) = cipher.decrypt(nonce, ciphertext) {
                                return String::from_utf8(decrypted).ok();
                    }
                }
            }
            if raw.len() == 32 {
                if let Ok(cipher) = Aes256Gcm::new_from_slice(raw) {
                    if let Ok(decrypted) = cipher.decrypt(nonce, ciphertext) {
                                return String::from_utf8(decrypted).ok();
                    }
                }
            }
        }

        return None;
    }

    // v10: AES-128-CBC with space IV
    if encrypted[0..3] == [0x76, 0x31, 0x30] {
        let ciphertext = &encrypted[3..];
        if ciphertext.len() < 16 {
            return None;
        }

        let iv = [0x20u8; 16];
        let mut buf = ciphertext.to_vec();

        let decrypted = Aes128CbcDec::new(keys.v10_key.as_slice().into(), &iv.into())
            .decrypt_padded_mut::<NoPadding>(&mut buf)
            .ok()?;

        // Remove PKCS7 padding
        let pad_len = *decrypted.last()? as usize;
        if pad_len == 0 || pad_len > 16 || pad_len > decrypted.len() {
            return String::from_utf8(decrypted.to_vec()).ok();
        }
        let unpadded = &decrypted[..decrypted.len() - pad_len];
        return String::from_utf8(unpadded.to_vec()).ok();
    }

    // Unrecognized prefix — try as plaintext
    String::from_utf8(encrypted.to_vec()).ok()
}

fn query_cookie_firefox(conn: &Connection, host: &str, name: &str) -> Option<String> {
    let mut stmt = conn
        .prepare(
            "SELECT value FROM moz_cookies WHERE host = ?1 AND name = ?2 ORDER BY lastAccessed DESC LIMIT 1",
        )
        .ok()?;

    stmt.query_row(rusqlite::params![host, name], |row| row.get(0))
        .ok()
}
