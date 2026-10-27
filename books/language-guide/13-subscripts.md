# 下标

**实现状态：** 内置集合 / `String` 下标 **已实现**；自定义 `subscript` 声明 **部分实现**（Phase 40/41 get/set MVP）；KeyPath 数组 / Optional 路径 **已实现**（Phase 46–48）

下标是访问集合元素的语法糖 `obj[i]`。SYX 同时支持 **内置类型下标**（如 `Array`、`String`、`Dictionary`）与 **自定义 `subscript` 声明**（get/set）。

## 数组下标

```syx
let items = [10, 20, 30]
print(items[0])                            // 10
items[1] = 99
print(items)                               // [10, 99, 30]
```

| 操作 | 状态 |
|------|------|
| `arr[i]` 读取 | **已实现** |
| `arr[i] = v` 写入 | **已实现** |
| 越界 | 运行时 **R003**（含负下标；≠ `nil` / 静默） |
| 负索引 | **R003**（与越界同一路径） |

来源：`crates/syx-driver/tests/fixtures/differential/suite1/array_index.sy`

## 字典 / Object 下标

```syx
let m = ["a": 1, "b": 2]
print(m["a"])                              // 1（Optional<Int>）
m["c"] = 3
print(m.count)                            // 3

let obj = { "name": "SYX", "version": 110 }
print(obj["name"])                         // "SYX"
```

> **Optional 返回：** `dict[k]` 始终返回 `Optional<V>`，因为「键不存在」与「值为 nil」运行时无法区分。

## 字符串下标

```syx
let s = "abc"
print(s[0])                                // "a"（grapheme Character）
print(s[1])                                // "b"
```

| 索引语义 | **Int grapheme offset**（Phase 84；ADR-227） |
|---------|---------------------------------------------|
| 复杂度 | **单次随机访问 O(n)**（`.nth` 字素迭代；≠ Swift opaque Index） |
| 返回类型 | grapheme `Character` |
| 越界 | **R003**（含负下标；≠ 空 Character） |

> **历史：** Phase 84 之前按 Unicode 标量索引；组合 emoji 可能拆分。Phase 84+ 字素簇对齐；ADR-227 钉死 O(n) Int offset 诚实（**非** opaque Index GA）。

完整 String 字符语义见 [第 4 章 字符串和字符](../../docs/adr/architecture-decisions.md)。

## 自定义 subscript 声明

类型内可声明 `subscript`（Phase 40/41 起 MVP，get/set 完整）：

```syx
class Matrix {
    var data: Array<Array<Int>>
    init(rows: Int, cols: Int) {
        var d: Array<Array<Int>> = []
        for _ in 0..<rows {
            var row: Array<Int> = []
            for _ in 0..<cols { row.append(0) }
            d.append(row)
        }
        self.data = d
    }

    subscript(row: Int, col: Int) -> Int {
        get { return data[row][col] }
        set { data[row][col] = newValue }
    }
}

let m = Matrix(rows: 2, cols: 2)
m[0, 0] = 1
m[1, 1] = 4
print(m[0, 0])                             // 1
print(m[1, 1])                             // 4
```

| 元素 | 说明 |
|------|------|
| `subscript(params) -> T` | 下标声明 |
| `get { … }` | 读取（**Phase 40 GA**） |
| `set { … }` | 写入（**Phase 41 MVP**） |
| `newValue` | `set` 块内的隐式参数 |

> **状态：** `subscript` 自定义声明 **部分实现**（Phase 40/41）；推荐用法是「**get 必填，set 选填**」。

### 单参数下标

```syx
class Grid {
    var cells: Array<String> = ["a", "b", "c", "d"]
    subscript(i: Int) -> String {
        get { return cells[i] }
        set { cells[i] = newValue }
    }
}

let g = Grid()
print(g[0])                                // "a"
g[0] = "x"
print(g[0])                                // "x"
```

## KeyPath 下标（Phase 46–48）

KeyPath 是类型安全的对象引用路径：

```syx
let arr = [1, 2, 3]

// 数组下标 KeyPath
let kp = \Array<Int>.[0]
print(kp(arr))                             // 1
```

### 属性 KeyPath

```syx
class Person {
    let name: String
    init(name: String) { self.name = name }
}

let p = Person(name: "Alice")
let nameKp = \Person.name
print(nameKp(p))                           // "Alice"
```

### Optional KeyPath（Phase 46）

```syx
class Box {
    let value: Int?
    init(value: Int?) { self.value = value }
}

let b = Box(value: 42)
let vKp = \Box.value?.description          // Optional 路径
print(vKp(b))                              // "42"
```

### KeyPath 数组下标

```syx
let arr = [1, 2, 3]
print((\Array<Int>.[0])(arr))              // 1
```

测试：`lang/array_subscript_check.rs`

## 下标与方法对比

| 需求 | 选 |
|------|-----|
| 数组 / 字典 / Object 元素访问 | 内置下标 |
| 自定义类型的索引访问 | 自定义 `subscript` |
| 类型安全引用路径 | KeyPath |
| 动态字符串字段访问 | `Object["key"]` |

## 完整示例

```syx
class SafeArray {
    private var data: Array<Int> = []
    private var maxIndex: Int = 0

    func append(_ x: Int) {
        data.append(x)
        maxIndex = data.count - 1
    }

    subscript(i: Int) -> Int? {             // get-only
        get {
            if (i < 0) { return nil }
            if (i > maxIndex) { return nil }
            return data[i]
        }
    }

    subscript(safe i: Int, default d: Int) -> Int {
        get { return self[i] ?? d }
    }
}

let a = SafeArray()
a.append(1)
a.append(2)
a.append(3)

print(a[0])                                 // Optional(1)
print(a[10])                                // nil
print(a[safe: 10, default: -1])             // -1
```

> **多标签参数：** 下标支持多参数（`subscript(row: Int, col: Int)`），也可多 label（`subscript(safe i: Int, default d: Int)`）。

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | 下标形参 / 返回类型不匹配 | 修正 |
| **P001** | subscript 语法错误 | 检查 get/set 块 |
| **R003** | Array / String 下标越界 | 检查索引范围 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 字素簇下标 | **已实现**（Phase 84 起） |
| 自定义 `subscript` get/set | **部分实现**（Phase 40/41） |
| KeyPath 完整表面 | **部分实现**（`\Type.prop` / `\Type.[i]` / `\Type?.prop` 已支持；dynamic member lookup 未实现） |
| `subscript(dynamicMember:)` | **未实现** |
| 协议下标要求 | **部分实现** |
| subscript 标签 / 多 label | **已实现** |

---

**下一章** → [继承](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：声明](../language-reference/05-declarations.md) · [语言参考：表达式](../language-reference/03-expressions.md#下标) · [第 4 章 字符串和字符](../../docs/adr/architecture-decisions.md) · [第 11 章 属性](../../docs/adr/architecture-decisions.md)
