# 声明

**实现状态：** 顶层与成员声明 **已实现**；继承 GA（Phase 51–55 + Phase 66 vtable）；`final class` Phase 60；`subscript` 部分实现；`convenience init` / `required init` **未实现**

模块由顶层 `Decl` 列表组成，入口为 `func main()`。

---

## 分类

```
decl ::= import-decl
      | var-global-decl
      | function-decl
      | init-decl
      | deinit-decl
      | class-decl
      | struct-decl
      | enum-decl
      | actor-decl
      | protocol-decl
      | extension-decl
      | data-class-decl
      | global-actor-decl
      | typealias-decl
      | operator-decl
      | macro-decl
      | attribute-decl           -- @attached 等
```

## import

```
import-decl ::= 'import' identifier ('.' identifier)* ('as' identifier)?
```

```syx
import Math
import local.deps as Deps
import utils.helpers as H
```

> 跨模块仅 `public` 符号可访问；internal → **S005**。

## 全局变量

```
var-global-decl ::= access? attrs? ('let' | 'var') identifier (':' type)? ('=' expr)?
```

```syx
let globalConst = 42
var globalVar: Int = 0
public var shared: String = "shared"        // 跨模块
```

> 全局变量 init 在进程启动时执行一次。

## function

```
function-decl ::= access? attrs? ('async')? 'func' identifier generic-params?
                  '(' param-list? ')' ('->' type)? throws-clause? effect-clause?
                  where-clause? ('=' expr | block)
```

```syx
func main() {
    …
}

public func add(a: Int, b: Int) -> Int {
    return a + b
}

async func fetch() -> String {
    return await httpGet()
}

func risky() throws -> Int {
    throw "fail"
}

func pureCalc(x: Int) -> Int requires Pure {
    return x + 1
}
```

| 子句 | 说明 |
|------|------|
| `access` | `public` / `internal` / `private` / `fileprivate` |
| `async` | 异步函数 |
| `throws` | 错误签名（`throws T` typed throws **部分实现**） |
| `requires Pure` / `requires IO` | 效果注解 |
| `where T: Equatable` | 约束子句 |
| 泛型 | `func id<T>(x: T) -> T` |
| 单表达式 | `func f(x: Int) -> Int = x * 2`（**部分实现**） |

## init

```
init-decl ::= access? attrs? ('async')? 'init' '(' param-list? ')' (throws-clause)? where-clause? block
```

```syx
init(x: Int) {
    self.field = x
}

init?(value: Int) {
    if (value < 0) { return nil }
    self.value = value
}

init throws {
    // ...
}
```

| 形式 | 状态 |
|------|------|
| `init(params)` 指定 init | **已实现** |
| `init?` 失败构造 | **已实现** |
| `init!` 隐式 unwrap | **未实现** |
| `convenience init` | **未实现** |
| `required init` | **未实现** |

## deinit

```
deinit-decl ::= access? attrs? 'deinit' block
```

```syx
deinit {
    print("cleanup")
}
```

> 仅 `class` / `actor` 成员；顶层 `deinit` → **P001**。
> 状态：**部分实现**（class ARC 归零触发；struct 不支持）。

## class

```
class-decl ::= access? attrs? ('final')? 'class' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' class-member* '}'
```

```syx
class Person {
    var name: String
    let id: Int
    init(name: String, id: Int) {
        self.name = name
        self.id = id
    }
    func greet() {
        print("Hi, \(self.name)")
    }
    deinit { print("bye") }
}

final class Token {                         // 禁止继承
    let value: String
    init(value: String) { self.value = value }
}

class Derived: Base {                       // 继承
    override func foo() { super.foo() }
}
```

成员可含 `var` / `let` / `func` / `init` / `deinit` / 计算属性 / `subscript`（**部分实现**）。

## struct

```
struct-decl ::= access? attrs? 'struct' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' struct-member* '}'
```

```syx
struct Point {
    var x: Int
    var y: Int

    func distance(to other: Point) -> Double {
        let dx = Double(x - other.x)
        let dy = Double(y - other.y)
        return (dx * dx + dy * dy).squareRoot()
    }

    mutating func translateBy(dx: Int, dy: Int) {
        x = x + dx
        y = y + dy
    }
}
```

> 值类型字段聚合（运行时 MVP）；`mutating func` 标记会修改 self 的方法。

## data class

```
data-class-decl ::= access? attrs? 'data' 'class' identifier generic-params?
                    '(' (var-or-let identifier ':' type (',' var-or-let identifier ':' type)*)? ')' where-clause?
                    '{' class-member* '}'
```

```syx
@Equatable
@Hashable
@Codable
data class User(name: String, age: Int) {
    func describe() -> String { return "User(\(name), \(age))" }
}
```

