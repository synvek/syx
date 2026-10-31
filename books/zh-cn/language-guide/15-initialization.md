# 构造过程

**实现状态：** `init` / 默认构造 / `init?` 失败构造 **已实现**；两段式构造 **已实现**（Phase 51+ 链式校验）；`required init` / `convenience init` **部分实现** / **未实现**；`throws init` **部分实现**

构造器在创建实例时初始化所有 stored 属性。SYX 构造器分两类：**指定构造器**（designated init，初始化所有 stored 字段）和 **便利构造器**（convenience init，调用其他 init）。本章覆盖语法、链式校验、失败构造与继承。

## init 基础

```syx
class Greeter {
    var name: String

    init(name: String) {
        self.name = name
    }
}

let g = Greeter(name: "SYX")
print(g.name)                              // "SYX"
```

| 关键字 | 说明 |
|--------|------|
| `init(params)` | 指定构造器 |
| `self.field = value` | 给 stored 属性赋值 |
| `super.init(...)` | 调父类构造器（class 继承时） |
| `convenience init(...)` | **未实现** |

> **`class` / `actor` 成员内 `init` 生成 `TypeName_init` 调用**（VM 内部约定）。

## struct / data class 构造

```syx
struct Point {
    var x: Int
    var y: Int
}

let p = Point(x: 1, y: 2)                 // 字段顺序构造
print(p.x)                                 // 1
```

- `struct` 字段顺序构造（编译器合成）
- `data class` 字段顺序构造 + 自动 `description()` + 证明宏合成

## 默认值

```syx
class Counter {
    var value: Int = 0
    let name: String

    init(name: String) {
        self.name = name
        // value 已有默认值 0，无需赋值
    }
}
```

- 带默认值的字段可在 `init` 中省略赋值
- `let` 字段必须在 `init` 结束前赋值（默认值或显式）

## 继承与 init 链（两段式）

子类 `init` **必须** 在 `super.init` 之前完成所有 stored 属性初始化（两段式）：

```syx
class Animal {
    var name: String
    init(name: String) { self.name = name }
}

class Dog: Animal {
    var breed: String

    init(name: String, breed: String) {
        self.breed = breed                  // 阶段 1：self 属性
        super.init(name: name)              // 阶段 2：super 链
    }
}
```

