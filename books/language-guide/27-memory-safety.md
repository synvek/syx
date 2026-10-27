# 内存安全

**实现状态：** 借用检查（**已实现** S011）；机器证明 harness（**Phase 32**）；Coq/Lean 外部证明 scaffold（**Phase 33**）；机器检验证明 prod（**Phase 34**）；Actor 隔离（**已实现** A002/C001）；`@unchecked Sendable`（**部分实现**）

内存安全防止 **数据竞争、悬垂引用、非法变异**。SYX 通过 **编译期借用检查 + 运行时 GC 屏障 + actor 隔离 + Sendable 约束** 四层保护构建安全网。

## 借用检查（编译期）

不可同时存在可变借用与共享借用（简化 NLL）：

```syx
func read(x: &Int) -> Int { return x }
func write(x: &mut Int) { x = x + 1 }

func main() {
    var n = 10
    let r1 = &n                              // 共享借用
    print(read(x: r1))                       // 10
    r1 = nil                                  // 释放共享借用

    let r2 = &mut n                          // 可变借用
    write(x: r2)                              // 合法
}
```

**S011** — Cannot mutate / borrow conflict

```syx
var n = 10
let r1 = &mut n                              // 可变借用
// let r2 = &n                              // ❌ S011：cannot borrow as immutable
// print(n)                                 // ❌ S011：cannot access through mutable borrow
r1 = nil                                     // 释放可变借用
let r3 = &n                                  // ✅ 现在可共享借用
```

测试：`p6_borrow_*.sy` · `p23_borrow_regions_v5_*.sy`

### 借用规则总结

| 状态 | 可变借用 | 共享借用 | 读访问 | 写访问 |
|------|----------|----------|--------|--------|
| 无借用 | ✅ | ✅ | ✅ | ✅ |
| 活跃可变借用 `&mut` | ❌ S011 | ❌ S011 | ❌ S011 | ❌ S011 |
| 活跃共享借用 `&` | ❌ S011 | ✅ | ✅ | ❌ S011 |

> **NLL（Non-Lexical Lifetimes）：** 借用作用域在「最后一次使用」结束，**不** 到词法末尾。简化模式让 Rust 风格借用更易写。

## 机器证明 harness（Phase 32+）

Phase 32 引入 **borrow 机器证明 harness**（`borrow_regions_machine_proof_enabled()`）：编译器将 region 图导出为 **proof obligation**，供 replay smoke 与外部证明工具链接。

| 阶段 | 能力 |
|------|------|
| Phase 31 | 形式化验证 scaffold（region 图 proof smoke） |
| Phase 32 | **机器证明 harness** — region 图 → proof obligation 导出 + replay |
| Phase 33 | **Coq/Lean 外部证明 scaffold** — obligation → `.v`/`.lean` 导出 + replay |
| Phase 34 | **机器检验证明 prod** — obligation replay + 外部 Coq/Lean 工具 hook prod |

启用：

```bash
SYX_BORROW_MACHINE_PROOF=1 syx run program.sy
SYX_BORROW_COQ_LEAN_EXPORT=1 syx run program.sy        # Phase 33 Coq/Lean 导出
SYX_BORROW_COQ_LEAN_PROOF_PROD=1 syx run program.sy    # Phase 34 机器检验 prod
```

测试 fixture：`p32_borrow_proof_*` · `p33_borrow_coq_lean_*` · `p34_borrow_coq_lean_proof_*`

## 生命周期

```syx
func longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if (x.count > y.count) { return x }
    return y
}
```

| 能力 | 状态 |
|------|------|
| 生命周期标注 `<'a>` | **部分实现**（S017） |
| 生命周期推断 | **已实现** |
| 跨 `await` 借用 | **部分实现**（async/actor 子集） |
| 高阶生命周期 | **未实现** |
| 生命周期 bound `'a: 'b` | **部分实现** |

测试：`p15_lifetime_a_*.sy` · `p17_lifetime_await_*.sy`

## Actor 隔离

