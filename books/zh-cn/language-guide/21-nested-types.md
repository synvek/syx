# 嵌套类型

**实现状态：** 顶层类型 **已实现**；类型内嵌套 `class` / `struct` / `enum` / `final class` **MVP GA**（Phase 139）；`Outer.Inner` 限定路径 **MVP GA**；嵌套可见性 / 继承 / 外层实例访问 **未完整**

Swift 允许在类型内嵌套 `struct` / `enum` / `class` / `protocol`。SYX **Phase 139 MVP** 支持常见嵌套声明与 `Outer.Inner` 路径引用；本章说明当前能力、诚实边界与推荐写法。

## 当前状态

| 能力 | 状态 |
|------|------|
| 顶层 `class` / `struct` / `enum` / `actor` | **已实现** |
| 顶层 `protocol` | **已实现** |
| 类型内 `class` / `struct` / `enum` / `final class` | **MVP GA**（Phase 139） |
| 嵌套 `protocol` | **部分实现** |
| `Outer.Inner` 引用语法（类型与构造） | **MVP GA** |
| 外部类型引用嵌套类型（常见 case） | **MVP GA** |
| 多层路径 `Outer.Middle.Inner` | **MVP GA** |
| 嵌套类型继承 / 外层实例访问 | **未实现** |
| `private` 嵌套可见性 | **未实现**（按文件作用域） |

> **诚实边界：** 内部仍用 `Outer_Inner` 名字修饰；源码中写 `Outer.Inner`，parser/semantic 解析为稳定路径。不是完整 Swift 嵌套作用域规则。

## 基本语法

```syx
class Outer {
    class Inner {
        func value() -> Int { return 7 }
    }
}

func main() {
    var inner = Outer.Inner()
    print(inner.value())                    // 7
}
```

类型注解同样使用点路径（内部映射为 `Outer_Inner`）：

```syx
var x: Outer.Inner = Outer.Inner()
```

## 多层嵌套

```syx
class Outer {
    class Middle {
        class Deep {
            func value() -> Int { return 9 }
        }
    }
}

func main() {
    var d = Outer.Middle.Deep()
    print(d.value())                        // 9
}
```

## 外部引用（常见 case）

嵌套类型可被外层同级类型持有或作为参数：

```syx
class Outer {
    class Inner {
        func value() -> Int { return 7 }
    }
}

class Holder {
    var inner: Outer.Inner
    init(inner: Outer.Inner) {
        self.inner = inner
    }
    func read() -> Int {
        return self.inner.value()
    }
}

func main() {
    var h = Holder(inner: Outer.Inner())
    print(h.read())                         // 7
}
```

跨模块：导出方用 `public class Outer { public class Inner { ... } }`，导入模块可写 `Outer.Inner()`（见 `nested_cross_module` fixture）。

## extension 内嵌套（部分）

```syx
struct Point {
    var x: Int
    var y: Int
}

extension Point {
    struct Polar {
        var r: Double
        var theta: Double
    }

    func toPolar() -> Polar {
        let r = Double((x * x + y * y).squareRoot())
        let theta = Double(x) / Double(y)
        return Polar(r: r, theta: theta)
    }
}
```

> **extension 内嵌套：** 部分支持；访问通常通过外层实例方法。

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 类型嵌套 | **MVP GA**（常见 case） |
| `Outer.Inner` 路径访问 | **MVP GA** |
| 嵌套类型继承 | **未实现** |
| 嵌套类型访问外层实例 | **未实现** |
| `private` 嵌套 | **未实现** |

## flat 命名（仍可用）

历史 flat 风格仍兼容，等价于手动名字修饰：

```syx
class OuterMarker_Inner {
    var value: Int = 0
}
```

新代码优先 `Outer.Inner` 点路径。

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **P001** | 嵌套语法解析失败 | 检查嵌套 decl 语法 |
| **S002** | 未知类型 / 未解析路径 | 确认外层类型名与嵌套成员存在 |

---

**下一章** → [扩展](22-extensions.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [第 10 章 结构体和类](10-structures-and-classes.md)
