# R-28 Browser red census

**Date:** 2026-11-11  
**Scope:** Browser harness/path closeout (22→0).  
**Script:** [`scripts/r28_browser_red_census.sh`](../../scripts/r28_browser_red_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-27 | exited (wasm **46/46**; Object **186/11**) |
| **R-28** | Browser **46 pass / 0 fail / 2 ignored** |
| Object / simd | unchanged (parked) |

## Delivery

1. **Single prelude** in [`browser_compile_run.rs`](../../crates/syx-driver/tests/browser/browser_compile_run.rs) — no `*_with_prelude_*` after manual inject (clears `Point_compare` W001 ×6).
2. **AOT exit=0 harness** — [`browser_compile.rs`](../../crates/syx-driver/tests/browser/browser_compile.rs) / e2e return tests use `print` + `run_wasm_capture` (W18 contract retained).
3. **Path pins** — `../../crates/syx-compiler-wasm` / `../../.github/workflows/ci.yml`.
4. Credibility [`r28_browser_closeout_contract.rs`](../../crates/syx-driver/tests/credibility/r28_browser_closeout_contract.rs).

## Gates (honest)

| KPI | Target |
|-----|--------|
| `browser_failed` | **0** |
| Playwright `#[ignore]` | ≤2 (pre-existing) |
| wasm | **46/46** regression |
| Object | **≤186 / ≤11** |

**Scoped honesty:** ≠ change AOT `main` wasm exit product contract; ≠ simd / OrderedSet wipe; ≠ lang inheritance / `super.` green.

## Still red (out of window)

- simd / OrderedSet / json Object bags
- lang poly_base_ref / JIT / Int8 (see R-29)

**Next:** [`r-29-lang-super-census.md`](r-29-lang-super-census.md) closed honesty + `super.method` MIR.
