# Declarations

**Implementation status:** top-level and member declarations **Implemented**; inheritance GA (Phase 51–55 + Phase 66 vtable); `final class` Phase 60; `subscript` partially implemented; `convenience init` / `required init` **Not implemented**

A module consists of a list of top-level `Decl`s, with `func main()` as the entry point.

---

## Classification

```
decl ::= import-decl
      | var-global-decl
      | function-decl
      | init-decl
      | deinit-decl
      | class-decl
      | struct-decl
      | enum-decl
      | actor-decl
      | protocol-decl
      | extension-decl
      | data-class-decl
      | global-actor-decl
      | typealias-decl
      | operator-decl
      | macro-decl
      | attribute-decl           -- @attached and friends
```

## import

```
import-decl ::= 'import' identifier ('.' identifier)* ('as' identifier)?
```

```syx
import Math
import local.deps as Deps
import utils.helpers as H
```

> Only `public` symbols are accessible across modules; internal → **S005**.

## Global Variables

```
var-global-decl ::= access? attrs? ('let' | 'var') identifier (':' type)? ('=' expr)?
```

```syx
let globalConst = 42
var globalVar: Int = 0
public var shared: String = "shared"        // Cross-module
```

> Global variable initializers run once at process startup.

## function

```
function-decl ::= access? attrs? ('async')? 'func' identifier generic-params?
                  '(' param-list? ')' ('->' type)? throws-clause? effect-clause?
                  where-clause? ('=' expr | block)
```

```syx
func main() {
    …
}

public func add(a: Int, b: Int) -> Int {
    return a + b
}

async func fetch() -> String {
    return await httpGet()
}

func risky() throws -> Int {
    throw "fail"
}

func pureCalc(x: Int) -> Int requires Pure {
    return x + 1
}
```

| Clause | Description |
|------|------|
| `access` | `public` / `internal` / `private` / `fileprivate` |
| `async` | Asynchronous function |
| `throws` | Error signature (`throws T` typed throws **Partially implemented**) |
| `requires Pure` / `requires IO` | Effect annotation |
| `where T: Equatable` | Constraint clause |
| Generics | `func id<T>(x: T) -> T` |
| Single expression | `func f(x: Int) -> Int = x * 2` (**Partially implemented**) |

## init

```
init-decl ::= access? attrs? ('async')? 'init' '(' param-list? ')' (throws-clause)? where-clause? block
```

```syx
init(x: Int) {
    self.field = x
}

init?(value: Int) {
    if (value < 0) { return nil }
    self.value = value
}

init throws {
    // ...
}
```

| Form | Status |
|------|------|
| `init(params)` designated | **Implemented** |
| `init?` failable | **Implemented** |
| `init!` implicit unwrap | **Not implemented** |
| `convenience init` | **Not implemented** |
| `required init` | **Not implemented** |

## deinit

```
deinit-decl ::= access? attrs? 'deinit' block
```

```syx
deinit {
    print("cleanup")
}
```

> Members of `class` / `actor` only; a top-level `deinit` → **P001**.
> Status: **Partially implemented** (triggered when a class ARC count reaches zero; `struct` unsupported).

## class

```
class-decl ::= access? attrs? ('final')? 'class' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' class-member* '}'
```

```syx
class Person {
    var name: String
    let id: Int
    init(name: String, id: Int) {
        self.name = name
        self.id = id
    }
    func greet() {
        print("Hi, \(self.name)")
    }
    deinit { print("bye") }
}

final class Token {                         // Inheritance forbidden
    let value: String
    init(value: String) { self.value = value }
}

class Derived: Base {                       // Inheritance
    override func foo() { super.foo() }
}
```

Members may include `var` / `let` / `func` / `init` / `deinit` / computed properties / `subscript` (**Partially implemented**).

## struct

```
struct-decl ::= access? attrs? 'struct' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' struct-member* '}'
```

```syx
struct Point {
    var x: Int
    var y: Int

    func distance(to other: Point) -> Double {
        let dx = Double(x - other.x)
        let dy = Double(y - other.y)
        return (dx * dx + dy * dy).squareRoot()
    }

    mutating func translateBy(dx: Int, dy: Int) {
        x = x + dx
        y = y + dy
    }
}
```

> Value-type field aggregation (runtime MVP); `mutating func` marks methods that modify `self`.

## data class

```
data-class-decl ::= access? attrs? 'data' 'class' identifier generic-params?
                    '(' (var-or-let identifier ':' type (',' var-or-let identifier ':' type)*)? ')' where-clause?
                    '{' class-member* '}'
```

```syx
@Equatable
@Hashable
@Codable
data class User(name: String, age: Int) {
    func describe() -> String { return "User(\(name), \(age))" }
}
```

> **Phase 50–52 proof macros** (stored-property `data class`). L1: automatic `==` / `hash` / `encode` / `decode`. L2: associated-value enums fully automatic (Phase 129).
> There is no Swift-style automatic `copy` (**Not implemented**).

