# Long native-metadata wire first-slice (W412 / ADR-756)

## Status

**Long native-metadata wire first-slice partial met** (public C ABI + inventory; production still via blob).

Fingerprint: `two-hundred-sixty-second` / `A-W412`.

## Index

| Prior | Status |
|-------|--------|
| W411 Long native-metadata deepen thin | **met** (`native_metadata_deepen_thin_signed=true`) |
| W410 Long MIR-free readiness thin | **met** (`long_mir_free_readiness_thin_signed=true`) |
| Medium execution-omit (W409) | **partial met** (`aot_execution_segment_omitted=true`) |
| This window native-metadata wire | **partial met** (`native_metadata_wire_first_slice_signed=true`) |
| `native_metadata_without_blob_gap_open` | **closed** (`false` — public no-blob entry) |
| Production default entry | **emit omit met (W413)** — see [`long-mir-free-emit-omit.md`](long-mir-free-emit-omit.md) |
| MIR-free / delete all metadata | **still unmet** (`mir_free_enabled=false`) |

## Wire contract (hard)

| Item | Meaning |
|------|---------|
| Public C ABI | `syx_rt_install_metadata_sidecar(ptr, len)` |
| Path | `decode_module_metadata_sidecar` → `install_native_task_hosts` → `install_runtime_from_sidecar_only` |
| Observability | `SIDECAR_STRING_POOL_PRIMARY` / `AOT_EXECUTION_SEGMENT_OMITTED` + `NATIVE_METADATA_ALTERNATE_ENTRY` |
| Not via | SYXD peel / `syx_rt_load_mir_blob` |
| Production `syx_user_init` (at wire signing) | was `load_mir_blob`; **W413** cut over to `install_metadata_sidecar` |
| Gap #1 | `native_metadata_without_blob_gap_open=false` |
| Honesty | wire **≠** MIR-free / **≠** delete blob / **≠** emit cutover / **≠** B-05 `[x]` |

## Public inventory

- `MirModuleLongHorizonNativeMetadataWireFirstSliceInventory`
- `mir_module_long_horizon_native_metadata_wire_first_slice_inventory`
- Predicates: deepen thin ok + sidecar decode ok + MIR-free off + inventory overlap
- Flags: `native_metadata_wire_first_slice_signed=true`; `native_metadata_deepen_thin_signed=true`; `long_mir_free_readiness_thin_signed=true`; `mir_free_enabled=false`; `delete_syx_mir_blob_enabled=false`; `blob_still_embedded=true`; `syx_user_init_still_loads_blob=true`; `native_metadata_without_blob_gap_open=false`; other Long gaps **still open**

Cross: [`long-mir-free-native-metadata-deepen.md`](long-mir-free-native-metadata-deepen.md), [`long-mir-free-readiness.md`](long-mir-free-readiness.md), [`b05-execution-omit.md`](b05-execution-omit.md), [`b05-sidecar-primary.md`](b05-sidecar-primary.md), [`b05-dual-load-runtime.md`](b05-dual-load-runtime.md), [`b05-aot-sidecar-embed.md`](b05-aot-sidecar-embed.md), [`wave-b-horizon-readiness.md`](wave-b-horizon-readiness.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Deleting `@syx_mir_blob` / stopping `syx_user_init` → `load_mir_blob` / MIR-free emit cutover — **out of scope** (≠ this window)
- Claiming B-06 `[x]` without ADR-792 exit narrow (live B-06 `[x]` via ADR-792 / W448 — [`b06-missing-align-exit.md`](b06-missing-align-exit.md); ≠ Missing 清零 / stub rewrite / B-07 `[x]` / this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds
- Clearing W411 deepen thin / W410 readiness thin

## Next

W412 Long native-metadata wire first-slice **已交**（ADR-756）→ W413 Long emit-omit first-slice **已交**（ADR-757）→ W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429+ — sidecar-omit deepen/wire/omit；或 TLS / Short；`mir_free_enabled=true` **out of scope**（≠ this window）.
