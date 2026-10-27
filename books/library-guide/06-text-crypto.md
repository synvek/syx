# Text 与 Crypto

## Text

**实现状态：** `Fmt` / `Regex` / `Encoding` / `StringBuilder` / `Bytes` 薄封装 **已实现**。

| 类型 | 典型 API |
|------|----------|
| `Fmt` | `print` / `printLine` / `format` / `debug` |
| `Regex` | `isMatch` / `findAll` / `replaceAll` |
| `Encoding` | `base64Encode` / `hexEncode` |
| `Bytes` / `StringBuilder` | 缓冲拼接 |

字符串内置方法（`split`、`matches`）仍走 CORE，不必 `import Text`。

## Crypto

**实现状态：** `Crypto.sha256` 等 **已实现**（Host）。动态插件另见 [examples/plugin_demo](../../examples/plugin_demo/) 与 [语言指南 · 运行时扩展](../language-guide/33-runtime-extensions.md)。

```syx
import Crypto

func main() {
    print(Crypto.sha256("data"))
}
```

## 其它嵌入包（短）

| 包 | 要点 |
|----|------|
| `DateTime` | `now` / `parseIso8601` / `format`；`Duration.seconds` 返回毫秒整数 |
| `Process` | `args` / `env` / `exec`；legacy `spawn`/`wait` 仍可能返回 `-1`，新路径用 `ProcessBuilder`/`Child` |
| `Testing` | `assert` / `assertEq` / `test` |
| `Di` | Service DI；见 [examples/service_di](../../examples/service_di/) |
| `Sql` | 见 [examples/sql_tool](../../examples/sql_tool/) |
| `Sync` | Channel / Mutex / Semaphore |
| `UI` | SynUI 桥；**deferred**，勿当产品 GUI |
