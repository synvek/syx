# SYX 编译器技术文档

面向 **Rust 实现的 SYX 编译器工具链**（`crates/`）。用户安装/CLI/标准库请优先读 [9_UsageGuide.md](./9_UsageGuide.md) 与 [books/](../books/)。

> **当前版本：** SYX **117.0** · Phase **139** · cache `syx-cache-v1` · audit driver ≥2600 / LSP ≥3050 · 许可证 [Apache-2.0](../LICENSE)  
> **原生 AOT：** LLVM sole · **实现状态 SSOT：** [books/shared/implementation-status.md](../books/shared/implementation-status.md)  
> **架构决议 SSOT：** [adr/architecture-decisions.md](./adr/architecture-decisions.md)

## 文档地图

### 用户

| 文档 | 说明 |
|------|------|
| [9_UsageGuide.md](./9_UsageGuide.md) | 安装、CLI、emit、项目配置、Native/WASM、并发控制面 |
| [8_QuickReference.md](./8_QuickReference.md) | 语法速查 |
| [books/language-guide/](../books/language-guide/) | 叙事式语言指南 |
| [books/language-reference/](../books/language-reference/) | 形式化语言参考 |
| [books/README.md](../books/README.md) | 电子书索引与构建 |

### 编译器流水线

| # | 文档 | 说明 |
|---|------|------|
| 1 | [1_LexicalAnalysis.md](./1_LexicalAnalysis.md) | `syx-lexer` |
| 2 | [2_SyntaxAnalysis.md](./2_SyntaxAnalysis.md) | `syx-parser` + `syx-ast` |
| 3 | [3_SemanticAnalysis.md](./3_SemanticAnalysis.md) | `syx-semantic` + `syx-stdlib` |
| 4 | [4_InstructionSet.md](./4_InstructionSet.md) | `syx-codegen` + `syx-vm` |
| 5 | [5_CompleteTechnicalDocumentation.md](./5_CompleteTechnicalDocumentation.md) | crate 总览与四条流水线 |
| 10 | [10_BackendParity.md](./10_BackendParity.md) | VM / MIR / LLVM / WASM |
| 11 | [11_NetworkStdlib.md](./11_NetworkStdlib.md) | 网络标准库 |
| 12 | [12_BackendDebuggingGuide.md](./12_BackendDebuggingGuide.md) | 后端调试 |

### 参考

| 文档 | 说明 |
|------|------|
| [env-registry.md](./env-registry.md) | 环境变量登记 |
| [syx-mir-text-v1.md](./syx-mir-text-v1.md) | MIR 文本格式 |
| [syxbc-text-v1.md](./syxbc-text-v1.md) | SYXBC 文本格式 |
| [wasm_native_opcode_matrix.md](./wasm_native_opcode_matrix.md) | WASM/native opcode 矩阵 |
| [capability-manifest-v0.md](./capability-manifest-v0.md) · [ir-capability-manifest-v0.md](./ir-capability-manifest-v0.md) · [capability-sandbox-policy-v0.md](./capability-sandbox-policy-v0.md) | capability |
| [adr/README.md](./adr/README.md) | 架构决议入口（仅 `architecture-decisions.md`） |
| [differential/](./differential/) | oracle / differential 说明 |
| [test-buckets.md](./test-buckets.md) | 测试分桶 |

### 分析（历史叙述；现状以 status 为准）

| 文档 | 说明 |
|------|------|
| [6_ComprehensiveAnalysisReport.md](./6_ComprehensiveAnalysisReport.md) | 竞争力分析（条幅可能过期） |
| [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) | 功能优先级（请交叉核对 status） |

并发专题已并入 UsageGuide / [18-concurrency](../books/language-guide/18-concurrency.md)；旧 [9_ConcurrencyMVP.md](./9_ConcurrencyMVP.md) 仅为重定向 stub。

---

## 阅读路径

- **学语言：** books/language-guide → UsageGuide → QuickReference  
- **读编译器：** 1 → 2 → 3 → 4 → 5 → BackendParity  
- **查决议/状态：** architecture-decisions · implementation-status  

## 仓库布局（摘要）

```
crates/              # 编译器 / 运行时 / LSP / 工具
packages/            # 标准库与内部清单
docs/                # 本目录
books/               # 用户电子书
plan/                # 状态分析与改进清单（SSOT）；topics / archive
examples/            # 示例程序
```

## 工具

- Rust 1.85+ · Cargo workspace · macOS / Linux / Windows

## 当前版本（SYX 117.0）

- **语言 / phase：** 117.0 / **139** · MIR cache `syx-cache-v1`
- **架构决议：** [adr/architecture-decisions.md](./adr/architecture-decisions.md)
- **Native：** LLVM only
- **Audit：** driver ≥2600 · LSP ≥3050（见 `scripts/audit_behavioral_tests.sh`）
- **实现状态：** [books/shared/implementation-status.md](../books/shared/implementation-status.md)
