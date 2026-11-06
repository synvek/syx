# Stage-2 parser dual-track census (S2-04)

**ADR:** [stage2-s2-04-open](adr/stage2-s2-04-open.md) · **Window:** S2-I143（baseline）· **Current:** S2-I155  
**Goal (S2-04):** fmt / LSP / lint share one tree; stop dual-maintaining `syx-cst` grammar.  
**This census:** honest baseline + ladder progress — **≠** unified / **≠** S2-04 `[x]`.

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

## Entry map (honest dual track · post-I155)

| Consumer | Entry | Tree |
|----------|-------|------|
| Compile / check / MIR | `FrontendPipeline` / `Parser::parse` | AST |
| lint (**CLI / buffer**) | `lint_source_auto` → CST probe + `lint_source` (AST Module rules) | CST-first **policy**（S2-I153）；rules still AST |
| LSP semantics | driver `analyze_document_for_lsp` / parse helpers | AST |
| fmt **write** (`format_file`) | `format_dry_run` → `format_source_auto` | CST-first; `CST001` → AST（S2-I151） |
| fmt **`--check`** | `format_check_path` → `format_dry_run` → `format_source_auto` | same SSOT |
| fmt **LSP** (`textDocument/formatting`) | `format_document_for_lsp` → `format_dry_run` | same SSOT（S2-I152） |
| fmt pure CST nail | `format_check_path_cst` / `format_cst` concat | CST only |

### Remaining honesty debt (post-I155)

Fmt **三端** + lint **CST-first policy** aligned; CST subset deepened with **if-let-simple** (S2-I154) + **guard-let-simple** (S2-I155). Remaining primary debts: more CST deepen (`if case` / range / …); **lint rules still require AST Module** (≠ shared lossless tree); **dual grammar** (`syx-parser` + `syx-cst`); compile still pure AST; LSP does not publish lint. **≠** S2-04 `[x]`.

## Key symbols (baseline presence)

| Symbol | Crate | Role |
|--------|-------|------|
| `pub fn format_source_auto` | `syx-fmt` | W873 CST-first + `CST001`→AST |
| `pub fn format_source_cst` | `syx-fmt` | CST-only (reindent GA) |
| `pub fn format_dry_run` | `syx-fmt` | write/`--check`/LSP format SSOT |
| `pub fn format_document_for_lsp` | `syx-driver` | S2-I152 LSP buffer format |
| `pub fn lint_source_auto` | `syx-lint` | S2-I153 CST-first lint policy |
| `pub fn lint_document_for_lsp` | `syx-driver` | S2-I153 lint buffer API |
| `fn take_if_let_simple` | `syx-cst` | S2-I154 if-let-simple subset |
| `fn take_guard_let_simple` | `syx-cst` | S2-I155 guard-let-simple subset |
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
11b. **S2-I155:** CST subset guard-let-simple — **landed** ([cst-subset-guard-let](adr/stage2-s2-04-cst-subset-guard-let.md)): `take_guard_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`). More deepen (if-case / range) / Frontend share tree → retire dual `syx-cst` grammar maintenance.
12. Dedicated S2-04 `[x]` claim.

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
- [plan/02](../plan/02-improvement-roadmap.md) S2-04
- W873 / ADR-1217 (`format_source_auto`); ADR-142 / M1-E04 (`syx-cst` start)
