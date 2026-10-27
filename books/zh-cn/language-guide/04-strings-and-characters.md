# 字符串和字符

**实现状态：** **已实现**（UTF-8 存储、字素簇计数 Phase 84、for-in、`.count`、下标、split/trim/... 全部 stdlib 方法、正则字面量 Phase 50+）；`Character` **Phase 139 MVP GA** 为与 `String` 区分的标量类型（非完整 Unicode Character 属性库）

字符串是 SYX 与 Swift 差异最显著的领域之一。本章会讲清「**字素簇 vs 标量**」、「**Character 类型**」、「**插值与转义**」、「**正则字面量**」四个关键点。

## 创建字符串

```syx
let s = "Hello"
let msg = "Hello, \(name)!"        // 插值
let multi = "a" + "b" + "c"        // 字符串拼接
let multiline = "line one\nline two"
let tabbed = "col1\tcol2\tcol3"
```

来源：`crates/syx-driver/tests/fixtures/lang/string_multiline.sy` · `examples/main.sy`

## 字符串字面量语法

| 形式 | 语法 | 说明 |
|------|------|------|
| 单行 | `"…"` | 双引号包裹；支持转义与插值 |
| 多行 | **MVP GA** | `"""`…`"""` 可含裸换行与 `\(expr)` 插值（Phase 139） |
| 转义 | `\n` `\t` `\\` `\"` `\(expr)` | 同 Swift |
| 插值 | `\(expr)` | 任意表达式（函数调用、算术、字段访问） |

```syx
let name = "World"
let count = 3
let s1 = "Hello, \(name)!"
let s2 = "count=\(count + 1)"        // 表达式
let s3 = "items: \([1,2,3].count)"  // 任意表达式合法
```

> **重要：** 单行字符串内不允许裸换行（**L001**）。多行 `"""` 字面量允许裸换行（Phase 139 MVP）。

## 转义序列

| 转义 | 含义 | 字节 |
|------|------|------|
| `\n` | 换行 (LF) | 0x0A |
| `\r` | 回车 (CR) | 0x0D |
| `\t` | 制表符 | 0x09 |
| `\\` | 反斜杠 | 0x5C |
| `\"` | 双引号 | 0x22 |
| `\'` | 单引号 | 0x27 |
| `\0` | 空字符 (NUL) | 0x00 |
| `\(…)` | 插值（无 0 字节，编译为 StringConcat） | — |
| `\x41` | ASCII 字节 | 0x41 |
| `\u{1F600}` | Unicode 码点 | emoji 0x1F600 |

```syx
let s = "tab\there\nnewline"
let emoji = "\u{1F600}"            // 😀
let zwnj = "\u{200C}"              // ZWNJ（零宽非连接符）
```

## 字符串常用方法

```syx
"a,b,c".split(",")                       // ["a", "b", "c"]
"  hi  ".trim()                          // "hi"
"ABC".lowercased()                       // "abc"
"abc".uppercased()                       // "ABC"
"file.txt".hasPrefix("file")             // true
"hello.txt".hasSuffix(".txt")            // true
"hello".substring(0, 3)                  // "hel"  [start, end) 标量索引
"banana".replacingOccurrences("a", "o")  // "bonono"
"abc123".matches("\\d+")                 // ["123"]（正则匹配返回子串数组）
"text".count                            // 4（字素簇个数）
"text".isEmpty                           // false
"  ".isEmpty                             // false（空格也是字素簇）
```

完整 String 方法注册表见 `crates/syx-stdlib/src/string_methods.rs`（`STRING_METHODS`）。`examples/main.sy` 给出端到端示例。

