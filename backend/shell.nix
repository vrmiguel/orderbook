with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "orderbook-backend";

    buildInputs = [
        glibcLocales
        openssl.dev
        openssl
        pkgconfig
        redis
    ];

    OPENSSL_DEV=openssl.dev;
}