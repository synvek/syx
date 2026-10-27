# B-02 retire-readiness thin (W475 / ADR-819)

## Status

**B-02 retire-readiness thin met** (policy+inventory): after W465–W466 thin+deepen, retire **preconditions** are signed — MIR primary present, deprecated `compile` production call sites zero, `compile_into*` still an AST second face, and retire blockers documented. This window does **not** retire the dual track, delete `compile`/`compile_into*`, enable a strict deprecate gate, or claim B-02 `[x]`. HostFn count **554**; B-01 **`[x]`** (W474); B-03～B-08 **`[x]`**; B-02 **still `[~]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-twenty-fifth` / `A-W475`.

## Index

| Prior | Status |
|-------|--------|
| W465 B-02 dual-track thin-open | **met** |
| W466 B-02 dual-track deepen thin | **met** |
| This window retire-readiness | **met** |
| W477 compile_into first-slice | **met** |
| AST→bytecode dual track retired | **still unmet** |
| Strict deprecate gate | **still unmet** |
| B-02 `[x]` | **still unmet** |
| B-01 `[x]` | **met** (W474 / ADR-818) |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + deprecated `compile` honesty | **met** (W465 / ADR-809) |
| 2. Deepen thin / call-site index | call-site table | **met** (W466 / ADR-810) |
| 3a. Retire-readiness | blockers documented; preconditions signed | **met** (W475 / ADR-819) |
| 3b. Retire dual track or strict deprecate gate | axis construction | **first-slice met** (W477); full retire/gate **still unmet** |
| 4. B-02 `[x]` | axis exit | **still unmet** |

## Retire blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| `compile_into` / `compile_into_with_semantic` production face | **yes** | Driver mono/dep still uses AST second face |
| Deprecated `compile` test residue | **yes** | Definition + test residue (e.g. `float_literal.rs`); **production call sites = 0** |
| Strict deprecate gate | **no** | Not enabled |
| Dual track retired | **no** | APIs still present |
| B-02 axis exit (`[x]`) | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Deepen floor | **yes** | W466 signed |
| Retire-readiness signed | **yes** | This window |
| Dual track retired | **no** | Still open |
| Strict deprecate gate | **no** | Frozen |
| B-02 `[x]` | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB02RetireReadinessThinInventory` / `gate_b02_retire_readiness_thin_inventory` |
| Flags | `b02_retire_readiness_signed=true`; `deepen_floor_ok=true`; `mir_primary_compile_path_present=true`; `ast_bytecode_compile_path_present=true`; `compile_into_ast_path_present=true`; `deprecated_compile_production_call_sites_zero=true`; `retire_blockers_documented=true`; `b02_dual_track_retired=false`; `b02_axis_exit_enabled=false`; `deprecate_gate_strict_enabled=false` |
| Forbidden claim | B-02 `[x]` / dual-track retired / delete `compile`/`compile_into*` / strict gate / `Decl::Export` / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | retire-readiness **≠** retired / **≠** B-02 `[x]` / **≠** delete APIs |

## Evidence

- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `GateB02RetireReadinessThinInventory` / `compile` / `compile_with_mir` / `compile_into`
- Meta: ~~`gate_b02_retire_readiness_thin.rs`~~ retired（F-10 W593）；proof = this SSOT + `GateB02RetireReadinessThinInventory`
- Floor: [`gate-b02-dual-track-deepen-thin.md`](gate-b02-dual-track-deepen-thin.md) / [`gate-b02-dual-track-thin.md`](gate-b02-dual-track-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-02, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `CodeGenerator::compile` / `compile_into*` / enabling strict deprecated gate / claiming B-02 `[x]` / claiming dual track retired
- `TokenKind::Export` / `Decl::Export` / parser `export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w474_*.plan.md`

## Next

W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
