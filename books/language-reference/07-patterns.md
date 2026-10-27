# Patterns

**Implementation status:** **Implemented** (`switch` / `if case` / `if let` / `guard let`); variant / tuple / wildcard / range / associated-value destructuring / where guard / type patterns **Implemented**; `case let x as T` **Not implemented**; `is` patterns **Partially implemented**

Patterns match value structure and bind variables. SYX patterns are similar to Swift's: they are used in `switch` / `case` / `let` contexts.

---

## Pattern Syntax

```
pattern ::= '_'                                  -- Wildcard
         |  identifier                           -- Binding
         |  'let' pattern                        -- Explicit binding
         |  literal                              -- Literal
         |  range-pattern                        -- Range
         |  variant-pattern                      -- Variant
         |  tuple-pattern                        -- Tuple
         |  type-identifier                      -- Type annotation
         |  '(' pattern ')'                      -- Parentheses
         |  pattern 'as' type                    -- Type pattern (**partial**)

range-pattern    ::= expr '..<' expr
variant-pattern  ::= '.' identifier ('(' pattern? (',' pattern)* ')')?
tuple-pattern    ::= '(' pattern (',' pattern)* ')'
```

## Variant Patterns

```
variant-pattern ::= '.' identifier ('(' pattern? (',' pattern)* ')')?
```

```syx
.ok(let value)
.err(let message)
.success(let data)
.failure(code: let c, message: let m)
.failure(let c, let m)                         // Positional destructuring
.empty                                         // No associated value
```

| Form | Meaning |
|------|------|
| `.ok` | No associated value |
| `.ok(let x)` | Destructures and binds a single associated value as `x` |
| `.failure(code: let c, message: let m)` | Named associated-value destructuring |
| `.failure(let c, let m)` | Positional destructuring |
| `.failure(let c, _) ` | Partial destructuring (`_` explicitly ignored) |

## Tuple Patterns

```syx
(a, b)                                        // Positional
(let x, let y)                                // Explicit binding
(1, let name)                                 // Mixed
```

> Element patterns match positionally; types must be compatible with the subject.

## Literal Patterns

```syx
case 1 { … }                                  // Integer
case "hello" { … }                            // String
case true { … }                               // Boolean
case nil { … }                                // nil
```

> Literal patterns compare by value; `==` goes through the `Equatable` witness.

## Range Patterns

```syx
case 0..<10 { … }                             // Half-open
case 10..<100 { … }
case 100..<1000 { … }
```

> Closed ranges `a...b` are **Partially implemented**. `0..<n` goes through `Range<Int>`.

## The Wildcard `_`

```syx
let (_, x) = (1, 2)                           // Discard the first
case .ok(let _) { … }                         // Discard the associated value
```

## Explicit `let` Binding

```syx
case let x { … }                              // Bind the whole subject
case .ok(let v) { … }                         // Bind the associated value
```

> `let x` is equivalent to `x` (patterns bind by default); the `let` keyword makes the intent explicit.

## Patterns in switch

```syx
switch (result) {
case .ok(let v) {
    print(v)
}
case .err(let e) where (e.length > 0) {
    print(e)
}
default {
    print("fallback")
}
}
```

> The complete form is `SwitchCase { pattern, guard, body }`.

## if case

```syx
if case .ok(let x) = result {
    print(x)
}
```

> `if case` is a convenient single-branch form of `switch`; its patterns are the same as those of a `switch` case.

## if let / guard let

```syx
if let x = optional { … }
if let x = optional, let y = other, cond { … }  // Multiple conditions + guard
guard let x = optional else { return }
```

> **Chained conditions:** separated by `,`; if any condition fails, the whole branch is skipped.
> **`guard else` must exit:** otherwise **S020**.

## Type Patterns (Partially Implemented)

```syx
case let x as String { … }                     // ⚠️ Partially implemented
```

> The current implementation supports only basic types on the cast host path; complete type patterns → later Phases.

## where Guards

```syx
case .ok(let v) where (v > 0) { … }
case let x where (x % 2 == 0) { … }
```

> A `where` expression must be `Bool`; separate multiple `where` patterns with `,`.

## Patterns and Types

- Pattern types must be compatible with the subject
- On failure, the branch is skipped (open domain) or a compile error is raised (a closed domain lacking `default` → **S007**)

| Scenario | Behavior when `default` is missing |
|------|-------------------|
| `enum` / `final class` / `Bool` | **S007** (must be exhaustive) |
| `Int` / `String` | Warning (**partial**) |
| Protocol / `Any` | `default` required |

## Patterns and Literal Inference

```syx
let p: any Named = obj
if case let x where (x is User) { … }         // x: any Named (subtype User)
```

> Type inference inside a pattern follows the clause after `case`; this differs from let-binding (where the subject type is fixed).

## Exhaustiveness Checking

On a closed domain (`enum` / `final class` / `Bool`), `switch` performs automatic exhaustiveness checking; missing `default` and incomplete case coverage → **S007**.

```syx
enum Color { case red, green, blue }

func describe(_ c: Color) -> String {
    switch (c) {
    case .red { return "red" }
    case .green { return "green" }
    case .blue { return "blue" }
    }                                          // ✅ Fully covered
}
```

## Not Implemented / Planned

| Swift pattern | SYX status |
|------------|---------|
| `case let x as T` | **Partially implemented** |
| Full value binding `case .red` without binding | Supported (`.red { }`) |
| `_` explicit ignore | **Implemented** |
| Nested patterns / complex destructuring | **Implemented** (basic) |
| Interval arrays `[1, 2, ...]` | **Not implemented** |
| Regex patterns | **Not implemented** |

## Complete Example

```syx
enum Tree {
    case leaf(Int)
    case branch(left: Tree, right: Tree)
}

func depth(_ t: Tree) -> Int {
    switch (t) {
    case .leaf { return 1 }
    case .branch(let l, let r) { return 1 + max(depth(l), depth(r)) }
    }
}

func describe(_ t: Tree) -> String {
    switch (t) {
    case .leaf(let n) where (n > 0) { return "leaf(\(n))" }
    case .leaf { return "leaf(0)" }
    case .branch(let l, let r) { return "branch(\(describe(l)), \(describe(r)))" }
    }
}

func main() {
    let t = Tree.branch(left: .leaf(1), right: .leaf(2))
    print(depth(t))                            // 2
    print(describe(t))                         // "branch(leaf(1), leaf(2))"
}
```

---

**Diagnostics:** S004 / S007 / S010 / S012 / S020

**Usage examples** → [Language Guide: Enumerations](../language-guide/09-enumerations.md) · [Control Flow](../language-guide/06-control-flow.md) · [Advanced Operators](../language-guide/30-advanced-operators.md)

**See also** → [Statements](04-statements.md) · [Expressions](03-expressions.md) · [Types](02-types.md)

**Tests:** `fixtures/format/switch_where_mvp` · `fixtures/format/switch_where_mvp` · `fixtures/format/switch_where_mvp`
