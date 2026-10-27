# 泛型

**实现状态：** 函数 / 类型 / where 子句 / 协议约束 / 关联类型 / 单态化 **已实现**；泛型特化（运行时） **未实现**；泛型反射 **部分实现**（`reflect_specialization` Phase 77）

泛型编写类型安全的复用代码。SYX 采用 **单态化**（monomorphization）：编译器为每个具体类型参数组合生成专用代码。优点是运行时无虚调度；缺点是代码体积可能膨胀。

## 泛型函数

```syx
func id<T>(x: T) -> T { return x }
func swap<T>(a: T, b: T) -> (T, T) { return (b, a) }

let n = id(x: 42)                          // Int
let s = id(x: "hi")                        // String
let (x, y) = swap(a: 1, b: 2)              // (Int, Int)
```

| 元素 | 说明 |
|------|------|
| `<T>` | 类型参数列表 |
| `<T, U>` | 多类型参数 |
| `<T: Equatable>` | 协议约束 |
| `<T where T: Comparable>` | where 子句 |

测试：`fixtures/parity/generics.sy` · `fixtures/parity/generics.sy`

## 泛型类型

```syx
struct Box<T> {
    var value: T
    init(value: T) { self.value = value }
}

enum Optional<T> {
    case some(T)
    case none
}

data class Pair<A, B>(first: A, second: B)
```

- 泛型参数在类型名后
- 类型体内可使用 `T` 作任何类型位置
- 构造器可使用类型参数

## where 子句

```syx
func eq<T>(a: T, b: T) -> Bool where T: Equatable {
    return a == b
}

func allEqual<T: Equatable>(_ items: Array<T>) -> Bool {
    if (items.count == 0) { return true }
    let first = items[0]
    for i in 1..<items.count {
        if (items[i] != first) { return false }
    }
    return true
}
```

| where 形式 | 状态 |
|------------|------|
| `where T: P` 单约束 | **已实现** |
| `where T: P1, T: P2` 多约束 | **已实现** |
| `where T: P<U>` 关联类型约束 | **部分实现** |
| `where T == U` 同类型约束 | **已实现** |
| `where T: P where U: Q` 嵌套 | **部分实现** |

失败测试：`fixtures/parity/generics.sy`

## 协议约束

```syx
func printAll<T>(items: Array<T>) where T: CustomStringConvertible {
    for item in items {
        print(item.description())
    }
}

printAll(items: [1, 2, 3])                  // 1 2 3
// printAll(items: [User()])                // S004：User not CustomStringConvertible
```

测试：`fixtures/macros/generic_witness_constraint.sy`

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

> **自动推断：** 当 `add` / `get` 签名唯一确定 `Item` 时，可省略 `typealias Item`。

## 协议作为类型参数约束

```syx
func map<T, U>(items: Array<T>, f: (T) -> U) -> Array<U> {
    var result: Array<U> = []
    for item in items { result.append(f(item)) }
    return result
}

let doubled = map(items: [1, 2, 3], f: { x in x * 2 })   // [2, 4, 6]
```

## 单态化

```syx
func id<T>(x: T) -> T { return x }

id(x: 42)                                  // 生成 id_int
id(x: "hi")                                // 生成 id_str
```

> **过程：** `syx-mono` 收集所有具体类型实例化，生成专用函数代码。每个实例有独立符号，运行时**无虚调用**。

| 优点 | 缺点 |
|------|------|
| 运行时零开销（无虚调度） | 代码体积膨胀 |
| 内联优化友好 | 编译时间增长 |
| 调试易（每实例独立栈） | 跨模块特化暴露内部细节 |

## 泛型与集合

```syx
let arr: Array<Int> = [1, 2, 3]
let dict: [String: Int] = ["a": 1, "b": 2]
let set: Set<Int> = [1, 2, 3]

func first<T>(_ arr: Array<T>) -> T? {
    if (arr.isEmpty) { return nil }
    return arr[0]
}

let firstInt = first(arr)                   // Int?
let firstString = first([1.0, 2.0])         // Double?（推断）
```

## 泛型与函数

```syx
func compose<A, B, C>(f: (B) -> C, g: (A) -> B) -> (A) -> C {
    return { x in f(g(x)) }
}

let f = compose(f: { x in x + 1 }, g: { x in x * 2 })
print(f(3))                                 // 7
```

## 泛型 extension

```syx
extension Array where Element: Equatable {
    func unique() -> Array<Element> {
        var seen: Array<Element> = []
        for item in self {
            if (!seen.contains(item)) { seen.append(item) }
        }
        return seen
    }
}

[1, 2, 2, 3].unique()                      // [1, 2, 3]
```

详见 [第 22 章 扩展](../../docs/adr/architecture-decisions.md)。

## 泛型与 actor

```syx
actor Holder<T> {
    var value: T?
    init() { self.value = nil }
    func put(_ v: T) { self.value = v }
    func get() -> T? { return self.value }
}

let h = Holder<Int>()
await h.put(42)
let n = await h.get()                       // 42
```

## 泛型与协议组合

```syx
protocol A { func a() }
protocol B { func b() }

func both<T>(x: T) where T: A, T: B {
    x.a()
    x.b()
}
```

## 泛型默认值（部分实现）

```syx
class Holder<T = Int> {                     // ⚠️ 部分实现
    var value: T
    init(v: T) { self.value = v }
}

let h1 = Holder<Int>(v: 1)
let h2 = Holder(v: 2)                       // 推断为 Holder<Int>
```

> **状态：** 泛型类型参数默认值 **部分实现**（`fixtures/macros/generic_witness_constraint.sy`）；推荐显式标注。

## 完整示例

```syx
func zipMap<A, B, C>(_ a: Array<A>, _ b: Array<B>, _ f: (A, B) -> C) -> Array<C> {
    let n = a.count < b.count ? a.count : b.count
    var result: Array<C> = []
    for i in 0..<n {
        result.append(f(a[i], b[i]))
    }
    return result
}

let r = zipMap([1, 2, 3], ["a", "b", "c"]) { n, s in
    return "\(n)-\(s)"
}
print(r)                                    // ["1-a", "2-b", "3-c"]
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | 类型不匹配 | 检查泛型实参 |
| **S010** | 协议约束不满足 | 实现所需方法 |
| **S012** | 关联类型错误 | 加 `typealias` / 修正 |
| **P001** | 泛型语法错误 | 检查 `<T>` 语法 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 泛型函数 / 类型 | **已实现**（单态化） |
| where 子句 | **已实现** |
| 关联类型 | **已实现** |
| 泛型默认值 | **部分实现** |
| 类型级编程（HKT） | **未实现** |
| 运行时泛型特化反射 | **未实现**（MVP `reflect_specialization` Phase 77） |
| Swift 部分特化 | **未实现**（仅全量单态化） |

---

**下一章** → [不透明类型和封装协议类型](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：泛型形参和实参](../language-reference/08-generic-parameters-and-arguments.md) · [第 22 章 扩展](../../docs/adr/architecture-decisions.md) · [第 23 章 协议](../../docs/adr/architecture-decisions.md) · [第 25 章 不透明类型](../../docs/adr/architecture-decisions.md)
