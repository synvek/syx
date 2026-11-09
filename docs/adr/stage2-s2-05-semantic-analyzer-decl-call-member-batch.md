# ADR: Stage-2 S2-05 — semantic_analyzer decl/call/member 批抽（S2-I253–I255）

**Status:** Accepted (S2-I253–I255) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I250–I252 analyzer core-faces batch](stage2-s2-05-semantic-analyzer-core-faces-batch.md), the locked next knife is **call/decl/type_member**. This window peels those three faces from [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs). It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Peel three faces (zero semantics; additional `impl SemanticAnalyzer` blocks):
   - **S2-I253** — `analyze_decls` + `analyze_decl` + `resolve_type_expr`…`validate_set_elem_hashable` → [`semantic_analyze_decl.rs`](../../crates/syx-semantic/src/semantic_analyze_decl.rs)
   - **S2-I254** — call-pre (`check_import_member_access`…`check_call_argument_labels`) + `analyze_call` + `infer_pattern_type`…`is_compatible` → [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)
   - **S2-I255** — `analyze_type_member` + satellites through `is_associated_type_constraint` → [`semantic_analyze_type_member.rs`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs)
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): three new `mod`s. Raise cross-module callees (e.g. `infer_simd_element_from_args`) to `pub(crate)` as needed.
3. Census: sidecar **11 → 14**. Analyzer honesty floor **4500 → 2000**. `lib.rs` floor stays **300**.
4. **Ladder next:** **S2-I256+** = concurrency/IDE peel — still ≠ S2-05 `[x]` / ≠ Stage-2 exit.
5. **Non-claims (hard):** ≠ S2-05 `[x]` this window / ≠ Stage-2 exit / ≠ migrate `#[cfg(test)]` / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars hold `analyze_decl` / `analyze_call` / `analyze_type_member` |
| Contract | `s2_05_semantic_analyzer_decl_call_member_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I253–I255 · analyzer floor 2000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Decision §4 locked next = **S2-I256+ concurrency/IDE**; **landed** as [concurrency-ide-infer-batch](stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md)（I256–I258）；next = **S2-I259 S2-05 claim**.
- Residual after I256–I258: entry/builtins hub only.
- Prior: [semantic-analyzer-core-faces-batch](stage2-s2-05-semantic-analyzer-core-faces-batch.md) · S2-I250–I252.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyzer-core-faces-batch](stage2-s2-05-semantic-analyzer-core-faces-batch.md) · S2-I250–I252
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
