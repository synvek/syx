# B-09 LLVM ObjectRefWrap emit residual retire first-slice (W514 / ADR-858)

## Status

**B-09 LLVM ObjectRefWrap emit residual retire first-slice met**: after W513 ArrayGet heap_slots, LLVM no longer emits wrap RT call / wrap declare for `MirOp::ObjectRefWrap` (honest nop). WASM AOT ObjectRefWrap was already nop. RT ABI `syx_rt_object_ref_wrap` **still present** (routed via mir-run `OBJECT_REF_WRAP` + ManagedHeap track). Value / VM / mir-run ObjectRef RC **still live**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, cover nested ObjectSet/LoadGlobal dual-track, delete RT ABI, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-fourth` / `A-W514`.

## Index

| Prior | Status |
|-------|--------|
| W476–W513 narrative → ArrayGet heap_slots | **met** |
| This window LLVM ObjectRefWrap **emit-path** retire | **met** |
| WASM ObjectRefWrap AOT | **already nop** (inventoried) |
| ObjectSet MutRefCell write-back | **met** (W515 / ADR-859) |
| LoadGlobal dual-track | **still unmet** |
| Full ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-p | Tracks → ObjectGet/ArrayGet → Retain/Release emit retire | **met** (W494–W513) |
| 4c-ii-q. LLVM wrap emit residual retire | ObjectRefWrap no LLVM call / omit declare | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Emit-path retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| LLVM ObjectRefWrap | **emit retired** | nop comment; no wrap RT call; omit declare |
| WASM AOT ObjectRefWrap | **already nop** | no Call (pre-W514) |
| RT ABI `syx_rt_object_ref_wrap` | **still present** | mir-run routed; ≠ deleted |
| mir-run `object_ref_wrap` track | **still live** | ManagedHeap + heap_slots |
| VM / mir-run / Value RC | **still live** | ≠ RC delete |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory` / `gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory` |
| Flags | `b09_llvm_objectref_wrap_emit_retire_first_slice_signed=true`; `b09_objectref_arrayget_heap_slots_first_slice_floor_ok=true`; `llvm_objectref_wrap_call_emit_retired=true`; `llvm_objectref_wrap_declare_omitted=true`; `wasm_objectref_wrap_aot_already_nop=true`; `rt_abi_object_ref_wrap_still_present=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | nested full cover / Value RC deleted / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | wrap emit retire **≠** nested cover / **≠** Value RC delete / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-llvm/src/lib.rs`](../../crates/syx-llvm/src/lib.rs) — ObjectRefWrap nop / ADR-858
- [`crates/syx-wasm-emit/src/mir_emit.rs`](../../crates/syx-wasm-emit/src/mir_emit.rs) — AOT already nop / ADR-858
- [`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs) — RT ABI retained + mir-run route / ADR-858
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory`
- Meta: `b09_llvm_objectref_wrap_emit_retire_first_slice.rs`
- Floor: [`gate-b09-objectref-arrayget-heap-slots-first-slice.md`](gate-b09-objectref-arrayget-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI wrap or `syx_rt_load_mir_blob`
- Nested ObjectSet/LoadGlobal deep dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w513_*.plan.md`

## Next

W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519 B-09 RT ABI ObjectRefWrap retire-readiness thin **已交**（ADR-863）→ W520+ — 优先 RT ABI retain/release retire-readiness / Value/VM/mir-run RC **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
