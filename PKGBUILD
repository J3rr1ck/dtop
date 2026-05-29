# Maintainer: Jerrick <jerrick@example.com>
pkgname=dtop
pkgver=3.0.1
pkgrel=1
pkgdesc="A high-performance system and GPU monitor with high-res charts and themes"
arch=('x86_64')
url="https://github.com/J3rr1ck/dtop"
license=('MIT') # Assuming MIT, update if different
depends=('gcc-libs' 'nvidia-utils')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('720d8917302b9107513446f9e5b82da1380c4dd04631511d91b83efc97870476') # Will be updated or user can use 'updpkgsums'

prepare() {
    cd "$pkgname-$pkgver"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --frozen
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    # install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
