# 函数

**实现状态：** `func` 声明 / 命名参数 / 默认参数 / `async` / `throws` / `requires Pure` / `requires IO` **全部已实现**；`@escaping` / `@autoclosure` / 泛型类型擦除反射 **未实现**

函数是 SYX 程序的基本构件。本章覆盖声明语法、参数、返回、效果注解（`requires`）、async/await、throws、与跨模块可见性。

## 定义与调用

```syx
func greet(name: String) {
    print("Hello, \(name)")
}

func add(a: Int, b: Int) -> Int {
    return a + b
}

func main() {
    greet(name: "SYX")               // "Hello, SYX"
    print(add(a: 1, b: 2))           // 3
}
```

> **命名参数：** 调用带标签的参数时 **必须** 写 `label: value`。形参可用 `_ name` 省略外部标签（调用处只能位置传参）；可用 `label name` 指定与内部名不同的外部标签（调用处必须写 `label:`，不能写内部名）。

```syx
func greet(name: String) { print("Hello, \(name)") }
func move(to destination: String) { }
func join(_ task: String) -> String { return task }

greet(name: "SYX")           // ✅ 必须带 name:
move(to: "home")             // ✅ 外部标签 to:，内部名 destination
join("worker")               // ✅ _ 省略标签 → 位置传参
join(task: "worker")         // ❌ S042：_ 参数不能带标签
```

| 形参写法 | 外部标签策略 | 调用写法 |
|----------|--------------|----------|
| `name: Type` | 同内部名 `name` | `name: value`（必填） |
| `label name: Type` | 外部名 `label` | `label: value`（必填） |
| `_ name: Type` | 省略 | 只能位置传参 |

| 元素 | 说明 |
|------|------|
| `func` | 关键字 |
| `name` | 函数名（`lowerCamelCase`） |
| `(param: Type, ...)` | 形参列表 |
| `-> ReturnType` | 返回类型（缺省 `Void`） |
| `block` | 函数体 |

## 多返回元组

SYX 元组可作为「多返回值」：

```syx
func divmod(a: Int, b: Int) -> (Int, Int) {
    return (a / b, a % b)
}

let (q, r) = divmod(a: 17, b: 5)
print("\(q) rem \(r)")           // 3 rem 2
```

元组解构 `(q, r) = …` 在 `let` / `var` 绑定中合法（MVP GA；`fixtures/lang/tuple.sy`）。命名元素 `(x: 1, y: 2)` 与成员 `.x` / `.0` 可用；非完整 Swift 元组协议。

## 默认参数

```syx
func greet(name: String, greeting: String = "Hello") {
    print("\(greeting), \(name)")
}

greet(name: "SYX")                       // "Hello, SYX"
greet(name: "SYX", greeting: "Hi")       // "Hi, SYX"
```

> **限制：** 默认值必须是编译期常量或裸字面量；不能引用运行时变量或调用函数。带默认值的形参可放在任意位置（无 Swift 的「默认参数放末尾」要求）。

## 访问控制

```syx
public func exported() { }               // 跨模块可 import
internal func moduleOnly() { }           // 模块内（默认）
private func hidden() { }                 // 文件 / 作用域内
```

详见 [第 28 章 访问控制](28-access-control.md)。

```syx
import math                               // 跨模块
math.add(a: 1, b: 2)                      // ✅ public
// math.internal_helper()                // ❌ S005：跨模块访问 internal
```

## async 函数

```syx
async func fetch() -> String {
    return await httpGet("https://example.com")
}
```

`async` 关键字把函数标记为 **协程**；函数体内可使用 `await`：

```syx
async func fetchTwo() -> (String, String) {
    let a = await fetchA()
    let b = await fetchB()
    return (a, b)
}
```

- `await` 仅在 `async` 函数体内合法
- 非 `async` 函数使用 `await` → 编译错误 **S020**
- `async func main()` 是合法的程序入口

> **实现：** 多 `await` 状态机（**已实现**，测试 `runtime/async_native.rs`）。底层通过 generator-based state machine；详见 [第 18 章 并发](18-concurrency.md)。

## async let

并行启动多个子任务，等所有完成：

```syx
async func parallel() -> Int {
    async let a = fetchA()              // 启动子任务
    async let b = fetchB()              // 启动子任务
    let x = await a                     // 等待 a
    let y = await b                     // 等待 b
    return x + y
}
```

> **语义：** `async let` desugar 为 detach + join（SYX 132 行为冻结）。`a` 是 `AsyncValue<T>`，`await a` 阻塞当前任务直到子任务完成。

## throws 函数

```syx
func loadFile(path: String) throws -> String {
    if (!File.exists(path)) {
        throw "not found: \(path)"
    }
    return File.read(path)
}
```

| 子句 | 含义 |
|------|------|
| `throws` | 函数可能抛出（不指定异常类型；MVP 是 `String` 或 `Result`） |
| `throws T` | 抛 `T` 类型（typed throws **部分实现**） |
| `throw expr` | 在 `throws` 函数内抛异常 |
| `try expr` | 调用 `throws` 表达式 |
| `try { } catch { }` | 块级捕获 |
| `try?` | 错误时返回 `nil` 或传播 |
| `try!` | 强制（错误时 panic） |

