# 包一览

工具链嵌入下列可 import 模块（`packages/internal/manifest.json` → `importable_modules`）：

| `import` | 路径 | 用户面 | 范例 |
|----------|------|--------|------|
| `Linalg` | `packages/linalg/mod.sy` | packed-SYX `Mat`（句柄 + `faer_*` Host） | [examples/linalg/](../../../examples/linalg/) |
| `Os` | `packages/os/mod.sy` | cpu/mem/user/hostname/disk/IPv4；失败抛 `OsError` | [examples/os_info/](../../../examples/os_info/) |
| `Log` | `packages/log/mod.sy` | 级别过滤 + 结构化字段行 | [examples/log_fields.sy](../../../examples/log_fields.sy) |
| `IO` | `packages/io/mod.sy` | `File` / `Directory` / `Stdin` | [examples/file_tool/](../../../examples/file_tool/) · [examples/io_demo.sy](../../../examples/io_demo.sy) |
| `Net` | `packages/net/` | `Http` / `Url`；TLS 客户端 | [examples/http_tool/](../../../examples/http_tool/) |
| `Crypto` | `packages/crypto/mod.sy` | 哈希等 | [examples/basic_demo.sy](../../../examples/basic_demo.sy) |
| `Text` | `packages/text/mod.sy` | `Fmt` / `Regex` / `Encoding` / `Bytes` | — |
| `DateTime` | `packages/datetime/mod.sy` | `DateTime` / `Duration` / `Clock` | — |
| `Process` | `packages/process/mod.sy` | args/env/exec；spawn/wait 仍有哨兵路径 | — |
| `Sync` | `packages/sync/mod.sy` | Channel / Mutex / Semaphore | [examples/concurrency_control/](../../../examples/concurrency_control/) |
| `Di` | `packages/di/mod.sy` | Service DI | [examples/service_di/](../../../examples/service_di/) |
| `Sql` | （stdlib Host + 包） | `:memory:` CRUD | [examples/sql_tool/](../../../examples/sql_tool/) |
| `Testing` | `packages/testing/mod.sy` | `assert` / `assertEq` | [examples/test_demo.sy](../../../examples/test_demo.sy) |
| `UI` | `packages/ui/mod.sy` | SynUI 桥；**非** 产品 GUI GA | [examples/gui_demo.sy](../../../examples/gui_demo.sy) |

`Math` 走 CORE prelude（`packages/core/`），**不必** `import Math` 即可用静态 `Math.*`；**没有** prelude `Mat`。

`Object` / `Dictionary` 同属 CORE（[`object.sy`](../../../packages/core/builtins/reference/object.sy)）； todo 4 Object **soft-close met**（默认字面量 → Dictionary；显式 `: Object`；≠ `.merge` / 真 HashMap）；详见 [language-guide 05](../language-guide/05-collection-types.md)。

`Metrics` / `Trace` / `Profiler` / `Observation` 有包源，但 **不在** 上表 `importable_modules` 里时不要当稳定用户面。

## 导入

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

同目录 `import foo` 仍解析本地 `foo.sy`；嵌入名（`Linalg`、`Os`）由工具链解析到 `packages/`。
