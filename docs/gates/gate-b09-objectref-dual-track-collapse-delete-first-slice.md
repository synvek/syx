# B-09 ObjectRef dual-track collapse true-delete first-slice (W547 / ADR-891)

## Status

**B-09 ObjectRef dual-track collapse true-delete first-slice met** (product+inventory): after W546 dual-track collapse readiness, DictBuild ObjectRef Arc+`maybe_track_*` dual birth is **collapsed** to heap-primary single-track (`object_ref_from_map` → `alloc_heap_slot` when `ManagedHeap::enabled()`, no DictBuild-site `maybe_track_objectref_value`) in VM + mir-run. Re-entry `maybe_track_*` residual **noted** (CallHost/ObjectGet/…). MutRefCell dual-track **out of scope**. `objectref_rc_migrate_enabled` **still false**. This window does **not** flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈92–95%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-seventh` / `A-W547`.

## Index

| Prior | Status |
|-------|--------|
| W476–W546 narrative → dual-track collapse readiness | **met** |
| This window dual-track collapse true-delete first-slice | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … MirOp/VM peel + post-mirop refresh | Tracks → peel → post-mirop migrate-readiness refresh | **met** (W494–W545) |
| 4c-ii-d0. Dual-track collapse readiness | Blockers documented | **met** (W546) |
| 4c-ii-d. True dual-track collapse | Productize collapse of Arc + `maybe_track_*` DictBuild birth | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Collapse honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Dual-track collapse true-delete signed | **met** | this window |
| DictBuild ObjectRef dual-track birth | **deleted** | heap-primary via `alloc_heap_slot` |
| DictBuild-site `maybe_track_objectref_value` | **deleted** | no dual register at birth |
| Re-entry `maybe_track_*` residual | **noted** | CallHost/ObjectGet/… |
| MutRefCell dual-track | **out of scope** | frozen this window |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | collapse ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Dual-track collapse true-delete signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | collapse ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory` / `gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory` |
| Flags | `b09_objectref_dual_track_collapse_delete_first_slice_signed=true`; `b09_objectref_dual_track_collapse_readiness_thin_floor_ok=true`; `dictbuild_objectref_dual_track_birth_deleted=true`; `dictbuild_objectref_heap_primary_birth_met=true`; `dual_track_collapse_met=true`; `mirop_vm_residual_peel_met=true`; `objectref_reentry_maybe_track_residual_noted=true`; `mutrefcell_dual_track_out_of_scope=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming collapse as migrate met / MutRefCell collapse / rewriting W546 Contract `still_present` as if current |
| Honesty | dual-track collapse **met** **≠** migrate flip **≠** enable **≠** B-09 `[x]` **≠** GC GA；W518/W531/W535–W546 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) / [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — DictBuild ObjectRef heap-primary birth
- [`crates/syx-runtime-core/src/value.rs`](../../crates/syx-runtime-core/src/value.rs) — HeapSlot ObjectRef get/set write-back
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory`
- Meta: `b09_objectref_dual_track_collapse_delete_first_slice.rs`
- Floor: [`gate-b09-objectref-dual-track-collapse-readiness-thin.md`](gate-b09-objectref-dual-track-collapse-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-891 / fingerprint `A-W547` / `qs1-platform-gate-v0:driver-gate-exec-three-hundred-and-ninety-seventh`

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- MutRefCell dual-track collapse this window
- Rewriting W518/W531/W535–W546 Contract table field claims as if current
- Claiming collapse as migrate flip / enable
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w546_*.plan.md`

## Next

W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
