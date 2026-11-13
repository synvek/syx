---
title: SYX 后端（VM、LLVM、Wasm）
description: SYX 由一条 MIR 流水线驱动三条后端 —— 栈式字节码 VM（syx run）、LLVM 原生 AOT（syx build --native）、以及 WebAssembly（syx build --wasm / syx run --wasm）。
lastUpdated: 2026-11-13
---
# SYX 后端（VM、LLVM、Wasm）

SYX 用**一条 MIR 流水线**驱动**三条**执行 / 产出后端。LLVM 是唯一的**原生 AOT** 后端。

| 后端 | 典型命令 | 角色 |
|------|----------|------|
| 栈式字节码 VM | `syx run` | 快速迭代 / 默认解释路径 |
| LLVM 原生 AOT | `syx build --native` | 原生机器码 |
| WebAssembly | `syx build --wasm` / `syx run --wasm` | Wasm 产出与运行 |

```text
.sy → … → MIR ─┬→ VM 字节码
               ├→ LLVM 目标文件 / 原生链接
               └→ Wasm
```

## 如何选择

- **日常学习与脚本：** VM（`syx run`）
- **原生二进制：** LLVM AOT（需要原生/LLVM 工具链 —— 见[安装](./install)）
- **浏览器 / Wasm 宿主：** `--wasm`

后端对等与调试说明见[编译器内部](/zh/compiler/)（[后端对等](/zh/compiler/backend-parity)、[后端调试](/zh/compiler/backend-debugging)）。

## 相关

- [SYX 是什么？](./what-is-syx)
- [现行能力](./capabilities) —— 诚实能力边界
- [快速指南](/zh/quick-guide/) —— 构建选项与项目
