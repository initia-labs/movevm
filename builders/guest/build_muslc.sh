#!/bin/sh
set -e # Note we are not using bash here but the Alpine default shell

# create artifacts directory
mkdir -p artifacts

# set pkg_config to allow cross compile
export PKG_CONFIG_ALLOW_CROSS=1

echo "Starting x86_64-unknown-linux-musl build"
export OPENSSL_DIR=/opt/x86_64-openssl
(cd libmovevm && cargo build --release --target x86_64-unknown-linux-musl --example movevmstatic)
(cd libcompiler && cargo build --release --target x86_64-unknown-linux-musl --example compilerstatic)
cp ./target/x86_64-unknown-linux-musl/release/examples/libmovevmstatic.a artifacts/libmovevm_muslc.x86_64.a
cp ./target/x86_64-unknown-linux-musl/release/examples/libcompilerstatic.a artifacts/libcompiler_muslc.x86_64.a

# See https://github.com/CosmWasm/wasmvm/issues/222#issuecomment-880616953 for two approaches to
# enable stripping through cargo (if that is desired).

echo "Starting aarch64-unknown-linux-musl build"
export CC=aarch64-linux-musl-gcc
export OPENSSL_DIR=/opt/aarch64-openssl

(cd libmovevm && cargo build --release --target aarch64-unknown-linux-musl --example movevmstatic)
(cd libcompiler && cargo build --release --target aarch64-unknown-linux-musl --example compilerstatic)
cp ./target/aarch64-unknown-linux-musl/release/examples/libmovevmstatic.a artifacts/libmovevm_muslc.aarch64.a
cp ./target/aarch64-unknown-linux-musl/release/examples/libcompilerstatic.a artifacts/libcompiler_muslc.aarch64.a