| 方法 | 签名 | 说明 |
|------|------|------|
| `split(sep: String)` | `Array<String>` | 按分隔符拆分 |
| `trim()` | `String` | 去首尾 Unicode 空白 |
| `lowercased()` | `String` | Unicode **simple case mapping**（Rust；**≠** `caseFolded`；**≠** locale） |
| `uppercased()` | `String` | Unicode **simple case mapping**（Rust；**≠** `caseFolded`；**≠** locale） |
| `normalizedNFC(text:)` | `String` | Unicode NFC（ADR-228） |
| `normalizedNFD(text:)` | `String` | Unicode NFD（ADR-228） |
| `caseFolded(text:)` | `String` | Unicode default/full case folding（如 `ß`→`ss`；ADR-228） |
| `hasPrefix(s)` | `Bool` | 前缀判断 |
| `hasSuffix(s)` | `Bool` | 后缀判断 |
| `substring(start, end)` | `String` | 标量子串 [start, end) |
| `replacingOccurrences(target, with)` | `String` | 替换（首次 / 全部依实现） |
| `matches(pattern)` | `Array<String>` | 正则匹配 |
| `count` | `Int` | 字素簇个数（**非** 字节数） |
| `isEmpty` | `Bool` | 字素簇数为 0 |
| `repeat(n)` | `String` | 重复 n 次 |

## 遍历字符串

```syx
for c in "abc" {
    print(c)    // 依次输出 "a", "b", "c"
}
```

循环变量 `c` 的类型为 **`Character`**（Phase 139 起为独立标量类型；运行时非多字符 `String`）。

> **实现状态：** `for-in` **已实现**；循环变量为 `Character`（MVP GA；`fixtures/lang/character.sy`）。

## 内部表示（重要）

| 层面 | SYX |
|------|-----|
| 存储 | UTF-8 字节数组（Rust `String` 后端） |
| `.count` / `substring` / `for-in` | **Unicode 字素簇**（Phase 84 起，`unicode-segmentation` 索引） |
| `Character` 类型 | **MVP GA**（Phase 139；运行时 `Value::Character` 标量；非完整 Unicode 属性库） |
| `Character.isASCII()` | **MVP GA**（单标量检查；非完整 ASCII 属性判定） |

> **Swift 对齐：** 组合 emoji（如 👨‍👩‍👧‍👦，家庭）在 Swift 中为 1 个「字符」；SYX 在 Phase 84 字素簇索引下也是 1。
>
> **回退标记：** Phase 84 之前 `.count` 按 Unicode 标量计数；老 fixture（`stdlib/string_index_honesty.rs` 等）保留标量行为测试。如发现 `.count` 与早期文档不一致，以当前行为为准。

## 下标 `s[i]`

```syx
let s = "syx"
let c: Character = s[0]      // grapheme Character
print(c)                     // "s"

let second = s[1]            // "w"
```

| 行为 | SYX |
|------|-----|
| 索引语义 | **Int grapheme offset**（Phase 84；ADR-227） |
| 复杂度 | **单次随机访问 O(n)**（无 opaque Index / 无 O(1) 缓存） |
| 返回类型 | `Character`（grapheme 簇） |
| 越界 | 运行时错误 **R003**（文案含 `out of bounds`；≠ 空 Character；≠ R013） |
| 与 Swift | **≠** opaque `String.Index`；SYX 用整数偏移 |

> **区间访问：** 暂未提供 `s[0..<3]` 切片语法；用 `substring(0, 3)`（同样为 O(n) grapheme offset）。钉测：`m2_s2_02_string_index_honesty`。

## 字符串拼接

```syx
let a = "Hello"
let b = ", World"
let c = a + b                       // "Hello, World"
let d = "\(a)\(b)!"                 // 插值拼接
let e = String.concat([a, b, "!"])  // 数组形式
```

`+` 运算符在 String 上是 `StringConcat` 指令的低级 lowering（**非** Core Host 函数）；运行时内联为字节拷贝。

## 字符串比较

```syx
"abc" == "abc"     // true（值相等，Equatable witness）
"abc" < "abd"      // true（字典序）
"abc" === "abc"    // false（不同 String 对象；=== 比较引用）
```

