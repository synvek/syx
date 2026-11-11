# R-32 Int(String)→Int? census

**Date:** 2026-11-11  
**Scope:** `Int(String)` failable parse → platform Int or Nil (`Int?`); cancel `Int {}` user face on this path.  
**Script:** [`scripts/r32_int_from_string_census.sh`](../../scripts/r32_int_from_string_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-31 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-32** | SEM-INTSTR cleared |
| Object / simd | **186/11** / **159** — unchanged |

## Root cause

`Int("42")` called zero-arg `Int_init` (empty class), left the String on the stack, returned `Int {}`; `+ 1` via `as_i64` default **0** → **1**; type `Named("Int")` rejected by `Int?` (S004).

## Delivery

1. MIR: [`mir_lower_expr_call_ident.rs`](../../crates/syx-mir/src/mir_lower_expr_call_ident.rs) — builtin `Int` + single String arg → `CallHost(IntFromString)` (no `Int_init`).
2. Host: [`HostFnId::IntFromString`](../../crates/syx-stdlib/src/registry.rs) + [`host_int_from_string`](../../crates/syx-vm/src/hosts.rs) — trim decimal parse → platform Int / Nil.
3. Semantic: [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs) — same shape → `Optional(Numeric(Int))`.
4. Arith: VM + mir-run numeric binop — `Nil` operand → **R003** (fail-closed).
5. Fixtures + [`int_from_string.rs`](../../crates/syx-driver/tests/lang/int_from_string.rs).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `int_from_string::*` | 2/2 | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ `Double(String)` / other numeric from-string GA
- ≠ `init?` syntax productization
- ≠ Optional some/none bag encoding (still bare Int \| Nil)
- ≠ changing zero-arg `Int()` empty instance path
- ≠ Iterator / Sequence (R-33)
