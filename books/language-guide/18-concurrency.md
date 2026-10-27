# Concurrency

**Implementation status:** `async` / `await` / `async let` / `Task` / `Task.detached` / `TaskGroup` / `actor` / `GlobalActor` / `Channel` / `select` / `Sendable` **Implemented**; structured concurrency MVP; distributed actor **partial**; bounded actor mailbox + shared pool **Phase 138 MVP**

The SYX concurrency model is based on **coroutines + actors**, consistent with Swift Concurrency. This chapter covers the `async` / `await` basics, `Task`, actor isolation, `Channel`, `GlobalActor`, Sendable, and the beginnings of distributed actors.

## async / await

```syx
import Net

async func fetch() -> String {
    return await Net.get("https://example.com")
}

async func main() {
    let body = await fetch()
    print(body)
}
```

- An `async` function may suspend / resume (generator state machine)
- `await` suspends the current task and waits for the asynchronous operation to finish
- Multiple-`await` state machines are **Implemented** (test `runtime/async_native.rs`)

> **Key limit:** `await` is legal only inside an `async` function body; using `await` in a non-`async` function → compile error **S020**.

## async let

Start multiple subtasks in parallel:

```syx
async func parallelFetch() -> (String, String) {
    async let a = fetchA()                 // Start the subtask immediately
    async let b = fetchB()                 // Start the subtask immediately
    let x = await a                        // Wait for a
    let y = await b                        // Wait for b
    return (x, y)
}
```

> **Semantics:** `async let name = expr` desugars to detach + join (**behavior frozen in SYX 132**). `a` is an `AsyncValue<T>`; `await a` blocks until the subtask completes.

## Task

`Task { }` starts an **unstructured** subtask:

```syx
let t = Task {
    print("in task")
}
await t.join()                             // Wait for completion

Task.detached {
    print("detached")
}
```

| Form | Behavior |
|------|------|
| `Task { … }` | Starts in the current actor context (subject to actor isolation) |
| `Task.detached { … }` | Does not inherit the current context; a separate actor context |
| `t.join()` | Waits for the task to finish (returns `()`) |
| `t.cancel()` | Cancels the task (**Partially implemented**) |
| `t.value` | Waits for and returns `Result<T, Error>` |

## TaskGroup

A structured concurrency task group:

```syx
async func fetchAll(urls: Array<String>) -> Array<String> {
    let group = TaskGroup<String>()
    for url in urls {
        group.add {
            return await Net.get(url)
        }
    }
    return await group.waitAll()
}
```

> **Status:** `TaskGroup` is **Implemented** (M3 style; not the full Swift `withTaskGroup` API). Basic `add` / `waitAll` / cancellation propagation are **Implemented**. Product surface `taskGroupCancel` (ADR-260) + `liveTaskCount` (ADR-261); the **A01 leaf ADR-272 is signed** (≠ EC-M3-2). `setDeadlineMs` (ADR-262) + `taskLocalSet`/`taskLocalGet` (ADR-263); the **A02 leaf ADR-273 is signed** (≠ `@TaskLocal` / ≠ EC-M3). `actorReentrancyPolicy` (ADR-264/265: `serial-no-reenter;await-hold`); the **A03 leaf ADR-274 is signed** (≠ Swift await-reentrancy GA / ≠ EC-M3).

## Actor Isolation

An `actor` protects its internal state; external access requires `await`:

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

let c = Counter()
await c.increment()                        // Crossing isolation requires await
print(await c.current())                   // 1
```

| Rule | Description |
|------|------|
| Internal methods | Automatically inside the actor isolation domain |
| External access | Must use `await` (isolation violation → **A002**) |
| Synchronous reentrancy on the same instance | **Forbidden** (ADR-264): re-entering the same instance synchronously inside a mailbox handler → **C009** (`serial-no-reenter`) |
| `await`/`sleep` inside a handler | **await-hold** (ADR-265): other messages in the same mailbox are not processed midway; **not** Swift await-reentrancy GA |
| `nonisolated func` | Explicit non-isolated marker (**Partially implemented**) |
| `actor` cannot be inherited | A top-level `class` cannot inherit an `actor` |
| `Sendable` fields | `Sendable` values held by an actor can safely cross tasks |

### Sendable

Types passed across isolation must be `Sendable`:

```syx
@Sendable
data class User(name: String, age: Int)

