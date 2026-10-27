# Math llvm F32 slice2 first-slice (W960 / ADR-1304)

## Status

**Math llvm F32 slice2 first-slice met (partial)** (ADR-1304 / W960): W907 `MathFloat32*`×8 `llvm=unsupported→real`；native llvm F32 slice2 smoke exact；`status` 仍 `partial`；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ `math_llvm_ga=true` / ≠ llvm Math GA / ≠ 全 F32×42 / ≠ QS1 Math 行 / ≠ HostFn+1 / ≠ W908–W911 F32 llvm raise。

Fingerprint: `eight-hundred-and-tenth` / `A-W960`（prior F32-W906：`eight-hundred-and-ninth` / `A-W959`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W959 F32 W906 llvm floor | `syx_math_llvm_f32_first_slice_inventory` |
| 2 | W907×8 capability llvm=real | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) W907 段 |
| 3 | Native llvm F32 slice2 smoke exact | `math_f32_slice2_smoke_native_llvm` in [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs) |
| 4 | F32 slice2 llvm inventory | `SyxMathLlvmF32Slice2FirstSliceInventory` / **`syx_math_llvm_f32_slice2_first_slice_signed=true`** / **`math_llvm_ga=false`** |
| 5 | Prior F32 lang | [`math-f32.md`](math-f32.md) / [`math-llvm-f32-first-slice.md`](math-llvm-f32-first-slice.md) |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1～Special llvm | **partial met**（W955–W958） |
| F32 W906 llvm first-slice | **partial met**（W959；×8） |
| F32 W907 llvm slice2 | **partial met**（W960；×8） |
| F32 W908 llvm slice3 | **partial met**（W961；详见 [`math-llvm-f32-slice3-first-slice.md`](math-llvm-f32-slice3-first-slice.md)；×8） |
| F32 W909 llvm slice4 | **partial met**（W962；详见 [`math-llvm-f32-slice4-first-slice.md`](math-llvm-f32-slice4-first-slice.md)；×8） |
| F32 W910 llvm slice5 | **partial met**（W963；详见 [`math-llvm-f32-slice5-first-slice.md`](math-llvm-f32-slice5-first-slice.md)；×8） |
| F32 W911 llvm soft-close | **partial met**（W964；详见 [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md)；×2） |
| llvm Math GA | **met (narrow)**（W965；详见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)；MathFloat* 84/84） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn / RT ABI。行为钉 = 既有 `MATH_F32_SLICE2_SMOKE` 经 `build_native_executable` 跑 native llvm AOT，stdout exact 对齐 wasm/mir F32 W907 smoke。复用 W955 `syx_rt_push_numeric64` 地板。

## Contract (hard)

| Item | Meaning |
|------|---------|
| F32 slice2 inventory | `SyxMathLlvmF32Slice2FirstSliceInventory` / `syx_math_llvm_f32_slice2_first_slice_inventory` |
| Flags | `f32_w906_floor_ok=true`; `f32_w907_llvm_raised=true`; `native_llvm_f32_slice2_smoke_present=true`; **`math_llvm_ga=false`**; **`syx_math_llvm_f32_slice2_first_slice_signed=true`** |
| Raised | W907 `MathFloat32Ceil`…`MathFloat32Hypot`（×8）`llvm=real` |
| Forbidden claim | llvm Math GA / QS1 Math / full F32×42 / W908–W911 llvm / `math_llvm_ga=true` / F-08 / HostFn+1 |
| Honesty | +F32-W907 llvm **partial met**（58/84 at W960；现累计见 slice3 页）≠ llvm Math GA |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_f32_slice2_first_slice.rs`
- Behavior: `math_f32_slice2_smoke_native_llvm`
- Capability: [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)
- Prior: [`math-llvm-f32-first-slice.md`](math-llvm-f32-first-slice.md)

## Frozen

- Claiming llvm Math GA / QS1 Math / full F32×42 / `math_llvm_ga=true`
- Raising W908–W911 F32 llvm this W960 charter window（W908 已于 W961 交）
- Opening F-08 / Wave D/E / HostFn+1 / `status=implemented` for Math*

## Next

W960 Math llvm F32 slice2 first-slice **已交**（ADR-1304）→ W961 Math llvm F32 slice3 first-slice **已交**（ADR-1305）→ W962 Math llvm F32 slice4 first-slice **已交**（ADR-1306）→ W963 Math llvm F32 slice5 first-slice **已交**（ADR-1307）→ W964 Math llvm F32 soft-close first-slice **已交**（ADR-1308）→ W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975+（`MATH_F32_SLICE4_SMOKE`×8 QS1 或 F-08 / D/E 仍需显式 ADR）
