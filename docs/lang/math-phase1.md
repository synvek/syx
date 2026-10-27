# Math Phase-1（W902 / ADR-1246）

Core prelude `Math`（[`packages/core/math.sy`](../../packages/core/math.sy)）补齐首片，对标 Java `Math` / Rust `f64` / Swift Darwin **常用面**的子集。

## 本窗交付

- **常量（小写 static getter）：** `pi` `e` `tau` `ln2` `ln10` `log2e` `log10e` `sqrt2` `fracPi2` `fracPi4`
- **Int：** `gcd` `lcm` `clamp` `sign`（纯 SYX；wasm while/gcd **不宣称**）
- **Float64 便利：** `fmin` `fmax` `fabs` `fclamp` `toDegrees` `toRadians`；`ln` = `log`（自然对数；≠ `log10`/`log2`）
- **新 Host（+12 → HostFn 566）：** asin acos atan atan2 exp exp2 log2 log10 hypot cbrt trunc abs
- **能力表：** 既有 + 新 `MathFloat*` → `vm/mir/wasm=real`，`llvm=unsupported`，`status=partial`（**W955 / ADR-1299：** Phase-1×21 `llvm→real`；见 [`math-llvm-first-slice.md`](math-llvm-first-slice.md)；≠ llvm Math GA）

## 非本窗

- F-07 / F-08 / wasm-gc
- 双曲、fmod、copysign、isNaN 族、Float32、SIMD（→ W903+；**Phase-2 已交** 见 [math-phase2.md](math-phase2.md)；**Soft 已交** 见 [math-soft.md](math-soft.md)；**Special 已交** 见 [math-special.md](math-special.md)；**F32 已交** 见 [math-f32.md](math-f32.md)）
- C-06 Soft（member-index compound）

## 证据

- `crates/syx-driver/tests/stdlib/math.rs`
- Matrix：`A-W902` / `seven-hundred-and-fifty-second`
