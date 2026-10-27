# Hosted Full-Site Capability Vectors (Wave A W379 thin + W380 deepen + W381 exit)

> **Status:** **Hosted full-site thin-open / inventory signed** (ADR-723 / W379) + **Hosted same-source deepen signed** (ADR-724 / W380) + **Hosted product met exit signed** (ADR-725 / W381) — Deployable same-source hosted consumer **met (narrow)** via inventory + deepen (partial sufficient) + exit charter; Hosted full site / marketing **still unmet**.  
> **Exit cross-index:** [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725.  
> **Marketing cross-index:** [`marketing-deploy-vectors.md`](marketing-deploy-vectors.md) / ADR-733–735 + [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735 — marketing product **met (narrow)**；Static tree **partial met**；Hosted full site **still unmet**.  
> **Website cross-index:** [`website-capability-vectors.md`](website-capability-vectors.md) / ADR-720–722 — independent website consumer **met (narrow)**; Hosted full site **unmet**.  
> **Website exit cross-index:** [`website-capability-exit.md`](website-capability-exit.md) / ADR-722 — Exit ≠ hosted full site (website axis).  
> **A-10 cross-index:** [`a10-capability-source.md`](a10-capability-source.md) / ADR-702–705 — A-10/M0-C05 `[x]` via scope narrow.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> Roadmap A-10 remains `[x]`. **Product met (narrow) ≠ marketing full site.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **Website product met (narrow)** | **cross-met** (W378) | inventory + deepen + exit；`capabilities_for_website` + HTML nail |
| **Website HTML same-source helper** | **cross-met** (W377) | `website_capabilities.rs` → `run_capabilities_report()` only；**not** a deployable site |
| **Hosted inventory thin** | **met** (W379) | this document + `hosted_site_thin.rs` + ADR-723 |
| **Marketing deploy inventory thin** | **cross-met** (W389) | [`marketing-deploy-vectors.md`](marketing-deploy-vectors.md) + ADR-733 |
| **Marketing same-source deepen** | **cross-partial** (W390) | `marketing_capabilities.rs` + `marketing_deploy_deepen.rs` + ADR-734 |
| **Marketing product met (narrow)** | **cross-met** (W391) | [`marketing-deploy-exit.md`](marketing-deploy-exit.md) + ADR-735 |
| **Static / marketing deploy tree** | **partial met** (W390) | temp/fixture HTML only；≠ production Pages/static；exit ≠ marketing deploy |
| **Deployable same-source hosted consumer** | **met** (W381 narrow) | inventory W379 + deepen W380 (partial sufficient) + exit ADR-725；`hosted_capabilities.rs` + fixture nail |
| **Hosted full site** | **unmet** | marketing / production deploy deferred；product met (narrow) ≠ marketing full site |

## Thin-open + deepen + exit definition

Hosted **thin-open / inventory** (W379) indexes the deploy/marketing gap. Hosted **deepen** (W380) adds a minimal full-HTML page + temp/fixture write nail. Hosted **product met** (W381) signs exit under scope narrow (deepen partial met sufficient). Product met ≠ marketing full site.

**Evidence nails:** `hosted_site_thin.rs` + `hosted_site_deepen.rs` + `hosted_site_exit.rs` + `hosted_capabilities.rs` + ADR-723–725.

## Frozen

- Hosting / deploying a full production / marketing website this window
- Writing hosted HTML into `docs/` or `books/` marketing trees
- Claiming Hosted full site / marketing **met** (narrow product met only)
- Adding a new `capabilities.toml` row with `status=implemented` for website/hosted
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)
- Rolling back website product **met (narrow)** / hosted inventory thin

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
