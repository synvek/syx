# SYX 语言快速参考

面向 **`.sy` 源码作者** 的语法与标准库速查。与 `syx-parser` / `syx-semantic` 当前能力对齐；工具链见 [9_UsageGuide.md](./9_UsageGuide.md)。

> **版本：** SYX **117.0** · Phase **139** · `syx-cache-v1` · 许可证 [Apache-2.0](../LICENSE)  
> **原生 AOT：** 仅 LLVM · **Host 全局：** [`packages/core/builtins/special/globals.sy`](../packages/core/builtins/special/globals.sy)（`print` / `readLine` / `reflect`）  
> **实现状态 SSOT：** [books/shared/implementation-status.md](../books/shared/implementation-status.md)  
> 历史 Phase 条幅已移除；细节以 status / ADR 为准。

---

## 目录

1. [类型系统](#类型系统)
2. [变量与入口](#变量与入口)
3. [控制流](#控制流)
4. [函数与访问控制](#函数与访问控制)
5. [模块](#模块)
6. [字符串](#字符串)
7. [数组](#数组)
8. [错误处理 try/catch](#错误处理-trycatch)
9. [Optional 空安全](#optional-空安全)
10. [Data Class](#data-class)
11. [Extension](#extension)
12. [Sealed Class 与 Switch](#sealed-class-与-switch)
13. [Effects 效果注解](#effects-效果注解)
14. [标准库 File / Directory / HTTP](#标准库-file--directory--http)
15. [Async / Await（MVP）](#async--awaitmvp)
16. [借用与宏（Phase 7）](#借用与宏phase-7)
17. [CLI 速查](#cli-速查)
18. [诊断代码](#诊断代码)

---

## 类型系统

### 数值类型

| 类型 | 示例 | 说明 |
|------|------|------|
| `Int8` … `Int64` | `42i8`, `1000i64` | 固定宽度有符号整数 |
| `Int128` | `42i128` | 128 位有符号整数（Phase 38） |
| `UInt8` … `UInt64` | `255u8` | 固定宽度无符号整数 |
| `UInt128` | `0xFFu128` | 128 位无符号整数（Phase 38） |
| `SIMD4<Int32>` 等 | `SIMD4.fromArray([1,2,3,4])` | 固定宽度 SIMD 向量（Phase 38） |
| **`Int`** | `42` | **平台字长**有符号整数（64 位 host → `Int64`，32 位 → `Int32`） |
| **`UInt`** | `100u` | **平台字长**无符号整数 |
| `Float32` / `Float` | `3.14f32` | 32 位浮点 |
| `Float64` / `Double` | `3.14` | 64 位浮点（无后缀浮点默认 `Double`） |

无后缀整数字面量默认类型为平台 **`Int`**。显式后缀（如 `42i8`）选择固定宽度类型。

### 基础与集合类型

| 类型 | 示例 | 说明 |
|------|------|------|
| `Bool` | `true` / `false` | 布尔 |
| `String` | `"hello"` | 字符串 |
| `Array<T>` | `[1, 2, 3]` | 动态数组 |
| `Set<T>` | `Set<Int>`、`[1,2,3]` 赋值给 `Set` | 去重集合 |
| `[K: V]` / `Dictionary<K,V>` | `["a": 1]`、`[String: Int]` | 静态字典（MVP 键须 `String`） |
| `Object` | `let o: Object = { "k": v }` | 动态异质袋（须标注；未标注字典→`Dictionary`） |
| `T?` | `nil` / `42` | 可空类型 |
| `Void` | — | 无返回值 |

**类型标注：** `let x: Int = 10`、`let m: [String: Int] = ["a": 1]`、`let s: Set<Int> = [1, 2, 3]`

> **注意：** SYX 使用 `func` 关键字与 `print()` 内置输出；**不是** Swift 的 `func`/`print` 全兼容子集。`class` / `struct` / `protocol` 可解析，运行时支持有限；日常推荐 `data class` + `extension`。

---

## 变量与入口

```syx
var count = 0      // 可变
let name = "SYX"   // 不可变
var x: Int = 10    // 显式类型

func main() {      // 程序入口
    print("Hello")
}
```

---

## 控制流

```syx
// if — 条件需括号
if (x > 0) {
    print("positive")
} else {
    print("non-positive")
}

// while — 条件需括号
while (x > 0) {
    x = x - 1
}

// for-in — Array / String / readLines 结果
for item in [1, 2, 3] {
    print(item)
}
for ch in "abc" {
    print(ch)
}

// switch — case 后冒号；支持范围与 variant
switch value {
case 1:
    print("one")
case 0..<10:
    print("small")
default:
    print("other")
}
```

| 构造 | 语法要点 |
|------|----------|
| `if` / `while` | 条件必须 `(expr)` |
| `for-in` | `for name in iterable { }`，无括号 |
| `switch` | `case pat:` 或 `default:`，花括号包裹整体 |
| `break` / `continue` | 支持 |

---

## 函数与访问控制

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

| 修饰符 | 可见性 |
|--------|--------|
| `public` | 跨模块可 import |
| `internal` | 模块内（默认） |
| `private` | 文件/模块内私有 |

**参数：** `func f(a: Int, b: Int)` — 调用 `f(a: 1, b: 2)`。无 Swift 式 `_` 省略标签语法。

---

## 模块

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

| 规则 | 说明 |
|------|------|
| 导入 | `import moduleName`（模块名对应 `.sy` 文件或 `syx.toml` 依赖键） |
| 导出 | 仅 `public func` 可被其他模块调用 |
| 跨模块 internal | 编译错误 **S005** |
| 项目依赖 | `syx.toml` 中 `[dependencies]` + `syx add` / `syx update` |

---

## 字符串

```syx
let name = "World"
print("Hello, \(name)!")                    // 插值

"a,b,c".split(",")                          // → Array
"  hi  ".trim()
"ABC".lowercased() / "abc".uppercased()
"file.txt".hasPrefix("file")
"hello".substring(0, 3)                     // [start, end)
"foo".replacingOccurrences("bar", "foo")    // (new, old)
"abc123".matches("\\d+")                    // 正则 → Array
"text".count                               // 字符数
```

| 方法 | 返回 |
|------|------|
| `.split(sep)` | `Array` |
| `.matches(pattern)` | `Array`（匹配串） |
| `.replacingOccurrences(with, of)` | `String` |

---

## 数组

```syx
let nums = [10, 20, 30]
print(nums.count)

var sum = 0
for x in nums {
    sum = sum + x
}
```

| 方法 | 说明 |
|------|------|
| `.count` | `Int` |
| `.contains(item)` | `Bool` |
| `.sorted()` | `Array`（升序） |
| `.reverse()` | `Array` |
| `.joined(sep)` | `String` |
| `.map` / `.filter` / `.reduce` | ✅ 支持闭包（见 `p4_hof.sy`） |

### Set 与 Dictionary

```syx
let scores: [String: Int] = { "a": 1, "b": 2 }
print(scores.count)          // 2

var tags: Set<Int> = [1, 2, 2, 3]
print(tags.count)            // 3（去重）
print(tags.insert(4))        // true（ADR-224 write-back）
print(tags.contains(4))      // true
print(scores.containsKey("a")) // true
```

| 类型 | 方法 | 返回 |
|------|------|------|
| `Set` | `.count` / `.contains` / `.insert` / `.remove`（mutation 钉 ADR-224）；`.union` / `.intersection` | Bool / Int |
| `[String: V]` | `.count` / `.containsKey` / `.remove` / `.keys` / `.values` | 见 stdlib |

### 范围与 readLine

```syx
print((1..<10).contains(5))
let line = readLine()
```

---

## 错误处理 try/catch

SYX 使用 **`try { } catch (name) { }`**，不是 Swift 的 `do-catch` / `throws`。

```syx
// 语句块
try {
    File.write("/tmp/x.txt", "data")
    let content = File.read("/tmp/x.txt")
    print(content)
} catch (err) {
    print("failed")
}

// 表达式级 try（失败走运行时错误路径）
let text = try File.read(path)
```

Host 函数（File、HTTP 等）失败时抛入 VM 异常，由 `catch` 捕获。

---

## Optional 空安全

```syx
let a: Int? = nil
print(a ?? 99)              // nil 合并

let b: Int? = 7
print(b!)                   // 强制解包；nil → R007

let s: String? = "hello"
print(s?.hasPrefix("he"))   // 可选链；nil 短路

if let y = b {               // 绑定非 nil
    print(y)
}

guard let z = b else {       // 失败走 else 分支
    return
}
print(z)
```

| 运算符 | 含义 |
|--------|------|
| `T?` | 可空类型 |
| `nil` | 空值 |
| `??` | 左非 nil 取左，否则取右 |
| `!` | 强制解包 |
| `?.` | 可选链式成员调用 |

---

## 协议与相等性（Equatable / Hashable）

内置类型自动实现 `Equatable`、`Hashable`、`CustomStringConvertible`、`CustomDebugStringConvertible`（数值、Bool、String、Array、Set、Dictionary、Object）。

| 运算符 / 成员 | 含义 |
|---------------|------|
| `==` / `!=` | 结构相等（数值用 promotion；集合深度相等） |
| `===` / `!==` | 引用标识相等（class 实例比 `__object_id`；值类型同 `==`） |
| `.description` | 用户可读字符串 |
| `.debugDescription` | 调试字符串（含类型前缀） |
| `hash(into:)` | 内置 witness，经 `Hasher` host 组合 |

| 类型 | 说明 |
|------|------|
| `Hasher` | `Hasher.newHasher()` / `Hasher.combine` / `Hasher.finalize` |
| `ObjectIdentifier` | `Hasher.objectId(obj)` |

不可 `Equatable` 的类型使用 `==` 会报 **S016**。

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

| 能力 | 状态 |
|------|------|
| 构造 `Type(a, b)` | ✅ |
| 字段访问 `p.x` | ✅ |
| `.description()` | ✅ |
| 自动 `copy` / `==` / `hash(into:)` | ✅ 编译器合成（`{Type}_copy` / 结构相等 / `hash(into:)`） |

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

## Sealed Class 与 Switch

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

变体定义**无** `case` 关键字；`switch` 的 `case` 分支使用 `.variant(let binding)` 等形式（见 [2_SyntaxAnalysis.md](2_SyntaxAnalysis.md) §5.5）。

| 模式 | 示例 |
|------|------|
| 整型 | `case 42:` |
| 半开范围 | `case 0..<13:` |
| Variant | `case .ok(let x):` |

---

## 扩展语法（§11 已实现）

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

func id<T>(x: Int) -> Int { return x }  // 单态化为 id_Int

@available(iOS 13)
@Main
func app() { print("hi") }

let evens = [1,2,3].filter({ |x| x % 2 == 0 })
let sum = [1,2,3].reduce({ |a, b| a + b })

let base = 10
let mapped = [1].map({ |x| x + base })  // 闭包捕获

throw "failed"
repeat { print(1) } while (false)
x += 1
```

详见 [2_SyntaxAnalysis.md](2_SyntaxAnalysis.md) §5 / §11。

---

## Effects 效果注解

函数声明可在返回类型后附加 **`requires Pure`** 或 **`requires IO`**；Pure 函数内禁止 I/O（含 `print`、File/HTTP）。

```syx
func compute(x: Int) -> Int requires Pure {
    return x * 2
}

func loadConfig() -> String requires IO {
    return File.read("config.txt")
}
```

违反 Pure 约束 → 编译期错误（`syx-meta`）。

---

## 标准库 File / Directory / HTTP

通过 **类型静态方法** 与 **实例方法** 调用（Host Functions 实现）。

### File

| 方法 | 说明 |
|------|------|
| `File.read(path)` | 读全文 → `String` |
| `File.readLines(path)` | 按行 → `Array` |
| `File.write(path, content)` | 写入 |
| `File.append(path, content)` | 追加 |
| `File.exists(path)` | → `Bool` |
| `File.delete(path)` | 删除 |
| `File.copy(from, to)` | 复制 |
| `File.move(from, to)` | 移动 |

### Directory

| 方法 | 说明 |
|------|------|
| `Directory.create(path)` | 创建目录 |
| `Directory.list(path)` | 列出内容 → `Array` |
| `Directory.exists(path)` | → `Bool` |

### HTTP

客户端基于 `reqwest`（blocking + rustls），支持 HTTPS。详见 [11_NetworkStdlib.md](11_NetworkStdlib.md)。

| 方法 | 说明 |
|------|------|
| `HTTP.fetch(url, options)` | 通用请求（method / headers / body / query / timeout_ms） |
| `HTTP.get(url)` | GET → 响应对象 |
| `HTTP.post(url, body)` | POST；body 为 `String` 或字典 |
| `HTTP.put/patch/delete(url, body?)` | 对应 HTTP 方法 |
| `HTTP.serve(addr, port, handler)` | blocking HTTP 服务端（handler 函数名） |
| `HTTP.response(status, body)` | 构建服务端响应对象 |

| Response 方法 | 说明 |
|---------------|------|
| `status()` | HTTP 状态码 |
| `ok()` | 2xx → `true` |
| `text()` / `body()` | 响应体字符串 |
| `json()` | 解析 JSON → 对象 |
| `header(name)` | 单个响应头 |
| `headers()` | 全部响应头 |

### URL

| 方法 | 说明 |
|------|------|
| `URL.parse(text)` | 解析 → `{ href, protocol, host, port, path, query }` |
| `URL.encodeQuery(dict)` | 字典 → query string |
| `URL.build(base, path, query)` | 拼接 URL |

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

### 全局

| 名称 | 说明 |
|------|------|
| `print(expr)` | 输出到 stdout |

> **已提供：** `readLine()` 从标准输入读取一行。  
> **未提供：** `exit()`、`assert()` 等全局函数。

---

## Async / Await（MVP）

```syx
async func fetch() -> String {
    let resp = await HTTP.get("https://example.com")
    return "ok"
}

// 结构化 Task（当前线程池 spawn）
let t = Task {
    work()
}
__syx_task_join(t)

// 显式并行
Task.detached {
    __syx_task_sleep(50)
}

// 带参 detach（Concurrency 或 host）
__syx_task_detach("worker", [42])
```

`syx.toml` 严格模式：

```toml
[concurrency]
strict = true   # 启用 C001–C003 等静态检查
```

| Host / API | 说明 |
|------------|------|
| `__syx_task_detach(name)` / `Concurrency.detach(name)` | 线程池 detached 任务 |
| `__syx_task_detach(name, args)` | 带参 spawn |
| `__syx_task_join` / `cancel` / `sleep` | 任务 join / 取消 / 睡眠 |
| `__syx_task_group_*` | TaskGroup 创建 / add / wait（按 group id 隔离） |
| `__syx_channel_create/send/recv` | Phase 8 async channel MVP（VM host） |
| `__syx_channel_try_recv` | Phase 15 非阻塞 recv（nil = 空）（`p15_channel_try_recv.sy`） |
| `__syx_channel_create(cap)` / `__syx_channel_try_send` | 有界 channel + 背压（ADR-267 / `m3_a04_backpressure`；满缓冲 trySend→false） |
| `__syx_channel_select([...])` | 多臂 select + RR 公平（ADR-268 / `m3_a04_fairness`；`select-fair-v1:round-robin`；叶子收口 ADR-269） |
| `Task { }` / `Task.detached { }` | 语法糖（生成 `__task_body_N`） |

| 限制 | 说明 |
|------|------|
| await 状态机 | 多 await 函数插入 `__syx_async_state`；`await __syx_task_sleep` 在 async 内 lowering |
| `async let` | `async let x = f()` → detach + 后续 await join（`p7_async_let.sy`） |
| 作用域 | `await` 仅允许在 `async func` 内 |
| Lint | async 函数无 await → **L005** |

---

## 借用与宏（Phase 7）

### 借用

```syx
func bump(x: &mut Int) {
    x = x + 1
}

func read_only(x: &Int) -> Int {
    return x
}
```

| 规则 | 诊断 |
|------|------|
| 同一作用域仅允许一个活跃 `&mut T` | **S011** |
| move 后 use | **S011**（moved value） |

Fixture：`p6_borrow_*`、`p7_borrow_mut.sy`、`p7_borrow_mut_fail.sy`。

### 宏

```syx
#if(DEBUG, print("debug on"))
#for(3, print("x"))
#assert(x > 0)
#macro twice(x) { print(x); print(x) }
let s = #stringify(1 + 1)
@Equatable
data class P(x: Int, y: Int)
```

由 `syx-macro::expand_module` 展开；深度超限 → **E002**。详见 [books/language-guide/19-macros.md](../books/language-guide/19-macros.md)。

---

## CLI 速查

```bash
cargo build --release              # 构建 syx
syx run file.sy                   # 编译并 VM 执行（完整标准库）
syx run .                          # 项目（syx.toml）
syx check .                        # 仅类型检查
syx build --native .               # MIR → app（LLVM sole；--object 仅 .o）
syx build --native --backend=llvm .  # MIR → app（实验性 LLVM）
syx build --wasm .                 # MIR → main.wasm（实验）
syx emit file.sy --format syxbc   # VM 字节码文本 → file.syxbc
syx emit . --format all -o emit/ # 六种 IR/字节码写入 emit/
syx run file.sy                   # MIR 单路径（Phase 12 默认）
syx format . / syx lint . / syx doc .
syx test .                         # 运行 tests/**/*.sy
syx repl
```

---

## 诊断代码

### 词法 / 语法

| 代码 | 说明 |
|------|------|
| L001 | 未闭合字符串 / 无效字符（lexer） |
| P001 | 意外 token / 语法错误 |

### 语义（编译期）

| 代码 | 说明 |
|------|------|
| S001 | 未定义类型 |
| S002 | 未定义标识符 |
| S003 | 类型不匹配 |
| S004 | 重复声明 |
| S005 | 无效访问级别（跨模块 internal） |
| S006 | 协议未满足（如 Iterator 缺 `next()`） |
| S007 | enum switch 非穷尽（按 case 名集合） |
| S008 | 跨 actor 直接访问实例字段 |
| S009 | where 子句未知协议约束 |
| S010 | 泛型实参不满足 where 协议约束 |
| S011 | 借用冲突 / use-after-move |

### 并发（strict 或 `@concurrent` 上下文）

| 代码 | 说明 |
|------|------|
| C001 | 非 MainActor 上下文调用 `@MainActor` 函数（`main` 隐式 MainActor） |
| C002 | 跨 actor 同步方法调用 |
| C003 | Task.detached / detach 捕获非 Sendable 类型（strict 或模块含 `class`） |
| C004 | Task 已取消 |
| C005 | Task 通道关闭 |
| C006 | Actor mailbox 关闭 |
| C009 | Actor 同步同实例 reentrancy 拒绝（serial-no-reenter） |
| C010 | Channel 已关闭（post-close send） |

### 运行时

| 代码 | 说明 |
|------|------|
| R001 | 文件读取失败 / registry semver 版本冲突 |
| R002 | 文件/目录写操作失败 |
| R003 | HTTP 请求失败 |
| R004 | HTTP POST 失败 |
| R005 | JSON 解析失败 |
| R006 | 无效正则 |
| R007 | 强制解包 nil |
| R011 | HTTP 请求超时 |
| R012 | HTTP serve 绑定失败 |
| R013 | URL 解析失败 |

### Lint（syx lint）

| 代码 | 规则摘要 |
|------|----------|
| L001–L012 | 命名、空函数、public 返回类型、async/await、行宽等 |

---

## 与 Swift 的主要差异（速记）

| Swift | SYX |
|-------|-----|
| `do { try } catch` | `try { } catch (err) { }` |
| `if cond {` | `if (cond) {` |
| `struct` 数据载体 | `data class Name(f: T)` |
| `import Module` | `import Math`（PascalCase 嵌入标准库模块名） |
| `print(_:separator:)` | `print(expr)` |
| `@main` | `func main()` |
| `throws` 函数 | Host 错误 + try/catch |

---

## 生态与 IDE（Phase 11 / SYX 4.5）

- **MIR 单路径**：用户模块始终走 MIR（Phase 12；`SYX_USE_MIR` 已移除）
- **LSP v4**：code actions、semantic tokens、inlay hints
- **Embed**：`compile_and_run_embedded`（见 ADR-003）

| 环境变量 / 路径 | 说明 |
|-----------------|------|
| `SYX_REGISTRY` | 生产 registry 默认 URL（`https://packages.sy.dev`）；覆盖 `[registry].default` |
| `SYX_REGISTRY_STAGING` | 无 `[registry].default` 时使用的 staging registry URL |
| `SYX_PUBLISHER_SIGNING_KEY` | 发布：32-byte Ed25519 seed（64 hex）；`syx publish` → `syx1:ed25519:<pk>:<sig>`（C-05） |
| `SYX_REGISTRY_VERIFY_KEY` | 安装：与发布者 verifying key 一致的 64 hex pin；嵌入 pk 必须匹配（C-02）；失配 `P011` |
| `SYX_REGISTRY_REQUIRE_SIGNATURE=1` | install 强制 ed25519 签名；Phase 5 拒绝 `syx1:stub` |
| `SYX_REGISTRY_ALLOW_STUB=1` | 测试专用：签名强制下仍接受 `syx1:stub` |
| `SYX_EMBED_DENY_UNKNOWN=1` | Embed v8：拒绝 `SYX_EMBED_CAPABILITIES` 中未知 capability（默认开） |
| `SYX_EMBED_CAPABILITIES` | Embed capability 白名单（`wasmtime,print,inprocess`） |
| `SYX_WASI=1` | 启用 wasmtime WASI preview1（`path_open` + 真 `fd_read`/`fd_write` 映射临时目录） |
| `SYX_WASI_GC_PHASE22=1` | WASI GC Phase 22：`syx_gc.alloc` 使用 noop arena host hook |
| `SYX_WASI_GC_PHASE23=1` | WASI GC Phase 23：bump arena + mark 清除 smoke |
| `SYX_REGISTRY_CDN_PHASE6=1` | Registry CDN Phase 6：staging signed URL install |
| `SYX_EMBED_SECCOMP=1` | Embed v9：seccomp/Job Object MVP 硬化 |
| `SYX_ATTACHED_HYGIENIC_V2=1` | `@attached` hygienic v2 模板（`{type}:{name}`） |
| `SYX_LLVM_RELEASE_BLOCKER=1` | LLVM strict parity 可选 PR release blocker（ADR-019） |
| `SYX_LLVM_STRICT_GREEN_WEEKS=N` | ADR-017 晋升计数（≥4 周绿可 promotion） |
| `SYX_LLVM_PARITY_GATE=strict` | LLVM Top-300 运行时 parity + 符号 strict gate（CI Phase 34） |
| `SYX_LLVM_AOT_PHASE9=0` | 禁用 LLVM AOT Phase 9 IR 路径（三后端统一 remembered set GC 直出 / ADR-044 gate） |
| `SYX_LLVM_AOT_PHASE8=0` | 禁用 LLVM AOT Phase 8 IR 路径（VM/native remembered set GC 直出 / ADR-042 gate） |
| `SYX_LLVM_AOT_PHASE7=0` | 禁用 LLVM AOT Phase 7 IR 路径（WASM/native GC intrinsics 全直出 / ADR-040 gate） |
| `SYX_LLVM_AOT_PHASE6=0` | 禁用 LLVM AOT Phase 6 IR 路径（GC intrinsics / guest heap / exec_op → 0% gate） |
| `SYX_LLVM_AOT_PHASE5=0` | 禁用 LLVM AOT Phase 5 IR 路径（CallHost/JSON / exec_op < 1% gate） |
| `SYX_LLVM_AOT_PHASE4=0` | 禁用 LLVM AOT Phase 4 IR 路径（ADR-028 Phase 4 / exec_op < 5% gate） |
| `SYX_LLVM_AOT_PHASE3=0` | 禁用 LLVM AOT Phase 3 IR 路径（ADR-028 Phase 3 / exec_op gate） |
| `SYX_LLVM_AOT_PHASE2=0` | 禁用 LLVM AOT Phase 2 IR 路径（ADR-028 Phase 2） |
| `SYX_REGISTRY_CDN_PHASE9=1` | Registry CDN Phase 9：failover 重试 + health probe |
| `SYX_REGISTRY_CDN_PHASE8=1` | Registry CDN Phase 8：多区域 URL + failover |
| `SYX_UNIFIED_GC_LTS=1` | 统一三后端 generational GC prod（VM + WASM + native 联动；ADR-045） |
| `SYX_UNIFIED_GC_LTS_PROD=1` | Unified GC LTS prod default：三后端 remembered set + promotion 默认 on |
| `SYX_WASI_GC_PHASE33=1` | WASI GC Phase 33：guest remembered set prod **默认 on** + VM `--run` 联动（ADR-043） |
| `SYX_WASI_GC_PHASE33_PROD=1` | WASI GC Phase 33 prod default：guest + VM/native generational 默认 on |
| `SYX_VM_NATIVE_GC_PHASE33=1` | VM/native generational GC prod（`syx run` + native codegen；ADR-043） |
| `SYX_VM_NATIVE_GC_PHASE33_PROD=1` | VM/native GC Phase 33 prod default：remembered set prod 默认 on |
| `SYX_WASI_GC_PHASE32=1` | WASI GC Phase 32：guest remembered set full generational GC prod 默认 on（ADR-041） |
| `SYX_WASI_GC_PHASE32_PROD=1` | WASI GC Phase 32 prod default：remembered set guest 默认 on |
| `SYX_WASI_GC_PHASE31=1` | WASI GC Phase 31：guest generational relocating GC prod 默认 on（ADR-039） |
| `SYX_WASI_GC_PHASE31_PROD=1` | WASI GC Phase 31 prod default：generational guest 默认 on |
| `SYX_WASI_GC_PHASE30=1` | WASI GC Phase 30：guest relocating GC prod 默认 on（ADR-037） |
| `SYX_WASI_GC_PHASE29=1` | WASI GC Phase 29：guest relocating GC MVP prod 默认 on（ADR-035） |
| `SYX_INKWELL_SPIKE=0` | 显式禁用 inkwell 路径，回退 clang text IR（Phase 34 ADR-044 prod default on 时 opt-out） |
| `SYX_INKWELL_SPIKE=1` | LLVM inkwell hello IR 编译（Phase 32 opt-in；Phase 33+ ADR-042/044 默认 GA 后非必需） |
| `SYX_BORROW_COQ_LEAN_PROOF_PROD=1` | borrow Coq/Lean **机器检验证明 prod**（obligation replay + 外部工具 hook；Phase 34） |
| `SYX_BORROW_COQ_LEAN_EXPORT=1` | borrow Coq/Lean 外部证明 scaffold（`.v`/`.lean` 导出 + replay；Phase 33） |
| `SYX_REGISTRY_CDN_PHASE15=1` | Registry CDN Phase 15：全球 CDN 多活写 + 边缘验签 E2E |
| `SYX_REGISTRY_CDN_PHASE14=1` | Registry CDN Phase 14：边缘缓存 + offline install 验签 E2E |
| `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` | 分布式 Actor Windows Named Pipe 真 API（Phase 32 `CreateNamedPipe`/`ConnectNamedPipe`） |
| `SYX_DIST_ACTOR_RUSTLS_CRATE=1` | 分布式 Actor rustls crate 证书链 / pin E2E（Phase 32） |
| `SYX_BORROW_MACHINE_PROOF=1` | borrow region 图机器证明 harness（Phase 32 proof obligation 导出） |
| `SYX_DIST_ACTOR_WIN_PIPE_NAMED_IO=1` | 分布式 Actor Windows Named Pipe 真 I/O（Phase 31 ER2；Phase 32 真 API 见上） |
| `SYX_WASI_GC_PHASE28=1` | WASI GC Phase 28：guest 统一堆 prod 默认 on |
| `SYX_WASI_GC_PHASE27=1` | WASI GC Phase 27：guest 统一堆接线（评估 gate） |
| `SYX_WASI_GC_PHASE27_EVAL=1` | WASI GC Phase 27 默认 on **评估** scaffold |
| `SYX_NET_STREAM_TLS=1` | Net stream 背压 + TLS 分层（ADR-032） |
| `SYX_NET_TCP_STREAM=1` | Net `stream()` 全双工 MVP（ADR-030） |
| `editors/vscode-syx/` | VS Code 扩展：LSP + `type: "syx"` DAP 调试；安装与断点调试见 [12_BackendDebuggingGuide.md](12_BackendDebuggingGuide.md) |

---

## 相关文档

- [9_UsageGuide.md](./9_UsageGuide.md) — 完整使用指南
- [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) — 功能优先级与实现状态
- 示例：`examples/p0_demo.sy`、`crates/syx-driver/tests/`
