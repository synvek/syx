# Collection Types

**Implementation status:** `Array<T>` / `Set<T>` / `Dictionary<K,V>` / `Object` are **production-usable** (Dictionary is still a String-shell, see below); HOFs `map/filter/reduce` are **GA** (three-back-end parity); user Sequence/Iterator `for-in` **Implemented** (Phase 71); the bare `Array` (without type arguments) has been **removed** (Phase 60). The honest product surface has been aligned with the method tables in this chapter (bare signatures in the facade + sema parameterization are annotated alongside); `OrderedSet` is **not production GA** (`ordered_set.sy` exists, host capabilities are all unsupported). Remaining: **monomorphization**, a true `HashMap<Value, Value>` key shell.

SYX provides four built-in collections: ordered `Array<T>`, deduplicating `Set<T>`, key-value `Dictionary<K,V>`, and dynamic `Object`. This chapter covers literals, methods, iteration, conversion to/from IO, and type constraints.

## Arrays

`Array<T>` is the most commonly used ordered mutable sequence:

```syx
let nums: Array<Int> = [1, 2, 3]
let mixed: Array<Any> = [1, "a", true]     // Requires an Any constraint
let empty: Array<Int> = []

print(nums.count)                          // 3
print(nums[0])                              // 1
nums[1] = 99                                // ✅ Mutable
```

> **Strong typing:** since Phase 60 a bare `Array` is **forbidden** (type arguments are required). Use `Array<Any>` or `Object` for mixed types.

### Common Methods

| Method | Signature | Description |
|------|------|------|
| `.count` | `Int` | Number of elements (test emptiness with `.count == 0`; there is no `.isEmpty`) |
| `.contains(item)` | `Bool` | Membership test (`Equatable`; there is no predicate version `.contains(where:)`) |
| `.sorted()` | `Array<T>` | Ascending sort (a new array) |
| `.sortBy(comparator)` | `Array<T>` | Custom comparison (the facade still uses bare `Array` / `Function`; ≠ `.sorted(by:)`) |
| `.reverse()` | `Array<T>` | Reversed (a new array; the real name is `.reverse()`, ≠ `.reversed()`) |
| `.chunked(size)` | `Array<Array<T>>` | Chunk by length (the facade still uses `Array<Array>`) |
| `.zip(other)` | `Array<Array<T>>` | Pair with `other` (when both sides share `T`; otherwise `Array<Array<Object>>`, ≠ a silent `Array<Array<T>>`; the facade still uses a bare `Array`) |
| `.joined(sep:)` | `String` | Join into a string with a separator |
| `.map(transform)` | `Array<U>` | HOF (**GA**); `U` is inferred from the closure result type (the facade still uses `Function`, ≠ mono / `map<U>`) |
| `.filter(predicate)` | `Array<T>` | HOF (**GA**); preserves the element type `T` (the facade still uses `Function`) |
| `.reduce(initial, combine)` | `U` | HOF (**GA**); `U` is inferred from `initial` (the facade still uses `initial: Int` / `-> Int` / `Function`, ≠ mono) |
| `.flatMap(transform)` | `Array<U>` | HOF (**GA**); when the closure result is `Array<U>`, one level is unwrapped to `Array<U>`, otherwise it is treated as a single element (the facade still uses `Function`, ≠ mono) |
| `.append(item)` | `Void` | Append |

There is also a facade `.binarySearch` (still `item: Int`). **Not implemented** (calling them → **S002**): `.isEmpty`, `.first` / `.last`, `.reversed()`, `.remove(at:)` / `.removeAll()`, `.index(of:)`, `.slice`, `.contains(where:)`, `.allSatisfy`, `.forEach`. Use `for` to iterate.

### Array Literals and Inference

```syx
let a = [1, 2, 3]                       // Infers Array<Int>
let b: Array<Float64> = [1.0, 2.0]      // Explicit
let c: Array<Int> = []                  // An empty array must be annotated or initialized
```

### Higher-Order Functions (HOF)

```syx
let nums = [1, 2, 3, 4, 5]

let doubled = nums.map({ x in x * 2 })             // [2, 4, 6, 8, 10]
let evens   = nums.filter({ x in x % 2 == 0 })     // [2, 4]
let sum     = nums.reduce(0, { acc, x in acc + x }) // 15
```

