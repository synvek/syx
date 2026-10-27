# 表达式

**实现状态：** **已实现**（`Expr` 枚举，完整优先级表）；自定义运算符 `operator func` **GA**（三后端 parity）；Optional 链 / 空合并 / 强制解包 **已实现**

表达式产生值。下表按**优先级从高到低**排列（与 parser 爬升一致）。

---

## 优先级（完整表）

| 级别 | 优先级 | 形式 | 运算符 / 说明 |
|------|--------|------|----------------|
| 1（最高） | — | **后缀** | `.` `[]` `()` `?.` `!`（后缀） `await`（后缀） |
| 2 | — | **一元前缀** | `-` `!` `~` `try` `&`（借用）`await`（前缀） |
| 3 | 100 | 乘除 | `*` `/` `%` `<<` `>>`（位移混合此级） |
| 4 | 90 | 加减 | `+` `-` |
| 5 | 80 | 位移 | `<<` `>>`（部分实现） |
| 6 | 70 | 范围 | `..<` `...` |
| 7 | 70 | 有序比较 | `<` `<=` `>` `>=` |
| 8 | 60 | 相等 | `==` `!=` `===` `!==` |
| 9 | 50 | 按位与 | `&` |
| 10 | 40 | 按位异或 | `^` |
| 11 | 30 | 按位或 | `\|` |
| 12 | 20 | 逻辑与 | `&&` |
| 13 | 10 | 逻辑或 | `\|\|` |
| 14 | 5 | 空合并 | `??` |
| 15 | 10 | 复合赋值 | `+=` `-=` `*=` `/=` `%=` |
| 16（最低） | 5 | 赋值 | `=` |

> **括号优先：** 任意位置可加 `()` 强制优先级，无副作用。
> **赋值表达式** 返回 `Void` 语义（不链式依赖）。

## 字面量表达式

```
literal ::= int-literal | float-literal | string-literal | bool-literal | nil-literal
          | array-literal | dict-literal | object-literal | regex-literal
```

```syx
42
3.14f32
"text"
true
nil
[1, 2, 3]
["k": "v"]
{ "name": "SYX" }
/\d+/
```

## 标识符

```
identifier-expr ::= identifier
```

```syx
count
MyType
self                                        // 当前实例
super                                       // 父类
```

## 元组表达式

```
tuple-expr ::= '(' expr (',' expr)+ ')'
            |  '(' expr ',' ')'              // 单元素元组
```

```syx
let t = (1, "a", true)                      // (Int, String, Bool)
let (x, y) = (1, 2)                         // 解构
```

## 一元与二元

```syx
-a                                          // 一元负
!flag                                       // 逻辑非
~bits                                       // 按位取反
a + b
a * b
a % b
a == b
a < b
a && b
a || b
expr?                                       // try 链：错误转 nil/传播
```

## 调用

```
call-expr ::= expr '(' arg-list? ')'
arg-list  ::= (label ':')? expr (',' (label ':')? expr)*
```

```syx
foo()
foo(x: 1, y: 2)                             // 命名参数
obj.method(arg)
arr.map { x in x * 2 }                      // 尾随闭包
```

> **命名参数：** 调用时**必须** 写 `label: value`（无 `_` 省略语法）。

## 成员访问

```
member-expr ::= expr '.' identifier
```

```syx
obj.field
obj.length
arr.map
self.name
```

## 下标

```
index-expr ::= expr '[' expr ']'
            |  expr '[' expr (',' expr)* ']'
```

```syx
arr[i]
dict["key"]                                 // Dictionary
s[0]                                        // String（单标量）
m[0, 1]                                     // Matrix 多参数
obj["field"]                                // Object 动态
```

**实现状态：**
- `arr[i]` / `dict["key"]` / `s[i]` **已实现**
- `s[0..<3]` 切片 **未实现**（用 `substring`）
- 自定义 `subscript` **部分实现**（Phase 40/41）

## 可选链与强制解包

```
optional-chain ::= expr '?' '.' identifier
                |  expr '?' '[' expr ']'
                |  expr '?' '(' arg-list? ')'
force-unwrap   ::= expr '!'
```

```syx
maybe?.length                               // Optional 链
maybe?.foo(1, 2)
maybe!                                       // 强制解包（nil → R007）
```

**诊断：** R007（运行时 nil 强制解包）

## await

```
await-expr ::= 'await' expr
```

```syx
await asyncFunc()
await ch.receive()                          // 异步接收
```

> 仅 `async` 函数体内合法；非 `async` 内 `await` → **S020**。

## 闭包表达式

