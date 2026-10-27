# 语法摘要

**实现状态：** 摘要表 **已实现**；与 `syx-parser` 严格对齐

本摘要提炼自 `syx-parser` 与 [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md)。完整 BNF / LR 推导表请参阅编译器文档。

---

## 记号

| 记号 | 含义 |
|------|------|
| `A ::= B` | A 推导为 B |
| `A \| B` | 择一 |
| `A?` | 零或一 |
| `A*` | 零或多 |
| `A+` | 一或多 |
| `[A]` | 分组 |
| `'kw'` | 关键字终结符 |
| `terminal` | 终结符 |
| `nonterm` | 非终结符 |

## 模块

```
module ::= decl*

decl ::= import-decl
      |  var-global-decl
      |  function-decl
      |  class-decl | struct-decl | enum-decl | protocol-decl
      |  extension-decl | actor-decl | data-class-decl
      |  global-actor-decl | typealias-decl
      |  operator-decl | macro-decl
      |  attribute-decl
```

## 导入与全局变量

```
import-decl       ::= 'import' dotted-name ('as' identifier)?

var-global-decl   ::= access? attrs? ('let' | 'var') identifier (':' type)? ('=' expr)?
```

## 函数

```
function-decl ::= access? attrs? 'async'? 'func' identifier generic-params?
                 '(' param-list? ')' ('->' type)? throws-clause? effect-clause?
                 where-clause? ('=' expr | block)

param-list    ::= param (',' param)*
param         ::= ('let' | 'var')? identifier (':' type)? ('=' expr)?
               |  attrs? identifier ':' type

throws-clause ::= 'throws' type?
effect-clause ::= 'requires' ('Pure' | 'IO')
where-clause  ::= 'where' where-pred (',' where-pred)*
```

## 构造 / 析构

```
init-decl   ::= access? attrs? 'async'? 'init' '(' param-list? ')' (throws-clause)? where-clause? block
             |  access? attrs? 'async'? 'init' '?' '(' param-list? ')' (throws-clause)? where-clause? block

deinit-decl ::= access? attrs? 'deinit' block
```

## 类

```
class-decl ::= access? attrs? ('final')? 'class' identifier generic-params?
              (':' type (',' type)*)? where-clause? '{' class-member* '}'

class-member ::= var-decl | func-decl | init-decl | deinit-decl
              |  computed-property | subscript-decl | typealias-decl
              |  nested-class-decl | nested-struct-decl | nested-enum-decl

var-decl         ::= attrs? ('let' | 'var') identifier (':' type)? ('=' expr)?
computed-property ::= attrs? ('static')? 'var' identifier ':' type '{' get-clause set-clause? '}'
get-clause       ::= 'get' block
set-clause       ::= 'set' (identifier)? block
subscript-decl   ::= attrs? ('static')? 'subscript' param-list ('->' type)? '{' get-clause set-clause? '}'
```

## 结构体

```
struct-decl ::= access? attrs? 'struct' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' struct-member* '}'

struct-member ::= var-decl | func-decl | computed-property | subscript-decl
                |  init-decl | nested-decl
```

## 枚举

```
enum-decl ::= access? attrs? 'enum' identifier generic-params?
             (':' type)? where-clause? '{' enum-case* (member)* '}'

enum-case ::= 'case' identifier ('(' (case-param (',' case-param)*)? ')')?
case-param ::= identifier (':' type)?
```

## data class

```
data-class-decl ::= access? attrs? 'data' 'class' identifier generic-params?
                   '(' (var-or-let identifier ':' type (',' var-or-let identifier ':' type)*)? ')'
                   where-clause? '{' class-member* '}'
```

## Actor

```
actor-decl ::= access? attrs? ('global')? 'actor' identifier generic-params?
              (':' type (',' type)*)? where-clause? '{' actor-member* '}'

global-actor-decl ::= access? attrs? 'global' 'actor' identifier '{' actor-member* '}'
```

## 协议

