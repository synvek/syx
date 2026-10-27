# C-09 residual maintenance first-slice (W951 / ADR-1295)

## Status

**C-09 residual maintenance first-slice met** (ADR-1295 / W951): rollup inventory nails W937–W950 entry + CI floor (property CI ∧ fuzz CI). HostFn count **629**; C-06～C-11 **`[x]`**（bench/coverage/fuzz/property claimed + `property_smoke` + fuzz scaffold + CI property + CI fuzz **维持**）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ any `c09_*_ga=true` / ≠ 无界 fuzz / ≠ proptestest / ≠ llvm-cov % gate / ≠ 回翻 C-09 `[~]`.

Fingerprint: `eight-hundred-and-first` / `A-W951`（prior CI fuzz：`eight-hundredth` / `A-W950`；prior CI property：`seven-hundred-and-ninety-ninth` / `A-W949`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | W949 property CI floor still signs | `syx_property_ci_first_slice_inventory` |
| 2 | W950 fuzz CI floor still signs | `syx_fuzz_ci_first_slice_inventory` |
| 3 | Rollup inventory signs maintenance | `SyxC09ResidualMaintenanceFirstSliceInventory` / **`syx_c09_residual_maintenance_first_slice_signed=true`** / **`c09_residual_maintenance_ga=false`** |
| 4 | Topics / UsageGuide honesty | [`plan/topics/syx-fuzz.md`](../../plan/topics/syx-fuzz.md) · [`plan/topics/syx-property.md`](../../plan/topics/syx-property.md) · [`docs/9_UsageGuide.md`](../../docs/9_UsageGuide.md) |
| 5 | Prior residual lang Index | [`c09-syx-fuzz-stub-first-slice.md`](c09-syx-fuzz-stub-first-slice.md) · [`c09-syx-property-stub-first-slice.md`](c09-syx-property-stub-first-slice.md) · [`c09-syx-coverage-stub-first-slice.md`](c09-syx-coverage-stub-first-slice.md) · [`c09-syx-bench-entry-first-slice.md`](c09-syx-bench-entry-first-slice.md) |
| 6 | Roadmap C-09 `[x]` retained | plan/02 C-09 **`[x]`** |
| 7 | ci.yml still nails `property_smoke` + bounded `syx_smoke` | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) |

## Index

| Prior | Status |
|-------|--------|
| Bench entry claimed | **met**（W937） |
| Coverage stub→claimed | **met**（W938–W940） |
| Fuzz stub→claimed | **met**（W941–W943） |
| Property stub→claimed + `property_smoke` | **met**（W944–W947） |
| Fuzz scaffold | **met**（W948） |
| CI property | **met**（W949） |
| CI fuzz (`-runs=1`) | **met**（W950） |
| Residual maintenance rollup | **met**（W951；`c09_residual_maintenance_ga=false`） |
| Platform residual maintenance | **met**（W952；详见 [`platform-residual-maintenance-first-slice.md`](platform-residual-maintenance-first-slice.md)） |
| F-08 / Wave D/E / HostFn+1 | **still frozen / unmet** |
| proptestest / scaffold GA / unbounded fuzz / llvm-cov % | **still unmet** |

## Behavior (hard)

本窗**不**新增 runner / CI job。维护地板 = W949 property CI inventory **且** W950 fuzz CI inventory 仍签；文档与代码对齐。

```bash
# Unchanged product entries (maintenance floor)
syx prop
syx fuzz

# PR CI floors retained (W949 / W950)
cargo test -p syx-driver --test property_smoke -- --nocapture
cargo fuzz run syx_smoke -- -runs=1
```

## Contract (hard)

| Item | Meaning |
|------|---------|
| Maintenance inventory | `SyxC09ResidualMaintenanceFirstSliceInventory` / `syx_c09_residual_maintenance_first_slice_inventory` |
| Flags | `ci_floor_ok=true`; `residual_entries_present=true`; **`c09_residual_maintenance_ga=false`**; **`syx_c09_residual_maintenance_first_slice_signed=true`** |
| CI floor | W949 `syx_property_ci_first_slice_inventory` ∧ W950 `syx_fuzz_ci_first_slice_inventory` |
| Forbidden claim | any `c09_*_ga=true` / F-08 / HostFn+1 / 无界 fuzz / proptestest / llvm-cov % gate / Wave D/E / 回翻 C-09 `[~]` |
| Honesty | maintenance rollup **≠** residual maintenance GA / **≠** new tooling |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
- Construction residual-maintenance / prior CI meta retired（CI `property_smoke` + `syx_smoke` remain）

## Frozen

- Claiming any `c09_*_ga=true` / opening F-08 / HostFn+1 / unbounded fuzz / proptestest / llvm-cov % gate
- Adding new CI jobs or installing new tools this window
- Opening Wave D/E without explicit ADR / reverting C-09 `[x]`

## Next

W951 C-09 residual maintenance **已交**（ADR-1295）→ W952 platform residual maintenance **已交**（ADR-1296）→ W953 Math residual maintenance **已交**（ADR-1297）→ W954 maintenance chain closeout **已交**（ADR-1298）→ W955 Math llvm Phase-1 **已交**（ADR-1299）→ W956 Math llvm Phase-2 **已交**（ADR-1300）→ W957 Math llvm Soft **已交**（ADR-1301）→ W958 Math llvm Special **已交**（ADR-1302）→ W959 Math llvm F32 **已交**（ADR-1303）→ W960 Math llvm F32 slice2 **已交**（ADR-1304）→ W961 Math llvm F32 slice3 **已交**（ADR-1305）→ W962 Math llvm F32 slice4 **已交**（ADR-1306）→ W963 Math llvm F32 slice5 **已交**（ADR-1307）→ W964 Math llvm F32 soft-close **已交**（ADR-1308）→ W965 Math llvm GA exit **已交**（ADR-1309）→ W966 QS1 Math **已交**（ADR-1310）→ W967 QS1 Math slice2 **已交**（ADR-1311）→ W968 QS1 Math Phase-2 **已交**（ADR-1312）→ W969 QS1 Math Soft **已交**（ADR-1313）→ W970 QS1 Math Special **已交**（ADR-1314）→ W971+（剩余 Phase-1×11 或 F32 QS1 切片或 F-08 / D/E 需显式 ADR）
