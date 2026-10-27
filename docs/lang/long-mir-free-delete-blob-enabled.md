# Long delete_syx_mir_blob_enabled first-slice (W434 / ADR-778)

## Status

**Long delete_syx_mir_blob_enabled first-slice met** (narrative flag `delete_syx_mir_blob_enabled=true` after W413 emit omit + W431 sidecar omit + W432 mir_free + W433 named stub clear; production already has no `@syx_mir_blob` / `@syx_metadata_sidecar` — this window does **not** change llvm/C emit).

Fingerprint: `two-hundred-eighty-fourth` / `A-W434`.

## Index

| Prior | Status |
|-------|--------|
| W433 Named HostFn stub CALL_HOST clear | **met** (`named_hostfn_stub_call_host_clear_signed=true`) |
| W432 Long MIR-free first-slice | **met** (`mir_free_enabled=true`) |
| W413 / W431 blob + sidecar emit omit | **met（生产）** (constants gone) |
| This window delete flag first-slice | **met** (`delete_syx_mir_blob_enabled_first_slice_signed=true`; `delete_syx_mir_blob_enabled=true`) |
| TLS product status raise | **met** (`status=implemented`；ADR-779 / W435) |
| TLS handler application-data | **met** (ADR-780 / W436；`HANDLED`) |
| TLS multi-conn (N=2) | **met** (ADR-781 / W437) |
| TLS ALPN (`syx/1`) | **met** (ADR-782 / W438) |
| Short absolute thresholds / default multi-root | absolute thresholds **met** (W439)；默认多根 **met** (W440 / ADR-784) |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Flag | `delete_syx_mir_blob_enabled=true` on current wave_b + Delete inventory |
| Emit | unchanged — already empty install + register; no blob/sidecar embed |
| Forbidden claim | TLS handler deepen / Short default multi-root (delivered W440) / B-05 `[x]` / this-window emit delete (**≠** delivered) |
| Honesty | Delete flag **≠** “deleted blob in this window” / **≠** TLS handler / **≠** B-axis `[x]` |

## Public inventory

- `MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory`
- `mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory`
- Predicates: named clear ok + mir_free ok + delete on + blob not embedded + inventory overlap (policy-fixed)
- Flags: `delete_syx_mir_blob_enabled_first_slice_signed=true`; **`delete_syx_mir_blob_enabled=true`**; `mir_free_enabled=true`; named clear signed; gap closed; `blob_still_embedded=false`; `sidecar_constant_still_embedded=false`

Historical note: W410–W433 inventories keep `delete_syx_mir_blob_enabled=false` as that-window honesty.

Cross: [`long-mir-free-named-stub-callhost-clear.md`](long-mir-free-named-stub-callhost-clear.md), [`long-mir-free-enabled.md`](long-mir-free-enabled.md), [`long-mir-free-emit-omit.md`](long-mir-free-emit-omit.md), [`wave-b-horizon-readiness.md`](wave-b-horizon-readiness.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Changing llvm/C emit (blob/sidecar already gone)
- Claiming TLS handler deepen / ALPN / multi-conn as this-window delivery
- Claiming B-06 `[x]` without ADR-792 exit narrow / claiming B-04/B-05 `[x]` without ADR-790/791 exit context
- Short default emit auto multi-root; absolute byte thresholds
- Claiming this window removed `@syx_mir_blob` from emit (already W413)
- Claiming production-default JIT
- Clearing W410–W433 historical inventory `delete_syx_mir_blob_enabled=false` honesty

## Next

W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
