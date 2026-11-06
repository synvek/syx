# ADR: Stage-2 S2-04 CST reindent — class stored-member track (S2-I148)

**Status:** Accepted (S2-I148) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I147 FunctionDecl 2→4 normalize](stage2-s2-04-cst-reindent-func-2to4.md), opt-in [`format_cst_reindent`](../../crates/syx-cst/src/lib.rs) covers FunctionDecl bodies (insert + nested relative + 2→4) but deliberately skips **class stored-member** lines. Completing that track is the next ladder step before an optional env gate or reindent GA.

## Decision

1. **Extend** the same opt-in `format_cst_reindent` (still not default `format_cst`).
2. **Class track:** add `class_body_depths` + `expect_class_lbrace`. On `Class` set expect; on matching `{` push absolute depth; on matching `}` pop.
3. **Newline:** prefer active `func_body_depths` (methods). Else, if inside `class_body_depths` with `depth >= c0`, apply the same missing-WS insert and 2→4 normalize with `levels = depth - c0 + 1` (still skip `}` lines).
4. **Unchanged:** default `format_cst` / `format_source_auto` / `--check` / `format_file`; no env gate; init/deinit bodies and extension out of scope.
5. **Non-claims (hard):** ≠ reindent GA / ≠ change default `format_cst` / ≠ env gate / ≠ wire auto/`format_file` / ≠ init/deinit body / ≠ extension / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Class stored indent | `format_cst_reindent_*class*` unit tests |
| Method still func-track | class+method fixture |
| Default concat / write | `s2_04_cst_reindent_func_body` (+ class-stored ADR nail) |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I149:** [cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) — `SYX_FMT_CST_REINDENT` experiment gate (**≠** GA).
- Next default = **reindent GA** → write migration — **not** S2-04 `[x]` / **not** Stage-2 exit.
- Prior: [cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) · [cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md).

## References

- [stage2-s2-04-cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) · S2-I147
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
