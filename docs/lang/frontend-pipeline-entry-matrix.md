# FrontendPipeline entry matrix (W461 / ADR-805 / B-03)

## Status

**B-03 FrontendPipeline entry matrix met** (policy+inventory): six driver entries (file / project / embed / LSP / REPL / browser) × `StopAfter` / `FrontendOptions` configuration surface is documented and signed; roadmap B-03 **`[~]→[x]`** (narrow — matrix doc acceptance only). HostFn count **554**; **≠** Pipeline API mutation / **≠** frontend `Module.exports` / **≠** daemon GA / **≠** MIR blob delete.

Fingerprint: `three-hundred-and-eleventh` / `A-W461`.

## Index

| Prior | Status |
|-------|--------|
| ADR-135 / M1-A01 FrontendPipeline | **met** (canonical pass order) |
| EC-M1-1 parity nails | **met** ([`frontend_pipeline_parity.rs`](../../crates/syx-driver/tests/lang/frontend_pipeline_parity.rs)) |
| W460 language exports thin-open | **met** (≠ this axis) |
| This window entry matrix doc | **met** |
| Frontend `Decl::Export` / auto-discovery | **still unmet** |
| B-01 MIR-consumer thin-open (W463) | **met** (inventory; still `[~]`) |
| B-01 `[x]` / AST codegen migration | **still unmet** |
| B-02 dual-track retired | **still `[~]`** |

## Entry × StopAfter matrix (hard)

| Entry | Typical `StopAfter` | Notes / evidence |
|-------|---------------------|------------------|
| file | `Mir` (default) | `FrontendPipeline::run` + driver file path |
| project | `Mir` | driver project build uses same Pipeline |
| embed | `Semantic` or `Mir` | codegen / embed injects then continues |
| LSP | `Semantic` / `Prepared` | syx-lsp stop points |
| REPL | `Mir` or `Semantic` | REPL driver |
| browser | `Mir` | parity nail in `frontend_pipeline_parity.rs` |

`StopAfter` variants (API): `Expanded` · `Prepared` · `Semantic` · `Mir` — see [`crates/syx-frontend/src/lib.rs`](../../crates/syx-frontend/src/lib.rs).

`FrontendOptions` knobs (documented, unchanged this window): `desugar_top_level`, `collecting_semantic`, `strict_concurrency`, import injection fields, `source`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `FrontendPipelineEntryMatrixInventory` / `frontend_pipeline_entry_matrix_inventory` |
| Flags | `b03_pipeline_entry_matrix_signed=true`; `documented_entry_count=6`; `stop_after_variants_documented=true`; `frontend_pipeline_api_present=true`; `pipeline_api_mutation_enabled=false`; `b03_axis_exit_enabled=true` |
| Forbidden claim | Pipeline API change / `Decl::Export` / auto-discovery / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | B-03 `[x]` (matrix doc) **≠** B-01/B-02 / **≠** frontend exports / **≠** API mutation |

## Evidence

- [`crates/syx-frontend/src/lib.rs`](../../crates/syx-frontend/src/lib.rs) inventory + `StopAfter` / `FrontendPipeline`
- Meta: `frontend_pipeline_entry_matrix.rs`
- Cross: [`plan/01-project-status.md`](../../plan/01-project-status.md) M1-A04；[`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-03

## Frozen

- Mutating `FrontendPipeline` / `StopAfter` / `FrontendOptions` behavior this window
- `TokenKind::Export` / `Decl::Export` / semantic→`aot_export_roots`
- Claiming daemon GA / exports auto-discovery / MIR blob re-embed
- Claiming B-01/B-02 `[x]` / adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