| 阶段 | 操作 |
|------|------|
| 阶段 1 | 初始化子类自己的 stored 属性（`self.breed = …`） |
| 阶段 2 | 调用 `super.init(...)` |
| 阶段 3 | （可选）继续修改继承属性（观察器被 init 抑制，详见 [第 11 章](11-properties.md#属性观察器)） |

> **顺序错误**（在 `super.init` 之前读 `self` 或未初始化字段）→ **S020** 编译错误。

### super.init 可省（当父类有默认构造）

```syx
class Animal {
    var name: String = "anon"
}

class Dog: Animal {
    var breed: String

    init(breed: String) {
        self.breed = breed
        // super.init() 隐式调用（Animal 有默认构造）
    }
}
```

> **隐式 super.init：** 父类所有 stored 属性有默认值时，子类可省略 `super.init`（编译器自动插入）。

## 失败构造 init?

`init?` 返回 `Optional<Self>`，可返回 `nil`：

```syx
class Port {
    let value: Int

    init?(value: Int) {
        if (value < 0 || value > 65535) {
            return nil
        }
        self.value = value
    }
}

let p1 = Port(value: 80)                   // Port?
let p2 = Port(value: 99999)                // nil

if let p = p1 {
    print("port \(p.value)")               // "port 80"
}
```

| 形式 | 状态 | 说明 |
|------|------|------|
| `init?` | **已实现** | 失败返回 `nil` |
| `init!` | **未实现** | 隐式 unwrap 失败构造 |
| `throws init` | **部分实现** | `init throws` 抛出错误 |

## throws init

```syx
class Parser {
    let source: String

    init(source: String) throws {
        if (source.isEmpty) {
            throw "empty source"
        }
        self.source = source
    }
}

let p = try? Parser(source: "")            // nil
let p2 = try? Parser(source: "x")          // Parser?
```

详见 [第 17 章 错误处理](17-error-handling.md)。

## init 参数默认值

```syx
class Server {
    let host: String
    let port: Int
    let timeout: Int

    init(host: String, port: Int = 8080, timeout: Int = 30) {
        self.host = host
        self.port = port
        self.timeout = timeout
    }
}

let s1 = Server(host: "localhost")
let s2 = Server(host: "x.com", port: 9000, timeout: 60)
```

## init 与属性包装

`@propertyWrapper` 字段的 `init` 需初始化 wrapper 而非 stored value：

```syx
@propertyWrapper
class Clamped {
    var wrappedValue: Int
    init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}

class User {
    @Clamped var age: Int

    init(age: Int) {
        self.age = age                     // 实际调用 Clamped_init
    }
}
```

详见 [第 19 章 宏](19-macros.md#propertywrapperphase-40-ga)。

## `self` 在 init 中

- `self` 在所有 stored 属性初始化完毕前**不可用**（除 `self.field = …` 赋值）
- `self` 在 `super.init` 之前**不可读**（仅可写）
- 闭包捕获 `self` 须等 init 结束（避免半构造对象）

```syx
class Foo {
    let name: String
    let printer: () -> Void

    init(name: String) {
        self.name = name
        // ❌ self.printer 之前不能读
        // self.printer = { print(self.name) }  // 错误

        self.printer = { [self] in          // 显式捕获（部分实现）
            print(self.name)
        }
    }
}
```

> **状态：** 显式捕获列表 `[self]` / `[weak self]` 部分实现；推荐先完成 init 再赋值闭包。

## 多 init 与重载

```syx
class User {
    let name: String
    let age: Int

    init(name: String) {
        self.init(name: name, age: 0)       // 委托（未实现 convenience）
    }

    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }
}
```

> **convenience init：** **未实现**（Phase 49 之后挂起；后续 Phase 计划）。当前用「指定 init 内调用其它 init」模拟（但缺 `convenience` 关键字）。

## enum 的 init

```syx
enum Port {
    case http
    case https
    case custom(Int)

    init(value: Int) {
        if (value == 80) { self = .http }
        else if (value == 443) { self = .https }
        else { self = .custom(value) }
    }
}
```

## actor 的 init

```syx
actor Counter {
    var value: Int

    init(value: Int) { self.value = value }

    func increment() { self.value = self.value + 1 }
}

let c = await Counter(value: 0)            // actor init 也可 await
```

## 完整示例

```syx
class Vehicle {
    var wheels: Int
    init(wheels: Int) { self.wheels = wheels }

    func describe() -> String { return "wheels=\(self.wheels)" }
}

class Car: Vehicle {
    var brand: String

    init(brand: String) {
        self.brand = brand
        super.init(wheels: 4)
    }

    override func describe() -> String {
        return "\(self.brand) car, \(super.describe())"
    }
}

class Truck: Vehicle {
    var payload: Int

    init(payload: Int) {
        self.payload = payload
        super.init(wheels: 6)
    }

    override func describe() -> String {
        return super.describe()
    }
}

func main() {
    let c = Car(brand: "Tesla")
    let t = Truck(payload: 5000)
    print(c.describe())                     // "Tesla car, wheels=4"
    print(t.describe())                     // "wheels=6"
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | init 形参 / 字段类型不匹配 | 修正 |
| **S020** | `super.init` 之前读 `self` | 调整顺序 |
| **S054** | 继承 `init` 签名不匹配 | 修正 |
| **R007** | `init?` 返回类型 `!` 解包 nil | 用 `if let` |
| **R016** | 必填字段未初始化 | 在 init 中赋值 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `init` 命名参数 | **已实现**（必填 `label: value`） |
| 两段式构造 | **已实现**（Phase 51 链式校验） |
| `convenience init` | **未实现**（规划中） |
| `required init` | **未实现** |
| `init?` / `init!` | `init?` **已实现**；`init!` 未实现 |
| `throws init` | **部分实现** |
| `defer` in init | **未实现** |
| `self.init` 委托 | 合法（无 `convenience` 标记） |
| `init` 中闭包捕获 `self` | 部分支持 |

---

**下一章** → [析构过程](16-deinitialization.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 11 章 属性](11-properties.md) · [第 14 章 继承](14-inheritance.md) · [第 16 章 析构过程](16-deinitialization.md) · [第 17 章 错误处理](17-error-handling.md)
