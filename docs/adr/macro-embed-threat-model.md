# Macro / embed threat model (C-5a · … · C-5h)

**Status:** living SSOT for macro/embed “sandbox” honesty  
**Date:** 2026-11-25  
**Axis:** C-5a…C-5h (≠ C-5 exited; next = **跨轴**，不宣称 C-5 `[x]`)

## Decision

Public claims must not describe macro re-exec or embed soft flags as **security isolation** / OS sandbox / hardening GA. Use this document as the threat-model SSOT.

## Threat model (four pins)

1. **Macro default = crash-containment reexec only**  
   External macros expand in a child process that re-execs the same `syx` binary (`SandboxMacroExpander`). Mode label: `sandbox-reexec-only`.  
   **C-5c-1:** `SYX_MACRO_TIMEOUT_MS` → kill child → **M040**.  
   **C-5c-2:** `SYX_MACRO_SECCOMP=1` on **Linux** → net-deny BPF (`socket`/`connect`/`bind`/`listen`); mode `seccomp-netdeny-wired`. Non-Linux → **M040**.  
   **C-5d / C-5e:** `SYX_MACRO_RSS_MB` → Linux `prlimit(RLIMIT_AS)` / Windows Job Object; else **M040**.  
   **C-5f:** `SYX_MACRO_MACOS_SANDBOX=1` → macOS Seatbelt net-deny; else **M040**.  
   **C-5g:** `SYX_MACRO_SECCOMP_ALLOWLIST=1` on **Linux** → **default-deny** allowlist BPF (v11 base + minimal runnable supplements); mode `seccomp-allowlist-wired`. Mutually exclusive with `SYX_MACRO_SECCOMP`. Non-Linux → **M040**.

2. **Embed flags**  
   **C-5b:** wasm sandbox fail-closed.  
   **C-5c-1:** timeout hard wall.  
   **C-5c-2:** `SYX_EMBED_SECCOMP=1` → Linux net-deny in worker; else null.  
   **C-5d / C-5e:** `SYX_EMBED_RSS_MB` → Linux prlimit / Windows Job Object; macOS → null.  
   **C-5f:** `SYX_EMBED_MACOS_SANDBOX=1` → wasmtime under `sandbox-exec`; in-process → null.  
   **C-5g:** `SYX_EMBED_SECCOMP_ALLOWLIST=1` → Linux default-deny allowlist in worker; mutually exclusive with `SYX_EMBED_SECCOMP` → null; non-Linux → null.  
   **MEMORY:** stdout-sum soft gate.

3. **Permission sandbox = HostFn S024 (C-5h default-on)**  
   **Default on** when env/CLI unset. Ungranted net/fs/env/process/plugin HostFn slices → **S024**.  
   Opt-out: `--no-permission-sandbox` / `SYX_PERMISSION_SANDBOX=0|false`.  
   Explicit on: `--permission-sandbox` / `=1`. Domain grants (`--allow-*` / `SYX_ALLOW_*` / TOML) unchanged.  
   **≠** OS isolation / macro seccomp / full HostFn GA / C-5 `[x]`.

4. **Seccomp / rlimit / Job Object / Seatbelt scope honesty**  
   Wired subset = Linux **net-deny** + Linux **allowlist thin** + Linux RLIMIT_AS + Windows Job Object + macOS Seatbelt (all opt-in).  
   Allowlist ≠ full syscall GA (supplements documented; expand/print under allowlist not guaranteed for all loads).  
   Probe strings `embed_v*_seccomp_probe` remain informational.

## Non-claims

- ≠ full seccomp allowlist GA / App Sandbox GA / WASI / cgroup  
- ≠ OS sandbox from permission-sandbox alone  
- ≠ C-5 `[x]` after C-5a…C-5h alone (true full syscall GA still park)  
- ≠ true guest heap metering  
- ≠ macOS RSS hard wall

## Related

- Roadmap: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) C-5 / C-5h  
- Census: [`c-5h-permission-default-on-census`](../census/README.md)
