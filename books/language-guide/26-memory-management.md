# 内存管理与垃圾回收

**实现状态：** **ARC** for class（Phase 73 · class `ObjectRef` retain/release · VM + WASM tier-2D + Native LLVM 直发）；**ManagedHeap v4 分代追踪 GC** 统一 VM / WASM / Native（Phase 75）；Inkwell LLVM default on（Phase 77）；`syx-rt` 直发 tier-3（Phase 77）；Borrow 借用检查（Phase 74 · SE-0461 + async/actor 子集）

SYX 内存模型是 **ARC + ManagedHeap 分代追踪 GC** 双轨：class 走 ARC 引用计数（与 Swift 类似的语义，但底层由 host 桥层管理），其余堆对象（Array / String / Dictionary / 闭包等）走 **统一分代追踪 GC**。本章讲清三后端（VM / WASM / Native）的内存协作、`gc` 与 `deinit` 的关系、borrow 借用检查、生命周期标注。

## 内存模型总览

| 后端 | class 引用 | 堆对象（Array/String/Dict/闭包） | 释放模型 |
|------|------------|-----------------------------------|----------|
| **VM** | ARC（class `ObjectRef` retain/release） | ManagedHeap v4 分代 GC | 强引用归零 → `deinit`；堆对象 → GC |
| **WASM** | ARC（tier-2D） | ManagedHeap v4 分代 GC（**Phase 75 统一**） | 同上 |
| **Native (LLVM)** | ARC（tier-3 直发） | ManagedHeap v4 分代 GC（统一屏障） | 同上 |

> **统一 GC LTS（Phase 75）：** VM、WASM、Native 三后端共享 ManagedHeap v4 compact + HeapSlot Array/String 统一实现；mark-sweep + slide compact。

## ARC（class 引用语义）

class 实例是 **堆分配 + 引用计数**。`ObjectRef` 跟踪 retain / release：

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

> **deinit 触发：** refcount 归零时调用 `Type_deinit`（class 显式 deinit）。详见 [第 16 章 析构过程](../../docs/adr/architecture-decisions.md)。

| 特性 | 状态 |
|------|------|
| class `ObjectRef` | **已实现**（Phase 73） |
| retain / release | **已实现** |
| ARC 归零 → deinit | **已实现**（Phase 102/103 MVP） |
| `weak var` 完整 | **部分实现**（词法保留，归零判定演进中） |
| `unowned var` | **部分实现** |

## ManagedHeap 分代追踪 GC

非 class 堆对象（`Array<T>` / `String` / `Dictionary<K,V>` / 闭包）走 **分代追踪 GC**：

| 机制 | 状态 |
|------|------|
| Mark-sweep | **已实现**（Phase 73） |
| 分代 young/old | **已实现**（Phase 75 统一） |
| Promotion | **已实现** |
| Remembered set / card table | **已实现**（三后端联动） |
| Slide compact | **已实现**（Phase 75 v4） |
| Relocating GC | **已实现**（Phase 37 ADR-037） |
| Write barrier | **已实现**（跨代写入触发 remember update） |

> **三后端联动：** ADR-045（Phase 34）将 VM / WASM / Native 的 GC 屏障统一；MIR v23 LICM 统一三后端屏障放置。

```syx
func makeLargeArray() -> Array<Int> {
    var arr: Array<Int> = []
    for i in 0..<10_000 { arr.append(i) }
    return arr                              // young gen 分配；出作用域由 GC 回收
}
```

### GC 配置（环境变量）

| 变量 | 效果 |
|------|------|
| `SYX_UNIFIED_GC_LTS=1` | 启用 Unified GC LTS（默认 prod） |
| `SYX_UNIFIED_GC_LTS_PROD=1` | 强制 prod 模式 |
| `SYX_WASI_GC_PHASE33=1` | 启用 WASI 分代 GC（Phase 33） |
| `SYX_VM_NATIVE_GC_PHASE33=1` | 启用 VM/native 分代 GC（Phase 33） |
| `SYX_GC_NURSERY=1048576` | nursery 大小（字节，默认 1 MB） |

### 何时触发 GC

- **自动：** 分配速率 / 堆占用阈值 / nursery 满
- **手动：** `import Runtime; Runtime.gc()` 强制回收（**部分实现**）
- **作用域退出：** ARC 归零立即触发 `deinit`；堆对象延迟 GC

## `deinit` 与 GC

