# ADR: Stage-4 S4-01 residual-scope（S3-I90）

**Status:** Accepted (S3-I90) · S4-01 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I89 stability SSOT](stage4-s4-01-stability-ssot.md), Primary KPIs are green (`stability_ssot_package_rows≥19` · `stability_ssot_helper≥1` · `manifest_stability_field_count=0` · `syx_doctest_subcommand_count=0`), while true `manifest.json` `stability` wiring, driver consumption of SSOT, `syx doctest` CLI, and packages doctest gates remain open. This window revises **S4-01 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S4-01 `[x]`, change manifest schema, implement doctest CLI, or exit Stage-4.

## Decision

1. **Scoped acceptance (S4-01 stays `[~]` this window):** Under Hard OUT（≠ 改 `manifest.json` schema / ≠ driver 读 SSOT / ≠ 实现 `syx doctest` / ≠ packages doctest 闸 / ≠ 批量 `packages/**/*.sy`）, honest S4-01 completion means:
   - **Primary:** I89 hinge holds（[`stability.toml`](../../packages/internal/stability.toml) plan ternary SSOT；[v0 mapping](../census/stdlib-stability-v0.md)）；
   - **KPI:** `stability_ssot_package_rows≥19` · `stability_ssot_helper≥1`；`manifest_stability_field_count=0` · `syx_doctest_subcommand_count=0`（honest deferred zeros）；
   - **Scaffolding:** `stage4_open_helper` · `s4_01_census_helper` · `residual_scope_helper` green.
2. **Deferred / supersede-only:** True `manifest.json` `stability` field · driver/runtime read of `stability.toml` · `syx doctest` CLI · packages doctest CI gate · batch package annotation edits are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`. Scope charter: [s4-01-manifest-doctest-scope-v0](../census/s4-01-manifest-doctest-scope-v0.md).
3. **Ladder next:** **S3-I91** = dedicated **S4-01 scoped `[x]` claim** under this acceptance — still ≠ Stage-4 exit.
4. **Non-claims (hard):** ≠ S4-01 `[x]` this window / ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 改 `manifest.json` / ≠ driver 读 SSOT / ≠ `syx doctest` / ≠ 批量包源码 / ≠ 开 S4-02…05 / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Manifest/doctest scope | [`docs/census/s4-01-manifest-doctest-scope-v0.md`](../census/s4-01-manifest-doctest-scope-v0.md) |
| Census | `residual_scope_helper≥1` · Current=**S3-I90** |
| Plan | S4-01 row stays `[~]` · scoped摘要 |
| Contract | `s4_01_residual_scope_contract` |
| Prior | [I89 stability-ssot](stage4-s4-01-stability-ssot.md) · [I88 open](stage4-open.md) |

## Consequences

- Residual-scope hinge landed；S4-01 was `[~]` under this window；Stage-4 stays `[~]`.
- **S3-I91 landed** as [stage4-s4-01-claim](stage4-s4-01-claim.md)（S4-01 `[x]` scoped claim）。
- Default next = **S3-I92**（S4-02 open）— not Stage-4 exit / not S4-02 construct.
- Prior: [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md) · S3-I89.
- Census Current → **S3-I91** · Claimed scoped `[x]`.

## References

- [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91
- [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md) · S3-I89
- [stage4-open](stage4-open.md) · S3-I88
- [s4-01-manifest-doctest-scope-v0](../census/s4-01-manifest-doctest-scope-v0.md)
- [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md) · S3-I84（形态模板）
- [stage4-stdlib-stability-doctest-census](../census/stage4-stdlib-stability-doctest-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-01
- [`packages/internal/stability.toml`](../../packages/internal/stability.toml)
