# B-09 LLVM·WASM ObjectRefRetain/Release emit-path retire first-slice (W511 / ADR-855)

## Status

**B-09 LLVM·WASM ObjectRefRetain/Release emit-path retire first-slice met**: after W510 ObjectRefRetain catch-all, LLVM no longer emits `call @syx_rt_object_ref_retain/release`, and WASM AOT no longer imports/Calls `__syx_aot_object_ref_retain/release`. RT ABI symbols remain (routed via mir-run `OBJECT_REF_RETAIN/RELEASE`). Value / VM / mir-run ObjectRef RC **still live**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, delete RT ABI, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-first` / `A-W511`.

## Index

| Prior | Status |
|-------|--------|
| W476–W510 narrative → ObjectRefRetain catch-all | **met** |
| This window LLVM·WASM Retain/Release **emit-path** retire | **met** |
| Value / VM / mir-run RC full delete | **still unmet** |
| ObjectGet residual re-entry + birth inventory | **met** (W512 / ADR-856) |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-n | Dead API → tracks → ObjectRefRetain catch-all | **met** (W494–W510) |
| 4c-ii-o. LLVM·WASM emit-path retire | Retain/Release no LLVM call / no WASM AOT Call | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Emit-path retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| LLVM ObjectRefRetain/Release | **emit retired** | nop comment; no `call @syx_rt_object_ref_*` |
| WASM AOT ObjectRefRetain/Release | **emit retired** | no import plan / no Call |
| RT ABI `syx_rt_object_ref_retain/release` | **still present** | mir-run routed; ≠ deleted |
| WASM host `func_wrap` | **may remain** | old modules / ABI stability |
| VM / mir-run / Value RC | **still live** | W510 catch-all retained |
| `object_ref_wrap` LLVM emit | **retired** (W514 / ADR-858) | nop + omit declare |
| `object_ref_wrap` WASM AOT | **already nop** | inventoried W514 |
| RT ABI / mir-run wrap | **still present** | ≠ deleted |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory` / `gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory` |
| Flags | `b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed=true`; `b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok=true`; `llvm_objectref_retain_release_call_emit_retired=true`; `wasm_objectref_retain_release_aot_call_emit_retired=true`; `llvm_wasm_objectref_retain_release_emit_retired=true`; `rt_abi_object_ref_retain_release_still_present=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC deleted / MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | emit-path retire **≠** Value RC delete / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-llvm/src/lib.rs`](../../crates/syx-llvm/src/lib.rs) — ObjectRefRetain/Release nop / ADR-855
- [`crates/syx-wasm-emit/src/mir_emit.rs`](../../crates/syx-wasm-emit/src/mir_emit.rs) — AOT import/Call omit / ADR-855
- [`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs) — RT ABI retained + mir-run route / ADR-855
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory`
- Meta: `b09_llvm_wasm_objectref_rc_emit_retire_first_slice.rs`
- Floor: [`gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md`](gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI `syx_rt_object_ref_*` or `syx_rt_load_mir_blob`
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w510_*.plan.md`

## Next

W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519 B-09 RT ABI ObjectRefWrap retire-readiness thin **已交**（ADR-863）→ W520 B-09 RT ABI ObjectRefRetain/Release retire-readiness thin **已交**（ADR-864）→ W521 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin **已交**（ADR-865）→ W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524+ — 优先 Release/StoreLocal/deinit **迁 ManagedHeap/GC**（分窗）→ Value API 删 → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
