# Advanced Operators

**Implementation status:** half-open ranges / optional chaining / force unwrap / nil coalescing / pattern matching / custom operators **GA** (`operator func` + three-back-end parity); bitwise operations / overflow behavior **Implemented**; custom prefix / postfix / infix **Implemented**; precedence / associativity declarations **Implemented**

An operator is a function with "**special syntax**". SYX is like Swift: built-in operators are part of the **language surface** (compiler hard-coded + Core Host bridge), and users may declare **custom operators** (`infix` / `prefix` / `postfix`) with precedence / associativity.

## Built-in Operator Cheat Sheet

For the complete list, see [Chapter 3, Basic Operators](03-basic-operators.md). This section adds the "advanced" subset:

### Bitwise Operations (Integers)

| Operator | Meaning | Type |
|--------|------|------|
| `&` | Bitwise AND | Integer |
| `\|` | Bitwise OR | Integer |
| `^` | Bitwise XOR | Integer |
| `~` | Bitwise NOT (unary) | Integer |
| `<<` | Left shift | Integer |
| `>>` | Right shift (arithmetic) | Integer |

```syx
let a: Int = 0b1100                        // 12
let b: Int = 0b1010                        // 10
print(a & b)                                // 8  (0b1000)
print(a | b)                                // 14 (0b1110)
print(a ^ b)                                // 6  (0b0110)
print(~a)                                   // -13
print(a << 2)                               // 48
print(a >> 1)                               // 6
```

> **Precedence:** bitwise operations come after comparison `==` and before logical `&&`. `a == b & c` means `a == (b & c)`.

### Custom Prefix / Postfix / Infix Operators

```syx
// Prefix
prefix operator !
func !<T>(value: Optional<T>) -> Bool {
    return value == nil
}

let x: Int? = nil
print(!x)                                   // true (custom)

// Postfix
postfix operator **
func **<T>(value: T) -> T where T: Numeric {
    return value * value
}

print(3**)                                  // 9

// Infix
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T {
    return f(value)
}

print(5 |> { $0 * 2 })                      // 10
```

### Precedence / Associativity

```syx
infix operator ^^ { precedence: 100, associativity: right }
func ^^<T>(a: T, b: T) -> T where T: Numeric {
    // A right-associative "power" operator
    return a * a
}
```

| Field | Value | Meaning |
|------|------|------|
| `precedence` | Integer (0–200) | Higher binds tighter |
| `associativity` | `left` / `right` / `none` | Same-precedence combining direction |
| `higherThan` / `lowerThan` | Operator name | Relative position |

> **Predefined precedences (partial):** `*` `/` `%` are 100; `+` `-` are 90; `<<` `>>` are 80; `<` `>` and friends are 70; `==` `!=` are 60; `&` is 50; `^` is 40; `\|` is 30; `&&` is 20; `||` is 10.

### Operator Functions and Protocol Constraints

```syx
infix operator <*> { precedence: 60 }
func <*> <T: Numeric>(a: Array<T>, b: Array<T>) -> Array<T> {
    var r: Array<T> = []
    for i in 0..<a.count {
        r.append(a[i] * b[i])
    }
    return r
}

let r = [1, 2, 3] <*> [4, 5, 6]            // [4, 10, 18]
```

> **Status:** `operator func` is **GA** (Phase 41+), with parity across the three back ends (VM / unified MIR / WASM).

## Half-Open Ranges

```syx
for i in 0..<10 {                           // [0, 10)
    print(i)
}

let r = 1..<10
print(r.contains(5))                        // true
print(r.contains(11))                       // false
print(r.contains(0))                        // true
```

| Form | Status | Meaning |
|------|------|------|
| `a..<b` | **Implemented** | Half-open [a, b) |
| `a...b` | **Partially implemented** | Closed [a, b] |
| `a..<` / `..<b` | **Not implemented** | One-sided infinite |

## Optional Chaining ?.

```syx
let maybe: String? = "hello"
let len = maybe?.count                    // Optional(5)
let bad = (nil as String?)?.count         // nil
```

Chains may be arbitrarily long:

```syx
class A { let b: B? = B() }
class B { let c: C? = C() }
class C { let name: String = "x" }

let a: A? = A()
print(a?.b?.c?.name)                        // Optional("x")
print(a?.b?.c?.name?.count)                // Optional(1)
```

