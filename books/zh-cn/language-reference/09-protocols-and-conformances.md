# 协议与一致性

**实现状态：** 协议声明 / 一致性 / witness 表 / 默认实现 / 关联类型 / 存在类型 `any P` **已实现**；`Self` 约束 / 协议组合 / 协议 extension where / HKT witness **部分实现**；`class-only protocol` **未实现**

---

## 协议声明

```
protocol-decl ::= access? attrs? 'protocol' identifier generic-params?
                 (':' type (',' type)*)? where-clause? '{' protocol-member* '}'

protocol-member ::= protocol-method
                 |  protocol-property
                 |  protocol-init
                 |  protocol-associatedtype
                 |  protocol-method-with-default
```

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}

protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}

protocol Iterator {
    func next() -> Element?
}
```

| 元素 | 说明 |
|------|------|
| `func name(params) -> R` | 必需方法 |
| `var name: T { get }` | 必需 get 属性 |
| `var name: T { get set }` | 必需 get + set |
| `static func name` | 必需类型方法 |
| `init(params)` | 必需初始化器 |
| `associatedtype X` | 关联类型 |
| `typealias X = Y` | 关联类型默认值（**部分实现**） |
| 默认实现 | 协议内可提供方法体 |

## 一致性（Conformance）

```
conformance ::= type ':' protocol (',' protocol)*
             |  extension type ':' protocol where-clause? '{' … '}'
```

```syx
class MyClass: Equatable, CustomStringConvertible {
    func ==(lhs: MyClass, rhs: MyClass) -> Bool { … }
    func description() -> String { … }
}

extension Array: CustomStringConvertible { … }
```

> `class` 声明 `: Protocol` 列表触发 witness 表生成（部分路径）。
> 一致性在模块内**单点声明**（避免多文件冲突）。

**诊断：** S006（Iterator 缺 `next`）、S010（不符合协议）、S012（关联类型）

## 内置协议

| 协议 | 说明 | 隐式一致性 |
|------|------|------------|
| `Equatable` | `==` | 内置值类型 |
| `Hashable` | 哈希 | 内置值类型 |
| `Comparable` | 有序比较 | 数值类型 GA |
| `CustomStringConvertible` | `.description()` | 内置值类型 |
| `CustomDebugStringConvertible` | 调试描述 | 内置值类型 |
| `Encoder` / `Decoder` | 格式无关编解码汇 | **协议面已钉**（ADR-230；容器 API → S7-02） |
| `Encodable` / `Decodable` / `Codable` | 序列化 | **协议面已钉**（`Codable: Encodable, Decodable`）；`@Codable` / `JSON.encode` 为默认后端 |
| `Iterator` | `next()` | 声明校验 |
| `Sequence` / `Collection` | 迭代 | **部分实现** |
| `Sendable` | 跨隔离安全 | 显式标注 |
| `Error` | 错误类型 | 自定义 |

定义见 `crates/syx-stdlib/src/protocols.rs`。

## 协议默认实现

```syx
protocol Greetable {
    func greet() -> String
    func greet() -> String { return "Hello" }   // 默认
}

class User: Greetable {
    // 继承默认 → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }    // 覆盖
}
```

> 默认实现通过 `extension` 或协议内方法体声明（**Phase 50+**）。

## 动态分发

| 形式 | 分派机制 |
|------|----------|
| `class` + protocol | witness 调用（`CallWitness`） |
| `opaque some` | 编译期具体（零虚调度） |
| `any P` existential | 运行时 witness 表 |

```syx
protocol Named { func name() -> String }

class A: Named { func name() -> String { return "A" } }
class B: Named { func name() -> String { return "B" } }

func greet(_ n: Named) { print("hi \(n.name())") }
greet(A())                                    // "hi A"
greet(B())                                    // "hi B"
```

## 协议作为类型

```
protocol-type ::= 'any' protocol
              |  'some' protocol
```

| 形式 | 含义 |
|------|------|
| `some P` | 不透明返回（编译期具体） |
| `any P` | 存在类型（运行时 witness） |

```syx
func make() -> some Equatable { 42 }          // 编译期 Int
let p: any CustomStringConvertible = obj      // 运行时 witness
```

详见 [语言指南：不透明类型](../language-guide/25-opaque-types.md)。

## 关联类型

```syx
protocol Container {
    associatedtype Item
    func add(_ item: Item)
    func get(at: Int) -> Item
}

class IntStack: Container {
    typealias Item = Int
    var items: Array<Int> = []
    func add(_ item: Int) { items.append(item) }
    func get(at i: Int) -> Int { return items[i] }
}
```

> **自动推断：** 当 `add` / `get` 签名唯一确定 `Item` 时，可省略 `typealias Item = …`。

## 协议 extension 约束

```
protocol-extension ::= 'extension' type ':' protocol where-clause? '{' … '}'
                    |  'extension' type where-clause? '{' … '}'
