# Math Float32（W906–W911 / ADR-1250–1255）

续 [math-special.md](math-special.md)。Float32 薄包装：C/libm 风格 `*f` 后缀（SYX 禁止同名 overload）。

## Math 进度（诚实）

| 轨 | 状态 |
| --- | --- |
| Float64 | Phase-1/2 + Soft + special 齐；实例方法见 [math-f64-methods.md](math-f64-methods.md) |
| Float32 | 六片共 **42** API（W906–W911）；Soft 对称收官；实例方法见 [math-f32-methods.md](math-f32-methods.md)（W917–W922 共×42；与 F64 parity） |

## 已交付

### 第一～五片（W906–W910 / ADR-1250–1254）

- 薄包装 / Soft 对称 / special / classify / remainder / nextUp → HostFn 至 **627**；证据见 `math.rs`

### Soft 收官片（W911 / ADR-1255）

- **`nextDownf` `mulAddf`**
- Host：`f32::next_down` / `f32::mul_add`；栈序同 f64 Soft
- **HostFn +2 → 629**；capability `vm/mir/wasm=real`（**W959–W965 / ADR-1303–1309：** MathFloat32* **84/84** `llvm=real` + narrow GA exit；见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)；≠ QS1 / ≠ `status=implemented`）
- `math.rs`

## 非本窗

- Float32 实例方法 → **已交** 见 [math-f32-methods.md](math-f32-methods.md)（W917–W922 ×42；与 F64 parity；增强轨收官→维护）
- F-07 / F-08 / CST Soft / llvm Math GA / QS1 Math 行 / 同名 overload

## 证据

- `crates/syx-driver/tests/stdlib/math.rs`
- Matrix tip 以活矩阵为准（W912+ 见 [math-f64-methods.md](math-f64-methods.md)）
