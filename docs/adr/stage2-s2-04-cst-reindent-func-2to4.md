# ADR: Stage-2 S2-04 CST reindent — FunctionDecl 2→4 normalize (S2-I147)

**Status:** Accepted (S2-I147) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I146 nested relative deepen](stage2-s2-04-cst-reindent-func-nested.md), opt-in [`format_cst_reindent`](../../crates/syx-cst/src/lib.rs) inserts missing indent at `(depth - d0 + 1) × 4` but **preserves** existing leading `"  "` / `"    "`. Two-space style remains a semantic debt before reindent GA. This window normalizes **2→4** inside FunctionDecl bodies only.

## Decision

1. **Extend** the same opt-in `format_cst_reindent` (still not default `format_cst`).
2. **2→4 normalize:** while inside an open FunctionDecl (`func_body_depths`, `depth >= d0`), after `Newline`, if the next token is leading `Whitespace` whose lexeme is **spaces-only** and `len == 2 × levels` (`levels = depth - d0 + 1`), emit `levels × 4` spaces instead and skip the original WS token. Already `4 × levels` is left unchanged; other lengths / tabs unchanged.
3. **Skip `}` lines:** if the token after that leading WS is `}`, do **not** normalize (aligned with insert skip).
4. **Keep** I145/I146 missing-WS insert behavior; class stored-field bodies still out of scope; default `format_cst` / `format_source_auto` / `--check` / `format_file` unchanged.
5. **Non-claims (hard):** ≠ reindent GA / ≠ change default `format_cst` / ≠ wire auto/`format_file` / ≠ class field track / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| 2→4 normalize | `format_cst_reindent_*2to4*` / updated unit tests in `syx-cst` |
| 4-space / missing-WS / class | non-regression tests |
| Default concat / write | `s2_04_cst_reindent_func_body` (+ 2to4 ADR nail) |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I148:** [cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) — class stored-member track (**≠** GA).
- Next default = optional auto experiment gate → **reindent GA** → write migration — **not** S2-04 `[x]` / **not** Stage-2 exit.
- Prior: [cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) · [cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md).

## References

- [stage2-s2-04-cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) · S2-I146
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
