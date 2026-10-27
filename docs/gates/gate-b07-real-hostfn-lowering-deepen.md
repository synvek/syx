# B-07 real HostFn lowering deepen thin (W450 / ADR-794)

> **Status:** **B-07 deepen thin floor signed** (ADR-794 / W450) — policy + inventory + meta only; first-slice **candidate / readiness** index on top of W449 thin + B-06 align floors.  
> Cross-index: [`gate-b07-real-hostfn-lowering.md`](gate-b07-real-hostfn-lowering.md) / ADR-793; [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / ADR-795; [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md) / ADR-796; [`gate-b07-real-hostfn-lowering-tcp-read-wire.md`](gate-b07-real-hostfn-lowering-tcp-read-wire.md) / ADR-797; [`gate-b06-llvm-stub-audit.md`](gate-b06-llvm-stub-audit.md); [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).  
> Roadmap B-07 is `[x]` (W456 / ADR-800；narrow). Live first-slice HttpGet wire **met** via ADR-795 / W451 (`first_slice_wire_enabled=true`). Live FileRead second-slice wire **met** via ADR-796 / W452 (`second_slice_wire_enabled=true`). Live TcpRead third-slice wire **met** via ADR-797 / W453 (`third_slice_wire_enabled=true`). **Not** global `lowering_enabled` (exit ≠ GA). Historical thin/deepen window **Not** claiming exit alone. **Not** batch stub rewrite. **Not** capability 批量 flip. **Not** language `Module.exports`. **Not** MIR blob delete this window. **Not** no-idle daemon.

Fingerprint: `three-hundredth` / `A-W450` (deepen charter); live wire: `A-W451` / `A-W452` / `A-W453`.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| Thin floor prerequisite | **in scope** | `b07_real_lowering_thin_signed=true` + thin preds |
| Category gap honesty | **in scope** | `hostfn_llvm_real >= named_aot_wired` |
| Named `*_aot` Wired floor | **in scope** | `named_aot_wired ≥ 1` |
| `hostfn_llvm_partial` countable | **in scope** | align partial count |
| First-slice candidate floor | **in scope** | named `*_aot` Wired proxy ≥1 |
| First-slice wire (HttpGet) | **live met** (W451) | `first_slice_wire_enabled=true`; see wire doc |
| Second-slice wire (FileRead) | **live met** (W452) | `second_slice_wire_enabled=true`; see file-read-wire |
| Third-slice wire (TcpRead) | **live met** (W453) | `third_slice_wire_enabled=true`; see tcp-read-wire |
| Global lowering / B-07 `[x]` | **out of scope** | `lowering_enabled=false`; deepen ≠ exit |

## Deepen ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin gap index | capability llvm=`real` vs stub/AOT wiring | **met** (W449 / ADR-793) |
| 2. Deepen thin / first-slice candidate readiness | this document | **met** (W450 / ADR-794) |
| 3. First-slice wire | narrow true HostFn lowering (HttpGet) | **met** (W451 / ADR-795) |
| 4. Second-slice wire | FileRead (`file_read_aot`) | **met** (W452 / ADR-796) |
| 5. Third-slice wire | TcpRead (`tcp_read`) | **met** (W453 / ADR-797) |
| 6. B-07 `[x]` | axis exit charter | **unmet** |

## Deepen-thin definition

B-07 **deepen thin floor** = this document + public `GateHostFnRealLoweringHorizonDeepenThinInventory` / `gate_hostfn_real_lowering_horizon_deepen_thin_inventory` (`b07_real_lowering_deepen_thin_signed=true`) documenting first-slice preference for HostFn claims that already have named `*_aot` Wired paths (candidate floor proxied by `named_aot_wired ≥ 1`). W450 added **policy + meta** only; W451 delivered the HttpGet wire; W452 delivered the FileRead second-slice wire; W453 delivered the TcpRead third-slice wire.

**Evidence nails:** `GateHostFnRealLoweringHorizonDeepenThinInventory` + roadmap B-07 `[x]`（F-10 W591：~~`gate_b07_real_hostfn_lowering_deepen_thin.rs`~~ retired；proof = this SSOT + Inventory）。

## Inventory

- `GateHostFnRealLoweringHorizonDeepenThinInventory` / `gate_hostfn_real_lowering_horizon_deepen_thin_inventory`
- Flags: `b07_real_lowering_deepen_thin_signed=true`; `b07_real_lowering_thin_signed=true`; `lowering_enabled=false`; live `first_slice_wire_enabled=true` (W451 HttpGet); live `second_slice_wire_enabled=true` (W452 FileRead); live third-slice via tcp-read-wire (W453 TcpRead)
- Predicates: thin floor + category gap + named AOT + partial countable + first-slice candidate floor + overlap

Honesty: deepen index ≠ B-07 `[x]` / ≠ global lowering / ≠ batch rewrite / ≠ MIR blob delete. Live HttpGet/FileRead/TcpRead/TcpWrite/TcpHandleRelease wires ≠ axis exit.

## Frozen

- Claiming B-07 `[x]` / global HostFn lowering expansion this window
- Batch POP `*_aot` rewrite / Missing 清零 complete / absolute HostFn↔stub matrix exit (live HttpGet + FileRead + TcpRead + TcpWrite + TcpHandleRelease wires via ADR-795 / ADR-796 / ADR-797 / ADR-798 / ADR-799 — [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-read-wire.md`](gate-b07-real-hostfn-lowering-tcp-read-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-write-wire.md`](gate-b07-real-hostfn-lowering-tcp-write-wire.md) / [`gate-b07-real-hostfn-lowering-tcp-handle-release-wire.md`](gate-b07-real-hostfn-lowering-tcp-handle-release-wire.md); ≠ batch rewrite / ≠ `[x]`)
- Capability table batch flip to llvm=`real`
- Language `Module.exports` → MIR `aot_export_roots` automatic discovery
- No-idle TLS daemon / Array alpn / thread-pool product API
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- Adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
