# CLAUDE.md

## Build & Dev

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev   # Full app
npm run dev                                           # Frontend only
npm run build                                        # TS check + Vite build
cd src-tauri && cargo check                          # Rust check
npm run tauri build                                  # Production bundle
```

`WEBKIT_DISABLE_DMABUF_RENDERER=1` required on Linux/Wayland. Kill stale port: `fuser -k 5173/tcp`

## Architecture

Tauri v2 desktop app. Rust backend + React/TS frontend. Reads JSONL logs from AI coding tools, displays token stats. Window close = quit (no tray).

### Data Flow

```
JSONL on disk → Rust parser → Tauri IPC → React frontend
Optional API limits → Rust HTTP → Tauri IPC → Sidebar
Auto-refresh 5min + manual button
```

### Backend (src-tauri/src/)

| File | Role |
|------|------|
| lib.rs | App setup, IPC registration, window icon, file watcher |
| parser.rs | JSONL parsing, ToolSource enum, pricing tables, aggregate_stats() |
| commands.rs | get_all_stats, get_available_tools — walkdir scanning |
| limits.rs | API limits (Claude/Cursor/Windsurf), validated session token CRUD, masked display |
| watcher.rs | notify-debouncer-mini (2s), emits "jsonl-changed" event |

### Frontend (src/)

| File | Role |
|------|------|
| App.tsx | Orchestrator, tool switching, polling |
| types.ts | TS interfaces matching Rust camelCase output |
| components/TitleBar.tsx | Custom title bar, programmatic drag |
| components/ResizeHandles.tsx | Window resize with cursor feedback |
| components/CustomCursor.tsx | Purple dot + trailing ring |
| components/Sidebar.tsx | Tool nav, limits (filtered by active tool), settings |
| components/Limits.tsx | API usage bars, per-tool filtering, hidden when empty |
| components/Settings.tsx | Language, theme, tokens (masked + reveal toggle) |
| components/Heatmap.tsx | 365-day grid, i18n days/months |
| components/StatCards.tsx | Model, 30d cost, streaks |
| components/ModelTable.tsx | Per-model breakdown |
| styles.css | Dark/light themes, CSS variables, purple accent |

### Data Sources

| Tool | Path | Filter |
|------|------|--------|
| Claude Code | ~/.claude/projects/**/*.jsonl | type: "assistant" + message.usage |
| Codex | ~/.codex/sessions/**/*.jsonl | type: "response" + usage |
| Gemini | ~/.gemini/sessions/**/*.jsonl | role: "model" + usageMetadata |
| Cursor | ~/.cursor/sessions/**/*.jsonl | type: "response" + usage |
| Windsurf | ~/.windsurf/sessions/**/*.jsonl | type: "response" + usage |

### IPC Commands

| Command | Args | Returns |
|---------|------|---------|
| get_all_stats | { billingType, tool } | GlobalStats |
| get_available_tools | — | Vec\<ToolAvailability\> |
| get_tool_limits | — | Vec\<ToolLimit\> |
| get_session_tokens | — | Vec\<SessionTokenInfo\> |
| set_session_token | { tool, token, orgId? } | Result<()> |

## Privacy/Security Notes

- Stats parsing is local-only.
- The optional Limits panel calls provider usage APIs only when a matching token is configured.
- Token tool keys are allowlisted (`claude`, `cursor`, `windsurf`) before writing local config files.
- The production app uses a CSP in `tauri.conf.json`.

## Conventions

- Rust structs: `#[serde(rename_all = "camelCase")]` — TS must match
- ToolSource: kebab-case (`"claude-code"`, `"codex"`, `"gemini"`, `"cursor"`, `"windsurf"`)
- BillingType: lowercase (`"subscription"`, `"api"`)
- i18n: all strings via `t()`, locales in `src/i18n/locales/`
- Theming: CSS vars + `[data-theme="light"]`
- No tray, no background process
- No test framework, no linter
