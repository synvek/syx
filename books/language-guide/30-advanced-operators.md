# 高级运算符

**实现状态：** 半开范围 / 可选链 / 强制解包 / 空合并 / 模式匹配 / 自定义运算符 **GA**（`operator func` + 三后端 parity）；位运算 / Overflow 行为 **已实现**；前缀 / 后缀 / 中缀自定义 **已实现**；优先级 / 结合性声明 **已实现**

运算符是「**特殊语法**」的函数。SYX 与 Swift 类似：内置运算符是**语言面**（编译器硬编码 + Core Host 桥），用户可声明**自定义运算符**（`infix` / `prefix` / `postfix`），并指定优先级 / 结合性。

## 内置运算符速查

完整列表见 [第 3 章 基本运算符](../../docs/adr/architecture-decisions.md)。本节补「高级」子集：

### 位运算（整数）

| 运算符 | 含义 | 类型 |
|--------|------|------|
| `&` | 按位与 | 整数 |
| `\|` | 按位或 | 整数 |
| `^` | 按位异或 | 整数 |
| `~` | 按位取反（一元） | 整数 |
| `<<` | 左移 | 整数 |
| `>>` | 右移（算术） | 整数 |

```syx
let a: Int = 0b1100                        // 12
let b: Int = 0b1010                        // 10
print(a & b)                                // 8  (0b1000)
print(a | b)                                // 14 (0b1110)
print(a ^ b)                                // 6  (0b0110)
print(~a)                                   // -13
print(a << 2)                               // 48
print(a >> 1)                               // 6
```

> **优先级：** 位运算在比较 `==` 之后、逻辑 `&&` 之前。`a == b & c` 解释为 `a == (b & c)`。

### 自定义前缀 / 后缀 / 中缀运算符

```syx
// 前缀
prefix operator !
func !<T>(value: Optional<T>) -> Bool {
    return value == nil
}

let x: Int? = nil
print(!x)                                   // true（自定义）

// 后缀
postfix operator **
func **<T>(value: T) -> T where T: Numeric {
    return value * value
}

print(3**)                                  // 9

// 中缀
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T {
    return f(value)
}

print(5 |> { $0 * 2 })                      // 10
```

### 优先级 / 结合性

```syx
infix operator ^^ { precedence: 100, associativity: right }
func ^^<T>(a: T, b: T) -> T where T: Numeric {
    // 右结合的「幂」运算符
    return a * a
}
```

| 字段 | 取值 | 含义 |
|------|------|------|
| `precedence` | 整数（0–200） | 数值越高越紧 |
| `associativity` | `left` / `right` / `none` | 同优先级结合方向 |
| `higherThan` / `lowerThan` | 运算符名 | 相对位置 |

> **预定义优先级（部分）：** `*` `/` `%` 为 100；`+` `-` 为 90；`<<` `>>` 为 80；`<` `>` 等为 70；`==` `!=` 为 60；`&` 为 50；`^` 为 40；`\|` 为 30；`&&` 为 20；`||` 为 10。

### 运算符函数与协议约束

```syx
infix operator <*> { precedence: 60 }
func <*> <T: Numeric>(a: Array<T>, b: Array<T>) -> Array<T> {
    var r: Array<T> = []
    for i in 0..<a.count {
        r.append(a[i] * b[i])
    }
    return r
}

let r = [1, 2, 3] <*> [4, 5, 6]            // [4, 10, 18]
```

> **状态：** `operator func` **GA**（Phase 41+），三后端（VM / 统一 MIR / WASM）parity。

## 半开范围

```syx
for i in 0..<10 {                           // [0, 10)
    print(i)
}

let r = 1..<10
print(r.contains(5))                        // true
print(r.contains(11))                       // false
print(r.contains(0))                        // true
```

| 形式 | 状态 | 含义 |
|------|------|------|
| `a..<b` | **已实现** | 半开 [a, b) |
| `a...b` | **部分实现** | 闭区间 [a, b] |
| `a..<` / `..<b` | **未实现** | 单边无限 |

## 可选链 ?.

```syx
let maybe: String? = "hello"
let len = maybe?.count                    // Optional(5)
let bad = (nil as String?)?.count         // nil
```

链式任意长：

```syx
class A { let b: B? = B() }
class B { let c: C? = C() }
class C { let name: String = "x" }

let a: A? = A()
print(a?.b?.c?.name)                        // Optional("x")
print(a?.b?.c?.name?.count)                // Optional(1)
```

