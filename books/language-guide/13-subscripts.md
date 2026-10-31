# Subscripts

**Implementation status:** built-in collection / `String` subscripts **Implemented**; custom `subscript` declarations **Partially implemented** (Phase 40/41 get/set MVP); KeyPath arrays / Optional paths **Implemented** (Phase 46–48)

A subscript is the syntactic sugar `obj[i]` for accessing collection elements. SYX supports both **built-in type subscripts** (such as `Array`, `String`, `Dictionary`) and **custom `subscript` declarations** (get/set).

## Array Subscripts

```syx
let items = [10, 20, 30]
print(items[0])                            // 10
items[1] = 99
print(items)                               // [10, 99, 30]
```

| Operation | Status |
|------|------|
| `arr[i]` read | **Implemented** |
| `arr[i] = v` write | **Implemented** |
| Out of bounds | Runtime **R003** (including negative indices; ≠ `nil` / silent) |
| Negative index | **R003** (same path as out of bounds) |

Source: `crates/syx-driver/tests/fixtures/differential/suite1/array_index.sy`

## Dictionary / Object Subscripts

```syx
func main() {
    let m = ["a": 1, "b": 2]
    print(m["a"])                          // 1 (Optional<Int>)
    m["c"] = 3
    print(m.count)                         // 3

    let obj: Object = { "name": "SYX", "version": 110 }
    print(obj["name"])                     // "SYX"
}
```

> **Optional return:** `dict[k]` always returns `Optional<V>`, because at runtime "the key does not exist" cannot be distinguished from "the value is nil".

## String Subscripts

```syx
let s = "abc"
print(s[0])                                // "a" (grapheme Character)
print(s[1])                                // "b"
```

| Index semantics | **Int grapheme offset** (Phase 84; ADR-227) |
|---------|---------------------------------------------|
| Complexity | **O(n) for a single random access** (`.nth` grapheme iteration; ≠ Swift's opaque Index) |
| Return type | grapheme `Character` |
| Out of bounds | **R003** (including negative indices; ≠ an empty Character) |

> **History:** before Phase 84 indexing counted Unicode scalars, so combined emoji could be split. Phase 84+ aligns with grapheme clusters; ADR-227 pins the honest O(n) Int offset (**not** opaque Index GA).

For complete String character semantics, see [Chapter 4, Strings and Characters](04-strings-and-characters.md).

## Custom subscript Declarations

A type may declare a `subscript` (MVP since Phase 40/41, with complete get/set):

```syx
class Matrix {
    var data: Array<Array<Int>>
    init(rows: Int, cols: Int) {
        var d: Array<Array<Int>> = []
        for _ in 0..<rows {
            var row: Array<Int> = []
            for _ in 0..<cols { row.append(0) }
            d.append(row)
        }
        self.data = d
    }

    subscript(row: Int, col: Int) -> Int {
        get { return self.data[row][col] }
        set { self.data[row][col] = newValue }
    }
}

func main() {
    let m = Matrix(rows: 2, cols: 2)
    m[0, 0] = 1
    m[1, 1] = 4
    print(m[0, 0])                         // 1
    print(m[1, 1])                         // 4
}
```

| Element | Description |
|------|------|
| `subscript(params) -> T` | Subscript declaration |
| `get { … }` | Read (**Phase 40 GA**) |
| `set { … }` | Write (**Phase 41 MVP**) |
| `newValue` | Implicit parameter inside the `set` block |

> **Status:** custom `subscript` declarations are **Partially implemented** (Phase 40/41); the recommended usage is "**get required, set optional**".

### Single-Parameter Subscripts

```syx
class Grid {
    var cells: Array<String> = ["a", "b", "c", "d"]
    subscript(i: Int) -> String {
        get { return self.cells[i] }
        set { self.cells[i] = newValue }
    }
}

func main() {
    let g = Grid()
    print(g[0])                            // "a"
    g[0] = "x"
    print(g[0])                            // "x"
}
```

## KeyPath Subscripts (Phase 46–48)

A KeyPath is a type-safe object reference path:

```syx
let arr = [1, 2, 3]

// Array subscript KeyPath
let kp = \Array<Int>.[0]
print(kp(arr))                             // 1
```

### Property KeyPaths

```syx
class Person {
    let name: String
    init(name: String) { self.name = name }
}

let p = Person(name: "Alice")
let nameKp = \Person.name
print(nameKp(p))                           // "Alice"
```

### Optional KeyPaths (Phase 46)

```syx
class Box {
    let value: Int?
    init(value: Int?) { self.value = value }
}

let b = Box(value: 42)
let vKp = \Box.value?.description          // Optional path
print(vKp(b))                              // "42"
```

### KeyPath Array Subscripts

```syx
let arr = [1, 2, 3]
print((\Array<Int>.[0])(arr))              // 1
```

Test: `lang/array_subscript_check.rs`

## Subscripts vs. Methods

| Need | Choose |
|------|-----|
| Array / Dictionary / Object element access | Built-in subscript |
| Indexed access on a custom type | Custom `subscript` |
| Type-safe reference paths | KeyPath |
| Dynamic string field access | `Object["key"]` |

## Complete Example

```syx
class SafeArray {
    private var data: Array<Int> = []
    private var maxIndex: Int = 0

    func append(_ x: Int) {
        self.data.append(x)
        self.maxIndex = self.data.count - 1
    }

    subscript(i: Int) -> Int? {             // get-only
        get {
            if i < 0 { return nil }
            if i > self.maxIndex { return nil }
            return self.data[i]
        }
    }

    subscript(safe i: Int, default d: Int) -> Int {
        get {
            if i < 0 { return d }
            if i > self.maxIndex { return d }
            return self.data[i]
        }
    }
}

func main() {
    var a = SafeArray()
    a.append(1)
    a.append(2)
    a.append(3)

    print(a[0])                             // Optional(1)
    print(a[10])                            // nil
    print(a[safe: 10, default: -1])         // -1
}
```

> **Multi-label parameters:** subscripts support multiple parameters (`subscript(row: Int, col: Int)`) and multiple labels (`subscript(safe i: Int, default d: Int)`).

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Subscript parameter / return type mismatch | Fix it |
| **P001** | subscript syntax error | Check the get/set blocks |
| **R003** | Array / String subscript out of bounds | Check the index range |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Grapheme-cluster subscripts | **Implemented** (since Phase 84) |
| Custom `subscript` get/set | **Partially implemented** (Phase 40/41) |
| Full KeyPath surface | **Partially implemented** (`\Type.prop` / `\Type.[i]` / `\Type?.prop` are supported; dynamic member lookup is not implemented) |
| `subscript(dynamicMember:)` | **Not implemented** |
| Protocol subscript requirements | **Partially implemented** |
| subscript labels / multiple labels | **Implemented** |

---

**Next chapter** → [Inheritance](14-inheritance.md)

**See also** → [Language Reference: Declarations](../language-reference/05-declarations.md) · [Language Reference: Expressions](../language-reference/03-expressions.md) · [Chapter 4, Strings and Characters](04-strings-and-characters.md) · [Chapter 11, Properties](11-properties.md)
