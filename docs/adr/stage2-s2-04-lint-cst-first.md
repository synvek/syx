# ADR: Stage-2 S2-04 lint CST-first policy (S2-I153)

**Status:** Accepted (S2-I153) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I152 LSP DocumentFormatting](stage2-s2-04-lsp-document-formatting.md), fmt **write / `--check` / LSP format** share `format_dry_run` → `format_source_auto` (CST-first). Census remaining honesty debt includes **lint still pure AST** (`lint_source` → `Parser::parse` only). This window aligns lint's **parse policy** with fmt — it does **not** run L001–L009 on CST nodes or claim a shared lossless tree.

## Decision

1. **`lint_source_auto`:** mirror `format_source_auto` shape:
   - `SYX_FMT_AST=1` / `true` → `lint_source` (AST-only hatch, shared with fmt).
   - else `parse_cst_subset` probe; on **Ok** or **`CST001`** → `lint_source`; other errors propagate (fail-closed).
2. **Product path:** [`lint_file`](../../crates/syx-driver/src/lib.rs) / CLI `syx lint` use `lint_source_auto`.
3. **Buffer API:** `lint_document_for_lsp(name, contents)` mirrors `format_document_for_lsp` (auto policy); LSP does **not** publish lint diagnostics this window.
4. **Retain** `lint_source` as AST-only for tests / explicit rollback.
5. **Honesty:** L001–L009 still require `syx_ast::Module` via `Parser::parse` after the CST probe. This window = **parse-strategy SSOT with fmt**, **≠** 「lint 与 fmt 共一棵 lossless 树」.
6. **Non-claims (hard):** ≠ run L001–L009 on CST / ≠ LSP lint diagnostics / ≠ delete `syx-cst` / ≠ change `FrontendPipeline` / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Auto policy | `lint_source_auto` → `parse_cst_subset` + `lint_source` |
| CLI path | `lint_file` → `lint_source_auto` |
| Buffer API | `lint_document_for_lsp` |
| Contract | `s2_04_lint_cst_first_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Tool-surface **CST-first / CST001→AST / SYX_FMT_AST** policy now covers fmt + lint.
- **S2-I154 landed:** [cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) — `take_if_let_simple` deepen (**≠** if case / ≠ S2-04 `[x]`).
- Next default = more CST deepen or Frontend share tree → retire dual `syx-cst` → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) · S2-I152.

## References

- [stage2-s2-04-lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) · S2-I152
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
