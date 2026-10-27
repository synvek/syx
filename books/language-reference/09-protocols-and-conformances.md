# Protocols and Conformances

**Implementation status:** protocol declarations / conformance / witness tables / default implementations / associated types / existential types `any P` **Implemented**; `Self` constraints / protocol composition / protocol extension where / HKT witness **Partially implemented**; `class-only protocol` **Not implemented**

---

## Protocol Declarations

```
protocol-decl ::= access? attrs? 'protocol' identifier generic-params?
                 (':' type (',' type)*)? where-clause? '{' protocol-member* '}'

protocol-member ::= protocol-method
                 |  protocol-property
                 |  protocol-init
                 |  protocol-associatedtype
                 |  protocol-method-with-default
```

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}

protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}

protocol Iterator {
    func next() -> Element?
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
| Default implementation | A method body may be provided inside the protocol |

## Conformance

```
conformance ::= type ':' protocol (',' protocol)*
             |  extension type ':' protocol where-clause? '{' … '}'
```

```syx
class MyClass: Equatable, CustomStringConvertible {
    func ==(lhs: MyClass, rhs: MyClass) -> Bool { … }
    func description() -> String { … }
}

extension Array: CustomStringConvertible { … }
```

> A `class` declaration's `: Protocol` list triggers witness table generation (partial path).
> Conformance is declared at a **single point** within a module (to avoid cross-file conflicts).

**Diagnostics:** S006 (Iterator missing `next`), S010 (does not conform), S012 (associated type)

## Built-in Protocols

| Protocol | Description | Implicit conformance |
|------|------|------------|
| `Equatable` | `==` | Built-in value types |
| `Hashable` | Hashing | Built-in value types |
| `Comparable` | Ordered comparison | Numeric types GA |
| `CustomStringConvertible` | `.description()` | Built-in value types |
| `CustomDebugStringConvertible` | Debug description | Built-in value types |
| `Encoder` / `Decoder` | Format-agnostic codec sink | **Protocol surface pinned** (ADR-230; container API → S7-02) |
| `Encodable` / `Decodable` / `Codable` | Serialization | **Protocol surface pinned** (`Codable: Encodable, Decodable`); `@Codable` / `JSON.encode` are the default back end |
| `Iterator` | `next()` | Declaration checking |
| `Sequence` / `Collection` | Iteration | **Partially implemented** |
| `Sendable` | Cross-isolation safety | Explicit annotation |
| `Error` | Error type | User-defined |

Defined in `crates/syx-stdlib/src/protocols.rs`.

## Protocol Default Implementations

```syx
protocol Greetable {
    func greet() -> String
    func greet() -> String { return "Hello" }   // Default
}

class User: Greetable {
    // Inherits the default → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }    // Override
}
```

> Default implementations are declared through an `extension` or a method body inside the protocol (**Phase 50+**).

## Dynamic Dispatch

| Form | Dispatch mechanism |
|------|----------|
| `class` + protocol | Witness call (`CallWitness`) |
| `opaque some` | Concrete at compile time (no virtual dispatch) |
| `any P` existential | Runtime witness table |

```syx
protocol Named { func name() -> String }

class A: Named { func name() -> String { return "A" } }
class B: Named { func name() -> String { return "B" } }

func greet(_ n: Named) { print("hi \(n.name())") }
greet(A())                                    // "hi A"
greet(B())                                    // "hi B"
```

## Protocols as Types

```
protocol-type ::= 'any' protocol
              |  'some' protocol
```

| Form | Meaning |
|------|------|
| `some P` | Opaque result (concrete at compile time) |
| `any P` | Existential type (runtime witness) |

```syx
func make() -> some Equatable { 42 }          // Int at compile time
let p: any CustomStringConvertible = obj      // Runtime witness
```

See [Language Guide: Opaque Types](../language-guide/25-opaque-types.md).

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

> **Automatic inference:** when the `add` / `get` signatures uniquely determine `Item`, `typealias Item = …` may be omitted.

## Protocol Extension Constraints

```
protocol-extension ::= 'extension' type ':' protocol where-clause? '{' … '}'
                    |  'extension' type where-clause? '{' … '}'
```

```syx
extension Collection where Element: Equatable {
    func unique() -> [Element] { … }
}
```

> **Status:** protocol extension constraints are **Partially implemented**; multi-constraint where clauses since Phase 42+.

## Protocol Composition

```syx
protocol A { func a() }
protocol B { func b() }

func use<T: A & B>(_ x: T) {
    x.a()
    x.b()
}
```

> **Status:** `&` composition constraints are **Partially implemented** (basic `T: A & B` works); use `where T: A, T: B` for the full form.

## Self Constraints

```syx
protocol Equatable {
    func ==(lhs: Self, rhs: Self) -> Bool
}
```

> `Self` refers to the concrete type implementing the protocol. **Status:** **Partially implemented** (basic witness dispatch works).

## `@unchecked Sendable`

```syx
@unchecked Sendable
class MyLegacy {
    var internal: Int = 0
}
```

> Suppresses `Sendable` checking; external synchronization is required to guarantee thread safety.

## Encoder / Decoder / Codable Protocols (ADR-230 / `M2-S7-01`)

```syx
public protocol Encoder { }
public protocol Decoder { }
public protocol Encodable { }
public protocol Decodable { }
public protocol Codable: Encodable, Decodable { }
```

The protocol layer provides an extensible encoding abstraction; the **default back end** is still dynamic `JSON.encode` / synthesis by the `@Codable` macro (not `JSONEncoder` GA).

```syx
@Codable
data class User(name: String, age: Int)

@Codable
enum Status {
    case ok
    case err
}
```

> **Status:** the protocol surface was pinned by ADR-230; `JSONEncoder`/`JSONDecoder` + `codingPath` pinned by ADR-231 / **`M2-S7-02`**; `JSONStream` NDJSON session + chunk feed pinned by ADR-232 / **`M2-S7-03`** (`codingPath=line[N].…`; **S7 closed out**). `@Codable` L1 GA; L2 associated-value enums partial GA. **Not** a CodingKeys DSL / metatype `T.Type` / real File S3-03 streaming / IANA.

## Use with Extensions

```syx
extension MyType: CustomStringConvertible {
    func description() -> String { return "MyType" }
}
```

> A protocol conformance may be declared at the original type declaration or in a separate extension. SYX allows cross-module extensions (**Phase 60+**).

## Synthesized Conformance

The `@Equatable` / `@Hashable` / `@Codable` macros synthesize witnesses automatically on `data class` and associated-value enums:

```syx
@Equatable
@Hashable
@Codable
data class Point(x: Int, y: Int)
```

See [Attributes](06-attributes.md) and [Language Guide: Macros](../language-guide/19-macros.md).

## Witness Table Implementation

`class` + protocol → witness table (runtime dispatch):

- A class holds `witness_table: *const WitnessTable`
- The `CallWitness` instruction goes through the witness table
- Multiple protocols: one table per protocol

```syx
class Multi: A, B, C {
    func a() { … }
    func b() { … }
    func c() { … }
}
```

> Internally the class stores three tables: `witness_a` / `witness_b` / `witness_c`.

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Protocol declaration | **Implemented** |
| Default implementation | **Implemented** (inside protocol + extension) |
| Associated types | **Implemented** |
| `Self` constraints | **Partially implemented** |
| `class-only protocol` | **Not implemented** (use a class constraint) |
| protocol extension where | **Partially implemented** |
| protocol composition `&` | **Partially implemented** |
| Existential `any P` | **Implemented** (witness table) |
| Opaque result `some P` | **Implemented** |
| retroactive conformance | Allowed across modules (unlike Swift) |
| `_const` protocols | **Not implemented** |

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

---

**Diagnostics:** S006 / S010 / S012 / S015 / S016 / P001

**Usage examples** → [Language Guide: Protocols](../language-guide/23-protocols.md)

**See also** → [Types](02-types.md) · [Declarations](05-declarations.md) · [Generic Parameters](08-generic-parameters-and-arguments.md) · [Attributes](06-attributes.md)
