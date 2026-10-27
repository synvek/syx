# B-02 strict deprecate gate (W478 / ADR-822)

## Status

**B-02 strict deprecate gate met** (construction): residual `CodeGenerator::compile` / `compile_into` / `compile_into_with_semantic` APIs carry `#[deprecated]`; the only external `CodeGenerator::compile(` test call site (`float_literal.rs`) is migrated to `lower_module` + `compile_with_mir`; driver production `compile_into*` remains zero (W477 floor). This window does **not** delete those APIs, claim dual track retired, or claim B-02 `[x]`. HostFn count **554**; B-01 **`[x]`**; B-02 **`[x]`** (W479 narrow; this gate doc historical); B-09 **`[~]`** (W476); B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-twenty-eighth` / `A-W478`.

## Index

| Prior | Status |
|-------|--------|
| W465 B-02 dual-track thin-open | **met** |
| W466 B-02 dual-track deepen thin | **met** |
| W475 B-02 retire-readiness | **met** |
| W477 B-02 compile_into first-slice | **met** |
| This window strict deprecate gate | **met** |
| AST→bytecode dual track retired | **still unmet** |
| B-02 `[x]` | **met** (W479 narrow) |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + deprecated `compile` honesty | **met** (W465) |
| 2. Deepen thin / call-site index | call-site table | **met** (W466) |
| 3a. Retire-readiness | blockers documented | **met** (W475) |
| 3b. Retire dual track or strict deprecate gate | axis construction | **gate met** (this window); full retire **still unmet** |
| 4. B-02 `[x]` | axis exit | **met** (W479 narrow) |

## Gate table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| `#[deprecated]` on `compile` / `compile_into*` | **yes** | Strict gate attrs |
| External `CodeGenerator::compile(` sites | **zero** | float_literal migrated to MIR |
| Driver production `compile_into*` call sites | **zero** | W477 floor |
| `compile_into*` API in codegen | **yes** | Honesty: API retained |
| Dual track retired | **no** | Frozen |
| B-02 `[x]` (narrow) | **yes** (W479) | See `plan/archive/gates/gate-b02-dual-track-exit.md` (F-10 W592 archived) |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| First-slice floor | **yes** | W477 |
| Strict gate signed | **yes** | This window |
| Dual track retired | **no** | Still open |
| B-02 `[x]` (narrow) | **yes** (W479) | gate path exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB02DeprecateGateStrictInventory` / `gate_b02_deprecate_gate_strict_inventory` |
| Flags | `b02_deprecate_gate_strict_signed=true`; `first_slice_floor_ok=true`; `deprecated_attrs_present=true`; `external_compile_call_sites_zero=true`; `compile_into_production_call_sites_zero=true`; `compile_into_api_present=true`; `deprecate_gate_strict_enabled=true`; `b02_dual_track_retired=false`; `b02_axis_exit_enabled=false` |
| Forbidden claim | B-02 `[x]` / dual-track retired / delete `compile`/`compile_into*` / `Decl::Export` / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | strict gate **≠** retired / **≠** B-02 `[x]` / **≠** delete APIs |

## Evidence

- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `GateB02DeprecateGateStrictInventory` / `#[deprecated]` on `compile`/`compile_into*`
- [`crates/syx-codegen/tests/float_literal.rs`](../../crates/syx-codegen/tests/float_literal.rs) MIR path
- Meta: `gate_b02_deprecate_gate_strict.rs`
- Floor: [`gate-b02-compile-into-first-slice.md`](gate-b02-compile-into-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-02

## Frozen

- Deleting `CodeGenerator::compile` / `compile_into*` / claiming B-02 `[x]` / claiming dual track retired (`retired=true`)
- `TokenKind::Export` / `Decl::Export` / parser `export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed / B-09 `[x]` / GC GA
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w477_*.plan.md`

## Next

W477 B-02 compile_into first-slice **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
