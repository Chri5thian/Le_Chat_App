pkgname=le-chat-app-git
_pkgname=Le_Chat_App
pkgver=0.1.0.r0.g0000000
pkgrel=1
pkgdesc="Unofficial desktop wrapper for Mistral AI's Le Chat (Tauri)"
arch=('x86_64')
url="https://github.com/Chri5thian/Le_Chat_App"
license=('MIT')
depends=('webkit2gtk' 'gtk3' 'libayatana-appindicator' 'openssl' 'hicolor-icon-theme')
makedepends=('git' 'rust' 'cargo' 'pkgconf')
provides=('le-chat-app')
conflicts=('le-chat-app')
source=("${_pkgname}::git+${url}.git")
sha256sums=('SKIP')

pkgver() {
  cd "${srcdir}/${_pkgname}"
  git describe --long --tags --always 2>/dev/null | sed 's/^v//;s/-/./g' || \
    printf '0.1.0.r0.g%s' "$(git rev-parse --short HEAD)"
}

build() {
  cd "${srcdir}/${_pkgname}"
  mkdir -p dist
  echo '<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Le Chat</title></head><body><p>Loading...</p></body></html>' > dist/index.html
  cargo build --frozen --release
}

package() {
  cd "${srcdir}/${_pkgname}"

  install -Dm755 "target/release/mistral-chat-wrapper" "${pkgdir}/usr/bin/le-chat"

  install -Dm644 src-tauri/le-chat.desktop "${pkgdir}/usr/share/applications/le-chat.desktop"
  sed -i 's/{{app_exec}}/le-chat/g; s/{{app_icon}}/le-chat/g' "${pkgdir}/usr/share/applications/le-chat.desktop"

  install -Dm644 src-tauri/icons/128x128.png "${pkgdir}/usr/share/icons/hicolor/128x128/apps/le-chat.png"
  install -Dm644 src-tauri/icons/32x32.png "${pkgdir}/usr/share/icons/hicolor/32x32/apps/le-chat.png"

  install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