> **比较语义：** `==` 按字素簇序列比较（Phase 84 起）；`<` 字典序（`Comparable` witness）。`===` 比较 VM/MIR 层的对象身份（不常用）。

## 正则字面量

```syx
let p = /\d+/                       // 正则字面量
let matches = "abc123def".matches(p)
for m in matches { print(m) }       // "123"

let ci = /hello/i                   // 大小写不敏感（编译期标记）
```

| 形式 | 状态 | 说明 |
|------|------|------|
| `/pattern/` | **已实现** | 由 `crates/syx-regex` 解析；与上下文消歧 |
| `/pattern/i` | **部分实现** | flag 在 parser 接受；运行时支持有限 |
| `/pattern/m` `/pattern/s` | **未实现** | 多行/单行 flag |

> **歧义消歧：** `/` 在注释起始上下文不出现（行注释用 `//`），但在除法表达式中可能歧义（`x/y` vs 正则）。Parser 通过前瞻 token 区分；典型情况无需担心。

详见 [语言参考：词法结构](../language-reference/01-lexical-structure.md#正则字面量)。

## 字符串与集合互转

```syx
let s = "a,b,c"
let parts = s.split(",")                  // ["a", "b", "c"]
let joined = parts.joined("-")            // "a-b-c"

let chars: Array<Character> = Array(s)    // 字素簇数组
let back = chars.joined("")               // "abc"
```

| 方向 | 方法 |
|------|------|
| String → Array<String> | `s.split(sep)` |
| Array<String> → String | `arr.joined(sep)` |
| String → Array<Character> | `Array(s)`（标准库） |
| Array<Character> → String | `chars.joined("")` |

## 完整示例

来源 `fixtures/lang/string_multiline.sy`：

```syx
func main() {
    let name = "SYX"
    print("Hello, \(name)!")

    print("  Hello, World  ".trim())

    let parts = "a,b,c".split(",")
    for item in parts {
        print(item)
    }

    for m in "abc123def".matches("\\d+") {
        print(m)
    }

    let digits: Array<Character> = Array("0123456789")
    print(digits.count)              // 10
    print(digits[0])                  // "0"
}
```

## 内部 API（Core 扩展）

字符串 witness（`hash` / `description` / `debugDescription`）通过 Core extension → Host 函数实现（详见 [第 32 章 运算符 vs Core Host](32-operators-vs-host.md)）：

```syx
extension String {
    func hash(into hasher: Hasher) {
        __syx_hasher_combine(hasher, self)
    }
    func description() -> String {
        return __syx_value_description(self)
    }
}
```

`String` 在 `crates/syx-stdlib/src/builtins/string.sy` 完整注册；用户代码一般无需直接调用 `__syx_*`。

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **L001** | 字符串未闭合 | 补全 `"` |
| **R003** | `s[i]` 越界（含负下标） | 检查索引；≠ 空 Character |
| **S004** | `String` 与 `Array` 互用 | 显式 `.split()` / `.joined()` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `Character` 是独立 struct | `Character` 是 `String` 类型别名（运行时单标量） |
| opaque `String.Index` | **Int grapheme offset**（O(n) 随机访问；≠ opaque Index） |
| `s[i]` 返回 `Character` | 返回 grapheme `Character`（ADR-226/227） |
| `s[range]` 切片 | `s.substring(start, end)` |
| `String(format:)` | **未实现**；用插值或 `Int.toString()` |
| `count` | `.count`（一致） |
| `String == String` | 字素簇序列相等（Phase 84 起） |
| `String` 字面量插值 | 兼容 Swift `\(expr)` 语法 |

---

**下一章** → [集合类型](05-collection-types.md)

**参见** → [语言参考：词法](../language-reference/01-lexical-structure.md) · [第 5 章 集合类型](05-collection-types.md) · [第 32 章 运算符 vs Core Host](32-operators-vs-host.md)
