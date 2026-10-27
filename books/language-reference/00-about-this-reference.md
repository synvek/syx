# 关于本参考

本参考描述 **SYX 语言** 的词法、语法、类型、表达式、语句、声明、特性（Attributes）、模式、泛型与协议的形式化规则，面向需要精确查阅语言形式的读者（编译器贡献者、SDK 维护者、深度语言用户）。

> **当前版本：** SYX **117.0**（Phase **139**）· `syx-cache-v1` · 许可证 [Apache-2.0](../../LICENSE) · 文档同步 2026-10-27
>
> 完整状态见 [shared/implementation-status.md](../shared/implementation-status.md)；架构决议见 [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md)；能力摘要见 [docs/adr/capabilities-summary.md](../../docs/adr/capabilities-summary.md)。

## 与《语言指南》的关系

| 本参考 | 《语言指南》 |
|--------|--------------|
| 形式化定义、语法表、诊断码 | 叙事讲解、完整范例、学习路径 |
| 「`if` 语句的语法是什么？」 | 「如何用 `if` 写分支逻辑？」 |

每章末尾「**用法示例**」链接到指南对应章节；指南每章末尾「**参见**」回链到参考。

## 章节结构

| # | 章节 | 内容 |
|---|------|------|
| 1 | [词法结构](../../docs/adr/architecture-decisions.md) | Token、关键字、字面量、注释、转义 |
| 2 | [类型](../../docs/adr/architecture-decisions.md) | 类型表达式、数值、String、集合、引用、不透明、SIMD |
| 3 | [表达式](../../docs/adr/architecture-decisions.md) | 表达式分类、优先级表 |
| 4 | [语句](../../docs/adr/architecture-decisions.md) | 变量声明、控制流、异常 |
| 5 | [声明](../../docs/adr/architecture-decisions.md) | import / func / class / struct / enum / actor / protocol / extension |
| 6 | [特性（Attributes）](../../docs/adr/architecture-decisions.md) | `@attached` / `@Service` / `@Around` / `@resultBuilder` / `@Equatable` 等 |
| 7 | [模式](../../docs/adr/architecture-decisions.md) | 变体、元组、switch / if case 模式 |
| 8 | [泛型形参和实参](../../docs/adr/architecture-decisions.md) | `<T>` / where / 协议约束 / 单态化 |
| 9 | [协议与一致性](../../docs/adr/architecture-decisions.md) | 协议声明、一致性、witness、存在类型 |
| 10 | [语法摘要](../../docs/adr/architecture-decisions.md) | EBNF / 模块 / 函数 / 类型 / 表达式 |
| A | [附录：实现状态矩阵](appendix-implementation-matrix.md) | 指向实现状态总表 |

## 记号约定

| 记号 | 含义 |
|------|------|
| `…` | 语法元素 |
| `\|` | 择一 |
| `?` | 可选 |
| `*` | 零次或多次 |
| `+` | 一次或多次 |
| `[x]` | 可选分组 |
| `{x}` | 零或多重复 |
| `(x \| y)` | 分组 |
| `terminal` | 终结符（关键字 / 标点） |
| `nonterm` | 非终结符 |

## 实现状态

每条规则或特性标注四档之一（详见 [shared/implementation-status.md](../shared/implementation-status.md)）：

| 档位 | 含义 |
|------|------|
| **已实现** / **GA** | `syx run` 或 parity / 硬闸可验证 |
| **部分实现** / **partial GA** | 可解析/编译但语义不完整 |
| **计划中** | roadmap 有记录，实现未完成 |
| **未实现** | 无对应 AST/语义 |
| **拒绝** | 有意 fail-closed（编译器会报错） |

## 诊断代码前缀

| 前缀 | 阶段 | 范围 | 示例 |
|------|------|------|------|
| L | 词法 / Lint | 词法单元、命名规范 | L001 未闭合字符串 / L001 函数名应小写 |
| P | 解析 | 语法错误 | P001 parse error |
| S | 语义 | 符号、类型、借用 | S002 未定义标识符 |
| A | Actor | 隔离违规 | A002 cross-actor access |
| C | 并发 | 跨隔离、Sendable | C003 非 Sendable |
| R | 运行时 | panic / trap | R007 force unwrap nil |
| H | Host 插件 | dlopen / 验签 | H001 插件未加载 |
| E | 宏 | 展开错误 | E002 宏展开深度超限 |
| W | 警告 | 编译期警告 | W001 #warning |

完整索引见 [语言指南附录 B 诊断代码索引](../language-guide/appendix-b-diagnostic-index.md)。

## 与编译器实现的关系

| 本参考 | 编译器源码 |
|--------|------------|
| 词法 | `crates/syx-lexer/src/` |
| 语法（AST） | `crates/syx-ast/src/lib.rs` |
| 解析 | `crates/syx-parser/src/` |
| 语义 | `crates/syx-semantic/src/` |
| 宏展开 | `crates/syx-macro/src/` |
| MIR | `crates/syx-mir/src/` |
| Codegen | `crates/syx-codegen/src/` / `crates/syx-llvm/src/` / `crates/syx-wasm-emit/src/` |
| VM | `crates/syx-vm/src/` |

完整 BNF 与 parser 同步更新于 [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md)。

## 工具链

- 安装 / CLI / 项目配置：[docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)
- 语法速查：[docs/8_QuickReference.md](../../docs/8_QuickReference.md)
- 编译器技术文档：[docs/README.md](../../docs/README.md)

---

**用法示例** → [语言指南](../language-guide/00-preface.md)
