# R-42 simd Object 159 honesty census

**Date:** 2026-11-12  
**Scope:** R-27/R-29 park — charter-first; inventory **simd Object residual** without wipe.  
**Script:** [`scripts/r42_simd_object_census.sh`](../../scripts/r42_simd_object_census.sh)  
**ADR:** [`r-42-simd-object-charter`](../adr/r-42-simd-object-charter.md)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-41 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**) |
| **R-42** | **exited** — simd Object **charter** + cluster census; residual **159** retained |
| LANG-POLY | parked → R-43+ |

## Charter (this window)

Deliver honest clustering + ADR knife gates. **Do not** rewrite [`simd.sy`](../../packages/core/simd.sy) Object signatures. **Do not** lower Object ceiling. Future façade slices require charter green lights.

## Cluster (public `packages/core/simd.sy`)

| Axis | Object tokens |
|------|-------------:|
| **Total `simd_object_residual`** | **159** |
| SIMD2 | 21 |
| SIMD4 | 43 |
| SIMD8 | 10 |
| SIMD16 | 85 |

| Family | Object tokens |
|--------|-------------:|
| fromArray | 4 |
| arith (`add*`/`sub*`/`mul*`/`div*`) | 48 |
| load_store | 12 |
| reduce | 4 |
| shuffle | 10 |
| bitwise | 30 |
| cmp_minmax | 39 |
| lane (`extract`/`replace`/`splat`) | 3 |
| other | 9 |

## Host / type gap

| Layer | Finding |
|-------|---------|
| Public façade | Vector args/returns mostly `Object` (`SIMD2`…`SIMD16`) |
| [`system_host.sy`](../../packages/internal/system_host.sy) `simd_*` | ~55 funcs; **~51** still `-> Object`; many args typed `Float64`/`String` placeholders (≠ honest vector types) |
| Semantic | `TypeId::Simd { lanes, element }` exists ([`semantic_types.rs`](../../crates/syx-semantic/src/semantic_types.rs)) |
| Runtime | `Value::Simd` + MIR simd ops exist |
| Gap | Public Object bag **≠** `TypeId::Simd` / host honesty — knife blocked until charter §允许动刀 |

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `simd_object_residual` | **≥159** (no wipe) | **159** |
| `object_sig_residual_count` | **≤180** | **180** |
| `object_sig_residual_files` | **≤10** | **10** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| ADR charter | present | **yes** |

## Non-claims

- ≠ 159→0 / `Sendable` wipe this window
- ≠ simd product generics GA
- ≠ json Object bags
- ≠ LANG-POLY / JIT
- ≠ inventory ceiling drop

## References

- [`r-41-orderedset-census`](r-41-orderedset-census.md) · left simd parked
- [`r-27-collections-object-census`](r-27-collections-object-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-42 → next **R-43**
