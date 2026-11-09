# ADR: Stage-3 S3-07 open — `syx init` 默认 VM 模板（S3-I82）

**Status:** Accepted (S3-I82) · S3-07 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I81 S3-06 claim](stage3-s3-06-claim.md), Stage-3 default work shifts to [plan/02 S3-07](../../plan/02-improvement-roadmap.md): **`syx init` 默认 VM 模板**，与当前可信后端一致。Honest inventory shows `Commands::Init` inlined in [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) with `--template` **`default_value = "native"`**, while `syx run` (no `--wasm`) already takes the **VM** path, and plan/01 notes Native is **not** verified under default builds. This window **opens** the workstream with an init-template **census baseline** — it does **not** claim S3-07 `[x]`, change `Init` defaults, or exit Stage-3.

## Gap: init default vs plan S3-07

| Claim | Today (honest) | plan/02 S3-07 target |
|-------|----------------|----------------------|
| 默认模板与可信后端一致 | `default_value = "native"`；else 臂 `print("native")` | 默认对齐 **VM / `syx run`** |
| 显式 `vm` 模板名 | **无** `vm` match 臂 | 可命名 / 默认可跑 VM 脚手架 |
| init→run 机械验收 | **无** init 集成测 | tempdir init + `syx run` 钉 |
| Native 脚手架诚实 | 默认名暗示 native；默认构建 ≠ 可跑 | `--template native` 可保留但非默认 |

Baseline: [stage3-init-template-census](../census/stage3-init-template-census.md).

## Decision

1. **Open S3-07** as an active Stage-3 workstream; flip plan S3-07 `[ ]` → `[~]`.
2. **S3-I82 first slice** — publish init-template census (+ script); add a thin contract under `syx-driver/tests/` (**no** `syx-cli` `Init` behavior edit this window).
3. **Ladder:** **S3-I83** = first construct knife（priority: 改默认 / 显式 `vm` 臂 + init→run 钉，由本窗 census 数字驱动）→ residual/claim-prep（可压缩）→ dedicated **S3-07 `[x]` claim**.
4. **Non-claims (hard):** ≠ S3-07 `[x]` / ≠ 本窗改 `Init` / `default_value` / ≠ 本窗宣称 init→run 必绿 / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ reopen S3-06 LSP 产品刀 / ≠ reopen S3-05 lint / S3-04 fib&lt;0.5 / ≠ change language semantics / ≠ expand HostFn surface.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Init-template census | [`docs/census/stage3-init-template-census.md`](../census/stage3-init-template-census.md) · `scripts/s3_07_init_template_census.sh` |
| Contract nail | `s3_07_init_template_census_contract` |
| Plan status | S3-07 `[~]` under this ADR · Stage-3 `[~]` · later **7/7** · S3-01…06 `[x]` |

## Consequences

- S3-07 moved `[ ]` → `[~]` under this open window; Stage-3 stays `[~]`（checklist later **7/7** under S3-I85）; S3-01…06 stay `[x]`.
- **S3-I83 landed** as [stage3-s3-07-default-vm](stage3-s3-07-default-vm.md)（default-vm construct）。
- **S3-I84 landed** as [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md)（residual-scope）。
- **S3-I85 landed** as [stage3-s3-07-claim](stage3-s3-07-claim.md)（S3-07 `[x]` scoped claim）。
- **S3-I86 landed** as [stage3-exit-open](stage3-exit-open.md)（Stage-3 exit open）。
- Default next = **S3-I87**（write `stage3-exit`）— not Stage-3 exit itself / not S3-07 `[x]` this open window.
- Prior: [stage3-s3-06-claim](stage3-s3-06-claim.md) · S3-I81.
- Init-template census Current → **S3-I85** · Claimed scoped `[x]`；exit census Current → **S3-I86**.

## References

- [stage3-s3-06-claim](stage3-s3-06-claim.md) · S3-I81
- [stage3-open](stage3-open.md) · S3-I1
- [stage3-init-template-census](../census/stage3-init-template-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-07
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `Commands::Init`
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §3.3
