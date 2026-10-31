# Opaque Types and Boxed Protocol Types

**Implementation status:** `some P` (opaque result types) **Implemented** (Phase 11 GA, `fixtures/format/opaque_some_mvp` / `fixtures/parity/protocol_witness.sy`); `any P` (existential types) **Implemented** (witness-table dynamic dispatch)

SYX offers two ways of "hiding the concrete type":

- `some P` — **opaque result type**: hidden at compile time; callers can only use it as `P`
- `any P` — **existential type**: dispatched at runtime through a witness table; the caller gets "some kind of `P`"

## some — Opaque Result Types

`some` hides concrete type information while preserving compile-time type inference:

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func makeShape() -> some Shape {            // Return type: some Shape (known at compile time)
    return Square(s: 2.0)                   // Actually a Square
}

func main() {
    let shape = makeShape()
    print(shape.area())                     // 4.0 (through the Shape interface)
}
```

| Behavior | Description |
|------|------|
| Caller | Sees `some Shape` (satisfies the Shape protocol) |
| Internally | Actually a `Square`, concrete at compile time |
| Implementation changes | `Square` → `Circle` does not break callers (API stability) |
| Performance | Zero virtual dispatch (static at compile time) |

> **Limit:** all return paths of the same function must return **the same concrete type**. `if cond { return A() } else { return B() }` is a compile error.

Tests: `fixtures/format/opaque_some_mvp` · `fixtures/parity/protocol_witness.sy`

## any — Existential Types

`any` is a container for "some kind of `P`"; dispatch at runtime goes through a witness table:

```syx
protocol Shape {
    func area() -> Double
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let s: any Shape = Circle(r: 2.0)
    print(s.area())                         // 12.56636
}
```

| Behavior | Description |
|------|------|
| Caller | Sees `any Shape` (existential type) |
| Internally | Actually a `Circle`, dispatched at runtime through a witness table |
| Multiple concrete types | The same variable may hold different implementations such as `Square` / `Circle` |
| Performance | Indirect dispatch (witness table) |

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let shapes: Array<any Shape> = [
        Square(s: 2.0),
        Circle(r: 3.0)
    ]
    for s in shapes {
        print(s.area())                     // 4.0, 28.27431
    }
}
```

Test: `fixtures/parity/protocol_witness.sy`

## some vs any

| Dimension | `some P` | `any P` |
|------|----------|---------|
| Compile-time type | **Preserved** (concrete type) | **Erased** (only `P`) |
| Runtime dispatch | **Static** (no witness) | **Dynamic** (witness table) |
| Multiple type values | ❌ (a function must return one type) | ✅ (`Array<any P>` may mix) |
| Performance | Zero virtual dispatch | Indirect call |
| Reflection | Visible | Only the `P` interface |

## Usage Scenarios

| Scenario | Choose |
|------|-----|
| A function returns a single concrete type, exposing only the protocol | `some P` |
| Storing multiple types conforming to `P` | `any P` |
| API stability + performance sensitive | `some P` |
| Polymorphic collections / factories | `any P` |

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

// Factory: returns multiple implementations → `any P`
func makeShape(kind: String) -> any Shape {
    if kind == "circle" { return Circle(r: 1.0) }
    return Square(s: 1.0)
}

// API: returns a single implementation → `some P`
func defaultShape() -> some Shape { return Square(s: 1.0) }

func main() {
    print(makeShape(kind: "circle").area())
    print(defaultShape().area())
}
```

## Casting `any P`

```syx
protocol Shape {
    func area() -> Double
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let s: any Shape = Circle(r: 2.0)
    if let c = s as? Circle {
        print("circle r=\(c.r)")            // "circle r=2.0"
    }
}
```

> **Status:** runtime `as?` / `as!` casting is **Implemented** (see [Chapter 20](20-type-casting.md)).

## Constraints on `some P`

```syx
func make() -> some Equatable {             // Satisfying Equatable is enough
    return 42                                // Actually an Int
}

func makeBox<T: Equatable>(_ v: T) -> some Equatable {
    return v
}
```

## Common Patterns with `any P` Collections

```syx
protocol Drawable {
    func draw()
}

class Button: Drawable { func draw() { print("[btn]") } }
class Text: Drawable { func draw() { print("[text]") } }

func main() {
    let ui: Array<any Drawable> = [
        Button(),
        Text(),
        Button()
    ]
    for el in ui { el.draw() }
}
```

## Combining with Generics

```syx
func makePair<T: Equatable>(_ a: T, _ b: T) -> some Equatable {
    return (a, b)                            // Tuple (T, T)
}
```

## Implementation Details of Existential Types

At runtime `any P` stores:

- The value itself (concrete type)
- A witness table (a function pointer for each `P` method)

```syx
protocol Named {
    func name() -> String
}

class User: Named {
    var n: String
    init(name: String) { self.n = name }
    func name() -> String { return self.n }
}

func main() {
    let p: any Named = User(name: "Alice")
    print(p.name())
    // Memory layout: [ value | witness table ptr ]
    // Dispatch: p.name() → witness_table.name(value)
}
```

> **Code generation:** the `CallWitness` instruction goes through the witness table; see `syx-codegen`.

## Complete Example

```syx
protocol Logger {
    func log(_ msg: String)
}

class ConsoleLogger: Logger {
    func log(_ msg: String) { print("[console] \(msg)") }
}

class FileLogger: Logger {
    let path: String
    init(path: String) { self.path = path }
    func log(_ msg: String) { print("[file:\(path)] \(msg)") }
}

// Factory: returns any P (multiple implementations)
func makeLogger(kind: String) -> any Logger {
    if (kind == "file") { return FileLogger(path: "/tmp/log") }
    return ConsoleLogger()
}

// API: returns some P (a single fixed one)
func defaultLogger() -> some Logger {
    return ConsoleLogger()
}

func main() {
    let l1: any Logger = makeLogger(kind: "file")
    let l2 = defaultLogger()
    l1.log("hello")
    l2.log("world")
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | `some P` returning multiple types | Unify into one concrete type |
| **S010** | Does not conform to the protocol | Add conformance |
| **P001** | Syntax error | Check `some` / `any` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `some P` opaque return | **Implemented** |
| `any P` existential type | **Implemented** |
| `some P` in parameter position | **Not implemented** (planned) |
| `any P` in return position | **Implemented** |
| `some Collection<Element>` associated types | **Partially implemented** |
| Full existential self-conformance | **Partially implemented** (basic witness) |
| Performance | `some P` zero virtual dispatch; `any P` indirect |

---

**Next chapter** → [Memory Management and Garbage Collection](26-memory-management.md)

**See also** → [Language Reference: Types](../language-reference/02-types.md) · [Chapter 20, Type Casting](20-type-casting.md) · [Chapter 23, Protocols](23-protocols.md) · [Chapter 24, Generics](24-generics.md)