> **Phase 50–52 证明宏**（stored 属性 `data class`）。L1：自动 `==` / `hash` / `encode` / `decode`。L2：关联值 enum 全自动（Phase 129）。
> 无 Swift 式自动 `copy`（**未实现**）。

## enum

```
enum-decl ::= access? attrs? 'enum' identifier generic-params?
             (':' type)? where-clause? '{' enum-case* (func | var | init)* '}'
enum-case ::= 'case' identifier ('(' (identifier (':' type)? (',' identifier (':' type)?)*)? ')')?
```

```syx
enum Direction { case north; case south; case east; case west }

enum NetworkResult {
    case success(String)
    case failure(code: Int, message: String)

    func isSuccess() -> Bool {
        switch (self) {
        case .success { return true }
        case .failure { return false }
        }
    }
}

enum Optional<T> { case some(T); case none }   // 已预声明
```

> 枚举可包含方法、计算属性、`init`；与 `class` 不同的内存布局（discriminated union）。

## actor

```
actor-decl ::= access? attrs? ('global')? 'actor' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' actor-member* '}'
```

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

global actor MainActor {                    // 全局 actor
}
```

> actor 状态对外隔离；外部访问须 `await`（**A002**）。

## protocol

```
protocol-decl ::= access? attrs? 'protocol' identifier generic-params?
                 (':' type (',' type)*)? where-clause? '{' protocol-member* '}'
```

```syx
protocol Describable {
    func description() -> String
    var name: String { get }
}

protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}

protocol Iterator {
    func next() -> String?
}
```

> 协议成员可包含 `func` / `var` / `init` / `associatedtype`；**可** 包含默认实现（Phase 50+）。

## extension

```
extension-decl ::= access? attrs? 'extension' type where-clause? '{' extension-member* '}'
```

```syx
extension Int {
    func doubled() -> Int { return self + self }
    var isEven: Bool { get { return self % 2 == 0 } }
}

extension Array where Element: Equatable {
    func unique() -> Array<Element> { … }
}

extension MyType: Describable {
    func description() -> String { return "MyType" }
}
```

> **where 约束：** Phase 40 单约束；Phase 42 多约束。
> **跨模块：** Phase 60+ 允许（与 Swift 不同）。

## global actor

```
global-actor-decl ::= access? attrs? 'global' 'actor' identifier '{' actor-member* '}'
```

```syx
global actor MainActor { }
global actor BackgroundActor { }
```

> 全局 actor 跨平台：UI 线程、GPU 线程等。

## typealias

```
typealias-decl ::= access? 'typealias' identifier generic-params? '=' type
```

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String
typealias StringMap = [String: String]
```

> **不创建新类型**；仅同义词。**Phase 40 GA**。

## 属性（成员字段）

```
property-decl ::= access? attrs? ('static')? ('let' | 'var') identifier (':' type)? ('=' expr)?
               |  access? attrs? ('static')? 'var' identifier ':' type computed-property
```

```syx
var count: Int = 0
let id: String
static let maxValue: Int = 100
static var debug: Bool = false
```

计算属性（**Phase 134–138 partial GA**）：

```syx
var doubled: Int {
    get { return self.base * 2 }
    set { self.base = newValue / 2 }
}
```

`subscript` 声明：

```syx
subscript(row: Int, col: Int) -> Int {
    get { return data[row][col] }
    set { data[row][col] = newValue }
}
```

> **状态：** 自定义 `subscript` **部分实现**（Phase 40/41）。

## 访问控制

```
access ::= 'public' | 'internal' | 'private' | 'fileprivate'
```

| 级别 | 跨模块 | 模块内 | 文件内 | 同 scope |
|------|--------|--------|--------|----------|
| `public` | ✅ | ✅ | ✅ | ✅ |
| `internal` | ❌ S005 | ✅ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ | ✅ |
| `private` | ❌ | ❌ | ✅ | ✅ |

> `open` **未实现**（用 `public` + `class` 表示可继承）。

详见 [语言指南：访问控制](../language-guide/28-access-control.md)。

---

**诊断：** S001 / S004 / S005 / S020 / S051–S055 / P001

**用法示例** → [语言指南：函数](../language-guide/07-functions.md) · [结构体和类](../language-guide/10-structures-and-classes.md) · [枚举](../language-guide/09-enumerations.md) · [并发](../language-guide/18-concurrency.md) · [协议](../language-guide/23-protocols.md) · [扩展](../language-guide/22-extensions.md) · [属性](../language-guide/11-properties.md)

**参见** → [类型](02-types.md) · [泛型形参](08-generic-parameters-and-arguments.md) · [协议](09-protocols-and-conformances.md) · [特性](06-attributes.md)
