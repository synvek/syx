# 语句

**实现状态：** **已实现**（`Stmt` 枚举）；控制流 / 异常 / 模式匹配 / async 全部覆盖；标签 **部分实现**；fallthrough **拒绝**

语句控制执行流程，不直接作为表达式值。`print` 是语言全局 Host（签名见 [`packages/core/builtins/special/globals.sy`](../../../packages/core/builtins/special/globals.sy)），调用写法像函数。

---

## 分类

```
stmt ::= var-decl-stmt
       | expr-stmt
       | return-stmt
       | if-stmt
       | while-stmt
       | repeat-while-stmt
       | for-in-stmt
       | break-stmt
       | continue-stmt
       | block-stmt
       | try-stmt
       | throw-stmt
       | switch-stmt
       | if-case-stmt
       | if-let-stmt
       | guard-let-stmt
       | print-stmt
       | macro-invoke-stmt
```

## 变量声明

```
var-decl ::= ('let' | 'var') pattern (':' type)? ('=' expr)?
          |  'async' 'let' pattern '=' expr
          |  'async' 'var' pattern '=' expr
```

```syx
let name = "SYX"
var count = 0
let x: Int = 10
async let result = await fetch()            // async let
async var lazyRes: Int? = nil
```

| 关键字 | 可变性 | 重新赋值 |
|--------|--------|----------|
| `let` | 不可变 | ❌ S003 |
| `var` | 可变 | ✅ |
| `async let` | 不可变 | ❌ |
| `async var` | 可变 | ✅ |

> **类型推断：** 首次赋值后 `let` 类型冻结；后续赋值不兼容 → **S004**。

**诊断：** S003（对 `let` 赋值）；S002（未定义）

## 表达式语句

```
expr-stmt ::= expr
```

```syx
foo()                                       // 函数调用
x = 1                                       // 赋值
arr.append(2)                                // 方法调用
```

> **赋值表达式** 是合法语句；不返回值用于链式。

## return

```
return-stmt ::= 'return' expr?
```

```syx
return
return 42
return result
```

> 无 `expr` 等价于 `return ()`（Void）；非 Void 函数必须 return expr。

## if

```
if-stmt ::= 'if' expr block ('else' (if-stmt | block))?
```

```syx
if condition {
    …
} else if other {
    …
} else {
    …
}
```

> **条件括号可选：** `if expr` 与 `if (expr)` 都合法；推荐省略（与 Swift 一致）。
> **条件类型** 必须 `Bool`；非 Bool → **S004**。
> **if 表达式**（Phase 100+）：作表达式时两分支必须存在且类型相同。

## while

```
while-stmt ::= 'while' expr block
```

```syx
while x > 0 {
    x = x - 1
}
```

> 条件括号可选。

## repeat while

```
repeat-while-stmt ::= 'repeat' block 'while' expr
```

```syx
repeat {
    n = n + 1
} while n < 3
```

> 循环体**至少**执行一次；条件在每次迭代后检查。条件括号可选。

## for-in

```
for-in-stmt ::= 'for' 'await'? pattern 'in' expr block
```

```syx
for item in items { … }
for c in "abc" { … }
for i in 0..<10 { … }
for await x in asyncSeq { … }
```

可迭代：

| 类型 | 元素类型 |
|------|----------|
| `Array<T>` | `T` |
| `String` | `String`（单标量） |
| `Set<T>` | `T` |
| `Dictionary<K, V>` | `(K, V)` 元组 |
| `0..<n` | `Int` |
| 自定义 `Iterator` | `String` |
| `Object` | `(String, Any)` |

## break / continue

```
break-stmt    ::= 'break' label?
continue-stmt ::= 'continue' label?
```

```syx
while true {
    if done { break }
    if skip { continue }
}

// 标签
outer: for x in items {
    inner: for y in x.subs {
        if y.shouldStop { break outer }     // ⚠️ 标签部分实现
    }
}
```

> 标签 / 跨层 break / continue **部分实现**。

## 块

```
block-stmt ::= '{' stmt* '}'
```

```syx
{
    let local = 1
    print(local)
}
```

## try / catch

