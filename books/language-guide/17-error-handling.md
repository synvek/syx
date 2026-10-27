# Error Handling

**Implementation status:** `try` / `catch` / `throw` / `try?` / `try!` / `Result<T, E>` **Implemented**; `throws T` typed throws **Partially implemented**; rethrows **Not implemented**

SYX provides three error-propagation paths:

1. **throws / try** — raising and catching exceptions (runtime errors)
2. **Result<T, E>** — an explicit algebraic type
3. **Optional** — using `nil` to mean failure (the lightest weight)

This chapter covers syntax, usage, Result integration, and cross-function propagation.

## try / catch Blocks

```syx
func main() {
    try {
        let content = File.read("/tmp/x")
        print(content)
    } catch (err) {
        print("failed: \(err)")
    }
}
```

- Inside a `try { }` block you may `throw` / call `throws` functions
- When an exception fires, control jumps **immediately** to `catch`, skipping the rest of the `try` block
- The MVP type of `err` is `String` (more complex error types are planned)

Source: `crates/syx-driver/tests/fixtures/lsp/result_generic.sy`

## throw Expressions

```syx
func divide(_ a: Int, _ b: Int) throws -> Int {
    if (b == 0) {
        throw "division by zero"
    }
    return a / b
}
```

- `throw expr` throws `expr` (MVP type `String`)
- Legal only inside a `throws` function
- Throwing a `String` / `Result.err` / a custom enum is all allowed (the type of `expr` is determined by the function signature's `throws T`)

## throws Function Declarations

```syx
func loadFile(path: String) throws -> String {
    if (!File.exists(path)) {
        throw "not found: \(path)"
    }
    return File.read(path)
}

func parseInt(s: String) throws -> Int {
    // Assuming Int.fromString exists and returns a Result
    let r = Int.fromString(s)
    switch (r) {
    case .ok(let n) { return n }
    case .err(let e) { throw e }
    }
}
```

| Signature | Meaning |
|------|------|
| `throws -> T` | May throw; callers need `try` |
| `throws E -> T` | Typed throws (**Partially implemented**), throws `E` |
| `async throws -> T` | async and throws |

## try Expressions

Three forms for calling `throws` functions:

### `try expr` (try is mandatory)

```syx
let s = try loadFile(path: "/tmp/x")      // String
```

- It must be wrapped in `try` (the compiler reports missing wrapping as P001)
- Equivalent to `try { let s = loadFile(...) }`

### `try? expr` (failure becomes nil)

```syx
let s = try? loadFile(path: "/tmp/x")     // String?
```

- Success → returns `T`
- Failure → returns `nil` (the exception is swallowed)

### `try! expr` (forced non-erroring)

```syx
let s = try! loadFile(path: "/tmp/x")     // String
```

- Success → returns `T`
- Failure → runtime panic

> **`try?` chains:** several `try?`s may be chained: `try? foo() + try? bar()`, and any failure makes the whole thing `nil` (depending on the exact semantics).

## Chained catch

```syx
try {
    let r = Http.get(url)
    print(r)
} catch (err) where (err.hasPrefix("5")) {
    print("server error: \(err)")
} catch (err) where (err.hasPrefix("4")) {
    print("client error: \(err)")
} catch (err) {
    print("other: \(err)")
}
```

> **`where` guards:** early versions supported `where` guards on try-catch blocks (**Partially implemented**); calling `try?` / switching on a Result directly is recommended.

## The Result Type

`Result<T, E>` is the standard solution for explicit error types ([Chapter 9, Enumerations](09-enumerations.md)):

```syx
func parse(s: String) -> Result<Int, String> {
    if (s.isEmpty) { return .err("empty") }
    return .ok(s.count)
}

let r = parse(s: "hello")
switch (r) {
case .ok(let n) {
    print("len=\(n)")
}
case .err(let msg) {
    print("err: \(msg)")
}
}
```

### Static Result Construction

`ok(value)` / `err(value)` are pre-declared convenience functions:

```syx
let r1 = ok(42)                            // Result<Int, Never>
let r2 = err("oops")                       // Result<Never, String>
```

### Result Chaining

```syx
let r: Result<Int, String> = ok(10)
    .map({ x in x * 2 })                   // ok(20)
    .flatMap({ x in x > 0 ? ok(x) : err("neg") })
```

> **Methods:** `.ok(v)` / `.err(v)` / `.isOk()` / `.map(f)` / `.flatMap(f)` / `.description` are all defined in `packages/core/builtins/result.sy`.

### @Codable + Result Serialization

```syx
@Codable
enum ApiError: Error {
    case notFound
    case serverError(Int, String)
}
```

Source: `fixtures/lsp/result_generic.sy` · `fixtures/lsp/result_generic.sy`

## throws and Actors

A `throws func` inside an `actor` is isolated automatically:

```syx
actor FileManager {
    func read(path: String) throws -> String {
        if (!File.exists(path)) { throw "not found" }
        return File.read(path)
    }
}

let m = FileManager()
try {
    let s = await m.read(path: "/tmp/x")
    print(s)
} catch (err) {
    print("failed: \(err)")
}
```

## Choosing an Error-Handling Strategy

| Scenario | Choose |
|------|-----|
| Runtime errors that must be handled (IO, network) | `throws` + `try / catch` |
| Recoverable logic errors (parsing, conversion) | `Result<T, E>` |
| A two-state "present/absent" (lookup, cache hit) | `Optional<T>` |
| Internal panic paths in libraries | `try!` / `fatalError` |
| Error types carrying multiple fields | enum + `Result` / typed throws |

## Complete Example

```syx
enum MathError {
    case divideByZero
    case overflow
}

func safeDivide(_ a: Int, _ b: Int) -> Result<Int, MathError> {
    if (b == 0) { return .err(.divideByZero) }
    if (a > 1000) { return .err(.overflow) }
    return .ok(a / b)
}

func doubleAndPrint(_ a: Int, _ b: Int) {
    switch (safeDivide(a, b)) {
    case .ok(let n) {
        print("\(a) / \(b) = \(n * 2)")
    }
    case .err(.divideByZero) {
        print("oops: divide by zero")
    }
    case .err(.overflow) {
        print("oops: overflow")
    }
    }
}

func main() {
    doubleAndPrint(10, 2)                  // "10 / 2 = 10"
    doubleAndPrint(10, 0)                  // "oops: divide by zero"
    doubleAndPrint(2000, 5)                // "oops: overflow"
}
```

## Comparison with the Swift Style

| Swift | SYX |
|-------|-----|
| `do { try … } catch { }` | `try { } catch { }` (no `do`) |
| The `Error` protocol | The MVP uses `String` / `Result` |
| `throws E` typed throws | **Partially implemented** |
| `try?` `try!` | **Implemented** |
| `rethrows` | **Not implemented** |
| stdlib `Result<T, E>` | **Implemented** (including static `ok` / `err`) |
| `error: { }` closure parameter | A `catch (err) { }` block |
| `try? foo()?.bar()` | Legal |

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S020** | `throw` inside a non-`throws` function | Add `throws` |
| **P001** | Calling a `throws` function without `try` | Add `try` / `try?` / `try!` |
| **R007** | `try!` failed | Change to `try?` / `try { } catch` |
| **S004** | `throws` type mismatch | Fix `throws T` |

---

**Next chapter** → [Concurrency](18-concurrency.md)

**See also** → [Language Reference: Statements](../language-reference/04-statements.md) · [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 9, Enumerations](09-enumerations.md) · [Chapter 18, Concurrency](18-concurrency.md)
