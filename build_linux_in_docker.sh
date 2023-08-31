#!/usr/bin/env bash

# cargo zigbuild --target aarch64-unknown-linux-gnu --release 
# cargo zigbuild --target x86_64-unknown-linux-gnu --release
 
cargo build --release --target=x86_64-unknown-linux-gnu

mkdir -p ./platform-build/web
wasm-pack build --target web --release --out-dir ./platform-build/web
tar -czvf web_rust_bls_flutter.tar.gz ./platform-build/web
mv web_rust_bls_flutter.tar.gz ./platform-build/ 