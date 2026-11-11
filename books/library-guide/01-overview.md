# Package Overview

The toolchain embeds the following importable modules (`packages/internal/manifest.json` → `importable_modules`):

| `import` | Path | User surface | Example |
|----------|------|--------|--------|
| `Numerics` | `packages/linalg/mod.sy` | packed-SYX `Mat` (Host-backed) | [examples/linalg/](../../examples/linalg/) |
| `System` | `packages/system/mod.sy` | `ProcessInfo` / `File` / `FilePath` / `Env` / process helpers; failures throw `SystemError` | [examples/os_info/](../../examples/os_info/) · [examples/file_tool/](../../examples/file_tool/) |
| `Logging` | `packages/log/mod.sy` | level filtering + structured field lines (`Log`) | [examples/log_fields.sy](../../examples/log_fields.sy) |
| `Networking` | `packages/networking/mod.sy` | `Http` / `Url`; TLS client | [examples/http_tool/](../../examples/http_tool/) |
| `Crypto` | `packages/crypto/mod.sy` | hashing and more | [examples/basic_demo.sy](../../examples/basic_demo.sy) |
| `Text` | `packages/text/mod.sy` | `Fmt` / `Regex` / `Encoding` / `Bytes` | — |
| `Synchronization` | `packages/sync/mod.sy` | Channel / Mutex / Semaphore | [examples/concurrency_control/](../../examples/concurrency_control/) |
| `SQL` | `packages/data/sql.sy` | `:memory:` CRUD; `SqlRow` accessors `row: Dictionary` (R-11; ≠ Object) | [examples/sql_tool/](../../examples/sql_tool/) |
| `Testing` | `packages/testing/mod.sy` | `assert` / `assertEq` | [examples/test_demo.sy](../../examples/test_demo.sy) |
| `DI` | `packages/di/mod.sy` | Service DI | [examples/service_di/](../../examples/service_di/) |
| `UI` | `packages/ui/mod.sy` | SynUI bridge; **not** product GUI GA | [examples/gui_demo.sy](../../examples/gui_demo.sy) |
| `Tracing` | `packages/trace/mod.sy` | spans (MVP) | — |
| `Metrics` | `packages/metrics/mod.sy` | counters / gauges (MVP) | — |
| `Profiling` | `packages/profiler/mod.sy` | wall / CPU sections (MVP) | — |
| `JSON` | CORE | encode / decode | — |
| `Fmt` | CORE / Text | formatting helpers | — |
| `OrderedSet` / `Deque` | CORE collections | ordered set / deque | — |
| `Math` / `Random` / `SIMD4` | CORE | math / RNG / SIMD helpers | — |

`Math` comes in through the CORE prelude (`packages/core/`), so static `Math.*` is usable **without** `import Math`; there is **no** prelude `Mat` (use `import Numerics`).

`Object` / `Dictionary` are likewise part of CORE ([`object.sy`](../../packages/core/builtins/reference/object.sy)); default literal → Dictionary; explicit `: Object` remains for residual paths.

## Importing

```syx
import Numerics
import System
import Logging

func main() {
    let a = Mat.from([1.0, 0.0, 0.0, 1.0], rows: 2, cols: 2)
    print(a.rows)
    print(ProcessInfo.processorCount)
    Log.info("ok")
}
```

`import foo` in the same directory still resolves the local `foo.sy`; embedded names (`Numerics`, `System`, …) are resolved by the toolchain to `packages/`.
