# Stage-2 parser dual-track census (S2-04)

**ADR:** [stage2-s2-04-open](adr/stage2-s2-04-open.md) · **Window:** S2-I143（baseline）· **Current:** S2-I164  
**Goal (S2-04):** fmt / LSP / lint share one tree; stop dual-maintaining `syx-cst` grammar.  
**This census:** honest baseline + ladder progress — S2-04 **scoped `[x]`** ([claim](adr/stage2-s2-04-claim.md)); **≠** runtime one-tree compile / **≠** Stage-2 exit.

## Command

```bash
bash scripts/s2_04_parser_census.sh
```

## S2-I143 baseline (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-parser/src/lib.rs` LOC | **5665** |
| `crates/syx-cst/src/lib.rs` LOC | **5952** |
| Combined grammar-face LOC | **11617** |
| Cargo.toml deps on `syx-cst` | **syx-cst**, **syx-fmt**, **syx-lint**（S2-I153）, **syx-driver**(dev) |
| Production compile path | `syx-frontend` → `syx_parser::Parser` (no CST) |
| lint path | `syx-lint` → `syx_parser::Parser` |
| LSP path | `syx-lsp` → driver → `Parser` / frontend (no `syx-cst` dep); **format** via `format_document_for_lsp` (S2-I152) |

## Freeze baseline (S2-I163 · 2026-11-06)

| Metric | Value |
|--------|------:|
| `syx_parser_lib_loc` | **5665** |
| `syx_cst_lib_loc` | **6716** |
| `combined_grammar_loc` | **12381** |
| Charter | [dual-grammar-freeze-charter](adr/stage2-s2-04-dual-grammar-freeze-charter.md) |

Raising combined grammar LOC is **not** progress toward S2-04 `[x]`; default = hold or trend down.

## Entry map (honest dual track · post-I164 claim)

| Consumer | Entry | Tree |
|----------|-------|------|
| Compile / check / MIR | `FrontendPipeline` / `Parser::parse` | AST（product path unchanged） |
| Lower probe | `lower_cst_subset_probe` → `Module` | CST→AST **probe**（S2-I160；S2-I162 +let/var/return） |
| lint (**CLI / buffer**) | `lint_source_auto` → CST Ok → **lower** → `lint_module`; `LWR001`/`CST001` → `lint_source` | CST-first + lower（S2-I161）；rules still AST |
| LSP semantics | driver `analyze_document_for_lsp` / parse helpers | AST |
| fmt **write** (`format_file`) | `format_dry_run` → `format_source_auto` | CST-first; `CST001` → AST（S2-I151） |
| fmt **`--check`** | `format_check_path` → `format_dry_run` → `format_source_auto` | same SSOT |
| fmt **LSP** (`textDocument/formatting`) | `format_document_for_lsp` → `format_dry_run` | same SSOT（S2-I152） |
| fmt pure CST nail | `format_check_path_cst` / `format_cst` concat | CST only |

### Remaining honesty debt (post-I164 claim)

S2-04 **scoped `[x]`** ([claim](adr/stage2-s2-04-claim.md)): interim dual-track SSOT + freeze KPI + lint honesty accepted. Remaining primary debts: **Frontend still Parser-only** (migrate later); **dual faces frozen not retired**; LSP lint diagnostics deferred; Stage-2 still needs **S2-05…07**. **≠** Stage-2 exit / **≠** runtime one-tree compile.

## Key symbols (baseline presence)

| Symbol | Crate | Role |
|--------|-------|------|
| `pub fn format_source_auto` | `syx-fmt` | W873 CST-first + `CST001`→AST |
| `pub fn format_source_cst` | `syx-fmt` | CST-only (reindent GA) |
| `pub fn format_dry_run` | `syx-fmt` | write/`--check`/LSP format SSOT |
| `pub fn format_document_for_lsp` | `syx-driver` | S2-I152 LSP buffer format |
| `pub fn lint_source_auto` | `syx-lint` | S2-I153 CST-first；S2-I161 lower on probe face |
| `pub fn lint_document_for_lsp` | `syx-driver` | S2-I153 lint buffer API |
| `fn take_if_let_simple` | `syx-cst` | S2-I154 if-let-simple subset |
| `fn take_guard_let_simple` | `syx-cst` | S2-I155 guard-let-simple subset |
| `fn take_if_case_simple` | `syx-cst` | S2-I156 if-case-simple subset |
| `fn take_integer_range_simple` | `syx-cst` | S2-I157 range-in-call subset |
| `fn take_optional_chain_simple` | `syx-cst` | S2-I158 optional-chain-simple subset |
| `pub fn lower_cst_subset_probe` | `syx-cst` | S2-I160 probe；S2-I162 +let/var/return |
| `pub fn format_source` | `syx-fmt` | AST pretty (`SYX_FMT_AST=1` hatch) |
| `parse_cst_subset` / `format_cst` | `syx-cst` | lossless subset |
| `Parser::parse` | `syx-parser` | production AST |

