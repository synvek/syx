# SYX 完整使用指南

> **Phase 139（SYX 117.0）：** Truth M0–M4 口径下的语言/工具链；SIMD16 guest-v22；SynUI deferred；Farm/Dist frozen v19；audit **2600/3050**；`syx-cache-v1`；原生 AOT **仅 LLVM**；许可证 [Apache-2.0](../LICENSE)。  
> 详见 [architecture-decisions.md](adr/architecture-decisions.md) 与 [implementation-status](../books/shared/implementation-status.md)。



本文档面向 **SYX 语言用户与工具链使用者**，涵盖安装、CLI、项目配置、语言特性、标准库、工具链与进阶编译选项。技术实现细节请参阅 [docs/README.md](./README.md) 中的编译器文档。

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
| clang / LLVM | Optional：`--features full-native` 时用 sibling `../llvm` 或 `SYX_LLVM_PREFIX`（inkwell 直出 `.o` + **静态嵌入 LLD**）；见 [plan/topics/native-llvm.md](../plan/topics/native-llvm.md) |

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

Core `Math` 增强轨（Soft / Special / F32·F64 实例，）**收官→维护签**（；HostFn **629**）。 Math llvm：**84/84** + 窄面 GA； QS1 Math **84/84 + narrow `qs1_math_ga=true`**； Math status **84/84 `status=implemented`**； Fmt status 试点 **FmtPrint/FmtPrintLine×2**； Path status 试点 **PathJoin/Normalize/Exists/Extension×4**； 中间 first-slice meta F-10 收缩 **×33**； File status 试点 **FileOpen/Close/Read/Write×4**； File rest **×11 / 全量 File\* `status=implemented`**； Http rest **×16 / 全量 Http\* `status=implemented`**； Sql core **×4 `status=implemented`**； Sql rest **×23 / 全量 Sql\* `status=implemented`**； Tcp **×7 / 全量 Tcp\* `status=implemented`**； Task **×8 / 全量 Task\* `status=implemented`**； Time/DateTime **×8 `status=implemented`**； Json **×11 / 全量 Json\* `status=implemented`**； Str/Unicode **×11 `status=implemented`**； Process/Child wasm-unsupported honesty **×14**； Log core ×4 mir+llvm **real**； Log rest ×8 / 全族 ×12 mir+l … ≠ 全量 HostFn status；仍 14 partial）。C-09 / 平台 / Math 维护链于 **收官**。详见 [docs/lang/qs1-d01-execute-adr-first-slice.md](lang/qs1-d01-execute-adr-first-slice.md)。

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

见 [examples/deps/](../examples/deps/) 与 [examples/deps_math/](../examples/deps_math/)：

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

`FileHandle`（`import IO`）提供带 `throws IOError` 的 open/read/write/close/seek/readAllBytes。路径拼装用 prelude `Path.join` / `normalize` / `exists` / `fileExtension`（OS 分隔符契约： / `ec_m2_1_path_contract`；**非** Path GA）。Path/File HostFn availability 已在能力矩阵诚实回写；native/WASM 列诚实加深已钉（`ec_m2_4_path_file_native_wasm_matrix`）；PathJoin llvm GA 首档已钉（`ec_m2_4_path_file_native_wasm_ga`；`PathJoin` `llvm=real` + native AOT 执行证据）；PathExtension llvm GA 二档已钉（`ec_m2_4_path_file_native_wasm_ga2`；`PathExtension` `llvm=real` + native AOT `"a/b.txt"` → `"txt"`）；PathNormalize llvm GA 三档已钉（`ec_m2_4_path_file_native_wasm_ga3`；`PathNormalize` `llvm=real` + native AOT `"a/b/../c"` → `"a/c"`；PathExists/File* 仍 `llvm=partial` / `wasm=host-assisted` / 产品 `native=partial`；**非** 全量 Path/File GA **非** wasm GA **非** EC-M2-4 全量退出）。`Path.join`×N Instant 微基准在测试层（`ec_m2_4_path_file_microbench`；**非** perf SLA **非** EC-M2-4 全量退出）。`Path.join`×N Criterion 正式 benchmark 首档在 `cargo bench`（`ec_m2_4_path_file_criterion`；**非** absolute SLA gate **非** EC-M2-4 全量退出）。示例：`examples/file_tool/`（ EC-M2-1 切片）。存活计数 `file_handle_live_count` 仅在测试/诊断层（ EC-M2-3 首钉），**不是**应用 API。

**EC-M2-3-Z：** post-Sx 四类生产路径（File `withOpen` / Sql `poolClose` / ProcessBuilder+`Child.wait` / Tcp host-release）live_count 回基线钉已签；**非** EC-M2-3 总勾 **非** Q-S5。钉测：`ec_m2_3_z_closeout`。

**EC-M2-4-Z：** A–H 八轴加深轨（availability/µbench/Criterion/matrix/GA1–3 closeout/Tcp GA4）证据链收口已签；**非** absolute SLA **非** 产品 `native=real`/`wasm=real` **非** 全量 HostFn GA。钉测：`ec_m2_4_z_closeout`。

**EC-M2-4 总勾：** 加深轨退出口径 A–H + Z 已签（诚实收窄）；四产品切片仍 `native=partial` / `wasm=host-assisted`；**非** absolute SLA **非** backends=`real`。钉测：`ec_m2_4_total_closeout`。

**M2 退出：** 四 EC 总勾已签前提下的 thin exit；开放债（SLA / backends=`real` / 全量 HostFn GA / Path GA / TLS server·accept）**不挡退出**。钉测：`m2_exit_closeout`。

**M3-A01 task tree 首钉：** `Concurrency.taskGroupCancel` 取消组内子任务；`taskGroupWait` 在 cancel 后及时返回；子 `join` 表面 **C004**；**非** silent-leak GA **非** EC-M3-* **非** deadline。钉测：`m3_a01_task_tree`。

**M3-A01 silent-leak 加深：** `Concurrency.liveTaskCount` / `__syx_task_live_count`；结构化 `taskGroupWait`（含 cancel+wait）后 live_count 回 baseline；未入组 unreaped detach 可观测 `LEAKED`；**非** EC-M3-2 **非** A01 叶子满勾（本窗）**非** deadline。钉测：`m3_a01_silent_leak`。

**M3-A01 leaf thin closeout：** task tree + silent-leak 双轴证据链收口；roadmap `M3-A01` `[x]`；**非** EC-M3-2 **非** 全量 structured-concurrency GA **非** A02–A03 **非** M3 退出。钉测：`m3_a01_closeout`。

**M3-A02 deadline 首钉：** `Concurrency.setDeadlineMs`；子任务继承；到期协作取消；长 sleep 及时返回；join **C004**；**非** TaskLocal GA **非** EC-M3 **非** A02 满勾。钉测：`m3_a02_deadline`。

**M3-A02 task-local 加深：** `Concurrency.taskLocalSet` / `taskLocalGet`；子任务克隆继承；子写不回写父；**非** `@TaskLocal` **非** EC-M3 **非** A02 满勾（本窗）。钉测：`m3_a02_task_local`。

**M3-A02 leaf thin closeout：** deadline + task-local 双轴证据链收口；roadmap `M3-A02` `[x]`；**非** EC-M3 **非** `@TaskLocal` GA **非** A03 **非** M3 退出。钉测：`m3_a02_closeout`。

**M3-A03 actor reentrancy 首钉：** `Concurrency.actorReentrancyPolicy`；同实例同步重入 → **C009**；**非** Swift await-reentrancy GA **非** EC-M3 **非** A03 满勾。钉测：`m3_a03_reentrancy`。

**M3-A03 await-hold 加深：** policy → `serial-no-reenter;await-hold`；handler 内 sleep/await 期间无 mid-await mailbox 插队；**非** Swift await-reentrancy GA **非** EC-M3 **非** A03 满勾（本窗）。钉测：`m3_a03_await_reentrancy`。

**M3-A03 leaf thin closeout：** reentrancy + await-hold 双轴证据链收口；roadmap `M3-A03` `[x]`；**非** EC-M3 **非** Swift await-reentrancy GA **非** B01 **非** M3 退出。钉测：`m3_a03_closeout`。

**M3-B01 isolation 薄首钉：** 独立 `blocking_pool`；`Concurrency.blockingSleep` / `isolationPolicy`；Task 池 size=1 时长阻塞不得饿死短 Task；**未**改道 `__syx_task_sleep` / HTTP/File；**非** epoll reactor **非** B02 **非** EC-M3 **非** B01 叶子 `[x]`。钉测：`m3_b01_isolation`。

**M3-B01 isolation 加深：** `Concurrency.setBlockingPoolSize` / `blockingPoolSize`；指纹 `task-pool+blocking-pool-v1-sized`；size=2 并行 `blockingSleep` + size=1 排队不得饿死短 Task；**非** B01 叶子 `[x]`（当时） **非** B02 **非** EC-M3 **非** reactor GA。钉测：`m3_b01_pool_sizing`。

**M3-B01 leaf thin closeout：** isolation + sizing 双轴证据链收口；roadmap `M3-B01` `[x]`；**非** EC-M3 **非** epoll reactor GA **非** B02 **非** M3 退出。钉测：`m3_b01_closeout`。

**M3-B02 async adapter 薄首钉：** `File.readAsync` / `__syx_file_read_async` + `HttpRequestAsync` 改道 `blocking_pool`；指纹原 `v1-thin`（加深后升档见 ）；**非** epoll reactor GA **非** 同步 File·Http 改道 **非** EC-M3。钉测：`m3_b02_async_adapter`。

**M3-B02 async adapter 加深：** `TcpStream.readAsync` / `__syx_tcp_read_async` 走 `blocking_pool`；指纹 `blocking-pool-async-adapter-v1-tcp`；**非** TcpWrite/Connect async **非** accept GA **非** epoll reactor GA **非** HttpRequestAsync capability TOML **非** EC-M3。钉测：`m3_b02_tcp_async`。

**M3-B02 leaf thin closeout：** File/HTTP thin + Tcp deepen 双轴证据链收口；roadmap `M3-B02` `[x]`；**非** EC-M3 **非** epoll reactor GA **非** TcpWrite·Connect async GA **非** M3 退出。钉测：`m3_b02_closeout`。

**M3-C01 Mutex RAII 薄首钉：** `Mutex.lockGuard` → `MutexGuard`（`deinit` quiet unlock）；指纹原 `v1-thin`（加深后升档见 ）；**非** C02 **非** EC-M3。钉测：`m3_c01_mutex_raii`。

**M3-C01 Mutex blocking 加深：** `__syx_mutex_lock_wait`（held 时 Condvar wait）；产品 `Mutex.lock` / `lockGuard` 改道 wait；指纹 `mutex-raii-guard-v1-blocking`；跨 Task 争用证据；try-lock HostFn 保留；**非** C02 **非** EC-M3。钉测：`m3_c01_mutex_blocking`。

**M3-C01 leaf thin closeout：** RAII thin + blocking deepen 双轴证据链收口；roadmap `M3-C01` `[x]`；**非** C02 **非** EC-M3 **非** M3 退出。钉测：`m3_c01_closeout`。

**M3-C02 Semaphore 薄首钉：** `Semaphore.create/wait/tryWait/signal`；指纹曾为 `semaphore-v1-thin`；钉测：`m3_c02_semaphore`。

**M3-C02 SemaphoreGuard RAII 加深：** `SemaphoreGuard` + `Semaphore.waitGuard`；指纹 `semaphore-v1-raii`；钉测：`m3_c02_semaphore_raii`。

**M3-C02 叶子薄收口：** 双轴证据链（thin + RAII）→ 叶子 `[x]`（Semaphore 切片）；**非** RWLock **非** Condvar 产品 **非** EC-M3 **非** C03。钉测：`m3_c02_closeout`。

**M3-C03 内存序 / Atomic 文档首钉：** 跨 Task 共享可变 → `Mutex` / `Semaphore` / `Channel` 建立 happens-before；**无**产品 `Atomic*` / `memory_order` 枚举；Wasm atomics **unsupported**；race/stress → C04。钉测：`m3_c03_memory_ordering`。

**M3-C03 叶子薄收口：** docs 证据链收口 → 叶子 `[x]`（内存序/Atomic 诚实文档切片）；**非**产品 Atomic API **非** Wasm atomics 满勾 **非** C04 **非** EC-M3。钉测：`m3_c03_closeout`。

**M3-C04 race/stress CI 抽样首钉：** 机读样例库 `race_stress_samples.toml` + 跨 Task Mutex 抽样冒烟；钉测：`m3_c04_race_stress`。

**M3-C04 chaos 抽样加深：** TaskGroup cancel 中断长 sleep → C004；样例库 `kind=chaos`；钉测：`m3_c04_chaos`。

**M3-C04 叶子收口：** stress + chaos 双轴证据链 → `[x]`；**非** TSAN **非** 杀进程·网络分区 chaos GA **非** EC-M3。钉测：`m3_c04_closeout`。

**M3-D01 structured logging 薄首钉：** `Log.setMinLevel` / `infoFields` / 指纹 `log-v1-thin`；钉测：`m3_d01_structured_logging`。

**M3-D01 全 level `*Fields` 加深：** `debugFields` / `warnFields` / `errorFields`；指纹 `log-v1-fields`；钉测：`m3_d01_log_fields`。

**M3-D01 叶子收口：** thin + fields 双轴证据链 → `[x]`；指纹 `log-v1-fields`；**非** JSON GA **非** D02–D04 **非** EC-M3。钉测：`m3_d01_closeout`。

**M3-D02 Trace/Span 薄首钉：** `Trace.start` / `end` / `current` / `contextLine`（含 `task_id`/`actor_id`）/ 指纹 `trace-v1-thin`；**非** OTel/W3C 传播 GA **非** D02 `[x]` **非** D03–D04 **非** EC-M3。钉测：`m3_d02_trace_span`。

**M3-D02 nested + Log context 加深：** nested LIFO；`Log.*Fields` 自动注入 `span`/`task_id`/`actor_id`；指纹 `trace-v1-logctx`；**非** OTel GA **非** 跨 Task span 继承 **非** D02 `[x]` **非** EC-M3。钉测：`m3_d02_trace_logctx`。

**M3-D02 叶子收口：** thin + logctx 双轴证据链 → `[x]`；指纹 `trace-v1-logctx`；**非** OTel GA **非** 跨 Task 继承 **非** D03–D04 **非** EC-M3。钉测：`m3_d02_closeout`。

**M3-D03 Metrics Counter 薄首钉：** `Metrics.inc` / `get` / `reset` / `snapshot` / 指纹 `metrics-v1-thin`；**非** OTel/Prometheus export **非** Gauge/Histogram/labels **非** D03 `[x]` **非** D04 **非** EC-M3。钉测：`m3_d03_metrics_counter`。

**M3-D03 Gauge + labeled counter 加深：** `Metrics.gaugeSet` / `gaugeGet` / `incLabels` / `getLabels`；snapshot 含 `g:name=v` 与 `name{k=v}=n`；指纹 `metrics-v1-gauge-labels`；**非** OTel/Prometheus export **非** Histogram **非** D04 **非** EC-M3。钉测：`m3_d03_metrics_gauge_labels`。

**M3-D03 叶子薄收口：** thin + deepen 双轴证据链 → `M3-D03` `[x]`；指纹保持 `metrics-v1-gauge-labels`；**非** OTel/Prometheus export **非** Histogram **非** D04 **非** EC-M3。钉测：`m3_d03_closeout`。

**M3-D04 Profiler 区段墙钟薄首钉：** `Profiler.start` / `stop` / `elapsed` / `reset` / `snapshot` / 指纹 `profiler-v1-thin`；**非** 采样 GA **非** CPU-time GA **非** pprof export **非** D04 `[x]` **非** EC-M3。钉测：`m3_d04_profiler_sections`。

**M3-D04 进程 CPU 区段加深：** `Profiler.cpuStart` / `cpuStop` / `cpuElapsed`；snapshot 含 `c:name=ms`；指纹 `profiler-v1-cpu`；**非** 采样 GA **非** pprof export **非** EC-M3。钉测：`m3_d04_profiler_cpu`。

**M3-D04 叶子薄收口：** thin + deepen 双轴证据链 → `M3-D04` `[x]`；指纹保持 `profiler-v1-cpu`；**非** 采样 GA **非** pprof export **非** EC-M3。钉测：`m3_d04_closeout`。

**EC-M3-4 Observability 证据链薄首钉：** D01–D04 四叶 closeout + 指纹证据链（`log-v1-fields` / `trace-v1-logctx` / `metrics-v1-gauge-labels` / `profiler-v1-cpu`）；roadmap 脚注 thin 已交；**非** 采样 **非** pprof **非** OTel **非** M3 退出。钉测：`ec_m3_4_observability`。

