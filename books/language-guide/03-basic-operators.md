# Basic Operators

**Implementation status:** **Implemented** (arithmetic / comparison / logical / assignment / range / nil coalescing / force unwrap / identity subset); custom `operator func` is covered in [Chapter 30, Advanced Operators](30-advanced-operators.md)

SYX operators fall mainly into four categories: **arithmetic, comparison, logical, assignment**. They are implemented underneath through [Core Host](32-operators-vs-host.md) intrinsic instructions and are broadly consistent with Swift; nothing relies on runtime interpretation.

## Arithmetic Operators

```syx
let a = 10 + 3      // 13
let b = 10 - 3      // 7
let c = 10 * 3      // 30
let d = 10 / 3      // 3 (integer division)
let m = 10 % 3      // 1 (modulo; sign follows the left operand)
let n = -a           // Unary minus
```

| Operator | Meaning | Applicable types |
|--------|------|----------|
| `+` `-` `*` `/` `%` | Add / subtract / multiply / divide / modulo | Integers, floats |
| `-x` | Unary minus | Numeric |
| `+x` | Unary plus (syntactically allowed) | Numeric |

> **Overflow:** integer arithmetic on the VM / MIR / WASM paths and fixed-width operations **always wrap two's complement** (`wrapping_*` is the floor; independent of debug/release). The debug **R011** panic once described in this book was **not delivered**. Check bounds explicitly when writing loops/bit operations, or move to a wider type.
>
> **Division by zero:** integer `/0` or `%0` triggers **R012 division by zero**; float division by zero yields `inf` / `nan`.

**Mixed types:** `Int + Double` follows the widening rule (`widen_for_binary`). Same-width integers (e.g. `Int8 + Int8`) keep that width; `Int8 + Int` is promoted to platform `Int`. `Int + UInt` triggers **S014** (avoiding a silent sign-bit wrap). Scalar bit operations `& | ^ << >> ~` follow the same kind (`Int8 << 1` wraps at 8 bits; `Int & UInt` → **S014**). Ordered comparisons `< <= > >=` follow the same kind (`UInt8` compares as unsigned; `==`/`!=` still use value equality). Unary `-` / `~` follow the operand's kind (`-(-128 as Int8)` wraps at 8 bits and stays `-128`).

## Comparison Operators

```syx
x == y      // Equal
x != y      // Not equal
x === y     // Identity (reference/type-sensitive subset; see below)
x !== y     // Identity negated
x < y       // Less than
x <= y      // Less than or equal
x > y       // Greater than
x >= y      // Greater than or equal
```

| Form | Applies to |
|------|------|
| `==` / `!=` | Value equality (`Equatable` protocol witness; custom types need `data class` + `@Equatable`) |
| `===` / `!==` | Identity (reference types compare references / metatype tags; see [Chapter 30, Advanced Operators](30-advanced-operators.md)) |
| `<` `<=` `>` `>=` | Ordered comparison (`Comparable` protocol witness; `data class` + L1 synthesis or custom) |

> **Not comparable:** using `<` and friends on a type without `Comparable` conformance → **S004** type mismatch.

## Logical Operators

```syx
if (a && b) { … }    // Short-circuit AND
if (a || b) { … }    // Short-circuit OR
if (!flag) { … }     // Logical NOT
```

| Operator | Meaning | Short-circuits |
|--------|------|--------|
| `&&` | Logical AND | ✅ The right side is evaluated only when the left is `true` |
| `\|\|` | Logical OR | ✅ The right side is evaluated only when the left is `false` |
| `!` | Logical NOT | Unary |

> Operands must be `Bool`; a non-`Bool` expression condition → **S004**.

## Nil Coalescing and Optional Chaining

```syx
let v = optional ?? defaultValue     // Nil coalescing
let len = maybeString?.count        // Optional chaining
let v = maybeString!                 // Force unwrap
```

| Operator | Meaning | Failure behavior |
|--------|------|----------|
| `??` | Nil coalescing | Takes the right side when the left is `nil` |
| `?.` | Optional chaining | Any `nil` link makes the whole thing `nil` |
| `!` | Force unwrap | `nil` → runtime error **R007** |

```syx
let s: String? = "hello"
print(s?.count)        // Optional(5)  ← Note: still an Optional
let n: Int = s?.count ?? 0   // 5
let bad: Int = s!.count      // 5 (force unwrap succeeds)
```

## Assignment and Compound Assignment

```syx
x = 1          // Simple assignment
x += 2         // Equivalent to x = x + 2
x -= 1
x *= 2
x /= 2
x %= 3
obj.field = v
arr[i] = v
```

| Form | Meaning | Failure |
|------|------|------|
| `=` | Simple assignment | On a `let` → **S003** |
| `+=` `-=` `*=` `/=` `%=` | Compound assignment | Same as above |
| `obj.field = v` | Member assignment | Read-only field or immutable object → **S003** |
| `arr[i] = v` | Subscript assignment | Out of bounds → runtime **R003** |

> **Assignment expressions:** in SYX assignment does **not** return a `Void` value usable for chaining (consistent with Swift). `a = b = c` is compile error **S004**.
>
> **Fixed-width compound assignment:** for fixed-width integers such as `Int8`, `+=`/`-=`/`*=`/`/=`/`%=`/`&=`/`|=`/`^=`/`<<=`/`>>=` desugar to binary operations carrying the `widen_for_binary` kind (e.g. `120i8 += 10` → `-126`; `-1i8 &= 15` → `15`; `1i8 |= 15` → `15`; `15i8 ^= 1` → `14`; `64i8 <<= 1` → `-128`; `-128i8 >>= 1` → `-64`). For platform `Int`, `+=`/`-=` may take a host fast path. The bitwise compound grammar (including shift compounds) is complete.

