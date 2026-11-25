# SYX environment variable registry (Phase 37)

Audit of `SYX_*` variables after GB config LTS. Status: **cli** (replaced by flag), **ci** (internal), **removed**, **deprecated** (fallback one season).

| Variable | Status | Replacement / notes |
|----------|--------|---------------------|
| `SYX_JIT_TIER2` | deprecated | `syx --jit` / `--no-jit` (opt-in; **default off**; experimental Suite-1) |
| `SYX_JIT_FASTPATH` | deprecated | `syx --jit-fastpath` |
| `SYX_LLVM_AOT` | deprecated | `syx --llvm-aot` |
| `SYX_EMBED_SANDBOX` | deprecated | `syx --embed-sandbox` (soft HTTP deny; **not** OS isolation — [threat-model](adr/macro-embed-threat-model.md)) |
| `SYX_EMBED_WASM_SANDBOX` | opt-in | wasmtime path; failure/unavailable → `syx_embed_v2_run` **null** (C-5b; ≠ OS isolation) |
| `SYX_EMBED_SECCOMP` | wired-subset-linux | Linux net-deny BPF in worker; non-Linux → null (C-5c-2); mutually exclusive with `SYX_EMBED_SECCOMP_ALLOWLIST` |
| `SYX_MACRO_SECCOMP` | wired-subset-linux | Linux child net-deny BPF; mode `seccomp-netdeny-wired`; non-Linux → M040 (C-5c-2); mutually exclusive with ALLOWLIST |
| `SYX_EMBED_SECCOMP_ALLOWLIST` | wired-subset-linux | Linux default-deny allowlist BPF (v11+supplements) in worker; non-Linux → null; mutex with SECCOMP (C-5g); ≠ full syscall GA |
| `SYX_MACRO_SECCOMP_ALLOWLIST` | wired-subset-linux | Linux child default-deny allowlist; non-Linux → M040; mutex with SECCOMP (C-5g) |
| `SYX_EMBED_TIMEOUT_MS` | enforced-when-set | hard wall (env wins over `syx --embed-timeout-ms`); in-process null / wasmtime kill (C-5c-1) |
| `SYX_MACRO_TIMEOUT_MS` | opt-in | macro subprocess wall clock; timeout → kill + M040 (C-5c-1); unset = no limit |
| `SYX_EMBED_MEMORY_MB` | soft-when-set | env wins over `syx --embed-memory-mb`; stdout-sum soft gate (C-5d honest; ≠ true heap) |
| `SYX_EMBED_RSS_MB` | wired-subset | Linux `prlimit(RLIMIT_AS)` · Windows Job Object `PROCESS_MEMORY_LIMIT` in worker / wasmtime child; macOS/other → null (C-5d/C-5e) |
| `SYX_MACRO_RSS_MB` | wired-subset | Linux child `prlimit` · Windows parent Job Object assign; over-limit → M040; macOS/other → M040 (C-5d/C-5e) |
| `SYX_EMBED_MACOS_SANDBOX` | wired-subset-macos | macOS: wasmtime under `sandbox-exec -n no-network`; in-process → null; non-macOS → null (C-5f); ≠ App Sandbox GA |
| `SYX_MACRO_MACOS_SANDBOX` | wired-subset-macos | macOS child `sandbox_init(SANDBOX_NAMED, no-network)`; non-macOS → M040 (C-5f) |
| `SYX_PERMISSION_SANDBOX` | default-on (C-5h) | unset/`1` → sandbox **on**; `0`/`false` → opt-out (`--no-permission-sandbox`); still ≠ OS isolation |
| `SYX_ALLOW_NET` | deprecated | `syx --allow-net` (M3-E02 / ADR-310) |
| `SYX_ALLOW_FS` | deprecated | `syx --allow-fs` (M3-E02 / ADR-311) |
| `SYX_ALLOW_ENV` | deprecated | `syx --allow-env` (W1035 / ADR-1379) |
| `SYX_ALLOW_PROCESS` | deprecated | `syx --allow-process` (W1035 / ADR-1379) |
| `SYX_ALLOW_PLUGIN` | deprecated | `syx --allow-plugin` (W1036 / ADR-1380) |
| `SYX_IR_CAPABILITY_GATE` | deprecated | `syx --ir-capability-gate` (M3-E03 / ADR-313) |
| `SYX_BACKEND` | deprecated | `syx --backend cranelift\|llvm` |
| `SYX_CRANELIFT_P19` | removed | P20 CI default (Phase 37 GD) |
| `SYX_CRANELIFT_P6`–`P18` | removed | minimal + P20 gate only |
| `SYX_LLVM_AOT_PHASE2`–`PHASE10` | removed | `llvm_aot_phase12_enabled()` internal |
| `SYX_LLVM_AOT_PHASE11` | ci | LLVM phase cascade |
| `SYX_LLVM_AOT_PHASE12` | ci | Phase 37 AOT gate |
| `SYX_MIR_OPT_V17`–`V24` | removed | v25 barrier only |
| `SYX_EMBED_V12`–`V21` | removed | unified `EmbedConfig` |
| `SYX_LLVM_STRICT_GREEN_WEEKS` | ci | inkwell/LLVM GA scaffolds |
| `SYX_LLVM_PERF_GREEN_SEASONS` | ci | perf release GA |
| `SYX_NIGHTLY` | ci | nightly blocker scaffold |
| `SYX_CI_PR` | ci | PR blocker scaffold |
| `SYX_CRANELIFT_P20` | deprecated | P20 prod default via config |
| `SYX_CRANELIFT_MINIMAL` | ci | `--minimal` build flag |
| `SYX_USE_MIR` | ci | MIR pipeline tests |
| `SYX_LLVM_PARITY_GATE` | ci | strict parity in CI |

Target: ≤40 read sites post-Phase-37 (down from ~200+).