actor Cache {
    var users: Array<User> = []
    func put(_ u: User) { users.append(u) }
    func get() -> Array<User> { return users }
}

let cache = Cache()
let u = User(name: "Alice", age: 30)
await cache.put(u)                         // Passing a Sendable value across isolation
let arr = await cache.get()                // Receiving an array of Sendable values
```

| Annotation | Description |
|------|------|
| `@Sendable` | Marks a type as safe to pass across isolation |
| `@unchecked Sendable` | Suppresses checking and accepts the data-race risk (**Partially implemented**) |
| Built-in `Sendable` | `Int` / `String` / `Array<T: Sendable>` and others are automatic |

> **Phase 60 strict mode:** `Sendable` checking is a compile error **S019** / **C003** E2E in `strict` mode.

## GlobalActor / @MainActor

Bind a group of declarations to a global actor:

```syx
global actor MainActor { }                 // Global actor definition

@MainActor
func updateUI() {
    // Runs on MainActor
}

@MainActor
class ViewModel {
    var title: String = ""
    func setTitle(_ s: String) { title = s } // Isolated on MainActor
}
```

- Global actors are cross-platform abstractions: the UI thread, the GPU thread, and so on
- Phase 34 `GlobalActor v14` supports global multi-active CDN routing and cross-region health probes
- Calls across GlobalActors require `await`

## Channel

Cross-task communication. The product surface is the `Channel` class from `import Sync` (not `__syx_channel_*`):

| Form | Meaning |
|------|------|
| `Channel()` | Default `init`: an unbounded channel (capacity `0`) |
| `Channel.create(n)` | Factory method: bounded when `n > 0`, unbounded when `n == 0`; the return type is **`Channel`** (not a bare `Object`) |

The difference between `Channel()` and `Channel.create(n)`: the former is the class's default constructor; the latter is a factory that takes a capacity. Semantically it is equivalent to "a constructor with arguments", but since the current `init()` / `init(capacity:)` overload would be mis-parsed, use `create` for bounded channels.

Instance methods: `send` / `recv` / `trySend` / `tryRecv` / `close` / `count()` / `capacity()`. Only `Channel.create(n)` remains as a static factory.

> The `Channel<Int>` generic syntax shown in tutorials is the planned form; the current MVP payload is `Object`, and types are constrained by calling convention.

```syx
import Sync

let ch = Channel.create(2)
ch.send(1)                                 // Synchronous by default
ch.send(2)
ch.close()
```

```syx
async func producer(ch: Channel<Int>) {
    for i in 0..<5 {
        ch.send(i)
    }
    ch.close()
}

async func consumer(ch: Channel<Int>) {
    while (true) {
        let v = ch.receive()
        if (v == nil) { break }
        print(v!)
    }
}

async func main() {
    let ch = Channel<Int>()
    async let p = producer(ch: ch)
    async let c = consumer(ch: ch)
    await p
    await c
}
```

| Operation | Sync / async | Description |
|------|------------|------|
| `ch.send(v)` | Blocking / `await` | Send |
| `ch.receive()` | Returns `T?` | Receive; `nil` means closed |
| `ch.close()` | Marks closed | **ADR-266**: really disconnects the sender; after the buffer drains it returns `nil`; further sends → **C010**; **not** a full backpressure/fairness hook |
| `ch.trySend(v)` | Non-blocking | **ADR-267**: when the bounded buffer is full → **false**; `count`/`capacity` are observable; **not** a full fairness hook / **not** the A04 hook |
| `for x in ch` | Async iteration | Equivalent to `while let x = ch.receive()` |
| `ch.count()` / `ch.capacity()` | Methods | Current length / capacity (ADR-267 backpressure pin) |

Source: `fixtures/parity/stdlib/channel_send.sy`

## select

Listening on multiple channels:

```syx
select {
case let v = chA.receive() { print("A: \(v)") }
case let v = chB.receive() { print("B: \(v)") }
case chC.send(42) { print("sent to C") }
}
```

> **Status:** `select` is **Implemented** (`fixtures/runtime/select_await_case_order.sy`); N-way cases plus a default context.

| Contract | Description |
|------|------|
| Several arms ready at once | **ADR-268**: thread-local round-robin (fingerprint `select-fair-v1:round-robin`); product pin `m3_a04_fairness`; **not** cross-thread/global cursor GA |
| A04 leaf | **ADR-269**: closes out the three axes close / backpressure / fairness (`m3_a04_closeout`); **not** cross-thread RR GA / **not** EC-M3 |

## distributed actor (Partially Implemented)

```syx
distributed actor ClusterNode {
    var id: String
    init(id: String) { self.id = id }

    distributed func ping() -> String {
        return "pong from \(self.id)"
    }
}
```

- Actors spanning processes / machines (**MVP spike**)
- A `distributed func` may be called remotely
- Complete distributed actors → later Phases

## Cancellation

```syx
let t = Task {
    for i in 0..<1000 {
        if (Task.isCancelled) { return }  // Check
        await doWork(i)
    }
}

