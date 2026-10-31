# Initialization

**Implementation status:** `init` / default initialization / `init?` failable initialization **Implemented**; two-phase initialization **Implemented** (Phase 51+ chain checking); `required init` / `convenience init` **Partially implemented** / **Not implemented**; `throws init` **Partially implemented**

An initializer initializes all stored properties when an instance is created. SYX has two kinds of initializer: **designated initializers** (initializing all stored fields) and **convenience initializers** (calling another `init`). This chapter covers syntax, chain checking, failable initialization, and inheritance.

## init Basics

```syx
class Greeter {
    var name: String

    init(name: String) {
        self.name = name
    }
}

let g = Greeter(name: "SYX")
print(g.name)                              // "SYX"
```

| Keyword | Description |
|--------|------|
| `init(params)` | Designated initializer |
| `self.field = value` | Assigning to a stored property |
| `super.init(...)` | Calling the superclass initializer (with class inheritance) |
| `convenience init(...)` | **Not implemented** |

> **Inside a `class` / `actor` member, `init` generates a `TypeName_init` call** (internal VM convention).

## struct / data class Initialization

```syx
struct Point {
    var x: Int
    var y: Int
}

let p = Point(x: 1, y: 2)                 // Field-order initialization
print(p.x)                                 // 1
```

- `struct` field-order initialization (synthesized by the compiler)
- `data class` field-order initialization + automatic `description()` + synthesis by the proof macros

## Default Values

```syx
class Counter {
    var value: Int = 0
    let name: String

    init(name: String) {
        self.name = name
        // value already has the default 0, so no assignment is needed
    }
}
```

- Fields with default values may skip assignment inside `init`
- A `let` field must be assigned by the end of `init` (default value or explicit)

## Inheritance and init Chains (Two-Phase)

A subclass `init` **must** finish initializing all its stored properties before `super.init` (two-phase):

```syx
class Animal {
    var name: String
    init(name: String) { self.name = name }
}

class Dog: Animal {
    var breed: String

    init(name: String, breed: String) {
        self.breed = breed                  // Phase 1: this class's own properties
        super.init(name: name)              // Phase 2: the super chain
    }
}
```

| Phase | Operation |
|------|------|
| Phase 1 | Initialize the subclass's own stored properties (`self.breed = …`) |
| Phase 2 | Call `super.init(...)` |
| Phase 3 | (Optional) Continue modifying inherited properties (observers are suppressed inside `init`, see [Chapter 11](11-properties.md)) |

> **Wrong order** (reading `self` before `super.init`, or reading an uninitialized field) → **S020** compile error.

### super.init May Be Omitted (When the Superclass Has a Default Initializer)

```syx
class Animal {
    var name: String = "anon"
}

class Dog: Animal {
    var breed: String

    init(breed: String) {
        self.breed = breed
        // super.init() is called implicitly (Animal has a default initializer)
    }
}
```

> **Implicit super.init:** when all superclass stored properties have default values, the subclass may omit `super.init` (the compiler inserts it).

## Failable Initialization `init?`

`init?` returns `Optional<Self>` and may return `nil`:

```syx
class Port {
    let value: Int

    init?(value: Int) {
        if (value < 0 || value > 65535) {
            return nil
        }
        self.value = value
    }
}

let p1 = Port(value: 80)                   // Port?
let p2 = Port(value: 99999)                // nil

if let p = p1 {
    print("port \(p.value)")               // "port 80"
}
```

| Form | Status | Description |
|------|------|------|
| `init?` | **Implemented** | Returns `nil` on failure |
| `init!` | **Not implemented** | Implicitly unwrapping failable initializer |
| `throws init` | **Partially implemented** | `init throws` throws an error |

## throws init

```syx
class Parser {
    let source: String

    init(source: String) throws {
        if (source.isEmpty) {
            throw "empty source"
        }
        self.source = source
    }
}

let p = try? Parser(source: "")            // nil
let p2 = try? Parser(source: "x")          // Parser?
```

See [Chapter 17, Error Handling](17-error-handling.md).

## init Parameter Defaults

