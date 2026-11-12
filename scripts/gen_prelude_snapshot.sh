#!/usr/bin/env bash
# S3-I54: generate crates/syx-driver/assets/prelude_snapshot.bin (+ prelude_vm_image.rkyv)
# from a release syx run. Does not compile .sy inside build.rs — run after (or this
# script will trigger) `cargo build -p syx-cli --release`.
#
# Chicken-egg: a tag-matching embed short-circuits ensure_prelude and writes no
# prelude_*.bin. This script zeros the assets and rebuilds tip syx first so cold
# store always runs, then rebuilds again so include_bytes picks up the new blobs.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

HELLO_SRC="$ROOT/examples/hello.sy"
if [[ ! -f "$HELLO_SRC" ]]; then
  echo "error: missing $HELLO_SRC" >&2
  exit 1
fi

OUT_DIR="$ROOT/crates/syx-driver/assets"
mkdir -p "$OUT_DIR"
OUT_BIN="$OUT_DIR/prelude_snapshot.bin"
OUT_RKYV="$OUT_DIR/prelude_vm_image.rkyv"

echo "invalidating embed assets so cold store can run…"
: > "$OUT_BIN"
: > "$OUT_RKYV"
echo "building tip syx (empty embed)…"
cargo build -p syx-cli --release

SYX_BIN="$ROOT/target/release/syx"
if [[ ! -x "$SYX_BIN" ]]; then
  echo "error: missing $SYX_BIN after build" >&2
  exit 1
fi

WORK="$(mktemp -d "${TMPDIR:-/tmp}/syx-gen-prelude.XXXXXX")"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

cp "$HELLO_SRC" "$WORK/hello.sy"
echo "cold-compiling prelude via $SYX_BIN run --with-cache…"
"$SYX_BIN" run --with-cache "$WORK/hello.sy" >/dev/null

PRELUDE="$(find "$WORK/target/cache" -type f -name 'prelude_*.bin' 2>/dev/null | head -n 1 || true)"
if [[ -z "$PRELUDE" || ! -s "$PRELUDE" ]]; then
  echo "error: no prelude_*.bin produced under $WORK/target/cache" >&2
  find "$WORK" -type f 2>/dev/null | head -50 >&2 || true
  exit 1
fi

cp "$PRELUDE" "$OUT_BIN"
echo "wrote $OUT_BIN ($(wc -c <"$OUT_BIN" | tr -d ' ') bytes) from $PRELUDE"

RKYV="$(find "$WORK/target/cache" -type f -name 'prelude_*.rkyv' 2>/dev/null | head -n 1 || true)"
if [[ -n "$RKYV" && -s "$RKYV" ]]; then
  cp "$RKYV" "$OUT_RKYV"
  echo "wrote $OUT_RKYV ($(wc -c <"$OUT_RKYV" | tr -d ' ') bytes) from $RKYV"
else
  echo "warn: no prelude_*.rkyv produced (rkyv embed stays empty until next cold store)" >&2
fi

echo "rebuilding syx so include_bytes picks up the assets…"
cargo build -p syx-cli --release
echo "verifying embed hit…"
bash "$ROOT/scripts/check_prelude_embed.sh" "$ROOT/target/release/syx"
echo "done — commit $OUT_BIN / $OUT_RKYV if this is a source landing"
