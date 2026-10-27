# 模式

**实现状态：** **已实现**（`switch` / `if case` / `if let` / `guard let`）；变体 / 元组 / 通配符 / 范围 / 关联值解构 / where 守卫 / 类型模式 **已实现**；`case let x as T` **未实现**；`is` 模式 **部分实现**

模式用于匹配值结构并绑定变量。SYX 模式与 Swift 类似：在 `switch` / `case` / `let` 上下文使用。

---

## 模式语法

```
pattern ::= '_'                                  -- 通配
         |  identifier                           -- 绑定
         |  'let' pattern                        -- 显式绑定
         |  literal                              -- 字面量
         |  range-pattern                        -- 范围
         |  variant-pattern                      -- 变体
         |  tuple-pattern                        -- 元组
         |  type-identifier                      -- 类型标注
         |  '(' pattern ')'                      -- 括号
         |  pattern 'as' type                    -- 类型模式（**部分**）

range-pattern    ::= expr '..<' expr
variant-pattern  ::= '.' identifier ('(' pattern? (',' pattern)* ')')?
tuple-pattern    ::= '(' pattern (',' pattern)* ')'
```

## 变体模式

```
variant-pattern ::= '.' identifier ('(' pattern? (',' pattern)* ')')?
```

```syx
.ok(let value)
.err(let message)
.success(let data)
.failure(code: let c, message: let m)
.failure(let c, let m)                         // 位置解构
.empty                                         // 无关联值
```

| 形式 | 含义 |
|------|------|
| `.ok` | 无关联值 |
| `.ok(let x)` | 解构单关联值，绑定 `x` |
| `.failure(code: let c, message: let m)` | 命名关联值解构 |
| `.failure(let c, let m)` | 位置解构 |
| `.failure(let c, _) ` | 部分解构（`_` 显式忽略） |

## 元组模式

```syx
(a, b)                                        // 位置
(let x, let y)                                // 显式绑定
(1, let name)                                 // 混合
```

> 元素模式按位置匹配；类型须与 subject 兼容。

## 字面量模式

```syx
case 1 { … }                                  // 整数
case "hello" { … }                            // 字符串
case true { … }                               // 布尔
case nil { … }                                // nil
```

> 字面量模式按值比较；`==` 走 `Equatable` witness。

## 范围模式

```syx
case 0..<10 { … }                             // 半开
case 10..<100 { … }
case 100..<1000 { … }
```

> 闭区间 `a...b` **部分实现**。`0..<n` 走 `Range<Int>`。

## 通配符 `_`

```syx
let (_, x) = (1, 2)                           // 丢弃第一个
case .ok(let _) { … }                         // 丢弃关联值
```

## `let` 显式绑定

```syx
case let x { … }                              // 绑定整个 subject
case .ok(let v) { … }                         // 绑定关联值
```

> `let x` 等价于 `x`（在模式中默认绑定）；`let` 关键字显式表达意图。

## switch 中的模式

```syx
switch (result) {
case .ok(let v) {
    print(v)
}
case .err(let e) where (e.length > 0) {
    print(e)
}
default {
    print("fallback")
}
}
```

> 完整 `SwitchCase { pattern, guard, body }`。

## if case

```syx
if case .ok(let x) = result {
    print(x)
}
```

> `if case` 是 `switch` 单分支的便捷形式；模式与 `switch` case 一致。

## if let / guard let

```syx
if let x = optional { … }
if let x = optional, let y = other, cond { … }  // 多条件 + 守卫
guard let x = optional else { return }
```

> **多条件链式：** 用 `,` 分隔；任一条件失败 → 整体分支跳过。
> **guard else 必须退出：** 否则 **S020**。

## 类型模式（部分实现）

```syx
case let x as String { … }                     // ⚠️ 部分实现
```

> 当前实现仅在 cast host 路径支持基础类型；完整类型模式 → 后续 Phase。

## where 守卫

```syx
case .ok(let v) where (v > 0) { … }
case let x where (x % 2 == 0) { … }
```

> `where` 表达式必须为 `Bool`；多 `where` 模式用 `,` 分隔。

## 模式与类型

- 模式类型须与 subject 兼容
- 不匹配时分支跳过（非封闭域）或编译错误（封闭域缺 `default` → **S007**）

| 场景 | 缺 `default` 行为 |
|------|-------------------|
| `enum` / `final class` / `Bool` | **S007**（必须穷尽） |
| `Int` / `String` | 警告（**部分**） |
| 协议 / `Any` | 必须 `default` |

## 模式与字面量推断

```syx
let p: any Named = obj
if case let x where (x is User) { … }         // x: any Named（子类型 User）
```

> 模式内类型推断按 `case` 后子句；与 let-bind 不同（subject 类型已定）。

## exhaustiveness 检查

封闭域（`enum` / `final class` / `Bool`）上 `switch` 自动做穷尽检查；缺 `default` 且未覆盖全部 case → **S007**。

```syx
enum Color { case red, green, blue }

func describe(_ c: Color) -> String {
    switch (c) {
    case .red { return "red" }
    case .green { return "green" }
    case .blue { return "blue" }
    }                                          // ✅ 全覆盖
}
```

## 未实现 / 计划中

| Swift 模式 | SYX 状态 |
|------------|---------|
| `case let x as T` | **部分实现** |
| 完整值绑定 `case .red` 无绑定 | 已支持（`.red { }`） |
| `_` 显式忽略 | **已实现** |
| 嵌套模式 / 复杂解构 | **已实现**（基础） |
| 区间数组 `[1, 2, ...]` | **未实现** |
| 正则模式 | **未实现** |

## 完整示例

```syx
enum Tree {
    case leaf(Int)
    case branch(left: Tree, right: Tree)
}

func depth(_ t: Tree) -> Int {
    switch (t) {
    case .leaf { return 1 }
    case .branch(let l, let r) { return 1 + max(depth(l), depth(r)) }
    }
}

func describe(_ t: Tree) -> String {
    switch (t) {
    case .leaf(let n) where (n > 0) { return "leaf(\(n))" }
    case .leaf { return "leaf(0)" }
    case .branch(let l, let r) { return "branch(\(describe(l)), \(describe(r)))" }
    }
}

func main() {
    let t = Tree.branch(left: .leaf(1), right: .leaf(2))
    print(depth(t))                            // 2
    print(describe(t))                         // "branch(leaf(1), leaf(2))"
}
```

---

**诊断：** S004 / S007 / S010 / S012 / S020

**用法示例** → [语言指南：枚举](../language-guide/09-enumerations.md) · [控制流](../language-guide/06-control-flow.md) · [高级运算符](../language-guide/30-advanced-operators.md)

**参见** → [语句](../../docs/adr/architecture-decisions.md) · [表达式](../../docs/adr/architecture-decisions.md) · [类型](../../docs/adr/architecture-decisions.md)

**测试：** `fixtures/format/switch_where_mvp` · `fixtures/format/switch_where_mvp` · `fixtures/format/switch_where_mvp`
