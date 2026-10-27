# IR Capability Manifest v0 (M1-B04/B05 / ADR-136)

Authoritative inventory: Rust enums (`MirOp`, `HostFnId`).  
Overrides: [`mir_op_capabilities.toml`](../packages/internal/mir_op_capabilities.toml), [`host_fn_capabilities.toml`](../packages/internal/host_fn_capabilities.toml).

## Purpose

Machine-readable per-op / per-host backend honesty. Default for any inventory name without an override is **`unsupported`** (explicit row still emitted — omit is forbidden).

## Generate / CI

```bash
python3 scripts/gen_ir_capabilities.py
python3 scripts/gen_ir_capabilities.py --check
```

Outputs:

- [`books/shared/mir-op-capabilities.md`](../books/shared/mir-op-capabilities.md)
- [`books/shared/host-fn-capabilities.md`](../books/shared/host-fn-capabilities.md)
- `packages/internal/mir_op_capabilities.generated.json`
- `packages/internal/host_fn_capabilities.generated.json`

## Status vocabulary

Feature-level `status` (overall row): same as M0 capability manifest, including `experimental-real-subset`.

Backend columns: `real`, `partial`, `host-assisted`, `stub`, `unsupported`, `noop`.

**Every HostFn / MirOp inventory name gets an explicit row** with a full backend map (missing TOML override ⇒ overall `unsupported` and all backends `unsupported`). Backend columns are authoritative for per-backend honesty; overall `status` is the gate summary (M3-E03 uses overall `unsupported` only).

**Honesty rule:** `status = "implemented"` is forbidden when any backend is `noop`, `stub`, or `unsupported` (enforced by `gen_ir_capabilities.py`).

## Consumers

- CI freshness (`--check`)
- `syx doctor --ir-capabilities`
- **M3-E03 / ADR-313:** opt-in compile gate (`--ir-capability-gate` / `SYX_IR_CAPABILITY_GATE`) reads overall HostFn `status`; `unsupported` → semantic **`S026`**. Fingerprint: `hostfn-capability-gate-v0:compile-unsupported`. Does **not** refuse `partial`/`host-assisted`; does **not** gate by backend columns; default off.
- LSP / semantic: `syx_semantic::host_fn_capability` (overall + per-backend lookup)