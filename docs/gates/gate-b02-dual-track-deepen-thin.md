# B-02 dual-track deepen thin (W466 / ADR-810)

## Status

**B-02 dual-track deepen thin met** (policy+inventory): call-site table documents `compile_with_mir` primary path, deprecated `compile` test residue (production call sites zero), and `compile_into*` AST second face; W465 thin-open floor still holds. Roadmap B-02 **stays `[~]`** — this window does **not** claim B-02 `[x]`, retire the dual track, or enable a strict deprecated gate. HostFn count **554**; B-03～B-08 **`[x]`** unchanged; B-01 **`[x]`** (W474); ≠ `Decl::Export` / ≠ B-01 type-shell migrate / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixteenth` / `A-W466`.

## Index

| Prior | Status |
|-------|--------|
| W465 B-02 dual-track thin-open | **met** |
| This window deepen thin / call-site table | **met** |
| AST→bytecode `compile` retired | **still unmet** |
| B-02 `[x]` / strict deprecate gate | **still unmet** |
| B-01 `[x]` | **met** (W474 / ADR-818) |

## Deepen ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + deprecated `compile` honesty | **met** (W465 / ADR-809) |
| 2. Deepen thin / call-site index | this document | **met** (W466 / ADR-810) |
| 3a. Retire-readiness | blockers documented | **met** (W475 / ADR-819) |
| 3b. Retire dual track or strict deprecate gate | axis construction | **still unmet** |
| 4. B-02 `[x]` | axis exit | **still unmet** |

## Call-site table (hard)

| Path | Sites | Present? | Meaning |
|------|-------|----------|---------|
| `compile_with_mir` | `syx-driver` `lib.rs` + `unified_compile.rs` | **yes** | Primary MIR bytecode path |
| `compile` (`#[deprecated]`) | definition in codegen; test residue `float_literal.rs` | **yes** | Honesty: dual-track residue; **production call sites = 0** |
| `compile_into` / `compile_into_with_semantic` | codegen API retained; driver prod **cleared** (W477) | **API yes / prod no** | W477 first-slice migrated to MIR |
| Dual track retired | — | **no** | Still open |
| B-02 axis exit (`[x]`) | — | **no** | deepen ≠ exit |
| Strict deprecate gate | — | **no** | Frozen this window |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB02DualTrackDeepenThinInventory` / `gate_b02_dual_track_deepen_thin_inventory` |
| Flags | `b02_dual_track_deepen_thin_signed=true`; `thin_open_floor_ok=true`; `mir_primary_compile_path_present=true`; `ast_bytecode_compile_path_present=true`; `compile_into_ast_path_present=true`; `deprecated_compile_production_call_sites_zero=true`; `call_sites_documented=true`; `b02_dual_track_retired=false`; `b02_axis_exit_enabled=false`; `deprecate_gate_strict_enabled=false` |
| Forbidden claim | B-02 `[x]` / dual-track retired / delete `compile`/`compile_into*` / `Decl::Export` / B-01 migrate / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | deepen **≠** B-02 `[x]` / **≠** retire dual track / **≠** thin reopen |

## Evidence

- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `GateB02DualTrackDeepenThinInventory` / `compile` / `compile_with_mir` / `compile_into`
- Meta: ~~`gate_b02_dual_track_deepen_thin.rs`~~ retired（F-10 W593）；proof = this SSOT + `GateB02DualTrackDeepenThinInventory`
- Floor: [`gate-b02-dual-track-thin.md`](gate-b02-dual-track-thin.md)
- Readiness: [`gate-b02-retire-readiness-thin.md`](gate-b02-retire-readiness-thin.md) (W475)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-02, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `CodeGenerator::compile` / `compile_into*` / enabling strict deprecated gate / claiming B-02 `[x]`
- B-01 type-shell migrate / deleting `compile_type_shells` / claiming B-01 `[x]`
- `TokenKind::Export` / `Decl::Export` / parser `export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w465_*.plan.md`

## Next

W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