**EC-M3-4 叶子薄收口：** thin 证据链 → `EC-M3-4` 总勾 `[x]`（诚实收窄）；**非** EC-M3-1～3 **非** 采样 **非** pprof **非** OTel **非** M3 退出。钉测：`ec_m3_4_closeout`。

**EC-M3-2 Silent-leak 证据链薄首钉：** A01 closeout + task-tree/silent-leak 双轴 + `liveTaskCount` / `TaskLiveCount`；roadmap 脚注 thin 已交；**非** async-let GA **非** 非 group 回收完备 **非** EC-M3-1/3 **非** M3 退出。钉测：`ec_m3_2_silent_leak`。

**EC-M3-2 叶子薄收口：** thin 证据链 → `EC-M3-2` 总勾 `[x]`（诚实收窄）；**非** async-let GA **非** 非 group 回收完备 **非** EC-M3-1/3 **非** M3 退出。钉测：`ec_m3_2_closeout`。

**M3-E01 权限模型文档薄首钉：** 五域 default-deny 意图（`docs/capability-sandbox-policy-v0.md`）；指纹 `permission-policy-v0:default-deny-intent`；roadmap `M3-E01` `[~]`；**非** 运行时强制 **非** CLI grant **非** HostFn 闸门 **非** EC-M3-3 **非** M3 退出。钉测：`m3_e01_permission_model`。

**M3-E01 叶子薄收口：** 策略文档证据链收口；roadmap `M3-E01` `[x]`；**非** 运行时强制 **非** E02–E03 **非** EC-M3-3 **非** M3 退出。钉测：`m3_e01_closeout`。

**M3-E02 权限授予薄首钉：** opt-in `--permission-sandbox` + `--allow-net` / `SYX_ALLOW_NET`；指纹 `permission-grants-v0:allow-net-cli`；未授权 net HostFn → `S024`（W52 起；曾用 `S023`）；roadmap `M3-E02` `[~]`；**非** 五域 GA **非** E03 **非** EC-M3-3 **非** M3 退出。钉测：`m3_e02_permission_grants`。

**M3-E02 权限授予加深：** `--allow-fs` / `SYX_ALLOW_FS` + `syx.toml` `[permissions]`；指纹 `permission-grants-v0:allow-net-fs-toml`；`FileRead`/`FileWrite` 未授权 → `S024`；roadmap `M3-E02` 仍 `[~]`；**非** 五域 GA **非** path allowlist **非** E03 **非** EC-M3-3。钉测：`m3_e02_permission_fs`。

** D-04 env+process 薄闸：** opt-in `--allow-env` / `--allow-process`；未授权 `EnvGet`/`EnvSet`/`ProcessExec`/`ProcessSpawn` → `S024`；**非** 五域 GA **非** default-on **非** plugin 闸 **非** D-04 `[x]`。钉测：`qs1_d04_permission_env` / `qs1_d04_permission_process`。

** D-04 env+process 薄闸：** opt-in `--allow-env` / `--allow-process`；未授权 `EnvGet`/`EnvSet`/`ProcessExec`/`ProcessSpawn` → `S024`；**非** 五域 GA **非** default-on **非** plugin 闸 **非** D-04 `[x]`。钉测：`qs1_d04_permission_env` / `qs1_d04_permission_process`。

** D-04 plugin 薄闸：** opt-in `--allow-plugin`；未授权 `__syx_plugin_*` → `S024`；**非** 五域 GA **非** default-on **非** plugin-id allowlist **非** D-04 `[x]`。钉测：`qs1_d04_permission_plugin`。

** HostFn 中间 meta F-10：** 删施工链中间 first-slice meta ×21；保留 status 收口 / Process honesty / D-04/F-08 活地板；**非** D-04 `[x]` **非** F-08 `[x]` **非** 全量 HostFn status。钉测：`qs1_hostfn_intermediate_meta_f10_first_slice`。

** D-04 default-on 产品决策 ADR：** 书面决策 = **保持 opt-in**；sandbox **不是** default-on；default-on **延至**显式执行窗；**非** D-04 `[x]` **非** 五域 GA **非** sandbox 默认开。钉测：`qs1_d04_default_on_adr_first_slice`。

** D-04 opt-in 书面缩小域：** D-04 **`[x]`** = opt-in + 五域薄闸齐；**非** sandbox 默认开 **非** 五域 default-on GA **非** default-on 执行。钉测：`qs1_d04_optin_soft_close_first_slice`。

** D-01 产品执行 ADR：** D-01 **产品执行 ADR met**；行 **仍冻**；**非** load-farm GA **非** 压测脚手架落地 **非** D-01 `[x]`。钉测：`qs1_d01_execute_adr_first_slice`。

** D-01 脚手架：** 既有 HTTP Criterion 钉成可复现报告薄入口；行 **仍冻**；**非** load-farm GA **非** 新 Criterion **非** D-01 `[x]`。复现：`cargo bench -p syx-driver --bench http_response_criterion -- --sample-size 10 --warm-up-time 1 --measurement-time 2`。钉测：`qs1_d01_scaffold_first_slice`。

** D-01 加深：** 既有 path/sql/tcp Criterion 钉成可复现报告；行 **仍冻**；**非** load-farm GA **非** actor Criterion **非** D-01 `[x]`。复现：`cargo bench -p syx-driver --bench path_join_criterion` / `--bench sql_exec_criterion` / `--bench tcp_connect_criterion`（同样 `--sample-size 10 --warm-up-time 1 --measurement-time 2`）。钉测：`qs1_d01_deepen_first_slice`。

** D-01 load-farm GA charter：** 书面冻结未来诚实宣称 load-farm GA 的 in-scope/OUT（HTTP + path/sql/tcp Criterion 可复现报告链）；行 **仍冻**；**非** load-farm GA **非** actor Criterion **非** D-01 `[x]`。钉测：`qs1_d01_load_farm_ga_charter_first_slice`。

** D-01 load-farm GA 宣称：** charter in-scope 完成面宣称 + soft-close；`| D-01 |` **`[x]`**；本窗 inventory `d01_load_farm_ga_not_claimed=false`；**非** actor Criterion **非** 绝对 SLA **非** 默认 CI 硬闸。钉测：`qs1_d01_load_farm_ga_claim_first_slice`。

** D-05/D-06 产品执行 ADR：** 书面冻结 Actor 共享池 + DiscardOldest 产品完成面（同 ADR）；行 **仍冻**；**非** Actor pool GA **非** DiscardOldest 产品 GA **非** D-05·06 `[x]`。钉测：`qs1_d05_d06_execute_adr_first_slice`。

** D-05/D-06 施工：** 钉既有 Actor 共享池 MVP + DiscardOldest MVP 薄入口；行 **仍冻**；**非** Actor pool GA **非** DiscardOldest 产品 GA **非** D-05·06 `[x]`。钉测：`qs1_d05_d06_construction_first_slice`。

** D-05/D-06 加深：** 钉 Actor 共享池 race-census + DiscardOldest 策略加深 SSOT；行 **仍冻**；**非** Actor pool GA **非** DiscardOldest 产品 GA **非** soft-close **非** D-05·06 `[x]`。钉测：`qs1_d05_d06_deepen_first_slice`。

** D-05/D-06 收窄 soft-close：** deepen SSOT 完成面翻行；`| D-05 |` / `| D-06 |` **`[x]`**；Wave D **6/6**；**仍** 伞 Actor pool / DiscardOldest 产品 GA **未宣称**。钉测：`qs1_d05_d06_soft_close_first_slice`。

** D-02 产品执行 ADR：** D-02 **产品执行 ADR met**；行 **仍冻**；**非** TSAN GA **非** loom GA **非** 已知问题清单落地 **非** D-02 `[x]`。钉测：`qs1_d02_execute_adr_first_slice`。

** D-02 已知问题清单：** 既有 M3-C04 corpus 钉成清单表体；行 **仍冻**；**非** 清单完成 **非** TSAN GA **非** loom GA **非** D-02 `[x]`。钉测：`qs1_d02_known_issues_first_slice`。

** D-02 已知问题清单加深：** 书面冻结仍开残差分类（TSAN CI / loom / kill·网络 chaos / wasm atomics / Actor 共享池竞态）；行 **仍冻**；**非** 清单完成 **非** TSAN CI **非** TSAN GA **非** D-02 `[x]`。钉测：`qs1_d02_known_issues_deepen_first_slice`。

** D-02 TSAN CI 采样：** nightly/opt-in `tsan-sample.yml`（`-Zsanitizer=thread` + 窄 race_stress mutex smoke）；行 **仍冻**；**非** TSAN GA **非** 默认 PR CI 硬闸 **非** 清单完成 **非** D-02 `[x]`。钉测：`qs1_d02_tsan_ci_sampling_first_slice`。

** D-02 已知问题清单 complete：** 残差 gap disposition→`deferred`（无 `open`）；行 **仍冻**；**非** TSAN GA **非** harden 完成 **非** D-02 `[x]`。钉测：`qs1_d02_known_issues_complete_first_slice`。

** D-02 TSAN sample harden：** `tsan-sample.yml` 滤 `m3_c04_race_stress` 前缀 + fail-closed；`tsan_ci=sampling_hardened`；行 **仍冻**；**非** TSAN GA **非** 默认 PR CI 硬闸 **非** D-02 `[x]`。钉测：`qs1_d02_tsan_harden_first_slice`。

** D-02 TSAN GA charter：** 书面冻结未来诚实宣称 TSAN GA 的 in-scope/OUT（`tsan-sample.yml` + race_stress fail-closed + list complete + `sampling_hardened`）；行 **仍冻**；**非** TSAN GA **非** 默认 PR CI 硬闸 **非** D-02 `[x]`。钉测：`qs1_d02_tsan_ga_charter_first_slice`。

** D-02 TSAN GA 宣称：** charter in-scope 完成面宣称 + soft-close；`| D-02 |` **`[x]`**；本窗 inventory `d02_tsan_ga_not_claimed=false`；**非** 默认 PR CI 硬闸 **非** loom GA **非** 全量 stress。钉测：`qs1_d02_tsan_ga_claim_first_slice`。

** D-03 产品执行 ADR：** D-03 **产品执行 ADR met**；行 **仍冻**；**非** OTel GA **非** pprof GA **非** 端到端示例落地 **非** D-03 `[x]`。钉测：`qs1_d03_execute_adr_first_slice`。

** D-03 端到端示例：** `examples/observability_demo.sy` 串联 Log+Trace+Metrics+Profiler（指纹 `obs-demo-v1`）；`cargo run -p syx-cli -- run ./examples/observability_demo.sy`；in-process 薄可观测性；行 **仍冻**；**非** OTel GA **非** pprof export **非** D-03 `[x]`。钉测：`qs1_d03_e2e_example_first_slice`。

** D-03 收窄 soft-close：** `| D-03 |` **`[x]`** = ADR-1604 in-process e2e 完成面；**非** OTel GA **非** pprof 产品 GA。钉测：`qs1_d03_soft_close_first_slice`。

** E-02 负例全集 charter：** 书面冻结 loopback fail-closed 负例全集 in-scope/OUT；仍 `e02_negative_tests_not_claimed`；**非** 负例全集宣称 **非** streaming GA **非** 产品 gRPC GA。钉测：`qs1_e02_negative_tests_complete_charter_first_slice`。

** E-02 负例全集宣称：** charter in-scope 完成面宣称 + soft-close；本窗 inventory `e02_negative_tests_not_claimed=false`；**非** streaming GA **非** 产品 gRPC GA。钉测：`qs1_e02_negative_tests_complete_claim_first_slice`。

** E-02 streaming GA charter：** 书面冻结 loopback streaming 完成面 in-scope/OUT（W1158–W1166）；仍 `e02_streaming_not_claimed`；**非** streaming GA 宣称 **非** 产品 gRPC GA。钉测：`qs1_e02_streaming_ga_charter_first_slice`。

** E-02 streaming GA 宣称：** charter in-scope 完成面宣称 + soft-close；本窗 inventory `e02_streaming_not_claimed=false`；**非** 产品 gRPC GA **非** 互操作全集。钉测：`qs1_e02_streaming_ga_claim_first_slice`。

** E-02 product gRPC·interop GA charter：** 书面冻结未来诚实宣称产品 gRPC GA + 互操作 GA 的 in-scope/OUT（同 ADR 双伞）；仍 `e02_grpc_not_claimed` ∧ `e02_interop_not_claimed`；**非** 产品 gRPC GA **非** 互操作全集 **非** 翻两伞。钉测：`qs1_e02_product_grpc_interop_ga_charter_first_slice`。

** E-02 product gRPC·interop GA 宣称：** charter in-scope 完成面宣称 + soft-close；本窗 inventory `e02_grpc_not_claimed=false` ∧ `e02_interop_not_claimed=false`；**非** tonic 全集 **非** 升 `net.grpc` **非** 改写 W1273 历史双伞。钉测：`qs1_e02_product_grpc_interop_ga_claim_first_slice`。

** E-03 产品执行 ADR：** 书面冻结「更多 DB」完成面 in-scope/OUT；行 **仍冻**；**非** 更多 DB 产品 GA **非** SQLite 加深 **非** E-03 `[x]`。钉测：`qs1_e03_execute_adr_first_slice`。

** E-03 scaffold：** 钉既有 `sql.sy` / `:memory:` / Sql.* / `data.sqlite` partial 为薄入口；行 **仍冻**；**非** SQLite 加深 **非** 更多 DB 产品 GA **非** E-03 `[x]`。钉测：`qs1_e03_scaffold_first_slice`。
** E-03 SQLite deepen：** 书面残差诚实 census SSOT（file DSN vs `:memory:`；`data.sqlite` partial；Orm OUT；第二 driver OUT）；行 **仍冻**；**非** Sql 施工加深 **非** 更多 DB 产品 GA **非** E-03 `[x]`。钉测：`qs1_e03_sqlite_deepen_first_slice`。
** E-03 soft-close：** 收窄 deepen 完成面翻 `| E-03 |` → `[x]`；**仍** `e03_more_db_ga_not_claimed`；**非** 更多 DB 产品 GA **非** Sql 施工加深。钉测：`qs1_e03_soft_close_first_slice`。
** E-04 execute ADR：** 书面冻结 SynUI 真 renderer/event/a11y 完成面 in-scope/OUT；行 **仍冻**；**非** SynUI 产品 GA **非** v13 施工 **非** E-04 `[x]`。钉测：`qs1_e04_execute_adr_first_slice`。
** E-04 scaffold：** 钉既有 `packages/ui` / `__syx_synui_*` 为薄入口；行 **仍冻**；**非** SynUI deepen **非** SynUI 产品 GA **非** E-04 `[x]`。钉测：`qs1_e04_scaffold_first_slice`。
** E-04 SynUI deepen：** 书面残差诚实 census SSOT（Renderer structural-only；Event opt-in；a11y OUT；像素/v13/Farm OUT）；行 **仍冻**；**非** SynUI runtime 施工 **非** SynUI 产品 GA **非** E-04 `[x]`。钉测：`qs1_e04_synui_deepen_first_slice`。
** E-04 soft-close：** 收窄 deepen 完成面翻 `| E-04 |` → `[x]`；**仍** `e04_synui_ga_not_claimed`；**非** SynUI 产品 GA **非** SynUI runtime 施工。钉测：`qs1_e04_soft_close_first_slice`。
** E-05 execute ADR：** 书面冻结 Farm/Dist v20+ 完成面 in-scope/OUT；行 **仍冻**；**非** Farm/Dist 产品 GA **非** v20 施工 **非** E-05 `[x]`。钉测：`qs1_e05_execute_adr_first_slice`。
** E-05 scaffold：** 钉既有 `build.farm` / farm-lease v19 / dist peer-announce-hold 为薄入口；行 **仍冻**；**非** Farm/Dist deepen **非** Farm/Dist 产品 GA **非** E-05 `[x]`。钉测：`qs1_e05_scaffold_first_slice`。
** E-05 Farm/Dist deepen：** 书面残差诚实 census SSOT（lease/billing/TCP；peer-announce/sticky/multihost；`build.farm` frozen；HSM/multi-region OUT）；行 **仍冻**；**非** Farm/Dist runtime 施工 **非** Farm/Dist 产品 GA **非** E-05 `[x]`。钉测：`qs1_e05_farm_dist_deepen_first_slice`。
** E-05 soft-close：** 收窄 deepen 完成面翻 `| E-05 |` → `[x]`；**仍** `e05_farm_dist_ga_not_claimed`；**非** Farm/Dist 产品 GA **非** Farm/Dist runtime 施工。钉测：`qs1_e05_soft_close_first_slice`。
** Wave soft-complete + F-08 park honesty：** 书面 Wave soft-complete @ **52/53**（A–E 满；F **10/11**）；重申 ADR-1375 default-no；`| F-08 |` **仍 `[ ]`**；**非** F-08 `[x]` **非** RT ABI 全清 **非** Wave **53/53**。钉测：`qs1_wave_soft_complete_park_honesty_first_slice`。
** Post-wave product-debt parked-charter：** 另立维护/产品债 census SSOT（F-08；D/E/P 伞；SharedCell/wasm-gc；HostFn status residual）；`| F-08 |` **仍 `[ ]`**；**非** 债已清 **非** F-08 `[x]` **非** RT ABI 全清 **非** Wave **53/53**。钉测：`qs1_post_wave_product_debt_parked_charter_first_slice`。
** HostFn status residual honesty census：** 加深 Process×14 `partial`/`wasm=unsupported` SSOT；**仍** `hostfn_status_batch_all_implemented=false`；**非** 升 `implemented` **非** 全量 HostFn status **非** HostFn+1 **非** F-08 `[x]`。钉测：`qs1_hostfn_status_residual_honesty_census_first_slice`。
** SharedCell / wasm-gc residual honesty census：** 加深 Arc still present ∧ wasm-gc unmet ∧ guest SYX GC not claimed SSOT；**非** Arc 清除 **非** wasm-gc 完成 **非** F-08 `[x]` **非** Wave **53/53**。钉测：`qs1_sharedcell_wasm_gc_residual_honesty_census_first_slice`。
** D/E/P umbrella residual honesty census：** 加深仍 park 产品-GA 伞 SSOT（P-star：P-ABI-v2 / P-FAER / P-SURF；D/E 样本）；**仍** `*_ga_not_claimed` / `p_surf_mat_not_claimed`；**非** 翻 not_claimed **非** 产品 GA **非** F-08 `[x]` **非** Wave **53/53**。钉测：`qs1_dep_umbrella_residual_honesty_census_first_slice`。
** Post-wave residual honesty census-chain closeout：** 书面钉 W1287–W1291 census 栈齐 ∧ 仍 park ∧ 默认不再 census deepen；**非** 债已清 **非** 产品 GA **非** F-08 `[x]` **非** Wave **53/53**。钉测：`qs1_post_wave_residual_honesty_census_chain_closeout_first_slice`。

