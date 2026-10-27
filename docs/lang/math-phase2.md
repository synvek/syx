# Math Phase-2（W903 / ADR-1247）

续 [math-phase1.md](math-phase1.md)。对标 Java `Math` / Rust `f64` 的双曲、余数/符号、分类、近 1 指数对数。

## 本窗交付

- **双曲：** `sinh` `cosh` `tanh`
- **余数 / 符号：** `fmod` `copysign`
- **分类（Bool）：** `isNaN` `isInfinite` `isFinite`
- **近 1：** `expm1` `log1p`
- **HostFn +10 → 576**；capability `vm/mir/wasm=real`，`llvm=unsupported`（**W956 / ADR-1300：** Phase-2×10 `llvm→real`；见 [`math-llvm-phase2-first-slice.md`](math-llvm-phase2-first-slice.md)；≠ llvm Math GA）

## 非本窗

- asinh/acosh/atanh、remainder、nextUp/nextDown、mulAdd → **已交** 见 [math-soft.md](math-soft.md)
- erf/erfc/tgamma → **已交** 见 [math-special.md](math-special.md)
- Float32 薄包装 → **已交** 见 [math-f32.md](math-f32.md)
- Float64 实例方法 → **已交** 见 [math-f64-methods.md](math-f64-methods.md)
- F-07 / F-08 / CST Soft / llvm Math GA

## 证据

- `crates/syx-driver/tests/stdlib/math.rs`
- Matrix：`A-W903` / `seven-hundred-and-fifty-third`
