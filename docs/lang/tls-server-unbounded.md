# TLS unbounded multi-conn first-slice (W442 / ADR-786)

## Status

**TLS unbounded multi-conn first-slice met** (`host_tls_serve` nonblocking accept until **idle** — 400ms continuous `WouldBlock` after ≥1 handled **and** no in-flight workers (W444); overall wait for first accept ≤15s; per-connection handshake + `HANDLED\n` + ALPN default `syx/1`; live ALPN from env **`SYX_TLS_SERVE_ALPN`** (W443 / ADR-787)；live concurrent thread-per-conn (W444 / ADR-788)；live language String alpn 5th arg (W445 / ADR-789)；`TlsServe` remains **`implemented`**; HostFn count **554**; env no-idle first-slice **met** (W457 / ADR-801；≠ language noIdle / ≠ daemon GA) / ≠ thread-pool product API / ≠ Array-typed alpn API / ≠ generic user callback).

Fingerprint: `two-hundred-ninety-second` / `A-W442`.

## Index

| Prior | Status |
|-------|--------|
| W435 TLS product status raise | **met** (`net.tls.server` `status=implemented`) |
| W436 TLS handler application-data | **met** |
| W437 TLS multi-conn (N=2) | **met** (historical fixed N; live unbounded) |
| W438 TLS ALPN (`syx/1`) | **met** |
| W441 `TlsServe` HostFn status raise | **met** (`status=implemented`) |
| This window unbounded accept (idle-timeout) | **met** (W442 charter sequential; live thread-per-conn W444) |
| Configurable ALPN (env) | **met** (W443 / ADR-787；`SYX_TLS_SERVE_ALPN`；later) |
| Language String alpn parameter (5th arg) | **met** (W445 / ADR-789；later) |
| Concurrent serve (thread-per-conn + idle) | **met** (W444 / ADR-788；later) |
| B-04 `[x]` (narrow exit) | **met** (W446 / ADR-790) |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Behavior | Bind → accept until **idle**; W442 charter: sequential; live (W444): thread-per-conn workers; each: handshake → app-data read → `HANDLED\n` → close |
| Idle | **400ms** continuous accept `WouldBlock` after `handled >= 1` → return 0 |
| First-accept budget | Overall ≤**15s** with no handled connection → return -1 |
| HostFn | `TlsServe` remains **`implemented`**; count **554** |
| Cap | `net.tls.server` remains **`implemented`** (W435) |
| Forbidden claim | No-idle daemon / thread-pool product API / Array alpn API / B-04 `[x]` without ADR-790 / MIR blob delete this window |
| Honesty | Unbounded idle-exit **≠** no-idle daemon / **≠** thread-pool product API / **≠** Array alpn API / **≠** B-04 `[x]`；env ALPN W443；concurrent W444；language String alpn W445 |

## Evidence

- [`crates/syx-vm/src/hosts.rs`](../../crates/syx-vm/src/hosts.rs) `host_tls_serve` (`TLS_SERVE_IDLE` / nonblocking accept loop)
- Nails: `qs1_tls_serve_native_llvm` / `qs1_tls_serve_wasm` (≥2 clients assert `HANDLED` + ALPN)
- Meta: `tls_server_unbounded.rs`
- Cross: [`tls-server-multi-conn.md`](tls-server-multi-conn.md), [`tls-server-hostfn-status.md`](tls-server-hostfn-status.md), [`tls-server-alpn.md`](tls-server-alpn.md), [`tls-server-alpn-config.md`](tls-server-alpn-config.md)

## Frozen

- Language noIdle arg / daemon GA without env bound (env no-idle first-slice delivered W457 / ADR-801)
- Thread-pool configuration / sizing product API
- Configurable TLS ALPN via env (delivered W443 / ADR-787 — historical W442 Frozen)
- Array-typed / structured alpn product API
- Generic user-callback handler API
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)
- Claiming production-default JIT / infinite daemon without idle exit

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
