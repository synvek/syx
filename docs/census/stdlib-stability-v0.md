# Stdlib package stability SSOT v0

**Window:** S3-I89 · S4-01 construct  
**SSOT file:** [`packages/internal/stability.toml`](../packages/internal/stability.toml)  
**Human table:** [`packages/README.md`](../packages/README.md) Status column  
**Plan target:** `stable` / `preview` / `experimental`（[plan/02 S4-01](../plan/02-improvement-roadmap.md)）

## Purpose

Provide a **machine-readable** package-level stability ternary aligned with plan/02, while keeping the README five-label Status table as the human-facing narrative. This does **not** wire `manifest.json`, implement `syx doctest`, or claim S4-01 `[x]`.

## Mapping (README → plan ternary)

| README Status (contains) | `stability` |
|--------------------------|-------------|
| Stable（含 Stable / MVP、Stable / evolving） | `stable` |
| MVP（无 Stable 前缀） | `preview` |
| Partial | `preview` |
| Experimental | `experimental` |
| Deferred | `experimental` |
| Tooling only（`internal`） | `experimental` |

## TOML schema

```toml
[meta]
version = "0.1"
schema = "syx-stdlib-stability-v0"

[[package]]
name = "Os"                 # package directory / display name
import = "Os"               # primary import token（empty for internal）
readme_status = "MVP"       # verbatim / shortened README Status
stability = "preview"       # stable | preview | experimental
```

## Honesty

- README labels remain coarse; SSOT is the **claim gate inventory** for later residual / claim windows.
- `internal` is tooling-only and maps to `experimental`（not a fourth enum in KPI counts）.
- Driver / CLI **do not** consume this file yet（Hard OUT for S3-I89）.

## References

- [stage4-s4-01-stability-ssot](adr/stage4-s4-01-stability-ssot.md) · S3-I89
- [stage4-open](adr/stage4-open.md) · S3-I88
- [stage4-stdlib-stability-doctest-census](stage4-stdlib-stability-doctest-census.md)
