# 词法结构

**实现状态：** **已实现**（`syx-lexer`，Token 完整；正则字面量 Phase 50+；块注释嵌套；标识符 UAX #31 XID — ADR-229 / `M2-S2-04`）

SYX 源文件为 UTF-8 文本（不含 BOM）。词法分析将源码切分为 `Token` 流，供语法分析使用。

---

## 字符集

| 元素 | 范围 |
|------|------|
| 源码编码 | **UTF-8**（无 BOM） |
| 行结束符 | `\n`（LF）/ `\r\n`（CRLF，规范化） |
| 空格 | ` ` / `\t` / `\v` / `\f` |

> 源码内任意位置可出现 **任意 Unicode 字符**（如注释、字符串字面量、标识符）；字符串字面量支持 `\u{…}` 转义。

## 标识符

```
identifier ::= letter (letter | digit | '_')*
letter     ::= [a-zA-Z_] | <unicode XID_Start>
```

| 规则 | 说明 |
|------|------|
| 首字符 | 字母或 `_` |
| 后续 | 字母、数字、`_` |
| 大小写敏感 | `Foo` ≠ `foo` |
| Unicode | **已实现**（`XID_Start` / `XID_Continue` via `unicode-xid`；ADR-229） |
| 规范化 | 标识符**不**做 NFC；比较为源码拼写相等 |
| 关键字 | 不可作普通标识符 |

```syx
count
_private
MyType
helloWorld
αβγ                           // Unicode XID（合法）
```

> **诊断：** L006 非法字符；L001 标识符命名 lint（lint 规则）。

## 关键字

保留字不可作普通标识符使用。完整列表分以下几类：

### 声明

| 关键字 | 含义 |
|--------|------|
| `class` | 引用类型 |
| `struct` | 值类型 |
| `enum` | 枚举 |
| `protocol` | 协议 |
| `extension` | 扩展 |
| `actor` | 并发隔离类型 |
| `func` | 函数 |
| `init` | 构造器 |
| `deinit` | 析构器 |
| `var` | 可变绑定 |
| `let` | 不可变绑定 |
| `import` | 导入模块 |
| `typealias` | 类型别名 |
| `associatedtype` | 关联类型 |
| `global` | 全局 actor（修饰 `actor`） |
| `data` | data class 修饰（`data class`） |
| `final` | 禁止子类化（`final class` / `final func`） |

### 访问控制

| 关键字 | 含义 |
|--------|------|
| `public` | 跨模块 |
| `internal` | 模块内（默认） |
| `private` | 文件 / 作用域 |
| `fileprivate` | 文件内 |

### 控制流

| 关键字 | 含义 |
|--------|------|
| `if` | 条件 |
| `else` | 否则 |
| `guard` | 早退守卫 |
| `switch` | 多路分支 |
| `case` | 分支 |
| `default` | 默认 |
| `for` | 循环 |
| `in` | for-in 容器 |
| `while` | 条件循环 |
| `repeat` | repeat while |
| `break` | 跳出 |
| `continue` | 继续 |
| `return` | 返回 |
| `fallthrough` | **拒绝**（SYX 不支持） |

### 并发

| 关键字 | 含义 |
|--------|------|
| `async` | 异步函数 |
| `await` | 异步等待 |
| `Task` | 任务 |
| `actor` | 隔离对象 |
| `global` | 全局 actor 修饰 |
| `isolated` | 隔离（部分） |
| `sending` | 跨隔离传递参数（**Phase 26**） |
| `distributed` | distributed actor（**partial**） |

### 类型

| 关键字 | 含义 |
|--------|------|
| `Self` | 实现类型 |
| `Type` | 类型元 |
| `some` | 不透明返回 |
| `any` | 存在类型 |

### 错误处理

| 关键字 | 含义 |
|--------|------|
| `try` | 错误传播 |
| `catch` | 捕获 |
| `throw` | 抛出 |
| `throws` | 错误签名（`throws T` typed throws **部分实现**） |

### 字面量

| 关键字 | 含义 |
|--------|------|
| `true` | 布尔真 |
| `false` | 布尔假 |
| `nil` | 空值 |

