# TcpListen Accept GA Vectors (Wave A W382–W385 · exit signed)

> **Status:** **Accept GA inventory thin met** (ADR-726 / W382) + **Accept HostFn llvm first-slice deepen partial met** (ADR-727 / W383) + **Accept HostFn wasm first-slice met** (ADR-728 / W384) + **Accept GA met exit signed (scope narrow)** (ADR-729 / W385).  
> **Exit cross-index:** [`tcp-accept-ga-exit.md`](tcp-accept-ga-exit.md) / ADR-729.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> **TLS cross-index:** [`tls-server-ga-met-exit.md`](tls-server-ga-met-exit.md) / ADR-739 — TLS GA **met (narrow)**；product `net.tls.server` **still unsupported**.  
> **Hosted cross-index:** [`hosted-site-exit.md`](hosted-site-exit.md) / ADR-725 — hosted product **met (narrow)** unchanged.  
> Roadmap A-10 remains `[x]`. Accept GA **met (narrow)**. **Exit ≠ TLS.** **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **`net.tcp.listener` product** | **cross-met** (W375) | caps `native+wasm=real` + `status=implemented`；listen/close nails；ADR-719 |
| **TcpListen / TcpListenerClose HostFn** | **cross-met** (W374) | `qs1_tcp_listen_close_wasm` + llvm listen/close；ADR-718 |
| **Accept GA inventory thin** | **met** (W382) | this document + `tcp_accept_ga_thin.rs` + ADR-726 |
| **Accept HostFn llvm first-slice** | **partial met** (W383) | `TcpListenerAccept` llvm=real；`qs1_tcp_listener_accept_native_llvm`；ADR-727 |
| **Accept HostFn wasm first-slice** | **met** (W384) | `TcpListenerAccept` wasm=real；`qs1_tcp_listener_accept_wasm`；ADR-728 |
| **TcpListen accept GA** | **met** (W385 narrow) | exit charter [`tcp-accept-ga-exit.md`](tcp-accept-ga-exit.md) + ADR-729；≠ TLS |
| **TLS server inventory thin** | **cross-met** (W386) | [`tls-server-ga-vectors.md`](tls-server-ga-vectors.md) + ADR-730 |
| **TLS server / `net.tls.server` product** | **unmet** | caps still unsupported；≠ TLS GA met narrow（ADR-739） |
| **TLS HostFn GA ladder** | **cross-met (narrow)** (W395) | [`tls-server-ga-met-exit.md`](tls-server-ga-met-exit.md)；≠ product status raise |

## Thin-open definition

Accept GA **thin-open / inventory** (W382) indexes the accept gap relative to residual listener product **4/4**.

## Deepen definition

Accept **llvm/native first-slice deepen** (W383) adds `TcpListenerAccept` + one loopback exact nail. **Partial met** was sufficient for exit prerequisites.

## Wasm first-slice definition

Accept **HostFn wasm first-slice** (W384) raises `TcpListenerAccept` wasm→real with one AOT exact nail.

## Exit definition

Accept GA **met exit (scope narrow)** (W385) = inventory met + llvm partial sufficient + wasm met + exit charter signed. Does **not** claim TLS server or caps `status=implemented` raise.

**Evidence nails:** `tcp_accept_ga_exit.rs` + this document + ADR-729.

## Frozen

- Claiming TLS server GA
- Raising `net.tcp.listener` / `TcpListenerAccept` `status=implemented` for full accept GA
- Re-opening residual product raises (4/4 closed)
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Rolling back website / hosted product **met (narrow)**
- New HostFn behavioral nails this window

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
