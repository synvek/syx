# Language exports Decl::Export first-slice (W480 / ADR-824)

## Status

**Language exports `Decl::Export` first-slice met** (ADR-824 / W480): lexer `TokenKind::Export` + keyword `"export"`, AST `Decl::Export { item }`, and parser `export func Name(...) { … }` → `Decl::Export` are delivered and signed. W462 syntax-spec deepen floor still holds (inventory historical lexer/decl/parser flags remain **false**). W480 inventory historical `semantic_fills_aot_export_roots=false` remains (honesty floor); live fill landed in **W481** (see [`language-exports-semantic-aot-export-roots.md`](language-exports-semantic-aot-export-roots.md)). Auto-discovery and exports GA remain **off**. HostFn count **554**; B-02 **`[x]`** (narrow); B-09 **`[~]`**; ≠ auto-discovery / ≠ exports GA / ≠ delete `compile` API / ≠ MIR blob / ≠ B-09 `[x]`.

Fingerprint: `three-hundred-and-thirtieth` / `A-W480`.

## Index

| Prior | Status |
|-------|--------|
| W460 language exports thin-open | **met** |
| W462 language exports syntax-spec deepen | **met** (honesty floor) |
| This window Decl::Export first-slice | **met** |
| Lexer `TokenKind::Export` | **met** |
| AST `Decl::Export` / parser `export func` | **met** |
| Semantic fills `aot_export_roots` | **met** (W481 live; W480 inventory floor still false) |
| Language exports auto-discovery | **still unmet** |
| Exports GA | **still unmet** |

## Surface (implemented)

```ebnf
ExportDecl   ::= "export" FunctionDecl
FunctionDecl ::= "func" Ident "(" ParamList? ")" Block
(* still deferred: *)
ExportList   ::= "export" "{" Ident ("," Ident)* "}"
```

Example:

```syx
export func greet() {
    print("hi")
}
```

Parser yields `Decl::Export { item: Box<Decl::Function { name: greet, … }> }`. Lowering unwraps Export → Function for MIR/codegen. W481 fills `MirModule.aot_export_roots` from Export names (this window's inventory still signs `semantic_fills=false` as honesty floor).

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsDeclExportFirstSliceInventory` / `language_exports_decl_export_first_slice_inventory` |
| Flags | `language_exports_decl_export_first_slice_signed=true`; `syntax_spec_floor_ok=true`; `lexer_export_keyword_present=true`; `decl_export_variant_present=true`; `parser_export_enabled=true`; `semantic_fills_aot_export_roots=false` (honesty floor); `language_exports_auto_discovery_enabled=false`; `exports_ga_claimed=false` |
| Honesty floor | `LanguageExportsSyntaxSpecDeepenInventory` keeps lexer/decl/parser **false** (historical) |
| Forbidden claim | claiming W480 inventory `semantic_fills=true` / auto-discovery / exports GA / delete compile API / B-09 `[x]` / MIR blob re-embed / HostFn+1 |
| Honesty | first-slice **≠** auto-discovery / **≠** exports GA / **≠** thin reopen |

## Evidence

- [`crates/syx-lexer/src/token.rs`](../../crates/syx-lexer/src/token.rs) `TokenKind::Export`
- [`crates/syx-ast/src/lib.rs`](../../crates/syx-ast/src/lib.rs) `Decl::Export`
- [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs) `parse_export_decl`
- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `LanguageExportsDeclExportFirstSliceInventory`
- Meta: `language_exports_decl_export_first_slice.rs`
- Floor: [`language-exports-syntax-spec-deepen.md`](language-exports-syntax-spec-deepen.md), [`language-exports-aot-roots-thin.md`](language-exports-aot-roots-thin.md)
- Follow-on: [`language-exports-semantic-aot-export-roots.md`](language-exports-semantic-aot-export-roots.md)

## Frozen

- Claiming W480 inventory `semantic_fills_aot_export_roots=true` / `export { … }` list / auto-discovery / exports GA
- Deleting `compile`/`compile_into*` / claiming `retired=true`
- B-09 unified heap / B-09 `[x]` / GC GA
- Changing emit / absolute thresholds / HostFn+1
- Claiming daemon GA / MIR blob re-embed / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w479_*.plan.md`

## Next

W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
