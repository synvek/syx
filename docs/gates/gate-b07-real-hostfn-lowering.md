# B-07 real HostFn lowering thin-open (W449 / ADR-793)

> **Status:** **B-07 thin floor signed** (ADR-793 / W449) — policy + inventory + meta only; indexes capability `backends.llvm=real` claims vs stub/AOT wiring using B-06 align floors.  
> Cross-index: [`gate-b06-llvm-stub-audit.md`](gate-b06-llvm-stub-audit.md) / ADR-643–658; [`gate-b06-missing-align-exit.md`](../../plan/archive/gates/gate-b06-missing-align-exit.md) / ADR-792（F-10 W584 archived）; [`gate-b07-real-hostfn-lowering-deepen.md`](gate-b07-real-hostfn-lowering-deepen.md) / ADR-794; [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / ADR-795; [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md) / ADR-796; [`gate-b07-real-hostfn-lowering-tcp-read-wire.md`](gate-b07-real-hostfn-lowering-tcp-read-wire.md) / ADR-797; [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).  
> Roadmap B-07 is `[~]`. Live deepen thin **met** via ADR-794 / W450. Live HttpGet first-slice wire **met** via ADR-795 / W451. Live FileRead second-slice wire **met** via ADR-796 / W452. Live TcpRead third-slice wire **met** via ADR-797 / W453. **Not** global `lowering_enabled` (exit ≠ GA). Historical thin/deepen window **Not** claiming exit alone. **Not** batch stub rewrite. **Not** capability 批量 flip. **Not** language `Module.exports`. **Not** MIR blob delete this window. **Not** no-idle daemon.

Fingerprint: `two-hundred-ninety-ninth` / `A-W449`.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| HostFn llvm=`real` count floor | **in scope** | `hostfn_llvm_real` from `llvm_stub_capability_align_inventory` (≥100) |
| Named `*_aot` Wired path | **in scope** | `named_aot_wired ≥ 1` |
| `call_host` / `call_host_aot` Wired | **in scope** | `call_host_path_wired=true` |
| Real lowering expansion | **out of scope** | `lowering_enabled=false` honesty |
| B-07 `[x]` / stub rewrite | **out of scope** | thin ≠ exit / ≠ semantic rewrite |

## Thin-open definition

B-07 **thin floor** = this document + public `GateHostFnRealLoweringHorizonThinInventory` / `gate_hostfn_real_lowering_horizon_thin_inventory` (`b07_real_lowering_thin_signed=true`) rollup B-06 align counts into a citeable B-07 gap index. W449 adds **policy + meta** only; it does **not** rewrite `llvm_stubs.rs`, expand real HostFn lowering, or flip capability rows.

**Evidence nails:** `GateHostFnRealLoweringHorizonThinInventory` + roadmap B-07 `[x]`（F-10 W591：~~`gate_b07_real_hostfn_lowering_thin.rs`~~ retired；proof = this SSOT + Inventory）。

## Inventory

- `GateHostFnRealLoweringHorizonThinInventory` / `gate_hostfn_real_lowering_horizon_thin_inventory`
- Flags: `b07_real_lowering_thin_signed=true`; `lowering_enabled=false`
- Predicates: hostfn real floor + named AOT wired + call_host wired + inventory overlap

Honesty: thin index ≠ B-07 `[x]` / ≠ true lowering delivery / ≠ Missing 清零 / ≠ MIR blob delete.

## Frozen

- Claiming B-07 `[x]` / global HostFn lowering expansion this window (live HttpGet + FileRead + TcpRead + TcpWrite + TcpHandleRelease wires via ADR-795 / W451 + ADR-796 / W452 + ADR-797 / W453 + ADR-798 / W454 + ADR-799 / W455 — [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-read-wire.md`](gate-b07-real-hostfn-lowering-tcp-read-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-write-wire.md`](gate-b07-real-hostfn-lowering-tcp-write-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-handle-release-wire.md`](gate-b07-real-hostfn-lowering-tcp-handle-release-wire.md); ≠ batch rewrite / ≠ `[x]`)
- Stub rewrite batch / Missing 清零 complete / absolute HostFn↔stub matrix exit
- Capability table batch flip to llvm=`real`
- Language `Module.exports` → MIR `aot_export_roots` automatic discovery
- No-idle TLS daemon / Array alpn / thread-pool product API
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