HOFs go through the `__syx_array_map` / `__syx_array_filter` / `__syx_array_reduce` Host functions (see [Chapter 32](32-operators-vs-host.md)); the closure parameter is the element `T` of `Array<Element>`. Semantically, `filter` yields `Array<T>`; `map` / `flatMap` yield `Array<U>`; `reduce` yields `U` (`U` comes from `initial`). Assigning an `Int→String` `map` to `Array<Int>`, or `reduce(0, …)` to `String` → **S004**. The builtin facade still declares `Function` / `Array` / `reduce(…)->Int`, not full monomorphization.

> **Three-back-end parity:** `map/filter/reduce` behave identically on the VM / unified MIR / WASM back ends. Verified by the test `fixtures/parity/hof.sy`.

### Array Subscripts

```syx
var arr = [10, 20, 30]
print(arr[0])                  // 10
arr[1] = 99                    // ✅
print(arr)                     // [10, 99, 30]
```

Subscripts must be `Int`. Out-of-bounds reads or writes (including negative indices) → runtime error **R003** (the message contains `out of bounds`); this is **not** a `nil` return and it is **not** a silently ignored write.

## Set

A deduplicating collection (based on `value_equals`, preserving insertion order):

```syx
let s: Set<Int> = [1, 2, 3, 2]   // Automatically deduplicated: {1, 2, 3}
print(s.count)                   // 3
print(s.contains(2))              // true
```

