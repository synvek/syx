> **SYX 117.0（Phase 139）** — VM · MIR · LLVM · WASM 后端 parity。ManagedHeap + ObjectRef；WASM SIMD guest-v22；Native **LLVM sole**。见 [architecture-decisions.md](../adr/architecture-decisions.md) · [implementation-status](../../books/shared/implementation-status.md)。

# SYX 后端 Parity（VM · MIR/WASM · LLVM）

[English](../10_BackendParity.md)


后端特性 parity 的**文档入口**（行为以 `crates/syx-driver/tests` 与 CI 为准）。

主 CI（`.github/workflows/ci.yml`）大致运行：

```bash
cargo test -p syx-driver \
  --test evergreen --test lang --test concurrency --test stdlib \
  --test observability --test security --test backends \
  -- --test-threads=1
cargo test -p syx-lsp
# + capability freshness / tip-leak gates（见 workflow）
```

Evergreen 内含 `parity_native` 等；历史独立 `parity_vm` / `parity_wasm` 套件名以仓库当前 `tests/` 布局为准。

## 三后端矩阵

| 维度 | VM（`syx-vm`） | WASM（`syx-wasm-run`） | Native LLVM（`syx-llvm` + `syx-rt`） |
|------|----------------|------------------------|--------------------------------------|
| 入口 | `syx run` | `syx run --wasm` / `syx build --wasm` | `syx build --native` |
| 角色 | 语义基准 / 日常开发 | 沙箱部署（CLI 近完整 stdlib） | **唯一**原生 AOT |
| 中端 | 统一 MIR | 统一 MIR | 统一 MIR |

## Native 后端（LLVM only）

- `syx build --native` → LLVM AOT；**无** Cranelift AOT（`syx-cranelift` 已退役）。
- 实验 JIT：`syx-cranelift-jit` + `SYX_JIT_TIER2=1`（Suite-1；默认关；≠ 生产默认）。
- Object / 链接：`--object`、`--release`、`full-native` 见 [9_UsageGuide.md](./9_UsageGuide.zh-CN.md)。
- ObjectRef / Host / 插件边界见 [`docs/adr/architecture-decisions.md`](../adr/architecture-decisions.md)。

## 审计与 fixture 策略

- 行为套件：`crates/syx-driver/tests/{lang,stdlib,native,concurrency,evergreen,backends,…}`
- 共享 fixture：`crates/syx-driver/tests/fixtures/`（若存在）
- 能力表：[`books/shared/capabilities-generated.md`](../../books/shared/capabilities-generated.md)

## 相关文档

- [架构决议](../adr/architecture-decisions.md)
- [后端调试指南](./12_BackendDebuggingGuide.zh-CN.md)
- [完整技术文档](./5_CompleteTechnicalDocumentation.zh-CN.md)
- [使用指南 · 编译与后端](./9_UsageGuide.zh-CN.md)
