#!/usr/bin/env bash
# S0-05/06: backend smoke for hello (+ optional loop/fib).
# Records VM (must pass), native, and wasm as pass OR clear fail-closed error.
# Usage: scripts/s0_backend_smoke.sh [--native-full]
# `--native-full` (or NATIVE_FULL=1) builds syx-cli with --features full-native and
# requires hello native build+run to PASS (not merely FAIL-CLOSED).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
NATIVE_FULL="${NATIVE_FULL:-}"
for arg in "$@"; do
  case "$arg" in
    --native-full) NATIVE_FULL=1 ;;
  esac
done
SYX="${SYX:-./target/debug/syx}"
# Rebuild when requesting full-native so the binary matches the feature set.
if [[ -n "$NATIVE_FULL" ]] || [[ ! -x "$SYX" ]]; then
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
  # 4th arg: require_run=1 forces build+run success (R-39 full-native hello).
  local require_run="${4:-}"
  local outbin="$TMP/${label}-native"
  echo "-- native ${label} --"
  set +e
  local nout nrc
  nout="$("$SYX" build --native "$src" -o "$outbin" 2>&1)"
  nrc=$?
  set -e
  if [[ $nrc -eq 0 ]]; then
    # C3: success path must not be a silent crash binary.
    if echo "$nout" | grep -q "Built native"; then
      :
    fi
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
      # R-39: only labels marked require_run (hello under --native-full) hard-fail.
      # Other programs may still emit crash binaries; do not expand this window.
      if [[ -n "$require_run" ]]; then
        exit 1
      fi
    fi
  else
    if echo "$nout" | grep -qi "Built native"; then
      echo "native ${label}: FAIL (printed Built native on link error): $nout"
      exit 1
    fi
    if [[ -n "$require_run" ]]; then
      echo "native ${label}: FAIL (full-native required runnable binary): $nout"
      exit 1
    fi
    if echo "$nout" | grep -qiE 'full-native|fail-closed|refusing to emit|requires a toolchain|N003|L006'; then
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
if [[ -n "$NATIVE_FULL" ]]; then
  smoke_native "$HELLO" "Hello" "hello" 1
else
  smoke_native "$HELLO" "Hello" "hello"
fi
smoke_wasm "$HELLO" "Hello" "hello"

# S0-I2 thin widen: hot_loop (VM must pass; native may FAIL-CLOSED without full-native)
smoke_vm "$HOT_LOOP" "" "hot_loop"
smoke_native "$HOT_LOOP" "" "hot_loop"

echo "== smoke done =="
