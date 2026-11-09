# SA-6 exit census

**Status:** SA-6 exit hinge met (SA-6-01 + SA-6-03 `[x]`) · next = **post-SA Residual / 维护**  
**Date:** 2026-11-09  
**Script:** [`scripts/sa_6_exit_census.sh`](../../scripts/sa_6_exit_census.sh) · ADR: [`docs/adr/sa-exit.md`](../adr/sa-exit.md)

## Exit hinge (SA-6)

| ID | Claim | Evidence |
|----|-------|----------|
| SA-6-01 | Exit census + scoped Object slice | Script green; see KPI below |
| SA-6-03 | SA exit ADR | [`docs/adr/sa-exit.md`](../adr/sa-exit.md) Accepted |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `prelude_match` | **1** | `CORE_PRELUDE_MODULES` = manifest |
| `importable_match` | **1** | embedded `IMPORTABLE` = manifest |
| `old_import_lines` | **0** | packages / examples / books |
| `public_syx_hostfn_spellings` | **0** | non-`packages/internal` `__syx_*` |
| `system_io_object_slice_cleared` | **1** | `File.stat` / `readAsync` ≠ Object |
| `filehandle_withopen_object_residual` | **0** | Cleared in R-1 (`body: Function`) |
| `object_sig_residual_count` | **>0** (inventory) | Honest residual; **≠** full-library Object=0 |

### System/IO Object slice

| API | Was | Now |
|-----|-----|-----|
| `File.stat` | `-> Object` | `-> Dictionary` **cleared** |
| `File.readAsync` | `-> Object` | `-> Dictionary` **cleared** |
| `FileHandle.withOpen` | `body: Object` | `body: Function` **cleared** (R-1) |

Domain sources: `packages/io/mod.sy` (+ regenerate `packages/system/mod.sy` via `scripts/gen_system_mod.py`).

## Scoped honesty (≠ literal Object=0)

`plan/02` historically said public Object signatures = 0. **SA-6-01 supersedes** that as a **scoped** KPI:

- Hard green: System/IO named residual above.
- Inventory: remaining `: Object` / `-> Object` in public packages (simd, Channel, JSON, Networking, …) stay as **`object_sig_residual_count`** → **post-SA Residual**.

## How to run

```bash
bash scripts/sa_6_exit_census.sh
```

## Next

**R-0 已交** — 见 [r-0-residual-census](r-0-residual-census.md)。下一 = **R-1**（`withOpen` / Channel）或 Residual 维护（见 [sa-exit](../adr/sa-exit.md) residual 表）。
