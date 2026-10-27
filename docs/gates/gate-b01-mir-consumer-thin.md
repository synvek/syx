# B-01 MIR-consumer thin-open (W463 / ADR-807)

## Status

**B-01 MIR-consumer thin-open met** (policy+inventory): primary path `CodeGenerator::compile_with_mir` consumes MIR function bodies; AST `compile_type_shells` / type-shell dual-track residue remains **present** and is **honestly signed**. This thin charter does **not** itself claim B-01 `[x]` (axis exit signed later by W474 / ADR-818); it does **not** complete AST codegen migration. HostFn count **554**; B-03～B-08 **`[x]`** unchanged; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-thirteenth` / `A-W463`.

## Index

| Prior | Status |
|-------|--------|
| W461 B-03 FrontendPipeline entry matrix | **met** (`[x]` narrow) |
| W462 language exports syntax-spec deepen | **met** (≠ this axis) |
| This window B-01 thin-open | **met** |
| B-01 deepen thin (W464) | **met** (residual cluster table; see [`gate-b01-mir-consumer-deepen-thin.md`](gate-b01-mir-consumer-deepen-thin.md)) |
| All backends MIR+type metadata only | **still unmet** |
| B-01 `[x]` / AST codegen migration complete | **axis `[x]` met** (W474 / ADR-818 narrow); migration-complete **still unmet** |
| B-02 dual-track thin-open (W465) | **met** (see [`gate-b02-dual-track-thin.md`](gate-b02-dual-track-thin.md)) |
| B-02 dual-track retired | **still unmet** (`[~]`) |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| MIR primary compile path (`compile_with_mir`) | **yes** | Function bodies from MIR |
| AST type-shell path (`compile_type_shells`) | **yes** | Honesty: dual-track residue |
| AST codegen migration complete | **no** | Not all backends MIR-only |
| B-01 axis exit (`[x]`) | **no** | thin ≠ exit |
| B-02 dual-track retired | **no** | Still open |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB01MirConsumerThinInventory` / `gate_b01_mir_consumer_thin_inventory` |
| Flags | `b01_mir_consumer_thin_signed=true`; `mir_primary_compile_path_present=true`; `ast_type_shell_path_present=true`; `ast_codegen_migration_complete=false`; `b01_axis_exit_enabled=false`; `b02_dual_track_retired=false`; `b03_pipeline_matrix_floor_ok=true` |
| Forbidden claim | B-01 `[x]` / AST migration complete / B-02 retired / `Decl::Export` / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | thin-open **≠** B-01 `[x]` / **≠** type-shell delete / **≠** B-02 |

## Evidence

- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `GateB01MirConsumerThinInventory` / `compile_with_mir` / `compile_type_shells`
- Meta: ~~`gate_b01_mir_consumer_thin.rs`~~ retired（F-10 W595）；proof = this SSOT + `GateB01MirConsumerThinInventory`
- Cross: [`frontend-pipeline-entry-matrix.md`](../lang/frontend-pipeline-entry-matrix.md), [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-01, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `compile_type_shells` / completing AST→MIR migration this window
- Claiming B-01 `[x]` / B-02 dual-track retired
- `TokenKind::Export` / `Decl::Export` / parser `export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w462_*.plan.md`

## Next

W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
