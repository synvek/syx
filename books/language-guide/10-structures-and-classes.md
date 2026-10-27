# 结构体和类

**实现状态：** `struct` / `class` / `data class` / `actor` / `enum` **已实现**（含 `final class` Phase 60、`data class` Phase 50–52 证明宏）；继承与多态 **GA**（Phase 51–55 + Phase 66 vtable）

SYX 提供五种类型声明：`struct`（值）、`class`（引用）、`data class`（不可变记录）、`actor`（并发隔离）、`enum`（ADT，已见 [第 9 章](../../docs/adr/architecture-decisions.md)）。本章讲清「**何时用哪个**」以及**内存语义**。

## struct

值类型字段聚合。Phase 70+ 起值语义逐步 GA（早期 MVP）：

```syx
struct Point {
    var x: Int
    var y: Int
}

func main() {
    let p = Point(x: 1, y: 2)
    print(p.x)                             // 1
    print(p.y)                             // 2
}
```

| 特性 | 状态 |
|------|------|
| 字段 `var` / `let` | **已实现** |
| 构造（按字段顺序） | **已实现**：`Point(1, 2)` |
| `mutating func` | **部分实现**（Phase 60 起基本可用） |
| 值类型完整拷贝 | **部分实现**（深拷贝边界演进中） |
| `Codable` 合成 | 通过 `@Codable` 宏（`data class` 优先） |

```syx
struct Counter {
    var value: Int = 0
    mutating func increment() { value = value + 1 }
}

var c = Counter()
c.increment()
print(c.value)                             // 1
```

> **`mutating`：** 标记会修改 `self` 的方法；调用方必须用 `var`（不能 `let`）。无标记的 mutating 方法在 `let` 上调用 → **S020**。

## class

引用类型对象。SYX class 是 **ARC** 引用计数 + class `ObjectRef`（**Phase 73 起**）：

```syx
class Greeter {
    var name: String

    init(name: String) {
        self.name = name
    }

    func hello() {
        print("Hello, \(self.name)")
    }
}

func main() {
    let g = Greeter(name: "World")
    g.hello()                               // "Hello, World"

    let g2 = g                              // 共享引用
    g2.name = "SYX"
    print(g.name)                           // "SYX"（同一对象）
}
```

| 特性 | 状态 |
|------|------|
| 字段 `var` / `let` | **已实现** |
| `init` | **已实现**（见 [第 15 章](../../docs/adr/architecture-decisions.md)） |
| `deinit` | **部分实现**（class ARC 归零触发，Phase 102/103） |
| 继承 | **GA**（Phase 51–55 + Phase 66 vtable） |
| `override` | **GA**（多态分派） |
| 引用语义 | **已实现** |

来源：`crates/syx-driver/tests/fixtures/parity/inheritance_dispatch.sy`

### final class

Phase 60 起 `final class` 用于禁止子类化的封闭类型：

```syx
final class Token {
    let value: String
    init(value: String) { self.value = value }
}
```

- `final` 禁止子类化
- `final` 允许编译器做 devirtualization（静态调用）
- ADT 变体族请用 `enum`（`switch` exhaustiveness MVP）

## data class

不可变记录的声明式语法。**Phase 50–52 证明宏** 在 stored 属性 `data class` 上自动合成 `==` / `hash` / `encode` / `decode`：

```syx
@Equatable
@Hashable
@Codable
data class Point(x: Int, y: Int)

func main() {
    let p1 = Point(x: 1, y: 2)
    let p2 = Point(x: 1, y: 2)
    print(p1 == p2)                        // true
    print(p1.description())                // "Point(x: 1, y: 2)"

    let json = p1.encode()                 // "{\"x\":1,\"y\":2}"
    let back = Point.decode(text: json)
    print(back == p1)                      // true
}
```

| 能力 | 状态 |
|------|------|
| 自动构造 | **已实现**（字段顺序） |
| 自动 `description()` | **已实现**（`"Type(field: value, ...)"`） |
| `@Equatable` 合成 `==` | **GA**（L1 stored `data class`） |
| `@Hashable` 合成 `hash(into:)` | **GA**（L1） |
| `@Codable` 合成 `encode`/`decode` | **GA**（L1 stored）；**L2** 关联值 enum 全自动 |
| Swift 式自动 `copy` | **未实现** |
| `Hashable` `Hashable` L2 generic | **未实现** |

