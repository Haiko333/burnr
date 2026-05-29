# Burnr

Desktop app that tracks token usage across AI coding tools (Claude Code, Codex, Gemini, Cursor, Windsurf).

100% local - no data sent online. Reads JSONL session logs from your filesystem.

## Features

- Token usage dashboard (input, output, cache)
- Cost estimation per model
- 365-day activity heatmap
- Per-model breakdown table
- API usage limits monitoring (Claude, Cursor, Windsurf)
- Dark/light theme
- Multi-language (EN/FR)
- Export to JSON/CSV

## Stack

- [Tauri v2](https://v2.tauri.app/) (Rust backend)
- React + TypeScript (frontend)
- Vite (bundler)

## Install

### Linux / macOS (one command)

```bash
curl -fsSL https://raw.githubusercontent.com/Haiko333/burnr/main/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/Haiko333/burnr/main/install.ps1 | iex
```

### Manual download

Grab the latest release for your platform from the [Releases page](https://github.com/Haiko333/burnr/releases/latest).

| Platform | Format |
|----------|--------|
| Linux x64 | `.AppImage`, `.deb` |
| macOS Apple Silicon | `.dmg` |
| macOS Intel | `.dmg` |
| Windows x64 | `.msi`, `.exe` |

---

## Build from source

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Platform-specific dependencies (see below)

### Linux

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Arch Linux
sudo pacman -S webkit2gtk-4.1 gtk3 libayatana-appindicator librsvg

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

### macOS

```bash
xcode-select --install
```

### Windows

- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (bundled with Windows 11, install on Windows 10)

## Development

```bash
# Install dependencies
npm install

# Run the app (frontend + Rust backend)
npm run tauri dev

# On Linux/Wayland, use:
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev
```

## Build

```bash
# Production build (creates platform-specific installer)
npm run tauri build
```

Output locations:
- **Linux**: `src-tauri/target/release/bundle/deb/` and `appimage/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Windows**: `src-tauri/target/release/bundle/msi/` and `nsis/`

## License

[MIT](LICENSE)
