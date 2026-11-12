#!/usr/bin/env bash
# S3-I48…I54: Stage-3 stdlib-cache / hello-startup census (S3-03).
# I53: cache roundtrip · I54: prelude embed.
# Does not claim S3-03 [x] this window (claim default I55).
# See docs/census/stage3-stdlib-cache-census.md and docs/adr/stage3-s3-03-prelude-embed.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-03 stdlib-cache census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

# Structural axes (stdlib load / cache / prelude).
embedded_std_include_str=$(count_rg 'include_str!' \
  crates/syx-driver/src/embedded_std.rs)
embedded_std_virtual_path=$(count_rg 'fn virtual_path|virtual_path\(' \
  crates/syx-driver/src/embedded_std.rs)
ensure_prelude_fn=$(count_rg 'fn ensure_prelude_modules_in_graph' \
  crates/syx-driver/src/prelude_glue.rs)
cached_module_unit=$(count_rg 'struct CachedModuleUnit|CachedModuleUnit::' \
  crates/syx-driver/src/compile_cache.rs)
incremental_cache_path_fn=$(count_rg 'fn incremental_cache_path' \
  crates/syx-driver/src)
syx_cache_v_tag=$(count_rg 'syx-cache-v' \
  crates/syx-driver/src/cache_fingerprint.rs)
stdlib_virtual_in_fingerprint=$(count_rg '<stdlib>' \
  crates/syx-driver/src/cache_fingerprint.rs \
  crates/syx-driver/src/compile_cache.rs \
  crates/syx-driver/src/compile_linked.rs \
  crates/syx-driver/src/prelude_glue.rs \
  crates/syx-driver/src/embedded_std.rs)
core_prelude_entries=$(count_rg '^\s*\("[A-Za-z0-9_]+",' \
  crates/syx-driver/src/embedded_std.rs)
# S3-I49 dedicated prelude snapshot (load/store + hit counter).
dedicated_prelude_cache_hit=$(count_rg 'load_prelude_snapshot|store_prelude_snapshot|PRELUDE_SNAPSHOT_HIT|prelude_cache_path|PreludeSnapshot' \
  crates/syx-driver/src --glob '!**/tests/**')
program_trim_helper=$(count_rg 'trim_program_to_entry_reachability' \
  crates/syx-codegen/src crates/syx-driver/src --glob '!**/tests/**')
# S3-I51 warm fixed tax hinges.
cache_first_helper=$(count_rg 'try_incremental_cache_hit|S3-I51 cache-first' \
  crates/syx-driver/src/compile_linked.rs)
fingerprint_once_lock=$(count_rg 'OnceLock|SYX_RUSTC_VERSION' \
  crates/syx-driver/src/cache_fingerprint.rs crates/syx-driver/build.rs)
lazy_host_helper=$(count_rg 'ensure_host_fn|S3-I51: skip full HostFnId' \
  crates/syx-vm/src --glob '!**/tests/**')
# S3-I52 warm residual hinges.
cache_tag_bake=$(count_rg 'SYX_CACHE_VERSION_TAG' \
  crates/syx-driver/build.rs crates/syx-driver/src/cache_fingerprint.rs)
load_from_env_once=$(count_rg 'load_from_env_inner|S3-I52: OnceLock' \
  crates/syx-host-plugin/src/loader.rs)
trim_skip_warm=$(count_rg 'skip secondary trim|S3-I52: skip secondary trim' \
  crates/syx-driver/src/run_exec.rs)
# S3-I53 cache roundtrip hinge (serde always-encode for bincode).
cache_roundtrip_helper=$(count_rg 'S3-I53: always encode for bincode' \
  crates/syx-codegen/src/lib.rs)
# S3-I54 prelude embed hinges.
prelude_embed_bytes=$(count_rg 'EMBEDDED_PRELUDE_SNAPSHOT|include_bytes!.*prelude_snapshot' \
  crates/syx-driver/src/compile_cache.rs)
prelude_embed_hit=$(count_rg 'PRELUDE_EMBED_HIT|S3-I54: empty-cache cold path' \
  crates/syx-driver/src/compile_cache.rs)
if [[ -f crates/syx-driver/assets/prelude_snapshot.bin ]]; then
  prelude_embed_asset_bytes=$(wc -c <crates/syx-driver/assets/prelude_snapshot.bin | tr -d ' ')
else
  prelude_embed_asset_bytes=0
fi

