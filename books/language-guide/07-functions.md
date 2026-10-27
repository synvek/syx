# Functions

**Implementation status:** `func` declarations / named parameters / default parameters / `async` / `throws` / `requires Pure` / `requires IO` are **all Implemented**; `@escaping` / `@autoclosure` / generic type-erasure reflection are **Not implemented**

Functions are the basic building blocks of SYX programs. This chapter covers declaration syntax, parameters, returns, effect annotations (`requires`), async/await, throws, and cross-module visibility.

## Definition and Call

```syx
func greet(name: String) {
    print("Hello, \(name)")
}

func add(a: Int, b: Int) -> Int {
    return a + b
}

func main() {
    greet(name: "SYX")               // "Hello, SYX"
    print(add(a: 1, b: 2))           // 3
}
```

> **Named parameters:** when calling a labeled parameter you **must** write `label: value`. A formal parameter may use `_ name` to drop the external label (then calls can only pass it positionally); it may use `label name` to give an external label different from the internal name (then calls must write `label:` and cannot write the internal name).

```syx
func greet(name: String) { print("Hello, \(name)") }
func move(to destination: String) { }
func join(_ task: String) -> String { return task }

greet(name: "SYX")           // ✅ The label name: is required
move(to: "home")             // ✅ External label to:, internal name destination
join("worker")               // ✅ _ drops the label → positional argument
join(task: "worker")         // ❌ S042: a `_` parameter cannot carry a label
```

| Formal parameter form | External label strategy | Call form |
|----------|--------------|----------|
| `name: Type` | Same as the internal name `name` | `name: value` (required) |
| `label name: Type` | External name `label` | `label: value` (required) |
| `_ name: Type` | Omitted | Only positionally |

| Element | Description |
|------|------|
| `func` | Keyword |
| `name` | Function name (`lowerCamelCase`) |
| `(param: Type, ...)` | Formal parameter list |
| `-> ReturnType` | Return type (defaults to `Void`) |
| `block` | Function body |

## Multiple Return Values via Tuples

SYX tuples can serve as "multiple return values":

```syx
func divmod(a: Int, b: Int) -> (Int, Int) {
    return (a / b, a % b)
}

let (q, r) = divmod(a: 17, b: 5)
print("\(q) rem \(r)")           // 3 rem 2
```

Tuple destructuring `(q, r) = …` is legal in `let` / `var` bindings (MVP GA; `fixtures/lang/tuple.sy`). Named elements `(x: 1, y: 2)` and members `.x` / `.0` are usable; this is not the complete Swift tuple protocol.

## Default Parameters

```syx
func greet(name: String, greeting: String = "Hello") {
    print("\(greeting), \(name)")
}

greet(name: "SYX")                       // "Hello, SYX"
greet(name: "SYX", greeting: "Hi")       // "Hi, SYX"
```

> **Limit:** a default value must be a compile-time constant or a bare literal; it cannot reference runtime variables or call functions. Parameters with defaults may be placed anywhere (there is no Swift-style "put default parameters last" requirement).

## Access Control

```syx
public func exported() { }               // Importable across modules
internal func moduleOnly() { }           // Within the module (default)
private func hidden() { }                 // Within the file / scope
```

See [Chapter 28, Access Control](28-access-control.md).

```syx
import math                               // Cross-module
math.add(a: 1, b: 2)                      // ✅ public
// math.internal_helper()                // ❌ S005: accessing internal across modules
```

## async Functions

```syx
async func fetch() -> String {
    return await httpGet("https://example.com")
}
```

The `async` keyword marks a function as a **coroutine**; inside its body you may use `await`:

```syx
async func fetchTwo() -> (String, String) {
    let a = await fetchA()
    let b = await fetchB()
    return (a, b)
}
```

- `await` is legal only inside an `async` function body
- Using `await` in a non-`async` function → compile error **S020**
- `async func main()` is a legal program entry point

> **Implementation:** multiple-`await` state machines (**Implemented**, test `runtime/async_native.rs`). Underneath this is a generator-based state machine; see [Chapter 18, Concurrency](18-concurrency.md).

## async let

Start multiple subtasks in parallel and wait for all of them:

```syx
async func parallel() -> Int {
    async let a = fetchA()              // Start a subtask
    async let b = fetchB()              // Start a subtask
    let x = await a                     // Wait for a
    let y = await b                     // Wait for b
    return x + y
}
```

> **Semantics:** `async let` desugars into detach + join (behavior frozen in SYX 132). `a` is an `AsyncValue<T>`, and `await a` blocks the current task until the subtask completes.

## throws Functions

```syx
func loadFile(path: String) throws -> String {
    if (!File.exists(path)) {
        throw "not found: \(path)"
    }
    return File.read(path)
}
```

| Clause | Meaning |
|------|------|
| `throws` | The function may throw (the exception type is not specified; the MVP uses `String` or `Result`) |
| `throws T` | Throws type `T` (typed throws **Partially implemented**) |
| `throw expr` | Throws inside a `throws` function |
| `try expr` | Calls a `throws` expression |
| `try { } catch { }` | Block-level catching |
| `try?` | Returns `nil` on error or propagates |
| `try!` | Forced (panics on error) |

