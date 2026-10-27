# Website Product Met Exit Charter (Wave A W378)

> **Status:** **Website product met exit signed** (ADR-722 / W378) — inventory + deepen locked by prior sub-charters; this window is **policy + meta** only with **explicit website scope narrow**.  
> **Vectors cross-index:** [`website-capability-vectors.md`](website-capability-vectors.md) / ADR-720–722.  
> **Hosted cross-index:** [`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723–725 + [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725 — Hosted product **met (narrow)**; marketing / Hosted full site **still unmet**.  
> **A-10 cross-index:** [`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705 — A-10/M0-C05 `[x]` unchanged.  
> **Marketing cross-index:** [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735 — marketing product **met (narrow)**；full site **still unmet**.  
> Roadmap A-10 remains `[x]`. Website product **met (narrow)**. **Exit ≠ hosted full site.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Website scope narrow (ADR-722)

Website product **met** = W376 inventory thin **met** + W377 same-source deepen **partial met is sufficient** (`capabilities_for_website` + `render_capabilities_html_snippet` → `run_capabilities_report()` only; nail `website_capabilities_same_source_count_and_lookup`) + this exit charter signed.

Hosted full site / marketing deploy / new `capabilities.toml` website row with `status=implemented` = **product defer** (cluster **Hosted full site** stays **unmet**; **Not** claimed as part of product met).

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Inventory thin-open | **met** (W376 / ADR-720) | [`website-capability-vectors.md`](website-capability-vectors.md) + `website_capability_thin.rs` |
| Same-source deepen | **partial met** (W377 / ADR-721) | `website_capabilities.rs` + `website_capability_deepen.rs` |
| Website scope narrow | **signed** (W378 / ADR-722) | this document + ADR-722 (hosted site deferred) |

Website product **met** is signed when the prerequisites above hold and this document's Status cites ADR-722. This window does **not** add new helper/behavioral nails and does **not** host a full site.

**Evidence nails:** `crates/syx-driver/tests/stdlib/website_capability_exit.rs` + plan/02 W378.

## Frozen

- Hosting / deploying a full production website this window
- Adding a new `capabilities.toml` row with `status=implemented` for website
- New helper / HostFn behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
