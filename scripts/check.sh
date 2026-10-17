#!/usr/bin/env bash

set -euo pipefail

echo "Checking Rust formatting..."
cargo fmt --all -- --check

echo "Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running tests..."
cargo test --all-targets

echo "All checks passed."
