# 不透明类型和封装协议类型

**实现状态：** `some P`（不透明返回类型）**已实现**（Phase 11 GA、`fixtures/format/opaque_some_mvp` / `fixtures/parity/protocol_witness.sy`）；`any P`（存在类型）**已实现**（witness 表动态分派）

SYX 提供两种「隐藏具体类型」的方式：

- `some P` — **不透明返回类型**：编译期隐藏，调用方只能按 `P` 使用
- `any P` — **存在类型**：运行时通过 witness 表分派，调用方拿到的是「某种 `P`」

## some — 不透明返回类型

`some` 隐藏具体类型信息，但保留编译期类型推导：

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func makeShape() -> some Shape {            // 返回类型：某个 Shape（编译期已知）
    return Square(s: 2.0)                   // 实际是 Square
}

func main() {
    let shape = makeShape()
    print(shape.area())                     // 4.0（按 Shape 接口）
}
```

| 行为 | 说明 |
|------|------|
| 调用方 | 看到的是 `some Shape`（满足 `Shape` 协议） |
| 内部 | 实际是 `Square`，编译期具体 |
| 实现变更 | `Square` → `Circle` 不破坏调用方（API 稳定） |
| 性能 | 零虚调度（编译期静态） |

> **限制：** 同一函数的所有返回路径必须返回**同一具体类型**。`if cond { return A() } else { return B() }` 编译错误。

测试：`fixtures/format/opaque_some_mvp` · `fixtures/parity/protocol_witness.sy`

## any — 存在类型

`any` 是「某种 `P`」的容器；运行时用 witness 表动态分派：

```syx
protocol Shape {
    func area() -> Double
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let s: any Shape = Circle(r: 2.0)
    print(s.area())                         // 12.56636
}
```

| 行为 | 说明 |
|------|------|
| 调用方 | 看到的是 `any Shape`（存在类型） |
| 内部 | 实际是 `Circle`，运行时用 witness 表分派 |
| 多具体类型 | 同一变量可持有 `Square` / `Circle` 等不同实现 |
| 性能 | 间接调度（witness table） |

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let shapes: Array<any Shape> = [
        Square(s: 2.0),
        Circle(r: 3.0)
    ]
    for s in shapes {
        print(s.area())                     // 4.0, 28.27431
    }
}
```

测试：`fixtures/parity/protocol_witness.sy`

## some vs any 对比

| 维度 | `some P` | `any P` |
|------|----------|---------|
| 编译期类型 | **保留**（具体类型） | **擦除**（仅 `P`） |
| 运行时调度 | **静态**（无 witness） | **动态**（witness 表） |
| 多类型值 | ❌（一个函数必须返回同一类型） | ✅（`Array<any P>` 可混合） |
| 性能 | 零虚调度 | 间接调用 |
| 反射 | 可见 | 仅 `P` 接口 |

## 使用场景

| 场景 | 选 |
|------|-----|
| 函数返回单一具体类型，只暴露协议 | `some P` |
| 存储多种符合 `P` 的类型 | `any P` |
| API 稳定 + 性能敏感 | `some P` |
| 多态集合 / 工厂 | `any P` |

```syx
protocol Shape {
    func area() -> Double
}

class Square: Shape {
    var s: Double
    init(s: Double) { self.s = s }
    func area() -> Double { return s * s }
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

// 工厂：返回多种实现 → `any P`
func makeShape(kind: String) -> any Shape {
    if kind == "circle" { return Circle(r: 1.0) }
    return Square(s: 1.0)
}

// API：返回单一实现 → `some P`
func defaultShape() -> some Shape { return Square(s: 1.0) }

func main() {
    print(makeShape(kind: "circle").area())
    print(defaultShape().area())
}
```

## `any P` 的 cast

```syx
protocol Shape {
    func area() -> Double
}

class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * r * r }
}

func main() {
    let s: any Shape = Circle(r: 2.0)
    if let c = s as? Circle {
        print("circle r=\(c.r)")            // "circle r=2.0"
    }
}
```

> **状态：** `as?` / `as!` 运行时 cast 已实现（详见 [第 20 章](20-type-casting.md)）。

## `some P` 的约束

```syx
func make() -> some Equatable {             // 满足 Equatable 即可
    return 42                                // 实际是 Int
}

func makeBox<T: Equatable>(_ v: T) -> some Equatable {
    return v
}
```

## `any P` 集合的常用模式

```syx
protocol Drawable {
    func draw()
}

class Button: Drawable { func draw() { print("[btn]") } }
class Text: Drawable { func draw() { print("[text]") } }

func main() {
    let ui: Array<any Drawable> = [
        Button(),
        Text(),
        Button()
    ]
    for el in ui { el.draw() }
}
```

## 与泛型结合

```syx
func makePair<T: Equatable>(_ a: T, _ b: T) -> some Equatable {
    return (a, b)                            // 元组 (T, T)
}
```

## 协议存在类型的实现细节

`any P` 在运行时存储：
- 值本体（实际类型）
- witness 表（每个 `P` 方法的函数指针）

```syx
protocol Named {
    func name() -> String
}

class User: Named {
    var n: String
    init(name: String) { self.n = name }
    func name() -> String { return self.n }
}

func main() {
    let p: any Named = User(name: "Alice")
    print(p.name())
    // 内存布局：[ value | witness table ptr ]
    // 调度：p.name() → witness_table.name(value)
}
```

> **代码生成：** `CallWitness` 指令走 witness 表；详见 `syx-codegen`。

## 完整示例

```syx
protocol Logger {
    func log(_ msg: String)
}

class ConsoleLogger: Logger {
    func log(_ msg: String) { print("[console] \(msg)") }
}

class FileLogger: Logger {
    let path: String
    init(path: String) { self.path = path }
    func log(_ msg: String) { print("[file:\(path)] \(msg)") }
}

// 工厂：返回 any P（多种实现）
func makeLogger(kind: String) -> any Logger {
    if (kind == "file") { return FileLogger(path: "/tmp/log") }
    return ConsoleLogger()
}

// API：返回 some P（固定一种）
func defaultLogger() -> some Logger {
    return ConsoleLogger()
}

func main() {
    let l1: any Logger = makeLogger(kind: "file")
    let l2 = defaultLogger()
    l1.log("hello")
    l2.log("world")
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | `some P` 返回多类型 | 统一为同一具体类型 |
| **S010** | 协议不符合 | 加一致性 |
| **P001** | 语法错误 | 检查 `some` / `any` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `some P` 不透明返回 | **已实现** |
| `any P` 存在类型 | **已实现** |
| `some P` 在参数位置 | **未实现**（规划中） |
| `any P` 在返回位置 | **已实现** |
| `some Collection<Element>` 关联类型 | **部分实现** |
| 完整 existential self-conformance | **部分实现**（基础 witness） |
| 性能 | `some P` 零虚调度；`any P` 间接 |

---

**下一章** → [内存管理与垃圾回收](26-memory-management.md)

**参见** → [语言参考：类型](../language-reference/02-types.md) · [第 20 章 类型转换](20-type-casting.md) · [第 23 章 协议](23-protocols.md) · [第 24 章 泛型](24-generics.md)
