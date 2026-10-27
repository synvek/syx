# A-10 Capability Same-Source (Wave A W358 thin + W359 LSP deepen + W360 floor + W361 exit)

> **Status:** **A-10 thin floor signed** (ADR-702 / W358) + **LSP same-source deepen** (ADR-703 / W359) + **A-10 floor signed** ([`a10-capability-floor.md`](a10-capability-floor.md) / ADR-704 / W360) + **A-10 / M0-C05 exit signed** ([`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705 / W361) via M0-C05 scope narrow — authoritative pipeline + LSP consumer; website product **met (narrow)** (W378 / ADR-722); hosted inventory thin **met** (W379 / ADR-723); hosted deepen **partial met** (W380 / ADR-724); hosted product **met (narrow)** (W381 / ADR-725); marketing full site still deferred.  
> **W381 cross-index:** Hosted product met exit ([`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725).  
> **W380 cross-index:** Hosted same-source deepen ([`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-724).  
> **W379 cross-index:** Hosted full-site thin inventory ([`hosted-site-vectors.md`](hosted-site-vectors.md) / ADR-723).  
> **W378 cross-index:** Website product met exit ([`website-capability-exit.md`](website-capability-exit.md) / ADR-722).  
> **W377 cross-index:** Website deepen (ADR-721).  
> **W376 cross-index:** Website thin inventory signed (ADR-720).  
> **W357 cross-index:** A-08 / Q-S4 exit signed ([`a08-qs4-exit.md`](a08-qs4-exit.md) / ADR-701).  
> Roadmap A-10 is `[x]`. M0-C05 **complete** (narrow). Website / hosted product **met (narrow)** ≠ marketing full site. Residual product **4/4** closed (W375).

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **权威源 schema** | **in scope** | `packages/internal/capabilities.toml` + [`capability-manifest-v0.md`](../capability-manifest-v0.md) |
| **生成器 + CI freshness** | **in scope** | `scripts/gen_capabilities.py` + `--check` + CI |
| **Books 同源矩阵** | **in scope** | `books/shared/capabilities-generated.md` |
| **Doctor 同源 JSON** | **in scope** | `packages/internal/capabilities.generated.json` + `crates/syx-driver/src/capabilities_report.rs` + `syx doctor --capabilities` |
| **LSP 消费同一源** | **partial met** | `syx-lsp` `syx.capabilities` → `capabilities_for_lsp` → `run_capabilities_report()`；nail `lsp_a10_capabilities_same_source_count_and_lookup` (ADR-703) |
| **网站消费同一源** | **met** (narrow) | W378 website + W381 hosted product met exit（ADR-722/725）；inventory+deepen；**仍 ≠** marketing full site |
| **Residual native raise** | **Frozen** | listener / pool / row / deque `wasm=unsupported` |

## Thin-open + LSP deepen + floor + exit definition

A-10 **thin floor** (W358) indexes M0-C01～C04. W359 adds LSP deepen. W360 signs floor. W361 signs **exit** under M0-C05 scope narrow (LSP side sufficient). Website product **met (narrow)** signed later by ADR-722 / W378 — **≠** hosted full site.

**Evidence nails:** `capability_thin.rs` + `capability_floor_thin.rs` + `capability_exit.rs` + LSP nail + roadmap A-10 `[x]`.

## Frozen

- Hosting / deploying a full production website this window
- Residual `native=partial→real` (listener / pool / row / deque)
- New LSP / HostFn behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
