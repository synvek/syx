# Macro / embed threat model (C-5a · C-5c-1 · C-5c-2 · C-5d · C-5e · C-5f)

**Status:** living SSOT for macro/embed “sandbox” honesty  
**Date:** 2026-11-25  
**Axis:** C-5a + C-5b + C-5c-1 + C-5c-2 + C-5d + C-5e + C-5f (≠ C-5 exited; next default = **C-5g**)

## Decision

Public claims must not describe macro re-exec or embed soft flags as **security isolation** / OS sandbox / hardening GA. Use this document as the threat-model SSOT.

## Threat model (four pins)

1. **Macro default = crash-containment reexec only**  
   External macros expand in a child process that re-execs the same `syx` binary (`SandboxMacroExpander`). This limits crash blast radius. It is **not** a permission sandbox: no fs/net/env/cwd deny by default, no memory hard limit by default, stderr discarded. Mode label: `sandbox-reexec-only`.  
   **C-5c-1:** when `SYX_MACRO_TIMEOUT_MS` is set, the parent applies a wall-clock deadline and **kills** the child on timeout (fail-closed **M040**). Unset → no timeout.  
   **C-5c-2:** when `SYX_MACRO_SECCOMP=1` on **Linux**, the child installs a **net-deny seccomp-BPF** (deny `socket`/`connect`/`bind`/`listen`; default-allow otherwise) before expand; install failure → **M040**. Mode → `seccomp-netdeny-wired`. Non-Linux + env → parent fail-closed **M040**.  
   **C-5d / C-5e:** when `SYX_MACRO_RSS_MB` is set: **Linux** child installs **`prlimit(RLIMIT_AS)`**; **Windows** parent assigns child to a Job Object with **`PROCESS_MEMORY_LIMIT`**; over-limit → **M040**. macOS / other + env → **M040**.  
   **C-5f:** when `SYX_MACRO_MACOS_SANDBOX=1` on **macOS**, the child installs Seatbelt **net-deny** via `sandbox_init(SANDBOX_NAMED, "no-network")` before expand; install failure → **M040**. Mode → `seatbelt-netdeny-wired`. Non-macOS + env → parent fail-closed **M040**.

2. **Embed flags = HTTP deny / optional wasmtime + opt-in hard timeout + opt-in seccomp + opt-in RSS hard wall + opt-in macOS Seatbelt**  
   `--embed-sandbox` / `SYX_EMBED_SANDBOX` mainly sets `native_no_http`. Optional wasmtime is not isolation GA.  
   **C-5b:** when `SYX_EMBED_WASM_SANDBOX=1`, wasm failure/unavailable → `syx_embed_v2_run` **null**.  
   **C-5c-1:** when `SYX_EMBED_TIMEOUT_MS` or CLI `--embed-timeout-ms` is set (env wins), timeout is a **hard wall**.  
   **C-5c-2:** when `SYX_EMBED_SECCOMP=1` on **Linux**, install net-deny BPF in the **worker thread**; Non-Linux + env → **null**.  
   **C-5d / C-5e:** when `SYX_EMBED_RSS_MB` is set: **Linux** → `prlimit`; **Windows** → Job Object; macOS / other → **null**.  
   **C-5f:** when `SYX_EMBED_MACOS_SANDBOX=1`: **macOS** wasmtime child is launched under **`sandbox-exec -n no-network`**; **in-process** path → **null** (Seatbelt is process-wide; no silent host install). Non-macOS + env → **null**.  
   **MEMORY:** `SYX_EMBED_MEMORY_MB` (env wins) ∪ CLI `--embed-memory-mb` remains a **stdout-sum soft gate**.

3. **`--permission-sandbox` = opt-in HostFn S024**  
   Compile-time HostFn domain gate (net/fs/env/process/plugin). **Not** OS process isolation. Default remains **off**.

4. **Seccomp / rlimit / Job Object / Seatbelt scope honesty**  
   Wired subset = **Linux net-deny** + **Linux RLIMIT_AS** + **Windows Job Object process-memory** + **macOS Seatbelt net-deny** (opt-in). ≠ full syscall GA, ≠ App Sandbox container GA, ≠ purebox. Probe strings `embed_v*_seccomp_probe` / `macos-sandbox-*-degraded` stay **informational** (≠ claim full GA). Full Linux allowlist filter remains park → **C-5g**.

## Non-claims

- ≠ full seccomp allowlist GA / full Job Object GA / App Sandbox GA / WASI / cgroup  
- ≠ true guest heap metering (MEMORY = stdout-sum soft)  
- ≠ permission-sandbox default-on  
- ≠ C-5 `[x]` after C-5a…C-5f alone  
- ≠ in-process orphan-thread kill  
- ≠ macOS RSS hard wall (`SYX_*_RSS_MB` on macOS still fail-closed)

## Related

- Historical table: [`architecture-decisions.md`](architecture-decisions.md) (External macros row)  
- Roadmap: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) C-5 / C-5f  
- Census: [`c-5a`](../census/c-5a-threat-model-census.md) · [`c-5b`](../census/c-5b-embed-wasm-fail-closed-census.md) · [`c-5c-1`](../census/c-5c-1-embed-macro-timeout-census.md) · [`c-5c-2`](../census/c-5c-2-seccomp-wire-census.md) · [`c-5d`](../census/c-5d-embed-rss-hard-census.md) · [`c-5e`](../census/c-5e-jobobject-thin-census.md) · [`c-5f-macos-sandbox-thin-census`](../census/c-5f-macos-sandbox-thin-census.md)
