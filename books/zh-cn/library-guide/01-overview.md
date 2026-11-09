# 包一览

工具链嵌入下列可 import 模块（`packages/internal/manifest.json` → `importable_modules`）：

| `import` | 路径 | 用户面 | 范例 |
|----------|------|--------|------|
| `Numerics` | `packages/linalg/mod.sy` | packed-SYX `Mat`（Host 后端） | [examples/linalg/](../../../examples/linalg/) |
| `System` | `packages/system/mod.sy` | `ProcessInfo` / `File` / `FilePath` / `Env` / process 辅助；失败抛 `SystemError` | [examples/os_info/](../../../examples/os_info/) · [examples/file_tool/](../../../examples/file_tool/) |
| `Logging` | `packages/log/mod.sy` | 级别过滤 + 结构化字段行（`Log`） | [examples/log_fields.sy](../../../examples/log_fields.sy) |
| `Networking` | `packages/networking/mod.sy` | `Http` / `Url`；TLS 客户端 | [examples/http_tool/](../../../examples/http_tool/) |
| `Crypto` | `packages/crypto/mod.sy` | 哈希等 | [examples/basic_demo.sy](../../../examples/basic_demo.sy) |
| `Text` | `packages/text/mod.sy` | `Fmt` / `Regex` / `Encoding` / `Bytes` | — |
| `Synchronization` | `packages/sync/mod.sy` | Channel / Mutex / Semaphore | [examples/concurrency_control/](../../../examples/concurrency_control/) |
| `SQL` | `packages/data/sql.sy` | `:memory:` CRUD | [examples/sql_tool/](../../../examples/sql_tool/) |
| `Testing` | `packages/testing/mod.sy` | `assert` / `assertEq` | [examples/test_demo.sy](../../../examples/test_demo.sy) |
| `DI` | `packages/di/mod.sy` | Service DI | [examples/service_di/](../../../examples/service_di/) |
| `UI` | `packages/ui/mod.sy` | SynUI 桥；**非** 产品 GUI GA | [examples/gui_demo.sy](../../../examples/gui_demo.sy) |
| `Tracing` | `packages/trace/mod.sy` | spans（MVP） | — |
| `Metrics` | `packages/metrics/mod.sy` | counters / gauges（MVP） | — |
| `Profiling` | `packages/profiler/mod.sy` | wall / CPU sections（MVP） | — |
| `JSON` | CORE | encode / decode | — |
| `Fmt` | CORE / Text | 格式化辅助 | — |
| `OrderedSet` / `Deque` | CORE collections | 有序集合 / 双端队列 | — |
| `Math` / `Random` / `SIMD4` | CORE | 数学 / RNG / SIMD 辅助 | — |

`Math` 走 CORE prelude（`packages/core/`），**不必** `import Math` 即可用静态 `Math.*`；**没有** prelude `Mat`（请 `import Numerics`）。

`Object` / `Dictionary` 同属 CORE（[`object.sy`](../../../packages/core/builtins/reference/object.sy)）；默认字面量 → Dictionary；显式 `: Object` 仍用于残余路径。

## 导入

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

同目录 `import foo` 仍解析本地 `foo.sy`；嵌入名（`Numerics`、`System`、…）由工具链解析到 `packages/`。
