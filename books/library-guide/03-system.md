# System

**Implementation status:** `ProcessInfo` cpu/mem/user/hostname/uptime/disk/load/`primaryIpv4` **Implemented** (failures throw `SystemError`); this is **not** a full sysinfo replacement. File I/O and path helpers ship under the same `import System` aggregate (`File`, `FilePath`, `Env`, process helpers). R-47: `Env` is a caseless enum namespace; `FilePath` is a value `struct` (`rawValue`) with instance methods plus String static bridges.

```syx
import System

func main() {
    print(ProcessInfo.processorCount)
    do {
        print(ProcessInfo.hostname())
    } catch (e) {
        print("hostname-unavailable")
    }
    let p = FilePath(rawValue: "/tmp").joining(other: "demo.txt")
    print(p.rawValue)
    print(FilePath.join(a: "/tmp", b: "demo.txt"))
}
```

Example: [examples/os_info/main.sy](../../examples/os_info/main.sy)

| Type | Role |
|------|------|
| `ProcessInfo` | Machine / user queries (`processorCount`, `hostname()`, …) |
| `Env` | Process environment (`get` / `set`) — caseless enum |
| `FilePath` | Path value type (`rawValue`) + join/normalize/exists/extension |
| `File` / `Directory` / `Stdin` | Filesystem and stdin (see [System I/O and Networking](05-system-networking.md)) |
| `SystemError` | Thrown when a query or I/O path fails |

`ProcessInfo` is separate from process spawn/exec helpers: the former queries machines/users; the latter handles args/env/exec. Several queries throw when a failure-inducing environment variable is injected (see comments in `packages/os/mod.sy`), so user code should use `try/catch` rather than relying on `-1` or empty-string sentinels.
