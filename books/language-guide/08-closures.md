# Closures

**Implementation status:** closure expressions / capture / HOF **Implemented** (three-back-end parity, tests `fixtures/parity/closures_capture.sy` / `fixtures/parity/hof.sy`); `@escaping` / `@autoclosure` / the closure shorthand `$0` **Partially implemented**; trailing closure syntax **supported**

A closure is a **self-contained callable block of code** that can capture surrounding variables. SYX closures follow the Swift style: `{ params in body }`.

## Basic Syntax

```syx
let inc = { x in x + 1 }
print(inc(41))                          // 42

let sum = { (a, b) in a + b }
print(sum(1, 2))                        // 3
```

| Form | Description |
|------|------|
| `{ x in x + 1 }` | Single parameter; `x` is the parameter name |
| `{ (a, b) in a + b }` | Multiple parameters |
| `{ print("hi") }` | No parameters; `() in` may be omitted |
| `{ x, y in … }` | No type annotations (inferred) |
| `{ (x: Int) -> Int in x + 1 }` | Explicit types |

## Type Annotations

When a closure serves as a variable / parameter / return value, its type can be annotated explicitly:

```syx
let f: (Int) -> Int = { x in x * 2 }
let g: (Int, Int) -> Int = { (a, b) in a + b }
let h: () -> Void = { print("hi") }
```

> **Type inference:** when passed as an argument, parameter types are inferred from the function signature; if inference fails, annotate the function signature.

## Higher-Order Functions (HOF)

Closures are most often used in HOFs (`map` / `filter` / `reduce`):

```syx
let nums = [1, 2, 3, 4, 5]

let doubled = nums.map({ x in x * 2 })               // [2, 4, 6, 8, 10]
let evens   = nums.filter({ x in x % 2 == 0 })       // [2, 4]
let sum     = nums.reduce(0, { acc, x in acc + x })  // 15
let flat    = nums.flatMap({ x in [x, -x] })         // [1, -1, 2, -2, ...]
```

HOFs go through the `__syx_array_map` / `__syx_array_filter` / `__syx_array_reduce` Host functions ([Chapter 32, Operators vs Core Host](32-operators-vs-host.md)); closure parameters correspond to the elements of `Array<Element>`.

> **Three-back-end parity:** `map/filter/reduce` behave identically on the VM / unified MIR / WASM back ends (test `fixtures/parity/hof.sy`).

## Trailing Closures

The last closure in a parameter list may move outside the `()`:

```syx,fragment
let nums = [1, 2, 3, 4, 5]
let evens = nums.filter { x in x % 2 == 0 }           // Equivalent to nums.filter({ x in x % 2 == 0 })
let dbl   = nums.map { x in x * 2 }                   // Trailing
```

If the closure is the **only** parameter, the `()` may also be omitted:

```syx,fragment
let nums = [1, 2, 3, 4, 5]
let dbl = nums.map { x in x * 2 }                     // Trailing
```

> **Multiple closures:** with several closure parameters, **only the last** may trail; the others still go inside `()`:
>
> ```syx
> arr.reduce(0, combine: { acc, x in acc + x })
> //                           ^ still labeled
> ```

## Capture

A closure can capture outer `let` / `var` bindings:

```syx
func makeCounter() -> () -> Int {
    var n = 0
    return { n = n + 1; n }
}

let c = makeCounter()
print(c())         // 1
print(c())         // 2
print(c())         // 3
```

| Capture | Behavior |
|------|------|
| `let` capture | By value (read-only inside the closure) |
| `var` capture | By reference (the closure may modify it) |
| Returned across functions | The closure retains the captured object (avoiding premature release) |

```syx
let name = "SYX"
let greet = { print("Hello, \(name)") }
greet()                        // "Hello, SYX"
```

> **Lifetime:** a closure holds its captured objects until it is itself released. Repeatedly calling the same closure modifying a captured `var` affects later calls (consistent with Swift).

## Closures as Parameters

```syx
func twice(f: () -> Int) -> Int {
    return f() + f()
}

let r = twice(f: { 21 })
print(r)                                    // 42
```

## Closures as Return Values

