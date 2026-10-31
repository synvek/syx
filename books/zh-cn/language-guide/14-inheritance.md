# 继承

**实现状态：** 单继承 + `override` + vtable 多态 **GA**（Phase 51–55 + Phase 66）；`super.method()` 静态绑定到 direct super vtable 槽；`final class` 禁止子类化（Phase 60）；运行时多态 vtable indirect call **已实现**（Phase 66）

继承是 class 的核心能力。SYX 单继承 + 必填 `override` 关键字的设计与 Swift 一致；运行时通过 vtable 多态分派。

## class 继承基础

```syx
class Animal {
    var name: String
    init(name: String) { self.name = name }

    func speak() {
        print("...")
    }
}

class Dog: Animal {
    override func speak() {
        print("woof")
    }
}

func main() {
    let d = Dog(name: "Rex")
    d.speak()                              // "woof"
}
```

| 规则 | 说明 |
|------|------|
| 语法 | `class Sub: Super { … }` |
| 单继承 | 仅一个父类（多继承 **未实现**） |
| `override` | 必填关键字（缺 → **S051**） |
| `super.method()` | 调用父类实现 |

> **必须 `override`：** 这是 SYX 严格契约的一部分，防止无意覆盖父类方法；也帮助 LSP / IDE 工具准确识别重写。

## 运行时多态（vtable）

基类引用持有子类实例时，方法调用经 **运行时 vtable 间接分派**：

```syx
class Animal {
    func speak() { print("animal") }
}

class Dog: Animal {
    override func speak() { print("woof") }
}

class Cat: Animal {
    override func speak() { print("meow") }
}

func describe(_ a: Animal) {
    a.speak()                              // 多态
}

func main() {
    describe(Dog())                        // "woof"
    describe(Cat())                        // "meow"
}
```

| 调用形式 | 分派 |
|----------|------|
| `Dog().speak()` | 静态已知类型，可 devirtualize（编译器可优化为直接调用） |
| `let a: Animal = Dog(); a.speak()` | **动态 vtable**（Phase 66 GA） |

> **实现：** Phase 66 引入「**单继承、module-local vtable**」；每个 class 持有 `vtable: Array<fn_ptr>`；`override` 替换对应槽位；`super.method()` 静态绑定到 immediate super 的 vtable 槽（**不** 走 receiver 动态类型）。

测试：`inheritance_polymorphic.rs` · `inheritance.rs` · LSP `lsp_phase66_real.rs`

## super 调用

```syx
class Base {
    func greet() { print("base") }
}

class Derived: Base {
    override func greet() {
        super.greet()                      // "base"
        print("derived")
    }
}
```

- `super.method()` 调用 **直接父类** 的 vtable 槽
- 不递归向上（A→B→C 中 `super.x` 只到 B）
- `super` 在静态作用域合法；在 override 链中正确链接

## final 禁止子类化

```syx
final class Token {
    let value: String
    init(value: String) { self.value = value }
}

// class SubToken: Token { }                // ❌ S053：cannot inherit from final
```

- `final class` 禁止被继承
- 允许编译器做 devirtualization（直接调用，无 vtable 间接）
- 与 Swift `final class` 等价

## 构造链

子类 `init` 必须调用 `super.init`（**Phase 51 GA**）：

```syx
class Animal {
    var name: String
    init(name: String) {
        self.name = name
    }
}

class Dog: Animal {
    var breed: String

    init(name: String, breed: String) {
        self.breed = breed
        super.init(name: name)             // 必须
    }
}
```

详见 [第 15 章 构造过程](15-initialization.md)。

## 多层继承

```syx
class A {
    func a() { print("A") }
}

class B: A {
    override func a() { print("B") }
}

class C: B {
    override func a() { print("C") }
}

let c = C()
c.a()                                      // "C"
let b: B = c
b.a()                                      // "C"（多态分派到 C）
let a: A = c
a.a()                                      // "C"
```

