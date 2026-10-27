# 访问控制

**实现状态：** `public` / `internal` / `private` / `fileprivate` **已实现**（S005 跨模块；fileprivate 同文件）；`open` **未实现**；`package` **未实现**

访问控制决定 **符号（变量、类型、函数、方法、属性、init）** 的可见性。SYX 四级可见性（`public` / `internal` / `private` / `fileprivate`），与 Swift 类似但 **`open` 关键字未实现**（用 `public` + `class` 表示可继承）。

## 级别

| 级别 | 跨模块 | 模块内 | 文件内 | 类型内 |
|------|--------|--------|--------|--------|
| `public` | ✅ | ✅ | ✅ | ✅ |
| `internal` | ❌ S005 | ✅ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ | ✅ |
| `private` | ❌ | ❌ | 同作用域 | ✅（Swift 风格） |

> **默认：** `internal`（无显式访问控制 → 模块内可见）。

## 示例

```syx
// 顶层
public func exported() { }                  // 跨模块
internal func moduleOnly() { }              // 模块内
private var secret = 0                       // 文件 / 作用域
fileprivate var alsoSecret = 0              // 文件内

class Container {
    public var open_field: Int = 0
    internal var internal_field: Int = 0
    private var private_field: Int = 0
}
```

## 跨模块

```syx
// 模块 A：math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func helper() -> Int {              // 跨模块不可见
    return 42
}
```

```syx
// 模块 B：main.sy
import math

func main() {
    print(math.add(a: 1, b: 2))              // ✅ public
    // print(math.helper())                  // ❌ S005：cross-module access to internal
}
```

测试：`fixtures/lsp/cross_module` · `examples/deps/`

## class 成员的访问级别

```syx
public class User {
    public let id: Int                       // 跨模块可见
    internal let team: String                // 模块内
    private let password: String             // 类内

    public init(id: Int, team: String, password: String) {
        self.id = id
        self.team = team
        self.password = password
    }

    public func publicMethod() { }           // 跨模块
    internal func internalMethod() { }       // 模块内
    private func privateMethod() { }         // 类内
}
```

> **类型可见性约束：** 成员的访问级别 **不能高于** 类型的访问级别（一个 `internal class` 不能有 `public func` 成员）。

## `public` + `class` = 可继承

```syx
public class Base {
    public init() {}
    public func greet() { print("base") }
}

public class Derived: Base {                 // ✅ 跨模块继承
    public override init() { super.init() }
    public override func greet() { print("derived") }
}
```

> **`open`：** SYX 不实现 `open` 关键字；`public class` 即「跨模块可继承 + 可重写」。`final class` 显式禁止继承。

## `fileprivate`

`fileprivate` 限定 **当前 `.sy` 文件** 内可见：

```syx
// file: util.sy
fileprivate var sharedState: Array<Int> = []

fileprivate func helper() { }

public func exposed() {
    helper()                                 // ✅ 同文件
    sharedState.append(1)
}
```

```syx
// file: main.sy
import util
// util.helper()                             // ❌ fileprivate
util.exposed()                              // ✅
```

## `private` 在类型内

```syx
class Counter {
    private var count: Int = 0

    func increment() { count = count + 1 }   // 类内访问
}

// let c = Counter()
// c.increment()                              // ✅ 通过 public 方法
// c.count                                    // ❌ private
```

> **当前实现：** `private` 默认按 Swift 5+ 规则（**类内** + 同文件 extension 可访问）；与 Swift `fileprivate` 类似但**不** 完全等同。Phase 后续可能拆分 `private` / `fileprivate` 严格语义。

## 协议一致性访问级别

```syx
protocol Describable {
    func describe() -> String
}

internal class User: Describable {           // 协议 + 类型同时 internal
    internal func describe() -> String { return "user" }
}
```

## 枚举 case 的访问

枚举 case 与枚举同级别；不能单独标注 `public` / `private`：

```syx
public enum Status {
    case ok
    case err
    case pending
}
```

## 跨模块扩展

`extension` 可在模块 B 中扩展模块 A 的类型（**Phase 60+**）：

```syx
// 模块 A
public class User { public init() {} }

// 模块 B
import a

extension a.User {
    public func extra() { print("extra") }   // 跨模块扩展
}
```

> **限制：** 不能为跨模块类型加 `required init` 或 retroactive conformance 警告（与 Swift 不同）。

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `public` | **已实现** |
| `internal`（默认） | **已实现** |
| `fileprivate` | **已实现** |
| `private`（类内 / 同 scope） | **已实现** |
| `open` | **未实现**（用 `public` + `class`） |
| `package` | **未实现**（规划中） |
| 跨模块 extension | **已实现**（Phase 60+） |
| `required` 构造器 | **未实现** |

## 完整示例

```syx
// module: userlib
public class User {
    public let id: Int
    public let name: String
    fileprivate var password: String

    public init(id: Int, name: String, password: String) {
        self.id = id
        self.name = name
        self.password = password
    }

    public func changePassword(_ new: String) {
        self.password = new                  // 类内访问 fileprivate
    }
}

public func greet(_ u: User) -> String {
    return "Hello, \(u.name)!"
}

fileprivate func internalHelper() -> String {
    return "internal"
}
```

```syx
// module: app
import userlib

func main() {
    let u = User(id: 1, name: "Alice", password: "secret")
    print(greet(u))                          // ✅ public
    u.changePassword("new")                  // ✅ public 方法
    // u.password                            // ❌ fileprivate
    // userlib.internalHelper()              // ❌ fileprivate
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S005** | 跨模块访问 internal / private | 改 public / 调整模块边界 |
| **S020** | 成员访问级别高于类型 | 改类型访问级别或成员降低 |

---

**下一章** → [模块与包](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 29 章 模块与包](../../docs/adr/architecture-decisions.md) · [第 10 章 结构体和类](../../docs/adr/architecture-decisions.md)
