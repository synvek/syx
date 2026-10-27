# Operators vs Core Host

**Implementation status:** Operator Intrinsic (language-surface hard-coded + LLVM / VM direct emit) **Implemented**; numeric static Host (`__syx_i8_min` and other per-type hosts + manifest `host_ops`) **Implemented** (Phase 45); collection HOF Host (`__syx_array_map` and others) **Implemented** (Phase 46); `BigInt` MVP **Implemented** (Phase 46); `Result` generic algebra (final class) **Implemented** (Phase 46+)

SYX splits **builtin capability** into two layers: user syntax mostly stays the same, but the **implementation path** differs.

## The Two-Layer Model

| Layer | Meaning | Example |
|----|------|------|
| **Operator Intrinsic** | Compiler hard-coded + LLVM / VM direct emit | `a + b` / `a == b` / `a += 1` |
| **Host Explicit** | Core `.sy` declaration → `__syx_{surface}_{op}` Host function | `Int8.abs(x)` / `Math.min(a, b)` / `arr.map(f)` |

> **Operators** (`+` / `==` / `<` / `&` / `|` and friends) **always** go through the Intrinsic layer; **non-operator builtins** (`Math.min` / `.abs` / `.description` / `.hash` / `Array.map`) go through the Host layer. Integer **`Type.min` / `Type.max`** are type bounds (`static let` / static computed `get`; not binary comparisons, and not written with `()`).

## Operator Intrinsic (Language-Surface Hard-Coded)

```syx
let s = a + b
x += 1
if (a == b) { … }
let lt = a < b
```

These expressions lower to `NumericAdd` / `NumericEq` / `NumericLt` and other MIR opcodes, compiled by codegen / LLVM into the corresponding native instructions (addsd / cmpeq, …). They do **not** go through a Core extension; they do **not** go through a Host call.

| Operator | MIR operation | Meaning |
|--------|---------|------|
| `+` | `NumericAdd` | Add |
| `-` | `NumericSub` | Subtract |
| `*` | `NumericMul` | Multiply |
| `/` | `NumericDiv` | Divide |
| `%` | `NumericMod` | Modulo |
| `==` | `ValueEq` / `NumericEq` | Equal |
| `!=` | `ValueNe` | Not equal |
| `<` | `NumericLt` | Less than |
| `<=` | `NumericLe` | Less than or equal |
| `>` | `NumericGt` | Greater than |
| `>=` | `NumericGe` | Greater than or equal |
| `&` | `BitAnd` | Bitwise AND |
| `\|` | `BitOr` | Bitwise OR |
| `^` | `BitXor` | Bitwise XOR |
| `<<` | `Shl` | Left shift |
| `>>` | `Shr` | Right shift |
| `&&` | `BoolAnd` | Logical AND (short-circuit) |
| `\|\|` | `BoolOr` | Logical OR (short-circuit) |
| `!` | `BoolNot` | Logical NOT |
| `??` | `NilCoalesce` | Nil coalescing |
| `?.` | `OptionalChain` | Optional chaining |

> **Three-back-end parity:** all Intrinsics behave identically on the VM / unified MIR / WASM back ends; Cranelift has been removed (Phase 77).

## Host Explicit (Core + Host Functions)

Non-operator builtin capabilities are declared readably in Core and call per-type named Host functions `__syx_{surface}_{op}`:

```syx
// Core (packages/core/builtins/numeric/int8.sy, illustrative)
public static func min() -> Int8 { return -128i8 }   // Type lower bound
public static func max() -> Int8 { return 127i8 }    // Type upper bound
public static func abs(x: Int8) -> Int8 {
    return __syx_i8_abs(x)
}
public static func clamp(x: Int8, lo: Int8, hi: Int8) -> Int8 {
    return __syx_i8_min(__syx_i8_max(x, lo), hi)
}

extension Int8 {
    func hash(into hasher: Hasher) {
        __syx_hasher_combine(hasher, self)
    }
    func description() -> String {
        return __syx_value_description(self)
    }
    func debugDescription() -> String {
        return __syx_value_debug_description(self)
    }
}
```

```syx
// User-side usage
let lo = Int8.min                                // Type lower bound
let hi = Int8.max
let m = Math.min(3, 5)                           // Binary comparison → Host
let a = Int8.abs(x: -3i8)                        // Goes through Host
print(x.description)                             // Goes through extension → Host
x.hash(into: h)
```

## Naming Contract (ADR-066 / ADR-067)

