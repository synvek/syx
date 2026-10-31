# Inheritance

**Implementation status:** single inheritance + `override` + vtable polymorphism **GA** (Phase 51–55 + Phase 66); `super.method()` binds statically to the direct super vtable slot; `final class` forbids subclassing (Phase 60); runtime polymorphic vtable indirect calls **Implemented** (Phase 66)

Inheritance is a core capability of classes. SYX's design — single inheritance plus a mandatory `override` keyword — matches Swift; dispatch at runtime goes through vtables.

## Class Inheritance Basics

```syx
class Animal {
    var name: String
    init(name: String) { self.name = name }

    func speak() {
        print("...")
    }
}

class Dog: Animal {
    override func speak() {
        print("woof")
    }
}

func main() {
    let d = Dog(name: "Rex")
    d.speak()                              // "woof"
}
```

| Rule | Description |
|------|------|
| Syntax | `class Sub: Super { … }` |
| Single inheritance | Only one superclass (multiple inheritance is **Not implemented**) |
| `override` | Mandatory keyword (omitting it → **S051**) |
| `super.method()` | Calls the superclass implementation |

> **`override` is mandatory:** this is part of SYX's strict contract, preventing accidental overrides of superclass methods; it also helps LSP / IDE tooling identify overrides accurately.

## Runtime Polymorphism (vtable)

When a base reference holds a subclass instance, method calls are **dispatched indirectly through the runtime vtable**:

```syx
class Animal {
    func speak() { print("animal") }
}

class Dog: Animal {
    override func speak() { print("woof") }
}

class Cat: Animal {
    override func speak() { print("meow") }
}

func describe(_ a: Animal) {
    a.speak()                              // Polymorphic
}

func main() {
    describe(Dog())                        // "woof"
    describe(Cat())                        // "meow"
}
```

| Call form | Dispatch |
|----------|------|
| `Dog().speak()` | Statically known type, may be devirtualized (the compiler can optimize it into a direct call) |
| `let a: Animal = Dog(); a.speak()` | **Dynamic vtable** (Phase 66 GA) |

> **Implementation:** Phase 66 introduced "**single inheritance, module-local vtable**"; each class holds `vtable: Array<fn_ptr>`; `override` replaces the corresponding slot; `super.method()` binds statically to the immediate super's vtable slot (it does **not** go through the receiver's dynamic type).

Tests: `inheritance_polymorphic.rs` · `inheritance.rs` · LSP `lsp_phase66_real.rs`

## super Calls

```syx
class Base {
    func greet() { print("base") }
}

class Derived: Base {
    override func greet() {
        super.greet()                      // "base"
        print("derived")
    }
}
```

- `super.method()` calls the **direct superclass**'s vtable slot
- It does not recurse upward (in A→B→C, `super.x` reaches only B)
- `super` is legal in static scope; it links correctly along override chains

## final Forbids Subclassing

```syx
final class Token {
    let value: String
    init(value: String) { self.value = value }
}

// class SubToken: Token { }                // ❌ S053: cannot inherit from final
```

- A `final class` cannot be inherited
- It lets the compiler devirtualize (direct calls, no vtable indirection)
- Equivalent to Swift's `final class`

## Initializer Chains

A subclass `init` must call `super.init` (**Phase 51 GA**):

```syx
class Animal {
    var name: String
    init(name: String) {
        self.name = name
    }
}

class Dog: Animal {
    var breed: String

    init(name: String, breed: String) {
        self.breed = breed
        super.init(name: name)             // Required
    }
}
```

See [Chapter 15, Initialization](15-initialization.md).

## Multi-Level Inheritance

```syx
class A {
    func a() { print("A") }
}

class B: A {
    override func a() { print("B") }
}

class C: B {
    override func a() { print("C") }
}

let c = C()
c.a()                                      // "C"
let b: B = c
b.a()                                      // "C" (polymorphic dispatch to C)
let a: A = c
a.a()                                      // "C"
```

