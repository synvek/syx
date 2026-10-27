# 附录 B：诊断代码索引

用户向诊断代码速查。编译器实现细节见 [docs/3_SemanticAnalysis.md](../../docs/3_SemanticAnalysis.md)。

> **当前版本：** SYX **117.0** · Phase **139** · 文档指纹 `diag-139`（以编译器源码为准）

## 语义（S 系列）

| 代码 | 含义 |
|------|------|
| S001 | 符号重复定义 |
| S002 | 未定义标识符 |
| S003 | 对不可变变量赋值 |
| S004 | 类型不匹配 |
| S005 | 跨模块访问 internal / private / fileprivate 符号 |
| S006 | `Iterator` 协议缺少 `next()` |
| S007 | Sealed / exhaustiveness 相关（switch 缺 case） |
| S008 | 类型窄化（如 `Int64 → Int8`） |
| S009 | 有符号/无符号混合（部分） |
| S010 | 协议不符合 / 泛型 where 约束不满足 |
| S011 | 借用 / 变异违规 |
| S012 | 关联类型错误 |
| S013 | 集合类型一致性 |
| S014 | 有符号 / 无符号混合 |
| S015 | `Hashable` 协议不符 |
| S016 | `Equatable` 协议不符 |
| S017 | 生命周期违规 |
| S019 | Sendable channel |
| S020 | MainActor await 后违规 / `guard let` `else` 未退出 / 非 `async` 内 `await` |
| S022 | SIMD 在禁用模式下使用 |
| S024 | 集合 hosted op 禁止 legacy `MethodCall`（Phase 48） |
| S025 | 语义层 hosted 方法须走 extension / `CallHost`（Phase 50） |
| S046 | `is` 数值目标禁止（`1 is Int` 等） |
| S050 | 继承 override 校验相关（`override` 缺 / 父类未声明 / 签名不匹配） |
| S051 | override 缺关键字 |
| S052 | override 父类未声明 |
| S053 | 继承 `final` 类 |
| S054 | override 签名不匹配 |
| S055 | override 父类 `final` 方法 |
| S056 | 计算属性观察器（拒绝） |

## Host 插件（H 系列）

| 代码 | 含义 |
|------|------|
| H001 | 插件未加载或插件 Host 不存在 |
| H002 | 编译期插件 manifest 与运行期 / 锁文件不一致 |
| H003 | 插件 ABI 版本不匹配 |
| H004 | 插件 `min_syx_version` 不满足 |

## 词法（L 系列）

| 代码 | 含义 |
|------|------|
| L001 | 未闭合字符串 / 无效词法（lexer） |
| L002–L012 | Lint 规则（见 [9_UsageGuide](../../docs/9_UsageGuide.md)） |

Lint 摘录：

| 代码 | 含义 |
|------|------|
| L001 | 函数名应小写开头（lint） |
| L002 | 重复函数名 |
| L003 | 空函数体 |
| L004 | public 函数应声明返回类型 |
| L005 | async 函数应使用 await |
| L006 | 非法字符（变量名） |
| L007 | 全局变量缺少初始化 |
| L008 | 空 import 名 |
| L009 | 裸 return |
| L010 | 行超过 120 字符 |
| L011 | 含 TODO/FIXME |
| L012 | 建议使用结构化日志而非 print |

## 语法（P 系列）

| 代码 | 含义 |
|------|------|
| P001 | 解析错误（语法不合法） |
| P001 | 编译时缺 `syx.lock`（manifest 未安装） |

## 运行时（R 系列）

| 代码 | 含义 |
|------|------|
| R003 | `as!` 失败；Array / String 下标越界；整数窄化越界 |
| R007 | 对 nil 强制解包 `!` |
| R008 | `readLine` 失败 |
| R011 | 整数溢出 Display（**≠ GA**；现行算术恒 wrap）。同码亦可用于其它域（如 HTTP client） |
| R012 | 整数除零 |
| R013 | URL 解析失败 |
| R014 | 递归过深（栈溢出） |
| R016 | 必填字段未初始化 |

## 并发（A / C 系列）

| 代码 | 含义 |
|------|------|
| A002 | Actor 隔离违规 |
| A003 | GlobalActor 相关 |
| C001 | actor 隔离违规（更细粒度） |
| C002 | distributed actor 边界 |
| C003 | 跨隔离传非 `Sendable` 类型 |
| C004 | Task 取消相关 |
| C005 | GlobalActor await 后违规 |
| C006 | structured concurrency 父任务取消传播 |
| C009 | Actor 同步同实例 reentrancy 拒绝（serial-no-reenter） |
| C010 | Channel 已关闭（post-close send） |

## 宏（E / W 系列）

| 代码 | 含义 |
|------|------|
| W001 | `#warning` 触发 |
| E001 | `#error` 触发 |
| E002 | 宏展开深度超限或循环展开 |
| E003 | 角色错位（如 `@freestanding(expression)` 用在语句位置） |
| E004 | external macro 在 WASM 编译期（`SYX_TARGET_WASM=1`） |
| E006 | `@Service` / `@Component` 错用在非类型上 |
| E007 | 无 `@Lazy` 环依赖 |
| E008 | `@Inject` 缺依赖 |
| E010 | DI typed resolve 动态变量 |

## 借用机器证明

| 阶段 | 触发 |
|------|------|
| Phase 32+ | `SYX_BORROW_MACHINE_PROOF=1` 启用 proof obligation 导出 |
| Phase 33+ | `SYX_BORROW_COQ_LEAN_EXPORT=1` 启用 Coq/Lean 导出 |
| Phase 34+ | `SYX_BORROW_COQ_LEAN_PROOF_PROD=1` prod 机器检验 |

## 完整工具链诊断

详见 [docs/9_UsageGuide.md — 诊断代码](../../docs/9_UsageGuide.md#诊断代码)。

---

**参见** → [语言参考：关于本参考](../language-reference/00-about-this-reference.md) · [实现状态总表](../shared/implementation-status.md) · [docs/3_SemanticAnalysis.md](../../docs/3_SemanticAnalysis.md)
