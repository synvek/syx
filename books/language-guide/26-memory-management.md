# Memory Management and Garbage Collection

**Implementation status:** **ARC** for class (Phase 73 · class `ObjectRef` retain/release · VM + WASM tier-2D + Native LLVM direct emit); **ManagedHeap v4 generational tracing GC** unified across VM / WASM / Native (Phase 75); Inkwell LLVM default on (Phase 77); `syx-rt` direct emit tier-3 (Phase 77); Borrow checking (Phase 74 · SE-0461 + async/actor subset)

The SYX memory model is a dual track of **ARC + ManagedHeap generational tracing GC**: classes go through ARC reference counting (semantics similar to Swift, but managed underneath by the host bridge layer), while the remaining heap objects (Array / String / Dictionary / closures, …) go through a **unified generational tracing GC**. This chapter explains the memory cooperation of the three back ends (VM / WASM / Native), the relationship between `gc` and `deinit`, borrow checking, and lifetime annotations.

## Memory Model Overview

| Back end | class references | Heap objects (Array/String/Dict/closures) | Reclamation model |
|------|------------|-----------------------------------|----------|
| **VM** | ARC (class `ObjectRef` retain/release) | ManagedHeap v4 generational GC | Strong refcount zero → `deinit`; heap objects → GC |
| **WASM** | ARC (tier-2D) | ManagedHeap v4 generational GC (**unified in Phase 75**) | Same as above |
| **Native (LLVM)** | ARC (tier-3 direct emit) | ManagedHeap v4 generational GC (unified barriers) | Same as above |

> **Unified GC LTS (Phase 75):** the VM, WASM and Native back ends share ManagedHeap v4 compact + the unified HeapSlot Array/String implementation; mark-sweep + slide compact.

## ARC (class Reference Semantics)

A class instance is **heap allocated + reference counted**. `ObjectRef` tracks retain / release:

```syx
class User {
    let name: String
    init(name: String) { self.name = name }
}

func main() {
    let a = User(name: "Alice")             // retain: 1
    let b = a                                // retain: 2
    let c = a                                // retain: 3

    b = nil                                  // release
    c = nil                                  // release
}                                            // release → deinit
```

> **deinit trigger:** `Type_deinit` is called when the refcount reaches zero (an explicit class deinit). See [Chapter 16, Deinitialization](16-deinitialization.md).

| Feature | Status |
|------|------|
| class `ObjectRef` | **Implemented** (Phase 73) |
| retain / release | **Implemented** |
| ARC zeroing → deinit | **Implemented** (Phase 102/103 MVP) |
| Complete `weak var` | **Partially implemented** (lexically reserved; zeroing judgment still evolving) |
| `unowned var` | **Partially implemented** |

## ManagedHeap Generational Tracing GC

Non-class heap objects (`Array<T>` / `String` / `Dictionary<K,V>` / closures) go through a **generational tracing GC**:

| Mechanism | Status |
|------|------|
| Mark-sweep | **Implemented** (Phase 73) |
| Generational young/old | **Implemented** (unified in Phase 75) |
| Promotion | **Implemented** |
| Remembered set / card table | **Implemented** (three-back-end linkage) |
| Slide compact | **Implemented** (Phase 75 v4) |
| Relocating GC | **Implemented** (Phase 37 ADR-037) |
| Write barrier | **Implemented** (a cross-generation write triggers a remember update) |

> **Three-back-end linkage:** ADR-045 (Phase 34) unified the GC barriers of VM / WASM / Native; MIR v23 LICM unified barrier placement across the three back ends.

```syx
func makeLargeArray() -> Array<Int> {
    var arr: Array<Int> = []
    for i in 0..<10_000 { arr.append(i) }
    return arr                              // young gen allocation; reclaimed by GC when it leaves scope
}
```

### GC Configuration (Environment Variables)

| Variable | Effect |
|------|------|
| `SYX_UNIFIED_GC_LTS=1` | Enables Unified GC LTS (prod by default) |
| `SYX_UNIFIED_GC_LTS_PROD=1` | Forces prod mode |
| `SYX_WASI_GC_PHASE33=1` | Enables WASI generational GC (Phase 33) |
| `SYX_VM_NATIVE_GC_PHASE33=1` | Enables VM/native generational GC (Phase 33) |
| `SYX_GC_NURSERY=1048576` | Nursery size (bytes, default 1 MB) |

### When GC Triggers

- **Automatic:** allocation rate / heap occupancy threshold / nursery full
- **Manual:** `import Runtime; Runtime.gc()` forces collection (**Partially implemented**)
- **Scope exit:** ARC zeroing triggers `deinit` immediately; heap objects are collected by GC later

## `deinit` and GC

| Type | Reclamation model | `deinit` timing |
|------|----------|---------------|
| `class` | ARC | Refcount reaches zero |
| `actor` | ARC + isolation | Refcount reaches zero + actor shutdown |
| `struct` | Stack | None (value type) |
| `enum` | Depends on the case | Depends on the case |
| `Array<T>` and others | GC | When GC reclaims (nondeterministic timing) |

