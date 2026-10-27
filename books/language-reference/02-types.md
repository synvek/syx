# Types

**Implementation status:** core types **Implemented**; `Int128` / `UInt128` Phase 38; `SIMD2/4/8/16<T>` Phase 38+; `Result<T, E>` generic algebra **Phase 21–23**; `BigInt` **Phase 46 MVP**; `some` / `any` Phase 11–12; references `&T` / `&mut T` **Partially implemented** (SE-0461 default on, Phase 72)

In source code SYX types are written as **type expressions (TypeExpr)**; the semantic layer maps them to **TypeId**.

---

## Type Expression Forms

```
type-expr ::= named-type
            | optional-type
            | array-type
            | dict-type
            | function-type
            | tuple-type
            | reference-type
            | opaque-type
            | metatype

named-type      ::= identifier ('<' type-args '>')?
optional-type   ::= type '?'
array-type      ::= '[' type ']' | 'Array' '<' type '>'
dict-type       ::= '[' type ':' type ']' | 'Dictionary' '<' type ',' type '>'
function-type   ::= '(' type-list? ')' '->' type
tuple-type      ::= '(' type (',' type)+ ')'
reference-type  ::= '&' 'mut'? lifetime? type
opaque-type     ::= 'some' type | 'any' type
metatype        ::= 'Self' | 'SelfType' | 'Type'
```

| Form | Example | AST |
|------|------|-----|
| Named type | `Int`, `Array<Int>` | `Named` |
| Optional | `String?`, `Int?` | `Optional` |
| Array | `[Int]`, `Array<String>` | `Array` |
| Dictionary | `[String: Int]` | `Dictionary` |
| Function type | `(Int) -> String` | `Function` |
| Tuple type | `(Int, String)` | `Tuple` |
| Reference | `&Int`, `&mut String` | `Reference` |
| Opaque | `some Equatable`, `any Protocol` | `OpaqueSome` / `OpaqueAny` |
| Metatype | `Self`, `Type` | `Metatype` |
| Empty | No return value | `Void` |

## Numeric Types

| Type | Size | Range | Status |
|------|------|------|------|
| `Int8` | 8-bit signed | -128 … 127 | **Implemented** |
| `Int16` | 16-bit signed | -32,768 … 32,767 | **Implemented** |
| `Int32` | 32-bit signed | -2³¹ … 2³¹-1 | **Implemented** |
| `Int64` | 64-bit signed | -2⁶³ … 2⁶³-1 | **Implemented** |
| `Int128` | 128-bit signed | -2¹²⁷ … 2¹²⁷-1 | **Phase 38 GA** |
| **`Int`** | Platform word size | 64-bit host → `Int64`; 32-bit → `Int32` | **Implemented** |
| `UInt8` | 8-bit unsigned | 0 … 255 | **Implemented** |
| `UInt16` / `UInt32` / `UInt64` | Fixed-width unsigned | | **Implemented** |
| `UInt128` | 128-bit unsigned | 0 … 2¹²⁸-1 | **Phase 38 GA** |
| **`UInt`** | Platform word size, unsigned | 64-bit host → `UInt64` | **Implemented** |
| `Float32` / `Float` | 32-bit IEEE 754 | ~1.4e-45 … ~3.4e38 | **Implemented** |
| `Float64` / `Double` | 64-bit IEEE 754 | ~4.9e-324 … ~1.8e308 | **Implemented** |

Literal suffixes select a fixed width; unsuffixed integers default to `Int` and unsuffixed floats default to `Double`.

## Basic Types

| Type | Description | Literal |
|------|------|--------|
| `Bool` | Boolean | `true` / `false` |
| `String` | UTF-8 string; grapheme-cluster semantics (Phase 84) | `"…"` |
| `Void` | No value returned | — |
| `Character` | Single-scalar `String` type name | A single `Character` literal is a `String` |
| `Any` | Any type | — |

## Collections and Dynamic Types

