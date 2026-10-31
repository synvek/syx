# The Basics

**Implementation status:** **Implemented** (including `Int128` / `UInt128` Phase 38, platform `Int`/`UInt`, type nesting MVP, `typealias` GA Phase 40)

This chapter covers constants, variables, type annotations, comments, numeric literals and type inference. Mastering these is a prerequisite for writing any SYX program.

## Variables and Constants

```syx
var mutable = 10
mutable = 20          // ✅

let immutable = 42
// immutable = 43     // ❌ Compile error S003 (cannot assign to let binding)
```

| Keyword | Meaning | Reassignment | Redeclaration in the same scope |
|--------|------|----------|------------------|
| `let` | Immutable binding | ❌ S003 | ❌ S001 |
| `var` | Mutable binding | ✅ | ❌ S001 |

> **Scope:** block scope `{ … }`; `for` / `while` / `func` each introduce a new scope. An inner scope may reuse an outer `let`/`var` name to "shadow" it (not recommended; it easily confuses readers).

**Diagnostics:** S003 (assignment to `let`), S001 (duplicate definition), S002 (undefined identifier).

## Type Annotations

Explicit types let readers see at a glance "what type this is", and force a choice when inference is ambiguous:

```syx
let x: Int = 10
let name: String = "SYX"
let items: Array<Int> = [1, 2, 3]
let scores: [String: Int] = ["a": 1, "b": 2]
let maybe: Int? = nil
```

| Annotation position | Syntax | Notes |
|---------|------|------|
| Local variable | `let x: T = expr` | Required explicitly when inference fails |
| Top-level variable | `var count: Int = 0` | Usually has a default value |
| Function parameter | `func f(x: Int)` | Mandatory (no type inference) |
| Function return | `-> T` | Defaults to `Void` (i.e. `()`) |
| Property | `var name: String` | Must be explicitly annotated or given an initial value |

**Type inference:** after its first assignment, the type of a `let` binding is **frozen**. A later incompatible assignment → **S004**.

```syx
let n = 10       // n: Int
// n = "hello"   // ❌ S004: cannot assign String to Int
```

## Comments

```syx
// Single-line comment

/* Block comment
   /* may nest */
   any line
*/

/// Documentation comment (extracted into API docs by `syx doc`)
/// Multiple lines: each starting with `///`
func add(a: Int, b: Int) -> Int { return a + b }
```

| Form | Use |
|------|------|
| `//` | Single line (to end of line or whole line) |
| `/* … */` | Block comment; **nesting supported** (same as Rust) |
| `///` | Documentation comment; `syx doc .` generates `docs/api.md` |

## Numeric Types and Literals

SYX offers both **platform word size** and **fixed width** integers, plus 128-bit and SIMD vectors (see [Chapter 31, SIMD](31-simd.md)).

| Type | Size | Range / notes |
|------|------|-------------|
| `Int8` | 8-bit signed | -128 … 127 |
| `Int16` | 16-bit signed | -32,768 … 32,767 |
| `Int32` | 32-bit signed | -2³¹ … 2³¹-1 |
| `Int64` | 64-bit signed | -2⁶³ … 2⁶³-1 |
| **`Int128`** | 128-bit signed | Phase 38; literal suffix `i128` |
| **`Int`** | Platform word size | 64-bit host → `Int64`; 32-bit host → `Int32` |
| `UInt8` / `UInt16` / `UInt32` / `UInt64` | Fixed-width unsigned | |
| **`UInt128`** | 128-bit unsigned | Phase 38; literal suffix `u128` |
| **`UInt`** | Platform word size, unsigned | 64-bit host → `UInt64` |
| `Float32` / `Float` | 32-bit floating point (IEEE 754) | |
| `Float64` / `Double` | 64-bit floating point | Unsuffixed floats default to `Double` |

Literal syntax:

```syx
let a = 42         // Int (platform word size)
let b = 100i8      // Int8
let c = 1000i64    // Int64
let d = 0xFFu8     // UInt8, hexadecimal
let e = 0b1010u16  // UInt16, binary
let f = 0o17u32    // UInt32, octal
let g = 1_000_000i64  // Underscore separators (readability, removed at compile time)
let h = 42i128     // Int128
let i = 3.14f32    // Float32
let j = 3.14       // Double (default)
let k = -2.5e10    // Double, scientific notation
```

> **Underscores** (`_`) may separate digits anywhere in a numeric literal and are ignored at compile time (e.g. `1_000_000` = `1000000`).
>
> **Suffix case:** `i8`/`I8`, `u8`/`U8`, `f32`/`F32` and so on are all accepted; lowercase is recommended for consistency with the Swift style.

**Signed vs. unsigned:** mixed arithmetic may trigger **S014** (avoiding silent wrapping). For explicit conversion, see [Chapter 20, Type Casting](20-type-casting.md).

