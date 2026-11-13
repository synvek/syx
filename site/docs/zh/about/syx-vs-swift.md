---
title: SYX 与 Swift
description: SYX 不是 Swift —— 借用了熟悉的 Swift 风格语法，但拥有独立语义、标准库与 Rust 工具链，既不是子集也不是超集。
lastUpdated: 2026-11-13
---
# SYX 与 Swift

**一句话：** SYX **不是** Swift。它借用了 Swift 风格的表层语法（`func`、`let` / `var`、协议、泛型、`async` / `await`），让上手更熟悉，但本身是一门**独立**语言，有自己的语义、标准库，以及 **Rust 实现**的工具链。

| | Swift | SYX |
|---|---|---|
| 定义方 | Apple / Swift.org | SYX Language Project |
| 编译器 | Swift 编译器（C++/Swift） | Rust 工作区（`syx-*` crate） |
| 源文件后缀 | `.swift` | `.sy` |
| 项目文件 | `Package.swift`（SPM） | `syx.toml` |
| 主要运行时 | Swift runtime 等 | 栈式 VM、LLVM 原生 AOT、Wasm |
| 关系 | — | 语法受启发；**不是**方言 |

## 常见语义分歧

名字常相同而含义不同 —— 尤其是：

- `String` / 字符下标与 Unicode 处理
- 内存模型与所有权预期
- 宏与 result builder 类特性
- 并发与跨模块 actor

差异的章节级导览见[语言指南前言](/zh/guide/00-preface)。若问「今天是否支持 *X*？」，请看[现行能力](./capabilities)。

## 相关

- [SYX 是什么？](./what-is-syx)
- [后端（VM / LLVM / Wasm）](./backends)
- [安装 SYX](./install)
