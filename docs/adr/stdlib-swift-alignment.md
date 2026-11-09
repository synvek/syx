# ADR: Stdlib Swift alignment charter（SA-0-01）

**Status:** Accepted (SA-0-01)  
**Date:** 2026-11-09  
**Context:** Credibility stages 0→4 have exited. Language surface already follows the Swift-style book ([language-book-ssot](language-book-ssot.md)), but the embedded stdlib still uses a large prelude, Java/Kotlin-shaped package APIs, and module names that diverge from the Swift open-source server ecosystem. Analysis: [`plan/todo-003.md`](../../plan/todo-003.md). Living backlog: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) (SA-0…SA-6).

## Decision

1. **Three-layer stdlib model**
   - **Core (implicit / prelude):** language-essential types, protocols, concurrency primitives, basic numerics/random/print — Swift standard library names where applicable.
   - **Official companion modules (toolchain, explicit `import`):** e.g. future `System`, `Networking`, `JSON`, `Synchronization`, `Testing`, `Logging`.
   - **Official independent packages (`syx add`, versioned separately):** e.g. `Collections`, `Crypto`, `Numerics`, `SQL`, `UI`, `DI`.

2. **Destructive alignment is allowed.** There is **no** compatibility window and **no** obligation to keep old module names or old APIs. Prefer delete/rename over `@available(..., deprecated, renamed:)`.

3. **Never name an official module `Foundation`.** Avoid implying Apple Foundation compatibility (`URLSession`, `FileManager`, …). Borrow Swift *naming style*, not the Foundation product surface.

4. **Package vs module naming**
   - Package / registry id: kebab-case with `syx-` prefix (e.g. `syx-collections`).
   - User `import` name: UpperCamelCase (e.g. `import Collections`).

5. **Error-handling default (locks SA-2-01):** implement Swift `do { try … } catch` for the language surface; stdlib sources use **`throws -> T`** only (not `-> T throws`). Construction of `do/catch` is **SA-2**, not this window.

6. **Machine-readable SSOT files (this stage)**
   - Target prelude whitelist: [`packages/internal/prelude_core.toml`](../../packages/internal/prelude_core.toml) (SA-0-02; behavior change in SA-1).
   - Rename map: [`packages/internal/module_rename_map.toml`](../../packages/internal/module_rename_map.toml) (SA-0-04; apply in SA-3).
   - Current prelude / importable lists must agree across `syx-config::CORE_PRELUDE_MODULES`, `packages/internal/manifest.json`, and `embedded_std` (SA-0-03).

7. **Non-claims (hard):** ≠ SA-1 prelude shrink this window · ≠ module rename landing · ≠ package API Swiftify · ≠ `syx doctest` · ≠ product-umbrella unpark · ≠ F-08 · ≠ HostFn+1 KPI · ≠ Apple Foundation compatibility.

## Consequences

- SA construction cites this ADR; SA-1 consumes `prelude_core.toml`; SA-3 consumes `module_rename_map.toml`.
- Capability / marketing narrative must not claim Foundation parity.
- SA-0…SA-3 exited; next default = **SA-4-01** (System API Swiftify).

## References

- [`plan/todo-003.md`](../../plan/todo-003.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · SA-0
- [`plan/01-project-status.md`](../../plan/01-project-status.md)
- [language-book-ssot.md](language-book-ssot.md)
- [Swift API Design Guidelines](https://www.swift.org/documentation/api-design-guidelines/)
