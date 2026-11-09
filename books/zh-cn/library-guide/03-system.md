# System

**实现状态：** `ProcessInfo` cpu/mem/user/hostname/uptime/disk/load/`primaryIpv4` **已实现**（失败抛 `SystemError`）；**不是** 完整 sysinfo。文件 I/O 与路径辅助同属 `import System` 聚合（`File`、`FilePath`、`Env`、process 辅助）。

```syx
import System

func main() {
    print(ProcessInfo.processorCount)
    do {
        print(ProcessInfo.hostname())
    } catch (e) {
        print("hostname-unavailable")
    }
    print(FilePath.join(a: "/tmp", b: "demo.txt"))
}
```

范例：[examples/os_info/main.sy](../../../examples/os_info/main.sy)

| 类型 | 角色 |
|------|------|
| `ProcessInfo` | 机器 / 用户查询（`processorCount`、`hostname()` 等） |
| `FilePath` | 路径字符串的 join / normalize / exists / extension |
| `File` / `Directory` / `Stdin` | 文件系统与标准输入（见 [System I/O 与 Networking](05-system-networking.md)） |
| `SystemError` | 查询或 I/O 失败时抛出 |

`ProcessInfo` 与 process spawn/exec 辅助分开：前者查机器/用户，后者处理 args/env/exec。若干查询在注入失败环境变量时抛错（见 `packages/os/mod.sy` 注释），用户代码应 `try/catch`，不要依赖 `-1` 或空串哨兵。
