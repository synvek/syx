# Marketing Deploy Vectors (Wave A W389 thin + W390 deepen + W391 exit)

> **Status:** **Marketing deploy inventory thin met** (ADR-733 / W389) + **Marketing same-source deepen signed** (ADR-734 / W390) + **Marketing product met exit signed** (ADR-735 / W391) — Marketing product **met (narrow)** via inventory + deepen (partial sufficient) + exit charter; Static/marketing deploy tree **still partial met**; Hosted full site / marketing full site **still unmet**.  
> **Exit cross-index:** [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735.  
> **Hosted cross-index:** [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725 — Deployable hosted consumer **met (narrow)**；Hosted full site **still unmet**.  
> **Website cross-index:** [`website-capability-exit.md`](website-capability-exit.md) / ADR-722 — website product **met (narrow)**；≠ marketing full site.  
> **TLS cross-index:** [`tls-server-ga-exit.md`](tls-server-ga-exit.md) / ADR-732 — honesty floor **met**；TLS GA **still unmet**.  
> **Accept cross-index:** [`tcp-accept-ga-exit.md`](tcp-accept-ga-exit.md) / ADR-729 — accept GA **met (narrow)** unchanged.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> Roadmap A-10 remains `[x]`. Marketing product **met (narrow)**. **Exit ≠ marketing full site.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **Website product met (narrow)** | **cross-met** (W378) | [`website-capability-exit.md`](website-capability-exit.md) + ADR-722 |
| **Deployable hosted consumer** | **cross-met** (W381 narrow) | [`hosted-site-exit.md`](hosted-site-exit.md) + ADR-725 |
| **Marketing deploy inventory thin** | **met** (W389) | this document + `marketing_deploy_thin.rs` + ADR-733 |
| **Marketing same-source deepen** | **partial met** (W390) | `marketing_capabilities.rs` + `marketing_deploy_deepen.rs` + ADR-734 |
| **Marketing product met (narrow)** | **met** (W391) | inventory + deepen (partial sufficient) + [`marketing-deploy-exit.md`](marketing-deploy-exit.md) + ADR-735 |
| **Static / marketing deploy tree** | **partial met** (W390) | temp/fixture HTML only；≠ production Pages/static tree；exit ≠ Static full met |
| **Hosted full site / marketing full site** | **unmet** | production deploy deferred；product met (narrow) ≠ full site |
| **TLS honesty-ladder floor** | **cross-met** (W388) | [`tls-server-ga-exit.md`](tls-server-ga-exit.md) + ADR-732；≠ TLS GA |
| **TLS server GA** | **cross-unmet** | deferred；≠ marketing exit |

## Thin-open + deepen + exit definition

Marketing **thin-open / inventory** (W389) indexes the Static/marketing deploy gap. Marketing **deepen** (W390) adds a same-source marketing-fixture HTML helper + temp/fixture write nail (path-guarded). Marketing **product met** (W391) signs exit under scope narrow (deepen partial met sufficient). Product met ≠ marketing full site / ≠ Static deploy tree full met.

**Evidence nails:** `marketing_deploy_thin.rs` + `marketing_deploy_deepen.rs` + `marketing_deploy_exit.rs` + `marketing_capabilities.rs` + this document + ADR-733–735.

## Frozen

- Hosting / deploying a full production / marketing website this window
- Writing marketing HTML into `docs/` or `books/` marketing trees
- Claiming marketing full site / Static deploy tree **met** (partial only)
- Real TLS HostFn / claiming TLS GA met
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Rolling back website / hosted / accept / TLS honesty floor / residual 4/4 / marketing inventory + deepen

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
