# ADR: Stage-3 S3-04 wall-strategy（S3-I69）

**Status:** Accepted (S3-I69) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I68 claim-prep](stage3-s3-04-claim-prep.md), Call/locals surface taxes and claim scaffolding are landed while JIT-off `fib(30)` still ≈**1.71 s** (target **0.5**, ~**3.4×**). Further interpreter micro-knives cannot close that gap under Hard OUTs. This window registers the **wall-strategy policy** (interpreter residual ceiling + allowed strategies + dual-axis KPIs). It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, Stage-3 exit, production-default JIT, or `Value: Copy`.

## Decision

1. **Interpreter residual ceiling (JIT off):** Under Hard OUT（≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ production-default JIT / ≠ full ISA rewrite）, honest **`fib30_interp_ceiling_s ≈ 1.5–1.8`** (machine census ≈1.71). **Interpret-only &lt; 0.5 is currently unreachable.**
2. **Claim primary axis = JIT-off `fib30_s`** (align plan「VM 热点」). Open a dedicated claim window **only when** primary approaches 0.5 — this window does **not**.
3. **Allowed strategy whitelist:**
   - (a) Interpreter micro-knives: **ROI exhausted** — no further opcode/frame micro-knives.
   - (b) Benchmark/CI secondary axis: `SYX_JIT_TIER2=1` → record `fib30_tier2_s` (evidence only; **≠** claim primary; **≠** production default).
   - (c) Next **S3-I70 residual-scope**: revise S3-04 acceptance semantics (scoped residual / dual-axis honesty) to pave a later `[x]`.
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ full ISA rewrite / ≠ product VM/JIT semantic knife.

## Evidence

| Check | Evidence |
|-------|----------|
| Ceiling | `fib30_interp_ceiling_s` · census `fib30_s` |
| Dual axis | `fib30_s` (JIT off) · `fib30_tier2_s` (opt-in or `na`) |
| Census | `wall_strategy_helper≥1` · Current=I69 |
| Contract | `s3_04_wall_strategy_contract` |

## Consequences

- Wall-strategy hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope：scoped acceptance）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md) · S3-I68.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md) · S3-I68
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
