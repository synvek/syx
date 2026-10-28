# SYX 快速使用指南

[English](../9_UsageGuide.md)


本文档面向 **SYX 语言用户与工具链使用者**，涵盖安装、CLI、项目配置、语言特性、标准库、工具链与进阶编译选项。技术实现细节请参阅 [docs/README.md](./README.zh-CN.md) 中的编译器文档。

---

## 目录

1. [简介](#简介)
2. [安装与构建](#安装与构建)
3. [快速开始](#快速开始)
4. [CLI 命令参考](#cli-命令参考)
5. [项目配置](#项目配置)
6. [语言基础](#语言基础)
7. [模块与依赖](#模块与依赖)
8. [标准库 API](#标准库-api)
9. [错误处理](#错误处理)
10. [空安全与 Optional](#空安全与-optional)
11. [高级语言特性](#高级语言特性)
12. [工具链](#工具链)
13. [编译与后端](#编译与后端)
14. [测试](#测试)
15. [示例项目](#示例项目)
16. [环境变量](#环境变量)
17. [诊断代码](#诊断代码)
18. [常见问题](#常见问题)

---

## 简介

SYX 是一门受 Swift 启发的编程语言，当前实现为 **Rust 编写的完整工具链**：

```
Source (.sy) → Lexer → Parser → Semantic → CodeGen → VM
                      ↘ (可选) MIR → optimize ↗
                      ↘ (可选) LLVM / WASM 后端
```

**默认运行方式**：编译为栈式字节码，在 **VmInstance**（别名 `WasmVM`）中解释执行，并通过 **Host Functions** 提供文件 I/O、HTTP 等能力。

**系统要求**

| 组件 | 版本 |
|------|------|
| Rust | 1.80+ |
| Cargo | 随 Rust 安装 |
| clang / LLVM | Optional：`--features full-native` 时用 sibling `../llvm` 或 `SYX_LLVM_PREFIX`（inkwell 直出 `.o` + **静态嵌入 LLD**）；见 [plan/topics/native-llvm.md](../../plan/topics/native-llvm.md) |

---

## 安装与构建

### 从源码构建

```bash
git clone <repo-url> synswx
cd synswx

# 调试构建
cargo build

# 发布构建（推荐日常使用）
cargo build --release
```

构建完成后，可通过 Cargo 调用 CLI：

```bash
cargo run -p syx-cli -- run examples/hello.sy
```

将 release 二进制加入 PATH（可选）：

```bash
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

### 验证安装

```bash
cargo test
```

全部通过后，工具链即可正常使用。

---

## 快速开始

### 单文件程序

创建 `hello.sy`：

```syx
func main() {
    print("Hello, SYX!")
}
```

运行：

```bash
syx run hello.sy
# 或
cargo run -p syx-cli -- run hello.sy
```

### 项目程序

目录结构：

```
my-app/
├── syx.toml
├── build.syb      # 可选：构建配置（Sy Build，不是源码）
└── main.sy
```

`syx.toml`：

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"
build = "build.syb"   # 可选；省略时默认查找 build.syb
```

`main.sy`：

```syx
func main() {
    print("Hello from project!")
}
```

运行：

```bash
cd my-app
syx run .
# 或
syx init my-app          # 生成 main.sy + build.syb + syx.toml
```

---

## CLI 命令参考

| 命令 | 说明 | 示例 |
|------|------|------|
| `syx run [path]` | 编译并执行文件或项目 | `syx run examples/file_tool` |
| `syx run --gui-visible [path]` | **已退役** — 旧 `libsyx_gui`/egui 路径；请用 SynUI `packages/ui` + `SYX_SYNUI_RUN=1` opt-in `run()` | 见下方 SynUI v2 说明 |
| `syx build [path]` | 编译（不执行） | `syx build examples/` |
| `syx build --native [path]` | MIR → native `app`（**LLVM sole**；`--object` 仅 `.o`） | `syx build --native .` |
| `syx build --wasm [path]` | MIR → WASM → `main.wasm` | `syx build --wasm .` |
| `syx emit [path] --format FMT` | 导出字节码 / IR（见下表） | `syx emit examples/hello.sy --format syxbc` |
| `syx run --wasm [path]` | 编译并运行 WASM（项目 / `.sy` / `.wasm`；支持 `print` 常量） | `syx run --wasm examples/hello.sy` |
| `syx check [path]` | 仅类型检查 | `syx check main.sy` |
| `syx test [path]` | 运行 `tests/**/*.sy`（递归；`--filter` / `--format human\|json\|tap\|junit`）；或单个 `.sy` | `syx test . --format junit` |
| `syx bench` / `cargo bench -p syx-driver --bench …` | C-09 窄 `[x]`：薄 allowlist CLI + Criterion / `compile_bench`（≠ coverage gate / fuzz / SLA） | `syx bench --list` · `syx bench path_join_criterion` |
| `syx cov` / `cargo llvm-cov -p syx-driver --test native --no-report` | C-09 coverage 窄 claimed（；文档 + CLI；≠ % gate / ≠ CI fail / ≠ fuzz） | `syx cov`（需本机 `cargo-llvm-cov`） |
| `syx fuzz` / `cargo fuzz run syx_smoke` | C-09 fuzz 窄 claimed + scaffold + CI `-runs=1` + C-09/平台维护地板（；≠ scaffold GA / ≠ 无界 fuzz） | `syx fuzz`（需 cargo-fuzz；CI **met** ；维护 **met**） |
| `syx prop` / `cargo test -p syx-driver --test property_smoke` | C-09 property 窄 claimed + `property_smoke` + CI first-slice + C-09/平台维护地板（；≠ proptestest GA / ≠ `c09_property_ci_ga`） | `syx prop` |
| `syx format [path]` | 格式化入口文件 | `syx format .` |
| `syx format --check [path]` | dry-run：递归检查 `.sy` 幂等（不写盘；C-06） | `syx format --check .` |
| `syx lint [path]` | 静态检查 | `syx lint .` |
| `syx doc [path]` | 生成 API 文档 | `syx doc .` → `docs/api.md` |
| `syx debug [path]` | 启动调试会话摘要 | `syx debug .` |
| `syx repl` | 交互式 REPL | `syx repl` |
| `syx add <name> --path PATH` | 添加 path 依赖并安装 | `syx add utils --path ../utils` |
| `syx add <name> --git URL [--rev TAG]` | 添加 git 依赖 | `syx add lib --git https://... --rev v1.0` |
| `syx install [path]` | 解析并安装依赖，写入 `syx.lock` | `syx install .` |
| `syx update [path]` | 重新解析并安装全部依赖 | `syx update .` |
| `syx publish [path] [--registry URL]` | 发布包到 registry | `syx publish .` |

### Ed25519 发布 → 验签（C-05）

发布与安装共用同一把 Ed25519 密钥对（单 publisher pin；与 C-02 trust-root 衔接）：

1. **发布**：设 `SYX_PUBLISHER_SIGNING_KEY` 为 32-byte seed 的 64 hex 字符 → `syx publish` → 产物签名形如 `syx1:ed25519:<pk>:<sig>`（checksum 上签）。未设密钥时 lock v8 强制策略拒签并报 `P012`。
2. **安装**：设 `SYX_REGISTRY_VERIFY_KEY` 为与发布者 verifying key 一致的 64 hex → install 验签；嵌入公钥必须匹配 pin（C-02）；失配或验签失败 → `P011`。
3. **诚实边界**：≠ trust-root GA / 多 publisher / 透明度日志 / 新 crypto；交叉证据见 `c02_*`（syx-pkg）与 `registry_signed_*`（evergreen）。

**路径参数**：省略时默认为当前目录 `.`。若路径是目录，则读取该目录下的 `syx.toml`；若是 `.sy` 文件，则作为单文件编译。

**全局选项**：

| 选项 | 说明 | 示例 |
|------|------|------|
| `--module-dir PATHS` | 额外模块搜索目录，多个路径用 `;` 分隔 | `syx --module-dir "./vendor" run main.sy` |
| `--enable-std` | 启用嵌入标准库（默认 `true`） | `syx --enable-std=false run main.sy` |
| `--backend BACKEND` | 历史标志；原生 AOT **仅 LLVM**（Cranelift AOT 已移除） | `syx build --native .` |

标准库模块（`math`、`io` 等）已嵌入 `syx` 可执行文件，**默认最高优先级**。关闭 `--enable-std` 后，`import math` 会从本地目录或 `--module-dir` 查找。

Core `Math` 增强轨（Soft / Special / F32·F64 实例，）**收官→维护签**（；HostFn **629**）。 Math llvm：**84/84** + 窄面 GA； QS1 Math **84/84 + narrow `qs1_math_ga=true`**； Math status **84/84 `status=implemented`**； Fmt status 试点 **FmtPrint/FmtPrintLine×2**； Path status 试点 **PathJoin/Normalize/Exists/Extension×4**； 中间 first-slice meta F-10 收缩 **×33**； File status 试点 **FileOpen/Close/Read/Write×4**； File rest **×11 / 全量 File\* `status=implemented`**； Http rest **×16 / 全量 Http\* `status=implemented`**； Sql core **×4 `status=implemented`**； Sql rest **×23 / 全量 Sql\* `status=implemented`**； Tcp **×7 / 全量 Tcp\* `status=implemented`**； Task **×8 / 全量 Task\* `status=implemented`**； Time/DateTime **×8 `status=implemented`**； Json **×11 / 全量 Json\* `status=implemented`**； Str/Unicode **×11 `status=implemented`**； Process/Child wasm-unsupported honesty **×14**； Log core ×4 mir+llvm **real**； Log rest ×8 / 全族 ×12 mir+l … ≠ 全量 HostFn status；仍 14 partial）。C-09 / 平台 / Math 维护链于 **收官**。详见 [docs/lang/qs1-d01-execute-adr-first-slice.md](../lang/qs1-d01-execute-adr-first-slice.md)。

---

## 项目配置

### `syx.toml` 清单

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies]
# 简单路径依赖
utils = "../utils/main.sy"

# 带 SemVer 的 path 依赖
[dependencies.local_math]
version = "1.0.0"
path = "../deps_math"

# git 依赖
[dependencies.lib]
version = "^0.2"
git = "https://github.com/org/lib.git"
rev = "v0.2.1"

# registry 依赖（需 [registry] 配置）
[registry]
default = "https://packages.sy.dev"

[dependencies.http-client]
version = "^1.0"
```

| 字段 | 说明 | 默认值 |
|------|------|--------|
| `name` | 项目名称 | `"app"` |
| `version` | 项目版本 | `"0.1.0"` |
| `main` | 入口 SYX 源文件（`.sy`） | `"main.sy"` |
| `build` | 构建配置文件路径（`.syb`） | 省略时默认 `build.syb`（若存在） |
| `dependencies` | 模块依赖映射 | 空 |
| `[plugins].paths` | Host 插件 manifest 路径（Phase 49+） | 空 |
| `[macros].exports` | 包导出宏名（Phase 51+） | 空 |
| `[registry].default` | 默认包 registry URL | `SYX_REGISTRY` 或生产默认 `https://packages.sy.dev` |

### `build.syb` 构建配置（Sy Build）

**扩展名 `.syb` = Sy Build**，是**声明式构建 DSL**，**不是** Sy 源码。不要在其中写 `func` / `class` / 业务逻辑；源码请用 `.sy`。

- 优先级：`build.syb` 中的 `native` / `plugins` / `features` / `compileOptions` **覆盖** `syx.toml` 同名配置；`syx.toml` 主要负责 `name` / `version` / `main` / `dependencies` / `build` 路径。
- 由 `syx init` 生成；可用 `build = "path/to/foo.syb"` 自定义路径。
- 解析器为行/块级 DSL（非完整 Sy 编译器）；任务图为 DAG，不是图灵完备脚本。

最小示例：

```syb
build {
    compileOptions {
        optLevel = release   # 或 debug
    }
}
```

常用块：

| 块 | 作用 | 示例 |
|----|------|------|
| `compileOptions` | 优化级别 | `optLevel = release` |
| `native` | 链接库与 flags | `libs = ["z"]` · `link_flags = ["-pthread"]` |
| `plugins` | Host 插件列表 | `"mycrypto"` |
| `features` | 特性开关 | `wasm_aot = true` |
| `task <name>` | 任务 DAG | `target = "wasm"` · `dependsOn = ["mir"]` |

任务 `target` 取值（执行器支持）：`mir` · `wasm` · `native` · `link` · `test`，以及 farm 相关 `remote` / `farm_queue` / `farm_worker`（需配置 URL 或环境变量）。

带任务图的示例：

```syb
build {
    compileOptions {
        optLevel = release
    }
    native {
        libs = ["z"]
        link_flags = ["-pthread"]
    }
    features {
        wasm_aot = true
    }
    task mir {
        target = "mir"
    }
    task wasm {
        dependsOn = ["mir"]
        target = "wasm"
    }
}
```

`syx build` / `syx build --native` / `syx build --wasm` 在项目目录下会读取该配置并合并进驱动选项；若定义了 `task`，则按依赖顺序执行。

### `syx.lock` 锁文件

执行 `syx install` / `syx add` / `syx update` 后生成。**编译带依赖的项目前必须先 install**（否则报 **P001**）。

```toml
[packages.local_math]
source = "path"
version = "1.0.0"
path = "../deps_math"

[packages.lib]
source = "git"
git = "https://github.com/org/lib.git"
rev = "abc123..."
dir = ".sy/vendor/lib"

[packages.http-client]
source = "registry"
registry = "https://packages.sy.dev"
version = "1.0.3"
checksum = "sha256:..."
dir = ".sy/vendor/http-client-1.0.3"
```

Git / registry 包安装到 `.sy/vendor/`（已加入 `.gitignore`）。path 依赖仅记录路径，不复制源码。

### 增量编译缓存

编译产物缓存在项目根目录 `target/cache/`（MIR/module bincode，指纹 `syx-cache-v1`）。若编译行为异常，可删除缓存后重试：

```bash
rm -rf target/cache
```

---

## 语言基础

### 类型

| 类型 | 示例 | 说明 |
|------|------|------|
| `Int` | `42` | 整数 |
| `Double` | `3.14` | 浮点数 |
| `Bool` | `true` / `false` | 布尔 |
| `String` | `"hello"` | 字符串 |
| `Array<T>` | `[1, 2, 3]` | 数组字面量 |
| `T?` | `nil` / `42` | 可空类型 |
| 字典 | `{ "k": "v" }` | 键值对象字面量 |

### 变量

```syx
var count = 0      // 可变
let name = "SYX"   // 不可变
var x: Int = 10    // 显式类型
```

### 控制流

```syx
if condition {
    // ...
} else {
    // ...
}

while x > 0 {
    x = x - 1
}

for item in arrayOrString {
    print(item)
}

switch value {
case 1:
    print("one")
case 0..<10:
    print("small")
default:
    print("other")
}
```

### 函数

```syx
func add(a: Int, b: Int) -> Int {
    return a + b
}

public func exported() -> Int {
    return 1
}

internal func helper() -> Int {
    return 0
}
```

### 字符串

```syx
let s = "Hello, \(name)!"           // 插值
let parts = "a,b,c".split(",")      // 分割
let lower = "ABC".lowercased()      // 小写
let ok = "file.txt".hasPrefix("file")
let sub = "hello".substring(0, 3)
let replaced = "foo".replacingOccurrences("bar", "foo")
let hits = "abc123".matches("\\d+") // 正则匹配，返回数组
let pattern = /\\d+/                // 正则字面量（部分上下文）
```

### 打印

```syx
print("message")
print(value)
```

---

## 模块与依赖

### 导入模块

```syx
import math

func main() {
    let sum = math.add(1, 2)
    print(sum)
}
```

依赖模块需提供 **`public`** 导出函数；`internal` / `private` 符号不能跨模块访问。

**示例模块** `math.sy`：

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {
    return 0
}
```

跨模块访问 `math.secret()` 将在语义分析阶段报错 **S005**。

### 依赖路径解析

- 相对路径：`../math.sy` 指向具体 SYX 文件
- 目录路径：指向含 `syx.toml` 的子项目目录
- 通过 `syx add math ../math.sy` 写入清单并更新 lockfile

### 多模块项目示例

见 [examples/deps/](../../examples/deps/) 与 [examples/deps_math/](../../examples/deps_math/)：

```bash
syx install examples/deps   # 生成 syx.lock
syx run examples/deps         # 输出 42（local_math.add(10, 32)）
```

---

## 标准库 API

标准库通过 **类型静态方法** 与 **实例方法** 暴露，由 `syx-stdlib` 注册表统一管理。

### File

| 方法 | 说明 |
|------|------|
| `File.read(path)` | 读取文件全文 → `String` |
| `File.readLines(path)` | 按行读取 → `Array<String>` |
| `File.write(path, content)` | 写入文件 |
| `File.append(path, content)` | 追加内容 |
| `File.exists(path)` | 文件是否存在 → `Bool` |
| `File.delete(path)` | 删除文件 |
| `File.copy(from, to)` | 复制文件 |
| `File.move(from, to)` | 移动文件 |

```syx
func main() {
  try {
    let n = FileHandle.withOpen(path: "/tmp/x.bin", mode: "w", body: { (h: Int) -> Int in
      return __syx_file_write_chunk(h, [])
    })
    print(n)
  } catch (err) {
    print(err.message)
  }
}
```

### Directory

| 方法 | 说明 |
|------|------|
| `Directory.create(path)` | 创建目录 |
| `Directory.list(path)` | 列出目录内容 → `Array` |
| `Directory.exists(path)` | 目录是否存在 → `Bool` |

### DateTime / Clock / Instant

`import DateTime` 后时钟语义分离（roadmap `M2-S5-01`）：

| API | 语义 |
|-----|------|
| `Clock.nowMs()` / prelude `Time.nowMs()` | **墙上时钟** UNIX epoch 毫秒（可受 NTP 调整影响） |
| `Instant.now()` / `Instant.elapsedMs(start:)` | **单调**进程内 tick（opaque Int；**不是** epoch ms） |
| `Duration.ms/seconds/minutes/hours` | Int-ms 跨度构造（兼容 helper） |
| `DateTime.parse` / `parseIso8601` / `format` / `now` | 窄 Gregorian + RFC3339 UTC/fixed-offset 核心已 **implemented**（M0-B01 + /222） |
| `DateTime.formatWithTimezone(ms, pattern, timezone)` | IANA 墙钟格式化（`M2-S5-02`，chrono-tz）；`pattern` 至少 `"iso"` |
| `DateTime.timezoneOffset(ms, timezone)` | 该时刻相对 UTC 的偏移（**分钟，东正**；如 NY 冬=-300 / 夏=-240）；未知区 **U001** |

```syx
func main() {
  let winter = DateTime.parseIso8601(text: "2024-01-15T12:00:00.000Z").ms
  let summer = DateTime.parseIso8601(text: "2024-07-15T12:00:00.000Z").ms
  print(try DateTime.timezoneOffset(ms: winter, timezone: "America/New_York")) // -300
  print(try DateTime.timezoneOffset(ms: summer, timezone: "America/New_York")) // -240
}
```

计时请用 `Instant`；日历/墙钟请用 `Clock`/`DateTime`。钉测：`m2_s5_01_instant_duration`、`m2_s5_02_iana_tzdb_dst`（IANA/DST）、`m2_s5_03_iso_rfc3339_roundtrip`、`m2_s5_04_retire_unsupported_preview`。**非** leap second / **非** timezone 全量 GA / **非** llvm DateTime=`real` / **非** M2 退出。

### HTTP

`import Net` 后使用 `Http`（非 `HTTP`）：

| 方法 | 说明 |
|------|------|
| `Http.get(url)` | GET → `HTTPResponse`（运输失败可 `try`/`catch`；无 typed throws 迁面） |
| `Http.post(url, body)` | POST → `HTTPResponse`；`body` 可为字符串或字典 |
| `Http.fetch(url, options)` | 通用请求 → `HTTPResponse`（options 含 `method`、`timeout_ms` 等） |
| `Http.request(url, options)` | 同 `fetch` → `HTTPResponse` |
| `resp.status()` / `resp.text()` / `resp.ok()` | 状态码 / 正文 / 2xx |
| `Url.parse(text)` | → `Url`（`href`/`host`/`port`/`path`/`query`） |
| `Http.serve(addr, port, handler)` | 基础 HTTP server（阻塞；`Http.shutdown()` 可 graceful 退出；测试可用 `SYX_HTTP_SERVE_MAX_REQUESTS`） |
| `Http.shutdown` | 停止 accept（hyper+tokio watch cancel）；完成当前请求后 `serve` 返回 |
| `Http.response(status, body)` | 构造 handler 响应 → `HTTPResponse`（`body` 宜为 String） |
| `HttpBodyStream.open/readChunk/close` | 客户端响应 body 分块读会话（；默认 get/fetch 仍缓冲） |


示例：`examples/http_tool/`（ client）；`examples/http_server_tool/`（ server；默认端口 18765）。另见 `examples/httpclient_demo.sy` / `httpserver_demo.sy`。

### Sql（SQLite）

`import Sql` 后使用多连接 facade（`throws SqlError`）：

| 方法 | 说明 |
|------|------|
| `Sql.open(dsn:)` | 打开连接（`":memory:"` 或文件路径）→ `Int` connId |
| `Sql.execOn(connId:, sql:)` | 在指定连接执行 |
| `Sql.queryOn(connId:, sql:)` | 查询 → **`Array` of `SqlRow`**（可用 `.count`） |
| `Sql.close(connId:)` | 关闭显式打开的连接 |

示例：`examples/sql_tool/`（ EC-M2-1 SQLite 服务切片）。存活计数 `sql_conn_live_count` 仅计显式连接（id≥1），在测试/诊断层（ EC-M2-3 首钉），**不是**应用 API。

### String 三视图（`M2-S2-01`）

默认字符模型是 **grapheme（字素簇）** `Character`：

| 视图 | API | 说明 |
|------|-----|------|
| Grapheme | `.count` / `s[i]` / `.substring` / `for-in` | 整数下标为 grapheme offset（见下） |
| Unicode scalar | `String.unicodeScalarCount(text:)` / `String.unicodeScalars(text:)` | 与 grapheme 可分叉（如 ZWJ 家庭 emoji） |
| UTF-8 bytes | `String.utf8Count(text:)` / `String.utf8Bytes(text:)` | 字节长度与 `Array<Int>`（0–255） |


### String 下标复杂度（`M2-S2-02`）

`s[i]` 与 `.substring(start, end)` 使用 **Int grapheme offset**（不是字节偏移，也不是 Swift opaque `String.Index`）。**单次随机访问为 O(n)**（实现经字素簇迭代 `.nth` / `.skip`，无隐藏 O(1) 缓存）。顺序扫描请用 `for-in`（按 grapheme）。钉测：`m2_s2_02_string_index_honesty`。**非** opaque Index GA。

### String normalization / case folding（`M2-S2-03`）

| API | 说明 |
|-----|------|
| `String.normalizedNFC(text:)` | Unicode **NFC** |
| `String.normalizedNFD(text:)` | Unicode **NFD** |
| `String.caseFolded(text:)` | Unicode **default/full case folding**（如 `ß`→`ss`） |
| `.lowercased()` / `.uppercased()` | Unicode **simple case mapping**（≠ `caseFolded`；≠ locale/Turkish） |

### 标识符 Unicode XID（`M2-S2-04`）

标识符遵循 UAX #31：**XID_Start**（或 `_`）起头，**XID_Continue**（或 `_`）接续。希腊/CJK 等合法；emoji 等非 XID_Start 为词法错误。标识符**不做** NFC；**非** UTS #39 confusables / IANA。钉测：`m2_s2_04_unicode_xid_identifiers`。

### String 实例方法

| 方法 | 说明 |
|------|------|
| `.split(sep)` | 分割为数组 |
| `.trim()` | 去首尾空白 |
| `.hasPrefix(s)` | 前缀判断 |
| `.lowercased()` / `.uppercased()` | Unicode simple case mapping（≠ `caseFolded`） |
| `.substring(start, end)` | 子串（Int grapheme offset；单次 O(n)，见 ） |
| `.replacingOccurrences(with, of)` | 替换 |
| `.matches(pattern)` | 正则匹配 |
| `.count` | grapheme 长度 |
| `String.unicodeScalarCount` / `unicodeScalars` | Unicode scalar 视图（静态） |
| `String.utf8Count` / `utf8Bytes` | UTF-8 字节视图（静态） |
| `String.normalizedNFC` / `normalizedNFD` / `caseFolded` | NFC/NFD + full case fold |

### Array 实例方法

| 方法 | 说明 |
|------|------|
| `.count` | 元素个数 |
| `.contains(item)` | 是否包含元素 |
| `.sorted()` | 升序排序（新数组） |
| `.reverse()` | 反转（新数组） |
| `.joined(sep)` | 用分隔符连接为字符串 |
| `.map(f)` / `.filter(f)` / `.reduce(f)` | ✅ 闭包 HOF（`p9_hof_closure.sy`，三后端 parity） |

### 范围与输入

```syx
print((1..<10).contains(5))   // true
let line = readLine()           // 从 stdin 读一行
```

### 迭代器（`M2-S1-01`）

`for x in iterable` 稳定子集支持：

- `Array` 元素顺序遍历
- `String` 按 **grapheme `Character`** 遍历（；≠ Unicode scalar）
- 自定义 `class C: Iterator`（`next() -> Int?`，`nil` 终止）
- `File.readLines()` 返回的数组

**协议声明**（语义检查；稳定子集返回类型为 `Int?`）：

```syx
protocol Iterator {
    func next() -> Int?
}
```

协议必须包含 `func next`，否则报错 **S006**。`Sequence.makeIterator` / 全量 `Collection` Index **仍 partial**。钉测：`m2_s1_01_collection_sequence_iterator`。Deque → `collections.deque`。

### 集合 mutation（`M2-S1-02`）

`var` 上的窄面 mutation 经 HeapSlot write-back 持久化：

| API | 语义 |
|-----|------|
| `d.remove(key)` | `[String:Int]` 真删除；返回 `Bool` |
| `s.insert(x)` / `s.remove(x)` | `Set<Int>`；返回是否新插入 / 是否曾存在 |
| `a[i] = v` / `__syx_array_set_at` | Array 下标与 setAt 写回 |


### Deque（`M2-S1-03`）

`Deque` Int 窄切 MVP（两端 push/pop + count/isEmpty；HeapSlot 写回）：

```syx
var d = Deque.new()
Deque.pushBack(items: d, item: 1)
Deque.pushFront(items: d, item: 0)
print(Deque.popFront(items: d))  // 0
print(Deque.popBack(items: d))   // 1
print(Deque.isEmpty(items: d))   // true；空 pop → nil
```

### 集合 HOF feature-off（`M2-S1-04`）

`Array.map` / `filter` / `reduce`（含 `__syx_array_*` CallHost）在非法 receiver、缺 Closure、或 interpreter feature-off 时 **fail-closed**：诊断码 **`U001` / unsupported**，**不得**以空数组 `[]` 或静默 `Nil` 当作成功。

```syx
// 非法：第二参不是 Closure → 运行失败（U001），不会打印 []
__syx_array_map([1, 2, 3], 0)
```

---

## 错误处理

### try / catch 块

```syx
try {
    File.write("/tmp/test.txt", "data")
    let content = File.read("/tmp/test.txt")
    print(content)
} catch (err) {
    print("failed")
}
```

Host 函数（File、HTTP 等）在失败时抛入 VM 异常机制，由 `catch` 捕获。

### 表达式级 try

```syx
let content = try File.read(path)
// 等价后缀形（与上式同一 AST：UnaryOp::Try）：
let content = File.read(path)?
```

parser 接受前缀 `try expr` 与后缀 `expr?`（同一 `UnaryOp::Try` 节点）。`syx format` 规范形为**后缀** `expr?`；前缀会被改写为后缀。语句级 `try { } catch` 不受影响。失败时按运行时错误处理。

---

## 空安全与 Optional

```syx
let a: Int? = nil
print(a ?? 99)          // nil 合并：左非 nil 取左，否则 99

let b: Int? = 7
print(b!)               // 强制解包；nil 时 trap（R007）

let s: String? = "hello"
print(s?.hasPrefix("he"))  // 可选链：nil 短路为 nil

if let y = optionalValue {
    print(y)
}
```

| 运算符 | 行为 |
|--------|------|
| `??` | Nil 合并 |
| `!` | 强制解包，nil → 运行时错误 |
| `?.` | 可选链式调用 |

---

## 高级语言特性

### Data Class

```syx
@Equatable
@Codable
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    print(p.x)
    print(p.description())  // "Point(x: 3, y: 4)"
    let json = p.encode()
    let back = Point.decode(text: json)
}
```

### Encoder / Decoder 协议（`M2-S7-01`）

语言提供可扩展编码协议面：`Encoder` / `Decoder` / `Encodable` / `Decodable`，以及 `Codable: Encodable, Decodable`。默认仍用 `JSON.encode` / `@Codable` 动态 JSON 路径。钉测：`m2_s7_01_encoder_decoder_protocol`。

### JSONEncoder / JSONDecoder + 字段路径（`M2-S7-02`）

嵌套子集用 `typeName` 静态 API（非 metatype `T.Type`）：

```syx
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  let u = User(name: "a", address: Address(city: "x", zip: "1"))
  let s = JSONEncoder.encode(value: u, typeName: "User")
  let back = try JSONDecoder.decode(text: s, typeName: "User")
  // 缺 required 键抛错，诊断含 codingPath=address（或 address.zip）
}
```

### JSONStream NDJSON（`M2-S7-03`）

可选 streaming MVP：session 写出 NDJSON 行，再分块 `feed` 后 `decodeNext`：

```syx
func main() {
  let w = JSONStream.openWriter()
  try JSONStream.write(handle: w, value: u, typeName: "User")
  let text = JSONStream.finishWriter(handle: w)
  let r = JSONStream.openReader()
  try JSONStream.feed(handle: r, chunk: text)
  let back = try JSONStream.decodeNext(handle: r, typeName: "User")
  // 缺键诊断含 codingPath=line[0].address
  JSONStream.closeReader(handle: r)
}
```

### Extension

```syx
extension Int {
    func doubled() -> Int {
        return self + self
    }
}
```

实例调用 `n.doubled()` 会 desugar 为 `Int_doubled(n)`。

### Sealed Class 与 Switch

```syx
sealed class Result {
    case ok(value: Int)
    case err(message: String)
}

switch value {
case .ok(let x):
    print(x)
case .err(let msg):
    print(msg)
}
```

支持 **范围 case**（`0..<13`）与 **variant 模式**。

### Async / Await

```syx
async func fetch() -> String {
    let data = await HTTP.get("https://example.com")
    return data
}
```

当前为 **MVP**：单 `await` 点、阻塞式 poll。非 async 函数内使用 `await` 会编译失败。

### 效果注解（Effects）

```syx
requires Pure
func compute(x: Int) -> Int {
    return x * 2
}

requires IO
func fetchData() -> String {
    return File.read("data.txt")
}
```

`requires Pure` 函数中不允许 I/O 操作，由 `syx-meta` 在编译期检查。

### 宏与元编程

 由 `syx-macro` 统一展开。详见 [books/language-guide/19-macros.md](../../books/language-guide/19-macros.md) · [](../adr/architecture-decisions.md) · 文档指纹 `macro-docs-v15:m23-service-v14-honesty`。

| 能力 | 说明 |
|------|------|
| 内置 `#assert` / `#for` / `#if` | 编译期展开 |
| 用户 `#macro` | freestanding；**语句位** token subst MVP（`macro-token-v4:stmt-token-subst-mvp`）+ **声明位**参数替换 MVP（`macro-token-v2:decl-param-subst-mvp`）+ **声明位卫生 MVP**（`macro-token-v3:decl-hygiene-mvp`）+ 表达式位 token/ident/literal MVP（`macro-token-v1:expr-token-mvp`）；卫生 v2（`SYX_MACRO_HYGIENE=0` 可关）；**非**完整 token 语法宏 → **134+** |
| `#stringify(expr)` | 表达式位字符串化（Phase 50） |
| `@attached` | member/peer；`body:` 真 AST（Phase 50） |
| `@Equatable` / `@Hashable` / `@Codable` | `data class` L1 合成；L2 associated-enum 部分 GA |
| `#externalMacro` | 外部宏 JSON 协议（Phase 51+） |
| `@propertyWrapper` | Phase 40 GA；`--no-property-wrapper` 关闭 |
| `#compileTime` | 常量求值子集 |
| `reflect(T.self)` | `syx-meta` Rust 侧反射 |
| Service DI v14 | `import Di` → `Di.resolve` / `Di.withRequest` / `Di.setProfile` / …；短糖 `resolve(...)`；`@Qualifier`/`@Primary`/`@Profile`；**协议 witness + 基类型多态 inject**；**`@Around` proceed + `@Before`/`@After` pointcut 字符串绑定 MVP**（`macro-service-v13:before-after-pointcut-bind-mvp`）；**`@Pointcut("Type.method")` 方法名 MVP** + **`execution(* T.m(..))` 名/`*` 模式 MVP** + **`@Pointcut("within(Type)")` 类型名模式 MVP**（`macro-service-v14:pointcut-within-name-mvp`）；**`@Configuration`/`@Bean` MVP**（**仅** `@Service`/`@Component` 实例方法 advice，**非**完整 AspectJ / Spring → **134+**）；`__syx_service_*` 仅内部 |

```syx
import Di

@Service
@Scope("request")
class Worker {
  init() {}
}

func handle() {
  let a = Di.resolve("Worker")
  let b = Di.resolve("Worker")  // 帧内同实例
}

func main() {
  // 无需手写 __syx_service_boot；用户面走 Di 包
  Di.withRequest("handle")
  Di.destroy()
}
```

示例：`examples/service_di/` · `examples/stmt_token_macro/`。内部 host 仍为 `__syx_service_*`（勿在应用代码中直接调用）。Token 宏 / AspectJ AOP / 完整 Spring → **134+**。

### Os 运行时查询（Phase 133–136 · partial GA）

`import Os` 提供与 `Process`（args/env/exec）**分离**的运行时查询 MVP：`Os.cpuCount()`、`Os.memoryTotal()` / `Os.memoryAvailable()` / `Os.uptimeMs()` / `Os.loadAverage()`（throws OsError）、`Os.userName()`、`Os.homeDir()`、`Os.hostname()`、`Os.diskTotal(path)`、`Os.diskAvailable(path)`。已 throws 的 API 无 `-1`/空串成功哨兵。**非**完整 sysinfo / network / cgroup → **138+**。

示例：`examples/os_info/`。

### 并发控制（ · M3 surface · ）

**主业务并发 API：** `Task` / `TaskGroup` / `async`/`await` / `Actor` / `Channel` / `choose`，以及 M3 表面（`taskGroupCancel`、`setDeadlineMs`、`taskLocal*`、`actorReentrancyPolicy`、Channel close/backpressure/fairness、`Mutex`/`Semaphore`）。叙事教程见 [books/language-guide/18-concurrency.md](../../books/language-guide/18-concurrency.md)。

**运行时架构（摘要）：**

| 组件 | 实现 |
|------|------|
| Driver 入口 | `SyxRuntime` — 可配置线程池 + 共享全局（`RwLock`） |
| VM 实例 | 每任务独立 `VmInstance`；通过 runtime bridge 访问共享全局 |
| Actor | 邮箱 + 每 actor 独立执行线程；可用 `Concurrency.setMaxActors` 限流 |
| 标准库 | `packages/core/concurrency.sy` 薄封装 |

**高级运行时配置（部署/运维，非普通应用首选 · ）：** `Concurrency.setWorkerCount` / `setCorePoolSize` / `setMaximumPoolSize` / `setKeepAliveMs` / `setTaskQueueCapacity` / `setTaskRejectPolicy` / `setMaxActors` / `setMailboxCapacity` / Actor shared pool；`blockingSleep` / `setBlockingPoolSize`（276）。`Channel` 默认无界构造；有界用 `Channel.create(n)`（返回 `Channel`，见 `packages/sync`）。**非** Java `ThreadPoolExecutor` 全集兼容；业务优先结构化并发，避免无节制 `detach`。

`syx.toml` 可选 `[concurrency] strict = true` 启用 C001–C003。示例：`examples/concurrency_control/`。

### 运行时 Host 插件（opt-in）

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libplugin.so
syx run app.sy
```

`syx.toml` 可选 `[plugins].paths`；索引锁 `syx-plugin-lock.json`。验签：`SYX_HOST_PLUGIN_VERIFY=1`。

---

## 工具链

### Embed API（Phase 14–15）

Phase 14+ Embed API（`syx_embed_v2_*`）：沙箱 / 超时 / 内存与 RSS 预算 / capability 白名单。详见 [架构决议](../adr/architecture-decisions.md)。

### 格式化 — `syx format`

格式化项目入口文件（`syx.toml` 中 `main` 指定）：

```bash
syx format .
```

覆盖：函数、变量、if/while/for-in、表达式、数组/下标、字典字面量、结构体声明等（AST 路径；unsupported → F001）。

### 静态检查 — `syx lint`

```bash
syx lint .
```

| 代码 | 规则 |
|------|------|
| L001 | 函数名应小写开头 |
| L002 | 重复函数名 |
| L003 | 空函数体 |
| L004 | public 函数应声明返回类型 |
| L005 | async 函数应使用 await |
| L006 | 变量名含非法字符 |
| L007 | 全局变量缺少初始化 |
| L008 | 空 import 名 |
| L009 | 裸 return |
| L010 | 行超过 120 字符 |
| L011 | 含 TODO/FIXME |
| L012 | 建议使用结构化日志而非 print |

### 文档生成 — `syx doc`

```bash
syx doc .
# 输出: docs/api.md
```

从源码中的文档注释生成 Markdown API  stub。

### 调试 — `syx debug`

```bash
# 交互式终端调试
syx debug examples/main.sy

# IDE 图形调试（DAP，VS Code 扩展使用）
syx debug --dap examples/main.sy
```

- **终端模式**：进入 `(debug)` 提示符，可 list/step/run/breakpoint（见 [12_BackendDebuggingGuide.md](./12_BackendDebuggingGuide.zh-CN.md)）
- **DAP 模式**：stdin/stdout 走 `Content-Length` 帧协议，由 `syx_debug::run_dap_server()` 实现；VS Code 需 `launch.json` 中 `"type": "syx"`，详见 [12_BackendDebuggingGuide.md — VS Code 图形调试](./12_BackendDebuggingGuide.zh-CN.md#vs-code-图形调试dap)

### REPL — `syx repl`

```bash
syx repl
syx> print(1 + 2)
syx> let x = 10
```

单行输入、即时编译执行。Ctrl-D 退出。

### 字节码与 IR 导出 — `syx emit`

将编译结果写入文件，用于调试、对比后端或阅读中间表示。不执行程序。

```bash
syx emit examples/hello.sy --format syxbc
syx emit examples/hello.sy --format mir
syx emit examples/hello.sy --format wasm -o hello.wasm
syx emit examples/ --format all -o emit/
```

**`--format` 取值**：

| 格式 | 说明 | 默认输出（单文件） | 默认输出（项目目录） |
|------|------|-------------------|---------------------|
| `syxbc` | VM 字节码文本（与 `syx run` 同路径） | `<stem>.syxbc`（与源文件同目录） | `main.syxbc` |
| `mir` | MIR 文本 | `<stem>.mir` | `main.mir` |
| `wasm` | WASM 二进制（MIR 直编译） | `<stem>.wasm` | `main.wasm` |
| `wat` | WASM 文本（由二进制反印） | `<stem>.wat` | `main.wat` |
| `llvm` | LLVM IR 文本 | `<stem>.ll` | `main.ll` |
| `clif` | Cranelift IR 文本 | `<stem>.clif` | `main.clif` |
| `all` | 以上六种全部写入目录 | — | `emit/` 目录下 `main.*` |

- **单格式**：可用 `-o PATH` 指定输出路径；省略时按上表默认命名。
- **`all`**：`-o` 为输出目录（默认：项目目录或源文件父目录下的 `emit/`）。
- **`syxbc`** 走 CodeGen → VM 指令集；其余格式走统一 MIR 路径（`build_unified_mir`）。
- **v1 为只写**：文本格式暂无官方解析器，仅供人类阅读与 diff。

格式规范见 [SYXBC Text v1](../syxbc-text-v1.md)、[SYX MIR Text v1](../syx-mir-text-v1.md)。`llvm` 与 `syx build --native` 的 IR 发射路径一致。`clif` 仅用于实验 JIT / 历史对比（Cranelift AOT 已移除）。

---

## 编译与后端

SYX 提供 **三种运行/编译方式**，用途不同，不要混用：

| 方式 | 命令 | 产出 | 适用场景 |
|------|------|------|----------|
| **VM（默认）** | `syx run` | 无文件，直接执行 | 日常开发；支持 `print`、File、HTTP、模块等完整 P0 能力 |
| **Native** | `syx build --native` | `app` 可执行文件 | **LLVM sole**；`--object` 仅 `.o`；`full-native` 见上文 |
| **WASM（CLI 本机）** | `syx build --wasm` / `syx run --wasm` | MIR 直编译的标准 `.wasm` | 近完整 stdlib（File/HTTP 等），经 wasmtime + `syx.mir_exec` host；**≠** 浏览器 `compiler.wasm` 子集 |
| **IR 导出** | `syx emit --format …` | `.syxbc` / `.mir` / `.wasm` / `.wat` / `.ll` / `.clif` | 调试与后端对比；不链接、不执行 |

> **注意**：`syx build --wasm` / `syx run --wasm` 将 SYX **MIR 直编译**为标准 WASM：guest 经 host 导入 `syx.mir_exec` 在 `MirRuntime` 上执行（栈、locals、stdlib/File/HTTP 等），能力接近本机 `syx run`。**不再**使用 SYXP 容器 / `syx.run_program`。浏览器内「编一点再跑」走另一条路径（`syx-compiler-wasm` / `examples/wasm_web`），见下文 [浏览器边界](#浏览器边界compilerwasm-子集)，**远小于**本机 CLI WASM。`syx emit --format wasm` 同为 MIR 直编译产物（导出调试用，不链接执行），见 [字节码与 IR 导出](#字节码与-ir-导出--syx-emit)。

### 默认路径（栈 VM）

```bash
syx run program.sy
syx run .                    # 项目目录（读取 syx.toml）
```

流程：`Parser → Semantic → MIR → CodeGen → VmInstance`

### MIR 优化路径（默认，仍走 VM）

Phase 11 起用户模块**默认**走 MIR；`SYX_USE_MIR=0` 可回退 AST 直编（Phase 12 移除）：

```bash
syx run program.sy          # 默认 MIR
SYX_USE_MIR=0 syx run program.sy   # 回退
```

流程：`Parser → Semantic → MIR → optimize → CodeGen → VM`

---

### Native 后端

#### 后端选择

| 后端 | 启用方式 | 说明 |
|------|----------|------|
| **LLVM**（sole） | `syx build --native` | 生产 native 路径；MIR → LLVM → 链接 `syx-rt` 生成 `app`；parity 见 [10_BackendParity.md](./10_BackendParity.zh-CN.md) |

Cranelift **AOT 已移除**。实验 JIT（Suite-1）见 `SYX_JIT_TIER2` / [implementation-status](../../books/shared/implementation-status.md)，与 `--native` 无关。

#### 构建可执行文件（默认）

在项目目录（含 `syx.toml`）或单文件 `.sy` 执行：

```bash
cd examples
syx build --native .
# Built native executable app (llvm)

./app
echo $?
```

默认产物为 **`app`**（项目目录）或与源文件同名的可执行文件（单文件 `.sy` 时输出到 `{项目}/target/debug/{源文件名}`，例如 `guidemo.sy` → `target/debug/guidemo`）。构建过程会生成临时 `.o` 并自动链接 `libsyx_rt.a`，无需手动 `clang` 链接。

**指定输出路径**：`-o` / `--output`（目录构建与单文件构建均可用）：

```bash
# 项目目录 → 自定义可执行文件名
syx build --native -o myapp .

# 单文件 → 自定义路径
syx build --native examples/guidemo.sy -o examples/target/debug/my-gui
```

**避免产物互相覆盖**：`examples/` 项目构建 `main.sy` 得到 `examples/target/debug/app`；若再构建 `examples/guidemo.sy` 而未指定 `-o`，现在会写入 `examples/target/debug/guidemo`（不再覆盖 `app`）。

#### 仅生成目标文件

```bash
syx build --native --object .
syx build --native --object .
# Built native object main.o (llvm)
```

`--object` 时产物为 `.o`（项目目录下 `main.o`，或单文件的 `*.o`）。需系统 `clang`/链接器可用。

#### 依赖与环境

- **clang / 系统链接器**：用于链接 `app` 或编译 LLVM IR 为 `.o`
- **Release 体积**：debug 构建的 `app` 约 30–35MB（含完整 runtime + 调试符号）；`cargo build --release -p syx-cli` 后 `syx build --native` 约 11–12MB；`strip app` 可降至约 **8–9MB**
- **体积 profile（Phase 4 CI）**：`parity-gate` 门禁 `hello --minimal` < 6MB、`examples/main --minimal` < 8MB；可选 `--no-http` / `--no-file` / `--minimal` 进一步缩小 `syx-rt` 链接面
- **`--release`**：`syx build --native --release` 启用 clang `-O2 -DNDEBUG`、release `syx-rt` 链接与 strip 标志；默认 debug 使用 `-O0 -g`。产物分别写入 `target/release/` 与 `target/debug/`。
- 显式 debug：`syx build --native --debug`（与 `--release` 互斥）
- 报告：`syx doctor --native-profile-report` 打印 profile、clang 参数与 rt 档案路径
- **LTO opt-in（Phase 94）**：`SYX_NATIVE_LTO=1 syx build --native --release` 启用 thin-LTO（`-flto=thin` + link `-flto`）

#### 完整示例

```bash
cd examples
syx build --native .
./app
echo "exit code: $?"

syx build --native -o myapp .
./myapp
```

---

### WASM 后端：main.wasm（CLI 本机路径）

此节描述 **CLI** `syx build --wasm` / `syx run --wasm`（wasmtime + MIR host）。浏览器 `compiler.wasm` 见 [浏览器边界](#浏览器边界compilerwasm-子集)。

#### 构建

```bash
cd examples
syx build --wasm .
# Built wasm examples/main.wasm
```

产物为 **wasm-encoder** 生成的标准 WASM 模块，导出 SYX 中的函数名（入口一般为 `main`）。

#### 运行（内置 wasmtime + MIR host）

`syx build --wasm` 将 SYX **MIR 直编译**为标准 WASM：guest 导出 `main` 等函数，每条 MIR 指令通过 host 导入 `syx.mir_exec(op, a, b) -> i32` 在 `MirRuntime` 上执行（栈、locals、stdlib/File/HTTP 等）。**不再**使用 SYXP 容器或 `syx.run_program`。

```bash
# 完整程序（stdlib、变量、插值、import math 等）
syx run --wasm examples/main.sy
syx run --wasm examples/

# 构建 .wasm（运行含跨函数调用时需同时保留 MIR 元数据，见 syx run --wasm 源码路径）
syx build --wasm .
syx run --wasm main.wasm
echo $?                    # main 的 i32 返回值
```

Host 导入：

| 导入 | 签名 | 用途 |
|------|------|------|
| `syx.mir_exec` | `(i32, i32, i32) -> i32` | 执行单条 MIR 操作码（见 `syx-wasm-emit::exec::op`） |

WASM 后端通过 MIR 直编译：`emit_wasm` 对无跳转函数走线性路径，含 `Jump`/`JumpIfFalse`/`JumpIfNil` 的函数走 global `pc` + dispatch loop 的 CFG 路径，与字节码 VM 控制流语义一致。

#### 可选：外部 wasmtime / wasmer

调试或查看模块结构时，仍可使用 [wasmtime CLI](https://wasmtime.dev/) 或 [wasmer](https://wasmer.io/)：

```bash
wasmtime --invoke main main.wasm
wasmer run main.wasm --invoke main
```

#### 查看导出符号

```bash
wasmtime objdump main.wasm
# 或（若已安装 wabt）
wasm-objdump -x main.wasm | grep Export
```

应能看到类似 `Export[main]` 的条目。

#### 完整示例

```bash
mkdir -p /tmp/syx_wasm_demo && cd /tmp/syx_wasm_demo
echo 'func main() { return 42 }' > main.sy
echo -e 'name = "demo"\nmain = "main.sy"' > syx.toml
syx run --wasm .
echo $?                    # 42
```

---

### 三种方式如何选择

```bash
# ✅ 日常开发、演示、集成测试 —— 完整语言 + 标准库
syx run examples/file_tool

# 🔬 验证 MIR → Native 流水线（LLVM）
syx build --native . && ./app; echo $?
syx build --native . && ./app; echo $?

# 🔬 验证 MIR → WASM 流水线（CLI 本机；近完整 host）
syx run --wasm . && echo $?
```

### 浏览器边界：compiler.wasm（子集）

若目标是「在浏览器里编一点 syx 再跑」，用：

- [`crates/syx-compiler-wasm`](../../crates/syx-compiler-wasm)：产出 `compiler.wasm`（**C ABI cdylib**，**无** wasm-bindgen），导出 `syx_compile_source_to_wasm_bytes`（FrontendPipeline → MIR → WASM bytes；注入 `browser_prelude`）以及 guest host 桥 `syx_guest_mir_exec` / `syx_guest_end_fn` / `syx_guest_take_catch_pc` / `syx_guest_get_return`
- [`examples/wasm_web/`](../../examples/wasm_web/)：页面 `fetch('./compiler.wasm')` 后编译用户源码；guest 的 `syx.*` AOT 导入接到编译器内 `MirRuntime`；AOT 导出的 `main` 为 **exit code 0**，页面用 `syx_guest_get_return` 展示 RT 返回值

能力是 **编译子集 + AOT**，**远小于**本机 `syx run` / `syx run --wasm`。**≠** Playwright 产品 GA；**≠** 本机 CLI WASM 能力加深。

**能力地板（诚实；非加深承诺）：** 无项目 / `syx.toml` / 包依赖解析；无完整 File / HTTP / TCP host；**另无** SQL/ORM、本机 TLS/WSS、动态插件、产品 GUI、对等 Task/Actor 调度。`getrandom` 为 `custom` 确定性填充（非浏览器 CSPRNG）。改编译器后须重建并 `cp` `compiler.wasm`；旧产物可能报 `Import "__wbindgen_placeholder__"`（当前路径无 wbindgen）。

与 CI 同源的本地步骤（见 `.github/workflows/ci.yml`）：

```bash
rustup target add wasm32-unknown-unknown
cargo build -p syx-compiler-wasm --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/syx_compiler_wasm.wasm examples/wasm_web/compiler.wasm
# 打开 examples/wasm_web/index.html（Playwright 用 file://；若浏览器拦 fetch，则在该目录起本地 HTTP，如 python3 -m http.server）
```

可选 GUI 冒烟（仅 `tier2w`；**≠** Playwright 产品 GA）：

```bash
cd examples/wasm_web && npm i && npm run test:gui
```

更细说明见 [`examples/wasm_web/README.md`](../../examples/wasm_web/README.md) 与 [`crates/syx-compiler-wasm/README.md`](../../crates/syx-compiler-wasm/README.md)。

### JIT（实验性）

`syx-vm` 提供 `JitCompiler` 热路径计数 stub，用于未来 AOT/JIT 降级策略，当前默认不启用。

---

## 测试

### 项目内测试

将 `.sy` 文件放入 `tests/`（**递归** `tests/**/*.sy`；也支持 `syx test path/to/file.sy`）：

```
my-app/
├── syx.toml
├── main.sy
└── tests/
    ├── case_a.sy
    └── nested/
        └── case_b.sy
```

```bash
syx test .
syx test . --filter case_a
syx test path/to/file.sy
syx test . --format json    # CI：{passed,failed,skipped,cases:[{name,status}]}
syx test . --format junit   # CI：JUnit XML <testsuite name="syx">
syx test . --format tap     # 或 SYX_TEST_TAP=1
```

带 `@Test` 的用例可用 `packages/testing` 断言（**需显式** `import Testing`，不会自动注入；≠ auto-import）：

```sy
import Testing

@Test
func adds() {
    assertEq(1 + 1, 2)
    assert(true)
}

async func inner() {
    __syx_task_sleep(0)
}

@Test
async func async_ok() {
    await inner()
    assert(true)
    assertEq(1, 1)
}
```

每个测试文件独立编译运行；失败非零退出。C-08 窄收官：JSON + JUnit + filter **`[x]`**；已支持递归 discovery、`import Testing` 断言、无 `@Test` 时 `Testing.test` 登记闭包在入口结束前执行（JSON 用例 `{文件}.{name}`）、以及 `@Test async` + 可完成 `await` → JSON pass。钉测：`syx_test_async_await_assert_eq_passes`。教程见 [books/language-guide/34-testing.md](../../books/language-guide/34-testing.md)。**≠** Testing auto-import；**≠** 完整 async test matrix。详见 [plan/topics/syx-test.md](../../plan/topics/syx-test.md)。

### 基准入口（C-09 first-slice · ）

C-09 bench 入口窄 `[x]`（ 文档 + `syx bench` 薄委派 + claimed；**非** ms SLA **非** coverage / fuzz）：

```bash
syx bench --list
syx bench path_join_criterion

# 等价 cargo
cargo bench -p syx-driver --bench path_join_criterion
cargo bench -p syx-driver --bench http_response_criterion
cargo bench -p syx-driver --bench sql_exec_criterion
cargo bench -p syx-driver --bench tcp_connect_criterion
cargo bench -p syx-driver --bench compile_bench
```

PR CI 对四 Criterion 用短采样；nightly 跑 `compile_bench`。C-09 **`[x]`**（窄 bench）；≠ coverage gate / fuzz / property / SLA。详见 [plan/topics/syx-bench.md](../../plan/topics/syx-bench.md)。

### 覆盖率入口（C-09 residual · stub + deepen）

产品 CLI 与等价 cargo（需已装 `cargo-llvm-cov`；窄入口 **claimed** ；**非** CI % 闸 **非** coverage % GA）：

```bash
syx cov

# 等价
cargo llvm-cov -p syx-driver --test native --no-report
```

详见 [plan/topics/syx-coverage.md](../../plan/topics/syx-coverage.md) 与 [docs/lang/c09-syx-coverage-stub-first-slice.md](../lang/c09-syx-coverage-stub-first-slice.md)。

### Fuzz 入口（C-09 residual · CI + / 维护）

产品 CLI 与等价 cargo（`fuzz/` scaffold **已交**；PR CI **有界钉** `-runs=1`； C-09 维护 + 平台维护地板 **已签**；**≠** scaffold GA **≠** 无界 fuzz）：

```bash
syx fuzz

# 等价（需本机 cargo-fuzz）
cargo fuzz run syx_smoke

# 有界 smoke（PR CI 硬门；）
cargo fuzz run syx_smoke -- -runs=1
```

详见 [plan/topics/syx-fuzz.md](../../plan/topics/syx-fuzz.md)、[docs/lang/c09-syx-fuzz-stub-first-slice.md](../lang/c09-syx-fuzz-stub-first-slice.md)、[docs/lang/c09-syx-residual-maintenance-first-slice.md](../lang/c09-syx-residual-maintenance-first-slice.md) 与 [docs/lang/platform-residual-maintenance-first-slice.md](../lang/platform-residual-maintenance-first-slice.md)。

### Property 入口（C-09 residual · CI + / 维护）

产品 CLI 与等价 cargo（`property_smoke` harness **已交**；PR CI **显式钉**； C-09 维护 + 平台维护地板 **已签**；**≠** proptestest GA **≠** `c09_property_ci_ga`）：

```bash
syx prop

# 等价
cargo test -p syx-driver --test property_smoke
```

详见 [plan/topics/syx-property.md](../../plan/topics/syx-property.md)、[docs/lang/c09-syx-property-stub-first-slice.md](../lang/c09-syx-property-stub-first-slice.md)、[docs/lang/c09-syx-residual-maintenance-first-slice.md](../lang/c09-syx-residual-maintenance-first-slice.md) 与 [docs/lang/platform-residual-maintenance-first-slice.md](../lang/platform-residual-maintenance-first-slice.md)。

### Cargo 集成测试

仓库内运行完整测试套件：

```bash
cargo test
cargo test -p syx-driver --test p0_integration
cargo test -p syx-driver --test p1_integration
```

网络相关测试 `p0_http_run` 默认 `#[ignore]`，需手动开启：

```bash
cargo test -p syx-driver p0_http_run -- --ignored
```

---

## 示例项目

| 路径 | 说明 |
|------|------|
| [examples/hello.sy](../../examples/hello.sy) | 最小 Hello World |
| [examples/main.sy](../../examples/main.sy) | 模块 + 字符串 + 数组 + 正则 |
| [examples/file_tool/](../../examples/file_tool/) | EC-M2-1 切片：Path + FileHandle 写读往返 |
| [examples/http_tool/](../../examples/http_tool/) | EC-M2-1 切片：Http.get refuse + loopback |
| [examples/http_server_tool/](../../examples/http_server_tool/) | EC-M2-1 切片：Http.serve + /health |
| [examples/sql_tool/](../../examples/sql_tool/) | EC-M2-1 切片：Sql `:memory:` CRUD |
| [examples/math.sy](../../examples/math.sy) | 可导出模块 |
| [examples/deps/](../../examples/deps/) | 依赖 math 的子项目 |
| [examples/module_main.sy](../../examples/module_main.sy) | 多模块入口示例 |
| [examples/os_info/](../../examples/os_info/) | Os 系统信息（try/catch） |
| [examples/linalg/](../../examples/linalg/) | packed-SYX `Mat`：乘/求解、形状、pad/padReflect（≠ 产品 Mat GA） |
| [examples/log_fields.sy](../../examples/log_fields.sy) | `Log` 级别过滤 + `infoFields` |

**推荐学习顺序**：`hello.sy` → `main.sy` → `file_tool/` → `http_tool/` → `sql_tool/` → `linalg/` → `deps/`

标准库叙事见 [books/library-guide/](../../books/library-guide/00-preface.md)。

---

## 环境变量

| 变量 | 值 | 效果 |
|------|-----|------|
| `SYX_USE_MIR` | `0` 或 `false` | 禁用 MIR，回退 AST 直编（默认开启 MIR） |
| `SYX_JIT_FASTPATH` | `1` | VM tier-0.5：热点函数 MIR fastpath 桩（实验） |
| `SYX_REGISTRY_MIRROR` | URL | 主 registry 不可达时的镜像 URL |
| `SYX_PUBLISHER_SIGNING_KEY` | 64 hex | 发布：Ed25519 32-byte seed；产出 `syx1:ed25519:<pk>:<sig>`（C-05） |
| `SYX_REGISTRY_VERIFY_KEY` | 64 hex | 安装：publisher verifying-key pin（C-02）；失配 `P011` |
| `SYX_HOST_PLUGIN_PATH` | 冒号分隔路径 | 加载 Host 插件 `.so`/`.dylib`（opt-in） |
| `SYX_HOST_PLUGIN_VERIFY` | `1` | 强制插件 ed25519 验签 |
| `SYX_HOST_PLUGINS` | `0` / `disabled` | 禁用插件加载（CI 对照） |
| `SYX_MACRO_HYGIENE` | `0` | 关闭用户 `#macro` 卫生 v2（临时） |
| `SYX_MACRO_TRACE` | `1` | 打印宏展开 trace（调试） |
| `SYX_EC_M2_1_HTTP_BASE` | URL origin | `examples/http_tool` 正例基址（如 `http://127.0.0.1:PORT`） |
| `SYX_HTTP_SERVE_MAX_REQUESTS` | 正整数 N | `Http.serve` 响应 N 次后返回（测试/运维钩；≠ `Http.shutdown` graceful；未设则需 shutdown 或进程退出） |

---

## 诊断代码

### 语义分析（S 系列）

| 代码 | 含义 |
|------|------|
| S001 | 符号重复定义 |
| S002 | 未定义标识符 |
| S003 | 对不可变变量赋值 |
| S004 | 类型不匹配 |
| S005 | 跨模块访问 internal 符号 |
| S006 | Protocol 缺少 `next()` |
| S024 | 集合 hosted op 禁止 legacy MethodCall |
| S025 | hosted 方法须 extension/CallHost |

### Host 插件（H 系列）

| 代码 | 含义 |
|------|------|
| H001 | 插件未加载或 Host 缺失 |
| H002 | 插件 manifest/锁与运行期不一致 |

### 宏（E 系列）

| 代码 | 含义 |
|------|------|
| E002 | 宏展开深度超限或循环 |

### 运行时（R 系列）

| 代码 | 含义 |
|------|------|
| R007 | 对 nil 强制解包（`!`） |
| R008 | `readLine` 读取 stdin 失败 |

### Lint（L 系列）

见 [工具链 — syx lint](#静态检查--syx-lint)。

---

## 常见问题

### `syx: command not found`

尚未安装 CLI。使用 `cargo run -p syx-cli --` 前缀，或执行 `cargo install --path crates/syx-cli`。

### 模块找不到

1. 确认 `syx.toml` 中 `dependencies` 配置正确
2. 运行 `syx install .` 生成/更新 `syx.lock`（缺失 lock 报 **P001**）
3. 依赖模块中的函数需标记 `public`

### HTTP 测试失败

网络环境不可达时，使用 compile-only 测试或忽略网络用例。本地开发可改用 mock 或跳过 HTTP 段。

### 编译缓存异常

```bash
rm -rf target/cache
syx check .
```

### 如何查看字节码或中间表示？

使用 `syx emit`（见 [字节码与 IR 导出](#字节码与-ir-导出--syx-emit)）：

```bash
syx emit examples/hello.sy --format syxbc   # VM 指令
syx emit examples/hello.sy --format mir       # MIR
syx emit examples/hello.sy --format llvm      # LLVM IR
syx emit examples/ --format all -o emit/     # 六种格式一次导出
```

### `--native` 如何运行？

默认生成 **`app` 可执行文件**，直接运行即可：

```bash
syx build --native .
./app
echo $?                      # 退出码
```

仅目标文件时使用 `--object`：

```bash
syx build --native --object .
file main.o                  # 确认为 object file
```

详见 [Native 后端](#native-后端)。

### Native 是否还支持 Cranelift？

**不支持 AOT。** `syx build --native` **仅**走 LLVM。历史上的 Cranelift AOT / `--backend=cranelift` 已移除。

实验性 **JIT tier-2**（`syx-cranelift-jit`，`SYX_JIT_TIER2=1`）是另一条 opt-in 路径，默认关闭，**不等于** `syx build --native`。


### `--wasm` 产出 `main.wasm`，如何运行？

使用内置运行器（推荐）：

```bash
syx run --wasm .              # 编译项目并运行
syx run --wasm main.wasm      # 运行已有 .wasm
echo $?                       # 查看返回值
```

也可选用外部 wasmtime：`wasmtime --invoke main main.wasm`。

详见 [WASM 后端：main.wasm（CLI 本机路径）](#wasm-后端mainwasmcli-本机路径)。浏览器内编译子集见 [浏览器边界](#浏览器边界compilerwasm-子集)（≠ 本 FAQ 的 CLI `main.wasm`）。

### `--native` 未生成可执行文件

默认应生成 **`app`**。若只有 `.o`，说明使用了 **`--object`**。若链接失败，请确认已安装 **clang** 且 `libsyx_rt.a` 可用（见 [plan/topics/profiles-sdk.md](../../plan/topics/profiles-sdk.md)）。

无源码树时可用环境变量指定运行时归档（C-11 窄 `[x]` 契约；≠ Release 全矩阵 CI 上传）：

- **`SYX_RT_STATIC_LIB`**：指向已有的 `libsyx_rt.a` / `syx_rt.lib`（或含 hashed archive 的 deps 目录）
- **`SYX_RT_PROFILE=<id>`**：按 SDK 安装布局查找 `bin/../lib/profiles/<id>/libsyx_rt.a`（回退 `../lib/<id>/`）；约定见 [`packaging/syx-sdk/README.md`](../../packaging/syx-sdk/README.md)

二者优先于编译期 embed 与 cargo `target/` 搜索。
### 与 Swift 的差异

SYX 语法相似但并非 Swift 超集。未实现或仅为 MVP 的特性包括：完整并发模型、泛型约束、属性包装、完整宏系统等。详见 [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.zh-CN.md)。

---

## 相关文档

| 文档 | 内容 |
|------|------|
| [8_QuickReference.md](./8_QuickReference.zh-CN.md) | 语法速查 |
| [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.zh-CN.md) | P0–P3 功能清单 |
| [5_CompleteTechnicalDocumentation.md](./5_CompleteTechnicalDocumentation.zh-CN.md) | 编译器架构 |
| [4_InstructionSet.md](./4_InstructionSet.zh-CN.md) | VM 指令集 |

---

**维护者:** SYX Core Team  
**最后更新:** 2026-06-13  
**版本:** v0.1
