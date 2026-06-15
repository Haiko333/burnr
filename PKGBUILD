pkgname=burnr
pkgver=0.1.1
pkgrel=1
pkgdesc="AI coding tools token usage tracker"
arch=('x86_64')
url="https://github.com/Haiko333/burnr"
license=('MIT')
depends=('webkit2gtk-4.1' 'gtk3' 'libayatana-appindicator')
makedepends=('rust' 'cargo' 'nodejs' 'npm' 'pkgconf')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Haiko333/burnr/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname-$pkgver"
  npm ci
}

build() {
  cd "$pkgname-$pkgver"
  npm run build
  cd src-tauri
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "src-tauri/target/release/burnr" "$pkgdir/usr/bin/burnr"
  install -Dm644 "src-tauri/icons/icon.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/burnr.png"

  install -Dm644 /dev/stdin "$pkgdir/usr/share/applications/burnr.desktop" << EOF
[Desktop Entry]
Name=Burnr
Comment=AI coding tools token usage tracker
Exec=burnr
Icon=burnr
Type=Application
Categories=Development;Utility;
StartupWMClass=burnr
EOF

  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
