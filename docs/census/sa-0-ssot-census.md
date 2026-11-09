# SA-0 SSOT census

**Current:** SA-0-03  
**Next:** SA-1-01  
**Charter:** [stdlib-swift-alignment.md](../adr/stdlib-swift-alignment.md)

## Purpose

Keep **current** prelude / importable / retired surfaces honest across:

- `crates/syx-config` → `CORE_PRELUDE_MODULES`
- `packages/internal/manifest.json`
- `crates/syx-driver` → `embedded_std::IMPORTABLE`
- `syx_stdlib::retired_ops_module_names`

Does **not** shrink prelude (SA-1) or rename modules (SA-3).

## Hinges fixed this window

| Issue | Fix |
|-------|-----|
| `Fmt` in config/CORE but missing from manifest prelude | Add `Fmt` to `build.rs` prelude + manifest |
| Trace / Metrics / Profiler importable in driver, absent from manifest | Add to `importable_modules` |
| `Text` both importable and retired | Remove from retired (manifest + `retired_ops_module_names`) |

## Artifacts

| File | Role |
|------|------|
| [`packages/internal/prelude_core.toml`](../../packages/internal/prelude_core.toml) | Target whitelist (SA-1) + `current_injected` snapshot |
| [`packages/internal/module_rename_map.toml`](../../packages/internal/module_rename_map.toml) | Rename map (SA-3) |
| [`scripts/sa_0_ssot_census.sh`](../../scripts/sa_0_ssot_census.sh) | Machine census |

## KPI

Run `bash scripts/sa_0_ssot_census.sh` → `prelude_match=1` · `importable_match=1` · `text_dual_cleared=1` · `sa0_ssot_helper=1`.
