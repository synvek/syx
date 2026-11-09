# Logging

**Implementation status:** levels + structured field lines **Implemented**; **not** JSON logging GA. Import the module as `Logging`; the type surface is still `Log`.

```syx
import Logging

func main() {
    Log.setMinLevel("INFO")
    Log.info("hello-log")
    print(Log.takeLast())
}
```

Example: [examples/log_fields.sy](../../examples/log_fields.sy)

| API | Description |
|-----|------|
| `debug` / `info` / `warn` / `error` | Plain message |
| `setMinLevel` / `minLevel` | Global floor: `DEBUG` / `INFO` / `WARN` / `ERROR` |
| `infoFields` and friends | Structured fields; empty string when below the floor |
| `takeLast` | Reads out the most recent successfully emitted line (for testing filtering) |
| `policy` | Policy fingerprint; ≠ JSON GA |

`Metrics` / `Tracing` / `Profiling` are adjacent observability packages on `importable_modules`; this chapter does not treat them as a product observability GA.
