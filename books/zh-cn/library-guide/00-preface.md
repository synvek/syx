# 前言

欢迎阅读 **《SYX 标准库指南》**。语言语法见 [语言指南](../language-guide/SUMMARY.md)；形式化规则见 [语言参考](../language-reference/SUMMARY.md)。本书只讲 **可 `import` 的嵌入包**：路径、诚实边界、以及仓库里能对照的范例。

> **本版对应：** SYX **117.0** / Phase **139** · HostFn ≈ **630** · 嵌入清单见 [`packages/internal/manifest.json`](../../../packages/internal/manifest.json) · 包概览 [`packages/README.md`](../../../packages/README.md)

## 和语言指南的分工

| 你要… | 去哪 |
|------|------|
| `func` / `class` / `import` 语法 | [语言指南](../language-guide/01-getting-started.md) |
| Host 插件 / `dlopen` | [语言指南 · 运行时扩展](../language-guide/33-runtime-extensions.md) |
| `import Linalg` / `Os` / `Log` / `IO` | **本书** |
| CLI、`syx.toml` | [docs/9_UsageGuide.md](../../../docs/9_UsageGuide.md) |

## 诚实约定

- **已实现**：烟测或 Host 可跑的表面。
- **部分实现**：可编译但语义不完整（Process 若干路径仍 `partial`）。
- **未宣称**：产品 `Mat` GA、E-00、`A[i,j]`、faer `add`/`sub` host、P-SURF `[x]`。

嵌入包源码在 [`packages/`](../../../packages/README.md)。**不要**把 `Mat` 并进 `packages/core/math.sy`。