Calling:

```syx
let s = try loadFile(path: "/tmp/x")            // Expression-level try
let s2 = try? loadFile(path: "/tmp/x")          // String? (nil means error)
let s3 = try! loadFile(path: "/tmp/x")          // String (panics on error)

try {
    let s = loadFile(path: "/tmp/x")
    print(s)
} catch (err) {
    print("failed: \(err)")
}
```

See [Chapter 17, Error Handling](17-error-handling.md).

## Effect Annotations (`requires`)

Mark a function as "pure" (no side effects) or "IO":

```syx
func pure(x: Int) -> Int requires Pure {
    return x + 1
}

func load() -> String requires IO {
    return File.read("data.txt")
}
```

| Annotation | Meaning | Compile-time checking |
|------|------|------------|
| `requires Pure` | No I/O / global state dependency | **Phase 50+** checks call chains (`syx-meta`) |
| `requires IO` | I/O allowed | Marker only |
| No annotation | Default (equivalent to IO) | No checking |

> **MVP status:** `syx-meta` is partially implemented (mentioned in [Chapter 32, Operators vs Core Host](32-operators-vs-host.md)). The complete effect system → later Phases. **Application-level code** can omit these; service layers / SDKs may use them.

## Generic Functions

```syx
func id<T>(x: T) -> T { return x }
func swap<T>(a: T, b: T) -> (T, T) { return (b, a) }

let n = id(x: 42)            // Int
let s = id(x: "hi")          // String
```

Generic parameters come after the function name; a `where` constraint may be added:

```syx
func eq<T>(a: T, b: T) -> Bool where T: Equatable {
    return a == b
}
```

See [Chapter 24, Generics](24-generics.md) and [Language Reference: Generic Parameters and Arguments](../language-reference/08-generic-parameters-and-arguments.md).

## Operator Functions

Custom operators (**GA**):

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

let r = 5 |> { $0 * 2 }      // 10
```

See [Chapter 30, Advanced Operators](30-advanced-operators.md).

## Functions as Values (First Class)

```syx
let f: (Int) -> Int = { x in x * 2 }
let r = f(3)                                  // 6

func apply(g: (Int) -> Int, x: Int) -> Int {
    return g(x)
}
print(apply(g: f, x: 4))                      // 8
```

The function type `(P) -> R` is first class: it may serve as a variable, parameter, return value, or `Array` element.

> **`$0` / `$1`:** closure shorthand (`{ $0 + $1 }`), corresponding to the first / second parameters; consistent with Swift's `$0`. **Partially implemented**.

## Nested Functions

```syx
func outer() -> Int {
    func inner(x: Int) -> Int { return x * 2 }
    return inner(x: 21)                       // 42
}
```

Inner functions may capture outer `let`/`var` bindings (see [Chapter 8, Closures](08-closures.md)).

## Recursion

```syx
func fib(n: Int) -> Int {
    if (n < 2) { return n }
    return fib(n: n - 1) + fib(n: n - 2)
}
```

SYX recursion uses the VM call stack; deep recursion triggers **R014 stack overflow**.

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S001** | Duplicate function name | Rename the function |
| **S004** | Parameter/return type mismatch | Fix the types |
| **S005** | Accessing an internal symbol across modules | Change it to `public` |
| **S042** | Call argument label missing, wrong, or a `_` parameter carrying a label | Supply the correct `label:` or change the parameter to `_` |
| **S020** | Using `await` in a non-`async` function | Add `async` or remove `await` |
| **R007** | `!` unwrapping `nil` | Change to `?` or `if let` |
| **R014** | Recursion too deep | Switch to iteration or increase the stack size |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `_` to drop argument labels | **Implemented** (`_ name`); fully positional calls work only when **all** formal parameters are `_` |
| Enforced label matching | **Implemented** (S042); there is no "fully positional escape" |
| Trailing default parameters may be omitted | In SYX they may be omitted in any position |
| `@escaping` | **Not implemented** |
| `@autoclosure` | **Not implemented** |
| `rethrows` | **Not implemented** |
| `inout` | **Not implemented**; borrow with `&mut T` (partially implemented) |
| `async throws` | **Implemented** |
| `async let` semantics | Desugars to detach+join (Swift uses structured concurrency) |
| Closure shorthand `$0`/`$1` | **Partially implemented** |
| Generic `where` clauses | **Implemented** |

## Implementation Status

- Named parameters: **Implemented**
- Default parameters: **Implemented**
- `async` / `await` / `async let`: **Implemented** (multiple-await state machine GA)
- `throws` / `try` / `try?` / `try!`: **Implemented**
- `requires Pure` / `requires IO`: **Partially implemented** (`syx-meta` MVP)
- Operator functions: **GA** (three-back-end parity)
- Nested functions: **Implemented**
- Closure shorthand `$0`: **Partially implemented**

---

**Next chapter** → [Closures](08-closures.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 17, Error Handling](17-error-handling.md) · [Chapter 18, Concurrency](18-concurrency.md) · [Chapter 30, Advanced Operators](30-advanced-operators.md)
