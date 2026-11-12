---
layout: home
title: SYX 语言
titleTemplate: false
description: SYX 是一门受 Swift 启发、由 Rust 实现编译器工具链的静态类型编程语言，统一 MIR 流水线支持栈式 VM、LLVM 原生 AOT 与 Wasm 三条后端。
hero:
  name: SYX
  text: 受 Swift 启发 · 由 Rust 驱动
  tagline: 一门现代编程语言，配套 VM、LLVM 原生 AOT 与 Wasm —— 源文件扩展名 .sy
  image:
    src: /syx-lang.png
    alt: SYX 语言示例 — Greeter 类、扩展与 main
  actions:
    - theme: brand
      text: 快速入门
      link: /zh/guide/01-getting-started
    - theme: brand
      text: 下载
      link: https://github.com/synvek/syx/releases/latest
    - theme: alt
      text: 语言指南
      link: /zh/guide/00-preface
    - theme: alt
      text: 语法速查
      link: /zh/quick-reference/
    - theme: alt
      text: VS Code
      link: /zh/editors/vscode
features:
  - title: 熟悉的语法
    details: Swift 风格的 func / let / var、协议、泛型、async/await —— 并诚实标注与 Swift 的语义差异。
  - title: 三条后端
    details: 栈式 VM 快速迭代（syx run）、LLVM 原生 AOT、Wasm 生成与运行 —— 统一 MIR 流水线。
  - title: VS Code + LSP
    details: 语法高亮、成员/import 补全、悬停、行内类型提示、保存格式化，以及基于 syx lsp 的 DAP 调试。
  - title: 嵌入标准库
    details: Math、IO、Net、Crypto、Os、Sync、Linalg 等可 import 包，边界文档齐全。
---

## SYX 是什么

**SYX**（读作 *sy-ex*，`/saɪˈɛks/`）是一门静态强类型的编程语言，语法受 Swift 启发，编译器工具链由 Rust 实现。它面向喜欢 Swift 的 `func` / `let` / `var` 风格、协议、泛型与 `async` / `await`，同时希望用一套工具链同时产出字节码 VM、原生代码与 WebAssembly 的开发者。

| 项目 | 内容 |
|------|------|
| 类型系统 | 静态强类型，多范式 |
| 语法来源 | 受 Swift 启发；语义独立 |
| 实现语言 | Rust 工作区（`syx-lexer`、`syx-parser`、`syx-semantic`、`syx-mir` 等） |
| 源文件 | `.sy`（UTF-8） |
| 项目清单 | `syx.toml` |
| 工具链二进制 | `syx`（`syx-cli`） |
| 后端 | 栈式 VM（`syx run`）、LLVM 原生 AOT（`syx build --native`）、Wasm（`syx build --wasm`） |
| 语言服务器 | `syx lsp` |
| 许可证 | Apache-2.0 |

SYX **不是** Swift 的子集或超集。许多名称相同，但语义可能差异很大 —— 特别是 `String` 字符语义、内存模型、宏、并发与跨模块 Actor。

## Hello, World

```sy
func main() {
    print("Hello, World")
}
```

```bash
cargo build --release -p syx-cli
./target/release/syx run examples/hello.sy
```

## 接下来读哪里

| 路径 | 适合谁 |
|------|--------|
| [快速指南](/zh/quick-guide/) | 安装、CLI、项目、Native / Wasm |
| [语法速查](/zh/quick-reference/) | 语法与标准库速查 |
| [编辑器 · VS Code](/zh/editors/vscode) | 语法高亮、补全、LSP、DAP |
| [语言指南](/zh/guide/00-preface) | 从入门到高级特性系统学习 |
| [语言参考](/zh/reference/00-about-this-reference) | 词法、语法、类型的形式化参考 |
| [标准库](/zh/library/00-preface) | 嵌入包：Linalg、Os、Log、IO、Net… |
| [编译器内部](/zh/compiler/) | 词法 → 语法 → 语义 → MIR → 后端 |
| [现行能力](/zh/about/capabilities) | 今天已经实现了什么（诚实边界） |

## 常见问题

### SYX 和 Swift 是一样的吗？

不是。SYX 借用了 Swift 的表层语法，但是一门独立语言，拥有自己的语义、标准库与工具链。它明确既不是 Swift 的子集也不是超集；具体差异见[语言指南前言](/zh/guide/00-preface)。

### SYX 有哪几条后端？

三条，全部由同一条 MIR 流水线驱动：栈式字节码 VM（`syx run`）、基于 LLVM 的原生 AOT（`syx build --native`），以及 WebAssembly（`syx build --wasm` / `syx run --wasm`）。LLVM 是唯一的原生 AOT 后端。

### SYX 的源文件后缀与项目结构是什么？

源文件后缀为 `.sy`。项目通过根目录的 `syx.toml` 声明，包含 `name`、`version`、`main` 等字段；模块通过 `import` 引入。

### 使用 SYX 必须先装 Rust 吗？

运行不必 —— 发布版提供预编译二进制。从源码构建工具链需要 Rust 1.85 或更高版本；构建完整原生工具链还需要本地 LLVM。

### 如何获得编辑器支持？

安装 **SYX Language** VS Code 扩展即可获得 TextMate 语法高亮以及 LSP 能力 —— 补全、悬停、跳转定义、行内类型提示、保存格式化，并通过 `syx lsp` 支持 DAP 调试。仓库同时提供 Neovim 与 Helix 配置，见[编辑器](/zh/editors/)。

> 文档从仓库 `books/` 与 `docs/` 同步。可通过导航栏语言选择器切回 **English**。

