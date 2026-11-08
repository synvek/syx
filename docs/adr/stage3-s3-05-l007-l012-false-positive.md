# ADR: Stage-3 S3-05 L007/L012 false-positive knife（S3-I73）

**Status:** Accepted (S3-I73) · S3-05 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I72 S3-05 open](stage3-s3-05-open.md), census showed [`examples/class_demo.sy`](../../examples/class_demo.sy) emitting **L007=1** (class field `VarGlobal` without init) and **L012=6** (`trim_start` line heuristic on indented `print(`). This window lands the **first construct hinge**: skip L007 inside `Class`/`Actor` members, and restrict L012 to column-0 `print(`. It does **not** claim S3-05 `[x]`, AST-ize L010–L011, CST lint rewrite, or Stage-3 exit.

## Decision

1. Thread `in_type_member: bool` through `lint_decl`; `Decl::Class` / `Decl::Actor` recurse with `true` so stored `VarGlobal` fields **skip L007**. Module-top uninitialized `VarGlobal` still emits L007.
2. Change L012 predicate from `line.trim_start().starts_with("print(")` to **`line.starts_with("print(")`** (column-0 only).
3. Nail with unit tests + census KPI flip (`class_demo_l007_count=0`, `class_demo_l012_count=0`) + construct contract.
4. **Non-claims (hard):** ≠ S3-05 `[x]` / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-06…07 / ≠ reopen S3-04 fib&lt;0.5 / ≠ AST-ize L010–L011 / ≠ CST lint rewrite / ≠ Struct/Extension member context this window / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| L007 guard | `in_type_member` · `S3-I73` in [`syx-lint/src/lib.rs`](../../crates/syx-lint/src/lib.rs) |
| L012 column-0 | `line.starts_with("print(")` · no `trim_start` |
| Fixture | `syx lint examples/class_demo.sy` → no L007/L012 |
| Unit tests | `class_field_and_indented_print_no_l007_l012` · `top_level_var_and_print_still_lint` |
| Contract | `s3_05_l007_l012_contract` · `s3_05_lint_ast_census_contract` |
| Census | Current=**S3-I73** · KPI counts **0** |

## Consequences

- Canonical L007/L012 false positives on `class_demo` cleared；S3-05 stays `[~]`；Stage-3 stays `[~]`（**4/7**）.
- **S3-I74 landed** as [stage3-s3-05-struct-extension-type-member](stage3-s3-05-struct-extension-type-member.md)（Struct/Extension `in_type_member`）。
- **S3-I75 landed** as [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md)（claim-prep）。
- **S3-I76 landed** as [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md)（residual-scope）。
- Default next = **S3-I77** scoped claim — not Stage-3 exit / not S3-05 claim.
- Prior: [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72.
- Lint-ast census Current → **S3-I76**.

## References

- [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72
- [stage3-lint-ast-census](../stage3-lint-ast-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
- [`crates/syx-lint/src/lib.rs`](../../crates/syx-lint/src/lib.rs)
