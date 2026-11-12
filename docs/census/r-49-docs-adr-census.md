# R-49 文档断链 / ADR 现行集 census

**Date:** 2026-11-12  
**Scope:** D6 version → 0.2.0; D4 current ADR shortlist; scoped 断链↓ (entry + UsageGuide).  
**Scripts:** [`scripts/r49_docs_link_check.py`](../../scripts/r49_docs_link_check.py) · [`scripts/r49_docs_adr_census.sh`](../../scripts/r49_docs_adr_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-48 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-49** | **exited** — docs/ADR hygiene (numbered ledger closed) |
| Park | JIT Suite-1 / `func…async`；≠ 全量 274 ADR 合并 / ≠ 724 断链清零 |

## Root cause

Public docs still advertised **117.0 / Phase 139** while Cargo is **0.2.0**; ADR sprawl buried living decisions in a 4000+ line historical ledger with hundreds of retired `native/`/`lang/` 404s; no scoped link gate ([todo-005](../../plan/todo-005.md) D4–D6).

## Delivery

1. Version banners: [`docs/README.md`](../../docs/README.md), [`docs/zh/README.zh-CN.md`](../../docs/zh/README.zh-CN.md), [`books/README.md`](../../books/README.md) → **0.2.0**.
2. Current shortlist: [`docs/adr/current.md`](../../docs/adr/current.md) (**15** living pointers); [`docs/adr/README.md`](../../docs/adr/README.md) points here first.
3. Historical honesty on [`architecture-decisions.md`](../../docs/adr/architecture-decisions.md) (native/lang 404s unrepaired by design).
4. UsageGuide en+zh: retire missing `examples/main.sy` / `math.sy` / `module_main.sy` / `lang/qs1-*` links.
5. Scoped link check: entry + UsageGuide **broken=0**.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `docs/README` version | **0.2.0**; no current `117.0`/`Phase 139` | **pass** |
| current shortlist rows | **10–20** | **15** |
| scoped_broken_total | **0** | **0** |
| `sy,output` | **12/12** | **12/12** |
| browser / wasm | **0** / **46/46** | **0** / **46/46** |
| Object / simd | **≤180/10** / **≥159** | **180/10** / **159** |

## Non-claims

- ≠ merge/archive all **274** ADR files
- ≠ repair all ~**724** docs 404s (esp. mega-ADR native/lang)
- ≠ restore deleted `docs/native/`
- ≠ zero public `plan/` citations
- ≠ D3 census merge / JIT / `func…async`

## References

- [`r-48-credibility-diet-census`](r-48-credibility-diet-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · numbered ledger closed after R-49
