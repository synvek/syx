#!/usr/bin/env bash
# Phase 67: permanent test count gate (total or behavioral).
set -euo pipefail

MIN_DRIVER=550
MIN_LSP=850
BEHAVIORAL=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --min-driver) MIN_DRIVER="$2"; shift 2 ;;
    --min-lsp) MIN_LSP="$2"; shift 2 ;;
    --behavioral) BEHAVIORAL=1; shift ;;
    *) echo "Unknown arg: $1" >&2; exit 2 ;;
  esac
done

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ "$BEHAVIORAL" -eq 1 ]]; then
  exec "$ROOT/scripts/audit_behavioral_tests.sh" --min-driver "$MIN_DRIVER" --min-lsp "$MIN_LSP"
fi

count_rg() {
  local path="$1"
  rg '#\[test\]' "$path" --count-matches 2>/dev/null | awk -F: '{s+=$2} END {print s+0}'
}

DRIVER=$(count_rg "crates/syx-driver")
LSP=$(count_rg "crates/syx-lsp")

echo "driver tests: $DRIVER (min $MIN_DRIVER)"
echo "lsp tests:    $LSP (min $MIN_LSP)"

fail=0
if [[ "$DRIVER" -lt "$MIN_DRIVER" ]]; then
  echo "FAIL: driver test count $DRIVER < $MIN_DRIVER" >&2
  fail=1
fi
if [[ "$LSP" -lt "$MIN_LSP" ]]; then
  echo "FAIL: lsp test count $LSP < $MIN_LSP" >&2
  fail=1
fi

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "OK: test counts meet Phase 67 gates"
