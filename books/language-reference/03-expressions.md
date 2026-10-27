# Expressions

**Implementation status:** **Implemented** (`Expr` enum, complete precedence table); custom operators via `operator func` **GA** (three-back-end parity); optional chaining / nil coalescing / force unwrap **Implemented**

Expressions produce values. The table below is ordered **from highest to lowest precedence** (matching how the parser climbs).

---

## Precedence (Complete Table)

| Level | Precedence | Form | Operator / description |
|------|--------|------|----------------|
| 1 (highest) | — | **Postfix** | `.` `[]` `()` `?.` `!` (postfix) `await` (postfix) |
| 2 | — | **Unary prefix** | `-` `!` `~` `try` `&` (borrow) `await` (prefix) |
| 3 | 100 | Multiplicative | `*` `/` `%` `<<` `>>` (shifts mix into this level) |
| 4 | 90 | Additive | `+` `-` |
| 5 | 80 | Bit shift | `<<` `>>` (Partially implemented) |
| 6 | 70 | Range | `..<` `...` |
| 7 | 70 | Ordered comparison | `<` `<=` `>` `>=` |
| 8 | 60 | Equality | `==` `!=` `===` `!==` |
| 9 | 50 | Bitwise AND | `&` |
| 10 | 40 | Bitwise XOR | `^` |
| 11 | 30 | Bitwise OR | `\|` |
| 12 | 20 | Logical AND | `&&` |
| 13 | 10 | Logical OR | `\|\|` |
| 14 | 5 | Nil coalescing | `??` |
| 15 | 10 | Compound assignment | `+=` `-=` `*=` `/=` `%=` |
| 16 (lowest) | 5 | Assignment | `=` |

> **Parentheses win:** `()` may be added anywhere to force precedence, with no side effects.
> **Assignment expressions** have `Void` semantics (do not depend on chaining).

## Literal Expressions

```
literal ::= int-literal | float-literal | string-literal | bool-literal | nil-literal
          | array-literal | dict-literal | object-literal | regex-literal
```

```syx
42
3.14f32
"text"
true
nil
[1, 2, 3]
["k": "v"]
{ "name": "SYX" }
/\d+/
```

## Identifiers

```
identifier-expr ::= identifier
```

```syx
count
MyType
self                                        // Current instance
super                                       // Superclass
```

## Tuple Expressions

```
tuple-expr ::= '(' expr (',' expr)+ ')'
            |  '(' expr ',' ')'              // One-element tuple
```

```syx
let t = (1, "a", true)                      // (Int, String, Bool)
let (x, y) = (1, 2)                         // Destructuring
```

## Unary and Binary

```syx
-a                                          // Unary minus
!flag                                       // Logical NOT
~bits                                       // Bitwise NOT
a + b
a * b
a % b
a == b
a < b
a && b
a || b
expr?                                       // try chain: convert error to nil / propagate
```

## Calls

```
call-expr ::= expr '(' arg-list? ')'
arg-list  ::= (label ':')? expr (',' (label ':')? expr)*
```

```syx
foo()
foo(x: 1, y: 2)                             // Named arguments
obj.method(arg)
arr.map { x in x * 2 }                      // Trailing closure
```

> **Named arguments:** the caller **must** write `label: value` (there is no `_` elision syntax).

## Member Access

```
member-expr ::= expr '.' identifier
```

```syx
obj.field
obj.length
arr.map
self.name
```

## Subscripts

```
index-expr ::= expr '[' expr ']'
            |  expr '[' expr (',' expr)* ']'
```

```syx
arr[i]
dict["key"]                                 // Dictionary
s[0]                                        // String (single scalar)
m[0, 1]                                     // Matrix multi-parameter
obj["field"]                                // Dynamic Object
```

**Implementation status:**
- `arr[i]` / `dict["key"]` / `s[i]` **Implemented**
- `s[0..<3]` slicing **Not implemented** (use `substring`)
- Custom `subscript` **Partially implemented** (Phase 40/41)

## Optional Chaining and Force Unwrap

```
optional-chain ::= expr '?' '.' identifier
                |  expr '?' '[' expr ']'
                |  expr '?' '(' arg-list? ')'
force-unwrap   ::= expr '!'
```

```syx
maybe?.length                               // Optional chain
maybe?.foo(1, 2)
maybe!                                       // Force unwrap (nil → R007)
```

**Diagnostic:** R007 (runtime nil force unwrap)

## await

```
await-expr ::= 'await' expr
```

```syx
await asyncFunc()
await ch.receive()                          // Asynchronous receive
```

