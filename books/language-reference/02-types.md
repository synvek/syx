# 类型

**实现状态：** 核心类型 **已实现**；`Int128` / `UInt128` Phase 38；`SIMD2/4/8/16<T>` Phase 38+；`Result<T, E>` 泛型代数 **Phase 21–23**；`BigInt` **Phase 46 MVP**；`some` / `any` Phase 11–12；引用 `&T` / `&mut T` **部分实现**（SE-0461 默认 on Phase 72）

SYX 类型在源码中以 **类型表达式（TypeExpr）** 书写，语义层映射为 **TypeId**。

---

## 类型表达式形式

```
type-expr ::= named-type
            | optional-type
            | array-type
            | dict-type
            | function-type
            | tuple-type
            | reference-type
            | opaque-type
            | metatype

named-type      ::= identifier ('<' type-args '>')?
optional-type   ::= type '?'
array-type      ::= '[' type ']' | 'Array' '<' type '>'
dict-type       ::= '[' type ':' type ']' | 'Dictionary' '<' type ',' type '>'
function-type   ::= '(' type-list? ')' '->' type
tuple-type      ::= '(' type (',' type)+ ')'
reference-type  ::= '&' 'mut'? lifetime? type
opaque-type     ::= 'some' type | 'any' type
metatype        ::= 'Self' | 'SelfType' | 'Type'
```

| 形式 | 示例 | AST |
|------|------|-----|
| 命名类型 | `Int`、`Array<Int>` | `Named` |
| 可选 | `String?`、`Int?` | `Optional` |
| 数组 | `[Int]`、`Array<String>` | `Array` |
| 字典 | `[String: Int]` | `Dictionary` |
| 函数类型 | `(Int) -> String` | `Function` |
| 元组类型 | `(Int, String)` | `Tuple` |
| 引用 | `&Int`、`&mut String` | `Reference` |
| 不透明 | `some Equatable`、`any Protocol` | `OpaqueSome` / `OpaqueAny` |
| 元类型 | `Self`、`Type` | `Metatype` |
| 空 | 无返回值 | `Void` |

## 数值类型

| 类型 | 大小 | 范围 | 状态 |
|------|------|------|------|
| `Int8` | 8 位有符号 | -128 … 127 | **已实现** |
| `Int16` | 16 位有符号 | -32 768 … 32 767 | **已实现** |
| `Int32` | 32 位有符号 | -2³¹ … 2³¹-1 | **已实现** |
| `Int64` | 64 位有符号 | -2⁶³ … 2⁶³-1 | **已实现** |
| `Int128` | 128 位有符号 | -2¹²⁷ … 2¹²⁷-1 | **Phase 38 GA** |
| **`Int`** | 平台字长 | 64 位 host → `Int64`；32 位 → `Int32` | **已实现** |
| `UInt8` | 8 位无符号 | 0 … 255 | **已实现** |
| `UInt16` / `UInt32` / `UInt64` | 固定宽度无符号 | | **已实现** |
| `UInt128` | 128 位无符号 | 0 … 2¹²⁸-1 | **Phase 38 GA** |
| **`UInt`** | 平台字长无符号 | 64 位 host → `UInt64` | **已实现** |
| `Float32` / `Float` | 32 位 IEEE 754 | ~1.4e-45 … ~3.4e38 | **已实现** |
| `Float64` / `Double` | 64 位 IEEE 754 | ~4.9e-324 … ~1.8e308 | **已实现** |

字面量后缀选择固定宽度；无后缀整数默认 `Int`，无后缀浮点默认 `Double`。

## 基础类型

| 类型 | 说明 | 字面量 |
|------|------|--------|
| `Bool` | 布尔 | `true` / `false` |
| `String` | UTF-8 字符串；字素簇语义（Phase 84） | `"…"` |
| `Void` | 无值返回 | — |
| `Character` | 单标量 `String` 类型名 | 单 `Character` 字面量 = `String` |
| `Any` | 任意类型 | — |

## 集合与动态类型

