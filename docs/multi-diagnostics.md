# Multi-diagnostics (S1-08 thin honesty)

**Status:** thin CLI parse GA started (S1-I21). **Not** S1-08 complete. Semantic / project / build·run multi-diag still out.

## Surfaces

| Surface | Behavior |
|---------|----------|
| **LSP** / `analyze_document_for_lsp` | Uses `Parser::parse_collecting` (+ semantic collecting where wired). Can report **multiple** P001-class syntax errors in one document. |
| **`syx check` (single file)** | Uses `check_file_with_options` → `Parser::parse_collecting` (capped). Surfaces **multiple** P001s then exits non-zero. Thin parse multi-diag product face. |
| **`syx check` (project) / build / run / default compile** | Still **first-error** (`Parser::parse`). One hard parse failure stops the pipeline. |

## Nail fixture

- [`fixtures/multi_syntax_errors.sy`](../fixtures/multi_syntax_errors.sy) — two funcs missing `{` bodies; expected ≥2× P001 under `parse_collecting` and single-file `syx check`.
- Alias kept: `fixtures/test_multi_syntax_errors.sy` (same content).

Parser unit test: `parse_collecting_fixture_multi_syntax_errors_sy` in `crates/syx-parser`.  
Driver nail: `s1_i21_check_file_multi_syntax_errors_reports_ge2_p001` in `crates/syx-driver/tests/lang`.

## Honest boundary

S1-08 remains `[~]`: CLI **parse** multi-diag thin face has started; delivering S1-08 `[x]` still needs semantic multi-diag and/or project-level / full-pipeline coverage. Until then: **parse multi-diag = LSP + single-file `syx check`**; **project check / build / run = first-error**.
