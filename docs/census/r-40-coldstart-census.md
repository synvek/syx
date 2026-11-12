# R-40 cold-start / cold-exec path honesty census

**Date:** 2026-11-12  
**Scope:** todo-005 E1–E2 / §4.5 — clean-dir hello / `fib(30)` remeasure; demote Stage-3 `<0.15` wall claims; embed fingerprint probe (regen only on miss).  
**Script:** [`scripts/r40_coldstart_census.sh`](../../scripts/r40_coldstart_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-39 | exited (wasm **46/46**; browser failed=**0**; Object **≤186/11**) |
| **R-40** | **exited** — honesty remeasure + Stage-3 claim demotion; embed tag **HIT** (no regen) |
| Object / simd | unchanged **186/11** |

## Clean-dir wall clock (release `syx`, macOS)

Protocol: warm `syx` pages once, then fresh temp dir (empty `target/`) for cold; same path for warm. Aligns with R-38 ~2.6 s narrative — **not** Stage-3 historical ≈0.08.

| Sample | Wall (s) | Notes |
|--------|----------|-------|
| `hello_cold_s` | **2.682** | clean temp; embed tag match |
| `hello_warm_s` | **0.010** | same path after cold |
| `fib30_cold_s` | **2.897** | clean temp `examples/fib.sy` (`fib(30)`) |
| `fib30_warm_s` | **0.296** | same path (compile cache + VM) |
| `embed_tag_match` | **1** | asset tag == baked `SYX_CACHE_VERSION_TAG` in binary |
| `prelude_embed_asset_bytes` | **6595124** | `crates/syx-driver/assets/prelude_snapshot.bin` |

Exact numbers are re-emitted by the census script on each machine. Product gate **cold &lt;0.3 s was not used** this window.

## Embed probe

- Fingerprint **match** → treat as embed **HIT** path available on empty-cache cold.
- Observed: HIT still leaves hello/fib cold in the **~2.5–3.1 s** band (same order as [`r-38-book-run-census`](r-38-book-run-census.md)).
- Honesty: **embed in place ≠ E1 `<0.3 s` GA**. No `gen_prelude_snapshot.sh` regen this window (miss-only).
- Product acceleration remains a later window; R-40 exits on claim alignment, not wall KPI.

## Stage-3 claim demotion

| Source | Action |
|--------|--------|
| [`stage3-stdlib-cache-census.md`](stage3-stdlib-cache-census.md) | Mark `hello_cold_s≈0.08` / `<0.15` as **superseded by R-40** clean-dir measure |
| [`stage3-s3-03-claim.md`](../adr/stage3-s3-03-claim.md) | Historical scoped claim retained; wall-clock **not** current product KPI |

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| hello/fib cold+warm recorded | yes | **yes** (script emits keys) |
| Stage-3 `<0.15` demoted | yes | **yes** (superseded markers) |
| embed miss → regen | if miss | **N/A** (tag match / HIT) |
| cold `<0.3` | **not a gate** | not claimed |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ cold-start / E1 `<0.3 s` product GA
- ≠ `fib(30)<0.5` product KPI
- ≠ full-book default `run` / book-pass hard gate
- ≠ C4 throw IR / hot_loop native
- ≠ Object/simd / LANG-POLY
- ≠ Linux full-native bitcode / LLVM rebuild (build debt; ≠ cold `syx run`)

## References

- [`r-38-book-run-census`](r-38-book-run-census.md) · cold ≈2.6–3.1 s
- [`scripts/gen_prelude_snapshot.sh`](../../scripts/gen_prelude_snapshot.sh) · miss-only regen
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-40 → next **R-41**
