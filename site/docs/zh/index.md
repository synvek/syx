---
layout: home
title: SYX 语言
hero:
  name: SYX
  text: 受 Swift 启发 · 由 Rust 驱动
  tagline: 一门现代编程语言，配套 VM、LLVM 原生 AOT 与 Wasm —— 源文件扩展名 .sy
  actions:
    - theme: brand
      text: 快速入门
      link: /zh/guide/01-getting-started
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

> 文档从仓库 `books/` 与 `docs/` 同步。可通过导航栏语言选择器切回 **English**。
