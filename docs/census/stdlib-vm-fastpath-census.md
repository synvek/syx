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

## Measure (release, warm binary pages, temp dir, no `--with-cache`)

| Sample | Wall |
|--------|------|
| `hello` `syx run` (fastpath) | **~0.37 s** |
| `hello` `SYX_STDLIB_MIR=1` | ~0.65 s |
| `hello` `--enable-std=false` | ~0.009 s |
| `python3 -c 'print("hi")'` | ~0.015 s |
| `node -e 'console.log("hi")'` | ~0.018 s |
| `prelude_snapshot.bin` size | **~1.99 MB** (was ~6.6 MB) |

## Non-claims

- ≠ Python/Node parity (~15 ms)
- ≠ zero-copy / rkyv mmap
- ≠ daemon
- ≠ on-demand link-only (still merges stdlib funcs from graph before trim)
- ≠ native/wasm path changes (`stdlib_mir` stays on for `syx build`)

## Next knives

1. Link only imported/reachable stdlib functions (avoid fat merge-before-trim).
2. Precomputed export symbol table (skip `export_symbols` parse of prelude sources).
3. Optional mmap/rkyv image; optional `syx daemon`.