## enum

```
enum-decl ::= access? attrs? 'enum' identifier generic-params?
             (':' type)? where-clause? '{' enum-case* (func | var | init)* '}'
enum-case ::= 'case' identifier ('(' (identifier (':' type)? (',' identifier (':' type)?)*)? ')')?
```

```syx
enum Direction { case north; case south; case east; case west }

enum NetworkResult {
    case success(String)
    case failure(code: Int, message: String)

    func isSuccess() -> Bool {
        switch (self) {
        case .success { return true }
        case .failure { return false }
        }
    }
}

enum Optional<T> { case some(T); case none }   // Pre-declared
```

> Enumerations may contain methods, computed properties, and `init`; their memory layout differs from `class` (discriminated union).

## actor

```
actor-decl ::= access? attrs? ('global')? 'actor' identifier generic-params?
               (':' type (',' type)*)? where-clause? '{' actor-member* '}'
```

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func current() -> Int { return value }
}

global actor MainActor {                    // Global actor
}
```

> Actor state is isolated from the outside; external access requires `await` (**A002**).

## protocol

```
protocol-decl ::= access? attrs? 'protocol' identifier generic-params?
                 (':' type (',' type)*)? where-clause? '{' protocol-member* '}'
```

```syx
protocol Describable {
    func description() -> String
    var name: String { get }
}

protocol Sequence {
    associatedtype Element
    func makeIterator() -> Iterator
}

protocol Iterator {
    func next() -> String?
}
```

> Protocol members may include `func` / `var` / `init` / `associatedtype`; default implementations are **allowed** (Phase 50+).

## extension

```
extension-decl ::= access? attrs? 'extension' type where-clause? '{' extension-member* '}'
```

```syx
extension Int {
    func doubled() -> Int { return self + self }
    var isEven: Bool { get { return self % 2 == 0 } }
}

extension Array where Element: Equatable {
    func unique() -> Array<Element> { … }
}

extension MyType: Describable {
    func description() -> String { return "MyType" }
}
```

> **where constraints:** Phase 40 single constraint; Phase 42 multiple constraints.
> **Cross-module:** allowed from Phase 60+ (unlike Swift).

## global actor

```
global-actor-decl ::= access? attrs? 'global' 'actor' identifier '{' actor-member* '}'
```

```syx
global actor MainActor { }
global actor BackgroundActor { }
```

> Global actors are cross-platform abstractions: the UI thread, the GPU thread, and so on.

## typealias

```
typealias-decl ::= access? 'typealias' identifier generic-params? '=' type
```

```syx
typealias Handle = UInt64
typealias Handler = (Int) -> String
typealias StringMap = [String: String]
```

> **No new type is created**; it is a synonym only. **Phase 40 GA**.

## Properties (Member Fields)

```
property-decl ::= access? attrs? ('static')? ('let' | 'var') identifier (':' type)? ('=' expr)?
               |  access? attrs? ('static')? 'var' identifier ':' type computed-property
```

```syx
var count: Int = 0
let id: String
static let maxValue: Int = 100
static var debug: Bool = false
```

Computed properties (**Phase 134–138 partial GA**):

```syx
var doubled: Int {
    get { return self.base * 2 }
    set { self.base = newValue / 2 }
}
```

`subscript` declarations:

```syx
subscript(row: Int, col: Int) -> Int {
    get { return data[row][col] }
    set { data[row][col] = newValue }
}
```

> **Status:** custom `subscript` **Partially implemented** (Phase 40/41).

## Access Control

```
access ::= 'public' | 'internal' | 'private' | 'fileprivate'
```

| Level | Cross-module | Within module | Within file | Same scope |
|------|--------|--------|--------|----------|
| `public` | ✅ | ✅ | ✅ | ✅ |
| `internal` | ❌ S005 | ✅ | ✅ | ✅ |
| `fileprivate` | ❌ | ❌ | ✅ | ✅ |
| `private` | ❌ | ❌ | ✅ | ✅ |

> `open` is **Not implemented** (use `public` + `class` to express inheritable).

See [Language Guide: Access Control](../language-guide/28-access-control.md).

---

**Diagnostics:** S001 / S004 / S005 / S020 / S051–S055 / P001

**Usage examples** → [Language Guide: Functions](../language-guide/07-functions.md) · [Structures and Classes](../language-guide/10-structures-and-classes.md) · [Enumerations](../language-guide/09-enumerations.md) · [Concurrency](../language-guide/18-concurrency.md) · [Protocols](../language-guide/23-protocols.md) · [Extensions](../language-guide/22-extensions.md) · [Properties](../language-guide/11-properties.md)

**See also** → [Types](02-types.md) · [Generic Parameters](08-generic-parameters-and-arguments.md) · [Protocols](09-protocols-and-conformances.md) · [Attributes](06-attributes.md)
