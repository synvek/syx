#!/usr/bin/env bash
# Phase 70: WASM AOT direct emit ratio gate (strict 99% / full main 100%).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MIN_DIRECT=""
MATRIX=0
FULL=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --min-direct)
      MIN_DIRECT="$2"
      shift 2
      ;;
    --matrix)
      MATRIX=1
      shift
      ;;
    --full)
      FULL=1
      shift
      ;;
    -h|--help)
      echo "usage: $0 [--matrix] [--full] [--min-direct RATIO] [source.sy]"
      exit 0
      ;;
    *)
      shift
      ;;
  esac
done

if [[ -z "$MIN_DIRECT" ]]; then
  if [[ "$FULL" -eq 1 ]]; then
    MIN_DIRECT="1.0"
  elif [[ "$MATRIX" -eq 1 ]]; then
    MIN_DIRECT="0.99"
  else
    MIN_DIRECT="0.95"
  fi
fi

cd "$ROOT"
cargo build -p syx-driver --bin wasm_aot_matrix_gate -q

if [[ "$MATRIX" -eq 1 ]]; then
  if [[ "$FULL" -eq 1 ]]; then
    RATIO="$(./target/debug/wasm_aot_matrix_gate --matrix --full)"
    echo "WASM AOT direct ratio (full tier incl. main): $RATIO (min $MIN_DIRECT)"
  else
    RATIO="$(./target/debug/wasm_aot_matrix_gate --matrix)"
    echo "WASM AOT direct ratio (strict fixtures min): $RATIO (min $MIN_DIRECT)"
  fi
else
  RATIO="$(./target/debug/wasm_aot_matrix_gate)"
  echo "WASM AOT direct ratio: $RATIO (min $MIN_DIRECT) for examples/hot_loop.sy"
fi

awk -v ratio="$RATIO" -v min="$MIN_DIRECT" 'BEGIN {
  if (ratio + 0 >= min + 0) exit 0; else exit 1
}' || {
  echo "FAIL: direct ratio $RATIO < $MIN_DIRECT" >&2
  exit 1
}

echo "PASS: WASM AOT ratio gate"
