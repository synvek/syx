# Generic Parameters and Arguments

**Implementation status:** generic functions / types / where clauses / protocol constraints / monomorphization **Implemented**; associated types / generic default values / generic specialization reflection **Partially implemented**

---

## Generic Parameters

```
generic-params ::= '<' identifier (',' identifier)* ('=' type)? (',' identifier ('=' type)?)* '>'
                |  '<' identifier (':' constraint)? (',' identifier (':' constraint)?)* ('where' where-clause)? '>'
```

Generic parameters are declared after the type or function name:

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

> `type_params: Vec<SmolStr>` in the AST for `Decl::Function` / `Decl::Class` / `Decl::Struct` and others.

## Generic Arguments

```
generic-args ::= '<' type (',' type)* '>'
```

```syx
let b = Box<Int>(value: 42)
let x = id(x: 42)                            // T = Int (inferred)
let y = id<String>(x: "hi")                  // Explicit
```

## Explicit vs. Inferred

| Call | Behavior |
|------|------|
| `id(x: 42)` | Infers `T = Int` |
| `id<String>(x: "hi")` | Explicit `T = String` |
| `Box<Int>(value: 42)` | Must be explicit (no constructor parameter to infer from) |
| `arr.map<U>(f: { x in ... })` | Explicit result type |

## where Clauses

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

> `WherePred { type_name, constraint }`.
> **Multiple constraints:** since Phase 42, `where T: P1, T: P2` is legal; violations → **S010**.

**Diagnostics:** S004 (where unsatisfied) / dedicated generic errors

## Protocols as Constraints

```syx
func printAll<T>(items: Array<T>) where T: CustomStringConvertible {
    for item in items {
        print(item.description())
    }
}
```

**Test:** `fixtures/macros/generic_witness_constraint.sy`

## Associated Types and Witnesses

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

> For the interaction between protocol associated types and generic witnesses, see [Protocols and Conformances](09-protocols-and-conformances.md).
> **Implementation status:** **Partially implemented** (`fixtures/macros/generic_witness_constraint.sy`, `fixtures/macros/generic_witness_constraint.sy`).

## Monomorphization

```
monomorphization ≡ ∀ (Type, …) instance of (Generic<…>) → specialized copy
```

The compiler generates specialized function/type code for each concrete type-argument combination (`syx-mono`).

```syx
func id<T>(x: T) -> T { return x }

id(x: 42)                                    // Generates id_int
id(x: "hi")                                  // Generates id_str
```

| Advantages | Disadvantages |
|------|------|
| Zero runtime overhead (no virtual dispatch) | Code size bloat |
| Inlining-friendly | Longer compile times |
| Easier debugging (independent stack per instance) | Cross-module specialization exposes internals |

## Generics and Extensions

```syx
extension Array where Element: Equatable {
    func unique() -> Array<Element> { … }
}
```

See [Declarations](05-declarations.md) and [Language Guide: Extensions](../language-guide/22-extensions.md).

## Generics and Functions

```syx
func compose<A, B, C>(f: (B) -> C, g: (A) -> B) -> (A) -> C {
    return { x in f(g(x)) }
}

let f = compose(f: { x in x + 1 }, g: { x in x * 2 })
print(f(3))                                    // 7
```

## Generic Default Values (Partially Implemented)

```syx
class Holder<T = Int> {                        // ⚠️ Partially implemented
    var value: T
    init(v: T) { self.value = v }
}
```

> **Status:** default values for generic type parameters are **Partially implemented** (`fixtures/macros/generic_witness_constraint.sy`); explicit annotation is recommended.

## Associated Type Defaults

```syx
protocol Container {
    associatedtype Item = Int                  // Associated type default
    func add(_ item: Item)
}
```

> **Status:** associated type defaults are **Partially implemented** (Phase 50+); an implementing type may omit `typealias Item = Int`.

## Generic Specialization

| Capability | Status |
|------|------|
| Monomorphization | **Implemented** |
| Runtime specialization | **Not implemented** |
| `reflect_specialization` | **Partially implemented** (Phase 77) |

## Type-Level Programming (HKT)

SYX does **not** support Higher-Kinded Types (type constructors as parameters). For example `func map<F<_>>(…)` is **Not implemented**.

## Recursive Generics

```syx
indirect enum Tree {                           // ⚠️ indirect is not implemented
    case leaf(Int)
    case branch(Tree, Tree)
}

// Alternative: wrap in a class
class TreeNode {
    var value: Int
    var left: TreeNode?
    var right: TreeNode?
    init(value: Int) { self.value = value }
}
```

> `indirect enum` is **Not implemented** (planned).

## Not Implemented

| Feature | Status |
|------|------|
| Generic specialization (runtime) | **Not implemented** |
| Type-level programming (HKT) | **Not implemented** |
| Full generic default values | **Partially implemented** |
| Recursive generics (indirect) | **Not implemented** |
| Generic typealias | **Partially implemented** |

## Complete Example

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

**Diagnostics:** S004 / S010 / S012

**Usage examples** → [Language Guide: Generics](../language-guide/24-generics.md)

**See also** → [Types](02-types.md) · [Declarations](05-declarations.md) · [Protocols](09-protocols-and-conformances.md) · [Grammar Summary](10-grammar-summary.md)