> **菱形继承：** 多继承 **未实现**；SYX 用 `protocol` 表达多态能力（详见 [第 23 章](23-protocols.md)）。

## 重写属性

子类可 `override var` 覆盖父类 computed 属性（vtable 多态读/写）：

```syx
class Base {
    var label: String {
        get { return "base" }
    }
    init() {}
}

class Derived: Base {
    override var label: String {
        get { return "derived" }
        set { /* ignore */ }
    }
    init() {}
}

let b: Base = Derived()
print(b.label)                            // "derived"
```

详见 [第 11 章 属性](11-properties.md#属性-override)。

## 重写初始化器

子类 `init` 必须调用 `super.init`，且所有 stored 属性必须赋值（两段式校验）：

```syx
class Animal {
    var name: String
    init(name: String) {
        self.name = name
    }
}

class Dog: Animal {
    var tricks: Array<String>

    init(name: String, tricks: Array<String>) {
        self.tricks = tricks
        super.init(name: name)             // 必须
    }
}
```

## override 与协议

实现协议方法 **不需要** `override`（不是继承父类）：

```syx
protocol Greetable {
    func greet() -> String
}

class User: Greetable {                    // 不写 override
    func greet() -> String { return "hi" } // 满足协议
}
```

> **区分：** `override` 仅用于重写父类方法；协议实现不带 `override`。

## 继承与 actor

`actor` 不能被继承（**未实现**）；actor 可与顶层 class 自由组合（actor 内嵌套 `class` 仍为后续薄片）：

```syx
class Session {
    let id: String
    init(id: String) { self.id = id }
}

actor Manager {
    var sessions: Array<Session> = []

    func open() -> Session {
        let s = Session(id: "x")
        self.sessions.append(s)
        return s
    }
}
```

## 完整示例

```syx
class Shape {
    var color: String
    init(color: String) { self.color = color }

    func area() -> Double { return 0.0 }
    func describe() -> String {
        return "\(self.color) shape, area=\(self.area())"
    }
}

class Circle: Shape {
    var radius: Double

    init(color: String, radius: Double) {
        self.radius = radius
        super.init(color: color)
    }

    override func area() -> Double {
        return 3.14159 * self.radius * self.radius
    }
}

class Rectangle: Shape {
    var width: Double
    var height: Double

    init(color: String, width: Double, height: Double) {
        self.width = width
        self.height = height
        super.init(color: color)
    }

    override func area() -> Double {
        return self.width * self.height
    }
}

func totalArea(_ shapes: Array<Shape>) -> Double {
    var sum = 0.0
    for s in shapes { sum = sum + s.area() }
    return sum
}

func main() {
    let shapes: Array<Shape> = [
        Circle(color: "red", radius: 2.0),
        Rectangle(color: "blue", width: 3.0, height: 4.0)
    ]
    for s in shapes { print(s.describe()) }
    print("total=\(totalArea(shapes))")
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S051** | override 缺关键字 | 加 `override` |
| **S052** | override 父类未声明 | 删 `override` |
| **S053** | 继承 `final` 类 | 改非 final 或删继承 |
| **S054** | override 签名不匹配 | 修正签名 |
| **S055** | override 父类 `final` 方法 | 不能 override `final` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 单继承 + `override` | **已实现** |
| `final class` | **已实现**（Phase 60） |
| vtable 多态 | **已实现**（Phase 66） |
| 多继承 | **未实现**（用 protocol） |
| `final class` / `final func` | **已实现**（lexer 无 `sealed`） |
| 协议组合（`protocol<A, B>`） | 部分实现（用 `&` / `where`） |
| `convenience init` | **部分实现** |
| `required init` | **未实现**（构造链 `super.init` 校验） |

---

**下一章** → [构造过程](15-initialization.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 11 章 属性](11-properties.md) · [第 15 章 构造过程](15-initialization.md) · [第 16 章 析构过程](16-deinitialization.md)
