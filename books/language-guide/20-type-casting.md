# 类型转换

**实现状态：** `is` 类型检查 / `as?` 失败转 nil / `as!` 失败抛错 / 数值字面量推断 / Optional 包装 **已实现**；完整 Swift 风格 cast（泛型擦除 / 协议存在性）**部分实现**

类型转换是「在运行期 / 编译期把一个值视为另一种类型」。SYX 提供 **隐式提升**、**显式 `as` / `as?` / `as!`、类型检查 `is`** 三种方式。

## 数值提升（隐式）

混合数值运算时编译器按语义规则提升（`widen_for_binary`）：

```syx
let x: Int = 1
let y = x + 1.0                          // Double（Int → Double）
let z: Int8 = 1
let w = z + 100i32                        // Int32（Int8 → Int32）
```

| 规则 | 说明 |
|------|------|
| 整数字面量 | 默认 `Int`；与 `Double` 运算时 `Int → Double`。显式 `Float` / `Double` 注解下，无后缀整数按该浮点类型解释（`let x: Double = 1`） |
| 有符号 + 无符号 | 触发 **S014**（避免静默符号位 wrap） |
| 整数 + 浮点 | 整数提升为浮点 |
| 浮点 + 浮点 | 同宽返回 `Double`，`Float + Double = Double` |
| `Int8` + `Int8` | 结果仍为 `Int8`（溢出按 2 的补码回绕） |
| `Int8` + `Int` | 提升为平台 `Int` |
| `Int8` + `Int32` | 提升为 `Int32` |

> **同符号整数窄化（`Int` → `Int8`）：** 赋值 / 初始化允许，经 `NumericCast`；值越界 → 运行时 **R003**（≠ 静默截断）。字面量越界仍是编译期 **S011**；错误后缀字面量（`8i16` → `Int8`）仍是 **S004**。
>
> **浮点 → 整数（`Double → Int8`）：** 赋值仍不隐式（**S004**）；显式 `as` / `as?` 向 0 截断（越界 R003 / nil）。S008 专用于跨 actor 字段，不是窄化码。

**诊断：** S014（有符号/无符号混合）、S004（类型不兼容）、R003（窄化越界）

## is 类型检查

运行时类型检查，结果为 `Bool`：

```syx
let a = [1, 2, 3]
print(a is Array)                         // true
print(a is Array<Int>)                    // true（运行时按基类型匹配；元素类型不擦除校验）

class Dog { init() {} }
let d = Dog()
print(d is Dog)                           // true
```

> **数值目标禁止：** `1 is Int` ❌（编译错误 **S046**）；`is` 主要用于 class / protocol / 集合基类型。

### 支持的目标类型

| 目标 | 匹配值 |
|------|--------|
| `Array` / `Array<T>` | 数组字面量 / `Value::Array` |
| `Set` | 集合 |
| `Dictionary` | 字典 |
| `String` / `Bool` / `Int*` / `Float*` / `Double` | 对应标量 |
| 用户 `class` / `data class` | 带 `__type` / `ObjectRef` 的实例 |
| `protocol`（existential） | 实现该协议的值 |

```syx
let v: any CustomStringConvertible = "hello"
print(v is String)                        // true
```

## 显式转换

### as? — 失败转 nil

```syx
class Animal { init() {} }
class Dog: Animal {
    override init() { super.init() }
    func bark() { print("woof") }
}

let a: Animal = Dog()
if let d = a as? Dog {
    d.bark()                              // "woof"
}
```

| 场景 | `as?` 行为 |
|------|-----------|
| 成功 | 返回目标类型（`Dog`） |
| 失败 | 返回 `nil` |
| 同符号整数（`Int as? Int8`） | 范围内转换；越界 → `nil`（≠ R003） |

### as! — 失败抛错

```syx
let d = a as! Dog                         // 失败运行时 R003
```

| 场景 | `as!` 行为 |
|------|-----------|
| 成功 | 返回目标类型 |
| 失败 | 运行时错误 **R003** |

### as — 强转（编译期已知）

```syx
let a: Animal = Dog()
let d = a as Dog                          // 已知为 Dog，编译通过

let n: Int = 8
let n8 = n as Int8                        // 同符号整数 → NumericCast；越界 R003

let d: Double = 1.5
let i = d as Int                          // 向 0 截断 → 1；越界 R003
let i8 = d as? Int8                       // 成功 → 1；越界 → nil
```

> **状态：** `as` 在编译期已知类型转换时使用；运行时失败抛错。同符号整数宽窄互转与显式 `Double`/`Float` → 整数走 `NumericCast`（`as` 越界 **R003**；`as?` 越界 → `nil`）。赋值仍不隐式 float→int。完整 Swift 风格 `as` / `as!` / `as?` 区分仍在细化中。

