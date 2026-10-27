# Math llvm Special first-slice (W958 / ADR-1302)

## Status

**Math llvm Special first-slice met (partial)** (ADR-1302 / W958): W905 Special `MathFloat*`×4 `llvm=unsupported→real`；native llvm special smoke exact；`status` 仍 `partial`；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ `math_llvm_ga=true` / ≠ llvm Math GA / ≠ QS1 Math 行 / ≠ HostFn+1 / ≠ F32 llvm raise。

Fingerprint: `eight-hundred-and-eighth` / `A-W958`（prior Soft：`eight-hundred-and-seventh` / `A-W957`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W957 Soft llvm floor | `syx_math_llvm_soft_first_slice_inventory` |
| 2 | Special×4 capability llvm=real | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) W905 段 |
| 3 | Native llvm Special smoke exact | `math_special_smoke_native_llvm` in [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs) |
| 4 | Special llvm inventory | `SyxMathLlvmSpecialFirstSliceInventory` / **`syx_math_llvm_special_first_slice_signed=true`** / **`math_llvm_ga=false`** |
| 5 | Prior Special lang | [`math-special.md`](math-special.md) |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1 llvm first-slice | **partial met**（W955；×21） |
| Phase-2 llvm first-slice | **partial met**（W956；×10） |
| Soft llvm first-slice | **partial met**（W957；×7） |
| Special HostFn (W905) | **met**（vm/mir/wasm） |
| Special llvm first-slice | **partial met**（W958；×4） |
| F32 W906 llvm first-slice | **partial met**（W959；详见 [`math-llvm-f32-first-slice.md`](math-llvm-f32-first-slice.md)；×8） |
| F32 W907 llvm slice2 | **partial met**（W960；详见 [`math-llvm-f32-slice2-first-slice.md`](math-llvm-f32-slice2-first-slice.md)；×8） |
| F32 W908 llvm slice3 | **partial met**（W961；详见 [`math-llvm-f32-slice3-first-slice.md`](math-llvm-f32-slice3-first-slice.md)；×8） |
| F32 W909 llvm slice4 | **partial met**（W962；详见 [`math-llvm-f32-slice4-first-slice.md`](math-llvm-f32-slice4-first-slice.md)；×8） |
| F32 W910 llvm slice5 | **partial met**（W963；详见 [`math-llvm-f32-slice5-first-slice.md`](math-llvm-f32-slice5-first-slice.md)；×8） |
| F32 W911 llvm soft-close | **partial met**（W964；详见 [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md)；×2） |
| llvm Math GA | **met (narrow)**（W965；详见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)；MathFloat* 84/84） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn / RT ABI。行为钉 = 既有 `MATH_SPECIAL_SMOKE` 经 `build_native_executable` 跑 native llvm AOT，stdout exact 对齐 wasm/mir Special smoke。复用 W955 `syx_rt_push_numeric64` 地板。

## Contract (hard)

| Item | Meaning |
|------|---------|
| Special llvm inventory | `SyxMathLlvmSpecialFirstSliceInventory` / `syx_math_llvm_special_first_slice_inventory` |
| Flags | `soft_llvm_floor_ok=true`; `special_llvm_raised=true`; `native_llvm_special_smoke_present=true`; **`math_llvm_ga=false`**; **`syx_math_llvm_special_first_slice_signed=true`** |
| Raised | Special `MathFloatErf`…`MathFloatLgamma`（×4）`llvm=real` |
| Forbidden claim | llvm Math GA / QS1 Math / F32 llvm / `math_llvm_ga=true` / F-08 / HostFn+1 |
| Honesty | Phase-1+2+Soft+Special llvm **partial met**（42/84 at W958；现累计见 F32 页）≠ llvm Math GA |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_special_first_slice.rs`
- Behavior: `math_special_smoke_native_llvm`
- Capability: [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)
- Prior: [`math-llvm-soft-first-slice.md`](math-llvm-soft-first-slice.md)
- Next F32: [`math-llvm-f32-first-slice.md`](math-llvm-f32-first-slice.md)

## Frozen

- Claiming llvm Math GA / QS1 Math / `math_llvm_ga=true`
- Raising remaining F32 W907–W911 llvm this Special page window charter（W906 已于 W959 交）
- Opening F-08 / Wave D/E / HostFn+1 / `status=implemented` for Math*

## Next

W958 Math llvm Special first-slice **已交**（ADR-1302）→ W959 Math llvm F32 first-slice **已交**（ADR-1303）→ W960 Math llvm F32 slice2 first-slice **已交**（ADR-1304）→ W961 Math llvm F32 slice3 first-slice **已交**（ADR-1305）→ W962 Math llvm F32 slice4 first-slice **已交**（ADR-1306）→ W963 Math llvm F32 slice5 first-slice **已交**（ADR-1307）→ W964 Math llvm F32 soft-close first-slice **已交**（ADR-1308）→ W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975+（`MATH_F32_SLICE4_SMOKE`×8 QS1 或 F-08 / D/E 仍需显式 ADR）