调用：

```syx
let s = try loadFile(path: "/tmp/x")            // 表达式级 try
let s2 = try? loadFile(path: "/tmp/x")          // String?（nil 表示错误）
let s3 = try! loadFile(path: "/tmp/x")          // String（错误 panic）

try {
    let s = loadFile(path: "/tmp/x")
    print(s)
} catch (err) {
    print("failed: \(err)")
}
```

详见 [第 17 章 错误处理](17-error-handling.md)。

## 效果注解（`requires`）

标记函数「纯」（无副作用）或「IO」：

```syx
func pure(x: Int) -> Int requires Pure {
    return x + 1
}

func load() -> String requires IO {
    return File.read("data.txt")
}
```

| 注解 | 含义 | 编译期检查 |
|------|------|------------|
| `requires Pure` | 无 I/O / 全局状态依赖 | **Phase 50+** 检查调用链（`syx-meta`） |
| `requires IO` | 允许 I/O | 标记用 |
| 无注解 | 默认（与 IO 等价） | 无检查 |

> **MVP 状态：** `syx-meta` 部分实现（[第 32 章 运算符 vs Core Host](32-operators-vs-host.md) 提及）。完整效果系统 → 后续 Phase。**应用层代码** 可不写；服务层 / SDK 可用。

## 泛型函数

```syx
func id<T>(x: T) -> T { return x }
func swap<T>(a: T, b: T) -> (T, T) { return (b, a) }

let n = id(x: 42)            // Int
let s = id(x: "hi")          // String
```

泛型参数在函数名后；可加 `where` 约束：

```syx
func eq<T>(a: T, b: T) -> Bool where T: Equatable {
    return a == b
}
```

详见 [第 24 章 泛型](24-generics.md) 与 [语言参考：泛型形参和实参](../language-reference/08-generic-parameters-and-arguments.md)。

## 运算符函数

自定义运算符（**GA**）：

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

let r = 5 |> { $0 * 2 }      // 10
```

详见 [第 30 章 高级运算符](30-advanced-operators.md)。

## 函数作为值（一等公民）

```syx
let f: (Int) -> Int = { x in x * 2 }
let r = f(3)                                  // 6

func apply(g: (Int) -> Int, x: Int) -> Int {
    return g(x)
}
print(apply(g: f, x: 4))                      // 8
```

函数类型 `(P) -> R` 是一等公民：可作变量、参数、返回值、`Array` 元素。

> **`$0` / `$1`：** 闭包简写（`{ $0 + $1 }`），对应首个 / 二个参数；与 Swift `$0` 一致。**部分实现**。

## 嵌套函数

```syx
func outer() -> Int {
    func inner(x: Int) -> Int { return x * 2 }
    return inner(x: 21)                       // 42
}
```

内部函数可捕获外层 `let`/`var`（详见 [第 8 章 闭包](08-closures.md)）。

## 递归

```syx
func fib(n: Int) -> Int {
    if (n < 2) { return n }
    return fib(n: n - 1) + fib(n: n - 2)
}
```

SYX 递归使用 VM 调用栈；深递归触发 **R014 stack overflow**。

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S001** | 函数名重复 | 改函数名 |
| **S004** | 参数/返回类型不匹配 | 修正类型 |
| **S005** | 跨模块访问 internal 符号 | 改 `public` |
| **S042** | 调用参数标签缺失、错误或 `_` 参数带标签 | 补正确 `label:` 或改形参为 `_` |
| **S020** | 非 `async` 函数使用 `await` | 加 `async` 或去掉 `await` |
| **R007** | `!` 解包 `nil` | 改 `?` 或 `if let` |
| **R014** | 递归过深 | 改迭代或增加栈大小 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `_` 省略参数标签 | **已实现**（`_ name`）；仅当 **全部** 形参均为 `_` 时可全位置调用 |
| 强制标签匹配 | **已实现**（S042）；无「全位置逃逸」 |
| 参数默认值可省略尾部 | SYX 任意位置均可 |
| `@escaping` | **未实现** |
| `@autoclosure` | **未实现** |
| `rethrows` | **未实现** |
| `inout` | **未实现**；用 `&mut T` 借用（部分实现） |
| `async throws` | **已实现** |
| `async let` 语义 | desugar detach+join（Swift 是结构化并发） |
| 闭包简写 `$0`/`$1` | **部分实现** |
| 泛型 where 子句 | **已实现** |

## 实现状态

- 命名参数：**已实现**
- 默认参数：**已实现**
- `async` / `await` / `async let`：**已实现**（多 await 状态机 GA）
- `throws` / `try` / `try?` / `try!`：**已实现**
- `requires Pure` / `requires IO`：**部分实现**（`syx-meta` MVP）
- 运算符函数：**GA**（三后端 parity）
- 嵌套函数：**已实现**
- 闭包简写 `$0`：**部分实现**

---

**下一章** → [闭包](08-closures.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 17 章 错误处理](17-error-handling.md) · [第 18 章 并发](18-concurrency.md) · [第 30 章 高级运算符](30-advanced-operators.md)
