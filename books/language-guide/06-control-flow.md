# 控制流

**实现状态：** `if` / `else` / `while` / `repeat while` / `for-in` / `for await in` / `break` / `continue` / `switch` / `where 守卫` / `if case` / `if let` / `guard let` **全部已实现**；标签 break/continue（label）**部分实现**；fallthrough **拒绝**

SYX 控制流与 Swift 风格一致：块用 `{ }` 包裹，模式匹配靠 `switch` + `where` 完成。本章按控制结构分类讲解。

> **条件括号可选：** `if` / `while` / `switch` 的条件表达式**不强制**加 `()`。以下两种写法都合法：
>
> ```syx
> if x > 0 { print("pos") }              // 推荐（与 Swift 一致）
> if (x > 0) { print("pos") }              // 仍合法（兼容旧代码）
> ```
>
> 本章正文示例优先使用括号**省略**的现代风格。

## if / else

```syx
if x > 0 {
    print("positive")
} else if x == 0 {
    print("zero")
} else {
    print("negative")
}
```

| 注意点 | 说明 |
|--------|------|
| 条件括号 | **可选**；`if x` 与 `if (x)` 都合法 |
| 条件类型 | 必须是 `Bool`；非 Bool → **S004** |
| `else if` | 链式合法；无嵌套深度限制 |
| 块省略 | `if x { print(x) }` 合法（单语句可省 `{}`） |

> **「if 表达式」：** Phase 100+ 起 `if` 可作表达式返回值；用于赋值时两分支必须存在且类型相同。

```syx
let label = if x > 0 { "pos" } else { "non-pos" }
```

## while

```syx
var x = 10
while x > 0 {
    print(x)
    x = x - 1
}
```

条件检查在循环体之前；条件为 `false` 时循环体不执行。

## repeat while

```syx
var n = 0
repeat {
    n = n + 1
    print(n)
} while n < 3
```

循环体 **至少** 执行一次；条件在每次迭代后检查。

> **Swift 对照：** Swift 用 `repeat { } while …`；SYX 相同。Swift 的 `do { } while …` 形式（不带 `repeat`）不识别。

## for-in

```syx
for item in [1, 2, 3] {
    print(item)
}

for c in "abc" {
    print(c)
}

for i in 0..<10 {
    print(i)
}

for await x in asyncSequence {
    print(x)
}
```

可迭代的源：

| 形式 | 元素类型 |
|------|----------|
| `Array<T>` | `T` |
| `Set<T>` | `T` |
| `Dictionary<K, V>` | `(K, V)` 元组 |
| `String` | `String`（单标量） |
| `0..<n` | `Int` |
| 自定义 `Iterator` | `String`（协议要求） |
| `Object` | `(String, Any)` |
| async sequence | `await` 元素类型 |

### `for await in`

