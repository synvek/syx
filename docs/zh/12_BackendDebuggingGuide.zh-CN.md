# SYX 后端调试指南

[English](../12_BackendDebuggingGuide.md)


> **现状对齐：** SYX **117.0** · Phase **139** · Native **LLVM sole** · [10_BackendParity.md](./10_BackendParity.zh-CN.md)

SYX 可在多条执行路径下运行同一份源码。本指南说明何时使用各路径、常见差异与排查步骤。

## 后端选择

| 路径 | 启用方式 | 适用场景 |
|------|----------|----------|
| **VM** | 默认 `syx run`（经 MIR → 字节码） | 日常开发、完整 stdlib、最快迭代 |
| **Native（LLVM）** | `syx build --native` | 发布、性能、无 VM 依赖 |
| **WASM** | `syx run --wasm` / `syx build --wasm` | 沙箱 / 浏览器部署（CLI ≠ `compiler.wasm` 子集） |
| **JIT（实验）** | `SYX_JIT_TIER2=1` + feature（见 status） | Suite-1 子集；**非** AOT、默认关 |

统一 MIR 构建：`build_unified_mir` 为 native / WASM 的单一中端入口（见 [docs/10](./10_BackendParity.zh-CN.md)）。**默认**合并 stdlib 公开符号进 MIR；`SYX_STDLIB_MIR=0` 可关闭。

## VS Code 扩展安装

1. 构建 CLI：`cargo build -p syx-cli`
2. 进入 `editors/vscode-syx/` 并安装扩展依赖：**`npm install`**（必须；否则扩展激活失败，调试器 `type: syx` 不可用）
3. 在 VS Code：**Extensions → … → Install from VSIX…**（安装 `npm run package` 生成的 `.vsix`）或 **Developer: Install Extension from Location…**（选择 `editors/vscode-syx` 目录）
4. 修改扩展源码或重新打包后，执行 **Developer: Reload Window** 使变更生效
5. （可选）设置 `syx.languageServerPath` 为绝对路径 `${workspaceFolder}/target/debug/syx`；未设置时扩展会默认查找 `<工作区>/target/debug/syx`

LSP 由扩展在打开 `.sy` 文件时**自动启动**，无需单独用调试配置运行 `syx lsp`。

### 点号成员补全

在 `.sy` 文件中输入 `.` 可触发成员补全（需 LSP 正常运行）：

- 内置集合：`String` / `Array` / `Set` / `Dictionary` 的方法（如 `xs.map`、`s.split`）
- 字面量与调用结果：`[1, 2].`、`getItems().`
- 标准库模块：`HTTP.get(...).` → `status` / `json`；`import Concurrency` 后 `Concurrency.` → `join` / `sleep`
- 成员补全列表**不会**混入无关的全局符号（如 `print`）

验证步骤：

1. `cargo build -p syx-cli` 与 `editors/vscode-syx` 下 `npm install`
2. F5 启动 Extension Development Host，打开 `examples/task_demo.sy`
3. 在数组变量后输入 `.`，应出现 `count`、`map` 等候选（`len` 已退役，见 S004）
4. 若无响应：检查 **Output → SYX LSP Trace**（设置 `syx.trace.server` 为 `messages` 或 `verbose`）

仓库根 `.vscode/launch.json` 已内置 SYX 调试配置，**安装扩展后即可图形调试**；扩展还提供 **SYX: Debug File** 作为等价入口。

## VS Code 图形调试（DAP）

### 前置条件

- 以**含 `target/debug/syx` 的目录**作为 VS Code 工作区根（例如本仓库根目录），不要只打开单个 `.sy` 文件
- 已安装 SYX Language 扩展（见上一节）
- `cargo build -p syx-cli` 已成功，`target/debug/syx` 存在

### 操作步骤

1. 打开目标 `.sy` 文件（如 `examples/main.sy`）
2. 在行号左侧点击设置断点（调试会话启动前可先设好；启动后断点会变为已验证状态）
3. 打开 **Run and Debug** 面板，选择以下配置之一：
   - **SYX: Debug Current File**（仓库 `.vscode/launch.json`）
   - **SYX: Debug File**（扩展 `initialConfigurations`）
