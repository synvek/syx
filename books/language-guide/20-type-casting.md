# Type Casting

**Implementation status:** `is` type checks / `as?` failure-to-nil / `as!` failure-throws / numeric literal inference / Optional wrapping **Implemented**; the full Swift-style cast set (generic erasure / protocol existential) **Partially implemented**

Type casting means "treating a value as another type at runtime / compile time". SYX offers three ways: **implicit promotion**, explicit **`as` / `as?` / `as!`**, and the **`is`** type check.

## Numeric Promotion (Implicit)

For mixed numeric operations the compiler promotes according to semantic rules (`widen_for_binary`):

```syx
let x: Int = 1
let y = x + 1.0                          // Double (Int → Double)
let z: Int8 = 1
let w = z + 100i32                        // Int32 (Int8 → Int32)
```

| Rule | Description |
|------|------|
| Integer literals | Default `Int`; in operations with `Double`, `Int → Double`. Under an explicit `Float` / `Double` annotation, an unsuffixed integer is interpreted as that float type (`let x: Double = 1`) |
| Signed + unsigned | Triggers **S014** (avoiding a silent sign-bit wrap) |
| Integer + float | The integer is promoted to float |
| Float + float | Same width returns `Double`; `Float + Double = Double` |
| `Int8` + `Int8` | The result is still `Int8` (overflow wraps two's complement) |
| `Int8` + `Int` | Promoted to platform `Int` |
| `Int8` + `Int32` | Promoted to `Int32` |

> **Same-sign integer narrowing (`Int` → `Int8`):** allowed in assignment / initialization, via `NumericCast`; an out-of-range value → runtime **R003** (≠ silent truncation). An out-of-range literal is still compile-time **S011**; a literal with the wrong suffix (`8i16` → `Int8`) is still **S004**.
>
> **Float → integer (`Double → Int8`):** assignment is still not implicit (**S004**); explicit `as` / `as?` truncates toward zero (out-of-range → R003 / nil). S008 is dedicated to cross-actor fields, not to narrowing.

**Diagnostics:** S014 (mixed signed/unsigned), S004 (incompatible types), R003 (narrowing out of range)

## is Type Checks

A runtime type check producing a `Bool`:

```syx
let a = [1, 2, 3]
print(a is Array)                         // true
print(a is Array<Int>)                    // true (matched at runtime by base type; element types are not checked through erasure)

class Dog { init() {} }
let d = Dog()
print(d is Dog)                           // true
```

> **Numeric targets forbidden:** `1 is Int` ❌ (compile error **S046**); `is` targets classes / protocols / collection base types.

### Supported Target Types

| Target | Matching values |
|------|--------|
| `Array` / `Array<T>` | Array literals / `Value::Array` |
| `Set` | Sets |
| `Dictionary` | Dictionaries |
| `String` / `Bool` / `Int*` / `Float*` / `Double` | The corresponding scalars |
| User `class` / `data class` | Instances carrying `__type` / `ObjectRef` |
| `protocol` (existential) | Values implementing that protocol |

```syx
let v: any CustomStringConvertible = "hello"
print(v is String)                        // true
```

## Explicit Conversion

### as? — Failure Becomes nil

```syx
class Animal { init() {} }
class Dog: Animal {
    override init() { super.init() }
    func bark() { print("woof") }
}

let a: Animal = Dog()
if let d = a as? Dog {
    d.bark()                              // "woof"
}
```

| Scenario | `as?` behavior |
|------|-----------|
| Success | Returns the target type (`Dog`) |
| Failure | Returns `nil` |
| Same-sign integers (`Int as? Int8`) | Converts when in range; out of range → `nil` (≠ R003) |

### as! — Failure Throws

```syx
let d = a as! Dog                         // Failure → runtime R003
```

| Scenario | `as!` behavior |
|------|-----------|
| Success | Returns the target type |
| Failure | Runtime error **R003** |

### as — Forced Cast (Known at Compile Time)

```syx
let a: Animal = Dog()
let d = a as Dog                          // Known to be Dog; compiles

let n: Int = 8
let n8 = n as Int8                        // Same-sign integer → NumericCast; out of range → R003

let d: Double = 1.5
let i = d as Int                          // Truncates toward zero → 1; out of range → R003
let i8 = d as? Int8                       // Success → 1; out of range → nil
```

> **Status:** `as` is used when the conversion is known at compile time; a runtime failure throws. Same-sign integer widening/narrowing and explicit `Double`/`Float` → integer go through `NumericCast` (`as` out of range → **R003**; `as?` out of range → `nil`). Assignment still does not implicitly convert float→int. The full Swift-style distinction between `as` / `as!` / `as?` is still being refined.

## Optional Conversion

```syx
let x: Int = 42
let y: Int? = x                           // T → T? implicit wrapping
let z: Int = y ?? 0                       // T? → T (nil coalescing)
```

| Direction | Form | Failure |
|------|------|------|
| `T` → `T?` | Implicit | Never fails |
| `T?` → `T` | `??` / `!` / `if let` | `!` failure **R007** |
| `T` → `U` (related) | `as?` / `as!` | `as!` → **R003** |

## Protocols and Existential Types

```syx
protocol Describable {
    func describe() -> String
}

class City: Describable {
    let name: String
    init(name: String) { self.name = name }
    func describe() -> String { return name }
}

let c: any Describable = City(name: "Beijing")
if let city = c as? City {
    print(city.name)                      // "Beijing"
}
```

> **existential:** `any Describable` is an existential type; dispatch at runtime goes through a witness table. See [Chapter 25, Opaque Types](25-opaque-types.md).

## Numeric Literal Conversion

```syx
let n: Int = 42
let n8: Int8 = n                          // Same-sign integer narrowing (out of range → R003)
let n8b: Int8 = 42i8                      // The literal suffix selects the type directly
let f: Double = Double(n)                 // Explicit conversion
let f2 = 3.14.toFloat()                   // Partially implemented
```

| Conversion | Form |
|------|------|
| `Int → Int8` | Implicit narrowing in assignment/initialization; out of range → **R003** |
| `Int → Double` | widening: implicit |
| `Double → Int` | explicit `as` / `as?`: truncates toward zero; out of range `as` → **R003**, `as?` → `nil`; assignment is still **S004** |
| `Int8 → Int` | widening: implicit |
| `String → Int?` | `Int.fromString("42")` → `ok(42)` |

```syx
let r = Int.fromString("42")
switch (r) {
case .ok(let n) { print("got \(n)") }     // "got 42"
case .err(let e) { print("err: \(e)") }
}
```

## Cross-Class Conversion

```syx
class Animal { init() {} }
class Dog: Animal { init() { super.init() } }
class Cat: Animal { init() { super.init() } }

let a: Animal = Dog()
if let d = a as? Dog {
    print("it's a dog")
} else if let c = a as? Cat {
    print("it's a cat")
}
```

- Subclass → superclass: upcast, **always** succeeds (use `as`)
- Superclass → subclass: downcast, may fail (use `as?` / `as!`)
- Unrelated types: compile error **S004**

## Choosing Between `as` and `as!`

| Form | Failure | When to use |
|------|------|--------|
| `as?` | `nil` | When the type is uncertain |
| `as!` | panic | When you are 100% sure |
| `as` | panic | Known at compile time |

## Complete Example

```syx
class Shape { init() {} func area() -> Double { return 0 } }
class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r; super.init() }
    override func area() -> Double { return 3.14159 * r * r }
}
class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s; super.init() }
    override func area() -> Double { return s * s }
}

func describe(_ s: Shape) -> String {
    if let c = s as? Circle {
        return "circle r=\(c.r), area=\(c.area())"
    }
    if let sq = s as? Square {
        return "square s=\(sq.s), area=\(sq.area())"
    }
    return "shape"
}

func main() {
    let shapes: Array<Shape> = [Circle(r: 2.0), Square(s: 3.0)]
    for s in shapes { print(describe(s)) }
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Incompatible conversion (including float→int, wrong suffix) | Check the type relationship |
| **S008** | Direct cross-actor access to instance fields | Go through messaging / isolation APIs |
| **S014** | Mixed signed/unsigned | Explicit cast |
| **S046** | `is` with a numeric target | Use an `==` comparison |
| **R003** | `as!` failure; integer narrowing out of range | Change to `as?` / keep the value in the target range |
| **R007** | `!` unwrapping nil | Use `?.` / `if let` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `is` type check | **Implemented** (collections + scalars + classes) |
| `as?` / `as!` / `as` | **Implemented** (runtime cast host) |
| Full cast protocol existential | **Partially implemented** (basic witness) |
| `AnyObject` / `Any` | Weakly supported (use `Object` / `any P`) |
| Generic erasure | **Partially implemented** |

---

**Next chapter** → [Nested Types](21-nested-types.md)

**See also** → [Language Reference: Types](../language-reference/02-types.md) · [Chapter 9, Enumerations](09-enumerations.md) · [Chapter 25, Opaque Types](25-opaque-types.md)
