# ADR: Stage-3 S3-04 residual-scope（S3-I70）

**Status:** Accepted (S3-I70) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I69 wall-strategy](stage3-s3-04-wall-strategy.md), Hard OUT interpreter residual is locked at **`fib30_interp_ceiling_s ≈ 1.5–1.8`** while plan/02 still literally says `fib(30) < 0.5 s`. Micro-knives and opt-in Tier2 do not close that gap. This window revises **S3-04 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S3-04 `[x]`, JIT-off &lt; 0.5, Stage-3 exit, production-default JIT, or `Value: Copy`.

## Decision

1. **Scoped acceptance (S3-04 stays `[~]` this window):** Under Hard OUT（≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ production-default JIT / ≠ full ISA rewrite / ≠ further interpreter micro-knives）, honest S3-04 completion means:
   - I57–I69 construct/policy hinges landed;
   - I68 claim-prep scaffolding green (`examples/fib*` · `fib_criterion`);
   - JIT-off primary wall time sits in **`fib30_interp_ceiling_s ≈ 1.5–1.8`** (honest residual, not a lie about &lt;0.5).
2. **Deferred literal &lt;0.5:** plan/open historical **`fib(30) < 0.5 s`** is **deferred / supersede-only** — restoring it as a claim gate requires a **superseding ADR**. It is **not** a current blocker for scoped `[x]`.
3. **Dual-axis honesty:** claim-prep primary remains JIT-off `fib30_s`; `fib30_tier2_s` is evidence only（≠ primary / ≠ production default）.
4. **Ladder next:** **S3-I71** = dedicated **S3-04 scoped `[x]` claim** under this acceptance — still ≠ Stage-3 exit / ≠ S3-05…07 this path’s default.
5. **Non-claims (hard):** ≠ S3-04 `[x]` this window / ≠ claim JIT-off `fib(30)` &lt; 0.5 / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ full ISA rewrite / ≠ interpreter micro-knife / ≠ product VM/JIT semantic knife.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` 0.5 |
| Census | `residual_scope_helper≥1` · Current=I70 |
| Plan | S3-04 row stays `[~]` · scoped摘要 |
| Contract | `s3_04_residual_scope_contract` |

## Consequences

- Residual-scope hinge landed；S3-04 was `[~]` under this window.
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open — lint AST/CST）— not Stage-3 exit.
- Prior: [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md) · S3-I69.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md) · S3-I69
- [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md) · S3-I68
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage1-s1-07-overflow-wrap-scope](stage1-s1-07-overflow-wrap-scope.md)（scoped acceptance 形态先例）
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