4. 按 **F5** 启动；首次运行会执行 `preLaunchTask` 构建 `syx-cli`
5. 调试工具栏：**Continue (F5)**、**Step Over (F10)**、**Step Into (F11)**、**Step Out (Shift+F11)**；Variables / Call Stack 面板查看 locals 与栈帧

底层命令等价于：

```bash
./target/debug/syx debug --dap path/to/file.sy
```

扩展通过 `type: "syx"` 调试器直接 spawn 上述二进制，经 stdin/stdout 与 DAP 通信。

### launch.json 示例

**必须使用 `type: "syx"`**，由 SYX 扩展注册的调试器启动 Rust 二进制。  
**切勿**使用 `"type": "node"`——Node 调试器会把 `syx` 当作 JavaScript 执行，导致 `SyntaxError: Invalid or unexpected token`。

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "SYX: Debug Current File",
      "type": "syx",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/syx",
      "args": ["debug", "--dap", "${file}"],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo build syx"
    },
    {
      "name": "SYX: Debug Project (examples)",
      "type": "syx",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/syx",
      "args": ["debug", "--dap", "${workspaceFolder}/examples"],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo build syx"
    }
  ]
}
```

| 字段 | 说明 |
|------|------|
| `type` | 固定为 `"syx"`（扩展贡献的调试器类型） |
| `program` | `syx` CLI 二进制路径 |
| `args` | `["debug", "--dap", "<文件或目录>"]`；`${file}` 为当前编辑器文件 |
| `preLaunchTask` | 可选；对应 `.vscode/tasks.json` 中的 `cargo build syx` |

### 常见故障排查

| 现象 | 原因 | 处理 |
|------|------|------|
| 控制台出现 `node ... ./target/debug/syx lsp` 且报 `SyntaxError: Invalid or unexpected token` | `launch.json` 使用了 `"type": "node"`，或误选了旧的 Node 调试配置 | 改为 `"type": "syx"`，选择 **SYX: Debug Current File** / **SYX: Debug File**；**不要**用 Node 配置启动 LSP |
| `Couldn't find a debug adapter descriptor for debug type 'syx'` | SYX 扩展未成功激活（常见：VSIX 未打包依赖、未执行 `npm install`、扩展加载时 `require` 失败） | 在 `editors/vscode-syx` 执行 `npm install` → **Developer: Reload Window**；或重新 **Install Extension from Location**；打开 **Help → Toggle Developer Tools** 查看 `SYX Language` 激活错误 |
| 点运行/调试无反应 | 工作区不是项目根目录，找不到 `target/debug/syx` | 打开含 `Cargo.toml` 与 `target/debug/syx` 的仓库根目录 |
| 调试控制台只显示 `cargo build` 警告和 `Finished dev profile` | 这是 **preLaunchTask** 构建输出，不是错误；DAP 调试走 stdio，**Debug Console 通常几乎无输出** | 构建成功后看顶部是否出现**橙色调试工具栏**、**CALL STACK** / **VARIABLES** 面板是否有内容；断点应变红且已验证 |
| 断点灰色、无法命中 | 调试会话未成功启动，或 DAP 握手未完成 | 确认 F5 后 Run and Debug 工具栏已激活；Rebuild CLI 并重载窗口 |
| LSP 无 hover/诊断 | `syx` 路径不对或未构建 | 设置 `syx.languageServerPath` 或先 `cargo build -p syx-cli` |
| 修改扩展后行为未变 | 扩展未重载 | **Developer: Reload Window** 或重新 Install from Location |

错误示例（**不要**使用）：

```json
{
  "name": "SYX Debug (DAP)",
  "type": "node",
  "program": "${workspaceFolder}/target/debug/syx",
  "args": ["debug", "--dap", "${file}"]
}
```

上述配置会让 VS Code 用 Node.js 解释 Rust 二进制，必然失败。

### 当前能力与限制

SYX VS Code 调试是 **VM 字节码级 DAP 调试**（Phase 5–13 逐步完善），不是完整的 Swift/LLVM 级调试器。

