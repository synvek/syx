# B-09 ObjectRef MirOp/VM residual peel true-delete first-slice (W544 / ADR-888)

## Status

**B-09 ObjectRef MirOp/VM residual peel true-delete first-slice met**: after W543 MirOp/VM residual peel readiness, `MirOp::ObjectRefWrap/Retain/Release` and VM `Instruction::ObjectRefRetain/Release` are **deleted**; mir-run `exec_mir_op` ObjectRef arms + `object_ref_wrap` helper stripped; codegen/LLVM/wasm ObjectRef residual peeled. DictBuild ObjectRef dual-track birth (`object_ref_from_map` + `maybe_track_*`) **kept**. Handle-table + mir-run opcode peel **met** (W541). Post-peel migrate-readiness refresh **met** (W542). MirOp/VM residual peel readiness **met** (W543). `objectref_rc_migrate_enabled` **still false**. This window does **not** flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈88–91%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-fourth` / `A-W544`.

## Index

| Prior | Status |
|-------|--------|
| W476–W543 narrative → MirOp/VM residual peel readiness | **met** |
| This window MirOp/VM residual peel true-delete | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … peel true-delete + post-peel refresh | Tracks → peel → migrate-readiness refresh | **met** (W494–W542) |
| 4c-ii-r0. MirOp/VM residual peel readiness | Blockers documented | **met** (W543) |
| 4c-ii-r. True MirOp/VM residual peel | Delete MirOp enum + VM instructions | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Peel honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| MirOp/VM residual peel true-delete signed | **met** | this window |
| `MirOp::ObjectRefWrap/Retain/Release` | **deleted** | peel target |
| VM `ObjectRefRetain/Release` | **deleted** | peel target |
| mir-run `object_ref_wrap` helper | **deleted** | peel companion |
| DictBuild ObjectRef dual-track birth | **kept** | live wrap path |
| Handle-table + mir-run opcode peel | **met** | W541 floor |
| Post-peel migrate-readiness refresh | **met** | W542 floor |
| MirOp/VM residual peel readiness | **met** | W543 floor |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | peel ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| MirOp/VM residual peel true-delete signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | peel ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory` / `gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory` |
| Flags | `b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed=true`; `b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok=true`; `handle_table_mir_run_opcode_peel_met=true`; `migrate_flip_blockers_refreshed_after_peel=true`; `mir_op_objectref_wrap_retain_release_deleted=true`; `vm_instruction_objectref_retain_release_deleted=true`; `mirop_vm_residual_peel_met=true`; `dictbuild_objectref_dual_track_birth_kept=true`; `object_ref_wrap_helper_deleted=true`; `dual_track_and_heap_coexistence_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming peel as migrate / rewriting W543 Contract `still_present` as if current |
| Honesty | MirOp/VM residual peel met **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W543 historical inventory snapshots **not rewritten** |

## Evidence

- Deleted: `MirOp::ObjectRefWrap/Retain/Release` in [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) + mir_text
- Deleted: VM `Instruction::ObjectRefRetain/Release` in [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) + codegen
- Deleted: mir-run `exec_mir_op` ObjectRef arms + `object_ref_wrap` in [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs)
- Kept: DictBuild + `object_ref_from_map` + `maybe_track_*` dual-track birth
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory`
- Meta: `b09_objectref_mirop_vm_residual_peel_delete_first_slice.rs`
- Floor: [`gate-b09-objectref-mirop-vm-residual-peel-readiness-thin.md`](gate-b09-objectref-mirop-vm-residual-peel-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W543 Contract table field claims as if current
- Claiming MirOp/VM peel as migrate flip / enable / `[x]`
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w543_*.plan.md`

## Next

W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
