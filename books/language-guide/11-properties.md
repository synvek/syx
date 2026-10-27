# Properties

**Implementation status:** **Partial GA** (Phase 138 Class Properties v6 MVP)

A property is a stored value associated with a type or an instance.

## Stored Properties

```syx
class Person {
    var name: String
    let id: Int
    init(name: String, id: Int) {
        self.name = name
        self.id = id
    }
}
```

| Keyword | Member |
|--------|------|
| `var` | Mutable |
| `let` | Immutable |

## Global and Static Properties

```syx
let globalVersion = 1

class Config {
    static var debug = false
    static let version: Int = 1
}
```

**Partial GA** — since Phase 134, `static var` / `static let` MVP inside a class (type-level storage slots).
**Phase 138 GA** — `willSet` / `didSet` MVP on a class `static var` (`lang-props-v6:static-observers-mvp`; with `oldValue` / `newValue` bindings; **not** the full set of struct / global observers).

```syx
class Counter {
    static var value: Int = 0 {
        willSet { print(newValue) }
        didSet { print(oldValue) }
    }
}
// Counter.value = 7 → willSet(7), didSet(0)
```

## Computed Properties

**Partial GA** — Swift-style computed MVP inside classes and extensions:

```syx
class Box {
    var base: Int
    init(base: Int) { self.base = base }
    var doubled: Int {
        get { return self.base * 2 }
    }
    var value: Int {
        get { return self.base }
        set { self.base = newValue }
    }
}

extension Box {
    var labeled: String {
        get { return "x" }
        set { self.base = 0 }
    }
}
```

| Capability | Status |
|------|------|
| class `var x: T { get { … } }` get-only | **Phase 134 GA** |
| class `var x: T { get set }` (`newValue`) | **Phase 135 GA** |
| Subclass `override var` computed (polymorphic read/write) | **Phase 135 GA** |
| extension `var x: T { get }` / `{ get set }` | **Phase 136 GA** (`lang-props-v4:extension-computed-mvp`) |
| `@Published` on a class stored `var` | **Phase 137 GA** (`lang-props-v5:published-mvp`; a narrow `willSet` notification `print("published:<name>")` before assignment; not full Combine) |
| class `static var` + `willSet` / `didSet` | **Phase 138 GA** (`lang-props-v6:static-observers-mvp`) |
| lazy thread safety / struct `@Published` / the full set of global observers | **139+** |

Fingerprint: `lang-props-v6:docs-honesty` — the complete Swift property model → **139+**.

## Property Observers

`willSet` / `didSet`: **Partial GA** (since Phase 65 direct assignment to stored properties can trigger them; deepened in Phase 134):

| Capability | Status |
|------|------|
| Stored property `willSet` / `didSet` | **Partial GA** |
| `oldValue` / `newValue` bindings | **GA** |
| **Inheritance**: a subclass instance writing an inherited field triggers the superclass observer | **Phase 134 GA** |
| **Suppressed inside `init`**: `self.field = …` does not trigger observers | **Phase 134 GA** (aligned with Swift) |
| class **`static var`** `willSet` / `didSet` | **Phase 138 GA** |
| Observers on computed properties | **Rejected** (S056) |
| `@Published`, global observer runtime | **Partial GA** (`@Published` → Phase 137; the full set of global observers → **139+**) |

```syx
class Animal {
    var name: String
        willSet { print("will") }
        didSet { print("did") }
    init(name: String) { self.name = name }  // Assignment inside init does not print
}
class Dog: Animal {
    init(name: String) { super.init(name: name) }
}
// Writing name through a Dog instance triggers Animal's willSet/didSet
```

## Property Override

A subclass may `override var` to replace a superclass computed property; reading and writing through a base reference goes through the subclass getter/setter (vtable polymorphism):

```syx
class Base {
    var label: String { get { return "base" } }
    init() {}
}
class Derived: Base {
    override var label: String { get { return "derived" } }
    init() {}
}
// let b: Base = Derived(); b.label → "derived"
```

## Lazy Properties

`lazy var`: **Phase 134 GA** (class only; initialization triggers on first read; **not** thread-safe MVP → full semantics in **139+**).

```syx
class Box {
    lazy var label: String = "ready"
    init() {}
}
```

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Computed properties + the full set of observers | Mainly stored; computed get/set + override + extension computed + static observers MVP GA |
| `lazy var` thread safety | MVP is not thread-safe |
| `@Published` and friends | **Phase 137 MVP** (class stored; full Combine → **139+**) |
| static observers | **Phase 138 MVP** (class `static var` only; full Swift static/lazy/global → **139+**) |
| Unified struct property syntax / the full wrapper+observer ordering set | **139+** |

---

**Next chapter** → [Methods](12-methods.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md)
