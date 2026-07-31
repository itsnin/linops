#!/usr/bin/env bash
# build musl static binaries for x86_64 and aarch64
# used by github actions release and for local testing
set -euo pipefail

echo "==> building x86_64-linux-musl"
cargo build --release --target x86_64-unknown-linux-musl

echo "==> building aarch64-linux-musl"
cargo install cross --quiet
cross build --release --target aarch64-unknown-linux-musl

echo "==> copying binaries to dist/"
mkdir -p dist
cp target/x86_64-unknown-linux-musl/release/linops  dist/linops
cp target/aarch64-unknown-linux-musl/release/linops dist/linops-aarch64

echo
echo "done artifacts in dist/"
ls -la dist/
