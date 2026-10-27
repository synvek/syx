# 集合类型

**实现状态：** `Array<T>` / `Set<T>` / `Dictionary<K,V>` / `Object` **产品可用**（Dictionary 仍为 String-shell，见下）；HOF `map/filter/reduce` **GA**（三后端 parity）；用户 Sequence/Iterator `for-in` **已实现**（Phase 71）；裸 `Array`（无类型参数）**已删除**（Phase 60）。产品诚实面已与本章方法表对齐（facade 裸签名处 + sema 参数化已旁注）；`OrderedSet` **非产品 GA**（`ordered_set.sy` 存在，host capabilities 全 unsupported）。余：**单态化**、真 `HashMap<Value, Value>` 键外壳。

SYX 提供四种内置集合：有序 `Array<T>`、去重 `Set<T>`、键值 `Dictionary<K,V>`、动态 `Object`。本章覆盖字面量、方法、迭代、IO 互转与类型约束。

## 数组 Array

`Array<T>` 是最常用的有序可变序列：

```syx
let nums: Array<Int> = [1, 2, 3]
let mixed: Array<Any> = [1, "a", true]     // 需 Any 约束
let empty: Array<Int> = []

print(nums.count)                          // 3
print(nums[0])                              // 1
nums[1] = 99                                // ✅ 可变
```

> **强类型：** Phase 60 起**禁止** 裸 `Array`（必须带类型参数）。需要混合类型用 `Array<Any>` 或 `Object`。

### 常用方法

| 方法 | 签名 | 说明 |
|------|------|------|
| `.count` | `Int` | 元素个数（空判断用 `.count == 0`，无 `.isEmpty`） |
| `.contains(item)` | `Bool` | 是否包含（`Equatable`；无谓词版 `.contains(where:)`） |
| `.sorted()` | `Array<T>` | 升序排序（新数组） |
| `.sortBy(comparator)` | `Array<T>` | 自定义比较（facade 仍裸 `Array` / `Function`；≠ `.sorted(by:)`） |
| `.reverse()` | `Array<T>` | 反转（新数组；真名 `.reverse()`，≠ `.reversed()`） |
| `.chunked(size)` | `Array<Array<T>>` | 按长度切块（facade 仍 `Array<Array>`） |
| `.zip(other)` | `Array<Array<T>>` | 与 `other` 成对（两侧同 `T` 时；否则 `Array<Array<Object>>`，≠ 静默 `Array<Array<T>>`；facade 仍裸 `Array`） |
| `.joined(sep:)` | `String` | 用分隔符连接为字符串 |
| `.map(transform)` | `Array<U>` | HOF（**GA**）；`U` 由闭包结果类型推断（facade 仍 `Function`，≠ mono / `map<U>`） |
| `.filter(predicate)` | `Array<T>` | HOF（**GA**）；保留元素类型 `T`（facade 仍 `Function`） |
| `.reduce(initial, combine)` | `U` | HOF（**GA**）；`U` 由 `initial` 推断（facade 仍 `initial: Int` / `-> Int` / `Function`，≠ mono） |
| `.flatMap(transform)` | `Array<U>` | HOF（**GA**）；闭包结果为 `Array<U>` 时解开一层得 `Array<U>`，否则按单元素（facade 仍 `Function`，≠ mono） |
| `.append(item)` | `Void` | 追加 |

另有 facade `.binarySearch`（仍 `item: Int`）。**未实现**（调用 → **S002**）：`.isEmpty`、`.first` / `.last`、`.reversed()`、`.remove(at:)` / `.removeAll()`、`.index(of:)`、`.slice`、`.contains(where:)`、`.allSatisfy`、`.forEach`。遍历用 `for`。

### 数组字面量与推断

```syx
let a = [1, 2, 3]                       // 推断 Array<Int>
let b: Array<Float> = [1.0, 2.0]        // 显式
let c: Array<Int> = []                  // 空数组必须标注或给初值
```

### 高阶函数（HOF）

```syx
let nums = [1, 2, 3, 4, 5]

let doubled = nums.map({ x in x * 2 })             // [2, 4, 6, 8, 10]
let evens   = nums.filter({ x in x % 2 == 0 })     // [2, 4]
let sum     = nums.reduce(0, { acc, x in acc + x }) // 15
```

