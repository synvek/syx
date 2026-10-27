# Grammar Summary

**Implementation status:** summary tables **Implemented**; kept strictly in sync with `syx-parser`

This summary is distilled from `syx-parser` and [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md). For the complete BNF / LR derivation tables, see the compiler documentation.

---

## Notation

| Notation | Meaning |
|------|------|
| `A ::= B` | A derives B |
| `A \| B` | Alternative |
| `A?` | Zero or one |
| `A*` | Zero or more |
| `A+` | One or more |
| `[A]` | Grouping |
| `'kw'` | Keyword terminal |
| `terminal` | Terminal |
| `nonterm` | Nonterminal |

## Modules

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

## Imports and Global Variables

```
import-decl       ::= 'import' dotted-name ('as' identifier)?

var-global-decl   ::= access? attrs? ('let' | 'var') identifier (':' type)? ('=' expr)?
```

## Functions

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

## Initialization / Deinitialization

```
init-decl   ::= access? attrs? 'async'? 'init' '(' param-list? ')' (throws-clause)? where-clause? block
             |  access? attrs? 'async'? 'init' '?' '(' param-list? ')' (throws-clause)? where-clause? block

deinit-decl ::= access? attrs? 'deinit' block
```

## Classes

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

## Structures

```
struct-decl ::= access? attrs? 'struct' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' struct-member* '}'

struct-member ::= var-decl | func-decl | computed-property | subscript-decl
                |  init-decl | nested-decl
```

## Enumerations

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

## Protocols

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

## Extensions

```
extension-decl ::= access? attrs? 'extension' type where-clause? ('{' extension-member* '}' | extension-method-default)
                |  access? attrs? 'extension' type ':' protocol (',' protocol)* where-clause? '{' … '}'
```

## Generics

```
generic-params ::= '<' identifier (',' identifier)* ('=' type)? (',' identifier ('=' type)?)* '>'
                |  '<' identifier (':' constraint)? (',' identifier (':' constraint)?)* ('where' where-clause)? '>'

where-pred ::= type-identifier ':' type
            |  type-identifier '==' type
            |  type-identifier conformance
```

## Types

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

## Statements

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

## Expressions

```
expr ::= assign-expr

assign-expr ::= ternary-expr ('=' assign-expr)?
             |  ternary-expr ('+=' | '-=' | '*=' | '/=' | '%=') assign-expr

ternary-expr ::= nil-coalesce                    -- SYX deliberately has no ?: ternary (use an if expression)

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

## Patterns

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

## Literals

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

## Access Control

```
access ::= 'public' | 'internal' | 'private' | 'fileprivate'
```

| Level | Cross-module | Within module | Within file |
|------|--------|--------|--------|
| `public` | ✅ | ✅ | ✅ |
| `internal` | ❌ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ |
| `private` | ❌ | ❌ | Same scope |

## Custom Operators

```
operator-decl ::= ('prefix' | 'infix' | 'postfix') 'operator' identifier
                  '{' 'precedence' ':' int ',' 'associativity' ':' ('left' | 'right' | 'none') '}'
```

## Attributes

```
attribute-decl ::= '@' identifier (attribute-args)?
attribute-args ::= '(' (attribute-arg (',' attribute-arg)*)? ')'
attribute-arg  ::= identifier (':' expr)?
                |  expr                          -- positional
                |  identifier ('as' type)?       -- cast
```

## Macro Declarations

```
macro-decl ::= 'macro' identifier '=' '#externalMacro' '(' 'module' ':' string ',' 'type' ':' string ')'
            |  '#macro' identifier '(' param-list? ')' block     -- User freestanding
```

## Entry Point

```
entry ::= 'func' 'main' '(' ')' ('async')? block
```

---

**Complete AST nodes:** `crates/syx-ast/src/lib.rs`

**Usage examples** → [Language Guide (whole book)](../language-guide/00-preface.md)

**Compiler BNF:** [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md) · [docs/1_LexicalAnalysis.md](../../docs/1_LexicalAnalysis.md)
