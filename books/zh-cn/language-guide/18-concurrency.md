# 并发

**实现状态：** `async` / `await` / `async let` / `Task` / `Task.detached` / `TaskGroup` / `actor` / `GlobalActor` / `Channel` / `select` / `Sendable` **已实现**；structured concurrency MVP；distributed actor **partial**；Actor mailbox 有界 + shared pool **Phase 138 MVP**

SYX 并发模型基于 **协程 + actor**，与 Swift Concurrency 风格一致。本章覆盖 `async` / `await` 基础、`Task`、`actor` 隔离、`Channel`、`GlobalActor`、Sendable 与 distributed actor 雏形。

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

- `async` 函数可暂停 / 恢复（生成器状态机）
- `await` 暂停当前任务，等待异步操作完成
- 多 `await` 状态机 **已实现**（测试 `runtime/async_native.rs`）

> **关键限制：** `await` 仅在 `async` 函数体内合法；非 `async` 函数使用 `await` → 编译错误 **S020**。

## async let

并行启动多个子任务：

```syx
async func parallelFetch() -> (String, String) {
    async let a = fetchA()                 // 立即启动子任务
    async let b = fetchB()                 // 立即启动子任务
    let x = await a                        // 等待 a
    let y = await b                        // 等待 b
    return (x, y)
}
```

> **语义：** `async let name = expr` desugar 为 detach + join（**SYX 132 行为冻结**）。`a` 是 `AsyncValue<T>`；`await a` 阻塞直到子任务完成。

## Task

`Task { }` 启动一个**非结构化**子任务：

```syx
let t = Task {
    print("in task")
}
await t.join()                             // 等待完成

Task.detached {
    print("detached")
}
```

| 形式 | 行为 |
|------|------|
| `Task { … }` | 在当前 actor 上下文启动（受 actor 隔离约束） |
| `Task.detached { … }` | 不继承当前上下文，独立 actor 上下文 |
| `t.join()` | 等待任务完成（返回 `()`） |
| `t.cancel()` | 取消任务（**部分实现**） |
| `t.value` | 等待并返回 `Result<T, Error>` |

## TaskGroup

结构化并发任务组：

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

> **状态：** `TaskGroup` **已实现**（M3 风格；非完整 Swift `withTaskGroup` API）。基础 `add` / `waitAll` / 取消传播 **已实现**。产品面 `taskGroupCancel`（ADR-260）+ `liveTaskCount`（ADR-261）；**A01 叶子 ADR-272 已签**（≠ EC-M3-2）。`setDeadlineMs`（ADR-262）+ `taskLocalSet`/`taskLocalGet`（ADR-263）；**A02 叶子 ADR-273 已签**（≠ `@TaskLocal` / ≠ EC-M3）。`actorReentrancyPolicy`（ADR-264/265：`serial-no-reenter;await-hold`）；**A03 叶子 ADR-274 已签**（≠ Swift await-reentrancy GA / ≠ EC-M3）。

## actor 隔离

`actor` 保护内部状态，外部访问须 `await`：

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

let c = Counter()
await c.increment()                        // 跨隔离必须 await
print(await c.current())                   // 1
```

| 规则 | 说明 |
|------|------|
| 内部方法 | 自动在 actor 隔离区 |
| 外部访问 | 必须 `await`（隔离违规 → **A002**） |
| 同步同实例重入 | **禁止**（ADR-264）：mailbox handler 内再同步调用同实例 → **C009**（`serial-no-reenter`） |
| handler 内 await/sleep | **await-hold**（ADR-265）：不中途处理同 mailbox 其他消息；**非** Swift await-reentrancy GA |
| `nonisolated func` | 显式标记非隔离（**部分实现**） |
| `actor` 不可继承 | 顶层 `class` 不可继承 `actor` |
| `Sendable` 字段 | actor 持有的 `Sendable` 值可安全跨任务 |

### Sendable

跨隔离传递的类型须 `Sendable`：

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
await cache.put(u)                         // 跨隔离传 Sendable 值
let arr = await cache.get()                // 接收 Sendable 值数组
```

| 注解 | 说明 |
|------|------|
| `@Sendable` | 标记类型可安全跨隔离传递 |
| `@unchecked Sendable` | 抑制检查，自担数据竞争风险（**部分实现**） |
| 内置 `Sendable` | `Int` / `String` / `Array<T: Sendable>` 等自动 |

> **Phase 60 严格模式：** `Sendable` 检查在 `strict` 模式下为编译错误 **S019** / **C003** E2E。

## GlobalActor / @MainActor

将一组声明绑定到全局 actor：

```syx
global actor MainActor { }                 // 全局 actor 定义

@MainActor
func updateUI() {
    // 跑在 MainActor 上
}

@MainActor
class ViewModel {
    var title: String = ""
    func setTitle(_ s: String) { title = s } // 隔离在 MainActor
}
```

- 全局 actor 跨平台：UI 线程、GPU 线程等
- Phase 34 `GlobalActor v14` 支持全球多活 CDN 路由与跨区域健康探针
- 跨 GlobalActor 调用须 `await`

