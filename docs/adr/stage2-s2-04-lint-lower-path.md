# ADR: Stage-2 S2-04 lint CST-success → lower path (S2-I161)

**Status:** Accepted (S2-I161) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I160 CST→AST lower probe](stage2-s2-04-cst-lower-probe.md), `lower_cst_subset_probe` produces a span-parity `Module` on the thin main+print face, but product lint still always secondary-`Parser::parse` after the CST probe ([I153](stage2-s2-04-lint-cst-first.md)). [I159](stage2-s2-04-frontend-share-tree-evaluation.md) allows Module-on-AST via **lower or Parser**. This window wires `lint_source_auto` to consume the probe lower on CST success — it does **not** expand the probe face, switch `FrontendPipeline`, or claim S2-04 `[x]`.

## Decision

1. **`lint_source_auto` (updated):**
   - `SYX_FMT_AST=1` / `true` → `lint_source` (unchanged hatch).
   - else `parse_cst_subset`:
     - **Ok(`root`)** → `lower_cst_subset_probe(&root)`:
       - **Ok(`module`)** → `lint_module(source, &module)` (**no** `Parser::parse` on this path).
       - **Err** (including `LWR001`) → `lint_source` (Parser fallback; preserves I153 behavior for out-of-probe CST-Ok sources).
     - **`CST001`** → `lint_source` (unchanged).
     - other CST errors → propagate (fail-closed).
2. **Retain** `lint_source` as AST-only for tests / hatch / rollback.
3. **Product entrypoints** (`lint_file` / `lint_document_for_lsp`) stay on `lint_source_auto` only.
4. **Honesty:** L001–L009 still run on `syx_ast::Module` — origin is now **lower (probe face) or Parser (fallback)**. **≠** lint-on-CST node semantics.
5. **Non-claims (hard):** ≠ expand `lower_probe` face / ≠ change `FrontendPipeline` / ≠ lint-on-CST / ≠ delete `syx-cst` / ≠ freeze dual grammar / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ deepen / ≠ rowan / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Lower path | `lint_source_auto` → `lower_cst_subset_probe` → `lint_module` |
| Fallback | `LWR001` / lower Err → `lint_source` |
| Contract | `s2_04_lint_lower_path_contract` |
| Frontend untouched | `syx-frontend` still no `syx-cst` dep |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- First **product-adjacent** consumer of CST→AST lower (lint tool path).
- Default next = expand probe face (widen Parser-free lint coverage) → freeze dual grammar → dedicated S2-04 `[x]` claim — **not** Stage-2 exit.
- **S2-I162 landed:** [cst-lower-probe-expand](stage2-s2-04-cst-lower-probe-expand.md) — W791–797 let/var/return (**≠** while/typed / ≠ S2-04 `[x]`).
- Prior: [cst-lower-probe](stage2-s2-04-cst-lower-probe.md) · S2-I160; [lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153.

## References

- [stage2-s2-04-cst-lower-probe](stage2-s2-04-cst-lower-probe.md) · S2-I160
- [stage2-s2-04-frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
