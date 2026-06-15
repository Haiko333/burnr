use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{parse_codex_jsonl, BillingType};

fn unique_temp_file(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("burnr-{name}-{nonce}.jsonl"))
}

#[test]
fn codex_parser_uses_last_token_usage_instead_of_cumulative_totals() {
    // Arrange
    let path = unique_temp_file("codex-last-usage");
    let content = r#"{"timestamp":"2026-06-15T20:00:00.000Z","type":"session_meta","payload":{"id":"session-1","cwd":"/home/alice/projects/burnr"}}
{"timestamp":"2026-06-15T20:00:01.000Z","type":"turn_context","payload":{"model":"gpt-5.5","cwd":"/home/alice/projects/burnr"}}
{"timestamp":"2026-06-15T20:00:02.000Z","type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":100,"cached_input_tokens":10,"output_tokens":20,"reasoning_output_tokens":5},"last_token_usage":{"input_tokens":100,"cached_input_tokens":10,"output_tokens":20,"reasoning_output_tokens":5}}}}
{"timestamp":"2026-06-15T20:00:03.000Z","type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":250,"cached_input_tokens":40,"output_tokens":45,"reasoning_output_tokens":15},"last_token_usage":{"input_tokens":150,"cached_input_tokens":30,"output_tokens":25,"reasoning_output_tokens":10}}}}"#;
    fs::write(&path, content).expect("fixture should be written");

    // Act
    let entries = parse_codex_jsonl(&path, BillingType::Subscription);
    let totals = entries.iter().fold((0, 0, 0), |acc, entry| {
        (
            acc.0 + entry.input_tokens,
            acc.1 + entry.cache_read_input_tokens,
            acc.2 + entry.output_tokens,
        )
    });

    // Assert
    fs::remove_file(&path).expect("fixture should be removed");
    assert_eq!(entries.len(), 2);
    assert_eq!(totals, (250, 40, 60));
    assert!(entries.iter().all(|entry| entry.project == "burnr"));
    assert!(entries.iter().all(|entry| entry.session_id == "session-1"));
}

#[test]
fn codex_parser_falls_back_to_delta_when_last_usage_is_missing() {
    // Arrange
    let path = unique_temp_file("codex-delta-usage");
    let content = r#"{"timestamp":"2026-06-15T20:00:00.000Z","type":"session_meta","payload":{"id":"session-2","cwd":"/tmp/example-app"}}
{"timestamp":"2026-06-15T20:00:01.000Z","type":"turn_context","payload":{"model":"o4-mini"}}
{"timestamp":"2026-06-15T20:00:02.000Z","type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":25,"cached_input_tokens":5,"output_tokens":10,"reasoning_output_tokens":2}}}}
{"timestamp":"2026-06-15T20:00:03.000Z","type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":60,"cached_input_tokens":15,"output_tokens":18,"reasoning_output_tokens":6}}}}"#;
    fs::write(&path, content).expect("fixture should be written");

    // Act
    let entries = parse_codex_jsonl(&path, BillingType::Subscription);
    let totals = entries.iter().fold((0, 0, 0), |acc, entry| {
        (
            acc.0 + entry.input_tokens,
            acc.1 + entry.cache_read_input_tokens,
            acc.2 + entry.output_tokens,
        )
    });

    // Assert
    fs::remove_file(&path).expect("fixture should be removed");
    assert_eq!(entries.len(), 2);
    assert_eq!(totals, (60, 15, 24));
    assert!(entries.iter().all(|entry| entry.project == "example-app"));
}