| Type | Literal example | Description |
|------|------------|------|
| `Array<T>` | `[1, 2, 3]` | Ordered dynamic array (**strongly typed**, no bare `Array`) |
| `Set<T>` | `Set<Int>` | Deduplicating set |
| `[K: V]` / `Dictionary<K, V>` | `["a": 1]` | Static key-value type, keys must be `Hashable` |
| `Object` | `{ "k": "v" }` | Dynamic key-value object |

> **Strong typing, Phase 60:** the bare `Array` has been removed; `Array<T>` is required. `Array<UInt8>` is a byte stream.

## Optional Types

```
T?  ::= Optional<T>
```

- `T?` means nullable; only `nil` and `T` values are legal
- Used together with `??` / `?.` / `!`

```syx
let a: Int? = 42
let b: Int? = nil
let c = a ?? 0                            // nil coalescing
let d = a?.description                    // optional chaining
let e = a!                                // force unwrap
```

**Diagnostic:** S004 type mismatch

## The Result Type

```
Result<T, E> ::= ok(T) | err(E)
```

- `Result<Ok, Err>` is used for error propagation
- `throws` functions are tied to `try` / `?` chains
- `Result<T, E>` is a final class providing `static ok` / `static err` / `isOk` and more

**Implementation status:** **Phase 21–23 builtin wire** (GA); Phase 46+ provides class methods; see [Language Guide: Enumerations](../language-guide/09-enumerations.md).

## BigInt

```
BigInt ::= <arbitrary precision integer>
```

- **Phase 46 MVP** (`__syx_bigint_*` + Core)
- `BigInt.fromString(s)` / `b.toString()` / `BigInt.add(a, b)`

## Reference Types

```
&'a mut? T
```

```syx
&Int           // Immutable borrow
&mut Int       // Mutable borrow
&'a Int        // With lifetime annotation (**Partially implemented**)
```

| Item | Description |
|------|------|
| Borrow checking | **Implemented** (borrow regions v8) |
| SE-0461 | **On by default** (Phase 72) |
| Lifetimes | **Partially implemented** (S017) |
| Borrows across `await` | **Partially implemented** |

**Diagnostics:** S011 borrow/mutation violation; S017 lifetime

## Opaque Types and Existential Types

```
some P    -- Opaque result type (concrete at compile time; only P is known outside)
any P     -- Existential type (dynamic dispatch through a runtime witness table)
```

```syx
func make() -> some Equatable { 42 }       // Int at compile time
let p: any CustomStringConvertible = "hi"   // Runtime witness
```

| Form | Status | Meaning |
|------|------|------|
| `some P` | **Implemented** | Opaque result (no virtual dispatch) |
| `any P` | **Implemented** | Existential type (witness table) |
| `some P` in parameter position | **Not implemented** | Planned |

See [Language Guide: Opaque Types](../language-guide/25-opaque-types.md).

## Metatypes

```syx
Self        // The current implementing type (class / struct / enum)
SelfType    // Same as Self (more explicit)
Type        // Metatype
```

> Inside a protocol default implementation, `Self` refers to "the concrete type implementing this protocol".

## SIMD Types

```
SIMD2<T> | SIMD4<T> | SIMD8<T> | SIMD16<T>
```

| Form | Status | Back end |
|------|------|------|
| `SIMD2<T>` | **Implemented** | Generic |
| `SIMD4<T>` | **Implemented** | SSE / NEON |
| `SIMD8<T>` | **Implemented** | AVX2 |
| `SIMD16<T>` | **Phase 38+ partial GA** | AVX-512 / WASM SIMD128 |

The element `T` must be a fixed-width numeric type (`Int8/16/32/64` / `UInt8/16/32/64` / `Float32/64`).

See [Language Guide: SIMD](../language-guide/31-simd.md).

## User-Defined Types

```
user-type ::= struct | class | data class | enum | actor | protocol
```

