# ADR: Stage-2 S2-04 CST→AST lower probe (S2-I160)

**Status:** Accepted (S2-I160) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** [S2-I159 frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) locked the second convergence axis as **CST→AST lower** and named the next construction window as a **thin lower probe** (≠ compile GA). Tools already share a CST-first face (I144–I153); compile still enters via `Parser::parse` only. This window lands a **probe-only** `Module` lower for a minimal print surface so parity can be tested — it does **not** switch `FrontendPipeline`, expand CST grammar, or claim S2-04 `[x]`.

## Decision

1. **Add** `syx_cst::lower_cst_subset_probe(&CstRoot) -> SyxResult<Module>` plus sugar `lower_cst_subset_probe_from_source(&SourceFile)` (= `parse_cst_subset` then probe lower).
2. **Probe surface (hard):** zero-arg `func main() { 0..N print(<int|ident|string|call-simple>) }` only — aligned with early W683/W790 + `take_print_simple_call` call-simple zero/simple-arg forms. Trivia ignored via significant tokens.
3. **Out of probe → `LWR001`:** even when `parse_cst_subset` returns Ok for a larger subset (let / while / class / multi-func / typed params / …), the probe lower **rejects** with code `LWR001`. Do **not** expand `validate_func_main_subset` in this window.
4. **Parity:** driver contract asserts span-zeroed `Module` equality vs `Parser::parse` on the probe fixture set.
5. **Dependency:** `syx-cst` gains `syx-ast` only (**≠** `syx-parser` / **≠** `syx-frontend`).
6. **Non-claims (hard):** ≠ change `FrontendPipeline` / ≠ delete `syx-cst` / ≠ full-subset lower / ≠ lint rewrite to lower / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ deepen (force-unwrap / switch / KeyPath) / ≠ rowan / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Probe API | `lower_cst_subset_probe` / `lower_cst_subset_probe_from_source` in `syx-cst` |
| Reject code | `LWR001` |
| Contract | `s2_04_cst_lower_probe_contract` |
| Frontend untouched | `syx-frontend` still no `syx-cst` dep |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Second axis moves from evaluation (I159) to a **testable lower knife**.
- Default next = expand probe face **or** optional lint `lower → lint_module` path → freeze dual grammar → dedicated S2-04 `[x]` claim — **not** Stage-2 exit; deepen only under I159 stop-line.
- **S2-I161 landed:** [lint-lower-path](stage2-s2-04-lint-lower-path.md) — `lint_source_auto` consumes probe lower (**≠** FrontendPipeline / ≠ expand probe / ≠ S2-04 `[x]`).
- Prior: [frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159.

## References

- [stage2-s2-04-frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