// From outside
t.cancel()
```

> **Cooperative cancellation:** a task must explicitly check `Task.isCancelled`; library code may respond automatically at `await` points.

## Application Concurrency (Main Business API)

Everyday business code should prefer structured concurrency and messaging surfaces over direct thread-pool knobs:

| Surface | Use |
|------|------|
| `Task` / `TaskGroup` / `async`/`await` / `async let` | Structured task tree (M3-A01 leaf: cancel / liveTaskCount; ADR-272) |
| `setDeadlineMs` / `taskLocalSet`/`Get` | Deadlines and task-locals (M3-A02 leaf; ADR-273) |
| `actor` / `actorReentrancyPolicy` | Actor isolation and reentrancy contracts (M3-A03 leaf; ADR-274) |
| `Channel` / `select` | Bounded channels, close / backpressure / fairness (M3-A04) |

## Advanced Runtime Tuning (Deploy/Ops)

> **ADR-270 / ADR-271 / M3-A05 leaf:** the `Concurrency.*` pools and actor capacity knobs below are **advanced runtime configuration**, **not** the preferred concurrency API for ordinary applications. Prefer the structured concurrency + bounded Channel/mailbox described above. The A05 leaf is signed (≠ EC-M3 / ≠ removing the pool API).

```syx
import Concurrency

