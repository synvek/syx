# B-09 ObjectRef dual-track collapse readiness thin (W546 / ADR-890)

## Status

**B-09 ObjectRef dual-track collapse readiness thin met** (policy+inventory): after W545 post-mirop-peel migrate-readiness refresh, dual-track collapse readiness is **signed** — blockers documented while DictBuild Arc ObjectRef birth + `maybe_track_*` dual-track **still present**. Handle-table + mir-run opcode peel **met** (W541). MirOp/VM residual peel **met** (W544). Post-mirop-peel migrate-readiness refresh **met** (W545). Dual-track collapse **still unmet**. `objectref_rc_migrate_enabled` **still false**. This window does **not** collapse DictBuild dual-track, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈90–93%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-sixth` / `A-W546`.

## Index

| Prior | Status |
|-------|--------|
| W476–W545 narrative → post-mirop-peel migrate-readiness refresh | **met** |
| This window dual-track collapse readiness thin | **met** |
| True dual-track collapse (DictBuild Arc + `maybe_track_*` product collapse) | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … MirOp/VM peel + post-mirop refresh | Tracks → peel → post-mirop migrate-readiness refresh | **met** (W494–W545) |
| 4c-ii-d0. Dual-track collapse readiness | Blockers documented; true collapse still unmet | **met** (this window) |
| 4c-ii-d. True dual-track collapse | Productize collapse of Arc + `maybe_track_*` dual-track | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Collapse honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Dual-track collapse readiness thin signed | **met** | this window |
| DictBuild ObjectRef dual-track birth (`object_ref_from_map` Arc) | **still present** | collapse target |
| `maybe_track_*` dual-register | **still present** | collapse target |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| True dual-track collapse | **still unmet** | readiness ≠ collapse |
| Handle-table + mir-run opcode peel | **met** | W541 |
| MirOp/VM residual peel | **met** | W544 |
| Post-mirop-peel migrate-readiness refresh | **met** | W545 |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Dual-track collapse readiness signed | **yes** | This window |
| True dual-track collapse completed | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefDualTrackCollapseReadinessThinInventory` / `gate_b09_objectref_dual_track_collapse_readiness_thin_inventory` |
| Flags | `b09_objectref_dual_track_collapse_readiness_thin_signed=true`; `b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_floor_ok=true`; `handle_table_mir_run_opcode_peel_met=true`; `mirop_vm_residual_peel_met=true`; `migrate_flip_blockers_refreshed_after_mirop_peel=true`; `dictbuild_objectref_dual_track_birth_still_present=true`; `dual_track_and_heap_coexistence_noted=true`; `dual_track_collapse_still_unmet=true`; `dual_track_collapse_blockers_documented=true`; `object_ref_from_map_arc_birth_noted=true`; `maybe_track_objectref_dual_register_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting DictBuild dual-track / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming readiness as collapse met |
| Honesty | dual-track collapse readiness **≠** dual-track collapse met **≠** migrate flip **≠** enable **≠** B-09 `[x]` **≠** GC GA；W518/W531/W535–W545 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-runtime-core/src/value.rs`](../../crates/syx-runtime-core/src/value.rs) — `object_ref_from_map` Arc birth
- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) / [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `maybe_track_*` dual-register
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefDualTrackCollapseReadinessThinInventory`
- Meta: ~~`b09_objectref_dual_track_collapse_readiness_thin.rs`~~ retired（F-10 W605）；proof = this SSOT + `GateB09ObjectrefDualTrackCollapseReadinessThinInventory`
- Floor: [`gate-b09-objectref-rc-migrate-readiness-post-mirop-peel-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-mirop-peel-refresh-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-890 / fingerprint `A-W546` / `qs1-platform-gate-v0:driver-gate-exec-three-hundred-and-ninety-sixth`

## Frozen

- Deleting DictBuild Arc + `maybe_track_*` dual-track this window
- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W545 Contract table field claims as if current
- Claiming collapse readiness as true collapse / migrate flip / enable
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w545_*.plan.md`

## Next

W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
