# B-02 compile_into first-slice (W477 / ADR-821)

## Status

**B-02 compile_into first-slice met** (construction): driver production `compile_into*` faces are migrated to MIR / `compile_with_mir` — `merge_stdlib_mono_instances` uses `lower_module` + `compile_with_mir`, and `compile_to_program` AST else-arm is cleared (`use_mir_pipeline()` always true). `CodeGenerator::compile` / `compile_into` / `compile_into_with_semantic` **API definitions remain**. This window does **not** delete those APIs, enable a strict deprecate gate, claim dual track retired, or claim B-02 `[x]`. HostFn count **554**; B-01 **`[x]`**; B-02 **still `[~]`**; B-09 **`[~]`** (W476); B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-twenty-seventh` / `A-W477`.

## Index

| Prior | Status |
|-------|--------|
| W465 B-02 dual-track thin-open | **met** |
| W466 B-02 dual-track deepen thin | **met** |
| W475 B-02 retire-readiness | **met** |
| This window compile_into first-slice | **met** |
| AST→bytecode dual track retired | **still unmet** |
| Strict deprecate gate | **met** (W478) |
| B-02 `[x]` | **met** (W479 narrow) |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + deprecated `compile` honesty | **met** (W465) |
| 2. Deepen thin / call-site index | call-site table | **met** (W466) |
| 3a. Retire-readiness | blockers documented | **met** (W475) |
| 3b. Retire dual track or strict deprecate gate | axis construction | **first-slice met** (W477); **gate met** (W478); full retire **still unmet** |
| 4. B-02 `[x]` | axis exit | **still unmet** |

## First-slice table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| `merge_stdlib_mono_instances` via MIR | **yes** | Was `compile_into`; now `lower_module` + `compile_with_mir` |
| `compile_to_program` AST else cleared | **yes** | Dead `compile_into_with_semantic` arm removed |
| Driver production `compile_into*` call sites | **zero** | No `CodeGenerator::compile_into*` in driver |
| `compile_into*` API in codegen | **yes** | Honesty: API retained |
| Dual track retired / B-02 `[x]` / strict gate | **no** | Frozen |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Retire-readiness floor | **yes** | W475 |
| First-slice signed | **yes** | This window |
| Dual track retired | **no** | Still open |
| Strict deprecate gate | **yes** (W478) | See `gate-b02-deprecate-gate-strict.md` |
| B-02 `[x]` | **no** | first-slice ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB02CompileIntoFirstSliceInventory` / `gate_b02_compile_into_first_slice_inventory` |
| Flags | `b02_compile_into_first_slice_signed=true`; `retire_readiness_floor_ok=true`; `stdlib_mono_compile_into_migrated=true`; `compile_to_program_ast_else_cleared=true`; `compile_into_production_call_sites_zero=true`; `compile_into_api_present=true`; `b02_dual_track_retired=false`; `b02_axis_exit_enabled=false`; `deprecate_gate_strict_enabled=false` (first-slice inventory honesty; W478 gate inventory sets true) |
| Forbidden claim | B-02 `[x]` / dual-track retired / delete `compile`/`compile_into*` / strict gate / `Decl::Export` / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | first-slice **≠** retired / **≠** B-02 `[x]` / **≠** delete APIs |

## Evidence

- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs) `merge_stdlib_mono_instances` + `compile_to_program`
- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `GateB02CompileIntoFirstSliceInventory` / `compile_into*` APIs retained
- Meta: `gate_b02_compile_into_first_slice.rs`
- Floor: [`gate-b02-retire-readiness-thin.md`](gate-b02-retire-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-02

## Frozen

- Deleting `CodeGenerator::compile` / `compile_into*` / claiming B-02 `[x]` / claiming dual track retired (gate itself delivered in W478)
- `TokenKind::Export` / `Decl::Export` / parser `export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed / B-09 `[x]` / GC GA
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w477_*.plan.md`

## Next

W477 B-02 compile_into first-slice **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