```syx
func makeAdder(n: Int) -> (Int) -> Int {
    return { x in x + n }
}

let add5 = makeAdder(n: 5)
print(add5(3))                              // 8
```

## Closures and Async

```syx
Task {
    print("in task")
}

Task.detached {
    print("detached")
}
```

`Task { … }` and `Task.detached { … }` accept closures; see [Chapter 18, Concurrency](18-concurrency.md).

## Closures and Channels

```syx
let ch = Channel<Int>()
ch.send { v in print("got: \(v)") }         // Receive closure (semantics pending; Phase 132 behavior)
```

See [Chapter 18, Concurrency](18-concurrency.md).

## Closures in @resultBuilder

```syx
@resultBuilder
struct ArrayBuilder {
    static func buildBlock<T>(_ items: T...) -> Array<T> { return items }
}

@ArrayBuilder
func makeList() -> Array<Int> {
    1
    2
    3
}
let arr = makeList()                        // [1, 2, 3]
```

See [Chapter 19, Macros](19-macros.md).

## Closure Shorthand `$0` / `$1`

```syx,fragment
let nums = [1, 2, 3, 4, 5]
let doubled = nums.map { $0 * 2 }          // Equivalent to { x in x * 2 }
let sum     = nums.reduce(0) { $0 + $1 }   // acc + element
```

| Shorthand | Meaning |
|------|------|
| `$0` | The 0th parameter |
| `$1` | The 1st parameter |
| `$N` | The Nth parameter |

> **Status:** **Partially implemented**. The explicit `{ x in … }` form is recommended for readability.

## Closures as Storage

```syx
class Handler {
    var callback: (Int) -> Void = { _ in }
}

let h = Handler()
h.callback = { x in print("got \(x)") }
h.callback(42)                              // "got 42"
```

A closure stored in a class property holds a `self` reference; be careful to avoid reference cycles (support for a `[weak self]` capture list is **planned**).

## Closures and Lifetimes

```syx
func callLater(f: () -> String) {
    let s = f()
    print(s)
}

callLater(f: { return "later" })
```

A closure may be passed across functions as a parameter; captured objects are not collected before the closure is released.

## Performance

- A closure is a heap-allocated `Closure` object (similar to Swift)
- Simple closures may be inlined (single-call scenarios such as `makeAdder`, inlining since Phase 30+)
- Closure allocation inside loops is a known hot spot; you can replace a `for-in` closure with `arr.forEach`

## Complete Example

```syx
func makeCounter(start: Int) -> () -> Int {
    var n = start
    return { n = n + 1; n }
}

func main() {
    // HOF
    let nums = [1, 2, 3, 4, 5]
    let stats = (
        sum: nums.reduce(0) { $0 + $1 },                    // 15
        avg: nums.reduce(0) { $0 + $1 } / nums.count,      // 3
        max: nums.reduce(Int.min) { $0 > $1 ? $0 : $1 }      // 5
    )
    print("sum=\(stats.sum) avg=\(stats.avg) max=\(stats.max)")

    // Capture
    let c = makeCounter(start: 10)
    print(c())      // 11
    print(c())      // 12

    // Trailing
    let evenSum = nums.filter { $0 % 2 == 0 }.reduce(0) { $0 + $1 }
    print(evenSum)  // 6
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Closure parameter/return type mismatch | Fix the type annotation |
| **S010** | Closure passed to an HOF whose protocol constraints it does not satisfy | Check protocol conformance |
| **P001** | Closure syntax error | Check the `{ params in body }` structure |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `$0` / `$1` closure shorthand | **Partially implemented** |
| Trailing closures | **Supported** |
| `@escaping` | **Not implemented** (closures escape by default) |
| `@autoclosure` | **Not implemented** |
| Capture lists `[weak self]` / `[unowned self]` | **Not implemented** (planned) |
| Multiple trailing closures `arr.sorted(by:) { $0 < $1 }` | Legal |
| Omitting `()` for a single parameter | Legal |

---

**Next chapter** → [Enumerations](09-enumerations.md)

**See also** → [Language Reference: Expressions](../language-reference/03-expressions.md) · [Chapter 7, Functions](07-functions.md) · [Chapter 24, Generics](24-generics.md) · [Chapter 19, Macros](19-macros.md)
