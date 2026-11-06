# ADR: Stage-2 S2-04 LSP DocumentFormatting (S2-I152)

**Status:** Accepted (S2-I152) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I151 fmt write migration](stage2-s2-04-fmt-write-migration.md), CLI **write** and **`--check`** share `format_dry_run` → `format_source_auto`. VS Code enables `editor.formatOnSave` and README claims formatting, but [`syx-lsp`](../../crates/syx-lsp/src/lib.rs) does not advertise `documentFormattingProvider`. This window closes that honesty debt for the editor surface — it does **not** claim S2-04 `[x]` or lint tree share.

## Decision

1. **Capability:** LSP `initialize` includes `documentFormattingProvider: true`.
2. **Handler:** `textDocument/formatting` reads the in-memory document buffer and calls driver [`format_document_for_lsp`](../../crates/syx-driver/src/lib.rs) → `format_dry_run` (same SSOT as CLI write/`--check`).
3. **Success:** return a single full-document `TextEdit` (`newText` = formatted).
4. **Failure:** return JSON `null` (no partial rewrite).
5. **Non-claims (hard):** ≠ lint tree share / ≠ change `FrontendPipeline` / ≠ delete `syx-cst` / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 / ≠ change `format_cst`/`format_dry_run_cst` concat / ≠ `textDocument/rangeFormatting`.

## Evidence

| Check | Evidence |
|-------|----------|
| Capability | `initialize_capabilities` → `documentFormattingProvider` |
| SSOT | `format_document_for_lsp` → `format_dry_run` |
| Handler | `textDocument/formatting` |
| Contract | `s2_04_lsp_format_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Fmt **three surfaces** (CLI write / `--check` / LSP format) share auto SSOT.
- **S2-I153 landed:** [lint-cst-first](stage2-s2-04-lint-cst-first.md) — lint CST-first policy aligned with fmt.
- Prior: [fmt-write-migration](stage2-s2-04-fmt-write-migration.md) · S2-I151.

## References

- [stage2-s2-04-fmt-write-migration](stage2-s2-04-fmt-write-migration.md) · S2-I151
- [stage2-s2-04-lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
