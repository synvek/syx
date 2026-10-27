# MIR residual honesty first-slice (W459 / ADR-803)

## Status

**MIR residual honesty first-slice met** (policy+inventory): production native already omits `@syx_mir_blob` / `@syx_metadata_sidecar` and `syx_user_init` does not call `syx_rt_load_mir_blob`; residual IR declare / RT ABI / dead encode were **honestly signed** at W459 — **not** deleted that window. HostFn count **554**; B-07 **`[x]`** (narrow) unchanged; ≠ language exports / ≠ daemon GA / ≠ delete ABI.

**Follow-on:** W484 / ADR-828 residual ABI **narrow cleanup** omits live IR declare + production dead encode while **retaining** RT ABI; historical inventory floors below **unchanged**. See [`mir-residual-abi-narrow-cleanup.md`](mir-residual-abi-narrow-cleanup.md).

Fingerprint: `three-hundred-and-ninth` / `A-W459`.

## Index

| Prior | Status |
|-------|--------|
| W413 Long emit-omit / W431 sidecar-omit emit | **met** (production no blob/sidecar) |
| W432–W434 MIR-free / named stub / delete flag | **met** (narrative) |
| Live tip honesty (W434+/W452) | **met** (scattered docs) |
| This window residual inventory rollup | **met** |
| Remove IR declare (live) | **met** (W484 / ADR-828 narrow cleanup) |
| Delete load RT ABI | **still unmet** (Frozen; B-05) |
| Language `Module.exports` → `aot_export_roots` | list **met** (W483); auto-discovery **still unmet** |

## Residual table (hard)

| Residual | W459 honesty inventory | Live after W484 |
|----------|------------------------|-----------------|
| IR `declare @syx_rt_load_mir_blob` | **yes** (floor) | **omitted** (narrow cleanup) |
| RT ABI `syx_rt_load_mir_blob` | **yes** | **yes** (retained) |
| `aot_compact_view` dead encode | **yes** (floor) | production path **omitted**; probe helper may remain |
| `@syx_mir_blob` / `@syx_metadata_sidecar` embed | **no** | **no** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `MirResidualHonestyInventory` / `mir_residual_honesty_inventory` |
| Flags | `residual_honesty_signed=true`; `production_blob_embedded=false`; `production_user_init_calls_load=false`; three residual bits **true**; `residual_cleanup_enabled=false` |
| Forbidden claim (W459) | Delete ABI/declare this window / language exports / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | Residual honesty **≠** residual cleanup / **≠** distance still >0 (distance already **0**) |

## Evidence

- [`crates/syx-rt/src/wave_b_horizon.rs`](../../crates/syx-rt/src/wave_b_horizon.rs) `MirResidualHonestyInventory`
- Meta: `mir_residual_honesty.rs`
- Cross: [`mir-residual-abi-narrow-cleanup.md`](mir-residual-abi-narrow-cleanup.md), [`long-mir-free-readiness.md`](long-mir-free-readiness.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- Source nails: [`syx-llvm`](../../crates/syx-llvm/src/lib.rs) (W484 omit declare); [`abi.rs`](../../crates/syx-rt/src/abi.rs) export retained

## Frozen

- Deleting `syx_rt_load_mir_blob` ABI (RT full clear) / claiming honesty window deleted declare
- Re-embedding `@syx_mir_blob` / `@syx_metadata_sidecar` in production emit
- Language `Module.exports` → MIR `aot_export_roots` automatic discovery
- Claiming daemon GA / Array alpn / thread-pool product API
- Adding HostFn (keep 554)
- Claiming production-default JIT
- Clearing historical W410–W433 inventory honesty strings
- Mutating W459 inventory floor flags (`residual_cleanup_enabled=false` etc.)

## Next

W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ … → W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W516+ — 优先 LLVM·WASM RC 退役 → 全量迁 / 翻 storage cleanup enabled / 真 enable；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
