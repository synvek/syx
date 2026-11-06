# ADR: Stage-2 S2-04 CST reindent — env experiment gate (S2-I149)

**Status:** Accepted (S2-I149) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I148 class stored-member track](stage2-s2-04-cst-reindent-class-stored.md), opt-in [`format_cst_reindent`](../../crates/syx-cst/src/lib.rs) covers FunctionDecl + class stored tracks, but [`format_source_cst`](../../crates/syx-fmt/src/lib.rs) still always concatenates via `format_cst`. This window wires an **experiment env gate** only — not reindent GA and not `format_file` migration.

## Decision

1. **Env:** `SYX_FMT_CST_REINDENT=1` or `true` (case-insensitive, same shape as `SYX_FMT_AST`) enables reindent in `format_source_cst`.
2. **`format_source_cst`:** after `parse_cst_subset`, call `format_cst_reindent` when the env is set; otherwise `format_cst`.
3. **`format_source_auto`:** no new branch; CST success path goes through `format_source_cst` (so env can affect `--check` when deliberately set). `SYX_FMT_AST=1` still forces AST.
4. **`format_dry_run_cst`:** decouple from `format_source_cst` — always `parse_cst_subset` + `format_cst` (×2 idempotency) so parallel pure-CST nails ignore the reindent env.
5. **`format_file` / default `format_cst`:** unchanged.
6. **Non-claims (hard):** ≠ reindent GA / ≠ change default `format_cst` / ≠ change `format_file` / ≠ claim write/`--check` alignment / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Env gate | `format_source_cst` + `env_cst_reindent_enabled` in `syx-fmt` |
| Pure CST dry-run | `format_dry_run_cst` uses concat only |
| Contract | `s2_04_cst_reindent_func_body` (+ env-gate ADR nail) |
| Write still AST | existing `s2_04_fmt_write_check_strategy` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- **Superseded by S2-I150:** [cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) — `format_source_cst` defaults to `format_cst_reindent`; `SYX_FMT_CST_REINDENT` reads removed.
- Prior: [cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md).

## References

- [stage2-s2-04-cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) · S2-I148
- [stage2-s2-04-cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) · S2-I150（supersedes env gate）
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
