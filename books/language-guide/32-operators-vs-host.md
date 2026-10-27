# 运算符 vs Core Host

**实现状态：** Operator Intrinsic（语言面硬编码 + LLVM / VM 直发）**已实现**；数值 static Host（`__syx_i8_min` 等 per-type host + manifest `host_ops`）**已实现**（Phase 45）；集合 HOF Host（`__syx_array_map` 等）**已实现**（Phase 46）；`BigInt` MVP **已实现**（Phase 46）；`Result` 泛型代数（final class）**已实现**（Phase 46+）

SYX 将 **builtin 能力** 分为两层：用户语法在多数情况下不变，但**实现路径**不同。

## 两层模型

| 层 | 含义 | 示例 |
|----|------|------|
| **Operator Intrinsic** | 编译器硬编码 + LLVM / VM 直发 | `a + b` / `a == b` / `a += 1` |
| **Host Explicit** | Core `.sy` 声明 → `__syx_{surface}_{op}` Host 函数 | `Int8.abs(x)` / `Math.min(a, b)` / `arr.map(f)` |

> **运算符**（`+` / `==` / `<` / `&` / `|` 等）**永远** 走 Intrinsic 层；**非运算符 builtin**（`Math.min` / `.abs` / `.description` / `.hash` / `Array.map`）走 Host 层。整型 **`Type.min` / `Type.max`** 是类型上下界（`static let` / static computed `get`；非二元比较，也无写 `()`）。

## Operator Intrinsic（语言面硬编码）

```syx
let s = a + b
x += 1
if (a == b) { … }
let lt = a < b
```

这些表达式 lowering 为 `NumericAdd` / `NumericEq` / `NumericLt` 等 MIR 操作码，由 codegen / LLVM 编译为对应原生指令（addsd / cmpeq 等）。**不** 走 Core extension；**不** 经 Host 调用。

| 运算符 | MIR 操作 | 含义 |
|--------|---------|------|
| `+` | `NumericAdd` | 加 |
| `-` | `NumericSub` | 减 |
| `*` | `NumericMul` | 乘 |
| `/` | `NumericDiv` | 除 |
| `%` | `NumericMod` | 取模 |
| `==` | `ValueEq` / `NumericEq` | 相等 |
| `!=` | `ValueNe` | 不等 |
| `<` | `NumericLt` | 小于 |
| `<=` | `NumericLe` | 小于等于 |
| `>` | `NumericGt` | 大于 |
| `>=` | `NumericGe` | 大于等于 |
| `&` | `BitAnd` | 位与 |
| `\|` | `BitOr` | 位或 |
| `^` | `BitXor` | 位异或 |
| `<<` | `Shl` | 左移 |
| `>>` | `Shr` | 右移 |
| `&&` | `BoolAnd` | 逻辑与（短路） |
| `\|\|` | `BoolOr` | 逻辑或（短路） |
| `!` | `BoolNot` | 逻辑非 |
| `??` | `NilCoalesce` | 空合并 |
| `?.` | `OptionalChain` | 可选链 |

> **三后端 parity：** 所有 Intrinsic 在 VM / unified MIR / WASM 三后端行为一致；Cranelift 已移除（Phase 77）。

## Host Explicit（Core + Host 函数）

非运算符 builtin 能力在 Core 中可读地声明，并调用按类型命名的 Host 函数 `__syx_{surface}_{op}`：

```syx
// Core（packages/core/builtins/numeric/int8.sy，示意）
public static func min() -> Int8 { return -128i8 }   // 类型下界
public static func max() -> Int8 { return 127i8 }    // 类型上界
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
// 用户侧写法
let lo = Int8.min                                // 类型下界
let hi = Int8.max
let m = Math.min(3, 5)                           // 二元比较 → Host
let a = Int8.abs(x: -3i8)                        // 走 Host
print(x.description)                             // 走 extension → Host
x.hash(into: h)
```

## 命名契约（ADR-066 / ADR-067）

