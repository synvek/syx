# 前言

欢迎阅读 **《SYX 语言指南》**。SYX（读作 **sy-ex**，`/saɪˈɛks/`）是一门受 Swift 语法启发的编程语言，由 Rust 实现的完整编译器工具链支撑（默认 **VM** + **LLVM** 原生 AOT + **WASM**；实验 JIT tier-2 opt-in）。源文件扩展名为 **`.sy`**，项目清单为 **`syx.toml`**。本指南帮助你系统学习 SYX 的语义与惯用法，并通过可运行示例建立直觉。

## SYX 是什么

SYX 提供：

- **语法：** Swift 风格的 `func` / `let` / `var`、协议、泛型、`async` / `await`、属性包装、结果构建器
- **类型：** 静态强类型，含 `Int8..Int64/128`、`UInt8..UInt64/128`、`Float32/64`、`SIMD2/4/8/16<T>`、`BigInt`、引用类型 `&T` / `&mut T`
- **后端：** 默认栈式 VM（`syx run`）+ Native LLVM（`syx build --native`）+ WASM（`syx run --wasm`）；Cranelift **AOT 已移除**（JIT tier-2 实验 opt-in）
- **模块：** `syx.toml` 项目 + `import` 路径依赖 + git/registry 依赖 + Host 插件（dlopen）
- **标准库：** 嵌入 `Math` / `IO` / `Net` / `Crypto` / `Di` / `Os` / `Sync` / `UI` 等；Core builtins 见 [`packages/core/builtins/`](../../../packages/core/builtins/)

> **重要：** SYX **不是** Swift 的子集或超集。许多名称相同，语义可能非常不同（特别在 String 字符语义、内存模型、宏、并发与跨模块 Actor 上）。

## 编译流水线

```
Source (.sy, UTF-8)
  → syx-lexer (Token 流)
  → syx-parser (AST, 递归下降)
  → syx-semantic (类型检查 + 符号表 + 借用)
  → syx-macro (固定点展开)
  → syx-mir (统一 IR)
  → syx-vm | syx-llvm | syx-wasm-emit
```

见 [docs/5_CompleteTechnicalDocumentation.md](../../../docs/5_CompleteTechnicalDocumentation.md) · [docs/3_SemanticAnalysis.md](../../../docs/3_SemanticAnalysis.md)。

## 本书约定

### 实现状态标记

每节顶部以 **实现状态** 行标注：

| 标记 | 含义 |
|------|------|
| **已实现** / **GA** | `syx run` 或 parity / 硬闸可验证 |
| **部分实现** | 可解析/编译但语义不完整 |
| **计划中** | roadmap 有记录，实现未完成 |
| **未实现** | 无对应 AST/语义 |

统一总表：[shared/implementation-status.md](../../shared/implementation-status.md)。

### 代码示例

除非另有说明，示例可在仓库根目录用：

```bash
cargo run -p syx-cli -- run path/to/example.sy
```

### 与参考文档的关系

| 需求 | 去向 |
|------|------|
| 叙事学习 | 本书（language-guide） |
| 形式化规则 | [language-reference](../language-reference/00-about-this-reference.md) |
| 嵌入包 API | [library-guide](../library-guide/00-preface.md) |
| 安装 / CLI | [docs/9_UsageGuide.md](../../../docs/9_UsageGuide.md) |

CLI、Native/WASM 编译、Embed API 与诊断码请参阅：

- [docs/9_UsageGuide.md](../../../docs/9_UsageGuide.md) — 完整使用指南
- [docs/8_QuickReference.md](../../../docs/8_QuickReference.md) — 语法速查
- [docs/12_BackendDebuggingGuide.md](../../../docs/12_BackendDebuggingGuide.md) — DAP 调试与后端调试

## 与 Swift 的总体差异（速览）

| 主题 | Swift 6 | SYX 117.0 |
|------|---------|-----------|
| String 字符 | 扩展字素簇（grapheme cluster） | **字素簇**（`unicode-segmentation`）；早期按 Unicode 标量 |
| 内存 | ARC（统一） | **ARC（class）+ ManagedHeap**（VM / WASM / Native） |
| 入口 | `@main` / 自由函数 | `func main()`（同步或 `async`） |
| `switch` | 隐式 break | `case { }` 块；无 fallthrough |
| 宏 | 内置 + 完整 Swift 宏 + 外部 macro | freestanding + 用户 `#macro` MVP + attached / `@resultBuilder` / DI 点切面 MVP |
| 运行时扩展 | 静态 stdlib + ABI 插件 | 静态 Host + opt-in dlopen 插件 + Embed API |
| 泛型 | 全量 + 特化 | 单态化（无运行时特化反射全集） |
| 并发 | Structured + actors + distributed | `async`/`await` + `actor` + `Task`/`TaskGroup` + `Channel` + `GlobalActor` MVP |
| 原生后端 | — | **LLVM sole**（Cranelift AOT 已移除） |

## 推荐学习路径

| 阶段 | 章节 | 目标 |
|------|------|------|
| **入门** | 1–8 | 能读能写普通 SYX 程序 |
| **类型系统** | 9–16 | 掌握 enum/class/struct/protocol/泛型/属性/构造/析构 |
| **高级特性** | 17–25 | 错误处理、并发、宏、协议、不透明类型 |
| **运行时与扩展** | 26–33 | 内存、借用、宏/AOP 实战、Host 插件、SIMD |
| **附录** | A、B | 状态总表 + 诊断码速查 |

每一阶段都可以跳到对应语言参考章节精确查表。

## 版本历史（最近 Phase）

| Phase | 营销版 | 主要变化 |
|------|--------|---------|


历史详见 [shared/implementation-status.md](../../shared/implementation-status.md) 与 ADR 序列。

---

**下一章** → [快速入门](01-getting-started.md)

**参见** → [语言参考：关于本参考](../language-reference/00-about-this-reference.md) · [实现状态总表](../../shared/implementation-status.md) · [完整使用指南](../../../docs/9_UsageGuide.md)
