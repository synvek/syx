# R-47 package API — System Env / FilePath census

**Date:** 2026-11-12  
**Scope:** System-domain Env caseless enum + FilePath value struct + F3 gen-only boundary honesty.  
**Script:** [`scripts/r47_package_api_census.sh`](../../scripts/r47_package_api_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-46 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-47** | **exited** — System Env/FilePath slice (≠ full F2/F3 delete) |
| Open | 编号收口后 park；R-47 joining 链残余 → **U-0** 消化 |

## Root cause

Package names were Swiftified (SA-3/SA-4) but API shape lagged: `Env` remained a Java-style `class`; `FilePath` was a String-helper enum; domain dirs `os`/`io`/`net` looked user-visible without an honest gen-only boundary ([todo-005](../../plan/todo-005.md) F2/F3).

## Delivery

1. [`packages/core/env.sy`](../../packages/core/env.sy): `public class Env` → caseless `public enum Env`.
2. [`packages/core/path.sy`](../../packages/core/path.sy): `struct FilePath { rawValue }` + instance methods; String static bridges retained.
3. Regen [`packages/system/mod.sy`](../../packages/system/mod.sy) via [`scripts/gen_system_mod.py`](../../scripts/gen_system_mod.py).
4. Driver: inject/export public `Struct` from imported modules ([`prelude_glue.rs`](../../crates/syx-driver/src/prelude_glue.rs)) so `import System` resolves `FilePath`.
5. F3 honesty: [`packages/system/README.md`](../../packages/system/README.md) + [`packages/networking/README.md`](../../packages/networking/README.md) nail domain = build-time gen only.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `Env` shape | caseless `enum` | **enum** |
| `FilePath` shape | `struct` + `rawValue` | **struct** |
| system/networking README gen-only | present | **pass** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ delete `os/`/`net/`/`io/`/`sync/`/`log/` trees
- ≠ Networking `HTTPClient` / wipe all `SyxHost.*`
- ≠ `ProcessInfo.processInfo.environment` dictionary surface
- ≠ （原）fix struct method-chain residual after `joining` — **已由 [`u-0-filepath-joining-census`](u-0-filepath-joining-census.md) 消化**
- ≠ JIT unpark / `func…async` / 新开 R-50

## References

- [`r-46-books-ch17-census`](r-46-books-ch17-census.md)
- [`u-0-filepath-joining-census`](u-0-filepath-joining-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-47 → U-0 → park / U-1
