# Hosted Product Met Exit Charter (Wave A W381)

> **Status:** **Hosted product met exit signed** (ADR-725 / W381) — inventory + deepen locked by prior sub-charters; this window is **policy + meta** only with **explicit hosted scope narrow**.  
> **Vectors cross-index:** [`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723–724.  
> **Website cross-index:** [`website-capability-exit.md`](website-capability-exit.md) / ADR-722 — website product **met (narrow)** unchanged.  
> **A-10 cross-index:** [`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705 — A-10/M0-C05 `[x]` unchanged.  
> **Marketing cross-index:** [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735 — marketing product **met (narrow)**；marketing full site **still unmet**.  
> Roadmap A-10 remains `[x]`. Hosted product **met (narrow)**. **Exit ≠ marketing full site.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Hosted scope narrow (ADR-725)

Hosted product **met** = W379 inventory thin **met** + W380 same-source deepen **partial met is sufficient** (`capabilities_for_hosted` + `render_hosted_capabilities_page` + temp/fixture write → `run_capabilities_report()` only; nail `hosted_capabilities_same_source_count_and_lookup`) + this exit charter signed.

Static / marketing deploy tree / Hosted full site (marketing semantics) / new `capabilities.toml` website/hosted row with `status=implemented` = **product defer** (clusters **Static / marketing deploy tree** and **Hosted full site** stay **unmet**; **Not** claimed as part of product met).

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Inventory thin-open | **met** (W379 / ADR-723) | [`hosted-site-vectors.md`](hosted-site-vectors.md) + `hosted_site_thin.rs` |
| Same-source deepen | **partial met** (W380 / ADR-724) | `hosted_capabilities.rs` + `hosted_site_deepen.rs` |
| Hosted scope narrow | **signed** (W381 / ADR-725) | this document + ADR-725 (marketing full site deferred) |

Hosted product **met** is signed when the prerequisites above hold and this document's Status cites ADR-725. This window does **not** add new helper/behavioral nails and does **not** host a marketing full site.

**Evidence nails:** `crates/syx-driver/tests/stdlib/hosted_site_exit.rs` + plan/02 W381.

## Frozen

- Hosting / deploying a full production / marketing website this window
- Writing hosted HTML into `docs/` or `books/` marketing trees
- Adding a new `capabilities.toml` row with `status=implemented` for website/hosted
- New helper / HostFn behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)
- Rolling back website product **met (narrow)** / hosted inventory + deepen

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
