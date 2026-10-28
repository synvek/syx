# SYX 语言教程 — 综合示例代码

> **本目录**：`tutorial/`
> **对应文档**：[`books/language-guide/`](../books/language-guide/)（33 章 + 2 附录）
> **目标**：为《SYX 语言指南》每一章提供一份可运行的 `.sy` 示例，覆盖该章在当前实现面上可用的语言特性。
> **可运行**：每个文件均含 `func main()`（或同步入口），可用 `syx run tutorial/XX-....sy` 验证；不支持的特性以 `// note:` 标明，不再使用错误的 Swift 伪语法。

---

## 章节目录

| # | 章节 | 示例文件 | 核心主题 |
|---|------|----------|----------|
| 00 | 前言 | — | （介绍性章节，无独立示例） |
| 01 | 快速入门 | [01-getting-started.sy](./01-getting-started.sy) | `func main()`、async 入口、`import`、CLI、错误排查 |
| 02 | 基础 | [02-the-basics.sy](./02-the-basics.sy) | `let`/`var`、类型注解、数值字面量、字面量后缀、`typealias`、嵌套、元组、原始字符串 |
| 03 | 基本运算符 | [03-basic-operators.sy](./03-basic-operators.sy) | 算术/比较/逻辑/空合并/范围/复合赋值/优先级 |
| 04 | 字符串和字符 | [04-strings-and-characters.sy](./04-strings-and-characters.sy) | 字符串字面量、`Character`、字素簇、转义、插值、String 方法、正则、String↔Array |
| 05 | 集合类型 | [05-collection-types.sy](./05-collection-types.sy) | `Array`/`Set`/`Dictionary`/`Object`、HOF、链式、for-in |
| 06 | 控制流 | [06-control-flow.sy](./06-control-flow.sy) | `if`/`while`/`for-in`/`switch`/`where` 守卫/`if let`/`if case`/`for await in` |
| 07 | 函数 | [07-functions.sy](./07-functions.sy) | 命名参数、默认参数、多返回、嵌套函数、效果注解、async/throws、重载、HOF |
| 08 | 闭包 | [08-closures.sy](./08-closures.sy) | 闭包表达式、捕获、尾随闭包、HOF 链、引用语义 |
| 09 | 枚举 | [09-enumerations.sy](./09-enumerations.sy) | 简单/关联值/泛型 enum、switch 穷尽、`Result` |
| 10 | 结构体和类 | [10-structures-and-classes.sy](./10-structures-and-classes.sy) | `struct`/`class`/`data class`/`actor`、ARC、`mutating`、`final` |
| 11 | 属性 | [11-properties.sy](./11-properties.sy) | 存储/静态/计算属性、`willSet`/`didSet`、`lazy`、`@Published` |
| 12 | 方法 | [12-methods.sy](./12-methods.sy) | 实例/静态方法、`mutating`、`override`、协议默认实现、self |
| 13 | 下标 | [13-subscripts.sy](./13-subscripts.sy) | 内置下标、自定义 `subscript`、多参数下标、KeyPath |
| 14 | 继承 | [14-inheritance.sy](./14-inheritance.sy) | 单继承、`override` 必填、`super`、`final class`、vtable |
| 15 | 构造过程 | [15-initialization.sy](./15-initialization.sy) | designated init、两段式构造、`init?`、委托构造、`required` |
| 16 | 析构过程 | [16-deinitialization.sy](./16-deinitialization.sy) | `deinit`、ARC 归零、deinit 链、actor deinit |
| 17 | 错误处理 | [17-error-handling.sy](./17-error-handling.sy) | `try`/`catch`/`throw`/`try?`/`try!`/`Result<T,E>` |
| 18 | 并发 | [18-concurrency.sy](./18-concurrency.sy) | `async`/`await`、`Task`、`TaskGroup`、Actor、Channel、GlobalActor、配置 |
| 19 | 宏 | [19-macros.sy](./19-macros.sy) | `#macro`、`@attached`、`@Codable`、`@resultBuilder`、Service DI、AOP |
| 20 | 类型转换 | [20-type-casting.sy](./20-type-casting.sy) | 数值提升、`is`/`as?`/`as!`、协议存在 cast、字面量推断 |
| 21 | 嵌套类型 | [21-nested-types.sy](./21-nested-types.sy) | `Outer.Inner` 路径、多层嵌套、flatten 推荐、嵌套 enum |
| 22 | 扩展 | [22-extensions.sy](./22-extensions.sy) | `extension` 方法/计算属性/构造器/协议/where 约束 |
| 23 | 协议 | [23-protocols.sy](./23-protocols.sy) | 协议声明、witness 表、关联类型、默认实现、`any P`、Self |
| 24 | 泛型 | [24-generics.sy](./24-generics.sy) | 泛型函数/类型/where/单态化、协议约束、关联类型 |
| 25 | 不透明类型 | [25-opaque-types.sy](./25-opaque-types.sy) | `some P` 编译期隐藏、`any P` 存在类型、性能取舍 |
| 26 | 内存管理 | [26-memory-management.sy](./26-memory-management.sy) | ARC + ManagedHeap GC、三后端、refcount、deinit、循环引用 |
| 27 | 内存安全 | [27-memory-safety.sy](./27-memory-safety.sy) | 借用检查、NLL、S011 冲突矩阵、Sendable |
| 28 | 访问控制 | [28-access-control.sy](./28-access-control.sy) | `public`/`internal`/`private`/`fileprivate`、跨模块 S005 |
| 29 | 模块与包 | [29-modules-and-packages.sy](./29-modules-and-packages.sy) | 单文件/多文件、`import` 路径、git/registry 依赖、`syx.toml`、锁 |
| 30 | 高级运算符 | [30-advanced-operators.sy](./30-advanced-operators.sy) | 位运算、自定义 infix/prefix/postfix、优先级/结合性 |
| 31 | SIMD | [31-simd.sy](./31-simd.sy) | `SIMD2/4/8/16<T>`、逐 lane 运算、归约、跨后端 |
| 32 | 运算符 vs Core Host | [32-operators-vs-host.sy](./32-operators-vs-host.sy) | Intrinsic vs Host、MIR 操作码、LLVM/Cranelift/WASM 映射 |
| 33 | 运行时扩展 | [33-runtime-extensions.sy](./33-runtime-extensions.sy) | dlopen 插件、`SYX_HOST_PLUGIN_PATH`、ed25519 验签、Embed API |
| A | 附录 A：实现状态矩阵 | — | （状态表，无独立示例） |
| B | 附录 B：诊断代码索引 | — | （诊断速查，无独立示例） |

