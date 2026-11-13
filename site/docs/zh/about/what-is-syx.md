---
title: SYX 是什么？
description: SYX 是一门静态类型、受 Swift 启发的编程语言，配套 Rust 实现的编译器工具链 —— 源文件 .sy、项目清单 syx.toml，一条 MIR 流水线支持 VM / LLVM / Wasm。
lastUpdated: 2026-11-13
---
# SYX 是什么？

**SYX**（读作 *sy-ex*，`/saɪˈɛks/`）是一门**静态类型、受 Swift 启发**的编程语言，编译器工具链由 Rust 实现。面向喜欢 Swift 风格 `func` / `let` / `var`、协议、泛型与 `async` / `await` 的开发者，同时提供可在字节码 VM 上运行、编译为原生代码、以及输出 WebAssembly 的单一开源工具链。

| 事实 | 取值 |
|------|------|
| 类型 | 编程语言 + 编译器工具链 |
| 语法血统 | 受 Swift 表层启发；**语义独立** |
| 实现 | Rust（`syx-lexer`、`syx-parser`、`syx-semantic`、`syx-mir` 等） |
| 源文件 | `.sy`（UTF-8） |
| 项目清单 | `syx.toml` |
| CLI | `syx`（`syx-cli`） |
| 后端 | 栈式 VM、LLVM 原生 AOT、WebAssembly |
| 许可证 | Apache-2.0 |

SYX **既不是** Swift 的子集，**也不是**超集。名字相近不代表行为相同 —— 见 [SYX 与 Swift](./syx-vs-swift)。

## 一分钟心智模型

```text
.sy 源码 → 词法 → 语法 → 语义 → MIR → VM | LLVM | Wasm
```

- **学语言：** [语言指南](/zh/guide/00-preface)
- **安装与运行：** [安装 SYX](./install)
- **诚实能力边界：** [现行能力](./capabilities)
