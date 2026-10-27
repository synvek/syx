# Memory Safety

**Implementation status:** borrow checking (**Implemented** S011); machine-proof harness (**Phase 32**); Coq/Lean external proof scaffold (**Phase 33**); machine-checked proof prod (**Phase 34**); Actor isolation (**Implemented** A002/C001); `@unchecked Sendable` (**Partially implemented**)

Memory safety prevents **data races, dangling references, and illegal mutation**. SYX builds a safety net from four layers: **compile-time borrow checking + runtime GC barriers + actor isolation + Sendable constraints**.

## Borrow Checking (Compile Time)

A mutable borrow cannot coexist with a shared borrow (simplified NLL):

```syx
func read(x: &Int) -> Int { return x }
func write(x: &mut Int) { x = x + 1 }

func main() {
    var n = 10
    let r1 = &n                              // Shared borrow
    print(read(x: r1))                       // 10
    r1 = nil                                  // Release the shared borrow

    let r2 = &mut n                          // Mutable borrow
    write(x: r2)                              // Legal
}
```

**S011** — Cannot mutate / borrow conflict

```syx
var n = 10
let r1 = &mut n                              // Mutable borrow
// let r2 = &n                              // ❌ S011: cannot borrow as immutable
// print(n)                                 // ❌ S011: cannot access through mutable borrow
r1 = nil                                     // Release the mutable borrow
let r3 = &n                                  // ✅ Now a shared borrow is allowed
```

Tests: `p6_borrow_*.sy` · `p23_borrow_regions_v5_*.sy`

### Summary of Borrow Rules

| State | Mutable borrow | Shared borrow | Read access | Write access |
|------|----------|----------|--------|--------|
| No borrow | ✅ | ✅ | ✅ | ✅ |
| Active mutable borrow `&mut` | ❌ S011 | ❌ S011 | ❌ S011 | ❌ S011 |
| Active shared borrow `&` | ❌ S011 | ✅ | ✅ | ❌ S011 |

> **NLL (Non-Lexical Lifetimes):** a borrow's scope ends at its **last use**, **not** at the end of the lexical block. This simplified model makes Rust-style borrowing easier to write.

## Machine-Proof Harness (Phase 32+)

Phase 32 introduced the **borrow machine-proof harness** (`borrow_regions_machine_proof_enabled()`): the compiler exports the region graph as **proof obligations**, for replay smoke and linking with external proof tools.

| Phase | Capability |
|------|------|
| Phase 31 | Formal verification scaffold (region graph proof smoke) |
| Phase 32 | **Machine-proof harness** — region graph → proof obligation export + replay |
| Phase 33 | **Coq/Lean external proof scaffold** — obligation → `.v`/`.lean` export + replay |
| Phase 34 | **Machine-checked proof prod** — obligation replay + external Coq/Lean tool hook prod |

Enabling:

```bash
SYX_BORROW_MACHINE_PROOF=1 syx run program.sy
SYX_BORROW_COQ_LEAN_EXPORT=1 syx run program.sy        # Phase 33 Coq/Lean export
SYX_BORROW_COQ_LEAN_PROOF_PROD=1 syx run program.sy    # Phase 34 machine-checked prod
```

Test fixtures: `p32_borrow_proof_*` · `p33_borrow_coq_lean_*` · `p34_borrow_coq_lean_proof_*`

## Lifetimes

```syx
func longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if (x.count > y.count) { return x }
    return y
}
```

| Capability | Status |
|------|------|
| Lifetime annotations `<'a>` | **Partially implemented** (S017) |
| Lifetime inference | **Implemented** |
| Borrows across `await` | **Partially implemented** (async/actor subset) |
| Higher-ranked lifetimes | **Not implemented** |
| Lifetime bounds `'a: 'b` | **Partially implemented** |

Tests: `p15_lifetime_a_*.sy` · `p17_lifetime_await_*.sy`

## Actor Isolation

Accessing actor state directly from outside → **A002** / **C001**:

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
}

let c = Counter()
// print(c.value)                            // ❌ A002: cross-actor access without await
print(await c.value)                        // ✅ Crossing isolation requires await
```

See [Chapter 18, Concurrency](18-concurrency.md#actor-isolation).

## Sendable

A type passed across isolation must be `Sendable`:

```syx
@Sendable
data class User(name: String, age: Int)

