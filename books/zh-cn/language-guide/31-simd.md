# SIMD

**实现状态：** `SIMD2<T>` / `SIMD4<T>` / `SIMD8<T>` / `SIMD16<T>` 语言面 **GA**；WASM guest `SIMD128` 直发 **GA**；Native LLVM SIMD16 增强（Phase 133–138）：i8x16 add/sub/neg/abs/umin/umax/load/store/shuffle/sat/compare/extract-replace/avgr_u/popcnt/ge/le/ne/bitmask **MVP partial GA**

SYX 提供 **固定宽度 SIMD 值类型** `SIMD2<T>` / `SIMD4<T>` / `SIMD8<T>` / `SIMD16<T>`，元素 `T` 为固定宽度数值（`Int8/16/32/64` / `UInt8/16/32/64` / `Float32/64`）。本章覆盖语言面、归约、配置、跨后端实现与 WASM 加速路径。

## 类型与字面量

```syx
let v4: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let v8: SIMD8<Float32> = SIMD8.fromArray([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
let v16: SIMD16<UInt8> = SIMD16.fromArray([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
                                          8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8])
```

| 形式 | 说明 |
|------|------|
| `SIMD2<T>` | 2 元素 |
| `SIMD4<T>` | 4 元素（最常用；x86 SSE / ARM NEON） |
| `SIMD8<T>` | 8 元素（AVX2 / NEON 2×） |
| `SIMD16<T>` | 16 元素（AVX-512 / WASM SIMD128） |

| 字面量构造 | 说明 |
|------------|------|
| `SIMD4.fromArray([…])` | 从 `Array<T>` 构造（长度必须匹配） |
| `SIMD4.zero` | 全 0 |
| `SIMD4.splat(x)` | 所有 lane = x |
| `SIMD4(value0, value1, value2, value3)` | 直接构造（**部分实现**） |

> **强类型：** `T` 必须是固定宽度数值；不允许 `Int`（平台字长）。

## 元素运算（逐 lane）

支持 `+` / `-` / `*` / `/` / `%` / `==` / `<` / `>` / `&` / `|` / `^` 等：

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let w: SIMD4<Int32> = SIMD4.fromArray([10i32, 20i32, 30i32, 40i32])

let s = v + w                               // [11, 22, 33, 44]
let q = v / w                               // [0, 0, 0, 0]
let eq = v == w                             // [false, false, false, false]
let lt = v < w                              // [true, true, true, true]
```

## 归约（reduce）

把整个 SIMD 归约为单值：

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])

let sum = SIMD4.reduceAdd(v)                // 10
let min = SIMD4.reduceMin(v)                // 1
let max = SIMD4.reduceMax(v)                // 4
let any = SIMD4.reduceAny(v < SIMD4.fromArray([2i32, 2i32, 2i32, 2i32]))   // true
```

| 归约 | 含义 |
|------|------|
| `reduceAdd` | 求和 |
| `reduceMin` | 最小 |
| `reduceMax` | 最大 |
| `reduceAny` | 任一 lane 为 `true` |
| `reduceAll` | 全部 lane 为 `true` |
| `reduceBitAnd` / `reduceBitOr` / `reduceBitXor` | 位归约 |

## 单 lane 访问

```syx
let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let first = v[0]                            // 1
let last = v[3]                             // 4
```

## 跨宽度

```syx
let v4: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32])
let v8: SIMD8<Int32> = v4.extended(to: SIMD8<Int32>.zero)  // [1, 2, 3, 4, 0, 0, 0, 0]
let lo: SIMD4<Int32> = v8.lowHalf()          // [1, 2, 3, 4]
let hi: SIMD4<Int32> = v8.highHalf()         // [0, 0, 0, 0]
```

> **状态：** `extended` / `lowHalf` / `highHalf` **部分实现**；与 WASM `i32x4.extend_*` 对齐。

## SIMD16 增强（Phase 133–138）

WASM guest-v16 起，SIMD16 i8x16 操作逐步加齐：

