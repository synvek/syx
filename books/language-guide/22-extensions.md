# 扩展

**实现状态：** `extension` 方法 / 协议一致性 / 计算属性（**Phase 136 GA**）/ 带约束扩展 **已实现**（Phase 40–42）；`extension` 中嵌套类型 **部分实现**；`extension` 跨模块 **已实现**（Phase 60 module-local witness）

`extension` 为已有类型追加功能（方法、计算属性、协议一致性、初始化器），不修改原类型声明。SYX 与 Swift 风格一致；多约束 where 子句在 Phase 42 起支持。

## extension 方法

```syx
extension Int {
    func doubled() -> Int { return self + self }
    func times(_ f: () -> Void) {
        for _ in 0..<self { f() }
    }
}

func main() {
    print(3.doubled())                    // 6
    3.times({ print("hi") })              // "hi" × 3
}
```

> **`self` 在 extension 内** 指代被扩展类型的实例（与 class 内方法一致）。

来源：`crates/syx-driver/tests/fixtures/types/greeter_extension.sy`

## extension 计算属性

**Phase 136 GA**（`lang-props-v4:extension-computed-mvp`）：

```syx
extension Int {
    var isEven: Bool { get { return self % 2 == 0 } }
    var squared: Int {
        get { return self * self }
        set { /* 仅示意；可重置 stored 字段 */ }
    }
}

print(4.isEven)                          // true
print(3.squared)                         // 9
```

| 形式 | 状态 |
|------|------|
| get-only | **已实现**（Phase 134） |
| get / set | **已实现**（Phase 135） |
| 跨模块 | **已实现**（Phase 60 module-local） |

## 协议一致性

```syx
protocol Named {
    var name: String { get }
}

extension User: Named {
    var name: String { return "User(\(self.id))" }
}

class User {
    let id: Int
    init(id: Int) { self.id = id }
}
```

> **绑定顺序无关：** `extension` 写在 class 声明之前也可；编译器后续扫描所有 extension 合成 witness。

## 协议默认实现

```syx
protocol Greetable {
    func greet() -> String
}

extension Greetable {
    func greet() -> String { return "Hello" }    // 默认
}

class User: Greetable {
    // 继承默认 → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }      // 覆盖
}
```

## 带约束扩展（where）

```syx
extension Array where Element: Equatable {
    func unique() -> Array<Element> {
        var seen: Array<Element> = []
        for item in self {
            if (!seen.contains(item)) {
                seen.append(item)
            }
        }
        return seen
    }
}

let nums = [1, 2, 2, 3, 3, 3]
print(nums.unique())                      // [1, 2, 3]
```

### 多约束

**Phase 42 起**支持多约束：

```syx
extension Array where Element: Equatable, Element: Hashable {
    func toSet() -> Set<Element> {
        var s: Set<Element> = []
        for item in self { s.insert(item) }
        return s
    }
}
```

> **约束不满足：** 编译错误 **S010**。测试：`fixtures/format/extension_where_mvp` / `fixtures/format/extension_where_mvp` / `fixtures/format/extension_where_mvp`。

## extension 初始化器

```syx
struct Point {
    var x: Int
    var y: Int
}

extension Point {
    init() {                              // 提供便利 init
        self.init(x: 0, y: 0)
    }
}

let p = Point()                            // (0, 0)
```

> **限制：** `extension` 内 `init` 必须委托到原类型指定 init；不能定义新 stored 字段。

## extension 跨模块

`extension` 可在模块 A 中扩展模块 B 的类型：

```syx
// 模块 A 中
extension String {
    func swaCount() -> Int { return self.count }
}
```

> **当前实现：** Phase 60 起 cross-module extension 完整支持；与 module-local witness 集成。Swift 的同 module 限制放宽（**注意**：这是 SYX 行为；Swift 不允许跨 module extension）。

## 给基础类型加方法

```syx
extension Int {
    func clamp(_ lo: Int, _ hi: Int) -> Int {
        if (self < lo) { return lo }
        if (self > hi) { return hi }
        return self
    }
}

let x = 100.clamp(0, 10)                  // 10
```

## extension 与访问控制

```syx
public extension Int {
    public func toHex() -> String { ... }   // 必须显式标注
}
```

> **当前状态：** extension 与成员的 `public` / `internal` 标注 **已实现**（S005 跨模块检查）。

## extension 与 KeyPath

```syx
extension User {
    var nameKp: KeyPath<User, String> { return \User.name }
}
```

> **状态：** KeyPath 在 extension 中可用（**部分实现**）。

## 完整示例

```syx
protocol Describable {
    func description() -> String
}

@Equatable
data class User(id: Int, name: String)

extension User: Describable {
    func description() -> String { return "User(\(id): \(name))" }
}

extension Array where Element: User {
    func findByName(_ n: String) -> User? {
        for u in self {
            if (u.name == n) { return u }
        }
        return nil
    }
}

extension Int {
    var isEven: Bool { return self % 2 == 0 }
    var isOdd: Bool  { return !self.isEven }
    func clamp(_ lo: Int, _ hi: Int) -> Int {
        if (self < lo) { return lo }
        if (self > hi) { return hi }
        return self
    }
}

func main() {
    let users = [User(id: 1, name: "Alice"), User(id: 2, name: "Bob")]
    print(users.findByName("Bob")?.description() ?? "n/a")

    let n = 42
    print(n.isEven, n.clamp(0, 10))         // false 10
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S001** | extension 中方法与已有冲突 | 改名 |
| **S010** | where 约束不满足 | 检查协议一致性 |
| **S005** | extension 跨模块访问 internal 符号 | 改 public |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `extension` 方法 / 属性 | **已实现** |
| `extension` 协议一致性 | **已实现** |
| `extension` 跨模块 | **已实现**（Phase 60；Swift 不允许） |
| `extension` where 多约束 | **已实现**（Phase 42） |
| `extension` 内 `init` | **已实现**（委托到指定 init） |
| `extension` 嵌套类型 | **部分实现** |
| `extension` 中 subscript | **部分实现** |
| `extension` 中 stored 字段 | **未实现**（只能 init / 计算属性） |
| retroactive conformance 警告 | **未实现**（SYX 允许跨 module 扩展） |

---

**下一章** → [协议](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [语言参考：协议与一致性](../language-reference/09-protocols-and-conformances.md) · [第 11 章 属性](../../docs/adr/architecture-decisions.md) · [第 23 章 协议](../../docs/adr/architecture-decisions.md)
