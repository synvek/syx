# Math Soft（W904 / ADR-1248）

续 [math-phase2.md](math-phase2.md)。Phase-2「非本窗」长尾首片：反双曲、IEEE remainder、相邻浮点、FMA。

## 本窗交付

- **反双曲：** `asinh` `acosh` `atanh`
- **IEEE remainder：** `remainder`（`x - y * round(x/y)`；≠ `fmod`）
- **相邻浮点：** `nextUp` `nextDown`（单参方向；比双参 nextafter 更清晰）
- **FMA：** `mulAdd(a, b, c)` → `a*b+c`
- **HostFn +7 → 583**；capability `vm/mir/wasm=real`，`llvm=unsupported`（**W957 / ADR-1301：** Soft×7 `llvm→real`；见 [`math-llvm-soft-first-slice.md`](math-llvm-soft-first-slice.md)；≠ llvm Math GA）

## 非本窗

- erf/erfc/tgamma/lgamma → **已交** 见 [math-special.md](math-special.md)
- Float32 薄包装 → **已交** 见 [math-f32.md](math-f32.md)
- Float64 实例方法 → **已交** 见 [math-f64-methods.md](math-f64-methods.md)
- F-07 / F-08 / CST Soft / llvm Math GA / QS1 Math 行

## 证据

- `crates/syx-driver/tests/stdlib/math.rs`
- Matrix：`A-W904` / `seven-hundred-and-fifty-fourth`
