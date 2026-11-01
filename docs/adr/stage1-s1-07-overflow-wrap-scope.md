# ADR: S1-07 scoped = integer overflow wrap SSOT (≠ trap GA / ≠ stage-1 exit)

**Status:** Accepted (S1-I25)  
**Date:** 2026-11-01  
**Context:** Stage-1 backlog item S1-07 was titled “整数溢出默认 trap” (Swift-like). Engineering reality and the language guide (Ch03 / Ch30) already document **two's-complement wrapping** on VM / MIR / WASM paths; debug **R011** was never delivered; **`&+` / `&-` / `&*`** are unimplemented; **`-Int.min`** wraps. Leaving S1-07 as `[~]` solely for undelivered trap contradicted “make claimed behavior true.”

## Decision

1. **S1-07 acceptance is scoped.** Completing S1-07 means:
   - Current integer arithmetic **wraps** (two's complement) on the documented default paths;
   - Language-guide honesty (Ch03 / Ch30 en/zh) matches that behavior;
   - A permanent nail asserts VM `Int` max+1 wraps and does **not** trap.
2. **Out / defer (not required for S1-07 `[x]`):**
   - Swift-default overflow **trap** on `+` / `-` / `*`;
   - Debug **R011** overflow Display / panic;
   - Wrapping operators **`&+` / `&-` / `&*`**;
   - Unary **`-Int.min` trap GA**.
3. **Stage-1 exit honesty.** Scoped S1-07 does **not** authorize claiming Stage-1 exit (examples 拉满 / 三后端, mono, diamond still owed or deferred). **Wrap SSOT ≠ trap GA ≠ stage-1 exit.**

## Consequences

- `plan/02` marks **S1-07 `[x]` (scoped)**; narrative must keep “≠ trap GA / ≠ R011 / ≠ `&+` / ≠ 阶段1退出”.
- Flipping default `+` to checked/trap requires a **superseding** ADR and book KPI revalidation.

## References

- [`books/language-guide/03-basic-operators.md`](../../books/language-guide/03-basic-operators.md)
- [`books/language-guide/30-advanced-operators.md`](../../books/language-guide/30-advanced-operators.md)
- [`docs/lang/a04-edge-semantics.md`](../lang/a04-edge-semantics.md) (historical A-04 overflow floor)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · S1-07
