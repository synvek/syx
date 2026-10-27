# TLS product status raise first-slice (W435 / ADR-779)

## Status

**TLS product status raise first-slice met** (`net.tls.server` `status=implemented` after TLS GA met (narrow); backends native/wasm=real, vm/mir=partial; `TlsServe` HostFn **still `partial` at W435 charter**; HostFn later raised in **W441 / ADR-785** → live **`implemented`**; W436 application-data first-slice **later met** — ≠ ALPN at W435 / ≠ multi-conn at W435).

Fingerprint: `two-hundred-eighty-fifth` / `A-W435`.

## Index

| Prior | Status |
|-------|--------|
| W395 TLS GA met exit (narrow) | **met** (≠ status raise at that window) |
| W393/W394 TlsServe llvm/wasm first-slice | **partial met / met** |
| This window product status raise | **met** (`status=implemented`) |
| TlsServe HostFn status | **met** (W441 / ADR-785；later；W435 charter kept partial) |
| TLS handler application-data | **met** (W436 / ADR-780；later) |
| TLS multi-conn (N=2) | **met** (W437 / ADR-781；later) |
| TLS ALPN (`syx/1`) | **met** (W438 / ADR-782；later) |
| Configurable ALPN (env) | **met** (W443 / ADR-787；later) |
| Language String alpn parameter (5th arg) | **met** (W445 / ADR-789；later) |
| Unbounded multi-conn (sequential idle) | **met** (W442 / ADR-786；later) |
| Concurrent serve (thread-per-conn + idle) | **met** (W444 / ADR-788；later) |
| Short absolute thresholds / default multi-root | absolute thresholds **met** (W439)；默认多根 **met** (W440 / ADR-784) |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Cap | `net.tls.server` **`status = "implemented"`** |
| Backends | native/wasm **real**; vm/mir **partial** (honest; not full matrix) |
| HostFn | W435 charter: `TlsServe` remains **`partial`**; live **`implemented`** (W441)；count **554** |
| Forbidden claim (this window) | ALPN / multi-conn / Short cutover / B-05 `[x]` / MIR blob delete / rewriting as if app-data or HostFn raise delivered in W435 |
| Honesty | Status raise **≠** GA met narrow rewrite / **≠** HostFn status raise (HostFn raise later W441) / **≠** ALPN |

## Evidence

- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml) `net.tls.server`
- Prior nails: `qs1_tls_serve_native_llvm` / `qs1_tls_serve_wasm`
- Later app-data: [`tls-server-handler-appdata.md`](tls-server-handler-appdata.md)
- Later HostFn status: [`tls-server-hostfn-status.md`](tls-server-hostfn-status.md)
- Cross: [`tls-server-ga-met-exit.md`](tls-server-ga-met-exit.md), [`tls-server-ga-vectors.md`](tls-server-ga-vectors.md), [`tls-server-ga-exit.md`](tls-server-ga-exit.md)

## Frozen

- TLS ALPN / multi-connection production API deepen (ALPN/N=2 delivered later in W437–W438; unbounded delivered later in W442 / ADR-786; env ALPN delivered later in W443 / ADR-787 — historical W435 Frozen)
- Array-typed / structured alpn product API
- Raising `TlsServe` HostFn to `status=implemented` (delivered later in W441 / ADR-785 — historical W435 Frozen; default kept partial at this window)
- Short default emit auto multi-root; absolute byte thresholds (absolute thresholds delivered W439; default multi-root delivered W440 — historical W435 Frozen)
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Rewriting W386–W395 historical “status raise deferred” honesty as if delivered then
- Claiming production-default JIT
- Adding HostFn (keep 554)

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