> **`?.` 语义：** 链路任一环节为 `nil` → 整体 `nil`。

## 强制解包 !

```syx
let present: String? = "hello"
let s = present!                            // "hello"（运行时确定非 nil）

let empty: String? = nil
let x = empty!                              // ❌ R007
```

> **三选一：** `?.` 链 / `??` 默认 / `!` 强解。

## 空合并 ??

```syx
let opt: Int? = nil
let v = opt ?? 0                            // 0
let v2 = opt ?? (1 + 2)                    // 3（右侧是表达式）
```

> **`??` 优先级：** 低于比较与逻辑；推荐加括号：
> ```syx
> let r = (a ?? 0) + 1
> ```

## if let / guard let

```syx
// if let
if let x = optional {
    print("got: \(x)")
} else {
    print("nil")
}

// guard let（必须早退）
guard let y = optional else {
    return
}
print(y)                                    // 此处 y 非 Optional
```

> **`else` 必须退出：** `guard let` 的 `else` 块若不 `return` / `throw` / `break` / `continue` → 编译错误 **S020**。

## try ?

```syx
let v = risky()?                            // String?（失败 nil）
let v2 = try! risky()                       // String（失败 panic）
```

## 模式匹配

```syx
switch (result) {
case .ok(let v) where (v > 0) {
    print("ok: \(v)")
}
case .err(let e) {
    print("err: \(e)")
}
default { }
}

if case .err(let e) = result {
    print("err: \(e)")
}
```

详见 [第 6 章 控制流](../../docs/adr/architecture-decisions.md) 与 [第 9 章 枚举](../../docs/adr/architecture-decisions.md)。

## 恒等 === / !==

引用或类型敏感相等子集：

```syx
let a = "hi"
let b = "hi"
print(a === b)                              // false（不同对象）
let c = a
print(a === c)                              // true（同一引用）
```

> **状态：** `===` MVP 子集；常用于 `class` / `Object` 引用比较。

## 自定义 Equatable / Comparable

```syx
struct Point: Equatable {
    var x: Int
    var y: Int
    func ==(other: Point) -> Bool { return x == other.x && y == other.y }
}

struct Score: Comparable {
    var value: Int
    func compare(other: Score) -> Int {
        if (value < other.value) { return -1 }
        if (value > other.value) { return 1 }
        return 0
    }
}
```

> **@Equatable / @Hashable / @Codable 宏：** 自动合成 `==` / `hash` / `encode` / `decode`（详见 [第 19 章 宏](../../docs/adr/architecture-decisions.md)）。

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 自定义运算符 | **已实现**（infix / prefix / postfix） |
| 优先级 / 结合性 | **已实现**（precedence 0–200） |
| 半开 `..<` / 闭区间 `...` | 半开 **已实现**；闭区间 **部分实现** |
| 可选链 `?.` | **已实现** |
| 强制解包 `!` | **已实现** |
| 空合并 `??` | **已实现** |
| `===` | **MVP 子集** |
| `#available` | **未实现** |
| 溢出行为 | 恒二补回绕（≠ Swift 默认 overflow trap；≠ 已交付的 debug **R011** panic） |

## 完整示例

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

infix operator <^> { precedence: 60 }
func <^> <T>(a: Array<T>, b: Array<T>) -> Array<T> where T: Numeric {
    var r: Array<T> = []
    for i in 0..<a.count { r.append(a[i] * b[i]) }
    return r
}

prefix operator √
func √(value: Double) -> Double {
    return value.squareRoot()
}

func main() {
    let r = 5 |> { $0 * 2 } |> { $0 + 1 }  // 11
    let dot = [1.0, 2.0, 3.0] <^> [4.0, 5.0, 6.0]  // [4.0, 10.0, 18.0]
    let side = √(16.0)                      // 4.0
    print("r=\(r) dot=\(dot) side=\(side)")
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | 运算符两侧类型不匹配 | 修正类型 |
| **S008** | 窄化（如 `Int64 → Int8`） | 检查范围 |
| **S020** | `guard let` `else` 未退出 | 加 return / throw |
| **R007** | `!` 解包 nil | 用 `?.` / `??` |

---

**下一章** → [SIMD](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：表达式](../language-reference/03-expressions.md) · [第 3 章 基本运算符](../../docs/adr/architecture-decisions.md) · [第 9 章 枚举](../../docs/adr/architecture-decisions.md)
