# SIMD

**Implementation status:** `SIMD2<T>` / `SIMD4<T>` / `SIMD8<T>` / `SIMD16<T>` language surface **GA**; WASM guest `SIMD128` direct emit **GA**; Native LLVM SIMD16 enhancements (Phase 133–138): i8x16 add/sub/neg/abs/umin/umax/load/store/shuffle/sat/compare/extract-replace/avgr_u/popcnt/ge/le/ne/bitmask **MVP partial GA**

SYX provides **fixed-width SIMD value types** `SIMD2<T>` / `SIMD4<T>` / `SIMD8<T>` / `SIMD16<T>`, where the element `T` is a fixed-width numeric (`Int8/16/32/64` / `UInt8/16/32/64` / `Float32/64`). This chapter covers the language surface, reductions, configuration, cross-back-end implementation, and the WASM acceleration path.

## Types and Literals

```syx
let v4: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let v8: SIMD8<Float32> = SIMD8.fromArray([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
let v16: SIMD16<UInt8> = SIMD16.fromArray([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
                                          8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8])
```

| Form | Description |
|------|------|
| `SIMD2<T>` | 2 elements |
| `SIMD4<T>` | 4 elements (most common; x86 SSE / ARM NEON) |
| `SIMD8<T>` | 8 elements (AVX2 / NEON 2×) |
| `SIMD16<T>` | 16 elements (AVX-512 / WASM SIMD128) |

| Literal construction | Description |
|------------|------|
| `SIMD4.fromArray([…])` | Construct from `Array<T>` (the length must match) |
| `SIMD4.zero` | All zeros |
| `SIMD4.splat(x)` | Every lane = x |
| `SIMD4(value0, value1, value2, value3)` | Direct construction (**Partially implemented**) |

> **Strong typing:** `T` must be a fixed-width numeric; platform-word `Int` is not allowed.

## Element-Wise Operations (per lane)

Supports `+` / `-` / `*` / `/` / `%` / `==` / `<` / `>` / `&` / `|` / `^` and more:

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let w: SIMD4<Int32> = SIMD4.fromArray([10i32, 20i32, 30i32, 40i32])

let s = v + w                               // [11, 22, 33, 44]
let q = v / w                               // [0, 0, 0, 0]
let eq = v == w                             // [false, false, false, false]
let lt = v < w                              // [true, true, true, true]
```

## Reductions (reduce)

Reducing the whole SIMD to a single value:

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])

let sum = SIMD4.reduceAdd(v)                // 10
let min = SIMD4.reduceMin(v)                // 1
let max = SIMD4.reduceMax(v)                // 4
let any = SIMD4.reduceAny(v < SIMD4.fromArray([2i32, 2i32, 2i32, 2i32]))   // true
```

| Reduction | Meaning |
|------|------|
| `reduceAdd` | Sum |
| `reduceMin` | Minimum |
| `reduceMax` | Maximum |
| `reduceAny` | Any lane is `true` |
| `reduceAll` | All lanes are `true` |
| `reduceBitAnd` / `reduceBitOr` / `reduceBitXor` | Bit reduction |

## Single-Lane Access

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let first = v[0]                            // 1
let last = v[3]                             // 4
```

## Cross-Width

```syx
let v4: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let v8: SIMD8<Int32> = v4.extended(to: SIMD8<Int32>.zero)  // [1, 2, 3, 4, 0, 0, 0, 0]
let lo: SIMD4<Int32> = v8.lowHalf()          // [1, 2, 3, 4]
let hi: SIMD4<Int32> = v8.highHalf()         // [0, 0, 0, 0]
```

> **Status:** `extended` / `lowHalf` / `highHalf` are **Partially implemented**; aligned with WASM `i32x4.extend_*`.

## SIMD16 Enhancements (Phase 133–138)

Since WASM guest-v16, SIMD16 i8x16 operations have been added progressively:

| Operation | Phase | Fingerprint |
|------|-------|------|
| add / sub | inherited | — |
| `mul` (i16x8) | inherited | — |
| `neg` | **131** | `lang-simd16-v1:neg-mvp` |
| `abs` | **131** | `lang-simd16-v2:abs-mvp` |
| `umin` / `umax` | **130** | `lang-simd16-v3:umin-umax-mvp` |
| `smin` / `smax` | inherited | — |
| `load` / `store` | 129 | `lang-simd16-v4:load-store-mvp` |
| `shuffle` | 130 | `lang-simd16-v5:shuffle-mvp` |
| `sat` (saturating add) | 131 | `lang-simd16-v6:sat-mvp` |
| `compare` (eq / ne / ge / le) | 132 | `lang-simd16-v7:compare-mvp` |
| `extract` / `replace` (lane) | 132 | `lang-simd16-v8:extract-replace-mvp` |
| `avgr_u` | **136** | `wasm-simd16-guest-v18` |
| `popcnt` | **137** | `wasm-simd16-guest-v19` |
| `bitmask` | **138** | `wasm-simd16-guest-v21` |

> **Full-set status:** SIMD16 is **partially GA**; full instruction-set coverage → **139+**.

## Configuration

Disabling SIMD from the CLI:

```bash
syx run --no-simd program.sy               # Compile-time S022
```

`syx.toml` configuration:

```toml
[features]
simd = false                                # Disable SIMD
```

```syx
import Runtime

