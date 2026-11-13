---
title: 安装 SYX
description: 安装 SYX 工具链 —— 从 GitHub Releases 下载预编译 syx 二进制，或用 Rust 1.85+ 从源码构建；原生 AOT 需要 LLVM。
lastUpdated: 2026-11-13
---
# 安装 SYX

仅仅*运行* SYX 程序**不需要**先装 Rust。发布版提供预编译的 `syx` 二进制。从源码构建工具链需要 **Rust 1.85+**；完整原生（LLVM）工具链还需要本地 LLVM。

## 方案 A —— 预编译发布版（推荐）

1. 打开最新发布页：[github.com/synvek/syx/releases/latest](https://github.com/synvek/syx/releases/latest)
2. 下载对应操作系统 / 架构的压缩包
3. 将 `syx` 放入 `PATH`
4. 验证：

```bash
syx --version
syx run path/to/hello.sy
```

## 方案 B —— 从源码构建

```bash
git clone https://github.com/synvek/syx.git
cd syx
cargo build --release -p syx-cli
./target/release/syx run examples/hello.sy
```

可选安装到 Cargo bin 目录：

```bash
cargo install --path crates/syx-cli
```

原生 AOT（`syx build --native`）需要 full-native / LLVM 特性集 —— 细节见[快速指南](/zh/quick-guide/)。

## 编辑器支持

安装 VS Code 的 **SYX Language** 扩展（TextMate + LSP + DAP）。见[编辑器](/zh/editors/)与 [VS Code](/zh/editors/vscode)。

## 下一步

- [源文件后缀 (.sy)](./file-extension) 与 `syx.toml`
- [后端](./backends) —— VM、LLVM、Wasm
- 语言指南中的[入门](/zh/guide/01-getting-started)
