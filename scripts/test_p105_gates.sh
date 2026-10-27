#!/usr/bin/env bash
# Phase 105 gate runner — avoids OOM from `cargo test -p syx-driver p105_` which
# compiles ALL integration binaries (runtime/farm/wasm/gui/platform/…) in parallel
# and can exceed tens of GB RSS.
#
# Also: never run unscoped `runtime` filters while multi-case select had a
# `while true` + broken Break lowering (fixed: nested if/else). That bug printed
# unbounded "d" lines and ballooned RSS past 60GB.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
THREADS="${TEST_THREADS:-1}"

run_suite() {
  local suite="$1"
  local filter="${2:-p105_}"
  echo "==> cargo test -p syx-driver --test ${suite} ${filter} (jobs=${CARGO_BUILD_JOBS}, threads=${THREADS})"
  cargo test -p syx-driver --test "$suite" "$filter" -- --test-threads="$THREADS"
}

run_suite runtime p105_
run_suite farm p105_
run_suite wasm p105_
run_suite gui p105_
run_suite native p105_
run_suite platform p105_
cargo test -p syx-lsp p105_ -- --test-threads="$THREADS"
cargo test -p syx-gui p105_ -- --test-threads="$THREADS"
bash scripts/audit_behavioral_tests.sh
echo "OK: Phase 105 gates finished"
