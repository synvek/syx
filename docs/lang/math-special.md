# Math Special（W905 / ADR-1249）

续 [math-soft.md](math-soft.md)。Soft「非本窗」特殊函数首片：误差函数与 gamma 族（via `libm`；稳定 Rust 无 `f64::erf/gamma`）。

## 本窗交付

- **误差函数：** `erf` `erfc`
- **Gamma：** `tgamma`（Γ(x)）· `lgamma`（ln|Γ(x)|）
- **HostFn +4 → 587**；VM 依赖 `libm`；capability `vm/mir/wasm=real`，`llvm=unsupported`（**W958 / ADR-1302：** Special×4 `llvm→real`；见 [`math-llvm-special-first-slice.md`](math-llvm-special-first-slice.md)；≠ llvm Math GA）

## 非本窗

- Float32 薄包装 → **已交** 见 [math-f32.md](math-f32.md)
- Float64 实例方法 → **已交** 见 [math-f64-methods.md](math-f64-methods.md)
- F-07 / F-08 / CST Soft / llvm Math GA / QS1 Math 行
- unstable `float_erf` / `float_gamma` feature

## 证据

- `crates/syx-driver/tests/stdlib/math.rs`
- Matrix：`A-W905` / `seven-hundred-and-fifty-fifth`
