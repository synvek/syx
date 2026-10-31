# Control Flow

**Implementation status:** `if` / `else` / `while` / `repeat while` / `for-in` / `for await in` / `break` / `continue` / labeled `break`/`continue` / `switch` / `where` guards / `if case` / `if let` / `guard let` are **all Implemented**; fallthrough **Rejected**

SYX control flow follows the Swift style: blocks are wrapped in `{ }`, and pattern matching is done with `switch` + `where`. This chapter explains each control structure by category.

> **Condition parentheses are optional:** `if` / `while` / `switch` conditions are **not required** to be wrapped in `()`. Both forms below are legal:
>
> ```syx
> if x > 0 { print("pos") }              // Recommended (consistent with Swift)
> if (x > 0) { print("pos") }              // Still legal (compatible with older code)
> ```
>
> The examples in this chapter prefer the modern style with parentheses **omitted**.

## if / else

```syx,fragment
let x = 1
if x > 0 {
    print("positive")
} else if x == 0 {
    print("zero")
} else {
    print("negative")
}
```

| Point | Description |
|--------|------|
| Condition parentheses | **Optional**; both `if x` and `if (x)` are legal |
| Condition type | Must be `Bool`; otherwise → **S004** |
| `else if` | Chaining is legal; no nesting depth limit |
| Omitting blocks | `if x { print(x) }` is legal (a single statement may omit `{}`) |

> **"if expressions":** since Phase 100+, `if` may return a value as an expression; when used in an assignment, both branches must exist and have the same type.

```syx,fragment
let x = 1
let label = if x > 0 { "pos" } else { "non-pos" }
```

## while

```syx
var x = 10
while x > 0 {
    print(x)
    x = x - 1
}
```

The condition is checked before the loop body; when the condition is `false`, the body does not execute.

## repeat while

```syx
var n = 0
repeat {
    n = n + 1
    print(n)
} while n < 3
```

The loop body executes **at least** once; the condition is checked after each iteration.

> **Swift comparison:** Swift uses `repeat { } while …`; SYX is the same. Swift's `do { } while …` form (without `repeat`) is not recognized.

## for-in

```syx,fragment
for item in [1, 2, 3] {
    print(item)
}

for c in "abc" {
    print(c)
}

for i in 0..<10 {
    print(i)
}
```

> **`for await`:** `for await x in asyncSeq { … }` awaits each asynchronous element; see [Chapter 18, Concurrency](18-concurrency.md). Not doctested in this chapter.

Iterable sources:

| Form | Element type |
|------|----------|
| `Array<T>` | `T` |
| `Set<T>` | `T` |
| `Dictionary<K, V>` | `(K, V)` tuple |
| `String` | `String` (single scalar) |
| `0..<n` | `Int` |
| Custom `Iterator` | `String` (protocol requirement) |
| `Object` | `(String, Any)` |
| async sequence | the `await` element type |

### `for await in`

`for await x in asyncSeq` awaits each asynchronous element in turn; see [Chapter 18, Concurrency](18-concurrency.md).

## break / continue

```syx,fragment
var done = false
var skip = false
while true {
    if done {
        break         // Exit the nearest loop
    }
    if skip {
        continue      // Go to the next round
    }
    break
}
```

| Form | Behavior |
|------|------|
| `break` | Exits the **innermost** loop or `switch` |
| `continue` | Jumps to the next round of the **innermost** loop |
| `break label` | Exits the loop with the given label (**Implemented**) |
| `continue label` | Jumps to the next round of the labeled loop (**Implemented**) |

> **Labels:** the syntax is `label: while (cond) { … break label … }`; SYX recognizes labels in nested loops, but recognition across functions / closures is still evolving.

## switch

`switch` is the core way to express multi-way branching in SYX; it supports **patterns + guards + exhaustiveness**:

```syx
enum Box {
    case ok(data: String)
    case err(e: String)
}
let value = Box.ok(data: "hi")
switch value {
case .ok(let data) {
    print(data)
}
case .err(let e) where e.count > 0 {
    print(e)
}
default {
    print("other")
}
}

let num = 5
switch num {
case 1 {
    print("one")
}
case 2, 3, 5, 7 {
    print("small prime")
}
case 0..<10 {
    print("single digit")
}
default {
    print("other")
}
}
```

| Form | Description |
|------|------|
| `case pattern` | Pattern matching |
| `case pattern where guard` | Guard (executes only when `guard` is true) |
| `case pat1, pat2` | Multiple patterns (`\|` is internal to the parser; the `pat1, pat2` form is legal) |
| `case 0..<10` | Range pattern (`Range<Int>` context) |
| `case .variant(let x)` | Enum/Result variant pattern |
| `default` | Default branch; not mandatory (may be omitted for a closed domain such as `enum` / `final class`, but that raises a warning) |

### switch Block Syntax (Important)

```syx
let x = 1
switch x {
case 1 {           // Braces wrap the case body; unlike Swift
    print("one")
}
case 2 {
    print("two")
}
}
```

> **Key point:** in SYX, **`case` is followed by a `{ }` block**, **not** `case 1:` plus indentation. Early Phases accepted the `case 1:` form (consistent with Swift), but the block form `case 1 { }` is **recommended**.

### Patterns (Details)

```syx
enum Outcome {
    case ok(value: String)
    case err(message: String)
    case failure(code: Int, message: String)
    case empty
}
let result = Outcome.ok(value: "hi")
switch result {
case .ok(let value) {                      // Destructuring variant
    print("ok: \(value)")
}
case .err(let message) {                   // Named associated value
    print("err: \(message)")
}
case .failure(code: let c, message: let m) {  // Named parameter destructuring
    print("fail code=\(c) msg=\(m)")
}
case .empty {                              // No associated value
    print("empty")
}
default {
    print("other")
}
}
```

See [Chapter 9, Enumerations](09-enumerations.md) and [Language Reference: Patterns](../language-reference/07-patterns.md).

### Range Cases

```syx
let n = 42
switch n {
case 0..<10 { print("single digit") }
case 10..<100 { print("two digits") }
case 100..<1000 { print("three digits") }
default { print("large") }
}
```

Closed ranges `a...b` are **Partially implemented**; prefer the half-open `a..<b` plus a trailing `Int.max`.

### Exhaustiveness Checking

`switch` performs automatic exhaustiveness checking over closed domains such as `enum` / `final class` / `Int` / `Bool`; a missing `default` with incomplete case coverage → **S007**.

```syx
enum Color { case red, green, blue }

func describe(_ c: Color) -> String {
    switch c {
    case .red { return "red" }
    case .green { return "green" }
    case .blue { return "blue" }
    }                                       // ✅ Fully covered
}
```

Non-closed domains (such as `String`, `Any`) require `default`.

### `where` Guards

```syx
let n = 4
switch n {
case let x where x < 0 {
    print("negative: \(x)")
}
case let x where x % 2 == 0 {
    print("even: \(x)")
}
case let x {
    print("odd: \(x)")
}
}
```

A `where` expression must be `Bool`; multiple `where`s and multiple patterns are separated by `,` (accepted by the parser).

### fallthrough

`fallthrough` is **Rejected** (compile error). A SYX `case` does **not** fall through automatically; you need an explicit `break` at the end of the case body (the default behavior is already break, so it may be omitted).

## if case

```syx
enum Box {
    case ok(data: String)
    case err
}
let result = Box.ok(data: "hi")
if case .ok(let data) = result {
    print(data)
} else {
    print("not ok")
}
```

`if case` is a convenient single-branch form of `switch`. Its patterns are the same as those of a `switch` case.

## if let / guard let

Unwrapping `Optional`:

```syx
func demoOptional() {
    let optional: Int? = 1
    if let x = optional {
        print("got: \(x)")
    } else {
        print("nil")
    }

    guard let y = optional else {
        return                              // Early exit (else must exit: return / throw / break / continue)
    }
    print("y is \(y)")                       // Here y has been unwrapped and is not Optional
}
demoOptional()
```

| Form | Behavior |
|------|------|
| `if let x = opt { }` | Enters the branch when non-`nil`; `x` is the unwrapped value; `else` is optional |
| `if let x = opt, let y = other` | Chained unwrapping (multiple conditions separated by commas) |
| `guard let x = opt else { … }` | Continues when non-`nil`; entering `else` on `nil` **must** exit (`return` / `throw` / `break` / `continue` / `fatalError`) |
| `guard let x = opt, cond else` | Multiple conditions + guard |

> **`else` must exit:** if a `guard`'s `else` block does not exit (missing `return` etc.) → compile error **S020**.

## throw

```syx
import IO

func loadFile(path: String) throws -> String {
    if !File.exists(path) {
        throw "not found"
    }
    return File.read(path: path)
}
```

`throw "msg"` throws a string exception (MVP type). For `try` / `catch`, see [Chapter 17, Error Handling](17-error-handling.md).

## Early Exit

SYX provides four early-exit paths: `return` / `throw` / `break` / `continue`. `return` may be omitted inside a function body (implicitly returning `Void`):

```syx
func maybeDouble(x: Int?) -> Int {
    guard let v = x else {
        return 0                          // ✅ Explicit
    }
    return v * 2
}
```

## Control Flow and Closures

A closure may carry control flow (such as `for` / `if`), but **break / continue** apply only to the nearest loop by default (they cannot cross closure boundaries). To break out of an outer loop, use a **label**:

```syx
func firstStop(groups: Array<Array<Int>>) -> Int {
    outer: for group in groups {
        for n in group {
            if n < 0 {
                break outer
            }
        }
    }
    return 0
}

func main() {
    print(firstStop(groups: [[1, 2], [-1, 9]]))
}
```

## Complete Example

```syx
import IO

func classify(path: String) -> String {
    if File.exists(path) {
        let content = File.read(path: path)
        let n = content.count

        switch n {
        case 0 { return "empty" }
        case 1..<100 { return "tiny" }
        case 100..<10_000 { return "medium" }
        default { return "large" }
        }
    }
    return "missing"
}

func main() {
    for p in ["/etc/hostname", "/tmp/empty.txt", "/tmp/big.txt"] {
        print("\(p) → \(classify(path: p))")
    }
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **P001** | Syntax parse error (mismatched brackets, missing operators, …) | Check the surrounding syntax |
| **S004** | Non-`Bool` `if` condition | Write an explicit comparison |
| **S007** | Non-exhaustive `switch` (`enum` / closed domain) | Add the missing cases or a `default` |
| **S010** | Pattern mismatch | Check whether all enum cases are present |
| **S020** | `guard let` `else` does not exit | Add `return` / `throw` etc. |
| **R003** | Subscript out of bounds | Check the index |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `if x` without parentheses | ✅ **Consistent**; both `if x` and `if (x)` are legal (omitting is recommended) |
| `switch x` | ✅ **Consistent**; both `switch x` and `switch (x)` are legal (omitting is recommended) |
| `case 1:` indentation style | The block style `case 1 { }` is recommended |
| `fallthrough` | **Rejected** (compile error) |
| Labeled `break label` | **Partially implemented** (still evolving) |
| `where case` | **Not implemented** (use `if case` chaining) |
| `if #available` | **Not implemented** |
| `do { try … } catch { }` | Use `try { } catch { }` (there is no `do` keyword) |

---

**Next chapter** → [Functions](07-functions.md)

**See also** → [Language Reference: Statements](../language-reference/04-statements.md) · [Chapter 7, Functions](07-functions.md) · [Chapter 17, Error Handling](17-error-handling.md)