** 插件 ABI v2 / faer 旁路（parked 章程）：** 方案 B 写入 roadmap。** P-SURF packed-SYX 轴前缀met**；** P-SURF packed-SYX 轴前缀met**；** P-SURF packed-SYX 轴前缀met**；** P-SURF packed-SYX 轴归约met**；** P-SURF packed-SYX 轴归约met**；** P-SURF packed-SYX 轴归约met**；** P-SURF packed-SYX 轴归约met**；** P-SURF packed-SYX 轴归约met**；** P-SURF packed-SYX 掩码逻辑met**；** P-SURF packed-SYX 掩码逻辑met**；** P-SURF packed-SYX 掩码选择/同形填充met**；** P-SURF packed-SYX 元素比较met**；** P-SURF packed-SYX 外积/同形/比较met**；** P-SURF packed-SYX 前缀累加/差分met**；** P-SURF packed-SYX 元素 log10/Γ/lerp/dot met**；** P-SURF packed-SYX 元素熔合/指数对数met**；** P-SURF packed-SYX 元素浮点二元met**；** P-SURF packed-SYX 元素二元极值/符号met**；** P-SURF packed-SYX 元素一元met**；** P-SURF packed-SYX 元素除/标量广播met**；** P-SURF packed-SYX 元素一元met**；** P-SURF packed-SYX 几何置换met**；** P-SURF packed-SYX 几何翻转met**；** P-SURF packed-SYX 结构复制met**；** P-SURF packed-SYX 归约/范数met**；** P-SURF packed-SYX 形状拼装met**；** P-SURF packed-SYX 切片/访问met**；** P-SURF packed-SYX 元素算术met**；** P-SURF packed-SYX 形状通用化met**；** P-SURF packed-SYX 便利/stats met**；** P-SURF solve/packed/shape 方法族met**；** P-SURF zeros 语言面met**；** P-SURF 独立新 Mat met**；** P-FAER cdylib F64LE packed zeros met**；** P-SURF operator 返回值met**；** P-SURF Mat 独立包落地met**；** P-FAER cdylib F64LE packed solve met**；** P-FAER HANDLE×cdylib drop met**；** P-SURF 书面 Mat 约束met**；** P-FAER HANDLE×cdylib to/from met**；** P-FAER HANDLE×cdylib solve met**；** P-ABI-v2 句柄 borrow met**；** P-FAER cdylib 装载met**；** P-FAER solve met**；** P-FAER 施工met**（from/zeros/mul/to）；** 句柄表met**（pin/sumh/drop）；** 加深met**（多槽 iov + host realloc）；** 施工met**；`P-FAER` / `P-ABI-v2` / `P-SURF` 仍 `[ ]`；**非** faer 产品 GA **非** Mat **非** ABI v2 完成 **非** 句柄 GA **非** HostFn+1 **非** F-08。详 [`plan/topics/plugin-abi-v2-linalg.md`](../plan/topics/plugin-abi-v2-linalg.md)。

**M3-E02 叶子薄收口：** thin+deepen 证据链收口；roadmap `M3-E02` `[x]`；**非** 五域 GA **非** E03 **非** EC-M3-3 **非** M3 退出。钉测：`m3_e02_closeout`。

**M3-E03 HostFn capability 闸薄首钉：** opt-in `--ir-capability-gate` / `SYX_IR_CAPABILITY_GATE`；指纹 `hostfn-capability-gate-v0:compile-unsupported`；表 `status=unsupported` → `S026`；roadmap `M3-E03` `[~]`；**非** 默认全局闸 **非** backend 列 GA **非** EC-M3-3 **非** M3 退出。钉测：`m3_e03_capability_gate`。

**M3-E03 叶子薄收口：** thin 证据链收口；roadmap `M3-E03` `[x]`；**非** 默认全局闸 **非** EC-M3-3 **非** M3 退出。钉测：`m3_e03_closeout`。

**EC-M3-3 Capability-sandbox 证据链薄首钉：** E01–E03 三叶 closeout + 指纹证据链（`permission-policy-v0:default-deny-intent` / `permission-grants-v0:allow-net-fs-toml` / `hostfn-capability-gate-v0:compile-unsupported`）；roadmap 脚注 thin 已交；总勾仍 `[ ]`；**非** 五域 GA **非** 默认全局闸 **非** EC-M3-1 **非** M3 退出。钉测：`ec_m3_3_capability_sandbox`。

**EC-M3-3 叶子薄收口：** thin 证据链 → `EC-M3-3` 总勾 `[x]`（诚实收窄）；**非** 五域 GA **非** 默认全局闸 **非** EC-M3-1 **非** M3 退出。钉测：`ec_m3_3_closeout`。

**EC-M3-1 高并发压测可复现证据链薄首钉：** HTTP Criterion（`ec_m2_4_http_criterion`）+ A03/A04/C04 closeout + C04 抽样语料；诊断面继承 `liveTaskCount` / `profiler-v1-cpu`；roadmap 脚注 thin 已交；总勾仍 `[ ]`；**非** load-farm GA **非** 绝对 SLA 硬闸 **非** TSAN **非** M3 退出。钉测：`ec_m3_1_stress_repro`。

**EC-M3-1 叶子薄收口：** thin 证据链 → `EC-M3-1` 总勾 `[x]`（诚实收窄）；四 EC 齐；**非** load-farm GA **非** 绝对 SLA 硬闸 **非** TSAN **非** M3 退出。钉测：`ec_m3_1_closeout`。

**M3 退出：** 四 EC 总勾已签前提下的 thin exit；开放债（load-farm / SLA / TSAN / 五域 GA / async-let / 采样·OTel 等）**不挡退出**；**停止**默认 M3 EC 收口列车。钉测：`m3_exit_closeout`。

**M4-L01 一阶 where 约束薄首钉：** 泛型函数 `where T: Equatable`；`Int` compile-pass；非 Equatable 用户类型 → `S010`；指纹 `generic-where-v0:first-order-s010`；roadmap `M4-L01` `[~]`；**非** EC-M4-L **非** L02 **非** `where T == U` **非** M4 退出。钉测：`m4_l01_first_order_where`。

**M4-L01 叶子薄收口：** thin 证据链 → `M4-L01` `[x]`（诚实收窄）；**非** EC-M4-L **非** L02 **非** `where T == U` **非** M4 退出。钉测：`m4_l01_closeout`。

**M4-L02 关联类型 typealias 薄首钉：** 协议 `associatedtype Item: Equatable` + 符合侧 `typealias` 见证；`Item = Int` compile-pass；`Item = Box`（非 Equatable）→ `S012`；指纹 `assoc-type-v0:typealias-s012`；roadmap `M4-L02` `[~]`（**assoc-only**）；**非** conditional conformance GA **非** EC-M4-L **非** 跨模块矩阵 **非** M4 退出。钉测：`m4_l02_assoc_type`。

**M4-L02 叶子薄收口：** thin 证据链 → `M4-L02` `[x]`（诚实 **assoc-only**）；**非** conditional conformance GA **非** EC-M4-L **非** 跨模块矩阵 **非** M4 退出。钉测：`m4_l02_closeout`。

**条件符合薄首钉：** `extension Array: Marker where Element: Equatable` → 泛型 `where T: Marker` compile-pass（`[Int]`）/ compile-fail（非 Equatable 元素 → **S010**）；指纹 `conditional-conform-v0:array-element-s010`；L02 仍 `[x]`（deepen，**非**重开 `[~]`）；**非** conditional GA **非** EC-M4-L **非** L03 **非** M4 退出。钉测：`m4_conditional_conformance`。

**条件符合叶子薄收口：** thin 证据链 meta 收口；L02 仍 `[x]`；**非** conditional GA **非** EC-M4-L **非** L03 **非** M4 退出。钉测：`m4_conditional_closeout`。

**M4-L03 existential `any` 绑定薄首钉：** `let x: any Equatable = 1` compile-pass；非 Equatable `Box` → **S014**；指纹 `existential-any-v0:binding-s014`；roadmap `M4-L03` `[~]`；**非** full existential opening **非** `-> some/any` 返回门控 **非** EC-M4-L **非** M4 退出。钉测：`m4_l03_existential_any`。

**M4-L03 叶子薄收口：** thin 证据链 → `M4-L03` `[x]`（诚实 **binding-only**）；**非** full existential opening **非** EC-M4-L **非** M4 退出。钉测：`m4_l03_closeout`。

**EC-M4-L 协议层证据薄首钉：** L02 assoc/conditional + L03 binding 证据链 meta；指纹 `assoc-type-v0:typealias-s012` / `conditional-conform-v0:array-element-s010` / `existential-any-v0:binding-s014`；**EC-M4-L 总勾仍开**；**非** full opening GA **非** conditional GA **非** M4 退出。钉测：`ec_m4_l_protocol_layer`。

**EC-M4-L 叶子薄收口：** thin 证据链 → `EC-M4-L` **总勾已签**；**非** full opening GA **非** conditional GA **非** 其余 EC-M4-* **非** M4 退出。钉测：`ec_m4_l_closeout`。

**M4-B01 静态独占 borrow 薄首钉：** 单 `&mut` compile-pass；同区域双 `&mut` → **S011**；指纹 `borrow-exclusive-v0:double-mut-s011`；roadmap `M4-B01` `[~]`；**非** NLL **非** noncopyable **非** EC-M4-B **非** M4 退出。钉测：`m4_b01_exclusive_borrow`。

**M4-B01 叶子薄收口：** thin 证据链 → `M4-B01` `[x]`；**非** NLL **非** noncopyable **非** EC-M4-B **非** M4 退出。钉测：`m4_b01_closeout`。

**M4-B02 move-consume 所有权薄首钉：** `String` by-value 消费一次 compile-pass；再使用 → **S011**（Use of moved value）；指纹 `move-consume-v0:string-s011`；roadmap `M4-B02` `[~]`（**move-consume-only**）；**非** `~Copyable` **非** assignment-move **非** EC-M4-B **非** M4 退出。钉测：`m4_b02_move_consume`。

**M4-B02 叶子薄收口：** thin 证据链 → `M4-B02` `[x]`（诚实 **move-consume-only**）；**非** `~Copyable` **非** EC-M4-B **非** M4 退出。钉测：`m4_b02_closeout`。

**M4-B03 language-complete 非目标薄首钉：** M4 叶面明确关闭 language-complete borrow 为**非目标**；指纹 `borrow-lang-complete-v0:nongoal`；roadmap `M4-B03` `[~]`（**nongoal**）；完备/NLL deepen → EC-M4-B；**非** NLL GA **非** language-complete GA **非** EC-M4-B **非** M4 退出。钉测：`m4_b03_nongoal`。

**M4-B03 叶子薄收口：** thin 证据链 → `M4-B03` `[x]`（诚实 **nongoal**）；**非** NLL GA **非** language-complete GA **非** EC-M4-B **非** M4 退出。钉测：`m4_b03_closeout`。

**EC-M4-B Named move-consume 薄首钉：** `class`/Named by-value 消费一次 compile-pass；再使用 → **S011**；指纹 `move-consume-v0:named-s011`；roadmap EC-M4-B thin（总勾仍 `[ ]`；**named-move-layer**）；**非** NLL **非** `~Copyable` GA **非** EC 总勾 **非** M4 退出。钉测：`ec_m4_b_named_move`。

**EC-M4-B 叶子薄收口：** thin 证据链 → `EC-M4-B` **总勾已签**（诚实 **named-move-layer**；String + Named S011）；**非** NLL GA **非** `~Copyable` GA **非** 其余 EC-M4-* **非** M4 退出。钉测：`ec_m4_b_closeout`。

**EC-M4-M 宏 sandbox 薄首钉：** 外部宏默认 reexec sandbox（非 in-process）；`macro_sandbox_mode` ∈ `{sandbox-reexec-only, seccomp}`；WASM 外部展开 → **E004**；指纹 `macro-sandbox-v0:reexec-default`；roadmap EC-M4-M thin（总勾仍 `[ ]`；**sandbox-reexec**）；**非** source-map GA **非** incremental-binary GA **非** EC 总勾 **非** M4 退出。钉测：`ec_m4_m_macro_sandbox`。

**EC-M4-M 叶子薄收口：** thin 证据链 → `EC-M4-M` **总勾已签**（诚实 **sandbox-reexec-layer**）；**非** source-map GA **非** incremental-binary GA **非** 其余 EC-M4-P/X **非** M4 退出。钉测：`ec_m4_m_closeout`。

**EC-M4-P 签名校验薄首钉：** ed25519 checksum `syx1:ed25519:` verify pass；篡改 checksum / 坏签 → fail；install **P011**；指纹 `pkg-sig-v0:ed25519-checksum`；roadmap EC-M4-P thin（总勾仍 `[ ]`；**signature-verify**）；**非** trust-root GA **非** solver GA **非** frozen GA **非** EC 总勾 **非** M4 退出。钉测：`ec_m4_p_signature_verify`。

**EC-M4-P 叶子薄收口：** thin 证据链 → `EC-M4-P` **总勾已签**（诚实 **signature-verify-layer**）；**非** trust-root GA **非** solver GA **非** frozen GA **非** EC-M4-X **非** M4 退出。钉测：`ec_m4_p_closeout`。

**EC-M4-X 产品门薄首钉：** 本里程碑 **product-gate defer**（不单选 WS/gRPC/UI）；指纹 `ec-m4-x-v0:product-gate`；roadmap EC-M4-X thin（总勾仍 `[ ]`）；**非** 真 WS **非** 真 gRPC **非** SynUI 解冻 **非** EC 总勾 **非** M4 退出。钉测：`ec_m4_x_product_gate`。

**EC-M4-X 叶子薄收口：** thin 证据链 → `EC-M4-X` **总勾已签**（诚实 **product-gate-layer**）；**非** 真 WS GA **非** 真 gRPC GA **非** SynUI GA **非** M4-X01–X04 满勾 **非** M4 退出。钉测：`ec_m4_x_closeout`。

**M4 退出：** 五 EC 总勾已签前提下的 thin exit；开放债（真 WS/gRPC/SynUI；trust-root / solver / frozen；NLL；source-map 等）**不挡退出**；**停止**默认 M4 EC 收口列车；**Truth M0–M4 列车收官**。钉测：`m4_exit_closeout`。

**M3-A04 Channel close 首钉：** `Channel.close` 真断开 sender；drain 后 recv **Nil**；再 send → **C010**；**非** backpressure/fairness 满勾 **非** EC-M3 **非** A04 满勾。钉测：`m3_a04_channel_close`。

**M3-A04 Channel backpressure 加深：** bounded `create(N)`；填满后 `trySend` → **false**；`count`/`capacity` 可观测；**非** fairness 满勾 **非** EC-M3 **非** A04 满勾。钉测：`m3_a04_backpressure`。

**M3-A04 Channel fairness 产品钉：** 多臂同时 ready 时 `__syx_channel_select` round-robin（`select-fair-v1:round-robin`）；seed=0 → 20 次 10/10；**非**跨线程 RR GA **非** EC-M3。钉测：`m3_a04_fairness`。

