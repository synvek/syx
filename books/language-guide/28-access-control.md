# Access Control

**Implementation status:** `public` / `internal` / `private` / `fileprivate` **Implemented** (S005 cross-module; `fileprivate` within the same file); `open` **Not implemented**; `package` **Not implemented**

Access control determines the visibility of **symbols (variables, types, functions, methods, properties, init)**. SYX has four visibility levels (`public` / `internal` / `private` / `fileprivate`), similar to Swift but the **`open` keyword is not implemented** (use `public` + `class` to express inheritable).

## Levels

| Level | Cross-module | Within module | Within file | Within type |
|------|--------|--------|--------|--------|
| `public` | ✅ | ✅ | ✅ | ✅ |
| `internal` | ❌ S005 | ✅ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ | ✅ |
| `private` | ❌ | ❌ | Same scope | ✅ (Swift style) |

> **Default:** `internal` (no explicit access control → visible within the module).

## Examples

```syx
// Top level
public func exported() { }                  // Cross-module
internal func moduleOnly() { }              // Within module
private var secret = 0                       // File / scope
fileprivate var alsoSecret = 0              // Within file

class Container {
    public var open_field: Int = 0
    internal var internal_field: Int = 0
    private var private_field: Int = 0
}
```

## Cross-Module

```syx
// Module A: math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func helper() -> Int {              // Not visible across modules
    return 42
}
```

```syx
// Module B: main.sy
import math

func main() {
    print(math.add(a: 1, b: 2))              // ✅ public
    // print(math.helper())                  // ❌ S005: cross-module access to internal
}
```

Tests: `fixtures/lsp/cross_module` · `examples/deps/`

## Access Levels of class Members

```syx
public class User {
    public let id: Int                       // Visible across modules
    internal let team: String                // Within module
    private let password: String             // Within the class

    public init(id: Int, team: String, password: String) {
        self.id = id
        self.team = team
        self.password = password
    }

    public func publicMethod() { }           // Cross-module
    internal func internalMethod() { }       // Within module
    private func privateMethod() { }         // Within the class
}
```

> **Type visibility constraint:** a member's access level **cannot be higher** than the type's (an `internal class` cannot have a `public func` member).

## `public` + `class` = Inheritable

```syx
public class Base {
    public init() {}
    public func greet() { print("base") }
}

public class Derived: Base {                 // ✅ Cross-module inheritance
    public override init() { super.init() }
    public override func greet() { print("derived") }
}
```

> **`open`:** SYX does not implement the `open` keyword; a `public class` means "inheritable + overridable across modules". `final class` explicitly forbids inheritance.

## `fileprivate`

`fileprivate` limits visibility to **the current `.sy` file**:

```syx
// file: util.sy
fileprivate var sharedState: Array<Int> = []

fileprivate func helper() { }

public func exposed() {
    helper()                                 // ✅ same file
    sharedState.append(1)
}
```

```syx
// file: main.sy
import util
// util.helper()                             // ❌ fileprivate
util.exposed()                              // ✅
```

## `private` Inside a Type

```syx
class Counter {
    private var count: Int = 0

    func increment() { count = count + 1 }   // Access within the class
}

// let c = Counter()
// c.increment()                              // ✅ through a public method
// c.count                                    // ❌ private
```

> **Current implementation:** `private` follows the Swift 5+ rule by default (**within the class** + accessible by an extension in the same file); similar to Swift's `fileprivate` but **not** fully equivalent. Later Phases may split the strict semantics of `private` / `fileprivate`.

## Access Level of Protocol Conformance

```syx
protocol Describable {
    func describe() -> String
}

internal class User: Describable {           // Protocol and type are both internal
    internal func describe() -> String { return "user" }
}
```

## Access to Enum Cases

Enum cases share the enum's level; they cannot be annotated `public` / `private` individually:

```syx
public enum Status {
    case ok
    case err
    case pending
}
```

## Cross-Module Extensions

An `extension` in module B may extend a type from module A (**Phase 60+**):

```syx
// Module A
public class User { public init() {} }

// Module B
import a

extension a.User {
    public func extra() { print("extra") }   // Cross-module extension
}
```

> **Limit:** you cannot add a `required init` to a cross-module type, nor get a retroactive conformance warning (unlike Swift).

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `public` | **Implemented** |
| `internal` (default) | **Implemented** |
| `fileprivate` | **Implemented** |
| `private` (within the class / same scope) | **Implemented** |
| `open` | **Not implemented** (use `public` + `class`) |
| `package` | **Not implemented** (planned) |
| Cross-module extension | **Implemented** (Phase 60+) |
| `required` initializers | **Not implemented** |

## Complete Example

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
        self.password = new                  // Accessing fileprivate within the class
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
    u.changePassword("new")                  // ✅ public method
    // u.password                            // ❌ fileprivate
    // userlib.internalHelper()              // ❌ fileprivate
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S005** | Accessing internal / private across modules | Change it to `public` / adjust the module boundary |
| **S020** | A member's access level higher than the type's | Change the type's level or lower the member's |

---

**Next chapter** → [Modules and Packages](29-modules-and-packages.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 29, Modules and Packages](29-modules-and-packages.md) · [Chapter 10, Structures and Classes](10-structures-and-classes.md)
