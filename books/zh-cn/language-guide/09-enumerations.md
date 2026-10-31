# 枚举

**实现状态：** 简单枚举 / 关联值 / 泛型枚举 / `switch` exhaustiveness / `if case` **全部已实现**；raw value 自动 Int **部分实现**；`indirect enum` **未实现**

枚举定义一组相关值。SYX 枚举是 Swift 风格「**代数数据类型**（ADT）」：每个 case 可以是简单标识，也可以携带关联值。

## 简单枚举

```syx
enum Status {
    case ok
    case err
    case pending
}

let s = Status.ok
print(s.tag)                              // 0（内部 tag）
```

> **关键字 `case`：** SYX 简单枚举 **必须** 写 `case`（与 Swift 1.x 早期风格一致；`enum Status { ok, err, pending }` 不合法）。

来源：`crates/syx-driver/tests/fixtures/lang/enum_associated.sy`

## 关联值（associated values）

```syx
enum NetworkResult {
    case success(String)
    case failure(code: Int, message: String)
}

let r1 = NetworkResult.success("OK")
let r2 = NetworkResult.failure(code: 404, message: "Not Found")
```

- 关联值可命名（`code:`, `message:`）或匿名（位置）
- 一个枚举可有多种 case 形态（discriminated union）
- `Result<T, E>` 标准库定义见 `packages/core/builtins/result.sy`

来源：`crates/syx-driver/tests/fixtures/lang/enum_associated.sy`

## switch 匹配

```syx
func describe(_ r: NetworkResult) -> String {
    switch (r) {
    case .success(let data) {
        return "ok: \(data)"
    }
    case .failure(code: let c, message: let m) {
        return "fail code=\(c) msg=\(m)"
    }
    default {                              // enum 可缺 default；非封闭域需
        return "?"
    }
    }
}
```

