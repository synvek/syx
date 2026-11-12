# Stdlib VM-image fastpath (first slice)

**Date:** 2026-11-12  
**Scope:** Interpret `syx run` skips prelude MIR rebuild; deduped embed + extension-method side table; optional rkyv/mmap image + Unix daemon hydrate reuse.

## Changes

| Piece | Behavior |
|-------|----------|
| `DriverOptions.stdlib_mir` | Default `true` (build/native/wasm). CLI **`syx run` sets `false`**. |
| `SYX_STDLIB_MIR` | `0/false` force off; `1/true` force on (debug). |
| `store_prelude_snapshot` | Dedupe `FunctionUnit` by name across modules; clear per-module `host_fn_names`; persist `extension_methods` + slim **rkyv** image. |
| `ModuleGraph.stdlib_extension_methods` | Filled on embed load → `collect_stdlib_extension_methods` skips re-parse. |
| Asset | `crates/syx-driver/assets/prelude_snapshot.bin` (+ `prelude_vm_image.rkyv`) via `scripts/gen_prelude_snapshot.sh` / `.ps1`. |

## Slice 2 (on-demand link + export table)

| Piece | Behavior |
|-------|----------|
| `stdlib_fastpath::link_stdlib_callees` | Pull MIR-reachable embed fns with Call remap |
| Skip stdlib re-lower / fat merge when `!want_stdlib_mir` | No parse-all-prelude in `build_mir_from_graph` |
| `PreludeSnapshot.module_exports` | Precomputed exports → skip `export_symbols` parse |
| `callable_extensions` from embed fn names | `Double.pi` etc. lower without stdlib MIR |

## Slice 3 (mmap/rkyv + daemon)

| Piece | Behavior |
|-------|----------|
| `rkyv_image` | Slim image: fn metadata + **bincode Instruction blobs**; load via embed or `memmap2` disk `.rkyv` |
| Process `OnceLock` hydrate | Skip re-deserialize within one process (daemon multi-run) |
| `syx daemon start/stop/ping` | Unix domain socket; warms prelude once |
| `syx run --daemon` / `SYX_USE_DAEMON=1` | Client forwards run to warm daemon |
| Hit counter | `PRELUDE_RKYV_HIT` |

## Measure (release, warm binary pages, temp dir, no `--with-cache`)

| Sample | Wall |
|--------|------|
| `hello` `syx run` (fastpath) | **~0.019–0.020 s** |
| `basic_demo` | OK (30 lines stdout) |
| `hello` `--enable-std=false` | ~0.009 s |
| `python3 -c 'print("hi")'` | ~0.015 s |
| `node -e 'console.log("hi")'` | ~0.018 s |
| `prelude_snapshot.bin` size | **~1.99 MB** |
| `prelude_vm_image.rkyv` size | **~116 KB** (fn blobs + export side table) |
| `hello` via `syx run --daemon` | ~0.01–0.02 s client wall (reuse is in daemon process) |

## Release / CI guarantee

| Gate | What it does |
|------|----------------|
| `ci.yml` Linux | `check_prelude_embed.sh` after release build |
| `ci.yml` `windows-smoke` | `check_prelude_embed.ps1` after release build |
| `release.yml` each matrix OS | **Regen** (`gen_prelude_snapshot.sh` / `.ps1`) then full-native build, then **verify** embed hit on the shipped binary |

Embed is keyed by `cache_version_tag` (`CARGO_PKG_VERSION` + `rustc -V` + tip digest). Checked-in assets alone do **not** guarantee every platform: a runner whose rustc differs from the gen machine will miss. Release regen bakes a matching snapshot into that platform’s artifact via `include_bytes!`.

**Windows gen footgun (fixed):** `gen_prelude_snapshot` truncates assets to force cold store. If cold-store discovery failed (notably `Get-ChildItem -Filter` + `-Recurse` on Windows PowerShell 5.1 missing `prelude_*.bin`), the script exited while leaving a **0-byte** `prelude_snapshot.bin`; a later `check_prelude_embed` then reported “missing/empty”. Gen now backs up before truncate and restores on failure; PS find uses `Where-Object` instead of `-Filter`.

## Non-claims

- ≠ every script ≤20 ms (import-heavy / compute-bound still slower)
- ≠ true zero-copy Instruction (still bincode-decode blobs inside rkyv)
- ≠ Windows daemon (Unix sock only; Windows still gets embed/rkyv via `.ps1` regen)
- ≠ native/wasm path changes (`stdlib_mir` stays on for `syx build`)
- ≠ one committed `.bin` forever valid on all rustc versions (release regen is the ship guarantee)

## Next knives

1. True Archive on hot Instruction subset / shared pages across clients.
2. Further shrink deserialize tax when rkyv embed is empty (still falls back to ~2 MB bincode).
