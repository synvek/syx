# Methods

**Implementation status:** instance methods / static methods / `override` / protocol default implementations / `mutating` **Implemented**; `@objc` / KVO / `inout` parameters **Not implemented**

A method is a function associated with a type. SYX has four kinds of method: instance methods, static methods, initializers (`init`, see [Chapter 15](15-initialization.md)), and deinitializers (`deinit`, see [Chapter 16](16-deinitialization.md)). This chapter covers the first two plus overriding.

## Instance Methods

```syx
class Counter {
    var value: Int = 0
    func increment() { value = value + 1 }
    func increment(by n: Int) { value = value + n }
    func describe() -> String { return "Counter(\(value))" }
}

let c = Counter()
c.increment()
c.increment(by: 5)
print(c.describe())                        // "Counter(6)"
```

- The implicit `self` may be omitted (when unambiguous); ambiguous cases need an explicit `self.x`
- Named parameters work as in functions: `label: value` is required

## Static Methods (`static func`)

```syx
class Factory {
    static func create() -> Factory { return Factory() }
    static var defaultName: String = "anon"
}

let f = Factory.create()
print(Factory.defaultName)
```

| Call | Form |
|------|------|
| `Type.method()` | `static func` / `static var` |
| `instance.method()` | Instance method |

> **static limits:** `static func` has no `self` referring to an instance; it can access only other `static` members.

## Protocol Default Implementations

A protocol may provide a default method implementation, and adopting types inherit the default:

```syx
protocol Greetable {
    func greet() -> String
    func greet() -> String { return "Hello" }   // Default implementation
}

class User: Greetable {
    // Not implementing greet() inherits the protocol default → "Hello"
}

class Loud: Greetable {
    func greet() -> String { return "HI!" }    // Overrides the default
}

print(User().greet())                        // "Hello"
print(Loud().greet())                        // "HI!"
```

## Overriding

A subclass overrides a superclass method (see [Chapter 14, Inheritance](14-inheritance.md)):

```syx
class Animal {
    func speak() { print("...") }
    func describe() -> String { return "Animal" }
}

class Dog: Animal {
    override func speak() { print("woof") }
    override func describe() -> String { return "Dog" }
}
```

- The `override` keyword is mandatory
- A missing `override` → **S051**
- A method not declared in the superclass → **S052**

## mutating Methods (struct / enum)

`mutating func` marks methods that modify `self`:

```syx
struct Counter {
    var value: Int = 0
    mutating func increment() { value = value + 1 }
}

var c = Counter()
c.increment()
print(c.value)                             // 1

let c2 = Counter()
// c2.increment()                          // ❌ S020: cannot mutate immutable
```

- It must be called on a `var` binding
- A method body that modifies `self` without the `mutating` marker → compile error

## Methods as Values

```syx
class Helper {
    func shout(_ s: String) { print("!!! \(s) !!!") }
}

let h = Helper()
let f = h.shout                              // Bound method
f("hello")                                   // !!! hello !!!

let arr: Array<(String) -> Void> = [h.shout]
arr[0]("world")                              // !!! world !!!
```

- When binding a method you write no `()`; its type is the function type `(P) -> R`
- `f` holds a strong reference to `h` (watch out for reference cycles)

## Extension Methods

`extension` adds methods to an existing type (see [Chapter 22, Extensions](22-extensions.md)):

```syx
extension Int {
    func doubled() -> Int { return self + self }
}

print(3.doubled())                           // 6
```

## Subscript Methods

`subscript` declares a subscript inside a type (see [Chapter 13, Subscripts](13-subscripts.md)):

```syx
class Matrix {
    var data: Array<Array<Int>> = [[1, 2], [3, 4]]
    subscript(row: Int, col: Int) -> Int {
        get { return data[row][col] }
        set { data[row][col] = newValue }
    }
}

let m = Matrix()
print(m[0, 1])                               // 2
```

## Static Storage

`static var` is type-level storage (shared across instances of a class):

```syx
class Counter {
    static var totalInstances: Int = 0
    init() { Counter.totalInstances = Counter.totalInstances + 1 }
}

Counter()
Counter()
print(Counter.totalInstances)                // 2
```

> **Status:** `static var` is **Implemented** (Phase 134+); `willSet`/`didSet` on a `static var` is **Phase 138 GA** (class only; see [Chapter 11](11-properties.md)).

## Methods and Actor Isolation

`actor` methods are isolated automatically:

```syx
actor Counter {
    var value: Int = 0
    func increment() { value = value + 1 }   // Inside the isolation domain, value may be modified
}

let c = Counter()
await c.increment()                          // Crossing isolation requires await
```

## Complete Example

```syx
@Equatable
data class Point(x: Int, y: Int) {
    func distance(to other: Point) -> Double {
        let dx = Double(x - other.x)
        let dy = Double(y - other.y)
        return (dx * dx + dy * dy).squareRoot()
    }
}

class Robot {
    var position: Point
    let id: Int
    static var nextId: Int = 0

    init(at p: Point) {
        self.position = p
        self.id = Robot.nextId
        Robot.nextId = Robot.nextId + 1
    }

    func moveTo(_ p: Point) {
        position = p
    }

    func describe() -> String {
        return "Robot #\(id) @ \(position)"
    }
}

func main() {
    let r1 = Robot(at: Point(x: 0, y: 0))
    let r2 = Robot(at: Point(x: 3, y: 4))
    r1.moveTo(Point(x: 1, y: 1))
    print(r1.describe())                      // "Robot #0 @ Point(x: 1, y: 1)"
    print(r2.position.distance(to: r1.position))
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S001** | Method name conflicts with a field | Rename it |
| **S004** | Parameter / return type mismatch | Fix it |
| **S020** | `mutating` called on a `let` | Change it to `var` |
| **S051** | Missing `override` keyword | Add `override` |
| **S052** | Overriding something not declared in the superclass | Remove `override` |
| **A002** | Cross-actor call without await | Add `await` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `@objc` / `@nonobjc` | **Not implemented** |
| `inout` parameters | **Not implemented**; borrow with `&mut T` |
| `final func` forbidding override | **Not implemented** (use `final class`) |
| `mutating func` | **Implemented** |
| `@MainActor func` | **Implemented** (see [Chapter 18](18-concurrency.md)) |
| `throws` methods | **Implemented** (`throws func` fully supported) |
| `async` methods | **Implemented** (`async func`) |

---

**Next chapter** → [Subscripts](13-subscripts.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 11, Properties](11-properties.md) · [Chapter 14, Inheritance](14-inheritance.md) · [Chapter 22, Extensions](22-extensions.md)