## Optional 转换

```syx
let x: Int = 42
let y: Int? = x                           // T → T? 隐式包装
let z: Int = y ?? 0                       // T? → T（nil 合并）
```

| 方向 | 形式 | 失败 |
|------|------|------|
| `T` → `T?` | 隐式 | 不失败 |
| `T?` → `T` | `??` / `!` / `if let` | `!` 失败 **R007** |
| `T` → `U` (related) | `as?` / `as!` | `as!` **R003** |

## 协议与存在类型

```syx
protocol Describable {
    func describe() -> String
}

class City: Describable {
    let name: String
    init(name: String) { self.name = name }
    func describe() -> String { return name }
}

let c: any Describable = City(name: "Beijing")
if let city = c as? City {
    print(city.name)                      // "Beijing"
}
```

> **existential：** `any Describable` 是存在类型；运行时通过 witness 表调度。详见 [第 25 章 不透明类型](../../docs/adr/architecture-decisions.md)。

## 数字字面量转换

```syx
let n: Int = 42
let n8: Int8 = n                          // 同符号整数窄化（越界 → R003）
let n8b: Int8 = 42i8                      // 字面量后缀直接选类型
let f: Double = Double(n)                 // 显式转换
let f2 = 3.14.toFloat()                   // 部分实现
```

| 转换 | 形式 |
|------|------|
| `Int → Int8` | 赋值/初始化隐式窄化；越界 → **R003** |
| `Int → Double` | widening：隐式 |
| `Double → Int` | 显式 `as` / `as?`：向 0 截断；越界 `as` → **R003**、`as?` → `nil`；赋值仍 **S004** |
| `Int8 → Int` | widening：隐式 |
| `String → Int?` | `Int.fromString("42")` → `ok(42)` |

```syx
let r = Int.fromString("42")
switch (r) {
case .ok(let n) { print("got \(n)") }     // "got 42"
case .err(let e) { print("err: \(e)") }
}
```

## 跨类转换

```syx
class Animal { init() {} }
class Dog: Animal { init() { super.init() } }
class Cat: Animal { init() { super.init() } }

let a: Animal = Dog()
if let d = a as? Dog {
    print("it's a dog")
} else if let c = a as? Cat {
    print("it's a cat")
}
```

- 子类 → 父类：upcast，**总是**成功（用 `as`）
- 父类 → 子类：downcast，可能失败（用 `as?` / `as!`）
- 不相关类型：编译错误 **S004**

## `as` 与 `as!` 的取舍

| 形式 | 失败 | 何时用 |
|------|------|--------|
| `as?` | `nil` | 类型不确定 |
| `as!` | panic | 100% 确定 |
| `as` | panic | 编译期已知 |

## 完整示例

```syx
class Shape { init() {} func area() -> Double { return 0 } }
class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r; super.init() }
    override func area() -> Double { return 3.14159 * r * r }
}
class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s; super.init() }
    override func area() -> Double { return s * s }
}

func describe(_ s: Shape) -> String {
    if let c = s as? Circle {
        return "circle r=\(c.r), area=\(c.area())"
    }
    if let sq = s as? Square {
        return "square s=\(sq.s), area=\(sq.area())"
    }
    return "shape"
}

func main() {
    let shapes: Array<Shape> = [Circle(r: 2.0), Square(s: 3.0)]
    for s in shapes { print(describe(s)) }
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | 转换不兼容（含 float→int、错误后缀） | 检查类型关系 |
| **S008** | 跨 actor 直接访问实例字段 | 经消息 / 隔离 API |
| **S014** | 有符号/无符号混合 | 显式 cast |
| **S046** | `is` 数值目标 | 改 `==` 比较 |
| **R003** | `as!` 失败；整数窄化越界 | 改 `as?` / 保证值在目标范围内 |
| **R007** | `!` 解包 nil | 用 `?.` / `if let` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `is` 类型检查 | **已实现**（集合 + 标量 + class） |
| `as?` / `as!` / `as` | **已实现**（运行时 cast host） |
| 完整 cast 协议存在性 | **部分实现**（基础 witness） |
| `AnyObject` / `Any` | 弱支持（用 `Object` / `any P`） |
| 泛型擦除 | **部分实现** |

---

**下一章** → [嵌套类型](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：类型](../language-reference/02-types.md) · [第 9 章 枚举](../../docs/adr/architecture-decisions.md) · [第 25 章 不透明类型](../../docs/adr/architecture-decisions.md)
