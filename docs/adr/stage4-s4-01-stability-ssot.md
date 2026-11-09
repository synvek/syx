# ADR: Stage-4 S4-01 stability SSOT（S3-I89）

**Status:** Accepted (S3-I89) · S4-01 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I88 Stage-4 open](stage4-open.md), census showed README five-label Status with **no** machine-readable plan ternary and `manifest_stability_field_count=0` / `syx_doctest_subcommand_count=0`. This window lands the **first construct hinge**: a package-level **`stability.toml` SSOT** (`stable|preview|experimental`) + v0 mapping doc. It does **not** claim S4-01 `[x]`, wire `manifest.json`, implement `syx doctest`, or exit Stage-4.

## Decision

1. **Add** [`packages/internal/stability.toml`](../../packages/internal/stability.toml) as SSOT：one `[[package]]` per README Status row；`stability` ∈ {`stable`,`preview`,`experimental`}。
2. **Publish** mapping charter [`docs/stdlib-stability-v0.md`](../stdlib-stability-v0.md)（README 五档 → 三元）。
3. **Point** [`packages/README.md`](../../packages/README.md) at the SSOT（narrative pointer only；**do not** rewrite Status table）。
4. **Ladder next:** **S3-I90** = S4-01 residual-scope（manifest 接线 / doctest 范围界定）— still ≠ S4-01 `[x]`.
5. **Non-claims (hard):** ≠ S4-01 `[x]` / ≠ Stage-4 exit / ≠ 改 `manifest.json` schema / ≠ driver 读 stability / ≠ 实现 `syx doctest` / ≠ 批量改 `packages/**/*.sy` / ≠ 开 S4-02…05 / ≠ HostFn+1 / ≠ F-08 / ≠ 产品伞 unpark / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| SSOT file | `packages/internal/stability.toml` · `[[package]]` ≥19 |
| Mapping | `docs/stdlib-stability-v0.md` |
| Census KPI | `stability_ssot_helper≥1` · `stability_ssot_package_rows≥19`；manifest/doctest 仍 0 |
| Contract | `s4_01_stability_ssot_contract` |
| Prior | [stage4-open](stage4-open.md) · S3-I88 |

## Consequences

- Machine-readable plan ternary inventory exists；Stage-4 stays `[~]`.
- **S3-I90 landed** as [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md)（scoped acceptance · deferred manifest/doctest）。
- **S3-I91 landed** as [stage4-s4-01-claim](stage4-s4-01-claim.md)（S4-01 `[x]` scoped claim）。
- Default next = **S3-I92**（S4-02 open）— not Stage-4 exit / not S4-02 construct.
- Prior: [stage4-open](stage4-open.md) · S3-I88.
- Census Current → **S3-I91** · Claimed scoped `[x]`.

## References

- [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91
- [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md) · S3-I90
- [stage4-open](stage4-open.md) · S3-I88
- [stdlib-stability-v0](../stdlib-stability-v0.md)
- [stage4-stdlib-stability-doctest-census](../stage4-stdlib-stability-doctest-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-01
- [`packages/internal/stability.toml`](../../packages/internal/stability.toml)
