# 错误处理

**实现状态：** `do` / `catch` / `throw` / 表达式 `try` / `try?` / `try!` / `Result<T, E>` **已实现**（SA-2-01）；`throws T` typed throws **部分实现**；rethrows **未实现**

SYX 提供三种错误传播路径：

1. **throws / do-catch / try expr** — 异常抛出与捕获（运行时错误）
2. **Result<T, E>** — 显式代数类型
3. **Optional** — 用 `nil` 表示失败（最轻量）

本章覆盖语法、用法、Result 集成与跨函数传播。宪章：[sa-2-01-do-catch](../../../docs/adr/sa-2-01-do-catch.md)。

> **教学说明（R-46）：** 示例优先 `enum …: Error` 与 `throw Case`。`throw "string"` 仍是部分宿主路径的残余 MVP — 不是主教学风格。

## do / catch 块

```syx
import System

enum IoError: Error {
    case readFailed
}

func main() {
    do {
        let content = File.read("/tmp/x")
        print(content)
    } catch err {
        print("failed: \(err)")
    }
}
```

- `do { }` 块内可 `throw` / 调 `throws` 函数
- 异常触发后**立即**跳到 `catch`，跳过 `do` 块剩余部分
- 优先裸 `catch { }` 或 `catch err { }`（括号形式 `catch (err)` 为旧教学写法）

来源：`crates/syx-driver/tests/fixtures/lsp/result_generic.sy`

### 跨函数 `try`（R-38 output 钉 · SEM-THROW）

`do { try g() } catch` 必须进入 catch（错误不得当普通值打印）：

```syx,output
// prints: caught
enum E: Error {
    case bad
}

func g() throws {
    throw E.bad
}

func main() {
    do {
        try g()
        print("value")
    } catch {
        print("caught")
    }
}
```

## throw 表达式

```syx
enum MathError: Error {
    case divisionByZero
}

func divide(_ a: Int, _ b: Int) throws -> Int {
    if b == 0 {
        throw MathError.divisionByZero
    }
    return a / b
}
```

- `throw expr` 抛出 `expr` — 优先符合 `Error` 的类型
- 仅在 `throws` 函数内合法
- `throw "string"` 仍作为残余 MVP 接受；新示例应使用 `Error` case

## throws 函数声明

```syx
import System

enum IoError: Error {
    case notFound
}

enum ParseError: Error {
    case empty
}

func loadFile(path: String) throws -> String {
    if !File.exists(path) {
        throw IoError.notFound
    }
    return File.read(path)
}

func parseInt(s: String) throws -> Int {
    if s.count == 0 {
        throw ParseError.empty
    }
    return 0
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
import System

enum IoError: Error {
    case notFound
}

func loadFile(path: String) throws -> String {
    if !File.exists(path) { throw IoError.notFound }
    return File.read(path)
}

func main() {
    let s = try loadFile(path: "/tmp/x")  // String
    print(s)
}
```

- 必须包 `try`（漏包编译器报 P001）
- 等价于 `do { let s = loadFile(...) }`

### `try? expr`（失败变 nil）

```syx
import System

enum IoError: Error {
    case notFound
}

func loadFile(path: String) throws -> String {
    if !File.exists(path) { throw IoError.notFound }
    return File.read(path)
}

func main() {
    let s = try? loadFile(path: "/tmp/x") // String?
    print(s)
}
```

- 成功 → 返回 `T`
- 失败 → 返回 `nil`（异常被吞掉）

### `try! expr`（强制无错）

```syx
import System

enum IoError: Error {
    case notFound
}

func loadFile(path: String) throws -> String {
    if !File.exists(path) { throw IoError.notFound }
    return File.read(path)
}

func main() {
    let s = try! loadFile(path: "/tmp/x") // String
    print(s)
}
```

- 成功 → 返回 `T`
- 失败 → 运行时 panic

> **`try?` 链：** 多个 `try?` 可串联：`try? foo() + try? bar()`，任一失败整式为 `nil`（取决于精确语义）。

## 链式 catch

~~~text
do {
    let r = Http.get(url)
    print(r)
} catch err {
    print("failed: \(err)")
}
~~~

> **`where` 守卫：** 早期版本支持 try-catch 上的 `where`（**部分实现**）；更推荐 `try?` / 对 Result 做 switch。

## Result 类型