```
protocol-decl ::= access? attrs? 'protocol' identifier generic-params?
                 (':' type (',' type)*)? where-clause? '{' protocol-member* '}'

protocol-member ::= protocol-method
                 |  protocol-property
                 |  protocol-init
                 |  protocol-associatedtype
                 |  protocol-method-default

protocol-method ::= 'func' identifier generic-params? '(' param-list? ')' ('->' type)? throws-clause?
protocol-method-default ::= 'func' identifier generic-params? '(' param-list? ')' ('->' type)? throws-clause? block
protocol-property ::= ('var' identifier ':' type accessors | 'let' identifier ':' type)
protocol-associatedtype ::= 'associatedtype' identifier ('=' type)?
```

## 扩展

```
extension-decl ::= access? attrs? 'extension' type where-clause? ('{' extension-member* '}' | extension-method-default)
                |  access? attrs? 'extension' type ':' protocol (',' protocol)* where-clause? '{' … '}'
```

## 泛型

```
generic-params ::= '<' identifier (',' identifier)* ('=' type)? (',' identifier ('=' type)?)* '>'
                |  '<' identifier (':' constraint)? (',' identifier (':' constraint)?)* ('where' where-clause)? '>'

where-pred ::= type-identifier ':' type
            |  type-identifier '==' type
            |  type-identifier conformance
```

## 类型

```
type ::= named-type
      |  optional-type
      |  array-type
      |  dict-type
      |  tuple-type
      |  function-type
      |  reference-type
      |  opaque-type
      |  metatype
      |  '(' type ')'

named-type     ::= identifier ('<' type-args '>')?
optional-type  ::= type '?'
array-type     ::= '[' type ']' | 'Array' '<' type '>'
dict-type      ::= '[' type ':' type ']' | 'Dictionary' '<' type ',' type '>'
tuple-type     ::= '(' type (',' type)+ ')'
function-type  ::= '(' type-list? ')' '->' type
reference-type ::= '&' 'mut'? lifetime? type
opaque-type    ::= 'some' type | 'any' type
metatype       ::= 'Self' | 'SelfType' | 'Type'
```

## 语句

```
stmt ::= var-decl-stmt
       | expr-stmt
       | return-stmt
       | if-stmt
       | while-stmt
       | repeat-while-stmt
       | for-in-stmt
       | break-stmt | continue-stmt
       | block-stmt
       | try-stmt
       | throw-stmt
       | switch-stmt
       | if-case-stmt
       | if-let-stmt | guard-let-stmt
       | print-stmt
       | macro-invoke-stmt

var-decl-stmt    ::= ('let' | 'var') pattern (':' type)? ('=' expr)?
                  |  'async' ('let' | 'var') pattern '=' expr
return-stmt      ::= 'return' expr?
if-stmt          ::= 'if' '(' expr ')' block ('else' (if-stmt | block))?
while-stmt       ::= 'while' '(' expr ')' block
repeat-while-stmt::= 'repeat' block 'while' '(' expr ')'
for-in-stmt      ::= 'for' 'await'? pattern 'in' expr block
break-stmt       ::= 'break' label?
continue-stmt    ::= 'continue' label?
block-stmt       ::= '{' stmt* '}'
try-stmt         ::= 'try' block ('catch' '(' identifier ')' block)+
throw-stmt       ::= 'throw' expr
switch-stmt      ::= 'switch' '(' expr ')' '{' case-clause* '}'
case-clause      ::= 'case' pattern ('where' expr)? block | 'default' block
if-case-stmt     ::= 'if' 'case' pattern '=' expr block ('else' block)?
if-let-stmt      ::= 'if' 'let' pattern ('=' expr (',' expr)*)? (',' cond)* block ('else' block)?
guard-let-stmt   ::= 'guard' 'let' pattern ('=' expr (',' expr)*)? (',' cond)* 'else' block
print-stmt       ::= 'print' '(' arg-list? ')'
macro-invoke-stmt::= '#' identifier '(' arg-list? ')'
```

## 表达式