## Range Operators

```syx
for i in 0..<10 { … }   // Half-open [0, 10)
for i in 1...5 { … }    // Closed [1, 5] (integer literal endpoints)
```

| Form | Meaning | Status |
|------|------|------|
| `a..<b` | Half-open `[a, b)` | **Implemented** |
| `a...b` | Closed `[a, b]` (**integer literal endpoints**) | **MVP GA** (Phase 139) |
| `a..<` | Unbounded on the right | **Not implemented** |
| `..<b` | Unbounded on the left | **Not implemented** |

The type of a range expression is `Range<Int>` (in `for-in` and `contains` contexts).

```syx
print((1..<10).contains(5))    // true
print((1..<10).contains(10))   // false
print((1...10).contains(10))  // true
print((1...10).contains(11))   // false
```

## Unary Prefix Operators

| Operator | Meaning | Applies to |
|--------|------|------|
| `-` | Unary minus | Numeric |
| `!` | Logical NOT / force unwrap | `Bool` / `Optional` |
| `try` | Error propagation / `?` chains | `throws` calls |
| `await` | Asynchronous wait | async context |
| `&` | Borrow expression | `&T` / `&mut T` |

## The Ternary (Conditional) Operator

SYX does **not** provide the `?:` ternary operator directly; use an `if` expression or a temporary variable instead:

```syx
// ✅ Recommended
let label = if (x > 0) { "pos" } else { "non-pos" }

// ❌ Not supported
// let label = x > 0 ? "pos" : "non-pos"   // Compile error
```

> **Why no `?:`:** consistent with Swift; the goal is to encourage readability, since nested ternaries are hard to read.

## Operator Precedence

For the complete precedence table, see [Language Reference: Expressions](../language-reference/03-expressions.md). Summary, **from highest to lowest**:

| Level | Operators |
|------|--------|
| 1 (highest) | `.` `[]` `()` `?.` `!` `await` (postfix) |
| 2 | `-` `!` `try` (unary prefix) |
| 3 | `*` `/` `%` |
| 4 | `+` `-` |
| 5 | `..<` `...` |
| 6 | `<` `<=` `>` `>=` |
| 7 | `==` `!=` `===` `!==` |
| 8 | `&&` |
| 9 | `\|\|` |
| 10 | `??` |
| 11 (lowest) | `=` `+=` `-=` `*=` `/=` `%=` |

> **Parentheses win:** `()` may be added anywhere to force precedence, with no side effects.

## Numeric Promotion and Literal Rules

```syx
let a: Int = 1
let b = a + 1.0   // Double (`1.0` is a Double literal; `a` is promoted to Double)
let c = a + 1     // Int (`1` is an Int literal)
let d = 1_000     // Int (underscores for readability)
```

| Literal | Inferred type |
|--------|----------|
| `42` | `Int` |
| `42i8` | `Int8` |
| `3.14` | `Double` |
| `3.14f32` | `Float32` |
| `"text"` | `String` |
| `true` | `Bool` |
| `nil` | Contextually inferred (usually `Optional<T>`) |

## Example: Accumulating

```syx
func main() {
    var sum = 0
    for i in 0..<5 {
        sum += i        // 0+1+2+3+4 = 10
    }
    print(sum)          // 10
}
```

## Custom Operators

`operator func` lets you declare new operators ([Chapter 30](30-advanced-operators.md)):

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

let result = 5 |> { $0 * 2 }    // 10
```

For the complete grammar, precedence groups, and custom infix / prefix / postfix operators, see [Chapter 30, Advanced Operators](30-advanced-operators.md).

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S003** | Assignment to `let` | Change it to `var` |
| **S004** | Incompatible operand types | Add an explicit conversion / change the type |
| **S008** | Narrowing (`Int64 → Int8`) | Explicit truncation / range check |
| **S014** | Mixed signed/unsigned | Explicit cast |
| **R007** | `!` unwrapping `nil` | Use `?.` / `??` / `if let` |
| **R011** | Integer overflow Display (**not delivered**; the current behavior is wrapping) | Check bounds; you can move to a wider type; the same code may be reused by other domains (e.g. the HTTP client) |
| **R012** | Integer division by zero | Check the divisor |
| **R003** | Array / string subscript out of bounds | Explicit range check |

For complete diagnostics, see [Appendix B: Diagnostic Code Index](appendix-b-diagnostic-index.md).

## Differences from Swift

| Point | SYX |
|----|-----|
| `===` | Supported, with a reference/type-sensitive subset |
| Custom operators | **GA** (`operator func` + three-back-end parity) |
| Overflow | Always two's-complement wrapping (≠ Swift's default trap; the Display for integer overflow **R011** was not delivered) |

---

**Next chapter** → [Strings and Characters](04-strings-and-characters.md)

**See also** → [Language Reference: Expressions](../language-reference/03-expressions.md) · [Chapter 30, Advanced Operators](30-advanced-operators.md) · [Chapter 32, Operators vs Core Host](32-operators-vs-host.md)