| Method | Description |
|------|------|
| `.count` | Number of elements (test emptiness with `.count == 0`; there is no `.isEmpty`) |
| `.contains(item)` | Membership test (**requires** elements conform to `Equatable`) |
| `.insert(item)` | Insert (ignored if already present) |
| `.remove(item)` | Whether removal succeeded (`Bool`; ≠ Swift's Optional) |
| `.union(other)` | Union |
| `.intersection(other)` | Intersection |

**Not implemented** (calling them → **S002**): `.isEmpty`, `.subtraction`, `.forEach`. Use `for` to iterate.

> **Storage back end:** at runtime this is `Value::Set(Vec<Value>)`, deduplicated linearly by `value_equals` and preserving insertion order; `.insert` / `.contains` / `.remove` / `.union` / `.intersection` all share the same predicate. This is **≠** a hash table and **≠** a user `hash(into:)`. The product facade lives in `packages/core/builtins/collections/set.sy` (instance `contains`/`insert`/`remove` and the static forms all go through the `__syx_set_*` Host).

## Dictionary

Both types and literals use **square brackets plus a colon**, distinguishing them from arrays `[T]` / `[elem, …]`:

```syx
let scores: [String: Int] = ["a": 1, "b": 2]
let byId: Dictionary<Int, Int> = {2: 4, 3: 6}
print(scores["a"])               // 1 (Optional<Int>)
print(byId[2])                   // 4 (Optional<Int>)
```

> **Key distinction:** when the parser sees **`[key: value, …]`** (first element followed by `:`), it is a Dictionary; **`[elem, …]`** (first element followed by `,` or `]`) is an Array. The two syntaxes do not conflict.

> **Storage back end (String-shell):** at runtime `Value::Dict` is still a `HashMap<String, Value>` (isomorphic to `Object`). `String` keys slot in directly; non-`String` keys (`Int`, classes, …) go into key/value slots encoded internally with `\x1fsyx:`. To user code, `.keys()` / `.count` expose **logical keys / entry counts** (the decoded `Int` etc.), without leaking the encoded string. When hashes collide but keys differ, entries coexist in a bucket chain (equal keys replace values). This is **≠** the completed true Value-key shell of `HashMap<Value, Value>`. The product facade lives in `packages/core/builtins/collections/dictionary.sy` (instance `containsKey`/`keys`/`values`/`remove` and the static forms all go through the `__syx_dict_*` Host; facade signatures still use bare `Array` / `Object` keys, while sema returns `Array<K>`/`Array<V>`; assignment pin: `dict_keys_values_type`).

### Common Methods

Product surface (aligned with the Core facade / dict Host; there is **no** `mapValues` / `merge` / `forEach` / `isEmpty`):

| Method | Signature | Description |
|------|------|------|
| `.count` | `Int` | Number of key-value pairs |
| `.containsKey(k)` | `Bool` | Whether the key is present |
| `.keys()` | `Array<K>` | Array of keys |
| `.values()` | `Array<V>` | Array of values |
| `.remove(k)` | `Bool` | Whether removal succeeded (≠ Swift's `V?`) |
| `d[k]` / `d[k] = v` | Read/write | Subscript (see above) |

> **Iteration and transformation:** use `for k in d.keys()` with subscripts to read values; value-side HOFs (`mapValues` and friends) are **not yet** provided — calling an unknown member → **S002**.

### Key Type Constraints

The key type `K` **must** conform to `Hashable`:

```syx
let k1: [String: Int] = ["x": 1]            // ✅ String: Hashable
let k2: [Int: String] = [1: "a", 2: "b"]    // ✅ Int: Hashable
// let k3: [Array<Int>: Int] = [[1]: 1]      // ❌ S004: Array<Int> is not Hashable
```

> **Hashable and key semantics:**
> - `data class` + `@Hashable`: the hash is synthesized field-wise, so instances can be used as Dict/Set keys.
> - A plain `class: Hashable` used as a key: **by instance identity** (the same reference is the same key; equal fields ≠ the same key).
> - User-written `func hash(into:)` / `==` do **not** drive Dict/Set lookup; an explicit `x.hash(into: h)` only updates a `Hasher` and has nothing to do with collections.

### Dictionary Literal Keys

```syx
let lit: [String: Int] = ["a": 1, "b": 2]    // String keys
let byNum: [Int: String] = [1: "x", 2: "y"]  // Integer keys
let tuple: [String: Array<Int>] = ["xs": [1,2,3]]
```

Keys may be arbitrary expressions (literals, identifiers, function return values, …).

## Object (Dynamic Objects)

An unannotated dictionary literal is inferred as a **`Dictionary`** (values must be homogeneous). A heterogeneous JSON bag must be **explicitly** annotated as `Object` (use it sparingly; prefer concrete types):

```syx
let scores = ["a": 1, "b": 2]                 // Dictionary (homogeneous)
let obj: Object = { "name": "SYX", "version": 110, "stable": true }
print(obj["name"])        // "SYX"
print(obj["version"])     // 110
print(obj["stable"])      // true
```

| Form | Syntax | Type |
|------|------|------|
| Dictionary | `["a": 1]` or `[String: Int] = …` | Homogeneous key-values (inferred by default) |
| Object | `let x: Object = { "k": v }` | Dynamic heterogeneous bag (must be annotated) |

| Method | Signature | Description |
|------|------|------|
| `obj[key]` | `Any?` | Read (Optional; same semantics as `valueForKey`) |
| `obj[key] = v` | `Void` | Write (same semantics as `put`) |
| `.valueForKey(key)` | `Object` | Explicit key read (`object.sy` facade) |
| `.put(key, value)` | `Object` | Explicit key write (returns the bag itself) |
| `.count` | `Int` | Number of keys |
| `.isEmpty` | `Bool` | Emptiness |
| `.containsKey(key)` | `Bool` | Whether the key is present |
| `.keys()` | `Array<String>` | Array of keys |
| `.values()` | `Array<Object>` | Array of values (type-erased) |
| `.remove(key)` | `Bool` | Whether removal succeeded (≠ `V?`) |
| `.description` | `String` | Description string |

> **todo 4 soft-close (W1253 / ADR-1599):** default literal → Dictionary; explicit `: Object` heterogeneous bag; the method surface above is **met**; pinned by `todo4_object_soft_close_first_slice`; **item 4 opens no further slices**. **OUT:** `.merge`; a true `HashMap<Value,Value>`; deeper Object generics; `object_bag_ga`.

> **No `.merge`:** neither Object nor Dictionary yet provides a merge API; copy through subscripts by hand (OUT / will not be revisited as item 4).

> **Type erasure:** `Object` carries no field types; a value read out must be converted with `as?` (see [Chapter 20](20-type-casting.md)). `.keys()` goes through `__syx_dict_keys`, with `Array<String>` in both facade and sema; `.values()` goes through `__syx_dict_values`, with `Array<Object>` in both facade and sema (≠ bare `Array` / ≠ `Array<Int>`).

```syx
let obj: Object = { "name": "SYX", "version": 110, "stable": true }
let v = obj["version"]
if let n = v as? Int {
    print("version is \(n)")
}
```

## Custom Sequence / Iterator

Since Phase 71, a user-defined `class` implementing `next()` can be traversed by `for-in` (strict three-end parity):

```syx
protocol Iterator {
    func next() -> String?
}

class Counter: Iterator {
    var current: Int
    let end: Int
    init(end: Int) { self.current = 0; self.end = end }

    func next() -> String? {
        if (current >= end) { return nil }
        let s = "\(current)"
        current = current + 1
        return s
    }
}

func main() {
    let c = Counter(end: 3)
    for s in c {
        print(s)          // 0, 1, 2
    }
}
```

> **Protocol requirement:** a custom `Iterator` must have `func next() -> String?` (consistent with stdlib); returning `nil` means the end. See [Chapter 23, Protocols](23-protocols.md).

## for-in Iteration

`for x in iterable { … }` supports:

| iterable | Element type | Description |
|----------|----------|------|
| `Array<T>` | `T` | Ordered traversal |
| `String` | `String` (single scalar) | Grapheme clusters |
| `Set<T>` | `T` | Unordered |
| `Dictionary<K,V>` | `(K, V)` tuple | |
| `0..<n` | `Int` | Half-open range |
| Custom `Iterator` | `String` | Phase 71+ |
| `Object` | `(String, Any)` tuple | Key-value pairs |
| `Array<String>` (`File.readLines()`) | `String` | Lines |

```syx
import IO

for item in [1, 2, 3] { print(item) }       // 1 2 3
for c in "abc" { print(c) }                  // a b c
for k in ["x": 1, "y": 2].keys() { print(k) }  // x y
for i in 0..<3 { print(i) }                  // 0 1 2
for line in File.readLines("a.txt") { print(line) }
```

## Binary IO (Phase 42 / 60)

After `import IO`, whole files can be read and written as `Array` bytes (elements are `UInt8`):

```syx
import IO

let path = "bytes.bin"
File.writeBytes(path, [0u8, 255u8, 42u8])
let bytes: Array<UInt8> = File.readBytes(path)

let h = FileHandle.open(path, "r")
let _ = FileHandle.read(h, 4096)
FileHandle.close(h)
```

- **Strong typing:** since Phase 60 `Array<UInt8>` is **required**; a bare `Array` reports **S004**.
- For large files, stream with `FileHandle`; `readBytes` loads the whole file into memory.

Tests: `stdlib/leak_file_stream.rs` · `stdlib/file_stream_watcher.rs`

## Selection Guide

| Need | Choose |
|------|-----|
| Ordered, duplicable, indexable | `Array<T>` |
| Deduplication, set operations | `Set<T>` |
| Static key-value mapping, needs compile-time types | `Dictionary<K, V>` |
| Dynamic JSON style, fields not fixed | `Object` (must be annotated; prefer `Dictionary` by default) |
| Byte stream | `Array<UInt8>` |
| Arbitrary iteration, custom `next()` | `class: Iterator` |

## Performance and Memory

- `Array<T>` is a value-wrapped `Vec<T>` reference; `Array<UInt8>` / `Array<String>` go through `ArrayBuild` (VM HeapSlot, Phase 76)
- `Dictionary<K, V>` has hash semantics on the product surface; the runtime is still a String-shell (`HashMap<String, Value>` + encoded slots for non-String keys); literals are materialized through the `materialize_dict` Host (Phase 77)
- `Set<T>` is implemented in `packages/core/builtins/collections/set.sy`
- When a collection element is a class, `arr.append(c)` shares the reference; it is **not** a deep copy

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S004** | Element types inconsistent between `Array` and `Dictionary` | Annotate explicitly or fix the literal |
| **S010** | Dictionary key type does not satisfy `Hashable` | Use String/Int or implement `Hashable` |
| **R003** | `arr[i]` out-of-bounds read/write (including negative indices) | Check the index; ≠ `nil` / silent |
| **S024** | Collection hosted ops forbid legacy `MethodCall` (Phase 48) | Use an `extension` / `CallHost` |

## Differences from Swift

| Point | SYX |
|----|-----|
| `Array` notation | `[T]` and `Array<T>` are equivalent |
| Bare `Array` | **Removed** (Phase 60); type arguments are required |
| Dictionary type | `[String: Int]` or `Dictionary<String, Int>` |
| Dictionary literal | **`["a": 1]`** (square brackets); Swift uses only `{ }` |
| Object | **`let o: Object = { "k": v }`** (must be annotated; unannotated → `Dictionary`) |
| Dictionary keys | **Must** satisfy **`Hashable`** (`Int`, `String`, …) |
| `Set` literal | `[1, 2, 3]` explicitly annotated as `Set<Int>` |
| `flatMap` | Implemented (flattens `Array<Array<T>>`) |
| `Range` (half-open) | `0..<n` (**Implemented**); the closed range `0...n` is partially implemented |
| `KeyPath` arrays | `\Array.[0]` (Phase 48); see [Chapter 13](13-subscripts.md) |

---

**Next chapter** → [Control Flow](06-control-flow.md)

**See also** → [Language Reference: Types](../language-reference/02-types.md) · [Chapter 13, Subscripts](13-subscripts.md) · [Chapter 23, Protocols](23-protocols.md) · [Implementation Status Master Table](../shared/implementation-status.md)
