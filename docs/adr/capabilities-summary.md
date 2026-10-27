# SYX 现行能力 ADR 摘要（公开）

> **状态：** 定稿（plan-001 Phase 5）。  
> **不含** 窗号 tip、Fingerprint 序词、逐窗 Decision 流水。  
> 开源读者以本文件为**现行能力主入口**；窗级历史流水在内部 ledger（不作为公开依赖）。

本摘要按**能力域**说明「已实现 / 诚实边界 / 公开 OUT」。机器可读能力表见仓库内 TOML；行为以 `crates/syx-driver/tests` 行为测为准。

## 语言与集合

**已实现**

- 默认对象字面量走向 `Dictionary`；显式 `: Object` 表示异构袋。
- Object 方法面：`count` / `isEmpty` / `valueForKey` / `put` / `containsKey` / `keys` / `values` / `remove`（下标读写同语义）。
- Array / Set / Dictionary 有可运行的语言面与 stdlib 钉；参数化与类型诚实边界以能力表为准。

**诚实边界**

- **无** `.merge`；**不是** 真 `HashMap<Value,Value>` 键外壳；**不宣称** `object_bag_ga` 产品完成。
- 集合「参数化完成」≠ 与 Swift 泛型全集对等。

**对照**

- [`packages/core/builtins/reference/object.sy`](../../packages/core/builtins/reference/object.sy)
- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml)
- 语言叙述：[books/language-guide/05-collection-types.md](../../books/language-guide/05-collection-types.md)

## 网络

**已实现**

- `Http.serve` / `Http.shutdown` 默认栈为 **hyper + tokio**（plain HTTP/1；Upgrade 可走 `upgradeUpstream` 字节隧道或 `upgradeTerminate` 帧终结路径）。
- 进程内 HTTPS（诊断 PEM env）与同端口 HMR-over-TLS 自动化钉存在；多连接重叠 accept 有行为钉。
- WebSocket / gRPC 有**收窄面**实现与实验包入口（`net.grpc` 等标 experimental）。

**诚实边界**

- 收窄面 **≠** WebSocket / gRPC / `net.http.server` / `net.tls.server` **产品 GA**。
- `upgradeTerminate` 覆盖代表性 RFC6455 正向与 fail-closed / close(1002) 矩阵；**≠** extension / close 码全集 / 浏览器 Playwright HMR GA。
- 生产路径仍可能依赖 sidecar（如 websocat）处，**不宣称**「已去 sidecar」。

**对照**

- 实现：[`crates/syx-vm/src/http.rs`](../../crates/syx-vm/src/http.rs)
- 包面：[`packages/net`](../../packages/net)
- 用法与 OUT：[docs/9_UsageGuide.md](../9_UsageGuide.md)（HTTP server / upgrade 相关节）

## HostFn / 后端

**已实现**

- HostFn 能力表约 **249** 行（其中约 **233** `implemented`、约 **16** `partial`；以 TOML 为准）。
- 四后端维度：vm / mir / llvm / wasm（cranelift 等标 unsupported 的路径须诚实失败，禁止静默空成功）。
- Math / File / Http / Sql / Tcp 等大批 HostFn 已有行为钉；native AOT 以 LLVM 为唯一 native 后端。

**诚实边界**

- `partial` 行（含部分 Process* 等）**不是**「已实现」；缺后端须 fail-closed。
- 公开 CI **不**断言 HostFn 矩阵 tip / 序词 fingerprint；能力以本摘要 + `host_fn_capabilities.toml` + 行为测为准。

**对照**

- [`packages/internal/host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml)
- 注册与 lowering：[`crates/syx-stdlib`](../../crates/syx-stdlib)

## 可观测 / 安全 / 并发

**已实现**

- Metrics / Trace / Profiler 等可观测 HostFn 族有 mir/llvm/wasm 行为钉与 `status=implemented` 面（细节见 HostFn 表）。
- Actor / Task / Channel 等并发面有语言与 runtime 钉；可 `import` 的包边界见标准库指南。
- 权限 / sandbox 相关闸为 **opt-in** 薄面（非默认全开 sandbox 产品）。

**诚实边界**

- 可观测「族齐」≠ 云厂商级 APM 产品；sandbox **不是** default-on GA。
- Actor 可重入 / 隔离模型以行为测为准；**不宣称** TSAN / loom 产品 GA。

**对照**

- [books/library-guide](../../books/library-guide/SUMMARY.md)
- 并发行为测：`crates/syx-driver/tests/concurrency`

## 明确不做（公开 OUT）

下列项**不是**公开「已交付」宣称（内部可另有施工账本，但不构成本仓开源承诺）：

| OUT | 说明 |
|-----|------|
| Playwright HMR 产品 GA | 浏览器真 HMR 全链路非公开完成面 |
| TSAN / loom 产品 GA | 竞态工具链非公开完成面 |
| F-08 RT ABI「全清」 | 运行时 ABI 清债非公开完成宣称 |
| 完整 WSS / WebSocket「营销 GA」 | 收窄面 ≠ 营销口号级产品 GA |
| SynUI / Farm / Dist 产品化 | SynUI 为 opt-in；Farm/Dist 冻结/延后叙事 |
| Object `.merge` / 真 HashMap 袋 | 集合 OUT，见上节 |
| 公开 CI tip / 窗号账本 | tip 与逐窗 Decision 不在公开主叙事 |

## 相关

- [ADR README](README.md) — 公开摘要 vs 历史全量 vs 内部窗流水  
- [plan/plan-001-cleanup.md](../../plan/plan-001-cleanup.md) — 开源清理分期  
- 历史全量（过渡，非主入口）：[architecture-decisions.md](architecture-decisions.md)  