**M3-A04 leaf thin closeout：** close + backpressure + fairness 三轴证据链收口；roadmap `M3-A04` `[x]`；**非**跨线程 RR GA **非** EC-M3 **非** A05 **非** M3 退出。钉测：`m3_a04_closeout`。

**M3-A05 池分层文档首钉：** 结构化并发为主业务 API；`Concurrency` 池/Actor 容量旋钮为**高级运行时配置**；**非**删池 API **非** EC-M3 **非** A05 满勾（本窗）。钉测：`m3_a05_pool_layering`。

**M3-A05 leaf thin closeout：** 分层文档证据链收口；roadmap `M3-A05` `[x]`；**非**删池 API **非** EC-M3 **非** ThreadPoolExecutor 全集 **非** B01 **非** M3 退出。钉测：`m3_a05_closeout`。

**EC-M2-1-Z：** A–F 六切片交付（file+Path / HTTP client / SQLite / HTTP server / Path deep / five-slice closeout）证据链收口已签；**非** 全量 Path GA。钉测：`ec_m2_1_z_closeout`。

**EC-M2-1 总勾：** 官方 stdlib 可交付 A–F + Z 证据链已签（诚实收窄）；`io.path` 仍 partial；Http 无 typed throws 全迁；**非** M2 退出。钉测：`ec_m2_1_total_closeout`。

**Q-S5：** 四类 handle multi-handle stress（File/Sql/Tcp/Process；N=4）live_count 回基线已签；**非** FileStream·Watcher·Pipe session **非** Listener·TLS **非** EC-M2-3 总勾。钉测：`q_s5_closeout`。

**EC-M2-3-F：** FileStream / FileWatcher / ChildPipe session `*_live_count` 首钉已签（仅测试/诊断层）；**非** HTTP body stream **非** Listener·TLS **非** EC-M2-3 总勾。钉测：`ec_m2_3_session_leak_closeout`。

**EC-M2-3-G：** HttpBodyStream session `http_body_stream_live_count` 首钉已签（仅测试/诊断层）；**非** Listener·TLS **非** EC-M2-3 总勾。钉测：`ec_m2_3_http_body_stream_leak_closeout`。

**EC-M2-3-H：** TcpListener bind/close session `tcp_listener_live_count` 首钉已签（仅测试/诊断层）；**非** accept **非** TLS **非** EC-M2-3 总勾（当时）。钉测：`ec_m2_3_tcp_listener_leak_closeout`。

**EC-M2-3 总勾：** 已有产品 handle/session 泄漏钉齐已签（诚实收窄）；`net.tls.server` 仍 unsupported；无 accept GA；**非** M2 退出。钉测：`ec_m2_3_total_closeout`。

**FileHandle RAII / `withOpen`（`M2-S3-02b`）：** 协议面 `Reader` / `Writer` / `Seekable` / `Closeable`；`FileHandle.openResource` → `FileResource` 实例（read/write/seek/close + `deinit` quiet-close）；`FileHandle.withOpen(path:mode:body:)` 对 `body(handle: Int)` **退出必 close**（资源契约；body 成功须返回 `>= 0`）。**非**语言关键字 `with`/`defer`；**非** `EC-M2-3` 总勾。钉测：`m2_s3_02b_filehandle_raii_with`。

**`readAllBytes`（`M2-S3-04`）：** `FileHandle.readAllBytes` 为整缓冲 `read_to_end`（公开面已无 `readMmap`）。**≠ OS mmap**；流式分块见 `FileStream`。`io.file.read_all_bytes` implemented。钉测：`m2_s3_04_mmap_rename`。**非** `memmap2`；**非** `EC-M2-4-Z`。

**FileStream / FileWatcher（`M2-S3-03`）：** `FileStream.open` → `readChunk(handle, maxBytes)`（空数组=EOF）→ `close`；`FileWatcher.open` → `poll` → `close`。**关闭后再操作必须失败**。默认 `File.read` 仍整缓冲；`File.watch` 仍是 one-shot（≠ lifecycle）。`io.file.stream` implemented；`io.file.watch` partial（mock 默认）。钉测：`m2_s3_03_file_stream_watcher`。**非** 递归/async notify GA；**非** `EC-M2-4-Z`。

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

**HTTP 命名类型（`M2-S4-01`）：** facade 返回 `HTTPResponse` / `Url`（及类型面 `HttpRequest`/`HttpHeaders`/`HttpBody`），**非**糊墙 `Object`。运行时仍可为 Object 袋 + MethodId 访问器。钉测：`m2_s4_01_http_types`。流式读见。**非** `EC-M2-4-Z`。

**TLS client 真实路径（`M2-S4-05`）：** HTTPS 走 reqwest+rustls **默认校验证书**；CI/诊断可用环境变量 `SYX_HTTP_EXTRA_ROOT_PEM`（PEM 文件路径）**追加**信任锚（≠ skip-verify / ≠ `danger_accept_invalid_certs`）。钉测：`m2_s4_05_client_tls`（loopback HTTPS 正例）。**非** skip-verify 产品 API；**非** server TLS GA；**非** `EC-M2-4-Z`。

**HTTP body stream（`M2-S4-04`）：** `HttpBodyStream.open` → `readChunk(handle, maxBytes)`（空串=EOF）→ `close`；session + `Read`（≠ `response.bytes`）。默认 `Http.get`/`fetch` **仍整缓冲**。钉测：`m2_s4_04_http_body_stream`。**非** server 流式；**非** wire chunked TE 产品 API；**非** File S3-03；**非** JSONStream；**非** `EC-M2-4-Z`。

**HTTP server graceful shutdown（`M2-S4-03b`）：** `Http.shutdown` → watch cancel（停 accept）；完成当前请求后 `Http.serve` 返回。钉测：`m2_s4_03b_server_graceful_shutdown`（**不用** `SYX_HTTP_SERVE_MAX_REQUESTS` 冒充 graceful）。`MAX_REQUESTS` 仍是独立测试钩。**非** server TLS GA；**非** `EC-M2-4-Z`。

**HTTP server 后端：默认** `Http.serve` / `Http.shutdown` / `upgradeUpstream` 字节隧道建在 **hyper + tokio** 上（[`crates/syx-vm/src/http.rs`](../crates/syx-vm/src/http.rs)；plain HTTP 经 hyper HTTP/1；Upgrade 头探测后走旁路字节隧道；handler 经隔离线程接入以保 `ProgramCtx` / native MIR）。**未做**（进程内帧终结 / serve 自算 Accept）。**非** axum/ntex/warp；**非** `net.http.server` 产品 GA；**非** HostFn+1；**非** 翻行。

**进程内 HTTPS：** 诊断 env `SYX_HTTP_SERVE_TLS_CERT` + `SYX_HTTP_SERVE_TLS_KEY`（PEM 路径）齐备时，accept 后 **rustls 终止**再走 hyper HTTP/1；客户端可用 `SYX_HTTP_EXTRA_ROOT_PEM` 追加信任锚（≠ skip-verify）。钉 `http_serve_https_loopback_get_with_extra_root`。**宣称** HTTPS GET met；**不宣称** WSS 产品 GA（HMR-over-HTTPS 见）；**不宣称** sgate 生产去 sidecar；**不宣称** `net.http.server` / `net.tls.server` 产品 GA。

**多连接重叠：** accept 后 `JoinSet` spawn（multi-thread runtime）；plain / Upgrade 可重叠处理；钉 `http_serve_overlap_slow_fast_plain_get`（慢请求持锁期间快请求须完成）。**宣称** 多连接可重叠 accept/处理met；**不宣称** 高并发 SLA / 连接池 / backpressure / 全量调优。

**重叠×HMR 交叉：** 慢 plain GET 持锁期间，同端口 HMR Upgrade（101 + Accept 透传 + 掩码 text-frame）须完成；钉 `http_serve_overlap_slow_plain_and_hmr_upgrade`。**宣称** 组合交叉met；**不宣称** 全量并发 SLA。

**Upgrade-over-TLS / HMR-over-HTTPS：** 诊断 PEM 齐备时，rustls 终止后 Upgrade 走与 plain 同语义的 `upgradeUpstream` 字节隧道（含 head 后 leftover；async `copy_bidirectional`）；同端口 Vite-like 路径：101 + Accept 透传 + 掩码 text-frame。钉 `http_serve_https_hmr_path_same_port_roundtrip`。**宣称** Upgrade-over-TLS / 同端口 HMR-over-HTTPS （自动化钉）met；**不宣称** WSS 产品 GA；**不宣称** sgate 生产去 sidecar；**不宣称** 浏览器 Playwright HMR GA。

**进程内帧终结 / serve 自算 Accept：** handler 返回显式 `upgradeTerminate: true`（**≠** 缺 `upgradeUpstream`——缺字段仍 502）时，serve 从客户端 `Sec-WebSocket-Key` **自算** `Sec-WebSocket-Accept` → 101 → 解一帧掩码 text 并 echo。与 `upgradeUpstream` **透传**路径二分：HMR 仍走上游；终结不拨号。钉 `http_serve_upgrade_terminate_accept_and_text_echo`。**宣称** 帧终结 / 自算 Accept met；**不宣称** WSS / WebSocket 产品 GA；**不宣称** 改写 HMR 透传。

**upgradeTerminate close-path：** text echo 之后读客户端 close（opcode 8）→ 回 unmasked close；钉 `http_serve_upgrade_terminate_accept_text_and_close`。**宣称** close-path met；**不宣称** 完整 RFC6455（binary/fragment/extension/ping）；**不宣称** WSS 产品 GA。

**upgradeTerminate binary echo：** text echo 之后再读一帧客户端 binary（opcode 2）→ 回 unmasked binary（`0x82`）→ 再 close 交换；钉 `http_serve_upgrade_terminate_accept_text_binary_and_close`。**宣称** binary echo met；**不宣称** 完整 RFC6455（fragment/extension/ping）；**不宣称** WSS 产品 GA。

**upgradeTerminate ping→pong：** text→binary 之后读客户端 ping（opcode 9）→ 回 unmasked pong（`0x8A` + 同 payload）→ 再 close 交换；钉 `http_serve_upgrade_terminate_accept_text_binary_ping_and_close`。**宣称** ping→pong met；**不宣称** 完整 RFC6455（fragment/extension/完整 keepalive）；**不宣称** WSS 产品 GA。

**upgradeTerminate 分片 text：** 客户端两帧分片 text（`FIN=0`+opcode 1 → `FIN=1`+continuation）→ 服务端拼装回单帧 unmasked text；其后 binary→ping/pong→close；钉 `http_serve_upgrade_terminate_accept_fragmented_text_binary_ping_and_close`。**宣称** 分片 text met；**不宣称** binary 分片 / 多段 continuation / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate binary 分片：** 分片 text 之后再两帧分片 binary（`FIN=0`+opcode 2 → `FIN=1`+continuation）→ 拼装回单帧 unmasked binary（`0x82`）→ ping/pong→close；钉 `http_serve_upgrade_terminate_accept_fragmented_text_fragmented_binary_ping_and_close`。**宣称** binary 分片met；**不宣称** 多段 continuation / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate 多段 continuation N=3：** frag-text/binary 均为三帧（`FIN=0` data + `FIN=0` mid cont + `FIN=1` final cont）拼装回单帧；钉 `http_serve_upgrade_terminate_accept_multi_continuation_n3_ping_and_close`。**宣称** 多段 continuation（N=3）met；**不宣称** 任意段数 / control-during-fragment / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate control-during-fragment：** N=3 text 在 frag1 与 mid continuation 之间可插 client ping → 服务端即时 pong，**不中止**拼装 → 单帧 `0x81` echo；binary 仍 N=3 无 mid-control；钉 `http_serve_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close`。**宣称** control-during-fragment（N=3 text / frag1→mid 插 ping）met；**不宣称** 任意段数 / binary mid-control / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate binary mid-control：** N=3 binary 在 frag1 与 mid continuation 之间可插 client ping → 即时 pong → 拼装回单帧 `0x82`；钉 `http_serve_upgrade_terminate_accept_control_during_fragment_n3_binary_ping_and_close`。**宣称** control-during-fragment（N=3 binary / frag1→mid 插 ping）met；**不宣称** 任意段数 / mid↔final control / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate 任意段数 N=4 text：** 分片 text 四帧（`FIN=0` data + 2× `FIN=0` cont + `FIN=1` cont）拼装回单帧 `0x81`；binary 生命周期仍 N=3；钉 `http_serve_upgrade_terminate_accept_multi_continuation_n4_text_ping_and_close`。**宣称** 任意段数（N=4 text）met；**不宣称** 无界 N / binary 任意-N / mid↔final control / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate binary 任意段数 N=4：** 分片 binary 四帧拼装回单帧 `0x82`；text 同生命周期保留 N=4；钉 `http_serve_upgrade_terminate_accept_multi_continuation_n4_text_and_binary_ping_and_close`。**宣称** binary 任意段数（N=4）met；**不宣称** 无界 N / mid↔final control / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate mid↔final control：** N=4 text 在 cont1 与 cont2 之间可插 client ping → 即时 pong → 拼装回单帧 `0x81`；钉 `http_serve_upgrade_terminate_accept_mid_final_control_n4_text_ping_and_close`。**宣称** mid↔final control（N=4 text / cont 间插 ping）met；**不宣称** 无界 N / binary mid↔final / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate binary mid↔final control：** N=4 binary 在 cont1 与 cont2 之间可插 client ping → 即时 pong → 拼装回单帧 `0x82`；钉 `http_serve_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close`。**宣称** binary mid↔final control（N=4 binary / cont 间插 ping）met；**不宣称** 无界 N / extension / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate 无界 N（N=5 text）：** 分片 text 五帧（`FIN=0` data + 3× `FIN=0` cont + `FIN=1` cont）拼装回单帧 `0x81`；钉 `http_serve_upgrade_terminate_accept_multi_continuation_n5_text_ping_and_close`。**宣称** 无界 N（N=5 text）met；**不宣称** 无界矩阵全集 / N=5 binary / extension / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate 无界 N（N=5 binary）：** 分片 binary 五帧拼装回单帧 `0x82`；text 同生命周期保留 N=5；钉 `http_serve_upgrade_terminate_accept_multi_continuation_n5_text_and_binary_ping_and_close`。**宣称** 无界 N（N=5 binary）met；**不宣称** 无界矩阵全集 / extension / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate close status+reason echo：** 客户端 close 带 2 字节 status + UTF-8 reason 时，服务端 unmasked 回显同 payload；空 close 仍回空 close；钉 `http_serve_upgrade_terminate_accept_text_close_status_reason_echo`。**宣称** close status+reason echo met；**不宣称** RSV / extension / close 负例全集 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate RSV fail-closed：** 客户端帧 RSV1/2/3 任一非 0 时，服务端 fail-closed（不 echo / 断连）；RSV=0 正向路径不变；钉 `http_serve_upgrade_terminate_accept_rsv_text_fail_closed`。**宣称** RSV fail-closed met；**不宣称** extension 协商 / 协议 close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate close body len=1 fail-closed：** 客户端 close payload 恰好 1 字节时，服务端 fail-closed（不回 close / 断连）；空 close 与 len≥2 echo 不变；钉 `http_serve_upgrade_terminate_accept_close_len1_fail_closed`。**宣称** close body len=1 fail-closed met；**不宣称** unmasked client / extension / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate unmasked-client fail-closed：** 客户端帧 MASK=0 时，服务端 fail-closed（不 echo / 断连）；masked 正向路径不变；钉 `http_serve_upgrade_terminate_accept_unmasked_text_fail_closed`。**宣称** unmasked-client fail-closed met；**不宣称** fragmented control / extension / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate fragmented-control（FIN=0 ping）fail-closed：** 客户端 ping FIN=0 时，服务端 fail-closed（不 pong / 断连）；FIN=1 ping 正向路径不变；钉 `http_serve_upgrade_terminate_accept_fin0_ping_fail_closed`。**宣称** fragmented-control（FIN=0 ping）fail-closed met；**不宣称** extension / reserved opcode / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate reserved-control-opcode（0xB–0xF）fail-closed：** 客户端帧 opcode ∈ 0xB..=0xF 时，服务端 fail-closed（不 echo / 断连）；合法 opcode 正向路径不变；钉 `http_serve_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed`。**宣称** reserved-control-opcode（0xB–0xF）fail-closed met；**不宣称** extension / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate data-reserved-opcode（0x3–0x7）fail-closed：** 客户端帧 opcode ∈ 0x3..=0x7 时，服务端 fail-closed（不 echo / 断连）；0xB..=0xF 与合法 opcode 行为不变；钉 `http_serve_upgrade_terminate_accept_reserved_opcode_0x3_fail_closed`。**宣称** data-reserved-opcode（0x3–0x7）fail-closed met；**不宣称** extension / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate oversized-control-payload（>125）fail-closed：** 控制帧（opcode 0x8/0x9/0xA）且 len7>125 时，服务端 fail-closed（不 pong / 断连；拒在扩展长度解析前）；≤125 control 与 data 帧不变；钉 `http_serve_upgrade_terminate_accept_oversized_ping_fail_closed`。**宣称** oversized-control-payload（>125）fail-closed met；**不宣称** extension / close(1002) 产品语义 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate illegal-close-status fail-closed：** 客户端 close payload len≥2 且 status ∈ 0..=999 / 1005 / 1006 / 1015 时，服务端 fail-closed（不 echo close / 断连）；空 close 与合法 status（如 1000+reason）不变；钉 `http_serve_upgrade_terminate_accept_close_status_1005_fail_closed`。**宣称** illegal-close-status fail-closed met；**不宣称** extension / close(1002) 产品回帧 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate close-reason-utf8 fail-closed：** 客户端 close payload `len>2` 且 reason（`payload[2..]`）非合法 UTF-8 时，服务端 fail-closed（不 echo close / 断连）；`len∈{0,2}` 与合法 UTF-8 reason（如 1000+"bye"）不变；钉 `http_serve_upgrade_terminate_accept_close_reason_non_utf8_fail_closed`。**宣称** close-reason-utf8 fail-closed met；**不宣称** UTF-8 text / extension / close(1002) 产品回帧 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate UTF-8 text + close-1004 fail-closed 三连簇：**（1）FIN=1 text 载荷非合法 UTF-8 → fail-closed（钉 `http_serve_upgrade_terminate_accept_text_non_utf8_fail_closed`）；（2）分片 text 拼装后非合法 UTF-8 → fail-closed（钉 `…_fragmented_text_non_utf8_fail_closed`）；（3）close status 1004 → fail-closed（钉 `…_close_status_1004_fail_closed`）。合法 UTF-8 text / binary / 空 close / 1000+reason 不变。**宣称** UTF-8 text fail-closed 第一簇 + close-1004 fail-closed met；**不宣称** UTF-8 text 产品 GA / extension / close(1002) 产品回帧 / 完整 codec；**不宣称** WSS 产品 GA。

