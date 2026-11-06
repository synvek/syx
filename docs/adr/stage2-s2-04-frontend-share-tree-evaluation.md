# ADR: Stage-2 S2-04 Frontend share-tree evaluation (S2-I159)

**Status:** Accepted (S2-I159) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I144 convergence strategy](stage2-s2-04-convergence-strategy.md) locked the **fmt tool surface** as the first convergence axis, I145–I153 completed that axis (reindent GA → write/check/LSP → lint CST-first policy). I154–I158 then deepened CST subset control-flow and call-arg faces. The remaining honesty debt is **structural**: dual grammar (~`syx-parser` + `syx-cst`), compile still pure AST via [`FrontendPipeline`](../../crates/syx-frontend/src/lib.rs), lint rules still require AST Module. Continuing endless `take_*` deepen has diminishing ROI toward S2-04 `[x]`. This window locks the **second convergence axis** only — parallel to I144 — it does **not** change `FrontendPipeline`, delete `syx-cst`, or claim S2-04 `[x]`.

## Decision

1. **Second convergence axis** = compile / Frontend and tools **share one parse product**. Preferred path = **CST→AST lower**: lossless CST is the authoritative parse face; a lower produces `syx_ast::Module` (or equivalent) for semantic/MIR. **≠** AST+trivia rewrite as the primary path; **≠** rowan.
2. **Window order (locked):** I159 evaluation → **CST→AST lower probe** (thin slice; ≠ full compile GA) → deepen **only** when it blocks Production Scope CST-success or lower → **freeze** `syx-cst` grammar expansion → **stop dual-maintaining** stmt/expr grammar → dedicated **S2-04 `[x]` claim**.
3. **Deepen stop-line:** default = no infinite `take_*` series. force-unwrap / KeyPath / switch (and similar) open only when they block lower or fmt CST-success path. Switch MVP surface **deferred** until after lower probe.
4. **Lint:** within S2-04, keep **Module-on-AST** (via lower or today's `Parser::parse`). **≠** lint-on-CST node semantics. 「共树」= same parse/lower origin, not rewriting L001–L009 on CST green tokens.
5. **LSP:** format already shares dry_run/auto SSOT (I152). Publishing lint diagnostics is **deferred** (≠ this window; ≠ hard gate for S2-04 `[x]`).
6. **`syx-cst` fate:** after dual grammar maintenance stops, keep it as lossless / fmt / reindent face, or fold into parser behind a facade. **Deleting the crate is not a hard gate.** Hard gate = **stop dual-maintaining two stmt/expr grammars**.
7. **S2-04 `[x]` acceptance checklist** (for the future claim ADR):
   - Compile and tools share a documented, testable parse strategy (CST-first + lower, or equivalent SSOT).
   - Census dual-grammar KPI stops expanding / trends down for maintenance surface.
   - Lint honesty: Module source remains explicit (lower or Parser).
   - **≠** Stage-2 exit (still requires S2-05…07).
8. **Non-claims (hard):** ≠ change `FrontendPipeline` implementation this window / ≠ delete `syx-cst` / ≠ implement lower / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ rowan / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 / ≠ force-unwrap or switch construction this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Second axis locked | this ADR Decision 1–7 |
| Fmt first axis complete | I144–I153 + census |
| Contract | `s2_04_frontend_share_tree_evaluation_contract` |
| Next window named | CST→AST lower probe (≠ GA) |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Default next construction window = **CST→AST lower probe** — not another opportunistic deepen knife.
- **S2-I160 landed:** [cst-lower-probe](stage2-s2-04-cst-lower-probe.md) — thin `lower_cst_subset_probe` + Parser Module parity (**≠** FrontendPipeline / ≠ S2-04 `[x]`).
- Deepen remains allowed only under Decision 3 stop-line.
- Prior: [cst-subset-optional-chain](stage2-s2-04-cst-subset-optional-chain.md) · S2-I158; [convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144.

## References

- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-s2-04-cst-subset-optional-chain](stage2-s2-04-cst-subset-optional-chain.md) · S2-I158
- [stage2-s2-04-open](stage2-s2-04-open.md) · S2-I143
- [stage2-parser-census](../stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
