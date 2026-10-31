# Extensions

**Implementation status:** `extension` methods / protocol conformance / computed properties (**Phase 136 GA**) / constrained extensions **Implemented** (Phase 40–42); nested types inside an `extension` **Partially implemented**; cross-module `extension` **Implemented** (Phase 60 module-local witness)

An `extension` adds capability (methods, computed properties, protocol conformances, initializers) to an existing type without modifying the original declaration. SYX follows the Swift style; multi-constraint `where` clauses have been supported since Phase 42.

## extension Methods

```syx
extension Int {
    func doubled() -> Int { return self + self }
    func times(_ f: () -> Void) {
        for _ in 0..<self { f() }
    }
}

func main() {
    print(3.doubled())                    // 6
    3.times({ print("hi") })              // "hi" × 3
}
```

> **`self` inside an extension** refers to the instance of the extended type (same as inside a class method).

Source: `crates/syx-driver/tests/fixtures/types/greeter_extension.sy`

## extension Computed Properties

**Phase 136 GA** (`lang-props-v4:extension-computed-mvp`):

```syx
extension Int {
    var isEven: Bool { get { return self % 2 == 0 } }
    var squared: Int {
        get { return self * self }
        set { /* illustration only; may reset a stored field */ }
    }
}

print(4.isEven)                          // true
print(3.squared)                         // 9
```

| Form | Status |
|------|------|
| get-only | **Implemented** (Phase 134) |
| get / set | **Implemented** (Phase 135) |
| Cross-module | **Implemented** (Phase 60 module-local) |

## Protocol Conformance

```syx
protocol Named {
    var name: String { get }
}

extension User: Named {
    var name: String { return "User(\(self.id))" }
}

class User {
    let id: Int
    init(id: Int) { self.id = id }
}
```

> **Binding order does not matter:** an `extension` may be written before the class declaration; the compiler later scans all extensions and synthesizes witnesses.

## Protocol Default Implementations

```syx
protocol Greetable {
    func greet() -> String
}

extension Greetable {
    func greet() -> String { return "Hello" }    // Default
}

class User: Greetable {
    // Inherits the default → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }      // Override
}
```

## Constrained Extensions (where)

```syx
extension Array where Element: Equatable {
    func unique() -> Array<Element> {
        var seen: Array<Element> = []
        for item in self {
            if (!seen.contains(item)) {
                seen.append(item)
            }
        }
        return seen
    }
}

let nums = [1, 2, 2, 3, 3, 3]
print(nums.unique())                      // [1, 2, 3]
```

### Multiple Constraints

Supported **since Phase 42**:

```syx
extension Array where Element: Equatable, Element: Hashable {
    func toSet() -> Set<Element> {
        var s: Set<Element> = []
        for item in self { s.insert(item) }
        return s
    }
}
```

> **Constraint not satisfied:** compile error **S010**. Tests: `fixtures/format/extension_where_mvp` / `fixtures/format/extension_where_mvp` / `fixtures/format/extension_where_mvp`.

## extension Initializers

```syx
struct Point {
    var x: Int
    var y: Int
}

extension Point {
    init() {                              // Convenience init
        self.init(x: 0, y: 0)
    }
}

let p = Point()                            // (0, 0)
```

> **Limit:** an `init` inside an `extension` must delegate to a designated init of the original type; it cannot define new stored fields.

## Cross-Module extension

An `extension` in module A may extend a type from module B:

```syx
// In module A
extension String {
    func swaCount() -> Int { return self.count }
}
```

> **Current implementation:** cross-module extensions have been fully supported since Phase 60; integrated with module-local witnesses. Swift's same-module restriction is relaxed (**note**: this is SYX behavior; Swift does not allow cross-module extensions).

## Adding Methods to Basic Types

```syx
extension Int {
    func clamp(_ lo: Int, _ hi: Int) -> Int {
        if (self < lo) { return lo }
        if (self > hi) { return hi }
        return self
    }
}

let x = 100.clamp(0, 10)                  // 10
```

## extension and Access Control

```syx
public extension Int {
    public func toHex() -> String { return "" }   // Must be annotated explicitly
}
```

> **Current status:** `public` / `internal` annotations on extensions and members are **Implemented** (S005 cross-module checking).

## extension and KeyPath

```syx
extension User {
    var nameKp: KeyPath<User, String> { return \User.name }
}
```

> **Status:** KeyPath is usable inside extensions (**Partially implemented**).

## Complete Example

```syx
protocol Describable {
    func description() -> String
}

@Equatable
data class User(id: Int, name: String)

extension User: Describable {
    func description() -> String { return "User(\(id): \(name))" }
}

extension Array where Element: User {
    func findByName(_ n: String) -> User? {
        for u in self {
            if (u.name == n) { return u }
        }
        return nil
    }
}

extension Int {
    var isEven: Bool { return self % 2 == 0 }
    var isOdd: Bool  { return !self.isEven }
    func clamp(_ lo: Int, _ hi: Int) -> Int {
        if (self < lo) { return lo }
        if (self > hi) { return hi }
        return self
    }
}

func main() {
    let users = [User(id: 1, name: "Alice"), User(id: 2, name: "Bob")]
    print(users.findByName("Bob")?.description() ?? "n/a")

    let n = 42
    print(n.isEven, n.clamp(0, 10))         // false 10
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S001** | An extension method conflicts with an existing one | Rename it |
| **S010** | `where` constraint not satisfied | Check protocol conformance |
| **S005** | An extension accessing an internal symbol across modules | Change it to `public` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `extension` methods / properties | **Implemented** |
| `extension` protocol conformance | **Implemented** |
| Cross-module `extension` | **Implemented** (Phase 60; Swift does not allow it) |
| `extension` where with multiple constraints | **Implemented** (Phase 42) |
| `init` inside an `extension` | **Implemented** (delegating to a designated init) |
| Nested types inside an `extension` | **Partially implemented** |
| `subscript` inside an `extension` | **Partially implemented** |
| Stored fields inside an `extension` | **Not implemented** (only init / computed properties) |
| Retroactive conformance warning | **Not implemented** (SYX allows cross-module extensions) |

---

**Next chapter** → [Protocols](23-protocols.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Language Reference: Protocols and Conformances](../language-reference/09-protocols-and-conformances.md) · [Chapter 11, Properties](11-properties.md) · [Chapter 23, Protocols](23-protocols.md)
