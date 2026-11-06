# ADR: Stage-2 S2-04 CST reindent — FunctionDecl nested deepen (S2-I146)

**Status:** Accepted (S2-I146) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I145 FunctionDecl depth-1 first knife](stage2-s2-04-cst-reindent-func-body.md), opt-in [`format_cst_reindent`](../../crates/syx-cst/src/lib.rs) only inserts 4 spaces at the function body's absolute brace depth. Nested braces (`if` / `while` / inner `{`) still leave missing leading whitespace unfixed, and a nested `func` could overwrite the single `func_body_depth` and drop outer tracking. This window **deepens** the same opt-in API only.

## Decision

1. **Extend** opt-in `format_cst_reindent` (same symbol; still not the default `format_cst`).
2. **Relative depth:** while inside an open FunctionDecl body with body-open depth `d0`, for any current brace `depth >= d0`, when a post-`Newline` line lacks leading `Whitespace` before the next non-trivia token (and that token is not `}`), insert `(depth - d0 + 1) × 4` spaces.
3. **Stack:** track FunctionDecl body depths on a stack (push on func `{`, pop on matching `}`) so nested methods restore the outer function.
4. **Unchanged:** existing leading `"  "` / `"    "` not rewritten; class stored-field bodies still out of scope; default `format_cst` / `format_source_auto` / `--check` / `format_file` unchanged; **no** 2→4 normalize.
5. **Non-claims (hard):** ≠ reindent GA / ≠ change default `format_cst` / ≠ wire auto/`format_file` / ≠ 2→4 normalize / ≠ class field track / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Nested relative indent | `format_cst_reindent_*` unit tests in `syx-cst` |
| Depth-1 non-regression | I145 cases still pass |
| Default concat / write | `s2_04_cst_reindent_func_body` (+ nested ADR nail) |
| Stack restore | nested-func unit coverage |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I147:** [cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) — FunctionDecl 2→4 normalize (**≠** GA).
- Next default = class field track → optional auto experiment gate → **reindent GA** → write migration — **not** S2-04 `[x]` / **not** Stage-2 exit.
- Prior: [cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) · [convergence-strategy](stage2-s2-04-convergence-strategy.md).

## References

- [stage2-s2-04-cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) · S2-I145
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
