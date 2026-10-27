# Website Same-Source Capability Vectors (Wave A W376–W381 hosted met exit)

> **Status:** **Website same-source thin-open / inventory signed** (ADR-720 / W376) + **Website same-source deepen signed** (ADR-721 / W377) + **Website product met exit signed** (ADR-722 / W378) + **Hosted full-site thin inventory signed** (ADR-723 / W379) + **Hosted same-source deepen signed** (ADR-724 / W380) + **Hosted product met exit signed** (ADR-725 / W381) — independent website consumer **met (narrow)**; Hosted inventory thin **met**; Deployable hosted consumer **met (narrow)**；Hosted full site / marketing **still unmet**.  
> **Exit cross-index:** [`website-capability-exit.md`](website-capability-exit.md) / ADR-722.  
> **Hosted cross-index:** [`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723–725 + [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725.  
> **A-10 cross-index:** [`a10-capability-source.md`](a10-capability-source.md) / ADR-702–705 — A-10/M0-C05 `[x]` via scope narrow.  
> **Marketing cross-index:** [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735 — marketing product **met (narrow)**；Hosted full site **still unmet**.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> Roadmap A-10 remains `[x]`. **Product met (narrow) ≠ marketing full site.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **权威源 schema** | **cross-met** (A-10) | `packages/internal/capabilities.toml` + [`capability-manifest-v0.md`](../capability-manifest-v0.md) |
| **生成器 + CI freshness** | **cross-met** (A-10) | `scripts/gen_capabilities.py` + `--check` + CI |
| **Books 同源矩阵** | **cross-met** (A-10) | `books/shared/capabilities-generated.md` |
| **Doctor 同源 JSON** | **cross-met** (A-10) | `capabilities.generated.json` + `capabilities_report.rs` + `syx doctor --capabilities` |
| **LSP 消费同一源** | **cross-partial-met** (A-10/W359) | `syx.capabilities` → `capabilities_for_lsp` → `run_capabilities_report()` |
| **独立 website 消费者** | **met** (W378 narrow) | inventory W376 + deepen W377 (partial sufficient) + exit ADR-722；`capabilities_for_website` + HTML nail |
| **Website inventory thin** | **met** (W376) | this document + `website_capability_thin.rs` + ADR-720 |
| **Hosted inventory thin** | **met** (W379) | [`hosted-site-vectors.md`](hosted-site-vectors.md) + `hosted_site_thin.rs` + ADR-723 |
| **Deployable hosted consumer** | **met** (W381 narrow) | inventory W379 + deepen W380 (partial sufficient) + exit ADR-725；`hosted_capabilities.rs` + fixture nail |
| **Hosted full site** | **unmet** | marketing / production deploy deferred；product met (narrow) ≠ marketing；exit ≠ deploy |

## Thin-open + deepen + exit definition

Website **thin-open / inventory** (W376) indexes the consumer gap. Website **deepen** (W377) adds a minimal docs-site/HTML same-source consumer nail. Website **product met** (W378) signs exit under scope narrow. Hosted **thin inventory** (W379) indexes deploy/marketing gap. Hosted **deepen** (W380) adds fixture/temp full-HTML nail. Hosted **product met** (W381) signs exit under scope narrow (deepen partial sufficient). Product met ≠ marketing full site.

**Evidence nails:** `website_capability_thin.rs` + `website_capability_deepen.rs` + `website_capability_exit.rs` + `website_capabilities.rs` + `hosted_site_thin.rs` + `hosted_site_deepen.rs` + `hosted_site_exit.rs` + `hosted_capabilities.rs`.

## Frozen

- Hosting / deploying a full production / marketing website this window
- Claiming Hosted full site / marketing **met** (narrow product met only)
- Adding a new `capabilities.toml` row with `status=implemented` for website
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
