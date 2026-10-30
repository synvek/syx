#!/usr/bin/env bash
# S0-05/06: backend smoke for hello (+ optional loop/fib).
# Records VM (must pass), native, and wasm as pass OR clear fail-closed error.
# Usage: scripts/s0_backend_smoke.sh [--native-full]
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
SYX="${SYX:-./target/debug/syx}"
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli ${NATIVE_FULL:+--features full-native}
  SYX=./target/debug/syx
fi

HELLO="${HELLO:-examples/hello.sy}"
HOT_LOOP="${HOT_LOOP:-examples/hot_loop.sy}"
TMP="${TMPDIR:-/tmp}/syx-s0-smoke-$$"
mkdir -p "$TMP"
trap 'rm -rf "$TMP"' EXIT

os_name="$(uname -s)"
arch_name="$(uname -m)"
echo "== S0 backend smoke ($os_name/$arch_name) =="

smoke_vm() {
  local src="$1"
  local expect="${2:-}"
  local label="$3"
  echo "-- VM ${label} --"
  local out
  out="$("$SYX" run "$src" 2>&1)" || { echo "VM FAIL (${label}): $out"; exit 1; }
  if [[ -n "$expect" ]]; then
    echo "$out" | grep -q "$expect" || { echo "VM FAIL (${label}): unexpected output: $out"; exit 1; }
  fi
  echo "VM ${label}: PASS"
}

smoke_native() {
  local src="$1"
  local expect="${2:-}"
  local label="$3"
  local outbin="$TMP/${label}-native"
  echo "-- native ${label} --"
  set +e
  local nout nrc
  nout="$("$SYX" build --native "$src" -o "$outbin" 2>&1)"
  nrc=$?
  set -e
  if [[ $nrc -eq 0 ]]; then
    set +e
    local runout rrc
    runout="$("$outbin" 2>&1)"
    rrc=$?
    set -e
    if [[ $rrc -eq 0 ]]; then
      if [[ -n "$expect" ]] && ! echo "$runout" | grep -q "$expect"; then
        echo "native ${label}: FAIL (built but unexpected output): $runout"
        exit 1
      fi
      echo "native ${label}: PASS (built + ran)"
    else
      echo "native ${label}: FAIL (built but run rc=$rrc): $runout"
      exit 1
    fi
  else
    if echo "$nout" | grep -qiE 'full-native|fail-closed|refusing to emit|requires a toolchain'; then
      echo "native ${label}: FAIL-CLOSED (clear error, no crash binary)"
      echo "$nout" | tail -3
    else
      echo "native ${label}: FAIL (unclear error): $nout"
      exit 1
    fi
  fi
}

smoke_wasm() {
  local src="$1"
  local expect="${2:-}"
  local label="$3"
  local outwasm="$TMP/${label}.wasm"
  echo "-- wasm ${label} --"
  set +e
  local wout wrc
  wout="$("$SYX" build --wasm "$src" -o "$outwasm" 2>&1)"
  wrc=$?
  set -e
  if [[ $wrc -eq 0 ]]; then
    set +e
    local rout rrc
    rout="$("$SYX" run --wasm "$src" 2>&1)"
    rrc=$?
    set -e
    if [[ $rrc -eq 0 ]]; then
      if [[ -n "$expect" ]] && ! echo "$rout" | grep -q "$expect"; then
        echo "wasm ${label}: FAIL (built but unexpected output): $rout"
        exit 1
      fi
      echo "wasm ${label}: PASS (built + ran)"
    else
      echo "wasm ${label}: FAIL (built but run rc=$rrc): $rout"
      exit 1
    fi
  else
    if echo "$wout" | grep -qiE 'validation|fail-closed|failed validation'; then
      echo "wasm ${label}: FAIL-CLOSED (clear validation error)"
      echo "$wout" | tail -5
    else
      echo "wasm ${label}: FAIL (unclear error): $wout"
      exit 1
    fi
  fi
}

smoke_vm "$HELLO" "Hello" "hello"
smoke_native "$HELLO" "Hello" "hello"
smoke_wasm "$HELLO" "Hello" "hello"

# S0-I2 thin widen: hot_loop (VM must pass; native may FAIL-CLOSED without full-native)
smoke_vm "$HOT_LOOP" "" "hot_loop"
smoke_native "$HOT_LOOP" "" "hot_loop"

echo "== smoke done =="