> **Diamond inheritance:** multiple inheritance is **Not implemented**; SYX expresses polymorphic capability with `protocol` (see [Chapter 23](23-protocols.md)).

## Overriding Properties

A subclass may use `override var` to replace a superclass computed property (vtable polymorphic read/write):

```syx
class Base {
    var label: String {
        get { return "base" }
    }
    init() {}
}

class Derived: Base {
    override var label: String {
        get { return "derived" }
        set { /* ignore */ }
    }
    init() {}
}

let b: Base = Derived()
print(b.label)                            // "derived"
```

See [Chapter 11, Properties](11-properties.md).

## Overriding Initializers

A subclass `init` must call `super.init`, and all stored properties must be assigned (two-phase checking):

```syx
class Animal {
    var name: String
    init(name: String) {
        self.name = name
    }
}

class Dog: Animal {
    var tricks: Array<String>

    init(name: String, tricks: Array<String>) {
        self.tricks = tricks
        super.init(name: name)             // Required
    }
}
```

## override and Protocols

Implementing a protocol method does **not** need `override` (it is not inheriting from a superclass):

```syx
protocol Greetable {
    func greet() -> String
}

class User: Greetable {                    // No override
    func greet() -> String { return "hi" } // Satisfies the protocol
}
```

> **Distinction:** `override` is used only to override a superclass method; protocol implementations carry no `override`.

## Inheritance and Actors

An `actor` cannot be inherited (**Not implemented**); actors freely compose with top-level classes (nested `class` inside `actor` remains a later slice):

```syx
class Session {
    let id: String
    init(id: String) { self.id = id }
}

actor Manager {
    var sessions: Array<Session> = []

    func open() -> Session {
        let s = Session(id: "x")
        self.sessions.append(s)
        return s
    }
}
```

## Complete Example

```syx
class Shape {
    var color: String
    init(color: String) { self.color = color }

    func area() -> Double { return 0.0 }
    func describe() -> String {
        return "\(self.color) shape, area=\(self.area())"
    }
}

class Circle: Shape {
    var radius: Double

    init(color: String, radius: Double) {
        self.radius = radius
        super.init(color: color)
    }

    override func area() -> Double {
        return 3.14159 * self.radius * self.radius
    }
}

class Rectangle: Shape {
    var width: Double
    var height: Double

    init(color: String, width: Double, height: Double) {
        self.width = width
        self.height = height
        super.init(color: color)
    }

    override func area() -> Double {
        return self.width * self.height
    }
}

func totalArea(_ shapes: Array<Shape>) -> Double {
    var sum = 0.0
    for s in shapes { sum = sum + s.area() }
    return sum
}

func main() {
    let shapes: Array<Shape> = [
        Circle(color: "red", radius: 2.0),
        Rectangle(color: "blue", width: 3.0, height: 4.0)
    ]
    for s in shapes { print(s.describe()) }
    print("total=\(totalArea(shapes))")
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S051** | Missing `override` keyword | Add `override` |
| **S052** | Overriding something not declared in the superclass | Remove `override` |
| **S053** | Inheriting a `final` class | Make it non-final or drop the inheritance |
| **S054** | Override signature mismatch | Fix the signature |
| **S055** | Overriding a `final` superclass method | `final` cannot be overridden |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Single inheritance + `override` | **Implemented** |
| `final class` | **Implemented** (Phase 60) |
| vtable polymorphism | **Implemented** (Phase 66) |
| Multiple inheritance | **Not implemented** (use protocols) |
| `final class` / `final func` | **Implemented** (the lexer has no `sealed`) |
| Protocol composition (`protocol<A, B>`) | Partially implemented (use `&` / `where`) |
| `convenience init` | **Partially implemented** |
| `required init` | **Not implemented** (constructor chains check `super.init`) |

---

**Next chapter** → [Initialization](15-initialization.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 11, Properties](11-properties.md) · [Chapter 15, Initialization](15-initialization.md) · [Chapter 16, Deinitialization](16-deinitialization.md)
