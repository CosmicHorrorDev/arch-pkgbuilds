# Maintainer: CosmicHorror <CosmicHorrorDev@pm.me>
# Maintainer: Aloso <ludwig.stecher@gmx.de>

pkgname=to-html
pkgver=0.1.5
pkgrel=1
pkgdesc='Render a terminal with ANSI colors as HTML'
arch=(x86_64)
url="https://github.com/Aloso/$pkgname"
license=(MIT)
depends=(util-linux)
makedepends=(cargo)
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
b2sums=('c6da3d285c1594c23e3ea0c25963a4e11fc7beac579e0bb1e2a41c1a5198c49b900a534a3d8c03207284c07cb46e90fe5c0a4b001bd1cc4be2cb9c578e463748')

prepare() {
  cd "$srcdir/$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --release --locked
}

check() {
  cd "$srcdir/$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo test --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # Install files that are generated with a build script
  find . -name to-html.bash -type f -exec install -Dm644 {} "$pkgdir/user/share/bash-completions/completions/to-html" \;
  find . -name _to-html -type f -exec install -Dm644 {} "$pkgdir/user/share/zsh/site-functions/_to_html" \;
  find . -name to-html.fish -type f -exec install -Dm644 {} "$pkgdir/user/share/fish/vendor_completions/to-html.fish" \;
}

# vi: filetype=sh shiftwidth=2 expandtab
