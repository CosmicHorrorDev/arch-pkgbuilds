# Maintainer: CosmicHorror <CosmicHorrorDev@pm.me>
# Maintainer: Aloso <ludwig.stecher@gmx.de>

pkgname=to-html
pkgver=0.1.6
pkgrel=1
pkgdesc='Render a terminal with ANSI colors as HTML'
arch=(x86_64)
url="https://github.com/Aloso/$pkgname"
license=(MIT)
depends=(util-linux)
makedepends=(cargo)
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha512sums=('c48e9cda4c5a62e275ce34a5f0e17d69b43fb1285cdcf97c49ea3619f6c897b9d1b3a19ba075c4db91b40db5775be60eca8c7480e506f13c122e63b0ae64b1d7')

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
  find . -name sd.fish -type f -exec install -Dm644 {} "$pkgdir/user/share/fish/vendor_completions/to-html.fish" \;
}

# vi: filetype=sh shiftwidth=2 expandtab
