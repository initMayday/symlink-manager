# Maintainer: initMayday <initMayday@protonmail.com>

_pkgname="symlink-manager"
pkgname="$_pkgname-git"
pkgrel=1
pkgver=r1.36c2288
pkgdesc="A way to manage your symlinks in lua"
arch=("x86_64" "arm")
url="https://github.com/initMayday/symlink-manager.git"
makedepends=("git")
depends=("lua")
provides=(symlink-manager)
conflicts=(symlink-manager)
license=("custom")
source=(git+$url)
sha256sums=("SKIP")

pkgver() {
    cd "${_pkgname}"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

package() {
    cd "${_pkgname}"
    install -Dm755 ./wrapper.sh "$pkgdir/usr/bin/symlink-manager"
    mkdir -p "$pkgdir/usr/share/${_pkgname}"
    cp -rf ./* "$pkgdir/usr/share/${_pkgname}/"
}
