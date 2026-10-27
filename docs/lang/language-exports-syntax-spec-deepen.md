# Language exports syntax-spec deepen (W462 / ADR-806)

## Status

**Language exports syntax-spec deepen met** (policy+inventory): proposed `export func Name(...)` → `MirModule.aot_export_roots` EBNF and mapping are documented and signed; W460 thin-open floor still holds. W462 inventory historical flags `lexer_export_keyword_present` / `decl_export_variant_present` / `parser_export_enabled` remain **false** (honesty floor). Live lexer/AST/parser Export landed in **W480** (see [`language-exports-decl-export-first-slice.md`](language-exports-decl-export-first-slice.md)). Semantic→roots fill remain **off**. HostFn count **554**; B-03～B-08 **`[x]`** unchanged; ≠ semantic roots / ≠ auto-discovery / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-twelfth` / `A-W462`.

## Index

| Prior | Status |
|-------|--------|
| W460 language exports thin-open | **met** |
| W440 default multi-root via `aot_export_roots` | **met** |
| This window syntax-spec deepen | **met** |
| Frontend syntax-spec (proposed EBNF) | **met** (docs) |
| Lexer `TokenKind::Export` | **met** (W480 live; W462 inventory floor still false) |
| AST `Decl::Export` / parser `export` | **met** (W480 live; W462 inventory floor still false) |
| Semantic fills `aot_export_roots` | **met** (W481 live; W462 inventory floor still false) |
| `export {…}` ExportList | **met** (W483 live; this window Index was deferred) |
| ExportList trailing comma | **met** (W486 / ADR-830) |
| Language exports auto-discovery | **still unmet** |

## Proposed EBNF (documented; ExportList implemented W483)

```ebnf
ExportDecl   ::= "export" FunctionDecl
FunctionDecl ::= "func" Ident "(" ParamList? ")" Block
ExportList   ::= "export" "{" Ident ("," Ident)* ","? "}"   (* live W483+W486 / ADR-827+ADR-830; optional trailing comma *)
```

Surface examples:

- `export func Name(...) { ... }` — wraps a function declaration; AST: `Decl::Export` around `Function` (W480)
- Module-level `export { Name, ... }` — **met** (W483 / ADR-827; `Decl::ExportList`)
- Optional trailing comma `export { Name, ... , }` — **met** (W486 / ADR-830)

## Mapping (hard)

| Source | Target |
|--------|--------|
| Exported function name set | `MirModule.aot_export_roots: Vec<SmolStr>` |
| Empty export set | main-only emit (W440 compatible) |
| Non-empty roots | default multi-root emit includes named closures |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsSyntaxSpecDeepenInventory` / `language_exports_syntax_spec_deepen_inventory` |
| Flags | `language_exports_syntax_spec_signed=true`; `proposed_syntax_documented=true`; `thin_open_floor_ok=true`; `lexer_export_keyword_present=false`; `decl_export_variant_present=false`; `parser_export_enabled=false`; `semantic_fills_aot_export_roots=false`; `language_exports_auto_discovery_enabled=false` |
| Forbidden claim | `TokenKind::Export` / `Decl::Export` / parser enabled / auto-discovery / exports GA / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | syntax-spec **≠** parser / **≠** thin-open reopen / **≠** B-01 `[x]` |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `LanguageExportsSyntaxSpecDeepenInventory`
- Meta: `language_exports_syntax_spec_deepen.rs`
- Floor: [`language-exports-aot-roots-thin.md`](language-exports-aot-roots-thin.md)
- Cross: [`b04-default-multi-root.md`](b04-default-multi-root.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Semantic→`aot_export_roots` fill / claiming W462 inventory lexer/decl/parser true (honesty floor)
- Claiming language exports auto-discovery / exports GA
- Changing emit / compact / absolute thresholds / FrontendPipeline this window
- Deleting `syx_rt_load_mir_blob` ABI / re-embedding blob/sidecar
- Claiming daemon GA / adding HostFn (keep 554)
- Claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w461_*.plan.md`

## Next

W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488+ — 可选 VM `GcCollect` 收敛 / mailbox Value / enable / 扫全 auto / exports GA / RT ABI 全清（慎）