| 类型 | 释放模型 | `deinit` 时机 |
|------|----------|---------------|
| `class` | ARC | refcount 归零 |
| `actor` | ARC + 隔离 | refcount 归零 + actor 关闭 |
| `struct` | 栈 | 无（值类型） |
| `enum` | 视 case | 视 case |
| `Array<T>` 等 | GC | GC 回收时（不确定时机） |

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

## 借用 &T / &mut T

借用是编译期**引用安全**机制（不涉及运行时）：

```syx
func read(x: &Int) -> Int { return x }      // 不可变借用
func write(x: &mut Int) { x = x + 1 }        // 可变借用

func main() {
    var n = 10
    print(read(x: &n))                       // 10
    write(x: &mut n)
    print(n)                                 // 11
}
```

| 形式 | 含义 |
|------|------|
| `&T` | 不可变借用（可同时多个） |
| `&mut T` | 可变借用（同时只能一个） |
| `&'a T` | 带生命周期标注（**部分实现**） |

> **借用规则：** 同一时刻不能同时有可变借用与任何其他借用。违反 → **S011** 编译错误。

```syx
var n = 10
let r1 = &mut n
// let r2 = &n                              // ❌ S011：cannot borrow as immutable while mutably borrowed
// print(n)                                 // ❌ S011：cannot access through mutable borrow
r1 = &mut n
```

**诊断：** S011（借用/变异违规）

## 生命周期 'a

```syx
func id<'a>(x: &'a String) -> &'a String { return x }
```

| 能力 | 状态 |
|------|------|
| 生命周期标注 | **部分实现**（S017） |
| NLL（Non-Lexical Lifetimes） | **已实现** |
| 跨 await 借用 | **部分实现** |
| 高阶生命周期 | **未实现** |

测试：`p15_lifetime_a_*.sy` · `p17_lifetime_await_*.sy`

## Native / LLVM 后端

Phase 75 起 **Inkwell Prod GA**（[ADR-044](../../docs/adr/architecture-decisions.md)）将真 inkwell crate IR builder 设为默认 native 编译路径：

- 统一分代 GC barriers 直出 native
- ARC class retain/release 走 `__syx_object_*` 直发
- `syx-rt` 静态库提供 host 桥层

**Phase 77 增强：**

- `materialize_array` / `materialize_dict` 走 HeapSlot
- `rt_direct_exec` 栈操作直发
- `reflect_specialization` 泛型特化反射 MVP
- obligation 导出（borrow 机器证明）

## 工具链缓存

- `target/cache/` MIR 增量编译（`syx-cache-v110`）
- 编译期行为异常时删除缓存后重试

## 与 Swift 的差异

| Swift ARC | SYX |
|-----------|-----|
| 引用归零即 deinit | **部分实现**（scope-exit + ARC 归零 MVP） |
| `weak` / `unowned` 完整 | **部分实现**（词法保留，归零判定演进） |
| `autoreleasepool` | **未实现**（用 GC 替代） |
| 完整 `move` 语义 | **部分实现**（borrow regions v8） |
| Swift 6 严格并发 | **部分实现**（Sendable / actor / `@MainActor`） |

## 完整示例

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
    process(b: &b)                           // 不可变借用

    // Array 元素修改走 GC write barrier
    for i in 0..<b.data.count {
        b.data[i] = b.data[i] + 1
    }
    // 局部变量出作用域 → ARC -1 → deinit
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S003** | 对 `let` 借用再赋值 | 改 `var` |
| **S011** | 同时可变 + 不可变借用 | 调整作用域 |
| **S017** | 生命周期违规 | 显式标注或重组代码 |
| **R011** | 整数溢出 | 检查边界 |
| **R016** | 必填字段未初始化 | 完整 init |

## 进一步阅读

- [第 27 章 内存安全](../../docs/adr/architecture-decisions.md) — 借用机器证明 / Sendable
- [第 16 章 析构过程](../../docs/adr/architecture-decisions.md) — deinit 触发
- [第 18 章 并发](../../docs/adr/architecture-decisions.md) — actor 隔离 / Sendable
- [docs/10_BackendParity.md](../../docs/10_BackendParity.md) — VM / MIR / LLVM / WASM 内存对齐
- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-045（统一 GC LTS）等

---

**下一章** → [内存安全](../../docs/adr/architecture-decisions.md)

**参见** → [第 16 章 析构过程](../../docs/adr/architecture-decisions.md) · [第 18 章 并发](../../docs/adr/architecture-decisions.md) · [docs/10_BackendParity.md](../../docs/10_BackendParity.md) · [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md)
