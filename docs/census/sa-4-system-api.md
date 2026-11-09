# SA-4 System / Networking API census

## SA-4-01 surface

| Item | Status |
|------|--------|
| `ProcessInfo` caseless enum (queries + `processorCount`) | ok |
| `FilePath` caseless enum (ex-`Path`) | ok |
| `SystemError: Error` (merges Os/IO/Process errors) | ok |
| User-facing `class Os` | removed |
| `import System` smoke | ok |

## SA-4-04 host

| Item | Status |
|------|--------|
| `packages/internal/system_host.sy` | stdlib `SyxHost` |
| `packages/internal/linalg_host.sy` | Numerics `SyxHost` (+ `plugin_*`) |
| non-internal `packages/**/*.sy` `__syx_*` | **0** (see `sa-4-host-surface.md`) |

## SA-4-02 Networking thin

| Item | Status |
|------|--------|
| `Http.fetch` / `request` without `options: Object` | ok |
| `Http.post` / `response` body typed as `String` | ok |
| `HttpBodyStream.openWithOptions` options bag removed | ok |

## Residual → later windows

| Item | Track |
|------|--------|
| `File.stat` / `File.readAsync` / `FileHandle.withOpen` still `Object` | SA-4-03 |
| `DateTime` parse bags fully modeled as value types | SA-4-03/07 |
| Deep HTTP Configuration / WebSocket `Object` sockets | SA-4-02 deepen / SA-4-07 |
| Apple Foundation name parity | out of SA-4 |

## Next

**SA-5 `[x]`** (library-guide / examples / `syx doctest`) — see [`sa-5-docs-tools.md`](sa-5-docs-tools.md). Next = **SA-6-01**.
