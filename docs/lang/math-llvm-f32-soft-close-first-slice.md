# Math llvm F32 soft-close first-slice (W964 / ADR-1308)

## Status

**Math llvm F32 soft-close first-slice met (partial)** (ADR-1308 / W964): W911 `MathFloat32NextDown`/`MulAdd`×2 `llvm=unsupported→real`；native llvm soft-close smoke exact；MathFloat* llvm **84/84**；`status` 仍 `partial`；**`math_llvm_ga=false`**；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ llvm Math GA / ≠ QS1 Math 行 / ≠ HostFn+1。

Fingerprint: `eight-hundred-and-fourteenth` / `A-W964`（prior F32-W910：`eight-hundred-and-thirteenth` / `A-W963`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W963 F32 W910 llvm floor | `syx_math_llvm_f32_slice5_first_slice_inventory` |
| 2 | W911×2 capability llvm=real | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) W911 段 |
| 3 | Native llvm soft-close smoke exact | `math_f32_soft_close_smoke_native_llvm` in [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs) |
| 4 | Soft-close llvm inventory | `SyxMathLlvmF32SoftCloseFirstSliceInventory` / **`syx_math_llvm_f32_soft_close_first_slice_signed=true`** / **`math_float_llvm_all_real=true`** / **`math_llvm_ga=false`** |
| 5 | Prior F32 lang | [`math-f32.md`](math-f32.md) / [`math-llvm-f32-slice5-first-slice.md`](math-llvm-f32-slice5-first-slice.md) |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1～Special llvm | **partial met**（W955–W958） |
| F32 W906～W910 llvm slices | **partial met**（W959–W963） |
| F32 W911 llvm soft-close | **partial met**（W964；×2） |
| MathFloat* llvm capability | **84/84 real** |
| llvm Math GA | **met (narrow)**（W965；详见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)；`math_llvm_ga=true` on GA-exit inventory；本页 historical inventory 仍 `false`） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn / RT ABI。行为钉 = 既有 `MATH_F32_SOFT_CLOSE_SMOKE` 经 `build_native_executable` 跑 native llvm AOT，stdout exact 对齐 wasm/mir F32 W911 soft-close smoke。复用 W955 `syx_rt_push_numeric64` 地板。

## Contract (hard)

| Item | Meaning |
|------|---------|
| Soft-close inventory | `SyxMathLlvmF32SoftCloseFirstSliceInventory` / `syx_math_llvm_f32_soft_close_first_slice_inventory` |
| Flags | `f32_w910_floor_ok=true`; `f32_w911_llvm_raised=true`; `native_llvm_f32_soft_close_smoke_present=true`; **`math_float_llvm_all_real=true`**; **`math_llvm_ga=false`**; **`syx_math_llvm_f32_soft_close_first_slice_signed=true`** |
| Raised | W911 `MathFloat32NextDown` `MathFloat32MulAdd`（×2）`llvm=real` |
| Forbidden claim | llvm Math GA / QS1 Math / `math_llvm_ga=true` / F-08 / HostFn+1 / `status=implemented` |
| Honesty | MathFloat* llvm **84/84 partial met** ≠ llvm Math GA |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_f32_soft_close_first_slice.rs`
- Behavior: `math_f32_soft_close_smoke_native_llvm`
- Capability: [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)
- Prior: [`math-llvm-f32-slice5-first-slice.md`](math-llvm-f32-slice5-first-slice.md)

## Frozen

- Claiming llvm Math GA / QS1 Math / `math_llvm_ga=true`
- Opening F-08 / Wave D/E / HostFn+1 / `status=implemented` for Math*

## Next

W964 Math llvm F32 soft-close first-slice **已交**（ADR-1308）→ W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975+（`MATH_F32_SLICE4_SMOKE`×8 QS1 或 F-08 / D/E 仍需显式 ADR）
