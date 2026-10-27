# TLS language noIdle parameter first-slice (W458 / ADR-802)

## Status

**TLS language noIdle parameter first-slice met** (`host_tls_serve` arity `(host, port, cert, key, alpn, noIdle)`; 6th Int: `≠0` skips 400ms idle; `0` → env `SYX_TLS_SERVE_NO_IDLE` → default idle; facade `serveOnce`/`serveOnceAlpn` pass `0`; `serveOnceNoIdle` / `serveOnceAlpnNoIdle` pass `1`; concurrent + HANDLED + ALPN retained; optional `SYX_TLS_SERVE_MAX_HANDLED` retained; `TlsServe` remains **`implemented`**; HostFn count **554**; ≠ Array alpn / ≠ thread-pool / ≠ daemon GA / ≠ language exports).

Fingerprint: `three-hundred-and-eighth` / `A-W458`.

## Index

| Prior | Status |
|-------|--------|
| W435–W445 TLS ladder | **met** |
| W457 env no-idle first-slice | **met** (fallback when language `noIdle=0`) |
| This window language noIdle 6th Int | **met** |
| Array-typed alpn / thread-pool product API | **still unmet** |
| Daemon GA (infinite product surface) | **still unmet** |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Arity | 6 args: host, port, cert, key, alpn, noIdle (Int) |
| Precedence | `noIdle ≠ 0` → skip idle; `noIdle == 0` → env `SYX_TLS_SERVE_NO_IDLE` → default 400ms idle |
| Facade | `serveOnce`/`serveOnceAlpn` → `0`; `serveOnceNoIdle`/`serveOnceAlpnNoIdle` → `1` |
| MAX_HANDLED | env optional bound retained (test/ops) |
| HostFn | still `TlsServe`; count **554**; status **implemented** |
| Forbidden claim | Array alpn / thread-pool / daemon GA / new HostFn / MIR blob delete this window |
| Honesty | Language noIdle **≠** daemon GA / **≠** HostFn+1 / **≠** Array alpn |

## Evidence

- [`crates/syx-vm/src/hosts.rs`](../../crates/syx-vm/src/hosts.rs) `no_idle_arg != 0 \|\| tls_serve_no_idle_env()`
- [`packages/net/tls.sy`](../../packages/net/tls.sy) `serveOnceNoIdle` / `serveOnceAlpnNoIdle`
- Nails: `tls_server_noidle_lang_native_cross_idle`
- Meta: `tls_server_noidle_lang.rs`
- Cross: [`tls-server-no-idle.md`](tls-server-no-idle.md), [`tls-server-alpn-lang.md`](tls-server-alpn-lang.md)

## Frozen

- Claiming daemon GA / infinite product HostFn without bound
- Array-typed / structured alpn product API
- Thread-pool configuration / sizing product API
- Generic user-callback handler API
- Changing default idle when `noIdle=0` and env unset
- Adding HostFn (keep 554)
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Claiming production-default JIT
- Language `Module.exports` → `aot_export_roots`

## Next

W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
