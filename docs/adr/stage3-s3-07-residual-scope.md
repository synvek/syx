# ADR: Stage-3 S3-07 residual-scope（S3-I84）

**Status:** Accepted (S3-I84) · S3-07 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I83 default-vm](stage3-s3-07-default-vm.md), Primary KPIs are green (`init_default_template_native=0` · `init_default_template_vm≥1` · `init_vm_named_template_count≥1` · `init_integration_test_count≥1`), while UsageGuide rewrite, Init crate extract, CLI-subprocess end-to-end init tests, and tightening `_` fallback to hard-error remain open. This window revises **S3-07 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S3-07 `[x]`, change `Init`, or exit Stage-3.

## Decision

1. **Scoped acceptance (S3-07 stays `[~]` this window):** Under Hard OUT（≠ UsageGuide 大改 / ≠ 抽 Init crate / ≠ CLI 子进程 init 测必过 / ≠ `_`→native 收紧为 error / ≠ change `Init` this window）, honest S3-07 completion means:
   - **Primary:** I83 hinge holds (`default_value = "vm"`；显式 `"vm"` / `"native"` 臂)；
   - **KPI:** `init_default_template_native=0` · `init_default_template_vm≥1` · `init_vm_named_template_count≥1`；
   - **Nail:** `init_integration_test_count≥1`（driver `run_project_with_options` 等价钉）；
   - **Scaffolding:** `stage3_init_census_helper` · `init_commands_helper` green.
2. **Deferred / supersede-only:** UsageGuide `--template` 全量叙事 · Init 抽独立 crate/模板目录 · CLI 子进程 `syx init`→`syx run` 端到端 · `_` unknown→hard error are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`.
3. **Ladder next:** **S3-I85** = dedicated **S3-07 scoped `[x]` claim** under this acceptance — still ≠ Stage-3 exit.
4. **Non-claims (hard):** ≠ S3-07 `[x]` this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ UsageGuide 大改 / ≠ 抽 Init crate / ≠ change `Init` / `default_value` this window / ≠ reopen S3-06 LSP / S3-05 lint / S3-04 fib&lt;0.5 / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Census | `residual_scope_helper≥1` · Current=**S3-I84** |
| Plan | S3-07 row stays `[~]` · scoped摘要 |
| Contract | `s3_07_residual_scope_contract` |
| Prior | [I83 default-vm](stage3-s3-07-default-vm.md) · [I82 open](stage3-s3-07-open.md) |

## Consequences

- Residual-scope hinge landed；S3-07 was `[~]` under this window.
- **S3-I85 landed** as [stage3-s3-07-claim](stage3-s3-07-claim.md)（S3-07 `[x]` scoped claim）。
- Default next = **S3-I86**（Stage-3 exit open）— not Stage-3 exit itself.
- Prior: [stage3-s3-07-default-vm](stage3-s3-07-default-vm.md) · S3-I83.
- Init-template census Current → **S3-I85** · Claimed scoped `[x]`.

## References

- [stage3-s3-07-claim](stage3-s3-07-claim.md) · S3-I85
- [stage3-s3-07-default-vm](stage3-s3-07-default-vm.md) · S3-I83
- [stage3-s3-07-open](stage3-s3-07-open.md) · S3-I82
- [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md) · S3-I80（形态模板）
- [stage3-init-template-census](../stage3-init-template-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-07