`Result<T, E>` 是显式错误类型的标准方案（[第 9 章 枚举](09-enumerations.md)）：

```syx
func parse(s: String) -> Result<String, String> {
    if (s.isEmpty) { return Result.err(value: "empty") }
    return Result.ok(value: s)
}

func main() {
    let r = parse(s: "hello")
    if (Result.isOk(result: r)) {
        print("ok")
    } else {
        print("err")
    }
}
```

### 静态 Result 构造

`Result.ok(value:)` / `Result.err(value:)` 是 String 型 MVP 构造器（`packages/core/builtins/algebraic/result.sy`）：

```syx
func main() {
    let r1 = Result.ok(value: "42")          // Result<String, String> success
    let r2 = Result.err(value: "oops")       // Result<String, String> failure
    print(Result.isOk(result: r1))
    print(Result.isErr(result: r2))
}
```

### Result 链式

薄 MVP check 面（完整 `.map` / `.flatMap` 泛型仍在加深）：

```syx
func main() {
    let r1 = Result.ok(value: "10")
    print(Result.isOk(result: r1))           // true
    let r2 = Result.err(value: "neg")
    print(Result.isErr(result: r2))          // true
}
```

> **方法：** `Result.ok` / `Result.err` / `Result.isOk` / `Result.isErr` / `.description` 在 `packages/core/builtins/algebraic/result.sy`。泛型 `.map` / `.flatMap` 仍是加深轨。

### @Codable + Result 序列化

```syx
@Codable
enum ApiError: Error {
    case notFound
    case serverError(Int, String)
}
```

来源：`fixtures/lsp/result_generic.sy` · `fixtures/lsp/result_generic.sy`

## throws 与 Actor

`actor` 内的 `throws` 函数自动隔离：

```syx
import System

enum IoError: Error {
    case notFound
}

actor FileManager {
    func read(path: String) throws -> String {
        if !File.exists(path) { throw IoError.notFound }
        return File.read(path)
    }
}

async func main() {
    let m = FileManager()
    do {
        let s = await m.read(path: "/tmp/x")
        print(s)
    } catch err {
        print("failed: \(err)")
    }
}
```

## 选择错误处理策略

| 场景 | 选择 |
|------|-----|
| 必须处理的运行时错误（IO、网络） | `throws` + `try / catch` |
| 可恢复逻辑错误（解析、转换） | `Result<T, E>` |
| 二态「有/无」（查找、缓存命中） | `Optional<T>` |
| 库内部 panic 路径 | `try!` / `fatalError` |
| 多字段错误类型 | enum + `Result` / typed throws |

## 完整示例

~~~text
enum MathError: Error {
    case divideByZero
    case overflow
}

func safeDivide(_ a: Int, _ b: Int) -> Result<Int, MathError> {
    if b == 0 { return .err(.divideByZero) }
    if a > 1000 { return .err(.overflow) }
    return .ok(a / b)
}

func doubleAndPrint(_ a: Int, _ b: Int) {
    switch safeDivide(a, b) {
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
~~~

## 与 Swift 风格对照

| Swift | SYX |
|-------|-----|
| `do { try … } catch { }` | `do { try … } catch { }` |
| `Error` 协议 | 优先 `enum …: Error`（string throw = 残余 MVP） |
| `throws E` typed throws | **部分实现** |
| `try?` `try!` | **已实现** |
| `rethrows` | **未实现** |
| stdlib `Result<T, E>` | **已实现**（含静态 `ok` / `err`） |
| `catch { }` / `catch err` | 优先裸或命名 catch（非 `catch (err)`） |
| `try? foo()?.bar()` | 合法 |

## 诊断

| 码 | 触发 | 修复 |
|------|------|------|
| **S020** | 非 `throws` 函数内 `throw` | 加 `throws` |
| **P001** | 调 `throws` 函数未包 `try` | 加 `try` / `try?` / `try!` |
| **R007** | `try!` 失败 | 改 `try?` / `do { } catch` |
| **S004** | `throws` 类型不匹配 | 修正 `throws T` |

---

**下一章** → [并发](18-concurrency.md)

**另见** → [语言参考：语句](../language-reference/04-statements.md) · [语言参考：声明](../language-reference/05-declarations.md) · [第 9 章 枚举](09-enumerations.md) · [第 18 章 并发](18-concurrency.md)
