# 泛型形参和实参

**实现状态：** 泛型函数 / 类型 / where 子句 / 协议约束 / 单态化 **已实现**；关联类型 / 泛型默认值 / 泛型特化反射 **部分实现**

---

## 泛型形参

```
generic-params ::= '<' identifier (',' identifier)* ('=' type)? (',' identifier ('=' type)?)* '>'
                |  '<' identifier (':' constraint)? (',' identifier (':' constraint)?)* ('where' where-clause)? '>'
```

泛型形参声明在类型或函数名后：

```syx
func swap<T>(a: T, b: T) -> (T, T) { … }

data class Box<T>(value: T)

enum Optional<T> {
    case some(T)
    case none
}

enum Result<T, E> {
    case ok(T)
    case err(E)
}
```

> `type_params: Vec<SmolStr>` 于 AST `Decl::Function` / `Decl::Class` / `Decl::Struct` 等。

## 泛型实参

```
generic-args ::= '<' type (',' type)* '>'
```

```syx
let b = Box<Int>(value: 42)
let x = id(x: 42)                            // T = Int（推断）
let y = id<String>(x: "hi")                  // 显式
```

## 显式与推断

| 调用 | 行为 |
|------|------|
| `id(x: 42)` | 推断 `T = Int` |
| `id<String>(x: "hi")` | 显式 `T = String` |
| `Box<Int>(value: 42)` | 必须显式（构造器无参数可推断） |
| `arr.map<U>(f: { x in ... })` | 显式返回类型 |

## where 子句

```
where-clause ::= 'where' where-pred (',' where-pred)*
where-pred   ::= type-identifier ':' type
              |  type-identifier '==' type
              |  type-identifier conformance
```

```syx
func eq<T>(a: T, b: T) -> Bool where T: Equatable {
    return a == b
}

func map<T, U>(items: Array<T>, f: (T) -> U) -> Array<U>
    where T: Equatable, U: Comparable {
    …
}
```

> `WherePred { type_name, constraint }`。
> **多约束：** Phase 42 起 `where T: P1, T: P2` 合法；不满足 → **S010**。

**诊断：** S004（where 不满足）/ 专用泛型错误

## 协议作为约束

```syx
func printAll<T>(items: Array<T>) where T: CustomStringConvertible {
    for item in items {
        print(item.description())
    }
}
```

**测试：** `fixtures/macros/generic_witness_constraint.sy`

## 关联类型与 witness

```syx
protocol Container {
    associatedtype Item
    func add(_ item: Item)
    func get(at: Int) -> Item
}

class IntStack: Container {
    typealias Item = Int
    …
}
```

> 协议关联类型与泛型 witness 交互见 [协议章](../../docs/adr/architecture-decisions.md)。
> **实现状态：** **部分实现**（`fixtures/macros/generic_witness_constraint.sy`、`fixtures/macros/generic_witness_constraint.sy`）。

## 单态化

```
monomorphization ≡ ∀ (Type, …) instance of (Generic<…>) → specialized copy
```

编译器为每个具体类型参数组合生成专用函数/类型代码（`syx-mono`）。

```syx
func id<T>(x: T) -> T { return x }

id(x: 42)                                    // 生成 id_int
id(x: "hi")                                  // 生成 id_str
```

| 优点 | 缺点 |
|------|------|
| 运行时零开销（无虚调度） | 代码体积膨胀 |
| 内联优化友好 | 编译时间增长 |
| 调试易（每实例独立栈） | 跨模块特化暴露内部细节 |

## 泛型与 extension

```syx
extension Array where Element: Equatable {
    func unique() -> Array<Element> { … }
}
```

详见 [声明 5](../../docs/adr/architecture-decisions.md) 与 [语言指南 扩展](../language-guide/22-extensions.md)。

## 泛型与函数

```syx
func compose<A, B, C>(f: (B) -> C, g: (A) -> B) -> (A) -> C {
    return { x in f(g(x)) }
}

let f = compose(f: { x in x + 1 }, g: { x in x * 2 })
print(f(3))                                    // 7
```

## 泛型默认值（部分实现）

```syx
class Holder<T = Int> {                        // ⚠️ 部分实现
    var value: T
    init(v: T) { self.value = v }
}
```

> **状态：** 泛型类型参数默认值 **部分实现**（`fixtures/macros/generic_witness_constraint.sy`）；推荐显式标注。

## 关联类型默认值

```syx
protocol Container {
    associatedtype Item = Int                  // 关联类型默认
    func add(_ item: Item)
}
```

> **状态：** 关联类型默认值 **部分实现**（Phase 50+）；实现类可省略 `typealias Item = Int`。

## 泛型特化

| 能力 | 状态 |
|------|------|
| 单态化 | **已实现** |
| 运行时特化 | **未实现** |
| `reflect_specialization` | **部分实现**（Phase 77） |

## 类型级编程（HKT）

SYX **不**支持 Higher-Kinded Types（类型构造子作参数）。例如 `func map<F<_>>(…)` **未实现**。

## 递归泛型

```syx
indirect enum Tree {                           // ⚠️ indirect 未实现
    case leaf(Int)
    case branch(Tree, Tree)
}

// 替代：class 包装
class TreeNode {
    var value: Int
    var left: TreeNode?
    var right: TreeNode?
    init(value: Int) { self.value = value }
}
```

> `indirect enum` **未实现**（规划中）。

## 未实现

| 特性 | 状态 |
|------|------|
| 泛型特化（运行时） | **未实现** |
| 类型级编程（HKT） | **未实现** |
| 完整泛型默认值 | **部分实现** |
| 递归泛型（indirect） | **未实现** |
| 泛型 typealias | **部分实现** |

## 完整示例

```syx
func zip<A, B, C>(_ a: Array<A>, _ b: Array<B>, _ f: (A, B) -> C) -> Array<C> {
    let n = a.length < b.length ? a.length : b.length
    var result: Array<C> = []
    for i in 0..<n {
        result.append(f(a[i], b[i]))
    }
    return result
}

let r = zip([1, 2, 3], ["a", "b", "c"]) { n, s in
    return "\(n)-\(s)"
}
print(r)                                       // ["1-a", "2-b", "3-c"]
```

---

**诊断：** S004 / S010 / S012

**用法示例** → [语言指南：泛型](../language-guide/24-generics.md)

**参见** → [类型](../../docs/adr/architecture-decisions.md) · [声明](../../docs/adr/architecture-decisions.md) · [协议](../../docs/adr/architecture-decisions.md) · [语法摘要](../../docs/adr/architecture-decisions.md)
