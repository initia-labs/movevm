#!/bin/bash
set -o errexit -o nounset -o pipefail

# create artifacts directory
mkdir -p artifacts

# set pkg_config to allow cross compile
export PKG_CONFIG_ALLOW_CROSS=1

# ref: https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html
export PATH="/opt/osxcross/target/bin:$PATH"
export LIBZ_SYS_STATIC=1

# See https://github.com/CosmWasm/wasmvm/issues/222#issuecomment-880616953 for two approaches to
# enable stripping through cargo (if that is desired).

echo "Starting aarch64-apple-darwin build"
export CC=aarch64-apple-darwin20.4-clang
export CXX=aarch64-apple-darwin20.4-clang++
(cd libmovevm && cargo build --release --target aarch64-apple-darwin)
(cd libcompiler && cargo build --release --target aarch64-apple-darwin)

echo "Starting x86_64-apple-darwin build"
export CC=o64-clang
export CXX=o64-clang++
(cd libmovevm && cargo build --release --target x86_64-apple-darwin)
(cd libcompiler && cargo build --release --target x86_64-apple-darwin)

# Create a universal library with both archs
lipo -output artifacts/libmovevm.dylib -create \
  ./target/x86_64-apple-darwin/release/deps/libmovevm.dylib \
  ./target/aarch64-apple-darwin/release/deps/libmovevm.dylib

lipo -output artifacts/libcompiler.dylib -create \
  ./target/x86_64-apple-darwin/release/deps/libcompiler.dylib \
  ./target/aarch64-apple-darwin/release/deps/libcompiler.dylib
