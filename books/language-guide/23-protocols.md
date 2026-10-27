# 协议

**实现状态：** 协议声明 / 一致性 / witness 表 / 默认实现 / 关联类型 **已实现**；`Self` 约束 / protocol extension where **部分实现**；协议存在类型 `any P` GA；`class-only protocol` **未实现**（用 `class` 约束）

协议定义能力契约（method / property / associated type 列表），类型通过 conformance 满足协议。SYX witness 表分派机制与 Swift 类似；用户 Sequence/Iterator 自定义 for-in（Phase 71）。

## 协议声明

```syx
protocol Describable {
    func description() -> String
}

protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}

protocol Iterator {
    func next() -> Int?
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

## 一致性

```syx
protocol Named {
    var name: String { get }
    func describe() -> String
}

class User: Named {
    let name: String
    init(name: String) { self.name = name }
    func describe() -> String { return "User(\(name))" }
}
```

- class / struct / enum 均可实现协议
- 协议一致性在模块内**单点声明**（避免多文件冲突）

## 默认实现

```syx
protocol Describable {
    func description() -> String
    func description() -> String { return "<no description>" }   // 默认
}

class Foo: Describable {
    // 不实现 description()，继承默认
}

class Bar: Describable {
    func description() -> String { return "bar" }                  // 覆盖
}

print(Foo().description())                    // "<no description>"
print(Bar().description())                    // "bar"
```

## 关联类型

```syx
protocol Container {
    associatedtype Item
    func add(_ item: Item)
    func get(at: Int) -> Item
}

class IntStack: Container {
    typealias Item = Int                      // 关联类型绑定
    var items: Array<Int> = []

    func add(_ item: Int) { items.append(item) }
    func get(at i: Int) -> Int { return items[i] }
}
```

> **关联类型自动推断：** 当 `add` / `get` 签名唯一确定 `Item` 时，可省略 `typealias Item = …`。

## 协议作为类型

```syx
let desc: any Describable = Foo()
print(desc.description())                    // 通过 witness 表调度
```

`any P` 是 **存在类型**（existential）；运行时用 witness 表动态分派。详见 [第 25 章 不透明类型](../../docs/adr/architecture-decisions.md)。

## Iterator 协议

`Iterator` 是 SYX for-in 集成的核心协议（ADR-223 / `M2-S1-01` 稳定子集）：

```syx
protocol Iterator {
    func next() -> Int?
}
```

- 必须有 `func next()`（缺 → **S006**）
- 返回 `nil` 表示结束

> **返回类型（稳定子集）：** 可钉子集为 `Int?`（`nil` 终止）；泛型 `Element?` 全类型面仍开。自定义 class `: Iterator` 可被 for-in 遍历（Phase 71+）。

```syx
class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}

func main() {
    var s: Int = 0
    for x in Counter() { s = s + x }
    print(s)                                  // 6
}
```

> **Phase 71 三端 strict：** 自定义 Iterator for-in 在 VM / unified MIR / WASM 三后端行为一致。钉测：`m2_s1_01_collection_sequence_iterator`。

## Sequence 协议

```syx
protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}
```

> **状态：** `Sequence.makeIterator` **仍 partial**（for-in 主路径走 `: Iterator` / 内建 Array·String·Range）；Element 关联类型已注册。**非** `makeIterator` GA。

## 内置协议

| 协议 | 说明 | 隐式一致性 |
|------|------|------------|
| `Equatable` | `==` | 内置值类型 |
| `Hashable` | `hash(into:)` | 内置值类型 |
| `CustomStringConvertible` | `.description()` | 内置值类型 |
| `CustomDebugStringConvertible` | 调试描述 | 内置值类型 |
| `Codable` | 序列化 | `data class` + `@Codable`（L1 GA；L2 partial） |
| `Iterator` | `next() -> Int?`（子集） | for-in 稳定子集 **implemented**（ADR-223） |
| `Sequence` / `Collection` | 迭代 / Index | **部分实现**（`makeIterator` / 全量 Collection Index 仍开） |
| `Sendable` | 跨隔离安全 | 显式标注 |
| `Error` | 错误类型 | 自定义 |
| `Comparable` | 有序比较 | 数值类型 GA；自定义需 `compare` |

定义在 `crates/syx-stdlib/src/protocols.rs`。

## 协议一致性合成

`@Equatable` / `@Hashable` / `@Codable` 宏在 `data class` / 关联值 enum 上自动合成 witness：

```syx
@Equatable
@Hashable
@Codable
data class Point(x: Int, y: Int)
```

详见 [第 19 章 宏](19-macros.md#证明宏l1-ga--l2-partialphase-5052119)。

## 协议作为泛型约束

```syx
func printAll<T: CustomStringConvertible>(items: Array<T>) {
    for item in items {
        print(item.description())
    }
}

printAll(items: [1, 2, 3])                  // 1 2 3（Int: CustomStringConvertible）
```

详见 [第 24 章 泛型](../../docs/adr/architecture-decisions.md)。

## witness 表

`class` + 协议 → witness 表（运行时调度）：

```syx
protocol Named { func name() -> String }

class A: Named { func name() -> String { return "A" } }
class B: Named { func name() -> String { return "B" } }

func greet(_ n: Named) { print("hi \(n.name())") }
greet(A())                                  // "hi A"
greet(B())                                  // "hi B"
```

> **实现：** witness 表是 class 的隐藏字段；`CallWitness` 指令走 vtable-like 调度。

## 协议 + actor

`actor` 可实现协议；actor 方法自动隔离：

```syx
protocol Counter {
    func increment()
    func current() -> Int
}

actor MyCounter: Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

let c: any Counter = MyCounter()
await c.increment()                         // 跨隔离必须 await
print(await c.current())                    // 1
```

## Self 约束

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}
```

`Self` 指实现该协议的具体类型。**状态：** **部分实现**（基础 witness 调度可工作）。

## 协议组合

```syx
protocol A { func a() }
protocol B { func b() }

func use<T: A & B>(_ x: T) {
    x.a()
    x.b()
}
```

> **状态：** `&` 组合约束 **部分实现**（基础 `T: A & B` 工作）；用 `where` 子句作为完整版。

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

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S006** | `Iterator` 缺 `next()` | 加 `func next() -> Int?`（稳定子集） |
| **S010** | 协议不符合 | 加一致性 / 实现方法 |
| **S012** | 关联类型错误 | 加 `typealias` / 检查签名 |
| **S015** | `Hashable` 不一致 | 实现 `hash(into:)` / `@Hashable` |
| **S016** | `Equatable` 不一致 | 实现 `==` / `@Equatable` |
| **P001** | 协议方法签名不匹配 | 修正签名 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 协议声明 | **已实现** |
| 默认实现 | **已实现** |
| 关联类型 | **已实现** |
| `Self` 约束 | **部分实现** |
| `class-only protocol` | **未实现**（用 class 约束） |
| protocol extension where | **部分实现** |
| protocol composition `&` | **部分实现** |
| 协议存在类型 `any P` | **已实现**（witness 表） |
| 不透明返回 `some P` | **已实现** |
| retroactive conformance | 跨 module 允许（与 Swift 不同） |

---

**下一章** → [泛型](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：协议与一致性](../language-reference/09-protocols-and-conformances.md) · [第 22 章 扩展](../../docs/adr/architecture-decisions.md) · [第 24 章 泛型](../../docs/adr/architecture-decisions.md) · [第 25 章 不透明类型](../../docs/adr/architecture-decisions.md)
