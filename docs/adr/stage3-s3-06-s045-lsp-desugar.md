# ADR: Stage-3 S3-06 S045 LSP desugar parity（S3-I79）

**Status:** Accepted (S3-I79) · S3-06 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I78 S3-06 open](stage3-s3-06-open.md), census inventory used `syx check` as a proxy while the real editor path is `analyze_document_for_lsp`. Honest probe: top-level `print(1)` is **check-green** but LSP emits **S045**, because Prepared options hard-coded `desugar_top_level: false` while compile uses [`should_desugar_top_level_stmts`](../../crates/syx-driver/src/thin_host.rs). This window lands the **first construct hinge**: LSP desugar parity with compile. It does **not** claim S3-06 `[x]`, zero book-fence false positives, rename/sig/hover knives, or Stage-3 exit.

## Decision

1. In [`analyze_document_for_lsp`](../../crates/syx-driver/src/lsp_ide.rs), set `FrontendOptions.desugar_top_level` via **`should_desugar_top_level_stmts(path, &manifest)`** when `path` is `Some`; untitled (`None`) → **`true`** (freestanding script parity). Annotate **S3-I79**.
2. Do **not** change `syx-lsp` protocol surface this window — `semantic_recheck_diagnostics` benefits through `analyze_document_for_lsp`.
3. Nail with contract + census KPI flip (`s045_lsp_desugar_nail_count=0`, book fixture semantic mismatch=0) + ladder Current=**S3-I79**.
4. **Non-claims (hard):** ≠ S3-06 `[x]` / ≠ 书中块零误报宣称 / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-07 / ≠ rename·signature·hover 产品刀 / ≠ 全 fence LSP sweep / ≠ reopen S3-05 lint / S3-04 fib&lt;0.5 / ≠ language semantics / ≠ change `diagnostics_for_source` this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Desugar hinge | `should_desugar_top_level_stmts` · `S3-I79` in [`lsp_ide.rs`](../../crates/syx-driver/src/lsp_ide.rs) |
| Nail | top-level `print(1)` → LSP semantic codes **no S045** |
| Book fixtures | sample `*_book_*.sy` mismatch **0** |
| Contract | `s3_06_s045_lsp_desugar_contract` |
| Census | Current=**S3-I79** · Claimed:** no** |

## Consequences

- S045 desugar false-positive hinge closed；S3-06 later claimed `[x]` by S3-I81；Stage-3 stays `[~]`（**6/7**）.
- **S3-I80 landed** as [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md)（scoped acceptance · deferred rename/sig/fence）。
- **S3-I81 landed** as [stage3-s3-06-claim](stage3-s3-06-claim.md)（S3-06 `[x]` scoped claim）。
- Default next = **S3-I82**（S3-07 open）— not Stage-3 exit.
- Prior: [stage3-s3-06-open](stage3-s3-06-open.md) · S3-I78.
- LSP-book census Current → **S3-I81** · Claimed scoped `[x]`.

## References

- [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md) · S3-I80
- [stage3-s3-06-open](stage3-s3-06-open.md) · S3-I78
- [stage3-lsp-book-census](../stage3-lsp-book-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-06
- [`crates/syx-driver/src/lsp_ide.rs`](../../crates/syx-driver/src/lsp_ide.rs)
- [`crates/syx-driver/src/thin_host.rs`](../../crates/syx-driver/src/thin_host.rs)
- [`crates/syx-driver/src/compile_file_context.rs`](../../crates/syx-driver/src/compile_file_context.rs)
