use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::cookies;
use crate::parser::ToolSource;

fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|c| c.join("burnr"))
}

fn session_file_path(tool: &str) -> Option<PathBuf> {
    config_dir().map(|d| d.join(format!("{}_session.txt", tool)))
}

fn read_session_token(tool: &str) -> Option<String> {
    // First try manually configured token file
    if let Some(path) = session_file_path(tool) {
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    // Fall back to auto-detected browser cookie
    let detected = cookies::detect_browser_cookies();
    detected.iter().find(|d| d.tool == tool).and_then(|d| d.session_key.clone())
}

fn read_org_id(tool: &str) -> Option<String> {
    // First try manual org file
    if let Some(dir) = config_dir() {
        let path = dir.join(format!("{}_org.txt", tool));
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    // Fall back to auto-detected cookie
    let detected = cookies::detect_browser_cookies();
    detected.iter().find(|d| d.tool == tool).and_then(|d| d.org_id.clone())
}

fn write_session_token(tool: &str, token: &str) -> Result<(), String> {
    let dir = config_dir().ok_or("Cannot determine config directory")?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}_session.txt", tool));
    fs::write(path, token).map_err(|e| e.to_string())
}

fn write_org_id(tool: &str, org_id: &str) -> Result<(), String> {
    let dir = config_dir().ok_or("Cannot determine config directory")?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}_org.txt", tool));
    fs::write(path, org_id).map_err(|e| e.to_string())
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
    pub source: String, // "manual", "detected", "none"
    pub browser: Option<String>,
}

#[tauri::command]
pub fn get_session_tokens() -> Vec<SessionTokenInfo> {
    let tools = ["claude", "cursor", "windsurf"];
    let detected = cookies::detect_browser_cookies();

    tools
        .iter()
        .map(|t| {
            let manual_path = session_file_path(t);
            let has_manual = manual_path
                .and_then(|p| fs::read_to_string(p).ok())
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

            let cookie_info = detected.iter().find(|d| d.tool == *t);
            let has_detected = cookie_info.map(|c| c.session_key.is_some()).unwrap_or(false);

            let (has_token, source, browser) = if has_manual {
                (true, "manual".to_string(), None)
            } else if has_detected {
                (
                    true,
                    "detected".to_string(),
                    cookie_info.and_then(|c| c.browser.clone()),
                )
            } else {
                (false, "none".to_string(), None)
            };

            SessionTokenInfo {
                tool: t.to_string(),
                has_token,
                source,
                browser,
            }
        })
        .collect()
}

#[tauri::command]
pub fn set_session_token(tool: String, token: String, org_id: Option<String>) -> Result<(), String> {
    write_session_token(&tool, &token)?;
    if let Some(org) = org_id {
        if !org.trim().is_empty() {
            write_org_id(&tool, org.trim())?;
        }
    }
    Ok(())
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

#[tauri::command]
pub fn debug_claude_limits() -> String {
    let session = match read_session_token("claude") {
        Some(s) => s,
        None => return "ERROR: No session token found (no cookie detected, no manual override)".to_string(),
    };

    let org_id = match read_org_id("claude") {
        Some(id) => id,
        None => return format!("ERROR: No org_id found. Session token starts with: {}...", &session[..session.len().min(20)]),
    };

    let url = format!("https://claude.ai/api/organizations/{}/usage", org_id);
    eprintln!("[burnr] debug: fetching {}", url);

    let client = reqwest::blocking::Client::new();
    let resp = match client
        .get(&url)
        .header("cookie", format!("sessionKey={}", session))
        .send()
    {
        Ok(r) => r,
        Err(e) => return format!("ERROR: HTTP request failed: {}", e),
    };

    let status = resp.status();
    let body = resp.text().unwrap_or_else(|e| format!("(failed to read body: {})", e));

    eprintln!("[burnr] debug: status={}, body={}", status, &body[..body.len().min(500)]);

    format!("status={}\norg_id={}\nurl={}\nbody={}", status, org_id, url, body)
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

    let client = reqwest::blocking::Client::new();
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

    let client = reqwest::blocking::Client::new();
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

    let client = reqwest::blocking::Client::new();
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