| 用户 API | Core / Host |
|----------|------------|
| `Int8.min` / `Int8.max` | 类型上下界（`static let` / computed get；非 Host） |
| `Math.min(a, b)` / `Math.max(a, b)` | 二元比较便利层 → `__syx_int_min` / `__syx_int_max` |
| `Int8.clamp` / `Math.clamp` | → `__syx_*_min` / `__syx_*_max` |
| `Math.gcd` / `lcm` / `clamp` / `sign` | 纯 SYX（W902 Math Phase-1） |
| `Math.pi` / `e` / `tau` / … | 常量 static getter（字面量；W902） |
| `Math.fabs` / `fmin` / `fmax` / `fclamp` | Float64 便利 → `__syx_f64_*` |
| `Math.sinh` / `cosh` / `tanh` / `fmod` / `copysign` / `expm1` / `log1p` | Phase-2 → `__syx_f64_*`（W903） |
| `Math.isNaN` / `isInfinite` / `isFinite` | Phase-2 Bool 分类 → `__syx_f64_isnan|isinf|isfinite` |
| `Math.asinh` / `acosh` / `atanh` / `remainder` / `nextUp` / `nextDown` / `mulAdd` | Soft → `__syx_f64_*`（W904；remainder ≠ fmod） |
| `Math.erf` / `erfc` / `tgamma` / `lgamma` | Soft special → `__syx_f64_*` via libm（W905） |
| `Math.sqrtf` / `sinf` / `cosf` / `tanf` / `expf` / `logf` / `fabsf` / `floorf` | F32 → `__syx_f32_*`（W906；*f 后缀） |
| `Math.ceilf` / `roundf` / `truncf` / `powf` / `asinf` / `acosf` / `atanf` / `hypotf` | F32 slice-2 → `__syx_f32_*`（W907） |
| `Math.atan2f` / `log2f` / `log10f` / `exp2f` / `cbrtf` / `sinhf` / `coshf` / `tanhf` | F32 slice-3 → `__syx_f32_*`（W908） |
| `Math.fmodf` / `copysignf` / `expm1f` / `log1pf` / `asinhf` / `acoshf` / `atanhf` / `erff` | F32 slice-4 → `__syx_f32_*`（W909；erff via libm） |
| `Math.erfcf` / `tgammaf` / `lgammaf` / `remainderf` / `isNaNf` / `isInfinitef` / `isFinitef` / `nextUpf` | F32 slice-5 → `__syx_f32_*`（W910；classify→Bool） |
| `Math.nextDownf` / `mulAddf` | F32 Soft 收官 → `__syx_f32_*`（W911） |
| `Float64.sqrt/sin/cos/tan/exp/log/floor/ceil` | F64 实例方法首片 → CallHost `MathFloat*`（W912） |
| `Float64.abs/trunc/round/asin/acos/atan/exp2/cbrt` | F64 实例方法第二片 → CallHost（W913；abs≡fabs） |
| `Float64.log2/log10/pow/atan2/hypot/sinh/cosh/tanh` | F64 实例方法第三片 → CallHost（W914；Phase-1 收口 + Soft 双曲首片） |
| `Float64.expm1/log1p/asinh/acosh/atanh/remainder/nextUp/nextDown` | F64 Soft 余项实例首片 → CallHost（W915） |
| `Float64.fmod/copysign/mulAdd/erf/erfc/tgamma/lgamma/isNaN` | F64 Soft 收口 + special 首片 + isNaN → CallHost（W916） |
| `Float64.isInfinite/isFinite` | F64 classify 收口 → CallHost（W917） |
| `Float32.sqrt/sin/cos/tan/exp/log` | F32 实例方法首片 → CallHost（W917） |
| `Float32.floor/ceil/abs/trunc/round/asin/acos/atan` | F32 实例方法第二片 → CallHost（W918；abs≡fabsf） |
| `Float32.log2/log10/pow/atan2/hypot/sinh/cosh/tanh` | F32 实例方法第三片 → CallHost（W919；Phase-1 收口 + Soft 双曲首片） |
| `Float32.expm1/log1p/asinh/acosh/atanh/remainder/nextUp/nextDown` | F32 Soft 余项实例首片 → CallHost（W920） |
| `Float32.fmod/copysign/mulAdd/erf/erfc/tgamma/lgamma/isNaN` | F32 Soft 收口+special+isNaN → CallHost（W921） |
| `Float32.exp2/cbrt/isInfinite/isFinite` | F32 实例收官 → CallHost（W922；与 F64×42 parity） |
| `Int8.abs(x)` | `__syx_i8_abs` |
| `.description` | extension → `__syx_value_description` |
| `.debugDescription` | extension → `__syx_value_debug_description` |
| `hash(into:)` | extension → `__syx_hasher_combine` |
| `Array.map(f)` | extension → `__syx_array_map` |
| `Array.filter(f)` | extension → `__syx_array_filter` |
| `Array.reduce(...)` | extension → `__syx_array_reduce` |
| `BigInt.fromString(s)` | `__syx_bigint_from_string` |
| `BigInt.toString(b)` | `__syx_bigint_to_string` |

