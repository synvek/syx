# SYX 环境变量登记表（Phase 37）

GB 配置 LTS 之后对 `SYX_*` 变量的清点。状态含义：**cli**（已被命令行参数取代）、**ci**（内部使用）、**removed**（已移除）、**deprecated**（已弃用，保留一个季节的回退）。

## 已弃用（请改用命令行参数）

| 变量 | 状态 | 替代 / 说明 |
|------|------|-------------|
| `SYX_JIT_TIER2` | deprecated | `syx --jit` / `--no-jit`（默认开启） |
| `SYX_JIT_FASTPATH` | deprecated | `syx --jit-fastpath` |
| `SYX_LLVM_AOT` | deprecated | `syx --llvm-aot` |
| `SYX_EMBED_SANDBOX` | deprecated | `syx --embed-sandbox` |
| `SYX_EMBED_TIMEOUT_MS` | deprecated | `syx --embed-timeout-ms` |
| `SYX_EMBED_MEMORY_MB` | deprecated | `syx --embed-memory-mb` |
| `SYX_PERMISSION_SANDBOX` | deprecated | `syx --permission-sandbox`（M3-E02 / ADR-310） |
| `SYX_ALLOW_NET` | deprecated | `syx --allow-net`（M3-E02 / ADR-310） |
| `SYX_ALLOW_FS` | deprecated | `syx --allow-fs`（M3-E02 / ADR-311） |
| `SYX_ALLOW_ENV` | deprecated | `syx --allow-env`（W1035 / ADR-1379） |
| `SYX_ALLOW_PROCESS` | deprecated | `syx --allow-process`（W1035 / ADR-1379） |
| `SYX_ALLOW_PLUGIN` | deprecated | `syx --allow-plugin`（W1036 / ADR-1380） |
| `SYX_IR_CAPABILITY_GATE` | deprecated | `syx --ir-capability-gate`（M3-E03 / ADR-313） |
| `SYX_BACKEND` | deprecated | `syx --backend cranelift\|llvm` |
| `SYX_CRANELIFT_P20` | deprecated | 通过配置使用 P20 生产默认 |

## 已移除

| 变量 | 状态 | 说明 |
|------|------|------|
| `SYX_CRANELIFT_P19` | removed | P20 为 CI 默认（Phase 37 GD） |
| `SYX_CRANELIFT_P6`–`P18` | removed | 仅保留 minimal + P20 闸门 |
| `SYX_LLVM_AOT_PHASE2`–`PHASE10` | removed | 由内部 `llvm_aot_phase12_enabled()` 取代 |
| `SYX_MIR_OPT_V17`–`V24` | removed | 仅保留 v25 屏障 |
| `SYX_EMBED_V12`–`V21` | removed | 统一为 `EmbedConfig` |

## 仅供 CI

| 变量 | 状态 | 说明 |
|------|------|------|
| `SYX_LLVM_AOT_PHASE11` | ci | LLVM 阶段级联 |
| `SYX_LLVM_AOT_PHASE12` | ci | Phase 37 AOT 闸门 |
| `SYX_LLVM_STRICT_GREEN_WEEKS` | ci | inkwell / LLVM GA 脚手架 |
| `SYX_LLVM_PERF_GREEN_SEASONS` | ci | 性能发布 GA |
| `SYX_NIGHTLY` | ci | nightly 阻断脚手架 |
| `SYX_CI_PR` | ci | PR 阻断脚手架 |
| `SYX_CRANELIFT_MINIMAL` | ci | `--minimal` 构建开关 |
| `SYX_USE_MIR` | ci | MIR 流水线测试 |
| `SYX_LLVM_PARITY_GATE` | ci | CI 中的严格后端对等 |

目标：Phase 37 之后读取点不超过 40 处（原先约 200+）。

环境变量的日常用法见[快速指南](/zh/quick-guide/)；后端差异见[后端对等](/zh/compiler/backend-parity)。