```
expr ::= assign-expr

assign-expr ::= ternary-expr ('=' assign-expr)?
             |  ternary-expr ('+=' | '-=' | '*=' | '/=' | '%=') assign-expr

ternary-expr ::= nil-coalesce                    -- SYX 故意无 ?: 三元（用 if 表达式）

nil-coalesce ::= logical-or ('??' logical-or)?

logical-or   ::= logical-and ('||' logical-and)*
logical-and  ::= bitwise-or ('&&' bitwise-or)*
bitwise-or   ::= bitwise-xor ('|' bitwise-xor)*
bitwise-xor  ::= bitwise-and ('^' bitwise-and)*
bitwise-and  ::= equality ('&' equality)*
equality     ::= comparison (('==' | '!=' | '===' | '!==') comparison)*
comparison   ::= range (('<' | '<=' | '>' | '>=') range)*
range        ::= additive (('..<' | '...') additive)?
additive     ::= multiplicative (('+' | '-') multiplicative)*
multiplicative ::= unary (('*' | '/' | '%' | '<<' | '>>') unary)*
unary        ::= ('-' | '!' | '~' | 'try' | 'await' | 'await?' | '&' | '&mut')* postfix

postfix      ::= primary postfix-op*
postfix-op   ::= '.' identifier
             |  '(' arg-list? ')'
             |  '[' expr (',' expr)* ']'
             |  '?' ('.' identifier | '(' arg-list? ')' | '[' expr ']')?
             |  '!'

primary      ::= literal
             |  identifier
             |  'self' | 'super' | 'Self' | 'this'
             |  '(' expr (',' expr)* ')'
             |  closure
             |  'await' expr
             |  'try' expr ('?' | '!')?
             |  if-expr
             |  switch-expr
             |  array-literal
             |  dict-literal
             |  object-literal
             |  string-literal
             |  regex-literal
```

## 模式

```
pattern ::= '_'
         |  identifier
         |  'let' pattern
         |  literal
         |  range-pattern
         |  variant-pattern
         |  tuple-pattern
         |  type-identifier
         |  '(' pattern ')'
         |  pattern 'as' type

range-pattern    ::= expr '..<' expr
variant-pattern  ::= '.' identifier ('(' pattern (',' pattern)* ')')?
tuple-pattern    ::= '(' pattern (',' pattern)* ')'
```

## 字面量

```
literal ::= int-literal
         |  float-literal
         |  string-literal
         |  bool-literal ('true' | 'false')
         |  nil-literal ('nil')
         |  array-literal
         |  dict-literal
         |  object-literal
         |  regex-literal
```

## 访问控制

```
access ::= 'public' | 'internal' | 'private' | 'fileprivate'
```

| 级别 | 跨模块 | 模块内 | 文件内 |
|------|--------|--------|--------|
| `public` | ✅ | ✅ | ✅ |
| `internal` | ❌ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ |
| `private` | ❌ | ❌ | 同 scope |

## 自定义运算符

```
operator-decl ::= ('prefix' | 'infix' | 'postfix') 'operator' identifier
                  '{' 'precedence' ':' int ',' 'associativity' ':' ('left' | 'right' | 'none') '}'
```

## 特性（Attributes）

```
attribute-decl ::= '@' identifier (attribute-args)?
attribute-args ::= '(' (attribute-arg (',' attribute-arg)*)? ')'
attribute-arg  ::= identifier (':' expr)?
                |  expr                          -- positional
                |  identifier ('as' type)?       -- cast
```

## 宏声明

```
macro-decl ::= 'macro' identifier '=' '#externalMacro' '(' 'module' ':' string ',' 'type' ':' string ')'
            |  '#macro' identifier '(' param-list? ')' block     -- 用户 freestanding
```

## 入口

```
entry ::= 'func' 'main' '(' ')' ('async')? block
```

---

**完整 AST 节点：** `crates/syx-ast/src/lib.rs`

**用法示例** → [语言指南全书](../language-guide/00-preface.md)

**编译器 BNF：** [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md) · [docs/1_LexicalAnalysis.md](../../docs/1_LexicalAnalysis.md)
