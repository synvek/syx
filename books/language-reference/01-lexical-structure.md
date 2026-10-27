# Lexical Structure

**Implementation status:** **Implemented** (`syx-lexer`, complete token set; regex literals Phase 50+; nested block comments; identifiers UAX #31 XID — ADR-229 / `M2-S2-04`)

SYX source files are UTF-8 text (without a BOM). Lexical analysis splits source into a `Token` stream for the parser.

---

## Character Set

| Element | Range |
|------|------|
| Source encoding | **UTF-8** (no BOM) |
| Line terminator | `\n` (LF) / `\r\n` (CRLF, normalized) |
| Whitespace | ` ` / `\t` / `\v` / `\f` |

> **Any Unicode character** may appear anywhere in source (comments, string literals, identifiers); string literals support the `\u{…}` escape.

## Identifiers

```
identifier ::= letter (letter | digit | '_')*
letter     ::= [a-zA-Z_] | <unicode XID_Start>
```

| Rule | Description |
|------|------|
| First character | Letter or `_` |
| Subsequent | Letter, digit, `_` |
| Case sensitivity | `Foo` ≠ `foo` |
| Unicode | **Implemented** (`XID_Start` / `XID_Continue` via `unicode-xid`; ADR-229) |
| Normalization | Identifiers are **not** NFC-normalized; comparison is source-spelling equality |
| Keywords | Cannot be used as ordinary identifiers |

```syx
count
_private
MyType
helloWorld
αβγ                           // Unicode XID (valid)
```

> **Diagnostics:** L006 illegal character; L001 identifier naming lint (lint rule).

## Keywords

Reserved words cannot be used as ordinary identifiers. The complete list is grouped as follows:

### Declarations

| Keyword | Meaning |
|--------|------|
| `class` | Reference type |
| `struct` | Value type |
| `enum` | Enumeration |
| `protocol` | Protocol |
| `extension` | Extension |
| `actor` | Concurrency-isolated type |
| `func` | Function |
| `init` | Initializer |
| `deinit` | Deinitializer |
| `var` | Mutable binding |
| `let` | Immutable binding |
| `import` | Import a module |
| `typealias` | Type alias |
| `associatedtype` | Associated type |
| `global` | Global actor (modifies `actor`) |
| `data` | data class modifier (`data class`) |
| `final` | Forbids subclassing (`final class` / `final func`) |

### Access Control

| Keyword | Meaning |
|--------|------|
| `public` | Across modules |
| `internal` | Within the module (default) |
| `private` | File / scope |
| `fileprivate` | Within the file |

### Control Flow

| Keyword | Meaning |
|--------|------|
| `if` | Condition |
| `else` | Otherwise |
| `guard` | Early-exit guard |
| `switch` | Multi-way branch |
| `case` | Branch |
| `default` | Default |
| `for` | Loop |
| `in` | for-in container |
| `while` | Conditional loop |
| `repeat` | repeat while |
| `break` | Break out |
| `continue` | Continue |
| `return` | Return |
| `fallthrough` | **Rejected** (not supported by SYX) |

### Concurrency

| Keyword | Meaning |
|--------|------|
| `async` | Asynchronous function |
| `await` | Asynchronous wait |
| `Task` | Task |
| `actor` | Isolated object |
| `global` | Global actor modifier |
| `isolated` | Isolation (partial) |
| `sending` | Pass parameters across isolation (**Phase 26**) |
| `distributed` | distributed actor (**partial**) |

### Types

| Keyword | Meaning |
|--------|------|
| `Self` | Implementing type |
| `Type` | Metatype |
| `some` | Opaque result |
| `any` | Existential type |

### Error Handling

| Keyword | Meaning |
|--------|------|
| `try` | Error propagation |
| `catch` | Catch |
| `throw` | Throw |
| `throws` | Error signature (`throws T` typed throws **Partially implemented**) |

### Literals

| Keyword | Meaning |
|--------|------|
| `true` | Boolean true |
| `false` | Boolean false |
| `nil` | Null value |

### Generics and Metaprogramming

| Keyword | Meaning |
|--------|------|
| `where` | Generic constraint |
| `requires` | Effect annotation |
| `macro` | User macro declaration |
| `compileTime` | Constant evaluation |
| `Pure` / `IO` | Effect labels |

### Contextual Keywords

| Keyword | Meaning |
|--------|------|
| `static` | Static (type method / field) |
| `final` | Forbids override |
| `override` | Override |
| `lazy` | Lazy property (class only) |
| `mutating` | Mutating method |
| `convenience` | Convenience init (**Not implemented**) |
| `required` | Required init (**Not implemented**) |
| `prefix` / `infix` / `postfix` | Custom operators |
| `associativity` | Associativity |
| `precedence` | Precedence |

The full list lives in `crates/syx-lexer/src/token.rs`.

## Literals

### Integer Literals

```
int-literal ::= digit+ (suffix)?
             | '0x' hex-digit+ (suffix)?
             | '0b' binary-digit+ (suffix)?
             | '0o' octal-digit+ (suffix)?

suffix ::= 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'isize'    -- signed
        | 'u8' | 'u16' | 'u32' | 'u64' | 'u128' | 'usize'    -- unsigned
```

| Form | Example | Inferred type |
|------|------|----------|
| Decimal | `42` | `Int` |
| Hexadecimal | `0xFF` | `Int` |
| Binary | `0b1010` | `Int` |
| Octal | `0o17` | `Int` |
| Suffix | `100i8` / `0xFFu8` / `42i128` | Corresponding fixed width |
| Underscore | `1_000_000i64` | `_` ignored at compile time |

> **Unsuffixed integers default** to platform **`Int`** (64-bit host → `Int64`; 32-bit → `Int32`).

### Floating-Point Literals

```
float-literal ::= digit+ '.' digit+ (exponent)? (suffix)?
               | digit+ exponent (suffix)?

exponent ::= ('e' | 'E') ('+' | '-')? digit+
suffix    ::= 'f32' | 'f64'
```

| Form | Example | Inferred type |
|------|------|----------|
| Decimal | `3.14` | `Double` |
| Suffix | `3.14f32` / `2.0f64` | `Float32` / `Float64` |
| Scientific notation | `2.5e10` | `Double` |
| Underscore | `1_000.5` | Same as above |

> **Unsuffixed floats default** to `Double`.

### Booleans and Nil

| Literal | Type |
|--------|------|
| `true` | `Bool` |
| `false` | `Bool` |
| `nil` | Contextually inferred (`T?` / `Optional<T>`) |

### String Literals

```
string-literal ::= '"' string-char* '"'
string-char    ::= <any except '"', '\', newline> | escape
escape         ::= '\' ('n' | 'r' | 't' | '\\' | '\"' | '\'' | '0' | 'x' hex-digit hex-digit | 'u' '{' unicode-codepoint '}')
               | '$' '(' expr ')'        -- interpolation
```

| Element | Meaning |
|------|------|
| `"..."` | Delimited by double quotes |
| `\n` `\r` `\t` | Common control characters |
| `\\` `\"` `\'` `\$` | Literal characters |
| `\0` | NUL (**warning**: may truncate the string) |
| `\x41` | ASCII byte (0x00–0xFF) |
| `\u{1F600}` | Unicode code point (U+0000–U+10FFFF) |
| `\(expr)` | Interpolation (arbitrary expression) |

```syx
"Hello, World"
"Hello, \(name)!"
"line one\nline two"
"emoji: \u{1F600}"
"ascii: \x41"
```

> **Raw newlines forbidden:** no bare newline inside a string; an unterminated string reports **L001**.
> **Storage:** UTF-8 byte array (Rust `String` back end).
> **Grapheme clusters:** `.length` / `substring` / `for-in` operate on Unicode **grapheme clusters** (since Phase 84, indexed with `unicode-segmentation`).

### Regex Literals

```
regex-literal ::= '/' regex-body '/' regex-flag?
regex-flag    ::= [a-zA-Z]+
```

| Form | Example | Status |
|------|------|------|
| `/pattern/` | `/\d+/` | **Implemented** |
| `/pattern/i` | `/hello/i` | Partially supported (flag accepted by parser) |
| `/pattern/m` | `/foo/m` | **Not implemented** |

> Disambiguated by `/` and context (not the start of a comment); the typical `let p = /\d+/` is unambiguous.

### Dictionary Literals

```
dict-literal ::= '[' dict-entry (',' dict-entry)* ']' | '{' dict-entry (',' dict-entry)* '}'
dict-entry   ::= expr ':' expr
```

| Form | Syntax | Type |
|------|------|------|
| Dictionary | `["a": 1, "b": 2]` | `[K: V]` static |
| Object | `{ "name": "SYX" }` | Dynamic `Object` |

> When the parser sees **`[key: value, …]`** (first element followed by `:`), it is a Dictionary; **`[elem, …]`** (first element followed by `,` or `]`) is an Array.

## Comments

| Form | Use |
|------|------|
| `// text` | Single line (to end of line or whole line) |
| `/* text */` | Block (**nesting supported**) |
| `/// text` | Documentation comment (extracted by `syx doc`) |

```syx
// Single-line comment

/* Block comment
   /* may nest */
   any line
*/

/// Documentation comment
/// Multiple lines: each starting with `///`
func add(a: Int, b: Int) -> Int { return a + b }
```

## Operators

### Arithmetic and Assignment

| Operator | Meaning | Precedence |
|--------|------|--------|
| `+` `-` `*` `/` `%` | Arithmetic | 100 / 90 |
| `=` | Assignment | 10 |
| `+=` `-=` `*=` `/=` `%=` | Compound assignment | 10 |
| `<<` `>>` | Bit shift | 80 |
| `&` `\|` `^` | Bitwise | 50 / 30 / 40 |

### Comparison and Logic

| Operator | Meaning | Precedence |
|--------|------|--------|
| `==` `!=` | Equality | 60 |
| `===` `!==` | Identity | 60 |
| `<` `<=` `>` `>=` | Ordered comparison | 70 |
| `&&` | Logical AND | 20 |
| `\|\|` | Logical OR | 10 |
| `!` | Logical NOT (unary) | — |
| `??` | Nil coalescing | 5 |
| `?` | try chain / Optional / `?.` | — |
| `!` (postfix) | Force unwrap | — |

### Others

| Operator | Meaning |
|--------|------|
| `->` | Function return type |
| `=>` | Closure fat arrow (**partially supported**) |
| `..<` | Half-open range |
| `...` | Closed range (**partially supported**) |
| `&` | Borrow `&T` / `&mut T` |
| `@` | Attribute `@attached` / `@Component` |
| `#` | Macro `#assert` |
| `.` | Member access |
| `?:` | Ternary (**deliberately not provided**; use an `if` expression; ≠ a pending gap) |

## Separators

```
separator ::= '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' | '.'
```

| Symbol | Use |
|------|------|
| `(` `)` | Tuple / function parameters / condition |
| `{` `}` | Block / dictionary Object / closure |
| `[` `]` | Array / subscript / Dictionary type |
| `,` | List separator |
| `:` | Named parameter / type annotation / dictionary key-value |
| `;` | Empty (optional; never required by SYX) |
| `.` | Member access / decimal point |

## Whitespace

```
whitespace ::= ' ' | '\t' | '\n' | '\r' | <unicode Zs>
```

- Mostly skipped at the lexical layer (except inside string literals)
- Comments are treated as whitespace
- Indentation is not enforced (**no** Python-style mandatory indentation)

---

**Diagnostics:** L001–L012 (lexical / lint)

**Usage examples** → [Language Guide: The Basics](../language-guide/02-the-basics.md) · [Strings](../language-guide/04-strings-and-characters.md)

**See also** → [Types](02-types.md) · [Expressions](03-expressions.md) · [Grammar Summary](10-grammar-summary.md)
