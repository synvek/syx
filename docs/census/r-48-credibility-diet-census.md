# R-48 credibility 文本断言瘦身 census

**Date:** 2026-11-12  
**Scope:** Delete Stage-3/4 Markdown-only claim-chain contracts; bind D2 claims to existing executable nails.  
**Script:** [`scripts/r48_credibility_diet_census.sh`](../../scripts/r48_credibility_diet_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-47 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-48** | **exited** — MD-only claim diet slice (≠ delete all ~168 contracts / D3 merge) |
| Open | R-49；JIT Suite-1 / `func…async` **park** |

## Root cause

Credibility `*_contract.rs` modules asserted ADR/census/plan Markdown strings (`S3-I103`, `[x]`, `4/5`, historical `<0.15`) as if they were product capability gates ([todo-005](../../plan/todo-005.md) D1/D2).

## Delivery

Deleted **8** Markdown-only claim modules and unwired `mod` lines in [`crates/syx-driver/tests/credibility/main.rs`](../../crates/syx-driver/tests/credibility/main.rs):

| Cluster | Files removed |
|---------|----------------|
| sequence_collection | `*_census` / `*_claim` / `*_residual_scope` |
| stdlib_cache | `*_census` / `*_claim` |
| structured_concurrency (2nd cluster) | `*_census` / `*_claim` / `*_residual_scope` |

### D2 claim → executable nail

| Claim | Was | Now (executable) |
|-------|-----|------------------|
| **S4-04** Sequence/Collection / custom Iterator | MD claim chain | [`crates/syx-driver/tests/lang/protocol_iterator.rs`](../../crates/syx-driver/tests/lang/protocol_iterator.rs) + [`r-33-iterator-witness-census`](r-33-iterator-witness-census.md) |
| **S3-03** stdlib / cold-start narrative | MD claim + historical `<0.15` | [`scripts/r40_coldstart_census.sh`](../../scripts/r40_coldstart_census.sh) + [`r-40-coldstart-census`](r-40-coldstart-census.md) (honest wall clock; ≠ revive MD claim) |

Kept: executable / source-shape contracts (`r20`…`r29`, etc.).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MD-only modules deleted | **≥8** | **8** |
| knife `mod` residual in `main.rs` | **0** | **0** |
| credibility test binary | compiles / green filter | **ok** (`system_import_smoke`) |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ delete all credibility contracts / D1 full 142 wipe
- ≠ D3 merge ~35 `s*_census.sh`
- ≠ R-49 ADR shortlist / link repair / `docs/README` 0.2.0
- ≠ JIT unpark / `func…async` / simd wipe

## References

- [`r-47-package-api-census`](r-47-package-api-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-48 → next **R-49**
