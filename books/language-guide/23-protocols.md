# Protocols

**Implementation status:** protocol declarations / conformance / witness tables / default implementations / associated types **Implemented**; `Self` constraints / protocol extension where **Partially implemented**; existential types `any P` GA; `class-only protocol` **Not implemented** (use a `class` constraint)

A protocol defines a capability contract (a list of methods / properties / associated types); types satisfy a protocol through conformance. The SYX witness-table dispatch mechanism is similar to Swift's; user `Sequence` / `Iterator` custom `for-in` is supported (Phase 71).

## Protocol Declarations

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

| Element | Description |
|------|------|
| `func name(params) -> R` | Required method |
| `var name: T { get }` | Required get-only property |
| `var name: T { get set }` | Required get + set |
| `static func name` | Required type method |
| `init(params)` | Required initializer |
| `associatedtype X` | Associated type |
| `typealias X = Y` | Associated type default (**Partially implemented**) |

## Conformance

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

- `class` / `struct` / `enum` may all implement protocols
- Protocol conformance is declared at a **single point** within a module (avoiding multi-file conflicts)

## Default Implementations

```syx
protocol Describable {
    func description() -> String
    func description() -> String { return "<no description>" }   // Default
}

class Foo: Describable {
    // Not implementing description() inherits the default
}

class Bar: Describable {
    func description() -> String { return "bar" }                  // Override
}

print(Foo().description())                    // "<no description>"
print(Bar().description())                    // "bar"
```

## Associated Types

```syx
protocol Container {
    associatedtype Item
    func add(_ item: Item)
    func get(at: Int) -> Item
}

class IntStack: Container {
    typealias Item = Int                      // Associated type binding
    var items: Array<Int> = []

    func add(_ item: Int) { items.append(item) }
    func get(at i: Int) -> Int { return items[i] }
}
```

> **Automatic associated-type inference:** when the `add` / `get` signatures uniquely determine `Item`, `typealias Item = …` may be omitted.

## Protocols as Types

```syx
protocol Describable {
    func description() -> String { return "witness" }
}
class Foo: Describable {}
let desc: any Describable = Foo()
print(desc.description())                    // Dispatched through the witness table
```

`any P` is an **existential type**; dispatch at runtime goes through a witness table. See [Chapter 25, Opaque Types](25-opaque-types.md).

## The Iterator Protocol

`Iterator` is the core protocol for SYX `for-in` integration (ADR-223 / `M2-S1-01` stable subset):

```syx
protocol Iterator {
    func next() -> Int?
}
```

- It must have `func next()` (missing → **S006**)
- Returning `nil` means the end

> **Return type (stable subset):** the pinnable subset is `Int?` (`nil` terminates); the generic `Element?` full type surface remains open. A custom `class: Iterator` can be traversed by `for-in` (Phase 71+).

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

> **Phase 71 three-back-end strict:** custom `Iterator` `for-in` behaves identically on the VM / unified MIR / WASM back ends. Pinning test: `m2_s1_01_collection_sequence_iterator`.

## The Sequence Protocol

```syx
protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}
```

> **Status:** `Sequence.makeIterator` is **still partial** (the `for-in` main path goes through `: Iterator` / built-in Array · String · Range); the `Element` associated type is registered. This is **not** `makeIterator` GA.

## Built-in Protocols

| Protocol | Description | Implicit conformance |
|------|------|------------|
| `Equatable` | `==` | Built-in value types |
| `Hashable` | `hash(into:)` | Built-in value types |
| `CustomStringConvertible` | `.description()` | Built-in value types |
| `CustomDebugStringConvertible` | Debug description | Built-in value types |
| `Codable` | Serialization | `data class` + `@Codable` (L1 GA; L2 partial) |
| `Iterator` | `next() -> Int?` (subset) | for-in stable subset **implemented** (ADR-223) |
| `Sequence` / `Collection` | Iteration / Index | **Partially implemented** (`makeIterator` / the full Collection Index surface remains open) |
| `Sendable` | Cross-isolation safety | Explicit annotation |
| `Error` | Error type | User-defined |
| `Comparable` | Ordered comparison | Numeric types GA; custom types need `compare` |

Defined in `crates/syx-stdlib/src/protocols.rs`.

## Synthesized Protocol Conformance

The `@Equatable` / `@Hashable` / `@Codable` macros synthesize witnesses automatically on `data class` / associated-value enums:

```syx
@Equatable
@Hashable
@Codable
data class Point(x: Int, y: Int)
```

See [Chapter 19, Macros](19-macros.md#proof-macros-l1-ga--l2-partial-phase-5052119).

## Protocols as Generic Constraints

```syx
func printAll<T: CustomStringConvertible>(items: Array<T>) {
    for item in items {
        print(item.description())
    }
}

printAll(items: [1, 2, 3])                  // 1 2 3 (Int: CustomStringConvertible)
```

See [Chapter 24, Generics](24-generics.md).

## Witness Tables

`class` + protocol → witness table (runtime dispatch):

```syx
protocol Named { func name() -> String }

class A: Named { func name() -> String { return "A" } }
class B: Named { func name() -> String { return "B" } }

func greet(_ n: Named) { print("hi \(n.name())") }
greet(A())                                  // "hi A"
greet(B())                                  // "hi B"
```

> **Implementation:** the witness table is a hidden field of the class; the `CallWitness` instruction goes through vtable-like dispatch.

## Protocols + actor

An `actor` may implement a protocol; actor methods are isolated automatically:

```syx
protocol Counter {
    func increment()
    func current() -> Int
}

actor MyCounter: Counter {
    var value: Int = 0
    func increment() { self.value = self.value + 1 }
    func current() -> Int { return self.value }
}

let c: any Counter = MyCounter()
await c.increment()                         // Crossing isolation requires await
print(await c.current())                    // 1
```

## Self Constraints

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}
```

`Self` refers to the concrete type implementing the protocol. **Status:** **Partially implemented** (basic witness dispatch works).

## Protocol Composition

```syx
protocol A { func a() }
protocol B { func b() }

func use<T: A & B>(_ x: T) {
    x.a()
    x.b()
}
```

> **Status:** `&` composition constraints are **Partially implemented** (basic `T: A & B` works); use a `where` clause for the full form.

## Complete Example

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

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S006** | `Iterator` missing `next()` | Add `func next() -> Int?` (stable subset) |
| **S010** | Does not conform to the protocol | Add conformance / implement the method |
| **S012** | Associated type error | Add a `typealias` / check the signature |
| **S015** | `Hashable` inconsistency | Implement `hash(into:)` / `@Hashable` |
| **S016** | `Equatable` inconsistency | Implement `==` / `@Equatable` |
| **P001** | Protocol method signature mismatch | Fix the signature |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Protocol declaration | **Implemented** |
| Default implementation | **Implemented** |
| Associated types | **Implemented** |
| `Self` constraints | **Partially implemented** |
| `class-only protocol` | **Not implemented** (use a class constraint) |
| protocol extension where | **Partially implemented** |
| protocol composition `&` | **Partially implemented** |
| Existential type `any P` | **Implemented** (witness table) |
| Opaque return `some P` | **Implemented** |
| retroactive conformance | Allowed across modules (unlike Swift) |

---

**Next chapter** → [Generics](24-generics.md)

**See also** → [Language Reference: Protocols and Conformances](../language-reference/09-protocols-and-conformances.md) · [Chapter 22, Extensions](22-extensions.md) · [Chapter 24, Generics](24-generics.md) · [Chapter 25, Opaque Types](25-opaque-types.md)
