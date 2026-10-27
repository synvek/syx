# Math Float32 实例方法（W917–W922 / ADR-1261–1266）

续 [math-f32.md](math-f32.md)。Rust 风格一元/双参/三参实例方法：`x.sin()` / `base.pow(exp)` / `a.mulAdd(b,c)`，与 `Math.*f` 并存（无同名 overload）。

## Math 进度（诚实）

| 轨 | 状态 |
| --- | --- |
| Float32 static (`Math.*f`) | 六片 **42** Soft 对称齐 |
| Float32 实例方法 | 六片共 **42**（W917–W922）；与 F64 实例 **parity**；仍不宣称 llvm Math GA / QS1 Math / F-07 |

## 已交付

### 第一片（W917 / ADR-1261）

- **`sqrt` `sin` `cos` `tan` `exp` `log`**
- Host：`__syx_f32_*` / CallHost 经 `lookup_float32_math_host`
- **HostFn 维持 629**；`math_f32_methods`

### 第二片（W918 / ADR-1262）

- **`floor` `ceil` `abs` `trunc` `round` `asin` `acos` `atan`**
- `abs` ≡ `Math.fabsf`
- **HostFn 维持 629**；`math_f32_methods`

### 第三片（W919 / ADR-1263）

- Phase-1 收口：**`log2` `log10` `pow` `atan2` `hypot`**
- Soft 双曲首片：**`sinh` `cosh` `tanh`**
- 栈序同 F64：`base.pow(exp)` / `y.atan2(x)` / `x.hypot(y)`
- **HostFn 维持 629**；`math_f32_methods`

### 第四片（W920 / ADR-1264）

- Soft 余项：**`expm1` `log1p` `asinh` `acosh` `atanh` `remainder` `nextUp` `nextDown`**
- 栈序同 F64：`x.remainder(y)`
- **HostFn 维持 629**；`math_f32_methods`

### 第五片（W921 / ADR-1265）

- Soft 收口：**`fmod` `copysign` `mulAdd`**
- special 首片：**`erf` `erfc` `tgamma` `lgamma`**
- classify 起手：**`isNaN`** → Bool
- 栈序同 F64：`x.fmod(y)` / `mag.copysign(sign)` / `a.mulAdd(b,c)`
- **HostFn 维持 629**；`math_f32_methods`

### 第六片（W922 / ADR-1266）

- Phase 余项：**`exp2` `cbrt`**
- classify 收口：**`isInfinite` `isFinite`** → Bool
- **HostFn 维持 629**；`math_f32_methods`
- **F32 实例×42 与 F64 parity**

## 非本窗

- F-07 / F-08 / CST Soft / llvm Math GA / QS1 Math

## 维护

W953 Math residual maintenance **已交**（ADR-1297）→ W954+ 维护；详见 [`math-residual-maintenance-first-slice.md`](math-residual-maintenance-first-slice.md)。

## 证据

- `packages/core/builtins/numeric/float32.sy`
- `crates/syx-driver/tests/stdlib/math_f32_methods.rs`
- Matrix tip 以活矩阵为准（增强轨收官→维护签 W953；parity 于 W922）