> **花括号：** SYX `case` **必须** 跟 `{ }` 块。详见 [第 6 章 控制流](06-control-flow.md#switch)。

### 命名关联值的解构

```syx,error=L001
case .failure(code: let c, message: let m) { … }    // 命名
case .failure(let c, let m) { … }                  // 位置
case let .success(data) { … }                       // 单参数简写
```

### `where` 守卫

```syx,error=L001
case .success(let data) where (data.count > 0) { … }
case .failure(let m) where (m.hasPrefix("5")) { … }
```

详见 [第 6 章 控制流](06-control-flow.md#守卫-where)。

## if case

```syx
enum NetworkResult {
    case success(String)
    case failure(String)
}
let result = NetworkResult.success("ok")
if case .success(let data) = result {
    print(data)
} else {
    print("not success")
}
```

`if case` 是 `switch` 单分支的便捷形式。`case` 后模式与 `switch` case 一致。

## 泛型枚举

```syx
enum Optional<T> {
    case some(T)
    case none
}

enum Result<T, E> {
    case ok(T)
    case err(E)
}
```

- 泛型参数在枚举名后
- 单态化生成 `Optional<Int>` / `Optional<String>` 等具体实例
- 标准 `Optional` 与 `Result` 是预声明的（不用自己写）

**已实现**（`fixtures/lang/enum_associated.sy`）

## 嵌套类型

```syx
enum Tree {
    case leaf(Int)
    case branch(left: Tree, right: Tree)
}
```

> **递归嵌套：** 单层递归编译为对象引用；深层递归需要 `indirect enum`（**未实现**，规划中）。当前用 `class` 包装替代。

## Raw value（部分实现）

```syx
enum Direction: Int {
    case north = 0
    case east = 1
    case south = 2
    case west = 3
}
```

| 能力 | 状态 |
|------|------|
| `: Int` raw value | **部分实现**（Phase 50 基础） |
| `: String` raw value | **未实现** |
| 隐式 raw value | **未实现**（必须显式赋值） |

> **替代：** 简单枚举可用普通 case + 显式 `Int` 转换。完整 raw value → 后续 Phase。

## 枚举的方法

```syx
enum Status {
    case ok, err, pending

    func isFinal() -> Bool {
        switch (self) {
        case .ok { return true }
        case .err { return true }
        case .pending { return false }
        }
    }

    func description() -> String {
        switch (self) {
        case .ok { return "ok" }
        case .err { return "err" }
        case .pending { return "..." }
        }
    }
}

print(Status.ok.isFinal())                 // true
print(Status.pending.description())        // "..."
```

> **`self`：** 在枚举方法内 `self` 是当前 case（已绑定 case + 关联值）。

## 枚举与协议

枚举可实现协议（`Equatable` / `Hashable` / `Codable` 等）：

```syx
@Equatable
enum Status {
    case ok
    case err
    case pending
}

print(Status.ok == Status.err)            // false
```

> **手动实现：** 关联值枚举的 `==` 由编译器按 case + 关联值生成（**已实现**，`Equatable` 合成）。

`@Hashable` / `@Codable` 详见 [第 19 章 宏](19-macros.md#证明宏l1-ga--l2-partialphase-5052119)。

## Codable L2（关联值 enum）

Phase 129 起，带关联值的 enum **全自动** 合成 `encode` / `decode`（指纹 `macro-codable-l2:associated-enum-auto`）：

```syx
@Codable
enum WireStatus {
    case ok(code: Int)
    case err(message: String)
}

func main() {
    let w = WireStatus.ok(code: 7)
    let text = w.encode()
    let back = WireStatus.decode(text: text)
}
```

限制：泛型关联值全集 / 任意嵌套 / 自定义 coding keys → 后续 Phase。

## exhaustiveness

枚举与 `final class` 封闭域在 `switch` 中自动做穷尽检查；未覆盖全部 case → **S007**。

```syx,error=S007
enum Color { case red, green, blue }

func describe(_ c: Color) -> String {
    switch (c) {
    case .red { return "red" }
    case .green { return "green" }
    case .blue { return "blue" }
    }                                       // ✅ 全覆盖
}

func incomplete(_ c: Color) -> String {
    switch (c) {
    case .red { return "red" }
    }                                       // ❌ S007：missing cases
}
```

## 与 `final class` 的取舍

| 需求 | 选 |
|------|-----|
| 单 case 共享结构、模式匹配 | `enum` |
| 含字段的方法或继承 | `class` / `final class` |
| 跨进程 / 反序列化 | `enum`（更小、更纯） |
| 关联值有命名参数 | `enum case`（`case failure(code:, message:)`） |
| 变体需要附加方法或共享基类 | `class`（多态）或 `final class`（封闭） |

> **注意：** lexer **无** `sealed` 关键字；ADT 用 `enum`，禁止子类化用 `final class`。

## 完整示例

```syx
import JSON

enum ApiResponse {
    case ok(data: String)
    case unauthorized
    case serverError(status: Int, body: String)
}

func handle(_ r: ApiResponse) -> String {
    switch (r) {
    case .ok(let data) where (data.count > 0) {
        return "OK: \(data)"
    }
    case .ok {
        return "OK (empty)"
    }
    case .unauthorized {
        return "401"
    }
    case .serverError(status: let s, body: let b) where (s >= 500) {
        return "5xx: \(b)"
    }
    case .serverError(status: let s, body: let b) {
        return "4xx[\(s)]: \(b)"
    }
    }
}

func main() {
    let r1 = ApiResponse.ok(data: "hello")
    let r2 = ApiResponse.serverError(status: 503, body: "down")
    print(handle(r1))                       // "OK: hello"
    print(handle(r2))                       // "5xx: down"
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S007** | `switch` 非穷尽（未覆盖所有 case） | 加缺失 case 或 `default` |
| **S010** | 关联值解构不匹配 | 检查 enum 定义 |
| **P001** | `case` 缺少花括号块 | 加 `{ }` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `case ok, err`（无 `case` 关键字） | **必须** `case ok` / `case err` |
| `indirect enum` 递归 | **未实现**（用 class 包装） |
| `String` / 字符 raw value | **未实现**（仅 Int 部分） |
| `CustomStringConvertible` 默认实现 | 需显式 `func description() -> String` |
| `@Codable` 关联值枚举 | **部分 GA**（L2 全自动 Phase 129） |
| `if case let` | `if case .variant(let x) = expr`（语法稍异） |

---

**下一章** → [结构体和类](10-structures-and-classes.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 6 章 控制流](06-control-flow.md) · [第 19 章 宏](19-macros.md) · [第 10 章 结构体和类](10-structures-and-classes.md)
