use crate::parser::{TokenUsage, ToolSource};

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

pub(crate) fn calculate_cost(tool: ToolSource, model: &str, usage: &TokenUsage) -> f64 {
    let pricing = match tool {
        ToolSource::ClaudeCode => get_claude_pricing(model),
        ToolSource::Codex => get_codex_pricing(model),
        ToolSource::Gemini => get_gemini_pricing(model),
        ToolSource::Cursor => get_cursor_pricing(model),
        ToolSource::Windsurf => get_windsurf_pricing(model),
    };
    // For OpenAI-style APIs (Codex, Gemini, Cursor, Windsurf), input_tokens includes
    // cached tokens. Subtract them to avoid double-counting.
    let cache_tokens = usage.cache_read_input_tokens + usage.cache_creation_input_tokens;
    let non_cache_input = if matches!(tool, ToolSource::ClaudeCode) {
        usage.input_tokens
    } else {
        usage.input_tokens.saturating_sub(cache_tokens)
    };
    let input = non_cache_input as f64 * pricing.input_per_million / 1_000_000.0;
    let output = usage.output_tokens as f64 * pricing.output_per_million / 1_000_000.0;
    let cache_read =
        usage.cache_read_input_tokens as f64 * pricing.cache_read_per_million / 1_000_000.0;
    let cache_creation =
        usage.cache_creation_input_tokens as f64 * pricing.cache_creation_per_million / 1_000_000.0;
    input + output + cache_read + cache_creation
}
