# ADR: Stage-3 S3-04 claim-prep（S3-I68）

**Status:** Accepted (S3-I68) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I67 can_stack_bind cache](stage3-s3-04-can-stack-bind-cache.md), Call/locals surface taxes are exhausted while `fib(30)` still ≈**1.7–1.8 s** (target **0.5**, ~**3.4×**). Further micro-knives cannot close that gap under Hard OUTs. This window lands **claim scaffolding** (examples + criterion + draft criteria) so S3-04 can eventually close when wall allows. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, Stage-3 exit, default JIT, or `Value: Copy`.

## Decision

1. Add [`examples/fib.sy`](../../examples/fib.sy) — `fib(n: 30)` wall repro (align census / claim wall axis).
2. Add [`examples/fib20.sy`](../../examples/fib20.sy) + [`crates/syx-driver/benches/fib_criterion.rs`](../../crates/syx-driver/benches/fib_criterion.rs) — CI-friendly criterion on fib(20); **≠** fib(30) inside criterion.
3. **Claim criteria draft** (this window **unmet**):
   - `fib30_s` **&lt; 0.5** (prefer release `syx`)
   - `fib_criterion` bench green
   - I57–I67 structural helpers remain
   - Dedicated S3-04 `[x]` claim window still required
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ full ISA rewrite.

## Evidence

| Check | Evidence |
|-------|----------|
| examples | `examples/fib.sy` · `examples/fib20.sy` · `examples_fib≥1` |
| criterion | `fib_criterion` bench · `fib_criterion_bench≥1` |
| Census | `claim_prep_helper≥1` · Current=I68 |
| Contract | `s3_04_claim_prep_contract` |

## Consequences

- Claim-prep hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy：interp ceiling + 白名单）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md) · S3-I67.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md) · S3-I67
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