**upgradeTerminate framing 负例 fail-closed 四连簇：**（1）FIN=0 close → fail-closed（钉 `http_serve_upgrade_terminate_accept_fin0_close_fail_closed`）；（2）无进行中消息时 continuation（opcode 0x0）→ fail-closed（钉 `…_unexpected_continuation_fail_closed`）；（3）首发 unsolicited client pong（opcode 0xA）→ fail-closed（钉 `…_unsolicited_pong_fail_closed`）；（4）`read_client_ws_frame` 对 close/ping/pong 中心化 `require_control_fin`；分片途中 FIN=0 close → fail-closed（钉 `…_fin0_close_during_fragment_fail_closed`）。合法 FIN=1 close / ping→pong / 分片 data 不变。**宣称** framing 负例 fail-closed 第一簇 met；**不宣称** extension / close(1002) 产品回帧 / 完整 RFC6455；**不宣称** WSS 产品 GA。

**upgradeTerminate close(1002) 产品回帧四连簇：** 协议错路径由 abort-only 升为 **先发 unmasked close(status=1002) 再关连接**（`write_protocol_close_1002`）：（1）RSV≠0 → close(1002)（钉 `…_accept_rsv_text_close_1002`）；（2）reserved opcode（0xB）→ close(1002)（钉 `…_accept_reserved_opcode_0xb_close_1002`）；（3）FIN=0 control（ping）→ close(1002)（钉 `…_accept_fin0_ping_close_1002`）；（4）unexpected continuation → close(1002)（钉 `…_accept_unexpected_continuation_close_1002`）。既有 framing/RSV/reserved/UTF-8 等 abort 钉改期望 1002 回帧。合法 lifecycle / 空 close / 1000+reason 不变。**宣称** close(1002) 产品回帧第一簇（RSV / reserved opcode / FIN=0 control / unexpected continuation）met；**不宣称** close 码全集 GA / extension / 完整 RFC6455；**不宣称** WSS 产品 GA。

**upgradeTerminate close(1002) 产品回帧第二簇：** 统一出口已挂；补产品专钉矩阵：（1）unmasked client → close(1002)（钉 `…_accept_unmasked_text_close_1002`）；（2）oversized ping（len7=126）→ close(1002)（钉 `…_accept_oversized_ping_close_1002`）；（3）close body len=1 → close(1002)（钉 `…_accept_close_len1_close_1002`）；（4）illegal close status 1005 → close(1002)（钉 `…_accept_close_status_1005_close_1002`）。既有 `*_fail_closed` 保留。**宣称** close(1002) 产品回帧第二簇（unmasked / oversized ping / close len=1 / status 1005）met；**不宣称** close 码全集 GA / UTF-8 text 产品 GA / extension；**不宣称** WSS 产品 GA。

**upgradeTerminate-over-TLS 产品簇：** HTTPS 上 `upgradeTerminate` 与 plain 同语义：（1）自算 Accept + text echo（钉 `http_serve_https_upgrade_terminate_accept_and_text_echo`）；（2）RSV≠0 → close(1002)（钉 `…_accept_rsv_text_close_1002`）；（3）text→binary→ping/pong→close（钉 `…_accept_text_binary_ping_and_close`）；（4）HMR-over-HTTPS Accept 仍透传上游、≠ serve 自算（钉 `http_serve_https_hmr_path_accept_still_relayed_not_serve_computed`）。**宣称** upgradeTerminate-over-TLS / WSS terminate 产品簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate close(1002) 产品回帧第三簇：** 统一 `InvalidData`→close(1002) 出口已挂；补产品专钉矩阵：（1）text 非 UTF-8 → close(1002)（钉 `…_accept_text_non_utf8_close_1002`）；（2）close reason 非 UTF-8 → close(1002)（钉 `…_accept_close_reason_non_utf8_close_1002`）；（3）unsolicited pong → close(1002)（钉 `…_accept_unsolicited_pong_close_1002`）；（4）illegal close status 1004 → close(1002)（钉 `…_accept_close_status_1004_close_1002`）。既有 `*_fail_closed` 保留。**宣称** close(1002) 产品回帧第三簇（text 非 UTF-8 / close-reason 非 UTF-8 / unsolicited pong / status 1004）met；**不宣称** close 码全集 GA / UTF-8 text 产品 GA / extension / WSS 产品 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate close(1002) 产品回帧第四簇：** 统一出口已挂；补产品专钉矩阵：（1）fragmented text 非 UTF-8 → close(1002)（钉 `…_accept_fragmented_text_non_utf8_close_1002`）；（2）FIN=0 close → close(1002)（钉 `…_accept_fin0_close_close_1002`）；（3）FIN=0 close mid-fragment → close(1002)（钉 `…_accept_fin0_close_during_fragment_close_1002`）；（4）reserved data opcode 0x3 → close(1002)（钉 `…_accept_reserved_opcode_0x3_close_1002`）。既有 `*_fail_closed` 保留；plain 专钉矩阵 16/16。**宣称** close(1002) 产品回帧第四簇（frag UTF-8 / FIN=0 close / FIN=0 close-during-fragment / reserved 0x3）met；**不宣称** close 码全集 GA / UTF-8 text 产品 GA / extension / WSS 产品 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS close(1002) 产品回帧第一簇：** HTTPS 上复用同套 `InvalidData`→close(1002) 出口；补 TLS 产品专钉：（1）reserved control opcode 0xB → close(1002)（钉 `http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_close_1002`）；（2）FIN=0 ping → close(1002)（钉 `…_accept_fin0_ping_close_1002`）；（3）unexpected continuation → close(1002)（钉 `…_accept_unexpected_continuation_close_1002`）；（4）unmasked client text → close(1002)（钉 `…_accept_unmasked_text_close_1002`）。**宣称** upgradeTerminate-over-TLS close(1002) 产品回帧第一簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** TLS close(1002) 矩阵全集 / 去 sidecar / Playwright。

**upgradeTerminate-over-TLS close(1002) 产品回帧第二簇：** 统一出口已挂；补 TLS 产品专钉：（1）oversized ping（len7=126）→ close(1002)（钉 `http_serve_https_upgrade_terminate_accept_oversized_ping_close_1002`）；（2）close body len=1 → close(1002)（钉 `…_accept_close_len1_close_1002`）；（3）illegal close status 1005 → close(1002)（钉 `…_accept_close_status_1005_close_1002`）；（4）text 非 UTF-8 → close(1002)（钉 `…_accept_text_non_utf8_close_1002`）。**宣称** upgradeTerminate-over-TLS close(1002) 产品回帧第二簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** TLS close(1002) 矩阵全集 / UTF-8 text 产品 GA / 去 sidecar / Playwright。

**upgradeTerminate-over-TLS close(1002) 产品回帧第三簇：** 统一出口已挂；补 TLS 产品专钉：（1）close reason 非 UTF-8 → close(1002)（钉 `http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_close_1002`）；（2）unsolicited pong → close(1002)（钉 `…_accept_unsolicited_pong_close_1002`）；（3）illegal close status 1004 → close(1002)（钉 `…_accept_close_status_1004_close_1002`）；（4）fragmented text 非 UTF-8 → close(1002)（钉 `…_accept_fragmented_text_non_utf8_close_1002`）。**宣称** upgradeTerminate-over-TLS close(1002) 产品回帧第三簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** TLS close(1002) 矩阵全集 / UTF-8 text 产品 GA / 去 sidecar / Playwright。

**upgradeTerminate-over-TLS close(1002) 产品回帧第四簇：** 统一出口已挂；补 TLS 产品专钉：（1）FIN=0 close → close(1002)（钉 `http_serve_https_upgrade_terminate_accept_fin0_close_close_1002`）；（2）FIN=0 close mid-fragment → close(1002)（钉 `…_accept_fin0_close_during_fragment_close_1002`）；（3）reserved data opcode 0x3 → close(1002)（钉 `…_accept_reserved_opcode_0x3_close_1002`）。**宣称** upgradeTerminate-over-TLS close(1002) 产品回帧第四簇 met；TLS close(1002) 专钉矩阵相对 plain **16/16** met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** close 码全集 GA / UTF-8 text 产品 GA / 去 sidecar / Playwright。

**upgradeTerminate illegal close-status 加深第一簇：** 统一出口已挂；补产品专钉：（1）plain close status 1006 → close(1002)（钉 `http_serve_upgrade_terminate_accept_close_status_1006_close_1002`）；（2）plain close status 1015 → close(1002)（钉 `…_accept_close_status_1015_close_1002`）；（3）plain close status 999（0–999 代表）→ close(1002)（钉 `…_accept_close_status_999_close_1002`）；（4）TLS close status 1006 → close(1002)（钉 `http_serve_https_upgrade_terminate_accept_close_status_1006_close_1002`）。**宣称** illegal close-status 加深第一簇 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate illegal close-status 加深第二簇：** 统一出口已挂；补产品专钉：（1）TLS close status 1015 → close(1002)（钉 `http_serve_https_upgrade_terminate_accept_close_status_1015_close_1002`）；（2）TLS close status 999 → close(1002)（钉 `…_accept_close_status_999_close_1002`）；（3）plain close status 0 → close(1002)（钉 `http_serve_upgrade_terminate_accept_close_status_0_close_1002`）；（4）plain close status 500 → close(1002)（钉 `…_accept_close_status_500_close_1002`）。**宣称** illegal close-status 加深第二簇 met；TLS 非法五码（1004/1005/1006/1015/999）与 plain 对齐；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS 正向 codec 加深第一簇：** 无新 runtime；HTTPS terminate 对齐 plain 正向四钉：（1）close status+reason echo（钉 `http_serve_https_upgrade_terminate_accept_text_close_status_reason_echo`）；（2）双 frag text+binary + ping + close（钉 `…_accept_fragmented_text_fragmented_binary_ping_and_close`）；（3）multi-continuation N=3 + ping + close（钉 `…_accept_multi_continuation_n3_ping_and_close`）；（4）mid↔final control N=4 text + ping + close（钉 `…_accept_mid_final_control_n4_text_ping_and_close`）。**宣称** upgradeTerminate-over-TLS 正向 codec 加深第一簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA / TLS 正向 codec 全集 / close 码全集 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS 正向 codec 加深第二簇：** 无新 runtime；HTTPS terminate 对齐 plain：（1）control-during-fragment N=3 text（钉 `http_serve_https_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close`）；（2）control-during-fragment N=3 binary（钉 `…_accept_control_during_fragment_n3_binary_ping_and_close`）；（3）multi-continuation N=4 text（钉 `…_accept_multi_continuation_n4_text_ping_and_close`）；（4）multi-continuation N=4 text+binary（钉 `…_accept_multi_continuation_n4_text_and_binary_ping_and_close`）。**宣称** upgradeTerminate-over-TLS 正向 codec 加深第二簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA / TLS 正向 codec 全集 / close 码全集 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS 正向 codec 加深第三簇：** 无新 runtime；HTTPS terminate 对齐 plain /：（1）mid↔final control N=4 binary（钉 `http_serve_https_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close`）；（2）multi-continuation N=5 text（钉 `…_accept_multi_continuation_n5_text_ping_and_close`）；（3）multi-continuation N=5 text+binary（钉 `…_accept_multi_continuation_n5_text_and_binary_ping_and_close`）；（4）fragmented text + FIN=1 binary（钉 `…_accept_fragmented_text_binary_ping_and_close`）。**宣称** upgradeTerminate-over-TLS 正向 codec 加深第三簇 met；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA / TLS 正向 codec 全集 / N5 无界全集 GA / close 码全集 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS 正向 twin 收口 + illegal-status TLS 补齐：** 无新 runtime；HTTPS terminate 对齐 plain /：（1）text + close（钉 `http_serve_https_upgrade_terminate_accept_text_and_close`）；（2）text + binary + close（钉 `…_accept_text_binary_and_close`）；（3）illegal close status 0 → close(1002)（钉 `…_accept_close_status_0_close_1002`）；（4）illegal close status 500 → close(1002)（钉 `…_accept_close_status_500_close_1002`）。**宣称** upgradeTerminate-over-TLS 正向 twin 收口 + illegal-status TLS 0/500 补齐 met / plain↔HTTPS 正向 twin 名表齐；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA / TLS 正向 codec 全集 / close 码全集 GA；**不宣称** 去 sidecar / Playwright。

**upgradeTerminate-over-TLS fail_closed 名表四连第一簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 plain `*_fail_closed` + 场景）：（1）reserved opcode 0xB（钉 `http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed`）；（2）FIN=0 ping（钉 `…_accept_fin0_ping_fail_closed`）；（3）unexpected continuation（钉 `…_accept_unexpected_continuation_fail_closed`）；（4）unmasked text（钉 `…_accept_unmasked_text_fail_closed`）。**宣称** fail_closed 名表四连第一簇 met / TLS fail_closed **4/16**；**不宣称** fail_closed TLS 名表全集 / WSS / WebSocket / `net.tls.server` 产品 GA / close 码全集 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**todo 8 总 soft-close / 项收口：** 无新 runtime / 无 HostFn+1；书面冻结项 8 完成面 SSOT：hyper+tokio 默认栈；同端口 `upgradeUpstream`/HMR（含 over-TLS）；重叠 accept + 重叠×HMR；HTTPS + sgate native；`upgradeTerminate` plain/over-TLS + close(1002) 主矩阵 + illegal 近界 soft-close；WSS 营销宣称边界 soft-close。**OUT（项级永久）：** Playwright HMR GA；全量并发 SLA；`ws_routes` 去 websocat；RFC6455 extension；close 码全集；`net.tls.server` HostFn GA；「完整 WSS/WebSocket 产品 GA」。钉 `todo8_overall_soft_close_first_slice`。**宣称** todo 8 soft-close / 项收口 met；**项 8 不再开片**；**不宣称** OUT 任一项「已交付」；**不宣称** 新进度。

**todo 4 Object soft-close / 项收口：** 无新 runtime / 无 HostFn+1；书面冻结：默认字面量 → `Dictionary`；显式 `: Object` 异构袋；方法面 `count`/`isEmpty`/`valueForKey`/`put`/`containsKey`/`keys`/`values`/`remove`（下标读写同语义）。**OUT（项级永久）：** `.merge`；真 `HashMap<Value,Value>`；Object 泛型深化；`object_bag_ga`。钉 `todo4_object_soft_close_first_slice`。**宣称** Object soft-close / 项收口 met；**项 4 不再开片**；**不宣称** OUT 任一项「已交付」；**不宣称** Wave 新进度。