| 能力 | 状态 |
|------|------|
| 启动/停止调试（含 Stop 按钮） | 支持（`terminate` + `terminated` 事件） |
| 断点（源行 → PC） | 支持；调试编译**强制 AST 路径**以保留 `instruction_spans`（默认 MIR 编译无行号映射） |
| Step Over / Into / Out | 支持（单 VM 指令步进；编辑器行仅在 `instruction_spans` 覆盖时更新） |
| Variables 面板 | 支持 `local_N` 槽位；需步进到赋值指令之后才有值 |
| Call Stack 高亮当前行 | 支持（`stackTrace` 返回 `source.path` + 行号） |
| 程序 stdout | 暂不在 Debug Console 显示（后续可加 `output` 事件） |
| async/await 复杂控制流 | 行号映射可能偏移；优先在简单语句行设断点 |

若断点仍为灰色 hollow 圆，表示该行没有可映射的字节码指令（如空行、纯类型声明行）；请在 `print(...)`、`let` 赋值等**可执行语句**行设断点。

重新编译后生效：

```bash
cargo build -p syx-cli
```

然后在 VS Code **Developer: Reload Window**，对 `examples/main.sy` 或 `crates/syx-driver/tests/p4_hof.sy` 试 F5 + 断点。

### 命令行快速验证

```bash
syx debug examples/main.sy          # 交互式终端调试
syx debug --dap examples/main.sy    # DAP 模式（VS Code 使用此模式）
```


不依赖 VS Code，可单独确认调试能力：

```bash
# 交互式终端调试（字节码级）
syx debug examples/main.sy

# DAP 模式（VS Code 使用此模式）
syx debug --dap examples/main.sy
```

DAP 模式下向 stdin 写入带 `Content-Length` 头的 JSON 请求，stdout 应返回同格式响应；`initialize` 后应收到 `initialized` 事件，`configurationDone` 后应收到 `stopped`（`reason: "entry"`）事件。

## VS Code Marketplace 发布草案（Phase 10）

> 以下为草案流程，**不要求实际上架**；版本与 `syx-cli` 对齐（当前 `0.4.0` / SYX 4.0）。

1. **前置**
   - `cargo build --release -p syx-cli`（用户机器需能调用 `syx lsp` / `syx debug --dap`）
   - Node.js 18+，`npm install` in `editors/vscode-syx/`
2. **打包**
   ```bash
   cd editors/vscode-syx
   npm run package    # vsce package → syx-language-0.4.0.vsix
   ```
3. **本地验证**
   - VS Code：**Install from VSIX…** 安装生成的 `.vsix`
   - 打开 `.sy`：验证 hover（类型）、rename、Find All References
   - F5 调试：断点命中行与 `instruction_spans` 一致
4. **发布（可选）**
   ```bash
   # 需 publisher 账号与 Personal Access Token
   npm run publish:marketplace
   ```
   - Marketplace 条目：`displayName` = SYX Language，`publisher` = syx
   - README 指向 docs/9 使用指南与 docs/12 调试指南
5. **CI 对齐**
   - `cargo test -p syx-lsp` ≥ 35
   - `cargo test -p syx-debug pc_to_line_parity`
   - Release workflow 强制 wasmtime + `SYX_WASI=1` E2E

## VS Code E2E（Phase 8）

1. 打开仓库根目录；`.vscode/launch.json` 含 **SYX: Debug Current File** 与 **SYX: Debug Project (examples)** 配置（`type: "syx"`）。
2. `tasks.json` 中 `cargo build syx` 为 Debug 配置的 `preLaunchTask`。
3. 在 `.sy` 文件设断点 → F5 启动 **SYX: Debug Current File** → 验证 `stackTrace` 显示源码行（`instruction_spans`）。
4. Watch：DAP `evaluate` 使用 `watch:local_0` 前缀。

## DAP 3.0（Phase 7）

- **Watch：** `evaluate` 表达式以 `watch:` 前缀注册监视（如 `watch:local_0`）
- **Async 栈帧：** `stackTrace` 在 async 挂起时追加 `async::<label>` 帧
- **Source map：** `DebugSession.source_map` 映射 PC ↔ 源码行（VM 路径）
- 条件断点与多帧能力见 Phase 6 小节

## DAP 2.0（Phase 6）