| 操作 | Phase | 指纹 |
|------|-------|------|
| add / sub | 继承 | — |
| `mul` (i16x8) | 继承 | — |
| `neg` | **131** | `lang-simd16-v1:neg-mvp` |
| `abs` | **131** | `lang-simd16-v2:abs-mvp` |
| `umin` / `umax` | **130** | `lang-simd16-v3:umin-umax-mvp` |
| `smin` / `smax` | 继承 | — |
| `load` / `store` | 129 | `lang-simd16-v4:load-store-mvp` |
| `shuffle` | 130 | `lang-simd16-v5:shuffle-mvp` |
| `sat` (saturating add) | 131 | `lang-simd16-v6:sat-mvp` |
| `compare` (eq / ne / ge / le) | 132 | `lang-simd16-v7:compare-mvp` |
| `extract` / `replace` (lane) | 132 | `lang-simd16-v8:extract-replace-mvp` |
| `avgr_u` | **136** | `wasm-simd16-guest-v18` |
| `popcnt` | **137** | `wasm-simd16-guest-v19` |
| `bitmask` | **138** | `wasm-simd16-guest-v21` |

> **全集状态：** SIMD16 **部分 GA**；全指令集覆盖 → **139+**。

## 配置

CLI 禁用 SIMD：

```bash
syx run --no-simd program.sy               # 编译期 S022
```

`syx.toml` 配置：

```toml
[features]
simd = false                                # 禁用 SIMD
```

```syx
import Runtime

Runtime.enableSimd(true)                    // 程序内切换
print(Runtime.simdEnabled())
```

## 后端支持

| 后端 | SIMD 状态 |
|------|----------|
| **VM 解释器** | 原生 lane 算术（每个 lane 走 VM 指令） |
| **Native (LLVM)** | MIR `SIMD_*` opcode → LLVM vector intrinsics |
| **WASM (guest-v16)** | 直发 `i8x16_*` / `i32x4_*` / `f32x4_*` SIMD128 指令 |
| **Cranelift** | **已移除**（Phase 77 GA LLVM sole backend） |

> **WASM 路径：** SYX WASM 走 **MIR 直编译**（`syx-wasm-emit`）；SIMD 指令直接编码到 `.wasm` 字节码，**不** 走 host `syx.mir_exec` spill。

## 性能对比

| 操作 | 普通标量循环 | SIMD4 (Float32) | 加速比 |
|------|--------------|-----------------|--------|
| `+` 1024 个 | ~50 µs | ~12 µs | ~4× |
| `*` 1024 个 | ~50 µs | ~12 µs | ~4× |
| reduceAdd | ~30 µs | ~5 µs | ~6× |
| 跨平台 WASM | 同上 | native SIMD128 | ~3-4× |

> **实测：** `p_simd.sy` 测试（`crates/syx-driver/tests/`）。

## 完整示例

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
    while (i < arr.count) {                  // 处理剩余
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

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S022** | SIMD 在禁用模式下使用 | 启用 SIMD / 改标量 |
| **S004** | SIMD 元素类型不匹配 | 检查 `T` 一致 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `SIMD2` / `SIMD4` / `SIMD8` / `SIMD16` | **已实现** |
| `SIMD3` | **未实现** |
| `SIMD64` | **未实现** |
| WASM SIMD128 | **已实现**（guest-v16） |
| LLVM vector intrinsics | **已实现** |
| Cranelift SIMD | **已移除**（Phase 77） |
| full SIMD16 指令集 | **部分实现**（→ 139+） |

## 进一步阅读

- [docs/adr/architecture-decisions.md](../../../docs/adr/architecture-decisions.md) — ADR-035（relocating GC）等
- [第 26 章 内存管理](26-memory-management.md) — Native 后端的 SIMD 内存布局
- [第 32 章 运算符 vs Core Host](32-operators-vs-host.md) — SIMD 与 Core 桥

---

**上一章** ← [高级运算符](30-advanced-operators.md)

**参见** → [实现状态总表](../../shared/implementation-status.md) · [docs/10_BackendParity.md](../../../docs/10_BackendParity.md)