HOF 走 `__syx_array_map` / `__syx_array_filter` / `__syx_array_reduce` Host 函数（详见 [第 32 章](../../docs/adr/architecture-decisions.md)）；闭包参数是 `Array<Element>` 的元素 `T`。语义上 `filter` 结果为 `Array<T>`；`map` / `flatMap` 结果为 `Array<U>`；`reduce` 结果为 `U`（`U` 来自 `initial`）。将 `Int→String` 的 `map` 赋给 `Array<Int>`，或把 `reduce(0, …)` 赋给 `String` → **S004**。builtin facade 仍声明 `Function` / `Array` / `reduce(…)->Int`，不是全量单态化。

> **三后端 parity：** `map/filter/reduce` 在 VM / 统一 MIR / WASM 三后端行为一致。测试 `fixtures/parity/hof.sy` 验证。

### 数组下标

```syx
var arr = [10, 20, 30]
print(arr[0])                  // 10
arr[1] = 99                    // ✅
print(arr)                     // [10, 99, 30]
```

下标必须是 `Int`。越界读写（含负下标）→ 运行时错误 **R003**（文案含 `out of bounds`）；**不是**返回 `nil`，也不是静默忽略写入。

## Set

去重集合（基于 `value_equals`，保留插入顺序）：

```syx
let s: Set<Int> = [1, 2, 3, 2]   // 自动去重：{1, 2, 3}
print(s.count)                   // 3
print(s.contains(2))              // true
```

| 方法 | 说明 |
|------|------|
| `.count` | 元素个数（空判断用 `.count == 0`，无 `.isEmpty`） |
| `.contains(item)` | 是否包含（**要求** 元素满足 `Equatable`） |
| `.insert(item)` | 插入（已存在则忽略） |
| `.remove(item)` | 是否删除成功（`Bool`；≠ Swift 式 Optional） |
| `.union(other)` | 并集 |
| `.intersection(other)` | 交集 |

**未实现**（调用 → **S002**）：`.isEmpty`、`.subtraction`、`.forEach`。遍历用 `for`。

> **存储后端：** 运行时是 `Value::Set(Vec<Value>)`，按 `value_equals` 线性去重，保留插入顺序；`.insert` / `.contains` / `.remove` / `.union` / `.intersection` 同一谓词。**≠** 哈希表，**≠** 用户 `hash(into:)`。产品 facade 在 `packages/core/builtins/collections/set.sy`（实例 `contains`/`insert`/`remove` 与 static 同走 `__syx_set_*` Host）。

## Dictionary 字典

类型与字面量均用 **方括号 + 冒号**，与数组 `[T]` / `[elem, …]` 区分：

```syx
let scores: [String: Int] = ["a": 1, "b": 2]
let byId: Dictionary<Int, Int> = {2: 4, 3: 6}
print(scores["a"])               // 1（Optional<Int>）
print(byId[2])                   // 4（Optional<Int>）
```

> **关键区分：** 解析器见到 **`[key: value, …]`**（首元素后跟 `:`）即 Dictionary；见到 **`[elem, …]`**（首元素后跟 `,` 或 `]`）即 Array。两者语法不冲突。

> **存储后端（String-shell）：** 运行时 `Value::Dict` 仍是 `HashMap<String, Value>`（与 `Object` 同形）。`String` 键直插槽；非 `String` 键（`Int` / class 等）走内部 `\x1fsyx:` 编码的 key/value 槽。用户面 `.keys()` / `.count` 暴露**逻辑键/条目数**（解码后的 `Int` 等），不泄漏编码串。哈希相同且键不等时在该桶拉链并存（相等键替换值）。**≠** 已完成的 `HashMap<Value, Value>` 真 Value 键外壳。产品 facade 在 `packages/core/builtins/collections/dictionary.sy`（实例 `containsKey`/`keys`/`values`/`remove` 与 static 同走 `__syx_dict_*` Host；facade 签名仍为裸 `Array` / `Object` 键，sema 返回 `Array<K>`/`Array<V>`；赋值钉：`dict_keys_values_type`）。

### 常用方法

产品面（与 Core facade / dict Host 对齐；**无** `mapValues` / `merge` / `forEach` / `isEmpty`）：

| 方法 | 签名 | 说明 |
|------|------|------|
| `.count` | `Int` | 键值对个数 |
| `.containsKey(k)` | `Bool` | 是否包含键 |
| `.keys()` | `Array<K>` | 键数组 |
| `.values()` | `Array<V>` | 值数组 |
| `.remove(k)` | `Bool` | 是否删除成功（≠ Swift 式 `V?`） |
| `d[k]` / `d[k] = v` | 读/写 | 下标（见上） |

> **遍历与变换：** 用 `for k in d.keys()` 与下标取值；值侧 HOF（`mapValues` 等）**尚未**提供——调用未知成员 → **S002**。

### 键类型约束

