use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::parser::ToolSource;

const ALLOWED_TOKEN_TOOLS: [&str; 3] = ["claude", "cursor", "windsurf"];
const HTTP_TIMEOUT_SECONDS: u64 = 10;

fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|c| c.join("burnr"))
}

fn validate_tool_key(tool: &str) -> Result<&'static str, String> {
    ALLOWED_TOKEN_TOOLS
        .iter()
        .copied()
        .find(|allowed| *allowed == tool)
        .ok_or_else(|| "Unsupported tool".to_string())
}

fn session_file_path(tool: &str) -> Option<PathBuf> {
    validate_tool_key(tool)
        .ok()
        .and_then(|safe_tool| config_dir().map(|d| d.join(format!("{}_session.txt", safe_tool))))
}

fn read_session_token(tool: &str) -> Option<String> {
    if let Some(path) = session_file_path(tool) {
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}

fn read_org_id(tool: &str) -> Option<String> {
    let safe_tool = validate_tool_key(tool).ok()?;
    if let Some(dir) = config_dir() {
        let path = dir.join(format!("{}_org.txt", safe_tool));
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}

fn ensure_config_dir() -> Result<PathBuf, String> {
    let dir = config_dir().ok_or("Cannot determine config directory")?;
    fs::create_dir_all(&dir).map_err(|_| "Could not create config directory".to_string())?;
    set_private_dir_permissions(&dir)?;
    Ok(dir)
}

#[cfg(unix)]
fn set_private_dir_permissions(path: &PathBuf) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|_| "Could not secure config directory permissions".to_string())
}

#[cfg(not(unix))]
fn set_private_dir_permissions(_path: &PathBuf) -> Result<(), String> {
    Ok(())
}

#[cfg(unix)]
fn write_private_file(path: PathBuf, content: &str) -> Result<(), String> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(&path)
        .map_err(|_| "Could not write token file".to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|_| "Could not write token file".to_string())?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
        .map_err(|_| "Could not secure token file permissions".to_string())
}

#[cfg(not(unix))]
fn write_private_file(path: PathBuf, content: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .map_err(|_| "Could not write token file".to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|_| "Could not write token file".to_string())
}

fn write_session_token(tool: &str, token: &str) -> Result<(), String> {
    let safe_tool = validate_tool_key(tool)?;
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err("Session token cannot be empty".to_string());
    }

    let dir = ensure_config_dir()?;
    let path = dir.join(format!("{}_session.txt", safe_tool));
    write_private_file(path, trimmed)
}

fn write_org_id(tool: &str, org_id: &str) -> Result<(), String> {
    let safe_tool = validate_tool_key(tool)?;
    let trimmed = org_id.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let dir = ensure_config_dir()?;
    let path = dir.join(format!("{}_org.txt", safe_tool));
    write_private_file(path, trimmed)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolLimit {
    pub tool: ToolSource,
    pub limit_type: String,
    pub current_usage: f64,
    pub limit_label: String,
    pub reset_time: Option<String>,
    pub requests_used: Option<u64>,
    pub requests_total: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionTokenInfo {
    pub tool: String,
    pub has_token: bool,
    pub source: String, // "manual", "none"
    pub browser: Option<String>,
    pub masked_token: Option<String>,
    pub masked_org: Option<String>,
}

fn mask_value(val: &str) -> String {
    let len = val.len();
    if len <= 8 {
        "••••••••".to_string()
    } else {
        format!("{}••••{}", &val[..4], &val[len - 4..])
    }
}

#[tauri::command]
pub fn get_session_tokens() -> Vec<SessionTokenInfo> {
    let tools = ["claude", "cursor", "windsurf"];

    tools
        .iter()
        .map(|t| {
            let token = read_session_token(t);
            let org = read_org_id(t);

            SessionTokenInfo {
                tool: t.to_string(),
                has_token: token.is_some(),
                source: if token.is_some() { "manual".to_string() } else { "none".to_string() },
                browser: None,
                masked_token: token.as_deref().map(mask_value),
                masked_org: org.as_deref().map(mask_value),
            }
        })
        .collect()
}

#[tauri::command]
pub fn set_session_token(tool: String, token: String, org_id: Option<String>) -> Result<(), String> {
    let safe_tool = validate_tool_key(&tool)?;
    write_session_token(safe_tool, &token)?;
    if safe_tool == "claude" {
        if let Some(org) = org_id {
            write_org_id(safe_tool, &org)?;
        }
    }
    Ok(())
}

fn http_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECONDS))
        .build()
        .map_err(|_| "Could not create HTTP client".to_string())
}

#[tauri::command]
pub fn get_tool_limits() -> Vec<ToolLimit> {
    let mut limits = Vec::new();

    if let Some(claude_limits) = fetch_claude_limits() {
        limits.extend(claude_limits);
    }

    if let Some(cursor_limits) = fetch_cursor_limits() {
        limits.extend(cursor_limits);
    }

    if let Some(windsurf_limits) = fetch_windsurf_limits() {
        limits.extend(windsurf_limits);
    }

    limits
}

#[derive(Debug, Deserialize)]
struct ClaudeUsageWindow {
    #[serde(default)]
    utilization: Option<f64>,
    #[serde(default)]
    resets_at: Option<String>,
}