### 泛型与元编程

| 关键字 | 含义 |
|--------|------|
| `where` | 泛型约束 |
| `requires` | 效果注解 |
| `macro` | 用户宏声明 |
| `compileTime` | 常量求值 |
| `Pure` / `IO` | 效果标签 |

### 上下文关键字

| 关键字 | 含义 |
|--------|------|
| `static` | 静态（类型方法 / 字段） |
| `final` | 禁止 override |
| `override` | 重写 |
| `lazy` | 惰性属性（class only） |
| `mutating` | 变异方法 |
| `convenience` | 便利 init（**未实现**） |
| `required` | 必填 init（**未实现**） |
| `prefix` / `infix` / `postfix` | 自定义运算符 |
| `associativity` | 结合性 |
| `precedence` | 优先级 |

完整列表见 `crates/syx-lexer/src/token.rs`。

## 字面量

### 整数字面量

```
int-literal ::= digit+ (suffix)?
             | '0x' hex-digit+ (suffix)?
             | '0b' binary-digit+ (suffix)?
             | '0o' octal-digit+ (suffix)?

suffix ::= 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'isize'    -- signed
        | 'u8' | 'u16' | 'u32' | 'u64' | 'u128' | 'usize'    -- unsigned
```

| 形式 | 示例 | 推断类型 |
|------|------|----------|
| 十进制 | `42` | `Int` |
| 十六进制 | `0xFF` | `Int` |
| 二进制 | `0b1010` | `Int` |
| 八进制 | `0o17` | `Int` |
| 后缀 | `100i8` / `0xFFu8` / `42i128` | 对应固定宽度 |
| 下划线 | `1_000_000i64` | 编译期忽略 `_` |

> **无后缀整数默认** 类型为平台 **`Int`**（64 位 host → `Int64`；32 位 → `Int32`）。

### 浮点字面量

```
float-literal ::= digit+ '.' digit+ (exponent)? (suffix)?
               | digit+ exponent (suffix)?

exponent ::= ('e' | 'E') ('+' | '-')? digit+
suffix    ::= 'f32' | 'f64'
```

| 形式 | 示例 | 推断类型 |
|------|------|----------|
| 十进制 | `3.14` | `Double` |
| 后缀 | `3.14f32` / `2.0f64` | `Float32` / `Float64` |
| 科学计数法 | `2.5e10` | `Double` |
| 下划线 | `1_000.5` | 同上 |

> **无后缀浮点默认** `Double`。

### 布尔与空

| 字面量 | 类型 |
|--------|------|
| `true` | `Bool` |
| `false` | `Bool` |
| `nil` | 上下文推断（`T?` / `Optional<T>`） |

### 字符串字面量

```
string-literal ::= '"' string-char* '"'
string-char    ::= <any except '"', '\', newline> | escape
escape         ::= '\' ('n' | 'r' | 't' | '\\' | '\"' | '\'' | '0' | 'x' hex-digit hex-digit | 'u' '{' unicode-codepoint '}')
               | '$' '(' expr ')'        -- interpolation
```

| 元素 | 含义 |
|------|------|
| `"..."` | 双引号包裹 |
| `\n` `\r` `\t` | 常见控制字符 |
| `\\` `\"` `\'` `\$` | 字面字符 |
| `\0` | NUL（**警告**：可能截断字符串） |
| `\x41` | ASCII 字节（0x00–0xFF） |
| `\u{1F600}` | Unicode 码点（U+0000–U+10FFFF） |
| `\(expr)` | 插值（任意表达式） |

```syx
"Hello, World"
"Hello, \(name)!"
"line one\nline two"
"emoji: \u{1F600}"
"ascii: \x41"
```

> **裸换行禁止：** 字符串内不允许裸换行；未闭合报 **L001**。
> **存储：** UTF-8 字节数组（Rust `String` 后端）。
> **字素簇：** `.length` / `substring` / `for-in` 按 Unicode **字素簇**（Phase 84 起，`unicode-segmentation` 索引）。

### 正则字面量

```
regex-literal ::= '/' regex-body '/' regex-flag?
regex-flag    ::= [a-zA-Z]+
```

