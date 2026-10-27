# Math llvm GA exit first-slice (W965 / ADR-1309)

## Status

**Math llvm GA exit first-slice met (narrow)** (ADR-1309 / W965): checklist SSOT below complete；product inventory signs **`math_llvm_ga=true`**；MathFloat* llvm **84/84**；历史 soft-close inventory **保持** `math_llvm_ga=false`；HostFn **629**；F-08 **仍冻**；Wave D/E **仍冻**；≠ QS1 Math 行 / ≠ `status=implemented` / ≠ HostFn+1。

Fingerprint: `eight-hundred-and-fifteenth` / `A-W965`（prior soft-close：`eight-hundred-and-fourteenth` / `A-W964`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W964 F32 soft-close floor | `syx_math_llvm_f32_soft_close_first_slice_inventory`（`math_float_llvm_all_real` ∧ historical `math_llvm_ga=false`） |
| 2 | MathFloat* llvm 84/84 | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) 无 `MathFloat*` `llvm=unsupported` |
| 3 | Native llvm smoke ladder | `math.rs`：host / hyperbolic / soft / special / f32 / slice2–5 / soft_close `*_smoke_native_llvm` |
| 4 | GA-exit llvm inventory | `SyxMathLlvmGaExitFirstSliceInventory` / **`syx_math_llvm_ga_exit_first_slice_signed=true`** / **`math_llvm_ga=true`** |
| 5 | Prior soft-close lang | [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md) |
| 6 | ga-evidence-audit tip | [`plan/topics/ga-evidence-audit.md`](../../plan/topics/ga-evidence-audit.md) |

## Index

| Prior | Status |
|-------|--------|
| Phase-1～Special + F32 llvm slices | **partial met**（W955–W964；84/84） |
| F32 soft-close floor | **partial met**（W964；historical `math_llvm_ga=false`） |
| llvm Math GA (narrow claimed) | **met**（W965） |
| QS1 Math pilot ×4 | **partial met**（W966；详见 [`qs1-math-first-slice.md`](qs1-math-first-slice.md)） |
| QS1 Math slice2 / HOST_SMOKE 齐 | **partial met**（W967；详见 [`qs1-math-slice2-first-slice.md`](qs1-math-slice2-first-slice.md)；**10/84**） |
| QS1 Math Phase-2 ×10 | **partial met**（W968；详见 [`qs1-math-phase2-first-slice.md`](qs1-math-phase2-first-slice.md)；**20/84**） |
| QS1 Math Soft ×7 | **partial met**（W969；详见 [`qs1-math-soft-first-slice.md`](qs1-math-soft-first-slice.md)；**27/84**） |
| QS1 Math Special ×4 | **partial met**（W970；详见 [`qs1-math-special-first-slice.md`](qs1-math-special-first-slice.md)；**31/84**） |
| QS1 Math Phase-1 gap ×11 | **partial met**（W971；详见 [`qs1-math-phase1-gap-first-slice.md`](qs1-math-phase1-gap-first-slice.md)；**42/84**） |
| QS1 Math F32 first-slice ×8 | **partial met**（W972；详见 [`qs1-math-f32-first-slice.md`](qs1-math-f32-first-slice.md)；**50/84**） |
| QS1 Math F32 slice2 ×8 | **partial met**（W973；详见 [`qs1-math-f32-slice2-first-slice.md`](qs1-math-f32-slice2-first-slice.md)；**58/84**） |
| QS1 Math F32 slice3 ×8 | **partial met**（W974；详见 [`qs1-math-f32-slice3-first-slice.md`](qs1-math-f32-slice3-first-slice.md)；**66/84**） |
| QS1 Math F32 slice4 ×8 | **partial met**（W975；详见 [`qs1-math-f32-slice4-first-slice.md`](qs1-math-f32-slice4-first-slice.md)；**74/84**） |
| QS1 Math F32 slice5 ×8 | **partial met**（W976；详见 [`qs1-math-f32-slice5-first-slice.md`](qs1-math-f32-slice5-first-slice.md)；**82/84**） |
| QS1 Math F32 soft-close ×2 | **partial met**（W977；详见 [`qs1-math-f32-soft-close-first-slice.md`](qs1-math-f32-soft-close-first-slice.md)；**84/84 rows**） |
| QS1 Math narrow GA | **met**（W978；详见 [`qs1-math-ga-exit-first-slice.md`](qs1-math-ga-exit-first-slice.md)） |
| Math* `status=implemented` | **still unmet**（仍 `partial`） |
| Math* `status=implemented` | **still unmet**（仍 `partial`） |
| F-08 / Wave D/E / HostFn+1 | **still frozen** |

## Behavior (hard)

本窗**不**新增 Math API / HostFn / RT ABI / 能力表 llvm raise。窄面 claimed = 既有证据地板齐 → inventory 翻 `math_llvm_ga=true`。行为钉仍靠既有 native llvm smoke exact（不新写 smoke 源）。

## Contract (hard)

