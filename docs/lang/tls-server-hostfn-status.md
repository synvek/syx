# TLS TlsServe HostFn status→implemented first-slice (W441 / ADR-785)

## Status

**TLS `TlsServe` HostFn status→implemented first-slice met** (`status=partial→implemented` on existing N=2 + HANDLED + ALPN `syx/1` behavior at W441 charter; live accept **unbounded** idle-timeout (W442 / ADR-786); live ALPN from env **`SYX_TLS_SERVE_ALPN`** (W443 / ADR-787); live concurrent thread-per-conn (W444 / ADR-788); live language String alpn 5th arg (W445 / ADR-789); HostFn count **554**; ≠ Array-typed alpn API / ≠ no-idle daemon / ≠ thread-pool product API / ≠ generic user callback).

Fingerprint: `two-hundred-ninety-first` / `A-W441`.

## Index

| Prior | Status |
|-------|--------|
| W435 TLS product status raise | **met** (`net.tls.server` `status=implemented`) |
| W436 TLS handler application-data | **met** |
| W437 TLS multi-conn (N=2) | **met** (historical fixed N; live unbounded) |
| W438 TLS ALPN (`syx/1`) | **met** |
| This window HostFn status raise | **met** (`TlsServe` `status=implemented`) |
| Unbounded multi-conn (sequential idle) | **met** (W442 / ADR-786；later) |
| Configurable ALPN (env) | **met** (W443 / ADR-787；`SYX_TLS_SERVE_ALPN`；later) |
| Language String alpn parameter (5th arg) | **met** (W445 / ADR-789；later) |
| Concurrent serve (thread-per-conn + idle) | **met** (W444 / ADR-788；later) |
| B-04 `[x]` (narrow exit) | **met** (W446 / ADR-790) |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| HostFn | `TlsServe` **`status = "implemented"`**; backends vm/mir/llvm/wasm=`real`; **cranelift=`unsupported`** (JIT axis; ≠ cranelift GA) |
| Behavior | W441 charter **unchanged** then: N=2 accepts + HANDLED + fixed ALPN `syx/1`; **live** (W442): sequential unbounded idle-timeout accept + HANDLED; **live** (W443): env ALPN defaulting to `syx/1` |
| Count | HostFn **554** (no new HostFn) |
| Cap | `net.tls.server` remains **`implemented`** (W435) |
| Forbidden claim | Array alpn API / no-idle daemon / thread-pool product API / B-04 `[x]` without ADR-790 / MIR blob delete this window / cranelift=`real` / rewriting W441 as if unbounded, env ALPN, concurrent, or language alpn delivered then |

## Evidence

- [`packages/internal/host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) `TlsServe`
- [`crates/syx-driver/tests/stdlib/hostfn_tls.rs`](../../crates/syx-driver/tests/stdlib/hostfn_tls.rs)
- Cross: [`tls-server-alpn.md`](tls-server-alpn.md), [`tls-server-status-raise.md`](tls-server-status-raise.md), [`tls-server-unbounded.md`](tls-server-unbounded.md)

## Frozen

- Unbounded multi-conn accept loop (delivered later in W442 / ADR-786 — historical W441 Frozen)
- No-idle infinite accept daemon / thread-pool configuration product API
- Configurable ALPN list API (env delivered later in W443 / ADR-787 — historical W441 Frozen) / Array-typed alpn product API / generic user-callback handler
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
