# Macro / embed threat model (C-5a)

**Status:** living SSOT for macro/embed “sandbox” honesty  
**Date:** 2026-11-24  
**Axis:** C-5a + C-5b (≠ C-5 exited; next default = C-6)

## Decision

Public claims must not describe macro re-exec or embed soft flags as **security isolation** / OS sandbox / hardening GA. Use this document as the threat-model SSOT.

## Threat model (four pins)

1. **Macro default = crash-containment reexec only**  
   External macros expand in a child process that re-execs the same `syx` binary (`SandboxMacroExpander`). This limits crash blast radius. It is **not** a permission sandbox: no fs/net/env/cwd deny, no timeout/memory hard limit by default, stderr discarded. Mode label: `sandbox-reexec-only`.

2. **Embed flags = soft resource / HTTP deny / optional wasmtime**  
   `--embed-sandbox` / `SYX_EMBED_SANDBOX` mainly sets `native_no_http`. Timeout / memory / RSS knobs are **soft budgets** (and some paths do not enforce them). Optional wasmtime is not isolation GA. **C-5b:** when `SYX_EMBED_WASM_SANDBOX=1`, wasm failure/unavailable → `syx_embed_v2_run` returns **null** (no silent in-process fallback). Unset env → default in-process unchanged.

3. **`--permission-sandbox` = opt-in HostFn S024**  
   Compile-time HostFn domain gate (net/fs/env/process/plugin). **Not** OS process isolation. Default remains **off**. Behavior unchanged by C-5a. See also [`capability-sandbox-policy-v0.md`](../capability-sandbox-policy-v0.md).

4. **`SYX_MACRO_SECCOMP` / `SYX_EMBED_SECCOMP` = label/probe until wired**  
   These env vars must **not** be documented as hardening GA. Macro mode when the env is set returns `seccomp-label-unwired` (filters are not installed). Embed `*seccomp_probe*` APIs are probe strings only.

## Non-claims

- ≠ real seccomp / Job Object / WASI capability / cgroup  
- ≠ wasm timeout / RSS hard enforcement (park → **C-5c+**)  
- ≠ permission-sandbox default-on  
- ≠ C-5 `[x]` after C-5a/C-5b alone

## Related

- Historical table: [`architecture-decisions.md`](architecture-decisions.md) (External macros row)  
- Roadmap: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) C-5 / C-5a / C-5b  
- Census: [`c-5a-threat-model-census`](../census/c-5a-threat-model-census.md) · [`c-5b-embed-wasm-fail-closed-census`](../census/c-5b-embed-wasm-fail-closed-census.md)
