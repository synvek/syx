# B-02 compile API delete (W482 / ADR-826)

> **Status:** **B-02 public `compile`/`compile_into*` delete signed** (ADR-826 / W482) — deletes three public AST-only entry points on `CodeGenerator` after W479 narrow `[x]` exit; signs `api_delete_enabled=true` + `b02_dual_track_retired=true` for the **public** dual-track face.  
> Cross-index: [`gate-b02-dual-track-exit.md`](../../plan/archive/gates/gate-b02-dual-track-exit.md) / ADR-823; [`gate-b02-deprecate-gate-strict.md`](gate-b02-deprecate-gate-strict.md) / ADR-822; [`gate-b02-compile-into-first-slice.md`](gate-b02-compile-into-first-slice.md) / ADR-821; [`language-exports-semantic-aot-export-roots.md`](../lang/language-exports-semantic-aot-export-roots.md) / ADR-825.  
> Roadmap B-02 remains `[x]` (narrow exit already signed; this window clears public API residual). **Not** private AST helper purge. **Not** exports auto-discovery. **Not** B-09 `[x]`. **Not** MIR blob delete this window.  
> Honesty: W477–W479 historical inventories keep `compile_into_api_present=true` / `api_delete_enabled=false` / `retired=false`.

Fingerprint: `three-hundred-and-thirty-second` / `A-W482`.

## Scope narrow (ADR-826)

B-02 public compile API **delete** (narrow) =
- W479 dual-track exit floor holds (historical inventory values)
- Delete `CodeGenerator::compile` / `compile_into` / `compile_into_with_semantic`
- Retain `compile_with_mir` + private `compile_*` helpers used by MIR/type-shell path
- Sign `GateB02CompileApiDeleteInventory` (`api_delete_enabled=true`; `compile_into_api_present=false`; `b02_dual_track_retired=true`)

Exports auto-discovery / B-09 unified heap / residual ABI cleanup / private helper body purge = **unmet**（**Not** claimed **met**）.

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Dual-track exit | **met** (W479 / ADR-823) | [`gate-b02-dual-track-exit.md`](../../plan/archive/gates/gate-b02-dual-track-exit.md) |
| Public entry delete | **signed** (W482 / ADR-826) | this document + `GateB02CompileApiDeleteInventory` |
| `compile_with_mir` retained | **met** | `syx-codegen` + driver production path |

**Evidence nails:** `crates/syx-driver/tests/native/gate_b02_compile_api_delete.rs` + `gate_b02_compile_api_delete_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB02CompileApiDeleteInventory` / `gate_b02_compile_api_delete_inventory` |
| Flags | `b02_compile_api_delete_signed=true`; `dual_track_exit_floor_ok=true`; `compile_into_api_present=false`; `api_delete_enabled=true`; `b02_dual_track_retired=true`; `compile_with_mir_present=true`; `language_exports_auto_discovery_enabled=false` |
| Honesty floor | W479 `GateB02DualTrackHorizonExitInventory` keeps `compile_into_api_present=true`; `api_delete_enabled=false`; `b02_dual_track_retired=false` |
| Forbidden claim | auto-discovery / exports GA / B-09 `[x]` / private helper purge / MIR blob re-embed / HostFn+1 / production-default JIT |
| Honesty | public API delete **≠** private AST helper purge / **≠** auto-discovery / **≠** B-09 `[x]` |

## Frozen

- Claiming private AST `compile_*` helpers cleared / claiming residual ABI cleanup
- Exports auto-discovery / `export {…}` / exports GA
- Claiming B-09 `[x]` / unified heap / GC GA
- Adding HostFn (keep 554) / claiming production-default JIT / MIR blob re-embed
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w481_*.plan.md`

## Next

W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490+ — 可选 enable；或双堆合一；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎）
