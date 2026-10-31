#!/usr/bin/env bash
# S1-10 thin slice: examples VM allowlist contract.
# Runs `syx check` + `syx run` (default VM only) on a frozen allowlist.
# native/wasm full matrix remains owed — this script does not claim three-backend GA.
# Usage: scripts/examples_vm_contract.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
SYX="${SYX:-./target/debug/syx}"
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli
  SYX=./target/debug/syx
fi

# Frozen allowlist (S0 green paths + single-file demos already green on VM).
# S1-I17 expanded ≥12; still VM-only — not examples-full / three-backend GA.
ALLOWLIST=(
  examples/hello.sy
  examples/io_demo.sy
  examples/module_demo.sy
  examples/task_demo.sy
  examples/basic_demo.sy
  examples/array_demo.sy
  examples/operator_demo.sy
  examples/object_demo.sy
  examples/hot_loop.sy
  examples/json_demo.sy
  examples/json_adv_demo.sy
  examples/log_fields.sy
  examples/hello2.sy
  examples/observability_demo.sy
  examples/class_demo.sy
  examples/collection_demo.sy
)

echo "== S1-10 examples VM allowlist contract =="
echo "syx: $SYX"
echo "backend: default VM only (native/wasm not in this slice)"
fail=0
for src in "${ALLOWLIST[@]}"; do
  if [[ ! -f "$src" ]]; then
    echo "MISSING: $src"
    fail=1
    continue
  fi
  echo "-- check $src --"
  if ! "$SYX" check "$src" >/tmp/syx-ex-check-$$.txt 2>&1; then
    echo "CHECK FAIL: $src"
    cat /tmp/syx-ex-check-$$.txt | head -20
    fail=1
    continue
  fi
  echo "-- run   $src --"
  if ! "$SYX" run "$src" >/tmp/syx-ex-run-$$.txt 2>&1; then
    echo "RUN FAIL: $src"
    cat /tmp/syx-ex-run-$$.txt | head -20
    fail=1
    continue
  fi
  echo "PASS: $src"
done
rm -f /tmp/syx-ex-check-$$.txt /tmp/syx-ex-run-$$.txt

if [[ $fail -ne 0 ]]; then
  echo "== S1-10 VM allowlist: FAIL =="
  exit 1
fi
echo "== S1-10 VM allowlist: PASS (${#ALLOWLIST[@]} files) =="
exit 0