```

```syx
extension Collection where Element: Equatable {
    func unique() -> [Element] { … }
}
```

> **状态：** 协议 extension 约束 **部分实现**；多约束 where 子句 Phase 42+。

## 协议组合

```syx
protocol A { func a() }
protocol B { func b() }

func use<T: A & B>(_ x: T) {
    x.a()
    x.b()
}
```

> **状态：** `&` 组合约束 **部分实现**（基础 `T: A & B` 工作）；用 `where T: A, T: B` 作为完整版。

## Self 约束

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}
```

> `Self` 指实现该协议的具体类型。**状态：** **部分实现**（基础 witness 调度可工作）。

## `@unchecked Sendable`

```syx
@unchecked Sendable
class MyLegacy {
    var internal: Int = 0
}
```

> 抑制 `Sendable` 检查；需外部同步保证线程安全。

## Encoder / Decoder / Codable 协议（ADR-230 / `M2-S7-01`）

```syx
public protocol Encoder { }
public protocol Decoder { }
public protocol Encodable { }
public protocol Decodable { }
public protocol Codable: Encodable, Decodable { }
```

协议层提供可扩展编码抽象；**默认后端**仍是动态 `JSON.encode` / `@Codable` 宏合成（非 `JSONEncoder` GA）。

```syx
@Codable
data class User(name: String, age: Int)

@Codable
enum Status {
    case ok
    case err
}
```

> **状态：** 协议面 ADR-230 已钉；`JSONEncoder`/`JSONDecoder` + `codingPath` ADR-231 / **`M2-S7-02`** 已钉；`JSONStream` NDJSON session + chunk feed ADR-232 / **`M2-S7-03`** 已钉（`codingPath=line[N].…`；**S7 收口**）。`@Codable` L1 GA；L2 关联值 enum 部分 GA。**非** CodingKeys DSL / metatype `T.Type` / File S3-03 真流 / IANA。

## 与 extension 联用

```syx
extension MyType: CustomStringConvertible {
    func description() -> String { return "MyType" }
}
```

> 协议一致性可在原类型声明处 / 单独 extension 声明。SYX 跨模块 extension 允许（**Phase 60+**）。

## conformance 合成

`@Equatable` / `@Hashable` / `@Codable` 宏在 `data class` / 关联值 enum 上自动合成 witness：

```syx
@Equatable
@Hashable
@Codable
data class Point(x: Int, y: Int)
```

详见 [特性（Attributes）](06-attributes.md) 与 [语言指南：宏](../language-guide/19-macros.md)。

## witness 表实现

`class` + 协议 → witness 表（运行时调度）：

- class 持有 `witness_table: *const WitnessTable`
- `CallWitness` 指令走 witness 表
- 多协议：每协议一张表

```syx
class Multi: A, B, C {
    func a() { … }
    func b() { … }
    func c() { … }
}
```

> class 内部存 `witness_a` / `witness_b` / `witness_c` 三张表。

## 与 Swift 差异

| Swift | SYX |
|-------|-----|
| 协议声明 | **已实现** |
| 默认实现 | **已实现**（协议内 + extension） |
| 关联类型 | **已实现** |
| `Self` 约束 | **部分实现** |
| `class-only protocol` | **未实现**（用 class 约束） |
| protocol extension where | **部分实现** |
| protocol composition `&` | **部分实现** |
| 存在类型 `any P` | **已实现**（witness 表） |
| 不透明返回 `some P` | **已实现** |
| retroactive conformance | 跨 module 允许（与 Swift 不同） |
| `_const` 协议 | **未实现** |

## 完整示例

```syx
protocol Shape {
    func area() -> Double
    func describe() -> String
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
    func describe() -> String { return "circle r=\(r)" }
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
    func describe() -> String { return "square s=\(s)" }
}

func total(_ shapes: Array<Shape>) -> Double {
    var sum = 0.0
    for s in shapes { sum = sum + s.area() }
    return sum
}

func main() {
    let shapes: Array<Shape> = [Circle(r: 2.0), Square(s: 3.0)]
    for s in shapes { print(s.describe()) }
    print("total=\(total(shapes))")
}
```

---

**诊断：** S006 / S010 / S012 / S015 / S016 / P001

**用法示例** → [语言指南：协议](../language-guide/23-protocols.md)

**参见** → [类型](02-types.md) · [声明](05-declarations.md) · [泛型形参](08-generic-parameters-and-arguments.md) · [特性](06-attributes.md)