## Channel

跨任务通信。产品面在 `import Sync` 的 `Channel` 类（非 `__syx_channel_*`）：

| 写法 | 含义 |
|------|------|
| `Channel()` | 默认 `init`：无界通道（capacity `0`） |
| `Channel.create(n)` | 工厂方法：`n > 0` 有界，`n == 0` 无界；返回类型为 **`Channel`**（不是裸 `Object`） |

`Channel()` 与 `Channel.create(n)` 的区别：前者是类的默认构造；后者是带容量参数的工厂，语义上等价于「带参构造」，因当前 `init()` / `init(capacity:)` 重载会误解析，有界通道请用 `create`。

实例方法：`send` / `recv` / `trySend` / `tryRecv` / `close` / `count()` / `capacity()`。静态工厂仅保留 `Channel.create(n)`。

> 教程里的 `Channel<Int>` 泛型语法为规划形态；当前 MVP 载荷为 `Object`，类型由调用约定约束。

```syx
import Sync

let ch = Channel.create(2)
ch.send(1)                                 // 默认同步
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

| 操作 | 同步 / 异步 | 说明 |
|------|------------|------|
| `ch.send(v)` | 阻塞 / `await` | 发送 |
| `ch.receive()` | 返回 `T?` | 接收；`nil` 表示已关闭 |
| `ch.close()` | 标记关闭 | **ADR-266**：真断开 sender；接收完缓冲后返回 `nil`；再 send → **C010**；**非** backpressure/fairness 满勾 |
| `ch.trySend(v)` | 非阻塞 | **ADR-267**：有界缓冲满 → **false**；`count`/`capacity` 可观测；**非** fairness 满勾 / **非** A04 满勾 |
| `for x in ch` | 异步迭代 | 等同 `while let x = ch.receive()` |
| `ch.count()` / `ch.capacity()` | 方法 | 当前长度 / 容量（ADR-267 背压钉） |

来源：`fixtures/parity/stdlib/channel_send.sy`

## select

多 channel 监听：

```syx
select {
case let v = chA.receive() { print("A: \(v)") }
case let v = chB.receive() { print("B: \(v)") }
case chC.send(42) { print("sent to C") }
}
```

> **状态：** `select` **已实现**（`fixtures/runtime/select_await_case_order.sy`）；N 路 case + default 上下文。

| 合同 | 说明 |
|------|------|
| 多臂同时 ready | **ADR-268**：thread-local round-robin（指纹 `select-fair-v1:round-robin`）；产品钉 `m3_a04_fairness`；**非**跨线程/全局 cursor GA |
| A04 叶子 | **ADR-269**：close / backpressure / fairness 三轴收口（`m3_a04_closeout`）；**非**跨线程 RR GA / **非** EC-M3 |

## distributed actor（部分实现）

```syx
distributed actor ClusterNode {
    var id: String
    init(id: String) { self.id = id }

    distributed func ping() -> String {
        return "pong from \(self.id)"
    }
}
```

- 跨进程 / 跨机的 actor（**MVP spike**）
- `distributed func` 可远程调用
- 完整 distributed actor → 后续 Phase

## 取消

```syx
let t = Task {
    for i in 0..<1000 {
        if (Task.isCancelled) { return }  // 检查
        await doWork(i)
    }
}

