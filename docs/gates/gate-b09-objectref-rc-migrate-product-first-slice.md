# B-09 ObjectRef RC migrate product flip first-slice (W555 / ADR-899)

## Status

**B-09 ObjectRef RC migrate product flip first-slice met** (policy+inventory+API): after W554 post-mutrefcell-peel migrate-readiness refresh, migrate-flip blockers are cleared — MutRefCell + ObjectRef peels **met**; DictBuild collapse **met**; strip-RC **met** (W537); deep-walk **met** (W539). This window signs **`objectref_rc_migrate_enabled=true`** and exposes `syx_rt::objectref_rc_migrate_enabled()` (readonly). SharedCell Arc lifetime substrate **still present** (≠ RC dual-track；≠ `storage_cleanup_enabled`). This window does **not** flip `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈97–99%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `four-hundred-and-fifth` / `A-W555`.

## Index

| Prior | Status |
|-------|--------|
| W476–W554 narrative → post-mutrefcell-peel refresh | **met** |
| This window ObjectRef RC migrate product flip | **met** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-ii-m1. Post-mutrefcell-peel migrate-readiness refresh | Re-sign blockers after peel | **met** (W554) |
| 4c-ii-m2. ObjectRef RC migrate product flip | Flip `objectref_rc_migrate_enabled` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Flip `storage_cleanup_enabled` | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Product honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Migrate product flip signed | **met** | this window |
| `objectref_rc_migrate_enabled` | **yes** | true |
| `syx_rt::objectref_rc_migrate_enabled()` | **yes** | returns true |
| MutRefCell / ObjectRef peels | **met** | W553 / W550 |
| DictBuild heap-primary birth | **met** | W547 |
| ObjectRef RC storage stripped | **met** | W537 |
| Deep field-graph GC walk | **met** | W539 |
| SharedCell Arc lifetime | **still present** | ≠ RC；≠ cleanup |
| `storage_cleanup_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | migrate ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Migrate product flip signed | **yes** | This window |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | migrate ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateProductFirstSliceInventory` / `gate_b09_objectref_rc_migrate_product_first_slice_inventory` |
| API | `syx_rt::objectref_rc_migrate_enabled() -> bool` (reads this inventory) |
| Flags | `b09_objectref_rc_migrate_product_first_slice_signed=true`; `b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_floor_ok=true`; `migrate_flip_blockers_refreshed_after_mutrefcell_peel=true`; `mutrefcell_peel_met=true`; `reentry_maybe_track_peel_met=true`; `dual_track_collapse_met=true`; `dictbuild_objectref_heap_primary_birth_met=true`; `objectref_rc_storage_stripped=true`; `deep_field_graph_gc_walk_met=true`; `sharedcell_arc_lifetime_still_present=true`; `objectref_rc_migrate_enabled=true`; `migrate_product_flip_met=true`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming migrate as cleanup or enable |
| Honesty | migrate product flip **≠** storage cleanup / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W496–W554 historical inventory snapshots **not rewritten**（仍 `migrate=false`） |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateProductFirstSliceInventory` + `objectref_rc_migrate_enabled()`
- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectRef + MutRefCell `maybe_track_*` absent
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — ObjectRef + MutRefCell `maybe_track_*` absent
- Meta: `b09_objectref_rc_migrate_product_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-migrate-readiness-post-mutrefcell-peel-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-mutrefcell-peel-refresh-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-899 / fingerprint `A-W555` / `qs1-platform-gate-v0:driver-gate-exec-four-hundred-and-fifth`

## Frozen

- Flipping `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W496–W554 Contract table field claims as if current
- Claiming migrate product flip as cleanup / enable / `[x]`
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w554_*.plan.md`

## Next

W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558 B-09 axis exit first-slice（窄 GC GA）**已交**（ADR-902）→ W559+ — 扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
