# 基本运算符

**实现状态：** **已实现**（算术 / 比较 / 逻辑 / 赋值 / 范围 / 空合并 / 强制解包 / 恒等子集）；自定义 `operator func` 见 [第 30 章 高级运算符](../../docs/adr/architecture-decisions.md)

SYX 运算符主要分 **算术、比较、逻辑、赋值** 四大类。底层通过 [Core Host](../../docs/adr/architecture-decisions.md) 的 intrinsic 指令实现，与 Swift 大体一致；不依赖任何运行时解释。

## 算术运算符

```syx
let a = 10 + 3      // 13
let b = 10 - 3      // 7
let c = 10 * 3      // 30
let d = 10 / 3      // 3（整数除）
let m = 10 % 3      // 1（取模，符号与左操作数一致）
let n = -a           // 一元负
```

| 运算符 | 含义 | 适用类型 |
|--------|------|----------|
| `+` `-` `*` `/` `%` | 加 / 减 / 乘 / 除 / 取模 | 整数、浮点 |
| `-x` | 一元负 | 数值 |
| `+x` | 一元正（语法允许） | 数值 |

> **溢出：** VM / MIR / WASM 路径整数算术与定宽运算 **恒按二补回绕**（`wrapping_*` 地板；与 debug/release 无关）。书本曾写的 debug **R011** panic **未交付**。请在写循环/位运算时显式检查边界，或换更大类型。
>
> **除零：** 整数 `/0` 或 `%0` 触发 **R012 division by zero**；浮点除零结果为 `inf` / `nan`。

**混合类型：** `Int + Double` 走 widening rule（`widen_for_binary`）。同宽整数（如 `Int8 + Int8`）保持该宽；`Int8 + Int` 提升为平台 `Int`。`Int + UInt` 触发 **S014**（避免静默符号位 wrap）。标量位运算 `& | ^ << >> ~` 与算术同走该 kind（`Int8 << 1` 按 8-bit wrap；`Int & UInt` → **S014**）。有序比较 `< <= > >=` 同走该 kind（如 `UInt8` 按无符号比较；`==`/`!=` 仍走值相等）。一元 `-` / `~` 同走操作数 kind（如 `-(-128 as Int8)` 按 8-bit wrap 仍为 `-128`）。

## 比较运算符

```syx
x == y      // 相等
x != y      // 不等
x === y     // 恒等（引用/类型敏感子集；详见下）
x !== y     // 恒等取反
x < y       // 小于
x <= y      // 小于等于
x > y       // 大于
x >= y      // 大于等于
```

| 形式 | 适用 |
|------|------|
| `==` / `!=` | 值相等（`Equatable` 协议 witness；自定义类型需 `data class` + `@Equatable`） |
| `===` / `!==` | 恒等（引用类型比较引用 / 元类型 tag；详见 [第 30 章 高级运算符](../../docs/adr/architecture-decisions.md)） |
| `<` `<=` `>` `>=` | 有序比较（`Comparable` 协议 witness；`data class` + L1 合成或自定义） |

> **不可比较：** 没有 `Comparable` 一致性的类型使用 `<` 等有序比较 → **S004** 类型不匹配。

## 逻辑运算符

```syx
if (a && b) { … }    // 短路与
if (a || b) { … }    // 短路或
if (!flag) { … }     // 逻辑非
```

| 运算符 | 含义 | 短路 |
|--------|------|------|
| `&&` | 逻辑与 | ✅ 右侧只在左侧为 `true` 时求值 |
| `\|\|` | 逻辑或 | ✅ 右侧只在左侧为 `false` 时求值 |
| `!` | 逻辑非 | 一元 |

> 操作数必须为 `Bool`；非 `Bool` 表达式条件 → **S004**。

## 空合并与可选链

```syx
let v = optional ?? defaultValue     // nil 合并
let len = maybeString?.count        // 可选链
let v = maybeString!                 // 强制解包
```

| 运算符 | 含义 | 失败行为 |
|--------|------|----------|
| `??` | nil 合并 | 左 `nil` 时取右 |
| `?.` | 可选链 | 链路任一环节 `nil` → 整体 `nil` |
| `!` | 强制解包 | `nil` → 运行时错误 **R007** |

```syx
let s: String? = "hello"
print(s?.count)        // Optional(5)  ← 注意：仍是 Optional
let n: Int = s?.count ?? 0   // 5
let bad: Int = s!.count      // 5（强制解包成功）
```

## 赋值与复合赋值

```syx
x = 1          // 简单赋值
x += 2         // 等价 x = x + 2
x -= 1
x *= 2
x /= 2
x %= 3
obj.field = v
arr[i] = v
```

| 形式 | 含义 | 失败 |
|------|------|------|
| `=` | 简单赋值 | 对 `let` → **S003** |
| `+=` `-=` `*=` `/=` `%=` | 复合赋值 | 同上 |
| `obj.field = v` | 成员赋值 | 字段只读或对象不可变 → **S003** |
| `arr[i] = v` | 下标赋值 | 越界 → 运行时 **R003** |