| 类型 | 字面量示例 | 说明 |
|------|------------|------|
| `Array<T>` | `[1, 2, 3]` | 有序动态数组（**强类型**，无裸 `Array`） |
| `Set<T>` | `Set<Int>` | 去重集合 |
| `[K: V]` / `Dictionary<K, V>` | `["a": 1]` | 静态键值类型，键须 `Hashable` |
| `Object` | `{ "k": "v" }` | 动态键值对象 |

> **强类型 Phase 60：** 裸 `Array` 已删除；必须 `Array<T>`。`Array<UInt8>` 是二进制流。

## 可选类型

```
T?  ::= Optional<T>
```

- `T?` 表示可空；仅 `nil` 与 `T` 值合法
- 配合 `??` / `?.` / `!` 使用

```syx
let a: Int? = 42
let b: Int? = nil
let c = a ?? 0                            // nil 合并
let d = a?.description                    // 可选链
let e = a!                                // 强制解包
```

**诊断：** S004 类型不匹配

## Result 类型

```
Result<T, E> ::= ok(T) | err(E)
```

- `Result<Ok, Err>` 用于错误传播
- `throws` 函数与 `try` / `?` 链关联
- `Result<T, E>` 是 final class，提供 `static ok` / `static err` / `isOk` 等

**实现状态：** **Phase 21–23 builtin wire**（GA）；Phase 46+ 提供类方法；详见 [语言指南：枚举](../language-guide/09-enumerations.md)。

## BigInt

```
BigInt ::= <arbitrary precision integer>
```

- **Phase 46 MVP**（`__syx_bigint_*` + Core）
- `BigInt.fromString(s)` / `b.toString()` / `BigInt.add(a, b)`

## 引用类型

```
&'a mut? T
```

```syx
&Int           // 不可变借用
&mut Int       // 可变借用
&'a Int        // 带生命周期标注（**部分实现**）
```

| 状态 | 说明 |
|------|------|
| 借用检查 | **已实现**（borrow regions v8） |
| SE-0461 | **默认 on**（Phase 72） |
| 生命周期 | **部分实现**（S017） |
| 跨 `await` 借用 | **部分实现** |

**诊断：** S011 借用/变异违规；S017 生命周期

## 不透明类型与存在类型

```
some P    -- 不透明返回类型（编译期具体，外部仅知 P）
any P     -- 存在类型（运行时 witness 表动态分派）
```

```syx
func make() -> some Equatable { 42 }       // 编译期 Int
let p: any CustomStringConvertible = "hi"   // 运行时 witness
```

| 形式 | 状态 | 含义 |
|------|------|------|
| `some P` | **已实现** | 不透明返回（零虚调度） |
| `any P` | **已实现** | 存在类型（witness 表） |
| `some P` 在参数位置 | **未实现** | 规划中 |

详见 [语言指南：不透明类型](../language-guide/25-opaque-types.md)。

## 元类型（metatype）

```syx
Self        // 当前实现类型（class / struct / enum）
SelfType    // 同 Self（更明确）
Type        // 类型元
```

> `Self` 在 protocol 默认实现中指「实现该协议的具体类型」。

## SIMD 类型

```
SIMD2<T> | SIMD4<T> | SIMD8<T> | SIMD16<T>
```

| 形式 | 状态 | 后端 |
|------|------|------|
| `SIMD2<T>` | **已实现** | 通用 |
| `SIMD4<T>` | **已实现** | SSE / NEON |
| `SIMD8<T>` | **已实现** | AVX2 |
| `SIMD16<T>` | **Phase 38+ partial GA** | AVX-512 / WASM SIMD128 |

元素 `T` 必须是固定宽度数值（`Int8/16/32/64` / `UInt8/16/32/64` / `Float32/64`）。

详见 [语言指南：SIMD](../language-guide/31-simd.md)。

## 用户定义类型

```
user-type ::= struct | class | data class | enum | actor | protocol
```