> **`?.` semantics:** if any link in the chain is `nil` → the whole thing is `nil`.

## Force Unwrap !

```syx
let present: String? = "hello"
let s = present!                            // "hello" (known non-nil at runtime)

let empty: String? = nil
let x = empty!                              // ❌ R007
```

> **Choose one of three:** `?.` chaining / `??` default / `!` force unwrap.

## Nil Coalescing ??

```syx
let opt: Int? = nil
let v = opt ?? 0                            // 0
let v2 = opt ?? (1 + 2)                    // 3 (the right side is an expression)
```

> **`??` precedence:** lower than comparison and logical; parentheses are recommended:
> ```syx
> let r = (a ?? 0) + 1
> ```

## if let / guard let

```syx
// if let
if let x = optional {
    print("got: \(x)")
} else {
    print("nil")
}

// guard let (must exit early)
guard let y = optional else {
    return
}
print(y)                                    // Here y is not Optional
```

> **`else` must exit:** if a `guard let`'s `else` block does not `return` / `throw` / `break` / `continue` → compile error **S020**.

## try ?

```syx
let v = risky()?                            // String? (nil on failure)
let v2 = try! risky()                       // String (panics on failure)
```

## Pattern Matching

```syx
switch (result) {
case .ok(let v) where (v > 0) {
    print("ok: \(v)")
}
case .err(let e) {
    print("err: \(e)")
}
default { }
}

if case .err(let e) = result {
    print("err: \(e)")
}
```

See [Chapter 6, Control Flow](06-control-flow.md) and [Chapter 9, Enumerations](09-enumerations.md).

## Identity === / !==

Reference- or type-sensitive equality subset:

```syx
let a = "hi"
let b = "hi"
print(a === b)                              // false (different objects)
let c = a
print(a === c)                              // true (same reference)
```

> **Status:** `===` MVP subset; commonly used for `class` / `Object` reference comparison.

## Custom Equatable / Comparable

```syx
struct Point: Equatable {
    var x: Int
    var y: Int
    func ==(other: Point) -> Bool { return x == other.x && y == other.y }
}

struct Score: Comparable {
    var value: Int
    func compare(other: Score) -> Int {
        if (value < other.value) { return -1 }
        if (value > other.value) { return 1 }
        return 0
    }
}
```

> **@Equatable / @Hashable / @Codable macros:** synthesize `==` / `hash` / `encode` / `decode` automatically (see [Chapter 19, Macros](19-macros.md)).

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Custom operators | **Implemented** (infix / prefix / postfix) |
| Precedence / associativity | **Implemented** (precedence 0–200) |
| Half-open `..<` / closed `...` | Half-open **Implemented**; closed **Partially implemented** |
| Optional chaining `?.` | **Implemented** |
| Force unwrap `!` | **Implemented** |
| Nil coalescing `??` | **Implemented** |
| `===` | **MVP subset** |
| `#available` | **Not implemented** |
| Overflow behavior | Always two's-complement wrapping (≠ Swift's default overflow trap; ≠ the delivered debug **R011** panic) |

## Complete Example

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

infix operator <^> { precedence: 60 }
func <^> <T>(a: Array<T>, b: Array<T>) -> Array<T> where T: Numeric {
    var r: Array<T> = []
    for i in 0..<a.count { r.append(a[i] * b[i]) }
    return r
}

prefix operator √
func √(value: Double) -> Double {
    return value.squareRoot()
}

func main() {
    let r = 5 |> { $0 * 2 } |> { $0 + 1 }  // 11
    let dot = [1.0, 2.0, 3.0] <^> [4.0, 5.0, 6.0]  // [4.0, 10.0, 18.0]
    let side = √(16.0)                      // 4.0
    print("r=\(r) dot=\(dot) side=\(side)")
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Operand types mismatch on both sides | Fix the types |
| **S008** | Narrowing (e.g. `Int64 → Int8`) | Check the range |
| **S020** | `guard let` `else` does not exit | Add return / throw |
| **R007** | `!` unwrapping nil | Use `?.` / `??` |

---

**Next chapter** → [SIMD](31-simd.md)

**See also** → [Language Reference: Expressions](../language-reference/03-expressions.md) · [Chapter 3, Basic Operators](03-basic-operators.md) · [Chapter 9, Enumerations](09-enumerations.md)
