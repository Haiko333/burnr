use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

fn get_watch_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = dirs::home_dir() {
        let candidates = [
            home.join(".claude").join("projects"),
            home.join(".codex").join("sessions"),
            home.join(".gemini").join("sessions"),
            home.join(".cursor").join("sessions"),
            home.join(".windsurf").join("sessions"),
        ];

        for path in candidates {
            if path.exists() {
                paths.push(path);
            }
        }
    }

    paths
}

pub fn start_watcher(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let paths = get_watch_paths();
        if paths.is_empty() {
            return;
        }

        let handle = app_handle.clone();
        let mut debouncer = match new_debouncer(
            Duration::from_secs(2),
            move |res: DebounceEventResult| {
                if let Ok(events) = res {
                    let has_jsonl = events.iter().any(|e| {
                        e.kind == DebouncedEventKind::Any
                            && e.path
                                .extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| ext == "jsonl")
                                .unwrap_or(false)
                    });
                    if has_jsonl {
                        let _ = handle.emit("jsonl-changed", ());
                    }
                }
            },
        ) {
            Ok(d) => d,
            Err(_) => return,
        };

        for path in &paths {
            let _ = debouncer
                .watcher()
                .watch(path, notify::RecursiveMode::Recursive);
        }

        loop {
            std::thread::sleep(Duration::from_secs(60));
        }
    });
}