| 声明 | 运行时 | 状态 |
|------|--------|------|
| `struct` | 值/字段聚合 MVP | **已实现** |
| `class` | 引用对象（ARC） | **已实现** |
| `data class` | 数据载体 + 证明宏 | **Phase 50–52** |
| `final class` | 禁止子类化 | **Phase 60 GA** |
| `enum` | 枚举与关联值（ADT） | **已实现** |
| `actor` | 隔离并发对象 | **已实现** |
| `protocol` | 协议类型（witness） | **已实现** |
| `extension` | 类型扩展 | **已实现** |
| `global actor` | 全局 actor | **已实现** |

## 泛型类型参数

```
generic-params ::= '<' identifier (',' identifier)* (':' constraint-list)? ('where' where-clause)? '>'
```

```syx
struct Box<T> { var value: T }
func id<T>(x: T) -> T { return x }
enum Optional<T> { case some(T); case none }
```

- 单态化（monomorphization）生成具体实例
- 约束：`where T: Equatable` / `T: Comparable` / `T: CustomStringConvertible` 等

详见 [泛型形参和实参](../../docs/adr/architecture-decisions.md)。

## 类型推断

```
inference ::= bidirectional | unidirectional
```

- **未标注** 时由语义分析推断
- `let` 绑定在首次赋值后**冻结**类型
- 后续赋值不兼容 → **S004**
- 上下文类型（`let arr: Array<Int> = [1, 2]`）反推元素类型

**诊断：** S004、S008（窄化）

## 类型嵌套

```
type-nesting ::= <not fully supported>
```

- 顶层 `class` / `struct` / `enum`：**已实现**
- 类型内嵌套另一类型声明：**部分实现**（parser 边界情况）
- 嵌套类型外部可见性：**未完整**
- 推荐 flat 命名（`OuterMarker_Inner`）

详见 [语言指南：嵌套类型](../language-guide/21-nested-types.md)。

## 内置协议（隐式一致性）

数值、`Bool`、`String`、`Array<T>`、`Set<T>`、`Dictionary<K, V>`、`Object` 等自动符合：

| 协议 | 说明 |
|------|------|
| `Equatable` | `==` |
| `Hashable` | 哈希 |
| `CustomStringConvertible` | `.description()` |
| `CustomDebugStringConvertible` | 调试描述 |
| `Iterator` | `func next() -> String?` |
| `Sequence` | 迭代器工厂 |

详见 [协议与一致性](../../docs/adr/architecture-decisions.md) · [crates/syx-stdlib/src/protocols.rs](../../crates/syx-stdlib/src/protocols.rs)。

## 类型别名

```
typealias-decl ::= 'typealias' identifier '=' type
```

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String
```

- `typealias` **不创建**新类型（仅同义词）
- 不支持 `where` 子句
- **Phase 40 GA**

## 内部类型（prelude）

预声明（无需 import）：

| 类型 | 含义 |
|------|------|
| `Optional<T>` / `T?` | 可选 |
| `Result<T, E>` | 错误返回 |
| `Range<T>` | 范围 |
| `Array<T>` | 数组 |
| `Set<T>` | 集合 |
| `Dictionary<K, V>` | 字典 |
| `Object` | 动态对象 |
| `String` / `Bool` / `Int` / `Double` | 基础类型 |
| `Int8` … `Int64` / `UInt8` … `UInt64` | 固定宽度 |
| `Int128` / `UInt128` | 128 位（Phase 38） |
| `Float32` / `Float64` | 浮点 |
| `SIMD2/4/8/16<T>` | SIMD |
| `BigInt` | 任意精度（Phase 46） |
| `Character` | 单标量 |
| `Void` | 无值 |
| `Any` / `AnyObject` | 任意类型 |

---

**诊断：** S004、S008、S010、S011、S017

**用法示例** → [语言指南：基础](../language-guide/02-the-basics.md) · [集合](../language-guide/05-collection-types.md) · [枚举](../language-guide/09-enumerations.md) · [不透明类型](../language-guide/25-opaque-types.md) · [SIMD](../language-guide/31-simd.md)

**参见** → [表达式](../../docs/adr/architecture-decisions.md) · [泛型形参](../../docs/adr/architecture-decisions.md) · [协议](../../docs/adr/architecture-decisions.md)
