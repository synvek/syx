# ADR: SA-4 System API Swiftify + host hide

- **Status:** Accepted (SA-4-01 + SA-4-04 exit)
- **Date:** 2026-11-09

## Decision

1. **Public System surface:** `ProcessInfo` / `FilePath` / `SystemError` replace user-facing `Os` / `Path` / `OsError`+`IOError`+`ProcessError`.
2. **Host hide:** `__syx_*` may appear only under `packages/internal/*_host.sy` (stdlib `system_host`, Numerics `linalg_host`). Public package sources call `SyxHost.*`.
3. **Load path:** workspace/embed prepends the appropriate host via `ensure_syx_host_for`; MIR always merges `SyxHost_*`.
4. **Networking thin:** primary `Http.fetch`/`request`/`post` drop `options: Object` / untyped body on the main path.

## Consequences

- Destructive: no `Os` / `Path` aliases.
- Residual Object on a few File/DateTime async paths → SA-4-03.
- Next: **SA-5-01** library-guide.
