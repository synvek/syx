# Deinitialization

**Implementation status:** explicit `deinit` on `class` / `actor` **Partially implemented** (Phase 102/103 weak ARC deinit MVP); `struct` deinit **not supported**; `isolated deinit` **Not implemented**

A deinitializer runs cleanup logic before an instance is released. SYX classes use **ARC reference counting** reaching zero → `deinit`; structs / enums have **no** `deinit` (value types are reclaimed automatically on the stack).

## deinit Syntax

```syx
class Resource {
    let name: String
    init(name: String) { self.name = name }

    deinit {
        print("cleanup: \(self.name)")
    }
}

func main() {
    let r = Resource(name: "r1")
    print("using r1")
}                                          // Output: "cleanup: r1"
```

- `deinit` may be declared only inside a `class` / `actor`
- It takes no parameters and returns nothing
- Multiple `deinit`s are called bottom-up along the override chain

## Current Limits

| Limit | Status | Description |
|------|------|------|
| Explicit `deinit` on class | **Implemented** (Phase 102/103 MVP) | Triggered when ARC reaches zero |
| actor `deinit` | **Implemented** | When the actor is released |
| struct `deinit` | **Not implemented** | Structs are not heap allocated |
| enum `deinit` | **Not implemented** | Value type |
| `isolated deinit` | **Not implemented** | Deinitialization inside actor isolation |
| Weak-ref zeroing trigger | **Partially implemented** | `__syx_weak_*` hosts registered; alive-before-drop semantics **not fully green** (ADR-137) |
| Automatic `super.deinit` call | **Implemented** | Automatic along the override chain |

## ARC Zero Trigger

A class instance triggers `deinit` when its strong refcount reaches zero (**since Phase 73**):

```syx
class Counter {
    static var count: Int = 0
    init() { Counter.count = Counter.count + 1 }
    deinit { Counter.count = Counter.count - 1 }
}

func demo() {
    let a = Counter()
    let b = Counter()
    print(Counter.count)                   // 2
}                                          // a, b released when demo returns
demo()
print(Counter.count)                       // 0
```

> **scope-exit path:** the current implementation also supports scope-exit triggering (a local variable leaving `{ }` executes `deinit` immediately, coexisting with ARC zeroing).

## Differences from Swift ARC

Swift's ARC is **deterministic**: `deinit` runs immediately when the refcount reaches zero. SYX is currently **partially deterministic**:

| Scenario | Swift | SYX |
|------|-------|-----|
| Local `let` leaving scope | Immediate deinit | Immediate on scope-exit (some paths) |
| A `let` referenced across functions | Immediate zeroing | ARC zeroing (near-immediate) |
| Reference cycles | Memory leak | Leaks likewise (no full weak support) |
| `weak` references | Automatically nil on zeroing | **Partially implemented** (hosts registered; the semantic pins are not all green) |
| `unowned` references | Strong reference (no safety net) | Lexically reserved, **incomplete** |

## Alternative: Explicit close

For deterministic resources (files, sockets, locks), prefer an **explicit close / release**:

```syx
import IO

class File {
    let path: String
    var handle: Any? = nil
    init(path: String) { self.path = path }

    func open() { self.handle = FileHandle.open(self.path, "r") }
    func close() {
        FileHandle.close(self.handle)
        self.handle = nil
    }
    deinit { self.close() }                // Fallback
}

func main() {
    let f = File(path: "/tmp/x")
    f.open()
    f.close()                              // Explicit release
}
```

> **Recommendation:** any **critical resource** (file / socket / lock / GPU buffer) should have an explicit `close()` / `release()` method; `deinit` should serve only as a "fallback".

## Weak References

```syx
class Node {
    let value: Int
    weak var parent: Node?                  // Lexically reserved; zeroing semantics **not fully** implemented
    var children: Array<Node> = []

    init(value: Int) { self.value = value }
}
```

| Keyword | Status | Description |
|--------|------|------|
| `weak var` | **Partially implemented** | `__syx_weak_*` host path; full field semantics **140+** |
| `unowned var` | Lexically reserved, **incomplete** | Strong reference, no safety net |

> **140+ plan:** complete field-level weak / unowned semantics (Model v1 frozen in [ADR-137](../../docs/adr/architecture-decisions.md)).

## deinit and Actor Isolation

Inside an actor, state lives in the actor isolation context; when `deinit` fires, the actor itself is being released, and the current implementation does **not** guarantee that `deinit` runs on the original actor:

```syx
actor Counter {
    var value: Int = 0
    init() { self.value = 0 }

    func increment() { self.value = self.value + 1 }
    deinit { print("counter bye, last value=\(self.value)") }
}
```

`isolated deinit` (running `deinit` on the original actor) is **Not implemented**; by default `deinit` runs on the caller's thread.

## What You Can Do Inside deinit

```syx
class Cleanup {
    var name: String
    init(name: String) { self.name = name }

    deinit {
        // ✅ Read self properties
        print("bye \(self.name)")

        // ❌ Cannot write stored properties (half-deinitialized)
        // self.name = "x"

        // ✅ Can post notifications / write logs
        // ✅ Can call global functions
        // ❌ Cannot call its own instance methods (half-deinitialized)
    }
}
```

## deinit and Overriding

```syx
class Base {
    deinit { print("base") }
}

class Derived: Base {
    deinit { print("derived") }
}

let d = Derived()
print("use d")
// On release it prints "derived" then "base" (the override chain calls super.deinit automatically)
```

> **Overriding `deinit` does not need `override`:** the `deinit` keyword itself means "implement destruction"; there is no override marker.

## Complete Example

```syx
class Logger {
    let name: String
    var logCount: Int = 0
    static var total: Int = 0

    init(name: String) {
        self.name = name
        Logger.total = Logger.total + 1
    }

    func log(_ s: String) {
        logCount = logCount + 1
        print("[\(name)] \(s)")
    }

    deinit {
        Logger.total = Logger.total - 1
    }
}

func process() {
    let l1 = Logger(name: "main")
    let l2 = Logger(name: "audit")
    l1.log("start")
    l2.log("open")
    print("alive=\(Logger.total)")         // 2
}                                          // l1, l2 released
print("alive=\(Logger.total)")             // 0
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **P001** | Top-level `deinit` | Move it inside a class / actor |
| **S004** | `deinit` with parameters | Remove them (it takes none) |
| **R007** | `!` unwrapping nil inside deinit | Use `if let` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Deterministic `deinit` | **Partially implemented** (scope-exit + ARC zeroing MVP) |
| `weak` / `unowned` | Lexically reserved, **incomplete** |
| `isolated deinit` (actor) | **Not implemented** |
| struct `deinit` | **Not implemented** (value types are not heap allocated) |
| Explicit `super.deinit` | **Automatic** (no explicit call needed) |
| Writing stored properties in `deinit` | **Forbidden** (half-deinitialized) |

---

**Next chapter** → [Error Handling](17-error-handling.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 15, Initialization](15-initialization.md) · [Chapter 26, Memory Management and Garbage Collection](26-memory-management.md)