- 条件断点：`setBreakpoints` 支持 `condition`（如 `local_0 == "7"`、`false` 禁用）
- `evaluate`：读取 `local_N` 槽位
- 多帧 `stackTrace`：`stack_summary` 含 fn/pc 与栈顶值
- 启动：`.vscode/launch.json` → `"type": "syx"` + `syx debug --dap ${file}`（见「VS Code 图形调试」）

## MIR 路径说明

- 仅对用户模块启用 MIR；嵌入 stdlib（`<stdlib>/…`）仍走 codegen，避免 stdlib 包装函数递归。
- `compile_with_mir`：类型 shell（dataclass init/hash、扩展）由 codegen 生成，函数体由 MIR 覆盖。
- 若 MIR 与 VM 输出不一致，先对比 `crates/syx-driver/tests/p5_mir_parity.rs`。

## Native 链接

1. 修改 `syx-rt` / `syx-mir-run` 后必须重建 runtime：
   ```bash
   cargo build --release -p syx-rt -p syx-cli
   ```
2. 体积优化：`syx build --native --minimal` 会重建 `syx-rt --no-default-features` 再链接。
3. 链接失败 `undefined symbol _syx_rt_*`：删除 stale `target/*/deps/libsyx_rt-*.a`，重建 `syx-rt`（driver 已按 mtime 选最新 archive）。

## 增量编译缓存

单文件测试目录下的 `.sy-cache/` 可能缓存旧 bytecode。stdlib 变更后若行为异常：

```bash
rm -rf path/to/tests/.sy-cache
```

## 网络测试

HTTP 回归使用 in-tree TCP mock（`p16_arch_eco::http_fetch_e2e_vm_and_native`、`p14_native_http`），无需外网。本地额外验证：

```bash
cargo test -p syx-driver --test p16_arch_eco http_fetch_e2e -- --test-threads=1
cargo test -p syx-driver --test p14_native_http -- --test-threads=1
```

## DAP / 断点

`syx debug` 启动**交互式终端调试**（字节码级，非 IDE 图形界面）：

```bash
# 单文件
syx debug examples/main.sy

# 项目目录（读 syx.toml）
syx debug examples
```

进入 `(debug)` 提示符后可用命令：

| 命令 | 说明 |
|------|------|
| `l` / `list [n]` | 列出 main 前 n 条 VM 字节码 |
| `s` / `step` | 单步查看下一条指令（不执行） |
| `b <pc>` | 在指令索引 pc 设断点（step 时提示） |
| `r` / `run` | 在 VM 上完整运行并打印 stdout |
| `q` | 退出 |

IDE 集成：`syx debug --dap`，走 **DAP 标准帧**（stdin/stdout，`Content-Length` + JSON body）。实现位于 `crates/syx-debug`；VS Code 扩展 `type: "syx"` 通过 `DebugAdapterExecutable` spawn 二进制。协议流程：

1. Client → `initialize` → Server 响应 + `initialized` 事件
2. Client → `launch` → Server 加载程序
3. Client → `setBreakpoints`（源行号 → PC，`instruction_spans` + `line_to_pc`）
4. Client → `configurationDone` → Server 发送 `stopped`（`reason: "entry"`）
5. 运行中 `continue` / `next` / `stepIn` / `stepOut` 暂停时再发 `stopped` 事件

支持命令：

| DAP 命令 | 说明 |
|----------|------|
| `setBreakpoints` | 源行号 → PC 映射（`instruction_spans` + `line_to_pc`） |
| `continue` | 运行至下一断点 |
| `next` / `stepIn` | 单步执行 VM 指令 |
| `stepOut` | 运行至当前函数 `Return` |
| `scopes` / `variables` | 结构化 locals（`value_description` + 类型名） |
| `stackTrace` | 当前函数名 + 源行 |
| `configurationDone` | 配置完成；触发入口 `stopped` |

`launch.json` 完整示例见上文「VS Code 图形调试」一节。

日常排查仍推荐：

- `syx run file.sy` + `print(...)` 观察输出
- `SYX_USE_MIR=1 syx run` 对比 MIR 与 VM
- `cargo test -p syx-driver --test p5_mir_parity` 回归 parity
