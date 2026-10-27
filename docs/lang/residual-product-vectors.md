# Residual Product Vectors (Wave A W368–W375; accept inventory W382–W385 exit)

> **Status:** **Residual deque first-slice** (ADR-712 / W368) + **SqlRowGetInt HostFn wasm** (ADR-713 / W369) + **SqlRow* HostFn wasm batch-5** (ADR-714 / W370) + **product `data.sqlite.row` raise signed** (ADR-715 / W371) + **SqlPool* HostFn wasm first-slice** (ADR-716 / W372) + **product `data.sqlite.pool` raise signed** (ADR-717 / W373) + **TcpListen* HostFn wasm first-slice** (ADR-718 / W374) + **product `net.tcp.listener` raise signed** (ADR-719 / W375) — product **4/4 met** (deque + row + pool + listener).  
> **W382 cross-index:** Accept GA thin inventory signed ([`tcp-accept-ga-vectors.md`](tcp-accept-ga-vectors.md) / ADR-726).  
> **W383 cross-index:** Accept HostFn llvm first-slice **partial met** (ADR-727).  
> **W384 cross-index:** Accept HostFn wasm first-slice **met** (ADR-728).  
> **W385 cross-index:** Accept GA met exit **signed (scope narrow)** (ADR-729).  
> **W386 cross-index:** TLS server inventory thin **met** ([`tls-server-ga-vectors.md`](tls-server-ga-vectors.md) / ADR-730).  
> **W387 cross-index:** TLS HostFn-gap honesty deepen **partial met** (ADR-731).  
> **W388 cross-index:** TLS honesty-ladder floor exit **signed** (ADR-732).  
> **W389–W391 cross-index:** Marketing product **met (narrow)** ([`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735) — Static **partial**；marketing full site **still unmet**.  
> **Residual product four-row axis closed.** Accept GA **met (narrow)**. TLS inventory thin **met**. HostFn-gap deepen **partial met**. Honesty floor **met**. **Not** TLS GA. **Not** website met. **Not** B-04/B-05/B-06 `[x]`. **Not** production-default JIT.

## Residual product clusters

| Cluster | Status | Evidence |
|---------|--------|----------|
| **`collections.deque`** | **met** (W368) | caps `native+wasm=real` + `status=implemented`; `a04_deque_empty_pop_nil_{native,wasm}`; ADR-248 + ADR-712 |
| **`data.sqlite.row`** | **met** (W371) | caps `native+wasm=real` + `status=implemented`; `qs1_sql_row_accessors_{native_llvm,wasm}` + GetInt dual nails + `m2_s8_02b_typed_row`; ADR-246 + ADR-715 |
| **`data.sqlite.pool`** | **met** (W373) | caps `native+wasm=real` + `status=implemented`; `qs1_sql_pool_lifecycle_{native_llvm,wasm}` + `m2_s8_01c`/`m2_s8_03b`; ADR-245 + ADR-717 |
| **`net.tcp.listener`** | **met** (W375) | caps `native+wasm=real` + `status=implemented`; `qs1_tcp_listen_close_wasm` + `m2_s4_tcp_listen_close_native_llvm` + `ec_m2_3_tcp_listener_leak`; ADR-255 + ADR-719 |
| **Accept GA inventory thin** | **met** (W382) | [`tcp-accept-ga-vectors.md`](tcp-accept-ga-vectors.md) + `tcp_accept_ga_thin.rs` + ADR-726 |
| **Accept HostFn llvm first-slice** | **partial met** (W383) | `TcpListenerAccept` + `qs1_tcp_listener_accept_native_llvm` + ADR-727 |
| **Accept HostFn wasm first-slice** | **met** (W384) | `qs1_tcp_listener_accept_wasm` + ADR-728 |
| **TcpListen accept GA** | **met** (W385 narrow) | [`tcp-accept-ga-exit.md`](tcp-accept-ga-exit.md) + ADR-729；≠ TLS |
| **TLS server inventory thin** | **met** (W386) | [`tls-server-ga-vectors.md`](tls-server-ga-vectors.md) + `tls_server_ga_thin.rs` + ADR-730 |
| **TLS HostFn-gap honesty deepen** | **partial met** (W387) | `tls_server_ga_deepen.rs` + ADR-731 |
| **TLS honesty-ladder floor** | **met** (W388 narrow) | [`tls-server-ga-exit.md`](tls-server-ga-exit.md) + ADR-732；≠ TLS GA |
| **TLS server GA** | **unmet** | deferred；≠ inventory；≠ deepen；≠ honesty floor |

## Residual HostFn progress

| HostFn | Status | Evidence |
|--------|--------|----------|
| **SqlRowGetInt** | **met** (W369) | `qs1_sql_prepare_bind_row_get_int_{native_llvm,wasm}`; ADR-713 |
| **SqlRowGetString** / **SqlRowGetFloat** / **SqlRowGetBlob** / **SqlRowIsNull** / **SqlRowColumnNames** | **met** (W370) | `qs1_sql_row_accessors_{native_llvm,wasm}`; ADR-714 HostFn wasm batch-5 |
| **SqlPoolCreate** / **SqlPoolAcquire** / **SqlPoolRelease** / **SqlPoolClose** | **met** (W372) | `qs1_sql_pool_lifecycle_{native_llvm,wasm}`; ADR-716 HostFn wasm first-slice |
| **TcpListen** / **TcpListenerClose** | **met** (W374) | `qs1_tcp_listen_close_wasm` + `m2_s4_tcp_listen_close_native_llvm`; ADR-718 HostFn wasm first-slice |
| **TcpListenerAccept** | **met** (W384 HostFn dual) | llvm+wasm=real；`qs1_tcp_listener_accept_{native_llvm,wasm}`；ADR-727+728；status still partial；accept GA met narrow ADR-729；≠ TLS |

## Raise definitions

Residual **product `net.tcp.listener` raise** (W375) is signed when caps are `native+wasm=real` + `status=implemented`, this document marks listener **met** and product **4/4**, and Status cites ADR-719. Honesty: accept GA **met (narrow)** (ADR-729); still ≠ TLS / ≠ website marketing. TcpListen* HostFn status remains `partial`. Accept exit signed by ADR-729 / W385 after inventory+llvm+wasm ladder.

**Evidence nails:** `residual_tcp_listener_thin.rs` + `tcp_accept_ga_thin.rs` + `tcp_accept_ga_deepen.rs` + `qs1_tcp_listener_accept_native_llvm` + `qs1_tcp_listen_close_wasm` + `m2_s4_tcp_listen_close_native_llvm` + prior residual thins.

## Frozen

- Claiming TLS server GA / implementing `net.tls.server` / rolling back accept GA **met (narrow)** / website **met** this window
- B-04 / B-05 / B-06 `[x]`; Wave B production cutover
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (axis closed at 4/4)

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
