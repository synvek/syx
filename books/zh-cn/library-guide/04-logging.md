# Logging

**实现状态：** 级别 + 结构化字段行 **已实现**；**不是** JSON 日志 GA。模块名 `import Logging`；类型面仍是 `Log`。

```syx
import Logging

func main() {
    Log.setMinLevel("INFO")
    Log.info("hello-log")
    print(Log.takeLast())
}
```

范例：[examples/log_fields.sy](../../../examples/log_fields.sy)

| API | 说明 |
|-----|------|
| `debug` / `info` / `warn` / `error` | 纯消息 |
| `setMinLevel` / `minLevel` | 全局下限：`DEBUG` / `INFO` / `WARN` / `ERROR` |
| `infoFields` 等 | 结构化字段；低于下限则空串 |
| `takeLast` | 读走最近成功发出的一行（测过滤用） |
| `policy` | 策略指纹；≠ JSON GA |

`Metrics` / `Tracing` / `Profiling` 是相邻可观测包（已在 `importable_modules`）；本章不把它们写成产品可观测 GA。