```
try-stmt ::= 'try' block 'catch' '(' identifier (':' type)? ')' block
             ('catch' '(' identifier (':' type)? ')' block)*
```

> 注：`try` 块用 `{ }`；`catch` 绑定**必须**加 `()`，因为 `catch err` 形式与语句歧义（catch 是关键字 + 标识符，无括号时与 `let`/binding 冲突）。

```syx
try {
    throw "error"
    print("no catch")
} catch (err) {
    print(err)                              // err: String（省略类型时的默认）
}
try {
    throw "typed"
} catch (err: String) {
    print(err)                              // 可选显式类型（类似 Java）
}
```

> **catch 多个：** 支持多个 `catch` 子句 + `where` 守卫（**部分实现**）。
> **catch 绑定类型** 省略时默认为 `String`；可写 `catch (e: T)`；typed throws（`throws T`）**部分实现**。

## switch

```
switch-stmt ::= 'switch' expr '{' case-clause* '}'
case-clause ::= 'case' pattern ('where' expr)? block
             |  'default' (':' | block)
```

```syx
switch value {
case 1 {
    print("one")
}
case 2, 3, 5, 7 {
    print("small prime")
}
case 0..<10 {
    print("single digit")
}
case .ok(let v) {
    print(v)
}
case .err(let e) where e.length > 0 {
    print(e)
}
default {
    print("other")
}
}
```

> 条件括号可选；`switch expr` 与 `switch (expr)` 都合法。

> **`case` 块语法：** SYX `case` **必须** 跟 `{ }` 块（**不是** `case 1:` + 缩进）。
> **`default`：** `enum` / 封闭域可省略（编译器检查全 case 覆盖 → **S007**）；非封闭域需 `default`。
> **`fallthrough`：** **拒绝**（编译错误）。

## if case / if let / guard let

```
if-case-stmt ::= 'if' 'case' pattern '=' expr block ('else' block)?
if-let-stmt  ::= 'if' 'let' pattern '=' expr block ('else' block)?
guard-stmt   ::= 'guard' 'let' pattern '=' expr 'else' block
```

```syx
if case .ok(let x) = result {
    print(x)
} else {
    print("not ok")
}

if let x = optional {
    print("got \(x)")
}

guard let y = optional else {
    return                                  // 必须早退
}
print("y is \(y)")
```

> **`guard let` else** 必须退出（`return` / `throw` / `break` / `continue` / 无限循环）— 否则 **S020**。
> **多模式**：`if let x = a, let y = b, cond { … }`（链式 + 守卫）— 合法。

## print

```
print-stmt ::= 'print' '(' arg-list? ')'
```

```syx
print(expr)
print("Hello", name, count)
```

> 内置输出语句（专用语句，**非**函数）。
> 调用形式为 `print(x)` 看起来像函数；底层是 statement 节点。

## throw

```
throw-stmt ::= 'throw' expr
```

```syx
throw "message"
throw err
```

> 仅在 `throws` 函数体内合法；非 throws 函数 → **S020**。

## 宏调用语句

```
macro-invoke-stmt ::= '#' identifier '(' arg-list? ')'
```

```syx
#assert(condition)
#for(3, print("x"))
#if(DEBUG) print("debug mode")
```

> 详见 [特性（Attributes）](06-attributes.md)。

## empty statement

```syx
;                                           // 允许空语句（不常用）
```

> SYX 不强制分号；可省略。

## 标签

```
label-stmt ::= identifier ':' stmt
```

```syx
outer: for x in items {
    for y in x.subs {
        if y.bad { break outer }            // ⚠️ 标签支持演进
    }
}
```

> **状态：** 标签基础支持；跨函数 / 跨闭包识别仍演进。

---

**诊断：** S003 / S004 / S007 / S010 / S017 / S020 / R007 / P001

**用法示例** → [语言指南：控制流](../language-guide/06-control-flow.md) · [错误处理](../language-guide/17-error-handling.md) · [并发](../language-guide/18-concurrency.md)

**参见** → [声明](05-declarations.md) · [模式](07-patterns.md) · [表达式](03-expressions.md) · [语法摘要](10-grammar-summary.md)
