# Multi-diagnostics (S1-08 scoped)

**Status:** CLI check multi-diag scoped GA (S1-I24). See [ADR](adr/stage1-s1-08-cli-multidiag-scope.md). **Not** build/run multi-diag. **Not** stage-1 exit.

## Surfaces

| Surface | Behavior |
|---------|----------|
| **LSP** / `analyze_document_for_lsp` | Uses `Parser::parse_collecting` (+ semantic collecting where wired). Multiple P001 / S00x in one document. |
| **`syx check` (single file)** | `check_file_with_options` → `parse_collecting` + linked compile; on failure may enrich with semantic collecting (≥2 real diags; S045 filtered). |
| **`syx check` (project / directory)** | `check_project_with_options` → walk package `.sy` with `parse_collecting`, aggregate capped diags; if clean, linked `compile_project`. Cross-file parse multi-diag. |
| **build / run / default compile** | Still **first-error**. |

## Nail fixtures

- [`fixtures/multi_syntax_errors.sy`](../fixtures/multi_syntax_errors.sy) — ≥2× P001 (single file).
- [`fixtures/multi_semantic_errors.sy`](../fixtures/multi_semantic_errors.sy) — ≥2 semantic diags (typically S002).
- [`fixtures/multi_project_syntax/`](../fixtures/multi_project_syntax/) — ≥2× P001 across `broken_a.sy` / `broken_b.sy`.
- Alias kept: `fixtures/test_multi_syntax_errors.sy`.

Driver nails: `s1_i21_*` / `s1_i22_*` / `s1_i24_check_project_multi_syntax_*` in `crates/syx-driver/tests/lang`.

## Honest boundary

S1-08 **scoped `[x]`**: LSP + single-file check (parse+semantic thin) + **project check parse multi-diag**. Still deferred: build/run/default-compile multi-diag; project-level semantic collecting matrix. **≠ stage-1 exit**.