**WSS 营销产品 soft-close（todo 8）：** 无新 runtime / 无 HostFn+1；书面冻结产品地板 SSOT：（1）同端口 `upgradeUpstream` 透传（含 HMR-over-TLS 自动化钉）；（2）`upgradeTerminate` plain + over-TLS（正向代表簇 + close(1002) 主矩阵 + illegal 近界 soft-close）；（3）sgate native HTTPS可承载上述路径。**OUT：** RFC6455 extension；close 码全集 GA；`net.tls.server` / `__syx_tls_serve` HostFn GA；真浏览器 Playwright HMR；全量并发 SLA；`ws_routes` websocat 去 sidecar；「完整 WSS/WebSocket 产品 GA」营销口号。钉 `wss_marketing_product_soft_close_first_slice`（索引既有代表测）。**宣称** WSS 营销产品 soft-close met / 该子轴不再开片追「WSS GA」；**不宣称** 完整 WSS/WebSocket 产品 GA / Playwright / SLA / 删 websocat / 项 8 总收口。

**sgate TLS native （todo 8）：** 无新 HostFn；`mode: native` → `tls_native.sy` 对配置 PEM 做 `Env.set(SYX_HTTP_SERVE_TLS_CERT/KEY)`，`main.sy` 在 `tls.port` 上 `Http.serve`（进程内 rustls，与 stdlib HTTPS 钉同语义）；配置可用 `SYX_SGATE_CONFIG`；保留 `tls.sy` `__syx_tls_serve`/`if false` 诚实占位。钉 `sgate_tls_native_first_slice` / test.sh native。**宣称** sgate TLS native met / 生产 HTTPS 可不依赖 socat；**不宣称** 删除 sidecar 代码 / `ws_routes` 去 sidecar / Playwright / `net.tls.server` HostFn GA / 项 8 总收口（WSS 营销宣称边界见 soft-close）。

**upgradeTerminate illegal close-status 近界采样轴 soft-close（todo 8）：** 无新 runtime；结清 soft 债 + TLS 边沿对齐至 9/990；近界 `N/999−N` 继续 deepen **OUT**：（1）TLS 8/991 fail_closed（钉 `http_serve_https_upgrade_terminate_accept_close_status_8_fail_closed` / `…_991_fail_closed`）；（2）plain 9/990 fail_closed（钉 `http_serve_upgrade_terminate_accept_close_status_9_fail_closed` / `…_990_fail_closed`）；（3）TLS 9/990 close(1002) 对齐（钉 `…_9_close_1002` / `…_990_close_1002`）。**宣称** illegal 近界采样轴 soft-close met / soft **4/4**TLS↔plain 边沿对齐至 9/990；该子轴不再开片；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 项 8 总收口 / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 8/991 收口 + plain 9/990 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 8（钉 `http_serve_https_upgrade_terminate_accept_close_status_8_close_1002`）；（2）TLS close status 991（钉 `…_991_close_1002`）；（3）plain close status 9（钉 `http_serve_upgrade_terminate_accept_close_status_9_close_1002`）；（4）plain close status 990（钉 `…_990_close_1002`）。**宣称** TLS 8/991 收口 + plain 9/990 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 7（钉 `http_serve_https_upgrade_terminate_accept_close_status_7_fail_closed`）；（2）TLS close status 992（钉 `…_992_fail_closed`）；（3）plain close status 8（钉 `http_serve_upgrade_terminate_accept_close_status_8_fail_closed`）；（4）plain close status 991（钉 `…_991_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 7/992 收口 + plain 8/991 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 7（钉 `http_serve_https_upgrade_terminate_accept_close_status_7_close_1002`）；（2）TLS close status 992（钉 `…_992_close_1002`）；（3）plain close status 8（钉 `http_serve_upgrade_terminate_accept_close_status_8_close_1002`）；（4）plain close status 991（钉 `…_991_close_1002`）。**宣称** TLS 7/992 收口 + plain 8/991 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 6（钉 `http_serve_https_upgrade_terminate_accept_close_status_6_fail_closed`）；（2）TLS close status 993（钉 `…_993_fail_closed`）；（3）plain close status 7（钉 `http_serve_upgrade_terminate_accept_close_status_7_fail_closed`）；（4）plain close status 992（钉 `…_992_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 6/993 收口 + plain 7/992 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 6（钉 `http_serve_https_upgrade_terminate_accept_close_status_6_close_1002`）；（2）TLS close status 993（钉 `…_993_close_1002`）；（3）plain close status 7（钉 `http_serve_upgrade_terminate_accept_close_status_7_close_1002`）；（4）plain close status 992（钉 `…_992_close_1002`）。**宣称** TLS 6/993 收口 + plain 7/992 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 5（钉 `http_serve_https_upgrade_terminate_accept_close_status_5_fail_closed`）；（2）TLS close status 994（钉 `…_994_fail_closed`）；（3）plain close status 6（钉 `http_serve_upgrade_terminate_accept_close_status_6_fail_closed`）；（4）plain close status 993（钉 `…_993_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 5/994 收口 + plain 6/993 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 5（钉 `http_serve_https_upgrade_terminate_accept_close_status_5_close_1002`）；（2）TLS close status 994（钉 `…_994_close_1002`）；（3）plain close status 6（钉 `http_serve_upgrade_terminate_accept_close_status_6_close_1002`）；（4）plain close status 993（钉 `…_993_close_1002`）。**宣称** TLS 5/994 收口 + plain 6/993 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 4（钉 `http_serve_https_upgrade_terminate_accept_close_status_4_fail_closed`）；（2）TLS close status 995（钉 `…_995_fail_closed`）；（3）plain close status 5（钉 `http_serve_upgrade_terminate_accept_close_status_5_fail_closed`）；（4）plain close status 994（钉 `…_994_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 4/995 收口 + plain 5/994 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 4（钉 `http_serve_https_upgrade_terminate_accept_close_status_4_close_1002`）；（2）TLS close status 995（钉 `…_995_close_1002`）；（3）plain close status 5（钉 `http_serve_upgrade_terminate_accept_close_status_5_close_1002`）；（4）plain close status 994（钉 `…_994_close_1002`）。**宣称** TLS 4/995 收口 + plain 5/994 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 3（钉 `http_serve_https_upgrade_terminate_accept_close_status_3_fail_closed`）；（2）TLS close status 996（钉 `…_996_fail_closed`）；（3）plain close status 4（钉 `http_serve_upgrade_terminate_accept_close_status_4_fail_closed`）；（4）plain close status 995（钉 `…_995_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 3/996 收口 + plain 4/995 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 3（钉 `http_serve_https_upgrade_terminate_accept_close_status_3_close_1002`）；（2）TLS close status 996（钉 `…_996_close_1002`）；（3）plain close status 4（钉 `http_serve_upgrade_terminate_accept_close_status_4_close_1002`）；（4）plain close status 995（钉 `…_995_close_1002`）。**宣称** TLS 3/996 收口 + plain 4/995 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 2（钉 `http_serve_https_upgrade_terminate_accept_close_status_2_fail_closed`）；（2）TLS close status 997（钉 `…_997_fail_closed`）；（3）plain close status 3（钉 `http_serve_upgrade_terminate_accept_close_status_3_fail_closed`）；（4）plain close status 996（钉 `…_996_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 2/997 收口 + plain 3/996 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 2（钉 `http_serve_https_upgrade_terminate_accept_close_status_2_close_1002`）；（2）TLS close status 997（钉 `…_997_close_1002`）；（3）plain close status 3（钉 `http_serve_upgrade_terminate_accept_close_status_3_close_1002`）；（4）plain close status 996（钉 `…_996_close_1002`）。**宣称** TLS 2/997 收口 + plain 3/996 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）TLS close status 100（钉 `http_serve_https_upgrade_terminate_accept_close_status_100_fail_closed`）；（2）TLS close status 998（钉 `…_998_fail_closed`）；（3）plain close status 2（钉 `http_serve_upgrade_terminate_accept_close_status_2_fail_closed`）；（4）plain close status 997（钉 `…_997_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status TLS 100/998 收口 + plain 2/997 加深：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品 `*_close_1002`：（1）TLS close status 100（钉 `http_serve_https_upgrade_terminate_accept_close_status_100_close_1002`）；（2）TLS close status 998（钉 `…_998_close_1002`）；（3）plain close status 2（钉 `http_serve_upgrade_terminate_accept_close_status_2_close_1002`）；（4）plain close status 997（钉 `…_997_close_1002`）。**宣称** TLS 100/998 收口 + plain 2/997 加深 met；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin（对齐 ）：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 `*_close_1002`）：（1）plain close status 1（钉 `http_serve_upgrade_terminate_accept_close_status_1_fail_closed`）；（2）plain close status 100（钉 `…_100_fail_closed`）；（3）plain close status 998（钉 `…_998_fail_closed`）；（4）TLS close status 1（钉 `http_serve_https_upgrade_terminate_accept_close_status_1_fail_closed`）。**宣称** illegal-status fail_closed soft-twin（对齐 ）met / 四码 soft-twin **4/4**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status 加深第三簇：** 无新 runtime（`reject_illegal_close_status` 已拒 `0..=999`）；产品采样代表点 → close(1002)：（1）plain close status 1（钉 `http_serve_upgrade_terminate_accept_close_status_1_close_1002`）；（2）plain close status 100（钉 `…_100_close_1002`）；（3）plain close status 998（钉 `…_998_close_1002`）；（4）TLS close status 1（钉 `http_serve_https_upgrade_terminate_accept_close_status_1_close_1002`）。**宣称** illegal close-status 加深第三簇 met / 采样 1/100/998（+ TLS 1）；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin 收口：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 TLS `*_close_1002`）：（1）TLS close status 0（钉 `http_serve_https_upgrade_terminate_accept_close_status_0_fail_closed`）；（2）TLS close status 500（钉 `…_500_fail_closed`）。**宣称** illegal-status fail_closed soft-twin 收口 met / illegal fail_closed **10/10**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin 第二簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；镜像 ）：（1）TLS close status 1015（钉 `http_serve_https_upgrade_terminate_accept_close_status_1015_fail_closed`）；（2）TLS close status 999（钉 `…_999_fail_closed`）；（3）plain close status 0（钉 `http_serve_upgrade_terminate_accept_close_status_0_fail_closed`）；（4）plain close status 500（钉 `…_500_fail_closed`）。**宣称** illegal-status fail_closed soft-twin 第二簇 met / illegal fail_closed **8/10**；**不宣称** close 码全集 GA / illegal fail_closed 全集 / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate illegal close-status fail_closed soft-twin 第一簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；镜像 ）：（1）plain close status 1006（钉 `http_serve_upgrade_terminate_accept_close_status_1006_fail_closed`）；（2）plain close status 1015（钉 `…_1015_fail_closed`）；（3）plain close status 999（钉 `…_999_fail_closed`）；（4）TLS close status 1006（钉 `http_serve_https_upgrade_terminate_accept_close_status_1006_fail_closed`）。**宣称** illegal-status fail_closed soft-twin 第一簇 met / illegal fail_closed **4/10**；**不宣称** close 码全集 GA / WSS / WebSocket / `net.tls.server` 产品 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate-over-TLS fail_closed 名表第四/末簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 plain `*_fail_closed` + / RSV 场景）：（1）FIN=0 close（钉 `http_serve_https_upgrade_terminate_accept_fin0_close_fail_closed`）；（2）FIN=0 close mid-fragment（钉 `…_accept_fin0_close_during_fragment_fail_closed`）；（3）reserved opcode 0x3（钉 `…_accept_reserved_opcode_0x3_fail_closed`）；（4）RSV text（钉 `…_accept_rsv_text_fail_closed`）。**宣称** fail_closed 名表第四/末簇 met / TLS fail_closed **16/16**（soft-twin）；**不宣称** WSS / WebSocket / `net.tls.server` 产品 GA / close 码全集 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate-over-TLS fail_closed 名表第三簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 plain `*_fail_closed` + 场景）：（1）close reason non-UTF-8（钉 `http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_fail_closed`）；（2）unsolicited pong（钉 `…_accept_unsolicited_pong_fail_closed`）；（3）illegal status 1004（钉 `…_accept_close_status_1004_fail_closed`）；（4）fragmented text non-UTF-8（钉 `…_accept_fragmented_text_non_utf8_fail_closed`）。**宣称** fail_closed 名表第三簇 met / TLS fail_closed **12/16**；**不宣称** fail_closed TLS 名表全集 / WSS / WebSocket / `net.tls.server` 产品 GA / close 码全集 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。

**upgradeTerminate-over-TLS fail_closed 名表第二簇：** 无新 runtime；名表 soft-twin（行为仍 close(1002)；对齐 plain `*_fail_closed` + 场景）：（1）oversized ping（钉 `http_serve_https_upgrade_terminate_accept_oversized_ping_fail_closed`）；（2）close len=1（钉 `…_accept_close_len1_fail_closed`）；（3）illegal status 1005（钉 `…_accept_close_status_1005_fail_closed`）；（4）text non-UTF-8（钉 `…_accept_text_non_utf8_fail_closed`）。**宣称** fail_closed 名表第二簇 met / TLS fail_closed **8/16**；**不宣称** fail_closed TLS 名表全集 / WSS / WebSocket / `net.tls.server` 产品 GA / close 码全集 GA；**不宣称** 去 sidecar / Playwright / 「新 abort-only 语义」。




**同端口 HMR 路径：** 客户端 Upgrade **只打 serve 口**；handler 返回 `upgradeUpstream` 指向本机 Vite-like WS 上游；101 + `Sec-WebSocket-Accept` 透传 + 掩码 text-frame 往返已钉（`http_serve_upgrade_proxy_hmr_path_same_port_roundtrip`）。**宣称** 同端口 HMR **产品路径**（自动化钉）met；**不宣称** 真浏览器 / Playwright HMR GA；**不宣称** 帧终结 / serve 自算 Accept。产品用法见 [tools/sgate/README.md](../tools/sgate/README.md)（`:8080`→`:9999`）。

**HTTP Upgrade → `upgradeUpstream`（进程内字节隧道）：** handler 可返回带 `upgradeUpstream`（上游 `http: / host:port/path`）的 Object；`Http.serve` 拨号上游、转发握手，收到 `101` 后双向拷贝字节。WebSocket 帧（含客户端掩码 text frame）**原样透传**；上游 101 响应头（含 `Sec-WebSocket-Accept`）**原样转发**（serve **不**按 Key 自算 Accept）。上游非 101 时回写客户端 `status.max(400)`（如 500→500、200→400）且**不开**隧道。缺 `upgradeUpstream`、空串 `upgradeUpstream`（视同缺字段）、handler 调用失败、拨号/写请求/读头失败（隧道前）→ 客户端 **502**（body 含 `Bad Gateway` 或字段名）。钉测：`http_serve_upgrade_proxy_roundtrip` / `http_serve_upgrade_proxy_masked_text_frame_roundtrip` / `http_serve_upgrade_proxy_non_101_maps_status_max_400` / `http_serve_upgrade_proxy_relays_sec_websocket_accept` / `http_serve_upgrade_proxy_missing_upgrade_upstream_returns_502` / `http_serve_upgrade_proxy_dial_fail_returns_502` / `http_serve_upgrade_proxy_empty_upgrade_upstream_returns_502` / `http_serve_upgrade_proxy_handler_err_returns_502` / `http_serve_upgrade_proxy_hmr_path_same_port_roundtrip`。产品用法见 [tools/sgate/README.md](../tools/sgate/README.md)。**非** WSS 产品 GA；**非** 真浏览器 HMR GA（HMR-over-HTTPS 见 ；进程内终结见 `upgradeTerminate`）。

**HTTP timeout / TLS 负例（`M2-S4-02b`）：** `Http.fetch`/`request` 的 `timeout_ms` 真超时 → **R012**；self-signed / bad cert fail-closed，禁 `status=0` 成功。`Http.get` **无** per-call timeout。钉测：`m2_s4_02b_client_timeout_tls`。**非** skip-verify API；**非** server TLS GA；**非** `EC-M2-4-Z`。

Http* HostFn availability 已在能力矩阵诚实回写（`ec_m2_4_http_availability`；**非** EC-M2-4 全量 **非** TLS server GA）。native/WASM 列诚实加深已钉（`ec_m2_4_http_native_wasm_matrix`）；HttpMakeResponse llvm GA 首档已钉（`ec_m2_4_http_native_wasm_ga`；`HttpMakeResponse` `llvm=real` + native AOT 执行证据）；HttpResponseStatus llvm GA 二档已钉（`ec_m2_4_http_native_wasm_ga2`；`HttpResponseStatus` `llvm=real` + native AOT MakeResponse+status → `"200"`）；HttpResponseText llvm GA 三档已钉（`ec_m2_4_http_native_wasm_ga3`；`HttpResponseText` `llvm=real` + native AOT MakeResponse+text → `"ok"`；其余 Http* 仍 `llvm=partial` / `wasm=host-assisted` / 产品 `native=partial`；**非** 全量 HTTP GA **非** wasm GA **非** TLS server GA **非** EC-M2-4 全量退出）。`Http.response`×N Instant 微基准在测试层（`ec_m2_4_http_microbench`；**非** perf SLA **非** EC-M2-4 全量退出）。`Http.response`×N Criterion 正式 benchmark 首档在 `cargo bench`（`ec_m2_4_http_criterion`；**非** absolute SLA gate **非** EC-M2-4 全量退出）。

`TcpStream.connect` / `read` / `write` 抛 `NetworkError`。`host_tcp_handle_release` / `TcpStream.closeHandle` 从 VM `TCP_STREAMS` 真 drop；存活计数 `tcp_stream_live_count` 仅在测试/诊断层，**不是**应用 API。Tcp* HostFn availability 已在能力矩阵诚实回写（`ec_m2_4_tcp_availability`；**非** EC-M2-4 全量 **非** TLS GA）。native/WASM 列诚实加深已钉（`ec_m2_4_tcp_native_wasm_matrix`）；TcpConnect llvm GA 首档已钉（`ec_m2_4_tcp_native_wasm_ga`；`TcpConnect` `llvm=real` + native AOT refuse → `-1` 执行证据）；TcpHandleRelease llvm GA 二档已钉（`ec_m2_4_tcp_native_wasm_ga2`；`TcpHandleRelease` `llvm=real` + native AOT loopback connect+release → `"true"`）；TcpWrite llvm GA 三档已钉（`ec_m2_4_tcp_native_wasm_ga3`；`TcpWrite` `llvm=real` + native AOT loopback connect+write `"hi"` → `"2"`）；四切片 native/WASM GA 三档收口评估已签署（`ec_m2_4_four_slice_native_wasm_ga3_closeout` / roadmap `EC-M2-4-G`；**非** 全量 EC-M2-4 **非** `EC-M2-4-Z`）；TcpRead llvm GA 四档已钉（`ec_m2_4_tcp_native_wasm_ga4` / roadmap `EC-M2-4-H`；`TcpRead` `llvm=real` + native AOT loopback connect+read → `"hi"`； 四 HostFn 齐）；`wasm=host-assisted` / 产品 `native=partial`；**非** 全量 Tcp GA **非** TLS GA **非** wasm GA **非** `EC-M2-4-Z` **非** EC-M2-4 全量退出）。`TcpStream.connect` 拒连×N Instant 微基准在测试层（`ec_m2_4_tcp_microbench`；**非** perf SLA **非** EC-M2-4 全量退出）。`TcpStream.connect` 拒连×N Criterion 正式 benchmark 首档在 `cargo bench`（`ec_m2_4_tcp_criterion`；16MB 栈；**非** absolute SLA gate **非** EC-M2-4 全量退出）。

`Process.spawn` / `Process.wait` 管理子进程句柄生命周期（`wait` 从 VM `CHILD_TABLE` 移除；失败哨兵 `-1`）。**ProcessBuilder / Child（`M2-S6-01`）：** `ProcessBuilder.create` → `cwd` / `env`（overlay）→ `spawn` → `Child.wait` / `kill` / `waitTimeout`；**wait/kill 后再操作必须失败**；超时 `waitTimeout` 会 kill 并 throws。`os.process.builder` implemented。钉测：`m2_s6_01_process_builder`。**ChildPipe（`M2-S6-02b`）：** `Child.stdout`/`stderr`/`stdin` take → `ChildPipe.readChunk`（空=EOF）/`write`/`close`；**关闭后再读写必须失败**；未 take 的端在 wait 时内部 drop（防阻塞）。`os.process.stdio` implemented。钉测：`m2_s6_02b_child_stdio_pipe`。**Child.signal（`M2-S6-03`）：** `Child.signal(handle, name)` 接受 `TERM`/`INT`/`KILL`（可选 `SIG` 前缀，大小写不敏感）；**发信号后句柄仍可 `Child.wait`**（≠ `Child.kill`：强制 kill + 移除 + wait）。Unix：`libc::kill`；Windows：仅 `KILL`（`TERM`/`INT` throws unsupported）。`os.process.signal` implemented。钉测：`m2_s6_03_child_signal`。**非** 全量 POSIX signal GA；**非** `Process.signalSelf`；**非** async select Pipe GA；**非** `EC-M2-4-Z`。存活计数 `process_child_live_count` 仅在测试/诊断层（ EC-M2-3 首钉），**不是**应用 API。EC-M2-3 四类 live-count 首钉已收口评估签署；总闸「泄漏=0」仍开，**不是**全量 EC-M2-3 / M2 退出。EC-M2-4 四切片 availability + Instant microbench 首档已收口评估签署（`ec_m2_4_four_slice_closeout`）；总闸全量仍开，**不是** EC-M2-4 全量 / M2 退出。EC-M2-4 四切片 Criterion 正式 benchmark 首档已收口评估签署（`ec_m2_4_four_slice_criterion_closeout`；≠ absolute SLA gate）；总闸全量仍开，**不是** EC-M2-4 全量 / M2 退出。EC-M2-4 四切片 native/WASM matrix 首档已收口评估签署（`ec_m2_4_four_slice_native_wasm_matrix_closeout`；≠ native/WASM GA）；总闸全量仍开，**不是** EC-M2-4 全量 / M2 退出。EC-M2-4 四切片 native/WASM GA 首档已收口评估签署（`ec_m2_4_four_slice_native_wasm_ga_closeout`；PathJoin / HttpMakeResponse / SqlOpen / TcpConnect llvm；≠ 全量 native/WASM GA）；总闸全量仍开，**不是** EC-M2-4 全量 / M2 退出。EC-M2-4 四切片 native/WASM GA 二档已收口评估签署（`ec_m2_4_four_slice_native_wasm_ga2_closeout`；PathExtension / HttpResponseStatus / SqlClose / TcpHandleRelease llvm；≠ 全量 native/WASM GA）；总闸全量仍开，**不是** EC-M2-4 全量 / M2 退出。EC-M2-1 五切片（file+Path / HTTP client/server / SQLite / Path deep）已收口评估签署（`ec_m2_1_five_slice_closeout`）；总闸全量仍开，**不是** Path GA / M2 退出。

示例：`examples/http_tool/`（ client）；`examples/http_server_tool/`（ server；默认端口 18765）。另见 `examples/httpclient_demo.sy` / `httpserver_demo.sy`。

### Sql（SQLite）

`import Sql` 后使用多连接 facade（`throws SqlError`）：

| 方法 | 说明 |
|------|------|
| `Sql.open(dsn:)` | 打开连接（`":memory:"` 或文件路径）→ `Int` connId |
| `Sql.execOn(connId:, sql:)` | 在指定连接执行 |
| `Sql.queryOn(connId:, sql:)` | 查询 → **`Array` of `SqlRow`**（可用 `.count`） |
| `Sql.close(connId:)` | 关闭显式打开的连接 |

**Typed Row（`M2-S8-02b`）：** 命名类型 `SqlRow`（禁糊墙 Object 成功）。`SqlRow.getInt`/`getString`/`getFloat`/`getBlob`/`isNull`/`columnNames`（缺列或类型错 throws `SqlError`）。`data.sqlite.row` implemented。钉测：`m2_s8_02b_typed_row`。**裸 SQL hatch：** `Sql.query`/`queryOn` **仍接受裸 SQL 字符串**（注入风险自负）；≠ typed query builder；≠ Orm GA。**非** `EC-M2-4-Z`。

**Connection pool（`M2-S8-01c` + `M2-S8-03b`）：** `Sql.poolCreate(dsn, maxSize)` → `poolAcquire` → 用 `*On` → `poolRelease`（归还 free list，**不** close）→ `poolClose`（关闭池内全部连接）。`maxSize` 耗尽时 `poolAcquire` **立即 throws**（非阻塞等待）。与裸 `Sql.open`/`close` 并存；池外连接不进 pool。`data.sqlite.pool` implemented。钉测：`m2_s8_01c_connection_pool`、`m2_s8_03b_pool_txn`。**事务门槛：** `poolRelease` 在连接仍有 open txn（`beginTransactionOn` 未 commit/rollback）时 **throws**（facade code 20）；同文件双 checkout 并发 `BEGIN IMMEDIATE` 至多一条成功，第二条 begin → busy（code 15）；`poolClose` 硬关池内 conn 并清除 txn 记账，`sql_conn_live_count` 不保留池 conn。**非** 阻塞/超时 acquire；**非** `EC-M2-4-Z`。

Sql* HostFn availability 已在能力矩阵诚实回写（`ec_m2_4_sql_availability`；**非** EC-M2-4 全量 **非** Orm GA）。native/WASM 列诚实加深已钉（`ec_m2_4_sql_native_wasm_matrix`）；SqlOpen llvm GA 首档已钉（`ec_m2_4_sql_native_wasm_ga`；`SqlOpen` `llvm=real` + native AOT `:memory:` 执行证据）；SqlClose llvm GA 二档已钉（`ec_m2_4_sql_native_wasm_ga2`；`SqlClose` `llvm=real` + native AOT open+close → `"ok"`）；SqlExecOn llvm GA 三档已钉（`ec_m2_4_sql_native_wasm_ga3`；`SqlExecOn` `llvm=real` + native AOT `:memory:` CREATE+INSERT → `"1"`；其余 Sql* 仍 `llvm=partial` / `wasm=host-assisted` / 产品 `native=partial`；**非** 全量 Sql GA **非** Orm GA **非** wasm GA **非** EC-M2-4 全量退出）。`:memory:` + `Sql.execOn` INSERT×N Instant 微基准在测试层（`ec_m2_4_sql_microbench`；**非** perf SLA **非** EC-M2-4 全量退出）。`:memory:` + `Sql.execOn` INSERT×N Criterion 正式 benchmark 首档在 `cargo bench`（`ec_m2_4_sql_criterion`；16MB 栈；**非** absolute SLA gate **非** EC-M2-4 全量退出）。

示例：`examples/sql_tool/`（ EC-M2-1 SQLite 服务切片）。存活计数 `sql_conn_live_count` 仅计显式连接（id≥1），在测试/诊断层（ EC-M2-3 首钉），**不是**应用 API。

### String 三视图（`M2-S2-01`）

默认字符模型是 **grapheme（字素簇）** `Character`：

| 视图 | API | 说明 |
|------|-----|------|
| Grapheme | `.count` / `s[i]` / `.substring` / `for-in` | 整数下标为 grapheme offset（见下） |
| Unicode scalar | `String.unicodeScalarCount(text:)` / `String.unicodeScalars(text:)` | 与 grapheme 可分叉（如 ZWJ 家庭 emoji） |
| UTF-8 bytes | `String.utf8Count(text:)` / `String.utf8Bytes(text:)` | 字节长度与 `Array<Int>`（0–255） |

`Character.isSingleScalar` 按 **scalar count == 1** 判断（非 grapheme `.count`）。钉测：`m2_s2_01_string_three_view`。**非** normalization / XID / IANA。

### String 下标复杂度（`M2-S2-02`）

`s[i]` 与 `.substring(start, end)` 使用 **Int grapheme offset**（不是字节偏移，也不是 Swift opaque `String.Index`）。**单次随机访问为 O(n)**（实现经字素簇迭代 `.nth` / `.skip`，无隐藏 O(1) 缓存）。顺序扫描请用 `for-in`（按 grapheme）。钉测：`m2_s2_02_string_index_honesty`。**非** opaque Index GA。

### String normalization / case folding（`M2-S2-03`）

| API | 说明 |
|-----|------|
| `String.normalizedNFC(text:)` | Unicode **NFC** |
| `String.normalizedNFD(text:)` | Unicode **NFD** |
| `String.caseFolded(text:)` | Unicode **default/full case folding**（如 `ß`→`ss`） |
| `.lowercased()` / `.uppercased()` | Unicode **simple case mapping**（≠ `caseFolded`；≠ locale/Turkish） |

**非** NFKC/NFKD / IANA。钉测：`m2_s2_03_string_normalization`。标识符 XID 见下节。

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

钉测：`m2_s1_02_collection_mutation`。**非** Dict 下标插入 GA / 泛型全集。Deque → `collections.deque`。

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

`data`/`collections.deque` implemented。钉测：`m2_s1_03_deque`。**非** Heap/PriorityQueue；**非** lazy sequence；**非** `Deque<T>` 泛型；**非** for-in / makeIterator GA；**非** `EC-M2-4-Z`。

### 集合 HOF feature-off（`M2-S1-04`）

`Array.map` / `filter` / `reduce`（含 `__syx_array_*` CallHost）在非法 receiver、缺 Closure、或 interpreter feature-off 时 **fail-closed**：诊断码 **`U001` / unsupported**，**不得**以空数组 `[]` 或静默 `Nil` 当作成功。

```syx
// 非法：第二参不是 Closure → 运行失败（U001），不会打印 []
__syx_array_map([1, 2, 3], 0)
```

钉测：`m2_s1_04_native_feature_off_no_silent`。**非** String 三视图 GA / llvm HOF GA。Deque → `collections.deque`。

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

编译器自动生成 `_init` 与 `_describe`；`@Equatable`/`@Hashable`/`@Codable` 为 L1 证明宏（stored 属性 `data class`）。详见 [books/language-guide/19-macros.md](../books/language-guide/19-macros.md)。

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

可选字段（`T?`）缺键可 decode。**非** CodingKeys DSL。钉测：`m2_s7_02_json_encoder_decoder_field_paths`。

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

**非** CodingKeys DSL；**非** File 真 stream（→ `M2-S3-03`）；**非** HTTP 流式 body（→ `M2-S4-04`）；**非** IANA（→ `M2-S5-02`）。钉测：`m2_s7_03_json_stream_ndjson`。

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

 由 `syx-macro` 统一展开。详见 [books/language-guide/19-macros.md](../books/language-guide/19-macros.md) · [](adr/architecture-decisions.md) · 文档指纹 `macro-docs-v15:m23-service-v14-honesty`。

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

**主业务并发 API：** `Task` / `TaskGroup` / `async`/`await` / `Actor` / `Channel` / `choose`，以及 M3 表面（`taskGroupCancel`、`setDeadlineMs`、`taskLocal*`、`actorReentrancyPolicy`、Channel close/backpressure/fairness、`Mutex`/`Semaphore`）。叙事教程见 [books/language-guide/18-concurrency.md](../books/language-guide/18-concurrency.md)。

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

Phase 49+ 动态扩展见 [books/language-guide/33-runtime-extensions.md](../books/language-guide/33-runtime-extensions.md) 与 [架构决议 · Host 插件](adr/architecture-decisions.md#host-插件)。

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libplugin.so
syx run app.sy
```

