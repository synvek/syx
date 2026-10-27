# Strings and Characters

**Implementation status:** **Implemented** (UTF-8 storage, grapheme-cluster counting Phase 84, for-in, `.count`, subscripts, split/trim/... and all other stdlib methods, regex literals Phase 50+); `Character` is a **Phase 139 MVP GA** scalar type distinct from `String` (not a complete Unicode Character property library)

Strings are one of the areas where SYX differs most from Swift. This chapter explains the four key points: "**grapheme clusters vs. scalars**", "**the `Character` type**", "**interpolation and escapes**", and "**regex literals**".

## Creating Strings

```syx
let s = "Hello"
let msg = "Hello, \(name)!"        // Interpolation
let multi = "a" + "b" + "c"        // String concatenation
let multiline = "line one\nline two"
let tabbed = "col1\tcol2\tcol3"
```

Sources: `crates/syx-driver/tests/fixtures/lang/string_multiline.sy` · `examples/main.sy`

## String Literal Syntax

| Form | Syntax | Description |
|------|------|------|
| Single line | `"…"` | Delimited by double quotes; supports escapes and interpolation |
| Multiline | **MVP GA** | `"""`…`"""` may contain raw newlines and `\(expr)` interpolation (Phase 139) |
| Escapes | `\n` `\t` `\\` `\"` `\(expr)` | Same as Swift |
| Interpolation | `\(expr)` | Arbitrary expressions (function calls, arithmetic, field access) |

```syx
let name = "World"
let count = 3
let s1 = "Hello, \(name)!"
let s2 = "count=\(count + 1)"        // Expression
let s3 = "items: \([1,2,3].count)"  // Any expression is legal
```

> **Important:** raw newlines are not allowed inside a single-line string (**L001**). Multiline `"""` literals allow raw newlines (Phase 139 MVP).

## Escape Sequences

| Escape | Meaning | Byte |
|------|------|------|
| `\n` | Newline (LF) | 0x0A |
| `\r` | Carriage return (CR) | 0x0D |
| `\t` | Tab | 0x09 |
| `\\` | Backslash | 0x5C |
| `\"` | Double quote | 0x22 |
| `\'` | Single quote | 0x27 |
| `\0` | Null character (NUL) | 0x00 |
| `\(…)` | Interpolation (no 0 byte; compiled to StringConcat) | — |
| `\x41` | ASCII byte | 0x41 |
| `\u{1F600}` | Unicode code point | emoji 0x1F600 |

```syx
let s = "tab\there\nnewline"
let emoji = "\u{1F600}"            // 😀
let zwnj = "\u{200C}"              // ZWNJ (zero-width non-joiner)
```

## Common String Methods

```syx
"a,b,c".split(",")                       // ["a", "b", "c"]
"  hi  ".trim()                          // "hi"
"ABC".lowercased()                       // "abc"
"abc".uppercased()                       // "ABC"
"file.txt".hasPrefix("file")             // true
"hello.txt".hasSuffix(".txt")            // true
"hello".substring(0, 3)                  // "hel"  [start, end) scalar index
"banana".replacingOccurrences("a", "o")  // "bonono"
"abc123".matches("\\d+")                 // ["123"] (regex matching returns an array of substrings)
"text".count                            // 4 (number of grapheme clusters)
"text".isEmpty                           // false
"  ".isEmpty                             // false (a space is also a grapheme cluster)
```

The complete String method registry lives in `crates/syx-stdlib/src/string_methods.rs` (`STRING_METHODS`). `examples/main.sy` gives an end-to-end example.

| Method | Signature | Description |
|------|------|------|
| `split(sep: String)` | `Array<String>` | Split by separator |
| `trim()` | `String` | Strip leading/trailing Unicode whitespace |
| `lowercased()` | `String` | Unicode **simple case mapping** (Rust; **≠** `caseFolded`; **≠** locale) |
| `uppercased()` | `String` | Unicode **simple case mapping** (Rust; **≠** `caseFolded`; **≠** locale) |
| `normalizedNFC(text:)` | `String` | Unicode NFC (ADR-228) |
| `normalizedNFD(text:)` | `String` | Unicode NFD (ADR-228) |
| `caseFolded(text:)` | `String` | Unicode default/full case folding (e.g. `ß`→`ss`; ADR-228) |
| `hasPrefix(s)` | `Bool` | Prefix test |
| `hasSuffix(s)` | `Bool` | Suffix test |
| `substring(start, end)` | `String` | Scalar substring [start, end) |
| `replacingOccurrences(target, with)` | `String` | Replacement (first / all depending on the implementation) |
| `matches(pattern)` | `Array<String>` | Regex matching |
| `count` | `Int` | Number of grapheme clusters (**not** bytes) |
| `isEmpty` | `Bool` | Zero grapheme clusters |
| `repeat(n)` | `String` | Repeated n times |

## Iterating Over Strings

```syx
for c in "abc" {
    print(c)    // Prints "a", "b", "c" in turn
}
```

The loop variable `c` has type **`Character`** (since Phase 139 a distinct scalar type; at runtime it is not a multi-character `String`).

> **Implementation status:** `for-in` is **Implemented**; the loop variable is `Character` (MVP GA; `fixtures/lang/character.sy`).

## Internal Representation (Important)

| Layer | SYX |
|------|-----|
| Storage | UTF-8 byte array (Rust `String` back end) |
| `.count` / `substring` / `for-in` | **Unicode grapheme clusters** (since Phase 84, indexed with `unicode-segmentation`) |
| `Character` type | **MVP GA** (Phase 139; runtime `Value::Character` scalar; not a complete Unicode property library) |
| `Character.isASCII()` | **MVP GA** (single-scalar check; not a complete ASCII property judgment) |

