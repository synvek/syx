# TcpListen Accept GA Met Exit Charter (Wave A W385)

> **Status:** **TcpListen accept GA met exit signed** (ADR-729 / W385) — inventory + llvm deepen + wasm first-slice locked by prior sub-charters; this window is **policy + meta** only with **explicit accept scope narrow**.  
> **Vectors cross-index:** [`tcp-accept-ga-vectors.md`](tcp-accept-ga-vectors.md) / ADR-726–728.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> **TLS cross-index:** [`tls-server-ga-met-exit.md`](tls-server-ga-met-exit.md) / ADR-739 — TLS GA **met (narrow)**；product status **still unsupported**.  
> **Hosted cross-index:** [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725 — hosted product **met (narrow)** unchanged.  
> Roadmap A-10 remains `[x]`. Accept GA **met (narrow)**. **Exit ≠ TLS server.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Accept scope narrow (ADR-729)

Accept GA **met** = W382 inventory thin **met** + W383 llvm first-slice **partial met is sufficient** (`TcpListenerAccept` llvm=real + `qs1_tcp_listener_accept_native_llvm`) + W384 HostFn wasm first-slice **met** (`wasm=real` + `qs1_tcp_listener_accept_wasm`) + this exit charter signed.

TLS server / `net.tls.server` / flipping `net.tcp.listener` or `TcpListenerAccept` to claim full production accept via `status=implemented` raise = **product defer** (TLS cluster stays **unmet**; HostFn status may remain `partial`; **Not** claimed as part of accept GA met narrow).

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Inventory thin-open | **met** (W382 / ADR-726) | [`tcp-accept-ga-vectors.md`](tcp-accept-ga-vectors.md) + `tcp_accept_ga_thin.rs` |
| Llvm first-slice deepen | **partial met** (W383 / ADR-727) | `host_tcp_listener_accept` + `qs1_tcp_listener_accept_native_llvm` + `tcp_accept_ga_deepen.rs` |
| Wasm first-slice | **met** (W384 / ADR-728) | `TcpListenerAccept` wasm=real + `qs1_tcp_listener_accept_wasm` |
| Accept scope narrow | **signed** (W385 / ADR-729) | this document + ADR-729 (TLS deferred) |

Accept GA **met** is signed when the prerequisites above hold and this document's Status cites ADR-729. This window does **not** add new HostFn behavioral nails and does **not** implement TLS server.

**Evidence nails:** `crates/syx-driver/tests/stdlib/tcp_accept_ga_exit.rs` + plan/02 W385.

## Frozen

- Implementing TLS server / claiming TLS GA this window
- Raising `net.tcp.listener` or `TcpListenerAccept` `status=implemented` to claim full accept GA
- New HostFn / behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)
- Rolling back website / hosted product **met (narrow)** / accept inventory + llvm + wasm

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
