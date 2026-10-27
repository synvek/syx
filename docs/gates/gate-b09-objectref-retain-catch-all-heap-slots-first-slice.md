# B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice (W510 / ADR-854)

## Status

**B-09 ObjectRefRetain catch-all ManagedHeap + heap_slots first-slice met**: after W509 MethodCall track, ObjectRefRetain and implicit retain surfaces (Dup / LoadLocal) dual-register stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and push `heap_slots` (identity-dedup). This window does **not** retire LLVM/WASM `@syx_rt_object_ref_*`, change MethodCall/ActorCall/DictBuild/CallHost/`finish_call`/`call_fn`, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixtieth` / `A-W510`.

## Index

| Prior | Status |
|-------|--------|
| W476–W509 narrative → tracks through MethodCall | **met** |
| This window ObjectRefRetain catch-all (VM+mir-run; Dup/LoadLocal) | **met** |
| LLVM·WASM ObjectRef RC emit-path retire | **met** (W511 / ADR-855) |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-m | Dead API → tracks → MethodCall | **met** (W494–W509) |
| 4c-ii-n. ObjectRefRetain catch-all | Retain / Dup / LoadLocal → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Retain catch-all contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM ObjectRefRetain → dual helpers | **wired** | After `object_ref_retain` |
| VM Dup / LoadLocal → dual helpers | **wired** | Implicit retain surface (W496) |
| mir-run OBJECT_REF_RETAIN / DUP / LOAD_LOCAL | **wired** | Parity with VM |
| MethodCall / ActorCall / DictBuild / CallHost / finish_call / call_fn | **retained** | Floor W497–W509 |
| LLVM/WASM Retain/Release **emit** | **retired** (W511)；RT ABI / Value RC **still live** | W512+ |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory` / `gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_retain_catch_all_heap_slots_first_slice_signed=true`; `b09_objectref_method_call_heap_slots_first_slice_floor_ok=true`; `vm_objectref_retain_managed_heap_registered=true`; `mir_run_objectref_retain_managed_heap_registered=true`; `vm_dup_loadlocal_objectref_managed_heap_registered=true`; `mir_run_dup_loadlocal_objectref_managed_heap_registered=true`; `vm_mir_run_objectref_retain_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC delete / MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | ObjectRefRetain catch-all **≠** Value RC delete / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectRefRetain + Dup + LoadLocal + dual helpers / ADR-854
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — OBJECT_REF_RETAIN + DUP + LOAD_LOCAL + dual helpers / ADR-854
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_retain_catch_all_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-method-call-heap-slots-first-slice.md`](gate-b09-objectref-method-call-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Value / VM / mir-run RC delete / full migrate / deleting live retain/release this window / changing RT ABI `syx_rt_object_ref_*`
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w509_*.plan.md`

## Next

W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