> **不带宏也能用：** 不写 `@Equatable` / `@Hashable` / `@Codable` 仍有 `description()`；写宏会追加 witness。

无 Swift 式自动 `copy`（**未实现**）；当前用显式构造 / `with`：

```syx
func with(_ p: Point, x: Int) -> Point {
    return Point(x: x, y: p.y)
}
```

## ADT 变体族

封闭代数数据类型请用 `enum`（**不是** `sealed` 关键字——lexer 无此关键字）：

```syx
enum Result<T, E> {
    case ok(T)
    case err(E)
}
```

> **历史：** Phase 50 之前文档曾描述 `sealed class`；当前 lexer **不接受** `sealed`。ADT 用 `enum`；禁止子类化用 `final class`。

## actor

并发隔离对象（详见 [第 18 章](../../docs/adr/architecture-decisions.md)）：

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

let c = Counter()
await c.increment()                       // 必须 await
print(await c.current())                  // 1
```

- `actor` 状态对外隔离
- 跨 actor 访问须 `await`
- 隔离违规 → **A002** / **C001**

## 选择指南

| 场景 | 选 | 理由 |
|------|----|------|
| 简单数据载体、需 `==` / `Hashable` | `data class` | 自动合成、最少代码 |
| 需要共享可变状态、引用语义 | `class` | ARC、引用相等 |
| 并发安全的状态 | `actor` | 隔离、避免锁 |
| 值类型、栈分配、性能敏感 | `struct` | 零堆分配 |
| 封闭变体族（ADT） | `enum` | exhaustiveness + 模式匹配 |
| 表达错误、状态机 | `enum` | `Result` / `Optional` / `Status` |

## 内存语义对比

| 类型 | 内存 | 拷贝 | 共享 |
|------|------|------|------|
| `struct` | 栈 / 内嵌 | 值拷贝（部分实现） | 不共享 |
| `class` | 堆（ARC） | 引用赋值（共享） | 共享 |
| `actor` | 堆（隔离） | 引用 | 隔离队列 |
| `data class` | 堆（值语义） | 引用（不可变推荐） | 引用 |
| `enum` | 栈 / 堆（按 case） | 视 case | 视 case |

## 完整示例

```syx
@Equatable
data class User(name: String, age: Int)

class Account {
    let owner: User
    var balance: Int

    init(owner: User, balance: Int) {
        self.owner = owner
        self.balance = balance
    }

    func deposit(_ amount: Int) {
        balance = balance + amount
    }
}

actor Bank {
    var accounts: Array<Account> = []
    func open(_ a: Account) { accounts.append(a) }
    func total() -> Int {
        var sum = 0
        for a in accounts { sum = sum + a.balance }
        return sum
    }
}

func main() {
    let u = User(name: "Alice", age: 30)
    let a = Account(owner: u, balance: 100)
    a.deposit(50)
    print(a.balance)                       // 150
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S001** | 重复定义 | 改类型名 |
| **S004** | 字段类型不匹配 | 修正 |
| **S010** | 协议不符合 | 加一致性 |
| **A002** | 跨 actor 访问未 await | 加 `await` |
| **R007** | 对 Optional 字段 `!` | 用 `?.` / `if let` |

## 与 Swift 的差异

| Swift struct | SYX |
|--------------|-----|
| 完整值语义拷贝 | **部分实现** |
| `mutating func` | **已实现**（Phase 60+） |
| 完整 `class` 单继承 + `final` | **已实现** |
| `actor` 隔离 | **已实现** |
| `data class` 自动 `==`/`hash`/`encode` | **GA**（L1 stored） |
| Swift 式自动 `copy` | **未实现** |
| `isolated deinit` | **未实现**（class 显式 deinit 已实现） |
| `inout` 形参 | **未实现**；用 `&mut T` 借用（部分实现） |
| 嵌套类型 | **部分实现**（见 [第 21 章](../../docs/adr/architecture-decisions.md)） |

---

**下一章** → [属性](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 9 章 枚举](../../docs/adr/architecture-decisions.md) · [第 11 章 属性](../../docs/adr/architecture-decisions.md) · [第 14 章 继承](../../docs/adr/architecture-decisions.md) · [第 19 章 宏](../../docs/adr/architecture-decisions.md)
