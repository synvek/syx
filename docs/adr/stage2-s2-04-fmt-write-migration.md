# ADR: Stage-2 S2-04 fmt write migration (S2-I151)

**Status:** Accepted (S2-I151) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I150 CST reindent GA](stage2-s2-04-cst-reindent-ga.md), the convergence hard prerequisite for migrating fmt **write** is met. Today [`format_file`](../../crates/syx-driver/src/lib.rs) still calls AST `format_source` while `--check` uses `format_dry_run` → `format_source_auto` (CST reindent GA). This is the census primary honesty debt (write drops comments / diverges from `--check`). This window migrates write onto the same SSOT — it does **not** claim S2-04 `[x]`.

## Decision

1. **`format_file`:** after reading the `.sy` file, call **`syx_fmt::format_dry_run`** and write `report.formatted` to disk (same fail-closed / F002 idempotency as `--check`).
2. **Product SSOT:** write success path = `format_source_auto` (via dry-run); `SYX_FMT_AST=1` remains the AST rollback hatch.
3. **Remove** the AST-only «dropped comments» stderr warning (write/`--check` aligned under CST success).
4. **Unchanged:** `format_cst` / `format_dry_run_cst` stay lexeme concat; `format_check_path` still `format_dry_run`.
5. **Non-claims (hard):** ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ LSP DocumentFormatting / ≠ lint tree share / ≠ delete `syx-cst` / ≠ change `FrontendPipeline` / ≠ change `format_cst`/`format_dry_run_cst` concat / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Write = dry-run/auto | `format_file` → `format_dry_run` |
| Check unchanged | `format_check_path` → `format_dry_run` |
| Contract | `s2_04_fmt_write_check_strategy` |
| Comment preserve (CST) | platform write/check align nail |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Primary fmt write/`--check` honesty debt **closed** for the tool surface.
- **S2-I152 landed:** [lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) — LSP format shares dry_run/auto SSOT.
- Prior: [cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) · S2-I150.

## References

- [stage2-s2-04-cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) · S2-I150
- [stage2-s2-04-lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) · S2-I152
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
