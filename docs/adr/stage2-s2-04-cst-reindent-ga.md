# ADR: Stage-2 S2-04 CST reindent GA (S2-I150)

**Status:** Accepted (S2-I150) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I149 env experiment gate](stage2-s2-04-cst-reindent-env-gate.md), CST reindent capability (FunctionDecl + class stored + nested + 2→4) is proven behind `SYX_FMT_CST_REINDENT`. [Convergence strategy](stage2-s2-04-convergence-strategy.md) forbids migrating fmt **write** until CST can reindent. This window declares **reindent GA** for the CST product path only — it does **not** migrate `format_file`.

## Decision

1. **`format_source_cst`:** after `parse_cst_subset`, **always** call `format_cst_reindent` (default Product Scope for `format_source_auto` / `--check` CST success path).
2. **`format_cst`:** remains lexeme concatenation (API preserved for pure round-trip / `format_dry_run_cst`).
3. **`format_dry_run_cst`:** remains concat-only (does not call `format_source_cst`) for parallel pure-CST nails.
4. **`SYX_FMT_CST_REINDENT`:** remove reads; superseded by GA default (rollback = `SYX_FMT_AST=1` or revert).
5. **`format_file`:** still AST `format_source` — write migration is a **later** window.
6. **Non-claims (hard):** ≠ write migration / ≠ claim write/`--check` already aligned / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| GA default | `format_source_cst` → `format_cst_reindent` |
| Concat preserved | `format_cst` + `format_dry_run_cst` |
| Write still AST | `s2_04_fmt_write_check_strategy` / `format_file` |
| Contract | `s2_04_cst_reindent_func_body` (+ GA ADR nail) |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- Convergence **reindent prerequisite** for write migration is **met**.
- **S2-I151 landed:** [fmt-write-migration](stage2-s2-04-fmt-write-migration.md) — `format_file` → `format_dry_run` / auto.
- Prior: [cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md).

## References

- [stage2-s2-04-cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) · S2-I149
- [stage2-s2-04-fmt-write-migration](stage2-s2-04-fmt-write-migration.md) · S2-I151
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