fn fetch_claude_limits() -> Option<Vec<ToolLimit>> {
    let session = read_session_token("claude")?;
    let org_id = read_org_id("claude")?;

    let url = format!("https://claude.ai/api/organizations/{}/usage", org_id);
    eprintln!("[burnr] claude limits: fetching {}", url);

    let client = http_client().ok()?;
    let resp = client
        .get(&url)
        .header("cookie", format!("sessionKey={}", session))
        .send()
        .ok()?;

    let status = resp.status();
    if !status.is_success() {
        eprintln!("[burnr] claude limits: HTTP {}", status);
        return None;
    }

    let body: serde_json::Value = match resp.json() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[burnr] claude limits: parse error: {}", e);
            return None;
        }
    };

    eprintln!("[burnr] claude limits: response keys: {:?}", body.as_object().map(|o| o.keys().collect::<Vec<_>>()));

    let mut limits = Vec::new();

    // Format: { "five_hour": { "utilization": 45.2, "resets_at": "..." }, "seven_day": { ... } }
    if let Some(five_hour) = body.get("five_hour") {
        if let Ok(w) = serde_json::from_value::<ClaudeUsageWindow>(five_hour.clone()) {
            limits.push(ToolLimit {
                tool: ToolSource::ClaudeCode,
                limit_type: "five_hour".to_string(),
                current_usage: w.utilization.unwrap_or(0.0),
                limit_label: "Claude - 5h".to_string(),
                reset_time: w.resets_at,
                requests_used: None,
                requests_total: None,
            });
        }
    }

    if let Some(seven_day) = body.get("seven_day") {
        if let Ok(w) = serde_json::from_value::<ClaudeUsageWindow>(seven_day.clone()) {
            limits.push(ToolLimit {
                tool: ToolSource::ClaudeCode,
                limit_type: "seven_day".to_string(),
                current_usage: w.utilization.unwrap_or(0.0),
                limit_label: "Claude - Weekly".to_string(),
                reset_time: w.resets_at,
                requests_used: None,
                requests_total: None,
            });
        }
    }

    // Fallback: try "windows" object with "5h"/"7d" keys
    if limits.is_empty() {
        if let Some(windows) = body.get("windows").and_then(|v| v.as_object()) {
            if let Some(five_h) = windows.get("5h") {
                if let Ok(w) = serde_json::from_value::<ClaudeUsageWindow>(five_h.clone()) {
                    limits.push(ToolLimit {
                        tool: ToolSource::ClaudeCode,
                        limit_type: "five_hour".to_string(),
                        current_usage: w.utilization.unwrap_or(0.0),
                        limit_label: "Claude - 5h".to_string(),
                        reset_time: w.resets_at,
                        requests_used: None,
                        requests_total: None,
                    });
                }
            }
            if let Some(seven_d) = windows.get("7d") {
                if let Ok(w) = serde_json::from_value::<ClaudeUsageWindow>(seven_d.clone()) {
                    limits.push(ToolLimit {
                        tool: ToolSource::ClaudeCode,
                        limit_type: "seven_day".to_string(),
                        current_usage: w.utilization.unwrap_or(0.0),
                        limit_label: "Claude - Weekly".to_string(),
                        reset_time: w.resets_at,
                        requests_used: None,
                        requests_total: None,
                    });
                }
            }
        }
    }

    if limits.is_empty() {
        eprintln!("[burnr] claude limits: no usable data in response");
        None
    } else {
        eprintln!("[burnr] claude limits: {} limits parsed", limits.len());
        Some(limits)
    }
}

#[derive(Debug, Deserialize)]
struct CursorUsageResponse {
    #[serde(default, rename = "numRequests")]
    num_requests: Option<u64>,
    #[serde(default, rename = "numRequestsTotal")]
    num_requests_total: Option<u64>,
}

fn fetch_cursor_limits() -> Option<Vec<ToolLimit>> {
    let session = read_session_token("cursor")?;

    let client = http_client().ok()?;
    let resp = client
        .get("https://www.cursor.com/api/usage")
        .header("cookie", format!("WorkosCursorSessionToken={}", session))
        .send()
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let data: CursorUsageResponse = resp.json().ok()?;

    let used = data.num_requests.unwrap_or(0);
    let total = data.num_requests_total.unwrap_or(500);
    let pct = if total > 0 {
        (used as f64 / total as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    Some(vec![ToolLimit {
        tool: ToolSource::Cursor,
        limit_type: "monthly".to_string(),
        current_usage: pct,
        limit_label: "Cursor - Monthly".to_string(),
        reset_time: None,
        requests_used: Some(used),
        requests_total: Some(total),
    }])
}

fn fetch_windsurf_limits() -> Option<Vec<ToolLimit>> {
    let session = read_session_token("windsurf")?;

    let client = http_client().ok()?;
    let resp = client
        .get("https://windsurf.com/api/usage")
        .header("cookie", format!("session={}", session))
        .send()
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let data: CursorUsageResponse = resp.json().ok()?;

    let used = data.num_requests.unwrap_or(0);
    let total = data.num_requests_total.unwrap_or(500);
    let pct = if total > 0 {
        (used as f64 / total as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    Some(vec![ToolLimit {
        tool: ToolSource::Windsurf,
        limit_type: "monthly".to_string(),
        current_usage: pct,
        limit_label: "Windsurf - Monthly".to_string(),
        reset_time: None,
        requests_used: Some(used),
        requests_total: Some(total),
    }])
}

#[cfg(test)]
mod tests {
    use super::validate_tool_key;

    #[test]
    fn validate_tool_key_rejects_path_traversal_and_unknown_tools() {
        assert!(validate_tool_key("claude").is_ok());
        assert!(validate_tool_key("cursor").is_ok());
        assert!(validate_tool_key("windsurf").is_ok());
        assert!(validate_tool_key("../claude").is_err());
        assert!(validate_tool_key("claude/session").is_err());
        assert!(validate_tool_key("codex").is_err());
    }
}