键类型 `K` **必须** 满足 `Hashable` 协议：

```syx
let k1: [String: Int] = ["x": 1]            // ✅ String: Hashable
let k2: [Int: String] = [1: "a", 2: "b"]    // ✅ Int: Hashable
// let k3: [Array<Int>: Int] = [[1]: 1]      // ❌ S004：Array<Int> not Hashable
```

> **Hashable 与键语义：**
> - `data class` + `@Hashable`：按字段合成哈希，可用作 Dict/Set 键。
> - 普通 `class: Hashable` 作键：**按实例身份**（同一引用为同一键；字段相等 ≠ 同键）。
> - 用户写的 `func hash(into:)` / `==` **不**驱动 Dict/Set 查找；显式 `x.hash(into: h)` 只更新 `Hasher`，与集合无关。

### Dictionary 字面量键

```syx
let lit: [String: Int] = ["a": 1, "b": 2]    // 字符串键
let byNum: [Int: String] = [1: "x", 2: "y"]  // 整数键
let tuple: [String: Array<Int>] = ["xs": [1,2,3]]
```

键为任意表达式（字面量、标识符、函数返回值等）。

## Object（动态对象）

未标注的字典字面量推断为 **`Dictionary`**（value 须同质）。异质 JSON 袋须**显式**标注 `Object`（应尽量少用，优先具体类型）：

```syx
let scores = ["a": 1, "b": 2]                 // Dictionary（同质）
let obj: Object = { "name": "SYX", "version": 110, "stable": true }
print(obj["name"])        // "SYX"
print(obj["version"])     // 110
print(obj["stable"])      // true
```

| 形式 | 语法 | 类型 |
|------|------|------|
| Dictionary | `["a": 1]` 或 `[String: Int] = …` | 同质键值（默认推断） |
| Object | `let x: Object = { "k": v }` | 动态异质袋（须标注） |

| 方法 | 签名 | 说明 |
|------|------|------|
| `obj[key]` | `Any?` | 读取（Optional；与 `valueForKey` 同语义） |
| `obj[key] = v` | `Void` | 写入（与 `put` 同语义） |
| `.valueForKey(key)` | `Object` | 显式读键（`object.sy` facade） |
| `.put(key, value)` | `Object` | 显式写键（返回袋自身） |
| `.count` | `Int` | 键个数 |
| `.isEmpty` | `Bool` | 空 |
| `.containsKey(key)` | `Bool` | 是否包含键 |
| `.keys()` | `Array<String>` | 键数组 |
| `.values()` | `Array<Object>` | 值数组（类型擦除） |
| `.remove(key)` | `Bool` | 是否删除成功（≠ `V?`） |
| `.description` | `String` | 描述字符串 |

> **todo 4 soft-close（W1253 / ADR-1599）：** 默认字面量 → Dictionary；显式 `: Object` 异构袋；上表方法面 **met**；钉 `todo4_object_soft_close_first_slice`；**项 4 不再开片**。**OUT：** `.merge`；真 `HashMap<Value,Value>`；Object 泛型深化；`object_bag_ga`。

> **无 `.merge`：** Object / Dictionary 均尚未提供合并 API；手写下标拷贝（OUT / 不再以项 4 开片）。

> **类型擦除：** `Object` 不带字段类型；读出值需用 `as?` 转换（详见 [第 20 章](../../docs/adr/architecture-decisions.md)）。`.keys()` 经 `__syx_dict_keys`，facade/sema 均为 `Array<String>`；`.values()` 经 `__syx_dict_values`，facade/sema 均为 `Array<Object>`（≠ 裸 `Array` / ≠ `Array<Int>`）。

```syx
let v = obj["version"]
if let n = v as? Int {
    print("version is \(n)")
}
```

## 自定义 Sequence / Iterator

Phase 71 起，用户自定义 `class` 实现 `next()` 即可被 `for-in` 遍历（三端 strict parity）：

```syx
protocol Iterator {
    func next() -> String?
}

class Counter: Iterator {
    var current: Int
    let end: Int
    init(end: Int) { self.current = 0; self.end = end }

    func next() -> String? {
        if (current >= end) { return nil }
        let s = "\(current)"
        current = current + 1
        return s
    }
}

func main() {
    let c = Counter(end: 3)
    for s in c {
        print(s)          // 0, 1, 2
    }
}
```

> **协议要求：** 自定义 `Iterator` 必须有 `func next() -> String?`（与 stdlib 一致）；返回 `nil` 表示结束。详见 [第 23 章 协议](../../docs/adr/architecture-decisions.md)。

