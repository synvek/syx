# B-09 storage-cleanup product flip first-slice (W556 / ADR-900)

## Status

**B-09 storage-cleanup product flip first-slice met** (policy+inventory+API): after W555 ObjectRef RC migrate product ON, W494 dead-API retire, and W495/W496 MemoryPool stub retire, this window signs **`storage_cleanup_enabled=true`** and exposes `syx_rt::storage_cleanup_enabled()` (readonly). SharedCell Arc lifetime substrate **still present** (≠ `unified_mir_runtime_heap_enabled`；≠ B-09 `[x]`). This window does **not** enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈98–99%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `four-hundred-and-sixth` / `A-W556`.

## Index

| Prior | Status |
|-------|--------|
| W476–W555 narrative → migrate product flip | **met** |
| W494 storage-cleanup dead-API first-slice | **met** |
| W495/W496 MemoryPool stub retire | **met** |
| This window storage-cleanup product flip | **met** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-ii-m2. ObjectRef RC migrate product flip | Flip `objectref_rc_migrate_enabled` | **met** (W555) |
| 4c-ii. Storage-layer cleanup product flip | Flip `storage_cleanup_enabled` | **met** (this window) |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Product honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Storage cleanup product flip signed | **met** | this window |
| `storage_cleanup_enabled` | **yes** | true |
| `syx_rt::storage_cleanup_enabled()` | **yes** | returns true |
| ObjectRef RC migrate product | **met** | W555 |
| Dead `Gc::collect*` API retired | **met** | W494 |
| MemoryPool type stub retired | **met** | W495/W496 |
| SharedCell Arc lifetime | **still present** | ≠ enable；≠ `[x]` |
| `unified_mir_runtime_heap_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | cleanup ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Storage cleanup product flip signed | **yes** | This window |
| Unified heap enabled | **no** | Still open |
| B-09 `[x]` / GC GA | **no** | cleanup ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09StorageCleanupProductFirstSliceInventory` / `gate_b09_storage_cleanup_product_first_slice_inventory` |
| API | `syx_rt::storage_cleanup_enabled() -> bool` (reads this inventory) |
| Flags | `b09_storage_cleanup_product_first_slice_signed=true`; `b09_objectref_rc_migrate_product_first_slice_floor_ok=true`; `objectref_rc_migrate_enabled=true`; `migrate_product_flip_met=true`; `b09_storage_cleanup_first_slice_floor_ok=true`; `storage_cleanup_dead_api_retired=true`; `memory_pool_type_stub_retired=true`; `sharedcell_arc_lifetime_still_present=true`; `storage_cleanup_enabled=true`; `storage_cleanup_product_flip_met=true`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming cleanup as enable or `[x]` |
| Honesty | storage cleanup product flip **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W494–W555 historical inventory snapshots **not rewritten**（仍 `cleanup=false`） |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09StorageCleanupProductFirstSliceInventory` + `storage_cleanup_enabled()`
- Meta: `b09_storage_cleanup_product_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-migrate-product-first-slice.md`](gate-b09-objectref-rc-migrate-product-first-slice.md)
- Historical: [`gate-b09-storage-cleanup-first-slice.md`](gate-b09-storage-cleanup-first-slice.md)（W494；Contract 不改写）
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-900 / fingerprint `A-W556` / `qs1-platform-gate-v0:driver-gate-exec-four-hundred-and-sixth`

## Frozen

- Flipping `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W494–W555 Contract table field claims as if current
- Claiming cleanup product flip as enable / `[x]`
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w555_*.plan.md`

## Next

W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558 B-09 axis exit first-slice（窄 GC GA）**已交**（ADR-902）→ W559+ — 扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
