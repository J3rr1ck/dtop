# Maintainer: Jerrick <jerrick@example.com>
pkgname=danger-monitor
pkgver=3.0.1
pkgrel=2
pkgdesc="A high-performance system and GPU monitor (danmon) with high-res charts and themes"
arch=('x86_64')
url="https://github.com/J3rr1ck/dtop"
license=('MIT')
depends=('gcc-libs' 'nvidia-utils')
makedepends=('rust' 'cargo')
source=("danger-monitor-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('720d8917302b9107513446f9e5b82da1380c4dd04631511d91b83efc97870476')

prepare() {
    cd "dtop-$pkgver"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "dtop-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "dtop-$pkgver"
    cargo test --frozen
}

package() {
    cd "dtop-$pkgver"
    install -Dm755 "target/release/danger-monitor" "$pkgdir/usr/bin/danmon"
}
