# SYX environment variable registry (Phase 37)

Audit of `SYX_*` variables after GB config LTS. Status: **cli** (replaced by flag), **ci** (internal), **removed**, **deprecated** (fallback one season).

| Variable | Status | Replacement / notes |
|----------|--------|---------------------|
| `SYX_JIT_TIER2` | deprecated | `syx --jit` / `--no-jit` (default on) |
| `SYX_JIT_FASTPATH` | deprecated | `syx --jit-fastpath` |
| `SYX_LLVM_AOT` | deprecated | `syx --llvm-aot` |
| `SYX_EMBED_SANDBOX` | deprecated | `syx --embed-sandbox` |
| `SYX_EMBED_TIMEOUT_MS` | deprecated | `syx --embed-timeout-ms` |
| `SYX_EMBED_MEMORY_MB` | deprecated | `syx --embed-memory-mb` |
| `SYX_PERMISSION_SANDBOX` | deprecated | `syx --permission-sandbox` (M3-E02 / ADR-310) |
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
