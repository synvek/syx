# 方法

**实现状态：** 实例方法 / 静态方法 / `override` / 协议默认实现 / `mutating` **已实现**；`@objc` / KVO / `inout` 形参 **未实现**

方法是与类型关联的函数。SYX 方法分四类：实例方法、静态方法、构造方法（init，见 [第 15 章](15-initialization.md)）、析构方法（deinit，见 [第 16 章](16-deinitialization.md)）。本章覆盖前两类与重写。

## 实例方法

```syx
class Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func increment(by n: Int) { value = value + n }
    func describe() -> String { return "Counter(\(value))" }
}

let c = Counter()
c.increment()
c.increment(by: 5)
print(c.describe())                        // "Counter(6)"
```

- 隐式 `self` 可省略（不歧义时）；歧义场景需 `self.x` 显式
- 命名参数与函数一致：`label: value` 必填

## 静态方法（static func）

```syx
class Factory {
    static func create() -> Factory { return Factory() }
    static var defaultName: String = "anon"
}

let f = Factory.create()
print(Factory.defaultName)
```

| 调用 | 形式 |
|------|------|
| `Type.method()` | `static func` / `static var` |
| `instance.method()` | 实例方法 |

> **static 限制：** `static func` 中无 `self` 引用实例；只能访问其它 `static` 成员。

## 协议默认实现

协议内可提供方法默认实现，用户类型继承默认：

```syx
protocol Greetable {
    func greet() -> String
    func greet() -> String { return "Hello" }   // 默认实现
}

class User: Greetable {
    // 不实现 greet()，继承协议默认 → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }    // 覆盖默认
}

print(User().greet())                        // "Hello"
print(Loud().greet())                        // "HI!"
```

## override 重写

子类重写父类方法（详见 [第 14 章 继承](14-inheritance.md)）：

```syx
class Animal {
    func speak() { print("...") }
    func describe() -> String { return "Animal" }
}

class Dog: Animal {
    override func speak() { print("woof") }
    override func describe() -> String { return "Dog" }
}
```

- 必须写 `override` 关键字
- 缺 `override` → **S051**
- 父类未声明 → **S052**

## mutating 方法（struct / enum）

`mutating func` 标记会修改 `self` 的方法：

```syx
struct Counter {
    var value: Int = 0
    mutating func increment() { value = value + 1 }
}

var c = Counter()
c.increment()
print(c.value)                             // 1

let c2 = Counter()
// c2.increment()                          // ❌ S020：cannot mutate immutable
```

- 必须在 `var` 绑定上调用
- 无 `mutating` 标记但函数体修改 `self` → 编译错误

## 方法作为值

```syx
class Helper {
    func shout(_ s: String) { print("!!! \(s) !!!") }
}

let h = Helper()
let f = h.shout                              // 绑定方法
f("hello")                                   // !!! hello !!!

let arr: Array<(String) -> Void> = [h.shout]
arr[0]("world")                              // !!! world !!!
```

- 绑定方法时不写 `()`，类型为 `(P) -> R` 函数
- `f` 持有 `h` 的强引用（注意循环引用）

## extension 方法

`extension` 为已有类型扩展方法（详见 [第 22 章 扩展](22-extensions.md)）：

```syx
extension Int {
    func doubled() -> Int { return self + self }
}

print(3.doubled())                           // 6
```

## 下标方法（subscript）

`subscript` 在类型内声明下标（详见 [第 13 章 下标](13-subscripts.md)）：

```syx
class Matrix {
    var data: Array<Array<Int>> = [[1, 2], [3, 4]]
    subscript(row: Int, col: Int) -> Int {
        get { return data[row][col] }
        set { data[row][col] = newValue }
    }
}

let m = Matrix()
print(m[0, 1])                               // 2
```

## 静态存储

`static var` 是类型级存储（class 跨实例共享）：

```syx
class Counter {
    static var totalInstances: Int = 0
    init() { Counter.totalInstances = Counter.totalInstances + 1 }
}

Counter()
Counter()
print(Counter.totalInstances)                // 2
```

> **状态：** `static var` **已实现**（Phase 134+）；`static var` 上的 willSet/didSet **Phase 138 GA**（class only；详见 [第 11 章](11-properties.md#全局与静态)）。

## 方法与 actor 隔离

`actor` 方法自动隔离：

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }   // 隔离区内可改 value
}

let c = Counter()
await c.increment()                          // 跨隔离必须 await
```

## 完整示例

```syx
@Equatable
data class Point(x: Int, y: Int) {
    func distance(to other: Point) -> Double {
        let dx = Double(x - other.x)
        let dy = Double(y - other.y)
        return (dx * dx + dy * dy).squareRoot()
    }
}

class Robot {
    var position: Point
    let id: Int
    static var nextId: Int = 0

    init(at p: Point) {
        self.position = p
        self.id = Robot.nextId
        Robot.nextId = Robot.nextId + 1
    }

    func moveTo(_ p: Point) {
        position = p
    }

    func describe() -> String {
        return "Robot #\(id) @ \(position)"
    }
}

func main() {
    let r1 = Robot(at: Point(x: 0, y: 0))
    let r2 = Robot(at: Point(x: 3, y: 4))
    r1.moveTo(Point(x: 1, y: 1))
    print(r1.describe())                      // "Robot #0 @ Point(x: 1, y: 1)"
    print(r2.position.distance(to: r1.position))
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S001** | 方法名与字段冲突 | 改名 |
| **S004** | 形参 / 返回类型不匹配 | 修正 |
| **S020** | `mutating` 调用于 `let` | 改 `var` |
| **S051** | override 缺关键字 | 加 `override` |
| **S052** | override 父类未声明 | 删 `override` |
| **A002** | 跨 actor 调用未 await | 加 `await` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `@objc` / `@nonobjc` | **未实现** |
| `inout` 形参 | **未实现**；用 `&mut T` 借用 |
| `final func` 禁止 override | **未实现**（用 `final class`） |
| `mutating func` | **已实现** |
| `@MainActor func` | **已实现**（详见 [第 18 章](18-concurrency.md)） |
| `throws` 方法 | **已实现**（`throws func` 完整支持） |
| `async` 方法 | **已实现**（`async func`） |

---

**下一章** → [下标](13-subscripts.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 11 章 属性](11-properties.md) · [第 14 章 继承](14-inheritance.md) · [第 22 章 扩展](22-extensions.md)
