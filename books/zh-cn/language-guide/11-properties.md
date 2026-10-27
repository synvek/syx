# 属性

**实现状态：** **部分 GA**（Phase 138 Class Properties v6 MVP）

属性是与类型或实例关联的存储值。

## 存储属性

```syx
class Person {
    var name: String
    let id: Int
    init(name: String, id: Int) {
        self.name = name
        self.id = id
    }
}
```

| 关键字 | 成员 |
|--------|------|
| `var` | 可变 |
| `let` | 不可变 |

## 全局与静态

```syx
let globalVersion = 1

class Config {
    static var debug = false
    static let version: Int = 1
}
```

**部分 GA** — Phase 134 起 class 内 `static var` / `static let` MVP（类型级存储槽）。
**Phase 138 GA** — class `static var` 上的 `willSet` / `didSet` MVP（`lang-props-v6:static-observers-mvp`；`oldValue` / `newValue` 绑定；**非** struct / 全局观察器全集）。

```syx
class Counter {
    static var value: Int = 0 {
        willSet { print(newValue) }
        didSet { print(oldValue) }
    }
}
// Counter.value = 7 → willSet(7), didSet(0)
```

## 计算属性

**部分 GA** — class 与 extension 内 Swift 式 computed MVP：

```syx
class Box {
    var base: Int
    init(base: Int) { self.base = base }
    var doubled: Int {
        get { return self.base * 2 }
    }
    var value: Int {
        get { return self.base }
        set { self.base = newValue }
    }
}

extension Box {
    var labeled: String {
        get { return "x" }
        set { self.base = 0 }
    }
}
```

| 能力 | 状态 |
|------|------|
| class `var x: T { get { … } }` get-only | **Phase 134 GA** |
| class `var x: T { get set }`（`newValue`） | **Phase 135 GA** |
| 子类 `override var` computed（多态读/写） | **Phase 135 GA** |
| extension `var x: T { get }` / `{ get set }` | **Phase 136 GA**（`lang-props-v4:extension-computed-mvp`） |
| `@Published` on class stored `var` | **Phase 137 GA**（`lang-props-v5:published-mvp`；赋值前 `willSet` 窄通知 `print("published:<name>")`；非完整 Combine） |
| class `static var` + `willSet` / `didSet` | **Phase 138 GA**（`lang-props-v6:static-observers-mvp`） |
| lazy thread-safety / struct `@Published` / 全局观察器全集 | **139+** |

指纹：`lang-props-v6:docs-honesty` — 完整 Swift 属性模型 → **139+**。

## 属性观察器

`willSet` / `didSet`：**部分 GA**（Phase 65 起存储属性直接赋值可触发；Phase 134 加深）：

| 能力 | 状态 |
|------|------|
| 存储属性 `willSet` / `didSet` | **部分 GA** |
| `oldValue` / `newValue` 绑定 | **GA** |
| **继承**：子类实例写父类字段触发父类观察器 | **Phase 134 GA** |
| **`init` 内抑制**：`self.field = …` 不触发观察器 | **Phase 134 GA**（对齐 Swift） |
| class **`static var`** `willSet` / `didSet` | **Phase 138 GA** |
| 计算属性观察器 | **拒绝**（S056） |
| `@Published`、全局观察器运行时 | **部分 GA**（`@Published` → Phase 137；全局观察器全集 → **139+**） |

```syx
class Animal {
    var name: String
        willSet { print("will") }
        didSet { print("did") }
    init(name: String) { self.name = name }  // init 赋值不打印
}
class Dog: Animal {
    init(name: String) { super.init(name: name) }
}
// Dog 实例写 name → 触发 Animal 的 willSet/didSet
```

## 属性 override

子类可 `override var` 覆盖父类 computed 属性；基类引用读写走子类 getter/setter（vtable 多态）：

```syx
class Base {
    var label: String { get { return "base" } }
    init() {}
}
class Derived: Base {
    override var label: String { get { return "derived" } }
    init() {}
}
// let b: Base = Derived(); b.label → "derived"
```

## 惰性属性

`lazy var`：**Phase 134 GA**（class only；首次读触发初始化；**非**线程安全 MVP → **139+** 完整语义）。

```syx
class Box {
    lazy var label: String = "ready"
    init() {}
}
```

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 计算属性 + 观察器全集 | 存储为主；computed get/set + override + extension computed + static observers MVP GA |
| `lazy var` 线程安全 | MVP 非线程安全 |
| `@Published` 等 | **Phase 137 MVP**（class stored；完整 Combine → **139+**） |
| static observers | **Phase 138 MVP**（class `static var` only；完整 Swift static/lazy/global → **139+**） |
| struct 属性语法统一 / wrapper+observer 排序全集 | **139+** |

---

**下一章** → [方法](12-methods.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md)
