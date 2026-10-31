# Multi-diagnostics (S1-08 thin honesty)

**Status:** thin start (S1-I19). **Not** S1-08 complete. **Not** CLI product GA.

## Surfaces

| Surface | Behavior |
|---------|----------|
| **LSP** / `analyze_document_for_lsp` | Uses `Parser::parse_collecting` (+ semantic collecting where wired). Can report **multiple** P001-class syntax errors in one document. |
| **`syx check` / default compile** | Still **first-error** (`Parser::parse`). One hard parse failure stops the pipeline. |

## Nail fixture

- [`fixtures/multi_syntax_errors.sy`](../fixtures/multi_syntax_errors.sy) — two funcs missing `{` bodies; expected ≥2× P001 under `parse_collecting`.
- Alias kept: `fixtures/test_multi_syntax_errors.sy` (same content).

Parser unit test: `parse_collecting_fixture_multi_syntax_errors_sy` in `crates/syx-parser`.

## Honest boundary

Delivering S1-08 `[x]` requires `syx check` (and/or a documented flag) to surface capped multi-diagnostics on the default CLI path. Until then: **GA face for multi-diag = LSP**; **CLI = first-error**.
