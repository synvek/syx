# ADR: Stage-2 S2-04 CST reindent — FunctionDecl body first knife (S2-I145)

**Status:** Accepted (S2-I145) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I144 convergence strategy](stage2-s2-04-convergence-strategy.md), CST reindent is the hard prerequisite before migrating fmt **write** (`format_file`) onto `format_source_auto`. Today's [`format_cst`](../../crates/syx-cst/src/lib.rs) only concatenates lexemes (byte-identical trivia stream) and does **not** reindent. Mutating the default `format_cst` would change `--check` Production Scope for any zero-indent function bodies. This window lands an **opt-in** first knife only.

## Decision

1. **Add** opt-in `syx_cst::format_cst_reindent(&CstRoot) -> String`.
2. **Leave** default `format_cst` as lexeme concatenation; leave `format_source_auto` / `--check` / `format_file` unchanged.
3. **Slice (this window):** inside a **FunctionDecl** body at brace depth **1**, when a line after `Newline` has **no** leading `Whitespace` before the next non-trivia token (and that token is not `}`), insert `"    "` (4 spaces). Existing `"  "` / `"    "` leading whitespace is **not** rewritten. Nested braces (`depth > 1`) and **class** bodies are **out of scope**.
4. **Non-claims (hard):** ≠ reindent GA / ≠ change default `format_cst` / ≠ change `--check` Product Scope / ≠ change `format_file` / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Opt-in API | `format_cst_reindent` in `syx-cst` |
| Default concat preserved | `format_cst` + unit roundtrip |
| Contract | `s2_04_cst_reindent_func_body` (driver tests) |
| Write still AST | existing `s2_04_fmt_write_check_strategy` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I146:** [cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) — nested relative depth + stack (**≠** GA).
- **S2-I147:** [cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) — FunctionDecl 2→4 normalize (**≠** GA).
- Next default = class track → optional auto experiment gate → **reindent GA** → write migration — **not** S2-04 `[x]` / **not** Stage-2 exit.
- Open: [stage2-s2-04-open](stage2-s2-04-open.md) · [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-s2-04-open](stage2-s2-04-open.md) · S2-I143
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
