# Maintainer: Konrad Wasowicz <exaroth@gmail.com>
pkgname=liveboat
pkgver=1.0.0
pkgrel=1
pkgdesc="Static page generator for Newsboat feeds"
arch=('x86_64')
url="https://github.com/exaroth/liveboat"
license=('MIT')
makedepends=('rust')
source=("https://github.com/exaroth/$pkgname/archive/refs/tags/v$pkgver.tar.gz")
md5sums=('0ee4fd359d8e64e0d08245ca657ed745')
# Non standard
_newsboat_githash="1ea89e860553634e37daf1cc6de2e00a379d6378"
_target="x86_64-unknown-linux-musl"

prepare() {
    echo $pkgdir
    cd "$pkgname-$pkgver"
    git clone https://github.com/newsboat/newsboat.git sub/newsboat
    cd sub/newsboat && git checkout $_newsboat_githash
    rustup target add $_target
}

build() {
    cd "$pkgname-$pkgver"
    make build
}

package() {
    cd "$pkgname-$pkgver"
	install -dm0755 "/$pkgdir/usr/bin"
    install target/$_target/release/$pkgname $pkgdir/usr/bin
    install -Dm644 -t "$pkgdir"/usr/share/licenses/$pkgname LICENSE
}