```
closure-expr ::= '{' closure-params? closure-body? '}'
closure-params ::= ('|' (identifier (',' identifier)* '|') | (identifier (',' identifier)*))
                |  '(' (identifier ':' type (',' identifier ':' type)*)? ')'
closure-body ::= 'in' expr | 'in' block
```

```syx
{ x in x + 1 }                              // 单表达式
{ (a, b) in a + b }                         // 多参数
{ print("hi") }                             // 无参数
{ (x: Int) -> Int in x * 2 }               // 显式类型
{ |x| x + x }                               // 管道参数（**部分支持**）
```

## 数组字面量

```
array-literal ::= '[' (expr (',' expr)*)? ']'
```

```syx
[10, 20, 30]
[]                                          // 空数组（需类型标注）
[1, 2, 3].map { $0 * 2 }                    // 立即调用
```

## 字典字面量

```
dict-literal ::= '[' dict-entry (',' dict-entry)* ']'
dict-entry   ::= expr ':' expr
object-literal ::= '{' object-entry (',' object-entry)* '}'
object-entry  ::= string-literal ':' expr
```

```syx
["a": 1, "b": 2]                            // Dictionary
{ "name": "SYX", "version": 110 }           // Object
```

## Task 块

```
task-expr ::= 'Task' ('.' 'detached')? closure-expr
```

```syx
Task { … }
Task.detached { … }
```

## 借用表达式

```
borrow-expr ::= '&' 'mut'? expr
```

```syx
&value
&mut counter
```

## 范围（半开 / 闭区间）

```
range-expr ::= expr '..<' expr
            |  expr '...' expr
            |  '..<' expr
            |  expr '..<'
```

```syx
0..<10                                      // 半开
1...5                                       // 闭区间（**部分实现**）
```

> 用于 `for-in` 与 `Range` 类型上下文。

## 模式表达式（switch / if case）

```
pattern-expr ::= pattern '=' expr
             |  'if' 'case' pattern '=' expr block
             |  'switch' '(' expr ')' '{' case-clause* '}'
```

```syx
if case .ok(let x) = result { … }
switch (result) {
case .ok(let v) { … }
default { … }
}
```

见 [模式](../../docs/adr/architecture-decisions.md)。

## 赋值表达式

```
assign-expr ::= lvalue '=' expr
            |  lvalue ('+=' | '-=' | '*=' | '/=' | '%=') expr
```

```syx
x = 1
x += 2
obj.field = v
arr[i] = v
```

> **赋值表达式** 结果为 `Void` 语义（不链式依赖）。`a = b = c` **未实现**。

## 自定义运算符

```
operator-decl ::= 'prefix'|'infix'|'postfix' 'operator' identifier
                  '{' 'precedence' ':' int ',' 'associativity' ':' ('left'|'right'|'none') '}'
```

```syx
infix operator |> { precedence: 50, associativity: left }
func |> <T>(value: T, f: (T) -> T) -> T { return f(value) }
```

完整语法见 [第 30 章 高级运算符](../language-guide/30-advanced-operators.md)。

## 表达式分类

| 类别 | 例子 |
|------|------|
| 字面量 | `42`, `"x"`, `[1,2,3]` |
| 标识符 | `count`, `self`, `super` |
| 路径 | `obj.field`, `arr[i]` |
| 调用 | `f()`, `obj.method(x)` |
| 一元 | `-x`, `!flag`, `&mut x` |
| 二元 | `a + b`, `a < b` |
| 范围 | `0..<n`, `1...5` |
| 可选 | `opt?.x`, `opt!` |
| 闭包 | `{ x in x + 1 }` |
| 异步 | `await future` |
| try | `try? risky()` |
| 控制流表达式 | `if (x) { a } else { b }` |
| 关键字 | `nil`, `true`, `false` |
| 字符串插值 | `"\(name)"` |

## 表达式类型检查

| 规则 | 说明 |
|------|------|
| 两侧类型一致 | 算术、比较要求（**S004** 不兼容） |
| 整数 + 浮点 | widening（Int → Double） |
| 有符号 + 无符号 | **S014** 拒绝 |
| Optional + T | `T?` 提供 nil 合并 / 链 / 强解 |
| 函数 + 函数 | 调用 / HOF |
| 元组 + 元组 | 元素类型分别匹配 |
| nil 字面量 | 推断为 `T?` |

---

**诊断：** S004 / S008 / S010 / S014 / S020 / R007

**用法示例** → [语言指南：运算符](../language-guide/03-basic-operators.md) · [高级运算符](../language-guide/30-advanced-operators.md)

**参见** → [语句](../../docs/adr/architecture-decisions.md) · [类型](../../docs/adr/architecture-decisions.md) · [模式](../../docs/adr/architecture-decisions.md) · [语法摘要](../../docs/adr/architecture-decisions.md)
