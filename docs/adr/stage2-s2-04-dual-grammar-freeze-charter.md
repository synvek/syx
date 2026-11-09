# ADR: Stage-2 S2-04 dual-grammar freeze charter (S2-I163)

**Status:** Accepted (S2-I163) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I162 lower probe expand](stage2-s2-04-cst-lower-probe-expand.md), tools consume CST-first + `lower_cst_subset_probe` on the W791–797 simple-stmt band ([I161](stage2-s2-04-lint-lower-path.md)), while compile still enters via `Parser::parse` / [`FrontendPipeline`](../../crates/syx-frontend/src/lib.rs). [I159](stage2-s2-04-frontend-share-tree-evaluation.md) window order names **freeze** `syx-cst` grammar expansion before a dedicated S2-04 `[x]` claim. Endless `take_*` deepen now conflicts with checklist item「Census dual-grammar KPI stops expanding」. This window locks the **freeze charter** only — it does **not** claim S2-04 `[x]`, change Frontend, or delete `syx-cst`.

## Decision

1. **Default stop expanding** `syx-cst` stmt/expr `take_*` surfaces (including new while / typed-bind / control-flow knives). **Exception gate:** open only when a change **blocks Production Scope fmt CST-success** or a **documented lower gate**, and only via a dedicated ADR (I159 deepen stop-line still applies).
2. **Parse-strategy SSOT (interim until Frontend migrate):**
   - **Tools:** CST-first + `lower_cst_subset_probe` / `lint_source_auto` / `format_source_auto` / `format_dry_run` / `format_document_for_lsp` (probe face + `LWR001`/`CST001` → Parser fallback where already wired).
   - **Compile / check / MIR:** `Parser::parse` → `FrontendPipeline` **until** a future Frontend-migrate window.
3. **KPI freeze baseline** (2026-11-06, `scripts/s2_04_parser_census.sh`):
   - `syx_parser_lib_loc` = **5665**
   - `syx_cst_lib_loc` = **6716**
   - `combined_grammar_loc` = **12381**
   - Subsequent windows must **not** treat raising combined grammar LOC as progress toward S2-04 `[x]`; trend target = hold or down for maintenance surface.
4. **`syx-cst` fate:** keep as lossless / fmt / reindent / lower-probe face. **Deleting the crate is not a hard gate.** Hard gate = **stop dual-maintaining** two stmt/expr grammars (I159 Decision 6).
5. **Next default** = dedicated **S2-04 `[x]` claim** (census ladder 12) — then Frontend migrate as needed. **≠** Stage-2 exit.
6. **Non-claims (hard):** ≠ S2-04 `[x]` this window / ≠ change `FrontendPipeline` / ≠ new `take_*` or lower-face expand this window / ≠ delete `syx-cst` / ≠ Stage-2 exit / ≠ rowan / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Freeze charter | this ADR Decision 1–5 |
| KPI baseline | Decision 3 + [stage2-parser-census](../census/stage2-parser-census.md) Freeze baseline |
| Contract | `s2_04_dual_grammar_freeze_contract` |
| Tools still CST+lower | `lint_source_auto` → `lower_cst_subset_probe` |
| Compile still Parser | `FrontendPipeline` / no `syx-cst` in `syx-frontend` |

## Consequences

- S2-04 stays `[~]` in this window; Stage-2 stays `[~]`.
- Default next construction window = **S2-04 `[x]` claim ADR** — not another opportunistic CST deepen.
- **S2-I164 landed:** [s2-04-claim](stage2-s2-04-claim.md) — plan **S2-04 `[x]`** scoped (**≠** Stage-2 exit / ≠ Frontend already on CST).
- Prior: [cst-lower-probe-expand](stage2-s2-04-cst-lower-probe-expand.md) · S2-I162; [frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159.

## References

- [stage2-s2-04-frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-cst-lower-probe-expand](stage2-s2-04-cst-lower-probe-expand.md) · S2-I162
- [stage2-s2-04-lint-lower-path](stage2-s2-04-lint-lower-path.md) · S2-I161
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
