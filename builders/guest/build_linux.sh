#!/bin/bash
set -o errexit -o nounset -o pipefail

# create artifacts directory
mkdir -p artifacts

# set pkg_config to allow cross compile
export PKG_CONFIG_ALLOW_CROSS=1

# See https://github.com/CosmWasm/wasmvm/issues/222#issuecomment-880616953 for two approaches to
# enable stripping through cargo (if that is desired).

echo "Starting x86_64-unknown-linux-gnu build"
export CC=clang
export CXX=clang++
export OPENSSL_STATIC=1
export OPENSSL_DIR=/opt/x86_64-openssl
(cd libmovevm && cargo build --release --target x86_64-unknown-linux-gnu)
(cd libcompiler && cargo build --release --target x86_64-unknown-linux-gnu)
cp ./target/x86_64-unknown-linux-gnu/release/libmovevm.so artifacts/libmovevm.x86_64.so
cp ./target/x86_64-unknown-linux-gnu/release/libcompiler.so artifacts/libcompiler.x86_64.so

echo "Starting aarch64-unknown-linux-gnu build"
export qemu_aarch64="qemu-aarch64 -L /usr/aarch64-linux-gnu"
export CC_aarch64_unknown_linux_gnu=clang
export AR_aarch64_unknown_linux_gnu=llvm-ar
export CFLAGS_aarch64_unknown_linux_gnu="--sysroot=/usr/aarch64-linux-gnu"
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="$qemu_aarch64"
export OPENSSL_STATIC=1
export OPENSSL_DIR=/opt/aarch64-openssl

# build libmovevm for aarch64
(cd libmovevm && cargo build --release --target aarch64-unknown-linux-gnu)
(cd libcompiler && cargo build --release --target aarch64-unknown-linux-gnu)
cp -R ./target/aarch64-unknown-linux-gnu/release/libmovevm.so artifacts/libmovevm.aarch64.so
cp -R ./target/aarch64-unknown-linux-gnu/release/libcompiler.so artifacts/libcompiler.aarch64.so
