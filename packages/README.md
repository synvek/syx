# SYX standard library (Phase 139)

Embedded modules shipped with the toolchain (SYX 117.0).

## Phase 139 highlights

- **Gate0:** P138 honest closeout; audit **2600/3050**（计划 2750，实测诚实下调）; manifest 117.0/phase 139; ADR-131; **SynUI deferred**; **Farm/Dist frozen at v19**
- **Track L — Language Surface (partial GA):** multiline `"""` · tuple · nested path · closed range · Character · call labels (`_`/external/mandatory) · remove `sealed` docs；**full Swift surface → 140+**
- **Track C (partial GA):** SIMD16 i8x16 all_true/any_true MVP (`wasm-simd16-guest-v22`); bitmask + popcnt + avgr_u + ge/le/ne/… inherited
- **GUI:** SynUI v12 inherited — **deferred** this phase (**no** SynUI v13 → **140+**)
- **Farm / Dist:** **frozen** at hsm-key-id / peer-announce-hold (v19); no v20 this phase
- **Os (inherited):** `primaryIpv4()` (`stdlib-os-v4`); disk/load + hostname/uptime inherited (**not** full sysinfo → **140+**)
- **DX:** `driver_gate` CR≥430；`lsp_phase139_real` ≥58；`syx-cache-v1`

## Packages

| Module | Path | Phase 139 notes |
|--------|------|-----------------|
| **Os** | `packages/os/mod.sy` | `import Os` — cpu/mem/user/home + hostname/uptime + disk/load + `primaryIpv4()`; **not** full sysinfo → **140+** |
| **Net** | `packages/net/` | `Http`/`Url` return `HTTPResponse`/`Url` (ADR-236); TLS client real path (ADR-237 / M2-S4-05); **not** server TLS GA / **not** streaming body |
| **UI** | `packages/ui/` | SynUI v12 host bridge inherited — **no v13 this phase** |
| **Di** | `packages/di/mod.sy` | Service DI v19 inherited from P138 |
| **Sync** | `packages/sync/mod.sy` | Channel length/capacity; Actor shared pool via `Concurrency` |
| **Linalg** | `packages/linalg/mod.sy` | `import Linalg` — packed-SYX `Mat`（W1057–W1100；padReflect 四向；**≠** 产品 Mat GA / **≠** E-00 / **≠** prelude）；指南 [`books/library-guide/02-linalg.md`](../books/library-guide/02-linalg.md)；范例 [`examples/linalg/`](../examples/linalg/) |
| **Core** | `packages/core/` | Builtins, `Concurrency` + Actor shared pool + DiscardOldest；**Math 增强轨收官→维护**（W902–W922 / ADR-1246–1266；F64/F32 static + 实例×42 parity；见 [`docs/lang/math-phase1.md`](../docs/lang/math-phase1.md) · [`math-phase2.md`](../docs/lang/math-phase2.md) · [`math-soft.md`](../docs/lang/math-soft.md) · [`math-special.md`](../docs/lang/math-special.md) · [`math-f32.md`](../docs/lang/math-f32.md) · [`math-f32-methods.md`](../docs/lang/math-f32-methods.md) · [`math-f64-methods.md`](../docs/lang/math-f64-methods.md)） |

See `packages/internal/manifest.json` for the full importable module list.
