# ADR: Stage-3 S3-05 residual-scope（S3-I76）

**Status:** Accepted (S3-I76) · S3-05 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I75 claim-prep](stage3-s3-05-claim-prep.md), Primary false-positive fixtures are green and I73/I74 hinges hold, while plan/open still literally aim at 「lint 基于 AST/CST」 with L010–L012 still text-hybrid. This window revises **S3-05 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S3-05 `[x]`, AST-ize L010–L012, full CST lint, or Stage-3 exit.

## Decision

1. **Scoped acceptance (S3-05 stays `[~]` this window):** Under Hard OUT（≠ AST-ize L010–L012 / ≠ full CST lint rewrite / ≠ S2-04 Frontend rewrite / ≠ further false-positive product knives beyond I73–I74）, honest S3-05 completion means:
   - **Primary:** `class_demo` / `struct_demo` / `struct_book_point` → **L007=0** and **L012=0**;
   - **Structural hinges:** I73 (`in_type_member` · column-0 L012) · I74 (Struct/Extension recurse) remain;
   - **Scaffolding:** I75 claim-prep green (`claim_prep_helper` · `scoped_fixture_helper` · census).
2. **Deferred / supersede-only:** L010–L011 text→AST · L012 Stmt AST · full CST lint / dual-track Frontend / 字面「全规则 AST/CST」are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`.
3. **Hybrid honesty:** L001–L009 remain AST; L010–L012 may stay via `lint_source_text` (`lint_rules_text_hybrid=3`) under scoped meaning.
4. **Ladder next:** **S3-I77** = dedicated **S3-05 scoped `[x]` claim** under this acceptance — still ≠ Stage-3 exit / ≠ S3-06…07 this window.
5. **Non-claims (hard):** ≠ S3-05 `[x]` this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-06…07 / ≠ reopen S3-04 fib&lt;0.5 / ≠ change `syx-lint` rule behavior this window / ≠ AST-ize L010–L012 / ≠ CST lint rewrite / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Census | `residual_scope_helper≥1` · Current=**S3-I76** |
| Plan | S3-05 row stays `[~]` · scoped摘要 |
| Contract | `s3_05_residual_scope_contract` |
| Prior | [I75 claim-prep](stage3-s3-05-claim-prep.md) · [I73](stage3-s3-05-l007-l012-false-positive.md) · [I74](stage3-s3-05-struct-extension-type-member.md) |

## Consequences

- Residual-scope hinge landed；S3-05 was `[~]` under this window.
- **S3-I77 landed** as [stage3-s3-05-claim](stage3-s3-05-claim.md)（S3-05 `[x]` scoped claim）。
- Default next = **S3-I78**（S3-06 open — LSP）— not Stage-3 exit.
- Prior: [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md) · S3-I75.
- Census Current → **S3-I77** · Claimed scoped `[x]`.

## References

- [stage3-s3-05-claim](stage3-s3-05-claim.md) · S3-I77
- [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md) · S3-I75
- [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md) · S3-I70（形态模板）
- [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72
- [stage3-lint-ast-census](../census/stage3-lint-ast-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
