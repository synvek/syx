# B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice (W523 / ADR-867)

## Status

**B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice met**: after W522 ObjectRefRetain opcode RC true-retire, VM `Instruction::Dup` / `LoadLocal` and mir-run `DUP` / `LOAD_LOCAL` **no longer** call `Value::object_ref_retain` (RC bump retired). W510 dual-track ManagedHeap + `heap_slots` registration **kept**. SSA LoadLocal already had no retain (W517). ObjectRefRelease / StoreLocal / deinit, `Value::object_ref_*` APIs, and RT ABI retain/release/wrap **still live**. This window does **not** delete Value RC APIs, strip Release/StoreLocal/deinit, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-third` / `A-W523`.

## Index

| Prior | Status |
|-------|--------|
| W476–W522 narrative → ObjectRefRetain opcode RC true-retire | **met** |
| This window Dup+LoadLocal ObjectRef RC true-retire | **met** |
| Release/StoreLocal/deinit RC → ManagedHeap/GC | **still unmet** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y1 | Tracks → ObjectRefRetain opcode RC retire | **met** (W494–W522) |
| 4c-ii-y2. Dup/LoadLocal RC retire | Stop RC bump on Dup + LoadLocal; keep dual-track | **met** (this window) |
| 4c-ii-y3. Release/deinit migrate | Release path → ManagedHeap/GC | **still unmet** |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Dup/LoadLocal retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM Dup / LoadLocal RC bump | **retired** | no `Value::object_ref_retain` |
| mir-run DUP / LOAD_LOCAL RC bump | **retired** | parity with VM |
| Dup/LoadLocal dual-track | **kept** | W510 `maybe_track_*` |
| SSA LoadLocal retain | **already none** | W517 floor |
| Release / StoreLocal / deinit ObjectRef RC | **still live** | `objectref_release_deinit_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory` / `gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory` |
| Flags | `b09_objectref_rc_dup_loadlocal_retire_first_slice_signed=true`; `b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok=true`; `b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok=true`; `vm_mir_run_dup_loadlocal_objectref_rc_retired=true`; `vm_mir_run_dup_loadlocal_dual_track_kept=true`; `objectref_release_deinit_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC full delete / Release strip / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change |
| Honesty | Dup/LoadLocal RC retire **≠** Value RC full delete / **≠** Release retire / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — Dup/LoadLocal RC bump retired / ADR-867
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — DUP/LOAD_LOCAL RC bump retired / ADR-867
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory`
- Meta: `b09_objectref_rc_dup_loadlocal_retire_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-retain-opcode-retire-first-slice.md`](gate-b09-objectref-rc-retain-opcode-retire-first-slice.md), [`gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md`](gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `Value::object_ref_*`; stripping ObjectRefRelease / StoreLocal / deinit; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w522_*.plan.md`

## Next

W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524 B-09 ObjectRefRelease opcode RC true-retire first-slice **已交**（ADR-868）→ W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice **已交**（ADR-869）→ W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*`（慎）→ 删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
