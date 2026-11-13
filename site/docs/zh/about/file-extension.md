---
title: SYX 源文件后缀 (.sy)
description: SYX 源文件使用 .sy 后缀（UTF-8）。项目通过 syx.toml 清单声明（name、version、main），模块用 import 引入。
lastUpdated: 2026-11-13
---
# SYX 源文件后缀（`.sy`）

SYX 源文件使用 **`.sy`** 后缀，编码为 UTF-8。多文件项目在仓库根目录用 **`syx.toml`** 清单声明。

## 最小布局

```text
my-app/
  syx.toml
  src/
    main.sy
```

`syx.toml` 示例字段（完整 schema 见[快速指南](/zh/quick-guide/)）：

```toml
name = "my-app"
version = "0.1.0"
main = "src/main.sy"
```

模块通过 `import` 引入。工具链二进制为 `syx`（`syx-cli`）。

## 对照表

| 项目 | 取值 |
|------|------|
| 源文件后缀 | `.sy` |
| 清单 | `syx.toml` |
| 运行 | `syx run file.sy` |
| 语言服务 | `syx lsp` |

## 相关

- [安装 SYX](./install)
- [SYX 是什么？](./what-is-syx)
- [快速指南](/zh/quick-guide/) —— CLI、项目、Native / Wasm