| User API | Core / Host |
|----------|------------|
| `Int8.min` / `Int8.max` | Type bounds (`static let` / computed get; not Host) |
| `Math.min(a, b)` / `Math.max(a, b)` | Binary comparison convenience layer → `__syx_int_min` / `__syx_int_max` |
| `Int8.clamp` / `Math.clamp` | → `__syx_*_min` / `__syx_*_max` |
| `Math.gcd` / `lcm` / `clamp` / `sign` | Pure SYX (W902 Math Phase-1) |
| `Math.pi` / `e` / `tau` / … | Constant static getters (literals; W902) |
| `Math.fabs` / `fmin` / `fmax` / `fclamp` | Float64 convenience → `__syx_f64_*` |
| `Math.sinh` / `cosh` / `tanh` / `fmod` / `copysign` / `expm1` / `log1p` | Phase-2 → `__syx_f64_*` (W903) |
| `Math.isNaN` / `isInfinite` / `isFinite` | Phase-2 Bool classification → `__syx_f64_isnan|isinf|isfinite` |
| `Math.asinh` / `acosh` / `atanh` / `remainder` / `nextUp` / `nextDown` / `mulAdd` | Soft → `__syx_f64_*` (W904; remainder ≠ fmod) |
| `Math.erf` / `erfc` / `tgamma` / `lgamma` | Soft special → `__syx_f64_*` via libm (W905) |
| `Math.sqrtf` / `sinf` / `cosf` / `tanf` / `expf` / `logf` / `fabsf` / `floorf` | F32 → `__syx_f32_*` (W906; *f suffix) |
| `Math.ceilf` / `roundf` / `truncf` / `powf` / `asinf` / `acosf` / `atanf` / `hypotf` | F32 slice-2 → `__syx_f32_*` (W907) |
| `Math.atan2f` / `log2f` / `log10f` / `exp2f` / `cbrtf` / `sinhf` / `coshf` / `tanhf` | F32 slice-3 → `__syx_f32_*` (W908) |
| `Math.fmodf` / `copysignf` / `expm1f` / `log1pf` / `asinhf` / `acoshf` / `atanhf` / `erff` | F32 slice-4 → `__syx_f32_*` (W909; erff via libm) |
| `Math.erfcf` / `tgammaf` / `lgammaf` / `remainderf` / `isNaNf` / `isInfinitef` / `isFinitef` / `nextUpf` | F32 slice-5 → `__syx_f32_*` (W910; classify→Bool) |
| `Math.nextDownf` / `mulAddf` | F32 Soft closeout → `__syx_f32_*` (W911) |
| `Float64.sqrt/sin/cos/tan/exp/log/floor/ceil` | F64 instance methods first slice → CallHost `MathFloat*` (W912) |
| `Float64.abs/trunc/round/asin/acos/atan/exp2/cbrt` | F64 instance methods second slice → CallHost (W913; abs≡fabs) |
| `Float64.log2/log10/pow/atan2/hypot/sinh/cosh/tanh` | F64 instance methods third slice → CallHost (W914; Phase-1 closeout + Soft hyperbolic first slice) |
| `Float64.expm1/log1p/asinh/acosh/atanh/remainder/nextUp/nextDown` | F64 Soft remainder instance first slice → CallHost (W915) |
| `Float64.fmod/copysign/mulAdd/erf/erfc/tgamma/lgamma/isNaN` | F64 Soft closeout + special first slice + isNaN → CallHost (W916) |
| `Float64.isInfinite/isFinite` | F64 classify closeout → CallHost (W917) |
| `Float32.sqrt/sin/cos/tan/exp/log` | F32 instance methods first slice → CallHost (W917) |
| `Float32.floor/ceil/abs/trunc/round/asin/acos/atan` | F32 instance methods second slice → CallHost (W918; abs≡fabsf) |
| `Float32.log2/log10/pow/atan2/hypot/sinh/cosh/tanh` | F32 instance methods third slice → CallHost (W919; Phase-1 closeout + Soft hyperbolic first slice) |
| `Float32.expm1/log1p/asinh/acosh/atanh/remainder/nextUp/nextDown` | F32 Soft remainder instance first slice → CallHost (W920) |
| `Float32.fmod/copysign/mulAdd/erf/erfc/tgamma/lgamma/isNaN` | F32 Soft closeout + special + isNaN → CallHost (W921) |
| `Float32.exp2/cbrt/isInfinite/isFinite` | F32 instance closeout → CallHost (W922; parity with F64×42) |
| `Int8.abs(x)` | `__syx_i8_abs` |
| `.description` | extension → `__syx_value_description` |
| `.debugDescription` | extension → `__syx_value_debug_description` |
| `hash(into:)` | extension → `__syx_hasher_combine` |
| `Array.map(f)` | extension → `__syx_array_map` |
| `Array.filter(f)` | extension → `__syx_array_filter` |
| `Array.reduce(...)` | extension → `__syx_array_reduce` |
| `BigInt.fromString(s)` | `__syx_bigint_from_string` |
| `BigInt.toString(b)` | `__syx_bigint_to_string` |

> **manifest:** the `host_ops` field of `lib/internal/manifest.json` lists all host functions; the compiler looks them up in the table and links directly during codegen.

## Phase 44 → 46 Evolution

