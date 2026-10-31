# Nested Types

**Implementation status:** top-level types **Implemented**; `class` / `struct` / `enum` / `final class` nested inside a type **MVP GA** (Phase 139); the `Outer.Inner` qualified path **MVP GA**; nested visibility / inheritance / outer-instance access **incomplete**

Swift allows nesting `struct` / `enum` / `class` / `protocol` inside a type. SYX **Phase 139 MVP** supports common nested declarations and `Outer.Inner` path references; this chapter describes the current capabilities, the honest boundaries, and recommended usage.

## Current Status

| Capability | Status |
|------|------|
| Top-level `class` / `struct` / `enum` / `actor` | **Implemented** |
| Top-level `protocol` | **Implemented** |
| `class` / `struct` / `enum` / `final class` inside a type | **MVP GA** (Phase 139) |
| Nested `protocol` | **Partially implemented** |
| `Outer.Inner` reference syntax (type and construction) | **MVP GA** |
| Referring to a nested type from an outer type (common case) | **MVP GA** |
| Multi-level paths `Outer.Middle.Inner` | **MVP GA** |
| Nested type inheritance / outer instance access | **Not implemented** |
| `private` nested visibility | **Not implemented** (file scope applies) |

> **Honest boundary:** internally the name is still mangled as `Outer_Inner`; you write `Outer.Inner` in source and parser/semantic resolve it to a stable path. This is not the complete Swift nested-scope rule set.

## Basic Syntax

```syx
class Outer {
    class Inner {
        func value() -> Int { return 7 }
    }
}

func main() {
    var inner = Outer.Inner()
    print(inner.value())                    // 7
}
```

Type annotations use the dotted path as well (mapped internally to `Outer_Inner`):

```syx
class Outer {
    class Inner {
        func value() -> Int { return 7 }
    }
}

var x: Outer.Inner = Outer.Inner()
```

## Multiple Levels of Nesting

```syx
class Outer {
    class Middle {
        class Deep {
            func value() -> Int { return 9 }
        }
    }
}

func main() {
    var d = Outer.Middle.Deep()
    print(d.value())                        // 9
}
```

## External References (Common Case)

A nested type may be held by an outer sibling type or used as a parameter:

```syx
class Outer {
    class Inner {
        func value() -> Int { return 7 }
    }
}

class Holder {
    var inner: Outer.Inner
    init(inner: Outer.Inner) {
        self.inner = inner
    }
    func read() -> Int {
        return self.inner.value()
    }
}

func main() {
    var h = Holder(inner: Outer.Inner())
    print(h.read())                         // 7
}
```

Across modules: the exporting side writes `public class Outer { public class Inner { ... } }`, and the importing module may write `Outer.Inner()` (see the `nested_cross_module` fixture).

## Nesting Inside Extensions (Partial)

```syx
struct Point {
    var x: Int
    var y: Int
}

extension Point {
    struct Polar {
        var r: Double
        var theta: Double
    }

    func toPolar() -> Point.Polar {
        let r = Double(self.x * self.x + self.y * self.y)
        let theta = Double(self.x) / Double(self.y)
        return Point.Polar(r: r, theta: theta)
    }
}
```

> **Nesting inside extensions:** partially supported; access usually goes through an outer instance method.

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Type nesting | **MVP GA** (common cases) |
| `Outer.Inner` path access | **MVP GA** |
| Nested type inheritance | **Not implemented** |
| Nested types accessing the outer instance | **Not implemented** |
| `private` nesting | **Not implemented** |

## Flat Naming (Still Usable)

The historical flat style remains compatible and is equivalent to manual name mangling:

```syx
class OuterMarker_Inner {
    var value: Int = 0
}
```

New code should prefer the `Outer.Inner` dotted path.

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **P001** | Nested syntax parse failure | Check the nested decl syntax |
| **S002** | Unknown type / unresolved path | Confirm the outer type name and nested member exist |

---

**Next chapter** → [Extensions](22-extensions.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 10, Structures and Classes](10-structures-and-classes.md)
