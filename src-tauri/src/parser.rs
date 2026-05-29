use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ToolSource {
    ClaudeCode,
    Codex,
    Gemini,
    Cursor,
    Windsurf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    #[serde(default)]
    pub input_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: u64,
    #[serde(default)]
    pub cache_read_input_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeRawEntry {
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    message: Option<Message>,
    #[serde(default)]
    timestamp: Option<String>,
    #[serde(default)]
    session_id: Option<String>,
}

// Codex JSONL: top-level has { type, timestamp, payload }
// type "session_meta" → payload.id (session_id), payload.cwd (project)
// type "turn_context" → payload.model
// type "event_msg" → payload.type == "token_count" → payload.info.total_token_usage
#[derive(Debug, Clone, Deserialize)]
struct CodexRawLine {
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    timestamp: Option<String>,
    #[serde(default)]
    payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct CodexTokenUsage {
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    cached_input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    reasoning_output_tokens: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiRawEntry {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default, rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsage>,
    #[serde(default)]
    timestamp: Option<String>,
    #[serde(default, rename = "sessionId")]
    session_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiUsage {
    #[serde(default, rename = "promptTokenCount")]
    prompt_token_count: u64,
    #[serde(default, rename = "candidatesTokenCount")]
    candidates_token_count: u64,
    #[serde(default, rename = "cachedContentTokenCount")]
    cached_content_token_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillingType {
    Subscription,
    Api,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
    pub project: String,
    pub session_id: String,
    pub cost_usd: f64,
    pub billing_type: BillingType,
    pub tool_source: ToolSource,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStats {
    pub project: String,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub total_cost_usd: f64,
    pub cost_is_estimated: bool,
    pub session_count: usize,
    pub entry_count: usize,
    pub models_used: Vec<ModelStats>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelStats {
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
    pub cost_usd: f64,
    pub entry_count: usize,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStats {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub total_cost_usd: f64,
    pub cost_is_estimated: bool,
    pub total_entries: usize,
    pub total_sessions: usize,
    pub projects: Vec<ProjectStats>,
    pub daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DailyUsage {
    pub date: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
    pub cost_usd: f64,
}

struct Pricing {
    input_per_million: f64,
    output_per_million: f64,
    cache_read_per_million: f64,
    cache_creation_per_million: f64,
}

fn get_claude_pricing(model: &str) -> Pricing {
    if model.contains("opus") {
        Pricing {
            input_per_million: 15.0,
            output_per_million: 75.0,
            cache_read_per_million: 1.5,
            cache_creation_per_million: 18.75,
        }
    } else if model.contains("haiku") {
        Pricing {
            input_per_million: 0.80,
            output_per_million: 4.0,
            cache_read_per_million: 0.08,
            cache_creation_per_million: 1.0,
        }
    } else {
        Pricing {
            input_per_million: 3.0,
            output_per_million: 15.0,
            cache_read_per_million: 0.30,
            cache_creation_per_million: 3.75,
        }
    }
}

fn get_codex_pricing(model: &str) -> Pricing {
    if model.contains("o3") {
        Pricing {
            input_per_million: 2.0,
            output_per_million: 8.0,
            cache_read_per_million: 0.50,
            cache_creation_per_million: 2.0,
        }
    } else if model.contains("o4-mini") {
        Pricing {
            input_per_million: 1.10,
            output_per_million: 4.40,
            cache_read_per_million: 0.275,
            cache_creation_per_million: 1.10,
        }
    } else {
        Pricing {
            input_per_million: 2.50,
            output_per_million: 10.0,
            cache_read_per_million: 1.25,
            cache_creation_per_million: 2.50,
        }
    }
}

fn get_gemini_pricing(model: &str) -> Pricing {
    if model.contains("2.5-pro") {
        Pricing {
            input_per_million: 1.25,
            output_per_million: 10.0,
            cache_read_per_million: 0.3125,
            cache_creation_per_million: 1.25,
        }
    } else if model.contains("2.5-flash") {
        Pricing {
            input_per_million: 0.15,
            output_per_million: 0.60,
            cache_read_per_million: 0.0375,
            cache_creation_per_million: 0.15,
        }
    } else {
        Pricing {
            input_per_million: 1.25,
            output_per_million: 10.0,
            cache_read_per_million: 0.3125,
            cache_creation_per_million: 1.25,
        }
    }
}

fn get_cursor_pricing(model: &str) -> Pricing {
    if model.contains("gpt-4") {
        Pricing {
            input_per_million: 2.50,
            output_per_million: 10.0,
            cache_read_per_million: 1.25,
            cache_creation_per_million: 2.50,
        }
    } else if model.contains("claude") {
        Pricing {
            input_per_million: 3.0,
            output_per_million: 15.0,
            cache_read_per_million: 0.30,
            cache_creation_per_million: 3.75,
        }
    } else {
        Pricing {
            input_per_million: 2.0,
            output_per_million: 8.0,
            cache_read_per_million: 0.50,
            cache_creation_per_million: 2.0,
        }
    }
}

fn get_windsurf_pricing(_model: &str) -> Pricing {
    Pricing {
        input_per_million: 2.0,
        output_per_million: 8.0,
        cache_read_per_million: 0.50,
        cache_creation_per_million: 2.0,
    }
}

fn calculate_cost(tool: ToolSource, model: &str, usage: &TokenUsage) -> f64 {
    let pricing = match tool {
        ToolSource::ClaudeCode => get_claude_pricing(model),
        ToolSource::Codex => get_codex_pricing(model),
        ToolSource::Gemini => get_gemini_pricing(model),
        ToolSource::Cursor => get_cursor_pricing(model),
        ToolSource::Windsurf => get_windsurf_pricing(model),
    };
    let input = usage.input_tokens as f64 * pricing.input_per_million / 1_000_000.0;
    let output = usage.output_tokens as f64 * pricing.output_per_million / 1_000_000.0;
    let cache_read =
        usage.cache_read_input_tokens as f64 * pricing.cache_read_per_million / 1_000_000.0;
    let cache_creation =
        usage.cache_creation_input_tokens as f64 * pricing.cache_creation_per_million / 1_000_000.0;
    input + output + cache_read + cache_creation
}

fn project_name_from_path(path: &Path) -> String {
    path.parent()
        .and_then(|p| {
            let dir_name = p.file_name()?.to_str()?;
            if dir_name == "subagents" {
                p.parent()?.parent()?.file_name()?.to_str().map(String::from)
            } else {
                Some(dir_name.to_string())
            }
        })
        .map(|name| name.replace('-', "/").trim_start_matches('/').to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn parse_claude_jsonl(path: &Path, billing_type: BillingType) -> Vec<LogEntry> {
    let project = project_name_from_path(path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.trim().is_empty() {
            continue;
        }

        let raw: ClaudeRawEntry = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(_) => continue,
        };

        if raw.r#type.as_deref() != Some("assistant") {
            continue;
        }

        let message = match raw.message {
            Some(m) => m,
            None => continue,
        };

        let usage = match message.usage {
            Some(u) => u,
            None => continue,
        };

        let model = message.model.unwrap_or_else(|| "unknown".to_string());
        let timestamp = raw
            .timestamp
            .and_then(|ts| DateTime::parse_from_rfc3339(&ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let session_id = raw.session_id.unwrap_or_default();
        let cost_usd = calculate_cost(ToolSource::ClaudeCode, &model, &usage);

        entries.push(LogEntry {
            timestamp,
            model,
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
            project: project.clone(),
            session_id,
            cost_usd,
            billing_type,
            tool_source: ToolSource::ClaudeCode,
        });
    }

    entries
}

pub fn parse_codex_jsonl(path: &Path, billing_type: BillingType) -> Vec<LogEntry> {
    let project = project_name_from_path(path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[burnr] codex: cannot open {:?}: {}", path, e);
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    // Two-pass: first collect session metadata (model, session_id), then token_count events
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let mut session_model = "unknown".to_string();
    let mut session_id = String::new();

    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let raw: CodexRawLine = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(_) => continue,
        };
        match raw.r#type.as_deref() {
            Some("session_meta") => {
                if let Some(payload) = &raw.payload {
                    if let Some(id) = payload.get("id").and_then(|v| v.as_str()) {
                        session_id = id.to_string();
                    }
                }
            }
            Some("turn_context") => {
                if let Some(payload) = &raw.payload {
                    if let Some(m) = payload.get("model").and_then(|v| v.as_str()) {
                        session_model = m.to_string();
                    }
                }
            }
            _ => {}
        }
    }

    eprintln!(
        "[burnr] codex: parsing {:?} (model={}, session={})",
        path, session_model, session_id
    );

    let mut parsed_count = 0u32;
    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let raw: CodexRawLine = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if raw.r#type.as_deref() != Some("event_msg") {
            continue;
        }

        let payload = match &raw.payload {
            Some(p) => p,
            None => continue,
        };

        if payload.get("type").and_then(|v| v.as_str()) != Some("token_count") {
            continue;
        }

        let info = match payload.get("info") {
            Some(i) if !i.is_null() => i,
            _ => continue,
        };

        let total_usage = match info.get("total_token_usage") {
            Some(u) => u,
            None => continue,
        };

        let codex_usage: CodexTokenUsage = match serde_json::from_value(total_usage.clone()) {
            Ok(u) => u,
            Err(_) => continue,
        };

        let usage = TokenUsage {
            input_tokens: codex_usage.input_tokens,
            output_tokens: codex_usage.output_tokens + codex_usage.reasoning_output_tokens,
            cache_read_input_tokens: codex_usage.cached_input_tokens,
            cache_creation_input_tokens: 0,
        };

        let timestamp = raw
            .timestamp
            .as_deref()
            .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let cost_usd = calculate_cost(ToolSource::Codex, &session_model, &usage);

        entries.push(LogEntry {
            timestamp,
            model: session_model.clone(),
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
            project: project.clone(),
            session_id: session_id.clone(),
            cost_usd,
            billing_type,
            tool_source: ToolSource::Codex,
        });
        parsed_count += 1;
    }

    eprintln!("[burnr] codex: {} entries parsed from {:?}", parsed_count, path);
    entries
}

pub fn parse_gemini_jsonl(path: &Path, billing_type: BillingType) -> Vec<LogEntry> {
    let project = project_name_from_path(path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.trim().is_empty() {
            continue;
        }

        let raw: GeminiRawEntry = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(_) => continue,
        };

        if raw.role.as_deref() != Some("model") {
            continue;
        }

        let gemini_usage = match raw.usage_metadata {
            Some(u) => u,
            None => continue,
        };

        let model = raw.model.unwrap_or_else(|| "unknown".to_string());
        let usage = TokenUsage {
            input_tokens: gemini_usage.prompt_token_count,
            output_tokens: gemini_usage.candidates_token_count,
            cache_read_input_tokens: gemini_usage.cached_content_token_count,
            cache_creation_input_tokens: 0,
        };

        let timestamp = raw
            .timestamp
            .and_then(|ts| DateTime::parse_from_rfc3339(&ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let session_id = raw.session_id.unwrap_or_default();
        let cost_usd = calculate_cost(ToolSource::Gemini, &model, &usage);

        entries.push(LogEntry {
            timestamp,
            model,
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
            project: project.clone(),
            session_id,
            cost_usd,
            billing_type,
            tool_source: ToolSource::Gemini,
        });
    }

    entries
}

// Cursor and Windsurf: placeholder parsers using same CodexRawLine format
// Will be updated when real data format is known
pub fn parse_cursor_jsonl(path: &Path, billing_type: BillingType) -> Vec<LogEntry> {
    eprintln!("[burnr] cursor: scanning {:?}", path);
    parse_generic_codex_format(path, billing_type, ToolSource::Cursor)
}

pub fn parse_windsurf_jsonl(path: &Path, billing_type: BillingType) -> Vec<LogEntry> {
    eprintln!("[burnr] windsurf: scanning {:?}", path);
    parse_generic_codex_format(path, billing_type, ToolSource::Windsurf)
}

fn parse_generic_codex_format(path: &Path, billing_type: BillingType, tool: ToolSource) -> Vec<LogEntry> {
    let project = project_name_from_path(path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let mut entries = Vec::new();
    let mut session_model = "unknown".to_string();
    let mut session_id = String::new();

    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let raw: CodexRawLine = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(_) => continue,
        };
        match raw.r#type.as_deref() {
            Some("session_meta") => {
                if let Some(payload) = &raw.payload {
                    if let Some(id) = payload.get("id").and_then(|v| v.as_str()) {
                        session_id = id.to_string();
                    }
                }
            }
            Some("turn_context") => {
                if let Some(payload) = &raw.payload {
                    if let Some(m) = payload.get("model").and_then(|v| v.as_str()) {
                        session_model = m.to_string();
                    }
                }
            }
            _ => {}
        }
    }

    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let raw: CodexRawLine = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if raw.r#type.as_deref() != Some("event_msg") {
            continue;
        }

        let payload = match &raw.payload {
            Some(p) => p,
            None => continue,
        };

        if payload.get("type").and_then(|v| v.as_str()) != Some("token_count") {
            continue;
        }

        let info = match payload.get("info") {
            Some(i) if !i.is_null() => i,
            _ => continue,
        };

        let total_usage = match info.get("total_token_usage") {
            Some(u) => u,
            None => continue,
        };

        let tok_usage: CodexTokenUsage = match serde_json::from_value(total_usage.clone()) {
            Ok(u) => u,
            Err(_) => continue,
        };

        let usage = TokenUsage {
            input_tokens: tok_usage.input_tokens,
            output_tokens: tok_usage.output_tokens + tok_usage.reasoning_output_tokens,
            cache_read_input_tokens: tok_usage.cached_input_tokens,
            cache_creation_input_tokens: 0,
        };

        let timestamp = raw
            .timestamp
            .as_deref()
            .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let cost_usd = calculate_cost(tool, &session_model, &usage);

        entries.push(LogEntry {
            timestamp,
            model: session_model.clone(),
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
            project: project.clone(),
            session_id: session_id.clone(),
            cost_usd,
            billing_type,
            tool_source: tool,
        });
    }

    entries
}

pub fn aggregate_stats(entries: &[LogEntry]) -> GlobalStats {
    let mut projects_map: HashMap<String, Vec<&LogEntry>> = HashMap::new();
    let mut daily_map: HashMap<String, DailyUsage> = HashMap::new();
    let mut all_sessions: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for entry in entries {
        projects_map
            .entry(entry.project.clone())
            .or_default()
            .push(entry);

        let date = entry.timestamp.format("%Y-%m-%d").to_string();
        let daily = daily_map.entry(date.clone()).or_insert_with(|| DailyUsage {
            date,
            ..Default::default()
        });
        daily.input_tokens += entry.input_tokens;
        daily.output_tokens += entry.output_tokens;
        daily.cache_creation_tokens += entry.cache_creation_input_tokens;
        daily.cache_read_tokens += entry.cache_read_input_tokens;
        daily.cost_usd += entry.cost_usd;

        all_sessions.insert(&entry.session_id);
    }

    let mut projects: Vec<ProjectStats> = projects_map
        .into_iter()
        .map(|(project, entries)| {
            let mut models_map: HashMap<String, ModelStats> = HashMap::new();
            let mut sessions: std::collections::HashSet<&str> = std::collections::HashSet::new();

            for entry in &entries {
                sessions.insert(&entry.session_id);
                let model_stats = models_map
                    .entry(entry.model.clone())
                    .or_insert_with(|| ModelStats {
                        model: entry.model.clone(),
                        ..Default::default()
                    });
                model_stats.input_tokens += entry.input_tokens;
                model_stats.output_tokens += entry.output_tokens;
                model_stats.cache_creation_tokens += entry.cache_creation_input_tokens;
                model_stats.cache_read_tokens += entry.cache_read_input_tokens;
                model_stats.cost_usd += entry.cost_usd;
                model_stats.entry_count += 1;
            }

            let models_used: Vec<ModelStats> = models_map.into_values().collect();
            let has_subscription = entries
                .iter()
                .any(|e| e.billing_type == BillingType::Subscription);

            ProjectStats {
                project,
                total_input_tokens: entries.iter().map(|e| e.input_tokens).sum(),
                total_output_tokens: entries.iter().map(|e| e.output_tokens).sum(),
                total_cache_creation_tokens: entries
                    .iter()
                    .map(|e| e.cache_creation_input_tokens)
                    .sum(),
                total_cache_read_tokens: entries.iter().map(|e| e.cache_read_input_tokens).sum(),
                total_cost_usd: entries.iter().map(|e| e.cost_usd).sum(),
                cost_is_estimated: has_subscription,
                session_count: sessions.len(),
                entry_count: entries.len(),
                models_used,
            }
        })
        .collect();

    projects.sort_by(|a, b| b.total_cost_usd.partial_cmp(&a.total_cost_usd).unwrap());

    let mut daily_usage: Vec<DailyUsage> = daily_map.into_values().collect();
    daily_usage.sort_by(|a, b| a.date.cmp(&b.date));

    let cost_is_estimated = entries
        .iter()
        .any(|e| e.billing_type == BillingType::Subscription);

    GlobalStats {
        total_input_tokens: entries.iter().map(|e| e.input_tokens).sum(),
        total_output_tokens: entries.iter().map(|e| e.output_tokens).sum(),
        total_cache_creation_tokens: entries.iter().map(|e| e.cache_creation_input_tokens).sum(),
        total_cache_read_tokens: entries.iter().map(|e| e.cache_read_input_tokens).sum(),
        total_cost_usd: entries.iter().map(|e| e.cost_usd).sum(),
        cost_is_estimated,
        total_entries: entries.len(),
        total_sessions: all_sessions.len(),
        projects,
        daily_usage,
    }
}
