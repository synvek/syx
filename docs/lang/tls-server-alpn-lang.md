# TLS language ALPN parameter first-slice (W445 / ADR-789)

## Status

**TLS language ALPN parameter first-slice met** (`host_tls_serve` arity `(host, port, cert, key, alpn)`; non-empty 5th String comma-separated sets `ServerConfig.alpn_protocols`; empty/whitespace → `SYX_TLS_SERVE_ALPN` → default `[syx/1]`; facade `serveOnce` passes `""`; `serveOnceAlpn` exposes language arg; concurrent + idle + `HANDLED\n` retained; `TlsServe` remains **`implemented`**; HostFn count **554**; B-04 **`[x]` met** (W446 / ADR-790 narrow exit); ≠ Array-typed alpn API / env no-idle first-slice **met** (W457；≠ language noIdle / ≠ daemon GA) / ≠ language exports auto-discovery).

Fingerprint: `two-hundred-ninety-fifth` / `A-W445`.

## Index

| Prior | Status |
|-------|--------|
| W435–W444 TLS ladder (product / app-data / HostFn / idle / env ALPN / concurrent) | **met** |
| This window language String alpn parameter | **met** |
| Array-typed alpn product API | **still unmet** |
| Env no-idle first-slice | **met** (W457 / ADR-801) |
| Language noIdle / daemon GA / thread-pool product API | **still unmet** |
| B-04 `[x]` (narrow exit) | **met** (W446 / ADR-790) |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Arity | 5 args: host, port, cert, key, alpn |
| Precedence | non-empty alpn arg → env `SYX_TLS_SERVE_ALPN` → default `syx/1` |
| Facade | `serveOnce` → `""`; `serveOnceAlpn(..., alpn)` |
| Concurrent / idle | W444 thread-per-conn + idle drain retained |
| HostFn | still `TlsServe`; count **554**; status **implemented** |
| Forbidden claim | Array alpn API / no-idle daemon / B-04 `[x]` without ADR-790 / MIR blob delete this window / new HostFn |
| Honesty | Language String alpn **≠** Array API / **≠** daemon / **≠** language exports auto-discovery |

## Evidence

- [`crates/syx-vm/src/hosts.rs`](../../crates/syx-vm/src/hosts.rs) `tls_serve_alpn_protocols(alpn_arg)`
- [`packages/net/tls.sy`](../../packages/net/tls.sy) `serveOnce` / `serveOnceAlpn`
- Nails: `qs1_tls_serve_native_llvm_alpn_lang` + empty-arg paths
- Meta: `tls_server_alpn_lang.rs`
- Cross: [`tls-server-concurrent.md`](tls-server-concurrent.md), [`tls-server-alpn-config.md`](tls-server-alpn-config.md)

## Frozen

- Language noIdle arg / daemon GA without env bound (env no-idle first-slice delivered W457 / ADR-801)
- Array-typed / structured alpn product API
- Thread-pool configuration / sizing product API
- Generic user-callback handler API
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
