#!/bin/bash
set -eu

REPO="Haiko333/burnr"
API="https://api.github.com/repos/$REPO/releases/latest"

echo "Installing Burnr..."

VERSION=$(curl -sL "$API" | grep '"tag_name"' | head -1 | sed -E 's/.*"([^"]+)".*/\1/' || echo "")
if [ -z "$VERSION" ]; then
  echo ""
  echo "No release found yet."
  echo "Burnr hasn't published a release. Build from source instead:"
  echo ""
  echo "  git clone https://github.com/$REPO.git && cd burnr"
  echo "  npm install && npm run tauri build"
  echo ""
  echo "Or check: https://github.com/$REPO/releases"
  exit 1
fi

echo "Latest version: $VERSION"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    if [ "$ARCH" = "x86_64" ]; then
      # Prefer AppImage for universal Linux support
      URL="https://github.com/$REPO/releases/download/$VERSION/burnr_${VERSION#v}_amd64.AppImage"
      DEST="$HOME/.local/bin/burnr"
      mkdir -p "$(dirname "$DEST")"
      echo "Downloading AppImage..."
      curl -L "$URL" -o "$DEST"
      chmod +x "$DEST"
      echo "Installed to $DEST"
      echo "Run with: burnr"
      echo "(Make sure ~/.local/bin is in your PATH)"
    else
      echo "Error: Unsupported architecture $ARCH"
      exit 1
    fi
    ;;
  Darwin)
    if [ "$ARCH" = "arm64" ]; then
      URL="https://github.com/$REPO/releases/download/$VERSION/Burnr_${VERSION#v}_aarch64.dmg"
    else
      URL="https://github.com/$REPO/releases/download/$VERSION/Burnr_${VERSION#v}_x64.dmg"
    fi
    TMPFILE="/tmp/burnr.dmg"
    echo "Downloading DMG..."
    curl -L "$URL" -o "$TMPFILE"
    echo "Mounting DMG..."
    hdiutil attach "$TMPFILE" -quiet
    echo "Installing to /Applications..."
    cp -R "/Volumes/Burnr/Burnr.app" /Applications/
    hdiutil detach "/Volumes/Burnr" -quiet
    rm "$TMPFILE"
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
