# ADR: S1-08 scoped = CLI check multi-diag (≠ build/run / ≠ stage-1 exit)

**Status:** Accepted (S1-I24)  
**Date:** 2026-11-01  
**Context:** S1-08 (error recovery / multi-diagnostics) started with LSP `parse_collecting`, then single-file `syx check` parse+semantic thin faces (S1-I21/I22). Project `syx check <dir>` still used first-error `compile_project`, so the CLI check surface was incomplete relative to the charter.

## Decision

1. **S1-08 acceptance is scoped.** Completing S1-08 means:
   - LSP continues to report capped multi-diagnostics via collecting parse/semantic;
   - Single-file `syx check` uses `check_file_with_options` (parse collecting + linked compile; semantic collecting enrichment on failure);
   - Project `syx check` uses `check_project_with_options`: walk package `.sy` with `parse_collecting`, aggregate capped diagnostics, then linked compile if parse-clean;
   - Nail fixtures document ≥2 P001 (file + project) and ≥2 semantic diags (single file).
2. **Out / defer (not required for S1-08 `[x]`):**
   - `syx build` / `syx run` / default compile multi-diag;
   - Project-level semantic collecting across all files (false-positive risk without import/prelude parity);
   - Claiming Stage-1 exit.
3. **Stage-1 exit honesty.** Multi-diag improves DX credibility but does not clear examples 拉满/三后端, S1-07 trap, mono, or diamond debts. **S1-08 scoped `[x]` alone does not authorize Stage-1 exit.**

## Consequences

- `plan/02` marks **S1-08 `[x]` (scoped)**; narrative must keep “≠ build·run multi-diag / ≠ 阶段1退出”.
- Expanding multi-diag to build/run requires a superseding ADR.

## References

- [`docs/multi-diagnostics.md`](../multi-diagnostics.md)
- [`fixtures/multi_project_syntax/`](../../fixtures/multi_project_syntax/)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · S1-08
