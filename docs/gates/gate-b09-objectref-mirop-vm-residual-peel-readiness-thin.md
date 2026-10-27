# B-09 ObjectRef MirOp/VM residual peel readiness thin (W543 / ADR-887)

## Status

**B-09 ObjectRef MirOp/VM residual peel readiness thin met** (policy+inventory): after W542 post-peel migrate-readiness refresh, MirOp/VM residual peel readiness is **signed** — blockers documented while `MirOp::ObjectRefWrap/Retain/Release` and VM `ObjectRefRetain/Release` **still present**. Handle-table + mir-run opcode peel **met** (W541). Post-peel migrate-readiness refresh **met** (W542). `objectref_rc_migrate_enabled` **still false**. This window does **not** delete MirOp/VM ObjectRef residual, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈86–89%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-third` / `A-W543`.

## Index

| Prior | Status |
|-------|--------|
| W476–W542 narrative → post-peel migrate-readiness refresh | **met** |
| This window MirOp/VM residual peel readiness thin | **met** |
| True MirOp/VM residual peel (enum/instruction delete) | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … peel true-delete + post-peel refresh | Tracks → peel → migrate-readiness refresh | **met** (W494–W542) |
| 4c-ii-r0. MirOp/VM residual peel readiness | Blockers documented; true peel still unmet | **met** (this window) |
| 4c-ii-r. True MirOp/VM residual peel | Delete MirOp enum + VM instructions | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Peel honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| MirOp/VM residual peel readiness thin signed | **met** | this window |
| `MirOp::ObjectRefWrap/Retain/Release` | **still present** | peel target |
| VM `ObjectRefRetain/Release` | **still present** | peel target |
| True MirOp/VM residual peel | **still unmet** | readiness ≠ peel |
| Wrap via MirOp dual-track (no handle table) | **noted** | `object_ref_wrap_via_mir_op_dual_track_noted` |
| Retain/Release RC-retired shells | **noted** | `mir_op_vm_objectref_retain_release_shells_noted` |
| Handle-table + mir-run opcode peel | **met** | W541 floor |
| Post-peel migrate-readiness refresh | **met** | W542 floor |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| MirOp/VM residual peel readiness signed | **yes** | This window |
| True MirOp/VM residual peel completed | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory` / `gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory` |
| Flags | `b09_objectref_mirop_vm_residual_peel_readiness_thin_signed=true`; `b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_floor_ok=true`; `handle_table_mir_run_opcode_peel_met=true`; `migrate_flip_blockers_refreshed_after_peel=true`; `mir_op_objectref_wrap_retain_release_still_present=true`; `vm_instruction_objectref_retain_release_still_present=true`; `mirop_vm_residual_peel_still_unmet=true`; `mirop_vm_residual_peel_blockers_documented=true`; `object_ref_wrap_via_mir_op_dual_track_noted=true`; `mir_op_vm_objectref_retain_release_shells_noted=true`; `dual_track_and_heap_coexistence_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting MirOp/VM ObjectRef residual / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming readiness as peel met |
| Honesty | MirOp/VM residual peel readiness **≠** true peel / **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W542 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) — `MirOp::ObjectRef*` still present
- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `ObjectRefRetain/Release` still present
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `exec_mir_op` ObjectRef arms (Wrap dual-track; Retain/Release shells)
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory`
- Meta: ~~`b09_objectref_mirop_vm_residual_peel_readiness_thin.rs`~~ retired（F-10 W602）；proof = this SSOT + `GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory`
- Floor: [`gate-b09-objectref-rc-migrate-readiness-post-peel-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-peel-refresh-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `MirOp::ObjectRef*` enum / VM `ObjectRefRetain/Release` instructions this window
- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W542 Contract table field claims as if current
- Claiming peel readiness as true peel / migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w542_*.plan.md`

## Next

W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
