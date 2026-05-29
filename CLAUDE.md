# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Run the full Tauri app (frontend + backend)
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev

# Frontend only (Vite dev server on port 5173)
npm run dev

# TypeScript type check + Vite production build
npm run build

# Rust check/build only
cd src-tauri && cargo check
cd src-tauri && cargo build

# Build production Tauri bundle
npm run tauri build
```

The `WEBKIT_DISABLE_DMABUF_RENDERER=1` env var is required on Wayland/Linux to avoid WebKit rendering issues.

If port 5173 is already in use from a stale process: `fuser -k 5173/tcp`

## Architecture

Burnr is a **Tauri v2** desktop app (Rust backend + React/TypeScript frontend) that reads JSONL session logs from AI coding tools and displays token usage statistics. Window close quits the app (no tray icon).

### Data Flow

```
JSONL files on disk → Rust parser → Tauri IPC (invoke) → React frontend
API limits (claude.ai, cursor.com) → Rust HTTP fetch → Tauri IPC → Sidebar limits
Data refreshes every 5 minutes + manual refresh button
```

### Backend (src-tauri/src/)

- **lib.rs** — Tauri app setup: registers IPC commands, starts file watcher
- **parser.rs** — Core parsing logic. Defines `ToolSource` enum (ClaudeCode, Codex, Gemini, Cursor, Windsurf), per-tool JSONL structs, pricing tables, and `aggregate_stats()`
- **commands.rs** — IPC commands: `get_all_stats(billingType, tool)` and `get_available_tools()`. Scans directories with walkdir
- **limits.rs** — Fetches real usage limits from external APIs (Claude.ai rate_limit_status, Cursor usage API, Windsurf). Reads session tokens from `~/.config/burnr/{tool}_session.txt`. Org IDs from `~/.config/burnr/{tool}_org.txt`. Provides `get_tool_limits`, `get_session_tokens`, `set_session_token` commands
- **cookies.rs** — Attempts auto-detection of browser cookies (Chromium/KWallet). Falls back silently on failure
- **watcher.rs** — Background thread using `notify-debouncer-mini` (2s debounce). Watches all tool directories and emits `"jsonl-changed"` event

### Frontend (src/)

- **App.tsx** — Main orchestrator. Loading/error/empty states, 5min polling interval, tool switching
- **types.ts** — Shared TypeScript interfaces matching Rust's camelCase serialized output
- **toolsConfig.tsx** — Tool list configuration with SVG icons
- **i18n/** — i18next setup with en.json/fr.json translations
- **hooks/useTheme.ts** — Dark/light theme with CSS variable switching via `data-theme` attribute
- **components/TitleBar.tsx** — Custom window controls (minimize/maximize/close), no OS decorations
- **components/Sidebar.tsx** — Tool list, inline Limits component, Settings button
- **components/Limits.tsx** — Progress bars showing API-fetched usage limits (5min auto-refresh, always visible with empty state)
- **components/Settings.tsx** — Language, theme, session token management (per-tool help icons), export/import
- **components/Header.tsx** — Stats row: Input, Output, Cache, Sessions, Total, Cost + refresh button
- **components/Heatmap.tsx** — 365-day activity grid
- **components/StatCards.tsx** — Most used model, 30d cost, streaks
- **components/ModelTable.tsx** — Per-model breakdown table
- **styles.css** — Dark/light themes via CSS variables, purple-only accent palette

### Data Sources

| Tool | Directory | Filter condition |
|------|-----------|-----------------|
| Claude Code | `~/.claude/projects/**/*.jsonl` | `type: "assistant"` with `message.usage` |
| Codex | `~/.codex/sessions/**/*.jsonl` | `type: "response"` with `usage` |
| Gemini | `~/.gemini/sessions/**/*.jsonl` | `role: "model"` with `usageMetadata` |
| Cursor | `~/.cursor/sessions/**/*.jsonl` | `type: "response"` with `usage` |
| Windsurf | `~/.windsurf/sessions/**/*.jsonl` | `type: "response"` with `usage` |

### Usage Limits (API-based)

- **Claude Code**: Fetches `GET https://api.claude.ai/api/organizations/{org_id}/rate_limit_status` with session cookie
- **Cursor**: Fetches `GET https://www.cursor.com/api/usage` with WorkosCursorSessionToken cookie
- **Windsurf**: Fetches usage API with session cookie
- Session tokens stored at `~/.config/burnr/{tool}_session.txt`
- Org IDs stored at `~/.config/burnr/{tool}_org.txt`

### IPC Commands

| Command | Args | Returns |
|---------|------|---------|
| `get_all_stats` | `{ billingType, tool }` | `GlobalStats` |
| `get_available_tools` | none | `Vec<ToolAvailability>` |
| `get_tool_limits` | none | `Vec<ToolLimit>` |
| `get_session_tokens` | none | `Vec<SessionTokenInfo>` |
| `set_session_token` | `{ tool, token, orgId? }` | `Result<()>` |

### Asset Locations

- **Tool logos**: `src/assets/logos/{claude,codex,gemini,cursor,windsurf}.png` (user-provided)
- **App icon**: `src-tauri/icons/icon.png` (used for taskbar)

## Key Conventions

- Rust output structs use `#[serde(rename_all = "camelCase")]` — TypeScript interfaces must match
- `ToolSource` serializes as kebab-case (`"claude-code"`, `"codex"`, `"gemini"`, `"cursor"`, `"windsurf"`)
- `BillingType` serializes as lowercase (`"subscription"`, `"api"`)
- i18n: all UI strings go through `react-i18next` `t()` function, translations in `src/i18n/locales/`
- Theming: CSS variables switch via `[data-theme="light"]` selector
- Window close → quit (no tray, no background process)
- No test framework set up yet
- No linter configured
