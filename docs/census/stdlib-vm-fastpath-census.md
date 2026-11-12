# Stdlib VM-image fastpath (first slice)

**Date:** 2026-11-12  
**Scope:** Interpret `syx run` skips prelude MIR rebuild; deduped embed + extension-method side table.

## Changes

| Piece | Behavior |
|-------|----------|
| `DriverOptions.stdlib_mir` | Default `true` (build/native/wasm). CLI **`syx run` sets `false`**. |
| `SYX_STDLIB_MIR` | `0/false` force off; `1/true` force on (debug). |
| `store_prelude_snapshot` | Dedupe `FunctionUnit` by name across modules; clear per-module `host_fn_names`; persist `extension_methods`. |
| `ModuleGraph.stdlib_extension_methods` | Filled on embed load → `collect_stdlib_extension_methods` skips re-parse. |
| Asset | `crates/syx-driver/assets/prelude_snapshot.bin` regen via `scripts/gen_prelude_snapshot.sh`. |

## Slice 2 (on-demand link + export table)

| Piece | Behavior |
|-------|----------|
| `stdlib_fastpath::link_stdlib_callees` | Pull MIR-reachable embed fns with Call remap |
| Skip stdlib re-lower / fat merge when `!want_stdlib_mir` | No parse-all-prelude in `build_mir_from_graph` |
| `PreludeSnapshot.module_exports` | Precomputed exports → skip `export_symbols` parse |
| `callable_extensions` from embed fn names | `Double.pi` etc. lower without stdlib MIR |

## Measure (release, warm binary pages, temp dir, no `--with-cache`)

| Sample | Wall |
|--------|------|
| `hello` `syx run` (fastpath) | **~0.019–0.020 s** |
| `basic_demo` | OK (30 lines stdout) |
| `hello` `--enable-std=false` | ~0.009 s |
| `python3 -c 'print("hi")'` | ~0.015 s |
| `node -e 'console.log("hi")'` | ~0.018 s |
| `prelude_snapshot.bin` size | **~2.0 MB** (was ~6.6 MB) |

## Non-claims

- ≠ every script ≤20 ms (import-heavy / compute-bound still slower)
- ≠ zero-copy / rkyv mmap
- ≠ daemon
- ≠ native/wasm path changes (`stdlib_mir` stays on for `syx build`)

## Next knives

1. Optional mmap/rkyv image; optional `syx daemon`.
2. Further shrink deserialize tax (~2 MB bincode still on every process).
