# Multi-diagnostics (S1-08 thin honesty)

**Status:** thin CLI parse **+ semantic** GA started (S1-I22). **Not** S1-08 complete. Project check / build·run multi-diag still out.

## Surfaces

| Surface | Behavior |
|---------|----------|
| **LSP** / `analyze_document_for_lsp` | Uses `Parser::parse_collecting` (+ semantic collecting where wired). Can report **multiple** P001 / S00x in one document. |
| **`syx check` (single file)** | Uses `check_file_with_options` → `parse_collecting` + full linked compile. On linked **failure**, thin `expand_macros` + `analyze_collecting*` may replace the first-error with **≥2** real semantic diags (S045 from undedesugared top-level stmts filtered). Linked compile success always wins. |
| **`syx check` (project) / build / run / default compile** | Still **first-error** (`Parser::parse` / first semantic abort). One hard failure stops the pipeline. |

## Nail fixtures

- [`fixtures/multi_syntax_errors.sy`](../fixtures/multi_syntax_errors.sy) — two funcs missing `{` bodies; expected ≥2× P001.
- [`fixtures/multi_semantic_errors.sy`](../fixtures/multi_semantic_errors.sy) — two independent undefined identifiers; expected ≥2 semantic diags (typically S002).
- Alias kept: `fixtures/test_multi_syntax_errors.sy` (same content as syntax nail).

Parser unit test: `parse_collecting_fixture_multi_syntax_errors_sy` in `crates/syx-parser`.  
Driver nails: `s1_i21_check_file_multi_syntax_*` / `s1_i22_check_file_multi_semantic_*` in `crates/syx-driver/tests/lang`.

## Honest boundary

S1-08 remains `[~]`: CLI **single-file** parse+semantic multi-diag thin face has started; delivering S1-08 `[x]` still needs project-level and/or full-pipeline (build·run) coverage. Until then: **multi-diag = LSP + single-file `syx check`**; **project check / build / run = first-error**.