echo "embedded_std_include_str=${embedded_std_include_str}"
echo "embedded_std_virtual_path=${embedded_std_virtual_path}"
echo "ensure_prelude_fn=${ensure_prelude_fn}"
echo "cached_module_unit=${cached_module_unit}"
echo "incremental_cache_path_fn=${incremental_cache_path_fn}"
echo "syx_cache_v_tag=${syx_cache_v_tag}"
echo "stdlib_virtual_path_mentions=${stdlib_virtual_in_fingerprint}"
echo "core_prelude_entry_approx=${core_prelude_entries}"
echo "dedicated_prelude_cache_hit=${dedicated_prelude_cache_hit}"
echo "program_trim_helper=${program_trim_helper}"
echo "cache_first_helper=${cache_first_helper}"
echo "fingerprint_once_lock=${fingerprint_once_lock}"
echo "lazy_host_helper=${lazy_host_helper}"
echo "cache_tag_bake=${cache_tag_bake}"
echo "load_from_env_once=${load_from_env_once}"
echo "trim_skip_warm=${trim_skip_warm}"
echo "cache_roundtrip_helper=${cache_roundtrip_helper}"
echo "prelude_embed_bytes=${prelude_embed_bytes}"
echo "prelude_embed_hit=${prelude_embed_hit}"
echo "prelude_embed_asset_bytes=${prelude_embed_asset_bytes}"

# hello wall time (optional if syx binary present).
HELLO_SRC="$ROOT/examples/hello.sy"
SYX_BIN=""
if [[ -x "$ROOT/target/release/syx" ]]; then
  SYX_BIN="$ROOT/target/release/syx"
elif [[ -x "$ROOT/target/debug/syx" ]]; then
  SYX_BIN="$ROOT/target/debug/syx"
elif command -v syx >/dev/null 2>&1; then
  SYX_BIN="$(command -v syx)"
fi

hello_cold_s="na"
hello_warm_s="na"
hello_nostd_s="na"
hello_prelude_hit_s="na"
hello_cache_bins_after="na"
entry_cache_bytes="na"
if [[ -n "$SYX_BIN" && -f "$HELLO_SRC" ]]; then
  WORK="$(mktemp -d "${TMPDIR:-/tmp}/syx-s3-03-hello.XXXXXX")"
  cp "$HELLO_SRC" "$WORK/hello.sy"
  # Cold: empty project cache.
  rm -rf "$WORK/target"
  cold_out="$(mktemp)"
  # Disk cache is opt-in (`--with-cache`); measure warm/prelude-hit with it enabled.
  if /usr/bin/time -p "$SYX_BIN" run --with-cache "$WORK/hello.sy" >"$WORK/out1.txt" 2>"$cold_out"; then
    hello_cold_s="$(awk '/^real / { print $2; exit }' "$cold_out")"
  else
    hello_cold_s="fail"
  fi
  ENTRY_BIN="$(find "$WORK/target/cache" -type f -name '*.bin' ! -name 'prelude_*' 2>/dev/null | head -1 || true)"
  if [[ -n "${ENTRY_BIN}" && -f "${ENTRY_BIN}" ]]; then
    entry_cache_bytes="$(wc -c < "${ENTRY_BIN}" | tr -d ' ')"
  fi
  # Prelude-hit: keep prelude_*.bin, delete entry module bins only.
  find "$WORK/target/cache" -type f -name '*.bin' ! -name 'prelude_*' -delete 2>/dev/null || true
  ph_out="$(mktemp)"
  if /usr/bin/time -p "$SYX_BIN" run --with-cache "$WORK/hello.sy" >"$WORK/out_ph.txt" 2>"$ph_out"; then
    hello_prelude_hit_s="$(awk '/^real / { print $2; exit }' "$ph_out")"
  else
    hello_prelude_hit_s="fail"
  fi
  # Warm: reuse same workdir / full cache.
  warm_out="$(mktemp)"
  if /usr/bin/time -p "$SYX_BIN" run --with-cache "$WORK/hello.sy" >"$WORK/out2.txt" 2>"$warm_out"; then
    hello_warm_s="$(awk '/^real / { print $2; exit }' "$warm_out")"
  else
    hello_warm_s="fail"
  fi
  # No-std: isolates non-prelude floor (probe: already near <0.15).
  nostd_out="$(mktemp)"
  rm -rf "$WORK/target"
  if /usr/bin/time -p "$SYX_BIN" --enable-std=false run "$WORK/hello.sy" >"$WORK/out3.txt" 2>"$nostd_out"; then
    hello_nostd_s="$(awk '/^real / { print $2; exit }' "$nostd_out")"
  else
    hello_nostd_s="fail"
  fi
  # Re-run once to count bins after a default-std warm (restore cache).
  "$SYX_BIN" run --with-cache "$WORK/hello.sy" >"$WORK/out4.txt" 2>/dev/null || true
  hello_cache_bins_after="$(find "$WORK/target/cache" -type f -name '*.bin' 2>/dev/null | wc -l | tr -d ' ')"
  rm -rf "$WORK" "$cold_out" "$warm_out" "$nostd_out" "$ph_out"
fi

echo "hello_entry=examples/hello.sy"
echo "hello_cold_s=${hello_cold_s}"
echo "hello_prelude_hit_s=${hello_prelude_hit_s}"
echo "hello_warm_s=${hello_warm_s}"
echo "hello_nostd_s=${hello_nostd_s}"
echo "entry_cache_bytes=${entry_cache_bytes}"
echo "hello_cache_bins_after=${hello_cache_bins_after}"
echo "hello_target_s=0.15"
echo "== census done (S3-03 I55 claim [x]; ≠ Stage-3 exit / ≠ S3-04 open this window) =="
