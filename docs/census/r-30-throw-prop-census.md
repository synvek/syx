# R-30 错误跨函数传播 census

**Date:** 2026-11-11  
**Scope:** `do { try g() } catch` must enter `catch` (VM bytecode + MIR interpreter).  
**Script:** [`scripts/r30_throw_prop_census.sh`](../../scripts/r30_throw_prop_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-29 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-30** | `try` rethrow pad kept + VM/MIR ReturnErr same-frame catch |
| SEM-THROW | **cleared** (enum Error + string throw fixtures) |

## Root cause

1. MIR `UnaryOp::Try` emitted `ReturnErr` on a separate `err_block` with **no CFG successor** from the try body → `remove_unreachable_blocks` dropped the pad; `BeginTry` kept a stale block id that collided with `EndTry` PC → throw continued as success (`print("value")`).
2. MIR interpreter `ReturnErr` always `return Ok(Nil)` without consulting same-frame outer `do`.
3. AST VM codegen (non-default) used `Catch` + `ConstStr("")` (swallow). Default path is MIR→bytecode.

## Delivery

1. [`mir_lower_expr_unary.rs`](../../crates/syx-mir/src/mir_lower_expr_unary.rs): `link(block, err_block)` exception edge.
2. [`mir_opt.rs`](../../crates/syx-mir/src/mir_opt.rs): `BeginTry` / `BindResult` targets reachable; unit `r30_try_keeps_return_err_after_optimize`.
3. [`syx-mir-run` `ReturnErr`](../../crates/syx-mir-run/src/runtime.rs): same-frame `take_catch` then jump; else propagate.
4. [`syx-codegen` `UnaryOp::Try`](../../crates/syx-codegen/src/lib.rs): pad = `ReturnErr` (AST path parity).
5. Fixtures + tests: `try_cross_fn.sy` / `try_cross_fn_string.sy` in [`errors.rs`](../../crates/syx-driver/tests/lang/errors.rs).
6. `typed_catch.sy` → `enum AppError: Error` (SA-2-02; cache purge exposed bare `data class` reject).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `errors::try_cross_fn_*` | 4/4 | pass |
| `errors::typed_catch_struct_error` | pass | pass |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ `try?` / `try!` semantic split (same AST `UnaryOp::Try` → all rethrow)
- ≠ native / wasm AOT throw IR legality
- ≠ Error display `E.bad` (R-36)
- ≠ value semantics / Int(String) / Iterator (R-31…R-33)

## Note

Incremental `target/cache` fingerprints source only relative to bake tag — after MIR compiler fixes, purge cache or touch fixtures before trusting `compile_file` hits.
