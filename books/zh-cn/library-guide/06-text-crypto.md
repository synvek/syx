# Text 与 Crypto

## Text

**实现状态：** `Fmt` / `Regex` / `Encoding` / `StringBuilder` / `Bytes` 薄封装 **已实现**。

| 类型 | 典型 API |
|------|----------|
| `Fmt` | `print` / `printLine` / `debug` 取 `Sendable`；`format(..., args: Array<Sendable>)`（R-12；≠ Object） |
| `Regex` | `isMatch` / `findAll` / `replaceAll` |
| `Encoding` | `base64Encode` / `hexEncode` |
| `Bytes` / `StringBuilder` | 缓冲拼接 |

字符串内置方法（`split`、`matches`）仍走 CORE，不必 `import Text`。

## Crypto

**实现状态：** `Crypto.sha256` 等 **已实现**（Host）。动态插件另见 [examples/plugin_demo](../../../examples/plugin_demo/) 与 [语言指南 · 运行时扩展](../language-guide/33-runtime-extensions.md)。

```syx
import Crypto

func main() {
    print(Crypto.sha256("data"))
}
```

## 其它嵌入包（短）

| 包 | 要点 |
|----|------|
| `Testing` | `assert` / `assertEq(Sendable,Sendable)` / `test(fn: Function)`（`import Testing`；R-12） |
| `DI` | Service DI；见 [examples/service_di](../../../examples/service_di/) |
| `SQL` | `SqlRow` 访问器 `row: Dictionary`（R-11）；见 [examples/sql_tool](../../../examples/sql_tool/) |
| `Synchronization` | Channel / Mutex / Semaphore |
| `UI` | SynUI 桥；**deferred**，勿当产品 GUI |
| `Tracing` / `Metrics` / `Profiling` | 可观测 MVP；非产品 OTel / Prometheus / pprof GA |
