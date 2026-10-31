# 基础

**实现状态：** **已实现**（包括 `Int128` / `UInt128` Phase 38、平台 `Int`/`UInt`、类型嵌套 MVP、`typealias` GA Phase 40）

本章介绍常量、变量、类型注解、注释、数值字面量与类型推断。掌握这些是写任何 SYX 程序的前提。

## 变量与常量

```syx
var mutable = 10
mutable = 20          // ✅

let immutable = 42
// immutable = 43     // ❌ 编译错误 S003（cannot assign to let binding）
```

| 关键字 | 含义 | 重新赋值 | 重新声明同作用域 |
|--------|------|----------|------------------|
| `let` | 不可变绑定 | ❌ S003 | ❌ S001 |
| `var` | 可变绑定 | ✅ | ❌ S001 |

> **作用域：** 块作用域 `{ … }`；`for` / `while` / `func` 都引入新作用域。子作用域可重名「shadow」外层 `let`/`var`（不推荐；易引起阅读混淆）。

**诊断：** S003（对 `let` 赋值）、S001（重复定义）、S002（未定义标识符）。

## 类型注解

显式类型用于让读者一眼看清「这是什么类型」，或在推断有歧义时强制指定：

```syx
let x: Int = 10
let name: String = "SYX"
let items: Array<Int> = [1, 2, 3]
let scores: [String: Int] = ["a": 1, "b": 2]
let maybe: Int? = nil
```

| 标注位置 | 语法 | 备注 |
|---------|------|------|
| 局部变量 | `let x: T = expr` | 推断失败时需要显式标注 |
| 顶层变量 | `var count: Int = 0` | 通常有默认值 |
| 函数形参 | `func f(x: Int)` | 必填（无类型推断） |
| 函数返回 | `-> T` | 缺省 `Void`（即 `()`） |
| 属性 | `var name: String` | 必须显式标注或给初值 |

**类型推断：** 首次赋值后 `let` 绑定的类型被**冻结**。后续赋值若不兼容 → **S004**。

```syx
let n = 10       // n: Int
// n = "hello"   // ❌ S004：cannot assign String to Int
```

## 注释

```syx
// 单行注释

/* 块注释
   /* 可嵌套 */
   任意行
*/

/// 文档注释（`syx doc` 提取为 API 文档）
/// 多行：以 `///` 开头
func add(a: Int, b: Int) -> Int { return a + b }
```

| 形式 | 用途 |
|------|------|
| `//` | 单行（行尾或整行） |
| `/* … */` | 块注释；**支持嵌套**（与 Rust 相同） |
| `///` | 文档注释；`syx doc .` 生成 `docs/api.md` |

## 数值类型与字面量

SYX 提供 **平台字长** 与 **固定宽度** 两套整数，外加 128 位与 SIMD 向量（详见 [第 31 章 SIMD](31-simd.md)）。

| 类型 | 大小 | 范围 / 说明 |
|------|------|-------------|
| `Int8` | 8 位有符号 | -128 … 127 |
| `Int16` | 16 位有符号 | -32 768 … 32 767 |
| `Int32` | 32 位有符号 | -2³¹ … 2³¹-1 |
| `Int64` | 64 位有符号 | -2⁶³ … 2⁶³-1 |
| **`Int128`** | 128 位有符号 | Phase 38；字面量后缀 `i128` |
| **`Int`** | 平台字长 | 64 位 host → `Int64`；32 位 host → `Int32` |
| `UInt8` / `UInt16` / `UInt32` / `UInt64` | 固定宽度无符号 | |
| **`UInt128`** | 128 位无符号 | Phase 38；字面量后缀 `u128` |
| **`UInt`** | 平台字长无符号 | 64 位 host → `UInt64` |
| `Float32` / `Float` | 32 位浮点（IEEE 754） | |
| `Float64` / `Double` | 64 位浮点 | 无后缀浮点默认 `Double` |

字面量语法：

```syx
let a = 42         // Int（平台字长）
let b = 100i8      // Int8
let c = 1000i64    // Int64
let d = 0xFFu8     // UInt8，十六进制
let e = 0b1010u16  // UInt16，二进制
let f = 0o17u32    // UInt32，八进制
let g = 1_000_000i64  // 下划线分隔（可读性，编译期去掉）
let h = 42i128     // Int128
let i = 3.14f32    // Float32
let j = 3.14       // Double（默认）
let k = -2.5e10    // Double 科学计数法
```

> **下划线**（`_`）可在数字字面量任意位置分隔，编译期被忽略（例：`1_000_000` = `1000000`）。
>
> **后缀大小写：** `i8`/`I8`、`u8`/`U8`、`f32`/`F32` 等均接受；推荐小写以与 Swift 风格一致。

**有符号 vs 无符号：** 混合运算可能触发 **S014**（避免静默 wrap）。显式转换见 [第 20 章 类型转换](20-type-casting.md)。

