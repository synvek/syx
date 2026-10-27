# 错误处理

**实现状态：** `try` / `catch` / `throw` / `try?` / `try!` / `Result<T, E>` **已实现**；`throws T` typed throws **部分实现**；rethrows **未实现**

SYX 提供三种错误传播路径：

1. **throws / try** — 异常抛出与捕获（运行时错误）
2. **Result<T, E>** — 显式代数类型
3. **Optional** — 用 `nil` 表示失败（最轻量）

本章覆盖语法、用法、Result 集成与跨函数传播。

## try / catch 块

```syx
func main() {
    try {
        let content = File.read("/tmp/x")
        print(content)
    } catch (err) {
        print("failed: \(err)")
    }
}
```

- `try { }` 块内可 `throw` / 调 `throws` 函数
- 异常触发后**立即**跳到 `catch`，跳过 `try` 块剩余部分
- `err` 类型 MVP 是 `String`（更复杂错误类型规划中）

来源：`crates/syx-driver/tests/fixtures/lsp/result_generic.sy`

## throw 表达式

```syx
func divide(_ a: Int, _ b: Int) throws -> Int {
    if (b == 0) {
        throw "division by zero"
    }
    return a / b
}
```

- `throw expr` 抛 `expr`（MVP 类型 `String`）
- 仅在 `throws` 函数内合法
- 抛 `String` / `Result.err` / 自定义 enum 都允许（`expr` 类型由函数签名的 `throws T` 决定）

## throws 函数声明

```syx
func loadFile(path: String) throws -> String {
    if (!File.exists(path)) {
        throw "not found: \(path)"
    }
    return File.read(path)
}

func parseInt(s: String) throws -> Int {
    // 假定有 Int.fromString，返回 Result
    let r = Int.fromString(s)
    switch (r) {
    case .ok(let n) { return n }
    case .err(let e) { throw e }
    }
}
```

| 签名 | 含义 |
|------|------|
| `throws -> T` | 可能抛出，调用方需 `try` |
| `throws E -> T` | typed throws（**部分实现**）抛 `E` |
| `async throws -> T` | async 且 throws |

## try 表达式

调用 `throws` 函数的三种形式：

### `try expr`（必须 try）

```syx
let s = try loadFile(path: "/tmp/x")      // String
```

- 必须用 `try` 包裹（编译器提示 P001 包装缺失）
- 等价于 `try { let s = loadFile(...) }`

### `try? expr`（失败转 nil）

```syx
let s = try? loadFile(path: "/tmp/x")     // String?
```

- 成功 → 返回 `T`
- 失败 → 返回 `nil`（异常被吞掉）

### `try! expr`（强制无错）

```syx
let s = try! loadFile(path: "/tmp/x")     // String
```

- 成功 → 返回 `T`
- 失败 → 运行时 panic

> **`try?` 链：** 多个 `try?` 可链式：`try? foo() + try? bar()`，任一失败 → 整体 `nil`（视具体语义）。

## 链式 catch

```syx
try {
    let r = Http.get(url)
    print(r)
} catch (err) where (err.hasPrefix("5")) {
    print("server error: \(err)")
} catch (err) where (err.hasPrefix("4")) {
    print("client error: \(err)")
} catch (err) {
    print("other: \(err)")
}
```

> **`where` 守卫：** 早期版本支持 try-catch 块的 where 守卫（**部分实现**）；推荐直接调 `try?` / `switch` Result。

## Result 类型

`Result<T, E>` 是显式错误类型的标准方案（[第 9 章 枚举](../../docs/adr/architecture-decisions.md)）：

```syx
func parse(s: String) -> Result<Int, String> {
    if (s.isEmpty) { return .err("empty") }
    return .ok(s.count)
}

let r = parse(s: "hello")
switch (r) {
case .ok(let n) {
    print("len=\(n)")
}
case .err(let msg) {
    print("err: \(msg)")
}
}
```

### Result 静态构造

`ok(value)` / `err(value)` 是预声明的便利函数：

```syx
let r1 = ok(42)                            // Result<Int, Never>
let r2 = err("oops")                       // Result<Never, String>
```

### Result 链式

```syx
let r: Result<Int, String> = ok(10)
    .map({ x in x * 2 })                   // ok(20)
    .flatMap({ x in x > 0 ? ok(x) : err("neg") })
```

> **方法：** `.ok(v)` / `.err(v)` / `.isOk()` / `.map(f)` / `.flatMap(f)` / `.description` 全部由 `packages/core/builtins/result.sy` 定义。

### @Codable + Result 序列化

```syx
@Codable
enum ApiError: Error {
    case notFound
    case serverError(Int, String)
}
```

来源：`fixtures/lsp/result_generic.sy` · `fixtures/lsp/result_generic.sy`

## throws 与 actor

`actor` 内的 `throws func` 自动隔离：

```syx
actor FileManager {
    func read(path: String) throws -> String {
        if (!File.exists(path)) { throw "not found" }
        return File.read(path)
    }
}

let m = FileManager()
try {
    let s = await m.read(path: "/tmp/x")
    print(s)
} catch (err) {
    print("failed: \(err)")
}
```

## 错误处理策略选择

| 场景 | 选 |
|------|-----|
| 必须处理的运行时错误（IO、网络） | `throws` + `try / catch` |
| 可恢复的逻辑错误（解析、转换） | `Result<T, E>` |
| 「有/无」二态（查找、缓存命中） | `Optional<T>` |
| 库内部 panic 路径 | `try!` / `fatalError` |
| 错误类型需要携带多字段 | enum + `Result` / typed throws |

## 完整示例

```syx
enum MathError {
    case divideByZero
    case overflow
}

func safeDivide(_ a: Int, _ b: Int) -> Result<Int, MathError> {
    if (b == 0) { return .err(.divideByZero) }
    if (a > 1000) { return .err(.overflow) }
    return .ok(a / b)
}

func doubleAndPrint(_ a: Int, _ b: Int) {
    switch (safeDivide(a, b)) {
    case .ok(let n) {
        print("\(a) / \(b) = \(n * 2)")
    }
    case .err(.divideByZero) {
        print("oops: divide by zero")
    }
    case .err(.overflow) {
        print("oops: overflow")
    }
    }
}

func main() {
    doubleAndPrint(10, 2)                  // "10 / 2 = 10"
    doubleAndPrint(10, 0)                  // "oops: divide by zero"
    doubleAndPrint(2000, 5)                // "oops: overflow"
}
```

## 与 Swift 风格对照

| Swift | SYX |
|-------|-----|
| `do { try … } catch { }` | `try { } catch { }`（无 `do`） |
| `Error` 协议 | MVP 用 `String` / `Result` |
| `throws E` typed throws | **部分实现** |
| `try?` `try!` | **已实现** |
| `rethrows` | **未实现** |
| `Result<T, E>` stdlib | **已实现**（含静态 `ok` / `err`） |
| `error: { }` 闭包参数 | `catch (err) { }` 块 |
| `try? foo()?.bar()` | 合法 |

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S020** | 非 `throws` 函数内 `throw` | 加 `throws` |
| **P001** | 调 `throws` 函数未 `try` | 加 `try` / `try?` / `try!` |
| **R007** | `try!` 失败 | 改 `try?` / `try { } catch` |
| **S004** | `throws` 类型不匹配 | 修正 `throws T` |

---

**下一章** → [并发](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：语句](../language-reference/04-statements.md) · [语言参考：声明](../language-reference/05-declarations.md) · [第 9 章 枚举](../../docs/adr/architecture-decisions.md) · [第 18 章 并发](../../docs/adr/architecture-decisions.md)
