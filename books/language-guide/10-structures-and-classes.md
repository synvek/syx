# Structures and Classes

**Implementation status:** `struct` / `class` / `data class` / `actor` / `enum` **Implemented** (including `final class` Phase 60, `data class` Phase 50–52 proof macros); inheritance and polymorphism **GA** (Phase 51–55 + Phase 66 vtable)

SYX provides five kinds of type declaration: `struct` (value), `class` (reference), `data class` (immutable record), `actor` (concurrency isolation), and `enum` (ADT, see [Chapter 9](09-enumerations.md)). This chapter explains "**which one to use when**" and their **memory semantics**.

## struct

A value-type field aggregate. Since Phase 70+, value semantics have been gradually becoming GA (earlier it was an MVP):

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

| Feature | Status |
|------|------|
| Fields `var` / `let` | **Implemented** |
| Construction (in field order) | **Implemented**: `Point(1, 2)` |
| `mutating func` | **Partially implemented** (basically usable since Phase 60) |
| Full value-type copy | **Partially implemented** (deep-copy boundaries still evolving) |
| `Codable` synthesis | Through the `@Codable` macro (`data class` preferred) |

```syx
struct Counter {
    var value: Int = 0
    mutating func increment() { value = value + 1 }
}

var c = Counter()
c.increment()
print(c.value)                             // 1
```

> **`mutating`:** marks methods that modify `self`; the caller must use `var` (not `let`). Calling an unmarked-but-mutating method on a `let` → **S020**.

## class

A reference-type object. A SYX class uses **ARC** reference counting plus the class `ObjectRef` (**since Phase 73**):

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

    let g2 = g                              // Shared reference
    g2.name = "SYX"
    print(g.name)                           // "SYX" (same object)
}
```

| Feature | Status |
|------|------|
| Fields `var` / `let` | **Implemented** |
| `init` | **Implemented** (see [Chapter 15](15-initialization.md)) |
| `deinit` | **Partially implemented** (triggered when a class ARC count reaches zero, Phase 102/103) |
| Inheritance | **GA** (Phase 51–55 + Phase 66 vtable) |
| `override` | **GA** (polymorphic dispatch) |
| Reference semantics | **Implemented** |

Source: `crates/syx-driver/tests/fixtures/parity/inheritance_dispatch.sy`

### final class

Since Phase 60, `final class` declares a closed type that forbids subclassing:

```syx
final class Token {
    let value: String
    init(value: String) { self.value = value }
}
```

- `final` forbids subclassing
- `final` lets the compiler devirtualize (static calls)
- For ADT variant families, use `enum` (`switch` exhaustiveness MVP)

## data class

A declarative syntax for immutable records. The **Phase 50–52 proof macros** synthesize `==` / `hash` / `encode` / `decode` automatically on a stored-property `data class`:

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

| Capability | Status |
|------|------|
| Automatic construction | **Implemented** (field order) |
| Automatic `description()` | **Implemented** (`"Type(field: value, ...)"`) |
| `@Equatable` synthesizing `==` | **GA** (L1 stored `data class`) |
| `@Hashable` synthesizing `hash(into:)` | **GA** (L1) |
| `@Codable` synthesizing `encode`/`decode` | **GA** (L1 stored); **L2** associated-value enums fully automatic |
| Swift-style automatic `copy` | **Not implemented** |
| `Hashable` L2 generics | **Not implemented** |

> **Usable without macros:** even without `@Equatable` / `@Hashable` / `@Codable`, you still get `description()`; writing the macros adds witnesses.

There is no Swift-style automatic `copy` (**Not implemented**); use explicit construction / `with` for now:

```syx
func with(_ p: Point, x: Int) -> Point {
    return Point(x: x, y: p.y)
}
```

## ADT Variant Families

For closed algebraic data types, use an `enum` (**not** the `sealed` keyword — the lexer has no such keyword):

```syx
enum Result<T, E> {
    case ok(T)
    case err(E)
}
```

> **History:** documentation before Phase 50 described a `sealed class`; the current lexer does **not** accept `sealed`. Use `enum` for ADTs and `final class` to forbid subclassing.

## actor

A concurrency-isolated object (see [Chapter 18](18-concurrency.md)):

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

let c = Counter()
await c.increment()                       // await is required
print(await c.current())                  // 1
```

- `actor` state is isolated from the outside
- Cross-actor access requires `await`
- Isolation violations → **A002** / **C001**

## Selection Guide

| Scenario | Choose | Reason |
|------|----|------|
| Simple data carrier, needs `==` / `Hashable` | `data class` | Automatic synthesis, minimal code |
| Shared mutable state, reference semantics | `class` | ARC, reference equality |
| Concurrency-safe state | `actor` | Isolation, no locks |
| Value type, stack allocated, performance sensitive | `struct` | Zero heap allocation |
| Closed variant family (ADT) | `enum` | Exhaustiveness + pattern matching |
| Expressing errors, state machines | `enum` | `Result` / `Optional` / `Status` |

## Memory Semantics Compared

| Type | Memory | Copy | Sharing |
|------|------|------|------|
| `struct` | Stack / inlined | Value copy (partially implemented) | Not shared |
| `class` | Heap (ARC) | Reference assignment (shared) | Shared |
| `actor` | Heap (isolated) | Reference | Isolation queue |
| `data class` | Heap (value semantics) | Reference (immutable recommended) | Reference |
| `enum` | Stack / heap (per case) | Depends on the case | Depends on the case |

## Complete Example

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

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S001** | Duplicate definition | Rename the type |
| **S004** | Field type mismatch | Fix it |
| **S010** | Protocol not satisfied | Add the conformance |
| **A002** | Cross-actor access without await | Add `await` |
| **R007** | `!` on an Optional field | Use `?.` / `if let` |

## Differences from Swift

| Swift struct | SYX |
|--------------|-----|
| Full value-semantics copy | **Partially implemented** |
| `mutating func` | **Implemented** (Phase 60+) |
| Full `class` single inheritance + `final` | **Implemented** |
| `actor` isolation | **Implemented** |
| `data class` automatic `==`/`hash`/`encode` | **GA** (L1 stored) |
| Swift-style automatic `copy` | **Not implemented** |
| `isolated deinit` | **Not implemented** (an explicit class `deinit` is implemented) |
| `inout` parameters | **Not implemented**; borrow with `&mut T` (partially implemented) |
| Nested types | **Partially implemented** (see [Chapter 21](21-nested-types.md)) |

---

**Next chapter** → [Properties](11-properties.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 9, Enumerations](09-enumerations.md) · [Chapter 11, Properties](11-properties.md) · [Chapter 14, Inheritance](14-inheritance.md) · [Chapter 19, Macros](19-macros.md)
