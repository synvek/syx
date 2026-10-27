# Math Float64 实例方法（W912–W917 / ADR-1256–1261）

续 [math-f32.md](math-f32.md) · [math-f32-methods.md](math-f32-methods.md)。Rust 风格实例方法与 `Math.*` 并存。

## Math 进度（诚实）

| 轨 | 状态 |
| --- | --- |
| Float64 static (`Math.*`) | Phase-1/2 + Soft + special 齐 |
| Float32 static | 六片 **42** Soft 对称齐 |
| Float64 实例方法 | 六片共 **42**（W912–W917）；Phase-1/Soft/special/classify 齐 |
| Float32 实例方法 | 六片共 **42**（W917–W922）；与 F64 实例 **parity** |

## 已交付

### 第一～五片（W912–W916 / ADR-1256–1260）

见历史切片；累计至 Soft 收口 + special + `isNaN`（**40**）。

### 第六片（W917 / ADR-1261）

- classify 收口：**`isInfinite` `isFinite`** → Bool
- **HostFn 维持 629**；`math_f64_methods`（classify；F32 首片同窗证据见 `math_f32_methods`）

## 非本窗

- toDegrees/toRadians/fclamp 等工具实例（仍仅 Math static）
- F-07 / F-08 / CST Soft / llvm Math GA / QS1 Math

## 维护

W953 Math residual maintenance **已交**（ADR-1297）→ W954+ 维护；详见 [`math-residual-maintenance-first-slice.md`](math-residual-maintenance-first-slice.md)。

## 证据

- `packages/core/builtins/numeric/float64.sy`
- `crates/syx-driver/tests/stdlib/math_f64_methods.rs`
- Matrix tip 以活矩阵为准（F32 实例 parity 见 [math-f32-methods.md](math-f32-methods.md)；增强轨收官→维护签 W953）
