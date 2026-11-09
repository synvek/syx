# SA-3 module remap census

**Current:** SA-3 exit (01…02 `[x]`; thin renames done; Collections merge deferred)  
**Next:** SA-4-01 (`System` API Swiftify)  
**Charter:** [stdlib-swift-alignment.md](../adr/stdlib-swift-alignment.md) · [module_rename_map.toml](../../packages/internal/module_rename_map.toml)

## Outcome

| Hinge | Status |
|-------|--------|
| `import System` | Os+Process+Env+Path+IO+Time+DateTime merged; old names removed |
| `import Networking` | former `Net`; old `import Net` = 0 |
| Thin renames | Sync→Synchronization, Log→Logging, Trace→Tracing, Profiler→Profiling, Sql→SQL, Di→DI, Linalg→Numerics |
| Collections | **deferred** — OrderedSet/Deque remain (S023 homonym shadow if merged) |
| packages/system + networking | aggregate `mod.sy` overlays for workspace/LSP |

## Residual

- Collections merge / ORM official status → SA-4/SA-5
- API Swiftify (`ProcessInfo` polish, hide `__syx_*`, HTTP* type renames) → **SA-4**
- Observation IMPORTABLE honesty → SA-0-05 residual

## Script

`bash scripts/sa_3_module_remap_census.sh` → `sa3_module_remap_helper=1` · `OK`
