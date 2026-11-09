# ADR: Stage-2 S2-05 — semantic_analyzer concurrency/IDE/infer 批抽（S2-I256–I258）

**Status:** Accepted (S2-I256–I258) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I253–I255 decl/call/member batch](stage2-s2-05-semantic-analyzer-decl-call-member-batch.md), the locked next knife is **concurrency/IDE**. This window also peels **infer/host** in the same batch. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Peel three faces from [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (zero semantics; additional `impl SemanticAnalyzer` blocks):
   - **S2-I256** — `check_concurrency_rules`…`collect_function_isolation` → [`semantic_concurrency.rs`](../../crates/syx-semantic/src/semantic_concurrency.rs)
   - **S2-I257** — `symbol_type`…`members_for_type` → [`semantic_ide.rs`](../../crates/syx-semantic/src/semantic_ide.rs)
   - **S2-I258** — `infer_enum_case_type_args`…`infer_expr_type_name` → [`semantic_infer_host.rs`](../../crates/syx-semantic/src/semantic_infer_host.rs)
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): three new `mod`s. Keep struct + entry + builtins/support in the analyzer hub.
3. Census: sidecar **14 → 17**. Analyzer honesty floor **2000 → 800**. `lib.rs` floor stays **300**.
4. **Ladder next:** **S2-I259** = dedicated **S2-05 `[x]` claim** (scoped: production faces peeled; hub = entry/builtins; tests-in-lib OK) — still ≠ Stage-2 exit.
5. **Non-claims (hard):** ≠ S2-05 `[x]` this window / ≠ Stage-2 exit / ≠ migrate `#[cfg(test)]` / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars hold `check_concurrency_rules` / `members_for_type` / `infer_call_type_args` |
| Contract | `s2_05_semantic_analyzer_concurrency_ide_infer_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I256–I258 · analyzer floor 800 |

## Consequences

- S2-05 stayed `[~]` through this peel; Stage-2 stayed `[~]`.
- Decision §4 locked next = **S2-I259 S2-05 claim**; **landed** as [s2-05-claim](stage2-s2-05-claim.md) — S2-05 `[x]` scoped; next = **S2-06 open**.
- Residual analyzer hub ≈ **900–1000** (entry + builtins/support) retained under claim scope.
- Prior: [decl-call-member-batch](stage2-s2-05-semantic-analyzer-decl-call-member-batch.md) · S2-I253–I255.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyzer-decl-call-member-batch](stage2-s2-05-semantic-analyzer-decl-call-member-batch.md) · S2-I253–I255
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
