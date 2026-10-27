# SYX Language Quick Reference

[简体中文](./zh/8_QuickReference.zh-CN.md)

Syntax and standard-library cheat sheet for **`.sy` source authors**. Aligned with current `syx-parser` / `syx-semantic` capabilities; toolchain usage in [9_UsageGuide.md](./9_UsageGuide.md).

> **Version:** SYX **117.0** · Phase **139** · `syx-cache-v1` · License [Apache-2.0](../LICENSE)  
> **Native AOT:** LLVM only · **Host globals:** [`packages/core/builtins/special/globals.sy`](../packages/core/builtins/special/globals.sy) (`print` / `readLine` / `reflect`)  
> **Implementation SSOT:** [books/shared/implementation-status.md](../books/shared/implementation-status.md)  
> Legacy Phase banners removed; details follow status / ADRs.

---

## Table of contents

1. [Type system](#type-system)
2. [Variables and entry point](#variables-and-entry-point)
3. [Control flow](#control-flow)
4. [Functions and access control](#functions-and-access-control)
5. [Modules](#modules)
6. [Strings](#strings)
7. [Arrays](#arrays)
8. [Error handling try/catch](#error-handling-trycatch)
9. [Optional null safety](#optional-null-safety)
10. [Data class](#data-class)
11. [Extension](#extension)
12. [Sealed class and switch](#sealed-class-and-switch)
13. [Effects annotations](#effects-annotations)
14. [Standard library File / Directory / HTTP](#standard-library-file--directory--http)
15. [Async / await (MVP)](#async--await-mvp)
16. [Borrowing and macros (Phase 7)](#borrowing-and-macros-phase-7)
17. [CLI cheat sheet](#cli-cheat-sheet)
18. [Diagnostic codes](#diagnostic-codes)

---

## Type system

### Numeric types

| Type | Examples | Notes |
|------|------|------|
| `Int8` … `Int64` | `42i8`, `1000i64` | Fixed-width signed integers |
| `Int128` | `42i128` | 128-bit signed integer (Phase 38) |
| `UInt8` … `UInt64` | `255u8` | Fixed-width unsigned integers |
| `UInt128` | `0xFFu128` | 128-bit unsigned integer (Phase 38) |
| `SIMD4<Int32>`, etc. | `SIMD4.fromArray([1,2,3,4])` | Fixed-width SIMD vectors (Phase 38) |
| **`Int`** | `42` | **Platform word** signed integer (64-bit host → `Int64`, 32-bit → `Int32`) |
| **`UInt`** | `100u` | **Platform word** unsigned integer |
| `Float32` / `Float` | `3.14f32` | 32-bit float |
| `Float64` / `Double` | `3.14` | 64-bit float (unsuffixed float literals default to `Double`) |

Unsuffixed integer literals default to platform **`Int`**. Explicit suffixes (e.g. `42i8`) pick fixed-width types.

### Basic and collection types

| Type | Examples | Notes |
|------|------|------|
| `Bool` | `true` / `false` | Boolean |
| `String` | `"hello"` | String |
| `Array<T>` | `[1, 2, 3]` | Dynamic array |
| `Set<T>` | `Set<Int>`, assign `[1,2,3]` to `Set` | Deduped set |
| `[K: V]` / `Dictionary<K,V>` | `["a": 1]`, `[String: Int]` | Static dictionary (MVP keys must be `String`) |
| `Object` | `let o: Object = { "k": v }` | Dynamic heterogeneous bag (annotation required; unannotated dict → `Dictionary`) |
| `T?` | `nil` / `42` | Optional type |
| `Void` | — | No return value |

**Type annotations:** `let x: Int = 10`, `let m: [String: Int] = ["a": 1]`, `let s: Set<Int> = [1, 2, 3]`

> **Note:** SYX uses the `func` keyword and built-in `print()`; it is **not** a fully Swift-compatible `func`/`print` subset. `class` / `struct` / `protocol` parse, but runtime support is limited; prefer `data class` + `extension` for day-to-day code.

---

## Variables and entry point

```syx
var count = 0      // mutable
let name = "SYX"   // immutable
var x: Int = 10    // explicit type

func main() {      // program entry
    print("Hello")
}
```

---

## Control flow

```syx
// if — condition in parentheses
if (x > 0) {
    print("positive")
} else {
    print("non-positive")
}

// while — condition in parentheses
while (x > 0) {
    x = x - 1
}

// for-in — Array / String / readLines results
for item in [1, 2, 3] {
    print(item)
}
for ch in "abc" {
    print(ch)
}

// switch — colon after case; ranges and variants supported
switch value {
case 1:
    print("one")
case 0..<10:
    print("small")
default:
    print("other")
}
```

| Construct | Syntax |
|------|----------|
| `if` / `while` | Condition must be `(expr)` |
| `for-in` | `for name in iterable { }`, no parentheses |
| `switch` | `case pat:` or `default:`; whole construct in `{ }` |
| `break` / `continue` | Supported |

---

## Functions and access control

```syx
func add(a: Int, b: Int) -> Int {
    return a + b
}

public func exported() -> Int {
    return 1
}

internal func helper() -> Int {
    return 0
}

private func hidden() -> Int {
    return -1
}
```

| Modifier | Visibility |
|--------|--------|
| `public` | Importable across modules |
| `internal` | Within module (default) |
| `private` | File/module private |

**Parameters:** `func f(a: Int, b: Int)` — call `f(a: 1, b: 2)`. No Swift-style `_` label elision.

---

## Modules

```syx
// Math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

// main.sy
import Math

func main() {
    print(Math.add(1, 2))
}
```

| Rule | Notes |
|------|------|
| Import | `import moduleName` (name maps to `.sy` file or `syx.toml` dependency key) |
| Export | Only `public func` callable from other modules |
| Cross-module `internal` | Compile error **S005** |
| Project deps | `[dependencies]` in `syx.toml` + `syx add` / `syx update` |

---

## Strings

```syx
let name = "World"
print("Hello, \(name)!")                    // interpolation

"a,b,c".split(",")                          // → Array
"  hi  ".trim()
"ABC".lowercased() / "abc".uppercased()
"file.txt".hasPrefix("file")
"hello".substring(0, 3)                     // [start, end)
"foo".replacingOccurrences("bar", "foo")    // (new, old)
"abc123".matches("\\d+")                    // regex → Array
"text".count                               // character count
```

| Method | Returns |
|------|------|
| `.split(sep)` | `Array` |
| `.matches(pattern)` | `Array` (match strings) |
| `.replacingOccurrences(with, of)` | `String` |

---

## Arrays

```syx
let nums = [10, 20, 30]
print(nums.count)

var sum = 0
for x in nums {
    sum = sum + x
}
```

| Method | Notes |
|------|------|
| `.count` | `Int` |
| `.contains(item)` | `Bool` |
| `.sorted()` | `Array` (ascending) |
| `.reverse()` | `Array` |
| `.joined(sep)` | `String` |
| `.map` / `.filter` / `.reduce` | ✅ Closures supported (see `p4_hof.sy`) |

### Set and Dictionary

```syx
let scores: [String: Int] = { "a": 1, "b": 2 }
print(scores.count)          // 2

var tags: Set<Int> = [1, 2, 2, 3]
print(tags.count)            // 3 (deduped)
print(tags.insert(4))        // true (ADR-224 write-back)
print(tags.contains(4))      // true
print(scores.containsKey("a")) // true
```

| Type | Methods | Returns |
|------|------|------|
| `Set` | `.count` / `.contains` / `.insert` / `.remove` (mutations ADR-224); `.union` / `.intersection` | Bool / Int |
| `[String: V]` | `.count` / `.containsKey` / `.remove` / `.keys` / `.values` | See stdlib |

### Ranges and readLine

```syx
print((1..<10).contains(5))
let line = readLine()
```

---

## Error handling try/catch

SYX uses **`try { } catch (name) { }`**, not Swift `do-catch` / `throws`.

```syx
// statement block
try {
    File.write("/tmp/x.txt", "data")
    let content = File.read("/tmp/x.txt")
    print(content)
} catch (err) {
    print("failed")
}

// expression-level try (failure → runtime error path)
let text = try File.read(path)
```

Host functions (File, HTTP, etc.) raise VM exceptions on failure; `catch` handles them.

---

## Optional null safety

```syx
let a: Int? = nil
print(a ?? 99)              // nil coalescing

let b: Int? = 7
print(b!)                   // force unwrap; nil → R007

let s: String? = "hello"
print(s?.hasPrefix("he"))   // optional chain; nil short-circuits

if let y = b {               // bind when non-nil
    print(y)
}

guard let z = b else {       // failure → else branch
    return
}
print(z)
```

| Operator | Meaning |
|--------|------|
| `T?` | Optional type |
| `nil` | Null value |
| `??` | Left if non-nil, else right |
| `!` | Force unwrap |
| `?.` | Optional member chain |

---

## Protocols and equality (Equatable / Hashable)

Built-in types automatically implement `Equatable`, `Hashable`, `CustomStringConvertible`, and `CustomDebugStringConvertible` (numeric types, Bool, String, Array, Set, Dictionary, Object).

| Operator / member | Meaning |
|---------------|------|
| `==` / `!=` | Structural equality (numeric promotion; deep collection equality) |
| `===` / `!==` | Identity equality (class instances compare `__object_id`; value types same as `==`) |
| `.description` | User-facing string |
| `.debugDescription` | Debug string (with type prefix) |
| `hash(into:)` | Built-in witness via `Hasher` host |

| Type | Notes |
|------|------|
| `Hasher` | `Hasher.newHasher()` / `Hasher.combine` / `Hasher.finalize` |
| `ObjectIdentifier` | `Hasher.objectId(obj)` |

Using `==` on types that are not `Equatable` → **S016**.

---

## Data Class

```syx
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    print(p.x)
    print(p.description())   // "Point(x: 3, y: 4)"
}
```

| Capability | Status |
|------|------|
| Construct `Type(a, b)` | ✅ |
| Field access `p.x` | ✅ |
| `.description()` | ✅ |
| Auto `copy` / `==` / `hash(into:)` | ✅ Compiler-synthesized (`{Type}_copy` / structural eq / `hash(into:)`) |

---

## Extension

```syx
extension Int {
    func doubled() -> Int {
        return self + self
    }
}

func main() {
    print(5.doubled())   // desugar → Int_doubled(5)
}
```

---

## Sealed class and switch

```syx
sealed class Result {
    ok(value: Int)
    err(message: String)
}

func describe(r: Result) {
    switch r {
    case .ok(let x):
        print(x)
    case .err(let msg):
        print(msg)
    }
}
```

Variant definitions **omit** the `case` keyword; `switch` arms use `.variant(let binding)` forms (see [2_SyntaxAnalysis.md](2_SyntaxAnalysis.md) §5.5).

| Pattern | Example |
|------|------|
| Integer | `case 42:` |
| Half-open range | `case 0..<13:` |
| Variant | `case .ok(let x):` |

---

## Extended syntax (§11 implemented)

```syx
enum Result {
    ok(value: Int)
    err(msg: String)
}

class Greeter {
    var name: String
    init(name: String) { self.name = name }
    func hello() { print(self.name) }
}

class Counter: IteratorProto {
    func next() -> Int { return 1 }
}

actor Counter {
    var value: Int
    init(value: Int) { self.value = value }
    func increment() { self.value = self.value + 1 }
}

func id<T>(x: Int) -> Int { return x }  // monomorphized to id_Int

@available(iOS 13)
@Main
func app() { print("hi") }

let evens = [1,2,3].filter({ |x| x % 2 == 0 })
let sum = [1,2,3].reduce({ |a, b| a + b })

let base = 10
let mapped = [1].map({ |x| x + base })  // closure capture

throw "failed"
repeat { print(1) } while (false)
x += 1
```

See [2_SyntaxAnalysis.md](2_SyntaxAnalysis.md) §5 / §11.

---

## Effects annotations

Function declarations may append **`requires Pure`** or **`requires IO`** after the return type; Pure functions must not perform I/O (including `print`, File/HTTP).

```syx
func compute(x: Int) -> Int requires Pure {
    return x * 2
}

func loadConfig() -> String requires IO {
    return File.read("config.txt")
}
```

Violating Pure → compile-time error (`syx-meta`).

---

## Standard library File / Directory / HTTP

Invoked via **static type methods** and **instance methods** (Host Functions).

### File

| Method | Notes |
|------|------|
| `File.read(path)` | Read entire file → `String` |
| `File.readLines(path)` | By line → `Array` |
| `File.write(path, content)` | Write |
| `File.append(path, content)` | Append |
| `File.exists(path)` | → `Bool` |
| `File.delete(path)` | Delete |
| `File.copy(from, to)` | Copy |
| `File.move(from, to)` | Move |

### Directory

| Method | Notes |
|------|------|
| `Directory.create(path)` | Create directory |
| `Directory.list(path)` | List entries → `Array` |
| `Directory.exists(path)` | → `Bool` |

### HTTP

Client uses `reqwest` (blocking + rustls) with HTTPS. See [11_NetworkStdlib.md](11_NetworkStdlib.md).

| Method | Notes |
|------|------|
| `HTTP.fetch(url, options)` | Generic request (method / headers / body / query / timeout_ms) |
| `HTTP.get(url)` | GET → response object |
| `HTTP.post(url, body)` | POST; body `String` or dictionary |
| `HTTP.put/patch/delete(url, body?)` | Matching HTTP verbs |
| `HTTP.serve(addr, port, handler)` | Blocking HTTP server (handler function name) |
| `HTTP.response(status, body)` | Build server response object |

| Response method | Notes |
|---------------|------|
| `status()` | HTTP status code |
| `ok()` | 2xx → `true` |
| `text()` / `body()` | Response body string |
| `json()` | Parse JSON → object |
| `header(name)` | Single response header |
| `headers()` | All response headers |

### URL

| Method | Notes |
|------|------|
| `URL.parse(text)` | Parse → `{ href, protocol, host, port, path, query }` |
| `URL.encodeQuery(dict)` | Dictionary → query string |
| `URL.build(base, path, query)` | Build URL |

```syx
import HTTP

try {
    let resp = HTTP.fetch("https://httpbin.org/get", { "timeout_ms": 10000 })
    if resp.ok() {
        print(resp.json())
    }
} catch (e) {
    print("network error")
}
```

### Globals

| Name | Notes |
|------|------|
| `print(expr)` | Write to stdout |

> **Available:** `readLine()` reads one line from stdin.  
> **Not available:** `exit()`, `assert()`, and similar globals.

---

## Async / await (MVP)

```syx
async func fetch() -> String {
    let resp = await HTTP.get("https://example.com")
    return "ok"
}

// Structured Task (thread-pool spawn today)
let t = Task {
    work()
}
__syx_task_join(t)

// Explicit parallelism
Task.detached {
    __syx_task_sleep(50)
}

// Detach with args (Concurrency or host)
__syx_task_detach("worker", [42])
```

`syx.toml` strict mode:

```toml
[concurrency]
strict = true   # enable C001–C003 static checks
```

| Host / API | Notes |
|------------|------|
| `__syx_task_detach(name)` / `Concurrency.detach(name)` | Thread-pool detached task |
| `__syx_task_detach(name, args)` | Spawn with arguments |
| `__syx_task_join` / `cancel` / `sleep` | Join / cancel / sleep |
| `__syx_task_group_*` | TaskGroup create / add / wait (isolated by group id) |
| `__syx_channel_create/send/recv` | Phase 8 async channel MVP (VM host) |
| `__syx_channel_try_recv` | Phase 15 non-blocking recv (nil = empty) (`p15_channel_try_recv.sy`) |
| `__syx_channel_create(cap)` / `__syx_channel_try_send` | Bounded channel + backpressure (ADR-267 / `m3_a04_backpressure`; full buffer trySend→false) |
| `__syx_channel_select([...])` | Multi-arm select + RR fairness (ADR-268 / `m3_a04_fairness`; `select-fair-v1:round-robin`; leaf ADR-269) |
| `Task { }` / `Task.detached { }` | Syntax sugar (generates `__task_body_N`) |

| Limit | Notes |
|------|------|
| await state machine | Multi-await functions insert `__syx_async_state`; `await __syx_task_sleep` lowered inside async |
| `async let` | `async let x = f()` → detach + later await join (`p7_async_let.sy`) |
| Scope | `await` only inside `async func` |
| Lint | async function with no await → **L005** |

---

## Borrowing and macros (Phase 7)

### Borrowing

```syx
func bump(x: &mut Int) {
    x = x + 1
}

func read_only(x: &Int) -> Int {
    return x
}
```

| Rule | Diagnostic |
|------|------|
| At most one active `&mut T` per scope | **S011** |
| Use after move | **S011** (moved value) |

Fixtures: `p6_borrow_*`, `p7_borrow_mut.sy`, `p7_borrow_mut_fail.sy`.

### Macros

```syx
#if(DEBUG, print("debug on"))
#for(3, print("x"))
#assert(x > 0)
#macro twice(x) { print(x); print(x) }
let s = #stringify(1 + 1)
@Equatable
data class P(x: Int, y: Int)
```

Expanded by `syx-macro::expand_module`; depth exceeded → **E002**. See [books/language-guide/19-macros.md](../books/language-guide/19-macros.md).

---

## CLI cheat sheet

```bash
cargo build --release              # build syx
syx run file.sy                   # compile and run on VM (full stdlib)
syx run .                          # project (syx.toml)
syx check .                        # type-check only
syx build --native .               # MIR → app (LLVM sole; --object for .o only)
syx build --wasm .                 # MIR → main.wasm (experimental)
syx emit file.sy --format syxbc   # VM bytecode text → file.syxbc
syx emit . --format all -o emit/ # write six IR/bytecode artifacts to emit/
syx run file.sy                   # single MIR path (Phase 12 default)
syx format . / syx lint . / syx doc .
syx test .                         # run tests/**/*.sy
syx repl
```

---

## Diagnostic codes

### Lexer / parser

| Code | Meaning |
|------|------|
| L001 | Unclosed string / invalid character (lexer) |
| P001 | Unexpected token / syntax error |

### Semantic (compile time)

| Code | Meaning |
|------|------|
| S001 | Undefined type |
| S002 | Undefined identifier |
| S003 | Type mismatch |
| S004 | Duplicate declaration |
| S005 | Invalid access level (cross-module internal) |
| S006 | Protocol not satisfied (e.g. Iterator missing `next()`) |
| S007 | Non-exhaustive enum switch (by case name set) |
| S008 | Cross-actor direct instance field access |
| S009 | Unknown protocol constraint in where clause |
| S010 | Generic argument fails where protocol constraint |
| S011 | Borrow conflict / use-after-move |

### Concurrency (strict or `@concurrent` context)

| Code | Meaning |
|------|------|
| C001 | Call `@MainActor` function outside MainActor (`main` is implicit MainActor) |
| C002 | Cross-actor synchronous method call |
| C003 | Task.detached / detach captures non-Sendable type (strict or module has `class`) |
| C004 | Task cancelled |
| C005 | Task channel closed |
| C006 | Actor mailbox closed |
| C009 | Actor sync same-instance reentrancy rejected (serial-no-reenter) |
| C010 | Channel closed (send after close) |

### Runtime

| Code | Meaning |
|------|------|
| R001 | File read failure / registry semver conflict |
| R002 | File/directory write failure |
| R003 | HTTP request failure |
| R004 | HTTP POST failure |
| R005 | JSON parse failure |
| R006 | Invalid regex |
| R007 | Force unwrap nil |
| R011 | HTTP request timeout |
| R012 | HTTP serve bind failure |
| R013 | URL parse failure |

### Lint (`syx lint`)

| Code | Rule summary |
|------|----------|
| L001–L012 | Naming, empty bodies, public return types, async/await, line width, etc. |

---

## Main differences from Swift (cheat sheet)

| Swift | SYX |
|-------|-----|
| `do { try } catch` | `try { } catch (err) { }` |
| `if cond {` | `if (cond) {` |
| `struct` as data carrier | `data class Name(f: T)` |
| `import Module` | `import Math` (PascalCase embedded stdlib module names) |
| `print(_:separator:)` | `print(expr)` |
| `@main` | `func main()` |
| `throws` functions | Host errors + try/catch |

---

## Ecosystem and IDE (Phase 11 / SYX 4.5)

- **Single MIR path:** user modules always go through MIR (Phase 12; `SYX_USE_MIR` removed)
- **LSP v4:** code actions, semantic tokens, inlay hints
- **Embed:** `compile_and_run_embedded` (see ADR-003)

| Env var / path | Notes |
|-----------------|------|
| `SYX_REGISTRY` | Production registry default URL (`https://packages.sy.dev`); overrides `[registry].default` |
| `SYX_REGISTRY_STAGING` | Staging registry URL when no `[registry].default` |
| `SYX_PUBLISHER_SIGNING_KEY` | Publish: 32-byte Ed25519 seed (64 hex); `syx publish` → `syx1:ed25519:<pk>:<sig>` (C-05) |
| `SYX_REGISTRY_VERIFY_KEY` | Install: 64 hex pin matching publisher verifying key; embedded pk must match (C-02); mismatch `P011` |
| `SYX_REGISTRY_REQUIRE_SIGNATURE=1` | Install requires ed25519 signature; Phase 5 rejects `syx1:stub` |
| `SYX_REGISTRY_ALLOW_STUB=1` | Tests only: accept `syx1:stub` even when signatures required |
| `SYX_EMBED_DENY_UNKNOWN=1` | Embed v8: reject unknown capabilities in `SYX_EMBED_CAPABILITIES` (default on) |
| `SYX_EMBED_CAPABILITIES` | Embed capability allowlist (`wasmtime,print,inprocess`) |
| `SYX_WASI=1` | Enable wasmtime WASI preview1 (`path_open` + real `fd_read`/`fd_write` temp dir mapping) |
| `SYX_WASI_GC_PHASE22=1` | WASI GC Phase 22: `syx_gc.alloc` noop arena host hook |
| `SYX_WASI_GC_PHASE23=1` | WASI GC Phase 23: bump arena + mark-clear smoke |
| `SYX_REGISTRY_CDN_PHASE6=1` | Registry CDN Phase 6: staging signed URL install |
| `SYX_EMBED_SECCOMP=1` | Embed v9: seccomp/Job Object MVP hardening |
| `SYX_ATTACHED_HYGIENIC_V2=1` | `@attached` hygienic v2 templates (`{type}:{name}`) |
| `SYX_LLVM_RELEASE_BLOCKER=1` | Optional LLVM strict parity PR release blocker (ADR-019) |
| `SYX_LLVM_STRICT_GREEN_WEEKS=N` | ADR-017 promotion counter (≥4 green weeks for promotion) |
| `SYX_LLVM_PARITY_GATE=strict` | LLVM Top-300 runtime parity + symbol strict gate (CI Phase 34) |
| `SYX_LLVM_AOT_PHASE9=0` | Disable LLVM AOT Phase 9 IR path (unified remembered-set GC IR / ADR-044 gate) |
| `SYX_LLVM_AOT_PHASE8=0` | Disable LLVM AOT Phase 8 IR path (VM/native remembered-set GC IR / ADR-042 gate) |
| `SYX_LLVM_AOT_PHASE7=0` | Disable LLVM AOT Phase 7 IR path (WASM/native GC intrinsics IR / ADR-040 gate) |
| `SYX_LLVM_AOT_PHASE6=0` | Disable LLVM AOT Phase 6 IR path (GC intrinsics / guest heap / exec_op → 0% gate) |
| `SYX_LLVM_AOT_PHASE5=0` | Disable LLVM AOT Phase 5 IR path (CallHost/JSON / exec_op < 1% gate) |
| `SYX_LLVM_AOT_PHASE4=0` | Disable LLVM AOT Phase 4 IR path (ADR-028 Phase 4 / exec_op < 5% gate) |
| `SYX_LLVM_AOT_PHASE3=0` | Disable LLVM AOT Phase 3 IR path (ADR-028 Phase 3 / exec_op gate) |
| `SYX_LLVM_AOT_PHASE2=0` | Disable LLVM AOT Phase 2 IR path (ADR-028 Phase 2) |
| `SYX_REGISTRY_CDN_PHASE9=1` | Registry CDN Phase 9: failover retries + health probe |
| `SYX_REGISTRY_CDN_PHASE8=1` | Registry CDN Phase 8: multi-region URLs + failover |
| `SYX_UNIFIED_GC_LTS=1` | Unified generational GC prod across three backends (VM + WASM + native; ADR-045) |
| `SYX_UNIFIED_GC_LTS_PROD=1` | Unified GC LTS prod default: remembered set + promotion on all backends |
| `SYX_WASI_GC_PHASE33=1` | WASI GC Phase 33: guest remembered set prod **default on** + VM `--run` linkage (ADR-043) |
| `SYX_WASI_GC_PHASE33_PROD=1` | WASI GC Phase 33 prod default: guest + VM/native generational default on |
| `SYX_VM_NATIVE_GC_PHASE33=1` | VM/native generational GC prod (`syx run` + native codegen; ADR-043) |
| `SYX_VM_NATIVE_GC_PHASE33_PROD=1` | VM/native GC Phase 33 prod default: remembered set prod default on |
| `SYX_WASI_GC_PHASE32=1` | WASI GC Phase 32: guest remembered set full generational GC prod default on (ADR-041) |
| `SYX_WASI_GC_PHASE32_PROD=1` | WASI GC Phase 32 prod default: remembered set guest default on |
| `SYX_WASI_GC_PHASE31=1` | WASI GC Phase 31: guest generational relocating GC prod default on (ADR-039) |
| `SYX_WASI_GC_PHASE31_PROD=1` | WASI GC Phase 31 prod default: generational guest default on |
| `SYX_WASI_GC_PHASE30=1` | WASI GC Phase 30: guest relocating GC prod default on (ADR-037) |
| `SYX_WASI_GC_PHASE29=1` | WASI GC Phase 29: guest relocating GC MVP prod default on (ADR-035) |
| `SYX_INKWELL_SPIKE=0` | Explicitly disable inkwell path, fall back to clang text IR (opt-out when Phase 34 ADR-044 prod default on) |
| `SYX_INKWELL_SPIKE=1` | LLVM inkwell hello IR compile (Phase 32 opt-in; optional after Phase 33+ ADR-042/044 default GA) |
| `SYX_BORROW_COQ_LEAN_PROOF_PROD=1` | Borrow Coq/Lean **machine-checked proof prod** (obligation replay + external tool hook; Phase 34) |
| `SYX_BORROW_COQ_LEAN_EXPORT=1` | Borrow Coq/Lean external proof scaffold (`.v`/`.lean` export + replay; Phase 33) |
| `SYX_REGISTRY_CDN_PHASE15=1` | Registry CDN Phase 15: global CDN multi-active write + edge signature E2E |
| `SYX_REGISTRY_CDN_PHASE14=1` | Registry CDN Phase 14: edge cache + offline install signature E2E |
| `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` | Distributed Actor Windows Named Pipe real API (Phase 32 `CreateNamedPipe`/`ConnectNamedPipe`) |
| `SYX_DIST_ACTOR_RUSTLS_CRATE=1` | Distributed Actor rustls crate cert chain / pin E2E (Phase 32) |
| `SYX_BORROW_MACHINE_PROOF=1` | Borrow region graph machine proof harness (Phase 32 obligation export) |
| `SYX_DIST_ACTOR_WIN_PIPE_NAMED_IO=1` | Distributed Actor Windows Named Pipe real I/O (Phase 31 ER2; Phase 32 real API above) |
| `SYX_WASI_GC_PHASE28=1` | WASI GC Phase 28: guest unified heap prod default on |
| `SYX_WASI_GC_PHASE27=1` | WASI GC Phase 27: guest unified heap wiring (evaluation gate) |
| `SYX_WASI_GC_PHASE27_EVAL=1` | WASI GC Phase 27 default-on **evaluation** scaffold |
| `SYX_NET_STREAM_TLS=1` | Net stream backpressure + TLS layering (ADR-032) |
| `SYX_NET_TCP_STREAM=1` | Net `stream()` full-duplex MVP (ADR-030) |
| `editors/vscode-syx/` | VS Code extension: LSP + `type: "syx"` DAP debug; install/breakpoints in [12_BackendDebuggingGuide.md](12_BackendDebuggingGuide.md) |

---

## Related documentation

- [9_UsageGuide.md](./9_UsageGuide.md) — full usage guide
- [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) — feature priority and implementation status
- Examples: `examples/p0_demo.sy`, `crates/syx-driver/tests/`
