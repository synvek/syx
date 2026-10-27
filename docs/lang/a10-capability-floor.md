# A-10 Capability Floor (Wave A W360; exit W361)

> **Status:** **A-10 floor signed** (ADR-704 / W360) — authority schema + gen/CI + books/doctor **met**; LSP same-source deepen **partial met** (ADR-703); website product **met (narrow)** (W378 / ADR-722); hosted product **met (narrow)** (W381 / ADR-725); marketing / Hosted full site **unmet**; residual **Frozen**.  
> **Exit cross-index:** **A-10 / M0-C05 exit signed** ([`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705 / W361) via M0-C05 scope narrow.  
> **W381 cross-index:** Hosted product met exit ([`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725).  
> **W379 cross-index:** Hosted full-site thin inventory ([`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723).  
> **W378 cross-index:** Website product met exit ([`website-capability-exit.md`](website-capability-exit.md) / ADR-722).  
> **W358–W359 cross-index:** thin-open ([`a10-capability-source.md`](a10-capability-source.md) / ADR-702) + LSP deepen (ADR-703).  
> Roadmap A-10 is `[x]` (W361). M0-C05 complete under narrow definition. Website / hosted product **met (narrow)** ≠ marketing full site. Residual product **4/4** closed.

## Floor clusters

| Cluster | Status | Evidence |
|---------|--------|----------|
| **权威源 schema + gen + CI** | **met** | `packages/internal/capabilities.toml` + `scripts/gen_capabilities.py` + `--check` + CI |
| **Books + Doctor 同源** | **met** | `books/shared/capabilities-generated.md` + `capabilities_report.rs` + `syx doctor --capabilities` |
| **LSP 同源 deepen** | **partial met** | `syx.capabilities` + `lsp_a10_capabilities_same_source_count_and_lookup`（ADR-703） |
| **网站同源消费** | **met** (narrow) | W378 exit（ADR-722）；inventory+deepen；hosted inventory thin W379；hosted full site **unmet** |
| **Residual native raise** | **Frozen** | listener / pool / row / deque `wasm=unsupported` |

## Floor charter definition

A-10 **floor** is signed when the clusters above are indexed and this document's Status cites ADR-704. **Exit** (`[x]` / M0-C05 complete) is signed separately by ADR-705 / [`a10-capability-exit.md`](a10-capability-exit.md) via explicit M0-C05 scope narrow (website deferred). Floor ≠ exit.

**Evidence nails:** `crates/syx-driver/tests/stdlib/capability_floor_thin.rs` + `capability_thin.rs` + LSP nail + [`a10-capability-source.md`](a10-capability-source.md).

## Frozen

- Hosting / deploying a full production website this window
- Residual `native=partial→real` (listener / pool / row / deque)
- New LSP / HostFn behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
