#!/usr/bin/env bash
# Fail when the built syx does not embed a tag-matching prelude snapshot
# (cold run would full-compile stdlib and write prelude_*.bin under target/cache).
# Usage: build syx first, then: bash scripts/check_prelude_embed.sh [path/to/syx]
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SYX_BIN="${1:-}"
if [[ -z "$SYX_BIN" ]]; then
  if [[ -x "$ROOT/target/release/syx" ]]; then
    SYX_BIN="$ROOT/target/release/syx"
  elif [[ -x "$ROOT/target/debug/syx" ]]; then
    SYX_BIN="$ROOT/target/debug/syx"
  else
    echo "error: syx binary missing (cargo build -p syx-cli [--release])" >&2
    exit 1
  fi
fi

ASSET="$ROOT/crates/syx-driver/assets/prelude_snapshot.bin"
if [[ ! -s "$ASSET" ]]; then
  echo "error: missing or empty $ASSET" >&2
  echo "  fix: bash scripts/gen_prelude_snapshot.sh   # or: powershell -File scripts/gen_prelude_snapshot.ps1" >&2
  exit 1
fi
echo "prelude_embed_asset_bytes=$(wc -c <"$ASSET" | tr -d ' ')"
RKYV_ASSET="$ROOT/crates/syx-driver/assets/prelude_vm_image.rkyv"
if [[ -s "$RKYV_ASSET" ]]; then
  echo "prelude_rkyv_asset_bytes=$(wc -c <"$RKYV_ASSET" | tr -d ' ')"
else
  echo "warn: empty or missing $RKYV_ASSET (rkyv fastpath disabled; bincode embed still required)" >&2
fi

HELLO_SRC="$ROOT/examples/hello.sy"
if [[ ! -f "$HELLO_SRC" ]]; then
  echo "error: missing $HELLO_SRC" >&2
  exit 1
fi

WORK="$(mktemp -d "${TMPDIR:-/tmp}/syx-check-prelude.XXXXXX")"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

cp "$HELLO_SRC" "$WORK/hello.sy"
# Default run must leave no target/cache (incremental_cache off).
"$SYX_BIN" run "$WORK/hello.sy" >/dev/null
if [[ -d "$WORK/target/cache" ]]; then
  echo "error: default syx run created target/cache (incremental_cache should be off)" >&2
  find "$WORK/target/cache" -type f >&2 || true
  exit 1
fi

# With --with-cache: embed HIT must not materialize prelude_*.bin (entry .bin is OK).
"$SYX_BIN" run --with-cache "$WORK/hello.sy" >/dev/null
PRELUDE="$(find "$WORK/target/cache" -type f -name 'prelude_*.bin' 2>/dev/null | head -n 1 || true)"

if [[ -n "$PRELUDE" ]]; then
  echo "error: prelude snapshot STALE or unloadable (embed miss)" >&2
  echo "  wrote: $PRELUDE ($(wc -c <"$PRELUDE" | tr -d ' ') bytes)" >&2
  echo "  fix: bash scripts/gen_prelude_snapshot.sh   # or: powershell -File scripts/gen_prelude_snapshot.ps1" >&2
  echo "  then commit crates/syx-driver/assets/prelude_snapshot.bin (+ prelude_vm_image.rkyv)" >&2
  exit 1
fi

echo "prelude_embed_ok=1"
echo "note: default run left no target/cache; --with-cache entry cache allowed"