| 形式 | 示例 | 状态 |
|------|------|------|
| `/pattern/` | `/\d+/` | **已实现** |
| `/pattern/i` | `/hello/i` | 部分支持（flag 在 parser 接受） |
| `/pattern/m` | `/foo/m` | **未实现** |

> 由 `/` 与上下文消歧（非注释起始）；典型 `let p = /\d+/` 无歧义。

### 字典字面量

```
dict-literal ::= '[' dict-entry (',' dict-entry)* ']' | '{' dict-entry (',' dict-entry)* '}'
dict-entry   ::= expr ':' expr
```

| 形式 | 语法 | 类型 |
|------|------|------|
| Dictionary | `["a": 1, "b": 2]` | `[K: V]` 静态 |
| Object | `{ "name": "SYX" }` | 动态 `Object` |

> 解析器见到 **`[key: value, …]`**（首元素后跟 `:`）即 Dictionary；见到 **`[elem, …]`**（首元素后跟 `,` 或 `]`）即 Array。

## 注释

| 形式 | 用途 |
|------|------|
| `// 文本` | 单行（行尾或整行） |
| `/* 文本 */` | 块（**支持嵌套**） |
| `/// 文本` | 文档注释（`syx doc` 提取） |

```syx
// 单行注释

/* 块注释
   /* 可嵌套 */
   任意行
*/

/// 文档注释
/// 多行：以 `///` 开头
func add(a: Int, b: Int) -> Int { return a + b }
```

## 运算符

### 算术与赋值

| 运算符 | 含义 | 优先级 |
|--------|------|--------|
| `+` `-` `*` `/` `%` | 算术 | 100 / 90 |
| `=` | 赋值 | 10 |
| `+=` `-=` `*=` `/=` `%=` | 复合赋值 | 10 |
| `<<` `>>` | 位移 | 80 |
| `&` `\|` `^` | 位运算 | 50 / 30 / 40 |

### 比较与逻辑

| 运算符 | 含义 | 优先级 |
|--------|------|--------|
| `==` `!=` | 相等 | 60 |
| `===` `!==` | 恒等 | 60 |
| `<` `<=` `>` `>=` | 有序比较 | 70 |
| `&&` | 逻辑与 | 20 |
| `\|\|` | 逻辑或 | 10 |
| `!` | 逻辑非（一元） | — |
| `??` | 空合并 | 5 |
| `?` | try 链 / Optional / `?.` | — |
| `!`（后缀） | 强制解包 | — |

### 其他

| 运算符 | 含义 |
|--------|------|
| `->` | 函数返回类型 |
| `=>` | 闭包 fat arrow（**部分支持**） |
| `..<` | 半开范围 |
| `...` | 闭区间（**部分支持**） |
| `&` | 借用 `&T` / `&mut T` |
| `@` | 特性 `@attached` / `@Component` |
| `#` | 宏 `#assert` |
| `.` | 成员访问 |
| `?:` | 三元（**故意不提供**；用 `if` 表达式；≠ 待办缺口） |

## 分隔符

```
separator ::= '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' | '.'
```

| 符号 | 用途 |
|------|------|
| `(` `)` | 元组 / 函数参数 / 条件 |
| `{` `}` | 块 / 字典 Object / 闭包 |
| `[` `]` | 数组 / 下标 / Dictionary 类型 |
| `,` | 列表分隔 |
| `:` | 命名参数 / 类型标注 / 字典键值 |
| `;` | 空（可选；SYX 不要求） |
| `.` | 成员访问 / 浮点小数点 |

## 空白

```
whitespace ::= ' ' | '\t' | '\n' | '\r' | <unicode Zs>
```

- 词法层多数被跳过（字符串字面量除外）
- 注释作为空白处理
- 缩进不强制（**无** Python 式强制缩进）

---

**诊断：** L001–L012（词法 / lint）

**用法示例** → [语言指南：基础](../language-guide/02-the-basics.md) · [字符串](../language-guide/04-strings-and-characters.md)

**参见** → [类型](02-types.md) · [表达式](03-expressions.md) · [语法摘要](10-grammar-summary.md)
