#!/usr/bin/env bash

# cargo zigbuild --target aarch64-unknown-linux-gnu --release 
# cargo zigbuild --target x86_64-unknown-linux-gnu --release

cargo build --release --target=x86_64-unknown-linux-gnu