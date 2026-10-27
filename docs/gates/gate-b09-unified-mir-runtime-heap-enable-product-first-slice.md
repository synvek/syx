# B-09 unified MirRuntime heap enable product flip first-slice (W557 / ADR-901)

## Status

**B-09 unified MirRuntime heap enable product flip first-slice met** (policy+inventory+API): after W556 storage-cleanup product ON, W555 migrate product ON, and W493 enable-readiness, this window signs **`unified_mir_runtime_heap_enabled=true`** and exposes `syx_rt::unified_mir_runtime_heap_enabled()` (readonly). SharedCell Arc lifetime substrate **still present** (≠ B-09 `[x]`；≠ GC GA). Runtime `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` is a **separate env gate** (default ON). This window does **not** claim B-09 `[x]` or GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈99%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `four-hundred-and-seventh` / `A-W557`.

## Index

| Prior | Status |
|-------|--------|
| W476–W556 narrative → migrate + cleanup product | **met** |
| W493 enable-readiness thin | **met** |
| This window unified MirRuntime heap enable product flip | **met** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-ii. Storage-layer cleanup product flip | Flip `storage_cleanup_enabled` | **met** (W556) |
| 4d. True enable product flip | Flip `unified_mir_runtime_heap_enabled` | **met** (this window) |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Product honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Unified enable product flip signed | **met** | this window |
| `unified_mir_runtime_heap_enabled` | **yes** | true |
| `syx_rt::unified_mir_runtime_heap_enabled()` | **yes** | returns true |
| Storage cleanup product | **met** | W556 |
| ObjectRef RC migrate product | **met** | W555 |
| Enable-readiness (W493) | **met** | historical |
| SharedCell Arc lifetime | **still present** | ≠ `[x]`；≠ GA |
| `b09_axis_exit_enabled` / GC GA | **no** | enable ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Unified enable product flip signed | **yes** | This window |
| B-09 `[x]` / GC GA | **no** | Still open |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory` / `gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory` |
| API | `syx_rt::unified_mir_runtime_heap_enabled() -> bool` (reads this inventory) |
| Flags | `b09_unified_mir_runtime_heap_enable_product_first_slice_signed=true`; `b09_storage_cleanup_product_first_slice_floor_ok=true`; `storage_cleanup_enabled=true`; `storage_cleanup_product_flip_met=true`; `objectref_rc_migrate_enabled=true`; `migrate_product_flip_met=true`; `b09_enable_readiness_thin_floor_ok=true`; `sharedcell_arc_lifetime_still_present=true`; `unified_mir_runtime_heap_enabled=true`; `unified_enable_product_flip_met=true`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping axis exit / GC GA / Roadmap B-09 `[x]` / wasm-gc / HostFn+1 / MIR distance change / claiming enable as `[x]` or rewriting `ManagedHeap::enabled()` |
| Honesty | unified enable product flip **≠** B-09 `[x]` / **≠** GC GA；W493–W556 historical inventory snapshots **not rewritten**（仍 `enable=false`） |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory` + `unified_mir_runtime_heap_enabled()`
- Meta: `b09_unified_mir_runtime_heap_enable_product_first_slice.rs`
- Floor: [`gate-b09-storage-cleanup-product-first-slice.md`](gate-b09-storage-cleanup-product-first-slice.md)
- Historical: [`gate-b09-enable-readiness-thin.md`](gate-b09-enable-readiness-thin.md)（W493；Contract 不改写）
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-901 / fingerprint `A-W557` / `qs1-platform-gate-v0:driver-gate-exec-four-hundred-and-seventh`

## Frozen

- Flipping `b09_axis_exit_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W493–W556 Contract table field claims as if current
- Claiming enable product flip as `[x]` / GA
- Changing `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` defaults this window
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w556_*.plan.md`

## Next

W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558 B-09 axis exit first-slice（窄 GC GA）**已交**（ADR-902）→ W559+ — 扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