> Legal only inside an `async` function body; `await` outside `async` → **S020**.

## Closure Expressions

```
closure-expr ::= '{' closure-params? closure-body? '}'
closure-params ::= ('|' (identifier (',' identifier)* '|') | (identifier (',' identifier)*))
                |  '(' (identifier ':' type (',' identifier ':' type)*)? ')'
closure-body ::= 'in' expr | 'in' block
```

```syx
{ x in x + 1 }                              // Single expression
{ (a, b) in a + b }                         // Multiple parameters
{ print("hi") }                             // No parameters
{ (x: Int) -> Int in x * 2 }               // Explicit types
{ |x| x + x }                               // Pipe parameters (**partially supported**)
```

## Array Literals

```
array-literal ::= '[' (expr (',' expr)*)? ']'
```

```syx
[10, 20, 30]
[]                                          // Empty array (needs a type annotation)
[1, 2, 3].map { $0 * 2 }                    // Immediate call
```

## Dictionary Literals

```
dict-literal ::= '[' dict-entry (',' dict-entry)* ']'
dict-entry   ::= expr ':' expr
object-literal ::= '{' object-entry (',' object-entry)* '}'
object-entry  ::= string-literal ':' expr
```

```syx
["a": 1, "b": 2]                            // Dictionary
{ "name": "SYX", "version": 110 }           // Object
```

## Task Blocks

```
task-expr ::= 'Task' ('.' 'detached')? closure-expr
```

```syx
Task { … }
Task.detached { … }
```

## Borrow Expressions

```
borrow-expr ::= '&' 'mut'? expr
```

```syx
&value
&mut counter
```

## Ranges (Half-Open / Closed)

```
range-expr ::= expr '..<' expr
            |  expr '...' expr
            |  '..<' expr
            |  expr '..<'
```

```syx
0..<10                                      // Half-open
1...5                                       // Closed range (**Partially implemented**)
```

> Used in `for-in` and `Range` type contexts.

## Pattern Expressions (switch / if case)

```
pattern-expr ::= pattern '=' expr
             |  'if' 'case' pattern '=' expr block
             |  'switch' '(' expr ')' '{' case-clause* '}'
```

```syx
if case .ok(let x) = result { … }
switch (result) {
case .ok(let v) { … }
default { … }
}
```

See [Patterns](07-patterns.md).

## Assignment Expressions

```
assign-expr ::= lvalue '=' expr
            |  lvalue ('+=' | '-=' | '*=' | '/=' | '%=') expr
```

```syx
x = 1
x += 2
obj.field = v
arr[i] = v
```

> **Assignment expressions** have `Void` semantics (do not depend on chaining). `a = b = c` is **Not implemented**.

## Custom Operators

```
operator-decl ::= 'prefix'|'infix'|'postfix' 'operator' identifier
                  '{' 'precedence' ':' int ',' 'associativity' ':' ('left'|'right'|'none') '}'
```

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }
```

The complete grammar is in [Chapter 30, Advanced Operators](../language-guide/30-advanced-operators.md).

## Expression Classification

| Category | Example |
|------|------|
| Literal | `42`, `"x"`, `[1,2,3]` |
| Identifier | `count`, `self`, `super` |
| Path | `obj.field`, `arr[i]` |
| Call | `f()`, `obj.method(x)` |
| Unary | `-x`, `!flag`, `&mut x` |
| Binary | `a + b`, `a < b` |
| Range | `0..<n`, `1...5` |
| Optional | `opt?.x`, `opt!` |
| Closure | `{ x in x + 1 }` |
| Async | `await future` |
| try | `try? risky()` |
| Control-flow expression | `if (x) { a } else { b }` |
| Keyword | `nil`, `true`, `false` |
| String interpolation | `"\(name)"` |

## Expression Type Checking

| Rule | Description |
|------|------|
| Both sides same type | Required for arithmetic and comparison (**S004** if incompatible) |
| Integer + float | widening (Int → Double) |
| Signed + unsigned | rejected with **S014** |
| Optional + T | `T?` offers nil coalescing / chaining / force unwrap |
| Function + function | Calls / HOF |
| Tuple + tuple | Element types matched individually |
| `nil` literal | Inferred as `T?` |

---

**Diagnostics:** S004 / S008 / S010 / S014 / S020 / R007

**Usage examples** → [Language Guide: Operators](../language-guide/03-basic-operators.md) · [Advanced Operators](../language-guide/30-advanced-operators.md)

**See also** → [Statements](04-statements.md) · [Types](02-types.md) · [Patterns](07-patterns.md) · [Grammar Summary](10-grammar-summary.md)
