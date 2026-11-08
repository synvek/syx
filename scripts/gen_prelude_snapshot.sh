#!/usr/bin/env bash
# S3-I54: generate crates/syx-driver/assets/prelude_snapshot.bin from a release syx run.
# Does not compile .sy inside build.rs — run after `cargo build -p syx-cli --release`.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SYX_BIN=""
if [[ -x "$ROOT/target/release/syx" ]]; then
  SYX_BIN="$ROOT/target/release/syx"
elif [[ -x "$ROOT/target/debug/syx" ]]; then
  SYX_BIN="$ROOT/target/debug/syx"
else
  echo "error: build syx first (cargo build -p syx-cli --release)" >&2
  exit 1
fi

HELLO_SRC="$ROOT/examples/hello.sy"
if [[ ! -f "$HELLO_SRC" ]]; then
  echo "error: missing $HELLO_SRC" >&2
  exit 1
fi

OUT_DIR="$ROOT/crates/syx-driver/assets"
mkdir -p "$OUT_DIR"
OUT_BIN="$OUT_DIR/prelude_snapshot.bin"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/syx-gen-prelude.XXXXXX")"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

cp "$HELLO_SRC" "$WORK/hello.sy"
# Cold compile writes prelude_{digest}.bin under project cache.
"$SYX_BIN" run "$WORK/hello.sy" >/dev/null

PRELUDE="$(find "$WORK/target/cache" -type f -name 'prelude_*.bin' 2>/dev/null | head -n 1 || true)"
if [[ -z "$PRELUDE" || ! -s "$PRELUDE" ]]; then
  echo "error: no prelude_*.bin produced under $WORK/target/cache" >&2
  find "$WORK" -type f 2>/dev/null | head -50 >&2 || true
  exit 1
fi

cp "$PRELUDE" "$OUT_BIN"
echo "wrote $OUT_BIN ($(wc -c <"$OUT_BIN" | tr -d ' ') bytes) from $PRELUDE"
echo "note: rebuild syx so include_bytes picks up the asset (tag must match SYX_CACHE_VERSION_TAG)"