Runtime.enableSimd(true)                    // Switch inside the program
print(Runtime.simdEnabled())
```

## Back-End Support

| Back end | SIMD status |
|------|----------|
| **VM interpreter** | Native lane arithmetic (each lane goes through VM instructions) |
| **Native (LLVM)** | MIR `SIMD_*` opcode → LLVM vector intrinsics |
| **WASM (guest-v16)** | Direct emit of `i8x16_*` / `i32x4_*` / `f32x4_*` SIMD128 instructions |
| **Cranelift** | **Removed** (Phase 77 GA, LLVM sole backend) |

> **WASM path:** SYX WASM goes through **direct MIR compilation** (`syx-wasm-emit`); SIMD instructions are encoded directly into the `.wasm` bytecode, **not** through a host `syx.mir_exec` spill.

## Performance Comparison

| Operation | Plain scalar loop | SIMD4 (Float32) | Speedup |
|------|--------------|-----------------|--------|
| `+` 1024 elements | ~50 µs | ~12 µs | ~4× |
| `*` 1024 elements | ~50 µs | ~12 µs | ~4× |
| reduceAdd | ~30 µs | ~5 µs | ~6× |
| Cross-platform WASM | same as above | native SIMD128 | ~3-4× |

> **Measured:** the `p_simd.sy` test (`crates/syx-driver/tests/`).

## Complete Example

```syx
func sumArray(_ arr: Array<Int32>) -> Int32 {
    var total: Int32 = 0
    for v in arr { total = total + v }
    return total
}

func sumArraySimd(_ arr: Array<Int32>) -> Int32 {
    var i = 0
    var acc: SIMD4<Int32> = SIMD4.zero
    while (i + 4 <= arr.count) {
        let v = SIMD4.fromArray([arr[i], arr[i+1], arr[i+2], arr[i+3]])
        acc = acc + v
        i = i + 4
    }
    var total = SIMD4.reduceAdd(acc)
    while (i < arr.count) {                  // Handle the remainder
        total = total + arr[i]
        i = i + 1
    }
    return total
}

func main() {
    let data: Array<Int32> = Array(repeating: 1, count: 1024) + Array(repeating: 2, count: 1024)
    print("scalar=\(sumArray(data))")         // 3072
    print("simd=\(sumArraySimd(data))")       // 3072
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **S022** | Using SIMD while it is disabled | Enable SIMD / switch to scalar |
| **S004** | SIMD element type mismatch | Check that `T` is consistent |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| `SIMD2` / `SIMD4` / `SIMD8` / `SIMD16` | **Implemented** |
| `SIMD3` | **Not implemented** |
| `SIMD64` | **Not implemented** |
| WASM SIMD128 | **Implemented** (guest-v16) |
| LLVM vector intrinsics | **Implemented** |
| Cranelift SIMD | **Removed** (Phase 77) |
| Full SIMD16 instruction set | **Partially implemented** (→ 139+) |

## Further Reading

- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-035 (relocating GC) and others
- [Chapter 26, Memory Management](26-memory-management.md) — SIMD memory layout on the Native back end
- [Chapter 32, Operators vs Core Host](32-operators-vs-host.md) — SIMD and the Core bridge

---

**Previous chapter** ← [Advanced Operators](30-advanced-operators.md)

**See also** → [Implementation Status Master Table](../shared/implementation-status.md) · [docs/10_BackendParity.md](../../docs/10_BackendParity.md)