actor 外直接访问 actor 状态 → **A002** / **C001**：

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
}

let c = Counter()
// print(c.value)                            // ❌ A002：cross-actor access without await
print(await c.value)                        // ✅ 跨隔离必须 await
```

详见 [第 18 章 并发](18-concurrency.md#actor-隔离)。

## Sendable

跨隔离传递类型须 `Sendable`：

```syx
@Sendable
data class User(name: String, age: Int)

actor Cache {
    var users: Array<User> = []
    func put(_ u: User) { users.append(u) } // User 满足 Sendable
}

let cache = Cache()
let u = User(name: "Alice", age: 30)
await cache.put(u)                          // 跨隔离传 Sendable 值
```

| 注解 | 状态 | 说明 |
|------|------|------|
| `@Sendable` | **已实现** | 标记类型可安全跨隔离传递 |
| `@unchecked Sendable` | **部分实现** | 抑制检查；自担风险 |

> **Phase 60 严格模式：** `Sendable` 检查在 `strict` 模式下为编译错误 **C003** E2E（`concurrency.rs`）。

```syx
// 非 Sendable 跨 actor → 编译错误
class NotSendable {
    var mutable: Int = 0                    // 引用类型 + 内部可变
}

let bad = NotSendable()
// await cache.put(bad)                     // ❌ C003：NotSendable not Sendable
```

## @unchecked Sendable

```syx
@unchecked Sendable
class MyLegacy {
    var internal: Int = 0
}
```

> **风险：** `@unchecked` 抑制检查；多线程同时访问需外部同步（如 `Lock` / `actor`），否则可能数据竞争。

## GC 与内存安全

**Unified GC LTS**（[ADR-045](../../docs/adr/architecture-decisions.md)）在三后端 VM/WASM/native 统一 remembered set 分代 GC，保证跨代引用在 minor collect 时不遗漏；与 borrow 检查互补 — 编译期借用 + 运行时 GC 屏障。

| 维度 | 编译期 | 运行时 |
|------|--------|--------|
| 借用检查 | ✅ S011 | N/A |
| 生命周期 | ✅ S017 | N/A |
| 跨代引用屏障 | N/A | ✅ write barrier |
| Actor 隔离 | 类型检查 A002 | queue serial |
| Sendable | 类型检查 C003 | witness |

## 完整示例

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

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S011** | 同时可变 + 不可变借用 | 调整作用域 |
| **S017** | 生命周期违规 | 显式标注 / 重组代码 |
| **A002** | 跨 actor 访问未 await | 加 `await` |
| **C001** | actor 隔离违规 | 移到 actor 内 |
| **C003** | 跨隔离传非 Sendable | 加 `@Sendable` / `@unchecked Sendable` |
| **S019** | Sendable channel | 用 Sendable 元素类型 |

## 与 Swift 的差异

| Swift 6 严格并发 | SYX |
|-----------------|-----|
| Sendable 检查 | **已实现**（strict 模式 C003 E2E） |
| Actor 隔离 | **已实现**（A002/C001） |
| 借用检查 | **已实现**（S011） |
| 生命周期 | **部分实现** |
| 机器证明 | **部分实现**（Phase 32+ scaffold） |
| 全局 actor | **已实现**（`@MainActor`） |
| `sending` 参数 | **已实现**（Phase 26） |
| 数据竞争静态检测 | **部分实现** |

## 进一步阅读

- [第 26 章 内存管理与垃圾回收](../../docs/adr/architecture-decisions.md) — ARC / 分代 GC / 借用
- [第 18 章 并发](../../docs/adr/architecture-decisions.md) — actor / async / Task
- [第 16 章 析构过程](../../docs/adr/architecture-decisions.md) — deinit / 资源释放

---

**下一章** → [访问控制](../../docs/adr/architecture-decisions.md)

**参见** → [第 26 章 内存管理](26-memory-management.md) · [第 18 章 并发](18-concurrency.md) · [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) · [plan/topics/gc-parity.md](../../plan/topics/gc-parity.md)