> **赋值表达式：** SYX 赋值 **不返回** `Void` 值用于链式（与 Swift 一致）。`a = b = c` 编译错误 **S004**。
>
> **定宽复合赋值：** `Int8` 等定宽整数的 `+=`/`-=`/`*=`/`/=`/`%=`/`&=`/`|=`/`^=`/`<<=`/`>>=` desugar 到带 `widen_for_binary` kind 的二元运算（如 `120i8 += 10` → `-126`；`-1i8 &= 15` → `15`；`1i8 |= 15` → `15`；`15i8 ^= 1` → `14`；`64i8 <<= 1` → `-128`；`-128i8 >>= 1` → `-64`）。平台 `Int` 的 `+=`/`-=` 可走 host 特判。位复合文法（含移位复合）已齐。

## 范围运算符

```syx
for i in 0..<10 { … }   // 半开 [0, 10)
for i in 1...5 { … }    // 闭区间 [1, 5]（整字面量端点）
```

| 形式 | 含义 | 状态 |
|------|------|------|
| `a..<b` | 半开 `[a, b)` | **已实现** |
| `a...b` | 闭区间 `[a, b]`（**整字面量端点**） | **MVP GA**（Phase 139） |
| `a..<` | 无限右开 | **未实现** |
| `..<b` | 无限左开 | **未实现** |

范围表达式的类型为 `Range<Int>`（`for-in` 与 `contains` 上下文）。

```syx
print((1..<10).contains(5))    // true
print((1..<10).contains(10))   // false
print((1...10).contains(10))  // true
print((1...10).contains(11))   // false
```

## 一元前缀

| 运算符 | 含义 | 适用 |
|--------|------|------|
| `-` | 一元负 | 数值 |
| `!` | 逻辑非 / 强制解包 | `Bool` / `Optional` |
| `try` | 错误传播 / `?` 链 | throws 调用 |
| `await` | 异步等待 | async 上下文 |
| `&` | 借用表达式 | `&T` / `&mut T` |

## 三元（条件）运算符

SYX **不直接** 提供 `?:` 三元运算符；用 `if` 表达式或临时变量替代：

```syx
// ✅ 推荐
let label = if (x > 0) { "pos" } else { "non-pos" }

// ❌ 不支持
// let label = x > 0 ? "pos" : "non-pos"   // 编译错误
```

> **为何没有 `?:`：** 与 Swift 一致；目标是鼓励可读性，三元的嵌套让代码难读。

## 运算符优先级

完整优先级表见 [语言参考：表达式](../language-reference/03-expressions.md#优先级摘要)。**从高到低** 摘要：

| 级别 | 运算符 |
|------|--------|
| 1（最高） | `.` `[]` `()` `?.` `!` `await`（后缀） |
| 2 | `-` `!` `try`（一元前缀） |
| 3 | `*` `/` `%` |
| 4 | `+` `-` |
| 5 | `..<` `...` |
| 6 | `<` `<=` `>` `>=` |
| 7 | `==` `!=` `===` `!==` |
| 8 | `&&` |
| 9 | `\|\|` |
| 10 | `??` |
| 11（最低） | `=` `+=` `-=` `*=` `/=` `%=` |

> **括号优先：** 任意位置可加 `()` 强制优先级，无副作用。

## 数值提升与字面量规则

```syx
let a: Int = 1
let b = a + 1.0   // Double（`1.0` 是 Double 字面量；`a` 提升为 Double）
let c = a + 1     // Int（`1` 是 Int 字面量）
let d = 1_000     // Int（下划线可读性）
```

| 字面量 | 推断类型 |
|--------|----------|
| `42` | `Int` |
| `42i8` | `Int8` |
| `3.14` | `Double` |
| `3.14f32` | `Float32` |
| `"text"` | `String` |
| `true` | `Bool` |
| `nil` | 上下文推断（通常 `Optional<T>`） |

## 示例：累加

```syx
func main() {
    var sum = 0
    for i in 0..<5 {
        sum += i        // 0+1+2+3+4 = 10
    }
    print(sum)          // 10
}
```

## 自定义运算符

`operator func` 允许声明新运算符（[第 30 章](../../docs/adr/architecture-decisions.md)）：

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }

let result = 5 |> { $0 * 2 }    // 10
```

完整语法、优先级组、自定义中缀 / 前缀 / 后缀运算符见 [第 30 章 高级运算符](../../docs/adr/architecture-decisions.md)。

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S003** | 对 `let` 赋值 | 改 `var` |
| **S004** | 运算符两侧类型不兼容 | 显式转换 / 换类型 |
| **S008** | 窄化（`Int64 → Int8`） | 显式截断 / 检查范围 |
| **S014** | 有符号/无符号混合 | 显式 cast |
| **R007** | `!` 解包 `nil` | 用 `?.` / `??` / `if let` |
| **R011** | 整数溢出 Display（**未交付**；现行为 wrap） | 检查边界；可换更大类型；同码可被其它域复用（如 HTTP client） |
| **R012** | 整数除零 | 检查分母 |
| **R003** | 数组 / 字符串下标越界 | 显式范围检查 |

完整诊断见 [附录 B 诊断代码索引](appendix-b-diagnostic-index.md)。

## 与 Swift 的差异

| 点 | SYX |
|----|-----|
| `===` | 支持，引用/类型敏感子集 |
| 自定义运算符 | **GA**（`operator func` + 三后端 parity） |
| 溢出 | 恒二补回绕（≠ Swift 默认 trap；整数溢出 Display **R011** 未交付） |

---

**下一章** → [字符串和字符](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：表达式](../language-reference/03-expressions.md) · [第 30 章 高级运算符](../../docs/adr/architecture-decisions.md) · [第 32 章 运算符 vs Core Host](../../docs/adr/architecture-decisions.md)
