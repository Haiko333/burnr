#!/bin/bash

REPO="Haiko333/burnr"
API="https://api.github.com/repos/$REPO/releases/latest"

echo "Installing Burnr..."
echo "Fetching latest release from GitHub..."

RESPONSE=$(curl -sL "$API")

# Debug: check if we got a valid response
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

case "$OS" in
  Linux)
    if [ "$ARCH" = "x86_64" ]; then
      # Check for FUSE (required to run AppImage)
      if ! command -v fusermount >/dev/null 2>&1 && ! command -v fusermount3 >/dev/null 2>&1; then
        echo ""
        echo "AppImage requires FUSE to run. Install it first:"
        echo ""
        if command -v pacman >/dev/null 2>&1; then
          echo "  sudo pacman -S fuse2"
        elif command -v apt >/dev/null 2>&1; then
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

      URL="https://github.com/$REPO/releases/download/$VERSION/burnr_${VERSION#v}_amd64.AppImage"
      DEST="$HOME/.local/bin/burnr"
      mkdir -p "$(dirname "$DEST")"
      echo "Downloading AppImage..."
      echo "URL: $URL"
      curl -fL "$URL" -o "$DEST"
      chmod +x "$DEST"
      echo ""
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
    echo "URL: $URL"
    curl -fL "$URL" -o "$TMPFILE"
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
