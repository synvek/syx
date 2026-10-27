> **最近更新：** 2026-09-16 — Wave A/B 轴收官；B-09 `[x]`（窄 GC GA / W558 / ADR-902）；HostFn 554；MIR distance 0；规划 SSOT 已精简见 `plan/01`+`plan/02`；后续优先 Wave F 卫生。  
> **历史窗日志：** 详见 [`plan/archive/02-improvement-roadmap-pre-w559-ledger.md`](../../plan/archive/02-improvement-roadmap-pre-w559-ledger.md)（冻结）。
> **最近更新：** 2026-09-16 · W542 / ADR-886 B-09 ObjectRef RC migrate-readiness post-peel refresh thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；post-peel migrate-readiness refresh **met**（≠ migrate flip ≠ MirOp/VM peel ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-16 · W541 / ADR-885 B-09 ObjectRef peel 表·opcodes true-delete first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；peel true-delete **met**（≠ MirOp/VM peel ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-16 · W540 / ADR-884 B-09 ObjectRef peel 表·opcodes readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；peel readiness **met**（≠ peel met ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-16 · W539 / ADR-883 B-09 ObjectRef deep field-graph GC walk true first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；deep-walk true first-slice **met**（≠ peel ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-16 · W538 / ADR-882 B-09 ObjectRef deep field-graph GC walk readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；deep-walk readiness **met**（≠ deep walk met ≠ peel ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W537 / ADR-881 B-09 ObjectRef strip-RC true-delete first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；strip-RC true-delete **met**（≠ peel ≠ deep walk ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W536 / ADR-880 B-09 ObjectRef strip-RC readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；strip-RC readiness **met**（≠ strip RC ≠ peel ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W535 / ADR-879 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；post-handle-delete refresh **met**（≠ migrate flip ≠ strip RC ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W534 / ADR-878 B-09 RT ABI handle retain·release·drop true-delete first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；handle C ABI **deleted**（≠ peel 表 ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W533 / ADR-877 B-09 RT ABI handle retain·release·drop retire-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；handle retire-readiness **met**（≠ handle ABI 真删 ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W532 / ADR-876 B-09 ObjectRef nested field dual-track first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；nested write-path dual-track **met**（≠ deep walk ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W532 / ADR-876 B-09 ObjectRef nested field dual-track first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；nested write-path dual-track **met**（≠ deep field-graph GC walk ≠ migrate flip ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W531 / ADR-875 B-09 ObjectRef RC migrate-readiness refresh thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；migrate-readiness refresh **met**（≠ migrate flip ≠ enable ≠ B-09 `[x]`；W518 stale refreshed）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W530 / ADR-874 B-09 RT ABI wrap·retain·release true-delete first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；RT opcode ABI delete **met**（≠ handle ABI 删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W529 / ADR-873 B-09 Value ObjectRef RC API delete first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；Value RC API delete **met**（≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W528 / ADR-872 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；StoreLocal/deinit RC retire **met**（≠ Value RC 全删 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W527 / ADR-871 B-09 GC↔Type_deinit sweep-finalizer first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；collect sweep Type_deinit **met**（≠ StoreLocal/deinit RC 真退役 ≠ Value RC 全删 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W526 / ADR-870 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；deinits heap unroot **met**（≠ StoreLocal/deinit RC 真退役 ≠ Value RC 全删 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W525 / ADR-869 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；StoreLocal heap unroot **met**（≠ StoreLocal RC 真退役 ≠ deinit 退役 ≠ Value RC 全删 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W524 / ADR-868 B-09 ObjectRefRelease opcode RC true-retire first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；Release opcode RC retire **met**（≠ Value RC 全删 ≠ StoreLocal·deinit 退役 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W523 / ADR-867 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；Dup/LoadLocal RC retire **met**（≠ Value RC 全删 ≠ Release 退役 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W522 / ADR-866 B-09 ObjectRefRetain opcode RC true-retire first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；retain opcode RC retire **met**（≠ Value RC 全删 ≠ Dup/LoadLocal 退役 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W521 / ADR-865 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；Value RC retire-readiness **met**（≠ Value RC 真删 ≠ ABI 真删 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W520 / ADR-864 B-09 RT ABI ObjectRefRetain/Release retire-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；retain/release retire-readiness **met**（≠ ABI 真删 ≠ Value RC 退役 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-15 · W519 / ADR-863 B-09 RT ABI ObjectRefWrap retire-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；wrap retire-readiness **met**（≠ wrap ABI 真删 ≠ retain/release 退役 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W518 / ADR-862 B-09 ObjectRef RC migrate-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；migrate-readiness **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W517 / ADR-861 B-09 SSA LoadLocal ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；SSA LoadLocal **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W516 / ADR-860 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；LoadGlobal **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]` ≠ SSA LoadLocal）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W515 / ADR-859 B-09 ObjectSet MutRefCell write-back ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ObjectSet **met**（≠ LoadGlobal ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W514 / ADR-858 B-09 LLVM ObjectRefWrap emit residual 退役；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；wrap emit **met**（≠ nested write-back ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W513 / ADR-857 B-09 ObjectRef ArrayGet ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ArrayGet **met**（≠ nested/wrap emit ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W512 / ADR-856 B-09 ObjectRef ObjectGet residual re-entry + birth inventory；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ObjectGet residual **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W511 / ADR-855 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；emit-path retire **met**（≠ Value RC 删除 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W510 / ADR-854 B-09 ObjectRefRetain catch-all（VM+mir-run；Dup/LoadLocal）ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ObjectRefRetain catch-all **met**（≠ LLVM·WASM RC 退役 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W509 / ADR-853 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；MethodCall track **met**（≠ ObjectRefRetain ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W508 / ADR-852 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ActorCall runtime track **met**（≠ MethodCall ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W507 / ADR-851 B-09 ObjectRef+MutRefCell mir-run `call_fn`/`call_closure` ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；mir-run call-return track **met**（≠ ActorCall/MethodCall ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W506 / ADR-850 B-09 ObjectRef+MutRefCell VM `finish_call` ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；VM call-return track **met**（≠ mir-run call-return ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W505 / ADR-849 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；MutRefCell mir-run CallHost track **met**（≠ MutRefCell 全量迁 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W502 / ADR-846 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；MutRefCell VM DictBuild track **met**（≠ MutRefCell 全量迁 ≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-14 · W501 / ADR-845 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；mir-run CallHost ObjectRef track **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W500 / ADR-844 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；CallHost ObjectRef track **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W499 / ADR-843 B-09 ObjectRef mir-run DictBuild ManagedHeap align；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；mir-run DictBuild ManagedHeap + `heap_slots` **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W498 / ADR-842 B-09 ObjectRef VM heap_slots；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；DictBuild `heap_slots` track/collect/compact **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W497 / ADR-841 B-09 ObjectRef VM ManagedHeap probe；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；DictBuild ManagedHeap dual-register **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W496 / ADR-840 B-09 ObjectRef RC migrate first-slice（窄）；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；MemoryPool type stub retired + migrate surface **met**（≠ full migrate ≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W495 / ADR-839 B-09 MemoryPool dead-shell first-slice；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；alloc/pool retired **met**（≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W494 / ADR-838 B-09 storage-cleanup first-slice（死 API 退役）；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；dead-API retire **met**（≠ cleanup enabled ≠ enable ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W493 / ADR-837 B-09 enable-readiness thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；enable-readiness **met**（≠ enable ≠ cleanup ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-13 · W492 / ADR-836 B-09 ObjectRef/MemoryPool storage honesty thin；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；storage honesty thin **met**（≠ enable ≠ cleanup ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W486 / ADR-830 `export {…}` 尾逗号；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；trailing comma **met**（≠ 扫全 auto ≠ exports GA ≠ B-09 enable）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W485 / ADR-829 B-09 GC narrative deepen；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；立项清单 **met**（≠ 统一堆开关 ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W484 / ADR-828 residual ABI 窄清理；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；IR declare/死 encode **omitted**（RT ABI 保留；≠ MIR 距离变化）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W483 / ADR-827 `export {…}` 列表首片；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；ExportList→roots **met**（≠ 扫全 auto ≠ exports GA）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W482 / ADR-826 删 `compile`/`compile_into*`；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；公开双轨入口 **retired**（≠ auto-discovery ≠ B-09 `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W481 / ADR-825 semantic→`aot_export_roots`；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；Export→roots fill **met**（≠ auto-discovery ≠ exports GA）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W480 / ADR-824 Decl::Export 首片；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；lexer/AST/parser Export **met**（≠ semantic roots ≠ exports GA）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W479 / ADR-823 B-02 窄口径 `[x]` exit；HostFn **554**；Wave B 轴 **B-01～B-08 `[x]`** + B-09 **`[~]`**；narrow exit **met**（≠ 删 API ≠ retired）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W478 / ADR-822 B-02 严 deprecate gate；HostFn **554**；Wave B 轴 **B-01+B-03～B-08 `[x]`** + B-02+B-09 **`[~]`**；严 gate **met**（≠ retired ≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W477 / ADR-821 B-02 compile_into 首片；HostFn **554**；Wave B 轴 **B-01+B-03～B-08 `[x]`** + B-02+B-09 **`[~]`**；首片 **met**（≠ retired ≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W476 / ADR-820 B-09 GC narrative thin；HostFn **554**；Wave B 轴 **B-01+B-03～B-08 `[x]`** + B-02+B-09 **`[~]`**；narrative thin **met**（≠ 统一堆 ≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W475 / ADR-819 B-02 retire-readiness；HostFn **554**；Wave B 轴 **B-01+B-03～B-08 `[x]`** + B-02 **`[~]`** + B-09 **`[ ]`**；retire-readiness **met**（≠ retired ≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-12 · W472 / ADR-816 B-01 Actor→MIR shell skip 真迁；HostFn **554**；Wave B 轴 **B-03～B-08 `[x]`** + B-01～B-02 **`[~]`** + B-09 **`[ ]`**；残差簇 **11**；无活跃 AST 壳；≠ B-01 `[x]` / ≠ Decl::Export；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W471 / ADR-815 B-01 Class→MIR shell skip 真迁；HostFn **554**；Wave B 轴 **B-03～B-08 `[x]`** + B-01～B-02 **`[~]`** + B-09 **`[ ]`**；残差簇 **12**；Actor 仍 AST；≠ B-01 `[x]` / ≠ Actor / ≠ Decl::Export；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W463 / ADR-807 B-01 MIR-consumer thin-open；HostFn **554**；Wave B 轴 **B-03～B-08 `[x]`** + B-01～B-02 **`[~]`** + B-09 **`[ ]`**；≠ B-01 `[x]` / ≠ Decl::Export；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W462 / ADR-806 language exports syntax-spec deepen；HostFn **554**；Wave B 轴 **B-03+B-04+B-05+B-06+B-07+B-08 `[x]`** + B-01～B-02 **`[~]`** + B-09 **`[ ]`**；≠ parser / ≠ auto-discovery；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W461 / ADR-805 B-03 FrontendPipeline 入口矩阵；HostFn **554**；Wave B 轴 **B-03+B-04+B-05+B-06+B-07+B-08 `[x]`** + B-01～B-02 **`[~]`** + B-09 **`[ ]`**；≠ Pipeline API 变更 / ≠ frontend exports；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W455 / ADR-799 B-07 TcpHandleRelease fifth-slice wire；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen+五切片 wire；HostFn-bound POP 已清；≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-11 · W454 / ADR-798 B-07 TcpWrite fourth-slice wire；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen+HttpGet+FileRead+TcpRead+TcpWrite wire；≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W453 / ADR-797 B-07 TcpRead third-slice wire；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen+HttpGet+FileRead+TcpRead wire；≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W452 / ADR-796 B-07 FileRead second-slice wire；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen+HttpGet+FileRead wire；≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W451 / ADR-795 B-07 HttpGet first-slice wire；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen+HttpGet wire；≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W450 / ADR-794 B-07 deepen thin；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（thin+deepen；≠ `[x]` / ≠ wire）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W449 / ADR-793 B-07 thin-open；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`** + B-07 **`[~]`**（≠ `[x]`）；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W448 / ADR-792 B-06 Missing/align `[x]` exit；HostFn **554**；Wave B 轴 **B-04+B-05+B-06+B-08 `[x]`**；B-07 仍 unmet；生产 native 无 MIR blob/sidecar（距离 **0**）
> **最近更新：** 2026-09-10 · W447 / ADR-791 B-05 Medium `[x]` exit；HostFn **554**；Wave B 轴 **B-04+B-05+B-08 `[x]`**；B-06 仍 `[~]`；生产 native 无 MIR blob/sidecar（距离 **0**）
# SYX 语言特性实现状态

> **文档策略：** 本表只记录 **当前** 语言/工具链能力。历史 Phase 营销段与诚实收口段已移除；演进细节见 Git 历史与 [architecture-decisions.md](../../docs/adr/architecture-decisions.md)。

## 当前版本

| 项 | 值 |
|----|-----|
| 语言 / 营销版本 | **SYX 117.0** |
| Manifest phase | **139** |
| MIR / 增量缓存 | **`syx-cache-v1`** |
| 行为化 audit 下限 | driver **≥2600** · LSP **≥3050** — **smoke / anti-delete only**（Wave A W1 / ADR-345 / C-10；**非**覆盖率或 release KPI） |
| 架构决议 | [architecture-decisions.md](../../docs/adr/architecture-decisions.md) |
| 特性总表 ground truth | 本文件 + `crates/syx-driver/tests/` · `cargo test -p syx-lsp` |

### Phase 139 能力摘要（已交付）

- **Gate0（已交付）：** P138 诚实收口；audit **2600/3050**；manifest 117.0/phase 139；ADR-131 + P76；**暂缓 SynUI**；**冻结 Farm/Dist v19**
- **Track L — Language Surface（部分 GA）：** multiline `"""` · tuple · nested path · closed range · Character · call labels · remove `sealed` docs
- **Track C（部分 GA）：** SIMD16 i8x16 all_true/any_true guest-v22 MVP
- **DX：** `driver_gate` CR≥430；`lsp_phase139_real` ≥58；`syx-cache-v1`

### Truth M0 W1–W2（ADR-132 · 已交）

- **闸门：** fmt 未知 AST **fail-closed 禁止写盘**；JIT **experimental-real-subset**（`SYX_JIT_TIER2` + `jit-tier2` Suite-1 Cranelift，默认关）；DateTime 历法 API 当时 **unsupported/preview**（后由 M0-B01 + ADR-222 替换窄核心；`datetime.calendar` overall=`partial` 因 native=noop）；WS/gRPC 叙事降级；llvm stub **WiredNoop**（POP）
- **热修：** Dict.remove 真删除；PBKDF2 四参数 experimental；TCP **integer handle** + `TcpStream`；Text 不再同时 retired+importable
- **文档/CI：** README 1.85 + LLVM sole；appendix 117.0；CI manifest 139/117.0

### Truth M0 W2–W4（ADR-133 · 已交）

- **Capability：** [`capabilities.toml`](../../packages/internal/capabilities.toml) → [capabilities-generated.md](capabilities-generated.md) · `syx doctor --capabilities`
- **Differential：** Suite-1/2 exact VM/MIR（WASM/native 诚实 skip 见 oracle）；CI 阻断
- **热修：** DateTime 窄 Gregorian/RFC3339；HTTP 禁止 `status=0` 成功
- **分桶：** semantic / golden / differential / fingerprint（fingerprint 辅门）

### Truth M0 W4–W6 Exit（ADR-134 · 已交）

- **Suite-1 复开：** arith / branch / call / string 四后端 exact；array/dict 诚实 skip（见 oracle-v0）
- **Suite-3/4：** HTTP loopback + connect-refused；SQLite 内存 COUNT/SELECT（VM+MIR）
- **M0-B07：** `FileHandle.readAllBytes`（原 `readMmap`）；S3-04 已改名收口 → ADR-241（≠ OS mmap）
- **EC-M0-1～5：** 全部勾选

### Truth M1 W1 FrontendPipeline（ADR-135 · 已交）

- **`syx-frontend`：** 唯一 `FrontendPipeline`（parse→macro→desugar→meta→async→semantic→mono→MIR）；`StopAfter::{Expanded,Prepared,Semantic,Mir}`
- **入口接线：** file/project/`build_mir_from_graph`/embed/LSP analyze/REPL/browser 走 Pipeline；embed 顺序与 file 一致；completion 软路径标明非 EC-M1-1
- **Fail-closed：** 用户路径不再吞 macro/meta/async；负例进 `frontend_pipeline_parity`
- **EC-M1-1：** 切片 `[~]`（MIR hash + 诊断码）；全入口矩阵 → 后续窗
- **冻结本窗：** 无 SynUI/Farm/SIMD/语言表面新 GA；无 HIR/JIT 主线

### Truth M1 W2 HIR + IR tables（ADR-136 · 已交）

- **`syx-hir`：** Suite-1 算术/字符串切片 HIR→MIR；与 FrontendPipeline MIR hash 对齐
- **MirOp/HostFn 表：** `gen_ir_capabilities.py` + TOML overrides；CI `--check`；`syx doctor --ir-capabilities`
- **EC-M1-3：** 切片 `[~]`（pilot MirOp：LLVM trap / mir-run `E_MIR_PILOT`）
- **B03 起步：** dataclass/enum 优先 MIR shells；`CodeGenerator::compile` deprecated
- **B02：** 仅方向（执行后端目标 = MIR + type metadata）
- **冻结本窗：** 无 Model 冻结全文；无 JIT/LSP/SynUI/Farm/SIMD/语言表面新 GA

### Truth M1 W3 Models（ADR-137/138/139 · 已交）

- **Memory Model v1（ADR-137）：** struct/value vs class/actor ObjectRef; ManagedHeap v4 LTS; weak hosts registered（语义钉未全绿）; borrow=static exclusive; **M1-C04** 禁新 GC 营销版本号
- **Error Model v1（ADR-138）：** typed failure contract; host fail-closed; M2 迁移清单 [`error_model_migration.toml`](../../packages/internal/error_model_migration.toml)
- **Concurrency Model v1（ADR-139）：** parent-linked cancel; TaskGroup MVP; Sendable C003; actor reentrancy **非 GA** → M3
- **EC-M1-2：** 切片 **`[~]`**（三 ADR + `model_v1_nails`）；完整 §9.1 语言钉 → 后续窗
- **M1-C01/C02/C03/C04：** ADR + 最小钉 **`[x]`**；stdlib Error 实现迁移 → M2
- **冻结本窗：** 无 JIT/LSP/SynUI/Farm/SIMD/全量 stdlib throws 迁移

### Truth M1 W4 JIT 真子集（ADR-140 · 已交）

- **Cranelift：** Suite-1 pilot MirOp 真 lowering（arith / locals / cmp / branch / call / return）；删除 instruction-count stub
- **VM：** `native_ready` 时实际调用函数指针；`jit_suite1_differential` exact
- **Deopt：** `Print`/`CallHost`/… → `UnsupportedMirOp(...)` + `deopt_count` / `jit_native_call_count`
- **默认：** opt-in（`SYX_JIT_TIER2=1` / `--jit`）；**非**生产默认
- **EC-M1-4：** 切片 **`[~]`**；M1-D01～D04 **`[x]`**
- **下一窗：** M1 W5 = M1-E LSP workspace thin slice
- **冻结本窗：** 无 LSP/Model 全冻结/SynUI/Farm/SIMD/默认开 JIT/性能 KPI

### Truth M1 W5 LSP workspace（ADR-141 · 已交）

- **Workspace：** `initialize` → `rootUri` / `workspaceFolders`；`workspace/symbol` 无 fixture 硬编码（空 root → `[]`）
- **Shape：** codeAction / inlayHint / references / workspaceSymbol **顶层数组**；`lsp_m1_w5` + CI `m1_w5_`
- **EC-M1-5：** 切片 **`[~]`**（CST 迁移未启动）
- **M1-E01/E02：** **`[x]`**；E03/E04/E05 未开
- **下一窗候选：** E03 cancellation/version **或** E04 CST formatter
- **冻结本窗：** 无 SynUI/Farm/SIMD/CST 全量/cancellation 主线；无 Phase audit 抬楼；不修 `basic_demo` SIMD

### Truth M1 W6 CST formatter（ADR-142 · 已交）

- **Lexer trivia：** `LineComment` / `BlockComment` / `Whitespace`；Parser 语义跳过
- **`syx-cst`：** 子集 `func name() { }` + trivia；`format_cst` 拼接 lexeme（byte-identical）
- **`syx-fmt`：** `format_source_cst` / `SYX_FMT_CST=1`；**默认**仍 AST + M0-A02 `F001`
- **EC-M1-5：** **`[x]`**（workspace W5 + CST started W6）
- **M1-E04：** **`[x]`**；全仓幂等 → M4-T04
- **下一窗：** M1 W7 = M1-E03 cancellation / document version
- **冻结本窗：** 无 E03/E05；无全语法 CST GA；无 SynUI/Farm/SIMD

### Truth M1 W7 LSP cancel + document version（ADR-143 · 已交）

- **DocumentState：** `text` + `version`；`didChange` 丢弃 `incoming <= stored`
- **`$/cancelRequest`：** 登记 id；抑制已取消请求的成功 `result`（单线程同步环，非抢占）
- **M1-E03：** **`[x]`**
- **M1 未退出：** EC-M1-1～4 仍 `[~]` — **本窗不构成 M1 里程碑退出**
- **冻结声明：** 无 E05；无 CST 扩面；无 EC-1～4 全量晋升；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M1 W8 = M1 exit thin charter（ADR-144）：晋升 EC-M1-1～4 切片退出定义（诚实 defer 全量矩阵）；**不**默认开 E05

### Truth M1 W8 exit（ADR-144 · 已交）

- **EC-M1-1～5：** 全部 **`[x]`**（切片定义签署，非全量矩阵）
- **CI：** `model_v1_nails` + `ec_m1_3_pilot_mir_op` 专用步（EC-1/4 既有）
- **M1 里程碑：已退出**
- **仍开放不挡退出：** M1-A04 / M1-B02 / M1-B03 `[~]`；M1-E05 P2 `[ ]`
- **冻结声明：** 无 E05；无 CST/矩阵/JIT 扩面；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **公开叙事禁宣称：** 全入口矩阵、全后端静默通配清除、生产默认 JIT、无损 formatter GA
- **下一窗：** M2 W1 = thin start charter（ADR-145）；择一 M2 P0 thin slice；**不**默认开 E05 / SynUI

### Truth M2 W1 Error TCP facade（ADR-145 · 已交）

- **NetworkError** + `TcpStream.connect` / `TcpSocket.connect` **`throws`**（签名：`-> TcpStream throws`）
- **禁** connect 失败返回 `status="fail"` 成功形对象
- **VM：** 跨调用 throw 展开（call-depth-aware try handlers）— 嵌套 `throws` 可到达调用方 `catch`
- **EC-M2-2：** **partial**（connect 首钉；host read/write `tcp-*-fail` 字符串仍债 → W2）
- **inventory：** `net.tcp.facade` `status = "partial"`
- **M2 未退出：** EC-M2-1/3/4 仍 `[ ]`；EC-M2-2 未全勾
- **冻结声明：** 无 E05；无全量 S3～S8；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W2 — 退役 host `tcp-read-fail` / `tcp-write-fail`

### Truth M2 W2 TCP read/write typed errors（ADR-146 · 已交）

- **Hosts：** `host_tcp_read` 成功 = 载荷 `String`；`host_tcp_write` 成功 = `Int` 字节数；失败 = `Err`（无 `tcp-*-ok/fail`）
- **Facade：** `read` → `String throws`；`write` → `Int throws`；host 失败包装为 `NetworkError`
- **EC-M2-2：** 仍 **partial**（`prep:sql`、`tcp-stream-*`、其它 facade）
- **M2 未退出**
- **冻结声明：** 无 FileHandle；无 E05；无 SynUI/Farm/SIMD；无全量 S*；无 Phase KPI
- **下一窗：** M2 W3 — FileHandle `IOError` 首钉 **或** `prep:sql` / `tcp-stream-*` 清扫；**不**默认全量 DateTime/HTTP GA

### Truth M2 W3 FileHandle IOError first nail（ADR-147 · 已交）

- **Facade：** `IOError`；`FileHandle.open` → `Int throws`；`close` → `throws`；host Err 包装为 `IOError`
- **Registry：** `FileHandle.open`/`close` 去掉 `lookup_type_method` host 直绑，Call 走 facade（否则 CallHost 绕过 throws）
- **Hosts：** `host_file_open` / `host_file_close` 失败 = `Err`（invalid close 不再 Bool 哨兵）
- **EC-M2-2：** 仍 **partial**（`prep:sql` 未清；read/write/seek 仍债）
- **inventory：** `io.file_handle` `status = "partial"`
- **M2 未退出**
- **冻结声明：** 无全量 S3 Reader/Writer/RAII/mmap；无 prep:sql 主线；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W4 — 退役 `prep:{sql}` **或** FileHandle read/write throws 加深；**不**默认全量 DateTime/HTTP GA

### Truth M2 W4 retire `prep:{sql}`（ADR-148 · 已交）

- **Host：** `host_sql_prepare` 成功 = `Int` stmt id（缓存 SQL 文本）；失败 = `Err`；**无** `prep:{sql}` 糊墙串
- **Facade：** `SqlError`；`Sql.prepare` → `Int throws`
- **Registry：** `SqlPrepare` 返回类型 `String` → `Int`
- **EC-M2-2：** 仍 **partial**（`prep:sql` 字面已清；`tcp-stream-*`、FileHandle r/w 仍债）
- **inventory：** `sql.prepare` `status = "partial"`
- **M2 未退出**
- **冻结声明：** 无全量 S8 bind/Connection/Transaction；无 FileHandle r/w 主线；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W5 — FileHandle read/write throws 加深 **或** `tcp-stream-*` 烟测串清扫；**不**默认全量 DateTime/HTTP/S8 GA

### Truth M2 W5 FileHandle read/write IOError（ADR-149 · 已交）

- **Facade：** `read` → `Array<UInt8> throws`；`write` → `Int throws`；host Err 包装为 `IOError`（code 3/4）
- **Registry：** `FileHandle.read`/`write` `host: None`（Call 走 facade，对齐 open/close）
- **EC-M2-2：** 仍 **partial**（`tcp-stream-*` 烟测串未清；seek/readAllBytes 仍债）
- **inventory：** `io.file_handle` 仍 `partial`（open/close/read/write throws）
- **M2 未退出**
- **冻结声明：** 无 seek 主改；无全量 S3 RAII/mmap；无 tcp-stream 主线；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W6 — 退役 `tcp-stream-*` 烟测串 **或** `FileHandle.seek` throws；**不**默认全量 DateTime/HTTP/S8 GA

### Truth M2 W6 retire `tcp-stream-*` smoke（ADR-150 · 已交）

- **Hosts：** `host_tcp_stream` / `_tls` / `_handle` 成功 = `Int` nbytes；失败 = `Err`；**无** `tcp-stream-*` / `tcp-handle-*` 糊墙串
- **顺手：** `host_net_backpressure` → `Bool`/`Err`（非背压 GA）
- **Registry：** 三 stream host 返回类型 `String` → `Int`；backpressure → `Bool`
- **EC-M2-2：** 仍 **partial**（legacy `TcpStream.ok`/`status` 字段；其它 experimental）
- **inventory：** `net.tcp.facade` 回写（smoke hosts 已清）
- **M2 未退出**
- **冻结声明：** 无真 duplex/TLS/backpressure GA；无 FileHandle seek 主改；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W7 — `FileHandle.seek` throws **或** 清 TcpStream legacy `ok`/`status`；**不**默认全量 DateTime/HTTP/S8 GA

### Truth M2 W7 clear TcpStream legacy `ok`/`status`（ADR-151 · 已交）

- **Facade：** 删除 `TcpStream.ok` / `status` 字段及赋值；成功仅 `handle`；失败仍 `throws NetworkError`
- **EC-M2-2：** 仍 **partial**（点名状态串路径已清；其它 experimental / S4 资源模型仍债）——**不**宣称 M2 退出
- **inventory：** `net.tcp.facade` legacy fields removed；仍 `partial`（M2-S4 深模型）
- **冻结声明：** 无 FileHandle seek 主改；无真 duplex/TLS GA；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W8 — `FileHandle.seek` throws；**不**默认全量 DateTime/HTTP/S8 GA

### Truth M2 W8 FileHandle.seek IOError（ADR-152 · 已交）

- **Host：** `host_file_seek` I/O 失败 → `Err`（禁 `Bool(false)` 哨兵）；成功推 `Bool(true)`
- **Registry：** `FileHandle.seek` → `host: None`（走 facade throws）
- **Facade：** `seek(handle, offset) throws` → `IOError(code: 5, …)`
- **EC-M2-2：** 仍 **partial**（readAllBytes / RAII / mmap / S4 深模型仍债）——**不**宣称 M2 退出
- **inventory：** `io.file_handle` open/close/read/write/**seek** throws；Remaining = readAllBytes / RAII / mmap
- **冻结声明：** 无 readAllBytes 主改；无全量 S3；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT；**不**宣称 M2 退出
- **下一窗：** M2 W9 — `readAllBytes` throws **或** EC-M2-2 / M2 exit 薄章程；**不**默认全量 DateTime/HTTP/S8 GA

### Truth M2 W9 FileHandle.readAllBytes IOError（ADR-153 · 已交）

- **Facade：** `readAllBytes(handle) -> Array<UInt8> throws` → `IOError(code: 6, …)`
- **Registry：** `FileHandle.readAllBytes` → `host: None`（防御性，防 CallHost 直绑）
- **Host：** 未改（已 fail-closed Err）；真 mmap 仍属 M2-S3-04
- **EC-M2-2：** 仍 **partial**（RAII / mmap / S4 深模型 / Sql bind 仍债）——**不**宣称 M2 退出
- **inventory：** `io.file_handle` open/close/read/write/seek/**readAllBytes** throws；Remaining = instance RAII / true mmap
- **冻结声明：** 无真 mmap；无 RAII；无 E05；无 SynUI/Farm/SIMD；无 Phase KPI；无默认开 JIT；**不**宣称 M2 退出
- **下一窗：** M2 W10 — EC-M2-2 / M2 exit 薄章程 **或** Sql bind/step；**不**默认全量 DateTime/HTTP/S8/SynUI GA

### Truth M2 W10 Sql.bindStep SqlError（ADR-154 · 已交）

- **Facade：** `bindStep(sql, value) -> Int throws` → `SqlError(code: 2, …)`
- **Registry：** `Sql.prepare` / `Sql.bindStep` → `host: None`（防御性）
- **Host：** 未改（`host_sql_bind_step` 已 fail-closed Err）
- **EC-M2-2：** 仍 **partial**（exec/query / Transaction / S4 / RAII 仍债）——**不**宣称 M2 退出
- **inventory：** `sql.prepare` prepare+bindStep throws；Remaining = multi-Connection / Transaction / stmt resource
- **冻结声明：** 无全量 S8；无 exec/query throws 主线；无 E05；无 SynUI；**不**宣称 M2 退出
- **下一窗：** M2 W11 — EC-M2-2 切片签署薄章程 **或** `Sql.exec`/`query` throws；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W11 Sql.exec/query SqlError（ADR-155 · 已交）

- **Host：** `host_sql_exec_rusqlite` 失败 → `Err(R003)`（禁 `unwrap_or(0)` 哨兵）
- **Facade：** `exec`/`query` → `Int`/`Array` **throws** → `SqlError(code: 3/4, …)`
- **Registry：** `Sql.exec` / `Sql.query` → `host: None`
- **EC-M2-2：** 仍 **partial**（Transaction / S4 / RAII / experimental 仍债）——**不**宣称 M2 退出或 EC `[x]`
- **inventory：** prepare+bindStep+exec+query throws；Remaining = multi-Connection / Transaction / stmt resource
- **冻结声明：** 无 Transaction 主线；无 EC 签署；无 E05；无 SynUI；**不**宣称 M2 退出
- **下一窗：** M2 W12 — EC-M2-2 切片签署薄章程 **或** Sql Transaction 薄竖切；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W12 Sql Transaction SqlError（ADR-156 · 已交）

- **Facade：** `beginTransaction` / `commitTransaction` / `rollbackTransaction` **throws** → `SqlError(code: 5/6/7, …)`
- **Registry：** 三方法 → `host: None`
- **Host：** 未改（已 fail-closed Err）
- **EC-M2-2：** 仍 **partial**（multi-Connection / S4 / RAII / experimental 仍债）——**不**宣称 M2 退出或 EC `[x]`
- **inventory：** prepare+bindStep+exec+query+txn throws；Remaining = multi-Connection / pool / stmt resource
- **冻结声明：** 无 EC 签署；无 multi-Connection；无 E05；无 SynUI；**不**宣称 M2 退出
- **下一窗：** M2 W13 — EC-M2-2 切片签署薄章程 **或** multi-Connection 薄竖切；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W13 EC-M2-2 切片签署（ADR-157 · 已交）

- **EC-M2-2：** **`[x]` 切片** — 官方生产 facade 无点名状态串/空成功哨兵（TcpStream；FileHandle 静态 API；Sql 含 txn）
- **证据：** 既有 `m2_error_*` + `error_facade_slice` CI
- **诚实 defer：** os.sentinel；grpc experimental；S4 深模型；RAII/mmap；multi-Connection / pool
- **EC-M2-1/3/4：** 仍 `[ ]`（EC-M2-1 已签 file+Path + HTTP client/server + SQLite / ADR-174～177；总勾仍开）——**不**宣称 M2 里程碑退出
- **冻结声明：** 无 multi-Connection 主改；无全量 M2 退出；无 SynUI/Farm/SIMD；无 E05；无 Phase KPI；无默认开 JIT
- **下一窗：** M2 W14 — multi-Connection 薄竖切 **或** os.sentinel 首钉；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W14 Os.sentinel 首钉（ADR-158 · 已交）

- **Facade：** `Os.hostname` / `Os.diskTotal` → **throws OsError**（code 1/2）
- **Host：** 空 hostname / disk 不可用 → `Err(R003)`（禁空串/`-1` 哨兵）
- **Registry：** `Os.hostname` / `Os.diskTotal` → `host: None`
- **inventory：** hostname+diskTotal throws；Remaining = 其余 Os API（memory/uptime/…）
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 Os；无 multi-Connection；无 SynUI；无 E05
- **下一窗：** M2 W15 — 续 Os 哨兵扩面 **或** multi-Connection 薄竖切；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W15 Os.sentinel 续钉（ADR-159 · 已交）

- **Facade：** `Os.diskAvailable` / `Os.primaryIpv4` → **throws OsError**（code 3/4）
- **Host：** disk avail 不可用 / 空 primaryIpv4 → `Err(R003)`（禁 `-1`/空串哨兵）
- **Registry：** `Os.diskAvailable` / `Os.primaryIpv4` → `host: None`
- **inventory：** +diskAvailable/primaryIpv4 throws；Remaining = memory*/uptime/loadAverage/userName/homeDir
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 memory/uptime 全量；无 multi-Connection；无 SynUI；无 E05
- **下一窗：** M2 W16 — 续 Os 哨兵（memory*/uptime/load）**或** multi-Connection S8-01 薄竖切；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W16 Sql multi-Connection 首钉（ADR-160 · 已交）

- **Facade：** `Sql.open` / `close` / `execOn` / `queryOn` → **throws SqlError**（code 8–11）
- **Host：** `HashMap` 连接表；`rusqlite_conn()` → 隐式 conn **0**；open id ≥ 1
- **Registry：** 四方法 → `host: None`
- **inventory：** open/close/execOn/queryOn；Remaining = prepare/txn/ORM 穿线、pool、stmt resource
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 prepareOn/txnOn/ORM 穿线；无 pool；无续 Os 本窗；无 SynUI；无 E05
- **下一窗：** M2 W17 — 续 Os（userName/homeDir）**或** Sql conn 穿线（prepareOn）；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W17 Os.userName / homeDir 哨兵（ADR-161 · 已交）

- **Facade：** `Os.userName` / `Os.homeDir` → **throws OsError**（code 5/6）
- **Host：** 缺 USER/USERNAME 或 HOME/USERPROFILE → `Err(R003)`（禁空串哨兵）
- **Registry：** `Os.userName` / `Os.homeDir` → `host: None`
- **inventory：** +userName/homeDir throws；Remaining = memory*/uptime/loadAverage
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 memory/uptime 全量；无 prepareOn；无 SynUI；无 E05
- **下一窗：** M2 W18 — Sql `prepareOn` 薄竖切 **或** Os memory/uptime 注入钩评估；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W18 Sql.prepareOn（ADR-162 · 已交）

- **Facade：** `Sql.prepareOn(connId, sql) -> Int throws`（code 12）
- **Host：** `with_conn_id` + `conn.prepare`；`PREPARED_SQL` 仍全局文本缓存（不绑 conn）
- **Registry：** `prepareOn` → `host: None`
- **inventory：** +prepareOn；Remaining = bindStepOn/txn/ORM 穿线、pool、stmt-conn 绑定
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 bindStepOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W19 — Sql `bindStepOn` **或** Os memory/uptime 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W19 Sql.bindStepOn（ADR-163 · 已交）

- **Facade：** `Sql.bindStepOn(connId, sql, value) -> Int throws`（code 13）
- **Host：** `with_conn_id` + `conn.execute(sql, params![value])`；不读 `PREPARED_SQL`
- **Registry：** `bindStepOn` → `host: None`；legacy `bindStep` 仍 conn 0
- **inventory：** +bindStepOn；Remaining = stmt-conn 绑定、txn/ORM 穿线、pool
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 stmt↔conn 绑定；无 txnOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W20 — stmt↔conn 绑定 **或** Os memory/uptime 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W20 Sql.bindPreparedOn（ADR-164 · 已交）

- **Facade：** `Sql.bindPreparedOn(connId, stmtId, value) -> Int throws`（code 14）
- **Host：** `PREPARED_SQL` 存 `(connId, sql)`；lookup + conn 校验 + `execute`；`bindStep`/`bindStepOn` 不变
- **Registry：** `bindPreparedOn` → `host: None`
- **inventory：** +bindPreparedOn；Remaining = txn/ORM 穿线、pool
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无真实 Statement 句柄；无 txnOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W21 — `beginTransactionOn` **或** Os `memoryTotal` 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W21 Sql txnOn（ADR-165 · 已交）

- **Facade：** `beginTransactionOn` / `commitTransactionOn` / `rollbackTransactionOn` throws（codes 15/16/17）
- **Host：** `with_conn_id` + BEGIN/COMMIT/ROLLBACK；不碰全局 `SQL_TXN_ACTIVE`；legacy txn 仍 conn 0
- **Registry：** 三 On → `host: None`
- **inventory：** +txnOn；Remaining = ORM 穿线、pool
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 per-conn `SQL_TXN_ACTIVE`；无 pool；无 ORM 穿线；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W22 — ORM conn 穿线薄钉 **或** Os `memoryTotal` 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W22 Orm.saveOn（ADR-166 · 已交）

- **Facade：** `saveOn(connId, table, record) -> Bool throws`（OrmError code 1）
- **Host：** `with_conn_id` + CREATE IF NOT EXISTS + INSERT；fail-closed；legacy `save` 仍 Bool/conn 0
- **inventory：** +Orm.saveOn；Remaining = findWhereOn 等、pool；**M2-S8-04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 findWhereOn/updateWhereOn/deleteWhereOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W23 — `findWhereOn` **或** Os `memoryTotal` 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W23 Orm.findWhereOn（ADR-167 · 已交）

- **Facade：** `findWhereOn(connId, table, col, val) -> Array throws`（OrmError code 2）
- **Host：** `with_conn_id` + 参数化 `SELECT … WHERE col = ?`；legacy `findWhere` 仍 Array/conn 0
- **inventory：** +findWhereOn；Remaining = updateWhereOn/deleteWhereOn、pool；**M2-S8-04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 updateWhereOn/deleteWhereOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W24 — `updateWhereOn` **或** Os `memoryTotal` 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W24 Orm.updateWhereOn（ADR-168 · 已交）

- **Facade：** `updateWhereOn(connId, table, col, val, newCol, newVal) -> Int throws`（OrmError code 3）
- **Host：** `with_conn_id` + 参数化 UPDATE WHERE；fail-closed；legacy `updateWhere` 仍 Int/conn 0/`unwrap_or(0)`
- **inventory：** +updateWhereOn；Remaining = deleteWhereOn、pool；**M2-S8-04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 deleteWhereOn；无 pool；无 Os memory 本窗；无 SynUI；无 E05
- **下一窗：** M2 W25 — `deleteWhereOn` **或** Os `memoryTotal` 注入钩；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W25 Orm.deleteWhereOn（ADR-169 · 已交）

- **Facade：** `deleteWhereOn(connId, table, col, val) -> Int throws`（OrmError code 4）
- **Host：** `with_conn_id` + 参数化 DELETE WHERE；fail-closed；legacy `deleteWhere` 仍 Int/conn 0/`unwrap_or(0)`
- **inventory：** +deleteWhereOn；Remaining = pool（+ os.sentinel memory*）；**M2-S8-04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 Os memory 本窗；无 SynUI；无 E05；ORM On CRUD（save/find/update/delete）收口
- **下一窗：** M2 W26 — Os `memoryTotal` 注入钩 + throws **或** pool 薄钉；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W26 Os.memoryTotal（ADR-170 · 已交）

- **Facade：** `Os.memoryTotal() -> Int throws`（OsError code 7）
- **Host：** `SYX_OS_MEMORY_TOTAL_FORCE_ERR` 注入钩；fail-closed（无 `-1`）；registry `host: None`
- **inventory：** +memoryTotal；Remaining = memoryAvailable/uptime/load、pool；**M2-S8-03/04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 memoryAvailable/uptime/load 本窗；无 SynUI；无 E05
- **下一窗：** M2 W27 — `memoryAvailable` 同构注入 + throws **或** pool 薄钉；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W27 Os.memoryAvailable（ADR-171 · 已交）

- **Facade：** `Os.memoryAvailable() -> Int throws`（OsError code 8）
- **Host：** `SYX_OS_MEMORY_AVAILABLE_FORCE_ERR` 注入钩；fail-closed（无 `-1`）；registry `host: None`
- **inventory：** +memoryAvailable；Remaining = uptimeMs/loadAverage、pool；**M2-S8-03/04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 uptime/load 本窗；无 SynUI；无 E05
- **下一窗：** M2 W28 — `uptimeMs` 同构注入 + throws；**不**默认全量 DateTime/HTTP/S4/SynUI GA

### Truth M2 W28 Os.uptimeMs（ADR-172 · 已交）

- **Facade：** `Os.uptimeMs() -> Int throws`（OsError code 9）
- **Host：** `SYX_OS_UPTIME_MS_FORCE_ERR` 注入钩；fail-closed（无 `-1`）；registry `host: None`
- **inventory：** +uptimeMs；Remaining = loadAverage、pool；**M2-S8-03/04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 loadAverage 本窗；无 SynUI；无 E05
- **下一窗：** M2 W29 — `loadAverage` 同构注入 + throws；之后建议改轨 EC-M2-1/3

### Truth M2 W29 Os.loadAverage（ADR-173 · 已交）

- **Facade：** `Os.loadAverage() -> Int throws`（OsError code 10）
- **Host：** `SYX_OS_LOAD_AVERAGE_FORCE_ERR` 注入钩；fail-closed（无 `-1`）；registry `host: None`
- **inventory：** +loadAverage；os.sentinel Int/empty 公开哨兵已清；Remaining = pool / S4 / RAII；**M2-S8-03/04 仍开**
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 SynUI；无 E05
- **下一窗：** M2 W30 — EC-M2-1 窄交付；**不**默认全量 S8/M2 退出

### Truth M2 W30 EC-M2-1 file-tool slice（ADR-174 · 已交）

- **Example：** `examples/file_tool` — `Path.join`/`normalize`/`exists` + `FileHandle` open/write/readAllBytes/close（`throws` `IOError`）
- **钉测：** `ec_m2_1_file_tool` + CI；仅官方 facade，无裸 `__syx_*`
- **EC-M2-1：** 总勾仍 `[ ]`；roadmap footnote **CLI file+Path 切片 `[x]`**；HTTP / SQLite 服务 → 后续窗
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-1；无 Http throws 迁面；无 Sql pool/example；无 RAII；无泄漏闸
- **下一窗：** M2 W31 — HTTP client 窄切片（loopback）；**不**默认 M2 退出

### Truth M2 W31 EC-M2-1 http-client slice（ADR-175 · 已交）

- **Example：** `examples/http_tool` — `Http.get` refuse `try`/`catch` → `refused`；`SYX_EC_M2_1_HTTP_BASE` + `/health` 正例
- **钉测：** `ec_m2_1_http_client`（ephemeral loopback + refuse）+ CI；仅官方 `Http`，无裸 `__syx_http_*`
- **EC-M2-1：** 总勾仍 `[ ]`；roadmap footnote **HTTP client 切片 `[x]`**（file+Path 已有）；SQLite 服务 → 后续窗
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 Http throws 全迁；无 M2-S4 Request/Response 类型化；无 TLS 全集；无 Sql 本窗；无全量 EC-M2-1
- **下一窗：** M2 W32 — SQLite 服务窄切片；**不**默认 pool / M2 退出

### Truth M2 W32 EC-M2-1 sql-service slice（ADR-176 · 已交）

- **Surface：** `import Sql`（[`packages/data/sql.sy`](../../packages/data/sql.sy)）
- **Example：** `examples/sql_tool` — `:memory:` `open` / `execOn` / `queryOn` / `close`（`throws` `SqlError`）
- **钉测：** `ec_m2_1_sql_service`（CRUD + invalid close）+ CI；无裸 `__syx_sql_*`
- **EC-M2-1：** 总勾仍 `[ ]`；roadmap footnote **SQLite 服务切片 `[x]`**；仍开：HTTP 基础 server、Path 深契约等
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 pool；无 Orm 扩面；无全量 EC-M2-1；无 EC-M2-3/4
- **下一窗：** M2 W33 — EC-M2-1 收口评估；**不**默认勾全量 / M2 退出

### Truth M2 W33 EC-M2-1 http-server slice（ADR-177 · 已交）

- **Host：** `SYX_HTTP_SERVE_MAX_REQUESTS` — N>0 时 `Http.serve` 响应 N 次后返回（测试钩）
- **Example：** `examples/http_server_tool` — `Http.serve` + `/health` → `Http.response(200, "ok")`
- **钉测：** `ec_m2_1_http_server`（loopback GET + MAX_REQUESTS=1）+ CI
- **EC-M2-1：** 总勾仍 `[ ]`；footnote **HTTP 基础 server 切片 `[x]`**；仍开：Path 深契约等
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-1；无 EC-M2-3/4；无 TLS server；无 Path 深契约
- **下一窗：** M2 W34 — EC-M2-3 FileHandle 泄漏首钉；**不**默认 M2 退出

### Truth M2 W34 EC-M2-3 FileHandle leak first nail（ADR-178 · 已交）

- **API：** `file_handle_live_count()`（测试/诊断层；非应用 facade）
- **钉测：** `ec_m2_3_file_leak`（open+close 回基线；open 未 close → +1；测末 force_close）+ CI
- **EC-M2-3：** 总勾仍 `[ ]`；footnote **FileHandle live-count 首钉 `[x]`**；仍开：Socket/Process/DB
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-3；无 RAII；无 Tcp/Sql/Process 泄漏闸；无 M2 退出宣称
- **下一窗：** M2 W35 — EC-M2-3 Sql 连接存活计数首钉；**不**默认 M2 退出

### Truth M2 W35 EC-M2-3 Sql conn leak first nail（ADR-179 · 已交）

- **API：** `sql_conn_live_count()`（仅显式 id≥1；测试/诊断层；非应用 facade）
- **钉测：** `ec_m2_3_sql_leak`（open+close 回基线；open 未 close → +1；conn 0 不计；测末 force_close）+ CI
- **EC-M2-3：** 总勾仍 `[ ]`；footnote **FileHandle + Sql conn live-count 首钉 `[x]`**；仍开：Socket/Process
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-3；无 pool/RAII；无 Tcp/Process 泄漏闸；无 M2 退出宣称
- **下一窗：** M2 W36 — EC-M2-3 Tcp socket 存活计数首钉；**不**默认 M2 退出

### Truth M2 W36 EC-M2-3 Tcp socket leak first nail（ADR-180 · 已交）

- **Host：** `host_tcp_handle_release` 真从 `TCP_STREAMS` remove；`tcp_stream_live_count()`（测试/诊断层）；facade `closeHandle`
- **钉测：** `ec_m2_3_tcp_leak`（loopback connect+release 回基线；未 close → +1；测末 force_close）+ CI
- **EC-M2-3：** 总勾仍 `[ ]`；footnote **FileHandle + Sql + Tcp socket live-count 首钉 `[x]`**；仍开：Process
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-3；无 Process 泄漏闸；无 TLS/Listener GA；无 M2 退出宣称
- **下一窗：** M2 W37 — EC-M2-3 Process 存活计数首钉；**不**默认 M2 退出

### Truth M2 W37 EC-M2-3 Process leak first nail（ADR-181 · 已交）

- **API：** `process_child_live_count()`（测试/诊断层；非应用 facade）；`wait` 已从 `CHILD_TABLE` remove
- **钉测：** `ec_m2_3_process_leak`（spawn+wait 回基线；spawn 未 wait → +1；测末 force_close）+ CI
- **EC-M2-3：** 总勾仍 `[ ]`；footnote **FileHandle + Sql + Tcp + Process live-count 首钉 `[x]`**（四类首钉齐；非全量泄漏=0）
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-3；无 ProcessBuilder/kill/timeout GA；无 M2 退出宣称
- **下一窗：** M2 W38 — EC-M2-3 四类 live-count 收口评估；默认**不**勾全量、**不**默认 M2 退出

### Truth M2 W38 EC-M2-3 four-nail closeout assessment（ADR-182 · 已交）

- **元钉：** `ec_m2_3_four_nail_closeout`（四 live-count API 可链；四泄漏钉测文件；CI 四步 + 本窗；inventory meta）
- **EC-M2-3：** 总勾仍 `[ ]`；footnote **四类首钉 `[x]` + 收口评估签署 `[x]`**（≠ Q-S5 / 全量泄漏=0）
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-3；无 RAII/pool；无 Path/EC-M2-4 产品交付；无 M2 退出宣称
- **下一窗：** M2 W39 — EC-M2-1 Path 深契约（M2-S3-01 win/unix）；**不**默认勾 EC-M2-1 全量

### Truth M2 W39 EC-M2-1 Path deep contract（ADR-183 · 已交）

- **契约：** `Path.join` / `normalize` / `fileExtension` OS 分隔符证据；registry 补 `fileExtension` TypeMethod
- **钉测：** `ec_m2_1_path_contract`（cfg unix/windows）+ Ubuntu hard-gate + `windows-smoke`
- **EC-M2-1：** 总勾仍 `[ ]`；footnote **Path deep contract 切片 `[x]`**；M2-S3-01 `[x]`
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-1；无 UNC/盘符 GA；无新 Path 表面；无 RAII；无 M2 退出宣称
- **下一窗：** M2 W40 — EC-M2-4 首钉；默认**不**勾全量、**不**默认 M2 退出

### Truth M2 W40 EC-M2-4 Path/File availability first nail（ADR-184 · 已交）

- **矩阵：** HostFn Path* + File* overrides（`vm=real`）；产品能力 `io.path`；regen `host-fn-capabilities.md`
- **钉测：** `ec_m2_4_path_file_availability` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File availability 首钉 `[x]`**；仍开：其它切片矩阵 + micro-benchmark
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Criterion；无 HTTP/Sql/Tcp availability 扩面；无 M2 退出宣称
- **下一窗：** M2 W41 — Path/File micro-benchmark 首档；默认**不**勾全量

### Truth M2 W41 EC-M2-4 Path/File micro-benchmark first tier（ADR-185 · 已交）

- **度量：** `Path.join`×10000 Instant 环（`examples/path_microbench` + `run_file_printed`；无 Criterion / 无 PR ms 地板）
- **钉测：** `ec_m2_4_path_file_microbench` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **availability + micro-benchmark 首档 `[x]`**；仍开：其它切片矩阵 + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Criterion；无 File open/close 主度量；无 M2 退出宣称
- **下一窗：** M2 W42 — HTTP HostFn availability 首钉；默认**不**勾全量

### Truth M2 W42 EC-M2-4 HTTP HostFn availability first nail（ADR-186 · 已交）

- **矩阵：** Http* facade HostFn overrides（`vm=real`）；`net.http` 回写 + `net.http.server`；regen
- **钉测：** `ec_m2_4_http_availability` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP availability `[x]`**；仍开：Sql/Tcp 等切片矩阵 + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 HttpRequestAsync；无 Sql/Tcp availability；无 HTTP microbench；无 M2 退出宣称
- **下一窗：** M2 W43 — Sql HostFn availability 首钉；默认**不**勾全量

### Truth M2 W43 EC-M2-4 Sql HostFn availability first nail（ADR-187 · 已交）

- **矩阵：** Sql* facade-core HostFn overrides（`vm=real`）；`data.sqlite` 回写；regen
- **钉测：** `ec_m2_4_sql_availability` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP + Sql availability `[x]`**；仍开：Tcp 等切片矩阵 + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Orm*；无 SqlMigrate/BindMulti；无 Tcp availability；无 Sql microbench；无 M2 退出宣称
- **下一窗：** M2 W44 — Tcp HostFn availability 首钉；默认**不**勾全量

### Truth M2 W44 EC-M2-4 Tcp HostFn availability first nail（ADR-188 · 已交）

- **矩阵：** TcpConnect/Read/Write + TcpHandleRelease overrides（`vm=real`）；`net.tcp` 回写；regen
- **钉测：** `ec_m2_4_tcp_availability` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP + Sql + Tcp availability `[x]`**；仍开：非 Path 切片 microbench + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 TLS/Sockbuf/refcount；无 Url/Ws availability；无 Tcp microbench；无 M2 退出宣称
- **下一窗：** M2 W45 — HTTP Instant micro-benchmark 首档；默认**不**勾全量

### Truth M2 W45 EC-M2-4 HTTP Instant micro-benchmark first tier（ADR-189 · 已交）

- **度量：** `Http.response`×N Instant（`examples/http_microbench`；无 Criterion / 无 PR ms 地板 / 无网络）
- **钉测：** `ec_m2_4_http_microbench` + CI；`net.http` tests 交叉引用
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP Instant microbench `[x]`**；仍开：Sql/Tcp microbench + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Criterion；无 Http.get 网络主度量；无 Sql/Tcp microbench；无 M2 退出宣称
- **下一窗：** M2 W46 — Sql Instant micro-benchmark 首档；默认**不**勾全量

### Truth M2 W46 EC-M2-4 Sql Instant micro-benchmark first tier（ADR-190 · 已交）

- **度量：** 单连接 `:memory:` + `Sql.execOn` INSERT×N Instant（`examples/sql_microbench`；深度有界递归；无 Criterion / 无 PR ms 地板 / 无文件 DB）
- **钉测：** `ec_m2_4_sql_microbench` + CI；`data.sqlite` tests 交叉引用
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP + Sql Instant microbench `[x]`**；仍开：Tcp microbench + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Criterion；无文件 DB 主度量；无 Tcp microbench；无 M2 退出宣称
- **下一窗：** M2 W47 — Tcp Instant micro-benchmark 首档；默认**不**勾全量

### Truth M2 W47 EC-M2-4 Tcp Instant micro-benchmark first tier（ADR-191 · 已交）

- **度量：** `TcpStream.connect` 拒连×N Instant（`examples/tcp_microbench`；深度有界递归；无 Criterion / 无 PR ms 地板 / 无 listener）
- **钉测：** `ec_m2_4_tcp_microbench` + CI；`net.tcp` tests 交叉引用
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File + HTTP + Sql + Tcp Instant microbench `[x]`**；仍开：全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 Criterion；无 loopback 成功路径主度量；无 M2 退出宣称
- **下一窗：** M2 W48 — EC-M2-4 四切片 closeout 评估；默认**不**勾全量

### Truth M2 W48 EC-M2-4 four-slice avail+microbench closeout（ADR-192 · 已交）

- **评估：** 八类首钉证据齐（四 availability + 四 Instant microbench）+ CI + inventory；**≠** 全量 EC-M2-4
- **钉测：** `ec_m2_4_four_slice_closeout` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **收口评估签署 `[x]`**；仍开：全量（≠ native/WASM GA / Criterion）
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无新 HostFn/microbench 产品面；无 M2 退出宣称
- **下一窗：** M2 W49 — EC-M2-1 五切片收口评估；默认**不**勾全量

### Truth M2 W49 EC-M2-1 five-slice closeout（ADR-193 · 已交）

- **评估：** 五切片证据齐（file+Path / HTTP client / SQLite / HTTP server / Path deep）+ CI + inventory；**≠** 全量 EC-M2-1
- **钉测：** `ec_m2_1_five_slice_closeout` + CI
- **EC-M2-1：** 总勾仍 `[ ]`；footnote **收口评估签署 `[x]`**；仍开：全量 Path GA / 其它交付面
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-1；无新 example/stdlib 产品面；无 M2 退出宣称
- **下一窗：** M2 W50 — EC-M2-4 Criterion/SLA 首档；默认**不**勾全量

### Truth M2 W50 EC-M2-4 Path/File Criterion first tier（ADR-194 · 已交）

- **度量：** `Path.join`×10000 Criterion（`examples/path_microbench` + `benches/path_join_criterion`；无硬 ms 地板）
- **钉测：** `path_join_criterion` + CI test + 短 `cargo bench`
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File Criterion 首档 `[x]`**（≠ absolute SLA gate）；仍开：HTTP/Sql/Tcp Criterion + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无硬 ms SLA fail；无其它切片 Criterion；无 M2 退出宣称
- **下一窗：** M2 W51 — EC-M2-4 HTTP Criterion 首档；默认**不**勾全量

### Truth M2 W51 EC-M2-4 HTTP Criterion first tier（ADR-195 · 已交）

- **度量：** `Http.response`×10000 Criterion（`examples/http_microbench` + `benches/http_response_criterion`；无网络 / 无硬 ms 地板）
- **钉测：** `http_response_criterion` + CI test + 短 `cargo bench`
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **HTTP Criterion 首档 `[x]`**（≠ absolute SLA gate）；仍开：Sql/Tcp Criterion + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无硬 ms SLA fail；无 Sql/Tcp Criterion；无 Http.get 网络主度量；无 M2 退出宣称
- **下一窗：** M2 W52 — EC-M2-4 Sql Criterion 首档；默认**不**勾全量

### Truth M2 W52 EC-M2-4 Sql Criterion first tier（ADR-196 · 已交）

- **度量：** `:memory:` + `Sql.execOn` INSERT×10000 Criterion（`examples/sql_microbench` + `benches/sql_exec_criterion`；16MB 栈线程；无硬 ms 地板）
- **钉测：** `sql_exec_criterion` + CI test + 短 `cargo bench`
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Sql Criterion 首档 `[x]`**（≠ absolute SLA gate）；仍开：Tcp Criterion + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无硬 ms SLA fail；无 Tcp Criterion；无文件 DB 主度量；无 M2 退出宣称
- **下一窗：** M2 W53 — EC-M2-4 Tcp Criterion 首档；默认**不**勾全量

### Truth M2 W53 EC-M2-4 Tcp Criterion first tier（ADR-197 · 已交）

- **度量：** `TcpStream.connect` 拒连×10000 Criterion（`examples/tcp_microbench` + `benches/tcp_connect_criterion`；16MB 栈；无 listener / 无硬 ms 地板）
- **钉测：** `tcp_connect_criterion` + CI test + 短 `cargo bench`
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Tcp Criterion 首档 `[x]`**（≠ absolute SLA gate）；四切片 Criterion 首档齐；仍开：Criterion 收口评估 + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无硬 ms SLA fail；无 listener 成功路径主度量；无 M2 退出宣称
- **下一窗：** M2 W54 — EC-M2-4 四切片 Criterion 收口评估；默认**不**勾全量

### Truth M2 W54 EC-M2-4 four-slice Criterion closeout（ADR-198 · 已交）

- **主轴：** 四切片 Criterion 正式 benchmark 首档收口评估（元钉 `ec_m2_4_four_slice_criterion_closeout`；不跑 Criterion 本体）
- **钉测：** 四 Criterion 测/bench 文件 + Cargo `[[bench]]` + CI `--test`/`--bench` + inventory
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Criterion 收口评估签署 `[x]`**（≠ absolute SLA gate / ≠ native/WASM GA）；仍开：native/WASM GA + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无新 Criterion/HostFn 产品面；无硬 ms SLA fail；无 M2 退出宣称
- **下一窗：** M2 W55 — EC-M2-4 Path/File native/WASM availability matrix 首档加深；默认**不**勾全量

### Truth M2 W55 EC-M2-4 Path/File native/WASM matrix first deepening（ADR-199 · 已交）

- **主轴：** Path/File 矩阵非 VM 列诚实加深（`llvm=partial` / `wasm=host-assisted` / `cranelift=unsupported`；产品 `native=partial`；≠ GA）
- **钉测：** `ec_m2_4_path_file_native_wasm_matrix` + CI；HostFn/product notes 回写 ADR-199
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File native/WASM matrix 首档加深 `[x]`**（≠ native/WASM GA）；仍开：HTTP/Sql/Tcp matrix 加深 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 llvm/wasm/native=`real`；无新 HostFn；无 M2 退出宣称
- **下一窗：** M2 W56 — EC-M2-4 HTTP native/WASM availability matrix 首档加深；默认**不**勾全量

### Truth M2 W56 EC-M2-4 HTTP native/WASM matrix first deepening（ADR-200 · 已交）

- **主轴：** HTTP 矩阵非 VM 列诚实加深（`llvm=partial` / `wasm=host-assisted` / `cranelift=unsupported`；产品 `native=partial`；≠ GA / ≠ TLS server GA）
- **钉测：** `ec_m2_4_http_native_wasm_matrix` + CI；HostFn/product notes 回写 ADR-200
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **HTTP native/WASM matrix 首档加深 `[x]`**（≠ native/WASM GA）；仍开：Sql/Tcp matrix 加深 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 llvm/wasm/native=`real`；无 HttpRequestAsync / TLS server GA；无新 HostFn；无 M2 退出宣称
- **下一窗：** M2 W57 — EC-M2-4 Sql native/WASM availability matrix 首档加深；默认**不**勾全量

### Truth M2 W57 EC-M2-4 Sql native/WASM matrix first deepening（ADR-201 · 已交）

- **主轴：** Sql 矩阵非 VM 列诚实加深（`llvm=partial` / `wasm=host-assisted` / `cranelift=unsupported`；产品 `native=partial`；≠ GA / ≠ Orm GA）
- **钉测：** `ec_m2_4_sql_native_wasm_matrix` + CI；HostFn/product notes 回写 ADR-201
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Sql native/WASM matrix 首档加深 `[x]`**（≠ native/WASM GA）；仍开：Tcp matrix 加深 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 llvm/wasm/native=`real`；无 Orm/SqlMigrate/SqlBindMulti 覆盖；无新 HostFn；无 M2 退出宣称
- **下一窗：** M2 W58 — EC-M2-4 Tcp native/WASM availability matrix 首档加深；默认**不**勾全量

### Truth M2 W58 EC-M2-4 Tcp native/WASM matrix first deepening（ADR-202 · 已交）

- **主轴：** Tcp 矩阵非 VM 列诚实加深（`llvm=partial` / `wasm=host-assisted` / `cranelift=unsupported`；产品 `native=partial`；≠ GA / ≠ TLS GA）
- **钉测：** `ec_m2_4_tcp_native_wasm_matrix` + CI；HostFn/product notes 回写 ADR-202
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Tcp native/WASM matrix 首档加深 `[x]`**（≠ native/WASM GA）；四切片 matrix 首档齐；仍开：matrix 收口评估 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 llvm/wasm/native=`real`；无 TLS/listen/sockbuf 覆盖；无新 HostFn；无 M2 退出宣称
- **下一窗：** M2 W59 — EC-M2-4 四切片 native/WASM matrix 收口评估；默认**不**勾全量

### Truth M2 W59 EC-M2-4 four-slice native/WASM matrix closeout（ADR-203 · 已交）

- **主轴：** 四切片 native/WASM matrix 首档收口评估（元钉 `ec_m2_4_four_slice_native_wasm_matrix_closeout`；不重跑列断言本体）
- **钉测：** `ec_m2_4_four_slice_native_wasm_matrix_closeout` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **四切片 native/WASM matrix 收口评估签署 `[x]`**；Instant + Criterion + matrix 三轴首档均已收口；仍开：native/WASM GA + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 backends=`real`；无新 HostFn/product；无 M2 退出宣称
- **下一窗：** M2 W60 — EC-M2-4 Path/File native/WASM GA 首档加深；默认**不**勾全量

### Truth M2 W60 EC-M2-4 Path/File native/WASM GA first deepening（ADR-204 · 已交）

- **主轴：** PathJoin `llvm=real` 首档（native AOT CallHost 执行证据；≠ 全量 Path/File GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_path_file_native_wasm_ga` + CI；矩阵钉 PathJoin carve-out；HostFn/product notes 回写 ADR-204
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File native/WASM GA 首档加深 `[x]`**；其余 9 HostFn 仍 `llvm=partial`；产品 `native=partial`；仍开：HTTP/Sql/Tcp GA + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 File* llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W61 — EC-M2-4 HTTP native/WASM GA 首档加深；默认**不**勾全量

### Truth M2 W61 EC-M2-4 HTTP native/WASM GA first deepening（ADR-205 · 已交）

- **主轴：** HttpMakeResponse `llvm=real` 首档（native AOT CallHost 执行证据；≠ 全量 HTTP GA / ≠ wasm GA / ≠ TLS）
- **钉测：** `ec_m2_4_http_native_wasm_ga` + CI；矩阵钉 HttpMakeResponse carve-out；HostFn/product notes 回写 ADR-205
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **HTTP native/WASM GA 首档加深 `[x]`**；其余 10 Http* 仍 `llvm=partial`；产品 `native=partial`；仍开：Sql/Tcp GA + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 HttpGet/Serve llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 TLS GA；无 M2 退出宣称
- **下一窗：** M2 W62 — EC-M2-4 Sql native/WASM GA 首档加深；默认**不**勾全量

### Truth M2 W62 EC-M2-4 Sql native/WASM GA first deepening（ADR-206 · 已交）

- **主轴：** SqlOpen `llvm=real` 首档（native AOT CallHost `:memory:` 执行证据；≠ 全量 Sql GA / ≠ Orm GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_sql_native_wasm_ga` + CI；矩阵钉 SqlOpen carve-out；HostFn/product notes 回写 ADR-206
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Sql native/WASM GA 首档加深 `[x]`**；其余 16 Sql* 仍 `llvm=partial`；产品 `native=partial`；仍开：Tcp GA + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 SqlExecOn llvm=`real`；无 Orm override；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W63 — EC-M2-4 Tcp native/WASM GA 首档加深；默认**不**勾全量

### Truth M2 W63 EC-M2-4 Tcp native/WASM GA first deepening（ADR-207 · 已交）

- **主轴：** TcpConnect `llvm=real` 首档（native AOT CallHost refuse → `-1` 执行证据；≠ 全量 Tcp GA / ≠ TLS GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_tcp_native_wasm_ga` + CI；矩阵钉 TcpConnect carve-out；HostFn/product notes 回写 ADR-207
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Tcp native/WASM GA 首档加深 `[x]`**；其余三 Tcp* 仍 `llvm=partial`；产品 `native=partial`；仍开：四切片 GA 收口 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 TcpRead/Write/Release llvm=`real`；无 TLS override；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W64 — EC-M2-4 四切片 native/WASM GA 收口评估；默认**不**勾全量

### Truth M2 W64 EC-M2-4 four-slice native/WASM GA closeout（ADR-208 · 已交）

- **主轴：** 四切片 GA 首档收口评估（元钉：四 GA 文件 + CI + inventory；≠ 全量 native/WASM GA / ≠ 产品 native=`real`）
- **钉测：** `ec_m2_4_four_slice_native_wasm_ga_closeout` + CI；inventory closeout key 回写 ADR-208
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **四切片 native/WASM GA 收口评估签署 `[x]`**；仍开：GA 二档加深 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无新 llvm=`real`；无产品 native/wasm=`real`；无 M2 退出宣称
- **下一窗：** M2 W65 — EC-M2-4 Path/File native/WASM GA 二档加深；默认**不**勾全量

### Truth M2 W65 EC-M2-4 Path/File native/WASM GA second deepening（ADR-209 · 已交）

- **主轴：** PathExtension `llvm=real` 二档（native AOT CallHost `"a/b.txt"` → `"txt"`；PathJoin 首档保留；≠ 全量 Path/File GA / ≠ File* GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_path_file_native_wasm_ga2` + CI；矩阵/首档钉 PathExtension carve-out；HostFn/product notes 回写 ADR-209
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File native/WASM GA 二档加深 `[x]`**；其余 8 Path/File 仍 `llvm=partial`；产品 `native=partial`；仍开：HTTP GA 二档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 PathNormalize/Exists/File* llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W66 — EC-M2-4 HTTP native/WASM GA 二档加深；默认**不**勾全量

### Truth M2 W66 EC-M2-4 HTTP native/WASM GA second deepening（ADR-210 · 已交）

- **主轴：** HttpResponseStatus `llvm=real` 二档（native AOT CallHost MakeResponse+status → `"200"`；HttpMakeResponse 首档保留；≠ 全量 HTTP GA / ≠ Get/Serve / ≠ wasm GA）
- **钉测：** `ec_m2_4_http_native_wasm_ga2` + CI；矩阵/首档钉 HttpResponseStatus carve-out；HostFn/product notes 回写 ADR-210
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **HTTP native/WASM GA 二档加深 `[x]`**；其余 9 Http* 仍 `llvm=partial`；产品 `native=partial`；仍开：Sql GA 二档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 HttpGet/Post/Serve/Text llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 TLS GA；无 M2 退出宣称
- **下一窗：** M2 W67 — EC-M2-4 Sql native/WASM GA 二档加深；默认**不**勾全量

### Truth M2 W67 EC-M2-4 Sql native/WASM GA second deepening（ADR-211 · 已交）

- **主轴：** SqlClose `llvm=real` 二档（native AOT CallHost `:memory:` open+close → `"ok"`；SqlOpen 首档保留；≠ 全量 Sql GA / ≠ Orm GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_sql_native_wasm_ga2` + CI；矩阵/首档钉 SqlClose carve-out；HostFn/product notes 回写 ADR-211
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Sql native/WASM GA 二档加深 `[x]`**；其余 15 Sql* 仍 `llvm=partial`；产品 `native=partial`；仍开：Tcp GA 二档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 SqlExecOn llvm=`real`；无 Orm override；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W68 — EC-M2-4 Tcp native/WASM GA 二档加深；默认**不**勾全量

### Truth M2 W68 EC-M2-4 Tcp native/WASM GA second deepening（ADR-212 · 已交）

- **主轴：** TcpHandleRelease `llvm=real` 二档（native AOT CallHost loopback connect+release → `"true"`；TcpConnect 首档保留；≠ 全量 Tcp GA / ≠ TcpRead/Write GA / ≠ TLS GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_tcp_native_wasm_ga2` + CI；矩阵/首档钉 TcpHandleRelease carve-out；HostFn/product notes 回写 ADR-212
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Tcp native/WASM GA 二档加深 `[x]`**；TcpRead/TcpWrite 仍 `llvm=partial`；产品 `native=partial`；仍开：四切片 GA2 closeout + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 TcpRead/TcpWrite llvm=`real`；无 TLS/Sockbuf override；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W69 — EC-M2-4 四切片 native/WASM GA 二档 closeout；默认**不**勾全量

### Truth M2 W69 EC-M2-4 four-slice native/WASM GA second deepening closeout（ADR-213 · 已交）

- **主轴：** 四切片 GA 二档元钉收口评估（PathExtension / HttpResponseStatus / SqlClose / TcpHandleRelease llvm；≠ 全量 native/WASM GA / ≠ 新 llvm=`real`）
- **钉测：** `ec_m2_4_four_slice_native_wasm_ga2_closeout`（四 GA2 文件 + CI + inventory）+ CI
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **四切片 native/WASM GA 二档收口评估签署 `[x]`**；仍开：GA 三档加深 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 backends/`llvm=real` 新增；无产品 native/wasm=`real`；无全量 EC-M2-4；无 M2 退出宣称
- **下一窗：** M2 W70 — EC-M2-4 Path/File native/WASM GA 三档加深；默认**不**勾全量

### Truth M2 W70 EC-M2-4 Path/File native/WASM GA third deepening（ADR-214 · 已交）

- **主轴：** PathNormalize `llvm=real` 三档（native AOT CallHost `"a/b/../c"` → `"a/c"` / Windows `"a\\c"`；PathJoin/PathExtension 首二档保留；≠ 全量 Path/File GA / ≠ PathExists/File* GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_path_file_native_wasm_ga3` + CI；矩阵/首二档钉 PathNormalize carve-out；HostFn/product notes 回写 ADR-214
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Path/File native/WASM GA 三档加深 `[x]`**；PathExists + File* 仍 `llvm=partial`；产品 `native=partial`；仍开：HTTP GA 三档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 PathExists/File* llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W71 — EC-M2-4 HTTP native/WASM GA 三档加深；默认**不**勾全量

### Truth M2 W71 EC-M2-4 HTTP native/WASM GA third deepening（ADR-215 · 已交）

- **主轴：** HttpResponseText `llvm=real` 三档（native AOT CallHost MakeResponse+text → `"ok"`；HttpMakeResponse/HttpResponseStatus 首二档保留；≠ 全量 HTTP GA / ≠ Get/Serve/TLS GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_http_native_wasm_ga3` + CI；矩阵/首二档钉 HttpResponseText carve-out；HostFn/product notes 回写 ADR-215
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **HTTP native/WASM GA 三档加深 `[x]`**；其余 8 Http* 仍 `llvm=partial`；产品 `native=partial`；仍开：Sql GA 三档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 HttpGet/Post/Serve llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 TLS GA；无 M2 退出宣称
- **下一窗：** M2 W72 — EC-M2-4 Sql native/WASM GA 三档加深；默认**不**勾全量

### Truth M2 W72 EC-M2-4 Sql native/WASM GA third deepening（ADR-216 · 已交）

- **主轴：** SqlExecOn `llvm=real` 三档（native AOT CallHost `:memory:` CREATE+INSERT → `"1"`；SqlOpen/SqlClose 首二档保留；≠ 全量 Sql GA / ≠ Orm/QueryOn/txn/bind GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_sql_native_wasm_ga3` + CI；矩阵/首二档钉 SqlExecOn carve-out；HostFn/product notes 回写 ADR-216
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Sql native/WASM GA 三档加深 `[x]`**；其余 Sql* 仍 `llvm=partial`；产品 `native=partial`；仍开：Tcp GA 三档 + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 SqlQueryOn/txn/bind/Orm/Migrate llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W73 — EC-M2-4 Tcp native/WASM GA 三档加深；默认**不**勾全量

### Truth M2 W73 EC-M2-4 Tcp native/WASM GA third deepening（ADR-217 · 已交）

- **主轴：** TcpWrite `llvm=real` 三档（native AOT CallHost loopback connect+write `"hi"` → `"2"` + release `"true"`；TcpConnect/TcpHandleRelease 首二档保留；≠ 全量 Tcp GA / ≠ TcpRead/TLS GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_tcp_native_wasm_ga3` + CI；矩阵/首二档钉 TcpWrite carve-out；HostFn/product notes 回写 ADR-217
- **EC-M2-4：** 总勾仍 `[ ]`；footnote **Tcp native/WASM GA 三档加深 `[x]`**；TcpRead 仍 `llvm=partial`；产品 `native=partial`；仍开：absolute SLA + 全量（GA3 closeout → ADR-218）
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 TcpRead/TLS/Sockbuf llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W74 — EC-M2-4-G GA3 收口评估 — **done as ADR-218**

### Truth M2 W74 EC-M2-4-G four-slice native/WASM GA3 closeout（ADR-218 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-4-G` 元钉收口（四 GA3 文件 + CI + inventory；≠ 新 llvm=`real` / ≠ `EC-M2-4-Z`）
- **钉测：** `ec_m2_4_four_slice_native_wasm_ga3_closeout` + CI
- **EC-M2-4：** 总勾仍 `[ ]`；`EC-M2-4-G` `[x]`；footnote **四切片 native/WASM GA 三档收口评估签署 `[x]`**；仍开：`EC-M2-4-Z` + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无 backends/`llvm=real` 新增；无产品 native/wasm=`real`；无全量 EC-M2-4；无 M2 退出宣称
- **下一窗：** M2 W75 — TcpRead llvm GA 加深；默认**不**勾 `EC-M2-4-Z` — **done as ADR-219**

### Truth M2 W75 EC-M2-4-H Tcp native/WASM GA4（ADR-219 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-4-H` — TcpRead `llvm=real` 四档（native AOT CallHost loopback connect+read `"hi"` → `"hi"` + release `"true"`；ADR-188 四 HostFn 齐；≠ 全量 Tcp GA / ≠ TLS GA / ≠ wasm GA）
- **钉测：** `ec_m2_4_tcp_native_wasm_ga4` + CI；矩阵/首三档钉 TcpRead carve-out；HostFn/product notes 回写 ADR-219；`syx_rt_tcp_read` 仍 POP
- **EC-M2-4：** 总勾仍 `[ ]`；`EC-M2-4-H` `[x]`；footnote **Tcp native/WASM GA 四档加深 `[x]`**；产品 `native=partial`；仍开：`EC-M2-4-Z` + absolute SLA + 全量
- **EC-M2-2：** 保持 **`[x]` 切片**；**不**宣称 M2 退出
- **冻结声明：** 无全量 EC-M2-4；无 TLS/Sockbuf llvm=`real`；无 wasm=`real`；无产品 native=`real`；无 M2 退出宣称
- **下一窗：** M2 W76 — `M2-S5-01` Instant/Duration 分离；默认**不**勾 `EC-M2-4-Z` — **done as ADR-220**

### Truth M2 W76 M2-S5-01 Instant/Duration separation（ADR-220 · 已交）

- **主轴 / Roadmap ID：** `M2-S5-01` — 单调 `Instant.now` / `elapsedMs` vs 墙上 `Clock`/`Time.nowMs`；`Duration.*` 保持 Int-ms；HostFn `InstantNowMs`
- **钉测：** `m2_s5_01_instant_duration` + CI；能力行 `datetime.clock` 诚实回写
- **Sx：** `M2-S5-01` `[x]`；**不**开 `M2-S5-02` IANA；**不**宣称 M2 退出
- **冻结声明：** 无 IANA tzdb；无 `EC-M2-4-Z`；无 llvm GA 加深；无 M2 退出宣称
- **下一窗：** M2 W77 — `M2-S5-03` ISO8601/RFC3339 round-trip；默认**不**开 IANA — **done as ADR-221**

### Truth M2 W77 M2-S5-03 ISO8601/RFC3339 round-trip（ADR-221 · 已交）

- **主轴 / Roadmap ID：** `M2-S5-03` — UTC/fixed-offset 标准集 round-trip（负 epoch / 闰日 / 2038+ / date-only / fail-closed）
- **钉测：** `m2_s5_03_iso_rfc3339_roundtrip` + CI；DateTimeParse/ParseIso/Format HostFn + `datetime.calendar` 诚实回写
- **Sx：** `M2-S5-03` `[x]`；**不**开 `M2-S5-02` IANA；**不**宣称 M2 退出
- **冻结声明：** 无 IANA/`formatWithTimezone`；无 week-date/ordinal；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W78 — `M2-S5-04` 废除 M0 临时 Unsupported 预览；默认**不**开 IANA 全量 GA — **done as ADR-222**

### Truth M2 W78 M2-S5-04 retire Unsupported preview（ADR-222 · 已交）

- **主轴 / Roadmap ID：** `M2-S5-04` — 废除 M0-A03 DateTime Unsupported/preview 残留；`DateTime.now` → `DateTimeNow` host；`datetime.calendar` → `implemented`（窄 Gregorian+RFC3339 核心）
- **钉测：** `m2_s5_04_retire_unsupported_preview` + CI；无 `parse("unsupported")` 黑客；IANA/`formatWithTimezone` 仍 U001
- **Sx：** `M2-S5-04` `[x]`；**不**开 `M2-S5-02` IANA；**不**宣称 timezone GA / M2 退出
- **冻结声明：** 无 IANA tzdb；无 `EC-M2-4-Z`；无 llvm GA 加深；无 M2 退出宣称
- **下一窗：** M2 W79 — `M2-S1-01` Collection/Sequence/Iterator 协议稳定子集；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-223**

### Truth M2 W79 M2-S1-01 Collection/Sequence/Iterator（ADR-223 · 已交）

- **主轴 / Roadmap ID：** `M2-S1-01` — Iterator for-in 稳定子集（自定义 `: Iterator` `next()->Int?` + Array/String/Range）；MirOp/capability 诚实
- **钉测：** `m2_s1_01_collection_sequence_iterator` + CI；`collections.protocol` → `implemented`；`MakeIterator`/`IteratorNext` vm/mir=`real`
- **Sx：** `M2-S1-01` `[x]`；**不**开 `M2-S1-02` mutation / `makeIterator` GA / Deque；**不**宣称 M2 退出
- **冻结声明：** 无 mutation 统一；无 IANA；无 `EC-M2-4-Z`；无 llvm GA 加深；无 M2 退出宣称
- **下一窗：** M2 W80 — `M2-S1-02` Array/Set/Dictionary mutation 语义统一（含 remove）；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-224**

### Truth M2 W80 M2-S1-02 collection mutation（ADR-224 · 已交）

- **主轴 / Roadmap ID：** `M2-S1-02` — `var` HeapSlot write-back：Dict.remove / Set.insert·remove（Bool）/ Array 下标 + setAt；SetBuild → `HeapSlotKind::Set`
- **钉测：** `m2_s1_02_collection_mutation` + CI；`collections.mutation` → `implemented`
- **Sx：** `M2-S1-02` `[x]`；**不**开 Deque / Dict 下标插入 / 泛型全集；**不**宣称 M2 退出
- **冻结声明：** 无 `M2-S1-03`/`04` 本窗勾；无 IANA；无 `EC-M2-4-Z`；无 llvm GA 加深；无 M2 退出宣称
- **下一窗：** M2 W81 — `M2-S1-04` native feature-off 不得 silent 空成功；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-225**

### Truth M2 W81 M2-S1-04 native feature-off no silent empty（ADR-225 · 已交）

- **主轴 / Roadmap ID：** `M2-S1-04` — Array HOF / CallHost `map`·`filter`·`reduce`：非法形状与 `not(interpreter)` **fail-closed `U001`**，禁止 `[]`/`Nil` silent 空成功
- **钉测：** `m2_s1_04_native_feature_off_no_silent` + CI；`collections.feature_off` → `implemented`；native 仍 `partial`（≠ llvm HOF GA）
- **Sx：** `M2-S1-04` `[x]`；**不**开 Deque / String 三视图 / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 `M2-S1-03`；无 `M2-S2-*` 本窗勾；无 IANA；无 `EC-M2-4-Z`；无 llvm HOF GA 加深；无 M2 退出宣称
- **下一窗：** M2 W82 — `M2-S2-01` Character/grapheme / scalar / UTF-8 三视图；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-226**

### Truth M2 W82 M2-S2-01 string three-view（ADR-226 · 已交）

- **主轴 / Roadmap ID：** `M2-S2-01` — Character(grapheme) / Unicode scalar / UTF-8 bytes 三视图；`for-in` 对齐 grapheme
- **钉测：** `m2_s2_01_string_three_view` + CI；`text.three_view` → `implemented`；native/wasm **real**（W207 / ADR-551；≠ llvm String GA）
- **Sx：** `M2-S2-01` `[x]`；**不**开 Index / normalization / XID / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 `M2-S2-02`/`03`/`04` 本窗勾；无 IANA；无 `EC-M2-4-Z`；无 llvm String GA；无 M2 退出宣称
- **下一窗：** M2 W83 — `M2-S2-02` 下标稳定 Index 或明确 O(n) grapheme offset；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-227**

### Truth M2 W83 M2-S2-02 string index honesty（ADR-227 · 已交）

- **主轴 / Roadmap ID：** `M2-S2-02` — 文档诚实：Int grapheme offset = **O(n)** 随机访问（**不**引入 opaque Index）
- **钉测：** `m2_s2_02_string_index_honesty` + CI；`text.index_honesty` → `implemented`；native 仍 `partial`
- **Sx：** `M2-S2-02` `[x]`；**不**开 normalization / XID / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 opaque `StrIndex` / `String.Index` GA；无 `M2-S2-03`/`04` 本窗勾；无 IANA；无 `EC-M2-4-Z`；无 llvm String GA；无 M2 退出宣称
- **下一窗：** M2 W84 — `M2-S2-03` normalization / case folding MVP；默认**不**开 XID / IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-228**

### Truth M2 W84 M2-S2-03 string normalization / case folding（ADR-228 · 已交）

- **主轴 / Roadmap ID：** `M2-S2-03` — NFC/NFD + `caseFolded` 已知向量；lower/upper 诚实为 Unicode simple case
- **钉测：** `m2_s2_03_string_normalization` + CI；`text.normalization` → `implemented`；Wave A W212 native/wasm=real（ADR-556）
- **Sx：** `M2-S2-03` `[x]`；**不**开 NFKC/NFKD / XID / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 NFKC/NFKD；无 locale/Turkish；无 `M2-S2-04` 本窗勾；无 IANA；无 `EC-M2-4-Z`；无 llvm String GA；无 M2 退出宣称
- **下一窗：** M2 W85 — `M2-S2-04` 标识符 Unicode XID；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-229**

### Truth M2 W85 M2-S2-04 Unicode XID identifiers（ADR-229 · 已交）

- **主轴 / Roadmap ID：** `M2-S2-04` — lexer UAX #31 XID_Start/Continue（`unicode-xid`）；S2 叶子收口
- **钉测：** `m2_s2_04_unicode_xid_identifiers` + CI；`language.unicode_identifiers` → `implemented`
- **Sx：** `M2-S2-04` `[x]`；**不**开 ident NFC / TR39 / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 UTS #39 confusables；无 ident NFC；无全量 LSP GA；无 IANA；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W86 — `M2-S7-01` Encoder/Decoder 协议；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-230**

### Truth M2 W86 M2-S7-01 Encoder/Decoder protocol（ADR-230 · 已交）

- **主轴 / Roadmap ID：** `M2-S7-01` — 可扩展编码协议面（`Codable: Encodable, Decodable` + Encoder/Decoder）
- **钉测：** `m2_s7_01_encoder_decoder_protocol` + CI；`data.coding.protocol` → `implemented`
- **Sx：** `M2-S7-01` `[x]`；**不**开 JSONEncoder/字段路径 / streaming / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 S7-02/S7-03 本窗勾；无 KeyedEncodingContainer GA；无 IANA；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W87 — `M2-S7-02` JSONEncoder/Decoder + 字段路径诊断；默认**不**开 streaming / IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-231**

### Truth M2 W87 M2-S7-02 JSONEncoder/Decoder + field paths（ADR-231 · 已交）

- **主轴 / Roadmap ID：** `M2-S7-02` — JSONEncoder/Decoder（`typeName` 静态 API）+ 递归 reflect + 缺键 `codingPath`（R010）
- **钉测：** `m2_s7_02_json_encoder_decoder_field_paths` + CI；`data.coding.json_encoder` → `implemented`；native 仍 `partial`
- **Sx：** `M2-S7-02` `[x]`；**不**开 streaming / CodingKeys DSL / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 S7-03 本窗勾；无 metatype `T.Type` API；无 KeyedEncodingContainer GA；无 IANA；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W88 — `M2-S7-03` streaming（可选 MVP）；默认**不**开 IANA / **不**勾 `EC-M2-4-Z` — **done as ADR-232**

### Truth M2 W88 M2-S7-03 JSONStream NDJSON（ADR-232 · 已交）

- **主轴 / Roadmap ID：** `M2-S7-03` — JSONStream NDJSON session + chunk feed；`codingPath=line[N].…`（R010）
- **钉测：** `m2_s7_03_json_stream_ndjson` + CI；`data.coding.json_stream` → `implemented`；native 仍 `partial`
- **Sx：** `M2-S7-03` `[x]`；**S7 叶子收口**；**不**开 CodingKeys / File S3-03 / HTTP body stream / IANA；**不**宣称 M2 退出
- **冻结声明：** 无 CodingKeys DSL；无 metatype `T.Type`；无 File/HTTP 真流；无 IANA；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W89 — `M2-S5-02` DateTime + IANA tzdb；默认**不**勾 `EC-M2-4-Z` — **done as ADR-233**

### Truth M2 W89 M2-S5-02 IANA tzdb DST（ADR-233 · 已交）

- **主轴 / Roadmap ID：** `M2-S5-02` — IANA via `chrono-tz`；`timezoneOffset(ms,tz)` 分钟东正；`formatWithTimezone`；NY 冬/夏 DST + leap-day
- **钉测：** `m2_s5_02_iana_tzdb_dst` + CI；`datetime.timezone` → `implemented`；native 仍 `partial`
- **Sx：** `M2-S5-02` `[x]`；**S5 叶子收口**；**不**开 leap second / 全量 tz GA / llvm DateTime=`real`；**不**宣称 M2 退出
- **冻结声明：** 无 leap second；无 week-date/ordinal ISO GA；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W90 — `M2-S3-02b` Reader/Writer/Seekable + RAII/`with`；默认**不**勾 `EC-M2-3` 总勾 / `EC-M2-4-Z` — **done as ADR-234**

### Truth M2 W90 M2-S3-02b FileHandle RAII/`withOpen`（ADR-234 · 已交）

- **主轴 / Roadmap ID：** `M2-S3-02b` — `Reader`/`Writer`/`Seekable`/`Closeable` + `openResource`/`withOpen` always-close；`file_handle_live_count` 资源契约
- **钉测：** `m2_s3_02b_filehandle_raii_with` + CI；`io.file.raii` → `implemented`；native 仍 `partial`
- **Sx：** `M2-S3-02b` `[x]`；S3 仍留 03/04；**不**开 `EC-M2-3` 总勾；**不**宣称 M2 退出
- **冻结声明：** 无语言 `with`/`defer`；无 stream/watcher；无 mmap；无 `EC-M2-3-Z` / `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W91 — `M2-S4-02b` client 超时 / TLS 负例；默认**不**勾 server TLS GA / `EC-M2-4-Z` — **done as ADR-235**

### Truth M2 W91 M2-S4-02b HTTP client timeout / TLS 负例（ADR-235 · 已交）

- **主轴 / Roadmap ID：** `M2-S4-02b` — hang loopback + `timeout_ms` → R012；self-signed HTTPS → rustls verify fail-closed
- **钉测：** `m2_s4_02b_client_timeout_tls` + CI；`net.http.timeout_tls` → `implemented`
- **Sx：** `M2-S4-02b` `[x]`；S4 仍留 01/03b/04/05；**不**开 S4-05 TLS client GA / server TLS；**不**宣称 M2 退出
- **冻结声明：** 无 skip-verify；无 `net.tls.server` implemented；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W92 — `M2-S4-01` Request/Response/Headers/Body/URL 类型；默认**不**勾 S4-05 / `EC-M2-4-Z`

### Truth M2 W92 M2-S4-01 HTTP Request/Response/Headers/Body/URL 类型（ADR-236 · 已交）

- **主轴 / Roadmap ID：** `M2-S4-01` — facade 返回 `HTTPResponse` / `Url`；类型面 `HttpRequest`/`HttpHeaders`/`HttpBody`（非糊墙 `Object`）
- **钉测：** `m2_s4_01_http_types` + CI；`net.http.types` → `implemented`
- **Sx：** `M2-S4-01` `[x]`；S4 仍留 03b/04/05；**不**开 S4-05 TLS client GA / streaming / server TLS；**不**宣称 M2 退出
- **冻结声明：** 无 skip-verify；无 `net.tls.server` implemented；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W93 — `M2-S4-05` TLS client 真实路径；默认**不**勾 server TLS GA / `EC-M2-4-Z`

### Truth M2 W93 M2-S4-05 TLS client 真实路径（ADR-237 · 已交）

- **主轴 / Roadmap ID：** `M2-S4-05` — HTTPS client 正例（reqwest+rustls）；诊断根 `SYX_HTTP_EXTRA_ROOT_PEM`；`net.tls.client` → `implemented`
- **钉测：** `m2_s4_05_client_tls` + CI；02b bad-cert 负例仍有效；`net.tls.server` 仍 unsupported
- **Sx：** `M2-S4-05` `[x]`；S4 仍留 03b/04；**不**开 server TLS GA / skip-verify 产品 API；**不**宣称 M2 退出
- **冻结声明：** 无 skip-verify/pin 用户面；无 `net.tls.server` implemented；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W94 — `M2-S4-03b` server graceful shutdown；默认**不**勾 streaming / server TLS GA / `EC-M2-4-Z`

### Truth M2 W94 M2-S4-03b server graceful shutdown（ADR-238 · 已交）

- **主轴 / Roadmap ID：** `M2-S4-03b` — `Http.shutdown` → `tiny_http::Server::unblock`；完成当前请求后 `Http.serve` 干净返回
- **钉测：** `m2_s4_03b_server_graceful_shutdown` + CI；**不用** `SYX_HTTP_SERVE_MAX_REQUESTS` 冒充 graceful；`net.http.server` 仍 **partial**
- **Sx：** `M2-S4-03b` `[x]`；S4 仅留 04 streaming；**不**开 server TLS GA / streaming；**不**宣称 M2 退出
- **冻结声明：** 无 streaming；无 `net.tls.server` implemented；无 `HTTPServer`/drain_ms；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W95 — `M2-S4-04` streaming body（可后置）；默认**不**勾 server TLS GA / `EC-M2-4-Z`

### Truth M2 W95 M2-S4-04 HTTP body stream（ADR-239 · 已交）

- **主轴 / Roadmap ID：** `M2-S4-04` — client 响应 `HttpBodyStream` open/readChunk/close；session + `Read` 增量（≠ `response.bytes()`）
- **钉测：** `m2_s4_04_http_body_stream` + CI；默认 `Http.get`/`fetch` 仍整缓冲；`net.http.stream` → `implemented`
- **Sx：** `M2-S4-04` `[x]`；**S4 七叶子收口**；**不**开 server 流式 / wire chunked TE / File S3-03；**不**宣称 M2 退出
- **冻结声明：** 无 server streaming；无 `HttpRequestAsync` GA；无 `net.tls.server` implemented；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W96 — `M2-S3-03` File 真 stream/chunk；默认**不**勾 mmap S3-04 / `EC-M2-4-Z`

### Truth M2 W96 M2-S3-03 File stream + watcher lifecycle（ADR-240 · 已交）

- **主轴 / Roadmap ID：** `M2-S3-03` — `FileStream` 分块读 + `FileWatcher` 生命周期；验收 **关闭后失败**
- **钉测：** `m2_s3_03_file_stream_watcher` + CI；`io.file.stream` → `implemented`；`io.file.watch` → `partial`（mock 默认）
- **Sx：** `M2-S3-03` `[x]`；S3 仅留 04 mmap；**不**开 EC-M2-3 总勾 / 递归 notify GA；**不**宣称 M2 退出
- **冻结声明：** 无 mmap；无 `EC-M2-3-Z`；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W97 — `M2-S3-04` mmap；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W97 M2-S3-04 mmap 已改名收口（ADR-241 · 已交）

- **主轴 / Roadmap ID：** `M2-S3-04` — 以 **「已改名」** 收口（`FileHandle.readAllBytes` = `read_to_end`；≠ OS mmap）
- **钉测：** `m2_s3_04_mmap_rename` + CI；`io.file.read_all_bytes` → `implemented`
- **Sx：** `M2-S3-04` `[x]`；**S3 叶子收口**；**不**引入 `memmap2`；**不**勾 `EC-M2-4-Z`；**不**宣称 M2 退出
- **冻结声明：** 无 OS mmap 宣称；无 `HostFnId::FileMmapRead` 改名；无 `EC-M2-3-Z`；无 M2 退出宣称
- **下一窗：** M2 W98 — `M2-S6-01` ProcessBuilder；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W98 M2-S6-01 ProcessBuilder（ADR-242 · 已交）

- **主轴 / Roadmap ID：** `M2-S6-01` — `ProcessBuilder` cwd/env overlay + `Child.kill`/`waitTimeout`；验收 **关闭/wait 后失败**
- **钉测：** `m2_s6_01_process_builder` + CI；`os.process.builder` → `implemented`
- **Sx：** `M2-S6-01` `[x]`；S6 仍留 02b/03；**不**开 Pipe GA / signal / EC-M2-3 总勾；**不**宣称 M2 退出
- **冻结声明：** 无 S6-02b Pipe；无 S6-03；无 `EC-M2-3-Z`；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W99 — `M2-S6-02b` Child + Stdio/Pipe；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W99 M2-S6-02b Child Stdio/Pipe（ADR-243 · 已交）

- **主轴 / Roadmap ID：** `M2-S6-02b` — `ChildPipe` 流式 stdout/stderr/stdin；验收 **流式测 / 无挂死**
- **钉测：** `m2_s6_02b_child_stdio_pipe` + CI；`os.process.stdio` → `implemented`
- **Sx：** `M2-S6-02b` `[x]`；S6 仅留 03 signal；**不**开 S6-03 / EC-M2-3 总勾；**不**宣称 M2 退出
- **冻结声明：** 无 signal；无 async select Pipe GA；无 `EC-M2-3-Z`；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W100 — `M2-S6-03` signal；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W100 M2-S6-03 Child signal（ADR-244 · 已交）

- **主轴 / Roadmap ID：** `M2-S6-03` — `Child.signal` TERM/INT/KILL；平台差异显式
- **钉测：** `m2_s6_03_child_signal` + CI；`os.process.signal` → `implemented`
- **Sx：** `M2-S6-03` `[x]`；**S6 叶子收口**；**不**开全量 POSIX / async handlers；**不**宣称 M2 退出
- **冻结声明：** 无全 POSIX；无 `Process.signalSelf`；无 `EC-M2-3-Z`；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W101 — `M2-S8-01c` Connection pool；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W101 M2-S8-01c Connection pool MVP（ADR-245 · 已交）

- **主轴 / Roadmap ID：** `M2-S8-01c` — `Sql.poolCreate`/`poolAcquire`/`poolRelease`/`poolClose`；并发窄测
- **钉测：** `m2_s8_01c_connection_pool` + CI；`data.sqlite.pool` → `implemented`
- **Sx：** `M2-S8-01c` `[x]`；S8 **未**叶子收口（仍开 `02b`/`03b`）；**不**开阻塞 acquire / 并发事务门槛；**不**宣称 M2 退出
- **冻结声明：** 无 `M2-S8-02b`/`03b` GA；无 live Statement；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W102 — `M2-S8-02b` typed Row；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W102 M2-S8-02b typed Row + 裸 SQL hatch（ADR-246 · 已交）

- **主轴 / Roadmap ID：** `M2-S8-02b` — 命名 `SqlRow` + 列访问器；裸 SQL hatch 文档诚实；禁糊墙成功
- **钉测：** `m2_s8_02b_typed_row` + CI；`data.sqlite.row` → `implemented`
- **Sx：** `M2-S8-02b` `[x]`；S8 **未**叶子收口（仍开 `03b`）；**不**开 query builder / Orm GA / 并发事务门槛；**不**宣称 M2 退出
- **冻结声明：** 无 `M2-S8-03b` GA；无 live Statement；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W103 — `M2-S8-03b` pool/并发事务；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W103 M2-S8-03b pool / 并发事务门槛（ADR-247 · 已交）

- **主轴 / Roadmap ID：** `M2-S8-03b` — pool×txn 卫生（release-mid-txn 拒 + IMMEDIATE busy + poolClose 泄漏诚实）
- **钉测：** `m2_s8_03b_pool_txn` + CI；加深 `data.sqlite.pool`
- **Sx：** `M2-S8-03b` `[x]`；**S8 叶子收口**；**不**开阻塞 acquire / DEFERRED 改设计；**不**宣称 M2 退出
- **冻结声明：** 无新 Sql API；无 live Statement；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W104 — `M2-S1-03` 可选 Deque；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W104 M2-S1-03 Deque MVP（ADR-248 · 已交）

- **主轴 / Roadmap ID：** `M2-S1-03` — Deque Int MVP（push/pop 两端 + count/isEmpty + HeapSlot 写回）
- **钉测：** `m2_s1_03_deque` + CI；`collections.deque` → `implemented`
- **Sx：** `M2-S1-03` `[x]`；**S1 叶子收口**（最后一枚 Sx）；**不**开 Heap/lazy / makeIterator GA；**不**宣称 M2 退出
- **冻结声明：** 无 Heap/PriorityQueue；无 lazy sequence；无 `EC-M2-*-Z`；无 M2 退出宣称
- **下一窗：** M2 W105 — `EC-M2-3-Z` 四钉泄漏收口；默认**不**勾 `EC-M2-4-Z`

### Truth M2 W105 EC-M2-3-Z 四钉生产路径泄漏收口（ADR-249 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-3-Z` — File `withOpen` / Sql poolClose / ProcessBuilder+Child.wait / Tcp host-release live_count 回基线
- **钉测：** `ec_m2_3_z_closeout` + CI；验收诚实收窄（≠ Q-S5 全局）
- **EC：** `EC-M2-3-Z` `[x]`；**EC-M2-3 总勾 / Q-S5 仍开**；**不**宣称 M2 退出
- **冻结声明：** 无新 live_count API；无 `EC-M2-4-Z`；无 M2 退出宣称
- **下一窗：** M2 W106 — `EC-M2-4-Z`；默认**不**宣称 M2 退出

### Truth M2 W106 EC-M2-4-Z 加深轨退出收口（ADR-250 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-4-Z` — A–H 八轴加深轨证据链齐（availability/µbench/Criterion/matrix/GA1–3 closeout/Tcp GA4）
- **钉测：** `ec_m2_4_z_closeout` + CI；验收诚实收窄（≠ absolute SLA / ≠ backends=`real`）
- **EC：** `EC-M2-4-Z` `[x]`；**EC-M2-4 总勾仍开**；**不**宣称 M2 退出
- **冻结声明：** 无 absolute SLA gate；无产品 `native=real`/`wasm=real`；无全量 HostFn GA；无 `EC-M2-1-Z`；无 M2 退出宣称
- **下一窗：** M2 W107 — `EC-M2-1-Z`；默认**不**宣称 M2 退出

### Truth M2 W107 EC-M2-1-Z 六切片证据链收口（ADR-251 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-1-Z` — A–F 六切片交付证据链齐（file+Path / HTTP client / SQLite / HTTP server / Path deep / five-slice closeout）
- **钉测：** `ec_m2_1_z_closeout` + CI；验收诚实收窄（≠ 全量 Path GA）
- **EC：** `EC-M2-1-Z` `[x]`；**EC-M2-1 总勾仍开**；\(P_{EC}=P_{M2}=100\%\)（**≠ M2 退出**）
- **冻结声明：** 无全量 Path GA；无 PathExists llvm=`real`；无 `io.path` backends→`real`；无 `Q-S5`；无 M2 退出宣称
- **下一窗：** M2 W108 — `Q-S5`；默认诚实收窄、**不**宣称 M2 退出

### Truth M2 W108 Q-S5 四类 multi-handle stress（ADR-252 · 已交）

- **主轴 / Roadmap ID：** `Q-S5` — 四类 handle multi-handle stress live_count 回基线（File/Sql/Tcp/Process；N=4）
- **钉测：** `q_s5_closeout` + CI；验收诚实收窄（≠ FileStream·Watcher·Pipe session / ≠ Listener·TLS）
- **质量门：** `Q-S5` `[x]`；**EC-M2-3 总勾仍开**；**不**宣称 M2 退出
- **冻结声明：** 无 session live_count 新 API；无 EC 总勾；无 M2 退出宣称
- **下一窗：** M2 W109 — EC-M2-3 session leak；默认**不**勾 EC-M2-3 总勾、**不**宣称 M2 退出

### Truth M2 W109 EC-M2-3-F Session Leak（ADR-253 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-3-F` — FileStream / FileWatcher / ChildPipe session live_count 首钉
- **钉测：** `ec_m2_3_{file_stream,file_watch,child_pipe}_leak` + `ec_m2_3_session_leak_closeout`；诊断 `*_live_count`/`*_force_close`
- **EC：** `EC-M2-3-F` `[x]`；**EC-M2-3 总勾仍开**；**不**宣称 M2 退出
- **冻结声明：** 无 HTTP body stream / Listener·TLS live_count；无产品 API 提升；无 M2 退出宣称
- **下一窗：** M2 W110 — HTTP body stream session leak；默认**不**勾 EC-M2-3 总勾

### Truth M2 W110 EC-M2-3-G HTTP Body Stream Session Leak（ADR-254 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-3-G` — HttpBodyStream session live_count 首钉
- **钉测：** `ec_m2_3_http_body_stream_leak` + `ec_m2_3_http_body_stream_leak_closeout`；诊断 `http_body_stream_live_count`/`force_close`
- **EC：** `EC-M2-3-G` `[x]`；**EC-M2-3 总勾仍开**；**不**宣称 M2 退出
- **冻结声明：** 无 Listener·TLS live_count；无产品 API 提升；无 server 流式 / wire chunked TE；无 M2 退出宣称
- **下一窗：** M2 W111 — Listener·TLS session leak；默认**不**勾 EC-M2-3 总勾

### Truth M2 W111 EC-M2-3-H TcpListener Session Leak（ADR-255 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-3-H` — 最小 TcpListener bind/close session live_count 首钉
- **钉测：** `ec_m2_3_tcp_listener_leak` + `ec_m2_3_tcp_listener_leak_closeout`；`TCP_LISTENERS` + 诊断 `tcp_listener_live_count`/`force_close`
- **EC：** `EC-M2-3-H` `[x]`；**EC-M2-3 总勾仍开**；**不**宣称 M2 退出
- **冻结声明：** 无 accept GA；无 TLS session / `net.tls.server` implemented；无产品 API 提升；无 M2 退出宣称
- **下一窗：** M2 W112 — EC-M2-3 总勾收口评估；默认停止再开 EC-M2-3-* 字母行（除非 Exit impact 宣称总勾）

### Truth M2 W112 EC-M2-3 Total Closeout（ADR-256 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-3` — 总勾收口（诚实收窄）
- **钉测：** `ec_m2_3_total_closeout`；已有产品 handle/session 泄漏钉齐；`net.tls.server` 保持 unsupported；无 accept GA
- **EC：** **`EC-M2-3` 总勾 `[x]`**；EC-M2-1/4 总勾仍开；**不**宣称 M2 退出
- **冻结声明：** 无 TLS server session / accept GA；无再开默认 EC-M2-3-* 字母行；无 M2 退出宣称
- **下一窗：** M2 W113 — EC-M2-1 总勾收口评估；默认**不**宣称 M2 退出

### Truth M2 W113 EC-M2-1 Total Closeout（ADR-257 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-1` — 总勾收口（诚实收窄）
- **钉测：** `ec_m2_1_total_closeout`；A–F 官方 stdlib 可交付 + Z 证据链齐；`io.path` 仍 partial；Http 无 typed throws 全迁
- **EC：** **`EC-M2-1` 总勾 `[x]`**；EC-M2-4 总勾仍开；**不**宣称 M2 退出
- **冻结声明：** 无全量 Path GA / PathExists·File*·wasm GA / Http typed-throws；无再开默认 EC-M2-1-* 字母行；无 M2 退出宣称
- **下一窗：** M2 W114 — EC-M2-4 总勾收口评估；默认**不**宣称 M2 退出

### Truth M2 W114 EC-M2-4 Total Closeout（ADR-258 · 已交）

- **主轴 / Roadmap ID：** `EC-M2-4` — 总勾收口（诚实收窄）
- **钉测：** `ec_m2_4_total_closeout`；A–H 加深轨 + Z 已签；四产品切片仍 `native=partial` / `wasm=host-assisted`
- **EC：** **`EC-M2-4` 总勾 `[x]`**；四总勾齐；**不**宣称 M2 退出
- **冻结声明：** 无 absolute SLA CI 硬门槛；无产品 backends=`real`；无全量 HostFn llvm GA；无再开默认 EC-M2-4-* 字母行；无 M2 退出宣称
- **下一窗：** M2 W115 — M2 退出（专用 exit charter；对齐 ADR-144）

### Truth M2 W115 Exit（ADR-259 · 已交）

- **主轴 / Roadmap ID：** M2 Exit — thin charter（对齐 ADR-144）
- **钉测：** `m2_exit_closeout`；四总勾齐 + 证据钉；开放债不挡退出
- **EC：** **M2 已退出**；无 EC-M3 勾选；无新 runtime
- **冻结声明：** 无 EC-M3-* 宣称；无再开默认 EC-M2-* 字母行；无 absolute SLA / backends=`real` / 全量 HostFn GA 冒充退出交付
- **下一窗：** W116 / M3 W1 — M3 thin start（`M3-A01`）→ **已交 ADR-260**；W117 silent-leak → **已交 ADR-261**；W118 deadline → **已交 ADR-262**；W119 task-local → **已交 ADR-263**；W120 reentrancy → **已交 ADR-264**；W121 await-hold → **已交 ADR-265**；W122 Channel close → **已交 ADR-266**；W123 backpressure → **已交 ADR-267**；W124 fairness → **已交 ADR-268**；W125 A04 closeout → **已交 ADR-269**；W126 A05 docs → **已交 ADR-270**；W127 A05 closeout → **已交 ADR-271**；W128 A01 closeout → **已交 ADR-272**；W129 A02 closeout → **已交 ADR-273**；W130 A03 closeout → **已交 ADR-274**；W131 B01 isolation → **已交 ADR-275**；W132 B01 deepen → **已交 ADR-276**；W133 B01 closeout → **已交 ADR-277**；W134 B02 adapter → **已交 ADR-278**；W135 B02 deepen → **已交 ADR-279**；W136 B02 closeout → **已交 ADR-280**；W137 C01 Mutex RAII → **已交 ADR-281**；W138 C01 blocking deepen → **已交 ADR-282**；W139 C01 closeout → **已交 ADR-283**；W140 C02 Semaphore → **已交 ADR-284**；现下一窗 = M3 W26（`M3-C02` deepen）

### Truth M3 W1 M3-A01 Task Tree First Nail（ADR-260 · 已交）

- **主轴 / Roadmap ID：** `M3-A01` — task tree 首钉
- **钉测：** `m3_a01_task_tree`；`Concurrency.taskGroupCancel` + HostFn `TaskGroupCancel`；cancel→wait 及时返回；join **C004**
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A01` `[~]` 首钉（叶子仍开）
- **冻结声明：** 无 deadline / actor reentrancy / Channel / 权限沙箱；无 EC-M3 勾选
- **下一窗：** M3 W2 — A01 silent-leak 加深（≠ EC-M3-2）→ **已交 ADR-261**

### Truth M3 W2 M3-A01 Silent-Leak Deepen（ADR-261 · 已交）

- **主轴 / Roadmap ID：** `M3-A01` — silent-leak 加深
- **钉测：** `m3_a01_silent_leak`；`Concurrency.liveTaskCount` + HostFn `TaskLiveCount`；wait/cancel+wait 回 baseline；unreaped detach → `LEAKED`
- **EC：** EC-M3-* 仍 `[ ]`（含 EC-M3-2）；`M3-A01` `[~]`（叶子仍开）
- **冻结声明：** 无 A01 `[x]` / 无 deadline 实现 / 无 B/C/D/E
- **下一窗：** M3 W3 — `M3-A02` deadline / task-local thin start → **已交 ADR-262**

### Truth M3 W3 M3-A02 Deadline Thin Start（ADR-262 · 已交）

- **主轴 / Roadmap ID：** `M3-A02` — deadline 首钉
- **钉测：** `m3_a02_deadline`；`Concurrency.setDeadlineMs` + HostFn `TaskSetDeadlineMs`；子继承；到期 → sleep 打断 / join **C004**
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A02` `[~]` 首钉（叶子仍开）；`M3-A01` 仍 `[~]`
- **冻结声明：** 无 TaskLocal GA / 无 A02 `[x]` / 无 A03+
- **下一窗：** M3 W4 — A02 task-local 加深 → **已交 ADR-263**

### Truth M3 W4 M3-A02 Task-Local Deepen（ADR-263 · 已交）

- **主轴 / Roadmap ID：** `M3-A02` — task-local 加深
- **钉测：** `m3_a02_task_local`；`Concurrency.taskLocalSet`/`taskLocalGet` + HostFn `TaskLocalSet`/`TaskLocalGet`；spawn 克隆继承；父子隔离
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A02` `[~]`（叶子仍开）；`M3-A01` 仍 `[~]`
- **冻结声明：** 无 `@TaskLocal` / 无 A02 `[x]` / 无 A03 实现
- **下一窗：** M3 W5 — `M3-A03` actor reentrancy thin start → **已交 ADR-264**

### Truth M3 W5 M3-A03 Actor Reentrancy Thin Start（ADR-264 · 已交）

- **主轴 / Roadmap ID：** `M3-A03` — actor reentrancy 首钉
- **钉测：** `m3_a03_reentrancy`；serial-no-reenter；同实例同步重入 → **C009**；`Concurrency.actorReentrancyPolicy` + HostFn `ActorReentrancyPolicy`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A03` `[~]`（叶子仍开）；`M3-A01`/`M3-A02` 仍 `[~]`
- **冻结声明：** 无 await-reentrancy GA / 无 A03 `[x]` / 无 A04+
- **下一窗：** M3 W6 — `M3-A03` await-reentrancy deepen → **已交 ADR-265**

### Truth M3 W6 M3-A03 Await-Hold Deepen（ADR-265 · 已交）

- **主轴 / Roadmap ID：** `M3-A03` — await-hold 加深
- **钉测：** `m3_a03_await_reentrancy`；policy `serial-no-reenter;await-hold`；handler sleep 期间无 mid-await mailbox 插队；并发 peek 见 `AFTER`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A03` `[~]`（叶子仍开）；`M3-A01`/`M3-A02` 仍 `[~]`
- **冻结声明：** 无 Swift await-reentrancy GA / 无 A03 `[x]` / 无 A04 实现
- **下一窗：** M3 W7 — `M3-A04` Channel thin start → **已交 ADR-266**

### Truth M3 W7 M3-A04 Channel Close Thin Start（ADR-266 · 已交）

- **主轴 / Roadmap ID：** `M3-A04` — Channel close 首钉
- **钉测：** `m3_a04_channel_close`；真断开 sender；drain 后 recv **Nil**；post-close send **C010**
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A04` `[~]`（叶子仍开）；A01–A03 仍 `[~]`
- **冻结声明：** 无 backpressure/fairness 满勾 / 无 A04 `[x]` / 无 A05+
- **下一窗：** M3 W8 — `M3-A04` backpressure deepen → **已交 ADR-267**

### Truth M3 W8 M3-A04 Backpressure Deepen（ADR-267 · 已交）

- **主轴 / Roadmap ID：** `M3-A04` — backpressure 加深
- **钉测：** `m3_a04_backpressure`；bounded `create(N)`；满缓冲 `trySend`→false；`length`/`capacity` 可观测
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A04` `[~]`（叶子仍开；fairness 未签）；A01–A03 仍 `[~]`
- **冻结声明：** 无 fairness 满勾 / 无 A04 `[x]` / 无 A05+
- **下一窗：** M3 W9 — `M3-A04` fairness product nail → **已交 ADR-268**

### Truth M3 W9 M3-A04 Fairness Product Nail（ADR-268 · 已交）

- **主轴 / Roadmap ID：** `M3-A04` — fairness 产品钉
- **钉测：** `m3_a04_fairness`；两路同时 ready → select RR 10/10；指纹 `select-fair-v1:round-robin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A04` `[~]`（三轴行为钉齐；叶子满勾 → W10）；A01–A03 仍 `[~]`
- **冻结声明：** 无 A04 `[x]` 本窗 / 无跨线程 RR GA / 无 A05+
- **下一窗：** M3 W10 — `M3-A04` leaf thin closeout → **已交 ADR-269**

### Truth M3 W10 M3-A04 Leaf Thin Closeout（ADR-269 · 已交）

- **主轴 / Roadmap ID：** `M3-A04` — 叶子薄收口
- **钉测：** `m3_a04_closeout`；三轴证据链（close / backpressure / fairness）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A04` `[x]`（诚实收窄）；A01–A03 仍 `[~]`；A05 → ADR-270 `[~]`
- **冻结声明：** 无 EC-M3 / 无跨线程 RR GA / 无 A05 实现 / 无 M3 退出
- **下一窗：** M3 W11 — `M3-A05` docs thin start → **已交 ADR-270**

### Truth M3 W11 M3-A05 Pool Layering Docs Thin Start（ADR-270 · 已交）

- **主轴 / Roadmap ID：** `M3-A05` — 业务 API vs 线程池旋钮分层（文档）
- **钉测：** `m3_a05_pool_layering`；结构化并发为主业务 API；池旋钮为高级运行时配置
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A05` `[~]`；A01–A03 仍 `[~]`；A04 `[x]`
- **冻结声明：** 无 A05 `[x]` 本窗 / 无删池 API / 无 B01 / 无 M3 退出
- **下一窗：** M3 W12 — `M3-A05` leaf thin closeout → **已交 ADR-271**

### Truth M3 W12 M3-A05 Leaf Thin Closeout（ADR-271 · 已交）

- **主轴 / Roadmap ID：** `M3-A05` — 叶子薄收口
- **钉测：** `m3_a05_closeout`；分层文档证据链（`m3_a05_pool_layering`）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A05` `[x]`（诚实收窄）；A01–A03 仍 `[~]`；A04 `[x]`
- **冻结声明：** 无 EC-M3 / 无删池 API / 无 B01 / 无 M3 退出
- **下一窗：** M3 W13 — `M3-A01` leaf thin closeout → **已交 ADR-272**

### Truth M3 W13 M3-A01 Leaf Thin Closeout（ADR-272 · 已交）

- **主轴 / Roadmap ID：** `M3-A01` — 叶子薄收口
- **钉测：** `m3_a01_closeout`；双轴证据链（task tree + silent-leak）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`（含 EC-M3-2）；`M3-A01` `[x]`（诚实收窄）；A02/A03 仍 `[~]`；A04/A05 `[x]`
- **冻结声明：** 无 EC-M3-2 / 无全量 SC GA / 无 A02–A03 `[x]` / 无 M3 退出
- **下一窗：** M3 W14 — `M3-A02` leaf thin closeout → **已交 ADR-273**

### Truth M3 W14 M3-A02 Leaf Thin Closeout（ADR-273 · 已交）

- **主轴 / Roadmap ID：** `M3-A02` — 叶子薄收口
- **钉测：** `m3_a02_closeout`；双轴证据链（deadline + task-local）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A02` `[x]`（诚实收窄）；A03 仍 `[~]`；A01/A04/A05 `[x]`
- **冻结声明：** 无 EC-M3 / 无 `@TaskLocal` GA / 无 A03 `[x]` / 无 M3 退出
- **下一窗：** M3 W15 — `M3-A03` leaf thin closeout → **已交 ADR-274**

### Truth M3 W15 M3-A03 Leaf Thin Closeout（ADR-274 · 已交）

- **主轴 / Roadmap ID：** `M3-A03` — 叶子薄收口
- **钉测：** `m3_a03_closeout`；双轴证据链（reentrancy + await-hold）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-A03` `[x]`（诚实收窄）；A01–A05 全 `[x]`
- **冻结声明：** 无 EC-M3 / 无 Swift await-reentrancy GA / 无 B01 实现 / 无 M3 退出
- **下一窗：** M3 W16 — `M3-B01` thin start → **已交 ADR-275**

### Truth M3 W16 M3-B01 Isolation Thin Start（ADR-275 · 已交）

- **主轴 / Roadmap ID：** `M3-B01` — async Task 池与 blocking 池隔离首钉
- **钉测：** `m3_b01_isolation`；`blocking_pool` + `Concurrency.blockingSleep` / `isolationPolicy`；Task pool=1 饿死负例
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B01` `[~]`；A01–A05 `[x]`
- **冻结声明：** 无 B01 `[x]` / 无 B02 / 无 epoll reactor / 无全 HostFn 改道 / 无 M3 退出
- **下一窗：** M3 W17 — `M3-B01` deepen → **已交 ADR-276**

### Truth M3 W17 M3-B01 Isolation Deepen（ADR-276 · 已交）

- **主轴 / Roadmap ID：** `M3-B01` — 可调 blocking 池容量 + 扩展隔离证据
- **钉测：** `m3_b01_pool_sizing`；`setBlockingPoolSize` / `blockingPoolSize`；指纹 `task-pool+blocking-pool-v1-sized`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B01` `[~]`；A01–A05 `[x]`
- **冻结声明：** 无 B01 `[x]` / 无 B02 / 无 epoll reactor / 无全 HostFn 改道 / 无 M3 退出
- **下一窗：** M3 W18 — `M3-B01` leaf thin closeout → **已交 ADR-277**

### Truth M3 W18 M3-B01 Leaf Thin Closeout（ADR-277 · 已交）

- **主轴 / Roadmap ID：** `M3-B01` — isolation + sizing 双轴证据链收口
- **钉测：** `m3_b01_closeout`；双轴证据（isolation + pool_sizing）→ roadmap `[x]`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B01` `[x]`（诚实收窄）；A01–A05 `[x]`
- **冻结声明：** 无 reactor GA / 无 B02 实现 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W19 — `M3-B02` thin start → **已交 ADR-278**

### Truth M3 W19 M3-B02 Async Adapter Thin Start（ADR-278 · 已交）

- **主轴 / Roadmap ID：** `M3-B02` — File/HTTP async adapter on `blocking_pool`
- **钉测：** `m3_b02_async_adapter`；`FileReadAsync` + HttpRequestAsync→blocking_pool；指纹 `blocking-pool-async-adapter-v1-thin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B02` `[~]`；B01 `[x]`
- **冻结声明：** 无 reactor GA / 无 Tcp async GA / 无同步 File·Http 改道 / 无 HttpRequestAsync capability TOML GA / 无 M3 退出
- **下一窗：** M3 W20 — `M3-B02` deepen → **已交 ADR-279**

### Truth M3 W20 M3-B02 Async Adapter Deepen（ADR-279 · 已交）

- **主轴 / Roadmap ID：** `M3-B02` — Tcp read async adapter on `blocking_pool`
- **钉测：** `m3_b02_tcp_async`；`TcpReadAsync` + `TcpStream.readAsync`；指纹 `blocking-pool-async-adapter-v1-tcp`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B02` 仍 `[~]`（当时）；≠ Tcp async GA / ≠ Write·Connect async
- **冻结声明：** 无 reactor GA / 无 B02 `[x]`（当时） / 无 HttpRequestAsync capability TOML / 无同步 Tcp 改道 / 无 M3 退出
- **下一窗：** M3 W21 — `M3-B02` leaf thin closeout → **已交 ADR-280**

### Truth M3 W21 M3-B02 Leaf Thin Closeout（ADR-280 · 已交）

- **主轴 / Roadmap ID：** `M3-B02` — File/HTTP thin + Tcp deepen 双轴证据链收口
- **钉测：** `m3_b02_closeout`（meta；证据 `m3_b02_async_adapter` + `m3_b02_tcp_async`）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-B02` `[x]`；B01 `[x]`
- **冻结声明：** 无 reactor GA / 无 TcpWrite·Connect async GA / 无 HttpRequestAsync capability TOML / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W22 — `M3-C01` thin start → **已交 ADR-281**

### Truth M3 W22 M3-C01 Mutex RAII Thin Start（ADR-281 · 已交）

- **主轴 / Roadmap ID：** `M3-C01` — MutexGuard RAII（scope-exit quiet unlock）
- **钉测：** `m3_c01_mutex_raii`；指纹原 `v1-thin`（加深后升档见 ADR-282）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C01` `[~]`；≠ C02
- **冻结声明：** 无 C01 `[x]` / 无 C02–C04 / 无 Mutex capability `real` GA / 无 M3 退出
- **下一窗：** M3 W23 — `M3-C01` deepen → **已交 ADR-282**

### Truth M3 W23 M3-C01 Mutex Blocking Deepen（ADR-282 · 已交）

- **主轴 / Roadmap ID：** `M3-C01` — 阻塞/争用 `lock_wait` + 跨 Task 证据
- **钉测：** `m3_c01_mutex_blocking`；指纹 `mutex-raii-guard-v1-blocking`；try-lock HostFn 保留
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C01` `[~]`（当时）；≠ C02
- **冻结声明：** 无 C01 `[x]`（当时）/ 无 C02–C04 / 无 capability `real` GA / 无 M3 退出
- **下一窗：** M3 W24 — `M3-C01` leaf thin closeout → **已交 ADR-283**

### Truth M3 W24 M3-C01 Leaf Thin Closeout（ADR-283 · 已交）

- **主轴 / Roadmap ID：** `M3-C01` — RAII thin + blocking deepen 双轴证据链收口
- **钉测：** `m3_c01_closeout`（meta；证据 `m3_c01_mutex_raii` + `m3_c01_mutex_blocking`）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C01` `[x]`；≠ C02
- **冻结声明：** 无 C02–C04 / 无 capability `real` GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W25 — `M3-C02` thin start → **已交 ADR-284**

### Truth M3 W25 M3-C02 Semaphore Thin Start（ADR-284 · 已交）

- **主轴 / Roadmap ID：** `M3-C02` — Semaphore create/wait/tryWait/signal
- **钉测：** `m3_c02_semaphore`；指纹曾为 `semaphore-v1-thin`；独立 SEMAPHORE_TABLE
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C02` `[~]`；≠ RWLock / ≠ Condvar 产品 / ≠ C02 `[x]`
- **冻结声明：** 无 C02 `[x]` / 无 RWLock·Condvar 产品 / 无 C03–C04 / 无 capability `real` GA / 无 M3 退出
- **下一窗：** M3 W26 — `M3-C02` deepen → **已交 ADR-285**

### Truth M3 W26 M3-C02 SemaphoreGuard RAII Deepen（ADR-285 · 已交）

- **主轴 / Roadmap ID：** `M3-C02` — SemaphoreGuard RAII（forget-signal / scope-exit）
- **钉测：** `m3_c02_semaphore_raii`；指纹 `semaphore-v1-raii`；`SemaphoreSignalQuiet` + `waitGuard`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C02` 仍 `[~]`（当时）；≠ RWLock / ≠ Condvar 产品
- **冻结声明：** 无 C02 `[x]`（当时）/ 无 RWLock·Condvar 产品 / 无 C03–C04 / 无 capability `real` GA / 无 M3 退出
- **下一窗：** M3 W27 — `M3-C02` leaf thin closeout → **已交 ADR-286**

### Truth M3 W27 M3-C02 Leaf Thin Closeout（ADR-286 · 已交）

- **主轴 / Roadmap ID：** `M3-C02` — thin + RAII 双轴证据链收口
- **钉测：** `m3_c02_closeout`（meta；证据 `m3_c02_semaphore` + `m3_c02_semaphore_raii`）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C02` `[x]`；≠ RWLock / ≠ Condvar 产品 / ≠ C03
- **冻结声明：** 无 C03–C04 / 无 RWLock·Condvar 产品 / 无 capability `real` GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W28 — `M3-C03` thin start → **已交 ADR-287**

### Truth M3 W28 M3-C03 Memory Ordering Docs Thin Start（ADR-287 · 已交）

- **主轴 / Roadmap ID：** `M3-C03` — Atomic + memory ordering 文档与 Model 对齐
- **钉测：** `m3_c03_memory_ordering`（meta；docs honesty）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C03` `[~]`（当时）；≠ 产品 Atomic API；≠ Wasm atomics GA；≠ C04
- **冻结声明：** 无 C03 `[x]`（当时）/ 无 C04 / 无产品 Atomic HostFn / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W29 — `M3-C03` leaf thin closeout → **已交 ADR-288**

### Truth M3 W29 M3-C03 Leaf Thin Closeout（ADR-288 · 已交）

- **主轴 / Roadmap ID：** `M3-C03` — docs 证据链收口
- **钉测：** `m3_c03_closeout`（meta；证据 `m3_c03_memory_ordering`）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C03` `[x]`；≠ 产品 Atomic API；≠ Wasm atomics GA；≠ C04
- **冻结声明：** 无 C04 / 无产品 Atomic HostFn / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W30 — `M3-C04` thin start → **已交 ADR-289**

### Truth M3 W30 M3-C04 Race/Stress CI Sampling Thin Start（ADR-289 · 已交）

- **主轴 / Roadmap ID：** `M3-C04` — race/stress CI 抽样 + 可复现样例库
- **钉测：** `m3_c04_race_stress`；样例库 `race_stress_samples.toml`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C04` `[~]`；≠ TSAN；≠ chaos 满勾（当时）
- **冻结声明：** 无 C04 `[x]` / 无 TSAN·loom GA / 无 chaos 满勾（当时）/ 无 M3-D / 无 M3 退出
- **下一窗：** M3 W31 — `M3-C04` deepen → **已交 ADR-290**

### Truth M3 W31 M3-C04 Chaos Sampling Deepen（ADR-290 · 已交）

- **主轴 / Roadmap ID：** `M3-C04` — TaskGroup cancel chaos 抽样
- **钉测：** `m3_c04_chaos`；样例库扩容 `kind=chaos`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C04` 仍 `[~]`；≠ TSAN；≠ 杀进程/网络分区 chaos GA；≠ C04 `[x]`
- **冻结声明：** 无 C04 `[x]` / 无 TSAN·loom GA / 无杀进程·网络分区农场 / 无 M3-D / 无 M3 退出
- **下一窗：** M3 W32 — `M3-C04` leaf thin closeout → **已交 ADR-291**

### Truth M3 W32 M3-C04 Leaf Thin Closeout（ADR-291 · 已交）

- **主轴 / Roadmap ID：** `M3-C04` — stress + chaos 双轴证据链收口
- **钉测：** `m3_c04_closeout`（meta）；证据钉 `m3_c04_race_stress` / `m3_c04_chaos`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-C04` `[x]`；≠ TSAN；≠ 杀进程/网络分区 chaos GA；≠ M3-D 实现
- **冻结声明：** 无 TSAN·loom GA / 无杀进程·网络分区农场 / 无 EC-M3 / 无 M3-D 实现 / 无 M3 退出
- **下一窗：** M3 W33 — `M3-D01` thin start → **已交 ADR-292**

### Truth M3 W33 M3-D01 Structured Logging Thin Start（ADR-292 · 已交）

- **主轴 / Roadmap ID：** `M3-D01` — min-level filter + `infoFields`
- **钉测：** `m3_d01_structured_logging`；指纹 `log-v1-thin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D01` `[~]`；≠ D02–D04；≠ D01 `[x]`；≠ EC-M3-4
- **冻结声明：** 无 D01 `[x]` / 无 D02–D04 / 无 handler·rotation·JSON GA / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W34 — `M3-D01` deepen → **已交 ADR-293**

### Truth M3 W34 M3-D01 All-Level Fields Deepen（ADR-293 · 已交）

- **主轴 / Roadmap ID：** `M3-D01` — 全 level `*Fields`
- **钉测：** `m3_d01_log_fields`；指纹 `log-v1-fields`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D01` 仍 `[~]`；≠ JSON GA；≠ D02–D04；≠ D01 `[x]`
- **冻结声明：** 无 D01 `[x]` / 无 JSON GA / 无 D02–D04 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W35 — `M3-D01` leaf thin closeout → **已交 ADR-294**

### Truth M3 W35 M3-D01 Leaf Thin Closeout（ADR-294 · 已交）

- **主轴 / Roadmap ID：** `M3-D01` — thin + fields 双轴证据链收口
- **钉测：** `m3_d01_closeout`（meta）；证据钉 `m3_d01_structured_logging` / `m3_d01_log_fields`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D01` `[x]`；≠ JSON GA；≠ D02–D04；≠ EC-M3-4
- **冻结声明：** 无 JSON GA / 无 D02–D04 实现 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W36 — `M3-D02` thin start → **已交 ADR-295**

### Truth M3 W36 M3-D02 Trace/Span Thin Start（ADR-295 · 已交）

- **主轴 / Roadmap ID：** `M3-D02` — Trace span + task/actor contextLine
- **钉测：** `m3_d02_trace_span`；指纹 `trace-v1-thin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D02` `[~]`；≠ OTel GA；≠ D03–D04；≠ D02 `[x]`
- **冻结声明：** 无 D02 `[x]` / 无 OTel GA / 无 D03–D04 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W37 — `M3-D02` deepen → **已交 ADR-296**

### Truth M3 W37 M3-D02 Nested + Log Context Deepen（ADR-296 · 已交）

- **主轴 / Roadmap ID：** `M3-D02` — nested LIFO + `Log.*Fields` auto context inject
- **钉测：** `m3_d02_trace_logctx`；指纹 `trace-v1-logctx`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D02` 仍 `[~]`；≠ OTel GA；≠ 跨 Task 继承；≠ D03–D04；≠ D02 `[x]`
- **冻结声明：** 无 D02 `[x]` / 无 OTel GA / 无跨 Task span 继承 / 无 D03–D04 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W38 — `M3-D02` leaf thin closeout → **已交 ADR-297**

### Truth M3 W38 M3-D02 Leaf Thin Closeout（ADR-297 · 已交）

- **主轴 / Roadmap ID：** `M3-D02` — thin + logctx 双轴证据链收口
- **钉测：** `m3_d02_closeout`（meta）；证据钉 `m3_d02_trace_span` / `m3_d02_trace_logctx`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D02` `[x]`；≠ OTel GA；≠ 跨 Task 继承；≠ D03–D04；≠ EC-M3-4
- **冻结声明：** 无 OTel GA / 无跨 Task span 继承 / 无 D03–D04 实现 / 无 EC-M3 / 无 M3 退出 / 无新 HostFn
- **下一窗：** M3 W39 — `M3-D03` thin start → **已交 ADR-298**

### Truth M3 W39 M3-D03 Metrics Counter Thin Start（ADR-298 · 已交）

- **主轴 / Roadmap ID：** `M3-D03` — 进程内命名 Counter + snapshot
- **钉测：** `m3_d03_metrics_counter`；指纹 `metrics-v1-thin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D03` `[~]`；≠ OTel/Prometheus GA；≠ Gauge/Histogram/labels；≠ D04；≠ D03 `[x]`
- **冻结声明：** 无 D03 `[x]` / 无 OTel/Prometheus GA / 无 D04 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W40 — `M3-D03` deepen → **已交 ADR-299**

### Truth M3 W40 M3-D03 Gauge + Labeled Counter Deepen（ADR-299 · 已交）

- **主轴 / Roadmap ID：** `M3-D03` — Gauge set/get + labeled counter
- **钉测：** `m3_d03_metrics_gauge_labels`；指纹 `metrics-v1-gauge-labels`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D03` 仍 `[~]`；≠ OTel/Prometheus GA；≠ Histogram；≠ D04；≠ D03 `[x]`
- **冻结声明：** 无 D03 `[x]` / 无 OTel/Prometheus GA / 无 Histogram / 无 D04 / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W41 — `M3-D03` leaf thin closeout → **已交 ADR-300**

### Truth M3 W41 M3-D03 Leaf Thin Closeout（ADR-300 · 已交）

- **主轴 / Roadmap ID：** `M3-D03` — thin + deepen 双轴证据链 meta 收口
- **钉测：** `m3_d03_closeout`；指纹保持 `metrics-v1-gauge-labels`（无 runtime 变更）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D03` `[x]`；≠ OTel/Prometheus GA；≠ Histogram；≠ D04；≠ EC-M3
- **冻结声明：** 无 OTel/Prometheus GA / 无 Histogram / 无 D04 实现 / 无 EC-M3 / 无 M3 退出 / 无新 HostFn
- **下一窗：** M3 W42 — `M3-D04` thin start → **已交 ADR-301**

### Truth M3 W42 M3-D04 Section Wall-Clock Profiler Thin Start（ADR-301 · 已交）

- **主轴 / Roadmap ID：** `M3-D04` — 命名区段墙钟 start/stop/elapsed + snapshot
- **钉测：** `m3_d04_profiler_sections`；指纹 `profiler-v1-thin`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D04` `[~]`；≠ 采样 GA；≠ CPU-time GA；≠ pprof export；≠ D04 `[x]`
- **冻结声明：** 无采样 / 无 CPU-time GA / 无 pprof export / 无 D04 `[x]` / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W43 — `M3-D04` deepen → **已交 ADR-302**

### Truth M3 W43 M3-D04 Process CPU Section Deepen（ADR-302 · 已交）

- **主轴 / Roadmap ID：** `M3-D04` — 区段进程 CPU start/stop/elapsed
- **钉测：** `m3_d04_profiler_cpu`；指纹 `profiler-v1-cpu`
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D04` 仍 `[~]`；≠ 采样 GA；≠ pprof export；≠ D04 `[x]`
- **冻结声明：** 无采样 / 无 pprof export / 无 D04 `[x]` / 无 EC-M3 / 无 M3 退出
- **下一窗：** M3 W44 — `M3-D04` leaf thin closeout → **已交 ADR-303**

### Truth M3 W44 M3-D04 Leaf Thin Closeout（ADR-303 · 已交）

- **主轴 / Roadmap ID：** `M3-D04` — thin + deepen 双轴证据链 meta 收口
- **钉测：** `m3_d04_closeout`；指纹保持 `profiler-v1-cpu`（无 runtime 变更）
- **EC：** EC-M3-* 仍 `[ ]`；`M3-D04` `[x]`；≠ 采样 GA；≠ pprof export；≠ EC-M3
- **冻结声明：** 无采样 / 无 pprof export / 无 EC-M3 / 无 M3 退出 / 无新 HostFn
- **下一窗：** M3 W45 — `EC-M3-4` thin start → **已交 ADR-304**

### Truth M3 W45 EC-M3-4 Observability Evidence Thin Start（ADR-304 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-4` — D01–D04 四叶 observability 证据链 meta 首钉
- **钉测：** `ec_m3_4_observability`（无 runtime）；指纹链 `log-v1-fields` / `trace-v1-logctx` / `metrics-v1-gauge-labels` / `profiler-v1-cpu`
- **EC：** EC-M3-4 总勾仍 `[ ]`（脚注 thin 已交）；EC-M3-1～3 仍 `[ ]`；≠ 采样 / ≠ pprof / ≠ OTel / ≠ M3 退出
- **冻结声明：** 无 EC-M3 总勾 / 无采样·pprof·OTel·JSON GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W46 — `EC-M3-4` leaf thin closeout → **已交 ADR-305**

### Truth M3 W46 EC-M3-4 Leaf Thin Closeout（ADR-305 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-4` — thin 证据链 meta 收口 → 总勾 `[x]`
- **钉测：** `ec_m3_4_closeout`（无 runtime）；保留 `ec_m3_4_observability` + D01–D04 closeout
- **EC：** **EC-M3-4 `[x]`**（诚实收窄）；EC-M3-1～3 仍 `[ ]`；≠ 采样 / ≠ pprof / ≠ OTel / ≠ M3 退出
- **冻结声明：** 无 EC-M3-1～3 勾选 / 无采样·pprof·OTel·JSON GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W47 — `EC-M3-2` thin start → **已交 ADR-306**

### Truth M3 W47 EC-M3-2 Silent-Leak Evidence Thin Start（ADR-306 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-2` — A01 task-tree + silent-leak 证据链 meta 首钉
- **钉测：** `ec_m3_2_silent_leak`（无 runtime）；诊断面 `liveTaskCount` / `TaskLiveCount`
- **EC：** EC-M3-2 总勾仍 `[ ]`（脚注 thin 已交）；EC-M3-1/3 仍 `[ ]`；EC-M3-4 保持 `[x]`；≠ async-let GA / ≠ 非 group 回收完备 / ≠ M3 退出
- **冻结声明：** 无 EC-M3-1/2/3 总勾 / 无 async-let·非 group GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W48 — `EC-M3-2` leaf thin closeout → **已交 ADR-307**

### Truth M3 W48 EC-M3-2 Leaf Thin Closeout（ADR-307 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-2` — thin 证据链 meta 收口 → 总勾 `[x]`
- **钉测：** `ec_m3_2_closeout`（无 runtime）；保留 `ec_m3_2_silent_leak` + A01 closeout
- **EC：** **EC-M3-2 `[x]`**（诚实收窄）；EC-M3-1/3 仍 `[ ]`；EC-M3-4 保持 `[x]`；≠ async-let GA / ≠ 非 group 回收完备 / ≠ M3 退出
- **冻结声明：** 无 EC-M3-1/3 勾选 / 无 async-let·非 group GA / 无新 HostFn / 无 M3 退出
- **下一窗：** M3 W49 — `M3-E01` thin start → **已交 ADR-308**

### Truth M3 W49 M3-E01 Permission Model Docs Thin Start（ADR-308 · 已交）

- **主轴 / Roadmap ID：** `M3-E01` — fs/net/env/process/plugin 默认策略文档首钉
- **钉测：** `m3_e01_permission_model`；文档 `docs/capability-sandbox-policy-v0.md`；指纹 `permission-policy-v0:default-deny-intent`
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E01` `[~]`；≠ 运行时强制；≠ E02–E03；≠ M3 退出
- **冻结声明：** 无 runtime default-deny / 无 CLI grant / 无 HostFn 闸门 / 无 EC-M3-3 / 无 M3 退出
- **下一窗：** M3 W50 — `M3-E01` leaf thin closeout → **已交 ADR-309**

### Truth M3 W50 M3-E01 Leaf Thin Closeout（ADR-309 · 已交）

- **主轴 / Roadmap ID：** `M3-E01` — 策略文档证据链收口（thin → `[x]`）
- **钉测：** `m3_e01_closeout`（证据：`m3_e01_permission_model` + `capability-sandbox-policy-v0`）
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E01` `[x]`；≠ 运行时强制；≠ E02–E03；≠ M3 退出
- **冻结声明：** 无 runtime default-deny / 无 CLI grant / 无 HostFn 闸门 / 无 EC-M3-3 / 无 M3 退出
- **下一窗：** M3 W51 — `M3-E02` thin start → **已交 ADR-310**

### Truth M3 W51 M3-E02 Permission Grants Thin Start（ADR-310 · 已交）

- **主轴 / Roadmap ID：** `M3-E02` — opt-in 权限沙箱 + `--allow-net` / `SYX_ALLOW_NET` + 未授权负例
- **钉测：** `m3_e02_permission_grants`；指纹 `permission-grants-v0:allow-net-cli`；诊断 `S024`（W52 起）
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E02` `[~]`；≠ 五域 GA；≠ E03；≠ M3 退出
- **冻结声明：** 无五域 default-deny GA / 无 `syx.toml` `[permissions]` 全解析 / 无 EC-M3-3 / 无 M3 退出
- **下一窗：** M3 W52 — `M3-E02` deepen → **已交 ADR-311**

### Truth M3 W52 M3-E02 Permission Grants Deepen（ADR-311 · 已交）

- **主轴 / Roadmap ID：** `M3-E02` — `fs` 授予 + `syx.toml` `[permissions]` 薄解析；诊断统一 `S024`
- **钉测：** `m3_e02_permission_fs`；指纹 `permission-grants-v0:allow-net-fs-toml`
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E02` 仍 `[~]`；≠ 五域 GA；≠ path allowlist；≠ E03；≠ M3 退出
- **冻结声明：** 无 E02 `[x]` / 无五域 GA / 无 env·process·plugin 闸门 / 无 EC-M3-3 / 无 M3 退出
- **下一窗：** M3 W53 — `M3-E02` leaf thin closeout → **已交 ADR-312**

### Truth M3 W53 M3-E02 Leaf Thin Closeout（ADR-312 · 已交）

- **主轴 / Roadmap ID：** `M3-E02` — thin+deepen 证据链收口（`[~]`→`[x]`）
- **钉测：** `m3_e02_closeout`（证据：`m3_e02_permission_grants` + `m3_e02_permission_fs`）
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E02` `[x]`；≠ 五域 GA；≠ E03；≠ M3 退出
- **冻结声明：** 无五域 GA / 无 E03 实现 / 无 EC-M3-3 / 无 M3 退出
- **下一窗：** M3 W54 — `M3-E03` thin start → **已交 ADR-313**

### Truth M3 W54 M3-E03 HostFn Capability Gate Thin Start（ADR-313 · 已交）

- **主轴 / Roadmap ID：** `M3-E03` — opt-in 编译期 HostFn capability table 联动（`status=unsupported` → `S026`）
- **钉测：** `m3_e03_capability_gate`；指纹 `hostfn-capability-gate-v0:compile-unsupported`；CLI `--ir-capability-gate` / `SYX_IR_CAPABILITY_GATE`
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E03` `[~]`；≠ 默认全局闸；≠ backend 列 GA；≠ 加载期闸；≠ M3 退出
- **冻结声明：** 无 EC-M3-1/3 勾选 / 无 E03 `[x]` / 无默认开启闸 / 无五域 GA / 无 M3 退出
- **下一窗：** M3 W55 — `M3-E03` leaf thin closeout → **已交 ADR-314**

### Truth M3 W55 M3-E03 Leaf Thin Closeout（ADR-314 · 已交）

- **主轴 / Roadmap ID：** `M3-E03` — thin 证据链收口（`[~]`→`[x]`）
- **钉测：** `m3_e03_closeout`（证据：`m3_e03_capability_gate` + `hostfn-capability-gate-v0:compile-unsupported`）
- **EC：** EC-M3-3 仍 `[ ]`；`M3-E03` `[x]`；≠ 默认全局闸；≠ backend 列 GA；≠ 加载期闸；≠ M3 退出
- **冻结声明：** 无 EC-M3-1/3 勾选 / 无默认开启闸 / 无五域 GA / 无 M3 退出
- **下一窗：** M3 W56 — `EC-M3-3` thin start → **已交 ADR-315**

### Truth M3 W56 EC-M3-3 Capability Sandbox Evidence Thin Start（ADR-315 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-3` — E01–E03 证据链 meta 首钉（总勾仍 `[ ]`）
- **钉测：** `ec_m3_3_capability_sandbox`（证据：`m3_e01_closeout` / `m3_e02_closeout` / `m3_e03_closeout` + 三指纹）
- **EC：** EC-M3-3 仍 `[ ]`（脚注 thin）；EC-M3-1 仍 `[ ]`；EC-M3-2/4 `[x]`；≠ 五域 GA；≠ 默认全局闸；≠ M3 退出
- **冻结声明：** 无 EC-M3-1/3 勾选 / 无五域 GA / 无默认开启闸 / 无 M3 退出
- **下一窗：** M3 W57 — `EC-M3-3` leaf thin closeout → **已交 ADR-316**

### Truth M3 W57 EC-M3-3 Leaf Thin Closeout（ADR-316 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-3` — thin 证据链收口（`[ ]`→`[x]`）
- **钉测：** `ec_m3_3_closeout`（证据：`ec_m3_3_capability_sandbox` + E01–E03 closeouts + 三指纹）
- **EC：** EC-M3-3 `[x]`；EC-M3-1 仍 `[ ]`；EC-M3-2/4 `[x]`；≠ 五域 GA；≠ 默认全局闸；≠ M3 退出
- **冻结声明：** 无 EC-M3-1 勾选 / 无五域 GA / 无默认开启闸 / 无 M3 退出
- **下一窗：** M3 W58 — `EC-M3-1` thin start → **已交 ADR-317**

### Truth M3 W58 EC-M3-1 Concurrency Stress Repro Evidence Thin Start（ADR-317 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-1` — HTTP Criterion + A03/A04/C04 证据链 meta 首钉（总勾仍 `[ ]`）
- **钉测：** `ec_m3_1_stress_repro`（证据：`http_response_criterion` / `m3_a03_closeout` / `m3_a04_closeout` / `m3_c04_closeout` + C04 samples；诊断面继承 `liveTaskCount` / `profiler-v1-cpu`）
- **EC：** EC-M3-1 仍 `[ ]`（脚注 thin）；EC-M3-2/3/4 `[x]`；≠ load-farm GA；≠ 绝对 SLA 硬闸；≠ TSAN；≠ M3 退出
- **冻结声明：** 无 EC-M3-1 总勾 / 无 load-farm·SLA·TSAN GA / 无新 HostFn / 无新 Criterion 产品 bench / 无 M3 退出
- **下一窗：** M3 W59 — `EC-M3-1` leaf thin closeout → **已交 ADR-318**

### Truth M3 W59 EC-M3-1 Leaf Thin Closeout（ADR-318 · 已交）

- **主轴 / Roadmap ID：** `EC-M3-1` — thin 证据链收口（`[ ]`→`[x]`）
- **钉测：** `ec_m3_1_closeout`（证据：`ec_m3_1_stress_repro` + HTTP Criterion + A03/A04/C04 + C04 samples）
- **EC：** **EC-M3-1～4 全 `[x]`**；≠ load-farm GA；≠ 绝对 SLA 硬闸；≠ TSAN；≠ M3 退出
- **冻结声明：** 无 M3 退出 / 无 load-farm·SLA·TSAN GA / 无新 HostFn / 无新 Criterion 产品 bench
- **下一窗：** M3 W60 — **M3 exit**（专用窗）→ **已交 ADR-319**

### Truth M3 W60 Exit Thin Charter（ADR-319 · 已交）

- **主轴 / Roadmap ID：** `M3 exit` — 四 EC 已齐前提下的里程碑薄退出
- **钉测：** `m3_exit_closeout`（证据：`ec_m3_1_closeout` / `ec_m3_2_closeout` / `ec_m3_3_closeout` / `ec_m3_4_closeout`）
- **EC：** **M3 已退出**；EC-M3-1～4 保持 `[x]`；开放债不挡；≠ load-farm / SLA / TSAN / 五域 GA / 采样·OTel 冒充退出交付；≠ SynUI / Farm / Dist
- **冻结声明：** 无 M4 EC 勾选 / 无 SynUI·Farm·Dist / 无新 HostFn / 无再开默认 M3 EC 收口列车
- **下一窗：** M4 W1 — `M4-L01` thin start → **已交 ADR-320**

### Truth M4 W1 M4-L01 First-Order Where Thin Start（ADR-320 · 已交）

- **主轴 / Roadmap ID：** `M4-L01` — 泛型函数一阶 `where T: P` + 稳定 S010（叶子仍 `[~]`）
- **钉测：** `m4_l01_first_order_where`（pass：`Int`+`Equatable`；fail：非 Equatable 用户类型 → `S010`；指纹 `generic-where-v0:first-order-s010`）
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L01` `[~]`；≠ L02；≠ `where T == U`；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4 勾选 / 无 L01 `[x]` / 无 L02 / 无角括号界 / 无 M4 退出
- **下一窗：** M4 W2 — `M4-L01` leaf thin closeout → **已交 ADR-321**

### Truth M4 W2 M4-L01 Leaf Thin Closeout（ADR-321 · 已交）

- **主轴 / Roadmap ID：** `M4-L01` — thin 证据链收口（`[~]`→`[x]`）
- **钉测：** `m4_l01_closeout`（证据：`m4_l01_first_order_where`；指纹 `generic-where-v0:first-order-s010`）
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L01` `[x]`；≠ L02；≠ `where T == U`；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4 勾选 / 无 L02 / 无角括号界 / 无 M4 退出
- **下一窗：** M4 W3 — `M4-L02` thin start → **已交 ADR-322**

### Truth M4 W3 M4-L02 Associated Type Thin Start（ADR-322 · 已交）

- **主轴 / Roadmap ID：** `M4-L02` — 协议关联类型约束 + 符合侧 `typealias` 见证 + 稳定 S012（叶子仍 `[~]`；**assoc-only**）
- **钉测：** `m4_l02_assoc_type`（pass：`Item = Int`；fail：`Item = Box` → `S012`；指纹 `assoc-type-v0:typealias-s012`）
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L02` `[~]`；≠ conditional GA；≠ 跨模块矩阵；≠ existential；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4 勾选 / 无 L02 `[x]` / 无 conditional conformance / 无 M4 退出
- **下一窗：** M4 W4 — `M4-L02` leaf thin closeout → **已交 ADR-323**

### Truth M4 W4 M4-L02 Leaf Thin Closeout（ADR-323 · 已交）

- **主轴 / Roadmap ID：** `M4-L02` — thin 证据链收口（`[~]`→`[x]`；**assoc-only**）
- **钉测：** `m4_l02_closeout`（证据：`m4_l02_assoc_type`；指纹 `assoc-type-v0:typealias-s012`）
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L02` `[x]`；≠ conditional GA；≠ 跨模块矩阵；≠ existential；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4 勾选 / 无 conditional conformance GA / 无 M4 退出
- **下一窗：** M4 W5 — conditional conformance thin start → **已交 ADR-324**

### Truth M4 W5 Conditional Conformance Thin Start（ADR-324 · 已交）

- **主轴：** conditional conformance thin（L02 deepen；L02 仍 `[x]` assoc-only）
- **钉测：** `m4_conditional_conformance`；指纹 `conditional-conform-v0:array-element-s010`；稳定 **S010**
- **EC：** EC-M4-* 仍 `[ ]`；≠ conditional GA；≠ L03；≠ 跨模块矩阵；≠ SynUI；≠ M4 退出；不重开 L02 `[~]`
- **冻结声明：** 无 EC-M4 勾选 / 无 conditional GA / 无 L03 / 无 M4 退出
- **下一窗：** M4 W6 — conditional leaf thin closeout → **已交 ADR-325**

### Truth M4 W6 Conditional Conformance Leaf Thin Closeout（ADR-325 · 已交）

- **主轴：** conditional conformance leaf thin closeout（L02 deepen 收口；L02 仍 `[x]`）
- **钉测：** `m4_conditional_closeout`（证据：`m4_conditional_conformance`；指纹 `conditional-conform-v0:array-element-s010`）
- **EC：** EC-M4-* 仍 `[ ]`；L03 仍 `[ ]`；≠ conditional GA；≠ SynUI；≠ M4 退出；不重开 L02 `[~]`
- **冻结声明：** 无 EC-M4 勾选 / 无 L03 thin 实现 / 无 conditional GA / 无 M4 退出
- **下一窗：** M4 W7 — `M4-L03` thin start → **已交 ADR-326**

### Truth M4 W7 M4-L03 Existential `any` Binding Thin Start（ADR-326 · 已交）

- **主轴 / Roadmap ID：** `M4-L03` — `any Equatable` 绑定赋值 + 稳定 S014（叶子 `[~]`）
- **钉测：** `m4_l03_existential_any`；指纹 `existential-any-v0:binding-s014`
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L03` `[~]`；≠ full opening；≠ `-> some/any` 返回门控；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4 勾选 / 无 L03 `[x]` / 无 return opaque gate / 无 M4 退出
- **下一窗：** M4 W8 — `M4-L03` leaf thin closeout → **已交 ADR-327**

### Truth M4 W8 M4-L03 Leaf Thin Closeout（ADR-327 · 已交）

- **主轴 / Roadmap ID：** `M4-L03` — thin 证据链收口（`[~]`→`[x]`；**binding-only**）
- **钉测：** `m4_l03_closeout`（证据：`m4_l03_existential_any`；指纹 `existential-any-v0:binding-s014`）
- **EC：** EC-M4-* 仍 `[ ]`；`M4-L03` `[x]`；≠ full opening；≠ SynUI；≠ M4 退出；三叶齐 ≠ EC-M4-L
- **冻结声明：** 无 EC-M4 勾选 / 无 full opening GA / 无 M4 退出
- **下一窗：** M4 W9 — EC-M4-L thin start → **已交 ADR-328**

### Truth M4 W9 EC-M4-L Protocol Layer Evidence Thin Start（ADR-328 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-L` — 协议层证据链薄首钉（assoc typealias + Array conditional + `any` binding）
- **钉测：** `ec_m4_l_protocol_layer`（证据：`m4_l02_closeout` / `m4_conditional_closeout` / `m4_l03_closeout`；指纹 S012 / S010 / S014）
- **EC：** EC-M4-L 总勾仍 `[ ]`；其余 EC-M4-* 仍 `[ ]`；≠ full opening GA；≠ conditional GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-L `[x]` / 无 opening GA / 无 M4 退出
- **下一窗：** M4 W10 — EC-M4-L leaf thin closeout → **已交 ADR-329**

### Truth M4 W10 EC-M4-L Leaf Thin Closeout（ADR-329 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-L` — thin 证据链收口（`[ ]`→`[x]`；协议层切片）
- **钉测：** `ec_m4_l_closeout`（证据：`ec_m4_l_protocol_layer` + L02/conditional/L03 closeout；指纹 S012 / S010 / S014）
- **EC：** EC-M4-L `[x]`；其余 EC-M4-* 仍 `[ ]`；≠ full opening GA；≠ conditional GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无其余 EC-M4 勾选 / 无 opening GA / 无 M4 退出
- **下一窗：** M4 W11 — M4-B01 thin start → **已交 ADR-330**

### Truth M4 W11 M4-B01 Static Exclusive Borrow Thin Start（ADR-330 · 已交）

- **主轴 / Roadmap ID：** `M4-B01` — 静态独占 borrow + 稳定 S011（叶子 `[~]`）
- **钉测：** `m4_b01_exclusive_borrow`；指纹 `borrow-exclusive-v0:double-mut-s011`
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B01` `[~]`；≠ NLL；≠ noncopyable；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 B01 `[x]` / 无 NLL / 无 M4 退出
- **下一窗：** M4 W12 — M4-B01 leaf thin closeout → **已交 ADR-331**

### Truth M4 W12 M4-B01 Leaf Thin Closeout（ADR-331 · 已交）

- **主轴 / Roadmap ID：** `M4-B01` — thin 证据链收口（`[~]`→`[x]`）
- **钉测：** `m4_b01_closeout`（证据：`m4_b01_exclusive_borrow`；指纹 `borrow-exclusive-v0:double-mut-s011`）
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B01` `[x]`；≠ NLL；≠ noncopyable；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 NLL / 无 B02 实现 / 无 M4 退出
- **下一窗：** M4 W13 — M4-B02 thin start → **已交 ADR-332**

### Truth M4 W13 M4-B02 Move-Consume Ownership Thin Start（ADR-332 · 已交）

- **主轴 / Roadmap ID：** `M4-B02` — String by-value 消费 + use-after-move S011（叶子 `[~]`；**move-consume-only**）
- **钉测：** `m4_b02_move_consume`；指纹 `move-consume-v0:string-s011`
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B02` `[~]`；≠ `~Copyable`；≠ assignment-move；≠ NLL；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 B02 `[x]` / 无 `~Copyable` / 无 M4 退出
- **下一窗：** M4 W14 — M4-B02 leaf thin closeout → **已交 ADR-333**

### Truth M4 W14 M4-B02 Leaf Thin Closeout（ADR-333 · 已交）

- **主轴 / Roadmap ID：** `M4-B02` — thin 证据链收口（`[~]`→`[x]`；**move-consume-only**）
- **钉测：** `m4_b02_closeout`（证据：`m4_b02_move_consume`；指纹 `move-consume-v0:string-s011`）
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B02` `[x]`；≠ `~Copyable`；≠ NLL；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 `~Copyable` GA / 无 B03 / 无 M4 退出
- **下一窗：** M4 W15 — M4-B03 thin start → **已交 ADR-334**

### Truth M4 W15 M4-B03 Language-Complete Borrow Non-Goal Thin Start（ADR-334 · 已交）

- **主轴 / Roadmap ID：** `M4-B03` — language-complete borrow **明确非目标**（叶子 `[~]`；**nongoal**；完备/NLL deepen → EC-M4-B）
- **钉测：** `m4_b03_nongoal`；指纹 `borrow-lang-complete-v0:nongoal`
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B03` `[~]`；≠ NLL GA；≠ language-complete GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 B03 `[x]` / 无 NLL 实现 / 无 M4 退出
- **下一窗：** M4 W16 — M4-B03 leaf thin closeout → **已交 ADR-335**

### Truth M4 W16 M4-B03 Leaf Thin Closeout（ADR-335 · 已交）

- **主轴 / Roadmap ID：** `M4-B03` — thin 证据链收口（`[~]`→`[x]`；**nongoal**）
- **钉测：** `m4_b03_closeout`（证据：`m4_b03_nongoal`；指纹 `borrow-lang-complete-v0:nongoal`）
- **EC：** EC-M4-B 仍 `[ ]`；`M4-B03` `[x]`；≠ NLL GA；≠ language-complete GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B / 无 NLL 实现 / 无 language-complete GA / 无 M4 退出
- **下一窗：** M4 W17 — EC-M4-B thin start → **已交 ADR-336**

### Truth M4 W17 EC-M4-B Named Move-Consume Thin Start（ADR-336 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-B` — Named (`class`) by-value 消费 + use-after-move S011（总勾仍 `[ ]`；**named-move-layer**）
- **钉测：** `ec_m4_b_named_move`；指纹 `move-consume-v0:named-s011`
- **EC：** EC-M4-B 仍 `[ ]`；≠ NLL；≠ `~Copyable` GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-B `[x]` / 无 NLL / 无 `~Copyable` / 无 M4 退出
- **下一窗：** M4 W18 — EC-M4-B leaf thin closeout → **已交 ADR-337**

### Truth M4 W18 EC-M4-B Leaf Thin Closeout（ADR-337 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-B` — thin 证据链收口（`[ ]`→`[x]`；**named-move-layer**）
- **钉测：** `ec_m4_b_closeout`（证据：`ec_m4_b_named_move`；指纹 `move-consume-v0:named-s011` + `move-consume-v0:string-s011`）
- **EC：** EC-M4-B `[x]`；其余 EC-M4-M/P/X 仍 `[ ]`；≠ NLL GA；≠ `~Copyable` GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无其余 EC / 无 NLL / 无 `~Copyable` / 无 M4 退出
- **下一窗：** M4 W19 — EC-M4-M thin start → **已交 ADR-338**

### Truth M4 W19 EC-M4-M Macro Sandbox Thin Start（ADR-338 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-M` — 宏 sandbox 薄首钉（默认 reexec + WASM E004）
- **钉测：** `ec_m4_m_macro_sandbox`；指纹 `macro-sandbox-v0:reexec-default`（并存 `macro-external-v2:plugin-prod`）
- **EC：** EC-M4-M 仍 `[ ]`；≠ source-map GA；≠ incremental-binary GA；≠ M4-M01–M04 满勾；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-M `[x]` / 无 source-map / 无 incremental-binary / 无 M4 退出
- **下一窗：** M4 W20 — EC-M4-M leaf thin closeout → **已交 ADR-339**

### Truth M4 W20 EC-M4-M Leaf Thin Closeout（ADR-339 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-M` — thin 证据链收口（`[ ]`→`[x]`；**sandbox-reexec-layer**）
- **钉测：** `ec_m4_m_closeout`（证据：`ec_m4_m_macro_sandbox`；指纹 `macro-sandbox-v0:reexec-default`）
- **EC：** EC-M4-M `[x]`；其余 EC-M4-P/X 仍 `[ ]`；≠ source-map GA；≠ incremental-binary GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无其余 EC / 无 source-map / 无 incremental-binary / 无 M4 退出
- **下一窗：** M4 W21 — EC-M4-P thin start → **已交 ADR-340**

### Truth M4 W21 EC-M4-P Signature Verify Thin Start（ADR-340 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-P` — 签名校验薄首钉（ed25519 checksum pass/deny）
- **钉测：** `ec_m4_p_signature_verify`；指纹 `pkg-sig-v0:ed25519-checksum`
- **EC：** EC-M4-P 仍 `[ ]`；≠ trust-root GA；≠ solver GA；≠ frozen GA；≠ M4-P01–P04 满勾；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-P `[x]` / 无 trust-root / 无 solver / 无 frozen / 无 M4 退出
- **下一窗：** M4 W22 — EC-M4-P leaf thin closeout → **已交 ADR-341**

### Truth M4 W22 EC-M4-P Leaf Thin Closeout（ADR-341 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-P` — thin 证据链收口（`[ ]`→`[x]`；**signature-verify-layer**）
- **钉测：** `ec_m4_p_closeout`（证据：`ec_m4_p_signature_verify`；指纹 `pkg-sig-v0:ed25519-checksum`）
- **EC：** EC-M4-P `[x]`；EC-M4-X 仍 `[ ]`；≠ trust-root GA；≠ solver GA；≠ frozen GA；≠ SynUI；≠ M4 退出
- **冻结声明：** 无 EC-M4-X / 无 trust-root / 无 solver / 无 frozen / 无 M4 退出
- **下一窗：** M4 W23 — EC-M4-X thin start → **已交 ADR-342**

### Truth M4 W23 EC-M4-X Product-Gate Thin Start（ADR-342 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-X` — 产品门薄首钉（本里程碑 defer；不单选 WS/gRPC/UI）
- **钉测：** `ec_m4_x_product_gate`；指纹 `ec-m4-x-v0:product-gate`
- **EC：** EC-M4-X 仍 `[ ]`；≠ 真 WS；≠ 真 gRPC；≠ SynUI 解冻；≠ M4-X01–X04 满勾；≠ M4 退出
- **冻结声明：** 无 EC-M4-X `[x]` / 无真协议 GA / 无 SynUI 解冻 / 无 M4 退出
- **下一窗：** M4 W24 — EC-M4-X leaf thin closeout → **已交 ADR-343**

### Truth M4 W24 EC-M4-X Leaf Thin Closeout（ADR-343 · 已交）

- **主轴 / Roadmap ID：** `EC-M4-X` — thin 证据链收口（`[ ]`→`[x]`；**product-gate-layer**）
- **钉测：** `ec_m4_x_closeout`（证据：`ec_m4_x_product_gate`；指纹 `ec-m4-x-v0:product-gate`）
- **EC：** EC-M4-X `[x]`；M4-X01–X04 仍 `[ ]`；≠ 真 WS GA；≠ 真 gRPC GA；≠ SynUI GA；≠ M4 退出
- **冻结声明：** 无 M4 退出 / 无真协议 GA / 无 SynUI 解冻 / 无 M4-X01–X04 满勾
- **下一窗：** M4 W25 — M4 exit thin → **已交 ADR-344**

### Truth M4 W25 M4 Exit Thin Charter（ADR-344 · 已交）

- **主轴 / Roadmap ID：** `M4 exit` — 五 EC 总勾齐后专用薄退出
- **钉测：** `m4_exit_closeout`（证据：`ec_m4_l/b/m/p/x_closeout`）
- **EC：** EC-M4-L/B/M/P/X 均 `[x]`（收窄口径）；M4-X01–X04 仍 `[ ]`；开放债不挡；≠ 真 WS/gRPC/SynUI GA
- **冻结声明：** 无 SynUI / Farm / Dist 解冻 / 无默认 M5 首叶 / 无再开默认 M4 EC 列车
- **下一窗：** Truth M0–M4 列车收官 → **Wave A W1（ADR-345）**

### Wave A W1 正确性地板加深（ADR-345 · 已交）

- **主轴：** A-01 收口 · A-02 Suite-2 `file_exists_roundtrip` · A-05 File Q-S1（`hostfn_qs1_matrix.toml` / `qs1_file`）· A-03 GA 证据政策 · C-10 audit smoke 叙事
- **钉测：** `suite2_file_exists_roundtrip` · `qs1_file_*` · `gen_ir_capabilities.py --check`
- **指纹：** `qs1-file-v0:open-close-read-write`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 SynUI/Farm/Dist 解冻；无全量 Q-L2；无 HTTP/Sql/Tcp/Process Q-S1；无默认 M5
- **下一窗：** Wave A W2（ADR-346）→ **已交**

### Wave A W2 正确性地板加深（ADR-346 · 已交）

- **主轴：** A-02 Suite-2 `file_overwrite_roundtrip` · A-05 Http Q-S1（`qs1-http-v0:get-success-refuse` / `qs1_http`）· A-03 exact 转换（resultbuilder optional + deinit scope）
- **钉测：** `suite2_file_overwrite_roundtrip` · `qs1_http_*` · `p103_resultbuilder_build_optional_stdout` · `p101_driver_lang_deinit_auto_scope_exit`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 array/dict SSA 解 skip；无 Sql/Tcp/Process Q-S1；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无默认 M5
- **下一窗：** Wave A W3（ADR-347）→ **已交**

### Wave A W3 正确性地板加深（ADR-347 · 已交）

- **主轴：** A-05 Tcp Q-S1（`qs1-tcp-v0:connect-success-refuse` / `qs1_tcp`）· A-03 exact（weak_nil_on_drop + array_sort_by）· A-02 **本窗跳过** · MutRefCell weak / deinit-mark 修复
- **钉测：** `qs1_tcp_*` · `p103_weak_nil_on_drop_stdout` · `p98_driver_array_sort_by_named_fn_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 Sql/Process Q-S1；无 File/Http llvm/wasm covered；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无默认 M5
- **下一窗：** Wave A W4（ADR-348）→ **已交**

### Wave A W4 正确性地板加深（ADR-348 · 已交）

- **主轴：** A-05 Process Q-S1（`qs1-process-v0:spawn-success-fail` / `qs1_process`）· A-03 exact（fmt + channel）· A-02 **本窗跳过**
- **钉测：** `qs1_process_*` · `p98_driver_fmt_host_format` · `p98_driver_channel_send_recv_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 Sql Q-S1；无 File/Http/Tcp/Process llvm→covered；无 fingerprint 海量改写；无 weak/MutRefCell 再改；无 SynUI/Farm/Dist；无默认 M5
- **下一窗：** Wave A W6（ADR-350）→ **已交**

### Wave A W5 正确性地板加深（ADR-349 · 已交）

- **主轴：** A-05 Sql Q-S1（`qs1-sql-v0:open-close-success-fail` / `qs1_sql`）· A-03 exact（mutex + expect_equal）· A-02 **本窗跳过**
- **钉测：** `qs1_sql_*` · `p98_driver_mutex_lock_host` · `p98_driver_testing_expect_equal_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 Sql pool/txn/Orm；无各包 llvm/wasm covered；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无默认 M5
- **下一窗：** Wave A W6（ADR-350）→ **已交**

### Wave A W6 正确性地板加深（ADR-350 · 已交）

- **主轴：** A-05 FileClose llvm 首格（`qs1-file-llvm-v0:close-bad-handle` / native exact）· A-03 exact（testing_test + log_info）· A-02 **本窗跳过** · native CallHost try/catch 接线（`syx_rt_take_catch` / `syx_rt_throw` + try-handler frames）
- **钉测：** `qs1_file_close_bad_handle_native_llvm` · `p98_driver_testing_test_host` · `p98_driver_log_info_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无全包 llvm/wasm covered；无 File* `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W7（ADR-351）→ **已交**

### Wave A W7 正确性地板加深（ADR-351 · 已交）

- **主轴：** A-05 FileRead llvm 单格（`qs1-file-llvm-v0:read-bad-handle` / native exact）· A-03 exact（process_exec + process_exec_output）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**
- **钉测：** `qs1_file_read_bad_handle_native_llvm` · `p98_driver_process_exec_host` · `p98_driver_process_exec_output_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无全包 llvm/wasm covered；无 File* `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W8（ADR-352）→ **已交**

### Wave A W8 正确性地板加深（ADR-352 · 已交）

- **主轴：** A-05 FileWrite llvm 单格（`qs1-file-llvm-v0:write-bad-handle` / native exact）· A-03 exact（spawn_wait + reflect_type）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**
- **钉测：** `qs1_file_write_bad_handle_native_llvm` · `p98_driver_process_spawn_wait_host` · `p98_driver_reflect_type_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无全包 llvm/wasm covered；无 File* `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W9（ADR-353）→ **已交**

### Wave A W9 正确性地板加深（ADR-353 · 已交）

- **主轴：** A-05 FileOpen llvm 单格（`qs1-file-llvm-v0:open-missing` / native exact；File 批 llvm 四格齐）· A-03 exact（codable_l14_roundtrip + deinit_arc_overwrite）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**
- **钉测：** `qs1_file_open_missing_native_llvm` · `p98_driver_codable_l14_embedded_roundtrip` · `p102_driver_lang_deinit_arc_on_overwrite`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / 非 File llvm covered；无 File* `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W10（ADR-354）→ **已交**

### Wave A W10 正确性地板加深（ADR-354 · 已交）

- **主轴：** A-05 HttpGet llvm 首格（`qs1-http-llvm-v0:get-refuse` / native refuse exact）· A-03 exact（datetime_iso + heap_string_offheap）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback-success llvm **本窗跳过**
- **钉测：** `qs1_http_get_refuse_native_llvm` · `p98_driver_datetime_iso_host` · `p98_driver_heap_string_offheap_regression`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Tcp+ llvm covered；无 HttpGet `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W11（ADR-355）→ **已交**

### Wave A W11 正确性地板加深（ADR-355 · 已交）

- **主轴：** A-05 TcpConnect llvm 首格（`qs1-tcp-llvm-v0:connect-refuse` / native refuse exact）· A-03 exact（file_watch + tcp_connect）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · native：`JumpIfFalse` 用 last SSA（非 HashMap 无序）；无 try 的 CallHost 不分 `ft*`；Throw 无 pad 早退；`tcp.sy` refuse `h == -1`
- **钉测：** `qs1_tcp_connect_refuse_native_llvm` · `p98_driver_file_watch_host` · `p98_driver_tcp_connect_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Process+ llvm covered；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W12（ADR-356）→ **已交**

### Wave A W12 正确性地板加深（ADR-356 · 已交）

- **主轴：** A-05 ProcessBuilderSpawn llvm 首格（`qs1-process-llvm-v0:spawn-missing` / native missing-cmd exact）· A-03 exact（http_get_smoke + udp_bind）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · Create/ChildWait/Sql llvm **本窗跳过**
- **钉测：** `qs1_process_spawn_missing_native_llvm` · `p98_driver_http_get_smoke` · `p98_driver_udp_bind_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Create·ChildWait·Sql llvm covered；无 ProcessBuilderSpawn `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W13（ADR-357）→ **已交**

### Wave A W13 正确性地板加深（ADR-357 · 已交）

- **主轴：** A-05 ChildWait llvm 首格（`qs1-process-llvm-v0:wait-success` / native spawn-wait success exact）· A-03 exact（gzip roundtrip + ed25519 sign/verify）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · ProcessBuilderCreate / Sql llvm **本窗跳过**
- **钉测：** `qs1_process_wait_success_native_llvm` · `p98_driver_gzip_encoding_hosts` · `p98_driver_ed25519_sign_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Create·Sql llvm covered；无 ChildWait `host_fn` → `llvm=real`；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W14（ADR-358）→ **已交**

### Wave A W14 正确性地板加深（ADR-358 · 已交）

- **主轴：** A-05 SqlOpen llvm 首格（`qs1-sql-llvm-v0:open-memory` / native :memory: open+close exact）· A-03 exact（array_flat_map + compile_embedded_ok）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · ProcessBuilderCreate / SqlClose llvm **本窗跳过**
- **钉测：** `qs1_sql_open_memory_native_llvm` · `p98_driver_array_flat_map_named_fn_host` · `p98_driver_compile_embedded_ok`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Create·SqlClose llvm covered；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W15（ADR-359）→ **已交**

### Wave A W15 正确性地板加深（ADR-359 · 已交）

- **主轴：** A-05 SqlClose llvm 首格（`qs1-sql-llvm-v0:close-invalid` / native invalid-close catch exact）· A-03 exact（base64 roundtrip + l14_decode）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · ProcessBuilderCreate llvm **本窗跳过** · CodableL14 encode/decode `materialize_receiver`（HeapSlot 字符串）
- **钉测：** `qs1_sql_close_invalid_native_llvm` · `p98_driver_encoding_base64_host` · `p98_driver_l14_decode_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm / Create llvm covered；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W16（ADR-360）→ **已交**

### Wave A W16 正确性地板加深（ADR-360 · 已交）

- **主轴：** A-05 ProcessBuilderCreate llvm 首格（`qs1-process-llvm-v0:create-success` / native create-success exact）· A-03 exact（l14 embedded roundtrip + register_migration）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback llvm **本窗跳过** · File wasm **本窗跳过** · gzip array I32 宿主修复 **本窗跳过**
- **钉测：** `qs1_process_create_success_native_llvm` · `p98_driver_codable_l14_embedded_roundtrip` · `p98_driver_l14_register_migration_host`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm covered；无 capability 表改动（Create 仍 `llvm=partial`）；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W17（ADR-361）→ **已交**

### Wave A W17 正确性地板加深（ADR-361 · 已交）

- **主轴：** A-05 HttpGet loopback llvm 加深（`qs1-http-llvm-v0:get-loopback` / native exact `["200","ok"]`）· A-03 exact（l14 encode + gzip array I32；`pop_bytes` materialize HeapSlot）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · File wasm **本窗跳过**（错误传播仍坏）
- **钉测：** `qs1_http_get_loopback_native_llvm` · `p98_driver_l14_encode_host_behavior` · `p98_driver_gzip_array_i32_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 wasm covered；无 capability 表改动（HttpGet 仍 `llvm=partial`）；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B
- **下一窗：** Wave A W18（ADR-362）→ **已交**

### Wave A W18 正确性地板加深（ADR-362 · 已交）

- **主轴：** A-05 FileOpen wasm 首格（`qs1-file-wasm-v0:open-missing` / AOT CallHost + Call `take_catch_pc` 分发；aot main `Return`→`i32.const 0`）· A-03 exact（http refused + datetime parse_roundtrip）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · 其余 HostFn wasm **本窗跳过**
- **钉测：** `qs1_file_open_missing_wasm` · `m0_b06_http_connect_refused_is_error` · `m0_b01_parse_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；仅 FileOpen `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W19（ADR-363）→ **已交**

### Wave A W19 正确性地板加深（ADR-363 · 已交）

- **主轴：** A-05 FileClose wasm 格（`qs1-file-wasm-v0:close-bad-handle` / 复用 W18 take_catch）· A-03 exact（datetime format RFC3339 + tcp connect `-1`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · FileRead/Write 与其它域 wasm **本窗跳过**
- **钉测：** `qs1_file_close_bad_handle_wasm` · `m0_b01_format_epoch_day_is_rfc3339` · `m0_b02_tcp_connect_returns_int_not_status_string`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；仅 FileOpen+FileClose `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W20（ADR-364）→ **已交**

### Wave A W20 正确性地板加深（ADR-364 · 已交）

- **主轴：** A-05 FileRead wasm 格（`qs1-file-wasm-v0:read-bad-handle` / 复用 W18 take_catch）· A-03 exact（tcp unknown-handle `"caught"` + file_watch `"Array([])"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · FileWrite 与其它域 wasm **本窗跳过**
- **钉测：** `qs1_file_read_bad_handle_wasm` · `m0_b02_tcp_unknown_handle_read_fails` · `p98_file_watch_os_stdout`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；仅 FileOpen+Close+Read `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W21（ADR-365）→ **已交**

### Wave A W21 正确性地板加深（ADR-365 · 已交）

- **主轴：** A-05 FileWrite wasm 格（`qs1-file-wasm-v0:write-bad-handle` / File 批 wasm 齐；复用 W18 take_catch）· A-03 exact（file_watch import `"[]"` + tcp write unknown-handle `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · 其它域 wasm **本窗跳过**
- **钉测：** `qs1_file_write_bad_handle_wasm` · `p98_file_watch_os_import_module_stdout` · `m2_error_tcp_write_unknown_handle_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File 批四格 `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W22（ADR-366）→ **已交**

### Wave A W22 正确性地板加深（ADR-366 · 已交）

- **主轴：** A-05 HttpGet wasm 首格（`qs1-http-wasm-v0:get-refuse` / 复用 W18 take_catch）· A-03 exact（tcp connect `"caught"` + tcp read unknown-handle `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · Tcp/Process/Sql wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_http_get_refuse_wasm` · `m2_error_tcp_connect_throws_not_status_fail` · `m2_error_tcp_read_unknown_handle_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File 批 + HttpGet refuse `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W23（ADR-367）→ **已交**

### Wave A W23 正确性地板加深（ADR-367 · 已交）

- **主轴：** A-05 TcpConnect wasm 首格（`qs1-tcp-wasm-v0:connect-refuse` / 复用 W18 take_catch；`tcp.sy` refuse 比较改 `h < 0` 以对齐 wasm AOT `if`）· A-03 exact（file open-missing + close-invalid `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · Process/Sql wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_tcp_connect_refuse_wasm` · `m2_error_io_open_missing_path_throws` · `m2_error_io_close_invalid_handle_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File 批 + HttpGet refuse + TcpConnect refuse `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W24（ADR-368）→ **已交**

### Wave A W24 正确性地板加深（ADR-368 · 已交）

- **主轴：** A-05 ProcessBuilderSpawn wasm 首格（`qs1-process-wasm-v0:spawn-missing` / 复用 W18 take_catch）· A-03 exact（file read + write invalid-handle `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · Process Create/Wait wasm、Sql wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_process_spawn_missing_wasm` · `m2_error_io_read_invalid_handle_throws` · `m2_error_io_write_invalid_handle_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File 批 + HttpGet refuse + TcpConnect refuse + ProcessSpawn missing `wasm=covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W25（ADR-369）→ **已交**

### Wave A W25 正确性地板加深（ADR-369 · 已交）

- **主轴：** A-05 ChildWait wasm（`qs1-process-wasm-v0:wait-success` / 复用 W18 take_catch）· A-03 exact（file seek + readAllBytes `"caught"`；`error_io_file` soft 清完）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · ProcessCreate wasm、Sql wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_process_wait_success_wasm` · `m2_error_io_seek_invalid_handle_throws` · `m2_error_io_read_all_bytes_invalid_handle_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File 批 + HttpGet refuse + TcpConnect refuse + ProcessSpawn/ChildWait wasm `covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W26（ADR-370）→ **已交**

### Wave A W26 正确性地板加深（ADR-370 · 已交）

- **主轴：** A-05 ProcessBuilderCreate wasm（`qs1-process-wasm-v0:create-success` / Process 批 wasm 齐；复用 W18 take_catch）· A-03 exact（sql close-invalid + prepare-invalid `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · Sql wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_process_create_success_wasm` · `m2_error_sql_close_invalid_throws` · `m2_error_sql_prepare_invalid_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File/Http refuse/Tcp/Process 批 wasm 齐；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W27（ADR-371）→ **已交**

### Wave A W27 正确性地板加深（ADR-371 · 已交）

- **主轴：** A-05 SqlOpen wasm 首格（`qs1-sql-wasm-v0:open-memory` / 复用 W18 take_catch）· A-03 exact（sql bindStep + exec invalid `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · SqlClose wasm 与 HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_sql_open_memory_wasm` · `m2_error_sql_bind_step_invalid_throws` · `m2_error_sql_exec_invalid_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File/Http refuse/Tcp/Process 批 + SqlOpen wasm `covered`；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W28（ADR-372）→ **已交**

### Wave A W28 正确性地板加深（ADR-372 · 已交）

- **主轴：** A-05 SqlClose wasm（`qs1-sql-wasm-v0:close-invalid` / Sql 批 wasm 齐；复用 W18 take_catch）· A-03 exact（sql query-invalid + nested-begin `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过** · HttpGet loopback wasm **本窗跳过**
- **钉测：** `qs1_sql_close_invalid_wasm` · `m2_error_sql_query_invalid_throws` · `m2_error_sql_nested_begin_transaction_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File/Http refuse/Tcp/Process/Sql 批 wasm 齐；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W29（ADR-373）→ **已交**

### Wave A W29 正确性地板加深（ADR-373 · 已交）

- **主轴：** A-05 HttpGet loopback wasm（`qs1-http-wasm-v0:get-loopback` / 对齐 W17 llvm；复用 W22 wasm 路径）· A-03 exact（sql prepareOn-invalid + txn nested-beginOn `"caught"`）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**
- **钉测：** `qs1_http_get_loopback_wasm` · `m2_error_sql_prepare_on_invalid_throws` · `m2_error_sql_txn_on_nested_begin_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；File/Http/Tcp/Process/Sql 批 + HttpGet loopback wasm；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W30（ADR-374）→ **已交**

### Wave A W30 正确性地板加深（ADR-374 · 已交）

- **主轴：** A-03 exact（sql bindStepOn-schema `ok\n1\ncaught` + bindPreparedOn-mismatch `ok\ncaught`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:bindstep-on-schema`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_bind_step_on_schema_isolation` · `m2_error_sql_bind_prepared_on_conn_mismatch`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W31（ADR-375）→ **已交**

### Wave A W31 正确性地板加深（ADR-375 · 已交）

- **主轴：** A-03 exact（sql prepareOn-schema `ok\ncaught` + bindStepOn write-isolation `ok\n1\n1\n1\n1\n0\n0`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:prepare-on-schema`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_prepare_on_schema_isolation` · `m2_error_sql_bind_step_on_write_isolation`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W32（ADR-376）→ **已交**

### Wave A W32 正确性地板加深（ADR-376 · 已交）

- **主轴：** A-03 exact（sql txn commit-isolation `ok\n1\n0` + rollback `ok\n0`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:txn-on-commit`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_txn_on_commit_isolation` · `m2_error_sql_txn_on_rollback`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W33（ADR-377）→ **已交**

### Wave A W33 正确性地板加深（ADR-377 · 已交）

- **主轴：** A-03 exact（sql bindPreparedOn positive `ok\n1\n1\n0` + beginTransaction ok `ok`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:bind-prepared-positive`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_bind_prepared_on_positive` · `m2_error_sql_begin_transaction_ok`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W34（ADR-378）→ **已交**

### Wave A W34 正确性地板加深（ADR-378 · 已交）

- **主轴：** A-03 exact（sql bindStep valid `ok\n1` + rollback ok `ok`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:bindstep-valid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_bind_step_valid_returns_int` · `m2_error_sql_rollback_ok_after_begin`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W35（ADR-379）→ **已交**

### Wave A W35 正确性地板加深（ADR-379 · 已交）

- **主轴：** A-03 exact（sql exec valid `ok\n1` + prepare valid `ok`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:exec-valid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_exec_valid_returns_int` · `m2_error_sql_prepare_valid_returns_int`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W36（ADR-380）→ **已交**

### Wave A W36 正确性地板加深（ADR-380 · 已交）

- **主轴：** A-03 exact（sql two-memory `ok\n1\n1\n0\n0` + COMMIT-without-begin `caught`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:two-memory`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_sql_two_memory_isolation` · `m2_error_sql_commit_without_begin_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W37（ADR-381）→ **已交**

### Wave A W37 正确性地板加深（ADR-381 · 已交）

- **主轴：** A-03 exact（orm findWhereOn isolation `ok\n1\n0` + updateWhereOn isolation `ok\n1\n1\n0\n0`）· A-05 矩阵指纹加深（`qs1-orm-product-v0:find-isolation`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_orm_find_where_on_isolation` · `m2_error_orm_update_where_on_isolation`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W38（ADR-382）→ **已交**

### Wave A W38 正确性地板加深（ADR-382 · 已交）

- **主轴：** A-03 exact（orm deleteWhereOn isolation `ok\n1\n0\n1` + saveOn isolation `ok\ntrue\n1\n0`）· A-05 矩阵指纹加深（`qs1-orm-product-v0:delete-isolation`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_orm_delete_where_on_isolation` · `m2_error_orm_save_on_isolation`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W39（ADR-383）→ **已交**

### Wave A W39 正确性地板加深（ADR-383 · 已交）

- **主轴：** A-03 exact（orm saveOn invalid-conn `caught` + findWhereOn invalid-conn `caught`）· A-05 矩阵指纹加深（`qs1-orm-product-v0:invalid-conn`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_orm_save_on_invalid_conn_throws` · `m2_error_orm_find_where_on_invalid_conn_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W40（ADR-384）→ **已交**

### Wave A W40 正确性地板加深（ADR-384 · 已交）

- **主轴：** A-03 exact（orm updateWhereOn invalid-conn `caught` + deleteWhereOn invalid-conn `caught`）· A-05 矩阵指纹加深（`qs1-orm-product-v0:update-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）· Orm 产品 soft **耗尽**
- **钉测：** `m2_error_orm_update_where_on_invalid_conn_throws` · `m2_error_orm_delete_where_on_invalid_conn_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W41（ADR-385）→ **已交**

### Wave A W41 正确性地板加深（ADR-385 · 已交）

- **主轴：** A-03 exact（os memoryTotal force-err `caught` + memoryAvailable force-err `caught`）· A-05 矩阵指纹加深（`qs1-os-product-v0:memory-force-err`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_os_memory_total_force_err_throws` · `m2_error_os_memory_available_force_err_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W42（ADR-386）→ **已交**

### Wave A W42 正确性地板加深（ADR-386 · 已交）

- **主轴：** A-03 exact（os uptimeMs force-err `caught` + loadAverage force-err `caught`）· A-05 矩阵指纹加深（`qs1-os-product-v0:uptime-force-err`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）· Os force-err inject **耗尽**
- **钉测：** `m2_error_os_uptime_ms_force_err_throws` · `m2_error_os_load_average_force_err_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W43（ADR-387）→ **已交**

### Wave A W43 正确性地板加深（ADR-387 · 已交）

- **主轴：** A-03 exact（os diskTotal invalid `caught` + diskAvailable invalid `caught`）· A-05 矩阵指纹加深（`qs1-os-product-v0:disk-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_os_disk_total_invalid_throws` · `m2_error_os_disk_available_invalid_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W44（ADR-388）→ **已交**

### Wave A W44 正确性地板加深（ADR-388 · 已交）

- **主轴：** A-03 exact（os userName missing `caught` + homeDir missing `caught`）· A-05 矩阵指纹加深（`qs1-os-product-v0:missing-env`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）· Os missing-env throw **耗尽**
- **钉测：** `m2_error_os_user_name_missing_throws` · `m2_error_os_home_dir_missing_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W45（ADR-389）→ **已交**

### Wave A W45 正确性地板加深（ADR-389 · 已交）

- **主轴：** A-03 exact（os memoryTotal ok `ok` + memoryAvailable ok `ok`；丢 `print(n)`）· A-05 矩阵指纹加深（`qs1-os-product-v0:memory-ok`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_os_memory_total_ok` · `m2_error_os_memory_available_ok`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W46（ADR-390）→ **已交**

### Wave A W46 正确性地板加深（ADR-390 · 已交）

- **主轴：** A-03 exact（os uptimeMs ok `ok` + loadAverage ok `ok`；丢 `print(n)`）· A-05 矩阵指纹加深（`qs1-os-product-v0:uptime-ok`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）· Os ok-path 产品 soft **耗尽**
- **钉测：** `m2_error_os_uptime_ms_ok` · `m2_error_os_load_average_ok`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W47（ADR-391）→ **已交**

### Wave A W47 正确性地板加深（ADR-391 · 已交）

- **主轴：** A-03 exact（sql nested-begin `caught` + service invalid-close `caught`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:nested-begin`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_03b_nested_begin_throws` · `ec_m2_1_sql_service_invalid_close_caught`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W48（ADR-392）→ **已交**

### Wave A W48 正确性地板加深（ADR-392 · 已交）

- **主轴：** A-03 exact（File withOpen throw `CAUGHT` + leak open+close `OPENED\\nCLOSED`）· A-05 矩阵指纹加深（`qs1-file-product-v0:raii-throw`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s3_02b_with_open_throw_live_count` · `ec_m2_3_file_leak_open_close_returns_to_baseline`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W49（ADR-393）→ **已交**

### Wave A W49 正确性地板加深（ADR-393 · 已交）

- **主轴：** A-03 exact（File withOpen success `OK` + FileStream leak open+close `OPENED\\nCLOSED`）· A-05 矩阵指纹加深（`qs1-file-product-v0:raii-success`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s3_02b_with_open_success_live_count` · `ec_m2_3_file_stream_leak_open_close_returns_to_baseline`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W50（ADR-394）→ **已交**

### Wave A W50 正确性地板加深（ADR-394 · 已交）

- **主轴：** A-03 exact（FileWatcher leak open+close `OPENED\\nCLOSED` + FileStream facade `12\\nFACADE_OK`）· A-05 矩阵指纹加深（`qs1-file-product-v0:watch-close`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_3_file_watch_leak_open_close_returns_to_baseline` · `m2_s3_03_file_stream_facade_open_close`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W51（ADR-395）→ **已交**

### Wave A W51 正确性地板加深（ADR-395 · 已交）

- **主轴：** A-03 exact（TcpListener bind+close `OPENED\\nCLOSED` + HttpBodyStream open+close `OPENED\\n200\\nCLOSED`）· A-05 矩阵指纹加深（`qs1-tcp-product-v0:listen-close`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_3_tcp_listener_leak_bind_close_returns_to_baseline` · `ec_m2_3_http_body_stream_leak_open_close_returns_to_baseline`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W52（ADR-396）→ **已交**

### Wave A W52 正确性地板加深（ADR-396 · 已交）

- **主轴：** A-03 exact（Process spawn+wait `SPAWNED\\nWAITED\\n0` + Sql leak open+close `OPENED\\nCLOSED`）· A-05 矩阵指纹加深（`qs1-process-product-v0:spawn-wait`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_3_process_leak_spawn_wait_returns_to_baseline` · `ec_m2_3_sql_leak_open_close_returns_to_baseline`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W53（ADR-397）→ **已交**

### Wave A W53 正确性地板加深（ADR-397 · 已交）

- **主轴：** A-03 exact（ChildPipe take+close `TAKEN\\nCLOSED\\n0` + Sql implicit conn0 `OK`）· A-05 矩阵指纹加深（`qs1-process-product-v0:pipe-close`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_3_child_pipe_leak_take_close_returns_to_baseline` · `ec_m2_3_sql_leak_implicit_conn0_not_counted`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W54（ADR-398）→ **已交**

### Wave A W54 正确性地板加深（ADR-398 · 已交）

- **主轴：** A-03 exact（ProcessBuilder cwd overlay `OK\\n0` + env overlay `OK\\n0`）· A-05 矩阵指纹加深（`qs1-process-product-v0:builder-cwd`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_01_cwd_overlay` · `m2_s6_01_env_overlay`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W55（ADR-399）→ **已交**

### Wave A W55 正确性地板加深（ADR-399 · 已交）

- **主轴：** A-03 exact（ProcessBuilder waitTimeout success `OK\\n0` + kill fail-after `KILLED\\nWAIT_FAIL\\nKILL_FAIL`）· A-05 矩阵指纹加深（`qs1-process-product-v0:wait-timeout`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_01_wait_timeout_success` · `m2_s6_01_kill_and_fail_after`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W56（ADR-400）→ **已交**

### Wave A W56 正确性地板加深（ADR-400 · 已交）

- **主轴：** A-03 exact（ProcessBuilder waitTimeout kills `TIMEOUT\\nGONE` + ChildPipe close-then-read `CLOSED\\n0\\nREAD_FAIL`）· A-05 矩阵指纹加深（`qs1-process-product-v0:timeout-kill`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_01_wait_timeout_kills` · `m2_s6_02b_close_then_read_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W57（ADR-401）→ **已交**

### Wave A W57 正确性地板加深（ADR-401 · 已交）

- **主轴：** A-03 exact（ChildPipe untaken wait `OK\\n141` + stdin echo `OK\\n3\\n3\\n0`）· A-05 矩阵指纹加深（`qs1-process-product-v0:untaken-wait`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_02b_untaken_wait_no_hang` · `m2_s6_02b_stdin_echo`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W58（ADR-402）→ **已交**

### Wave A W58 正确性地板加深（ADR-402 · 已交）

- **主轴：** A-03 exact（Child.signal illegal name `BAD_NAME` + KILL fail-after `WAITED\\nSIGNAL_FAIL`）· A-05 矩阵指纹加深（`qs1-process-product-v0:signal-illegal`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_03_illegal_name_throws` · `m2_s6_03_kill_smoke_then_signal_after_wait_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W59（ADR-403）→ **已交**

### Wave A W59 正确性地板加深（ADR-403 · 已交）

- **主轴：** A-03 exact（Sql pool busy `BUSY_FAIL\\nAFTER_OK\\nDONE` + mid-txn `MID_FAIL\\nAFTER_COMMIT_OK\\nMID2_FAIL\\nAFTER_ROLLBACK_OK\\nDONE`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:pool-busy`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_03b_second_immediate_busy` · `m2_s8_03b_release_mid_txn_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W60（ADR-404）→ **已交**

### Wave A W60 正确性地板加深（ADR-404 · 已交）

- **主轴：** A-03 exact（Sql pool close-after `AFTER_FAIL\\nDONE` + dual-checkout `OK\\n2`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:pool-close`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_03b_pool_close_live_count` · `m2_s8_03b_dual_checkout_sequential_txn`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W61（ADR-405）→ **已交**

### Wave A W61 正确性地板加深（ADR-405 · 已交）

- **主轴：** A-03 exact（Sql pool double-release `DOUBLE_FAIL\\nAFTER_FAIL\\nDONE` + interleaved exec `OK\\n2`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:double-release`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_01c_close_and_double_release_fail` · `m2_s8_01c_interleaved_exec_on_file_dsn`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W62（ADR-406）→ **已交**

### Wave A W62 正确性地板加深（ADR-406 · 已交）

- **主轴：** A-03 exact（SqlRow getInt/mismatch `OK\\n7\\nalice\\nMISMATCH_FAIL\\nMISS_FAIL` + columnNames/isNull `OK\\n2\\ntrue\\nfalse`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:typed-row`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_02b_get_int_string_and_mismatch` · `m2_s8_02b_column_names_and_is_null`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W63（ADR-407）→ **已交**

### Wave A W63 正确性地板加深（ADR-407 · 已交）

- **主轴：** A-03 exact（Sql pool exhaust SAME `GOT2\\nEXHAUST\\nREUSE\\nSAME\\nOK` + sql_service CRUD `ok\\n1\\nclosed`）· A-05 矩阵指纹加深（`qs1-sql-product-v0:pool-exhaust`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s8_01c_max_size_exhaust_and_reuse` · `ec_m2_1_sql_service_memory_crud_ok`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；无 sql_tool example 改动；A-05 仍 `[~]`
- **下一窗：** Wave A W64（ADR-408）→ **已交**

### Wave A W64 正确性地板加深（ADR-408 · 已交）

- **主轴：** A-03 exact（mmap readAllBytes `OK\\n36\\n83\\n49` + Http.response accessors `200\\nok\\ntrue`）· A-05 矩阵指纹加深（`qs1-file-product-v0:readall-bytes`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s3_04_read_all_bytes_roundtrip` · `m2_s4_01_make_response_accessors`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W65（ADR-409）→ **已交**

### Wave A W65 正确性地板加深（ADR-409 · 已交）

- **主轴：** A-03 exact（Url.parse fields `example.com\\n/a\\nq=1` + http_tool refuse `refused`）· A-05 矩阵指纹加深（`qs1-http-product-v0:url-parse`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s4_01_url_parse_fields` · `ec_m2_1_http_client_refuse_catchable`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；无 http_tool example 改动；A-05 仍 `[~]`
- **下一窗：** Wave A W66（ADR-410）→ **已交**

### Wave A W66 正确性地板加深（ADR-410 · 已交）

- **主轴：** A-03 exact（loopback Http.get accessors `200\\nok\\ntrue` + Tcp connect-close `OPENED\\nCLOSED`）· A-05 矩阵指纹加深（`qs1-http-product-v0:loopback-get`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s4_01_loopback_http_get_accessors` · `ec_m2_3_tcp_leak_connect_close_returns_to_baseline`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W67（ADR-411）→ **已交**

### Wave A W67 正确性地板加深（ADR-411 · 已交）

- **主轴：** A-03 exact（http_tool loopback-ok `refused\\nok\\n200\\nok` + Http.shutdown `listening\\nstopped`）· A-05 矩阵指纹加深（`qs1-http-product-v0:client-ok`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_1_http_client_loopback_ok` · `m2_s4_03b_graceful_shutdown_loopback`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 SynUI/Farm/Dist；无 Wave B；无 http_tool example 改动；A-05 仍 `[~]`
- **下一窗：** Wave A W68（ADR-412）→ **已交**

### Wave A W68 正确性地板加深（ADR-412 · 已交）

- **主轴：** A-03 exact（UTC ISO millis `86400000\\n1970-01-02T00:00:00.000Z` + leap-day `true\\n2020-02-29T12:00:00.000Z`）· A-05 矩阵指纹加深（`qs1-datetime-product-v0:utc-roundtrip`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s5_03_utc_millis_roundtrip` · `m2_s5_03_leap_day_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W69（ADR-413）→ **已交**

### Wave A W69 正确性地板加深（ADR-413 · 已交）

- **主轴：** A-03 exact（post-2038 `true\\n2100-01-01T00:00:00.000Z` + negative-epoch `-1000\\n-1000`）· A-05 矩阵指纹加深（`qs1-datetime-product-v0:epoch-edges`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s5_03_post_2038_roundtrip` · `m2_s5_03_negative_epoch_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W70（ADR-414）→ **已交**

### Wave A W70 正确性地板加深（ADR-414 · 已交）

- **主轴：** A-03 exact（fixed-offset `86400000` + date-only `2020-02-29`）· A-05 矩阵指纹加深（`qs1-datetime-product-v0:fixed-offset-date`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s5_03_fixed_offset_to_utc_ms` · `m2_s5_03_date_only_yyyy_mm_dd`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W71（ADR-415）→ **已交**

### Wave A W71 正确性地板加深（ADR-415 · 已交）

- **主轴：** A-03 exact（FileStream chunk `OK\\n4\\n4096\\nCLOSED_FAIL` + ChildPipe stdout `OK\\n4\\n4096\\n0`）· A-05 矩阵指纹加深（`qs1-io-product-v0:stream-chunks`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s3_03_file_stream_chunk_and_close_fail` · `m2_s6_02b_stdout_stream_chunks`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W72（ADR-416）→ **已交**

### Wave A W72 正确性地板加深（ADR-416 · 已交）

- **主轴：** A-03 exact（file_tool `ok\\n3\\ntrue` + FileWatcher poll `1\\nCLOSED_FAIL`）· A-05 矩阵指纹加深（`qs1-file-product-v0:tool-watcher`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_1_file_tool_roundtrip_ok_exists` · `m2_s3_03_file_watcher_poll_and_close_fail`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W73（ADR-417）→ **已交**

### Wave A W73 正确性地板加深（ADR-417 · 已交）

- **主轴：** A-03 exact（JSONDecoder `caught\\nJSONDecoder missing required key 'address' codingPath=address` + JSONStream `caught\\nJSONDecoder missing required key 'address' codingPath=line[0].address`）· A-05 矩阵指纹加深（`qs1-json-product-v0:coding-path`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s7_02_missing_key_coding_path` · `m2_s7_03_missing_key_line_coding_path`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA/timezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W74（ADR-418）→ **已交**

### Wave A W74 正确性地板加深（ADR-418 · 已交）

- **主轴：** A-03 exact（IANA leap-day `2020-02-29T00:00:00.000Z` + unknown zone `caught\\nDateTime: unknown IANA timezone \"Not/AZone\" (no tzdb match)`）· A-05 矩阵指纹加深（`qs1-datetime-product-v0:iana-leap-u001`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s5_02_leap_day_still_green` · `m2_s5_02_unknown_zone_u001`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 formatWithTimezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W75（ADR-419）→ **已交**

### Wave A W75 正确性地板加深（ADR-419 · 已交）

- **主轴：** A-03 exact（JSON optional `{title: t}\n{"note":"n2","title":"t2"}\n{note: n2, title: t2}` + nested encode/decode `{"address":{"city":"SF","zip":"94102"},"name":"ada"}\n{address: {city: SF, zip: 94102}, name: ada}`）· A-05 矩阵指纹加深（`qs1-json-product-v0:optional-nested`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s7_02_optional_field_omit_and_present` · `m2_s7_02_nested_encode_decode_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA formatWithTimezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W76（ADR-420）→ **已交**

### Wave A W76 正确性地板加深（ADR-420 · 已交）

- **主轴：** A-03 exact（JSONStream reader `{address: {city: SF, zip: 94102}, name: ada}\n{address: {city: NY, zip: 10001}, name: bob}` + writer NDJSON two users + blank line + buffered encode）· A-05 矩阵指纹加深（`qs1-json-product-v0:stream-roundtrip`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s7_03_reader_chunk_feed_decode_next` · `m2_s7_03_writer_two_users_ndjson`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA formatWithTimezone 叙事改写；无 Deque nil/Nil 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W77（ADR-421）→ **已交**

### Wave A W77 正确性地板加深（ADR-421 · 已交）

- **主轴：** A-03 exact（Deque empty-pop `true\nnil\nnil\n0` + FIFO `3\n1\n2\n3\n0`）· A-05 矩阵指纹加深（`qs1-collection-product-v0:deque-nil`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_03_empty_pop_nil` · `m2_s1_03_push_back_pop_front_fifo`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 IANA formatWithTimezone 叙事改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W78（ADR-422）→ **已交**

### Wave A W78 正确性地板加深（ADR-422 · 已交）

- **主轴：** A-03 exact（IANA formatWithTimezone NY `2024-01-15T07:00:00.000-05:00\n2024-07-15T08:00:00.000-04:00` + DST offsets `-300\n-240`）· A-05 矩阵指纹加深（`qs1-datetime-product-v0:iana-format-ny`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s5_02_format_with_timezone_ny` · `m2_s5_02_ny_dst_offsets`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 format 双路径残留；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W79（ADR-423）→ **已交**

### Wave A W79 正确性地板加深（ADR-423 · 已交）

- **主轴：** A-03 exact（Deque mixed `1\n3\n2\ntrue` + writeback `2\n10\n20`）· A-05 矩阵指纹加深（`qs1-collection-product-v0:deque-mixed`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_03_push_front_pop_back_and_mixed` · `m2_s1_03_var_writeback_persists`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 collection_mutation 整串改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W80（ADR-424）→ **已交**

### Wave A W80 正确性地板加深（ADR-424 · 已交）

- **主轴：** A-03 exact（Dict remove `true\nfalse\ntrue\n1\nfalse\ntrue` + Set insert/remove `true\nfalse\ntrue\n3\ntrue\nfalse\n2`）· A-05 矩阵指纹加深（`qs1-collection-product-v0:mutation-persist`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_02_dict_remove_persists` · `m2_s1_02_set_insert_remove_persists`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 Array subscript/setAt 整串改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W81（ADR-425）→ **已交**

### Wave A W81 正确性地板加深（ADR-425 · 已交）

- **主轴：** A-03 exact（Array subscript `1\n9\n3` + setAt `1\n8\n3\n7\n7`）· A-05 矩阵指纹加深（`qs1-collection-product-v0:array-assign`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_02_array_subscript_assign` · `m2_s1_02_array_set_at_persists`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 string_normalization 整串改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W82（ADR-426）→ **已交**

### Wave A W82 正确性地板加深（ADR-426 · 已交）

- **主轴：** A-03 exact（NFC compose `é\né` + caseFolded `ss\nabc`）· A-05 矩阵指纹加深（`qs1-string-product-v0:nfc-fold`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s2_03_nfc_composes_combining_acute` · `m2_s2_03_case_folded_full_and_ascii`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 NFD soft / lowercased 整串改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W83（ADR-427）→ **已交**

### Wave A W83 正确性地板加深（ADR-427 · 已交）

- **主轴：** A-03 exact（NFD roundtrip `2\né` + lowercased `ß\nss`）· A-05 矩阵指纹加深（`qs1-string-product-v0:nfd-lower`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s2_03_nfd_roundtrip_via_nfc` · `m2_s2_03_lowercased_not_case_folded`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 string_three_view 整串改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W84（ADR-428）→ **已交**

### Wave A W84 正确性地板加深（ADR-428 · 已交）

- **主轴：** A-03 exact（for-in grapheme `1\n2` + isSingleScalar `true\nfalse\nfalse`）· A-05 矩阵指纹加深（`qs1-string-product-v0:three-view-iter`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s2_01_for_in_is_grapheme` · `m2_s2_01_is_single_scalar_honest`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 grapheme description / scalar soft / utf8 soft 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W85（ADR-429）→ **已交**

### Wave A W85 正确性地板加深（ADR-429 · 已交）

- **主轴：** A-03 exact（utf8 view `2\n2\n104\n1\n18` + scalar view `1\n5\n2\n2`）· A-05 矩阵指纹加深（`qs1-string-product-v0:utf8-scalar`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s2_01_utf8_view` · `m2_s2_01_scalar_view_diverges_from_grapheme`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 grapheme description soft 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W86（ADR-430）→ **已交**

### Wave A W86 正确性地板加深（ADR-430 · 已交）

- **主轴：** A-03 exact（grapheme `1\n👨‍👩‍👧` + index ZWJ `1\n👨‍👩‍👧`）· A-05 矩阵指纹加深（`qs1-string-product-v0:grapheme-zwj`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s2_01_grapheme_length_and_subscript` · `m2_s2_02_zwj_family_subscript_still_one_grapheme`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/TERM/without_close/http_server 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W87（ADR-431）→ **已交**

### Wave A W87 正确性地板加深（ADR-431 · 已交）

- **主轴：** A-03 exact（tcp Int `8\n8\n8` + HTTPS EXTRA_ROOT `200\nok\ntrue`）· A-05 矩阵指纹加深（`qs1-net-product-v0:tcp-https`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_tcp_stream_valid_returns_int` · `m2_s4_05_https_loopback_ok_with_extra_root`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/TERM/without_close/http_server 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W88（ADR-432）→ **已交**

### Wave A W88 正确性地板加深（ADR-432 · 已交）

- **主轴：** A-03 exact（TERM wait `OK\n-1` + tcp invalid `caught`）· A-05 矩阵指纹加深（`qs1-process-product-v0:term-tcp-fail`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s6_03_term_then_wait_nonzero` · `m2_error_tcp_stream_invalid_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W89（ADR-433）→ **已交**

### Wave A W89 正确性地板加深（ADR-433 · 已交）

- **主轴：** A-03 exact（path join/normalize/extension unix `a/b\na/c\nb\n\ngz` + IO bytes `65\n66\n67`）· A-05 矩阵指纹加深（`qs1-path-product-v0:join-bytes`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `ec_m2_1_path_contract_join_normalize_extension` · `io_read_write_bytes_roundtrip`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W90（ADR-434）→ **已交**

### Wave A W90 正确性地板加深（ADR-434 · 已交）

- **主轴：** A-03 exact（FileHandle read bytes `10\n20\n30` + HTTP chunked multi-kb constructed body）· A-05 矩阵指纹加深（`qs1-io-product-v0:handle-chunk`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `file_handle_read_bytes` · `m2_s4_04_chunked_read_multi_kb`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W91（ADR-435）→ **已交**

### Wave A W91 正确性地板加深（ADR-435 · 已交）

- **主轴：** A-03 exact（withOpen host inject `[R001] withOpen body failed (host inject)` + ISO/IANA fail-closed U002/U001 Display）· A-05 矩阵指纹加深（`qs1-io-product-v0:inject-iso`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s3_02b_host_inject_fail_live_count` · `m2_s5_03_fail_closed_invalid_and_iana`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path/timeout-port 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W92（ADR-436）→ **已交**

### Wave A W92 正确性地板加深（ADR-436 · 已交）

- **主轴：** A-03 exact（HOF map/filter missing Closure U001 Display）· A-05 矩阵指纹加深（`qs1-collection-product-v0:hof-u001`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_04_map_missing_closure_fails` · `m2_s1_04_filter_missing_closure_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path/timeout-port/http.rs 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W93（ADR-437）→ **已交**

### Wave A W93 正确性地板加深（ADR-437 · 已交）

- **主轴：** A-03 exact（HOF reduce missing Closure U001 Display + emoji ident start L001 Display）· A-05 矩阵指纹加深（`qs1-collection-product-v0:hof-reduce`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s1_04_reduce_missing_closure_fails` · `m2_s2_04_emoji_ident_start_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path/timeout-port/http.rs 改写；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W94（ADR-438）→ **已交**

### Wave A W94 正确性地板加深（ADR-438 · 已交）

- **主轴：** A-03 exact（`http.rs` R012 稳定文案 + timeout hang Display + HOF filter illegal receiver U001）· A-05 矩阵指纹加深（`qs1-http-product-v0:timeout-r012`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s4_02b_timeout_hang_loopback` · `m2_s1_04_filter_illegal_receiver_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server/TLS dual-path；无扩大改写 R003/serve；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W95（ADR-439）→ **已交**

### Wave A W95 正确性地板加深（ADR-439 · 已交）

- **主轴：** A-03 exact（`http.rs` R003 稳定文案 + bad-cert / self-signed without EXTRA_ROOT Display）· A-05 矩阵指纹加深（`qs1-http-product-v0:tls-r003`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s4_02b_bad_cert_https_fail_closed` · `m2_s4_05_self_signed_still_fail_closed_without_extra_root`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server；无 `error_tcp_stream` dual-path；无扩大改写 read/R011/serve；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W96（ADR-440）→ **已交**

### Wave A W96 正确性地板加深（ADR-440 · 已交）

- **主轴：** A-03 exact（tcp stream tls invalid Display + HOF reduce illegal receiver U001）· A-05 矩阵指纹加深（`qs1-net-product-v0:tcp-tls-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_tcp_stream_tls_invalid_throws` · `m2_s1_04_reduce_illegal_receiver_fails`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server 动态 port；无扩大改写 read/R011/serve；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W97（ADR-441）→ **已交**

### Wave A W97 正确性地板加深（ADR-441 · 已交）

- **主轴：** A-03 exact（tcp stream plain invalid Display + handle invalid Display）· A-05 矩阵指纹加深（`qs1-net-product-v0:tcp-stream-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_error_tcp_stream_invalid_throws` · `m2_error_tcp_stream_handle_invalid_throws`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server 动态 port；无 graceful body；无 IANA/Codable 弱钉；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W98（ADR-442）→ **已交**

### Wave A W98 正确性地板加深（ADR-442 · 已交）

- **主轴：** A-03 exact（graceful HTTP body `ok`/`bye` + IANA unknown U001 Display）· A-05 矩阵指纹加深（`qs1-http-product-v0:graceful-body`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **钉测：** `m2_s4_03b_graceful_shutdown_loopback` · `m2_s5_04_unknown_iana_still_fail_closed`
- **规划 SSOT：** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
- **冻结声明：** 无 A-02 SSA 解 skip；无 ObjectGet 产品钉；无 capability 表改动；无 fingerprint 海量改写；无 hostname/without_close/http_server_tool 动态 port；无 Codable 弱钉；无 SynUI/Farm/Dist；无 Wave B；A-05 仍 `[~]`
- **下一窗：** Wave A W99（ADR-443）→ **已交**


### Wave A W99 正确性地板加深（ADR-443 · 已交）

- **主轴：** A-03 exact（tcp read/write unknown-handle Display `[R012]`）· A-05 矩阵指纹加深（`qs1-net-product-v0:tcp-rw-unknown`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_tcp_facade.rs`（host `__syx_tcp_read` / `__syx_tcp_write`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W99
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · File invalid-handle Display · Wave B
- **下一窗：** Wave A W100（ADR-444）→ **已交**


### Wave A W100 正确性地板加深（ADR-444 · 已交）

- **主轴：** A-03 exact（FileHandle read/write invalid-handle Display `[R003] Invalid file handle 999999`）· A-05 矩阵指纹加深（`qs1-file-product-v0:invalid-handle`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_io_file.rs`（host `__syx_file_read_chunk` / `__syx_file_write_chunk`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W100
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · FileStream/Watcher invalid · close/seek/readAllBytes Display · Wave B
- **下一窗：** Wave A W101（ADR-445）→ **已交**


### Wave A W101 正确性地板加深（ADR-445 · 已交）

- **主轴：** A-03 exact（FileHandle close/seek invalid-handle Display `[R003] Invalid file handle 999999`）· A-05 矩阵指纹加深（`qs1-file-product-v0:close-seek-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_io_file.rs`（host `__syx_file_close` / `__syx_file_seek`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W101
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · readAllBytes Display · FileStream/Watcher invalid · Wave B
- **下一窗：** Wave A W102（ADR-446）→ **已交**


### Wave A W102 正确性地板加深（ADR-446 · 已交）

- **主轴：** A-03 exact（FileHandle readAllBytes invalid Display + FileStream invalid Display）· A-05 矩阵指纹加深（`qs1-file-product-v0:readall-stream-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_io_file.rs`（`__syx_file_read_all_bytes`）· `file_stream_watcher.rs`（`__syx_file_stream_close`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W102
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · FileWatcher invalid · Wave B
- **下一窗：** Wave A W103（ADR-447）→ **已交**


### Wave A W103 正确性地板加深（ADR-447 · 已交）

- **主轴：** A-03 exact（FileWatcher close/poll invalid-handle Display `[R003] FileWatcher invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-file-product-v0:watcher-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `file_stream_watcher.rs`（`__syx_file_watcher_close` / `__syx_file_watcher_poll`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W103
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · ChildPipe invalid · Wave B
- **下一窗：** Wave A W104（ADR-448）→ **已交**


### Wave A W104 正确性地板加深（ADR-448 · 已交）

- **主轴：** A-03 exact（ChildPipe close/readChunk invalid-handle Display `[R003] ChildPipe invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-process-product-v0:childpipe-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `child_stdio_pipe.rs`（`__syx_child_pipe_close` / `__syx_child_pipe_read_chunk`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W104
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · Wave B
- **下一窗：** Wave A W105（ADR-449）→ **已交**









### Wave A W105 正确性地板加深（ADR-449 · 已交）

- **主轴：** A-03 exact（ChildPipe write + FileStream readChunk invalid-handle Display `[R003] … invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-io-product-v0:pipe-stream-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `child_stdio_pipe.rs`（`__syx_child_pipe_write`）· `file_stream_watcher.rs`（`__syx_file_stream_read_chunk`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W105
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Wave B
- **下一窗：** Wave A W106（ADR-450）→ **已交**




### Wave A W106 正确性地板加深（ADR-450 · 已交）

- **主轴：** A-03 exact（Child wait + kill invalid-handle Display `[R003] Child invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-process-product-v0:child-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `process_builder.rs`（`__syx_child_wait` / `__syx_child_kill`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W106
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · ProcessBuilder invalid · HttpBodyStream · Wave B
- **下一窗：** Wave A W107（ADR-451）→ **已交**




### Wave A W107 正确性地板加深（ADR-451 · 已交）

- **主轴：** A-03 exact（ProcessBuilder cwd + env invalid-handle Display `[R003] ProcessBuilder invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-process-product-v0:builder-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `process_builder.rs`（`__syx_process_builder_cwd` / `__syx_process_builder_env`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W107
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · ProcessBuilder spawn · Child signal/waitTimeout · HttpBodyStream · Sql soft · Wave B
- **下一窗：** Wave A W108（ADR-452）→ **已交**




### Wave A W108 正确性地板加深（ADR-452 · 已交）

- **主轴：** A-03 exact（ProcessBuilder spawn + Child signal invalid-handle Display）· A-05 矩阵指纹加深（`qs1-process-product-v0:spawn-signal-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `process_builder.rs`（`__syx_process_builder_spawn` / `__syx_child_signal`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W108
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Child waitTimeout · HttpBodyStream · Sql soft · Wave B
- **下一窗：** Wave A W109（ADR-453）→ **已交**




### Wave A W109 正确性地板加深（ADR-453 · 已交）

- **主轴：** A-03 exact（Child waitTimeout + stdout invalid-handle Display `[R003] Child invalid handle 999999`）· A-05 矩阵指纹加深（`qs1-process-product-v0:timeout-stdout-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `process_builder.rs`（`__syx_child_wait_timeout` / `__syx_child_stdout`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W109
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Child stderr/stdin · HttpBodyStream · Sql soft · Wave B
- **下一窗：** Wave A W110（ADR-454）→ **已交**




### Wave A W110 正确性地板加深（ADR-454 · 已交）

- **主轴：** A-03 exact（Child stderr + stdin invalid-handle Display `[R003] Child invalid handle 999999`；takeStdio 三角收口）· A-05 矩阵指纹加深（`qs1-process-product-v0:stderr-stdin-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `process_builder.rs`（`__syx_child_stderr` / `__syx_child_stdin`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W110
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Sql soft · Wave B
- **下一窗：** Wave A W111（ADR-455）→ **已交**




### Wave A W111 正确性地板加深（ADR-455 · 已交）

- **主轴：** A-03 exact（Sql close unknown-connection + execOn not-found Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:conn-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_close` / `__syx_sql_exec_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W111
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · Wave B
- **下一窗：** Wave A W112（ADR-456）→ **已交**




### Wave A W112 正确性地板加深（ADR-456 · 已交）

- **主轴：** A-03 exact（Sql queryOn + prepareOn unknown-connection Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:query-prepare-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_query_on` / `__syx_sql_prepare_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W112
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · txn/bind*On · Wave B
- **下一窗：** Wave A W113（ADR-457）→ **已交**




### Wave A W113 正确性地板加深（ADR-457 · 已交）

- **主轴：** A-03 exact（Sql beginTransactionOn + commitTransactionOn unknown-connection Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:txn-on-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_begin_transaction_on` / `__syx_sql_commit_transaction_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W113
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · rollback On · bind*On · Wave B
- **下一窗：** Wave A W114（ADR-458）→ **已交**




### Wave A W114 正确性地板加深（ADR-458 · 已交）

- **主轴：** A-03 exact（Sql rollbackTransactionOn + bindStepOn unknown-connection Display；txn 三角收口）· A-05 矩阵指纹加深（`qs1-sql-product-v0:rollback-bindstep-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_rollback_transaction_on` / `__syx_sql_bind_step_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W114
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · bindPreparedOn · Wave B
- **下一窗：** Wave A W115（ADR-459）→ **已交**




### Wave A W115 正确性地板加深（ADR-459 · 已交）

- **主轴：** A-03 exact（Sql bindPreparedOn unknown-stmt + poolAcquire unknown-pool Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:bindprep-pool-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_bind_prepared_on` / `__syx_sql_pool_acquire`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W115
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · bindPrepared mismatch · poolRelease/Close · Wave B
- **下一窗：** Wave A W116（ADR-460）→ **已交**




### Wave A W116 正确性地板加深（ADR-460 · 已交）

- **主轴：** A-03 exact（Sql poolRelease + poolClose unknown-pool Display；pool 三角收口）· A-05 矩阵指纹加深（`qs1-sql-product-v0:pool-unknown`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_sql_conn.rs`（`__syx_sql_pool_release` / `__syx_sql_pool_close`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W116
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · HttpBodyStream · Orm soft · bindPrepared mismatch · Wave B
- **下一窗：** Wave A W119（ADR-463）→ **已交**








### Wave A W225 正确性地板加深（ADR-569 · 已交）

- **主轴：** HostFn **JsonDecoderDecode** llvm=real — flat Box decode `{n: 42}` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W218/W224 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `json_encoder_decoder_field_paths.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W225
- **非目标：** Json* wasm=real · DateTimeFormat llvm · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W226+（datetime.clock / net.http.stream / DateTimeFormat llvm / Offset wasm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W226 正确性地板加深（ADR-570 · 已交）

- **主轴：** HostFn **DateTimeFormat** llvm=real — UTC ISO format `1970-01-02T00:00:00.000Z` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W68 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iso_rfc3339_roundtrip.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W226
- **非目标：** Parse* llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W227+（datetime.clock / net.http.stream / Offset·Format wasm / DateTimeParse* llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W227 正确性地板加深（ADR-571 · 已交）

- **主轴：** HostFn **DateTimeParseIso8601** llvm=real — UTC ISO parse ms `86400000` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W68 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iso_rfc3339_roundtrip.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W227
- **非目标：** DateTimeParse llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W228+（datetime.clock / net.http.stream / Offset·Format wasm / DateTimeParse llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W228 正确性地板加深（ADR-572 · 已交）

- **主轴：** HostFn **DateTimeParse** llvm=real — date-only `2020-02-29` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W70 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iso_rfc3339_roundtrip.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W228
- **非目标：** Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W229+（datetime.clock / net.http.stream / Offset·Format wasm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W229 正确性地板加深（ADR-573 · 已交）

- **主轴：** HostFn **StrCaseFolded** llvm=real — caseFolded `ß→ss` / `AbC→abc` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W82/W212 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W229
- **非目标：** Str* 批量 llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W230+（datetime.clock / net.http.stream / Offset·Format wasm / StrNormalizedNFC llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W230 正确性地板加深（ADR-574 · 已交）

- **主轴：** HostFn **StrNormalizedNFC** llvm=real — NFC compose `é\né` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W82/W212 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eightieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W230
- **非目标：** NFKC · Str* 批量 llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W231+（datetime.clock / net.http.stream / Offset·Format wasm / StrNormalizedNFD llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W231 正确性地板加深（ADR-575 · 已交）

- **主轴：** HostFn **StrNormalizedNFD** llvm=real — NFD roundtrip exact `2\né` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W83 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W231
- **非目标：** NFKD · Str* 批量 llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W232+（datetime.clock / net.http.stream / Offset·Format wasm / StrInstLowercased llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W232 正确性地板加深（ADR-576 · 已交）

- **主轴：** HostFn **StrInstLowercased** llvm=real — lowercased≠caseFolded exact `ß\nss` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W83 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W232
- **非目标：** Uppercased · Str* 批量 llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W233+（datetime.clock / net.http.stream / Offset·Format wasm / StrInstUppercased llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**


### Wave A W233 正确性地板加深（ADR-577 · 已交）

- **主轴：** HostFn **StrInstUppercased** llvm=real — uppercased ASCII `AbC→ABC` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ 新建 VM 钉；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W233
- **非目标：** locale/Turkish · Str* 批量 llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W234+（datetime.clock / net.http.stream / Offset·Format wasm / StrUnicodeScalarCount llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**


### Wave A W234 正确性地板加深（ADR-578 · 已交）

- **主轴：** HostFn **StrUnicodeScalarCount** llvm=real — scalar_count `ab→2` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ three-view VM；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W234
- **非目标：** grapheme GA · StrUnicodeScalars/Utf8* 批量 · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W235+（datetime.clock / net.http.stream / Offset·Format wasm / StrUtf8Count llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**


### Wave A W235 正确性地板加深（ADR-579 · 已交）

- **主轴：** HostFn **StrUtf8Count** llvm=real — utf8_count `hi→2` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ three-view VM；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W235
- **非目标：** StrUtf8Bytes/Scalars 批量 · grapheme GA · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W236+（datetime.clock / net.http.stream / Offset·Format wasm / StrUnicodeScalars llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**


### Wave A W236 正确性地板加深（ADR-580 · 已交）

- **主轴：** HostFn **StrUnicodeScalars** llvm=real — `scalars(ab).length→2` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ three-view VM；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W236
- **非目标：** Array 元素 GA · StrUtf8Bytes 批量 · grapheme GA · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W237+（datetime.clock / net.http.stream / Offset·Format wasm / StrUtf8Bytes llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**






### Wave A W267 正确性地板加深（ADR-611 · text 收官 + Fmt/print HostFn wasm 扩批 8 包 · 已交）

- **主轴：** HostFn **text 收官 + Fmt/print wasm8** — StrUnicodeScalars + StrUtf8Bytes + CharacterIsSingleScalar + FmtPrint + FmtPrintLine + FileStreamOpen + FileStreamReadChunk + FileStreamClose wasm=real；**批=8**（text HostFn wasm 收官）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventeenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `fmt_print_host.rs` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W267
- **非目标：** Task*/TaskGroup* wasm · TimeNowMs/InstantNowMs wasm · Clock GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W268+（Path·File HostFn wasm / Task* wasm / Wave A 结构债） → **已交**

### Wave A W268 正确性地板加深（ADR-612 · Path/File HostFn wasm 扩批 8 包 · 已交）

- **主轴：** HostFn **Path/File wasm8** — PathJoin + PathNormalize + PathExists + PathExtension + FileOpen + FileClose + FileRead + FileWrite wasm=real；复用 path_file wasm + qs1_file catch wasm；**批=8**；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `path_file_native_wasm_ga.rs` · `hostfn_file.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W268
- **非目标：** Task*/TaskGroup* wasm · FileSeek/Mmap/WithOpen/Watcher wasm · TimeNowMs/InstantNowMs wasm · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W269+（FileSeek/Mmap/Watcher wasm / Task* wasm / Wave A 结构债） → **已交**












- **W446 / ADR-790:** B-04 Short `[x]` exit charter met (construction chain W300–W440; `AotShortHorizonExitInventory` / `b04_short_exit_signed=true`; roadmap B-04 **`[x]`** narrow); Wave B axis delivery **2 `[x]`** (B-04+B-08); HostFn 554; B-05/B-06 still `[~]`; ≠ language `Module.exports` auto-discovery / ≠ emit change / ≠ no-idle daemon / ≠ MIR blob delete this window; production native no blob/sidecar (distance **0**).
- **W445 / ADR-789:** TLS language ALPN parameter first-slice met (`host_tls_serve` arity 5 — non-empty String alpn arg overrides env/default; empty → `SYX_TLS_SERVE_ALPN` → `syx/1`; facade `serveOnceAlpn`; concurrent + idle retained; `TlsServe` still `implemented`); HostFn 554; B-04/B-05/B-06 still `[~]`; ≠ Array alpn API / ≠ no-idle daemon / ≠ B-04 `[x]` / ≠ MIR blob.
- **W444 / ADR-788:** TLS concurrent serve first-slice met (`host_tls_serve` thread-per-conn workers for handshake + `HANDLED\n`; idle exit retained; drain before return; `SYX_TLS_SERVE_ALPN` retained at W444 charter; live language alpn W445); HostFn 554; B-04/B-05/B-06 still `[~]`; ≠ no-idle daemon / ≠ thread-pool product API / ≠ Array alpn API / ≠ B-04 `[x]` / ≠ MIR blob.
- **W443 / ADR-787:** TLS configurable ALPN first-slice met (`host_tls_serve` reads `SYX_TLS_SERVE_ALPN` comma-separated; default `[syx/1]`; idle unbounded + HANDLED retained; HostFn arity unchanged; `TlsServe` still `implemented`); HostFn 554; B-04/B-05/B-06 still `[~]`; ≠ language/HostFn alpn parameter / ≠ concurrent at W443 (delivered W444) / ≠ B-04 `[x]` / ≠ MIR blob.
- **W442 / ADR-786:** TLS unbounded multi-conn first-slice met (`host_tls_serve` sequential idle-timeout accept — 400ms after ≥1 handled; overall 8s; HANDLED + ALPN `syx/1` at W442; live env ALPN W443); HostFn 554; B-04 still `[~]`; ≠ concurrent / ≠ language alpn param / ≠ B-04 `[x]` / ≠ MIR blob.
- **W441 / ADR-785:** TLS `TlsServe` HostFn status→implemented first-slice met (`status=partial→implemented` on N=2 + HANDLED + ALPN `syx/1` at W441); live accept unbounded (W442); HostFn 554; B-04 still `[~]`; ≠ concurrent / ≠ configurable ALPN / ≠ B-04 `[x]` / ≠ MIR blob.
- **W440 / ADR-784:** B-04 Short default emit multi-root first-slice met (`MirModule.aot_export_roots` drives default emit/compact; empty roots still main-only); B-04 still `[~]`; ≠ language `Module.exports` discovery / ≠ B-04 `[x]` / ≠ TLS / ≠ MIR blob; HostFn 554.
- **W439 / ADR-783:** B-04 Short absolute-threshold first-slice met (`absolute_threshold_count=3`; ceilings 86/86/165); default emit still main-only at W439; B-04 still `[~]`; ≠ default multi-root (delivered W440) / ≠ B-05 `[x]`; HostFn 554.
- **W438 / ADR-782:** TLS ALPN first-slice met (fixed `syx/1` on N=2 + HANDLED at W438); `TlsServe` HostFn still partial at W438; live implemented (W441); live accept unbounded (W442); ≠ configurable ALPN / ≠ concurrent / ≠ Short / ≠ B-05 `[x]`; HostFn 554.
- **W437 / ADR-781:** TLS multi-conn first-slice met (N=2 sequential accepts at W437; `HANDLED` each); live unbounded (W442); `TlsServe` HostFn still partial at W437; ≠ concurrent / ≠ Short / ≠ B-05 `[x]`; HostFn 554.
- **W436 / ADR-780:** TLS handler application-data first-slice met (`HANDLED` after handshake; single accept at W436 charter); `TlsServe` HostFn still partial; ≠ ALPN / ≠ Short / ≠ B-05 `[x]`; HostFn 554.
- **W435 / ADR-779:** TLS product status raise first-slice met (`net.tls.server` `status=implemented`; native/wasm=real; vm/mir=partial); `TlsServe` HostFn still partial; ≠ handler/ALPN/multi-conn / ≠ Short / ≠ B-05 `[x]`; HostFn 554.
- **W434 / ADR-778:** Long delete_syx_mir_blob_enabled first-slice met (`delete_syx_mir_blob_enabled=true`); emit already without blob/sidecar; ≠ TLS raise / ≠ Short / ≠ B-05 `[x]`; HostFn 554.
- **W433 / ADR-777:** Long named HostFn stub CALL_HOST clear first-slice met (json×2 + tcp/tls → `rt_call_host_native`); `named_hostfn_stub_call_host_clear_signed=true`; `named_hostfn_stub_call_host_gap_open=false`; ≠ delete flag / ≠ TLS raise / ≠ B-05 `[x]`; HostFn 554.
- **W432 / ADR-776:** Long MIR-free first-slice met (`mir_free_enabled=true`); ≠ named stub clear / ≠ B-05 `[x]`; HostFn 554.
- **W431 / ADR-775:** Long sidecar-omit emit met (llvm/C stub stop sidecar; empty+register production init); `sidecar_constant_still_embedded=false`; ≠ MIR-free; HostFn 554.
- **W430 / ADR-774:** Long sidecar-omit C stub align wire met (C stub register after sidecar); sidecar still embedded; ≠ emit omit / ≠ MIR-free; HostFn 554.
- **W429 / ADR-773:** Long sidecar-omit Register-only wire met (empty→register→call_fn); sidecar still embedded; C stub register gap still open; ≠ emit omit / ≠ MIR-free; HostFn 554.
- **W428 / ADR-772:** Long sidecar-omit wire Empty-pool met (`syx_rt_install_empty_runtime`); sidecar still embedded; ≠ emit omit / ≠ MIR-free; HostFn 554.
- **W427 / ADR-771:** Long sidecar-omit deepen thin met; sidecar still embedded; ≠ wire / ≠ emit omit / ≠ MIR-free; HostFn 554.
### Wave A W426 正确性地板加深（ADR-770 · Long sidecar-omit thin-open inventory · 已交）

- **主轴：** **Long sidecar-omit thin-open inventory**（新建 `long-mir-free-sidecar-omit.md` + `long_mir_free_sidecar_omit_thin.rs` + `MirModuleLongHorizonSidecarOmitThinInventory`；`sidecar_omit_thin_signed=true`；**`sidecar_constant_still_embedded=true`（仍嵌）**；host gap closed 保持；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** emit omit / **非** MIR-free / **非** 去 sidecar 已交 / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-sixth` / `A-W426` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_sidecar_omit_thin.rs` · syx-mir/syx-rt sidecar-omit thin inventory · `ga-evidence-audit.md` W426
- **Frozen：** 删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；批量具名 HostFn stub 改写；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W427+ — sidecar-omit deepen/wire/omit；或 TLS / Short

### Wave A W425 正确性地板加深（ADR-769 · Long host-bridge pure-native gap close · 已交）

- **主轴：** **Long host-bridge pure-native gap close**（新建 `long-mir-free-host-bridge-gap-close.md` + `long_mir_free_host_bridge_gap_close.rs` + `MirModuleLongHorizonHostBridgePureNativeGapCloseInventory`；`host_bridge_pure_native_gap_close_signed=true`；**`host_bridge_pure_native_gap_open=false`**；Print/CallHost wire 保持；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** MIR-free / **非** 去 sidecar / **非** 具名 stub 清零 / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-fifth` / `A-W425` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_host_bridge_gap_close.rs` · syx-mir/syx-rt gap-close inventory · `ga-evidence-audit.md` W425
- **Frozen：** 删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；批量具名 HostFn stub 改写；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W426 Long sidecar-omit thin **已交**（ADR-770）→ W427+

### Wave A W424 正确性地板加深（ADR-768 · Long host-bridge CallHost wire first-slice · 已交）

- **主轴：** **Long host-bridge CallHost wire first-slice**（`syx_rt_call_host`/`_aot` → `rt_call_host_native`；新建 `long-mir-free-host-bridge-callhost-wire.md` + `long_mir_free_host_bridge_callhost_wire.rs` + `MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory`；`host_bridge_call_host_wire_first_slice_signed=true`；`call_host_mir_opcode_bridge_gap_open=false`；Print wire 保持；**`host_bridge_pure_native_gap_open=true`（仍 open）**；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** host gap close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-fourth` / `A-W424` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_host_bridge_callhost_wire.rs` · syx-mir/syx-rt CallHost wire inventory · `ga-evidence-audit.md` W424
- **Frozen：** host-bridge 硬关；CallHost fail-closed；批量具名 HostFn stub 改写；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427+

### Wave A W423 正确性地板加深（ADR-767 · Long host-bridge Print wire first-slice · 已交）

- **主轴：** **Long host-bridge Print wire first-slice**（`syx_rt_print`/`_aot` → `rt_print_native`；新建 `long-mir-free-host-bridge-wire.md` + `long_mir_free_host_bridge_wire.rs` + `MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory`；`host_bridge_print_wire_first_slice_signed=true`；`print_mir_opcode_bridge_gap_open=false`；**`host_bridge_pure_native_gap_open=true`（仍 open）**；CallHost **仍桥**；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** CallHost wire / **非** host gap close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-third` / `A-W423` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_host_bridge_wire.rs` · syx-mir/syx-rt Print wire inventory · `ga-evidence-audit.md` W423
- **Frozen：** host-bridge 硬关；CallHost 去 `rt_direct_exec`；Print fail-closed；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W424 Long host-bridge CallHost wire **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427+

### Wave A W422 正确性地板加深（ADR-766 · Long host-bridge deepen thin-open inventory · 已交）

- **主轴：** **Long host-bridge deepen thin-open inventory**（新建 `long-mir-free-host-bridge-deepen.md` + `long_mir_free_host_bridge_deepen_thin.rs` + `MirModuleLongHorizonHostBridgeDeepenThinInventory`；`host_bridge_deepen_thin_signed=true`；`host_bridge_thin_signed=true`；**`host_bridge_pure_native_gap_open=true`（仍 open）**；按类前置条件表；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** wire / **非** host gap close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-second` / `A-W422` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_host_bridge_deepen_thin.rs` · syx-mir/syx-rt host-bridge deepen inventory · `ga-evidence-audit.md` W422
- **Frozen：** host-bridge 硬关；Print/`CallHost` 去 `rt_direct_exec`；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W423 Long host-bridge Print wire **已交**（ADR-767）→ W424 Long host-bridge CallHost wire **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427+

### Wave A W421 正确性地板加深（ADR-765 · Long host-bridge thin-open inventory · 已交）

- **主轴：** **Long host-bridge thin-open inventory**（新建 `long-mir-free-host-bridge.md` + `long_mir_free_host_bridge_thin.rs` + `MirModuleLongHorizonHostBridgeThinInventory`；`host_bridge_thin_signed=true`；**`host_bridge_pure_native_gap_open=true`（仍 open）**；gap #3 closed 保持；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** host gap close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventy-first` / `A-W421` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_host_bridge_thin.rs` · syx-mir/syx-rt host-bridge thin inventory · `ga-evidence-audit.md` W421
- **Frozen：** host-bridge 硬关；Print/`CallHost` 去 `rt_direct_exec`；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire **已交**（ADR-767）→ W424+

### Wave A W420 正确性地板加深（ADR-764 · Long closure-async MIR-free gap #3 close · 已交）

- **主轴：** **Long closure-async MIR-free gap #3 close**（新建 `long-mir-free-closure-async-gap-close.md` + `long_mir_free_closure_async_gap_close.rs` + `MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory`；`closure_async_mir_free_gap_close_signed=true`；**`closure_async_mir_free_gap_open=false`**；四子 wire signed + 子旗 closed 保持；`host_bridge_pure_native_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** MIR-free / **非** 去 sidecar / **非** host-bridge closed / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventieth` / `A-W420` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_gap_close.rs` · syx-mir/syx-rt gap-close inventory · `ga-evidence-audit.md` W420
- **Frozen：** host-bridge 硬关；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W421 Long host-bridge thin **已交**（ADR-765）→ W422+

### Wave A W419 正确性地板加深（ADR-763 · Long closure-async Actor wire first-slice · 已交）

- **主轴：** **Long closure-async Actor wire first-slice**（`syx_rt_actor_*` fail-closed；`mir_function_needs_mir_entry` 去掉全部 Actor*（恒 false）；新建 `long-mir-free-closure-async-actor-wire.md` + `long_mir_free_closure_async_actor_wire.rs` + `MirModuleLongHorizonClosureAsyncActorWireFirstSliceInventory`；`closure_async_actor_wire_first_slice_signed=true`；`actor_mir_interpret_fallback_gap_open=false`；`closure_async_mir_free_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap #3 close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-ninth` / `A-W419` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_actor_wire.rs` · syx-rt Actor fail-closed 单测 · syx-llvm entry 单测 · syx-mir/syx-rt Actor wire inventory · `ga-evidence-audit.md` W419
- **Frozen：** 关 `closure_async_mir_free_gap_open`；真 native Actor；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W420 Long gap #3 close **已交**（ADR-764）→ W421+

### Wave A W418 正确性地板加深（ADR-762 · Long closure-async Task wire first-slice · 已交）

- **主轴：** **Long closure-async Task wire first-slice**（`syx_rt_task_*` fail-closed；`mir_function_needs_mir_entry` 去掉 Task*；新建 `long-mir-free-closure-async-task-wire.md` + `long_mir_free_closure_async_task_wire.rs` + `MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory`；`closure_async_task_wire_first_slice_signed=true`；`task_mir_interpret_fallback_gap_open=false`；`closure_async_mir_free_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap #3 close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-eighth` / `A-W418` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_task_wire.rs` · syx-rt Task fail-closed 单测 · syx-llvm entry 单测 · syx-mir/syx-rt Task wire inventory · `ga-evidence-audit.md` W418
- **Frozen：** 关 `closure_async_mir_free_gap_open`；改 Actor* stubs/entry；真 native Task；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421+

### Wave A W417 正确性地板加深（ADR-761 · Long closure-async Await wire first-slice · 已交）

- **主轴：** **Long closure-async Await wire first-slice**（`syx_rt_await_*` fail-closed；`mir_function_needs_mir_entry` 去掉 Await*；新建 `long-mir-free-closure-async-await-wire.md` + `long_mir_free_closure_async_await_wire.rs` + `MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory`；`closure_async_await_wire_first_slice_signed=true`；`await_mir_interpret_fallback_gap_open=false`；`closure_async_mir_free_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap #3 close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-seventh` / `A-W417` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_await_wire.rs` · syx-rt Await fail-closed 单测 · syx-llvm entry 单测 · syx-mir/syx-rt Await wire inventory · `ga-evidence-audit.md` W417
- **Frozen：** 关 `closure_async_mir_free_gap_open`；改 Task*/Actor* stubs/entry；真 native Await；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W418 Long Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420+

### Wave A W416 正确性地板加深（ADR-760 · Long closure-async wire first-slice · CallFn · 已交）

- **主轴：** **Long closure-async wire first-slice（CallFn）**（`syx_rt_call_fn` 无 native fail-closed；新建 `long-mir-free-closure-async-wire.md` + `long_mir_free_closure_async_wire.rs` + `MirModuleLongHorizonClosureAsyncWireFirstSliceInventory`；`closure_async_wire_first_slice_signed=true`；`call_fn_mir_interpret_fallback_gap_open=false`；`closure_async_mir_free_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap #3 close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-sixth` / `A-W416` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_wire.rs` · syx-rt `syx_rt_call_fn` fail-closed 单测 · syx-mir/syx-rt wire inventory · `ga-evidence-audit.md` W416
- **Frozen：** 关 `closure_async_mir_free_gap_open`；改 Task*/Actor* entry this window；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W417 Long Await wire first-slice **已交**（ADR-761）→ W418+

### Wave A W415 正确性地板加深（ADR-759 · Long closure-async MIR-free deepen thin · 已交）

- **主轴：** **Long closure-async MIR-free deepen thin-open inventory**（新建 `long-mir-free-closure-async-deepen.md` + `long_mir_free_closure_async_deepen_thin.rs` + `MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory`；`closure_async_mir_free_deepen_thin_signed=true`；`closure_async_mir_free_gap_open=true`（仍 open）；按类前置条件表；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap close / **非** MIR-free / **非** 去 sidecar / **非** wire / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-fifth` / `A-W415` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_deepen_thin.rs` · syx-mir/syx-rt Long closure-async deepen 单测 · `ga-evidence-audit.md` W415
- **Frozen：** 关 `closure_async_mir_free_gap_open`；改 `mir_function_needs_mir_entry`；删 `@syx_metadata_sidecar`；`mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long Await wire first-slice **已交**（ADR-761）→ W418+

### Wave A W414 正确性地板加深（ADR-758 · Long closure-async MIR-free thin · 已交）

- **主轴：** **Long closure-async MIR-free thin-open inventory**（新建 `long-mir-free-closure-async.md` + `long_mir_free_closure_async_thin.rs` + `MirModuleLongHorizonClosureAsyncMirFreeThinInventory`；`closure_async_mir_free_thin_signed=true`；`closure_async_mir_free_gap_open=true`（仍 open）；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap close / **非** MIR-free / **非** 去 sidecar / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-fourth` / `A-W414` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_closure_async_thin.rs` · syx-mir/syx-rt Long closure-async thin 单测 · `ga-evidence-audit.md` W414
- **Frozen：** gap close / 改 `mir_function_needs_mir_entry`；删 sidecar / `mir_free_enabled=true`；host-bridge 硬关；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long Await wire first-slice **已交**（ADR-761）→ W418+

### Wave A W413 正确性地板加深（ADR-757 · Long emit-omit first-slice · 已交）

- **主轴：** **Long emit-omit first-slice**（llvm/C stub 停 `@syx_mir_blob` / `load_mir_blob`；`@syx_metadata_sidecar` + `syx_rt_install_metadata_sidecar`；新建 `long-mir-free-emit-omit.md` + `long_mir_free_emit_omit.rs` + `MirModuleLongHorizonEmitOmitFirstSliceInventory`；`emit_omit_first_slice_signed=true`；两 omit gap **closed**；`blob_still_embedded=false`；`sidecar_constant_still_embedded=true`；`mir_free_enabled=false`；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** MIR-free / **非** 无一切嵌入元数据 / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-third` / `A-W413` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_emit_omit.rs` · syx-llvm shim/C stub 单测 · syx-mir/syx-rt emit-omit inventory · `ga-evidence-audit.md` W413
- **Frozen：** `mir_free_enabled=true`；删 `@syx_metadata_sidecar`；closure-async / host-bridge；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417+

### Wave A W412 正确性地板加深（ADR-756 · Long native-metadata wire first-slice · 已交）

- **主轴：** **Long native-metadata wire first-slice**（公开 C ABI `syx_rt_install_metadata_sidecar`；新建 `long-mir-free-native-metadata-wire.md` + `long_mir_free_native_metadata_wire.rs` + `MirModuleLongHorizonNativeMetadataWireFirstSliceInventory`；`native_metadata_wire_first_slice_signed=true`；`native_metadata_without_blob_gap_open=false`（gap #1 closed）；`mir_free_enabled=false`；blob **仍嵌入**；`syx_user_init` **仍** load blob；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** MIR-free / **非** 删 blob / **非** emit cutover / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-second` / `A-W412` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_native_metadata_wire.rs` · syx-rt `syx_rt_install_metadata_sidecar` 单测 · syx-mir/syx-rt wire inventory · `ga-evidence-audit.md` W412
- **Frozen：** 删除 `@syx_mir_blob` / 停 `syx_user_init`→`load_mir_blob` / MIR-free emit 切流；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W413 Long emit-omit first-slice **已交**（ADR-757）→ W414+

### Wave A W411 正确性地板加深（ADR-755 · Long native-metadata deepen thin · 已交）

- **主轴：** **Long native-metadata-without-blob deepen thin-open inventory**（新建 `long-mir-free-native-metadata-deepen.md` + `long_mir_free_native_metadata_deepen_thin.rs` + `MirModuleLongHorizonNativeMetadataDeepenThinInventory`；`native_metadata_deepen_thin_signed=true`；`native_metadata_without_blob_gap_open=true`（仍 open）；`mir_free_enabled=false`；blob **仍嵌入**；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** gap close / **非** 删除 `@syx_mir_blob` / **非** MIR-free / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixty-first` / `A-W411` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_native_metadata_deepen_thin.rs` · syx-mir/syx-rt Long deepen 单测 · `ga-evidence-audit.md` W411
- **Frozen：** gap close / 公开无 blob 入口；删除 `@syx_mir_blob` / MIR-free emit；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W412 Long native-metadata wire first-slice **已交**（ADR-756）→ W413+

### Wave A W410 正确性地板加深（ADR-754 · Long MIR-free readiness thin · 已交）

- **主轴：** **Long MIR-free readiness thin-open inventory**（新建 `long-mir-free-readiness.md` + `long_mir_free_readiness_thin.rs` + `MirModuleLongHorizonMirFreeReadinessThinInventory`；`long_mir_free_readiness_thin_signed=true`；`mir_free_enabled=false`；`delete_syx_mir_blob_enabled=false`；`blob_still_embedded=true`；`syx_user_init_still_loads_blob=true`；Long gaps open；前置 Medium execution-omit；B-04/B-05/B-06 **仍 `[~]`**）；HostFn **554**；**非** 删除 `@syx_mir_blob` / **非** MIR-free / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixtieth` / `A-W410` · gate **47/47** · soft **0**
- **证据：** `long_mir_free_readiness_thin.rs` · syx-mir/syx-rt Long readiness 单测 · `ga-evidence-audit.md` W410
- **Frozen：** 删除 `@syx_mir_blob` / MIR-free emit；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W411 Long native-metadata deepen thin **已交**（ADR-755）→ W412+

### Wave A W409 正确性地板加深（ADR-753 · B-05 Medium AOT execution-segment omit · 已交）

- **主轴：** **B-05 Medium AOT execution-segment omit first-slice**（`encode_module_aot_embed_blob` execution 空；`install_runtime_from_sidecar_only`；新建 `b05-execution-omit.md` + `b05_execution_omit.rs` + `MirModuleMediumHorizonExecutionOmitInventory`；`execution_omit_signed=true`；`aot_execution_segment_omitted=true`；sidecar-primary/dual-load/embed 保持；B-05 **仍 `[~]`**）；HostFn **554**；**非** 删除 `@syx_mir_blob` / **非** MIR-free / **非** B-05 `[x]`；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-ninth` / `A-W409` · gate **47/47** · soft **0**
- **证据：** `b05_execution_omit.rs` · syx-mir/syx-rt execution-omit/sidecar-only 单测 · `ga-evidence-audit.md` W409
- **Frozen：** 删除 `@syx_mir_blob` / MIR-free；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W410 Long MIR-free readiness thin **已交**（ADR-754）→ W411+

### Wave A W408 正确性地板加深（ADR-752 · B-05 Medium sidecar-primary deepen · 已交）

- **主轴：** **B-05 Medium sidecar-primary deepen**（公开 `module_with_execution_string_table_stripped`；`encode_module_aot_embed_blob` execution=compact(strip)；`syx_rt_load_mir_blob` SYXD reset pool + sidecar-primary；新建 `b05-sidecar-primary.md` + `b05_sidecar_primary.rs` + `MirModuleMediumHorizonSidecarPrimaryInventory`；`sidecar_primary_signed=true`；`sidecar_string_pool_primary=true`；`execution_string_table_stripped=true`；dual-load/embed/cutover/dual/codec 保持；B-05 **仍 `[~]`**）；HostFn **554**；**非** 删除 `@syx_mir_blob` / **非** 整段 execution-omit / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-eighth` / `A-W408` · gate **47/47** · soft **0**
- **证据：** `b05_sidecar_primary.rs` · syx-mir/syx-rt sidecar-primary/strip 单测 · `ga-evidence-audit.md` W408
- **Frozen：** 删除 `@syx_mir_blob` / MIR-free；整段 execution-omit；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W409 B-05 Medium AOT execution-segment omit **已交**（ADR-753）→ W410 Long MIR-free readiness thin **已交**（ADR-754）→ W411+

### Wave A W407 正确性地板加深（ADR-751 · B-05 Medium dual-load runtime first-slice · 已交）

- **主轴：** **B-05 Medium dual-load runtime first-slice**（公开 `peel_dual_blob_package_sidecar` + `install_metadata_sidecar_view`；`syx_rt_load_mir_blob` SYXD 双边 peel + sidecar→RT string pool；新建 `b05-dual-load-runtime.md` + `b05_dual_load_runtime.rs` + `MirModuleMediumHorizonDualLoadRuntimeInventory`；`dual_load_runtime_signed=true`；`dual_load_runtime_enabled=true`；`load_path_is_whole_module=false`；embed/cutover/dual/codec 保持；B-05 **仍 `[~]`**）；HostFn **554**；**非** 删除 `@syx_mir_blob` / **非** execution-omit / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-seventh` / `A-W407` · gate **47/47** · soft **0**
- **证据：** `b05_dual_load_runtime.rs` · syx-mir/syx-rt dual-load/peel/install 单测 · `ga-evidence-audit.md` W407
- **Frozen：** 删除 `@syx_mir_blob` / MIR-free；execution-omit embed；B-04/B-05/B-06 `[x]`；TLS status=implemented
- **下一窗：** Wave A W408 B-05 Medium sidecar-primary deepen **已交**（ADR-752）→ W409+

### Wave A W406 正确性地板加深（ADR-750 · B-05 Medium AOT sidecar embed first-slice · 已交）

- **主轴：** **B-05 Medium AOT sidecar embed first-slice**（公开 `encode_module_aot_embed_blob`；llvm `@syx_mir_blob`=SYXD package；`syx_rt_load_mir_blob` peel execution；新建 `b05-aot-sidecar-embed.md` + `b05_aot_sidecar_embed.rs` + `MirModuleMediumHorizonAotSidecarEmbedInventory`；`aot_sidecar_embed_signed=true`；`aot_embeds_sidecar=true`；`load_path_is_whole_module=true`；cutover/dual/codec 保持；B-05 **仍 `[~]`**）；HostFn **554**；**非** dual-load runtime / **非** 删除 `@syx_mir_blob` / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-sixth` / `A-W406` · gate **47/47** · soft **0**
- **证据：** `b05_aot_sidecar_embed.rs` · syx-mir/syx-llvm/syx-rt aot embed/peel 单测 · `ga-evidence-audit.md` W406
- **Frozen：** dual-load runtime；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob` / MIR-free；TLS status=implemented
- **下一窗：** Wave A W407+ — dual-load runtime / TLS / absolute thresholds / Long MIR-free

### Wave A W405 正确性地板加深（ADR-749 · B-05 Medium production cutover honesty · 已交）

- **主轴：** **B-05 Medium production cutover honesty flip**（新建 `b05-production-cutover.md` + `b05_production_cutover_flip.rs` + `MirModuleMediumHorizonProductionCutoverInventory`；`production_cutover_signed=true`；Medium `cutover_enabled=true`；`aot_embeds_sidecar=false`；load 仍 whole-module；dual/codec wire 保持；B-05 **仍 `[~]`**）；HostFn **554**；**非** 默认 AOT sidecar embed / **非** dual-load runtime / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-fifth` / `A-W405` · gate **47/47** · soft **0**
- **证据：** `b05_production_cutover_flip.rs` · syx-mir medium_horizon/production_cutover 单测 · `ga-evidence-audit.md` W405
- **Frozen：** 默认 AOT sidecar embed；dual-load runtime；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W407 B-05 Medium dual-load runtime first-slice **已交**（ADR-751）→ W408+

### Wave A W404 正确性地板加深（ADR-748 · B-05 Medium dual-blob production wire · 已交）

- **主轴：** **B-05 Medium dual-blob production wire**（公开 `encode_module_dual_blob_package` / `decode_module_dual_blob_package`；新建 `b05-dual-blob-production-wire.md` + `b05_dual_blob_production_wire.rs` + `MirModuleMediumHorizonDualBlobProductionWireInventory`；`production_dual_load_enabled=true`；`dual_blob_reopen_gap_open=false`；`aot_embeds_sidecar=false`；Medium cutover **仍 false**；B-05 **仍 `[~]`**）；HostFn **554**；**非** 默认 AOT sidecar embed / **非** Medium cutover / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-fourth` / `A-W404` · gate **47/47** · soft **0**
- **证据：** `b05_dual_blob_production_wire.rs` · syx-mir dual_blob/medium_horizon 单测 · `ga-evidence-audit.md` W404
- **Frozen：** 默认 AOT sidecar embed；Medium cutover；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+

### Wave A W403 正确性地板加深（ADR-747 · B-05 Medium real codec first-slice · 已交）

- **主轴：** **B-05 Medium real codec first-slice wire**（公开 `encode_module_metadata_sidecar` / `decode_module_metadata_sidecar`；新建 `b05-real-codec-first-slice.md` + `b05_real_codec_first_slice.rs` + `MirModuleMediumHorizonRealCodecFirstSliceInventory`；`real_codec_enabled=true`；`real_codec_reopen_gap_open=false`；dual-load/sidecar-embed/Medium cutover **仍 false**；B-05 **仍 `[~]`**）；HostFn **554**；**非** 双 blob 生产 / **非** Medium cutover / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-third` / `A-W403` · gate **47/47** · soft **0**
- **证据：** `b05_real_codec_first_slice.rs` · syx-mir real_codec/metadata_sidecar 单测 · `ga-evidence-audit.md` W403
- **Frozen（W403 签字时）：** AOT sidecar embed；dual-load；Medium cutover；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+

### Wave A W402 正确性地板加深（ADR-746 · B-05 Medium cutover deepen thin · 已交）

- **主轴：** **B-05 Medium cutover deepen thin-open inventory**（policy+meta；新建 `b05-production-cutover-deepen.md` + `b05_production_cutover_deepen_thin.rs` + `MirModuleMediumHorizonCutoverDeepenThinInventory`；`cutover_deepen_thin_signed=true`；`cutover`/`real_codec`/dual-load/sidecar **仍 false**；gaps open；B-05 **仍 `[~]`**）；HostFn **554**；**非** 真 codec / **非** 双 blob 生产 / **非** Medium cutover flip / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-second` / `A-W402` · gate **47/47** · soft **0**
- **证据：** `b05_production_cutover_deepen_thin.rs` · syx-mir medium_horizon/cutover_deepen 单测 · `ga-evidence-audit.md` W402
- **Frozen：** Medium 生产旗 flip；真 codec / dual-blob；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+

### Wave A W401 正确性地板加深（ADR-745 · B-05 Medium production reopen thin · 已交）

- **主轴：** **B-05 Medium production reopen thin-open inventory**（policy+meta；新建 `b05-production-reopen.md` + `b05_production_reopen_thin.rs` + `MirModuleMediumHorizonProductionReopenThinInventory`；`production_reopen_thin_signed=true`；`cutover`/`real_codec`/dual-load/sidecar **仍 false**；gaps open；B-05 **仍 `[~]`**）；HostFn **554**；**非** 真 codec / **非** 双 blob 生产 / **非** B-05 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifty-first` / `A-W401` · gate **47/47** · soft **0**
- **证据：** `b05_production_reopen_thin.rs` · syx-mir medium_horizon/production_reopen 单测 · `ga-evidence-audit.md` W401
- **Frozen：** Medium 生产旗 flip；真 codec / dual-blob；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W402 B-05 Medium cutover deepen thin **已交**（ADR-746）→ W403+

### Wave A W400 正确性地板加深（ADR-744 · B-04 Short production cutover honesty · 已交）

- **主轴：** **B-04 Short production cutover honesty flip**（Short 链 `cutover_enabled=true`；`production_cutover_signed=true`；默认 emit/compact **仍 main-only**；wire+size-CI 保持；`absolute_threshold_count=0`；新建 `b04-production-cutover.md` + `b04_production_cutover_flip.rs` + `AotShortHorizonProductionCutoverInventory`；B-04 **仍 `[~]`**）；HostFn **554**；**非** 默认多根行为切流 / **非** 绝对阈值表 / **非** B-04 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fiftieth` / `A-W400` · gate **47/47** · soft **0**
- **证据：** `b04_production_cutover_flip.rs` · syx-mir cutover/short_horizon 单测 · `ga-evidence-audit.md` W400
- **Frozen：** 默认 emit 自动多根；绝对字节阈值表；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W401 B-05 Medium production reopen thin **已交**（ADR-745）→ W402+

### Wave A W399 正确性地板加深（ADR-743 · B-04 Short size golden CI gate · 已交）

- **主轴：** **B-04 Short size golden CI gate deepen**（关系三谓词门控；`ci_gate_enabled=true` / `size_ci_gate_enabled=true`；`size_ci_reopen_gap_open=false`；`absolute_threshold_count=0`；W399 时 `cutover_enabled` **仍 false**；新建 `b04-size-ci-gate.md` + `b04_size_ci_gate_deepen.rs` + `AotShortHorizonSizeCiGateInventory`；B-04 **仍 `[~]`**）；HostFn **554**；**非** cutover flip / **非** 绝对阈值表 / **非** B-04 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-ninth` / `A-W399` · gate **47/47** · soft **0**
- **证据：** `b04_size_ci_gate_deepen.rs` · syx-mir size_ci/golden/short_horizon 单测 · `ga-evidence-audit.md` W399
- **Frozen：** cutover flip（W399）；绝对字节阈值表；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented
- **下一窗：** Wave A W400 B-04 Short production cutover honesty flip **已交**（ADR-744）→ W401+

### Wave A W398 正确性地板加深（ADR-742 · B-04 Short multi-root production wire · 已交）

- **主轴：** **B-04 Short multi-root production wire deepen**（`reachable_fn_names_from_roots` / `llvm_emit_function_names_with_roots` / shaken+compact `_with_roots`；默认空根仍 main-only；`multi_root_reopen_gap_open=false`；`production_multi_root_enabled=true`；cutover/size-CI **仍 false**；新建 `b04-multi-root-wire.md` + `b04_multi_root_wire_deepen.rs` + `AotShortHorizonMultiRootWireInventory`；B-04 **仍 `[~]`**）；HostFn **554**；**非** cutover flip / **非** B-04 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-eighth` / `A-W398` · gate **47/47** · soft **0**
- **证据：** `b04_multi_root_wire_deepen.rs` · syx-mir multi_root/export_root 单测 · `ga-evidence-audit.md` W398
- **Frozen：** cutover/size-CI flip；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；semantic auto-export 根；TLS status=implemented
- **下一窗：** Wave A W399 B-04 Short size golden CI gate **已交**（ADR-743）→ W400+

### Wave A W397 正确性地板加深（ADR-741 · B-04 Short cutover deepen thin · 已交）

- **主轴：** **B-04 Short cutover deepen thin-open inventory**（policy+meta；新建 `b04-production-cutover-deepen.md` + `b04_production_cutover_deepen_thin.rs` + `AotShortHorizonCutoverDeepenThinInventory`；`cutover_deepen_thin_signed=true`；三旗 **仍 false**；emit **仍 main-only**；gaps 仍 open；B-04 **仍 `[~]`**）；W396 reopen thin / W338 defer 保持；TLS GA **met (narrow)** / HostFn **554** 保持；**非** multi-root wire / **非** cutover flip / **非** B-04 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-seventh` / `A-W397` · gate **47/47** · soft **0**
- **证据：** `b04_production_cutover_deepen_thin.rs` · syx-mir cutover_deepen 单测 · `ga-evidence-audit.md` W397
- **Frozen：** 三旗 flip；生产多根 emit wire；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented；handler deepen；清 W338/W396
- **下一窗：** Wave A W398 B-04 Short multi-root production wire **已交**（ADR-742）→ W399+

### Wave A W396 正确性地板加深（ADR-740 · B-04 Short production reopen thin · 已交）

- **主轴：** **B-04 Short production reopen thin-open inventory**（policy+meta；新建 `b04-production-reopen.md` + `b04_production_reopen_thin.rs` + `AotShortHorizonProductionReopenThinInventory`；`production_reopen_thin_signed=true`；cutover/multi-root/size-CI **仍 false**；gaps open；B-04 **仍 `[~]`**）；TLS GA **met (narrow)** / HostFn **554** 保持；**非** cutover flip / **非** B-04 `[x]` / **非** MIR-free；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-sixth` / `A-W396` · gate **47/47** · soft **0**
- **证据：** `b04_production_reopen_thin.rs` · syx-mir production_reopen 单测 · `ga-evidence-audit.md` W396
- **Frozen：** cutover flip；B-04/B-05/B-06 `[x]`；删除 `@syx_mir_blob`；TLS status=implemented；handler deepen；清 W338 defer
- **下一窗：** Wave A W398 B-04 Short multi-root production wire **已交**（ADR-742）→ W399+

### Wave A W395 正确性地板加深（ADR-739 · TLS GA met exit narrow · 已交）

- **主轴：** **TLS GA met exit**（policy+meta；新建 `tls-server-ga-met-exit.md` + `tls_server_ga_met_exit.rs`；分轴于 honesty floor；TLS GA **met (narrow)** = inventory + llvm partial sufficient + wasm met + exit；`net.tls.server` **仍 unsupported**；HostFn **554**）；honesty floor / llvm / wasm / inventory 保持；accept·marketing·website·hosted **met (narrow)**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** status=implemented / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-fifth` / `A-W395` · gate **47/47** · soft **0**
- **证据：** `tls_server_ga_met_exit.rs` · vectors GA met · `ga-evidence-audit.md` W395
- **Frozen：** status=implemented；新 HostFn/handler；改写 honesty floor；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W396 B-04 Short production reopen thin **已交**（ADR-740）→ W397+

### Wave A W394 正确性地板加深（ADR-738 · TLS HostFn wasm first-slice · 已交）

- **主轴：** **TLS HostFn wasm first-slice**（`TlsServe` **wasm→real** + `qs1_tls_serve_wasm`；wasm first-slice **met**；llvm **仍 partial met**；TLS GA / `net.tls.server` status=implemented **仍 unmet**；HostFn **554**）；honesty floor **met**；marketing·accept·website·hosted **met (narrow)**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** TLS GA met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-fourth` / `A-W394` · gate **47/47** · soft **0**
- **证据：** `hostfn_tls.rs` wasm nail · deepen 翻锁 · vectors wasm met · `ga-evidence-audit.md` W394
- **Frozen：** TLS GA met 宣称；status=implemented；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；honesty-floor/accept/marketing reopen
- **下一窗：** Wave A W395+ — TLS GA exit (narrow) / B reopen

### Wave A W393 正确性地板加深（ADR-737 · TLS HostFn llvm first-slice deepen · 已交）

- **主轴：** **TLS HostFn llvm first-slice deepen**（真 HostFn；新增 `TlsServe`/`__syx_tls_serve` + `host_tls_serve` + `packages/net/tls.sy` + `qs1_tls_serve_native_llvm` + `tls_server_hostfn_deepen.rs`；llvm first-slice **partial met**；implement inventory **仍 met**；wasm / TLS GA / `net.tls.server` status=implemented **仍 unmet**；HostFn **554**）；honesty floor **met**；marketing·accept·website·hosted **met (narrow)**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** wasm / **非** TLS GA met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-third` / `A-W393` · gate **47/47** · soft **0**
- **证据：** `hostfn_tls.rs` · `tls_server_hostfn_deepen.rs` · vectors llvm partial · `ga-evidence-audit.md` W393
- **Frozen：** wasm first-slice；TLS GA met 宣称；status=implemented；noop 伪装；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；honesty-floor/accept/marketing reopen
- **下一窗：** Wave A W394+ — wasm first-slice / TLS GA exit / B reopen

### Wave A W392 正确性地板加深（ADR-736 · TLS HostFn implement thin-open inventory · 已交）

- **主轴：** **TLS HostFn implement thin-open / inventory**（policy+meta；更新 `tls-server-ga-vectors.md` + 新建 `tls_server_hostfn_thin.rs`；implement inventory thin **met**；候选 `TlsServe`/`__syx_tls_serve`；llvm-only 预告；wasm defer；禁 noop；`tls.sy` 规划位 unwired；TLS GA / 真 HostFn / status raise **仍 unmet**；HostFn **553**）；honesty floor **met**；marketing·accept·website·hosted **met (narrow)**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** llvm first-slice / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-second` / `A-W392` · gate **47/47** · soft **0**
- **证据：** `tls_server_hostfn_thin.rs` · vectors implement inventory · `ga-evidence-audit.md` W392
- **Frozen：** 本窗 HostFn 实现 / noop 伪装 / status raise；TLS GA claim；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；honesty floor/accept/marketing reopen
- **下一窗：** Wave A W393+ → llvm first-slice / wasm / TLS GA exit (narrow) / B reopen

### Wave A W391 正确性地板加深（ADR-735 · Marketing product met exit · scope narrow · 已交）

- **主轴：** **Marketing product met exit**（policy+meta；新建 `marketing-deploy-exit.md` + `marketing_deploy_exit.rs`；marketing product **met (narrow)** = inventory + deepen partial sufficient + exit；Static **仍 partial**；Hosted full site / marketing full site / 真部署 **仍 unmet**）；website·hosted **met (narrow)**；accept GA **met (narrow)**；TLS honesty floor **met**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** full site met / 真 TLS HostFn / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-forty-first` / `A-W391` · gate **47/47** · soft **0**
- **证据：** `marketing-deploy-exit.md` · `marketing_deploy_exit.rs` · vectors product met (narrow) · `ga-evidence-audit.md` W391
- **Frozen：** marketing full site / Static **met** 宣称；真部署；TLS HostFn / TLS GA claim；新 helper；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/TLS floor/residual reopen
- **下一窗：** Wave A W392 → **已交 ADR-736**（TLS HostFn implement thin inventory）；W393+

### Wave A W390 正确性地板加深（ADR-734 · Marketing same-source deepen · 已交）

- **主轴：** **Marketing same-source deepen**（helper+fixture；新建 `marketing_capabilities.rs` + `marketing_deploy_deepen.rs`；Static/marketing deploy tree **partial met**；inventory thin **仍 met**；Hosted full site / marketing met / 真部署 **仍 unmet**）；website·hosted **met (narrow)**；accept GA **met (narrow)**；TLS honesty floor **met**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** marketing met / 真 TLS HostFn / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fortieth` / `A-W390` · gate **47/47** · soft **0**
- **证据：** `marketing_capabilities.rs` · `marketing_deploy_deepen.rs` · vectors Static partial · `ga-evidence-audit.md` W390
- **Frozen：** marketing full site / Static **met** 宣称（仅 partial）；真部署；TLS HostFn / TLS GA claim；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/TLS floor/residual reopen
- **下一窗：** Wave A W391 → **已交 ADR-735**（marketing product met exit narrow）；W392+

### Wave A W389 正确性地板加深（ADR-733 · Marketing deploy thin-open inventory · 已交）

- **主轴：** **Marketing deploy thin-open / inventory**（policy+meta；新建 `marketing-deploy-vectors.md` + `marketing_deploy_thin.rs`；marketing inventory thin **met**；Static/Hosted full site / 真部署 **仍 unmet**）；website·hosted **met (narrow)**；accept GA **met (narrow)**；TLS honesty floor **met**；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** marketing met / 真 TLS HostFn / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-ninth` / `A-W389` · gate **47/47** · soft **0**
- **证据：** `marketing-deploy-vectors.md` · `marketing_deploy_thin.rs` · hosted/website 交叉 · `ga-evidence-audit.md` W389
- **Frozen：** marketing full site / 真部署；TLS HostFn / TLS GA claim；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/TLS floor/residual reopen
- **下一窗：** Wave A W390 → **已交 ADR-734**（marketing same-source deepen）；W391+

### Wave A W388 正确性地板加深（ADR-732 · TLS honesty-ladder floor exit · ≠ TLS GA · 已交）

- **主轴：** **TLS honesty-ladder floor exit**（policy+meta；新建 `tls-server-ga-exit.md` + `tls_server_ga_exit.rs`；honesty floor **met**；TLS GA / HostFn 实现 **仍 unmet**；HostFn **553**）；inventory thin **met**；HostFn-gap deepen **partial met**；accept GA **met (narrow)**；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** TLS GA met / 真 HostFn / status raise / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-eighth` / `A-W388` · gate **47/47** · soft **0**
- **证据：** `tls-server-ga-exit.md` · `tls_server_ga_exit.rs` · vectors floor met · `ga-evidence-audit.md` W388
- **Frozen：** TLS GA met 宣称；TLS HostFn 实现 / noop 伪装 / status raise；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/residual reopen
- **下一窗：** Wave A W389 → **已交 ADR-733**（marketing deploy thin inventory）；W390+

### Wave A W387 正确性地板加深（ADR-731 · TLS HostFn-gap honesty deepen · 已交）

- **主轴：** **TLS HostFn-gap honesty deepen**（policy+meta；更新 `tls-server-ga-vectors.md` + 新建 `tls_server_ga_deepen.rs`；deepen **partial met**；TLS GA / HostFn 实现 **仍 unmet**；HostFn **553**）；inventory thin **met**；accept GA **met (narrow)**；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 真 HostFn / status raise / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-seventh` / `A-W387` · gate **47/47** · soft **0**
- **证据：** `tls_server_ga_deepen.rs` · vectors partial · caps notes ADR-731 · `ga-evidence-audit.md` W387
- **Frozen：** TLS server HostFn 实现 / noop 伪装 / status raise；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/residual reopen
- **下一窗：** Wave A W388 → **已交 ADR-732**（TLS honesty-ladder floor exit ≠ GA）；W389+

### Wave A W386 正确性地板加深（ADR-730 · TLS server thin-open inventory · 已交）

- **主轴：** **TLS server thin-open / inventory**（policy+meta；新建 `tls-server-ga-vectors.md` + `tls_server_ga_thin.rs`；TLS inventory thin **met**；TLS GA / `net.tls.server` **仍 unmet**）；accept GA **met (narrow)**；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 实现 TLS / HostFn / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-sixth` / `A-W386` · gate **47/47** · soft **0**
- **证据：** `tls-server-ga-vectors.md` · `tls_server_ga_thin.rs` · caps notes ADR-730 · `ga-evidence-audit.md` W386
- **Frozen：** TLS server 实现 / status raise / HostFn；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；accept/residual reopen
- **下一窗：** Wave A W387 → **已交 ADR-731**（TLS HostFn-gap honesty deepen）；W388+

### Wave A W385 正确性地板加深（ADR-729 · TcpListen accept GA met exit · scope narrow · 已交）

- **主轴：** **TcpListen accept GA met exit**（policy+meta；新建 `tcp-accept-ga-exit.md` + `tcp_accept_ga_exit.rs`；accept GA **met (narrow)**；TLS **仍 unmet**）；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** caps status raise / TLS / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-fifth` / `A-W385` · gate **47/47** · soft **0**
- **证据：** `tcp-accept-ga-exit.md` · `tcp_accept_ga_exit.rs` · vectors accept GA met · `ga-evidence-audit.md` W385
- **Frozen：** TLS server GA；caps status=implemented raise；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；residual reopen
- **下一窗：** Wave A W386 → **已交 ADR-730**（TLS server thin inventory）；W387+

### Wave A W384 正确性地板加深（ADR-728 · TcpListenerAccept HostFn wasm first-slice · 已交）

- **主轴：** **TcpListenerAccept HostFn wasm first-slice**（`qs1_tcp_listener_accept_wasm` + wasm→real；Accept HostFn wasm **met**；accept GA / TLS **仍 unmet**）；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** accept GA met / TLS / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-fourth` / `A-W384` · gate **47/47** · soft **0**
- **证据：** `qs1_tcp_listener_accept_wasm` · caps wasm=real · vectors wasm met · `ga-evidence-audit.md` W384
- **Frozen：** accept GA met / TLS；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；residual reopen
- **下一窗：** Wave A W385 → **已交 ADR-729**（accept GA met exit narrow）；W386+

### Wave A W383 正确性地板加深（ADR-727 · TcpListen accept HostFn llvm first-slice deepen · 已交）

- **主轴：** **TcpListen accept HostFn llvm/native first-slice deepen**（`TcpListenerAccept` + `qs1_tcp_listener_accept_native_llvm` + `tcp_accept_ga_deepen.rs`；Accept HostFn llvm **partial met**；accept GA / wasm / TLS **仍 unmet**）；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** accept GA met / wasm accept / TLS / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-third` / `A-W383` · gate **47/47** · soft **0**
- **证据：** `TcpListenerAccept` · `qs1_tcp_listener_accept_native_llvm` · `tcp_accept_ga_deepen.rs` · vectors partial · `ga-evidence-audit.md` W383
- **Frozen：** accept GA met / wasm claim；TLS；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；residual reopen
- **下一窗：** Wave A W384 → **已交 ADR-728**（accept wasm first-slice）；W385+

### Wave A W382 正确性地板加深（ADR-726 · TcpListen accept GA thin-open inventory · 已交）

- **主轴：** **TcpListen accept GA thin-open / inventory**（policy+meta；新建 `tcp-accept-ga-vectors.md` + `tcp_accept_ga_thin.rs`；Accept inventory thin **met**；accept GA **仍 unmet**；TLS 仍 unmet）；residual **4/4**；website·hosted **met (narrow)** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 实现 accept / TLS / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-second` / `A-W382` · gate **47/47** · soft **0**
- **证据：** `tcp-accept-ga-vectors.md` · `tcp_accept_ga_thin.rs` · residual Accept inventory thin · `ga-evidence-audit.md` W382
- **Frozen：** 实现 accept GA / TLS claim；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；residual reopen
- **下一窗：** Wave A W383 → **已交 ADR-727**（accept llvm first-slice）；W384+

### Wave A W381 正确性地板加深（ADR-725 · Hosted product met exit · scope narrow · 已交）

- **主轴：** **Hosted product met exit**（policy+meta；新建 `hosted-site-exit.md` + `hosted_site_exit.rs`；scope narrow = inventory met + deepen partial 即足；Deployable hosted consumer **partial met→met (narrow)**；Hosted full site / marketing **仍 unmet**）；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** marketing deploy / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirty-first` / `A-W381` · gate **47/47** · soft **0**
- **证据：** `hosted-site-exit.md` · `hosted_site_exit.rs` · vectors Deployable met (narrow) · a10 交叉 ADR-725 · `ga-evidence-audit.md` W381
- **Frozen：** marketing/`docs|books` 上线；新 caps implemented 行；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W382 → **已交 ADR-726**（accept GA thin inventory）；W383+

### Wave A W380 正确性地板加深（ADR-724 · Hosted same-source deepen · 已交）

- **主轴：** **Hosted same-source deepen**（`hosted_capabilities.rs`：`capabilities_for_hosted` + `render_hosted_capabilities_page` + temp/fixture write → `run_capabilities_report()`；`hosted_site_deepen.rs` exact schema/count/`datetime.calendar` + fixture；Deployable hosted consumer **unmet→partial met**）；inventory 仍 met；**仍 ≠** Hosted full site met / marketing deploy；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirtieth` / `A-W380` · gate **47/47** · soft **0**
- **证据：** `hosted-site-vectors.md` · `hosted_capabilities.rs` · `hosted_site_deepen.rs` · a10 交叉 ADR-724 · `ga-evidence-audit.md` W380
- **Frozen：** Hosted full site met / marketing/`docs|books` 上线；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W381 → **已交 ADR-725**（hosted product met exit）；W382+

### Wave A W379 正确性地板加深（ADR-723 · Hosted full-site thin-open inventory · 已交）

- **主轴：** **Hosted full-site thin-open / inventory**（policy+meta；新建 `hosted-site-vectors.md` + `hosted_site_thin.rs`；Hosted inventory thin **met**；Hosted full site **仍 unmet**；website product **met (narrow)** 保持）；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** marketing deploy / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-ninth` / `A-W379` · gate **47/47** · soft **0**
- **证据：** `hosted-site-vectors.md` · `hosted_site_thin.rs` · website vectors Hosted inventory thin · a10 交叉 ADR-723 · `ga-evidence-audit.md` W379
- **Frozen：** Hosted full site met / marketing deploy；新 website/hosted caps implemented 行；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退；website met narrow 回退
- **下一窗：** Wave A W380 → **已交 ADR-724**（hosted deepen）；W381+

### Wave A W378 正确性地板加深（ADR-722 · Website product met exit · scope narrow · 已交）

- **主轴：** **Website product met exit**（policy+meta；新建 `website-capability-exit.md` + `website_capability_exit.rs`；scope narrow = inventory met + deepen partial 即足；独立 website 消费者 **partial met→met (narrow)**；Hosted full site **仍 unmet**）；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 完整站托管 / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-eighth` / `A-W378` · gate **47/47** · soft **0**
- **证据：** `website-capability-exit.md` · `website_capability_exit.rs` · vectors consumer met (narrow) · a10 交叉 ADR-722 · `ga-evidence-audit.md` W378
- **Frozen：** 完整站托管；新 website caps implemented 行；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W379 → **已交 ADR-723**（hosted thin inventory）；W380+

### Wave A W377 正确性地板加深（ADR-721 · Website same-source deepen · 已交）

- **主轴：** **Website same-source deepen**（`website_capabilities.rs`：`capabilities_for_website` + `render_capabilities_html_snippet` → `run_capabilities_report()`；`website_capability_deepen.rs` exact schema/count/`datetime.calendar` + HTML；独立 website 消费者 **unmet→partial met**）；inventory 仍 met；**仍 ≠** website met / 完整站；residual **4/4**；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-seventh` / `A-W377` · gate **47/47** · soft **0**
- **证据：** `website-capability-vectors.md` · `website_capabilities.rs` · `website_capability_deepen.rs` · a10 交叉 ADR-721 · `ga-evidence-audit.md` W377
- **Frozen：** website met / 完整站托管；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W378 → **已交 ADR-722**（website product met exit）；W379+

### Wave A W376 正确性地板加深（ADR-720 · Website same-source thin-open inventory · 已交）

- **主轴：** **Website same-source thin-open / inventory**（policy+meta；新建 `website-capability-vectors.md` + `website_capability_thin.rs`；索引权威源/gen/books/doctor/LSP 已有 vs 独立 website 消费者 **仍 unmet**；inventory signed ≠ website met）；residual 产品 **4/4** 保持；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** website met / **非** 建站 / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-sixth` / `A-W376` · gate **47/47** · soft **0**
- **证据：** `website-capability-vectors.md` · `website_capability_thin.rs` · a10 source/exit 交叉 ADR-720 · `ga-evidence-audit.md` W376
- **Frozen：** website met / 建站；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W377 → **已交 ADR-721**（website deepen）；W378+

### Wave A W375 正确性地板加深（ADR-719 · Residual product net.tcp.listener raise · 已交）

- **主轴：** Residual **product `net.tcp.listener` dual-backend raise**（`native+wasm=real` + `status=implemented`；复用 `qs1_tcp_listen_close_wasm` + `m2_s4_tcp_listen_close_native_llvm` + `ec_m2_3_tcp_listener_leak`；`residual-product-vectors.md` 产品 **4/4** + `residual_tcp_listener_thin.rs`）；TcpListen* HostFn status 仍 partial；deque+row+pool+listener 全 met；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**仍 ≠** accept GA / ≠ TLS；**非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-fifth` / `A-W375` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_tcp_listener_thin.rs` · `capabilities.toml` net.tcp.listener · qs1/llvm listen nails · `ga-evidence-audit.md` W375
- **Frozen：** accept GA / TLS 宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W376 → **已交 ADR-720**（website thin inventory）；W377+

### Wave A W374 正确性地板加深（ADR-718 · Residual TcpListen* HostFn wasm first-slice · 已交）

- **主轴：** Residual **TcpListen* HostFn wasm first-slice**（TcpListen/TcpListenerClose `wasm=real`；status 仍 partial；`qs1_tcp_listen_close_wasm`；`residual-product-vectors.md` HostFn listen **met** + `residual_tcplisten_thin.rs`）；产品 listener **仍 unmet**（产品仍 **3/4**）；SqlPool*/SqlRow* 仍 wasm=real；deque+row+pool 仍 met；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 产品 listener raise / **非** 四行清零 / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-fourth` / `A-W374` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_tcplisten_thin.rs` · `host_fn_capabilities.toml` TcpListen* · `qs1_tcp_listen_close_wasm` · `m2_s4_tcp_listen_close_native_llvm` · `ga-evidence-audit.md` W374
- **Frozen：** 产品 listener raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W375 → **已交 ADR-719**（product net.tcp.listener raise）；W376+

### Wave A W373 正确性地板加深（ADR-717 · Residual product data.sqlite.pool raise · 已交）

- **主轴：** Residual **product `data.sqlite.pool` dual-backend raise**（`native+wasm=real` + `status=implemented`；复用 `qs1_sql_pool_lifecycle_{native_llvm,wasm}` + m2_s8_*；`residual-product-vectors.md` 产品 **3/4** + `residual_sqlite_pool_thin.rs`）；listener **仍 unmet**；TcpListen* **仍** unsupported；SqlPool* HostFn 仍 wasm=real；deque+row 仍 met；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 四行清零 / **非** listener raise / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-third` / `A-W373` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_sqlite_pool_thin.rs` · `capabilities.toml` data.sqlite.pool · qs1 pool lifecycle · `ga-evidence-audit.md` W373
- **Frozen：** 产品 listener raise；TcpListen* 本窗 raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W374 → **已交 ADR-718**（TcpListen* HostFn wasm）；W375+

### Wave A W372 正确性地板加深（ADR-716 · Residual SqlPool* HostFn wasm first-slice · 已交）

- **主轴：** Residual **SqlPool* HostFn wasm first-slice**（Create/Acquire/Release/Close `wasm=real`；status 仍 partial；`qs1_sql_pool_lifecycle_wasm`；`residual-product-vectors.md` HostFn pool **met** + `residual_sqlpool_thin.rs`）；产品 pool/listener **仍 unmet**（产品仍 **2/4**）；TcpListen* **仍** unsupported；六 SqlRow* 仍 wasm=real；deque+row 仍 met；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 产品 pool raise / **非** 四行清零 / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-second` / `A-W372` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_sqlpool_thin.rs` · `host_fn_capabilities.toml` SqlPool* · `qs1_sql_pool_lifecycle_{native_llvm,wasm}` · `ga-evidence-audit.md` W372
- **Frozen：** 产品 pool/listener raise；TcpListen* 本窗 raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W373 → **已交 ADR-717**（product data.sqlite.pool raise）；W374+

### Wave A W371 正确性地板加深（ADR-715 · Residual product data.sqlite.row raise · 已交）

- **主轴：** Residual **product `data.sqlite.row` dual-backend raise**（`native+wasm=real` + `status=implemented`；复用 `qs1_sql_row_accessors_{native_llvm,wasm}` + GetInt 双钉 + `m2_s8_02b`；`residual-product-vectors.md` 产品 **2/4** + `residual_sqlite_row_thin.rs`）；pool/listener **仍 unmet**；SqlPool*/TcpListen* **仍** unsupported；deque 仍 met；六 SqlRow* HostFn 仍 wasm=real；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 四行清零 / **非** pool/listener raise / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twenty-first` / `A-W371` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_sqlite_row_thin.rs` · `capabilities.toml` data.sqlite.row · qs1 accessors/GetInt · `sql_typed_row.rs` · `ga-evidence-audit.md` W371
- **Frozen：** 产品 pool/listener raise；SqlPool*/TcpListen* 本窗 raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W372 → **已交 ADR-716**（SqlPool* HostFn wasm）；W373+

### Wave A W370 正确性地板加深（ADR-714 · Residual SqlRow* HostFn wasm batch-5 · 已交）

- **主轴：** Residual **SqlRow* HostFn wasm batch-5**（GetString/Float/Blob/IsNull/ColumnNames `wasm=unsupported→real`；status 仍 `partial`；新建 `qs1_sql_row_accessors_wasm` exact alice/1.5/2/true/false/4；六 SqlRow* HostFn wasm 全 real）；产品 row/pool/listener **仍 unmet**；SqlPool*/TcpListen* **仍** unsupported；deque 仍 met；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 四行清零 / **非** 产品 row raise / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twentieth` / `A-W370` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_sqlrow_accessors_thin.rs` · `qs1_sql_row_accessors_wasm` · `host_fn_capabilities.toml` 五者 · `ga-evidence-audit.md` W370
- **Frozen：** 产品 row/pool/listener raise；SqlPool*/TcpListen* 本窗 raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W371 → **已交 ADR-715**（product data.sqlite.row raise）；W372+

### Wave A W369 正确性地板加深（ADR-713 · Residual SqlRowGetInt HostFn wasm first-slice · 已交）

- **主轴：** Residual **SqlRowGetInt HostFn wasm first-slice**（`host_fn` `wasm=unsupported→real`；status 仍 `partial`；复用 `qs1_sql_prepare_bind_row_get_int_wasm`；`residual-product-vectors.md` HostFn 进度 + `residual_sqlrow_getint_thin.rs`）；产品 row/pool/listener **仍 unmet**；deque 仍 met；其余 SqlRow*/SqlPool*/TcpListen* **仍** wasm unsupported；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 四行清零 / **非** 产品 row raise / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-nineteenth` / `A-W369` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_sqlrow_getint_thin.rs` · `hostfn_sql.rs` GetInt nails+meta · `host_fn_capabilities.toml` SqlRowGetInt · `ga-evidence-audit.md` W369
- **Frozen：** 产品 row/pool/listener raise；其余 SqlRow*/SqlPool*/TcpListen* 本窗批 raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W370 → **已交 ADR-714**（SqlRow* HostFn wasm batch-5）；W371+

### Wave A W368 正确性地板加深（ADR-712 · Residual deque first-slice · 已交）

- **主轴：** Residual **deque first-slice**（`collections.deque` `native+wasm=real` + `status=implemented`；复用 `a04_deque_empty_pop_nil_{native,wasm}`；`residual-product-vectors.md` + `residual_deque_thin.rs`）；listener/pool/row **仍** partial + wasm unsupported；A-01～A-10 全 `[x]`；B-08/Q-L5 **仍 `[x]`**；**非** 四行清零 / **非** website met / **非** B-04/B-05/B-06 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-eighteenth` / `A-W368` · gate **47/47** · soft **0**
- **证据：** `residual-product-vectors.md` · `residual_deque_thin.rs` · `a04_deinit_nil.rs` deque nails · `capabilities.toml` collections.deque · `ga-evidence-audit.md` W368
- **Frozen：** listener/pool/row raise；四行清零宣称；website met；B-04/B-05/B-06 `[x]`；生产默认 JIT；Q-L5/B-08 回退
- **下一窗：** Wave A W369 → **已交 ADR-713**（SqlRowGetInt HostFn wasm）；W370+

### Wave A W367 正确性地板加深（ADR-711 · Q-L5 / B-08 JIT exit charter · 已交）

- **主轴：** Q-L5 / B-08 **JIT exit charter**（`ql5-jit-exit.md` + `jit_exit.rs`；scope narrow = thin+deepen×3+floor + Suite-1 opt-in；deopt partial 即足；生产默认/全量 MirOp/微基准 **仍 unmet deferred**）；B-08 **`[~]→[x]`**；Q-L5 **`[ ]→[x]`**（语言门槛 5/5）；A-01～A-10 全 `[x]`；**非** 生产默认 JIT / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventeenth` / `A-W367` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-exit.md` · `jit_exit.rs` · floor/vectors · thin/deepen/floor_thin 翻锁 · `runtime.jit.tier2` notes ADR-711 · `ga-evidence-audit.md` W367
- **Frozen：** 生产默认 JIT；扩 supported MirOp；微基准 GA；residual；website met；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W368 → **已交 ADR-712**（residual deque first-slice）；W369+

### Wave A W366 正确性地板加深（ADR-710 · Q-L5 / B-08 JIT floor charter · 已交）

- **主轴：** Q-L5 / B-08 **JIT floor charter**（`ql5-jit-floor.md` + `jit_floor_thin.rs`；索引 thin+deepen×3 + 七钉 deopt **partial met**；更多 MirOp/微基准/生产默认 **仍 unmet**）；B-08 **仍 `[~]`**；Q-L5 **仍 `[ ]`**（floor ≠ exit）；A-01～A-10 全 `[x]`；**非** B-08/`[x]` / **非** 生产默认 JIT / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixteenth` / `A-W366` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-floor.md` · `jit_floor_thin.rs` · `ql5-jit-vectors.md` · `jit_thin.rs` · `jit_deepen.rs` · `runtime.jit.tier2` notes ADR-710 · `ga-evidence-audit.md` W366
- **Frozen：** Q-L5 checked / B-08 `[x]` / exit；生产默认 JIT；扩 supported MirOp；微基准 GA；residual；website met；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W367 → **已交 ADR-711**（Q-L5 / B-08 JIT exit）；W368+

### Wave A W365 正确性地板加深（ADR-709 · Q-L5 / B-08 JIT deepen batch-3 · 已交）

- **主轴：** Q-L5 / B-08 **JIT deepen batch-3**（`jit_setbuild_deopts` + `jit_loadglobal_deopts`；Deopt **仍 partial met**（Print+CallHost+ConstStr+ArrayBuild+DictBuild+SetBuild+LoadGlobal）；更多 MirOp/微基准/生产默认 **仍 unmet**）；B-08 **仍 `[~]`**；Q-L5 **仍 `[ ]`**；A-01～A-10 全 `[x]`；**非** B-08/`[x]` / **非** 生产默认 JIT / **非** residual / **非** floor；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifteenth` / `A-W365` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-vectors.md` · `jit_deepen.rs` · `jit_suite1_differential.rs` · crate SetBuild/LoadGlobal 对称 · `runtime.jit.tier2` notes ADR-709 · `ga-evidence-audit.md` W365
- **Frozen：** Q-L5 checked / B-08 `[x]`；生产默认 JIT；扩 supported MirOp；微基准 GA；residual；website met；B-04/B-05/B-06 `[x]`；floor/exit
- **下一窗：** Wave A W366 → **已交 ADR-710**（Q-L5 / B-08 JIT floor）；W367+

### Wave A W364 正确性地板加深（ADR-708 · Q-L5 / B-08 JIT deepen batch-2 · 已交）

- **主轴：** Q-L5 / B-08 **JIT deepen batch-2**（`jit_arraybuild_deopts` + `jit_dictbuild_deopts`；Deopt **仍 partial met**（Print+CallHost+ConstStr+ArrayBuild+DictBuild）；更多 MirOp/微基准/生产默认 **仍 unmet**）；B-08 **仍 `[~]`**；Q-L5 **仍 `[ ]`**；A-01～A-10 全 `[x]`；**非** B-08/`[x]` / **非** 生产默认 JIT / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fourteenth` / `A-W364` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-vectors.md` · `jit_deepen.rs` · `jit_suite1_differential.rs` · crate Jump_flat 对称 · `runtime.jit.tier2` notes ADR-708 · `ga-evidence-audit.md` W364
- **Frozen：** Q-L5 checked / B-08 `[x]`；生产默认 JIT；扩 supported MirOp；微基准 GA；residual；website met；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W365 → **已交 ADR-709**（Q-L5 / B-08 JIT deepen batch-3）；W366+

### Wave A W363 正确性地板加深（ADR-707 · Q-L5 / B-08 JIT deepen batch-1 · 已交）

- **主轴：** Q-L5 / B-08 **JIT deepen batch-1**（`jit_callhost_deopts` + `jit_conststr_deopts`；Deopt **仍 partial met**（Print+CallHost+ConstStr）；更多 MirOp/微基准/生产默认 **仍 unmet**）；B-08 **仍 `[~]`**；Q-L5 **仍 `[ ]`**；A-01～A-10 全 `[x]`；**非** B-08/`[x]` / **非** 生产默认 JIT / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-thirteenth` / `A-W363` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-vectors.md` · `jit_deepen.rs` · `jit_suite1_differential.rs` · `runtime.jit.tier2` notes ADR-707 · `ga-evidence-audit.md` W363
- **Frozen：** Q-L5 checked / B-08 `[x]`；生产默认 JIT；扩 supported MirOp；residual native raise；website met；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W364 → **已交 ADR-708**（Q-L5 / B-08 JIT deepen batch-2）；W365+

### Wave A W362 正确性地板加深（ADR-706 · Q-L5 / B-08 JIT thin-open · 已交）

- **主轴：** Q-L5 / B-08 **JIT thin-open**（新建 `ql5-jit-vectors.md` + `jit_thin.rs`；索引 Cranelift tier-2 Suite-1 + opt-in 默认关；deopt **partial met**；更多 MirOp/微基准/生产默认 **unmet**）；B-08 **`[ ]→[~]`**；Q-L5 **仍 `[ ]`**；A-01～A-10 全 `[x]`；**非** B-08/`[x]` / **非** 生产默认 JIT / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-twelfth` / `A-W362` · gate **47/47** · soft **0**
- **证据：** `ql5-jit-vectors.md` · `jit_thin.rs` · `plan/topics/jit.md` · `runtime.jit.tier2` notes ADR-706 · `ga-evidence-audit.md` W362
- **Frozen：** Q-L5 checked / B-08 `[x]`；生产默认 JIT；residual native raise；website met；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W363 → **已交 ADR-707**（Q-L5 / B-08 JIT deepen batch-1）；W364+

### Wave A W361 正确性地板加深（ADR-705 · A-10 / M0-C05 exit · scope narrow · 已交）

- **主轴：** A-10 / M0-C05 **exit charter**（新建 `a10-capability-exit.md` + `capability_exit.rs`；M0-C05 scope narrow = 权威源+books/doctor+LSP 同源；website **defer unmet**）；A-10 **`[~]→[x]`**；A-01～A-10 全 `[x]`；**非** website met / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-eleventh` / `A-W361` · gate **47/47** · soft **0**
- **证据：** `a10-capability-exit.md` · `capability_exit.rs` · floor/source 交叉 · caps header ADR-705 · `ga-evidence-audit.md` W361
- **Frozen：** website met；residual native raise；Q-L5 `[x]`；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W362 → **已交 ADR-706**（Q-L5 / B-08 JIT thin-open）；W363+

### Wave A W360 正确性地板加深（ADR-704 · A-10 floor charter · 已交）

- **主轴：** A-10 **floor charter**（新建 `a10-capability-floor.md` + `capability_floor_thin.rs`；索引 schema/gen/books/doctor **met**；LSP **partial met**；网站 **unmet**；residual **Frozen**）；A-10 **仍 `[~]`**（floor ≠ exit）；**非** A-10 `[x]` / **非** M0-C05 complete / **非** residual / **非** website；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-tenth` / `A-W360` · gate **47/47** · soft **0**
- **证据：** `a10-capability-floor.md` · `capability_floor_thin.rs` · `a10-capability-source.md` · caps header ADR-704 · `ga-evidence-audit.md` W360
- **Frozen：** A-10 `[x]` / M0-C05 complete；website met；exit；residual native raise；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W361 → **已交 ADR-705**（A-10 / M0-C05 exit）；W362+

### Wave A W359 正确性地板加深（ADR-703 · A-10 LSP 同源 deepen · 已交）

- **主轴：** A-10 **LSP 同源 deepen**（`syx.capabilities` → `capabilities_for_lsp` → `run_capabilities_report()`；nail `lsp_a10_capabilities_same_source_count_and_lookup`；LSP **partial met**；网站 **仍 unmet**）；A-10 **仍 `[~]`**；**非** A-10 `[x]` / **非** M0-C05 complete / **非** floor/exit / **非** residual；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-ninth` / `A-W359` · gate **47/47** · soft **0**
- **证据：** `a10-capability-source.md` · `syx-lsp` nail · `capability_thin.rs` · caps header ADR-703 · `ga-evidence-audit.md` W359
- **Frozen：** A-10 `[x]` / M0-C05 complete；website met 宣称；floor/exit；residual native raise；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W360 → **已交 ADR-704**（A-10 floor charter）；W361+

### Wave A W358 正确性地板加深（ADR-702 · A-10 capability 同源 thin-open · 已交）

- **主轴：** A-10 **capability 同源 thin-open**（新建 `a10-capability-source.md` + `capability_thin.rs`；索引 capabilities.toml→gen→books/doctor/CI；LSP/网站 runtime **unmet**）；A-10 **`[ ]→[~]`**；**非** A-10 `[x]` / **非** M0-C05 complete / **非** residual / **非** LSP·网站生产接线本窗；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-eighth` / `A-W358` · gate **47/47** · soft **0**
- **证据：** `a10-capability-source.md` · `capability_thin.rs` · caps header ADR-702 · `ga-evidence-audit.md` W358
- **Frozen：** A-10 `[x]` / M0-C05 complete；LSP/website 生产接线；residual native raise；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W359 → **已交 ADR-703**（A-10 LSP deepen）；W360+

### Wave A W357 正确性地板加深（ADR-701 · A-08 / Q-S4 exit charter · 已交）

- **主轴：** A-08 / Q-S4 **exit charter**（新建 `a08-qs4-exit.md` + `a08_qs4_exit.rs`；roadmap A-08 **`[~]→[x]`**；Q-S4 **`[ ]→[x]`**）；**非** 全体 A-* `[x]`（A-10 仍开）/ **非** 新行为钉 / **非** leap-second / full tz GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-seventh` / `A-W357` · gate **47/47** · soft **0**
- **证据：** `a08-qs4-exit.md` · `a08_qs4_exit.rs` · `a08-qs4-floor.md` · datetime caps notes ADR-701 · `ga-evidence-audit.md` W357
- **Frozen：** 全体 A-* `[x]`；新 HostFn 钉；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W358 → **已交 ADR-702**（A-10 capability thin-open）；W359+

### Wave A W356 正确性地板加深（ADR-700 · A-08 / Q-S4 floor charter · 已交）

- **主轴：** A-08 / Q-S4 **floor charter**（新建 `a08-qs4-floor.md` + `a08_qs4_floor_thin.rs`；索引六钉 + thin 五 clusters；floor signed ≠ exit）；A-08 **仍 `[~]`**；Q-S4 **仍 `[ ]`**；**非** A-08/Q-S4 `[x]` / **非** 新行为钉 / **非** leap-second / full tz GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-sixth` / `A-W356` · gate **47/47** · soft **0**
- **证据：** `a08-qs4-floor.md` · `a08_qs4_floor_thin.rs` · `a08-datetime-vectors.md` · datetime caps notes ADR-700 · `ga-evidence-audit.md` W356
- **Frozen：** A-08/Q-S4 `[x]` this window；新 HostFn 钉；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W357 → **已交 ADR-701**（A-08/Q-S4 exit charter）；W358+

### Wave A W355 正确性地板加深（ADR-699 · A-08 DateTime deepen batch-3 · 已交）

- **主轴：** A-08 **DateTime deepen batch-3**（`a08_iana_europe_london_winter_offset` + `a08_iana_europe_london_summer_offset` exact 0/60；IANA deepen **仍 partial met** 累计 6 钉）；A-08 **仍 `[~]`**；Q-S4 **仍 `[ ]`**；**非** A-08/Q-S4 `[x]` / **非** floor/exit / **非** leap-second / full tz GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fifth` / `A-W355` · gate **47/47** · soft **0**
- **证据：** `a08_datetime_deepen.rs` · `a08-datetime-vectors.md` · datetime caps notes ADR-699 · `ga-evidence-audit.md` W355
- **Frozen：** A-08/Q-S4 `[x]` this window；floor/exit；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W356 → **已交 ADR-700**（A-08/Q-S4 floor charter）；W357+

### Wave A W354 正确性地板加深（ADR-698 · A-08 DateTime deepen batch-2 · 已交）

- **主轴：** A-08 **DateTime deepen batch-2**（`a08_iana_format_with_timezone_tokyo` + `a08_calendar_reject_2023_apr31` exact；IANA/历法 **仍 partial met** 累计 4 钉）；A-08 **仍 `[~]`**；Q-S4 **仍 `[ ]`**；**非** A-08/Q-S4 `[x]` / **非** floor/exit / **非** leap-second / full tz GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-fourth` / `A-W354` · gate **47/47** · soft **0**
- **证据：** `a08_datetime_deepen.rs` · `a08-datetime-vectors.md` · datetime caps notes ADR-698 · `ga-evidence-audit.md` W354
- **Frozen：** A-08/Q-S4 `[x]` this window；floor/exit；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W355 → **已交 ADR-699**（A-08 DateTime deepen batch-3）；W356+

### Wave A W353 正确性地板加深（ADR-697 · A-08 DateTime deepen batch-1 · 已交）

- **主轴：** A-08 **DateTime deepen batch-1**（`a08_iana_asia_tokyo_offset` + `a08_calendar_reject_2023_feb29` exact；IANA deepen + 历法加深 **partial met**）；A-08 **仍 `[~]`**；Q-S4 **仍 `[ ]`**；**非** A-08/Q-S4 `[x]` / **非** floor/exit / **非** leap-second / full tz GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-third` / `A-W353` · gate **47/47** · soft **0**
- **证据：** `a08_datetime_deepen.rs` · `a08-datetime-vectors.md` · datetime caps notes ADR-697 · `ga-evidence-audit.md` W353
- **Frozen：** A-08/Q-S4 `[x]` this window；floor/exit；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W354 → **已交 ADR-698**（A-08 DateTime deepen batch-2）；W355+

### Wave A W352 正确性地板加深（ADR-696 · A-08 DateTime thin-open · 已交）

- **主轴：** A-08 / Q-S4 **DateTime thin-open**（policy+meta；新建 `a08-datetime-vectors.md` + `a08_qs4_thin.rs`；索引既有 Instant/IANA/ISO/HostFn/package 钉；历法加深/全量 IANA/leap-second **unmet**）；A-08 **`[ ]→[~]`**；Q-S4 **仍 `[ ]`**；**非** A-08/Q-S4 `[x]` / **非** 新行为钉 / **非** residual native raise / **非** more RFC 本窗；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-second` / `A-W352` · gate **47/47** · soft **0**
- **证据：** `a08-datetime-vectors.md` · `a08_qs4_thin.rs` · datetime caps notes ADR-696 · `ga-evidence-audit.md` W352
- **Frozen：** A-08/Q-S4 `[x]` this window；新 HostFn 钉；leap-second / full tz GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W353 → **已交 ADR-697**（A-08 DateTime deepen batch-1）；W354+

### Wave A W351 正确性地板加深（ADR-695 · A-07 / Q-S3 exit charter · 已交）

- **主轴：** A-07 / Q-S3 **exit charter**（policy+meta；新建 `a07-qs3-exit.md` + `a07_qs3_exit.rs`；roadmap A-07 **`[~]→[x]`**；Q-S3 **`[ ]→[x]`**）；**非** 全体 A-* `[x]` / **非** 新行为钉 / **非** full RFC 表 GA / **非** residual native raise / **非** pbkdf2 status raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundred-first` / `A-W351` · gate **47/47** · soft **0**
- **证据：** `a07-qs3-exit.md` · `a07_qs3_exit.rs` · `a07-qs3-floor.md` · caps notes ADR-695 · `ga-evidence-audit.md` W351
- **Frozen：** 全体 A-* `[x]`；新 HostFn 钉；full RFC 表 GA；pbkdf2 experimental→implemented；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W352 → **已交 ADR-696**（A-08 DateTime thin-open）；W353+

### Wave A W350 正确性地板加深（ADR-694 · A-07 / Q-S3 floor charter · 已交）

- **主轴：** A-07 / Q-S3 **floor charter**（policy+meta；新建 `a07-qs3-floor.md` + `a07_qs3_floor_thin.rs`；索引六钉 clusters；floor signed ≠ exit）；A-07 **仍 `[~]`**；Q-S3 **仍 `[ ]`**；**非** A-07/Q-S3 `[x]` / **非** 新行为钉 / **非** full RFC 表 GA / **非** residual native raise / **非** pbkdf2 status raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-two-hundredth` / `A-W350` · gate **47/47** · soft **0**
- **证据：** `a07-qs3-floor.md` · `a07_qs3_floor_thin.rs` · `a07-crypto-vectors.md` · caps notes ADR-694 · `ga-evidence-audit.md` W350
- **Frozen：** A-07/Q-S3 `[x]` this window；新 HostFn 钉；full RFC 表 GA；pbkdf2 experimental→implemented；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W351 → **已交 ADR-695**（A-07/Q-S3 exit charter）；W352+

### Wave A W349 正确性地板加深（ADR-693 · A-07 RFC deepen batch-3 · 已交）

- **主轴：** A-07 **RFC known-answer + tamper deepen batch-3**（`a07_rfc_pbkdf2_c2_vector` + `a07_tamper_ed25519_sig_byte_flip`；`a07-crypto-vectors.md` RFC/tamper **仍 partial met**，累计 6 钉）；A-07 **仍 `[~]`**；Q-S3 **仍 `[ ]`**；**非** A-07/Q-S3 `[x]` / **非** floor charter / **非** full RFC 表 GA / **非** residual native raise / **非** pbkdf2 status raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-ninth` / `A-W349` · gate **47/47** · soft **0**
- **证据：** `a07_crypto_rfc.rs` · `a07-crypto-vectors.md` · `a07_qs3_thin.rs` · caps notes ADR-693 · `ga-evidence-audit.md` W349
- **Frozen：** A-07/Q-S3 `[x]`；floor charter；full RFC 表 GA；pbkdf2 experimental→implemented；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W350 → **已交 ADR-694**（A-07/Q-S3 floor charter）；W351+

### Wave A W348 正确性地板加深（ADR-692 · A-07 RFC deepen batch-2 · 已交）

- **主轴：** A-07 **RFC known-answer deepen batch-2**（`a07_rfc_sha512_empty_vector` + `a07_rfc_hmac_sha256_rfc4231_jefe`；`a07-crypto-vectors.md` RFC/tamper **仍 partial met**，累计 4 钉）；A-07 **仍 `[~]`**；Q-S3 **仍 `[ ]`**；**非** A-07/Q-S3 `[x]` / **非** full RFC 表 GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-eighth` / `A-W348` · gate **47/47** · soft **0**
- **证据：** `a07_crypto_rfc.rs` · `a07-crypto-vectors.md` · `a07_qs3_thin.rs` · caps notes ADR-692 · `ga-evidence-audit.md` W348
- **Frozen：** A-07/Q-S3 `[x]`；full RFC 表 GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W349 → **已交 ADR-693**（RFC deepen batch-3）；W350+

### Wave A W347 正确性地板加深（ADR-691 · A-07 RFC/tamper deepen · 已交）

- **主轴：** A-07 **RFC known-answer + tamper 行为钉**（`a07_rfc_sha256_empty_vector` + `a07_tamper_ed25519_verify_false`；`a07-crypto-vectors.md` RFC/tamper **partial met**）；A-07 **仍 `[~]`**；Q-S3 **仍 `[ ]`**；**非** A-07/Q-S3 `[x]` / **非** full RFC 表 GA / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-seventh` / `A-W347` · gate **47/47** · soft **0**
- **证据：** `a07_crypto_rfc.rs` · `a07-crypto-vectors.md` · `a07_qs3_thin.rs` · caps notes ADR-691 · `ga-evidence-audit.md` W347
- **Frozen：** A-07/Q-S3 `[x]`；full RFC 表 GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W348 → **已交 ADR-692**（RFC deepen batch-2）；W349+

### Wave A W346 正确性地板加深（ADR-690 · A-07 Crypto thin-open · 已交）

- **主轴：** A-07 **Crypto thin-open**（policy+meta；新建 `a07-crypto-vectors.md` + `a07_qs3_thin.rs`；索引既有 ed25519/pbkdf2/host/package 钉；RFC/tamper **unmet**）；A-07 **`[ ]→[~]`**；Q-S3 **仍 `[ ]`**；**非** A-07/Q-S3 `[x]` / **非** 新行为钉 / **非** residual native raise；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-sixth` / `A-W346` · gate **47/47** · soft **0**
- **证据：** `a07-crypto-vectors.md` · `a07_qs3_thin.rs` · caps notes ADR-690 · `ga-evidence-audit.md` W346
- **Frozen：** A-07/Q-S3 `[x]`；RFC/tamper GA；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W347 → **已交 ADR-691**（RFC/tamper deepen）；W348+

### Wave A W345 正确性地板加深（ADR-689 · A-06 / Q-S2 exit charter · 已交）

- **主轴：** A-06 / Q-S2 **exit charter**（policy+meta；新建 `a06-qs2-exit.md` + `a06_qs2_exit.rs`；`a06_exit_enabled=true` / `qs2_complete_enabled=true`；roadmap A-06 **`[~]→[x]`**；Q-S2 **`[ ]→[x]`**）；**非** 全体 A-* `[x]` / **非** 新行为钉；A-07/A-08/A-10 仍开；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-fifth` / `A-W345` · gate **47/47** · soft **0**
- **证据：** `a06-qs2-exit.md` · `a06_qs2_exit.rs` · `a06_exit_readiness.rs` · `ga-evidence-audit.md` W345
- **Frozen：** 全体 A-* `[x]`；新 wire/TLS 行为钉；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W346 → **已交 ADR-690**（A-07 Crypto thin-open）；W347+

### Wave A W344 正确性地板加深（ADR-688 · A-06 malformed/full-loopback floor · 已交）

- **主轴：** A-06 **malformed GA / full-loopback floor**（`m2_s4_a06_malformed_truncated_response_loopback` + `m2_s4_a06_malformed_bad_status_line_loopback` exact R003；新建 `a06-qs2-floor.md` + `a06_qs2_floor_thin.rs`；`relational_malformed_ga_ok`；exit readiness triad **3/3 floor met**）；A-06 **仍 `[~]`**；Q-S2 **仍 `[ ]`**；**非** A-06/Q-S2 `[x]`（exit = W345）；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-fourth` / `A-W344` · gate **47/47** · soft **0**
- **证据：** wire malformed nails · `a06-qs2-floor.md` · `a06_exit_readiness.rs` · `ga-evidence-audit.md` W344
- **Frozen：** A-06/Q-S2 `[x]` this window；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W345 → **已交 ADR-689**（A-06/Q-S2 exit charter）；W346+

### Wave A W343 正确性地板加深（ADR-687 · timeout_tls + tls.client native raise · 已交）

- **主轴：** A-06 residual **`timeout_tls` + `tls.client` native=partial→real**（复用 dual-backend + W342 native AOT 钉；扩展 `A06ExitReadinessInventory` `relational_native_raise_ok` + `native_raise_enabled=true`；malformed GA 仍 unmet）；A-06 **仍 `[~]`**；Q-S2 **仍 `[ ]`**；**非** A-06/Q-S2 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-third` / `A-W343` · gate **47/47** · soft **0**
- **证据：** `capabilities.toml` native raise · `http_native_wasm_matrix/ga` 翻锁 · `a06_exit_readiness.rs` · `a06-exit-readiness.md` · `ga-evidence-audit.md` W343
- **Frozen：** A-06/Q-S2 `[x]`；malformed GA / full loopback exit charter；B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W344+ → **已交 ADR-688**（malformed floor）；W345 A-06/Q-S2 exit charter

### Wave A W342 正确性地板加深（ADR-686 · A-06 native hang/HTTPS AOT 钉 · 已交）

- **主轴：** A-06 **native hang/HTTPS AOT 行为钉**（`m2_s4_02b_timeout_hang_loopback_native_aot` + `m2_s4_05_https_loopback_ok_with_extra_root_native_aot` + `m2_s4_05_self_signed_fail_closed_native_aot`；扩展 `A06ExitReadinessInventory` native nail preds；malformed GA 仍 unmet）；A-06 **仍 `[~]`**；Q-S2 **仍 `[ ]`**；**非** native raise / **非** A-06/Q-S2 `[x]`；Wave A / Wave B 退出条件已满足仍在；矩阵指纹 `qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-second` / `A-W342` · gate **47/47** · soft **0**
- **证据：** native AOT nails · `a06_exit_readiness.rs` W342 preds · `a06-exit-readiness.md` · `a06-network-negatives.md` · `capabilities.toml` tests+notes · `ga-evidence-audit.md` W342
- **Frozen：** A-06/Q-S2 `[x]`；timeout_tls/tls.client native raise（本窗）；malformed GA exit charter
- **下一窗：** Wave A W343+ → **已交 ADR-687**（native raise）；W344+ A-06/Q-S2 exit charter

### Wave A W341 正确性地板加深（ADR-685 · A-06 退出就绪 inventory deepen · 已交）

- **主轴：** A-06 **退出就绪 inventory**（公开 `A06ExitReadinessInventory` / `a06_exit_readiness_inventory`；新建 `a06-exit-readiness.md` + `a06_exit_readiness_thin.rs`；退出就绪三条件文档化：thin met；native hang/HTTPS unmet；malformed GA unmet）；A-06 **仍 `[~]`**；Q-S2 **仍 `[ ]`**；**非** A-06/Q-S2 `[x]` / **非** native raise；Wave A / Wave B 退出条件已满足仍在；B-04/B-05/B-06 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninety-first` / `A-W341`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `A06ExitReadinessInventory` · `a06_exit_readiness_inventory` · syx-rt a06_exit_readiness tests · `a06-exit-readiness.md` · `a06_exit_readiness_thin.rs` · `a06-network-negatives.md` · `ga-evidence-audit.md` W341
- **诚实：** ≠ A-06/Q-S2 `[x]` · ≠ timeout_tls/tls.client native raise · ≠ hang/HTTPS nails this window · ≠ B-04/B-05/B-06 `[x]`
- **下一窗：** Wave A W342+（native hang/HTTPS AOT nails → A-06 exit） → ADR-686+

### Wave A W340 正确性地板加深（ADR-684 · Wave B 结构退出章程 · 已交）

- **主轴：** Wave B **结构退出章程**（新建 `wave-b-exit.md` + `wave_b_exit_thin.rs`；退出三条件 3/3 诚实延后已齐 → 进度宣称 **「Wave B 退出条件已满足」**）；B-04/B-05/B-06 **均仍 `[~]`**（defer ≠ axis `[x]`）；readiness inventory 旗标仍 `wave_b_exit_enabled=false`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninetieth` / `B-W340`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `wave-b-exit.md` · `wave_b_exit_thin.rs` · `wave-b-exit-readiness.md` · triad defer 三臂 · `mir-aot.md` · `ga-evidence-audit.md` W340
- **诚实：** Wave B exit met 已允许 · ≠ B-04/B-05/B-06 `[x]` · ≠ 生产多根 / 真 CI · ≠ 真 codec / Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise · readiness 旗标未翻 true
- **下一窗：** Wave A W342+ → **已交 ADR-686**（本窗 native AOT）；W343+ native raise / exit charter

### Wave A W339 正确性地板加深（ADR-683 · B-05 Medium 诚实延后签收 · 已交）

- **主轴：** B-05 **Medium 诚实延后签收**（公开 `MirModuleMediumHorizonHonestDeferInventory` / `mir_module_medium_horizon_honest_defer_inventory`；新建 `wave-b-exit-triad-b05-defer.md` + `b05_medium_honest_defer_thin.rs`；退出三条件第 2 行诚实延后签收 met → triad **3/3** defer met）；B-04/B-05/B-06 **均仍 `[~]`**；**非** 真 codec / 双 blob 生产；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-ninth` / `B-W339`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleMediumHorizonHonestDeferInventory` · `mir_module_medium_horizon_honest_defer_inventory` · syx-mir medium_horizon_honest_defer tests · `wave-b-exit-triad-b05-defer.md` · `b05_medium_honest_defer_thin.rs` · `wave-b-exit-readiness.md` · `mir-aot.md` · `ga-evidence-audit.md` W339
- **诚实：** ≠ 真 codec / 双 blob 生产 · ≠ B-04/B-05/B-06 `[x]` · ≠ 生产多根 / 真 CI · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W340+ → **已交 ADR-684**（本窗）；原 W339 Next 关闭

### Wave A W338 正确性地板加深（ADR-682 · B-04 Short 诚实延后签收 · 已交）

- **主轴：** B-04 **Short 诚实延后签收**（公开 `AotShortHorizonHonestDeferInventory` / `aot_short_horizon_honest_defer_inventory`；新建 `wave-b-exit-triad-b04-defer.md` + `b04_short_honest_defer_thin.rs`；退出三条件第 1 行诚实延后签收 met；B-05 triad 仍未 met；B-06 defer 仍 met）；B-04/B-05/B-06 **均仍 `[~]`**；**非** Wave B exit met / **非**「Wave B 退出条件已满足」；**非** 生产多根 / 真 CI；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-eighth` / `B-W338`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotShortHorizonHonestDeferInventory` · `aot_short_horizon_honest_defer_inventory` · syx-mir short_horizon_honest_defer tests · `wave-b-exit-triad-b04-defer.md` · `b04_short_honest_defer_thin.rs` · `wave-b-exit-readiness.md` · `mir-aot.md` · `ga-evidence-audit.md` W338
- **诚实：** ≠ Wave B exit met · ≠ 「Wave B 退出条件已满足」 · ≠ 生产多根 / 真 CI · ≠ B-04/B-05/B-06 `[x]` · ≠ 真 codec · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W339+ → **已交 ADR-683**（本窗）；原 W338 Next 关闭

### Wave A W337 正确性地板加深（ADR-681 · B-06 Missing 清零诚实延后签收 · 已交）

- **主轴：** B-06 **Missing 清零诚实延后签收**（公开 `LlvmStubMissingClearHonestDeferInventory` / `llvm_stub_missing_clear_honest_defer_inventory`；新建 `wave-b-exit-triad-b06-defer.md` + `b06_missing_clear_honest_defer_thin.rs`；退出三条件第 3 行诚实延后签收 met；B-04/B-05 triad 仍未 met）；B-04/B-05/B-06 **均仍 `[~]`**；**非** Wave B exit met / **非**「Wave B 退出条件已满足」；**非** Missing 清零完成；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-seventh` / `B-W337`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `LlvmStubMissingClearHonestDeferInventory` · `llvm_stub_missing_clear_honest_defer_inventory` · syx-rt missing_clear_honest_defer tests · `wave-b-exit-triad-b06-defer.md` · `b06_missing_clear_honest_defer_thin.rs` · `wave-b-exit-readiness.md` · `mir-aot.md` · `ga-evidence-audit.md` W337
- **诚实：** ≠ Wave B exit met · ≠ 「Wave B 退出条件已满足」 · ≠ Missing 清零完成 · ≠ B-04/B-05/B-06 `[x]` · ≠ 生产多根 / 真 CI · ≠ 真 codec · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W338+ → **已交 ADR-682**（本窗）；原 W337 Next 关闭

### Wave A W336 正确性地板加深（ADR-680 · Wave B 结构退出就绪 inventory deepen · 已交）

- **主轴：** Wave B **结构退出就绪 inventory deepen**（公开 `WaveBStructuralExitReadinessInventory` / `wave_b_structural_exit_readiness_inventory`；新建 `wave-b-exit-readiness.md` + `wave_b_exit_readiness_thin.rs`；退出三条件文档化且均未 met）；B-04/B-05/B-06 **均仍 `[~]`**；**非** Wave B exit met / **非**「Wave B 退出条件已满足」；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-sixth` / `B-W336`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `WaveBStructuralExitReadinessInventory` · `wave_b_structural_exit_readiness_inventory` · syx-rt structural_exit_readiness tests · `wave-b-exit-readiness.md` · `wave_b_exit_readiness_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W336
- **诚实：** ≠ Wave B exit met · ≠ 「Wave B 退出条件已满足」 · ≠ B-04/B-05/B-06 `[x]` · ≠ 生产多根 / 真 CI · ≠ 真 codec / Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W337+ → **已交 ADR-681**（本窗）；原 W336 Next 关闭

### Wave A W335 正确性地板加深（ADR-679 · Wave B 三轴 horizon 结构就绪 rollup inventory deepen · 已交）

- **主轴：** Wave B **三轴 horizon 结构就绪 rollup inventory deepen**（公开 `WaveBTriAxisHorizonReadinessInventory` / `wave_b_tri_axis_horizon_readiness_inventory`；新建 `wave-b-horizon-readiness.md` + `wave_b_horizon_thin.rs`）；B-04/B-05/B-06 **均仍 `[~]`**；**非** Wave B exit；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-fifth` / `B-W335`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `WaveBTriAxisHorizonReadinessInventory` · `wave_b_tri_axis_horizon_readiness_inventory` · syx-rt wave_b_tri_axis tests · `wave-b-horizon-readiness.md` · `wave_b_horizon_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W335
- **诚实：** ≠ Wave B exit · ≠ B-04/B-05/B-06 `[x]` · ≠ 生产多根 / 真 CI · ≠ 真 codec / 双 blob 生产 · ≠ Missing 清零 · ≠ stub rewrite · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W336+ → **已交 ADR-680**（本窗）；原 W335 Next 关闭

### Wave A W334 正确性地板加深（ADR-678 · Wave B B-06 Missing clear×align horizon readiness inventory deepen · 已交）

- **主轴：** Wave B **B-06 Missing clear×align horizon readiness inventory deepen**（公开 `LlvmStubClearAlignHorizonReadinessInventory` / `llvm_stub_clear_align_horizon_readiness_inventory`；加深 `b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 **仍 `[~]`**；B-04/B-05 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-fourth` / `B-W334`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `LlvmStubClearAlignHorizonReadinessInventory` · `llvm_stub_clear_align_horizon_readiness_inventory` · syx-rt clear_align_horizon tests · `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W334
- **诚实：** ≠ B-06 `[x]` · ≠ Missing 清零 · ≠ stub rewrite · ≠ 绝对 align 矩阵 exit · ≠ B-07 · ≠ 真 codec · ≠ B-04 生产多根 / 真 CI · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W335+ → **已交 ADR-679**（本窗）；原 W334 Next 关闭

### Wave A W333 正确性地板加深（ADR-677 · Wave B B-05 Medium horizon cutover readiness / codec×dual-blob rollup inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium horizon cutover readiness / codec×dual-blob rollup inventory deepen**（公开 `MirModuleMediumHorizonCutoverReadinessInventory` / `mir_module_medium_horizon_cutover_readiness_inventory`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-third` / `B-W333`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleMediumHorizonCutoverReadinessInventory` · `mir_module_medium_horizon_cutover_readiness_inventory` · syx-mir medium_horizon_cutover tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W333
- **诚实：** ≠ B-05 `[x]` · ≠ 真 codec · ≠ 双 blob 生产 · ≠ encode/load/AOT 变更 · ≠ B-04 生产多根 / 真 CI · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W334+ → **已交 ADR-678**（本窗）；原 W333 Next 关闭

### Wave A W332 正确性地板加深（ADR-676 · Wave B B-04 Short horizon cutover readiness / export×size-CI rollup inventory deepen · 已交）

- **主轴：** Wave B **B-04 Short horizon cutover readiness / export×size-CI rollup inventory deepen**（公开 `AotShortHorizonCutoverReadinessInventory` / `aot_short_horizon_cutover_readiness_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-second` / `B-W332`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotShortHorizonCutoverReadinessInventory` · `aot_short_horizon_cutover_readiness_inventory` · syx-mir short_horizon_cutover tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W332
- **诚实：** ≠ B-04 `[x]` · ≠ 生产多根切流 · ≠ 真 size golden CI · ≠ 绝对阈值 · ≠ encode/compact 变更 · ≠ 真 codec · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W333+ → **已交 ADR-677**（本窗）；原 W332 Next 关闭

### Wave A W331 正确性地板加深（ADR-675 · Wave B B-06 Missing residual / clear-readiness inventory deepen · 已交）

- **主轴：** Wave B **B-06 Missing residual / clear-readiness inventory deepen**（公开 `LlvmStubMissingClearReadinessInventory` / `llvm_stub_missing_clear_readiness_inventory`；加深 `b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 **仍 `[~]`**；B-04/B-05 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighty-first` / `B-W331`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `LlvmStubMissingClearReadinessInventory` · `llvm_stub_missing_clear_readiness_inventory` · syx-rt missing_clear tests · `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `ga-evidence-audit.md` W331
- **诚实：** ≠ B-06 `[x]` · ≠ Missing 清零 · ≠ stub rewrite · ≠ B-07 · ≠ 真 codec · ≠ B-04 生产多根 / 真 CI · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W332+ → **已交 ADR-676**（本窗）；原 W331 Next 关闭

### Wave A W330 正确性地板加深（ADR-674 · Wave B B-04 size golden CI cutover readiness / CI-gate alignment inventory deepen · 已交）

- **主轴：** Wave B **B-04 size golden CI cutover readiness / CI-gate alignment inventory deepen**（公开 `AotBlobSizeGoldenCiCutoverReadinessInventory` / `aot_blob_size_golden_ci_cutover_readiness_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eightieth` / `B-W330`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotBlobSizeGoldenCiCutoverReadinessInventory` · `aot_blob_size_golden_ci_cutover_readiness_inventory` · syx-mir golden_ci_cutover tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W330
- **诚实：** ≠ B-04 `[x]` · ≠ 真 size golden CI · ≠ 绝对阈值 · ≠ 生产多根切流 · ≠ encode/compact 变更 · ≠ 真 codec · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W331+ → **已交 ADR-675**（本窗）；原 W330 Next 关闭

### Wave A W329 正确性地板加深（ADR-673 · Wave B B-05 Medium dual-blob cutover readiness / dual-load-contract inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob cutover readiness / dual-load-contract inventory deepen**（公开 `MirModuleDualBlobCutoverReadinessInventory` / `mir_module_dual_blob_cutover_readiness_inventory`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-ninth` / `B-W329`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobCutoverReadinessInventory` · `mir_module_dual_blob_cutover_readiness_inventory` · syx-mir dual_blob_cutover tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W329
- **诚实：** ≠ B-05 `[x]` · ≠ 真 codec · ≠ AOT sidecar 嵌入 · ≠ 双 blob 生产 load · ≠ encode/load/AOT 变更 · ≠ B-04 生产多根 / size golden CI · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W330+ → **已交 ADR-674**（本窗）；原 W329 Next 关闭

### Wave A W328 正确性地板加深（ADR-672 · Wave B B-04 export-root cutover readiness / emit-compact alignment inventory deepen · 已交）

- **主轴：** Wave B **B-04 export-root cutover readiness / emit-compact alignment inventory deepen**（公开 `AotReachabilityExportRootCutoverReadinessInventory` / `aot_reachability_export_root_cutover_readiness_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-eighth` / `B-W328`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotReachabilityExportRootCutoverReadinessInventory` · `aot_reachability_export_root_cutover_readiness_inventory` · syx-mir cutover_readiness tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W328
- **诚实：** ≠ B-04 `[x]` · ≠ 生产多根切流 · ≠ size golden CI · ≠ 真 codec · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W329+ → **已交 ADR-673**（本窗）；原 W328 Next 关闭

### Wave A W327 正确性地板加深（ADR-671 · Wave B B-05 Medium dual-blob AOT-path / embed-contract inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob AOT-path / embed-contract inventory deepen**（公开 `MirModuleDualBlobAotEmbedInventory` / `mir_module_dual_blob_aot_embed_inventory`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-seventh` / `B-W327`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobAotEmbedInventory` · `mir_module_dual_blob_aot_embed_inventory` · syx-mir aot_embed tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W327
- **诚实：** ≠ B-05 `[x]` · ≠ 真 codec · ≠ AOT sidecar 嵌入 · ≠ encode/load/AOT 变更 · ≠ B-04 size golden CI · ≠ 生产多根 · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W328+ → **已交 ADR-672**（本窗）；原 W327 Next 关闭

### Wave A W326 正确性地板加深（ADR-670 · Wave B B-04 export-roots-beyond-main reachability inventory deepen · 已交）

- **主轴：** Wave B **B-04 export-roots-beyond-main reachability inventory deepen**（公开 `AotReachabilityExportRootInventory` / `aot_reachability_export_root_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-sixth` / `B-W326`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotReachabilityExportRootInventory` · `aot_reachability_export_root_inventory` · `aot_reachability_fixture_main_and_export_orphan` · syx-mir export_root tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W326
- **诚实：** ≠ B-04 `[x]` · ≠ 生产多根切流 · ≠ size golden CI · ≠ 真 codec · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W327+ → **已交 ADR-671**（本窗）；原 W326 Next 关闭

### Wave A W325 正确性地板加深（ADR-669 · Wave B B-05 Medium dual-blob codec readiness / load-contract inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob codec readiness / load-contract inventory deepen**（公开 `MirModuleDualBlobCodecReadinessInventory` / `mir_module_dual_blob_codec_readiness_inventory`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-fifth` / `B-W325`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobCodecReadinessInventory` · `mir_module_dual_blob_codec_readiness_inventory` · syx-mir codec_readiness tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W325
- **诚实：** ≠ B-05 `[x]` · ≠ 真 codec · ≠ 双 blob 生产 · ≠ encode/load/AOT 变更 · ≠ B-04 size golden CI · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W326+ → **已交 ADR-670**（本窗）；原 W325 Next 关闭

### Wave A W324 正确性地板加深（ADR-668 · Wave B B-04 size golden candidate / relational CI-hook inventory deepen · 已交）

- **主轴：** Wave B **B-04 size golden candidate / relational CI-hook inventory deepen**（公开 `AotBlobSizeGoldenCandidateInventory` / `aot_blob_size_golden_candidate_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-fourth` / `B-W324`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `AotBlobSizeGoldenCandidateInventory` · `aot_blob_size_golden_candidate_inventory` · syx-mir golden_candidate tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W324
- **诚实：** ≠ B-04 `[x]` · ≠ size golden CI 门控 · ≠ 绝对阈值 · ≠ production encode 变更 · ≠ 真 codec · ≠ B-06 Missing 清零 · ≠ A-06 `[x]` · ≠ timeout_tls/tls.client native raise
- **下一窗：** Wave A W325+ → **已交 ADR-669**（本窗）；原 W324 Next 关闭

### Wave A W323 正确性地板加深（ADR-667 · Wave B B-05 Medium dual-blob decode-design / sidecar round-trip probe inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob decode-design / sidecar round-trip probe inventory deepen**（公开 `MirModuleDualBlobDecodeProbe` / `mir_module_dual_blob_decode_probe`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-third` / `B-W323`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobDecodeProbe` · `mir_module_dual_blob_decode_probe` · syx-mir dual_blob_decode_probe tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W323
- **非目标：** B-05 `[x]` · 真 codec · 双 blob 生产 · encode/load/AOT 变更 · B-04 size golden CI · B-06 Missing 清零 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W324+（B-05 real codec 评估 / B-04 size golden CI 评估 / B-06 Missing 清零评估 / A-06 exit / 残余 native=partial）

### Wave A W322 正确性地板加深（ADR-666 · Wave B B-05 Medium dual-blob encode-design / sidecar probe inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob encode-design / sidecar probe inventory deepen**（公开 `MirModuleMetadataSidecarView` / `MirModuleDualBlobEncodeProbe` / `mir_module_dual_blob_encode_probe`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-second` / `B-W322`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobEncodeProbe` · `mir_module_dual_blob_encode_probe` · `MirModuleMetadataSidecarView` · syx-mir dual_blob_encode_probe tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W322
- **非目标：** B-05 `[x]` · 真 codec · 双 blob 生产 · encode/load/AOT 变更 · B-04 size golden CI · B-06 Missing 清零 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W324+（B-05 real codec 评估 / B-04 size golden CI 评估 / B-06 Missing 清零评估 / A-06 exit / 残余 native=partial） → **已交 ADR-668**

### Wave A W321 正确性地板加深（ADR-665 · Wave B B-05 Medium dual-blob shape inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium dual-blob shape inventory deepen**（公开 `MirModuleDualBlobShapeInventory` / `mir_module_dual_blob_shape_inventory`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventy-first` / `B-W321`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleDualBlobShapeInventory` · `mir_module_dual_blob_shape_inventory` · syx-mir dual_blob_shape tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W321
- **非目标：** B-05 `[x]` · 真 codec · 双 blob 生产 · encode 格式变更 · B-04 size golden CI · B-06 Missing 清零 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W322+（B-05 real codec 评估 / B-04 size golden CI 评估 / B-06 Missing 清零评估 / A-06 exit / 残余 native=partial） → **已交 ADR-666**

### Wave A W320 正确性地板加深（ADR-664 · Wave B B-04 layered AOT blob size baseline inventory deepen · 已交）

- **主轴：** Wave B **B-04 layered AOT blob size baseline inventory deepen**（公开 `encode_module_compact_only_for_inventory` / `AotBlobSizeBaselineInventory` / `aot_blob_size_baseline_inventory`；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventieth` / `B-W320`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `encode_module_compact_only_for_inventory` · `aot_blob_size_baseline_inventory` · syx-mir baseline tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W320
- **非目标：** B-04 `[x]` · size golden CI · 绝对字节阈值 · 生产 encode 语义变更 · 真 codec · B-06 Missing 清零 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W321+（B-05 real codec 评估 / B-04 size golden CI 评估 / B-06 Missing 清零评估 / A-06 exit / 残余 native=partial） → **已交 ADR-665**

### Wave A W319 正确性地板加深（ADR-663 · Wave B B-05 Medium metadata-table inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium metadata-table inventory deepen**（公开 `MirModuleMetadataTableInventory` / `mir_module_metadata_table_inventory` / `aot_codec_fixture_main_with_const_strs`；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-ninth` / `B-W319`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `MirModuleMetadataTableInventory` · `mir_module_metadata_table_inventory` · `aot_codec_fixture_main_with_const_strs` · syx-mir table inventory tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W319
- **非目标：** B-05 `[x]` · 真 codec · 双 blob · encode 格式变更 · B-06 Missing 清零 · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W320+（B-05 real codec 首切片评估 / B-04 size golden 基线 inventory / B-06 Missing 清零评估 / A-06 exit / 残余 native=partial） → **已交 ADR-664**

### Wave A W318 正确性地板加深（ADR-662 · Wave B B-05 production native-instruction-omit wire deepen · 已交）

- **主轴：** Wave B **B-05 production native-instruction-omit wire deepen**（公开 `module_with_native_instructions_omitted` + `encode_module_compact_for_aot` omit+strip wire + syx-llvm `aot_compact_view` blob；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-eighth` / `B-W318`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `module_with_native_instructions_omitted` · `encode_module_compact_for_aot` · `aot_compact_view` · syx-mir/llvm omit wire tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W318
- **非目标：** B-05 `[x]` · 真 codec · 通用 `encode_module` omit · B-06 Missing 清零 · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W319+（B-05 real codec 评估 / B-06 Missing 清零评估 / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-663**

### Wave A W317 正确性地板加深（ADR-661 · Wave B B-05 Medium native-instruction-omit inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium native-instruction-omit inventory deepen**（公开 `MirModuleNativeOmitInventory` / `mir_module_native_instruction_omit_inventory` + large-eligible/orphan/CallIndirect + encode-unchanged 单测；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-seventh` / `B-W317`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_module_native_instruction_omit_inventory` · `MirModuleNativeOmitInventory` · syx-mir omit tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W317
- **非目标：** B-05 `[x]` · 真 codec · 生产 omit wire · B-06 Missing 清零 · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W318+（B-05 omit wire / real codec 评估 / B-06 Missing 清零评估 / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-662**

### Wave A W316 正确性地板加深（ADR-660 · Wave B B-05 production debug-strip wire deepen · 已交）

- **主轴：** Wave B **B-05 production debug-strip wire deepen**（公开 `module_with_debug_locals_stripped` + `encode_module_compact_for_aot` strip wire + syx-llvm `aot_compact_view` blob；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-sixth` / `B-W316`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `module_with_debug_locals_stripped` · `encode_module_compact_for_aot` · `aot_compact_view` · syx-mir/llvm strip wire tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W316
- **非目标：** B-05 `[x]` · 真 codec · 指令 omit · 通用 `encode_module` strip · B-06 Missing 清零 · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W317+（B-05 real codec / B-06 Missing 清零评估 / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-661**

### Wave A W315 正确性地板加深（ADR-659 · Wave B B-06 Missing/emit-declare 差集 inventory deepen · 已交）

- **主轴：** Wave B **B-06 declare-diff inventory deepen**（公开 `LlvmStubDeclareDiffInventory` / `llvm_stub_declare_diff_inventory` + defined↔audit / declared/missing/extra/`missing_not_in_rt==0` 单测；加深 `b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 **仍 `[~]`**；B-04/B-05 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-fifth` / `B-W315`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `llvm_stub_declare_diff_inventory` · `LlvmStubDeclareDiffInventory` · syx-rt declare-diff tests · `llvm_stub_audit.rs` · `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W315
- **非目标：** B-06 `[x]` · stub rewrite · B-07 · Missing 清零 · unlinkable 误读 · B-05 真 codec · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W316+（B-05 real codec / B-06 Missing 清零评估 / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-660**

### Wave A W314 正确性地板加深（ADR-658 · Wave B B-06 audit↔capability align inventory deepen · 已交）

- **主轴：** Wave B **B-06 align inventory deepen**（公开 `LlvmStubCapabilityAlignInventory` / `llvm_stub_capability_align_inventory` + stub↔report/hostfn_llvm/named_aot/call_host/HostFnId 交叉单测；加深 `b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 **仍 `[~]`**；B-04/B-05 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-fourth` / `B-W314`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `llvm_stub_capability_align_inventory` · `LlvmStubCapabilityAlignInventory` · syx-rt align tests · `llvm_stub_audit.rs` · `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W314
- **非目标：** B-06 `[x]` · stub rewrite · B-07 · 绝对 HostFn↔stub 矩阵 exit · B-05 真 codec · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W315+（B-06 Missing/emit-declare 差集首切片 / B-05 real codec / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-659**

### Wave A W313 正确性地板加深（ADR-657 · Wave B B-06 audit report deepen · 已交）

- **主轴：** Wave B **B-06 audit report deepen**（公开 `LlvmStubAuditReport` / `llvm_stub_audit_report` / `classify_llvm_stub_block` + 桶和/关系地板/POP≠Wired 单测；加深 `b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 **仍 `[~]`**；B-04/B-05 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-third` / `B-W313`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `llvm_stub_audit_report` · `LlvmStubAuditReport` · syx-rt audit tests · `llvm_stub_audit.rs` · `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W313
- **非目标：** B-06 `[x]` · stub rewrite · B-07 · B-05 真 codec · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W314+（B-06 audit↔capability 对齐首切片 / B-05 real codec / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-658**

### Wave A W312 正确性地板加深（ADR-656 · Wave B B-05 Medium debug-metadata strip inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium debug-metadata strip inventory deepen**（公开 `aot_codec_fixture_main_with_debug_locals` + `MirModuleDebugStripInventory` / `mir_module_debug_strip_inventory` + 单调/关系/encode-unchanged 单测；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-second` / `B-W312`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `aot_codec_fixture_main_with_debug_locals` · `mir_module_debug_strip_inventory` · `MirModuleDebugStripInventory` · syx-mir strip tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W312
- **非目标：** B-05 `[x]` · 真 codec 拆分 · 生产 strip · 指令流 omit · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W313+（B-05 real codec首切片 / B-06 deepen / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-657**

### Wave A W311 正确性地板加深（ADR-655 · Wave B B-05 Medium codec partition inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium codec partition inventory deepen**（公开 `MirModuleCodecPartitionInventory` / `mir_module_codec_partition_inventory` + 单调/关系/encode-unchanged 单测；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixty-first` / `B-W311`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_module_codec_partition_inventory` · `MirModuleCodecPartitionInventory` · syx-mir partition tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W311
- **非目标：** B-05 `[x]` · 真 codec 拆分 · 指令流 omit · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W312+（B-05 real codec首切片 / B-06 deepen / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-656**

### Wave A W310 正确性地板加深（ADR-654 · Wave B B-05 Medium layout inventory deepen · 已交）

- **主轴：** Wave B **B-05 Medium layout inventory deepen**（公开 `MirModuleLayoutInventory` / `mir_module_layout_inventory` + 单调/关系单测；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixtieth` / `B-W310`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_module_layout_inventory` · `MirModuleLayoutInventory` · syx-mir layout tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W310
- **非目标：** B-05 `[x]` · codec 拆分 · 指令流 omit · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W311+（B-05 codec首切片 / B-06 deepen / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-655**

### Wave A W309 正确性地板加深（ADR-653 · Wave B B-05 encode/load honesty deepen · 已交）

- **主轴：** Wave B **B-05 encode/load honesty deepen**（公开 `MirModuleBlobStats` / `mir_module_blob_stats` + round-trip；加深 `b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 **仍 `[~]`**；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-ninth` / `B-W309`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_module_blob_stats` · `MirModuleBlobStats` · syx-mir round-trip tests · `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W309
- **非目标：** B-05 `[x]` · codec 拆分 · 指令流 omit · B-04 size golden CI · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W310+（B-05 codec起步 / B-04 size golden CI / A-06 exit / 残余 native=partial） → **已交 ADR-654**

### Wave A W308 正确性地板加深（ADR-652 · Wave B B-04 size fixture inventory deepen · 已交）

- **主轴：** Wave B **B-04 size fixture inventory deepen**（公开 `aot_size_fixture_*` + 单调/关系单测；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-eighth` / `B-W308`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `aot_size_fixture_main_with_orphan_body` · `aot_size_fixture_call_indirect_full_emit` · syx-mir fixture tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W308
- **非目标：** B-04 `[x]` · 体积 golden CI · 绝对字节阈值表 · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W309+（B-04 size golden CI / exit评估 / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-653**

### Wave A W307 正确性地板加深（ADR-651 · Wave B B-04 size inventory floor deepen · 已交）

- **主轴：** Wave B **B-04 size inventory floor deepen**（公开 `AotBlobSizeReport` / `aot_blob_size_report` + syx-mir 合成可测；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-seventh` / `B-W307`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `aot_blob_size_report` · `AotBlobSizeReport` · syx-mir size tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W307
- **非目标：** B-04 `[x]` · 体积 golden CI · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W308+（B-04 size golden CI / exit评估 / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-652**

### Wave A W306 正确性地板加深（ADR-650 · Wave B B-04 AOT compact wire deepen · 已交）

- **主轴：** Wave B **B-04 AOT compact wire deepen**（生产 compact encode + register/main/emit 索引对齐 + syx-llvm 单测；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-sixth` / `B-W306`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `aot_compact_view` · append_aot_runtime_shim / compile_native_c_stub · emit fn_name_to_index · syx-llvm tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W306
- **非目标：** B-04 `[x]` · 体积 golden CI · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W307+（B-04 size golden deepen / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-651**

### Wave A W305 正确性地板加深（ADR-649 · Wave B B-04 fn-delete + index remap analysis deepen · 已交）

- **主轴：** Wave B **B-04 fn-delete + index remap analysis deepen**（`compact_module_for_aot_blob` + `build_fn_index_remap` + `remap_fn_indices_in_module` + inventory encode；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；AOT 生产仍 stub；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-fifth` / `B-W305`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `compact_module_for_aot_blob` · remap helpers · syx-mir inventory tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W305
- **非目标：** B-04 `[x]` · AOT compact wire · 体积 golden CI · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W306+（B-04 AOT compact wire / size golden deepen / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-650**

### Wave A W304 正确性地板加深（ADR-648 · Wave B B-04 blob body-strip 接入 deepen · 已交）

- **主轴：** Wave B **B-04 blob body-strip 接入 deepen**（`shaken_module_for_aot_blob` + `encode_module_for_aot` + syx-mir 单测；AOT 路径接入；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-fourth` / `B-W304`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `shaken_module_for_aot_blob` · `encode_module_for_aot` · syx-llvm AOT encode · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W304
- **非目标：** B-04 `[x]` · fn vec 删条 · 体积 golden · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W305+（B-04 fn delete / size golden deepen / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-649**

### Wave A W303 正确性地板加深（ADR-647 · Wave B B-04 LLVM emit 接入 deepen · 已交）

- **主轴：** Wave B **B-04 LLVM emit 接入 deepen**（`llvm_emit_function_names` + emit/register 过滤 + syx-mir/syx-llvm 单测；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-third` / `B-W303`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `llvm_emit_function_names` · syx-llvm emit/register · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W303
- **非目标：** B-04 `[x]` · blob encode 过滤 · 体积可测 · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W304+（B-04 blob 接入 deepen / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-648**

### Wave A W302 正确性地板加深（ADR-646 · Wave B B-04 reachability analysis-only deepen · 已交）

- **主轴：** Wave B **B-04 reachability analysis-only deepen**（`reachable_fn_names` + syx-mir 单测；加深 `b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 **仍 `[~]`**；B-05/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-second` / `B-W302`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `reachable_fn_names` · syx-mir tests · `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `ga-evidence-audit.md` W302
- **非目标：** B-04 `[x]` · emit/llvm+blob 过滤 · 体积可测 · B-05 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W303+（B-04 emit 接入 deepen / B-05 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-647**

### Wave A W301 正确性地板加深（ADR-645 · Wave B B-05 MIR/metadata blob thin-open · 已交）

- **主轴：** Wave B **B-05 MIR/metadata blob thin-open**（索引 `mir-aot.md` Medium + 整模块 encode/load；`b05-mir-metadata-blob.md` + `b05_metadata_blob_thin.rs`）；B-05 `[ ]→[~]`；B-04/B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifty-first` / `B-W301`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `b05-mir-metadata-blob.md` · `b05_metadata_blob_thin.rs` · `mir-aot.md` · `encode_module` / `syx_rt_load_mir_blob` · `ga-evidence-audit.md` W301
- **非目标：** B-05 `[x]` · blob 拆分实现 · 指令流 omit · B-04 `[x]` · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W302+（B-05 deepen / B-04 deepen / A-06 exit / 残余 native=partial） → **已交 ADR-646**

### Wave A W300 正确性地板加深（ADR-644 · Wave B B-04 MIR tree-shake thin-open · 已交）

- **主轴：** Wave B **B-04 MIR tree-shake thin-open**（索引 `mir-aot.md` Short + intra-fn DCE；`b04-mir-tree-shake.md` + `b04_tree_shake_thin.rs`）；B-04 `[ ]→[~]`；B-06 仍 `[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fiftieth` / `B-W300`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `b04-mir-tree-shake.md` · `b04_tree_shake_thin.rs` · `mir-aot.md` · `syx-mir` DCE helpers · `ga-evidence-audit.md` W300
- **非目标：** B-04 `[x]` · 跨函数 shake · 体积可测 · B-05 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉
- **下一窗：** Wave A W301+（Wave B B-05 MIR/metadata blob thin-open） → **已交 ADR-645**

### Wave A W299 正确性地板加深（ADR-643 · Wave B B-06 audit thin-open · 已交）

- **主轴：** Wave B **B-06 llvm stub audit thin-open**（索引既有 `llvm_stub_audit.rs`；`b06-llvm-stub-audit.md` + `b06_llvm_audit_thin.rs`）；B-06 `[ ]→[~]`；Wave A 退出条件已满足仍在；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-ninth` / `B-W299`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `b06-llvm-stub-audit.md` · `b06_llvm_audit_thin.rs` · `llvm_stub_audit.rs` · `ga-evidence-audit.md` W299
- **非目标：** B-06/B-07 `[x]` · stub 语义改写 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · capabilities native 改写 · 新行为钉
- **下一窗：** Wave A W300+（Wave B B-04 MIR tree-shake thin-open） → **已交 ADR-644**

### Wave A W298 正确性地板加深（ADR-642 · Wave A 结构退出 charter · 已交）

- **主轴：** Wave A **结构退出 charter**（plan/02 退出三条件 Q-L2+Q-S1+differential 顶层签收；`wave-a-exit.md` + `wave_a_exit.rs`）；进度 **Wave A 退出条件已满足**；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `wave-a-exit.md` · `wave_a_exit.rs` · `a02-ql2-floor.md` / `a05-qs1-floor.md` / `a04-edge-semantics.md` · `ga-evidence-audit.md` W298
- **非目标：** A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · Suite-2/3/4 wasm skip 全清零 · 新行为钉 · capabilities native 改写 · Wave B 实现 · A-07/A-08
- **下一窗：** Wave A W299+（Wave B B-06 audit thin-open） → **已交 ADR-643**

### Wave A W297 正确性地板加深（ADR-641 · A-02/Q-L2 exit charter · 已交）

- **主轴：** A-02 / Q-L2 **exit charter**（Suite-1 47/47 + W200–W284 产品 pilot floor；`a02-ql2-floor.md` + `a02_ql2_exit.rs`）；A-02/`Q-L2` `[~]/`[ ]`→`[x]`；A-05 仍 `[x]`；A-06 仍 `[~]`；Q-S2 仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a02-ql2-floor.md` · `a02_ql2_exit.rs` · `suite1.rs` / `suite2.rs` · `mir_op_capabilities.toml` · `ga-evidence-audit.md` W297
- **非目标：** Wave A 退出 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · Suite-2/3/4 wasm skip 全清零 · 新行为钉 · capabilities native 改写 · A-07/A-08
- **下一窗：** Wave A W298+（Wave A 结构退出 charter） → **已交 ADR-642**

### Wave A W296 正确性地板加深（ADR-640 · A-05/Q-S1 exit charter · 已交）

- **主轴：** A-05 / Q-S1 **exit charter**（核心 File/Http/Tcp/Process/Sql 四后端地板政策签名；`a05-qs1-floor.md` + `a05_qs1_exit.rs`）；A-05/`Q-S1` `[~]/`[ ]`→`[x]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a05-qs1-floor.md` · `a05_qs1_exit.rs` · `hostfn_qs1_matrix.toml` · `qs1_{file,http,tcp,process,sql}.rs` · `ga-evidence-audit.md` W296
- **非目标：** Wave A 退出 · A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉 · capabilities native 改写 · Orm/Os 全矩阵 · A-07/A-08
- **下一窗：** Wave A W297+（A-02/Q-L2 exit charter） → **已交 ADR-641**

### Wave A W295 正确性地板加深（ADR-639 · A-06/Q-S2 网络负例 thin-open · 已交）

- **主轴：** A-06 **网络负例 thin-open**（timeout/TLS/refuse/Display 既有钉索引；`a06-network-negatives.md` + `a06_qs2_thin.rs`）；A-06 `[ ]→[~]`；Q-S2 检查项仍 `[ ]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a06-network-negatives.md` · `a06_qs2_thin.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W295
- **非目标：** A-06/Q-S2 `[x]` · timeout_tls/tls.client native raise · 新行为钉 · capabilities native 改写 · A-05 `[x]` · Wave A 退出 · A-07/A-08
- **下一窗：** Wave A W296+（A-05/Q-S1 exit charter） → **已交 ADR-640**

### Wave A W294 正确性地板加深（ADR-638 · net.http.server native 硬升格 · 已交）

- **主轴：** `net.http.server` **native=partial→real**（status 仍 partial；wasm 已 real）；复用 graceful shutdown dual-backend 钉；matrix/ga 诚实锁翻写；HttpServe/Shutdown notes 对齐；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_native_wasm_{matrix,ga}.rs` · `host_fn_capabilities.toml` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W294
- **非目标：** status=implemented · timeout_tls/tls.client · TLS server · streaming · listener · websocket · pool/row · 其它 native=partial · A-05 `[x]` · Wave A 退出 · A-06 开窗
- **下一窗：** Wave A W295+（A-06 网络负例 thin-open） → **已交 ADR-639**

### Wave A W293 正确性地板加深（ADR-637 · net.http client native 硬升格 · 已交）

- **主轴：** `net.http` **client-only native=partial→real**（status 仍 partial；wasm 已 real）；复用 qs1_http 四对 dual-backend 钉；matrix 诚实锁翻写 + ga keep-real；HttpGet/Post/Request notes 对齐；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_native_wasm_{matrix,ga}.rs` · `host_fn_capabilities.toml` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W293
- **非目标：** status=implemented · server/timeout_tls/tls.client · listener · websocket · pool/row · 其它 native=partial · A-05 `[x]` · Wave A 退出 · A-06
- **下一窗：** Wave A W294+（net.http.server native hard raise） → **已交 ADR-638**

### Wave A W292 正确性地板加深（ADR-636 · data.sqlite native 硬升格 · 已交）

- **主轴：** `data.sqlite` **native=partial→real**（status 仍 partial；wasm 已 real）；复用 qs1_sql + sql_native_wasm_ga 双端钉；matrix/ga 诚实锁翻写；核心 Sql*×17 HostFn notes 对齐；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `sql_native_wasm_{matrix,ga}.rs` · `host_fn_capabilities.toml` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W292
- **非目标：** status=implemented · Orm · pool/row · http/TLS/listener · websocket · 其它 native=partial · A-05 `[x]` · Wave A 退出 · A-06
- **下一窗：** Wave A W293+（net.http client native hard raise） → **已交 ADR-637**

### Wave A W291 正确性地板加深（ADR-635 · net.tcp native 硬升格 · 已交）

- **主轴：** `net.tcp` **native=partial→real**（status 仍 partial；wasm 已 real）；复用 qs1_tcp + tcp_native_wasm_ga 双端钉；matrix/ga 诚实锁翻写；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-forty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `tcp_native_wasm_{matrix,ga}.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W291
- **非目标：** status=implemented · TLS/listen-accept · websocket · 其它 native=partial · A-05 `[x]` · Wave A 退出 · A-06
- **下一窗：** Wave A W292+（data.sqlite native hard raise） → **已交 ADR-636**

### Wave A W290 正确性地板加深（ADR-634 · A-05 product/parity OR 收束 · 已交）

- **主轴：** A-05 **product/parity 历史指纹 OR trim** — 五 `qs1_*.rs` 删除 product/parity/wasm-v0/llvm-v0 死臂；保留 live gate + evidence/结构臂；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fortieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `qs1_*.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W290
- **非目标：** A-05 `[x]` · Wave A 退出 · native=partial→real · websocket · scrub A-05 历史长行
- **下一窗：** Wave A W291+（net.tcp native hard raise） → **已交 ADR-635**

### Wave A W289 正确性地板加深（ADR-633 · A-05 gate/wave OR 收束 · 已交）

- **主轴：** A-05 **W143+ gate/wave OR trim** — 五 `qs1_*.rs` 历史 `driver-gate-exec-*` / `wave=A-W*` OR 折叠为当前指纹；product/parity 历史 OR 保留；`plan/02` A-05 脚注收口；A-05 仍 `[~]`；矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `qs1_*.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W289
- **非目标：** A-05 `[x]` · Wave A 退出 · native=partial→real · websocket · product OR 全砍
- **下一窗：** Wave A W290+（product OR trim） → **已交 ADR-634**

### Wave A W288 正确性地板加深（ADR-632 · A-04/Q-L4 exit charter · 已交）

- **主轴：** A-04/Q-L4 **exit charter** — 七域 thin-slice 证据地板政策签名（deinit/nil + exception + overflow + NaN + Unicode + actor）；`a04-edge-semantics.md` Status → **Q-L4 floor signed**；`plan/02` A-04 `[~]→[x]` + Q-L4 `[ ]→[x]`；`a04_ql4_exit.rs` + 六套 `a04_*` meta 翻锁；**非** 第八行为域；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_ql4_exit.rs` · `a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W288
- **非目标：** Wave A 退出 · await-hold GA · native=partial→real · websocket · qs1 历史 OR 臂大规模删除
- **下一窗：** Wave A W289+（A-05 OR trim） → **已交 ADR-633**

### Wave A W287 正确性地板加深（ADR-631 · Actor plumbing + A-04 actor 薄第六切 · 已交）

- **主轴：** Actor dual-backend plumbing + A-04/Q-L4 **actor ordering thin sixth slice** — isolation counter + C009 sync self-call + policy fingerprint + serial mutate exact native_mir + wasm AOT；`a04-edge-semantics.md` actor ordering 节（无剩余 deferred）；**批=4 规范钉**；A-04 曾 `[~]`（7/7）· Q-L4 曾 `[ ]` · ActorReentrancyPolicy wasm=real · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_actor.rs` · `a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W287
- **非目标：** Q-L4 `[x]` · Wave A 退出 · await-hold GA · Swift await-reentrancy · websocket
- **下一窗：** Wave A W288+（Q-L4 exit charter） → **已交 ADR-632**

### Wave A W286 正确性地板加深（ADR-630 · A-04 Unicode 薄第五切 · 已交）

- **主轴：** A-04/Q-L4 **Unicode thin fifth slice** — UTF-8≠NFC 等同 + combining 三视图 + isSingleScalar 诚实 + case-map honesty exact native_mir + wasm AOT；`a04-edge-semantics.md` Unicode 节；**批=4 规范钉**；A-04 仍 `[~]`（deinit+nil+exception+overflow+NaN+Unicode = 6/7）· Q-L4 仍 `[ ]` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_unicode.rs` · `a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W286
- **下一窗：** Wave A W287+（A-04 actor ordering） → **已交**
- **非目标：** actor ordering · NFKC/NFKD · locale/Turkish · `text.*` 产品升格 · websocket · Wave A / Q-L4 退出
- **下一窗：** Wave A W287+（A-04 actor ordering）

### Wave A W285 正确性地板加深（ADR-629 · A-04 NaN 薄第四切 · 已交）

- **主轴：** A-04/Q-L4 **NaN thin fourth slice** — NaN/inf/-inf Display + IEEE eq exact（`sqrt(-1)` / `pow(10,400)` / `log(0)` / `n==n`）native_mir + wasm AOT；`a04-edge-semantics.md` NaN 节；**批=4 规范钉**；A-04 仍 `[~]`（deinit+nil+exception+overflow+NaN = 5/7）· Q-L4 仍 `[ ]` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_nan.rs` · `a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W285
- **非目标：** 书 `/0`→inf/nan GA · 关系比较 NaN · `Float.nan` API · Unicode/actor · Wave A / Q-L4 退出
- **下一窗：** Wave A W286+（A-04 Unicode / actor） → **已交**

### Wave A W284 正确性地板加深（ADR-628 · datetime.calendar 产品 dual-backend 升格 · 已交）

- **主轴：** 产品 capability **datetime.calendar** native=noop→real + wasm=host-assisted→real + status=partial→implemented（窄 Gregorian+RFC3339；IANA→timezone）；HostFn DateTime* 已 llvm/wasm=real（W222–W260）本窗翻锁；**≥4 钉**；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `iso_rfc3339_roundtrip.rs` · `retire_unsupported_preview.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W284
- **非目标：** A-04 NaN/Unicode · Q-L4 退出 · Wave A 结构退出 · leap second / full tz GA · websocket · 新 DateTime HostFn 批
- **下一窗：** Wave A W285+（A-04 NaN） → **已交**

### Wave A W283 正确性地板加深（ADR-627 · A-04 overflow 薄第三切 4 钉 · 已交）

- **主轴：** A-04/Q-L4 **overflow thin third slice** — i64 add/sub/mul wrapping exact（`9223372036854775807+1` / `min-1` / `halfmax*2`）native_mir + wasm AOT；`a04-edge-semantics.md` overflow 节；**批=4 规范钉**；A-04 仍 `[~]`（deinit+nil+exception+overflow = 4/7）· Q-L4 仍 `[ ]` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_overflow.rs` · `docs/lang/a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W283
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · R011 overflow Display · llvm native_strict 大字面量 · NaN/Unicode/actor · calendar
- **下一窗：** Wave A W284+（A-04 NaN/Unicode / calendar HostFn llvm） → **已交**

### Wave A W282 正确性地板加深（ADR-626 · timeout_tls + tls.client 产品 wasm2 · 已交）

- **主轴：** 产品 capability **net.http.timeout_tls + net.tls.client** wasm=host-assisted→real；本窗新建 hang R012 / EXTRA_ROOT HTTPS / self-signed fail-closed **wasm AOT** 钉（非纯翻锁）；**批=2**；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_client_timeout_tls.rs` · `http_client_tls.rs` · `http_native_wasm_matrix.rs` · `http_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W282
- **非目标：** A-04 overflow · Q-L4 退出 · Wave A 结构退出 · native/status 升格 · server TLS · calendar/websocket
- **下一窗：** Wave A W283+（A-04 overflow / calendar） → **已交**

### Wave A W281 正确性地板加深（ADR-625 · A-04 exception 薄第二切 4 钉 · 已交）

- **主轴：** A-04/Q-L4 **exception thin second slice** — typed AppError throw/catch message+code + catch-continues `after` + File.open missing catch `err.code=1` native/wasm exact；`a04-edge-semantics.md` exception 节；**批=4 规范钉**；A-04 仍 `[~]`（deinit+nil+exception = 3/7）· Q-L4 仍 `[ ]` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_exception.rs` · `docs/lang/a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W281
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · overflow/NaN/Unicode/actor · TLS/calendar wasm
- **下一窗：** Wave A W282+（TLS 产品 wasm / A-04 overflow） → **已交**

### Wave A W280 正确性地板加深（ADR-624 · net.http.server + data.sqlite 产品 wasm2 · 已交）

- **主轴：** 产品 capability **net.http.server + data.sqlite** wasm=host-assisted→real；闭合 W271 Serve/Shutdown + W272/W273 Sql* HostFn wasm 后的产品行滞后；复用 graceful shutdown / qs1_sql wasm 钉；**批=2**；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirtieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_native_wasm_matrix.rs` · `http_native_wasm_ga.rs` · `sql_native_wasm_matrix.rs` · `sql_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W280
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · status→implemented · TLS/calendar wasm
- **下一窗：** Wave A W281+（A-04 扩面 / TLS·calendar 产品 wasm） → **已交**

### Wave A W279 正确性地板加深（ADR-623 · net.http + net.tcp 产品 wasm2 · 已交）

- **主轴：** 产品 capability **net.http + net.tcp** wasm=host-assisted→real；闭合 W270–W274 HostFn wasm 后的产品行滞后；复用 qs1_http/tcp wasm 钉；**批=2**；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_native_wasm_matrix.rs` · `tcp_native_wasm_matrix.rs` · `tcp_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W279
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · net.http/net.tcp status→implemented · net.http.server wasm
- **下一窗：** Wave A W280+（A-04 扩面 / 产品余轴 wasm） → **已交**

### Wave A W278 正确性地板加深（ADR-622 · datetime.clock + net.http.stream 产品 wasm2 · 已交）

- **主轴：** 产品 capability **datetime.clock + net.http.stream** wasm=host-assisted→real；闭合 W256/W257 native 竖切后的 wasm 滞后；复用 W274 TimeNowMs/InstantNowMs + W271 HttpBodyStream* HostFn wasm 钉；**批=2**；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `instant_duration.rs` · `http_body_stream.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W278
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · net.http/net.tcp product partial→implemented
- **下一窗：** Wave A W279+（A-04 扩面 / 产品余轴 wasm） → **已交**

### Wave A W277 正确性地板加深（ADR-621 · A-04 薄首切 deinit+nil 4 钉 · 已交）

- **主轴：** A-04/Q-L4 **deinit+nil thin slice** — scope-exit+arc-overwrite deinit + ConstNil print + Deque empty-pop nil + TaskLocalGet missing→nil native/wasm exact；`docs/lang/a04-edge-semantics.md` charter；**批=4 规范钉**；A-04 → `[~]` · Q-L4 仍 `[ ]` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `a04_deinit_nil.rs` · `docs/lang/a04-edge-semantics.md` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W277
- **非目标：** A-04 全量七域 · Q-L4 退出 · Wave A 结构退出 · spawn inherit GA · product clock/stream wasm
- **下一窗：** Wave A W278+（A-04 扩面 / 产品 wasm 对齐） → **已交**

### Wave A W276 正确性地板加深（ADR-620 · TaskGroup* HostFn wasm 收官 4 包 · 已交）

- **主轴：** HostFn **TaskGroup* wasm4** — TaskGroupCreate + TaskGroupAdd + TaskGroupWait + TaskGroupCancel wasm=real；WasmMirBridge `groups` + cancel-aware sleep + spawn cancel TLS；**批=4 扩窗例外**（Task HostFn wasm 轴收官）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-sixth`）· gate 仍 **47/47** · soft 仍 **0** · llvm+wasm host-assisted **0**
- **证据：** `host_fn_capabilities.toml` · `syx-wasm-run/bridge.rs` · `task_group_hosts_wasm.rs` · `task_group_hosts_llvm.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W276
- **非目标：** spawn inherit GA · Deadline sleep→C004 GA · process status=implemented · Wave A 结构退出 · A-04 全量
- **下一窗：** Wave A W277+（Wave A 结构债 / A-04） → **已交**

### Wave A W275 正确性地板加深（ADR-619 · Task* HostFn wasm bridge 首切 4 包 · 已交）

- **主轴：** HostFn **Task* wasm4** — TaskLocalSet + TaskLocalGet + TaskSetDeadlineMs + TaskLiveCount wasm=real；WasmMirBridge `install_wasm_task_hosts` + join `mark_reaped`；**批=4 扩窗例外**（bridge 首切）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `syx-wasm-run/bridge.rs` · `task_hosts_wasm.rs` · `task_hosts_llvm.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W275
- **非目标：** TaskGroup* wasm · spawn inherit GA · Deadline sleep→C004 GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W276+（TaskGroup* HostFn wasm / Wave A 结构债 / A-04） → **已交**

### Wave A W274 正确性地板加深（ADR-618 · Tcp×4 + Time×2 HostFn wasm 收官 6 包 · 已交）

- **主轴：** HostFn **Tcp×4 + Time×2 wasm6** — TcpConnect + TcpRead + TcpWrite + TcpHandleRelease + TimeNowMs + InstantNowMs wasm=real；复用 Connect refuse wasm + 新建 Read/Write/Release + Time/Instant wasm；**批=6 扩窗例外**（清完非 Task llvm+wasm host-assisted 余债）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_tcp.rs` · `instant_duration.rs` · `tcp_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W274
- **非目标：** Task*/TaskGroup* wasm · TcpListen/ListenerClose wasm · product net/clock native=real · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W275+（Task* HostFn wasm bridge / Wave A 结构债 / A-04） → **已交**

### Wave A W273 正确性地板加深（ADR-617 · Sql 余债 HostFn wasm 收官 9 包 · 已交）

- **主轴：** HostFn **Sql 余债 wasm9** — SqlPrepareOn + SqlBindStepOn + SqlBindPreparedOn + SqlBeginTransaction + SqlCommitTransaction + SqlRollback + SqlBeginTransactionOn + SqlCommitTransactionOn + SqlRollbackTransactionOn wasm=real；新建 PrepareOn/Bind*On + legacy txn + txn On wasm；**批=9 扩窗例外**（Sql* HostFn wasm 收官）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `sql_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W273
- **非目标：** Task*/TaskGroup* wasm · Tcp* wasm · TimeNowMs/InstantNowMs · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W274+（Tcp*×4 / Time* / Task* bridge / Wave A 结构债） → **已交**

### Wave A W272 正确性地板加深（ADR-616 · Sql 核心 HostFn wasm 扩批 8 包 · 已交）

- **主轴：** HostFn **Sql 核心 wasm8** — SqlOpen + SqlClose + SqlExec + SqlQuery + SqlPrepare + SqlBindStep + SqlExecOn + SqlQueryOn wasm=real；复用 Open/Close wasm + 新建 CRUD/On wasm；**批=8**；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `sql_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W272
- **非目标：** Task*/TaskGroup* wasm · Sql txn/PrepareOn/Bind*On wasm · TimeNowMs/InstantNowMs · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W273+（Sql 余债 / Tcp* / Task* bridge / Wave A 结构债） → **已交**

### Wave A W271 正确性地板加深（ADR-615 · Http 残余 HostFn wasm 收官 8 包 · 已交）

- **主轴：** HostFn **Http 残余 wasm8** — HttpBodyStreamOpen + OpenOpts + ReadChunk + Close + HttpPost + HttpRequest + HttpServe + HttpShutdown wasm=real；新建 BodyStream/Post/Request/Serve·Shutdown wasm；**批=8**（Http* HostFn wasm 收官）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twenty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `http_body_stream.rs` · `hostfn_http.rs` · `http_server_graceful_shutdown.rs` · `http_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W271
- **非目标：** Task*/TaskGroup* wasm · Sql* wasm · TimeNowMs/InstantNowMs · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W272+（Sql 核心 HostFn wasm / Task* bridge / Wave A 结构债） → **已交**

### Wave A W270 正确性地板加深（ADR-614 · Http types/Get HostFn wasm 扩批 8 包 · 已交）

- **主轴：** HostFn **Http types/Get wasm8** — HttpMakeResponse + HttpResponseStatus + HttpResponseText + HttpResponseOk + HttpResponseHeader + HttpResponseHeaders + HttpResponseJson + HttpGet wasm=real；复用 make_response_accessors_wasm + qs1 HttpGet wasm + 新建 Header/Headers/Json wasm；**批=8**；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twentieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `http_types.rs` · `hostfn_http.rs` · `http_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W270
- **非目标：** Task*/TaskGroup* wasm · HttpServe/Shutdown/BodyStream wasm · TimeNowMs/InstantNowMs · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W271+（Http BodyStream/Serve 或 Sql 核心 wasm / Task* bridge / Wave A 结构债） → **已交**

### Wave A W269 正确性地板加深（ADR-613 · File 残余 HostFn wasm 收官 8 包 · 已交）

- **主轴：** HostFn **File 残余 wasm8** — FileSeek + FileMmapRead + FileWithOpen + FileWithOpenFail + FileCloseQuiet + FileWatcherOpen + FileWatcherPoll + FileWatcherClose wasm=real；新建 Seek/CloseQuiet wasm + 复用 Mmap/WithOpen/Watcher；**批=8**（File* HostFn wasm 收官）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-nineteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `path_file_native_wasm_ga.rs` · `filehandle_raii_with.rs` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W269
- **非目标：** Task*/TaskGroup* wasm · TimeNowMs/InstantNowMs wasm · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W270+（Http types/Get HostFn wasm / Task* wasm / Wave A 结构债） → **已交**

### Wave A W266 正确性地板加深（ADR-610 · Str*/text HostFn wasm 扩批 8 包 · 已交）

- **主轴：** HostFn **text HostFn wasm8** — StrNormalizedNFC + StrNormalizedNFD + StrCaseFolded + StrInstLowercased + StrInstUppercased + StringCharAt + StrUnicodeScalarCount + StrUtf8Count wasm=real；复用 NFC/caseFold wasm + 新增 NFD/lower/upper/CharAt/ScalarCount/Utf8Count wasm 钉；**批=8**；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_normalization.rs` · `string_index_honesty.rs` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W266
- **非目标：** Scalars/Utf8Bytes/IsSingleScalar wasm · locale/Turkish · Task* wasm · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W267+（three-view 余 3 wasm / Task* HostFn wasm / Wave A 结构债） → **已交**

### Wave A W265 正确性地板加深（ADR-609 · JsonStream writer HostFn wasm 3 包 · 已交）

- **主轴：** HostFn **JsonStream writer wasm3** — JsonStreamOpenWriter + JsonStreamWrite + JsonStreamFinishWriter wasm=real；复用 box_stream_roundtrip_wasm + 新增 writer finish wasm exact `{"n":42}`；**批=3 扩窗例外**（与 W264 合 JsonStream* HostFn wasm **7/7 收官**）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `json_stream_ndjson.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W265
- **非目标：** CodingKeys/streaming GA · Deadline sleep→C004 GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W266+（Str*/Task* HostFn wasm / Wave A 结构债） → **已交**

### Wave A W264 正确性地板加深（ADR-608 · JsonStream reader HostFn wasm 4 包 · 已交）

- **主轴：** HostFn **JsonStream reader wasm4** — JsonStreamOpenReader + JsonStreamFeed + JsonStreamDecodeNext + JsonStreamCloseReader wasm=real；复用 `m2_s7_03_box_stream_roundtrip_wasm`；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fourteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `json_stream_ndjson.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W264
- **非目标：** JsonStream writer wasm · CodingKeys/streaming GA · Deadline sleep→C004 GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W265+（JsonStream writer HostFn wasm / Wave A 结构债） → **已交**

### Wave A W263 正确性地板加深（ADR-607 · TaskGroup* HostFn llvm 生命周期 4 包 · 已交）

- **主轴：** HostFn **TaskGroup* llvm lifecycle4** — TaskGroupCreate + TaskGroupAdd + TaskGroupWait + TaskGroupCancel llvm=real；syx-rt `groups` 表 + cancel-aware sleep + CallHost join `pending_catch`；`SYX_NATIVE_STRICT` cancel→wait C004 钉；**status 仍 partial** · **wasm=host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-thirteenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `bridge.rs` · `task_group_hosts_llvm.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W263
- **非目标：** JsonStream* wasm · Deadline sleep→C004 GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W264+（JsonStream* HostFn wasm / Wave A 结构债） → **已交**

### Wave A W262 正确性地板加深（ADR-606 · Json* HostFn wasm 窄 4 包 · 已交）

- **主轴：** HostFn **Json* wasm narrow4** — JsonEncode + JsonDecode + JsonEncoderEncode + JsonDecoderDecode wasm=real；复用 S7-01/S7-02 wasm roundtrip + field_paths `DYNAMIC_JSON` `run_wasm`；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-twelfth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `json_encoder_decoder_field_paths.rs` · `encoder_decoder_protocol.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W262
- **非目标：** TaskGroupCancel llvm · JsonStream* wasm · CodingKeys/streaming GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W263+（TaskGroupCancel llvm / JsonStream* HostFn wasm / Wave A 结构债） → **已交**

### Wave A W261 正确性地板加深（ADR-605 · Task* HostFn llvm 窄 4 包 · 已交）

- **主轴：** HostFn **Task* llvm narrow4** — TaskLocalSet + TaskLocalGet + TaskSetDeadlineMs + TaskLiveCount llvm=real；syx-rt bridge 注册 + join `mark_reaped`；四枚 `SYX_NATIVE_STRICT` 钉（set→get `V` / missing `nil` / `DEADLINE_SET` / orphan+join `LEAKED`+`CLEANED`）；**status 仍 partial** · **wasm=host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eleventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `bridge.rs` · `task_hosts_llvm.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W261
- **非目标：** TaskGroupCancel llvm · Json* wasm · spawn inherit / sleep→C004 GA · process status=implemented · Wave A 结构退出
- **下一窗：** Wave A W262+（Json* HostFn wasm / TaskGroupCancel llvm / Wave A 结构债） → **已交**

### Wave A W260 正确性地板加深（ADR-604 · DateTime Parse/Now HostFn wasm 竖切 · 已交）

- **主轴：** HostFn **wasm 扩批** — DateTimeParse + DateTimeParseIso8601 + DateTimeNow wasm=real；三枚新 `run_wasm` exact（`86400000` / `2020-02-29` / epoch-scale ms）；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-tenth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iso_rfc3339_roundtrip.rs` · `retire_unsupported_preview.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W260
- **非目标：** Task* llvm · process status=implemented · Json* wasm · Wave A 结构退出
- **下一窗：** Wave A W261+（Task* llvm narrow4 bridge / Json* HostFn wasm / Wave A 结构债） → **已交**

### Wave A W259 正确性地板加深（ADR-603 · DateTime Offset/Format HostFn wasm 竖切 · 已交）

- **主轴：** HostFn **首批 wasm=real** — DateTimeTimezoneOffset + DateTimeFormat + DateTimeFormatWithTimezone；Offset 复用 W221 wasm 钉 + Format×2 新增 `run_wasm` exact；**status 仍 partial** · **llvm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iana_tzdb_dst.rs` · `iso_rfc3339_roundtrip.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W259
- **非目标：** Task* llvm · process status=implemented · Parse/Now wasm · HostFn 全表 wasm GA · Wave A 结构退出
- **下一窗：** Wave A W260+（Task* llvm bridge / HostFn wasm 扩批 / Wave A 结构债） → **已交**

### Wave A W258 正确性地板加深（ADR-602 · os.process.* 产品 native 竖切 · 已交）

- **主轴：** 产品 **os.process.builder** + **os.process.stdio** + **os.process.signal** native=real（status=partial · wasm=unsupported 不变 → overall 仍 partial）；复用 W243–W247 ProcessBuilder/Child HostFn llvm=real + offline narrow native AOT（`SYX_NATIVE_STRICT`）；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `host_fn_capabilities.toml` · `process_builder.rs` · `child_stdio_pipe.rs` · `child_signal.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W258
- **非目标：** process status=implemented / wasm 升格 · Offset/Format wasm · Task* llvm · Wave A 结构退出
- **下一窗：** Wave A W259+（Offset·Format wasm / Task* HostFn / Wave A 结构债） → **已交**

### Wave A W257 正确性地板加深（ADR-601 · net.http.stream 产品 native 竖切 · 已交）

- **主轴：** 产品 **net.http.stream** native=real（status=implemented · wasm=host-assisted 不变）；复用 W242 HttpBodyStream 四件套 llvm=real + open/readChunk/close 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `host_fn_capabilities.toml` · `http_body_stream.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W257
- **非目标：** os.process.* 产品升格 · Offset/Format wasm · TLS · stream wasm=real · Wave A 结构退出
- **下一窗：** Wave A W258+（os.process.* 产品升格 / Offset·Format wasm / Task* HostFn / Wave A 结构债） → **已交**

### Wave A W256 正确性地板加深（ADR-600 · datetime.clock 产品竖切 + Time/Instant HostFn llvm · 已交）

- **主轴：** 产品 **datetime.clock** native=real（status=implemented · wasm=host-assisted 不变）；HostFn **TimeNowMs** + **InstantNowMs** llvm=real；Clock.nowMs epoch-scale + Instant.now monotonic tick 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `host_fn_capabilities.toml` · `instant_duration.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W256
- **非目标：** net.http.stream / os.process.* 产品升格 · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W257+（net.http.stream·os.process.* 产品升格 / Offset·Format wasm / Task* HostFn / Wave A 结构债） → **已交**

### Wave A W255 正确性地板加深（ADR-599 · legacy txn 三件套 + DateTimeNow 批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — SqlBeginTransaction + SqlCommitTransaction + SqlRollback + DateTimeNow；implicit conn-0 txn `true/true/1/true/true/1` + DateTime.now epoch-scale ms 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm=host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fifth`）· **legacy Sql HostFn llvm 轴收官** · gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `retire_unsupported_preview.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W255
- **非目标：** 产品 datetime.clock 升格 · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W256+（datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W254 正确性地板加深（ADR-598 · legacy Sql CRUD 四件套批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — SqlExec + SqlQuery + SqlPrepare + SqlBindStep；implicit conn-0 exec/query/prepare/bindStep `0`/`1`/`1`/stmt≥1/`1`/`2` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm=host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W254
- **非目标：** 产品升格 · legacy txn 三件套 · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W255+（legacy txn 三件套 / DateTimeNow / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W253 正确性地板加深（ADR-597 · 其余 SqlRow* 五件套批 · 已交）

- **主轴：** HostFn **五件套** llvm=real — SqlRowGetString + SqlRowGetFloat + SqlRowGetBlob + SqlRowIsNull + SqlRowColumnNames；`:memory:` `alice`/`1.5`/blob.length/`true`/`false`/names.length 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm=unsupported** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W253
- **非目标：** 产品升格 · 玩具 SqlExec/SqlQuery · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W254+（玩具 Sql* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W252 正确性地板加深（ADR-596 · SqlPool* 四件套批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — SqlPoolCreate + SqlPoolAcquire + SqlPoolRelease + SqlPoolClose；文件 DSN create/acquire/release/reuse/close 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm=unsupported** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W252
- **非目标：** 产品升格 · 其余 SqlRow* llvm · 玩具 SqlExec/SqlQuery · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W253+（其余 SqlRow* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W251 正确性地板加深（ADR-595 · Sql Prepare/Bind *On + RowGetInt 批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — SqlPrepareOn + SqlBindStepOn + SqlBindPreparedOn + SqlRowGetInt；`:memory:` bindStep/prepare/bindPrepared + getInt→`42` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundred-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W251
- **非目标：** 产品升格 · SqlPool* llvm · 其余 SqlRow* · 玩具 SqlExec/SqlQuery · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W252+（SqlPool* / 其余 SqlRow* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W250 正确性地板加深（ADR-594 · Sql QueryOn + txn *On 批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — SqlQueryOn + SqlBeginTransactionOn + SqlCommitTransactionOn + SqlRollbackTransactionOn；`:memory:` queryOn + commit→1 / rollback→0 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-one-hundredth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_sql.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W250
- **非目标：** 产品升格 · SqlPrepare*/Bind*/Pool*/Row* llvm · 玩具 SqlExec/SqlQuery · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W251+（SqlPrepare*/Bind*/Pool*/Row* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W249 正确性地板加深（ADR-593 · HttpRequest + Response accessor 批 · 已交）

- **主轴：** HostFn **五件套** llvm=real — HttpRequest + HttpResponseOk + HttpResponseHeader + HttpResponseHeaders + HttpResponseJson；HttpRequest loopback `200`/`ok` + Response ok/header/headers/json 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `hostfn_http.rs` · `http_types.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W249
- **非目标：** 产品升格 · Sql* llvm · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W250+（Sql* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W248 正确性地板加深（ADR-592 · FileWithOpen Closure 修复 + Http 批 · 已交）

- **主轴：** HostFn **四件套** llvm=real — FileWithOpen + HttpPost + HttpServe + HttpShutdown；withOpen→`OK` + HttpPost loopback `200`/`ok` + graceful `listening`/`stopped` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；LLVM MakeClosure 按名解析 fn_index；**status 仍 partial** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `syx-llvm` MakeClosure · `filehandle_raii_with.rs` · `hostfn_http.rs` · `http_server_graceful_shutdown.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W248
- **非目标：** 产品升格 · HttpRequest llvm · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W249+（HttpRequest / Sql* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W247 正确性地板加深（ADR-591 · ChildStderr+FileWithOpenFail+HttpGet+TcpListen 批 · 已交）

- **主轴：** HostFn **五件套** llvm=real — ChildStderr + FileWithOpenFail + HttpGet + TcpListen + TcpListenerClose；stderr→`OK`/`4`/`4096`/`0` + with_open_fail→`R001` + HttpGet loopback `200`/`ok` + Tcp bind+close `OPENED`/`CLOSED` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**FileWithOpen 延期**（后由 W248 交）；**status 仍 partial** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `child_stdio_pipe.rs` · `filehandle_raii_with.rs` · `hostfn_http.rs` · `leak_tcp_listener.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W247
- **非目标：** FileWithOpen llvm · 产品升格 · DateTimeNow llvm · Offset/Format wasm · Wave A 结构退出
- **下一窗：** Wave A W248+（FileWithOpen Closure CallHost 修复 / HttpPost·Request·Serve·Shutdown / Sql* / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W246 正确性地板加深（ADR-590 · Child 生命周期+stdin/write 批升 · 已交）

- **主轴：** HostFn Child **五件套** llvm=real — ChildKill + ChildWaitTimeout + ChildSignal + ChildStdin + ChildPipeWrite；kill fail-after + waitTimeout→`OK`/`0` + TERM→`-1` + stdin echo→`OK`/`3`/`3`/`0` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 unsupported** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `process_builder.rs` · `child_signal.rs` · `child_stdio_pipe.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W246
- **非目标：** 产品 `os.process.stdio`/`signal` native=real · ChildStderr llvm · Offset/Format wasm · DateTimeNow llvm · Wave A 结构退出
- **下一窗：** Wave A W247+（ChildStderr / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / FileWithOpen 批 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W245 正确性地板加深（ADR-589 · Child stdout/pipe 批升 · 已交）

- **主轴：** HostFn Child stdout/pipe **四件套** llvm=real — ChildWait + ChildStdout + ChildPipeReadChunk + ChildPipeClose；stdout+readChunk×4+close+wait →`OK`/`4`/`4096`/`0` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 unsupported** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `child_stdio_pipe.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W245
- **非目标：** 产品 `os.process.stdio` native=real · ChildKill/WaitTimeout/Signal/Stdin llvm · Offset/Format wasm · DateTimeNow llvm · Wave A 结构退出
- **下一窗：** Wave A W246+（ChildKill·WaitTimeout·Signal·Stdin/Write 批 / datetime.clock·net.http.stream·os.process.* 产品升格 / Offset·Format wasm / DateTimeNow 政策 / FileWithOpen 批 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W244 正确性地板加深（ADR-588 · FileWatcher+FileCloseQuiet 批升 · 已交）

- **主轴：** HostFn FileWatcher **三件套** + **FileCloseQuiet** llvm=real — Open/Poll/Close + close_quiet；watcher →`1`/`CLOSED_FAIL` + close_quiet→`OK` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W244
- **非目标：** Child* llvm · 产品 clock/stream/`os.process.builder` 升格 · Offset/Format wasm · DateTimeNow llvm · Wave A 结构退出
- **下一窗：** Wave A W245+（Child*·ChildPipe 批 / datetime.clock·net.http.stream·os.process.builder 产品升格 / Offset·Format wasm / DateTimeNow 政策 / FileWithOpen 批 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W243 正确性地板加深（ADR-587 · ProcessBuilder 批升 · 已交）

- **主轴：** HostFn ProcessBuilder **四件套** llvm=real — Create + Cwd + Env + Spawn；create+cwd+env+spawn →`OK`/`0` + marker 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 unsupported** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `process_builder.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W243
- **非目标：** 产品 `os.process.builder` native=real · FileWatcher llvm · Child* llvm · DateTimeNow llvm · Offset/Format wasm=real · Wave A 结构退出
- **下一窗：** Wave A W244+（FileWatcher llvm / datetime.clock·net.http.stream·os.process.builder 产品升格 / Offset·Format wasm / DateTimeNow 政策 / Child* 批 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W242 正确性地板加深（ADR-586 · HttpBodyStream 批升 · 已交）

- **主轴：** HostFn HttpBodyStream **四件套** llvm=real — Open + OpenOpts + ReadChunk + Close；loopback open+readChunk×4→`4` + open_opts→`OK` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `http_body_stream.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W242
- **非目标：** 产品 `net.http.stream` native=real · DateTimeNow llvm · Offset/Format wasm=real · FileWatcher · Wave A 结构退出
- **下一窗：** Wave A W243+（datetime.clock / net.http.stream 产品升格 / Offset·Format wasm / DateTimeNow 政策 / FileWatcher llvm / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W241 正确性地板加深（ADR-585 · FileStream+Fmt 批升 · 已交）

- **主轴：** HostFn **五件套** llvm=real — FileStreamOpen/ReadChunk/Close + FmtPrint/FmtPrintLine；`hi` length→`2` + fmt `hi` 离线窄面 native AOT（`SYX_NATIVE_STRICT`；FmtPrint stdout flush）；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninety-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `file_stream_watcher.rs` · `fmt_print_host.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W241
- **非目标：** DateTimeNow llvm · Offset/Format wasm=real · HttpBodyStream · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W242+（datetime.clock / net.http.stream / Offset·Format wasm / DateTimeNow 政策 / HttpBodyStream llvm / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W240 正确性地板加深（ADR-584 · JsonStream reader 批升 · 已交）

- **主轴：** HostFn JsonStream **reader 四件套** llvm=real — OpenReader + Feed + DecodeNext + CloseReader；Box roundtrip `{n: 42}` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninetieth`）· gate 仍 **47/47** · soft 仍 **0**；JsonStream 全 7 HostFn llvm=real 收官
- **证据：** `host_fn_capabilities.toml` · `json_stream_ndjson.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W240
- **非目标：** DateTimeNow llvm · Offset/Format wasm=real · FmtPrint 批 · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W241+（datetime.clock / net.http.stream / Offset·Format wasm / DateTimeNow 政策 / FmtPrint 批 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W239 正确性地板加深（ADR-583 · 扩窗批升 · 已交）

- **主轴：** HostFn **六件套** llvm=real — StringCharAt + JsonEncode + JsonDecode + JsonStreamOpenWriter + JsonStreamWrite + JsonStreamFinishWriter；`ab[0]→a` / dynamic `42` / writer finish `{"n":42}` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_index_honesty.rs` · `json_encoder_decoder_field_paths.rs` · `json_stream_ndjson.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W239
- **非目标：** JsonStream reader 四件套 · DateTimeNow llvm · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W240+（datetime.clock / net.http.stream / Offset·Format wasm / JsonStream reader llvm / DateTimeNow 政策 / Wave A 结构债 / HostFn inventory 批升） → **已交**

### Wave A W238 正确性地板加深（ADR-582 · 已交）

- **主轴：** HostFn **CharacterIsSingleScalar** llvm=real — `is_single_scalar(a)→true` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ three-view VM；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W238
- **非目标：** ZWJ family llvm GA · StringCharAt HostFn 升格 · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W239+（datetime.clock / net.http.stream / Offset·Format wasm / StringCharAt llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**

### Wave A W237 正确性地板加深（ADR-581 · 已交）

- **主轴：** HostFn **StrUtf8Bytes** llvm=real — `utf8_bytes(hi).length→2` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ three-view VM；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W237
- **非目标：** 元素索引 GA · grapheme GA · Offset/Format wasm=real · 产品 clock/stream/TLS 升格 · Wave A 结构退出
- **下一窗：** Wave A W238+（datetime.clock / net.http.stream / Offset·Format wasm / CharacterIsSingleScalar llvm / Wave A 结构债 / HostFn inventory 深化） → **已交**

### Wave A W224 正确性地板加深（ADR-568 · 已交）

- **主轴：** HostFn **JsonEncoderEncode** llvm=real — flat Box encode `{"n":42}` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W218 mir/wasm/VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `json_encoder_decoder_field_paths.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W224
- **非目标：** JsonDecoderDecode llvm · Json* wasm=real · 产品 clock/stream/TLS 升格 · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W225+（datetime.clock / net.http.stream / JsonDecoderDecode llvm / Offset wasm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W223 正确性地板加深（ADR-567 · 已交）

- **主轴：** HostFn **DateTimeFormatWithTimezone** llvm=real — NY winter formatWithTimezone `2024-01-15T07:00:00.000-05:00` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W78 VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iana_tzdb_dst.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W223
- **非目标：** 产品 clock/stream/TLS 升格 · JsonEncoder* llvm · Offset/Format wasm=real · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W224+（datetime.clock / net.http.stream / JsonEncoderEncode llvm / Offset wasm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W222 正确性地板加深（ADR-566 · 已交）

- **主轴：** HostFn **DateTimeTimezoneOffset** llvm=real — NY winter timezoneOffset `-300` 离线窄面 native AOT（`SYX_NATIVE_STRICT`）+ W221 mir/wasm/VM 回归；**status 仍 partial** · **wasm 仍 host-assisted** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `host_fn_capabilities.toml` · `iana_tzdb_dst.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W222
- **非目标：** 产品 clock/stream/TLS 升格 · FormatWithTimezone llvm · HostFn wasm=real · HostFn 批量 · Wave A 结构退出
- **下一窗：** Wave A W223+（datetime.clock / net.http.stream / FormatWithTimezone llvm / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W221 正确性地板加深（ADR-565 · 已交）

- **主轴：** 产品能力 **datetime.timezone** 单条 native/wasm=real — IANA timezoneOffset NY DST `-300`/`-240` 离线窄面（S5-02 VM + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventy-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `iana_tzdb_dst.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W221
- **非目标：** Clock/Instant now exact · leap second · llvm DateTime=real · http.stream/TLS 产品升格 · HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W222+（datetime.clock / net.http.stream / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W220 正确性地板加深（ADR-564 · 已交）

- **主轴：** 产品能力 **collections.dict** 单条 native/wasm=real — Dictionary.remove 离线窄面（复用 W201 mir-run native/wasm 钉 + 本窗 backends_real meta）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `collection_mutation.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W220
- **非目标：** Dict insert/subscript GA · Deque/HOF · datetime/HTTP 产品升格 · HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W221+（datetime/net.http.stream / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W219 正确性地板加深（ADR-563 · 已交）

- **主轴：** 产品能力 **data.coding.json_stream** 单条 native/wasm=real — JSONStream flat Box write→finish→feed→decodeNext roundtrip `{n: 42}` 离线窄面（S7-03 VM + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `json_stream_ndjson.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W219
- **非目标：** nested/codingPath 产品加深 · File/HTTP streaming · JsonStream* HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W220+（collections.dict / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W218 正确性地板加深（ADR-562 · 已交）

- **主轴：** 产品能力 **data.coding.json_encoder** 单条 native/wasm=real — JsonEncoderEncode+JsonDecoderDecode flat Box roundtrip `{n: 42}` 离线窄面（S7-02 VM + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `json_encoder_decoder_field_paths.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W218
- **非目标：** nested/codingPath 产品加深 · json_stream 产品升格 · JsonEncoder* HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W219+（data.coding.json_stream / collections.dict / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W217 正确性地板加深（ADR-561 · 已交）

- **主轴：** 产品能力 **data.coding.protocol** 单条 native/wasm=real — JsonEncode+JsonDecode 动态 roundtrip `42` 离线窄面（S7-01 VM + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `encoder_decoder_protocol.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W217
- **非目标：** @Codable GA · json_encoder/json_stream 产品升格 · Json* HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W218+（data.coding.json_encoder / collections.dict / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W216 正确性地板加深（ADR-560 · 已交）

- **主轴：** 产品能力 **net.http.types** 单条 native/wasm=real — Http.response + Url.parse 离线窄面（W64/W65 VM + W148 llvm + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `http_types.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W216
- **非目标：** loopback Http.get wasm GA · net.http/server/stream/TLS · HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W217+（data.coding.protocol / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W215 正确性地板加深（ADR-559 · 已交）

- **主轴：** 产品能力 **io.path** status=implemented — PathNormalize + PathExtension 窄面 mir-run native/wasm 钉（W213 join/exists + W146/W147/W70 llvm 已齐）；backends 仍 **native+wasm=real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `path_file_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W215
- **非目标：** UNC/drive · full Path GA · HostFn Path* inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W216+（产品第十二格 / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W214 正确性地板加深（ADR-558 · 已交）

- **主轴：** 产品能力 **io.file.read_all_bytes** 单条 native/wasm=real — read_to_end 窄面（W64 VM + W194 llvm + 本窗 mir-run native/wasm 钉）；**status 仍 implemented** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `path_file_native_wasm_ga.rs` · `mmap_rename.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W214
- **非目标：** OS mmap · FileMmapRead HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W215+（io.path status 深化 / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W213 正确性地板加深（ADR-557 · 已交）

- **主轴：** 产品能力 **io.path** 单条 native/wasm=real — PathJoin + PathExists 窄面（W146/W147 llvm + 本窗 mir-run native/wasm 钉）；**status 曾 partial**（W215 升 implemented）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `path_file_native_wasm_ga.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W213
- **非目标：** PathNormalize/PathExtension wasm（→ **已交 W215**）· HostFn Path* inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W214+（io.file.read_all_bytes / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W212 正确性地板加深（ADR-556 · 已交）

- **主轴：** 产品能力 **text.normalization** 单条 native/wasm=real — NFC compose + caseFolded 窄面（W82/W83 VM exact + 本窗 native/wasm 钉）；status/mir 仍 **implemented/real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `string_normalization.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W212
- **非目标：** NFKC/NFKD · locale case · HostFn inventory 批量 · llvm String GA · Wave A 结构退出
- **下一窗：** Wave A W213+（io.path / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W211 正确性地板加深（ADR-555 · 已交）

- **主轴：** 产品能力 **io.file.watch** mir 轴收口 — FileWatcher.open/poll/close 窄面 mir-run exact（W72/W103/W138 VM + W206 native/wasm + 本窗 mir 钉）；status/mir **implemented/real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W211
- **非目标：** recursive/async notify GA · `text.normalization`（→ **已交 W212**）· HostFn inventory 批量 · Wave A 结构退出
- **下一窗：** Wave A W212+（text.normalization / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W210 正确性地板加深（ADR-554 · 已交）

- **主轴：** 产品能力 **text.index_honesty** 单条 native/wasm=real — Int grapheme offset O(n) 窄面（ASCII s[0]/s[1] + ZWJ s[0]；W83 VM exact + 本窗 native/wasm 钉）；status/mir 仍 **implemented/real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixtieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `string_index_honesty.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W210
- **非目标：** product capability 批量 · StringCharAt HostFn inventory · opaque Index / llvm String GA · Wave A 结构退出
- **下一窗：** Wave A W211+（io.file.watch mir deepen / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W209 正确性地板加深（ADR-553 · 已交）

- **主轴：** MirOp **CallHost** inventory llvm+wasm=real — Suite-1 pilot MirOp **收官**（窄面 `__syx_str_unicode_scalar_count` native/wasm 钉 + differential 16/16）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_op_capabilities.toml` · `pilot_callhost_mir.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W209
- **非目标：** product capability 批量 · `text.index_honesty` · 全 HostFn inventory GA · Wave A 结构退出
- **下一窗：** Wave A W210+（text.index_honesty / Wave A 结构债 / HostFn inventory 深化）→ **已交**

### Wave A W208 正确性地板加深（ADR-552 · 已交）

- **主轴：** MirOp **Print** inventory wasm=real — Suite-1 Int/String/Bool/Nil 窄面 print（W203 bool/nil + 本窗 Int/String wasm 钉 + differential 16/16）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-eighth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_op_capabilities.toml` · `pilot_print_wasm.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W208
- **非目标：** product capability 批量 · CallHost MirOp · `text.index_honesty` · 全类型 Print GA · Wave A 结构退出
- **下一窗：** Wave A W209+（CallHost / text.index_honesty / Wave A 结构债）→ **已交**

### Wave A W207 正确性地板加深（ADR-551 · 已交）

- **主轴：** 产品能力 **text.three_view** 单条 native/wasm=real — grapheme length/subscript/for-in + scalar/utf8 views 窄面（W84–W86 VM exact + 本窗 native/wasm 钉）；status/mir 仍 **implemented/real** · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-seventh`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `string_three_view.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W207
- **非目标：** 产品 capability 批量 native=real · llvm String GA · opaque Index · `text.index_honesty` · Print/CallHost MirOp · Wave A 结构退出
- **下一窗：** Wave A W208+（Print wasm / CallHost / text.index_honesty / Wave A 结构债）→ **已交**

### Wave A W206 正确性地板加深（ADR-550 · 已交）

- **主轴：** 产品能力 **io.file.watch** 单条 native/wasm=real — FileWatcher.open/poll/close 窄面（W72/W103/W138 VM exact + 本窗 native/wasm 钉）；mir/status 仍 **partial**（mir 轴 defer W211）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-sixth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W206
- **非目标：** 产品 capability 批量 native=real · recursive/async notify · mir=real / status=implemented（→ **已交 W211**）· `text.three_view` · Print/CallHost MirOp · Wave A 结构退出
- **下一窗：** Wave A W207+（text.three_view / Print wasm / CallHost / Wave A 结构债）→ **已交**

### Wave A W205 正确性地板加深（ADR-549 · 已交）

- **主轴：** 产品能力 **collections.feature_off** 单条 native/wasm=real — Array.map/filter/reduce 非法 shape U001 fail-closed（W92–W96 VM exact + 本窗 native/wasm 钉）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-fifth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `native_feature_off_no_silent.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W205
- **非目标：** 产品 capability 批量 native=real · `io.file.watch` · llvm HOF GA · Print/CallHost MirOp · Wave A 结构退出
- **下一窗：** Wave A W206+（io.file.watch / Print wasm / CallHost / Wave A 结构债）→ **已交**

### Wave A W204 正确性地板加深（ADR-548 · 已交）

- **主轴：** 产品能力 **io.file.stream** 单条 native/wasm=real — FileStream.open/readChunk/close 窄面（W50/W71/W136 VM exact + 本窗 native/wasm 钉）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-fourth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `file_stream_watcher.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W204
- **非目标：** 产品 capability 批量 native=real · `io.file.watch` / `collections.feature_off` · Print/CallHost MirOp · Wave A 结构退出
- **下一窗：** Wave A W205+（feature_off / Print wasm / CallHost / Wave A 结构债）→ **已交**

### Wave A W203 正确性地板加深（ADR-547 · 已交）

- **主轴：** MirOp **ConstBool** + **ConstNil** inventory llvm/wasm=real — RT 栈对齐（`syx_rt_push_bool` / PushRtBool + push_nil / PushRtNil）；print/branch 窄面 VM/native/wasm 钉 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-third`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_op_capabilities.toml` · `syx-rt` push_bool · `syx-llvm` ConstBool · `syx-wasm-emit`/`syx-wasm-run` · `pilot_const_bool_nil.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W203
- **非目标：** 产品 capability 批量 native=real · Print wasm=real · CallHost deepen · io.file.stream / feature_off · Wave A 结构退出
- **下一窗：** Wave A W204+（产品第四格 / Print wasm / Wave A 结构债）→ **已交**

### Wave A W202 正确性地板加深（ADR-546 · 已交）

- **主轴：** 产品能力 **io.file.raii** 单条 native/wasm=real — FileHandle.withOpen 成功/抛错 live_count 恢复 + host inject `[R001]` 窄面（W48/W49/W91 VM exact + 本窗 native/wasm 钉）；wasm host CallHost 无 try 失败传播与 VM embed 对齐；mir-run try/catch `take_catch` 弹出防 Catch→Throw 死循环 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-second`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `filehandle_raii_with.rs` · `syx-wasm-run` host_fail · `syx-mir-run` take_catch · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W202
- **非目标：** 产品 capability 批量 native=real · ConstBool/ConstNil MirOp emit · FileStream/FileWatcher/mmap EC-M2-3 · Wave A 结构退出
- **下一窗：** Wave A W203+（ConstBool / 下一 capability 行 / Wave A 结构债）→ **已交**

### Wave A W201 正确性地板加深（ADR-545 · 已交）

- **主轴：** 产品能力 **collections.mutation** 单条 native/wasm=real — Dict.remove / Set.insert·remove / Array subscript·setAt 窄面（W80/W81 VM exact + 本窗 native/wasm 钉）；mir-run `dict_build` HeapSlot write-back 与 VM 对齐；wasm AOT `SetBuild` 映射 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifty-first`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `collection_mutation.rs` · `syx-mir-run` dict_build · `syx-wasm-emit`/`syx-wasm-run` SetBuild · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W201
- **非目标：** 产品 capability 批量 native=real · ConstBool/ConstNil MirOp emit · Dict subscript-insert / Deque / HOF · Wave A 结构退出
- **下一窗：** Wave A W202+（ConstBool / 下一 capability 行 / Wave A 结构债）→ **已交**

### Wave A W200 正确性地板加深（ADR-544 · 已交）

- **主轴：** 产品能力 **collections.protocol** 单条 native/wasm=real — Iterator for-in 子集证据（W198 differential `array_for_in` + native/wasm custom Counter parity）；`collection_sequence_iterator` native/wasm behavioral 钉；p4 fixture 路径卫生 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fiftieth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `capabilities.toml` · `collection_sequence_iterator.rs` · `fixtures/parity/actor_isolation.sy` · `fixtures/lang/enum_associated.sy` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W200
- **非目标：** 产品 capability 批量 native=real · ConstBool/ConstNil MirOp emit · Wave A 结构退出
- **下一窗：** Wave A W201+（下一 capability 行 / ConstBool / Wave A 结构债）→ **已交**

### Wave A W199 正确性地板加深（ADR-543 · 已交）

- **主轴：** A-02 Suite-1 pilot **inventory 诚实收口** — 16 MirOp 产品四后端 vm/mir/llvm/wasm=`real` notes 更新；ArrayGet/ObjectGet/MakeIterator/IteratorNext 过期 for-in notes 清除；`syx-wasm-run` `runs_for_in_with_cfg` 编译债修复 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-ninth`）· gate 仍 **47/47** · soft 仍 **0**
- **证据：** `mir_op_capabilities.toml` · `syx-wasm-run` · `hostfn_qs1_matrix.toml` · `collection_sequence_iterator.rs` · `ga-evidence-audit.md` W199
- **非目标：** Wave A 结构退出 · 产品 `native=real` 批量 · ConstBool/ConstNil emit 深修
- **下一窗：** Wave A W200+（产品 native 政策 / Wave A 结构债）→ **已交**

### Wave A W198 正确性地板加深（ADR-542 · 已交）

- **主轴：** A-02 **Suite-1 pilot 收官** — `array_for_in` native+wasm 控制流修复（LLVM `IteratorNext`→`pop_bool` 写入 SSA；wasm `IteratorNext`→`TakeRtI32` + LoadLocal/StoreLocal 栈卫生）；Suite-1 differential **47/47** exact `6` · `MakeIterator`/`IteratorNext` mir_op llvm/wasm=`real` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-eighth`）· soft 仍 **0**
- **证据：** `syx-llvm` · `syx-wasm-emit` · `array_for_in.oracle.json` · `mir_op_capabilities.toml` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W198
- **非目标：** Wave A 结构退出 · 产品 `native=real` 批量 · 自定义 Iterator witness GA 加深
- **下一窗：** Wave A W199+（inventory 深度 / 产品 native 政策）→ **已交**

### Wave A W197 正确性地板加深（ADR-541 · 已交）

- **主轴：** A-02 array_index+dict_subscript **wasm** 薄切片（过期 oracle skip 清除；Suite-1 四后端 exact：`array_index` `"8"`、`dict_subscript` `100`/`200`/`2`；`array_for_in` 双端仍 skip）· `ArrayGet`/`ObjectGet` mir_op wasm=`real` 诚实注解 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-seventh`）· soft 仍 **0** · gate 仍 **46/47**（**非** Q-L2 收官 / **非** 顶 47）
- **证据：** `array_index.oracle.json` · `dict_subscript.oracle.json` · `mir_op_capabilities.toml` ArrayGet/ObjectGet · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W197
- **非目标：** 政策收口顶 47 · unskip `array_for_in` · IteratorNext/JumpIfFalse · 产品 `native=real` / Wave A 退出
- **下一窗：** Wave A W198+（`array_for_in` 控制流 → **47/47**）→ **已交**

### Wave A W196 正确性地板加深（ADR-540 · 已交）

- **主轴：** A-02 dict_subscript native 薄切片（ObjectGet/.length 已通；过期 oracle skip；Suite-1 `dict_subscript` **仅 native** unskip exact `100`/`200`/`2`；wasm/for-in/`array_index` wasm 仍 skip）· `ObjectGet` mir_op llvm=`real` 诚实注解 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-sixth`）· soft 仍 **0** · gate 仍 **46/47**（**非** Q-L2 收官 / **非** 顶 47）
- **证据：** `dict_subscript.oracle.json` · `mir_op_capabilities.toml` ObjectGet · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W196
- **非目标：** 政策收口顶 47 · unskip for-in / array_index wasm / dict wasm · 修 IteratorNext · 产品 `native=real` / Wave A 退出
- **下一窗：** Wave A W197+（array_index wasm / dict wasm / array_for_in 控制流）→ **已交**（index+dict wasm unskip）

### Wave A W195 正确性地板加深（ADR-539 · 已交）

- **主轴：** A-02 ArrayGet native 薄切片（`syx_rt_array_get` + LLVM emit；修 silent skip/index 泄漏；Suite-1 `array_index` **仅 native** unskip exact `"8"`；wasm/for-in/dict 仍 skip）· A-03 mmap `print(b[0])` 收回 exact `"4\n65"` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-fifth`）· soft 仍 **0** · gate 仍 **46/47**（**非** Q-L2 收官 / **非** 顶 47）
- **证据：** `syx-rt/abi.rs` · `syx-llvm` · `mir_op_capabilities.toml` ArrayGet · `array_index.oracle.json` · `path_file_native_wasm_ga.rs` mmap · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W195
- **非目标：** 政策收口顶 47 · unskip for-in/dict/wasm · 产品 `native=real` / Wave A 退出
- **下一窗：** Wave A W196+（array_for_in / dict_subscript / wasm array_index）→ **已交**（dict_subscript native unskip）

### Wave A W194 正确性地板加深（ADR-538 · 已交）

- **主轴：** A-03 FileSeek+FileMmapRead native product exec（`path_file_native_wasm_ga.rs` VM↔native strict：seek `print("ok")` + mmap `print(b.length)==4`；元素下标 `b[0]` 因 native ArrayGet/A-02 诚实 defer；`llvm=partial`→`real`；wasm 仍 host-assisted；产品 native 仍 partial；`OTHER_PATH_FILE_HOSTS` **清空**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-fourth`）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `host_fn_capabilities.toml` · `path_file_native_wasm_ga.rs`（2 exec + ga7 columns）· `path_file_native_wasm_matrix.rs` · `reset_file_handles_for_tests` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W194
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · `native=real` / Wave A 退出 · 改 host/facade（含真 OS mmap）
- **下一窗：** Wave A W195+（A-02/SSA 薄切片）→ **已交**（ArrayGet native thin；`array_index` native unskip）

### Wave A W193 正确性地板加深（ADR-537 · 已交）

- **主轴：** A-03 FileRead+FileWrite native product exec（`path_file_native_wasm_ga.rs` VM↔native strict：write `print("ok")` + read roundtrip `print("hello")`；`llvm=partial`→`real`；wasm 仍 host-assisted；产品 native 仍 partial）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-third`）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `host_fn_capabilities.toml` · `path_file_native_wasm_ga.rs`（2 exec + ga6 columns）· `path_file_native_wasm_matrix.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W193
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileSeek/FileMmapRead llvm=real · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W194+（A-02/SSA · FileSeek/FileMmapRead native 薄加深）→ **已交**（FileSeek+FileMmapRead native exec；OTHER 清空）

### Wave A W192 正确性地板加深（ADR-536 · 已交）

- **主轴：** A-03 FileOpen+FileClose native product exec（`path_file_native_wasm_ga.rs` VM↔native strict：open `print(h)==1` + close `print("ok")`；`llvm=partial`→`real`；wasm 仍 host-assisted；产品 native 仍 partial）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-second`）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `host_fn_capabilities.toml` · `path_file_native_wasm_ga.rs`（2 exec + columns）· `path_file_native_wasm_matrix.rs` · `reset_file_handles_for_tests` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W192
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileRead/Write llvm=real · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W193+（A-02/SSA · FileRead/Write native 薄加深）→ **已交**（FileRead+FileWrite native exec）

### Wave A W191 正确性地板加深（ADR-535 · 已交）

- **主轴：** A-03 ProcessBuilder.create + cwd Display API 窗（`TypeMethod` create/cwd `host: None`；host R003 + facade throws 已齐；create empty-command bare R003 + facade Display `[R007] ProcessBuilder create failed`；cwd invalid-handle facade Display `[R007] ProcessBuilder cwd failed`；**单失败面政策**；**ProcessBuilder Display 收齐**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-forty-first`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（create bare + 2 Display + cwd bare 保留）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W191
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileOpen native · `native=real` / Wave A 退出 · 改 host（含 cwd 空 path）/ facade · qs1 err.code 变更
- **下一窗：** Wave A W192+（FileOpen native 薄加深 · A-02/SSA）→ **已交**（FileOpen+FileClose native exec）

### Wave A W190 正确性地板加深（ADR-534 · 已交）

- **主轴：** A-03 ProcessBuilder.env Display API 窗（仅 `TypeMethod` `ProcessBuilder.env` `host: None`；host R003 + facade throws 已齐；invalid-handle + empty-key facade Display `[R007] ProcessBuilder env failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fortieth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W190
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · ProcessBuilder.create/cwd Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade · qs1 err.code 变更
- **下一窗：** Wave A W191+（ProcessBuilder.create/cwd Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（ProcessBuilder.create+cwd Display 收齐）

### Wave A W189 正确性地板加深（ADR-533 · 已交）

- **主轴：** A-03 ProcessBuilder.spawn Display API 窗（仅 `TypeMethod` `ProcessBuilder.spawn` `host: None`；host R003 + facade throws 已齐；invalid-handle + missing-cmd facade Display `[R007] ProcessBuilder spawn failed`；qs1 spawn-missing caught `err.code` **R003→4**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-ninth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `hostfn_process.rs`（3× err.code）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W189
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · ProcessBuilder.create/cwd/env Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W190+（ProcessBuilder.env Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（ProcessBuilder.env Display）

### Wave A W188 正确性地板加深（ADR-532 · 已交）

- **主轴：** A-03 ChildPipe.write Display API 窗（仅 `TypeMethod` `ChildPipe.write` `host: None`；host R003 + facade throws 已齐；invalid-handle + write-after-close stdin facade Display `[R007] ChildPipe write failed`；**ChildPipe close/readChunk/write Display 收齐**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-eighth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `child_stdio_pipe.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W188
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileOpen native · ProcessBuilder Display · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W189+（FileOpen native 薄加深 · A-02/SSA · ProcessBuilder Display）→ **已交**（ProcessBuilder.spawn Display）

### Wave A W187 正确性地板加深（ADR-531 · 已交）

- **主轴：** A-03 ChildPipe.readChunk Display API 窗（仅 `TypeMethod` `ChildPipe.readChunk` `host: None`；host R003 + facade throws 已齐；invalid-handle + read-after-close facade Display `[R007] ChildPipe read failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-seventh`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `child_stdio_pipe.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W187
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · ChildPipe.write Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W188+（ChildPipe.write Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（ChildPipe.write Display；ChildPipe Display 收齐）

### Wave A W186 正确性地板加深（ADR-530 · 已交）

- **主轴：** A-03 Child.stdin Display API 窗（仅 `TypeMethod` `Child.stdin` `host: None`；host R003 + facade throws 已齐；invalid-handle + stdin-after-kill facade Display `[R007] Child stdin take failed`；**Child stdout/stderr/stdin Display 收齐**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-sixth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W186
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileOpen native · ChildPipe Display · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W187+（FileOpen native 薄加深 · A-02/SSA · ChildPipe readChunk/write Display）→ **已交**（ChildPipe.readChunk Display）

### Wave A W185 正确性地板加深（ADR-529 · 已交）

- **主轴：** A-03 Child.stderr Display API 窗（仅 `TypeMethod` `Child.stderr` `host: None`；host R003 + facade throws 已齐；invalid-handle + stderr-after-kill facade Display `[R007] Child stderr take failed`；**stdio take Display 链续进**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-fifth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W185
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.stdin Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W186+（Child.stdin Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.stdin Display；stdio take Display 收齐）

### Wave A W184 正确性地板加深（ADR-528 · 已交）

- **主轴：** A-03 Child.stdout Display API 窗（仅 `TypeMethod` `Child.stdout` `host: None`；host R003 + facade throws 已齐；invalid-handle + stdout-after-kill facade Display `[R007] Child stdout take failed`；**stdio take Display 链启动**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-fourth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W184
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.stderr/stdin Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W185+（Child.stderr Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.stderr Display；stdio take 链续进）

### Wave A W183 正确性地板加深（ADR-527 · 已交）

- **主轴：** A-03 Child.waitTimeout Display API 窗（仅 `TypeMethod` `Child.waitTimeout` `host: None`；host R003 + facade throws 已齐；invalid-handle + waitTimeout-after-kill facade Display `[R007] Child waitTimeout failed`；**Child wait/kill/signal/waitTimeout Display 收齐**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-third`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003 + wait_timeout_kills 回归）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W183
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileOpen native · Child stdio Display · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W184+（FileOpen native 薄加深 · A-02/SSA · Child stdio Display）→ **已交**（Child.stdout Display；stdio take 链启动）

### Wave A W182 正确性地板加深（ADR-526 · 已交）

- **主轴：** A-03 Child.signal Display API 窗（仅 `TypeMethod` `Child.signal` `host: None`；host R003 + facade throws 已齐；invalid-handle + signal-after-kill facade Display `[R007] Child signal failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-second`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003）· `child_signal.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W182
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.waitTimeout Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W183+（Child.waitTimeout Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.waitTimeout Display；生命周期 Display 收齐）

### Wave A W181 正确性地板加深（ADR-525 · 已交）

- **主轴：** A-03 Child.wait Display API 窗（仅 `TypeMethod` `Child.wait` `host: None`；host R003 + facade throws 已齐；invalid-handle + wait-after-kill facade Display `[R007] Child wait failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirty-first`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003 + kill_and_fail_after 回归）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W181
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.signal Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W182+（Child.signal Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.signal Display）

### Wave A W180 正确性地板加深（ADR-524 · 已交）

- **主轴：** A-03 Child.kill Display API 窗（仅 `TypeMethod` `Child.kill` `host: None`；host R003 + facade throws 已齐；invalid-handle + kill-after-kill facade Display `[R007] Child kill failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirtieth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `process_builder.rs`（2 Display + bare R003 + kill_and_fail_after 回归）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W180
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.wait/signal Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W181+（Child.wait Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.wait Display）

### Wave A W179 正确性地板加深（ADR-523 · 已交）

- **主轴：** A-03 Sql.poolAcquire Display API 窗（仅 `TypeMethod` `Sql.poolAcquire` `host: None`；host R003 + facade throws 已齐；unknown-pool + acquire-after-close facade Display `[R007] sql poolAcquire failed`；**同池 Acquire/Release/Close Display 三角收齐**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-ninth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `error_sql_conn.rs`（2 Display + bare R003 回归）· `sql_connection_pool.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W179
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Child.kill Display · FileOpen native · `poolCreate` facade · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W180+（Child.kill Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（Child.kill Display）

### Wave A W178 正确性地板加深（ADR-522 · 已交）

- **主轴：** A-03 Sql.poolRelease Display API 窗（仅 `TypeMethod` `Sql.poolRelease` `host: None`；host R003 + facade throws 已齐；unknown-pool + double-release facade Display `[R007] sql poolRelease failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-eighth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `error_sql_conn.rs`（2 Display + bare R003 回归）· `sql_connection_pool.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W178
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · Sql.poolAcquire Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W179+（Sql.poolAcquire Display · Child.kill Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**（poolAcquire；同池三角收齐）

### Wave A W177 正确性地板加深（ADR-521 · 已交）

- **主轴：** A-03 Sql.poolClose Display API 窗（仅 `TypeMethod` `Sql.poolClose` `host: None`；host R003 + facade throws 已齐；unknown-pool + close-after-close facade Display `[R007] sql poolClose failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-seventh`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `error_sql_conn.rs`（2 Display + bare R003 回归）· `sql_connection_pool.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W177
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W178+（FileOpen native 薄加深 · A-02/SSA · 结构债收口仅 47 之后）→ **已交**（poolRelease Display）

### Wave A W176 正确性地板加深（ADR-520 · 已交）

- **主轴：** A-03 ChildPipe close Display API 窗（仅 `TypeMethod.close` `host: None`；host R003 + facade throws 已齐；invalid-handle + close-after-close facade Display `[R007] ChildPipe close failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-sixth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**
- **证据：** `registry.rs` · `child_stdio_pipe.rs`（2 Display + bare R003 回归）· `leak_child_pipe.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W176
- **非目标：** A-02/SSA · Sql.poolClose Display · FileOpen native · `native=real` / Wave A 退出 · 改 host/facade
- **下一窗：** Wave A W177+（Sql.poolClose Display · FileOpen native 薄加深 · A-02/SSA）→ **已交**

### Wave A W175 正确性地板加深（ADR-519 · 已交）

- **主轴：** A-03 TcpListener close Display API 窗（host miss → R003；facade `close throws` + `NetworkError`；`TypeMethod.close` `host: None`；invalid-handle + close-after-close facade Display `[R007] TcpListener close failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-fifth`；无新 HostFn 行）· A-02 **本窗跳过** · soft 仍 **0** · gate 仍 **46/47**（47 只等 A-02）
- **证据：** `hosts.rs` · `tcp.sy` · `registry.rs` · `leak_tcp_listener.rs`（2 Display + leak 回归）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W175
- **非目标：** A-02/SSA unskip 或政策收口顶 47 · ChildPipe/Sql.poolClose · FileOpen native · `native=real` / Wave A 退出 · 新 soft
- **下一窗：** Wave A W176+（A-02/SSA · FileOpen native 薄加深 · ChildPipe/poolClose Display）→ **已交**

### Wave A W174 正确性地板加深（ADR-518 · 已交）

- **主轴：** A-03 HttpBodyStream close Display API 窗（host miss → R003；facade `close throws` + `IOError`；`TypeMethod.close` `host: None`；invalid-handle + close-after-close facade Display `[R007] HttpBodyStream close failed`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-fourth`；无新 HostFn 行）· A-02 **本窗跳过** · soft one-liner 仍 **0** · gate **46/47** · **无 capability 海量变更**
- **证据：** `http.rs` · `http_body_stream.sy` · `registry.rs` · `http_body_stream.rs`（2 Display）· `leak_http_body_stream.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W174
- **非目标：** A-02/SSA unskip · open/readChunk facade throws · product `native=real` / `wasm=real` · Wave A 退出宣称 · 新 soft one-liner · Xvfb
- **下一窗：** Wave A W175+（A-02/SSA · Wave A 结构债收口叙事）→ **已交**

### Wave A W173 正确性地板加深（ADR-517 · 已交）

- **主轴：** A-03 driver_gate soft one-liner 清债第二十三片（`guidemo-import` soft → SynUI surface/exec exact；默认构建 pin `packages/ui` ≥15 `__syx_synui_*` + registry；`--features gui-synui` 下 embedded exec；soft-debt-zero meta；**soft one-liner 0**）· A-03 卫生 LSP fingerprint / 死 `guidemo.sy` ignore / `gui_demo.sy` 注释 · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-third`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（1 soft one-liner 删除 + SynUI exact + soft-debt-zero meta）· `lsp_phase98_real.rs` · `native_gui_parity.rs` · `gui_demo.sy` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W173
- **非目标：** 恢复 `guidemo.sy` / 复活 `__syx_gui_*` · HttpBodyStream close Display API · A-02/SSA · Xvfb / 像素 product GA · Wave A 终章全文 · product `native=real` / `wasm=real` · Wave B
- **下一窗：** Wave A W174+（HttpBodyStream close Display · A-02/SSA · Wave A closeout 叙事）→ **已交**

### Wave A W172 正确性地板加深（ADR-516 · 已交）

- **主轴：** A-03 driver_gate soft one-liner 清债第二十二片（`borrow-mandatory` soft → `*_meta_allowable_charter`；`gui-dlopen-prod` soft → source exact；`let lib`/模块表面退出 soft-debt 计数；2 软钉删除、2 exact/meta 新增；**meta 三钉收官**）· A-03 卫生删除 `borrow_prover_mandatory.rs` 分散 fingerprint · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-second`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft one-liner 删除 + meta charter + prod source exact）· `borrow_prover_mandatory.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W172
- **非目标：** `native-guidemo-import-ui-v2` · 恢复 guidemo.sy · Xvfb / 像素 product GA · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W173+（`native-guidemo-import-ui-v2` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W171 正确性地板加深（ADR-515 · 已交）

- **主轴：** A-03 driver_gate soft one-liner 清债第二十一片（`browser-tier2u` + `dist-prod-ga` soft → `*_meta_allowable_charter`；`let lib` 形态退出 soft-debt 计数；2 软钉删除、2 meta charter 新增；**meta 永久收口**）· A-03 卫生删除 `browser_tier2q/r/s/t/u` 分散 fingerprint · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twenty-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft one-liner 删除 + 2 meta charter）· `browser_tier2q–u.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W171
- **非目标：** `borrow-mandatory` meta closeout · `gui-dlopen-prod` / `native-guidemo-import-ui-v2` · 其余 3 处 soft one-liner · 恢复 guidemo.sy · Xvfb · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W172+（`borrow-mandatory` meta closeout / `gui-dlopen-prod` / `native-guidemo-import-ui-v2` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W170 正确性地板加深（ADR-514 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第二十片（`gui-widgets-bitmap` headless label/pixel exact + `net-h2-alpn` http2_enabled default-on source exact；2 软钉删除、2 exact 新增；**GUI widgets + h2 表面**）· A-03 卫生 `gui_winit_visible.rs` 删除 widgets 行 contains + `net_http2.rs` 删除分散 fingerprint · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twentieth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + 2 exact）· `gui_winit_visible.rs` · `net_http2.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W170
- **非目标：** `gui-dlopen-prod` / `native-guidemo-import-ui-v2` · meta 三钉改写 · 其余 5 处 `lib.rs contains` · 公网 ALPN · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W171+（`gui-dlopen-prod` / `native-guidemo-import-ui-v2` · meta closeout · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W169 正确性地板加深（ADR-513 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十九片（`ensure_gui_loaded` API exact + `gui-dlopen-default` hashmap fallback exact；2 软钉删除、2 exact 新增；**GUI 弱耦合首片**）· A-03 卫生 `gui_winit_softbuffer.rs` 删除分散 fingerprint contains · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-nineteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + 2 exact）· `gui_winit_softbuffer.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W169
- **非目标：** `net-h2-alpn` / `gui-dlopen-prod` / `gui-widgets` / `native-guidemo-import-ui-v2` · meta 三钉改写 · 其余 7 处 `lib.rs contains` · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W170+（`net-h2-alpn` / `gui-widgets` / `gui-dlopen-prod` / `native-guidemo-import-ui-v2` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W168 正确性地板加深（ADR-512 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十八片（`llvm-multi-link` hello.sy native link+run exec exact + `native-llvm-locals-v7` invalidation exact fold；2 软钉删除、2 exact 新增；**heap 轴收官后转向 LLVM/native link / locals 轴**）· A-03 卫生 `llvm_gui_callhost.rs` pivot `guidemo.sy` → `hello.sy` + 删除分散 fingerprint contains · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + 2 exact）· `llvm_gui_callhost.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W168
- **非目标：** `net-h2-alpn` / `gui-dlopen-*` / `native-guidemo-import-ui-v2` · meta 三钉改写 · 其余 9 处 `lib.rs contains` · `ensure_gui_loaded` / `gui-widgets` · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W169+（`net-h2-alpn` / `gui-dlopen-*` / `native-guidemo-import-ui-v2` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W167 正确性地板加深（ADR-511 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十七片（`heap_v74` + `heap_v75` 单钉 → invalidation exact fold；2 软钉删除、2 exact 新增；**heap 历史 fingerprint 系列收官**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventeenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + 2 invalidation exact）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W167
- **非目标：** `llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · meta 三钉改写 · 其余 11 处 `lib.rs contains` · `native-llvm-locals-v7` · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W168+（`llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · `native-llvm-locals-v7` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）→ **已交**

### Wave A W166 正确性地板加深（ADR-510 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十六片（`heap_v58` + `heap_v69` 单钉 → invalidation exact fold：`heap_v58/v69_phase*_fingerprint` + active v110 cluster + cache_v110 gate；2 软钉删除、2 exact 新增）· A-03 卫生 `llvm_phi_clang.rs` pivot `main.sy` → `hello.sy` · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + 2 invalidation exact）· `llvm_phi_clang.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W166
- **非目标：** `llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · meta 三钉改写 · 其余 13 处 `lib.rs contains` · `heap_v74`/`heap_v75` · `native-llvm-locals-v7` · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W167+（`llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · heap v74/v75 · `native-llvm-locals-v7` · GA exec closeout meta · HttpBodyStream close · A-02/SSA）

### Wave A W165 正确性地板加深（ADR-509 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十五片（`heap_v72` + `heap_v73` duplicate pair → invalidation exact merge：`heap_v72/v73_phase*_fingerprint` + active v110 cluster + cache_v110 gate；4 软钉删除、2 exact 新增）· A-03 卫生 `wasm_runtime_zero.rs` 删除 p99/p100 fingerprint contains（行为测试已覆盖）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（4 soft contains 删除 + 2 invalidation exact）· `wasm_runtime_zero.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W165
- **非目标：** `llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · meta 三钉改写 · 其余 15 处 `lib.rs contains` · `heap_v58`/`heap_v69`/`heap_v74`/`heap_v75` · `native-llvm-locals-v7` · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W167（ADR-511）→ **已交**

### Wave A W164 正确性地板加深（ADR-508 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十四片（`syx-cache-v73` + `syx-cache-v74` bare-tag → invalidation exact fold：historical comment chain + active tag v110 + cache_v110 gate；软钉删除）· A-03 卫生 `aot_matrix.rs` dead main/guidemo 移除、`main_ratio` pivot hello.sy · A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fourteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + invalidation 断言）· `aot_matrix.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W164
- **非目标：** `llvm-multi-link` / `net-h2-alpn` / `gui-dlopen-*` · meta 三钉改写 · 其余 19 处 `lib.rs contains` · heap 历史 fingerprint 批量 · 分散 wasm fingerprint contains · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W165（ADR-509）→ **已交**

### Wave A W163 正确性地板加深（ADR-507 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十三片（`audit-align-v1:ci-script-adr-1550-1950` + `adr-1700-2000` → script exact fold：probe oracle `MIN_DRIVER=2600`/`MIN_LSP=3050` + audit script structure；软钉删除）· A-03 卫生 `wasm_runtime_zero.rs` + `wasm_runtime_smoke.rs` 路径 pivot（`main.sy`/`guidemo.sy` → `hot_loop.sy`/`hello.sy`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-thirteenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + script 断言）· `wasm_runtime_zero.rs` · `wasm_runtime_smoke.rs` · `wasm_mir_exec_zero.rs` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W163
- **非目标：** `net-h2-alpn` / `gui-dlopen-*` / `llvm-multi-link` · meta 三钉改写 · 其余 21 处 `lib.rs contains` · heap/cache 历史 fingerprint · 分散 wasm fingerprint contains · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W164（ADR-508）→ **已交**

### Wave A W162 正确性地板加深（ADR-506 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十二片（`codable-hostfn-prune-v2` + `codable-host-body-prune-v1` → inventory exact fold：HostFnId 名无 `_l5`…`_l13`；vm/registry 无 L5/L9/L13 impl；软钉删除）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-twelfth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft contains 删除 + inventory 断言保留/重命名）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W162
- **非目标：** `wasm_runtime_zero` 路径修复 · `net-h2-alpn` / `gui-dlopen-*` / `llvm-multi-link` · meta 三钉改写 · 其余 23 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W163（ADR-507）→ **已交**

### Wave A W161 正确性地板加深（ADR-505 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十一片（`lang-deinit-auto-v1` + `lang-deinit-arc-v1` → product exec exact fold：W2/W9 `heap_deinit_order.sy` stdout `deinit:\n1\ndone\ndeinit:\n2`；软钉删除）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eleventh`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 soft 删除 + 2 exact 重命名）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W161
- **非目标：** `wasm_runtime_zero` 路径修复 · `net-h2-alpn` / `gui-dlopen-*` / `llvm-multi-link` · codable-prune 对 · meta 三钉改写 · 其余 25 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W162（ADR-506）→ **已交**

### Wave A W160 正确性地板加深（ADR-504 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第十片（`farm-tcp-frames-v1` → bytes `pushed`（lease + push→get；非 VM `run()`）；`farm-frame-lease-v1` → err `farm lease hard reject: farm lease expired`（`farm_lease_acquire(0)` + `farm_worker_run_cycle`；probe oracle））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-tenth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W160
- **非目标：** `net-h2-alpn` / `gui-dlopen-*` / `llvm-multi-link` · `wasm_runtime_zero` 路径修复 · meta 三钉改写 · 其余 27 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W161（ADR-505）→ **已交**

### Wave A W159 正确性地板加深（ADR-503 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第九片（`wasm-guidemo-zero-v1` → `mir_exec==0` && `run_op==0`（`examples/hello.sy`；probe pivot：`guidemo.sy` 缺失；非 VM `run()`）；`wasm-guest-control-v1` → banned histogram ops 全 0（`examples/hot_loop.sy`；probe pivot：`main.sy` 缺失））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-ninth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W159
- **非目标：** `net-h2-alpn` / `gui-dlopen-*` / `llvm-multi-link` / `farm-tcp-frames` · meta 三钉改写 · 其余 29 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W160（ADR-504）→ **已交**

### Wave A W158 正确性地板加深（ADR-502 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第八片（`gc-churn-stress-v1` → `gc:churn:stress-a\ngc:churn:stress-b\ndone`（`__syx_gc_churn_heap`）；`farm-tcp-3node-v2` → 三节点 bytes `node-{i}`（lease + `fetch_cache_remote`；非 VM `run()`））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-eighth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W158
- **非目标：** `net-h2-alpn` / `gui-dlopen-*` / `wasm-guidemo-zero` · meta 三钉改写 · 其余 31 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W159（ADR-503）→ **已交**

### Wave A W157 正确性地板加深（ADR-501 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第七片（`farm-tcp-prod-v1` → bytes `tenant-cache-hit`（lease + `fetch_cache_remote`；非 VM `run()`）；`gc-generational-v13-quad-v1` → `0\ngc-done`（`__syx_vm_native_gc_collect`；probe pivot：`__syx_gc_collect` S002））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-seventh`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W157
- **非目标：** `net-h2-alpn` / `gui-dlopen-prod` · meta 三钉改写 · 其余 33 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W158（ADR-502）→ **已交**

### Wave A W156 正确性地板加深（ADR-500 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第六片（`plugin-cdn-prod-v1` → `cdn-p89` poison+reinstall echo；`wasm-aot-zero-run-op-v1` → toolchain `measure_aot_run_op_count == 0`（非 VM `run()`；probe pivot））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-sixth`；无新 HostFn 行）· **Meta 章程化**：`browser-tier2u` / `dist-prod-ga` / `borrow-mandatory-v3` 维持可接受 contains · A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W156
- **非目标：** meta 三钉 → exec 改写 · `net-h2-alpn`（本窗未选）· `gui-dlopen` / `farm-tcp` / `gc-generational` · 其余 35 处 `lib.rs contains` · 分散 fingerprint 收紧 · GA exec closeout meta · HttpBodyStream close Display · product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W157（ADR-501）→ **已交**

### Wave A W155 正确性地板加深（ADR-499 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第五片（`sql-migrations-v1` → `1`（`__syx_sql_migrate`）；`orm-query-ga-v1` → `1`（`__syx_orm_find_where` rows.length））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fifth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W155
- **非目标：** `browser-tier2u` / `dist-prod` / `borrow-mandatory` meta 钉 · 其余 37 处 `lib.rs contains` · `sql_migrations` / `orm_query` 分散 `contains` 收紧 · GA exec closeout meta 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W156（ADR-500）→ **已交**

### Wave A W154 正确性地板加深（ADR-498 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第四片（`codable-l14-surface-v1` → `val` L14 encode→decode；`native-async-resume-v4` → 4 行 nested await stdout 顺序）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-fourth`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W154
- **非目标：** `browser-tier2u` / `dist-prod` / `borrow-mandatory` contains 钉 · 其余 39 处 `lib.rs contains` · `codable_l14` / `async_native` 分散 `contains` 收紧 · GA exec closeout meta 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W155（ADR-499）→ **已交**

### Wave A W153 正确性地板加深（ADR-497 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第三片（`net-grpc-stream-v1` → `grpc-ok:88:echo`（probe oracle；host arg order）；`file-watch-os-default-v1` → `Array([])` missing-path）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-third`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W153
- **非目标：** `browser-tier2u` / `dist-prod` / `borrow-mandatory` / `codable_l4` contains 钉 · 其余 41 处 `lib.rs contains` · `net_grpc_stream` / `file_watch_os` 分散 `contains` 收紧 · GA exec closeout meta 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W154（ADR-498）→ **已交**

### Wave A W152 正确性地板加深（ADR-496 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债第二片（`reflect-cast-strict-quad-v1` → `true`（`Point(x:7) is Point`）；`select-async-ga-v1` → `88`（sync `select` recv））· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-second`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W152
- **非目标：** `net-grpc-stream-v1` driver_gate 钉 · 其余 43 处 `lib.rs contains` · `runtime_cast` / `net_grpc_stream` / `select_async` 分散 `contains` 收紧 · GA exec closeout meta 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W153（ADR-497）→ **已交**

### Wave A W151 正确性地板加深（ADR-495 · 已交）

- **主轴：** A-03 driver_gate `lib.rs contains` 清债首片（`native-string-exact-v1` → `Hello, SYX!`；`hof-witness-quad-v1` → `hof_sort_closure.sy` `1`/`3`；String closure parser-blocked 以 Int sortBy fixture 为 oracle）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:driver-gate-exec-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `driver_gate.rs`（2 exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W151
- **非目标：** 其余 45 处 `lib.rs contains` · `native_string_interp` / `hof_generic_witness` 软断言收紧 · GA exec closeout meta 钉 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W152（ADR-496）→ **已交**

### Wave A W150 正确性地板加深（ADR-494 · 已交）

- **主轴：** A-03 Tcp native strict VM↔native product exec exact（TcpConnect refuse → `-1`，TcpHandleRelease → `true`；Native/WASM GA **Tcp 首片**；Path/Http/Sql/Tcp 四域 native exec 首片 **收官**）· A-05 矩阵指纹加深（`qs1-net-product-v0:tcp-native-wasm-exec-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `tcp_native_wasm_ga.rs`（native exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W150
- **非目标：** Tcp* `llvm=real` promotion · `driver_gate.rs` `lib.rs contains` 海量改写 · GA exec closeout meta 钉 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W151（ADR-495）→ **已交**

### Wave A W149 正确性地板加深（ADR-493 · 已交）

- **主轴：** A-03 Sql native strict VM↔native product exec exact（SqlOpen → `1`，SqlClose → `ok`；Native/WASM GA **Sql 首片**）· A-05 矩阵指纹加深（`qs1-sql-product-v0:sql-native-wasm-exec-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**（测内 `reset_rusqlite_store` 隔离）
- **证据：** `sql_native_wasm_ga.rs`（native exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W149
- **非目标：** Sql* `llvm=real` promotion · `driver_gate.rs` `lib.rs contains` 海量改写 · Tcp native exec 加深 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W150（ADR-494）→ **已交**

### Wave A W148 正确性地板加深（ADR-492 · 已交）

- **主轴：** A-03 Http native strict VM↔native product exec exact（HttpResponseStatus → `200`，HttpMakeResponse → `created`；Native/WASM GA **Http 首片**）· A-05 矩阵指纹加深（`qs1-http-product-v0:http-native-wasm-exec-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `http_native_wasm_ga.rs`（native exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W148
- **非目标：** Http* `llvm=real` promotion · `driver_gate.rs` `lib.rs contains` 海量改写 · Sql/Tcp native exec 加深 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W149（ADR-493）→ **已交**

### Wave A W147 正确性地板加深（ADR-491 · 已交）

- **主轴：** A-03 PathExists GA4 `llvm=real` promotion + native strict VM↔native product exec exact（exists true → `true`，false → `false`）· capability/inventory/matrix 同步（`host_fn_capabilities.toml` / `capabilities.toml` / `error_model_migration.toml` / `path_file_native_wasm_matrix.rs`）· A-05 矩阵指纹加深（`qs1-path-product-v0:path-exists-ga4`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm 变更**
- **证据：** `path_file_native_wasm_ga.rs` GA4 钉簇 · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W147
- **非目标：** File* `llvm=real` promotion · `driver_gate.rs` `lib.rs contains` 海量改写 · Http/Sql/Tcp native exec 加深 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· product `native=real` / `wasm=real` · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W148（ADR-492）→ **已交**

### Wave A W146 正确性地板加深（ADR-490 · 已交）

- **主轴：** A-03 Path/File native strict VM↔native product exec exact（PathJoin → `a/b/c`，PathExtension → `txt`；Native/WASM GA **首项**）· A-05 矩阵指纹加深（`qs1-path-product-v0:native-wasm-exec-first`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm/capability 变更**
- **证据：** `path_file_native_wasm_ga.rs`（native exec 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W146
- **非目标：** PathExists GA4 / `llvm=real` promotion · `driver_gate.rs` `lib.rs contains` 海量改写 · `parity_gate_files_exist` runtime exists 钉 · Http/Sql/Tcp native exec 加深 · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · parity/stdlib 变更 · Wave B
- **下一窗：** Wave A W147（ADR-491）→ **已交**

### Wave A W145 正确性地板加深（ADR-489 · 已交）

- **主轴：** A-03 parity/stdlib fixture **收官** `run_file_printed` product exact（`process_env.sy` → `true`，`udp_echo.sy` → `true`；parity/stdlib **14/14**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-stdlib-complete`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm 变更**
- **证据：** `driver_gate.rs`（parity exact 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W145
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · `parity_gate_files_exist` runtime exists 钉 · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W146（ADR-490）→ **已交**

### Wave A W144 正确性地板加深（ADR-488 · 已交）

- **主轴：** A-03 parity/stdlib fixture 扩展 `run_file_printed` product exact（`channel_send.sy` → `84`，`datetime_iso.sy` → `1000`；parity/stdlib **12/14**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-channel-datetime`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm 变更**（`datetime_iso.sy` 补 `import DateTime`）
- **证据：** `driver_gate.rs`（parity exact 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W144
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · `parity_gate_files_exist` runtime exists 钉 · parity/stdlib 末 2 fixture · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W145（ADR-489）→ **已交**

### Wave A W143 正确性地板加深（ADR-487 · 已交）

- **主轴：** A-03 parity/stdlib fixture 扩展 `run_file_printed` product exact（`array_sort.sy` → `1`/`2`/`3`，`mutex_lock.sy` → `locked`/`unlocked`；parity/stdlib **10/14**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-array-sync`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime/registry/vm 变更**（parser soft-keyword `select`/`timeout` + `sync/mod.sy` `__syx_value_compare` 使 `import Sync` 可编译）
- **证据：** `driver_gate.rs`（parity exact 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W143
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · `parity_gate_files_exist` runtime exists 钉 · parity/stdlib 其余 4 fixture · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W144（ADR-488）→ **已交**

### Wave A W142 正确性地板加深（ADR-486 · 已交）

- **主轴：** A-03 parity/stdlib fixture 扩展 `run_file_printed` product exact（`process_exec.sy` → `0`，`crypto_ed25519.sy` → `true`；parity/stdlib **8/14**）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-process-crypto`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime 变更**
- **证据：** `driver_gate.rs`（parity exact 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W142
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · `parity_gate_files_exist` runtime exists 钉 · parity/stdlib 其余 6 fixture · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W143（ADR-487）→ **已交**

### Wave A W141 正确性地板加深（ADR-485 · 已交）

- **主轴：** A-03 parity/stdlib fixture 扩展 `run_file_printed` product exact（`fmt_print.sy` → `parity-84`，`encoding_base64.sy` → `eA==`）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-extend`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime 变更**
- **证据：** `driver_gate.rs`（parity exact 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W141
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · `parity_gate_files_exist` runtime exists 钉 · parity/stdlib 其余 fixture · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W142（ADR-486）→ **已交**

### Wave A W140 正确性地板加深（ADR-484 · 已交）

- **主轴：** A-03 parity fixture `exists` → `run_file_printed` product exact（`reflect_self.sy` → Point fmt_debug 单行，`tcp_connect.sy` → `p85-tcp--1`；parity/stdlib 四件套收官）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-complete`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime 变更**
- **证据：** `driver_gate.rs`（parity exact 钉）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W140
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · parity/stdlib 其余 fixture · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W141（ADR-485）→ **已交**

### Wave A W139 正确性地板加深（ADR-483 · 已交）

- **主轴：** A-03 parity fixture `exists` → `run_file_printed` product exact（`hof_sort_closure.sy` → `1`/`3`，`string_builder.sy` → `p85-sb`；`reflect_self` / `tcp_connect` exists 钉未改）· A-05 矩阵指纹加深（`qs1-platform-gate-v0:parity-fixture-exact`；无新 HostFn 行）· A-02 **本窗跳过** · **无 runtime 变更**
- **证据：** `driver_gate.rs`（parity exact 钉）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W139
- **非目标：** `driver_gate.rs` `lib.rs contains` 海量改写 · Native/WASM GA inventory 深化 · HttpBodyStream close Display（否决）· Display 新钉 · A-02/SSA · capability · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W140（ADR-484）→ **已交**

### Wave A W138 正确性地板加深（ADR-482 · 已交）

- **主轴：** A-03 FileWatcher poll-after-close + close-after-close facade Display exact（W133 `uncaught_throw_message` 复用；**最小 registry** FileWatcher `open`/`poll`/`close` 走 facade 非 CallHost bypass，对齐 FileStream ADR-480 / FileHandle ADR-147）· A-05 矩阵指纹加深（`qs1-file-product-v0:display-filewatcher`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `file_stream_watcher.rs`（Display 钉新建）· `syx-stdlib` `registry.rs` FileWatcher TypeMethod · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W138
- **非目标：** FileWatcher open-missing Display（host mock 恒成功）· HttpBodyStream close Display（facade 非 throws + host 静默；否决）· host `__syx_file_watcher_*` Display · W72 `CLOSED_FAIL` 软钉改动 · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W139（ADR-483）→ **已交**

### Wave A W137 正确性地板加深（ADR-481 · 已交）

- **主轴：** A-03 Os hostname/primaryIpv4 force-err Display exact（W133 `uncaught_throw_message` 复用；**最小 runtime** `SYX_OS_HOSTNAME_FORCE_ERR` / `SYX_OS_PRIMARY_IPV4_FORCE_ERR` inject，对齐 W133–W134 配方）· A-05 矩阵指纹加深（`qs1-os-product-v0:display-hostname-ipv4`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `error_os_sentinel.rs`（Display 钉新建）· `syx-vm` `os_hosts.rs` inject · `packages/os/mod.sy` 注释 · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W137
- **非目标：** HttpBodyStream close Display（facade 非 throws + host 静默；否决）· FileWatcher Display · host `__syx_os_hostname` / `__syx_os_primary_ipv4` Display · `ok_or_throws` 成功路径改 exact · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W138（ADR-482）→ **已交**

### Wave A W136 正确性地板加深（ADR-480 · 已交）

- **主轴：** A-03 FileStream open-missing + read-after-close facade Display exact（W133 `uncaught_throw_message` 复用；**最小 registry** FileStream `open`/`readChunk`/`close` 走 facade 非 CallHost bypass，对齐 FileHandle ADR-147）· A-05 矩阵指纹加深（`qs1-cross-product-v0:display-filestream`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `file_stream_watcher.rs`（Display 钉新建）· `syx-stdlib` `registry.rs` FileStream TypeMethod · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W136
- **非目标：** HttpBodyStream close Display（facade 非 throws + host 静默；否决）· host `__syx_file_stream_open` Display · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W137（ADR-481）→ **已交**

### Wave A W135 正确性地板加深（ADR-479 · 已交）

- **主轴：** A-03 File open-missing + Sql bindPreparedOn conn-mismatch facade Display exact（W133 `uncaught_throw_message` 复用；**最小 VM** `try_handler_frames` 截断修复 facade `BeginTry` 泄漏）· A-05 矩阵指纹加深（`qs1-cross-product-v0:display-open-bindprep`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `error_io_file.rs` / `error_sql_conn.rs`（Display 钉新建）· `syx-vm` `try_handler_frames` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W135
- **非目标：** host `__syx_file_open` Display · HttpBodyStream close Display · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W136（ADR-480）→ **已交**

### Wave A W134 正确性地板加深（ADR-478 · 已交）

- **主轴：** A-03 Os memoryAvailable/loadAverage force-err Display exact（W133 `uncaught_throw_message` 复用，无新 runtime）· A-05 矩阵指纹加深（`qs1-os-product-v0:force-err-display-complete`；Os 四路 inject force-err Display **收官**；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `error_os_sentinel.rs`（Display 钉新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W134
- **非目标：** File open-missing Display · bindPrepared mismatch Display · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · HttpBodyStream close · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W135（ADR-479）→ **已交**

### Wave A W133 正确性地板加深（ADR-477 · 已交）

- **主轴：** A-03 Os uptimeMs/memoryTotal force-err Display exact（VM `uncaught_throw_message` 修复空 `[R007]`）· A-05 矩阵指纹加深（`qs1-os-product-v0:force-err-display`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `error_os_sentinel.rs`（Display 钉新建）· `syx-vm` `uncaught_throw_message` · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W133
- **非目标：** Os memoryAvailable/loadAverage force-err Display · open-missing / bindPrepared mismatch Display · ObjectGet 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · HttpBodyStream close · SqlRow/Orm 新钉 · wasm / llvm qs1 钉改动 · Wave B
- **下一窗：** Wave A W134（ADR-478）→ **已交**

### Wave A W132 正确性地板加深（ADR-476 · 已交）

- **主轴：** A-03 qs1 Sql native llvm `err.code` ObjectGet exact（llvm 五域 ObjectGet **收官**；wasm+llvm 双阶梯收官；W127 `mir-run` catch 复用，无新 runtime）· A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget-llvm-complete`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `hostfn_sql.rs`（native llvm 钉加深）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W132
- **非目标：** Os force-err Display · open-missing / bindPrepared mismatch Display · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · HttpBodyStream close · Display 新钉 · SqlRow/Orm 新钉 · wasm / 其它 llvm 钉改动 · Wave B
- **下一窗：** Wave A W133（ADR-477）→ **已交**

### Wave A W131 正确性地板加深（ADR-475 · 已交）

- **主轴：** A-03 qs1 Tcp+Process native llvm `err.code` ObjectGet exact（llvm 阶梯首格；W127 `mir-run` catch 复用，无新 runtime）· A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget-llvm-tcp-process`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `hostfn_tcp.rs` / `hostfn_process.rs`（native llvm 钉加深）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W131
- **非目标：** llvm Sql ObjectGet · Os force-err Display · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · Display 新钉 · SqlRow/Orm 新钉 · wasm 钉改动 · Wave B
- **下一窗：** Wave A W132（ADR-476）→ **已交**

### Wave A W130 正确性地板加深（ADR-474 · 已交）

- **主轴：** A-03 qs1 Sql wasm AOT `err.code` ObjectGet exact（wasm 五域 ObjectGet **收官**；W127 `mir-run` catch 复用，无新 runtime）+ Http connect-refused host Display exact · A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget-wasm-complete`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `hostfn_sql.rs` / `http_client_timeout_tls.rs`（钉加深/新建）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W130
- **非目标：** llvm ObjectGet 批量 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · Os force-err Display · Display 新钉 · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W131（ADR-475）→ **已交**

### Wave A W129 正确性地板加深（ADR-473 · 已交）

- **主轴：** A-03 qs1 Tcp+Process wasm AOT `err.code` ObjectGet exact（W127 `mir-run` catch 复用，无新 runtime）· A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget-wasm-tcp-process`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `hostfn_tcp.rs` / `hostfn_process.rs`（wasm 钉加深）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W129
- **非目标：** wasm Sql ObjectGet · llvm 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · Http refuse Display · Os force-err Display · Display 新钉 · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W130（ADR-474）→ **已交**

### Wave A W128 正确性地板加深（ADR-472 · 已交）

- **主轴：** A-03 qs1 File+Http wasm AOT `err.code` ObjectGet exact（对称 W127 llvm；W127 `mir-run` catch 复用，无新 runtime）· A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget-wasm`；无新 HostFn 行）· A-02 **本窗跳过**
- **证据：** `hostfn_file.rs` / `hostfn_http.rs`（wasm 钉加深）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W128
- **非目标：** wasm Tcp/Process/Sql ObjectGet 批量 · llvm 钉改动 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · Display 新钉 · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W129（ADR-473）→ **已交**

### Wave A W127 正确性地板加深（ADR-471 · 已交）

- **主轴：** runtime native llvm catch 物化（`mir-run` structured host catch + llvm `DictBuild`→`syx_rt_dict_build`）· A-03 qs1 File/Http native llvm `err.code` ObjectGet exact · A-05 矩阵指纹加深（`qs1-cross-product-v0:catch-code-objectget`；无新 HostFn/wasm 格）· A-02 **本窗跳过**
- **证据：** `hostfn_file.rs` / `hostfn_http.rs`（native llvm 钉加深）· `mir-run` / `syx-llvm` runtime · `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W127
- **非目标：** wasm 五域 ObjectGet 批量 · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · Display 新钉 · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W128（ADR-472）→ **已交**

### Wave A W126 正确性地板加深（ADR-470 · 已交）

- **主轴：** A-03 exact（HttpBodyStream openWithOptions timeout + TLS fail-closed Display）· A-05 矩阵指纹加深（`qs1-http-product-v0:bodystream-timeout-tls`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `http_client_timeout_tls.rs`（`__syx_http_body_stream_open_opts`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W126
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W127（ADR-471）→ **已交**

### Wave A W125 正确性地板加深（ADR-469 · 已交）

- **主轴：** A-03 exact（HttpBodyStream openWithOptions invalid-method + options-not-object Display）· A-05 矩阵指纹加深（`qs1-http-product-v0:bodystream-open-opts`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `http_body_stream.rs`（`__syx_http_body_stream_open_opts`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W125
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · SqlRow/Orm 新钉 · Wave B
- **下一窗：** Wave A W126（ADR-470）→ **已交**

### Wave A W124 正确性地板加深（ADR-468 · 已交）

- **主轴：** A-03 exact（Orm updateWhereOn + deleteWhereOn unknown-conn Display）· A-05 矩阵指纹加深（`qs1-orm-product-v0:update-delete-unknown`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_orm_conn.rs`（`__syx_orm_update_where_on` / `__syx_orm_delete_where_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W124
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · bindPrepared mismatch · HttpBodyStream close · SqlRow/Http 新钉 · Wave B
- **下一窗：** Wave A W125（ADR-469）→ **已交**

### Wave A W123 正确性地板加深（ADR-467 · 已交）

- **主轴：** A-03 exact（Orm saveOn record-not-object + no-columns Display）· A-05 矩阵指纹加深（`qs1-orm-product-v0:saveon-param`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_orm_conn.rs`（`__syx_orm_save_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W123
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm update/delete unknown-conn · bindPrepared mismatch · HttpBodyStream close · SqlRow/Http 新钉 · Wave B
- **下一窗：** Wave A W124（ADR-468）→ **已交**

### Wave A W122 正确性地板加深（ADR-466 · 已交）

- **主轴：** A-03 exact（Orm saveOn + findWhereOn unknown-conn Display）· A-05 矩阵指纹加深（`qs1-orm-product-v0:conn-unknown`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `error_orm_conn.rs`（`__syx_orm_save_on` / `__syx_orm_find_where_on`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W122
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm update/delete soft 改钉 · bindPrepared mismatch · HttpBodyStream close · SqlRow/Http 新钉 · Wave B
- **下一窗：** Wave A W123（ADR-467）→ **已交**


### Wave A W121 正确性地板加深（ADR-465 · 已交）

- **主轴：** A-03 exact（SqlRow getFloat + getBlob type-mismatch Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:sqlrow-float-blob`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `sql_typed_row.rs`（`__syx_sql_row_get_float` / `__syx_sql_row_get_blob`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W121
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm soft · bindPrepared mismatch · HttpBodyStream close · Http 新钉 · SqlRow 其它新门槛 · Wave B
- **下一窗：** Wave A W122（ADR-466）→ **已交**


### Wave A W120 正确性地板加深（ADR-464 · 已交）

- **主轴：** A-03 exact（Http request invalid-method + options-not-object Display）· A-05 矩阵指纹加深（`qs1-http-product-v0:request-method-options`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `http_client_timeout_tls.rs`（`__syx_http_request`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W120
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm soft · bindPrepared mismatch · HttpBodyStream close · SqlRow 新钉 · 网络 loopback · Wave B
- **下一窗：** Wave A W121（ADR-465）→ **已交**


### Wave A W119 正确性地板加深（ADR-463 · 已交）

- **主轴：** A-03 exact（SqlRow `__type=SqlRow` + getString type-mismatch Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:sqlrow-type-tag-string`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `sql_typed_row.rs`（`__syx_json_decode` + `__syx_sql_row_get_string`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W119
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm soft · bindPrepared mismatch · HttpBodyStream · Invalid HTTP method · getFloat/getBlob mismatch · Wave B
- **下一窗：** Wave A W120（ADR-464）→ **已交**


### Wave A W118 正确性地板加深（ADR-462 · 已交）

- **主轴：** A-03 exact（SqlRow missing-column + getInt type-mismatch Display）· A-05 矩阵指纹加深（`qs1-sql-product-v0:sqlrow-missing-mismatch`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `sql_typed_row.rs`（`__syx_sql_row_get_string` / `__syx_sql_row_get_int`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W118
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm soft · bindPrepared mismatch · HttpBodyStream · Invalid HTTP method · SqlRow `__type=SqlRow` · Wave B
- **下一窗：** Wave A W119（ADR-463）→ **已交**


### Wave A W117 正确性地板加深（ADR-461 · 已交）

- **主轴：** A-03 exact（HttpBodyStream readChunk invalid-handle + SqlRow accessor requires-object Display）· A-05 矩阵指纹加深（`qs1-io-product-v0:httpbody-sqlrow-invalid`；无新 HostFn/wasm 格）· A-02 **本窗跳过** · IOError.code ObjectGet **本窗跳过**（runtime 债）
- **证据：** `http_body_stream.rs`（`__syx_http_body_stream_read_chunk`）· `sql_typed_row.rs`（`__syx_sql_row_get_string`）· `hostfn_qs1_matrix.toml` · `ga-evidence-audit.md` W117
- **非目标：** ObjectGet · A-02/SSA · capability · fingerprint 海量 · hostname · without_close · http_server 动态 port · Codable · open-missing · Orm soft · HttpBodyStream close · Invalid HTTP method · Wave B
- **下一窗：** Wave A W118（ADR-462）→ **已交**


### Phase 138 能力摘要（已交付）

- **Gate0（已交付）：** P137 诚实收口；audit **2600/3000**（计划 2750，实测诚实下调）；manifest 116.0/phase 138；ADR-130 + P75；EXCLUDE `p137_behavioral` / gate0 fill；**暂缓 SynUI**；**冻结 Farm/Dist v19**
- **Class Properties v6（部分 GA）：** `static var` willSet/didSet MVP；P137 v5 继承；**完整 Swift 属性模型 → 139+**
- **Macro M28 + Service DI v19（部分 GA）：** multi-token decl rewrite；`within` wildcard pointcut；M27 继承；**完整 token-stream / AspectJ / Spring → 139+**
- **GUI：** SynUI v12 继承 — **本阶段暂缓**；**非** SynUI v13 → **139+**
- **Concurrency（部分 GA）：** Actor shared pool MVP；DiscardOldest 等继承；**非** Java ThreadPoolExecutor 全集 → **139+**
- **Wasm SIMD guest（部分 GA）：** **SIMD16 i8x16 bitmask** MVP；popcnt 等继承；SIMD16 全集 → **139+**
- **Farm / Dist：** **冻结 v19**（hsm-key-id / peer-announce-hold）；本阶段无新表面
- **Os（部分 GA）：** primaryIpv4 MVP；disk/load 继承；**非**完整 sysinfo → **139+**
- **DX：** audit **2600/3000**（计划 2750，实测诚实下调）；`driver_gate` CR≥420；`lsp_phase138_real` ≥56；`syx-cache-v110`

### Phase 137 能力摘要（已交付）

- **Gate0（已交付）：** P136 诚实收口；audit **2600/2950**；manifest 115.0/phase 137；ADR-129 + P74；EXCLUDE `p136_behavioral` / gate0 fill；**暂缓 SynUI**；**冻结 Farm/Dist v19**
- **Class Properties v5（部分 GA）：** `@Published` class stored MVP；P136 v4 继承；**完整 Swift 属性模型 → 138+**
- **Macro M27 + Service DI v18（部分 GA）：** token decl-expand；`!within` pointcut；M26 继承；**完整 token-stream / AspectJ / Spring → 138+**
- **GUI：** SynUI v12 继承 — **本阶段暂缓**；**非** SynUI v13 → **138+**
- **Concurrency（部分 GA）：** Task DiscardOldest MVP；core/max/keepAlive 继承；**非** Java ThreadPoolExecutor 全集 → **138+**
- **Wasm SIMD guest（部分 GA）：** **SIMD16 i8x16 popcnt** MVP；avgr_u 等继承；SIMD16 全集 → **138+**
- **Farm / Dist：** **冻结 v19**（hsm-key-id / peer-announce-hold）；本阶段无新表面
- **Os（部分 GA）：** disk/load MVP；hostname/uptime 继承；**非**完整 sysinfo → **138+**
- **DX：** audit **2600/2950**；`driver_gate` CR≥410；`lsp_phase137_real` ≥54；`syx-cache-v109`

### Phase 136 能力摘要（已交付）

- **Gate0（已交付）：** P135 诚实收口；audit **2500/2900**；manifest 114.0/phase 136；ADR-128 + P73；EXCLUDE `p135_behavioral` / gate0 fill；功能向测试套件（已清理 Gate0 behavioral 垫数）；**暂缓 SynUI**（无 v13）
- **Class Properties v4（部分 GA）：** extension computed `{ get }` / `{ get set }` MVP；P135 v3 继承；**完整 Swift 属性模型 → 137+**
- **Macro M26 + Service DI v17（部分 GA）：** token-stream 窄切片；`within || args` pointcut compose；M25 继承；**完整 token-stream / AspectJ / Spring → 137+**
- **GUI：** SynUI v12 继承 — **本阶段暂缓**深化；**非** SynUI v13 / 全控件库 / Web/Mobile **SYX language** 全集 GA → **137+**
- **Concurrency（部分 GA）：** Task core/max/keepAlive MVP；P135 CallerRuns + P134 Abort 继承；**非** Java ThreadPoolExecutor 全集 → **137+**
- **Wasm SIMD guest（部分 GA）：** **SIMD16 i8x16 avgr_u** MVP；ge / le / ne / neg / abs 等继承；SIMD16 全集 → **137+**
- **Farm / Dist / Os（部分 GA）：** Farm hsm-key-id（CKA_ID 形 stub）；Dist peer-announce-hold；Os hostname/uptime；**非**真 HSM / 运营商 BGP / 完整 sysinfo → **137+**
- **DX：** audit **2500/2900**（垫数清理后诚实重标定）；`driver_gate` CR≥400；`lsp_phase136_real` ≥52；`syx-cache-v108`

### Phase 135 能力摘要（部分 GA 已交付）

- **Gate0（已交付）：** P134 诚实收口；audit **6650/3650**；manifest 113.0/phase 135；ADR-127 + P72；EXCLUDE `p134_behavioral` / gate0 fill；`p135_behavioral` 真实地板
- **Class Properties v3（部分 GA）：** super.init 字段回读修复；computed `{ get set }` MVP；property `override` MVP；P134 v2 继承；**完整 Swift 属性模型 → 137+**（原 136+ 债务随阶段推进）
- **Macro M25 + Service DI v16（部分 GA）：** decl-token subst MVP；`within && args` pointcut compose MVP；M24 继承；**完整 token 语法宏 / AspectJ / Spring → 137+**
- **GUI：** **SynUI v12（部分 GA）** — navigation · nav Signal bind · Web/Mobile 表面加深；v11 继承；本阶段继承、不再深化 → **137+**
- **Concurrency（部分 GA）：** Task 队列 CallerRuns 拒绝 MVP；P134 有界 Abort 继承；**非** Java ThreadPoolExecutor 全集 → **137+**
- **Wasm SIMD guest：** **SIMD16 i8x16 ge** MVP（部分 GA）；le / ne / neg / abs 等继承；SIMD16 全集 → **137+**
- **Farm / Dist（部分 GA）：** Farm hsm-key-label；Dist peer-announce-priority；**非**真 HSM / 运营商 BGP → **137+**
- **DX：** audit **6650/3650**；`driver_gate` CR≥410；`lsp_phase135_real` ≥50；`syx-cache-v107`

### Phase 134 能力摘要（部分 GA 已交付）

- **Gate0（已交付）：** P133 诚实收口；audit **6500/3600**；manifest 112.0/phase 134；ADR-126 + P71；EXCLUDE `p133_behavioral` / gate0 fill；`p134_behavioral` 真实地板
- **Class Properties v2（部分 GA）：** 继承观察器；init 抑制；computed get-only MVP；static/lazy MVP；**完整 Swift 属性模型 → 135+**
- **Macro M24 + Service DI v15（部分 GA）：** 表达式位 `#macro` token subst MVP；`@Pointcut("args(...)")` 名模式 MVP；M23 继承；**完整 token 语法宏 / AspectJ / Spring → 135+**
- **GUI：** **SynUI v11（部分 GA）** — tabView · tab Signal bind · Web/Mobile 控件加深；v10 继承；**非**全控件库 / Web/Mobile **SYX language** 全集 GA → **135+**
- **Concurrency（部分 GA）：** 有界 Task 队列 Abort 拒绝 MVP；P133 worker/maxActors/mailbox 继承；**非** Java ThreadPoolExecutor 全集 → **135+**
- **Wasm SIMD guest：** **SIMD16 i8x16 le** MVP（部分 GA）；ne / neg / abs 等继承；SIMD16 全集 → **135+**
- **Farm / Dist（部分 GA）：** Farm hsm-session-pin；Dist peer-announce-ttl；**非**真 HSM / 运营商 BGP → **135+**
- **DX：** audit **6500/3600**；`driver_gate` CR≥400；`lsp_phase134_real` ≥48；`syx-cache-v106`

### Phase 133 能力摘要（部分 GA 已交付）
- **Macro M23 + Service DI v14（部分 GA）：** 语句位 `#macro` token subst MVP（`macro-token-v4:stmt-token-subst-mvp`）；`@Pointcut("within(Type)")` 类型名模式 MVP（`macro-service-v14:pointcut-within-name-mvp`）；M22 继承（decl-hygiene · Before/After）；文档指纹 `macro-docs-v15:m23-service-v14-honesty`；**完整 token 语法宏 / 完整 AspectJ / 完整 Spring → 134+**
- **GUI：** **SynUI v10（部分 GA）** — stack · scrollView Signal bind · Web/Mobile 布局加深；v9 继承；**非**全控件库 / Web/Mobile **SYX language** 全集 GA → **134+**
- **Os / Concurrency（部分 GA）：** `packages/os` cpu/memory/user 运行时查询 MVP；Task worker 可配；Actor max-threads + mailbox 有界；**非** Java ThreadPoolExecutor 全集 → **134+**
- **Wasm SIMD guest：** **SIMD16 i8x16 ne** MVP（部分 GA）；neg / abs / umin/umax / load/store / shuffle / sat / compare / extract-replace 继承；SIMD16 全集 → **134+**
- **Farm / Dist（部分 GA）：** Farm hsm-multi-slot；Dist peer-announce-epoch-persist；**非**真 HSM / 运营商 BGP → **134+**
- **DX（Gate0）：** audit **6350/3550**；`driver_gate` CR≥390；`lsp_phase133_real` ≥46；`syx-cache-v105`

### Phase 132 能力摘要（部分 GA 已交付）

- **Macro M22 + Service DI v13（部分 GA）：** 声明位 `#macro` 卫生 MVP + linked/run_file 收口；`@Before`/`@After` pointcut 字符串绑定 MVP；M21 继承（decl-param · execution 名模式）；文档指纹 `macro-docs-v14:m22-service-v13-honesty`；**完整 token 语法宏 / 完整 AspectJ / 完整 Spring → 134+**
- **GUI：** **SynUI v9（部分 GA）** — image Signal 绑定；divider；Web/Mobile 控件加深；v8 继承（textArea/list bind · spacer · Web/Mobile 表面）；**非**全控件库 / Web/Mobile **SYX language** 全集 GA → **133+**
- **Wasm SIMD guest：** **SIMD16 i8x16 neg** MVP（部分 GA）；abs / umin/umax / load/store / shuffle / sat / compare / extract-replace 继承；SIMD16 全集 → **133+**
- **Farm / Dist：** Farm hsm-digest-streaming（DigestInit/Update/Final 流式联验，部分 GA）；Dist peer-announce-conflict-epoch；**非**真 HSM / 运营商 BGP → **133+**
- **DX：** audit **6200/3500**；`driver_gate` CR≥380；`lsp_phase132_real` ≥44 semantic matrix shipped；`syx-cache-v104`

### Phase 131 能力摘要（继承）

- **Macro M21 + Service DI v12（部分 GA）：** 声明位 `#macro` 参数替换 MVP；`execution(* T.m(..))` 方法名/`*` 模式 MVP；M20 继承（token expr · `@Pointcut` 方法名）；文档指纹 `macro-docs-v13:m21-service-v12-honesty`；**完整 token 语法宏 / 完整 AspectJ / 完整 Spring → 133+**
- **GUI：** **SynUI v8（部分 GA）** — textArea/list Signal 绑定；spacer；Web/Mobile 表面加深；v7 继承（image/scrollView · picker bind · Web language 表面）；**非**全控件库 / Web/Mobile **SYX language** 全集 GA → **133+**
- **Wasm SIMD guest：** **SIMD16 i8x16 abs** MVP（部分 GA）；umin/umax / load/store / shuffle / sat / compare / extract-replace 继承；SIMD16 全集 → **133+**
- **Farm / Dist：** Farm hsm-digest-bound（`C_Digest`+hsm-ref 联验，部分 GA）；Dist peer-announce-fanout；**非**真 HSM / 运营商 BGP → **133+**
- **DX：** audit **6050/3450**；`driver_gate` CR≥370；`lsp_phase131_real` ≥42 semantic matrix shipped；`syx-cache-v103`

### Phase 130 能力摘要（继承）

- **Macro M20 + Service DI v11（部分 GA）：** token 表达式位 `#macro` MVP；`@Pointcut` 方法名 MVP；M19 继承（Codable L2 · Bean 参数注入）；文档指纹 `macro-docs-v12:m20-service-v11-honesty`；**完整 token 语法宏 / 完整 AspectJ / 完整 Spring → 132+**
- **GUI：** **SynUI v7（部分 GA）** — image/scrollView；picker Signal 绑定；Web language 表面 MVP；v6 继承（textArea/list · progress bind · desktop layout）；**非**全控件库 / Web/Mobile **SYX language** 全集 GA → **132+**
- **Wasm SIMD guest：** **SIMD16 i8x16 umin/umax** MVP（部分 GA）；load/store / shuffle / sat / compare / extract-replace 继承；SIMD16 全集 → **132+**
- **Farm / Dist：** Farm hsm-sign-verify（`C_Verify`+hsm-ref 联验，部分 GA）；Dist peer-announce-ack；**非**真 HSM / 运营商 BGP → **132+**
- **DX：** audit **5900/3400**；`driver_gate` CR≥360；`lsp_phase130_real` ≥40 semantic matrix shipped；`syx-cache-v102`

### Phase 129 能力摘要（继承）

- **Macro M19 + Service DI v10：** `@Codable` L2 associated-value enum 全自动合成；`@Bean` 参数 `@Autowired` MVP；M18 继承（`@Lazy` · cycle-proxy）；文档指纹 `macro-docs-v11:m19-service-v10-honesty`
- **GUI：** SynUI v6 partial — textArea/list；progress Number Signal 绑定；desktop run 布局 GA
- **Wasm / Farm / Dist：** SIMD16 load/store；Farm hsm-sign-bound；Dist peer-announce sticky
- **DX：** audit **5750/3350**；`syx-cache-v101`

状态档位：**已实现** · **部分实现** · **计划中** · **未实现** · **GA**（已实现且有硬闸/parity）

## 词法与基础

| 特性 | 状态 | 说明 |
|------|------|------|
| 标识符、关键字 | **已实现** | `syx-lexer` |
| 整型/浮点字面量与后缀 | **已实现** | `42i8`、`3.14f32` |
| 字符串字面量与插值 | **已实现** | `"Hello, \(name)!"` |
| 正则字面量 | **已实现** | `/pattern/` |
| 块注释嵌套 | **已实现** | `/* … */` |
| 行注释 | **已实现** | `//` |

## 类型系统

| 特性 | 状态 | 说明 |
|------|------|------|
| 固定宽度整数 Int8–Int64、UInt8–UInt64 | **已实现** | |
| Int128 / UInt128 | **已实现** | Phase 38 · `42i128` / `0xFFu128` |
| SIMD2/4/8/16\<T\> | **已实现** | 语言面 GA；WASM guest-v11 零 scratch；SIMD16 extract/replace MVP（Phase 125）；Min/Max 继承 |
| 平台 Int / UInt | **已实现** | 64 位 host → Int64 |
| Float32/Float64、Float/Double | **已实现** | |
| Bool、String、Void | **已实现** | |
| 内置类型可读 facade（Core Authority） | **已实现** | Phase 44+ · `packages/core/builtins/**` + `@builtin` |
| 数值 static Host（`__syx_i8_min` 等） | **已实现** | Phase 45 · per-type Host + manifest `host_ops` |
| 协议 witness Core 化（hash/description/debugDescription） | **已实现** | Phase 46 · extension-only → Host |
| 集合 HOF Host（`__syx_array_map` 等） | **已实现** | Phase 46 · Core extension |
| BigInt MVP | **已实现** | Phase 46 · `__syx_bigint_*` + Core |
| Result 代数（final class） | **已实现** | Phase 46+ · static ok/err/isOk |
| `Array<T>` 强类型（禁止裸 Array） | **已实现** | Phase 60 · IO 字节流 `Array<UInt8>` |
| `Dictionary<K,V> where K: Hashable` | **已实现** | Phase 60 · 键 witness hash+eq |
| `_Builtin` / manifest v2 | **已实现** | Phase 44–45 · `entries[]` + kind / host_ops / auto_conformance |
| Optional `T?`、nil | **已实现** | |
| `Array<T>`、Set、Dictionary、Object | **已实现** | 裸 `Array` 已删除 |
| Result 泛型 | **已实现** | Phase 21–23 |
| 引用类型 `&T` / `&mut T` | **部分实现** | borrow regions v8 · **SE-0461 默认 on（Phase 72）** |
| 生命周期 `'a` | **部分实现** | S017 |
| `some Protocol` / `any Protocol` | **已实现** | witness 表 |
| 类型嵌套 | **部分实现** | 有限 parser 支持 |
| 类型别名 `typealias` | **已实现** | Phase 40 GA 默认 on |

## 字符串

| 特性 | 状态 | 说明 |
|------|------|------|
| UTF-8 存储 | **已实现** | Rust String 后端 |
| `.length`（字素簇计数） | **已实现** | Phase 84 · grapheme；ADR-226 三视图默认字符模型 |
| `for-in` 遍历 | **已实现** | ADR-226：元素为 grapheme `Character`（≠ 标量） |
| Unicode scalar 视图 | **已实现** | ADR-226 · `unicodeScalarCount` / `unicodeScalars` |
| UTF-8 bytes 视图 | **已实现** | ADR-226 · `utf8Count` / `utf8Bytes`（`Array<Int>`） |
| Character 类型 | **部分实现** | P139 distinct + ADR-226 `isSingleScalar` 按 scalar；≠ Index GA |
| `s[i]` 下标 | **已实现** | Int grapheme offset；**O(n)** 随机访问（ADR-227 / `M2-S2-02` 已钉；≠ opaque Index） |
| split/trim/substring 等方法 | **已实现** | `STRING_METHODS`；substring 为 grapheme 索引 |

## 集合

| 特性 | 状态 | 说明 |
|------|------|------|
| 数组字面量 `[1,2,3]` | **已实现** | |
| `arr[i]` 下标 | **已实现** | `ArrayGet` |
| Set、Dictionary | **已实现** | |
| map/filter/reduce HOF | **已实现** | 闭包参数 |
| 用户自定义 Sequence/Iterator | **已实现** | Phase 71 · custom class for-in 三端 strict（VM / unified MIR / WASM） |

## 控制流

| 特性 | 状态 | 说明 |
|------|------|------|
| if / else（条件括号可选） | **已实现** | |
| while / repeat while（条件括号可选） | **已实现** | |
| for-in | **已实现** | Array、String、Range |
| for await in | **已实现** | async sequence |
| break / continue | **已实现** | |
| switch / case / default | **已实现** | |
| switch where 守卫 | **已实现** | |
| if case / if let / guard let | **已实现** | |

## 函数与闭包

| 特性 | 状态 | 说明 |
|------|------|------|
| func 声明 | **已实现** | |
| 命名参数 | **已实现** | |
| throws / try/catch | **已实现** | |
| requires Pure / IO | **部分实现** | MVP |
| 闭包 `{ x in … }` | **已实现** | 捕获 |
| async func / await | **已实现** | |
| async let | **已实现** | desugar detach+join |

## 类型声明

| 特性 | 状态 | 说明 |
|------|------|------|
| struct | **已实现** | |
| class | **已实现** | 引用语义 MVP |
| data class | **部分实现** | `@Equatable`/`@Hashable`/`@Codable` L1（Phase 50–52）；无 Swift 式 copy |
| final class | **已实现** | Phase 60 · 替代 `sealed`；exhaustiveness MVP |
| enum（含关联值） | **已实现** | |
| actor | **已实现** | |
| protocol | **部分实现** | witness 分发 |
| extension | **部分实现** | 方法为主 |
| global actor | **已实现** | |
| init | **已实现** | |
| deinit | **部分实现** | P103 weak ARC deinit MVP；struct deinit / reflect witness 债务 P104 |
| 继承 | **GA** | `class Derived: Base` + 必填 `override`；vtable MVP + `super`（S051–S055） |
| 计算属性 | **部分实现** | extension 有限 |
| 下标 subscript 声明 | **部分实现** | Phase 40/41 get/set MVP |
| 属性观察器 willSet/didSet | **GA** | stored property hooks；`oldValue`/`newValue` Phase 65 |
| 自定义运算符 `operator func` | **GA** | 内置重载 hard error S041–S048；三后端 parity |

## 泛型与协议

| 特性 | 状态 | 说明 |
|------|------|------|
| 函数/类型泛型 | **已实现** | 单态化 |
| where 子句 | **已实现** | |
| 协议关联类型 | **部分实现** | S012 |
| 协议默认实现 | **已实现** | |
| Equatable/Hashable 等内置协议 | **部分实现** | 内置类型自动；用户 `data class` + `@Equatable`/`@Hashable` L1 |
| Comparable 协议 | **GA** | Phase 69 · `compare(other: Self) -> Int` + S057 + witness sort codegen + LLVM ArrSorted |

## 并发

| 特性 | 状态 | 说明 |
|------|------|------|
| Task / Task.detached | **已实现** | Phase 133 · `Concurrency.setWorkerCount` / `workerCount` 可配 worker 数 MVP |
| TaskGroup | **已实现** | |
| actor 隔离 | **已实现** | Phase 133 · `Concurrency.setMaxActors` / `maxActors` + `setMailboxCapacity` / `mailboxCapacity` 有界 mailbox MVP |
| GlobalActor / @MainActor | **已实现** | |
| Sendable / sending | **已实现** | Phase 60 · strict 模式 **C003** E2E（`concurrency.rs`） |
| channel / select | **已实现** | |
| distributed actor | **部分实现** | MVP spike |

## 宏

| 特性 | 状态 | 说明 |
|------|------|------|
| `#assert` `#if` `#for` 等 freestanding | **GA** | `syx-macro` builtin；角色错位 → **E003**（M9 继承） |
| `#stringify(expr)` / `#compileTime` | **GA** | builtin pass 硬化（M9 继承） |
| 用户 `#macro` + 卫生 v2/v3/v4 | **部分实现** | Phase 40 MVP + Phase 50 `MacroParamId`；Phase 130 表达式位 token/ident/literal MVP；Phase 131 声明位参数替换 MVP（`macro-token-v2:decl-param-subst-mvp`）；Phase 132 声明位卫生 MVP（`macro-token-v3:decl-hygiene-mvp`）；Phase 133 语句位 token subst MVP（`macro-token-v4:stmt-token-subst-mvp`）；完整 token 语法宏 **134+** |
| `@attached` 真 AST | **部分实现** | Phase 50 `parse_fragment`；conformance 未知协议不得静默忽略（M9） |
| `@Equatable` / `@Hashable` / `@Codable` L1 | **GA** | stored `data class` only |
| `@Codable` L2（enum + generic witness） | **部分实现** | Phase 129 associated-value enum 全自动合成 **GA**；泛型关联值全集 / 任意嵌套 / coding keys DSL **131+** |
| `#externalMacro` plugin-prod | **部分实现** | cdylib JSON 生产路径；Linux seccomp / Darwin reexec-only 诚实标签 |
| `@propertyWrapper` | **已实现** | Phase 40 GA |
| `@resultBuilder` | **GA** | engine 固定点管线第六 pass；`buildOptional` + multi-stmt |
| 宏包 `[macros].exports` | **GA** | Phase 120 schema 与 `SyxManifest` 对齐 |
| `@Service` / `@Component` + `@Inject` ctor + 字段 `@Autowired` | **GA** | Phase 125 Service DI v6：多包扫描 + 真实 ctor/field inject · 基类型/协议型多态 inject · 字段 `@Qualifier` 元数据 · `@Primary`；M14 Qualifier/Primary/Profile · `Di.withRequest` · request/prototype scope 继承 |
| `@Around` 方法拦截 MVP | **部分实现** | 仅 `@Service`/`@Component` 实例方法 delegate 包装 + 窄 hook；可与 `@Pointcut` 方法名绑定（指纹 `macro-service-v11:pointcut-name-mvp`）；Phase 132 `@Before`/`@After` pointcut 字符串绑定（`macro-service-v13:before-after-pointcut-bind-mvp`）；Phase 133 `@Pointcut("within(Type)")` 类型名模式（`macro-service-v14:pointcut-within-name-mvp`）；**非**完整 AspectJ / 切面类 |
| `@Pointcut` 方法名 MVP | **部分实现** | Phase 130 `@Pointcut("Type.method")` 窄字符串模式（指纹 `macro-service-v11:pointcut-name-mvp`）；Phase 131 `execution(* T.m(..))` 名模式（`macro-service-v12:pointcut-execution-name-mvp`）；Phase 133 `within(Type)` 类型名模式（`macro-service-v14:pointcut-within-name-mvp`）；完整 AspectJ → **134+** |
| 完整 token 语法宏 / 完整 AspectJ AOP / 完整 Spring | **未实现** | Phase 133 已交付 stmt-token MVP + within(Type) MVP + M22 继承；完整面 → **134+** |

## 运行时扩展（Host 插件）

| 特性 | 状态 | 说明 |
|------|------|------|
| 静态 `HostFnId` 注册 | **已实现** | 维护者路径；见 `plan/topics/hostfn-extension.md` |
| dlopen 插件（opt-in） | **部分实现** | Phase 49+ `syx-host-plugin`、`SYX_HOST_PLUGIN_PATH` |
| 编译期 `plugin.json` + lock | **部分实现** | `syx-plugin-lock.json`、H002 |
| ed25519 插件验签 | **部分实现** | Phase 50–51 `SYX_HOST_PLUGIN_VERIFY` |
| Embed 插件路径 | **部分实现** | Phase 51 `syx_embed_v2_set_plugin_paths` |

## 内存与安全

| 特性 | 状态 | 说明 |
|------|------|------|
| ARC | **已实现** | Phase 73 · class `ObjectRef` retain/release；VM + WASM tier-2D + Native LLVM 直发 |
| GC（VM） | **已实现** | Phase 73 · mark-sweep `GcCollect` + `collect_global` 对齐 |
| GC（WASM/native 统一） | **已实现** | Phase 75 · ManagedHeap v4 compact + HeapSlot Array/String 统一；mark-sweep + slide compact |
| Native LLVM exec_op | **已实现** | Phase 74 · tier-3 直发；`llvm_ir_exec_op_count==0` CI hard |
| Native inkwell IR | **已实现** | Phase 75 · inkwell default on；`SYX_LLVM_TEXT_EMIT=1` opt-in |
| Native multi-module link | **已实现** | Phase 75 · `build_native_executable` deps project GA |
| 跨模块 Actor | **已实现** | Phase 74 · `public actor` 导出 + actor_cluster v2 四端 stdout hard |
| 借用检查 | **部分实现** | Phase 74 · SE-0461 + async/actor 子集；`borrow_phase74_heap` compile-fail fixtures |
| 工具链 `target/cache` 增量编译 | **已实现** | `syx-cache-v105`（Phase 133；相对 v104 全量失效） |
| Registry lock v11 | **已实现** | Phase 77 · plugin_unload_epoch + marketplace_cdn_base + farm_queue_url |
| Heap v6 host materialize | **已实现** | Phase 77 · VM ArraySet/Dict + `materialize_array`/`materialize_dict` |
| inkwell llvm Module | **已实现** | Phase 77 · `emit_inkwell_llvm_module` + `llvm_inkwell_phase77_llvm_ready` |
| RT full-tier direct | **已实现** | Phase 77 · `rt_direct_exec` stack ops; main fixtures exec==0 |
| build.syb farm queue | **已实现** | Phase 77 · `farm_queue` target + `SYX_BUILD_FARM_QUEUE` |
| 泛型特化反射 | **部分实现** | Phase 77 · `reflect_specialization` MVP |
| Borrow 机器证明义务 | **部分实现** | Phase 77 · obligation JSON export + replay CI |
| Registry lock v10 | **已实现** | Phase 76 · multi-region CDN + cache_remote_nodes |
| 分布式 cache push | **已实现** | Phase 76 · `push_cache_remote` + `SYX_CACHE_REMOTE_NODES` |
| VM HeapSlot parity | **已实现** | Phase 76 · VM ArrayBuild/HeapCompact slot-backed |
| inkwell crate IR | **已实现** | Phase 76 · `emit_inkwell_crate_module` + optional `inkwell` dep |
| Crypto（`import Crypto`） | **已实现** | Phase 38 · SHA-256/512、HMAC；embed sandbox S021 |
| 访问控制 public/internal/private/fileprivate | **已实现** | S005 跨模块；fileprivate 同文件 |
| `build.syb` Task DAG 执行器 | **部分实现** | Phase 65 · mir/wasm/native/link/test runner MVP；Phase 66 LSP cycle/unknown 诊断 |
| WASM AOT tier-1 直发 | **已实现** | Phase 65–69 · hot_loop ≥95%；strict matrix ≥99% |
| WASM AOT tier-2A guest 直发 | **已实现** | Phase 68–69 · ObjectGet/ArraySet/ValueEq/RefEq/CallWitness/ArrSorted |
| WASM AOT tier-2B guest 直发 | **已实现** | Phase 69 · MakeClosure/CallClosure/AwaitPoll/Suspend guest direct |
| WASM AOT tier-2C guest 直发 | **GA** | Phase 70 · ActorCall/Send/Register + TaskSpawn/Join + BeginTry/Catch guest direct |
| WASM AOT SIMD128 guest 直发 | **GA** | guest-v16 · SIMD16 i8x16 ne MVP（Phase 133）；Not/neg/abs/umin/umax/load/store/shuffle/sat/compare/extract-replace 继承；SIMD16 全集 → **134+** |
| Native LLVM tier-2 emit | **已实现** | Phase 71 · tier-2A/B/C 全矩阵 yes（DictGet/Iterator/tier-2C 直发 + strict parity） |
| VM JIT tier-2 (Cranelift) | **切片已实现** | ADR-140 / M1 W4 · Suite-1 pilot 真子集 opt-in（`SYX_JIT_TIER2=1` / `--jit`）；默认 off；非生产默认 |
| 浏览器编译器 `syx-compiler-wasm` | **GA** | Phase 72 · prelude v3 hash parity + Playwright chromium/firefox/webkit stdout hard |
| Registry publish→install→run | **部分实现** | Phase 70 lock v9 CDN mirror + signed E2E + Windows `.dll` stdout hard |


## 平台与运行时（当前）

| 特性 | 状态 | 说明 |
|------|------|------|
| SynUI v2 host 桥（`packages/ui`） | **部分实现** | Phase 133 SynUI v10（部分 GA）：stack · scrollView Signal bind · Web/Mobile 布局加深；v9 继承（image Signal bind · divider · Web/Mobile 控件加深 · textArea/list bind · spacer）；**非**全控件 / Web/Mobile language 全集 → **134+** |
| `packages/os` 运行时查询 | **部分实现** | Phase 133–136 · `Os.cpuCount` / `memoryTotal` / `memoryAvailable` / `userName` / `homeDir` / `hostname` / `uptimeMs` MVP；**非**完整 sysinfo / disk / network / cgroup → **137+** |
| 旧 syx-gui / egui / skia OT 栈 | **已退役** | Phase 124 起由 SynUI path-dep 替换；不得双栈冒充 SynUI GA |
| Farm pkcs11-so adapter / multi-slot | **部分实现** | Phase 133 · hsm-multi-slot MVP（`farm-lease-token-v16`）；pkcs11-`.so` adapter + digest-streaming / sign-verify 继承 |
| Dist multi-PoP / epoch persist | **部分实现** | Phase 133 · peer-announce-epoch-persist MVP（`dist-cdn-bgp-v16`）；multi-PoP / multi-region mesh / conflict-epoch 继承 |
| Native LLVM sole backend | **GA** | Cranelift 已移除；CLI 默认 `--backend=llvm` |

## 模块

| 特性 | 状态 | 说明 |
|------|------|------|
| import | **已实现** | |
| IO 文本 File API | **已实现** | Phase 37 |
| IO 二进制 readBytes/writeBytes + FileHandle | **已实现** | Phase 60 · `Array<UInt8>` · `stdlib.rs` |
| 跨模块 qualified 调用（deps） | **已实现** | `examples/deps` · `local_math.add` → 42 |
| syx.toml 依赖 | **已实现** | `syx-pkg` |
| 插件 manifest / lock | **部分实现** | Phase 49+ `[plugins]`、`syx-plugin-lock.json` |
| Codable/JSON | **部分实现** | `@Codable` L1 + `JSON.encode` Host；泛型 enum Phase 53+ |

## Swift 对标缺口

| 特性 | 状态 |
|------|------|
| 结果构建器 @resultBuilder | **GA** | Phase 119 engine 管线；`buildOptional` + multi-stmt |
| 完整 `as`/`as!`/`as?` 类型转换 | **部分实现** | `as?` MVP |
| 泛型特化反射 | **未实现** |
| 属性观察器 willSet/didSet | **GA** |
| 插件热卸载 | **未实现** |

---

参见：[语言指南附录 A](../language-guide/appendix-a-implementation-matrix.md) · [语言参考附录](../language-reference/appendix-implementation-matrix.md)
