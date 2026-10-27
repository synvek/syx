# Statements

**Implementation status:** **Implemented** (`Stmt` enum); control flow / exceptions / pattern matching / async are fully covered; labels **Partially implemented**; fallthrough **Rejected**

Statements control the flow of execution and do not directly serve as expression values. `print` is a language-level global Host (signature in [`packages/core/builtins/special/globals.sy`](../../packages/core/builtins/special/globals.sy)) whose call syntax looks like a function.

---

## Classification

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

## Variable Declarations

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

| Keyword | Mutability | Reassignment |
|--------|--------|----------|
| `let` | Immutable | ❌ S003 |
| `var` | Mutable | ✅ |
| `async let` | Immutable | ❌ |
| `async var` | Mutable | ✅ |

> **Type inference:** a `let` type is frozen after its first assignment; a later incompatible assignment → **S004**.

**Diagnostics:** S003 (assignment to `let`); S002 (undefined)

## Expression Statements

```
expr-stmt ::= expr
```

```syx
foo()                                       // Function call
x = 1                                       // Assignment
arr.append(2)                                // Method call
```

> An **assignment expression** is a legal statement; it returns no value usable for chaining.

## return

```
return-stmt ::= 'return' expr?
```

```syx
return
return 42
return result
```

> With no `expr`, this is equivalent to `return ()` (Void); a non-Void function must `return expr`.

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

> **Condition parentheses are optional:** both `if expr` and `if (expr)` are legal; omitting them is recommended (consistent with Swift).
> **Condition type** must be `Bool`; anything else → **S004**.
> **if expression** (Phase 100+): when used as an expression, both branches must exist and have the same type.

## while

```
while-stmt ::= 'while' expr block
```

```syx
while x > 0 {
    x = x - 1
}
```

> Condition parentheses are optional.

## repeat while

```
repeat-while-stmt ::= 'repeat' block 'while' expr
```

```syx
repeat {
    n = n + 1
} while n < 3
```

> The loop body executes **at least** once; the condition is checked after each iteration. Condition parentheses are optional.

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

Iterable:

| Type | Element type |
|------|----------|
| `Array<T>` | `T` |
| `String` | `String` (single scalar) |
| `Set<T>` | `T` |
| `Dictionary<K, V>` | `(K, V)` tuple |
| `0..<n` | `Int` |
| Custom `Iterator` | `String` |
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

// Labels
outer: for x in items {
    inner: for y in x.subs {
        if y.shouldStop { break outer }     // ⚠️ Labels are partially implemented
    }
}
```

> Labels / cross-level `break` / `continue` are **Partially implemented**.

## Blocks

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

> Note: a `try` block uses `{ }`; `catch` bindings **must** carry `()`, because the `catch err` form is ambiguous with statements (catch is a keyword plus an identifier, which without parentheses collides with `let`/binding).

```syx
try {
    throw "error"
    print("no catch")
} catch (err) {
    print(err)                              // err: String (default when the type is omitted)
}
try {
    throw "typed"
} catch (err: String) {
    print(err)                              // Explicit type is optional (Java-like)
}
```

> **Multiple catch clauses:** several `catch` clauses plus `where` guards are supported (**Partially implemented**).
> **Catch binding type** defaults to `String` when omitted; `catch (e: T)` may be written; typed throws (`throws T`) are **Partially implemented**.

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

> Condition parentheses are optional; both `switch expr` and `switch (expr)` are legal.

> **`case` block syntax:** a SYX `case` **must** be followed by a `{ }` block (**not** `case 1:` plus indentation).
> **`default`:** may be omitted for an `enum` / closed domain (the compiler checks full case coverage → **S007**); an open domain needs `default`.
> **`fallthrough`:** **Rejected** (compile error).

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
    return                                  // Early exit required
}
print("y is \(y)")
```

> **`guard let` else** must exit (`return` / `throw` / `break` / `continue` / an infinite loop) — otherwise **S020**.
> **Multiple patterns:** `if let x = a, let y = b, cond { … }` (chaining + guards) — legal.

## print

```
print-stmt ::= 'print' '(' arg-list? ')'
```

```syx
print(expr)
print("Hello", name, count)
```

> A built-in output statement (a dedicated statement, **not** a function).
> The call form `print(x)` looks like a function; underneath it is a statement node.

## throw

```
throw-stmt ::= 'throw' expr
```

```syx
throw "message"
throw err
```

> Legal only inside a `throws` function body; in a non-`throws` function → **S020**.

## Macro Invocation Statements

```
macro-invoke-stmt ::= '#' identifier '(' arg-list? ')'
```

```syx
#assert(condition)
#for(3, print("x"))
#if(DEBUG) print("debug mode")
```

> See [Attributes](06-attributes.md).

## empty statement

```syx
;                                           // An empty statement is allowed (uncommon)
```

> SYX does not require semicolons; they may be omitted.

## Labels

```
label-stmt ::= identifier ':' stmt
```

```syx
outer: for x in items {
    for y in x.subs {
        if y.bad { break outer }            // ⚠️ Label support is still evolving
    }
}
```

> **Status:** basic label support; recognition across functions / closures is still evolving.

---

**Diagnostics:** S003 / S004 / S007 / S010 / S017 / S020 / R007 / P001

**Usage examples** → [Language Guide: Control Flow](../language-guide/06-control-flow.md) · [Error Handling](../language-guide/17-error-handling.md) · [Concurrency](../language-guide/18-concurrency.md)

**See also** → [Declarations](05-declarations.md) · [Patterns](07-patterns.md) · [Expressions](03-expressions.md) · [Grammar Summary](10-grammar-summary.md)
