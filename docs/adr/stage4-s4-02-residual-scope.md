# ADR: Stage-4 S4-02 residual-scope（S3-I94）

**Status:** Accepted (S3-I94) · S4-02 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I93 file-registry hinge](stage4-s4-02-file-registry-hinge.md), Primary KPIs are green (`file_registry_hinge_helper≥1` · `file_registry_integration_test_count≥1` · `file_registry_evergreen_roundtrip_count≥2` · `registry_semver_r001_count≥1`), while public registry GA, lock v16 field trim, CLI five-verb UX rewrite, and multi-publisher/CDN GA remain open. This window revises **S4-02 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S4-02 `[x]`, change `syx-pkg`/CLI, trim lock v16, or exit Stage-4.

## Decision

1. **Scoped acceptance (S4-02 stays `[~]` this window):** Under Hard OUT（≠ 改 `syx-pkg`/CLI / ≠ lock v16 全面裁剪 / ≠ 公网 registry GA / ≠ 多 publisher/CDN GA）, honest S4-02 completion means:
   - **Primary:** I93 hinge holds（[`s4-02-file-registry-v0`](../census/s4-02-file-registry-v0.md)；`file://` round-trip + lock + semver）；
   - **KPI:** `file_registry_hinge_helper≥1` · `file_registry_integration_test_count≥1` · `file_registry_evergreen_roundtrip_count≥2` · `registry_semver_r001_count≥1`；
   - **Scaffolding:** `s4_02_open_helper` · `s4_02_census_helper` · `residual_scope_helper` green.
2. **Deferred / supersede-only:** Public `packages.sy.dev` GA · lock schema v16 field trim · CLI five-verb UX rewrite · multi-publisher/CDN GA are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`. Scope charter: [s4-02-registry-residual-scope-v0](../census/s4-02-registry-residual-scope-v0.md).
3. **Ladder next:** **S3-I95** = dedicated **S4-02 scoped `[x]` claim** under this acceptance — still ≠ Stage-4 exit.
4. **Non-claims (hard):** ≠ S4-02 `[x]` this window / ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 改 `syx-pkg`/CLI / ≠ lock v16 全面裁剪 / ≠ 公网 registry GA / ≠ 开 S4-03…05 / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Residual scope | [`docs/census/s4-02-registry-residual-scope-v0.md`](../census/s4-02-registry-residual-scope-v0.md) |
| Census | `residual_scope_helper≥1` · Current=**S3-I94** |
| Plan | S4-02 row stays `[~]` · scoped摘要 |
| Contract | `s4_02_residual_scope_contract` |
| Prior | [I93 file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · [I92 open](stage4-s4-02-open.md) |

## Consequences

- Residual-scope hinge landed；S4-02 was `[~]` this window；Stage-4 was `[~]`（**1/5**）.
- **S3-I95 landed** as [stage4-s4-02-claim](stage4-s4-02-claim.md)（S4-02 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I96**（S4-03 open）— not Stage-4 exit / not S4-03 construct / not public registry GA.
- Prior: [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · S3-I93.
- Package-registry census Current → **S3-I95** · Claimed scoped `[x]`。

## References

- [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95
- [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · S3-I93
- [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92
- [s4-02-registry-residual-scope-v0](../census/s4-02-registry-residual-scope-v0.md)
- [s4-02-file-registry-v0](../census/s4-02-file-registry-v0.md)
- [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md) · S3-I90（形态模板）
- [stage4-package-registry-census](../census/stage4-package-registry-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-02
- [`crates/syx-pkg`](../../crates/syx-pkg)
