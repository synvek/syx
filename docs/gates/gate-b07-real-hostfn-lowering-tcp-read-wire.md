# B-07 real HostFn lowering third-slice wire (W453 / ADR-797)

> **Status:** **B-07 third-slice wire signed** (ADR-797 / W453) — narrow `syx_rt_tcp_read` → `rt_call_host_native(HostFnId::TcpRead)` only; policy + inventory + meta.  
> Cross-index: [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md) / ADR-796; [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / ADR-795; [`gate-b07-real-hostfn-lowering-deepen.md`](gate-b07-real-hostfn-lowering-deepen.md) / ADR-794; [`gate-b07-real-hostfn-lowering.md`](gate-b07-real-hostfn-lowering.md) / ADR-793; [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).  
> Roadmap B-07 is `[x]` (W456 / ADR-800；narrow). **Not** global `lowering_enabled` (exit ≠ GA). Historical wire window **Not** claiming exit. **Not** global `lowering_enabled`. **Not** batch POP rewrite. **Not** capability 批量 flip. **Not** language `Module.exports`. **Not** MIR blob delete this window. **Not** no-idle daemon.  
> Honesty: no remaining HostFn-bound named `*_aot` POP after W451/W452; this slice wires the existing `syx_rt_tcp_read` declare (not a new `*_aot` symbol). Remaining `dataclass_aot` / `enum_aot` / `witness_aot` are non-HostFn IR helpers.

Fingerprint: `three-hundred-and-third` / `A-W453`.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| Thin + deepen + HttpGet + FileRead | **prerequisite** | W449–W452 signed |
| `syx_rt_tcp_read` → TcpRead | **in scope** | `rt_call_host_native(host_fn_index(HostFnId::TcpRead))` |
| Other POP stubs / new `*_aot` symbols | **out of scope** | unchanged / not created |
| Global HostFn lowering expansion | **out of scope** | `lowering_enabled=false` |
| B-07 `[x]` | **out of scope** | wire ≠ exit |

## Wire contract

| Stub | Behavior |
|------|----------|
| `syx_rt_http_get_aot` | HostFn `HttpGet` (W451) — unchanged |
| `syx_rt_file_read_aot` | HostFn `FileRead` (W452) — unchanged |
| `syx_rt_tcp_read` | HostFn dispatch via `rt_call_host_native` / `HostFnId::TcpRead` — **no** `rt_direct_exec(POP)` |
| `syx_rt_tcp_write` | HostFn `TcpWrite` (W454) — see [`gate-b07-real-hostfn-lowering-tcp-write-wire.md`](gate-b07-real-hostfn-lowering-tcp-write-wire.md) |
| `syx_rt_tcp_handle_release` and other POP | **Unchanged** |

Honesty: TcpRead third-slice wire ≠ B-07 `[x]` / ≠ batch stub rewrite / ≠ MIR blob delete.

## Deepen ladder (live)

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin gap index | W449 | **met** |
| 2. Deepen thin / candidate readiness | W450 | **met** |
| 3. First-slice wire (HttpGet) | W451 | **met** |
| 4. Second-slice wire (FileRead) | W452 | **met** |
| 5. Third-slice wire (TcpRead) | this document | **met** (`third_slice_wire_enabled=true`) |
| 6. Fourth-slice wire (TcpWrite) | W454 | **met** |
| 7. B-07 `[x]` | axis exit charter | **unmet** |

## Inventory

- `GateHostFnRealLoweringHorizonThirdSliceWireInventory` / `gate_hostfn_real_lowering_horizon_third_slice_wire_inventory`
- Flags: `b07_real_lowering_third_slice_wire_signed=true`; `third_slice_wire_enabled=true`; first+second live; `lowering_enabled=false`
- Predicates: second-slice floor + TcpRead wire body + overlap

**Evidence nails:** `crates/syx-driver/tests/native/gate_b07_real_hostfn_lowering_tcp_read_wire.rs` + roadmap B-07 still `[~]`.

## Frozen

- Claiming B-07 `[x]` / global HostFn lowering expansion this window
- Batch POP rewrite / Missing 清零 complete / absolute HostFn↔stub matrix exit
- Creating new `*_aot` symbols / capability table batch flip
- Language `Module.exports` → MIR `aot_export_roots` automatic discovery
- No-idle TLS daemon / Array alpn / thread-pool product API
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
