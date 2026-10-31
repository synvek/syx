# Error Handling

**Implementation status:** `try` / `catch` / `throw` / `try?` / `try!` / `Result<T, E>` **Implemented**; `throws T` typed throws **Partially implemented**; rethrows **Not implemented**

SYX provides three error-propagation paths:

1. **throws / try** — raising and catching exceptions (runtime errors)
2. **Result<T, E>** — an explicit algebraic type
3. **Optional** — using `nil` to mean failure (the lightest weight)

This chapter covers syntax, usage, Result integration, and cross-function propagation.

## try / catch Blocks

```syx
import IO

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
import IO

func loadFile(path: String) throws -> String {
    if (!File.exists(path)) {
        throw "not found: \(path)"
    }
    return File.read(path)
}

func parseInt(s: String) throws -> Int {
    if (s.count == 0) {
        throw "empty"
    }
    return 0
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
import IO

func loadFile(path: String) throws -> String {
    if (!File.exists(path)) { throw "not found" }
    return File.read(path)
}

func main() {
    let s = try loadFile(path: "/tmp/x")  // String
    print(s)
}
```

- It must be wrapped in `try` (the compiler reports missing wrapping as P001)
- Equivalent to `try { let s = loadFile(...) }`

### `try? expr` (failure becomes nil)

```syx
import IO

func loadFile(path: String) throws -> String {
    if (!File.exists(path)) { throw "not found" }
    return File.read(path)
}

func main() {
    let s = try? loadFile(path: "/tmp/x") // String?
    print(s)
}
```

- Success → returns `T`
- Failure → returns `nil` (the exception is swallowed)

### `try! expr` (forced non-erroring)

```syx
import IO

func loadFile(path: String) throws -> String {
    if (!File.exists(path)) { throw "not found" }
    return File.read(path)
}

func main() {
    let s = try! loadFile(path: "/tmp/x") // String
    print(s)
}
```

- Success → returns `T`
- Failure → runtime panic

> **`try?` chains:** several `try?`s may be chained: `try? foo() + try? bar()`, and any failure makes the whole thing `nil` (depending on the exact semantics).

## Chained catch

~~~text
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
~~~

> **`where` guards:** early versions supported `where` guards on try-catch blocks (**Partially implemented**); calling `try?` / switching on a Result directly is recommended.

## The Result Type

`Result<T, E>` is the standard solution for explicit error types ([Chapter 9, Enumerations](09-enumerations.md)):

```syx
func parse(s: String) -> Result<String, String> {
    if (s.isEmpty) { return Result.err(value: "empty") }
    return Result.ok(value: s)
}

func main() {
    let r = parse(s: "hello")
    if (Result.isOk(result: r)) {
        print("ok")
    } else {
        print("err")
    }
}
```

### Static Result Construction

`Result.ok(value:)` / `Result.err(value:)` are the String-typed MVP constructors (`packages/core/builtins/algebraic/result.sy`):

```syx
func main() {
    let r1 = Result.ok(value: "42")          // Result<String, String> success
    let r2 = Result.err(value: "oops")       // Result<String, String> failure
    print(Result.isOk(result: r1))
    print(Result.isErr(result: r2))
}
```

### Result Chaining

Thin MVP check surface (full `.map` / `.flatMap` generics still deepen later):

```syx
func main() {
    let r1 = Result.ok(value: "10")
    print(Result.isOk(result: r1))           // true
    let r2 = Result.err(value: "neg")
    print(Result.isErr(result: r2))          // true
}
```

> **Methods:** `Result.ok` / `Result.err` / `Result.isOk` / `Result.isErr` / `.description` live in `packages/core/builtins/algebraic/result.sy`. Generic `.map` / `.flatMap` remain a deepen track.

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
import IO

actor FileManager {
    func read(path: String) throws -> String {
        if (!File.exists(path)) { throw "not found" }
        return File.read(path)
    }
}

async func main() {
    let m = FileManager()
    try {
        let s = await m.read(path: "/tmp/x")
        print(s)
    } catch (err) {
        print("failed: \(err)")
    }
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

~~~text
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
~~~

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
