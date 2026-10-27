# TLS no-idle first-slice (W457 / ADR-801)

## Status

**TLS no-idle long-running first-slice met** (env opt-in): non-empty `SYX_TLS_SERVE_NO_IDLE` skips the 400ms idle exit in `host_tls_serve`; optional positive `SYX_TLS_SERVE_MAX_HANDLED` bounds success return when `handled ≥ N` and workers empty (test/ops); default path (no env) retains 400ms idle after ≥1 handled; concurrent thread-per-conn + `HANDLED\n` + ALPN (language 5th arg / `SYX_TLS_SERVE_ALPN` / default `syx/1`) retained; `TlsServe` remains **`implemented`**; HostFn count **554**; language noIdle **met** (W458；≠ daemon GA) / ≠ Array alpn / ≠ thread-pool product API / ≠ daemon GA / ≠ new HostFn.

Fingerprint: `three-hundred-and-seventh` / `A-W457`.

## Index

| Prior | Status |
|-------|--------|
| W435–W445 TLS ladder (product / app-data / HostFn / idle / ALPN / concurrent / language alpn) | **met** |
| W442 default idle 400ms exit | **still met** (default path unchanged) |
| This window env no-idle + optional max-handled | **met** |
| Language `noIdle` parameter | **met** (W458 / ADR-802) |
| Array-typed alpn / thread-pool product API | **still unmet** |
| Daemon GA (infinite product surface) | **still unmet** |
| Language exports auto-discovery | **still unmet** |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Default | Unset env → idle exit after `handled≥1` && workers empty && **400ms** continuous WouldBlock (unchanged) |
| `SYX_TLS_SERVE_NO_IDLE` | Non-empty → **skip** idle break |
| `SYX_TLS_SERVE_MAX_HANDLED` | Optional positive int N → success return when `handled ≥ N` && workers empty |
| Concurrent / ALPN / HANDLED | W444–W445 retained |
| HostFn | still `TlsServe`; arity unchanged; count **554**; status **implemented** |
| Forbidden claim | Array alpn / thread-pool / daemon GA / new HostFn / MIR blob delete this window |
| Honesty | Env no-idle first-slice + language noIdle (W458) **≠** daemon GA / **≠** HostFn+1 |

## Evidence

- [`crates/syx-vm/src/hosts.rs`](../../crates/syx-vm/src/hosts.rs) `tls_serve_no_idle_env` / `tls_serve_max_handled_env`
- [`packages/net/tls.sy`](../../packages/net/tls.sy) facade comments (env; ≠ language noIdle)
- Nails: `tls_server_no_idle_native_cross_idle` (+ default idle regression via qs1 / prior TLS)
- Meta: `tls_server_no_idle.rs`
- Cross: [`tls-server-unbounded.md`](tls-server-unbounded.md), [`tls-server-concurrent.md`](tls-server-concurrent.md), [`tls-server-alpn-lang.md`](tls-server-alpn-lang.md)

## Frozen

- Claiming daemon GA without bound (language noIdle first-slice delivered W458 / ADR-802)
- Claiming daemon GA / infinite product HostFn without env opt-in
- Array-typed / structured alpn product API
- Thread-pool configuration / sizing product API
- Generic user-callback handler API
- Changing default idle when env unset
- Adding HostFn / changing `TlsServe` arity (keep 554)
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Claiming production-default JIT

## Next

W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