```syx
class Resource {
    let id: Int
    init(id: Int) { self.id = id }
    deinit { print("bye #\(id)") }
}

class Container {
    var res: Resource?
    init() { self.res = Resource(id: 1) }   // ARC +1
    deinit { print("container bye") }
}

func main() {
    let c = Container()                      // c: ARC +1
    // c.res: ARC +1
}                                            // c: ARC -1 → deinit → c.res: ARC -1 → deinit
```

## Borrowing &T / &mut T

Borrowing is a compile-time **reference safety** mechanism (it does not involve the runtime):

```syx
func read(x: &Int) -> Int { return x }      // Immutable borrow
func write(x: &mut Int) { x = x + 1 }        // Mutable borrow

func main() {
    var n = 10
    print(read(x: &n))                       // 10
    write(x: &mut n)
    print(n)                                 // 11
}
```

| Form | Meaning |
|------|------|
| `&T` | Immutable borrow (several at once) |
| `&mut T` | Mutable borrow (only one at a time) |
| `&'a T` | With a lifetime annotation (**Partially implemented**) |

> **Borrow rules:** a mutable borrow cannot coexist with any other borrow at the same time. A violation → **S011** compile error.

```syx
var n = 10
let r1 = &mut n
// let r2 = &n                              // ❌ S011: cannot borrow as immutable while mutably borrowed
// print(n)                                 // ❌ S011: cannot access through mutable borrow
r1 = &mut n
```

**Diagnostic:** S011 (borrow/mutation violation)

## Lifetimes 'a

```syx
func id<'a>(x: &'a String) -> &'a String { return x }
```

| Capability | Status |
|------|------|
| Lifetime annotations | **Partially implemented** (S017) |
| NLL (Non-Lexical Lifetimes) | **Implemented** |
| Borrows across `await` | **Partially implemented** |
| Higher-ranked lifetimes | **Not implemented** |

Tests: `p15_lifetime_a_*.sy` · `p17_lifetime_await_*.sy`

## Native / LLVM Back End

Since Phase 75, **Inkwell Prod GA** ([ADR-044](../../docs/adr/architecture-decisions.md)) has made the real inkwell crate IR builder the default native compilation path:

- Unified generational GC barriers emitted directly to native
- ARC class retain/release goes through `__syx_object_*` direct emit
- The `syx-rt` static library provides the host bridge layer

**Phase 77 enhancements:**

- `materialize_array` / `materialize_dict` go through HeapSlot
- `rt_direct_exec` stack operations emitted directly
- `reflect_specialization` generic specialization reflection MVP
- obligation export (borrow machine proofs)

## Toolchain Cache

- `target/cache/` MIR incremental compilation (`syx-cache-v110`)
- If compile-time behavior looks wrong, delete the cache and retry

## Differences from Swift

| Swift ARC | SYX |
|-----------|-----|
| Refcount zeroing means immediate deinit | **Partially implemented** (scope-exit + ARC zeroing MVP) |
| Complete `weak` / `unowned` | **Partially implemented** (lexically reserved; zeroing judgment still evolving) |
| `autoreleasepool` | **Not implemented** (GC replaces it) |
| Complete `move` semantics | **Partially implemented** (borrow regions v8) |
| Swift 6 strict concurrency | **Partially implemented** (Sendable / actor / `@MainActor`) |

## Complete Example

```syx
class Buffer {
    var data: Array<UInt8> = []
    init(size: Int) {
        for _ in 0..<size { data.append(0) }
    }
    deinit { print("Buffer freed") }
}

func process(b: &Buffer) {
    for i in 0..<b.data.count { b.data[i] = UInt8(i % 256) }
}

func main() {
    let b = Buffer(size: 10)                // ARC +1
    process(b: &b)                           // Immutable borrow

    // Array element modification goes through the GC write barrier
    for i in 0..<b.data.count {
        b.data[i] = b.data[i] + 1
    }
    // The local variable leaves scope → ARC -1 → deinit
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S003** | Reassigning a borrowed `let` | Change it to `var` |
| **S011** | Mutable + immutable borrow at the same time | Restructure the scopes |
| **S017** | Lifetime violation | Annotate explicitly or restructure the code |
| **R011** | Integer overflow | Check the bounds |
| **R016** | Required field not initialized | Complete the init |

## Further Reading

- [Chapter 27, Memory Safety](27-memory-safety.md) — borrow machine proofs / Sendable
- [Chapter 16, Deinitialization](16-deinitialization.md) — deinit triggering
- [Chapter 18, Concurrency](18-concurrency.md) — actor isolation / Sendable
- [docs/10_BackendParity.md](../../docs/10_BackendParity.md) — VM / MIR / LLVM / WASM memory alignment
- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-045 (unified GC LTS) and others

---

**Next chapter** → [Memory Safety](27-memory-safety.md)

**See also** → [Chapter 16, Deinitialization](16-deinitialization.md) · [Chapter 18, Concurrency](18-concurrency.md) · [docs/10_BackendParity.md](../../docs/10_BackendParity.md) · [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md)
