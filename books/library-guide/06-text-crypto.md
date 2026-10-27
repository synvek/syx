# Text and Crypto

## Text

**Implementation status:** thin wrappers `Fmt` / `Regex` / `Encoding` / `StringBuilder` / `Bytes` **Implemented**.

| Type | Typical API |
|------|----------|
| `Fmt` | `print` / `printLine` / `format` / `debug` |
| `Regex` | `isMatch` / `findAll` / `replaceAll` |
| `Encoding` | `base64Encode` / `hexEncode` |
| `Bytes` / `StringBuilder` | Buffered concatenation |

Built-in string methods (`split`, `matches`) still come from CORE, so `import Text` is unnecessary.

## Crypto

**Implementation status:** `Crypto.sha256` and others **Implemented** (Host). For dynamic plugins, see also [examples/plugin_demo](../../examples/plugin_demo/) and [Language Guide · Runtime Extensions](../language-guide/33-runtime-extensions.md).

```syx
import Crypto

func main() {
    print(Crypto.sha256("data"))
}
```

## Other Embedded Packages (Short)

| Package | Key points |
|----|------|
| `DateTime` | `now` / `parseIso8601` / `format`; `Duration.seconds` returns whole milliseconds |
| `Process` | `args` / `env` / `exec`; legacy `spawn`/`wait` may still return `-1`, use `ProcessBuilder`/`Child` on new paths |
| `Testing` | `assert` / `assertEq` / `test` |
| `Di` | Service DI; see [examples/service_di](../../examples/service_di/) |
| `Sql` | See [examples/sql_tool](../../examples/sql_tool/) |
| `Sync` | Channel / Mutex / Semaphore |
| `UI` | SynUI bridge; **deferred**, do not treat as a product GUI |