// Advanced runtime tuning (deploy/ops) — not the primary business concurrency API
Concurrency.setWorkerCount(8)
Concurrency.setCorePoolSize(2)
Concurrency.setMaximumPoolSize(8)
Concurrency.setKeepAliveMs(30_000)
Concurrency.setTaskQueueCapacity(100)
Concurrency.setTaskRejectPolicy("abort")   // "abort" | "callerRuns" | "discardOldest"
Concurrency.setMaxActors(64)
Concurrency.setMailboxCapacity(256)
```

> **Status:** worker / maxActors / mailbox **Phase 133 partial GA**; queue / reject / core-max-keepAlive **Phase 134–136 partial GA**; actor shared pool **Phase 138 partial GA**. **Not** the full Java `ThreadPoolExecutor` compatibility set → **139+**; **not** EC-M3; the A05 leaf **ADR-271** is signed.

### Task Pool vs. Blocking Pool (ADR-275 / ADR-276 / M3-B01)

`Concurrency.blockingSleep` goes to a separate `blocking_pool` (fingerprint `task-pool+blocking-pool-v1-sized`); `Concurrency.setBlockingPoolSize` / `blockingPoolSize` set a fixed capacity (core=max=n). The existing `Concurrency.sleep` / `__syx_task_sleep` still block on the calling task worker. HostFns such as HTTP/File have **not** been rerouted. The B01 leaf **ADR-277** is signed. **Not** epoll/kqueue reactor GA; **not** EC-M3.

`File.readAsync` (ADR-278) and `TcpStream.readAsync` (ADR-279) go through `blocking_pool` pending+await (fingerprint in `Concurrency.ioAdapterPolicy`, currently `v1-tcp`); the synchronous `File.read` / `Http.get` / `TcpStream.read` are **not** rerouted. The B02 leaf **ADR-280** is signed. **Not** TcpWrite/Connect async; **not** reactor GA; **not** EC-M3.

### Mutex RAII / Blocking (ADR-281 / ADR-282 / ADR-283 / M3-C01)

`Mutex.lockGuard` returns a `MutexGuard`; when the scope exits, `deinit` calls `__syx_mutex_unlock_quiet`. The product `Mutex.lock` / `lockGuard` go through `__syx_mutex_lock_wait` (Condvar wait while held; ADR-282). The existing `__syx_mutex_lock` remains a try-lock (Bool). The fingerprint is in `Concurrency.syncPolicy` (`mutex-raii-guard-v1-blocking`). The C01 leaf **ADR-283** is signed. **Not** RWLock; **not** a product Condvar; **not** EC-M3.

### Semaphore (ADR-284 / M3-C02)

`Semaphore.create(permits)` / `wait` / `tryWait` / `signal` (a counting semaphore; a separate table); ADR-285: `SemaphoreGuard` / `waitGuard` (quiet signal on scope exit). The fingerprint is in `Concurrency.semaphorePolicy` (`semaphore-v1-raii`). ADR-286: the C02 leaf is signed (Semaphore slice). **Not** RWLock; **not** a product Condvar; **not** EC-M3.

### Memory Ordering / Atomics (Honest Boundaries) (ADR-287 / ADR-288 / M3-C03)

**Shared mutable state** across Tasks / Actors must go through `Mutex` / `Semaphore` / `Channel` (or actor isolation); these synchronization operations establish **happens-before** within the slice definition. This slice does **not** offer a language-level `Atomic*` / `memory_order` (SeqCst/Acquire/Release) product surface; it does **not** deliver an `AtomicInt` / CAS / fetchAdd stdlib API. `__syx_wasm_atomic_add_i32` is registered but its capability is **unsupported** (≠ Wasm atomics product GA). Aligned with Memory Model v1 (ADR-137) and Concurrency Model v1 (ADR-139 / Sendable). ADR-288: the C03 leaf is signed (docs slice). **Not** EC-M3.

### race/stress/chaos CI Sampling (ADR-289 / ADR-290 / ADR-291 / M3-C04)

Reproducible concurrency stress samples live in `packages/internal/race_stress_samples.toml` (`kind=stress`: Mutex/Semaphore contention + C04 sampling smoke; `kind=chaos`: TaskGroup cancellation interruption + C04's own `m3_c04_chaos`). CI wiring: `m3_c04_race_stress` / `m3_c04_chaos` / `m3_c04_closeout`. The leaf is closed out (ADR-291). Honest interpretation: a regression gate corpus; chaos = cooperative cancellation/interruption sampling. **Not** TSAN/loom GA; **not** a kill-process/network-partition chaos farm; **not** the full EC-M3-1 high-concurrency stress hook.

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `actor` isolation | **Implemented** |
| `async` / `await` | **Implemented** (multiple-await state machine GA) |
| `Task { }` | **Implemented** |
| `Task.detached` | **Implemented** |
| `TaskGroup` | **Implemented** (basic add / waitAll) |
| `async let` | **Implemented** (desugars to detach+join) |
| Structured Concurrency | **MVP** (not the full Swift set) |
| `GlobalActor` / `@MainActor` | **Implemented** |
| `Sendable` | **Implemented** (strict mode C003 E2E) |
| `Channel` | **Implemented** |
| `select` | **Implemented** |
| `AsyncSequence` `for await in` | **Implemented** |
| `distributed actor` | **partial** MVP spike |
| `Cancellation` | **Cooperative** (partially implemented) |
| Full Java `ThreadPoolExecutor` set | **Not implemented** (**139+**) |

## Complete Example

```syx
import Net

actor Downloader {
    var results: Array<String> = []
    func put(_ r: String) { results.append(r) }
    func all() -> Array<String> { return results }
}

async func main() {
    let urls = [
        "https://example.com",
        "https://example.org",
        "https://example.net"
    ]
    let dl = Downloader()
    let group = TaskGroup<Void>()

    for url in urls {
        group.add {
            let body = await Net.get(url)
            await dl.put(body)
        }
    }

    await group.waitAll()
    let all = await dl.all()
    print("downloaded \(all.count) pages")
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S020** | Using `await` in a non-`async` function | Add `async` |
| **A002** | Cross-actor access without `await` | Add `await` |
| **C001** | Actor isolation violation | Move into the actor isolation domain |
| **C003** | Passing a non-`Sendable` value across isolation | Add `@Sendable` / `@unchecked Sendable` |
| **R007** | Unwrapping nil from `await ch.receive()` | Check with `for-in` or `while let` |

---

**Next chapter** → [Macros](19-macros.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [UsageGuide Concurrency Control](../../docs/9_UsageGuide.md) · [Error Handling](17-error-handling.md)