## for-in 遍历

`for x in iterable { … }` 支持：

| iterable | 元素类型 | 说明 |
|----------|----------|------|
| `Array<T>` | `T` | 顺序遍历 |
| `String` | `String`（单标量） | 字素簇 |
| `Set<T>` | `T` | 无序 |
| `Dictionary<K,V>` | `(K, V)` 元组 | |
| `0..<n` | `Int` | 半开范围 |
| 自定义 `Iterator` | `String` | Phase 71+ |
| `Object` | `(String, Any)` 元组 | 键值对 |
| `Array<String>`（`File.readLines()`） | `String` | 行 |

```syx
for item in [1, 2, 3] { print(item) }       // 1 2 3
for c in "abc" { print(c) }                  // a b c
for (k, v) in ["x": 1, "y": 2] { print(k) }  // x y
for i in 0..<3 { print(i) }                  // 0 1 2
for line in File.readLines("a.txt") { print(line) }
```

## 二进制 IO（Phase 42 / 60）

`import IO` 后整文件读写 `Array` 字节（元素为 `UInt8`）：

```syx
import IO

File.writeBytes(path, [0u8, 255u8, 42u8])
let bytes: Array<UInt8> = File.readBytes(path)

let h = FileHandle.open(path, "r")
let chunk: Array<UInt8> = FileHandle.read(h, 4096)
FileHandle.close(h)
```

- **强类型：** Phase 60 后**必须** `Array<UInt8>`；裸 `Array` 报 **S004**。
- 大文件读 `FileHandle` 流式；`readBytes` 整文件加载到内存。

测试：`stdlib/leak_file_stream.rs` · `stdlib/file_stream_watcher.rs`

## 选择指南

| 需求 | 选 |
|------|-----|
| 有序、可重复、可索引 | `Array<T>` |
| 去重、集合运算 | `Set<T>` |
| 静态键值映射、需编译期类型 | `Dictionary<K, V>` |
| 动态 JSON 风格、字段不固定 | `Object`（须显式标注；默认优先 `Dictionary`） |
| 字节流 | `Array<UInt8>` |
| 任意迭代、自定义 `next()` | `class: Iterator` |

## 性能与内存

- `Array<T>` 是值包装的 `Vec<T>` 引用；`Array<UInt8>` / `Array<String>` 走 `ArrayBuild`（VM HeapSlot，Phase 76）
- `Dictionary<K, V>` 产品面走哈希语义；运行时仍是 String-shell（`HashMap<String, Value>` + 非 String 键编码槽）；字面量经 `materialize_dict` Host 物化（Phase 77）
- `Set<T>` 实现位于 `packages/core/builtins/collections/set.sy`
- 集合元素是 class 时，`arr.append(c)` 共享引用；**不** 深拷贝

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | `Array` 与 `Dictionary` 元素类型不一致 | 显式标注或修正字面量 |
| **S010** | Dictionary 键类型不满足 `Hashable` | 改用 String/Int 或实现 `Hashable` |
| **R003** | `arr[i]` 越界读写（含负下标） | 检查索引；≠ `nil` / 静默 |
| **S024** | 集合 hosted op 禁止 legacy `MethodCall`（Phase 48） | 改用 `extension` / `CallHost` |

## 与 Swift 的差异

| 点 | SYX |
|----|-----|
| `Array` 写法 | `[T]` 与 `Array<T>` 等价 |
| 裸 `Array` | **已删除**（Phase 60）；必须带类型参数 |
| Dictionary 类型 | `[String: Int]` 或 `Dictionary<String, Int>` |
| Dictionary 字面量 | **`["a": 1]`**（方括号）；Swift 仅 `{ }` |
| Object | **`let o: Object = { "k": v }`**（须标注；未标注→`Dictionary`） |
| Dictionary 键 | **须** 满足 **`Hashable`**（`Int`、`String` 等） |
| `Set` 字面量 | `[1, 2, 3]` 显式标注为 `Set<Int>` |
| `flatMap` | 已实现（`Array<Array<T>>` 扁平化） |
| `Range`（半开） | `0..<n`（**已实现**）；闭区间 `0...n` 部分实现 |
| `KeyPath` 数组 | `\Array.[0]`（Phase 48）；详见 [第 13 章](../../docs/adr/architecture-decisions.md) |

---

**下一章** → [控制流](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：类型](../language-reference/02-types.md) · [第 13 章 下标](../../docs/adr/architecture-decisions.md) · [第 23 章 协议](../../docs/adr/architecture-decisions.md) · [实现状态总表](../shared/implementation-status.md)