`syx.toml` 可选 `[plugins].paths`；索引锁 `syx-plugin-lock.json`。验签：`SYX_HOST_PLUGIN_VERIFY=1`。

现行 ABI v1 为 UTF-8 字符串进出（输出约 8 KiB）。**：** P-SURF packed-SYX padReflect 四向（`padRowsBeforeReflect`/`padRowsAfterReflect`/`padColsBeforeReflect`/`padColsAfterReflect`；≠ 产品 GA）；**：** P-SURF packed-SYX padConstant 四向（`padRowsBeforeConstant`/`padRowsAfterConstant`/`padColsBeforeConstant`/`padColsAfterConstant`；≠ 产品 GA）；**：** P-SURF packed-SYX pad 边框（`padRowsBefore`/`padRowsAfter`/`padColsBefore`/`padColsAfter`；≠ 产品 GA）；**：** P-SURF packed-SYX 取舍/丢弃（`takeRows`/`dropRows`/`takeCols`/`dropCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 标量掩码（`lessThanScalar`/`notEqualToScalar`/`lessEqualToScalar`/`greaterEqualToScalar`；≠ 产品 GA）；**：** P-SURF packed-SYX 差分加深/标量掩码（`maxAbsDiff`/`hamming`/`equalToScalar`/`greaterThanScalar`；≠ 产品 GA）；**：** P-SURF packed-SYX 差分/整数损失（`absDiff`/`squaredDiff`/`manhattan`/`sumSquaredDiff`；≠ 产品 GA）；**：** P-SURF packed-SYX 硬激活加深（`leakyRelu`/`hardTanh`/`heaviside`/`relu6`；≠ 产品 GA）；**：** P-SURF packed-SYX clip/threshold （`clampMin`/`clampMax`/`relu`/`threshold`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴前缀（`cummaxRows`/`cummaxCols`/`cummin`/`cummax`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴前缀（`diffRows`/`diffCols`/`cumminRows`/`cumminCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴前缀（`cumsumRows`/`cumsumCols`/`cumprodRows`/`cumprodCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴归约（`varRows`/`varCols`/`rangeRows`/`rangeCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴归约（`argminCols`/`argmaxRows`/`stdRows`/`stdCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴归约（`meanCols`/`prodRows`/`argminRows`/`argmaxCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴归约（`meanRows`/`prodCols`/`maxRows`/`minCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 轴归约（`sumRows`/`sumCols`/`minRows`/`maxCols`；≠ 产品 GA）；**：** P-SURF packed-SYX 掩码逻辑（`logicalXor`/`countTrue`/`noneTrue`/`fillWhere`；≠ 产品 GA）；**：** P-SURF packed-SYX 掩码逻辑（`logicalAnd`/`logicalOr`/`anyTrue`/`allTrue`；≠ 产品 GA）；**：** P-SURF packed-SYX 掩码选择/同形填充（`zerosLike`/`filledLike`/`choose`/`logicalNot`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素比较（`notEqual`/`less`/`lessEqual`/`greaterEqual`；≠ 产品 GA）；**：** P-SURF packed-SYX 外积/同形/比较（`outer`/`onesLike`/`equal`/`greater`；≠ 产品 GA）；**：** P-SURF packed-SYX 前缀累加/差分（`cumsum`/`cumprod`/`diff`/`countNonzero`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素 log10/Γ/lerp/dot （`log10`/`tgamma`/`lerp`/`dot`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素熔合/指数对数（`mulAdd`/`exp2`/`log2`/`cbrt`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素浮点二元（`hypot`/`pow`/`fmod`/`remainder`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素二元极值/符号（`maximum`/`minimum`/`copysign`/`reciprocal`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素一元（`floor`/`ceil`/`round`/`trunc`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素除/标量广播（`div`/`addScalar`/`subScalar`/`divScalar`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素一元（`sign`/`sqrt`/`squared`/`clamp`；≠ 产品 GA）；**：** P-SURF packed-SYX 几何置换（`rot180`/`swapRows`/`swapCols`/`roll`；≠ 产品 GA）；**：** P-SURF packed-SYX 几何翻转（`flipud`/`fliplr`/`rot90`/`antidiagonal`；≠ 产品 GA）；**：** P-SURF packed-SYX 结构复制（`tile`/`kron`/`tril`/`triu`；≠ 产品 GA）；**：** P-SURF packed-SYX 归约/范数（`min`/`max`/`argmin`/`argmax`/`frobeniusNorm`；≠ 产品 GA）；**：** P-SURF packed-SYX 形状拼装（`reshape`/`flatten`/`hstack`/`vstack`/`fromDiagonal`；≠ 产品 GA）；**：** P-SURF packed-SYX 切片/访问（`withAt`/`row`/`col`/`diagonal`/`trace`/`abs`；≠ 产品 GA）；**：** P-SURF packed-SYX 元素算术（`+`/`-`/`negated`/`hadamard`/`filled`/`mean`；≠ 产品 GA）；**：** P-SURF packed-SYX 形状通用化（任意形 `eye`/`ones`/`sum`/`transposed`/`copy` + `scale`；≠ 产品 GA）；**：** P-SURF packed-SYX 便利/stats （`at`/`transposed`/`copy`/`eye`/`ones`/`sum`；≠ 产品 GA）；**：** P-SURF solve/packed/shape 方法族（`Mat.solve` / `packed` / shape / `from`；≠ 产品 GA）；**：** P-SURF zeros 语言面（`Mat.zeros` 接 in-process `__syx_plugin_faer_zeros`；≠ 产品 GA）；**：** P-SURF 独立新 Mat （`let c = a * b` 独立对象，LHS packed 不改；≠ 产品 GA）；**：** P-FAER cdylib F64LE packed zeros （`__syx_plugin_faer_cdylib_zeros`；≠ Mat 语言面）；**：** P-SURF operator 返回值（`let c = a * b` 不破坏右操作数；≠ 产品 GA）；**：** P-SURF Mat 独立包落地（`import Linalg` / `class Mat` HANDLE wrap；≠ 产品 GA）；**：** cdylib F64LE packed solve （`__syx_plugin_faer_cdylib_solve`）；**：** HANDLE×cdylib drop （`__syx_plugin_faer_cdylib_hdrop` + alloc `drop_fn`）；**：** P-SURF 书面 Mat 约束（HANDLE 包一层 / `operator func` / 禁并入 math.sy；≠ 落地）；**：** HANDLE×cdylib to/from （`__syx_plugin_faer_cdylib_hfrom` / `_hto`）；**：** HANDLE×cdylib solve （`__syx_plugin_faer_cdylib_hsolve`）；**：** 句柄 borrow （alloc borrow/pin + `__syx_plugin_faer_cdylib_hmul`）；**：** P-FAER cdylib 装载（站外 `plugins/faerlin` dlopen + F64LE mul）；**：** P-FAER solve （in-process `__syx_plugin_faer_solve`；dense LU）；**：** P-FAER 施工（`__syx_plugin_faer_from` / `_zeros` / `_mul` / `_to`；句柄带 shape）；**：** ABI v2 句柄表（`SYX_PLUGIN_BUF_HANDLE` + `__syx_plugin_v2probe_pin` / `_sumh` / `_drop`）； 多槽 iov + realloc 仍真； 单槽 f64 仍真；`P-FAER` / `P-ABI-v2` / `P-SURF` 仍 `[ ]`。数值核走插件协议 + faer，不加 `HostFnId`、不改语言。详 [`plan/topics/plugin-abi-v2-linalg.md`](../plan/topics/plugin-abi-v2-linalg.md)。**非** faer 产品 GA、**非** Mat、**非** ABI v2 完成、**非** 句柄 GA、**非** F-08、**非** Math 扩面、**非** 四后端插件 GA。

---

## 工具链

### Embed API（Phase 14–15）

Phase 14+ Embed API（`syx_embed_v2_*`）：沙箱 / 超时 / 内存与 RSS 预算 / capability 白名单。详见 [架构决议](adr/architecture-decisions.md)。

- **v1（冻结）：** `syx_embed_v1_compile_run` / `syx_embed_v1_free_result`
- **v2：** `syx_embed_v2_compile` / `syx_embed_v2_run` / `syx_embed_v2_free`

### 格式化 — `syx format`

格式化项目入口文件（`syx.toml` 中 `main` 指定）：

```bash
syx format .
```

**C-06（––1176）：** `syx format --check [path]` 递归 dry-run `.sy`——断言 `format(format(x)) == format(x)`；会改写或 F001 → 非零退出且**不写盘**。：Index/Dict、Class/Init、Extension/Protocol、Protocol access/继承诚实、Actor/`deinit`、Enum/TypeAlias、`data class`、`final class`、`global actor`、Subscript get-only/get+set、stored willSet/didSet、computed get-only、`lazy var`、computed get+set、`override var`、`operator func`、`#macro`（User 空/非空 decl_body + BuiltIn/External）、模块级/表达式/语句 `#name(args)`、模块顶层语句、非空 `decl_body`、`throw`、`try`/`catch`、闭包 `{ (params) in … }`（不重建 `-> Ret`）、`await`、`<expr>!` force-unwrap、`as`/`as?`/`is`、`T.self`、`?.`、`..<`/`...`、`switch` 字面 case、`if let`、`guard let`（先 else 后 body）、`repeat {…} while`、`if case .variant[(let x)]`。**：** `if case (a, b) = subject { … }`（`Expr::TuplePattern`；IfCase 收 VariantPattern|TuplePattern；TaskBlock → F001）；`format/tuple_pattern_mvp` 绿；corpus `would_change=0`（自然 Δ≈0；MVP +5）；gre … ≠ C-06 `[x]`。钉测：`c06_fmt_*` / `c06_fmt_check_*`。

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

