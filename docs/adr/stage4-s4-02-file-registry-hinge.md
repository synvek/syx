# ADR: Stage-4 S4-02 file-registry hinge（S3-I93）

**Status:** Accepted (S3-I93) · S4-02 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I92 S4-02 open](stage4-s4-02-open.md), census showed CLI/`syx-pkg` inventory green while Stage-4 still lacked a **Primary construct hinge**. Existing `file://` publish/install tests and R001 semver nails already pass. This window elevates that evidence into a **machine-checkable hinge** (charter + census KPIs + contract). It does **not** claim S4-02 `[x]`, change `syx-pkg`/CLI behavior, trim lock v16, declare public registry GA, or exit Stage-4.

## Decision

1. **Publish** charter [`docs/census/s4-02-file-registry-v0.md`](../census/s4-02-file-registry-v0.md)（Primary = `file://` round-trip + lock + semver；公网 GA / lock trim deferred）。
2. **Nail Primary KPIs** on existing tests（no product edit）：
   - `file_registry_integration_test_count≥1` · `registry_install_from_file_registry`
   - `file_registry_evergreen_roundtrip_count≥2` · `publish_file_registry` + `publish_install_roundtrip`
   - `registry_semver_r001_count≥1` · `r001_diamond_registry_`
3. **Ladder next:** **S3-I94** = S4-02 residual-scope — still ≠ S4-02 `[x]`.
4. **Non-claims (hard):** ≠ S4-02 `[x]` / ≠ Stage-4 exit / ≠ 改 `syx-pkg`/CLI / ≠ lock v16 全面裁剪 / ≠ 公网 registry GA / ≠ S4-03…05 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Charter | [`docs/census/s4-02-file-registry-v0.md`](../census/s4-02-file-registry-v0.md) |
| Census KPI | `file_registry_hinge_helper≥1` · integration/evergreen/R001 counts |
| Contract | `s4_02_file_registry_hinge_contract` |
| Prior | [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92 |

## Consequences

- File-registry Primary hinge landed；S4-02 was `[~]` this window；Stage-4 was `[~]`（**1/5**）.
- **S3-I94 landed** as [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md)（scoped acceptance · deferred public GA/lock trim）。
- **S3-I95 landed** as [stage4-s4-02-claim](stage4-s4-02-claim.md)（S4-02 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I96**（S4-03 open）— not Stage-4 exit / not public registry GA.
- Prior: [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92.
- Package-registry census Current → **S3-I95** · Claimed scoped `[x]`。

## References

- [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95
- [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md) · S3-I94
- [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92
- [s4-02-file-registry-v0](../census/s4-02-file-registry-v0.md)
- [stage4-package-registry-census](../census/stage4-package-registry-census.md)
- [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md) · S3-I89（construct 形态）
- [plan/02](../../plan/02-improvement-roadmap.md) S4-02
- [`crates/syx-pkg`](../../crates/syx-pkg)
