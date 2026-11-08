# ADR: Stage-3 S3-05 claim-prep（S3-I75）

**Status:** Accepted (S3-I75) · S3-05 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I74 Struct/Extension type-member](stage3-s3-05-struct-extension-type-member.md), L007 type-member coverage spans Class/Actor/Struct/Extension and L012 is column-0 only; canonical fixtures report **L007=0 / L012=0**. Construct knives for the false-positive axis are exhausted under Hard OUTs. This window lands **claim scaffolding** (criteria draft + fixture registry + census helpers) so S3-05 can eventually close. It does **not** claim S3-05 `[x]`, AST-ize L010–L012, full CST lint, or Stage-3 exit.

## Decision

1. Publish **scoped `[x]` claim criteria draft** (this window **unmet** — dedicated claim window still required):
   - **Primary:** `class_demo` / `struct_demo` / `struct_book_point` → **L007=0** and **L012=0** (false-positive axis).
   - **Structural hinges hold:** I73 (`in_type_member` · column-0 L012) · I74 (Struct/Extension recurse) remain in `syx-lint`.
   - **Census green:** `scripts/s3_05_lint_ast_census.sh` emits helpers + zero fixture counts.
2. **Deferred (cite only; I76 residual-scope writes acceptance):** L010–L011 text hybrid · L012 Stmt AST · full CST lint / S2-04 Frontend rewrite.
3. Register fixtures (no new product files): [`examples/class_demo.sy`](../../examples/class_demo.sy) · [`examples/struct_demo.sy`](../../examples/struct_demo.sy) · [`crates/syx-driver/tests/fixtures/lang/struct_book_point.sy`](../../crates/syx-driver/tests/fixtures/lang/struct_book_point.sy).
4. Nail with `claim_prep_helper` / `scoped_fixture_helper` census KPIs + `s3_05_claim_prep_contract`.
5. **Ladder:** next = **S3-I76 residual-scope** → **S3-I77 scoped `[x]` claim**.
6. **Non-claims (hard):** ≠ S3-05 `[x]` / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-06…07 / ≠ reopen S3-04 fib&lt;0.5 / ≠ change `syx-lint` rule behavior this window / ≠ AST-ize L010–L012 / ≠ CST lint rewrite / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Criteria draft | this ADR · Primary / Deferred |
| Fixtures | `class_demo` · `struct_demo` · `struct_book_point` |
| Census | `claim_prep_helper≥1` · `scoped_fixture_helper≥1` · Current=**S3-I75** |
| Contract | `s3_05_claim_prep_contract` |
| Prior knives | [I73](stage3-s3-05-l007-l012-false-positive.md) · [I74](stage3-s3-05-struct-extension-type-member.md) |

## Consequences

- Claim-prep hinge landed；S3-05 stays `[~]`；Stage-3 stays `[~]`（**4/7**）.
- **S3-I76 landed** as [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md)（scoped acceptance · deferred hybrid）。
- Default next = **S3-I77** scoped claim — not Stage-3 exit.
- Prior: [stage3-s3-05-struct-extension-type-member](stage3-s3-05-struct-extension-type-member.md) · S3-I74.
- Lint-ast census Current → **S3-I76**.

## References

- [stage3-s3-05-struct-extension-type-member](stage3-s3-05-struct-extension-type-member.md) · S3-I74
- [stage3-s3-05-l007-l012-false-positive](stage3-s3-05-l007-l012-false-positive.md) · S3-I73
- [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72
- [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md) · S3-I68（形态模板）
- [stage3-lint-ast-census](../stage3-lint-ast-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