> **Swift alignment:** combined emoji (e.g. 👨‍👩‍👧‍👦, a family) count as 1 "character" in Swift; under the Phase 84 grapheme-cluster index, SYX also counts 1.
>
> **Legacy markers:** before Phase 84, `.count` counted Unicode scalars; old fixtures (`stdlib/string_index_honesty.rs` and others) keep scalar behavior tests. If `.count` disagrees with earlier documentation, trust the current behavior.

## Subscripts `s[i]`

```syx
let s = "syx"
let c: Character = s[0]      // Grapheme Character
print(c)                     // "s"

let second = s[1]            // "w"
```

| Behavior | SYX |
|------|-----|
| Index semantics | **Int grapheme offset** (Phase 84; ADR-227) |
| Complexity | **O(n) for a single random access** (no opaque Index / no O(1) cache) |
| Return type | `Character` (grapheme cluster) |
| Out of bounds | Runtime error **R003** (message contains `out of bounds`; ≠ an empty Character; ≠ R013) |
| vs. Swift | **≠** opaque `String.Index`; SYX uses integer offsets |

> **Range access:** the slicing syntax `s[0..<3]` is not yet provided; use `substring(0, 3)` (also an O(n) grapheme offset). Pinning test: `m2_s2_02_string_index_honesty`.

## String Concatenation

```syx
let a = "Hello"
let b = ", World"
let c = a + b                       // "Hello, World"
let d = "\(a)\(b)!"                 // Concatenation via interpolation
let e = String.concat([a, b, "!"])  // Array form
```

The `+` operator on `String` is the low-level lowering of the `StringConcat` instruction (**not** a Core Host function); at runtime it is inlined into a byte copy.

## String Comparison

```syx
"abc" == "abc"     // true (value equality, Equatable witness)
"abc" < "abd"      // true (lexicographic)
"abc" === "abc"    // false (different String objects; === compares references)
```

> **Comparison semantics:** `==` compares grapheme-cluster sequences (since Phase 84); `<` is lexicographic (`Comparable` witness). `===` compares object identity at the VM/MIR layer (rarely used).

## Regex Literals

```syx
let p = /\d+/                       // Regex literal
let matches = "abc123def".matches(p)
for m in matches { print(m) }       // "123"

let ci = /hello/i                   // Case insensitive (compile-time marker)
```

| Form | Status | Description |
|------|------|------|
| `/pattern/` | **Implemented** | Parsed by `crates/syx-regex`; disambiguated by context |
| `/pattern/i` | **Partially implemented** | The flag is accepted by the parser; runtime support is limited |
| `/pattern/m` `/pattern/s` | **Not implemented** | Multiline / single-line flags |

> **Disambiguation:** `/` does not occur at the start of a comment (line comments use `//`), but it can be ambiguous in division expressions (`x/y` vs. a regex). The parser distinguishes them by token lookahead; typical cases need no attention.

See [Language Reference: Lexical Structure](../language-reference/01-lexical-structure.md).

## Converting Between Strings and Collections

```syx
let s = "a,b,c"
let parts = s.split(",")                  // ["a", "b", "c"]
let joined = parts.joined("-")            // "a-b-c"

let chars: Array<Character> = Array(s)    // Array of grapheme clusters
let back = chars.joined("")               // "abc"
```

| Direction | Method |
|------|------|
| String → Array<String> | `s.split(sep)` |
| Array<String> → String | `arr.joined(sep)` |
| String → Array<Character> | `Array(s)` (standard library) |
| Array<Character> → String | `chars.joined("")` |

## Complete Example

Source `fixtures/lang/string_multiline.sy`:

```syx
func main() {
    let name = "SYX"
    print("Hello, \(name)!")

    print("  Hello, World  ".trim())

    let parts = "a,b,c".split(",")
    for item in parts {
        print(item)
    }

    for m in "abc123def".matches("\\d+") {
        print(m)
    }

    let digits: Array<Character> = Array("0123456789")
    print(digits.count)              // 10
    print(digits[0])                  // "0"
}
```

## Internal APIs (Core Extensions)

The String witnesses (`hash` / `description` / `debugDescription`) are implemented through Core extensions → Host functions (see [Chapter 32, Operators vs Core Host](32-operators-vs-host.md)):

```syx
extension String {
    func hash(into hasher: Hasher) {
        __syx_hasher_combine(hasher, self)
    }
    func description() -> String {
        return __syx_value_description(self)
    }
}
```

`String` is fully registered in `crates/syx-stdlib/src/builtins/string.sy`; user code generally need not call `__syx_*` directly.

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **L001** | Unterminated string | Close the `"` |
| **R003** | `s[i]` out of bounds (including negative indices) | Check the index; ≠ an empty Character |
| **S004** | Using `String` and `Array` interchangeably | Use an explicit `.split()` / `.joined()` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `Character` is a standalone struct | `Character` is a `String` type alias (a single runtime scalar) |
| opaque `String.Index` | **Int grapheme offset** (O(n) random access; ≠ opaque Index) |
| `s[i]` returns `Character` | Returns a grapheme `Character` (ADR-226/227) |
| `s[range]` slicing | `s.substring(start, end)` |
| `String(format:)` | **Not implemented**; use interpolation or `Int.toString()` |
| `count` | `.count` (consistent) |
| `String == String` | Grapheme-cluster sequence equality (since Phase 84) |
| `String` literal interpolation | Compatible with Swift's `\(expr)` syntax |

---

**Next chapter** → [Collection Types](05-collection-types.md)

**See also** → [Language Reference: Lexical Structure](../language-reference/01-lexical-structure.md) · [Chapter 5, Collection Types](05-collection-types.md) · [Chapter 32, Operators vs Core Host](32-operators-vs-host.md)