## Ladder

1. **S2-I144:** Convergence-strategy ADR — **locked** ([stage2-s2-04-convergence-strategy](adr/stage2-s2-04-convergence-strategy.md)).
2. **S2-I145:** CST reindent FunctionDecl first knife — **landed** ([cst-reindent-func-body](adr/stage2-s2-04-cst-reindent-func-body.md)).
3. **S2-I146:** CST reindent nested deepen — **landed** ([cst-reindent-func-nested](adr/stage2-s2-04-cst-reindent-func-nested.md)).
4. **S2-I147:** CST reindent 2→4 normalize — **landed** ([cst-reindent-func-2to4](adr/stage2-s2-04-cst-reindent-func-2to4.md)).
5. **S2-I148:** CST reindent class stored-member — **landed** ([cst-reindent-class-stored](adr/stage2-s2-04-cst-reindent-class-stored.md)).
6. **S2-I149:** CST reindent env experiment gate — **landed** ([cst-reindent-env-gate](adr/stage2-s2-04-cst-reindent-env-gate.md)).
7. **S2-I150:** CST reindent GA — **landed** ([cst-reindent-ga](adr/stage2-s2-04-cst-reindent-ga.md)).
8. **S2-I151:** fmt write migration — **landed** ([fmt-write-migration](adr/stage2-s2-04-fmt-write-migration.md)).
9. **S2-I152:** LSP DocumentFormatting — **landed** ([lsp-document-formatting](adr/stage2-s2-04-lsp-document-formatting.md)): `textDocument/formatting` → `format_document_for_lsp` / dry_run/auto (**≠** lint 共树 / ≠ S2-04 `[x]`).
10. **S2-I153:** lint CST-first — **landed** ([lint-cst-first](adr/stage2-s2-04-lint-cst-first.md)): `lint_source_auto` / `lint_document_for_lsp` (**≠** lint-on-CST / ≠ S2-04 `[x]`).
11. **S2-I154:** CST subset if-let-simple — **landed** ([cst-subset-if-let](adr/stage2-s2-04-cst-subset-if-let.md)): `take_if_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
11b. **S2-I155:** CST subset guard-let-simple — **landed** ([cst-subset-guard-let](adr/stage2-s2-04-cst-subset-guard-let.md)): `take_guard_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
11c. **S2-I156:** CST subset if-case-simple — **landed** ([cst-subset-if-case](adr/stage2-s2-04-cst-subset-if-case.md)): `take_if_case_simple` (**≠** KeyPath / ≠ Frontend / ≠ S2-04 `[x]`).
11d. **S2-I157:** CST subset range-in-call-simple — **landed** ([cst-subset-range-in-call](adr/stage2-s2-04-cst-subset-range-in-call.md)): `take_integer_range_simple` (**≠** RangeDyn / ≠ Frontend / ≠ S2-04 `[x]`).
11e. **S2-I158:** CST subset optional-chain-simple — **landed** ([cst-subset-optional-chain](adr/stage2-s2-04-cst-subset-optional-chain.md)): `take_optional_chain_simple` (**≠** force-unwrap / ≠ Frontend / ≠ S2-04 `[x]`).
11f. **S2-I159:** Frontend share-tree evaluation — **locked** ([frontend-share-tree-evaluation](adr/stage2-s2-04-frontend-share-tree-evaluation.md)): second axis = CST→AST lower; deepen stop-line (**≠** FrontendPipeline impl / ≠ S2-04 `[x]`).
11g. **S2-I160:** CST→AST lower probe — **landed** ([cst-lower-probe](adr/stage2-s2-04-cst-lower-probe.md)): `lower_cst_subset_probe` + Parser Module parity (**≠** FrontendPipeline / ≠ S2-04 `[x]`).
11h. **S2-I161:** lint CST-success→lower — **landed** ([lint-lower-path](adr/stage2-s2-04-lint-lower-path.md)): `lint_source_auto` → `lower_cst_subset_probe`; `LWR001`→Parser (**≠** FrontendPipeline / ≠ expand probe / ≠ S2-04 `[x]`).
11i. **S2-I162:** lower probe expand — **landed** ([cst-lower-probe-expand](adr/stage2-s2-04-cst-lower-probe-expand.md)): W791–797 let/var/return (**≠** while/typed / ≠ FrontendPipeline / ≠ S2-04 `[x]`).
11j. **S2-I163:** dual-grammar freeze charter — **locked** ([dual-grammar-freeze-charter](adr/stage2-s2-04-dual-grammar-freeze-charter.md)): stop default `take_*` expand; KPI freeze combined **12381** (**≠** S2-04 `[x]` / ≠ Frontend migrate).
12. **S2-I164:** dedicated S2-04 `[x]` claim — **landed** ([s2-04-claim](adr/stage2-s2-04-claim.md)): scoped plan `[x]`; Stage-2 `[~]` (**≠** Stage-2 exit / ≠ Frontend already on CST). Next = **S2-05** open or Frontend migrate.