`for await x in asyncSeq` 逐个等待异步元素，详见 [第 18 章 并发](18-concurrency.md#for-await-in)。

## break / continue

```syx
while true {
    if done {
        break         // 跳出最近循环
    }
    if skip {
        continue      // 进入下一轮
    }
}
```

| 形式 | 行为 |
|------|------|
| `break` | 跳出 **最近一层** 循环或 `switch` |
| `continue` | 跳到 **最近一层** 循环下一轮 |
| `break label` | 跳出指定标签的循环（**部分实现**） |
| `continue label` | 跳到指定标签的循环下一轮（**部分实现**） |

> **标签（label）：** 语法 `label: while (cond) { … break label … }`；SYX 标签在嵌套循环里识别，但跨函数 / 闭包识别仍演进中。

## switch

`switch` 是 SYX 表达多路分支的核心；支持 **模式 + 守卫 + exhaustiveness**：

```syx
switch value {
case 1 {
    print("one")
}
case 2, 3, 5, 7 {
    print("small prime")
}
case 0..<10 {
    print("single digit")
}
case .ok(let data) {
    print(data)
}
case .err(let e) where e.count > 0 {
    print(e)
}
default {
    print("other")
}
}
```

| 形式 | 说明 |
|------|------|
| `case pattern` | 模式匹配 |
| `case pattern where guard` | 守卫（`guard` 为真才执行） |
| `case pat1, pat2` | 多模式（`\|` 在 parser 内部，`pat1, pat2` 写法合法） |
| `case 0..<10` | 范围模式（`Range<Int>` 上下文） |
| `case .variant(let x)` | 枚举/Result 变体模式 |
| `default` | 默认分支；不强制（`enum` / `final class` 封闭域时可省略，但有 warning） |

### switch 块语法（重要）

```syx
switch x {
case 1 {           // 花括号包裹 case 体；与 Swift 不同
    print("one")
}
case 2 {
    print("two")
}
}
```

> **关键点：** SYX **`case` 后跟 `{ }` 块**，**不是** `case 1:` 单行 + 缩进。早期 Phase 接受 `case 1:` 写法（与 Swift 一致），但**推荐** 块式 `case 1 { }` 风格。

### 模式（详细）

```syx
switch result {
case .ok(let value) {                      // 解构变体
    print("ok: \(value)")
}
case .err(let message) {                   // 命名关联值
    print("err: \(message)")
}
case .failure(code: let c, message: let m) {  // 命名参数解构
    print("fail code=\(c) msg=\(m)")
}
case .empty {                              // 无关联值
    print("empty")
}
default {
    print("other")
}
}
```

详见 [第 9 章 枚举](../../docs/adr/architecture-decisions.md) 与 [语言参考：模式](../language-reference/07-patterns.md)。

### 范围 case

```syx
switch n {
case 0..<10 { print("single digit") }
case 10..<100 { print("two digits") }
case 100..<1000 { print("three digits") }
default { print("large") }
}
```

闭区间 `a...b` **部分实现**；推荐半开 `a..<b` + 末尾 `Int.max`。

### exhaustiveness 检查

`switch` 在 `enum` / `final class` / `Int` / `Bool` 等封闭域上自动做 exhaustiveness 校验；缺 `default` 且未覆盖全部 case → **S007**。

```syx
enum Color { case red, green, blue }

func describe(_ c: Color) -> String {
    switch c {
    case .red { return "red" }
    case .green { return "green" }
    case .blue { return "blue" }
    }                                       // ✅ 全覆盖
}
```

非封闭域（如 `String`、`Any`）需 `default`。

### 守卫 `where`

```syx
switch n {
case let x where x < 0 {
    print("negative: \(x)")
}
case let x where x % 2 == 0 {
    print("even: \(x)")
}
case let x {
    print("odd: \(x)")
}
}
```

`where` 表达式必须为 `Bool`；多 `where` 与多模式用 `,` 分隔（parser 接受）。

### fallthrough

`fallthrough` **拒绝**（编译错误）。SYX `case` **不** 自动 fallthrough；需要在 case 体末尾显式 `break`（默认行为已是 break，可省略）。

## if case

```syx
if case .ok(let data) = result {
    print(data)
} else {
    print("not ok")
}
```

`if case` 是 `switch` 单分支的便捷形式。模式同 `switch` case。

## if let / guard let

解包 `Optional`：

```syx
if let x = optional {
    print("got: \(x)")
} else {
    print("nil")
}

guard let y = optional else {
    return                              // 早退（else 必须退出：return / throw / break / continue）
}
print("y is \(y)")                       // 此处 y 已被解包，非 Optional
```

| 形式 | 行为 |
|------|------|
| `if let x = opt { }` | 非 `nil` 进入分支，`x` 是解包后的值；可选 `else` |
| `if let x = opt, let y = other` | 链式解包（逗号分隔多条件） |
| `guard let x = opt else { … }` | 非 `nil` 继续；`nil` 进入 `else` **必须** 退出（`return` / `throw` / `break` / `continue` / `fatalError`） |
| `guard let x = opt, cond else` | 多条件 + 守卫 |

> **`else` 必须退出：** `guard` 的 `else` 块若不退出（缺 `return` 等）→ 编译错误 **S020**。

## throw

```syx
func loadFile(path: String) throws -> String {
    if !File.exists(path) {
        throw "not found: \(path)"
    }
    return File.read(path)
}
```

`throw "msg"` 抛出字符串异常（MVP 类型）。`try` / `catch` 见 [第 17 章 错误处理](../../docs/adr/architecture-decisions.md)。

## 早退

SYX 提供 `return` / `throw` / `break` / `continue` 四种早退路径。函数体内 `return` 可省（隐式返回 `Void`）：

```syx
func maybeDouble(x: Int?) -> Int {
    guard let v = x else {
        return 0                          // ✅ 显式
    }
    return v * 2
}
```

## 控制流与闭包

闭包可携带控制流（如 `for` / `if`），但 **break / continue** 默认只作用于最近循环（不能跨闭包边界）。如需跳出闭包外层循环，用 **label**（**部分实现**）：

```syx
outer: for item in items {
    for sub in item.subs {
        if sub.shouldStop {
            break outer                    // ⚠️ 标签支持演进中
        }
    }
}
```

## 完整示例

```syx
import IO

func classify(path: String) -> String {
    if !File.exists(path) {
        return "missing"
    }

    let content = File.read(path)
    let n = content.count

    switch n {
    case 0 { return "empty" }
    case 1..<100 { return "tiny" }
    case 100..<10_000 { return "medium" }
    default { return "large" }
    }
}

func main() {
    for p in ["/etc/hostname", "/tmp/empty.txt", "/tmp/big.txt"] {
        print("\(p) → \(classify(path: p))")
    }
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **P001** | 语法解析错误（括号不匹配、操作符缺失等） | 检查附近语法 |
| **S004** | `if` 条件非 `Bool` | 显式比较 |
| **S007** | `switch` 非穷尽（`enum` / 封闭域） | 加缺失 case 或 `default` |
| **S010** | 模式不匹配 | 检查 enum case 是否完整 |
| **S020** | `guard let` `else` 未退出 | 加 `return` / `throw` 等 |
| **R003** | 下标越界 | 检查索引 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `if x` 无括号 | ✅ **一致**；`if x` 与 `if (x)` 都合法（推荐省略） |
| `switch x` | ✅ **一致**；`switch x` 与 `switch (x)` 都合法（推荐省略） |
| `case 1:` 缩进风格 | 推荐 `case 1 { }` 块风格 |
| `fallthrough` | **拒绝**（编译错误） |
| 标签 `break label` | **部分实现**（演进中） |
| `where case` | **未实现**（用 `if case` 链式） |
| `if #available` | **未实现** |
| `do { try … } catch { }` | 用 `try { } catch { }`（无 `do` 关键字） |

---

**下一章** → [函数](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：语句](../language-reference/04-statements.md) · [第 7 章 函数](../../docs/adr/architecture-decisions.md) · [第 17 章 错误处理](../../docs/adr/architecture-decisions.md)
