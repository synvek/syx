# ADR: Stage-3 S3-04 plan claim（S3-I71）

**Status:** Accepted (S3-I71) · S3-04 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I70 residual-scope](stage3-s3-04-residual-scope.md), Hard OUT construct/policy (I57–I69), claim-prep (I68), and scoped acceptance (I70) are landed. JIT-off `fib30_s` sits in **`fib30_interp_ceiling_s ≈ 1.5–1.8`**; literal **`fib(30) < 0.5`** remains **deferred** (user-agreed). This dedicated window flips the plan checkbox only — aligned with S3-01/S3-02/S3-03 scoped claim. It does **not** exit Stage-3, open S3-05 construction, claim Node-parity, or restore &lt;0.5 as a hard gate.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I56](stage3-s3-04-open.md) · VM-hotspot census |
| Construct I57–I67 | call-args / frame / loadlocal / stack-args / tryfree / locals-prep / frame-swap / begintry / tryfree-dispatch / jit-env / can_stack_bind |
| Claim-prep | [S3-I68](stage3-s3-04-claim-prep.md) · `examples_fib≥1` · `fib_criterion_bench≥1` |
| Wall-strategy | [S3-I69](stage3-s3-04-wall-strategy.md) · `fib30_interp_ceiling_s` · dual-axis |
| Residual-scope | [S3-I70](stage3-s3-04-residual-scope.md) · scoped acceptance · deferred &lt;0.5 |
| Wall KPI | census `fib30_s` ∈ ceiling ≈1.5–1.8（JIT off） |
| Contract | `s3_04_claim_contract` · `s3_04_vm_hotspot_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-05…07 construct this window · **≠** claim literal &lt;0.5 |

## Scoped `[x]` meaning

Under Hard OUT（≠ `Value: Copy` / ≠ remove `locals_frames` / ≠ production-default JIT / ≠ full ISA rewrite / ≠ further interpreter micro-knives）, **VM hotspot** workstream is complete when I57–I70 hinges + I68 scaffolding hold and JIT-off `fib30_s` honestly sits in **`fib30_interp_ceiling_s ≈ 1.5–1.8`**, with `fib_criterion` (fib20) green — **not** 「`fib(30) < 0.5 s`」 / **not** 「Node/Python parity」 / **not** 「Stage-3 exit」 / **not** 「production-default JIT」. Restoring literal &lt;0.5 as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-04 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR（checklist **4/7 ≈ 57%**）.
3. Update VM-hotspot census Current=I71 · **Claimed: scoped `[x]`**；next default = **S3-I72 S3-05 open** — **≠** Stage-3 exit / **≠** open S3-05 construction this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 open or construct this window / ≠ claim JIT-off `fib(30)` &lt; 0.5 / ≠ `Value: Copy` / ≠ production-default JIT / ≠ remove `locals_frames` / ≠ full ISA rewrite / ≠ interpreter micro-knife / ≠ product VM/JIT semantic knife.

## Consequences

- S3-04 plan-claimed `[x]`; Stage-3 stays `[~]`（checklist **4/7 ≈ 57%**）.
- Default next = **S3-I72**（S3-05 open — lint AST/CST）— not Stage-3 exit.
- Prior: [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md) · S3-I70 · open [S3-I56](stage3-s3-04-open.md).
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md) · S3-I70
- [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md) · S3-I69
- [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md) · S3-I68
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-s3-03-claim](stage3-s3-03-claim.md) · S3-I55（claim 窗形态）
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
