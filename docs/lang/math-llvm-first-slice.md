# Math llvm Phase-1 first-slice (W955 / ADR-1299)

## Status

**Math llvm Phase-1 first-slice met (partial)** (ADR-1299 / W955): W902 Phase-1 `MathFloat*`×21 `llvm=unsupported→real`；native llvm host smoke exact；`status` 仍 `partial`；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ `math_llvm_ga=true` / ≠ llvm Math GA / ≠ QS1 Math 行 / ≠ HostFn+1 / ≠ Phase-2/Soft/Special/F32 llvm raise。

Fingerprint: `eight-hundred-and-fifth` / `A-W955`（prior closeout：`eight-hundred-and-fourth` / `A-W954`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Phase-1×21 capability llvm=real | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) W902 段 |
| 2 | Native llvm host smoke exact | `math_host_smoke_native_llvm` in [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs) |
| 3 | Math llvm inventory | `SyxMathLlvmFirstSliceInventory` / **`syx_math_llvm_first_slice_signed=true`** / **`math_llvm_ga=false`** |
| 4 | Prior Phase-1 lang | [`math-phase1.md`](math-phase1.md) |
| 5 | Closeout floor | [`maintenance-chain-closeout-first-slice.md`](maintenance-chain-closeout-first-slice.md)（meta F-10 retired；inventory 仍签） |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1 HostFn (W902) | **met**（vm/mir/wasm） |
| Phase-1 llvm first-slice | **partial met**（W955；×21） |
| Phase-2 llvm first-slice | **partial met**（W956；详见 [`math-llvm-phase2-first-slice.md`](math-llvm-phase2-first-slice.md)；×10） |
| Soft llvm first-slice | **partial met**（W957；详见 [`math-llvm-soft-first-slice.md`](math-llvm-soft-first-slice.md)） |
| Special llvm first-slice | **partial met**（W958；详见 [`math-llvm-special-first-slice.md`](math-llvm-special-first-slice.md)） |
| F32 llvm | **partial met**（W959–W964 MathFloat* **84/84**；详见 [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md)） |
| llvm Math GA | **met (narrow)**（W965；详见 [`math-llvm-ga-exit-first-slice.md`](math-llvm-ga-exit-first-slice.md)） |
| Maintenance chain closeout | **met**（W954；meta F-10 retired） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn。行为钉 = 既有 `MATH_HOST_SMOKE` 经 `build_native_executable` 跑 native llvm AOT，stdout exact 对齐 wasm host smoke。

**RT 地板：** LLVM `ConstNumeric` 改走 `syx_rt_push_numeric64(kind, bits:i64)`（历史 `syx_rt_push_numeric` 仅传 bits 低 32 位，会把多数 f64 字面量截成 0）。
## Contract (hard)

| Item | Meaning |
|------|---------|
| Math llvm inventory | `SyxMathLlvmFirstSliceInventory` / `syx_math_llvm_first_slice_inventory` |
| Flags | `phase1_llvm_raised=true`; `native_llvm_host_smoke_present=true`; **`math_llvm_ga=false`**; **`syx_math_llvm_first_slice_signed=true`** |
| Raised | Phase-1 `MathFloatSin`…`MathFloatAbs`（×21）`llvm=real` |
| Forbidden claim | llvm Math GA / QS1 Math / `math_llvm_ga=true` / F-08 / HostFn+1 / Phase-2+ llvm raise |
| Honesty | Phase-1 llvm **partial met** ≠ llvm Math GA |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_first_slice.rs`
- Behavior: `math_host_smoke_native_llvm`
- RT: `syx_rt_push_numeric64`（[`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs)）+ LLVM ConstNumeric emit
- Capability: [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)

## Frozen

- Claiming llvm Math GA / QS1 Math / `math_llvm_ga=true`
- Opening F-08 / Wave D/E / HostFn+1 / `status=implemented` for Math*

## Next

W955 Math llvm Phase-1 first-slice **已交**（ADR-1299）→ W956 Math llvm Phase-2 first-slice **已交**（ADR-1300）→ W957 Math llvm Soft first-slice **已交**（ADR-1301）→ W958 Math llvm Special first-slice **已交**（ADR-1302）→ W959 Math llvm F32 first-slice **已交**（ADR-1303）→ W960 Math llvm F32 slice2 first-slice **已交**（ADR-1304）→ W961 Math llvm F32 slice3 first-slice **已交**（ADR-1305）→ W962 Math llvm F32 slice4 first-slice **已交**（ADR-1306）→ W963 Math llvm F32 slice5 first-slice **已交**（ADR-1307）→ W964 Math llvm F32 soft-close first-slice **已交**（ADR-1308）→ W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975+（`MATH_F32_SLICE4_SMOKE`×8 QS1 或 F-08 / D/E 仍需显式 ADR）