// 外部
t.cancel()
```

> **协作式取消：** 任务须显式检查 `Task.isCancelled`；库代码可在 `await` 处自动响应。

## 应用并发（主业务 API）

日常业务代码优先使用结构化并发与消息面，而不是直接调线程池旋钮：

| 表面 | 用途 |
|------|------|
| `Task` / `TaskGroup` / `async`/`await` / `async let` | 结构化任务树（M3-A01 leaf：cancel / liveTaskCount；ADR-272） |
| `setDeadlineMs` / `taskLocalSet`/`Get` | 截止与 task-local（M3-A02 leaf；ADR-273） |
| `actor` / `actorReentrancyPolicy` | Actor 隔离与重入合同（M3-A03 leaf；ADR-274） |
| `Channel` / `select` | 有界通道、close / backpressure / fairness（M3-A04） |

## 高级运行时调优（部署/运维）

> **ADR-270 / ADR-271 / M3-A05 leaf：** 下列 `Concurrency.*` 池与 Actor 容量旋钮是**高级运行时配置**，**不是**普通应用首选并发 API。优先用上一节结构化并发 + 有界 Channel/mailbox。A05 叶子已签（≠ EC-M3 / ≠ 删池 API）。

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

> **状态：** worker / maxActors / mailbox **Phase 133 partial GA**；queue / reject / core-max-keepAlive **Phase 134–136 partial GA**；Actor shared pool **Phase 138 partial GA**。**非** Java `ThreadPoolExecutor` 全集兼容 → **139+**；**非** EC-M3；A05 叶子 **ADR-271** 已签。

### Task 池 vs blocking 池（ADR-275 / ADR-276 / M3-B01）

`Concurrency.blockingSleep` 走独立 `blocking_pool`（指纹 `task-pool+blocking-pool-v1-sized`）；`Concurrency.setBlockingPoolSize` / `blockingPoolSize` 可调固定容量（core=max=n）。现有 `Concurrency.sleep` / `__syx_task_sleep` 仍在调用 Task worker 上阻塞。HTTP/File 等 HostFn **未**改道。B01 叶子 **ADR-277** 已签。**非** epoll/kqueue reactor GA；**非** EC-M3。

`File.readAsync`（ADR-278）与 `TcpStream.readAsync`（ADR-279）走 `blocking_pool` pending+await（指纹见 `Concurrency.ioAdapterPolicy`，现为 `v1-tcp`）；同步 `File.read` / `Http.get` / `TcpStream.read` **未**改道。B02 叶子 **ADR-280** 已签。**非** TcpWrite/Connect async；**非** reactor GA；**非** EC-M3。

### Mutex RAII / blocking（ADR-281 / ADR-282 / ADR-283 / M3-C01）

`Mutex.lockGuard` 返回 `MutexGuard`；作用域退出时 `deinit` 调用 `__syx_mutex_unlock_quiet`。产品 `Mutex.lock` / `lockGuard` 走 `__syx_mutex_lock_wait`（held 时 Condvar wait；ADR-282）。既有 `__syx_mutex_lock` 仍为 try-lock（Bool）。指纹见 `Concurrency.syncPolicy`（`mutex-raii-guard-v1-blocking`）。C01 叶子 **ADR-283** 已签。**非** RWLock；**非** 产品 Condvar；**非** EC-M3。

### Semaphore（ADR-284 / M3-C02）

`Semaphore.create(permits)` / `wait` / `tryWait` / `signal`（计数信号量；独立表）；ADR-285：`SemaphoreGuard` / `waitGuard`（作用域退出 quiet signal）。指纹见 `Concurrency.semaphorePolicy`（`semaphore-v1-raii`）。ADR-286：C02 叶子已签收（Semaphore 切片）。**非** RWLock；**非** 产品 Condvar；**非** EC-M3。

### 内存序 / Atomic（诚实边界）（ADR-287 / ADR-288 / M3-C03）

跨 Task / Actor **共享可变状态**须经 `Mutex` / `Semaphore` / `Channel`（或 Actor 隔离）；这些同步操作在切片定义下建立 **happens-before**。本切片**不**提供语言级 `Atomic*` / `memory_order`（SeqCst/Acquire/Release）产品面；**不**交付 `AtomicInt` / CAS / fetchAdd stdlib API。`__syx_wasm_atomic_add_i32` 已登记但 capability **unsupported**（≠ Wasm atomics 产品 GA）。对齐 Memory Model v1（ADR-137）与 Concurrency Model v1（ADR-139 / Sendable）。ADR-288：C03 叶子已签收（docs 切片）。**非** EC-M3。

### race/stress/chaos CI 抽样（ADR-289 / ADR-290 / ADR-291 / M3-C04）

可复现并发 stress 样例库见 `packages/internal/race_stress_samples.toml`（`kind=stress`：Mutex/Semaphore 争用 + C04 抽样冒烟；`kind=chaos`：TaskGroup cancel 中断 + C04 自有 `m3_c04_chaos`）。CI 接线 `m3_c04_race_stress` / `m3_c04_chaos` / `m3_c04_closeout`。叶子已收口（ADR-291）。诚实释义：regression gate corpus；chaos = 协作式取消/中断抽样。**非** TSAN/loom GA；**非** 杀进程/网络分区 chaos 农场；**非** EC-M3-1 高并发压测总勾。

## 与 Swift 差异

| Swift | SYX |
|-------|-----|
| `actor` 隔离 | **已实现** |
| `async` / `await` | **已实现**（多 await 状态机 GA） |
| `Task { }` | **已实现** |
| `Task.detached` | **已实现** |
| `TaskGroup` | **已实现**（基础 add / waitAll） |
| `async let` | **已实现**（desugar detach+join） |
| Structured Concurrency | **MVP**（非完整 Swift 全集） |
| `GlobalActor` / `@MainActor` | **已实现** |
| `Sendable` | **已实现**（strict 模式 C003 E2E） |
| `Channel` | **已实现** |
| `select` | **已实现** |
| `AsyncSequence` `for await in` | **已实现** |
| `distributed actor` | **partial** MVP spike |
| `Cancellation` | **协作式**（部分实现） |
| Java `ThreadPoolExecutor` 全集 | **未实现**（**139+**） |

## 完整示例

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

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S020** | 非 `async` 函数使用 `await` | 加 `async` |
| **A002** | 跨 actor 访问未 `await` | 加 `await` |
| **C001** | actor 隔离违规 | 移到 actor 隔离区 |
| **C003** | 跨隔离传非 `Sendable` | 加 `@Sendable` / `@unchecked Sendable` |
| **R007** | `await ch.receive()` 解包 nil | 检查 `for-in` 或 `while let` |

---

**下一章** → [宏](19-macros.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [UsageGuide 并发控制](../../../docs/9_UsageGuide.md) · [错误处理](17-error-handling.md)
