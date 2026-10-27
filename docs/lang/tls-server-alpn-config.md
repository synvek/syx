# TLS configurable ALPN first-slice (W443 / ADR-787)

## Status

**TLS configurable ALPN first-slice met** (`host_tls_serve` sets `ServerConfig.alpn_protocols` from env **`SYX_TLS_SERVE_ALPN`** — comma-separated list; unset/empty/whitespace-only → default `[syx/1]`; idle unbounded accept + `HANDLED\n` retained; live concurrent thread-per-conn (W444 / ADR-788)；live language String alpn 5th arg (W445 / ADR-789)；`TlsServe` remains **`implemented`**; HostFn count **554**; ≠ Array-typed alpn API / ≠ no-idle daemon / ≠ thread-pool product API).

Fingerprint: `two-hundred-ninety-third` / `A-W443`.

## Index

| Prior | Status |
|-------|--------|
| W435 TLS product status raise | **met** |
| W436–W442 ladder (app-data / N=2 / fixed ALPN / HostFn / unbounded) | **met** |
| This window env configurable ALPN | **met** (`SYX_TLS_SERVE_ALPN`) |
| Language String alpn parameter (5th arg) | **met** (W445 / ADR-789；later) |
| Concurrent serve (thread-per-conn + idle) | **met** (W444 / ADR-788；later) |
| B-04 `[x]` (narrow exit) | **met** (W446 / ADR-790) |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Env | `SYX_TLS_SERVE_ALPN` = comma-separated protocols (order preserved) |
| Default | unset / empty → `[syx/1]` (W438 compatibility) |
| HostFn | W443 charter: arity `(host, port, cert, key)`; live arity **5** (W445)；count **554**; status **implemented** |
| Behavior retained | idle unbounded accept + HANDLED; live thread-per-conn (W444) |
| Forbidden claim | Array alpn API / no-idle daemon / thread-pool API / B-04 `[x]` without ADR-790 / MIR blob delete this window |
| Honesty | Env ALPN **≠** Array alpn API / **≠** no-idle daemon / **≠** thread-pool product API；concurrent delivered later W444；language String alpn delivered later W445 |

## Evidence

- [`crates/syx-vm/src/hosts.rs`](../../crates/syx-vm/src/hosts.rs) `tls_serve_alpn_protocols` / `SYX_TLS_SERVE_ALPN`
- Nails: `qs1_tls` default `syx/1` + env override llvm path
- Meta: `tls_server_alpn_config.rs`
- Cross: [`tls-server-alpn.md`](tls-server-alpn.md), [`tls-server-unbounded.md`](tls-server-unbounded.md)

## Frozen

- No-idle infinite accept daemon (HostFn never returns)
- Thread-pool configuration / sizing product API
- Array-typed / structured alpn product API
- Generic user-callback handler API
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Claiming production-default JIT

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
