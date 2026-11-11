#!/usr/bin/env bash
# R-37 thin local Format gate (mirrors CI `cargo fmt --all -- --check`).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
echo "== R-37 fmt check =="
cargo fmt --all -- --check
echo "r37_fmt_check_ok=1"
