# 编译器内部

面向 Rust SYX 编译器工具链（`crates/`）的技术文档。

```
源码 (.sy)
  → 词法 → 语法 → 语义 → MIR
                        ├→ VM
                        ├→ LLVM 原生
                        └→ Wasm
```

| 文档 | 说明 |
|------|------|
| [词法分析](./lexical-analysis) | `syx-lexer` |
| [语法分析](./syntax-analysis) | `syx-parser` + `syx-ast` |
| [语义分析](./semantic-analysis) | `syx-semantic` + `syx-stdlib` |
| [指令集](./instruction-set) | `syx-codegen` + `syx-vm` |
| [完整技术文档](./technical-overview) | Crate 总览与流水线 |
| [后端对等](./backend-parity) | VM / MIR / LLVM / Wasm |
| [网络标准库](./network-stdlib) | 网络相关包 |
| [后端调试](./backend-debugging) | 后端调试指南 |

日常使用见 [快速指南](/zh/quick-guide/)。架构决议见 [关于](/zh/about/architecture)。
