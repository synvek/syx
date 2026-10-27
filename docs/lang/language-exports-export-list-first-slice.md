# Language exports `export {…}` list first-slice (W483 / ADR-827)

> **Status:** **`export { Name, … }` list first-slice signed** (ADR-827 / W483) — AST `Decl::ExportList` + parser brace list + `lower_module` push names into `MirModule.aot_export_roots` (dedupe; coexists with `export func`).  
> Cross-index: [`language-exports-export-list-trailing-comma.md`](language-exports-export-list-trailing-comma.md) / ADR-830; [`language-exports-semantic-aot-export-roots.md`](language-exports-semantic-aot-export-roots.md) / ADR-825; [`language-exports-decl-export-first-slice.md`](language-exports-decl-export-first-slice.md) / ADR-824; [`language-exports-syntax-spec-deepen.md`](language-exports-syntax-spec-deepen.md) / ADR-806 (`ExportList` EBNF).  
> Roadmap B-02 **`[x]`**; B-09 **`[~]`**. **Not** scan-all `public`/`func` auto-discovery. **Not** `language_exports_auto_discovery_enabled=true`. **Not** exports GA. **Not** B-09 `[x]`. **Not** MIR blob delete this window.  
> Honesty: W481/W480/W462/W460 inventories keep `language_exports_auto_discovery_enabled=false`.

Fingerprint: `three-hundred-and-thirty-third` / `A-W483`.

## Scope narrow (ADR-827)

`export {…}` list **complete** (narrow) =
- W462 ExportList EBNF documented
- W480–W481 `export func` → roots ladder
- This window (W483 historical): `Decl::ExportList` + parser `{ Ident ("," Ident)* }` (non-empty; trailing comma deferred) + lower fill
- Trailing comma: **met** in W486 / ADR-830 — see [`language-exports-export-list-trailing-comma.md`](language-exports-export-list-trailing-comma.md)

Scan-all auto-discovery / exports GA / B-09 unified heap = **unmet**（**Not** claimed **met**）. List ≠ auto-discovery.

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Semantic→roots | **met** (W481 / ADR-825) | [`language-exports-semantic-aot-export-roots.md`](language-exports-semantic-aot-export-roots.md) |
| ExportList syntax + lower | **signed** (W483 / ADR-827) | this document + `LanguageExportsExportListFirstSliceInventory` |

**Evidence nails:** `crates/syx-driver/tests/native/language_exports_export_list_first_slice.rs` + `language_exports_export_list_first_slice_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsExportListFirstSliceInventory` / `language_exports_export_list_first_slice_inventory` |
| Flags | `language_exports_export_list_first_slice_signed=true`; `semantic_roots_floor_ok=true`; `export_list_syntax_enabled=true`; `language_exports_auto_discovery_enabled=false`; `exports_ga_claimed=false` |
| Honesty floor | W481 inventory keeps `language_exports_auto_discovery_enabled=false` |
| Forbidden claim | scan-all auto / `auto_discovery=true` / exports GA / B-09 `[x]` / residual ABI cleanup / HostFn+1 / MIR blob re-embed |
| Honesty | export list **≠** scan-all auto-discovery / **≠** exports GA |

## Frozen

- Claiming `language_exports_auto_discovery_enabled=true` / scan-all `public`/`func`
- Exports GA / B-09 `[x]` / unified heap / GC GA
- Residual ABI cleanup / claiming production blob delete this window
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w482_*.plan.md`

## Next

W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W516+ — 优先 LLVM·WASM RC 退役 → 全量迁 / 翻 storage cleanup enabled / 真 enable；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