- **Phase 44:** builtin types moved into Core (`packages/core/builtins/**`)
- **Phase 45:** `min` / `max` / `abs` changed from compiler hard-coded to **Core extension + Host** (per-type host + `host_ops` field)
- **Phase 46:** protocol witnesses (`hash` / `description` / `debugDescription`) changed from compiler hard-coded to Core extensions; collection HOFs (`map` / `filter` / `reduce`) became Host-based; `BigInt` MVP; `Result` algebra (final class)

> **Phase 46 removal:** the compiler's `ToDescription` / bare `HasherCombine` fallback; builtin witnesses go **only** through Core extension → Host.

## `__syx_*` vs the User Surface

| Category | User surface | Internal |
|------|--------|------|
| Numeric type methods | `Int8.abs` / `Int8.clamp` | `__syx_i8_abs` / `__syx_i8_min|max` |
| Protocol witnesses | `x.description` | `__syx_value_description` |
| Collection HOFs | `arr.map(f)` | `__syx_array_map` |
| Arbitrary Host functions | `MyHost.foo(x)` | `__syx_myhost_foo` |

> **Application code** should avoid calling `__syx_*` directly (internal APIs may change between versions). Use the convenience layer exposed by `lib/*.sy`.

## Collection HOFs

```syx
extension Array {
    func map(transform: (Element) -> Element) -> Array<Element> {
        return __syx_array_map(self, transform)
    }

    func filter(predicate: (Element) -> Bool) -> Array<Element> {
        return __syx_array_filter(self, predicate)
    }

    func reduce(_ initial: Element, _ combine: (Element, Element) -> Element) -> Element {
        return __syx_array_reduce(self, initial, combine)
    }
}
```

> **Phase 46 status:** `map` / `filter` / `reduce` are fully Host-based on `Array` / `Set` / `Dictionary`; three-back-end parity.

## BigInt MVP

```syx
extension BigInt {
    public static func fromString(_ s: String) -> BigInt {
        return __syx_bigint_from_string(s)
    }

    public func toString() -> String {
        return __syx_bigint_to_string(self)
    }

    public static func add(_ a: BigInt, _ b: BigInt) -> BigInt {
        return __syx_bigint_add(a, b)
    }
}
```

> **Status:** `BigInt` basic arithmetic MVP (add / sub / mul / div / cmp / fromString / toString).

## Result Generics (final class)

```syx
// packages/core/builtins/result.sy
final class Result<T, E> {
    public let value: T?
    public let error: E?

    private init(ok: T) { self.value = ok; self.error = nil }
    private init(err: E) { self.value = nil; self.error = err }

    public static func ok(_ v: T) -> Result<T, E> {
        return Result(ok: v)
    }

    public static func err(_ e: E) -> Result<T, E> {
        return Result(err: e)
    }

    public func isOk() -> Bool { return value != nil }
}
```

> **Status:** `Result<T, E>` is a final class (**Phase 46 GA**) providing `static ok` / `static err` / `isOk` and other basic APIs; extensions (such as `map` / `flatMap`) are left to each application.

## Comparison with Intrinsics

| Need | Path |
|------|------|
| `+` `-` `*` `/` | **Intrinsic** (compiler hard-coded) |
| `==` `<` `>` | **Intrinsic** (`Equatable` / `Comparable` witness) |
| `Int8.min` / `max` (bounds) | **`static let` / computed get** |
| `Math.min` / `Int8.abs` / `clamp` | **Host** (`__syx_int_min` / `__syx_i8_abs` / `__syx_*_min|max`) |
| `.description` | **Host** (`__syx_value_description`) |
| `arr.map(f)` | **Host** (`__syx_array_map`) |
| `BigInt.add` | **Host** (`__syx_bigint_add`) |
| User-defined operators | **Intrinsic path** (`operator func` + precedence declaration) |

## Complete Example

```syx
// Core layer (packages/core/builtins/numeric/int8.sy)
public static let min: Int8 = -128i8
public static func abs(x: Int8) -> Int8 { return __syx_i8_abs(x) }

extension Int8 {
    func description() -> String {
        return __syx_value_description(self)
    }
}

// User code
let a: Int8 = 5i8
let b: Int8 = 3i8
print(Int8.min)                               // -128 (type lower bound)
print(Math.min(Int(a), Int(b)))               // 3 (binary comparison → Host)
print(a.description)                          // "5" (goes through Host)

// Operators are still Intrinsic
let sum = a + b                               // 8 (Intrinsic)
let ok = a < b                                // false (Intrinsic)
```

## Further Reading

- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-066 / ADR-067
- [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md) — static Host registry
- [Chapter 33, Runtime Extensions](33-runtime-extensions.md) — dynamic Host plugins

---

**Next chapter** → [Runtime Extensions (Dynamic Host Plugins)](33-runtime-extensions.md)

**See also** → [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) · [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md) · [Chapter 26, Memory Management](26-memory-management.md)
