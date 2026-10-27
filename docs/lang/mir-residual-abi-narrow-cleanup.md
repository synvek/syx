# MIR residual ABI narrow cleanup (W484 / ADR-828)

> **Status:** **residual ABI narrow cleanup signed** (ADR-828 / W484) — production IR omits `declare @syx_rt_load_mir_blob`; production shim/C stub use compact-only (no dead sidecar encode).  
> Cross-index: [`mir-residual-honesty.md`](mir-residual-honesty.md) / ADR-803 (historical floor); [`long-mir-free-emit-omit.md`](long-mir-free-emit-omit.md) / W413; [`long-mir-free-sidecar-omit-emit.md`](long-mir-free-sidecar-omit-emit.md) / W431; [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).  
> Roadmap B-02 **`[x]`**; B-09 **`[~]`**. MIR blob distance **0** (unchanged — ≠ this window removed production blob).  
> **Not** RT ABI delete. **Not** scan-all auto / exports GA. **Not** B-09 `[x]`.

Fingerprint: `three-hundred-and-thirty-fourth` / `A-W484`.

## Scope narrow (ADR-828)

Residual ABI narrow cleanup **complete** (narrow) =
- W459 honesty signed three residuals present + cleanup off
- W413/W431 production omit blob/sidecar already met
- This window: omit IR declare + stop production dead encode; **retain** RT `syx_rt_load_mir_blob`

RT ABI full delete / B-09 unified heap / exports GA / scan-all auto = **unmet**.

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Residual honesty | **met** (W459 / ADR-803) | [`mir-residual-honesty.md`](mir-residual-honesty.md) |
| Emit omit / sidecar omit | **met** | long-mir-free-* omit docs |
| Narrow cleanup | **signed** (W484 / ADR-828) | this document + `MirResidualAbiNarrowCleanupInventory` |

**Evidence nails:** `crates/syx-driver/tests/native/mir_residual_abi_narrow_cleanup.rs` + `mir_residual_abi_narrow_cleanup_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `MirResidualAbiNarrowCleanupInventory` / `mir_residual_abi_narrow_cleanup_inventory` |
| Flags | `mir_residual_abi_narrow_cleanup_signed=true`; `residual_honesty_floor_ok=true`; `residual_cleanup_enabled=true`; `residual_ir_declare_omitted=true`; `residual_dead_sidecar_encode_omitted=true`; `residual_rt_abi_exported=true`; `rt_abi_delete_enabled=false`; `production_blob_embedded=false` |
| Honesty floor | W459 `MirResidualHonestyInventory` keeps `residual_ir_declare_present=true`; `residual_aot_compact_view_dead=true`; `residual_cleanup_enabled=false` |
| Forbidden claim | RT ABI delete / scan-all auto / exports GA / B-09 `[x]` / HostFn+1 / claiming this window set MIR distance from >0 to 0 |
| Honesty | narrow cleanup **≠** MIR blob distance change / **≠** RT ABI delete / **≠** B-09 `[x]` |

## Frozen

- Deleting `syx_rt_load_mir_blob` RT ABI / `rt_abi_delete_enabled=true`
- Claiming `language_exports_auto_discovery_enabled=true` / exports GA
- B-09 `[x]` / unified heap / GC GA
- Re-embedding `@syx_mir_blob` / `@syx_metadata_sidecar`
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w483_*.plan.md`

## Next

W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W516+ — 优先 LLVM·WASM RC 退役 → 全量迁 / 翻 storage cleanup enabled / 真 enable；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
