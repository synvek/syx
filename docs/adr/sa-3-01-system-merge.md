# ADR: SA-3-01 System merge + SA-3-02 Networking

**Status:** Accepted (SA-3)  
**Date:** 2026-11-09  
**Context:** SA-2 language surface exited. Module names still Os/IO/Net per Java/Kotlin-era packages. Rename map: [`packages/internal/module_rename_map.toml`](../../packages/internal/module_rename_map.toml).

## Decision

1. **Destructive remap, no old-name aliases.** `import Os|Process|Env|Path|IO|Time|DateTime` → **`import System`**. `import Net` → **`import Networking`**.
2. **Aggregate sources, keep domain files.** `packages/system/mod.sy` and `packages/networking/mod.sy` are generated views; `embedded_std` concatenates the same parts for non-workspace embeds.
3. **≠ SA-4.** Type names may remain `Os`/`File`/`Http*`; hide `__syx_*` and Swift API polish are SA-4.
4. **Thin renames same stage:** Synchronization, Logging, Tracing, Profiling, SQL, DI, Numerics. **Collections merge deferred** (OrderedSet/Deque homonym `S023`).

## Consequences

- Manifest / IMPORTABLE SSOT lists new names only.
- Books/examples/tools/tests rewritten to new imports.
- Next default = **SA-4-01**.