## References

- [stage2-s2-04-open](adr/stage2-s2-04-open.md) · S2-I143
- [stage2-s2-04-convergence-strategy](adr/stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-s2-04-cst-reindent-func-body](adr/stage2-s2-04-cst-reindent-func-body.md) · S2-I145
- [stage2-s2-04-cst-reindent-func-nested](adr/stage2-s2-04-cst-reindent-func-nested.md) · S2-I146
- [stage2-s2-04-cst-reindent-func-2to4](adr/stage2-s2-04-cst-reindent-func-2to4.md) · S2-I147
- [stage2-s2-04-cst-reindent-class-stored](adr/stage2-s2-04-cst-reindent-class-stored.md) · S2-I148
- [stage2-s2-04-cst-reindent-env-gate](adr/stage2-s2-04-cst-reindent-env-gate.md) · S2-I149
- [stage2-s2-04-cst-reindent-ga](adr/stage2-s2-04-cst-reindent-ga.md) · S2-I150
- [stage2-s2-04-fmt-write-migration](adr/stage2-s2-04-fmt-write-migration.md) · S2-I151
- [stage2-s2-04-lsp-document-formatting](adr/stage2-s2-04-lsp-document-formatting.md) · S2-I152
- [stage2-s2-04-lint-cst-first](adr/stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-s2-04-cst-subset-if-let](adr/stage2-s2-04-cst-subset-if-let.md) · S2-I154
- [stage2-s2-04-cst-subset-guard-let](adr/stage2-s2-04-cst-subset-guard-let.md) · S2-I155
- [stage2-s2-04-cst-subset-if-case](adr/stage2-s2-04-cst-subset-if-case.md) · S2-I156
- [stage2-s2-04-cst-subset-range-in-call](adr/stage2-s2-04-cst-subset-range-in-call.md) · S2-I157
- [stage2-s2-04-cst-subset-optional-chain](adr/stage2-s2-04-cst-subset-optional-chain.md) · S2-I158
- [stage2-s2-04-frontend-share-tree-evaluation](adr/stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-cst-lower-probe](adr/stage2-s2-04-cst-lower-probe.md) · S2-I160
- [stage2-s2-04-lint-lower-path](adr/stage2-s2-04-lint-lower-path.md) · S2-I161
- [stage2-s2-04-cst-lower-probe-expand](adr/stage2-s2-04-cst-lower-probe-expand.md) · S2-I162
- [stage2-s2-04-dual-grammar-freeze-charter](adr/stage2-s2-04-dual-grammar-freeze-charter.md) · S2-I163
- [stage2-s2-04-claim](adr/stage2-s2-04-claim.md) · S2-I164
- [plan/02](../plan/02-improvement-roadmap.md) S2-04
- W873 / ADR-1217 (`format_source_auto`); ADR-142 / M1-E04 (`syx-cst` start)
