# ADR: Stage-2 S2-04 CST→AST lower probe expand (simple-stmt) (S2-I162)

**Status:** Accepted (S2-I162) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I161 lint→lower](stage2-s2-04-lint-lower-path.md), product lint consumes `lower_cst_subset_probe` on the I160 print-only face; most CST-Ok `func main` bodies still hit `LWR001` and fall back to `Parser::parse`. Census / I161 Consequences name **expand probe face** as the next default. This window widens the probe to the early W791–W797 simple-stmt band — it does **not** add while/typed/control-flow, switch Frontend, freeze dual grammar, or claim S2-04 `[x]`.

## Decision

1. **Expand** `lower_cst_subset_probe` body surface (still zero-arg single `func main()` only):
   - keep I160 `print(<int|ident|string|call-simple>)` [`;`]
   - add untyped `let <ident> = <simple>` [`;`] → `Stmt::VarDecl { mutable: false, ty: None, … }`
   - add untyped `var <ident> = <simple>` [`;`] → `Stmt::VarDecl { mutable: true, … }`
   - add `return <simple>` [`;`] / bare `return` / `return;` → `Stmt::Return`
2. **`<simple>` for bind/return RHS:** Integer / Ident / String / Double / Bool / Nil only (**≠** call-simple on RHS; print retains call-simple).
3. **Out of face → `LWR001`:** while / if / typed bind (`: T`) / multi-func / non-`main` / params — unchanged fail-closed.
4. **Parity:** span-zeroed `Module` ≡ `Parser::parse`; `lint_source_auto` issues ≡ `lint_source` on expanded fixtures (Parser-free path).
5. **Non-claims (hard):** ≠ while/if/typed/multi-func / ≠ expand `validate_func_main_subset` / ≠ change `FrontendPipeline` / ≠ freeze dual grammar / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ deepen / ≠ rowan / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Expand knife | `take_let_stmt` / `take_var_stmt` / `take_return_stmt` in `lower_probe.rs` |
| Contract | `s2_04_cst_lower_probe_contract` + `s2_04_lint_lower_path_contract` |
| Lint path | I161 wiring unchanged; wider lower Ok coverage |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Parser-free lint coverage grows on W791–797-shaped sources.
- Default next = further expand **or** freeze dual grammar → dedicated S2-04 `[x]` claim — **not** Stage-2 exit.
- **S2-I163 landed:** [dual-grammar-freeze-charter](stage2-s2-04-dual-grammar-freeze-charter.md) — KPI freeze + stop default `take_*` (**≠** S2-04 `[x]`).
- Prior: [lint-lower-path](stage2-s2-04-lint-lower-path.md) · S2-I161; [cst-lower-probe](stage2-s2-04-cst-lower-probe.md) · S2-I160.

## References

- [stage2-s2-04-lint-lower-path](stage2-s2-04-lint-lower-path.md) · S2-I161
- [stage2-s2-04-cst-lower-probe](stage2-s2-04-cst-lower-probe.md) · S2-I160
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
