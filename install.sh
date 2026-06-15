#!/bin/bash

REPO="Haiko333/burnr"
API="https://api.github.com/repos/$REPO/releases/latest"

echo "Installing Burnr..."

# Arch Linux: build from source with native webkit2gtk (AppImage crashes on Arch)
if command -v pacman >/dev/null 2>&1; then
  echo "Arch Linux detected — building from source for native WebKit compatibility."
  echo ""

  # Install build dependencies
  DEPS=(webkit2gtk-4.1 gtk3 libayatana-appindicator rust nodejs npm pkgconf base-devel)
  MISSING=()
  for dep in "${DEPS[@]}"; do
    if ! pacman -Qi "$dep" >/dev/null 2>&1; then
      MISSING+=("$dep")
    fi
  done

  if [ ${#MISSING[@]} -gt 0 ]; then
    echo "Installing dependencies: ${MISSING[*]}"
    sudo pacman -S --needed --noconfirm "${MISSING[@]}"
  fi

  # Clone and build
  TMPDIR=$(mktemp -d)
  echo "Cloning source..."
  git clone --depth 1 "https://github.com/$REPO.git" "$TMPDIR/burnr"
  cd "$TMPDIR/burnr"

  echo "Installing frontend dependencies..."
  npm ci

  echo "Building (this may take a few minutes)..."
  npm run build
  cd src-tauri
  cargo build --release

  # Install binary
  DEST="$HOME/.local/bin/burnr"
  mkdir -p "$(dirname "$DEST")"
  cp target/release/burnr "$DEST"
  chmod +x "$DEST"

  # Desktop entry
  DESKTOP_DIR="$HOME/.local/share/applications"
  ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
  mkdir -p "$DESKTOP_DIR" "$ICON_DIR"
  cp icons/icon.png "$ICON_DIR/burnr.png"

  cat > "$DESKTOP_DIR/burnr.desktop" << DESKTOP
[Desktop Entry]
Name=Burnr
Comment=AI coding tools token usage tracker
Exec=$DEST
Icon=burnr
Type=Application
Categories=Development;Utility;
StartupWMClass=burnr
DESKTOP

  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
  fi
  if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
  fi

  # Cleanup
  rm -rf "$TMPDIR"

  echo ""
  echo "Installed to $DEST"
  echo "Desktop entry created — Burnr should appear in your app launcher."
  echo "(You may need to log out/in or restart your panel)"
  echo "Done!"
  exit 0
fi

# Non-Arch: download prebuilt from GitHub releases
echo "Fetching latest release from GitHub..."

RESPONSE=$(curl -sL "$API")

if echo "$RESPONSE" | grep -q "Not Found"; then
  echo ""
  echo "No release available yet (build may still be in progress)."
  echo ""
  echo "Build from source instead:"
  echo "  git clone https://github.com/$REPO.git && cd burnr"
  echo "  npm install && npm run tauri build"
  echo ""
  echo "Or check: https://github.com/$REPO/releases"
  exit 1
fi

VERSION=$(echo "$RESPONSE" | grep '"tag_name"' | head -1 | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
  echo ""
  echo "Could not parse release version."
  echo "API response (first 200 chars):"
  echo "$RESPONSE" | head -c 200
  echo ""
  echo ""
  echo "Check manually: https://github.com/$REPO/releases"
  exit 1
fi

echo "Latest version: $VERSION"

OS="$(uname -s)"
ARCH="$(uname -m)"

# Extract asset download URL by matching filename pattern in the release JSON
find_asset_url() {
  local pattern="$1"
  echo "$RESPONSE" | grep -o '"browser_download_url": *"[^"]*'"$pattern"'[^"]*"' | head -1 | sed -E 's/.*"(https:[^"]+)".*/\1/'
}

case "$OS" in
  Linux)
    if [ "$ARCH" = "x86_64" ]; then
      # Check for FUSE (required to run AppImage)
      if ! command -v fusermount >/dev/null 2>&1 && ! command -v fusermount3 >/dev/null 2>&1; then
        echo ""
        echo "AppImage requires FUSE to run. Install it first:"
        echo ""
        if command -v apt >/dev/null 2>&1; then
          echo "  sudo apt install libfuse2"
        elif command -v dnf >/dev/null 2>&1; then
          echo "  sudo dnf install fuse-libs"
        else
          echo "  Install fuse2 or libfuse2 with your package manager"
        fi
        echo ""
        echo "Then re-run this script."
        exit 1
      fi

      URL=$(find_asset_url "amd64\.AppImage")
      if [ -z "$URL" ]; then
        echo ""
        echo "Could not find AppImage asset in release $VERSION."
        echo "Check available assets at: https://github.com/$REPO/releases/tag/$VERSION"
        exit 1
      fi

      DEST="$HOME/.local/bin/burnr"
      mkdir -p "$(dirname "$DEST")"
      echo "Downloading AppImage..."
      echo "URL: $URL"
      if ! curl -fL "$URL" -o "$DEST"; then
        echo ""
        echo "Download failed."
        echo "Check available assets at: https://github.com/$REPO/releases/tag/$VERSION"
        rm -f "$DEST"
        exit 1
      fi
      chmod +x "$DEST"

      # Create .desktop entry
      DESKTOP_DIR="$HOME/.local/share/applications"
      ICON_DIR="$HOME/.local/share/icons"
      mkdir -p "$DESKTOP_DIR" "$ICON_DIR"

      curl -sL "https://raw.githubusercontent.com/$REPO/main/src-tauri/icons/icon.png" -o "$ICON_DIR/burnr.png"

      cat > "$DESKTOP_DIR/burnr.desktop" << DESKTOP
[Desktop Entry]
Name=Burnr
Comment=AI coding tools token usage tracker
Exec=$DEST
Icon=$ICON_DIR/burnr.png
Type=Application
Categories=Development;Utility;
StartupWMClass=burnr
DESKTOP

      if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
      fi

      echo ""
      echo "Installed to $DEST"
      echo "Desktop entry created — Burnr should appear in your app launcher."
      echo "(You may need to log out/in or restart your panel)"
    else
      echo "Error: Unsupported architecture $ARCH"
      exit 1
    fi
    ;;
  Darwin)
    if [ "$ARCH" = "arm64" ]; then
      URL=$(find_asset_url "aarch64\.dmg")
    else
      URL=$(find_asset_url "x64\.dmg")
    fi

    if [ -z "$URL" ]; then
      echo ""
      echo "Could not find DMG asset in release $VERSION."
      echo "Check available assets at: https://github.com/$REPO/releases/tag/$VERSION"
      exit 1
    fi

    TMPFILE="/tmp/burnr.dmg"
    echo "Downloading DMG..."
    echo "URL: $URL"
    if ! curl -fL "$URL" -o "$TMPFILE"; then
      echo ""
      echo "Download failed."
      echo "Check available assets at: https://github.com/$REPO/releases/tag/$VERSION"
      rm -f "$TMPFILE"
      exit 1
    fi
    echo "Mounting DMG..."
    hdiutil attach "$TMPFILE" -quiet
    echo "Installing to /Applications..."
    cp -R "/Volumes/Burnr/Burnr.app" /Applications/
    hdiutil detach "/Volumes/Burnr" -quiet
    rm "$TMPFILE"
    echo ""
    echo "Installed to /Applications/Burnr.app"
    echo "Run from Launchpad or: open /Applications/Burnr.app"
    ;;
  *)
    echo "Error: Unsupported OS: $OS"
    echo "For Windows, download the installer from:"
    echo "  https://github.com/$REPO/releases/latest"
    exit 1
    ;;
esac

echo "Done!"