| Declaration | Runtime | Status |
|------|--------|------|
| `struct` | Value/field aggregate MVP | **Implemented** |
| `class` | Reference object (ARC) | **Implemented** |
| `data class` | Data carrier + proof macros | **Phase 50–52** |
| `final class` | Subclassing forbidden | **Phase 60 GA** |
| `enum` | Enumeration with associated values (ADT) | **Implemented** |
| `actor` | Isolated concurrent object | **Implemented** |
| `protocol` | Protocol type (witness) | **Implemented** |
| `extension` | Type extension | **Implemented** |
| `global actor` | Global actor | **Implemented** |

## Generic Type Parameters

```
generic-params ::= '<' identifier (',' identifier)* (':' constraint-list)? ('where' where-clause)? '>'
```

```syx
struct Box<T> { var value: T }
func id<T>(x: T) -> T { return x }
enum Optional<T> { case some(T); case none }
```

- Monomorphization generates concrete instances
- Constraints: `where T: Equatable` / `T: Comparable` / `T: CustomStringConvertible` and more

See [Generic Parameters and Arguments](08-generic-parameters-and-arguments.md).

## Type Inference

```
inference ::= bidirectional | unidirectional
```

- When **unannotated**, types are inferred by semantic analysis
- A `let` binding's type is **frozen** after its first assignment
- A later incompatible assignment → **S004**
- Contextual types (`let arr: Array<Int> = [1, 2]`) infer the element type backwards

**Diagnostics:** S004, S008 (narrowing)

## Type Nesting

```
type-nesting ::= <not fully supported>
```

- Top-level `class` / `struct` / `enum`: **Implemented**
- Declaring one type nested inside another: **Partially implemented** (parser edge cases)
- Outside visibility of nested types: **incomplete**
- Prefer flat naming (`OuterMarker_Inner`)

See [Language Guide: Nested Types](../language-guide/21-nested-types.md).

## Built-in Protocols (Implicit Conformance)

Numerics, `Bool`, `String`, `Array<T>`, `Set<T>`, `Dictionary<K, V>`, `Object` and others automatically conform to:

| Protocol | Description |
|------|------|
| `Equatable` | `==` |
| `Hashable` | Hashing |
| `CustomStringConvertible` | `.description()` |
| `CustomDebugStringConvertible` | Debug description |
| `Iterator` | `func next() -> String?` |
| `Sequence` | Iterator factory |

See [Protocols and Conformances](09-protocols-and-conformances.md) · [crates/syx-stdlib/src/protocols.rs](../../crates/syx-stdlib/src/protocols.rs).

## Type Aliases

```
typealias-decl ::= 'typealias' identifier '=' type
```

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String
```

- `typealias` does **not** create a new type (a synonym only)
- No `where` clause is supported
- **Phase 40 GA**

## Internal Types (prelude)

Pre-declared (no import needed):

| Type | Meaning |
|------|------|
| `Optional<T>` / `T?` | Optional |
| `Result<T, E>` | Error return |
| `Range<T>` | Range |
| `Array<T>` | Array |
| `Set<T>` | Set |
| `Dictionary<K, V>` | Dictionary |
| `Object` | Dynamic object |
| `String` / `Bool` / `Int` / `Double` | Basic types |
| `Int8` … `Int64` / `UInt8` … `UInt64` | Fixed width |
| `Int128` / `UInt128` | 128-bit (Phase 38) |
| `Float32` / `Float64` | Floating point |
| `SIMD2/4/8/16<T>` | SIMD |
| `BigInt` | Arbitrary precision (Phase 46) |
| `Character` | Single scalar |
| `Void` | No value |
| `Any` / `AnyObject` | Any type |

---

**Diagnostics:** S004, S008, S010, S011, S017

**Usage examples** → [Language Guide: The Basics](../language-guide/02-the-basics.md) · [Collections](../language-guide/05-collection-types.md) · [Enumerations](../language-guide/09-enumerations.md) · [Opaque Types](../language-guide/25-opaque-types.md) · [SIMD](../language-guide/31-simd.md)

**See also** → [Expressions](03-expressions.md) · [Generic Parameters](08-generic-parameters-and-arguments.md) · [Protocols](09-protocols-and-conformances.md)
