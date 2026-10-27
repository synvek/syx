# 析构过程

**实现状态：** `class` / `actor` 显式 `deinit` **部分实现**（Phase 102/103 weak ARC deinit MVP）；`struct` deinit **不支持**；`isolated deinit` **未实现**

析构器在实例释放前执行清理逻辑。SYX class 采用 **ARC 引用计数** 触发归零 → `deinit`；struct / enum **没有** `deinit`（值类型在栈上自动回收）。

## deinit 语法

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
}                                          // 输出："cleanup: r1"
```

- `deinit` 只能在 `class` / `actor` 内声明
- 无参数、无返回
- 多个 `deinit` 在 override 链中按层级调用（自底向上）

## 当前限制

| 限制 | 状态 | 说明 |
|------|------|------|
| class 显式 `deinit` | **已实现**（Phase 102/103 MVP） | ARC 归零触发 |
| actor `deinit` | **已实现** | actor 释放时 |
| struct `deinit` | **未实现** | struct 无堆分配 |
| enum `deinit` | **未实现** | 值类型 |
| `isolated deinit` | **未实现** | actor 隔离内析构 |
| 弱引用触发归零 | **部分实现** | `__syx_weak_*` hosts registered; alive-before-drop semantics **not fully green** (ADR-137) |
| `super.deinit` 自动调用 | **已实现** | override 链自动 |

## ARC 归零触发

class 实例在 strong refcount 归零时触发 `deinit`（**Phase 73 起**）：

```syx
class Counter {
    static var count: Int = 0
    init() { Counter.count = Counter.count + 1 }
    deinit { Counter.count = Counter.count - 1 }
}

do {
    let a = Counter()
    let b = Counter()
    print(Counter.count)                   // 2
}                                          // a, b 释放
print(Counter.count)                       // 0
```

> **scope-exit 路径：** 当前实现也支持 scope-exit 触发（局部变量出 `{}` 立即执行 deinit，与 ARC 归零并存）。

## 与 Swift ARC 的差异

Swift 的 ARC 是 **确定性** 的：refcount 归零时立即 `deinit`。SYX 当前是 **部分确定性**：

| 场景 | Swift | SYX |
|------|-------|-----|
| 局部 `let` 出作用域 | 立即 deinit | scope-exit 立即（部分路径） |
| 跨函数引用的 `let` | 立即归零 | ARC 归零（接近立即） |
| 循环引用 | 内存泄漏 | 同样泄漏（无 weak 全集） |
| `weak` 引用 | 归零时自动 nil | **部分实现**（hosts registered；语义钉未全绿） |
| `unowned` 引用 | 强引用（无安全网） | 词法保留，**未完整** |

## 替代方案：显式 close

确定性资源（文件、socket、锁）推荐用 **显式 close / release**：

```syx
class File {
    let path: String
    var handle: Any? = nil
    init(path: String) { self.path = path }

    func open() { self.handle = FileHandle.open(path, "r") }
    func close() {
        if let h = handle {
            FileHandle.close(h)
            self.handle = nil
        }
    }
    deinit { close() }                     // 兜底
}

func main() {
    let f = File(path: "/tmp/x")
    f.open()
    f.close()                              // 显式释放
}
```

> **建议：** 任何**关键资源**（文件 / socket / 锁 / GPU buffer）都应有 `close()` / `release()` 显式方法，`deinit` 仅作「兜底」。

## 弱引用

```syx
class Node {
    let value: Int
    weak var parent: Node?                  // 词法保留；**未完整**实现归零语义
    var children: Array<Node> = []

    init(value: Int) { self.value = value }
}
```

| 关键字 | 状态 | 说明 |
|--------|------|------|
| `weak var` | **部分实现** | `__syx_weak_*` host path; full field semantics **140+** |
| `unowned var` | 词法保留，**未完整** | 强引用，无安全网 |

> **140+ 计划：** 完整 field-level weak / unowned 语义（Model v1 frozen in [ADR-137](../../../docs/adr/architecture-decisions.md)).

## deinit 与 actor 隔离

actor 内部状态在 actor 隔离上下文中，`deinit` 触发时 actor 本身正在释放；当前实现**不**保证 `deinit` 跑在原 actor 上：

```syx
actor Counter {
    var value: Int = 0
    init() { value = 0 }

    func increment() { value = value + 1 }
    deinit { print("counter bye, last value=\(value)") }
}
```

`isolated deinit`（在原 actor 上执行 deinit）**未实现**；deinit 默认跑在调用方线程。

## deinit 中能做什么

```syx
class Cleanup {
    var name: String
    init(name: String) { self.name = name }

    deinit {
        // ✅ 读 self 属性
        print("bye \(self.name)")

        // ❌ 不能写 stored 属性（half-deinitialized）
        // self.name = "x"

        // ✅ 可以发通知 / 记录日志
        // ✅ 可以调全局函数
        // ❌ 不能调自己的实例方法（half-deinitialized）
    }
}
```

## deinit 与 override

```syx
class Base {
    deinit { print("base") }
}

class Derived: Base {
    deinit { print("derived") }
}

let d = Derived()
print("use d")
// 释放时先 print "derived" 再 print "base"（override 链自动调用 super.deinit）
```

> **重写 deinit 不需要 `override`：** `deinit` 关键字本身表达「实现析构」，无 override 标记。

## 完整示例

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
}                                          // l1, l2 释放
print("alive=\(Logger.total)")             // 0
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **P001** | 顶层 `deinit` | 移到 class / actor 内 |
| **S004** | `deinit` 带参数 | 删参数（无参） |
| **R007** | deinit 中 `!` 解包 nil | 用 `if let` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 确定性 `deinit` | **部分实现**（scope-exit + ARC 归零 MVP） |
| `weak` / `unowned` | 词法保留，**未完整** |
| `isolated deinit` (actor) | **未实现** |
| struct `deinit` | **未实现**（值类型无堆分配） |
| `super.deinit` 显式 | **自动**（无需显式调用） |
| `deinit` 写 stored 属性 | **禁止**（half-deinitialized） |

---

**下一章** → [错误处理](17-error-handling.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 15 章 构造过程](15-initialization.md) · [第 26 章 内存管理与垃圾回收](26-memory-management.md)