## 布尔与空

```syx
let ok: Bool = true
let ng: Bool = false
let empty: String? = nil
let present: String? = "hi"
```

| 字面量 | 类型 |
|--------|------|
| `true` / `false` | `Bool` |
| `nil` | 任意 `T?`（nil literal，类型从上下文推断） |

## 类型别名 `typealias`

为长类型名取短名（**Phase 40 GA**）：

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String

func register(h: Handle) {}
let f: Handler = { n in "id=\(n)" }
```

- `typealias` **不创建**新类型，只是同义词
- 不支持 `where` 子句；泛型参数必须 1:1 复制
- 等价于 Swift `typealias`，但不能用 `associatedtype` 替代

**诊断：** 循环定义 → S004（type recursion detected）

## 类型嵌套

类型内嵌套子类型是 Swift 的核心模式。SYX **有限支持**：

```syx
class Outer {
    var name: String
    init(name: String) { self.name = name }

    class Inner {           // ⚠️ 部分实现
        var value: Int = 0
    }
}

let o = Outer(name: "x")
let i = Outer.Inner()      // 可能需要 flatten（见下）
```

**当前状态：**

- 顶层 `class` / `struct` / `enum`：**已实现**
- 类型内嵌套另一类型声明：**部分实现**（parser 能力有限）
- 嵌套类型外部可见性：**未完整**

**替代做法**（推荐，与 Swift module 风格一致）：

```syx
struct OuterMarker { }
struct OuterMarker_Inner {
    value: Int
}
```

详见 [第 21 章 嵌套类型](21-nested-types.md)。

## `nil` 与可选类型预览

```syx
let empty: String? = nil
let present: String? = "hi"
let length: Int? = present?.count    // 4
let safe: Int = empty?.count ?? 0   // 0（空合并）
let crash: Int = present!.count     // 4（强制解包）
```

三种解包方式：

| 形式 | 失败时 | 适用 |
|------|--------|------|
| `?.` 可选链 | 表达式整体为 `nil` | 多级访问，避免嵌套 if |
| `!` 强制解包 | 运行时错误 **R007** | 100% 确定非 nil |
| `??` 空合并 | 走默认值 | 提供回退 |

完整用法见 [第 30 章 高级运算符](30-advanced-operators.md)。

## 命名规范

SYX 社区惯例（不强制；`syx lint` 给出 L001–L012 警告）：

| 类别 | 命名 | 示例 |
|------|------|------|
| 类型 | `UpperCamelCase` | `Greeter`, `MyType` |
| 函数 / 变量 / 属性 | `lowerCamelCase` | `greet`, `firstName` |
| 常量 | `lowerCamelCase` | `defaultPort`（**不** 用全大写） |
| 枚举 case | `lowerCamelCase` | `case success`, `case failure` |
| 协议 | `UpperCamelCase`（常以 `able`/`ible`/`Convertible` 结尾） | `Iterator`, `CustomStringConvertible` |
| 文件 | `kebab-case.sy` 或 `snake_case.sy` | `main.sy`, `local_math.sy` |

## 与 Swift 的差异

| 点 | SYX |
|----|-----|
| `Int` 字长 | 平台字长（64 位 host → `Int64`）；Swift Apple 平台固定 64 位 |
| `Int128` / `UInt128` | Phase 38 GA（Swift 仍无原生 128 位） |
| `typealias` | **已实现**（Phase 40 GA） |
| 多行字符串 `"""` | **MVP GA**（Phase 139）；可含裸换行与 `\(expr)` 插值 | 非 Swift raw/`#"""` 定界全集 |
| 元组语法 | **MVP GA**（Phase 139）：`(T, U)` 类型与字面量、命名元素 `(x: 1, y: 2)`、`let (x, y) = t` 解构、`.0`/`.x` 成员 | 非任意元数协议 / 完整 Swift 元组特性 |
| 数字字面量下划线 | ✅ 支持（与 Swift 一致） |
| 前缀 `0x` / `0o` / `0b` | ✅ 支持 |

## 实现状态与限制

- 显式 `typealias`：**GA**（Phase 40；`fixtures/parity/generics.sy`）
- `Int128` / `UInt128`：**GA**（Phase 38）
- 类型嵌套：**部分实现**；建议用 flatten 命名
- 多行字符串字面量 `"""`：**MVP GA**（`fixtures/lang/string_multiline.sy`）
- 元组类型/字面量/命名元素：**MVP GA**（`fixtures/lang/tuple.sy`）

```syx
let pair: (Int, String) = (1, "a")
print(pair.0)
let named = (x: 10, y: 20)
print(named.x)
let (a, b) = (3, 4)
```


---

**下一章** → [基本运算符](03-basic-operators.md)

**参见** → [语言参考：词法](../language-reference/01-lexical-structure.md) · [语言参考：类型](../language-reference/02-types.md) · [第 3 章 基本运算符](03-basic-operators.md)