> **manifest：** `lib/internal/manifest.json` 的 `host_ops` 字段列出所有 host 函数；编译器在 codegen 时查表直接链接。

## Phase 44 → 46 演进

- **Phase 44：** builtin 类型迁入 Core（`packages/core/builtins/**`）
- **Phase 45：** `min` / `max` / `abs` 从编译器硬编码改为 **Core extension + Host**（per-type host + `host_ops` 字段）
- **Phase 46：** 协议 witness（`hash` / `description` / `debugDescription`）从编译器硬编码改为 Core extension；集合 HOF（`map` / `filter` / `reduce`）Host 化；`BigInt` MVP；`Result` 代数（final class）

> **Phase 46 删除：** 编译器 `ToDescription` / bare `HasherCombine` fallback；builtin witness **仅** 走 Core extension → Host。

## `__syx_*` vs 用户面

| 类别 | 用户面 | 内部 |
|------|--------|------|
| 数值类型方法 | `Int8.abs` / `Int8.clamp` | `__syx_i8_abs` / `__syx_i8_min|max` |
| 协议 witness | `x.description` | `__syx_value_description` |
| 集合 HOF | `arr.map(f)` | `__syx_array_map` |
| 任意 Host 函数 | `MyHost.foo(x)` | `__syx_myhost_foo` |

> **应用代码** 应避免直接调用 `__syx_*`（内部 API 可能在版本间变更）。使用 `lib/*.sy` 暴露的便利层。

## 集合 HOF

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

> **Phase 46 状态：** `map` / `filter` / `reduce` 在 `Array` / `Set` / `Dictionary` 上全部 Host 化；三后端 parity。

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

> **状态：** `BigInt` 基础算术 MVP（add / sub / mul / div / cmp / fromString / toString）。

## Result 泛型（final class）

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

> **状态：** `Result<T, E>` 是 final class（**Phase 46 GA**），提供 `static ok` / `static err` / `isOk` 等基础 API；扩展（如 `map` / `flatMap`）见各应用。

## 与 Intrinsic 的对照

| 需求 | 路径 |
|------|------|
| `+` `-` `*` `/` | **Intrinsic**（编译器硬编码） |
| `==` `<` `>` | **Intrinsic**（`Equatable` / `Comparable` witness） |
| `Int8.min` / `max`（上下界） | **`static let` / computed get** |
| `Math.min` / `Int8.abs` / `clamp` | **Host**（`__syx_int_min` / `__syx_i8_abs` / `__syx_*_min|max`） |
| `.description` | **Host**（`__syx_value_description`） |
| `arr.map(f)` | **Host**（`__syx_array_map`） |
| `BigInt.add` | **Host**（`__syx_bigint_add`） |
| 用户自定义运算符 | **Intrinsic 路径**（`operator func` + 优先级声明） |

## 完整示例

```syx
// Core 层（packages/core/builtins/numeric/int8.sy）
public static let min: Int8 = -128i8
public static func abs(x: Int8) -> Int8 { return __syx_i8_abs(x) }

extension Int8 {
    func description() -> String {
        return __syx_value_description(self)
    }
}

// 用户代码
let a: Int8 = 5i8
let b: Int8 = 3i8
print(Int8.min)                               // -128（类型下界）
print(Math.min(Int(a), Int(b)))               // 3（二元比较 → Host）
print(a.description)                          // "5"（走 Host）

// 运算符仍然是 Intrinsic
let sum = a + b                               // 8（Intrinsic）
let ok = a < b                                // false（Intrinsic）
```

## 进一步阅读

- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-066 / ADR-067
- [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md) — 静态 Host 注册表
- [第 33 章 运行时扩展](../../docs/adr/architecture-decisions.md) — 动态 Host 插件

---

**下一章** → [运行时扩展（动态 Host 插件）](../../docs/adr/architecture-decisions.md)

**参见** → [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) · [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md) · [第 26 章 内存管理](26-memory-management.md)
