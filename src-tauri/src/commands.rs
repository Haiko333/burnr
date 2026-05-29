use std::path::PathBuf;
use walkdir::WalkDir;

use crate::parser::{self, BillingType, GlobalStats, LogEntry, ToolSource};

fn get_claude_projects_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".claude").join("projects"))
}

fn get_codex_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".codex").join("sessions"))
}

fn get_gemini_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".gemini").join("sessions"))
}

fn get_cursor_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".cursor").join("sessions"))
}

fn get_windsurf_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".windsurf").join("sessions"))
}

fn scan_jsonl_dir(dir: &PathBuf, tool: ToolSource, billing_type: BillingType) -> Vec<LogEntry> {
    if !dir.exists() {
        eprintln!("[burnr] {:?}: directory {:?} does not exist", tool, dir);
        return Vec::new();
    }

    eprintln!("[burnr] {:?}: scanning {:?}", tool, dir);
    let mut all_entries = Vec::new();
    let mut file_count = 0u32;

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            file_count += 1;
            let entries = match tool {
                ToolSource::ClaudeCode => parser::parse_claude_jsonl(path, billing_type),
                ToolSource::Codex => parser::parse_codex_jsonl(path, billing_type),
                ToolSource::Gemini => parser::parse_gemini_jsonl(path, billing_type),
                ToolSource::Cursor => parser::parse_cursor_jsonl(path, billing_type),
                ToolSource::Windsurf => parser::parse_windsurf_jsonl(path, billing_type),
            };
            all_entries.extend(entries);
        }
    }

    eprintln!(
        "[burnr] {:?}: found {} files, {} total entries",
        tool, file_count, all_entries.len()
    );
    all_entries
}

fn scan_tool_entries(tool: ToolSource, billing_type: BillingType) -> Vec<LogEntry> {
    match tool {
        ToolSource::ClaudeCode => {
            get_claude_projects_dir()
                .map(|dir| scan_jsonl_dir(&dir, tool, billing_type))
                .unwrap_or_default()
        }
        ToolSource::Codex => {
            get_codex_dir()
                .map(|dir| scan_jsonl_dir(&dir, tool, billing_type))
                .unwrap_or_default()
        }
        ToolSource::Gemini => {
            get_gemini_dir()
                .map(|dir| scan_jsonl_dir(&dir, tool, billing_type))
                .unwrap_or_default()
        }
        ToolSource::Cursor => {
            get_cursor_dir()
                .map(|dir| scan_jsonl_dir(&dir, tool, billing_type))
                .unwrap_or_default()
        }
        ToolSource::Windsurf => {
            get_windsurf_dir()
                .map(|dir| scan_jsonl_dir(&dir, tool, billing_type))
                .unwrap_or_default()
        }
    }
}

fn scan_all_entries(billing_type: BillingType) -> Vec<LogEntry> {
    let mut all = Vec::new();
    all.extend(scan_tool_entries(ToolSource::ClaudeCode, billing_type));
    all.extend(scan_tool_entries(ToolSource::Codex, billing_type));
    all.extend(scan_tool_entries(ToolSource::Gemini, billing_type));
    all.extend(scan_tool_entries(ToolSource::Cursor, billing_type));
    all.extend(scan_tool_entries(ToolSource::Windsurf, billing_type));
    all.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    all
}

#[tauri::command]
pub fn get_all_stats(
    billing_type: Option<BillingType>,
    tool: Option<ToolSource>,
) -> GlobalStats {
    let bt = billing_type.unwrap_or(BillingType::Subscription);
    let entries = match tool {
        Some(t) => scan_tool_entries(t, bt),
        None => scan_all_entries(bt),
    };
    parser::aggregate_stats(&entries)
}

#[tauri::command]
pub fn get_available_tools() -> Vec<ToolAvailability> {
    vec![
        ToolAvailability {
            tool: ToolSource::ClaudeCode,
            available: get_claude_projects_dir()
                .map(|d| d.exists())
                .unwrap_or(false),
        },
        ToolAvailability {
            tool: ToolSource::Codex,
            available: get_codex_dir().map(|d| d.exists()).unwrap_or(false),
        },
        ToolAvailability {
            tool: ToolSource::Gemini,
            available: get_gemini_dir().map(|d| d.exists()).unwrap_or(false),
        },
        ToolAvailability {
            tool: ToolSource::Cursor,
            available: get_cursor_dir().map(|d| d.exists()).unwrap_or(false),
        },
        ToolAvailability {
            tool: ToolSource::Windsurf,
            available: get_windsurf_dir().map(|d| d.exists()).unwrap_or(false),
        },
    ]
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAvailability {
    pub tool: ToolSource,
    pub available: bool,
}
