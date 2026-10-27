# Package Overview

The toolchain embeds the following importable modules (`packages/internal/manifest.json` → `importable_modules`):

| `import` | Path | User surface | Example |
|----------|------|--------|--------|
| `Linalg` | `packages/linalg/mod.sy` | packed-SYX `Mat` (handle + `faer_*` Host) | [examples/linalg/](../../examples/linalg/) |
| `Os` | `packages/os/mod.sy` | cpu/mem/user/hostname/disk/IPv4; failures throw `OsError` | [examples/os_info/](../../examples/os_info/) |
| `Log` | `packages/log/mod.sy` | level filtering + structured field lines | [examples/log_fields.sy](../../examples/log_fields.sy) |
| `IO` | `packages/io/mod.sy` | `File` / `Directory` / `Stdin` | [examples/file_tool/](../../examples/file_tool/) · [examples/io_demo.sy](../../examples/io_demo.sy) |
| `Net` | `packages/net/` | `Http` / `Url`; TLS client | [examples/http_tool/](../../examples/http_tool/) |
| `Crypto` | `packages/crypto/mod.sy` | hashing and more | [examples/basic_demo.sy](../../examples/basic_demo.sy) |
| `Text` | `packages/text/mod.sy` | `Fmt` / `Regex` / `Encoding` / `Bytes` | — |
| `DateTime` | `packages/datetime/mod.sy` | `DateTime` / `Duration` / `Clock` | — |
| `Process` | `packages/process/mod.sy` | args/env/exec; spawn/wait still has sentinel paths | — |
| `Sync` | `packages/sync/mod.sy` | Channel / Mutex / Semaphore | [examples/concurrency_control/](../../examples/concurrency_control/) |
| `Di` | `packages/di/mod.sy` | Service DI | [examples/service_di/](../../examples/service_di/) |
| `Sql` | (stdlib Host + package) | `:memory:` CRUD | [examples/sql_tool/](../../examples/sql_tool/) |
| `Testing` | `packages/testing/mod.sy` | `assert` / `assertEq` | [examples/test_demo.sy](../../examples/test_demo.sy) |
| `UI` | `packages/ui/mod.sy` | SynUI bridge; **not** product GUI GA | [examples/gui_demo.sy](../../examples/gui_demo.sy) |

`Math` comes in through the CORE prelude (`packages/core/`), so static `Math.*` is usable **without** `import Math`; there is **no** prelude `Mat`.

`Object` / `Dictionary` are likewise part of CORE ([`object.sy`](../../packages/core/builtins/reference/object.sy)); todo 4 Object **soft-close met** (default literal → Dictionary; explicit `: Object`; ≠ `.merge` / a real HashMap); see [language-guide 05](../language-guide/05-collection-types.md).

`Metrics` / `Trace` / `Profiler` / `Observation` have package sources, but do **not** treat them as a stable user surface while they are absent from the `importable_modules` table above.

## Importing

```syx
import Linalg
import Os
import Log

func main() {
    let a = Mat.from([1.0, 0.0, 0.0, 1.0], rows: 2, cols: 2)
    print(a.rows)
    print(Os.cpuCount())
    Log.info("ok")
}
```

`import foo` in the same directory still resolves the local `foo.sy`; embedded names (`Linalg`, `Os`) are resolved by the toolchain to `packages/`.