## Booleans and Nil

```syx
let ok: Bool = true
let ng: Bool = false
let empty: String? = nil
let present: String? = "hi"
```

| Literal | Type |
|--------|------|
| `true` / `false` | `Bool` |
| `nil` | Any `T?` (nil literal, type inferred from context) |

## Type Aliases (`typealias`)

Give a short name to a long type (**Phase 40 GA**):

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String

func register(h: Handle) {}
let f: Handler = { n in "id=\(n)" }
```

- `typealias` does **not** create a new type; it is only a synonym
- No `where` clause is supported; generic parameters must be copied 1:1
- Equivalent to Swift's `typealias`, but it cannot replace `associatedtype`

**Diagnostic:** a cyclic definition → S004 (type recursion detected)

## Type Nesting

Nesting sub-types inside a type is a core Swift pattern. SYX supports it **in a limited way**:

```syx
class Outer {
    var name: String
    init(name: String) { self.name = name }

    class Inner {           // ⚠️ Partially implemented
        var value: Int = 0
    }
}

let o = Outer(name: "x")
let i = Outer.Inner()      // Flattening may be needed (see below)
```

**Current status:**

- Top-level `class` / `struct` / `enum`: **Implemented**
- Declaring another type inside a type: **Partially implemented** (limited parser capability)
- Outside visibility of nested types: **incomplete**

**Alternative** (recommended, consistent with the Swift module style):

```syx
struct OuterMarker { }
struct OuterMarker_Inner {
    value: Int
}
```

See [Chapter 21, Nested Types](21-nested-types.md).

## `nil` and a Preview of Optionals

```syx
let empty: String? = nil
let present: String? = "hi"
let length: Int? = present?.count    // 4
let safe: Int = empty?.count ?? 0   // 0 (nil coalescing)
let crash: Int = present!.count     // 4 (force unwrap)
```

Three ways to unwrap:

| Form | On failure | Best for |
|------|--------|------|
| `?.` optional chaining | The whole expression is `nil` | Multi-level access, avoiding nested ifs |
| `!` force unwrap | Runtime error **R007** | When you are 100% sure it is non-nil |
| `??` nil coalescing | Falls back to the default | Providing a fallback |

See [Chapter 30, Advanced Operators](30-advanced-operators.md) for complete usage.

## Naming Conventions

Community conventions in SYX (not enforced; `syx lint` reports L001–L012 warnings):

| Category | Naming | Example |
|------|------|------|
| Types | `UpperCamelCase` | `Greeter`, `MyType` |
| Functions / variables / properties | `lowerCamelCase` | `greet`, `firstName` |
| Constants | `lowerCamelCase` | `defaultPort` (**not** ALL CAPS) |
| Enum cases | `lowerCamelCase` | `case success`, `case failure` |
| Protocols | `UpperCamelCase` (often ending in `able`/`ible`/`Convertible`) | `Iterator`, `CustomStringConvertible` |
| Files | `kebab-case.sy` or `snake_case.sy` | `main.sy`, `local_math.sy` |

## Differences from Swift

| Point | SYX |
|----|-----|
| `Int` width | Platform word size (64-bit host → `Int64`); Swift fixes it at 64 bits on Apple platforms |
| `Int128` / `UInt128` | Phase 38 GA (Swift still has no native 128-bit integers) |
| `typealias` | **Implemented** (Phase 40 GA) |
| Multiline strings `"""` | **MVP GA** (Phase 139); may contain raw newlines and `\(expr)` interpolation — not the full Swift raw/`#"""` delimiter set |
| Tuple syntax | **MVP GA** (Phase 139): `(T, U)` types and literals, named elements `(x: 1, y: 2)`, `let (x, y) = t` destructuring, `.0`/`.x` members — not arbitrary arity protocols / the full Swift tuple feature set |
| Underscores in numeric literals | ✅ Supported (same as Swift) |
| `0x` / `0o` / `0b` prefixes | ✅ Supported |

## Implementation Status and Limits

- Explicit `typealias`: **GA** (Phase 40; `fixtures/parity/generics.sy`)
- `Int128` / `UInt128`: **GA** (Phase 38)
- Type nesting: **Partially implemented**; flattening into names is recommended
- Multiline string literals `"""` : **MVP GA** (`fixtures/lang/string_multiline.sy`)
- Tuple types / literals / named elements: **MVP GA** (`fixtures/lang/tuple.sy`)

```syx
let pair: (Int, String) = (1, "a")
print(pair.0)
let named = (x: 10, y: 20)
print(named.x)
let (a, b) = (3, 4)
```

---

**Next chapter** → [Basic Operators](03-basic-operators.md)

**See also** → [Language Reference: Lexical Structure](../language-reference/01-lexical-structure.md) · [Language Reference: Types](../language-reference/02-types.md) · [Chapter 3, Basic Operators](03-basic-operators.md)