| Item | Meaning |
|------|---------|
| GA-exit inventory | `SyxMathLlvmGaExitFirstSliceInventory` / `syx_math_llvm_ga_exit_first_slice_inventory` |
| Flags | `f32_soft_close_floor_ok=true`; `native_llvm_smokes_all_present=true`; **`math_float_llvm_all_real=true`**; **`math_llvm_ga=true`**; **`syx_math_llvm_ga_exit_first_slice_signed=true`** |
| Honesty floor | `SyxMathLlvmF32SoftCloseFirstSliceInventory` 保持 **`math_llvm_ga=false`**（historical） |
| Forbidden claim | QS1 Math 行 / Math* `status=implemented` / F-08 / HostFn+1 / Wave D/E |
| Honesty | 窄面 llvm Math GA **met** ≠ QS1 Math ≠ `status=implemented` ≠ F-08 |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- Meta: `math_llvm_ga_exit_first_slice.rs`
- Smokes: [`math.rs`](../../crates/syx-driver/tests/stdlib/math.rs)（rollup 既有）
- Floor: [`math-llvm-f32-soft-close-first-slice.md`](math-llvm-f32-soft-close-first-slice.md)

## Frozen

- Claiming full MathFloat* QS1 / `qs1_math_ga=true` / Math* `status=implemented` / HostFn+1（W966 pilot ×4 另页）
- Opening F-08 / Wave D/E
- Editing historical W955–W964 inventories to flip their `math_llvm_ga=true`

## Next

W965 Math llvm GA exit first-slice **已交**（ADR-1309）→ W966 QS1 Math first-slice **已交**（ADR-1310）→ W967 QS1 Math slice2 first-slice **已交**（ADR-1311）→ W968 QS1 Math Phase-2 first-slice **已交**（ADR-1312）→ W969 QS1 Math Soft first-slice **已交**（ADR-1313）→ W970 QS1 Math Special first-slice **已交**（ADR-1314）→ W971 QS1 Math Phase-1 gap first-slice **已交**（ADR-1315）→ W972 QS1 Math F32 first-slice **已交**（ADR-1316）→ W973 QS1 Math F32 slice2 first-slice **已交**（ADR-1317）→ W974 QS1 Math F32 slice3 first-slice **已交**（ADR-1318）→ W975 QS1 Math F32 slice4 first-slice **已交**（ADR-1319）→ W976 QS1 Math F32 slice5 first-slice **已交**（ADR-1320）→ W977 QS1 Math F32 soft-close first-slice **已交**（ADR-1321）→ W978 QS1 Math GA exit first-slice **已交**（ADR-1322）→ W979 Math status=implemented first-slice **已交**（ADR-1323）→ W980 Math status=implemented slice2 first-slice **已交**（ADR-1324）→ W981 Math status=implemented Phase-2 first-slice **已交**（ADR-1325）→ W982 Math status=implemented Soft first-slice **已交**（ADR-1326）→ W983 Math status=implemented Special first-slice **已交**（ADR-1327）→ W984 Math status=implemented Phase-1 gap first-slice **已交**（ADR-1328）→ W985 Math status=implemented F32 first-slice **已交**（ADR-1329）→ W986 Math status=implemented F32 slice2 first-slice **已交**（ADR-1330）→ W987 Math status=implemented F32 slice3 first-slice **已交**（ADR-1331）→ W988 Math status=implemented F32 slice4 first-slice **已交**（ADR-1332）→ W989 Math status=implemented F32 slice5 first-slice **已交**（ADR-1333）→ W990 Math status=implemented F32 soft-close first-slice **已交**（ADR-1334）→ W991 remaining HostFn status=implemented Fmt first-slice **已交**（ADR-1335）→ W992 remaining HostFn status=implemented Path first-slice **已交**（ADR-1336）→ W993 Math/QS1 中间 first-slice meta F-10 收缩 **已交**（ADR-1337）→ W994 remaining HostFn status=implemented File first-slice **已交**（ADR-1338）→ W995 remaining HostFn status=implemented File rest first-slice **已交**（ADR-1339）→ W996 remaining HostFn status=implemented Http rest first-slice **已交**（ADR-1340）→ W997 remaining HostFn status=implemented Sql core first-slice **已交**（ADR-1341）→ W998 remaining HostFn status=implemented Sql rest first-slice **已交**（ADR-1342）→ W999 remaining HostFn status=implemented Tcp first-slice **已交**（ADR-1343）→ W1000 remaining HostFn status=implemented Task first-slice **已交**（ADR-1344）→ W1001 remaining HostFn status=implemented Time/DateTime first-slice **已交**（ADR-1345）→ W1002 remaining HostFn status=implemented Json first-slice **已交**（ADR-1346）→ W1003 remaining HostFn status=implemented Str/Unicode first-slice **已交**（ADR-1347）→ W1004 Process/Child wasm-unsupported honesty first-slice **已交**（ADR-1348）→ W1005 Log core ×4 mir+llvm backend first-slice **已交**（ADR-1349）→ W1006 Log rest ×8 mir+llvm backend first-slice **已交**（ADR-1350）→ W1007+（Log wasm host-assisted→real、或 Trace/Metrics/Profiler 试点、或 Codable 后端 / 诚实 ADR；F-08 / D/E 需显式 ADR）
