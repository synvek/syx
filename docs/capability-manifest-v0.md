# Capability Manifest v0 (M0-C / ADR-133)

Authoritative source: [`packages/internal/capabilities.toml`](../packages/internal/capabilities.toml).

## Purpose

Single machine-readable table of stdlib/runtime honesty (not the builtin taxonomy in `manifest.json`). Consumers:

1. Generated markdown → [`books/shared/capabilities-generated.md`](../books/shared/capabilities-generated.md)
2. `syx doctor --capabilities`
3. CI freshness check (`scripts/gen_capabilities.py` + `git diff --exit-code`)

## Schema

```toml
[meta]
version = "0.1"
syx_version = "117.0"
phase = 139
schema = "syx-capability-v0"

[[capability]]
id = "net.tcp"                    # stable id
feature = "net.tcp.stream"        # dotted feature name
surface = "TcpStream"             # optional public surface
status = "partial"                # see Feature status
backends = { vm = "real", native = "partial", wasm = "host-assisted", mir = "real" }
platforms = ["macos", "linux", "windows"]
tests = ["tcp_stream_handle"]         # optional gate / fixture names
notes = "Integer handle MVP (M0-B02)"
since = "M0-B02"
```

### Feature `status`

| Value | Meaning |
|-------|---------|
| `implemented` | Production-ready on **all listed backends** (no `noop` / `stub` / `unsupported` columns) |
| `partial` | Real paths exist; gaps documented (allowed weak backends) |
| `experimental` | Usable but not GA; may change |
| `experimental-real-subset` | Opt-in real execution on a documented subset (e.g. JIT Suite-1); default-off; ≠ GA |
| `preview` | Narrow / incomplete surface |
| `unsupported` | Must fail closed (no silent success) |
| `stub` | Exists but does not execute user semantics |
| `frozen` | No further surface growth this milestone |

**Honesty rule:** `status = "implemented"` is forbidden when any backend is `noop`, `stub`, or `unsupported`. Prefer `partial` / `experimental` / `experimental-real-subset` and keep backend columns truthful. Enforced by `scripts/gen_capabilities.py` (`--check` and generate).

### Backend values

| Value | Meaning |
|-------|---------|
| `real` | Executes real semantics |
| `partial` | Subset / assisted |
| `host-assisted` | Relies on host bridge |
| `stub` | Present but non-executing |
| `unsupported` | Explicit fail / skip |
| `noop` | Wired no-op (e.g. POP) |

## Regeneration

```bash
python3 scripts/gen_capabilities.py
python3 scripts/gen_capabilities.py --check
```

Do not hand-edit `books/shared/capabilities-generated.md`.
