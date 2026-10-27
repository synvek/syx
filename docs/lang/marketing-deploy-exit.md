# Marketing Product Met Exit Charter (Wave A W391)

> **Status:** **Marketing product met exit signed** (ADR-735 / W391) — inventory + deepen locked by prior sub-charters; this window is **policy + meta** only with **explicit marketing scope narrow**.  
> **Vectors cross-index:** [`marketing-deploy-vectors.md`](marketing-deploy-vectors.md) / ADR-733–734.  
> **Hosted cross-index:** [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725 — Deployable hosted consumer **met (narrow)**；Hosted full site **still unmet**.  
> **Website cross-index:** [`website-capability-exit.md`](website-capability-exit.md) / ADR-722 — website product **met (narrow)** unchanged.  
> **A-10 cross-index:** [`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705 — A-10/M0-C05 `[x]` unchanged.  
> **TLS cross-index:** [`tls-server-ga-exit.md`](tls-server-ga-exit.md) / ADR-732 — honesty floor **met**；llvm first-slice **cross-partial**（ADR-737）；wasm first-slice **cross-met**（ADR-738）；TLS GA **cross-met (narrow)**（ADR-739）；product status **still unsupported**.  
> Roadmap A-10 remains `[x]`. Marketing product **met (narrow)**. **Exit ≠ marketing full site.** **Exit ≠ Static deploy tree met.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Marketing scope narrow (ADR-735)

Marketing product **met** = W389 inventory thin **met** + W390 same-source deepen **partial met is sufficient** (`capabilities_for_marketing` + `render_marketing_capabilities_page` + path-guarded temp/fixture write → `run_capabilities_report()` only) + this exit charter signed.

Static / marketing deploy tree full **met** / Hosted full site / marketing full site / new `capabilities.toml` marketing row with `status=implemented` = **product defer** (Static stays **partial met**; Hosted full site / marketing full site stay **unmet**; **Not** claimed as part of product met).

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Inventory thin-open | **met** (W389 / ADR-733) | [`marketing-deploy-vectors.md`](marketing-deploy-vectors.md) + `marketing_deploy_thin.rs` |
| Same-source deepen | **partial met** (W390 / ADR-734) | `marketing_capabilities.rs` + `marketing_deploy_deepen.rs` |
| Marketing scope narrow | **signed** (W391 / ADR-735) | this document + ADR-735 (full site deferred) |

Marketing product **met** is signed when the prerequisites above hold and this document's Status cites ADR-735. This window does **not** add new helper/behavioral nails and does **not** host a marketing full site.

**Evidence nails:** `crates/syx-driver/tests/stdlib/marketing_deploy_exit.rs` + plan/02 W391.

## Frozen

- Hosting / deploying a full production / marketing website this window
- Writing marketing HTML into `docs/` or `books/` marketing trees
- Claiming Static / marketing deploy tree **met** (partial only) or marketing full site **met**
- Adding a new `capabilities.toml` row with `status=implemented` for marketing
- New helper / HostFn behavioral nails this window
- Real TLS HostFn / claiming TLS GA met
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Rolling back website / hosted / accept / TLS honesty floor / residual 4/4 / marketing inventory + deepen

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
