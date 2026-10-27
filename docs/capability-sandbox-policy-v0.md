# Capability Sandbox Policy v0 (M3-E01 / ADR-308 · M3-E02 / ADR-310–312 · M3-E03 / ADR-313–314)

Fingerprint (policy intent): `permission-policy-v0:default-deny-intent`

Fingerprint (grants deepen): `permission-grants-v0:allow-net-fs-toml`

Fingerprint (grants thin, retained): `permission-grants-v0:allow-net-cli`

Fingerprint (HostFn capability gate thin): `hostfn-capability-gate-v0:compile-unsupported`

## Status

**E01 docs leaf signed** (ADR-309). **E02 grants leaf signed** (ADR-310–312): opt-in permission sandbox + `net`/`fs` domain CLI/env/TOML grants + unauthorized fail-closed (`S024`). **E03 capability-table compile gate leaf signed** (ADR-313–314): opt-in `--ir-capability-gate` refuses overall `status=unsupported` HostFns with **`S026`**. Still ≠ EC-M3-3; ≠ five-domain default-deny GA; ≠ default-on capability gate; ≠ backend-column GA; ≠ load-time gate.

## Purpose

Define default policy for five domains, the grant surface, and the HostFn capability-table compile linkage so later windows can open EC-M3-3 without inventing the model ad hoc.

## Relation to existing surfaces

| Surface | Role | Not this policy |
|---------|------|-----------------|
| [`capability-manifest-v0.md`](capability-manifest-v0.md) / [`capabilities.toml`](../packages/internal/capabilities.toml) | Capability honesty (what backends implement) | Not a permission sandbox |
| [`ir-capability-manifest-v0.md`](ir-capability-manifest-v0.md) / HostFn capability table | Per-HostFn backend honesty | Not permission grants (E02) |
| Embed sandbox / `SYX_EMBED_SANDBOX` | Narrow embed/host deny paths | Not EC-M3-3 product CLI sandbox |
| This policy v0 + grants + capability gate | Product intent + opt-in net/fs + opt-in table gate | Not five-domain GA / not default-on |

## Domains (default deny intent)

| Domain | Intent | Explicit grant | Notes |
|--------|--------|----------------|-------|
| `fs` | Deny filesystem read/write/list unless granted | **`--allow-fs` / `SYX_ALLOW_FS` / `[permissions].allow_fs`** | Gates `FileRead` / `FileWrite` under sandbox |
| `net` | Deny connect/listen/DNS unless granted | **`--allow-net` / `SYX_ALLOW_NET` / `[permissions].allow_net`** | Gates `TcpConnect` / `HttpGet` under sandbox |
| `env` | Deny env read/write unless granted | Key allowlist or `env` blanket | Align Process/Os env later |
| `process` | Deny spawn/exec/kill unless granted | Binary allowlist or `process` blanket | Align Process HostFns later |
| `plugin` | Deny loading external plugins/macros unless granted | Plugin id allowlist | Align `#externalMacro` / cdylib later |

**Product intent:** operations in these domains are **default deny** (or available only after an explicit grant). Un-granted attempts must **fail closed** (typed error), never silent success.

## Grants v0 (M3-E02 thin + deepen + closeout)

Opt-in only (default sandbox **off** — existing toolchain unchanged):

| Surface | Flag / env / TOML | Effect |
|---------|-------------------|--------|
| Enable sandbox | `--permission-sandbox` / `SYX_PERMISSION_SANDBOX=1` / `[permissions].sandbox` | Un-granted domain HostFns fail with **`S024`** |
| Grant net | `--allow-net` / `SYX_ALLOW_NET=1` / `[permissions].allow_net` | Allows `TcpConnect` / `HttpGet` |
| Grant fs | `--allow-fs` / `SYX_ALLOW_FS=1` / `[permissions].allow_fs` | Allows `FileRead` / `FileWrite` |
| Grant env | `--allow-env` / `SYX_ALLOW_ENV=1` / `[permissions].allow_env` | Allows `EnvGet` / `EnvSet` (W1035 thin) |
| Grant process | `--allow-process` / `SYX_ALLOW_PROCESS=1` / `[permissions].allow_process` | Allows `ProcessExec` / `ProcessSpawn` (W1035 thin) |
| Grant plugin | `--allow-plugin` / `SYX_ALLOW_PLUGIN=1` / `[permissions].allow_plugin` | Allows `__syx_plugin_*` prefix (W1036 thin) |

Example `syx.toml` slice:

```toml
[permissions]
sandbox = true
allow_net = true
allow_fs = true
allow_env = true
allow_process = true
allow_plugin = true
```

Unauthorized negative: sandbox on + missing domain grant → compile-time **`S024`** (prelude name-shadow remains **`S023`**).

## HostFn capability gate v0 (M3-E03 thin + closeout)

Opt-in only (default gate **off**):

| Surface | Flag / env | Effect |
|---------|------------|--------|
| Enable gate | `--ir-capability-gate` / `SYX_IR_CAPABILITY_GATE=1` | HostFns with overall table `status=unsupported` fail with **`S026`** |

- Lookup: [`host_fn_capabilities.generated.json`](../packages/internal/host_fn_capabilities.generated.json) overall `status` only (not backend columns).
- Does **not** refuse `partial` / `host-assisted`.
- Orthogonal to E02 permission grants (`S024`).

## Honesty (this window)

- E01 five-domain default-deny intent remains **docs-only** for **plugin-id allowlist / five-domain GA / true default-on execute** except opt-in `net`/`fs`/`env`/`process`/`plugin` thin gates (≠ five-domain runtime GA).
- **W1038 / ADR-1382 default-on product decision** = **remain opt-in** (sandbox stays off unless `--permission-sandbox` / `SYX_PERMISSION_SANDBOX` / `[permissions].sandbox`); default-on **deferred** to an explicit execute window (**docs-only** until then).
- **W1039 / ADR-1383 D-04 `[x]`** = **opt-in written narrowed-domain** (five-domain thin gates). **docs-only** remains for **plugin-id allowlist / five-domain default-on GA / true default-on execute**.
- **No** five-domain runtime default-deny GA (opt-in `net` + `fs` + `env` + `process` + `plugin` thin slices).
- **No** path-granular fs allowlist.
- HostFn capability-table compile gate is **opt-in unsupported-only** (≠ default global; ≠ backend-column GA; ≠ load-time).
- **No** EC-M3-3 checkbox; **no** M3 exit claim; E03 leaf signed under narrowed acceptance.

## Follow-ons

1. **EC-M3-1** — high-concurrency stress reproducibility evidence thin/closeout.
2. Optional deepen: refuse `partial` / backend-column gates / load-time MIR scan; **default-on execute** / plugin-id allowlist.
