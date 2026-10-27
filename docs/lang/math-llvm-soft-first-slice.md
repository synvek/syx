# Math llvm Soft first-slice (W957 / ADR-1301)

## Status

**Math llvm Soft first-slice met (partial)** (ADR-1301 / W957): W904 Soft `MathFloat*`×7 `llvm=unsupported→real`；native llvm soft smoke exact；`status` 仍 `partial`；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ `math_llvm_ga=true` / ≠ llvm Math GA / ≠ QS1 Math 行 / ≠ HostFn+1 / ≠ Special/F32 llvm raise。

Fingerprint: `eight-hundred-and-seventh` / `A-W957`（prior Phase-2：`eight-hundred-and-sixth` / `A-W956`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W956 Phase-2 llvm floor | `syx_math_llvm_phase2_first_slice_inventory` |
| 2 | Soft×7 capability llvm=real | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) W904 段 |
| 3 | Native llvm Soft smoke exact | `math_soft_smoke_native_llvm` in [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs) |
| 4 | Soft llvm inventory | `SyxMathLlvmSoftFirstSliceInventory` / **`syx_math_llvm_soft_first_slice_signed=true`** / **`math_llvm_ga=false`** |
| 5 | Prior Soft lang | [`math-soft.md`](math-soft.md) |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1 llvm first-slice | **partial met**（W955；×21） |
| Phase-2 llvm first-slice | **partial met**（W956；×10） |
| Soft HostFn (W904) | **met**（vm/mir/wasm） |
| Soft llvm first-slice | **partial met**（W957；×7） |
| Special llvm first-slice | **partial met**（W958；详见 [`math-llvm-special-first-slice.md`](math-llvm-special-first-slice.md)；×4） |
| F32 llvm | **partial met**（W959–W964 MathFloat* **84/84**；详见 [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md)） |
| llvm Math GA | **met (narrow)**（W965；详见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn / RT ABI。行为钉 = 既有 `MATH_SOFT_SMOKE` 经 `build_native_executable` 跑 native llvm AOT，stdout exact 对齐 wasm/mir Soft smoke。复用 W955 `syx_rt_push_numeric64` 地板。

## Contract (hard)

| Item | Meaning |
|------|---------|
| Soft llvm inventory | `SyxMathLlvmSoftFirstSliceInventory` / `syx_math_llvm_soft_first_slice_inventory` |
| Flags | `phase2_llvm_floor_ok=true`; `soft_llvm_raised=true`; `native_llvm_soft_smoke_present=true`; **`math_llvm_ga=false`**; **`syx_math_llvm_soft_first_slice_signed=true`** |
| Raised | Soft `MathFloatAsinh`…`MathFloatMulAdd`（×7）`llvm=real` |
| Forbidden claim | llvm Math GA / QS1 Math / Special·F32 llvm / `math_llvm_ga=true` / F-08 / HostFn+1 |
| Honesty | Phase-1+2+Soft llvm **partial met**（38/84 at W957；现累计见 Special 页）≠ llvm Math GA |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_soft_first_slice.rs`
- Behavior: `math_soft_smoke_native_llvm`
- Capability: [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)
- Prior: [`math-llvm-phase2-first-slice.md`](math-llvm-phase2-first-slice.md)
- Next Special: [`math-llvm-special-first-slice.md`](math-llvm-special-first-slice.md)

## Frozen

- Claiming llvm Math GA / QS1 Math / `math_llvm_ga=true`
- Raising F32 llvm this Soft page window charter（Special 已于 W958 交）
- Opening F-08 / Wave D/E / HostFn+1 / `status=implemented` for Math*

## Next

W957 Math llvm Soft first-slice **已交**（ADR-1301）→ W958 Math llvm Special first-slice **已交**（ADR-1302）→ W959 Math llvm F32 first-slice **已交**（ADR-1303）→ W960 Math llvm F32 slice2 first-slice **已交**（ADR-1304）→ W961 Math llvm F32 slice3 first-slice **已交**（ADR-1305）→ W962 Math llvm F32 slice4 first-slice **已交**（ADR-1306）→ W963 Math llvm F32 slice5 first-slice **已交**（ADR-1307）→ W964 Math llvm F32 soft-close first-slice **已交**（ADR-1308）→ W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975+（`MATH_F32_SLICE4_SMOKE`×8 QS1 或 F-08 / D/E 仍需显式 ADR）