- **终端模式**：进入 `(debug)` 提示符，可 list/step/run/breakpoint（见 [12_BackendDebuggingGuide.md](12_BackendDebuggingGuide.md)）
- **DAP 模式**：stdin/stdout 走 `Content-Length` 帧协议，由 `syx_debug::run_dap_server()` 实现；VS Code 需 `launch.json` 中 `"type": "syx"`，详见 [12_BackendDebuggingGuide.md — VS Code 图形调试](12_BackendDebuggingGuide.md#vs-code-图形调试dap)

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

格式规范见 [SYXBC Text v1](syxbc-text-v1.md)、[SYX MIR Text v1](syx-mir-text-v1.md)。`llvm` 与 `syx build --native` 的 IR 发射路径一致。`clif` 仅用于实验 JIT / 历史对比（Cranelift AOT 已移除）。

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
| **LLVM**（sole） | `syx build --native` | 生产 native 路径；MIR → LLVM → 链接 `syx-rt` 生成 `app`；parity 见 [10_BackendParity.md](10_BackendParity.md) |

Cranelift **AOT 已移除**。实验 JIT（Suite-1）见 `SYX_JIT_TIER2` / [implementation-status](../books/shared/implementation-status.md)，与 `--native` 无关。

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
- **GUI SynUI v10（Phase 133 · partial GA）：** `import UI` / `packages/ui` 走 `__syx_synui_*` host 桥（stack · scrollView Signal bind · Web/Mobile 布局加深 · v9 继承 image Signal bind · divider · Web/Mobile 控件加深 · textArea/list Signal bind · spacer · picker/progress bind · desktop run Column+Scroll+rekey · toggle/radio Signal bind · Signal↔widget bind · desktop onClick 真接线需 HOLD/`--gui-visible`/`SYX_SYNUI_RUN=1` · checkbox/slider · `row`/`textField`/`column` · structural `renderFingerprint` CI）。需同级 checkout `../synui`（CI sibling gate）。旧 `syx-gui` / egui/skia **已退役**。**不得**称 wgpu HeadlessRenderer GA / 全控件库 / Web/Mobile **SYX language** 全集 GA（→ **134+**）。
- **GUI 报告：** `syx doctor --gui-backend-report` 打印 backend 指纹（SynUI path-dep / host-bridge）
- **Codable L11（Phase 94）**：纯 protobuf wire，与 L10 `L10:` 前缀不兼容

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

- [`crates/syx-compiler-wasm`](../crates/syx-compiler-wasm)：产出 `compiler.wasm`（**C ABI cdylib**，**无** wasm-bindgen），导出 `syx_compile_source_to_wasm_bytes`（FrontendPipeline → MIR → WASM bytes；注入 `browser_prelude`）以及 guest host 桥 `syx_guest_mir_exec` / `syx_guest_end_fn` / `syx_guest_take_catch_pc` / `syx_guest_get_return`
- [`examples/wasm_web/`](../examples/wasm_web/)：页面 `fetch('./compiler.wasm')` 后编译用户源码；guest 的 `syx.*` AOT 导入接到编译器内 `MirRuntime`；AOT 导出的 `main` 为 **exit code 0**，页面用 `syx_guest_get_return` 展示 RT 返回值

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

更细说明见 [`examples/wasm_web/README.md`](../examples/wasm_web/README.md) 与 [`crates/syx-compiler-wasm/README.md`](../crates/syx-compiler-wasm/README.md)。

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

每个测试文件独立编译运行；失败非零退出。C-08 窄收官：JSON + JUnit + filter **`[x]`**；已支持递归 discovery、`import Testing` 断言、无 `@Test` 时 `Testing.test` 登记闭包在入口结束前执行（JSON 用例 `{文件}.{name}`）、以及 `@Test async` + 可完成 `await` → JSON pass。钉测：`syx_test_async_await_assert_eq_passes`。教程见 [books/language-guide/34-testing.md](../books/language-guide/34-testing.md)。**≠** Testing auto-import；**≠** 完整 async test matrix。详见 [plan/topics/syx-test.md](../plan/topics/syx-test.md)。

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

PR CI 对四 Criterion 用短采样；nightly 跑 `compile_bench`。C-09 **`[x]`**（窄 bench）；≠ coverage gate / fuzz / property / SLA。详见 [plan/topics/syx-bench.md](../plan/topics/syx-bench.md)。

### 覆盖率入口（C-09 residual · stub + deepen）

产品 CLI 与等价 cargo（需已装 `cargo-llvm-cov`；窄入口 **claimed** ；**非** CI % 闸 **非** coverage % GA）：

```bash
syx cov

# 等价
cargo llvm-cov -p syx-driver --test native --no-report
```

详见 [plan/topics/syx-coverage.md](../plan/topics/syx-coverage.md) 与 [docs/lang/c09-syx-coverage-stub-first-slice.md](lang/c09-syx-coverage-stub-first-slice.md)。

### Fuzz 入口（C-09 residual · CI + / 维护）

产品 CLI 与等价 cargo（`fuzz/` scaffold **已交**；PR CI **有界钉** `-runs=1`； C-09 维护 + 平台维护地板 **已签**；**≠** scaffold GA **≠** 无界 fuzz）：

```bash
syx fuzz

# 等价（需本机 cargo-fuzz）
cargo fuzz run syx_smoke

# 有界 smoke（PR CI 硬门；）
cargo fuzz run syx_smoke -- -runs=1
```

详见 [plan/topics/syx-fuzz.md](../plan/topics/syx-fuzz.md)、[docs/lang/c09-syx-fuzz-stub-first-slice.md](lang/c09-syx-fuzz-stub-first-slice.md)、[docs/lang/c09-syx-residual-maintenance-first-slice.md](lang/c09-syx-residual-maintenance-first-slice.md) 与 [docs/lang/platform-residual-maintenance-first-slice.md](lang/platform-residual-maintenance-first-slice.md)。

### Property 入口（C-09 residual · CI + / 维护）

产品 CLI 与等价 cargo（`property_smoke` harness **已交**；PR CI **显式钉**； C-09 维护 + 平台维护地板 **已签**；**≠** proptestest GA **≠** `c09_property_ci_ga`）：

```bash
syx prop

# 等价
cargo test -p syx-driver --test property_smoke
```

详见 [plan/topics/syx-property.md](../plan/topics/syx-property.md)、[docs/lang/c09-syx-property-stub-first-slice.md](lang/c09-syx-property-stub-first-slice.md)、[docs/lang/c09-syx-residual-maintenance-first-slice.md](lang/c09-syx-residual-maintenance-first-slice.md) 与 [docs/lang/platform-residual-maintenance-first-slice.md](lang/platform-residual-maintenance-first-slice.md)。

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
| [examples/hello.sy](../examples/hello.sy) | 最小 Hello World |
| [examples/main.sy](../examples/main.sy) | 模块 + 字符串 + 数组 + 正则 |
| [examples/file_tool/](../examples/file_tool/) | EC-M2-1 切片：Path + FileHandle 写读往返 |
| [examples/http_tool/](../examples/http_tool/) | EC-M2-1 切片：Http.get refuse + loopback |
| [examples/http_server_tool/](../examples/http_server_tool/) | EC-M2-1 切片：Http.serve + /health |
| [examples/sql_tool/](../examples/sql_tool/) | EC-M2-1 切片：Sql `:memory:` CRUD |
| [examples/math.sy](../examples/math.sy) | 可导出模块 |
| [examples/deps/](../examples/deps/) | 依赖 math 的子项目 |
| [examples/module_main.sy](../examples/module_main.sy) | 多模块入口示例 |
| [examples/os_info/](../examples/os_info/) | Os 系统信息（try/catch） |
| [examples/linalg/](../examples/linalg/) | packed-SYX `Mat`：乘/求解、形状、pad/padReflect（≠ 产品 Mat GA） |
| [examples/log_fields.sy](../examples/log_fields.sy) | `Log` 级别过滤 + `infoFields` |

**推荐学习顺序**：`hello.sy` → `main.sy` → `file_tool/` → `http_tool/` → `sql_tool/` → `linalg/` → `deps/`

标准库叙事见 [books/library-guide/](../books/library-guide/00-preface.md)。

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

默认应生成 **`app`**。若只有 `.o`，说明使用了 **`--object`**。若链接失败，请确认已安装 **clang** 且 `libsyx_rt.a` 可用（见 [plan/topics/profiles-sdk.md](../plan/topics/profiles-sdk.md)）。

无源码树时可用环境变量指定运行时归档（C-11 窄 `[x]` 契约；≠ Release 全矩阵 CI 上传）：

- **`SYX_RT_STATIC_LIB`**：指向已有的 `libsyx_rt.a` / `syx_rt.lib`（或含 hashed archive 的 deps 目录）
- **`SYX_RT_PROFILE=<id>`**：按 SDK 安装布局查找 `bin/../lib/profiles/<id>/libsyx_rt.a`（回退 `../lib/<id>/`）；约定见 [`packaging/syx-sdk/README.md`](../packaging/syx-sdk/README.md)

二者优先于编译期 embed 与 cargo `target/` 搜索。
### 与 Swift 的差异

SYX 语法相似但并非 Swift 超集。未实现或仅为 MVP 的特性包括：完整并发模型、泛型约束、属性包装、完整宏系统等。详见 [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md)。

---

## 相关文档

| 文档 | 内容 |
|------|------|
| [8_QuickReference.md](./8_QuickReference.md) | 语法速查 |
| [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) | P0–P3 功能清单 |
| [5_CompleteTechnicalDocumentation.md](./5_CompleteTechnicalDocumentation.md) | 编译器架构 |
| [4_InstructionSet.md](./4_InstructionSet.md) | VM 指令集 |

---

**维护者:** SYX Core Team  
**最后更新:** 2026-06-13  
**版本:** v0.1
