# ADR: Stage-2 S2-05 — semantic_analyzer 核心面批抽（S2-I250–I252）

**Status:** Accepted (S2-I250–I252) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I249 semantic helpers extract](stage2-s2-05-semantic-helpers-extract.md), [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) had grown to **7287** LOC — the relocated giant face. The prior lock “I250 = S2-05 claim” is **superseded**: claim is deferred until analyzer secondary peels reduce the face. This window batches the three largest faces out of the analyzer hub.

## Decision

1. Peel three core faces from `semantic_analyzer.rs` into new sidecars (zero semantics; additional `impl SemanticAnalyzer` blocks):
   - **S2-I250** — `register_nested_type_path` + `collect_nested_type_decl` + `collect_decls` → [`semantic_collect_decls.rs`](../../crates/syx-semantic/src/semantic_collect_decls.rs)
   - **S2-I251** — `analyze_expr`…`unify_element_types` + `analyze_expr_inner` → [`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)
   - **S2-I252** — `analyze_block` + `analyze_stmt` → [`semantic_analyze_stmt.rs`](../../crates/syx-semantic/src/semantic_analyze_stmt.rs)
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): `mod semantic_collect_decls;` / `mod semantic_analyze_expr;` / `mod semantic_analyze_stmt;`. Struct + entry/residual faces stay in `semantic_analyzer.rs`.
3. Census: sidecar **8 → 11**. Introduce **analyzer honesty floor** **4500** (landed ≈4876; plan mid-target 5500 overshot). `lib.rs` floor stays **300**.
4. **Ladder next:** **S2-I253+** = `analyze_call` / `analyze_decl` / `analyze_type_member` batch peel — still ≠ S2-05 `[x]` / ≠ Stage-2 exit.
5. **Non-claims (hard):** ≠ S2-05 `[x]` this window / ≠ Stage-2 exit / ≠ migrate `#[cfg(test)]` / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars hold `collect_decls` / `analyze_expr_inner` / `analyze_stmt` |
| Contract | `s2_05_semantic_analyzer_core_faces_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I250–I252 · analyzer floor 4500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Decision §4 locked next = **S2-I253+ call/decl/type_member**; **landed** as [decl-call-member-batch](stage2-s2-05-semantic-analyzer-decl-call-member-batch.md)（I253–I255）；next = **S2-I256+ concurrency/IDE**（≠ claim）.
- Residual after this batch window moved to I253–I255 sidecars; hub kept entry/infer/concurrency/IDE.
- Prior: [semantic-helpers-extract](stage2-s2-05-semantic-helpers-extract.md) · S2-I249（claim lock superseded by I250–I252）.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-helpers-extract](stage2-s2-05-semantic-helpers-extract.md) · S2-I249
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