actor Cache {
    var users: Array<User> = []
    func put(_ u: User) { users.append(u) } // User satisfies Sendable
}

let cache = Cache()
let u = User(name: "Alice", age: 30)
await cache.put(u)                          // Passing a Sendable value across isolation
```

| Annotation | Status | Description |
|------|------|------|
| `@Sendable` | **Implemented** | Marks a type as safe to pass across isolation |
| `@unchecked Sendable` | **Partially implemented** | Suppresses checking; you accept the risk |

> **Phase 60 strict mode:** `Sendable` checking is a compile error **C003** E2E in `strict` mode (`concurrency.rs`).

```syx
// A non-Sendable crossing actors → compile error
class NotSendable {
    var mutable: Int = 0                    // Reference type + internal mutability
}

let bad = NotSendable()
// await cache.put(bad)                     // ❌ C003: NotSendable is not Sendable
```

## @unchecked Sendable

```syx
@unchecked Sendable
class MyLegacy {
    var internal: Int = 0
}
```

> **Risk:** `@unchecked` suppresses checking; concurrent access from multiple threads needs external synchronization (such as `Lock` / `actor`), otherwise a data race may occur.

## GC and Memory Safety

**Unified GC LTS** ([ADR-045](../../docs/adr/architecture-decisions.md)) unifies remembered-set generational GC across the three back ends VM/WASM/native, guaranteeing that cross-generation references are not missed during a minor collect; it complements borrow checking — compile-time borrowing + runtime GC barriers.

| Dimension | Compile time | Runtime |
|------|--------|--------|
| Borrow checking | ✅ S011 | N/A |
| Lifetimes | ✅ S017 | N/A |
| Cross-generation reference barriers | N/A | ✅ write barrier |
| Actor isolation | Type check A002 | queue serial |
| Sendable | Type check C003 | witness |

## Complete Example

```syx
@Sendable
data class Item(id: Int, name: String)

actor Store {
    private var items: Array<Item> = []
    func add(_ i: Item) { items.append(i) }
    func count() -> Int { return items.count }
    func all() -> Array<Item> { return items }
}

func main() async {
    let store = Store()
    let items = [
        Item(id: 1, name: "Apple"),
        Item(id: 2, name: "Banana"),
        Item(id: 3, name: "Cherry")
    ]
    for i in items { await store.add(i) }
    print(await store.count())              // 3
    print(await store.all().count)         // 3
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S011** | Mutable + immutable borrow at the same time | Restructure the scopes |
| **S017** | Lifetime violation | Annotate explicitly / restructure the code |
| **A002** | Cross-actor access without await | Add `await` |
| **C001** | Actor isolation violation | Move it inside the actor |
| **C003** | Passing a non-Sendable value across isolation | Add `@Sendable` / `@unchecked Sendable` |
| **S019** | Sendable channel | Use a Sendable element type |

## Differences from Swift

| Swift 6 strict concurrency | SYX |
|-----------------|-----|
| Sendable checking | **Implemented** (strict mode C003 E2E) |
| Actor isolation | **Implemented** (A002/C001) |
| Borrow checking | **Implemented** (S011) |
| Lifetimes | **Partially implemented** |
| Machine proofs | **Partially implemented** (Phase 32+ scaffold) |
| Global actors | **Implemented** (`@MainActor`) |
| `sending` parameters | **Implemented** (Phase 26) |
| Static data-race detection | **Partially implemented** |

## Further Reading

- [Chapter 26, Memory Management and Garbage Collection](26-memory-management.md) — ARC / generational GC / borrowing
- [Chapter 18, Concurrency](18-concurrency.md) — actor / async / Task
- [Chapter 16, Deinitialization](16-deinitialization.md) — deinit / resource release

---

**Next chapter** → [Access Control](28-access-control.md)

**See also** → [Chapter 26, Memory Management](26-memory-management.md) · [Chapter 18, Concurrency](18-concurrency.md) · [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) · [plan/topics/gc-parity.md](../../plan/topics/gc-parity.md)