---

## 使用方式

### 1) 配合《语言指南》阅读

每章顶部注释列出"本章涉及语言特性"，对照 `books/language-guide/<章>.md` 阅读效果最好。

### 2) 想跑起来

在仓库根目录：

```bash
cargo run -p syx-cli -- run tutorial/01-getting-started.sy
# 或使用已构建二进制：
./target/debug/syx run tutorial/25-opaque-types.sy
```

全部 33 个文件均可 `syx run`。若干章节刻意避开尚未稳定的表面（例如自定义 `subscript`、`super.init`、不可靠的 `throw`→`catch`、会挂起的关联值 `switch`、缺失的 plugin host），详见各文件内 `// note:`。

### 3) 想看更多真实可运行示例

- `crates/syx-driver/tests/fixtures/` — 编译器集成测试语料
- `examples/` — 用户向示例
- `books/language-guide/<章>.md` — 章节内嵌示例（叙事可能超前于本目录可运行子集）

### 4) 想对照 Swift 写同款代码

`books/language-guide/<章>.md` 每章末尾都有「与 Swift 的差异」表格。常见陷阱：SYX enum **不要**写 `case` 关键字；记录用 `class`/`data class`，不是 Swift 风格 `struct { var x: Int }`；Dictionary 字面量用 `{ "k": v }`；闭包参数须写成 `{ (x) in ... }`。

---

## 实现状态摘要（截至 SYX 117.0 / Phase 139）

| 类别 | 实现状态 |
|------|----------|
| **已实现（GA）** | 类型系统基础、字符串/集合、控制流、函数、闭包、类/结构体/枚举/actor、构造/析构、错误处理 try/catch、并发（async/await/Task/Actor/Channel）、扩展、协议 witness、单态化泛型、where 约束、访问控制、模块/包/锁、位运算、自定义运算符、SIMD2/4/8/16、ARC + ManagedHeap GC、Host plugin |
| **部分 GA（partial）** | 不透明类型 `some P`（子集）、`throws T` typed throws、rethrows、`@objc`/KVO、`inout`、`weak`/`unowned` 完整、Self 约束、协议 extension where 全集、`@escaping`/`@autoclosure`、泛型反射全集、AspectJ 切面类、JSON 路径完整化、CompileTime 求值 |
| **计划中** | 完整 Swift 属性、AspectJ 切面类、token-stream 完整 API、Result 泛型完整集合 |
| **未实现** | `@available`、`@objc` 桥接、Objective-C 互操作、Swift-Package 兼容 |

---

## 贡献

发现示例错误 / 想要补全某特性？

1. 直接编辑对应 `XX-<章>.sy`
2. 保持顶部注释的"本章涉及语言特性"清单同步
3. 提交 PR 时在 description 引用对应章节

---

**最后更新：** 2026-10-28
**维护者：** SYX 核心团队
**版本：** 对应 SYX 117.0 / Phase 139（tutorial 全量 `syx run` 可运行）
