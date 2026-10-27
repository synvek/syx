# B-09 ObjectRefRetain opcode RC true-retire first-slice (W522 / ADR-866)

## Status

**B-09 ObjectRefRetain opcode RC true-retire first-slice met**: after W521 Value RC retire-readiness, VM `Instruction::ObjectRefRetain` and mir-run `OBJECT_REF_RETAIN` **no longer** call `Value::object_ref_retain` (RC bump retired). W510 dual-track ManagedHeap + `heap_slots` registration **kept**. Dup/LoadLocal retain, ObjectRefRelease / StoreLocal / deinit, `Value::object_ref_*` APIs, and RT ABI retain/release/wrap **still live**. This window does **not** delete Value RC APIs, strip Dup/LoadLocal/Release, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-second` / `A-W522`.

## Index

| Prior | Status |
|-------|--------|
| W476–W521 narrative → Value RC retire-readiness | **met** |
| This window ObjectRefRetain opcode RC true-retire | **met** |
| Dup/LoadLocal ObjectRef RC true retire | **still unmet** |
| Release/deinit RC → ManagedHeap/GC | **still unmet** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-x | Tracks → Value RC retire-readiness | **met** (W494–W521) |
| 4c-ii-y1. ObjectRefRetain opcode RC retire | Stop RC bump on explicit retain opcode; keep dual-track | **met** (this window) |
| 4c-ii-y2. Dup/LoadLocal RC retire | Stop implicit retain RC | **still unmet** |
| 4c-ii-y3. Release/deinit migrate | Release path → ManagedHeap/GC | **still unmet** |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Retain opcode retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM `ObjectRefRetain` RC bump | **retired** | no `Value::object_ref_retain` |
| mir-run `OBJECT_REF_RETAIN` RC bump | **retired** | parity with VM |
| Retain opcode dual-track | **kept** | W510 `maybe_track_*` |
| Dup / LoadLocal ObjectRef RC | **still live** | `dup_loadlocal_objectref_rc_still_live` |
| Release / deinit ObjectRef RC | **still live** | `objectref_release_deinit_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory` / `gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory` |
| Flags | `b09_objectref_rc_retain_opcode_retire_first_slice_signed=true`; `b09_objectref_rc_retire_readiness_thin_floor_ok=true`; `vm_mir_run_objectref_retain_opcode_rc_retired=true`; `vm_mir_run_objectref_retain_opcode_dual_track_kept=true`; `dup_loadlocal_objectref_rc_still_live=true`; `objectref_release_deinit_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC full delete / Dup·LoadLocal RC delete / Release strip / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change |
| Honesty | opcode retain RC retire **≠** Value RC full delete / **≠** Dup/LoadLocal retire / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `ObjectRefRetain` RC bump retired / ADR-866
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `OBJECT_REF_RETAIN` RC bump retired / ADR-866
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory`
- Meta: `b09_objectref_rc_retain_opcode_retire_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-retire-readiness-thin.md`](gate-b09-objectref-rc-retire-readiness-thin.md), [`gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md`](gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `Value::object_ref_*`; stripping Dup/LoadLocal retain; stripping ObjectRefRelease / StoreLocal / deinit; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w521_*.plan.md`

## Next

W521 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin **已交**（ADR-865）→ W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524 B-09 ObjectRefRelease opcode RC true-retire first-slice **已交**（ADR-868）→ W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice **已交**（ADR-869）→ W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*`（慎）→ 删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
