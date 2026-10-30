# Generics

**Implementation status:** functions / types / where clauses / protocol constraints / associated types / monomorphization **Implemented**; runtime generic specialization **Not implemented**; generic reflection **Partially implemented** (`reflect_specialization` Phase 77)

Generics write type-safe reusable code. SYX uses **monomorphization**: the compiler generates specialized code for each concrete type-argument combination. The advantage is no virtual dispatch at runtime; the disadvantage is possible code-size bloat.

## Generic Functions

```syx
func id<T>(x: T) -> T { return x }
func swap<T>(a: T, b: T) -> (T, T) { return (b, a) }

let n = id(x: 42)                          // Int
let s = id(x: "hi")                        // String
let (x, y) = swap(a: 1, b: 2)              // (Int, Int)
```

| Element | Description |
|------|------|
| `<T>` | Type parameter list |
| `<T, U>` | Multiple type parameters |
| `<T: Equatable>` | Protocol constraint |
| `<T where T: Comparable>` | where clause |

Tests: `fixtures/parity/generics.sy` · `fixtures/parity/generics.sy`

## Generic Types

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

- Generic parameters come after the type name
- Inside the type body, `T` may be used in any type position
- Constructors may use type parameters

## where Clauses

```syx
func eq<T>(a: T, b: T) -> Bool where T: Equatable {
    // Witness `==` for arbitrary `T: Equatable` lands with protocol conformance (S1-05+).
    return true
}

func allEqual<T: Equatable>(_ items: Array<T>) -> Bool {
    if (items.count == 0) { return true }
    // Element-wise `==` via Equatable witness: same follow-on as `eq` above.
    return items.count > 0
}
```

| where form | Status |
|------------|------|
| `where T: P` single constraint | **Implemented** |
| `where T: P1, T: P2` multiple constraints | **Implemented** |
| `where T: P<U>` associated-type constraint | **Partially implemented** |
| `where T == U` same-type constraint | **Implemented** |
| `where T: P where U: Q` nesting | **Partially implemented** |

Failure test: `fixtures/parity/generics.sy`

## Protocol Constraints

```syx
func printAll<T>(items: Array<T>) where T: CustomStringConvertible {
    for item in items {
        print(item.description())
    }
}

printAll(items: [1, 2, 3])                  // 1 2 3
// printAll(items: [User()])                // S004: User is not CustomStringConvertible
```

Test: `fixtures/macros/generic_witness_constraint.sy`

## Associated Types

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

> **Automatic inference:** when the `add` / `get` signatures uniquely determine `Item`, `typealias Item` may be omitted.

## Protocols as Type-Parameter Constraints

```syx
func map<T, U>(items: Array<T>, f: (T) -> U) -> Array<U> {
    var result: Array<U> = []
    for item in items { result.append(f(item)) }
    return result
}

let doubled = map(items: [1, 2, 3], f: { x in x * 2 })   // [2, 4, 6]
```

## Monomorphization

```syx
func id<T>(x: T) -> T { return x }

id(x: 42)                                  // Generates id_int
id(x: "hi")                                // Generates id_str
```

> **Process:** `syx-mono` collects all concrete type instantiations and generates specialized function code. Each instance has an independent symbol and there are **no virtual calls** at runtime.

| Advantages | Disadvantages |
|------|------|
| Zero runtime overhead (no virtual dispatch) | Code size bloat |
| Inlining-friendly | Longer compile times |
| Easier debugging (independent stack per instance) | Cross-module specialization exposes internals |

## Generics and Collections

```syx
let arr: Array<Int> = [1, 2, 3]
let dict: [String: Int] = ["a": 1, "b": 2]
let set: Set<Int> = [1, 2, 3]

func first<T>(_ arr: Array<T>) -> T? {
    if (arr.count == 0) { return nil }
    return arr[0]
}

let firstInt = first(arr)                   // Int?
let firstString = first([1.0, 2.0])         // Double? (inferred)
```

## Generics and Functions

```syx
func compose<A, B, C>(f: (B) -> C, g: (A) -> B) -> (A) -> C {
    return { x in f(g(x)) }
}

let f = compose(f: { x in x + 1 }, g: { x in x * 2 })
print(f(3))                                 // 7
```

## Generic Extensions

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

See [Chapter 22, Extensions](22-extensions.md).

## Generics and Actors

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

## Generics and Protocol Composition

```syx
protocol A { func a() }
protocol B { func b() }

func both<T>(x: T) where T: A, T: B {
    x.a()
    x.b()
}
```

## Generic Default Values (Partially Implemented)

```syx
class Holder<T = Int> {                     // ⚠️ Partially implemented
    var value: T
    init(v: T) { self.value = v }
}

let h1 = Holder<Int>(v: 1)
let h2 = Holder(v: 2)                       // Inferred as Holder<Int>
```

> **Status:** default values for generic type parameters are **Partially implemented** (`fixtures/macros/generic_witness_constraint.sy`); explicit annotation is recommended.

## Complete Example

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

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Type mismatch | Check the generic arguments |
| **S010** | Protocol constraint not satisfied | Implement the required methods |
| **S012** | Associated type error | Add a `typealias` / fix it |
| **P001** | Generic syntax error | Check the `<T>` syntax |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Generic functions / types | **Implemented** (monomorphization) |
| where clauses | **Implemented** |
| Associated types | **Implemented** |
| Generic default values | **Partially implemented** |
| Type-level programming (HKT) | **Not implemented** |
| Runtime generic specialization reflection | **Not implemented** (MVP `reflect_specialization` Phase 77) |
| Swift partial specialization | **Not implemented** (only full monomorphization) |

---

**Next chapter** → [Opaque Types and Boxed Protocol Types](25-opaque-types.md)

**See also** → [Language Reference: Generic Parameters and Arguments](../language-reference/08-generic-parameters-and-arguments.md) · [Chapter 22, Extensions](22-extensions.md) · [Chapter 23, Protocols](23-protocols.md) · [Chapter 25, Opaque Types](25-opaque-types.md)
