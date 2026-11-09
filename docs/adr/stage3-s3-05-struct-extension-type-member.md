# ADR: Stage-3 S3-05 Struct/Extension type-member knife（S3-I74）

**Status:** Accepted (S3-I74) · S3-05 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I73 L007/L012 knife](stage3-s3-05-l007-l012-false-positive.md), `Class`/`Actor` members recurse with `in_type_member: true`, but `Decl::Struct` / `Decl::Extension` still fell through `_ => {}` — a **blind spot** (members never walked; not an intentional L007 skip). This window extends the same type-member context to Struct `members` and Extension `methods`. It does **not** claim S3-05 `[x]`, AST-ize L010–L012, or exit Stage-3.

## Decision

1. Recurse `Decl::Struct { members }` with `in_type_member: true` (same arm family as Class/Actor).
2. Recurse `Decl::Extension { methods }` with `in_type_member: true` so extension-local decls stay typed-member context (and method rules like L001 still run).
3. Nail with unit test + `examples/struct_demo.sy` / `struct_book_point.sy` L007=0 + census KPIs + construct contract.
4. **Non-claims (hard):** ≠ S3-05 `[x]` / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-06…07 / ≠ reopen S3-04 fib&lt;0.5 / ≠ L012 Stmt AST-ize / ≠ AST-ize L010–L011 / ≠ CST lint rewrite / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Struct recurse | `Decl::Struct { members }` · `S3-I74` in [`syx-lint/src/lib.rs`](../../crates/syx-lint/src/lib.rs) |
| Extension recurse | `Decl::Extension { methods }` · `in_type_member: true` |
| Fixture | `syx lint examples/struct_demo.sy` · `struct_book_point.sy` → no L007 |
| Unit test | `struct_field_and_extension_no_l007` |
| Contract | `s3_05_struct_extension_contract` |
| Census | Current=**S3-I74** · `struct_book_point_l007_count=0` |

## Consequences

- Struct/Extension type-member blind spot closed；S3-05 stays `[~]`；Stage-3 stays `[~]`（**4/7**）.
- **S3-I75 landed** as [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md)（claim 判据草案 + fixture 注册）。
- **S3-I76 landed** as [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md)（residual-scope）。
- **S3-I77 landed** as [stage3-s3-05-claim](stage3-s3-05-claim.md)（S3-05 `[x]` scoped claim）。
- Default next = **S3-I78**（S3-06 open）— not Stage-3 exit.
- Prior: [stage3-s3-05-l007-l012-false-positive](stage3-s3-05-l007-l012-false-positive.md) · S3-I73.
- Census Current → **S3-I77** · Claimed scoped `[x]`.

## References

- [stage3-s3-05-l007-l012-false-positive](stage3-s3-05-l007-l012-false-positive.md) · S3-I73
- [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72
- [stage3-lint-ast-census](../census/stage3-lint-ast-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
