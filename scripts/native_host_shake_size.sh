#!/usr/bin/env bash
# Gate: examples/hello.sy --native --release must stay under MAX_HELLO_BYTES after host-shake.
# Soft-skips when the CLI lacks --features full-native (L006).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Measured floor after sql/http/tls shake (~7.8MB on darwin release); leave headroom.
# Further cuts need core-RT thinning (zstd/regex/MIR init), not domain shake.
MAX_HELLO_BYTES="${MAX_HELLO_BYTES:-9437184}" # 9 MiB
SYX_BIN="${SYX_BIN:-./target/release/syx}"
if [[ ! -x "$SYX_BIN" ]]; then
  SYX_BIN="./target/debug/syx"
fi

OUT="$(mktemp -d)/hello_shake"
mkdir -p "$(dirname "$OUT")"

err="$(mktemp)"
if ! "$SYX_BIN" build --native --release -o "$OUT" examples/hello.sy 2>"$err"; then
  if grep -Eq 'L006|full-native' "$err"; then
    echo "soft-skip native host-shake size: toolchain without --features full-native"
    cat "$err"
    rm -f "$err"
    exit 0
  fi
  cat "$err" >&2
  rm -f "$err"
  exit 1
fi
rm -f "$err"

bytes="$(wc -c <"$OUT" | tr -d ' ')"
echo "native-host-shake hello_bytes=$bytes max=$MAX_HELLO_BYTES"
if [[ "$bytes" -gt "$MAX_HELLO_BYTES" ]]; then
  echo "FAIL: hello native release $bytes > $MAX_HELLO_BYTES" >&2
  exit 1
fi

# Optional: prove rusqlite/sqlite not in hello
if command -v nm >/dev/null 2>&1; then
  if nm "$OUT" 2>/dev/null | grep -Eiq 'sqlite3_open|rusqlite'; then
    echo "FAIL: hello binary still references sqlite/rusqlite" >&2
    exit 1
  fi
  echo "native-host-shake hello_sqlite=absent"
fi
echo "native-host-shake status=ok"
