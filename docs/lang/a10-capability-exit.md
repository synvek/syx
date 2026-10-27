# A-10 / M0-C05 Exit Charter (Wave A W361)

> **Status:** **A-10 / M0-C05 exit signed** (ADR-705 / W361) — thin-open + LSP same-source deepen + A-10 floor locked by prior sub-charters; this window is **policy + meta** only with **explicit M0-C05 scope narrow**.  
> **Floor cross-index:** [`a10-capability-floor.md`](a10-capability-floor.md) / ADR-704.  
> **Source cross-index:** [`a10-capability-source.md`](a10-capability-source.md) / ADR-702–704.  
> **W381 cross-index:** Hosted product met exit signed ([`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725) — product met (narrow) ≠ marketing full site.  
> **W379 cross-index:** Hosted full-site thin inventory signed ([`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723) — inventory met ≠ hosted full site.  
> **W378 cross-index:** Website product met exit signed ([`website-capability-exit.md`](website-capability-exit.md) / ADR-722) — product met (narrow) ≠ hosted full site.  
> **W377 cross-index:** Website deepen signed (ADR-721).  
> **W376 cross-index:** Website thin inventory signed (ADR-720).  
> Roadmap A-10 is `[x]`. M0-C05 is **complete** under the narrow definition below. Website product **met (narrow)** signed separately (ADR-722); hosted product **met (narrow)** signed (ADR-725); marketing / Hosted full site remains **deferred unmet**.

## M0-C05 scope narrow (ADR-705)

M0-C05 / A-10 **complete** = authoritative `capabilities.toml` + gen/CI **met** + books/doctor same-source **met** + **at least one** IDE/runtime same-source consumer (W359 LSP `syx.capabilities` nail; **partial met** is sufficient).

Independent website same-source consumer was **product defer** at A-10 exit (W361); later signed **met (narrow)** by ADR-722 / W378. Hosted inventory thin signed by ADR-723 / W379; deepen by ADR-724 / W380; product **met (narrow)** by ADR-725 / W381. Marketing / Hosted full site remains **deferred unmet**.

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Thin-open | **met** (W358 / ADR-702) | [`a10-capability-source.md`](a10-capability-source.md) + `capability_thin.rs` |
| LSP same-source deepen | **partial met** (W359 / ADR-703) | `syx.capabilities` + `lsp_a10_capabilities_same_source_count_and_lookup` |
| A-10 floor clusters | **met** (W360 / ADR-704) | [`a10-capability-floor.md`](a10-capability-floor.md) |
| M0-C05 scope narrow | **signed** (W361 / ADR-705) | this document + ADR-705 (website defer) |

A-10 / M0-C05 `[x]` is signed when the prerequisites above hold and this document's Status cites ADR-705. This window does **not** add new LSP/HostFn behavioral nails and does **not** build a website.

**Evidence nails:** `crates/syx-driver/tests/stdlib/capability_exit.rs` + plan/02 A-10 flip.

## Frozen

- Hosting / deploying a full production website this window
- Residual `native=partial→real` (listener / pool / row / deque)
- New LSP / HostFn behavioral nails this window
- Q-L5 `[x]` this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
