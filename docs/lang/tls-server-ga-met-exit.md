# TLS Server GA Met Exit Charter (Wave A W395)

> **Status:** **TLS GA met exit signed** (ADR-739 / W395) — implement inventory + llvm first-slice + wasm first-slice locked by prior sub-charters; this window is **policy + meta** only with **explicit TLS GA scope narrow**.  
> **Vectors cross-index:** [`tls-server-ga-vectors.md`](tls-server-ga-vectors.md) / ADR-736–738.  
> **Status-raise cross-index:** [`tls-server-status-raise.md`](tls-server-status-raise.md) / ADR-779 — product status raise **later met**（W435；≠ this W395 charter）.  
> **Honesty-floor cross-index:** [`tls-server-ga-exit.md`](tls-server-ga-exit.md) / ADR-732 — honesty floor **met**；**≠** this GA met charter（分轴）.  
> **Accept cross-index:** [`tcp-accept-ga-exit.md`](tcp-accept-ga-exit.md) / ADR-729 — accept GA **met (narrow)** unchanged.  
> **Residual cross-index:** [`residual-product-vectors.md`](residual-product-vectors.md) / ADR-719 — product **4/4** closed.  
> **Marketing cross-index:** [`marketing-deploy-exit.md`](marketing-deploy-exit.md) / ADR-735 — marketing product **met (narrow)** unchanged.  
> Roadmap A-10 remains `[x]`. TLS GA **met (narrow)**. **Exit ≠ `net.tls.server` status=implemented at this window**（product defer；later W435）. Live B-06 is `[x]` via ADR-792 / W448 narrow exit ([`b06-missing-align-exit.md`](b06-missing-align-exit.md)). **Not** Missing 清零 / stub rewrite / B-07 `[x]`. Live B-07 is `[~]` via ADR-793 / W449 thin ([`b07-real-hostfn-lowering.md`](b07-real-hostfn-lowering.md)). **Not** production-default JIT.

## TLS GA scope narrow (ADR-739)

TLS GA **met** = W392 implement inventory thin **met** + W393 llvm first-slice **partial met is sufficient** (`TlsServe` llvm=real + `qs1_tls_serve_native_llvm`) + W394 HostFn wasm first-slice **met** (`wasm=real` + `qs1_tls_serve_wasm`) + this exit charter signed.

Raising `net.tls.server` to `status=implemented` / backends product raise / full handler loop / ALPN / multi-connection production API = **product defer** at this window (`net.tls.server` stays **unsupported** here; `TlsServe` HostFn status may remain `partial`; **Not** claimed as part of TLS GA met narrow). Later W435 / ADR-779 raises product status only（≠ handler deepen）.

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Implement inventory thin | **met** (W392 / ADR-736) | [`tls-server-ga-vectors.md`](tls-server-ga-vectors.md) + `tls_server_hostfn_thin.rs` |
| Llvm first-slice deepen | **partial met** (W393 / ADR-737) | `host_tls_serve` + `qs1_tls_serve_native_llvm` + `tls_server_hostfn_deepen.rs` |
| Wasm first-slice | **met** (W394 / ADR-738) | `TlsServe` wasm=real + `qs1_tls_serve_wasm` |
| TLS GA scope narrow | **signed** (W395 / ADR-739) | this document + ADR-739（product status deferred） |

TLS GA **met** is signed when the prerequisites above hold and this document's Status cites ADR-739. This window does **not** add new HostFn behavioral nails and does **not** raise product caps status.

**Evidence nails:** `crates/syx-driver/tests/stdlib/tls_server_ga_met_exit.rs` + plan/02 W395.

## Frozen

- Rewriting this charter as if W395 delivered `status=implemented`
- New HostFn / behavioral nails / handler·ALPN·multi-conn as part of GA met narrow
- Rewriting honesty-floor exit ([`tls-server-ga-exit.md`](tls-server-ga-exit.md)) semantics
- Claiming B-06 `[x]` without ADR-792 exit narrow; Wave B production cutover (B-04/B-05/B-06 `[x]` met via narrow exits when citing ADR-790/791/792)
- Production-default JIT / Q-L5/B-08 rollback
- Re-opening residual product raises (4/4 closed)
- Rolling back website / hosted / accept / marketing / honesty floor / llvm+wasm nails

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