```syx
class Server {
    let host: String
    let port: Int
    let timeout: Int

    init(host: String, port: Int = 8080, timeout: Int = 30) {
        self.host = host
        self.port = port
        self.timeout = timeout
    }
}

let s1 = Server(host: "localhost")
let s2 = Server(host: "x.com", port: 9000, timeout: 60)
```

## init and Property Wrappers

The `init` of an `@propertyWrapper` field must initialize the wrapper rather than the stored value:

```syx
@propertyWrapper
class Clamped {
    var wrappedValue: Int
    init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}

class User {
    @Clamped var age: Int

    init(age: Int) {
        self.age = age                     // Actually calls Clamped_init
    }
}
```

See [Chapter 19, Macros](19-macros.md).

## `self` Inside init

- `self` is **unavailable** until all stored properties are initialized (except for `self.field = …` assignments)
- `self` is **not readable** before `super.init` (write-only)
- A closure capturing `self` must wait until `init` finishes (avoiding half-constructed objects)

```syx
class Foo {
    let name: String
    let printer: () -> Void

    init(name: String) {
        self.name = name
        // ❌ Cannot read self.printer before it is set
        // self.printer = { print(self.name) }  // Error

        self.printer = { [self] in          // Explicit capture (partially implemented)
            print(self.name)
        }
    }
}
```

> **Status:** explicit capture lists `[self]` / `[weak self]` are partially implemented; finish `init` first, then assign the closure.

## Multiple inits and Overloading

```syx
class User {
    let name: String
    let age: Int

    init(name: String) {
        self.init(name: name, age: 0)       // Delegation (convenience is not implemented)
    }

    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }
}
```

> **convenience init:** **Not implemented** (suspended after Phase 49; planned for later Phases). For now, simulate it by "calling another init from a designated init" (without the `convenience` keyword).

## enum Initialization

```syx
enum Port {
    case http
    case https
    case custom(Int)

    init(value: Int) {
        if (value == 80) { self = Port.http }
        else if (value == 443) { self = Port.https }
        else { self = Port.custom(value) }
    }
}
```

## actor Initialization

```syx
actor Counter {
    var value: Int

    init(value: Int) { self.value = value }

    func increment() { self.value = self.value + 1 }
}

let c = await Counter(value: 0)            // An actor init may be awaited too
```

## Complete Example

```syx
class Vehicle {
    var wheels: Int
    init(wheels: Int) { self.wheels = wheels }

    func describe() -> String { return "wheels=\(self.wheels)" }
}

class Car: Vehicle {
    var brand: String

    init(brand: String) {
        self.brand = brand
        super.init(wheels: 4)
    }

    override func describe() -> String {
        return "\(self.brand) car, \(super.describe())"
    }
}

class Truck: Vehicle {
    var payload: Int

    init(payload: Int) {
        self.payload = payload
        super.init(wheels: 6)
    }

    override func describe() -> String {
        return super.describe()
    }
}

func main() {
    let c = Car(brand: "Tesla")
    let t = Truck(payload: 5000)
    print(c.describe())                     // "Tesla car, wheels=4"
    print(t.describe())                     // "wheels=6"
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | init parameter / field type mismatch | Fix it |
| **S020** | Reading `self` before `super.init` | Reorder |
| **S054** | Inherited `init` signature mismatch | Fix it |
| **R007** | `!` unwrapping a nil `init?` result | Use `if let` |
| **R016** | Required field not initialized | Assign it inside `init` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `init` named parameters | **Implemented** (`label: value` required) |
| Two-phase initialization | **Implemented** (Phase 51 chain checking) |
| `convenience init` | **Not implemented** (planned) |
| `required init` | **Not implemented** |
| `init?` / `init!` | `init?` **Implemented**; `init!` not implemented |
| `throws init` | **Partially implemented** |
| `defer` in init | **Not implemented** |
| `self.init` delegation | Legal (without the `convenience` marker) |
| Closures capturing `self` in `init` | Partially supported |

---

**Next chapter** → [Deinitialization](16-deinitialization.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 11, Properties](11-properties.md) · [Chapter 14, Inheritance](14-inheritance.md) · [Chapter 16, Deinitialization](16-deinitialization.md) · [Chapter 17, Error Handling](17-error-handling.md)
