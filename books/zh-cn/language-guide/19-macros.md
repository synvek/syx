# 宏

**实现状态：** **M28 + Service DI v19（SYX 116.0）** — 见 [ADR-130](../../../docs/adr/architecture-decisions.md) · [架构决议](../../../docs/adr/architecture-decisions.md#宏系统) · 文档指纹 `macro-docs-v20:m28-service-v19-honesty`

## SYX 116.0：Macro M28 + Service DI v19（GA vs partial）

| 能力 | 档位 | 说明 |
|------|------|------|
| **声明位 `#macro` multi-token decl rewrite MVP** | **部分 GA** | 声明位 `Ident`+`Punct`+`Group` token 流可 rewrite 为 rename/wrap 函数 decl（指纹 `macro-token-v9:token-rewrite-mvp`）；**非**完整 token-stream / 用户 SYX 展开逻辑 → **139+** |
| **`@Pointcut("within(*Service)")` / `within(Foo*)` 类型名通配 MVP** | **部分 GA** | `@Pointcut("within(*Service)")` 后缀 / `within(Foo*)` 前缀窄通配（指纹 `macro-service-v19:pointcut-within-wildcard-mvp`）；**非** AspectJ 全表达式 / 切面类 → **139+** |

## SYX 115.0：Macro M27 + Service DI v18（GA vs partial）

| 能力 | 档位 | 说明 |
|------|------|------|
| **声明位 `#macro` decl-expand MVP** | **部分 GA** | 声明位单 `Ident` token 可 expand 为 rename/wrap 函数 decl（指纹 `macro-token-v8:decl-expand-mvp`）；**非**完整 token-stream / 用户 SYX 展开逻辑 → **138+** |
| **`@Pointcut("!within(Type)")` 否定 MVP** | **部分 GA** | `@Pointcut("!within(Foo)")` 窄否定类型名模式（指纹 `macro-service-v18:pointcut-not-within-mvp`）；**非** AspectJ 全表达式 / 切面类 → **138+** |

## SYX 114.0：Macro M26 + Service DI v17（GA vs partial）

| 能力 | 档位 | 说明 |
|------|------|------|
| Freestanding builtin（`#assert` `#if` `#for` `#stringify` `#compileTime`） | **GA** | `syx-macro` builtin pass；角色错位 → **E003**（M9 继承） |
| 用户 `#macro` + Hygiene v2 | **部分** | 语句位文本替换；表达式/声明位参数替换 + 声明位卫生 MVP 见下行 |
| **Token 表达式位 `#macro` MVP** | **部分 GA** | 表达式位 `#macro` ident/literal 参数替换；Phase 134 表达式树加深（指纹 `macro-token-v5:expr-token-subst-mvp`）；Phase 130 初版（`macro-token-v1:expr-token-mvp`）；**非**完整 token 语法宏 |
| **声明位 `#macro` token-stream 枚举 MVP** | **部分 GA** | 声明位 `#macro` 参数可枚举为 `Ident` / `Punct` / `Group` 三态 token 列表（指纹 `macro-token-v7:token-stream-mvp`）；**非**完整 token-stream / 用户 SYX 展开逻辑 → **137+** |
| **声明位 `#macro` token subst 加深 MVP** | **部分 GA** | 声明位 freestanding `#macro` 函数名 / 参数默认值 / enum payload 等 ident/literal 替换加深（指纹 `macro-token-v6:decl-token-subst-mvp`）；Phase 131 初版（`macro-token-v2:decl-param-subst-mvp`）；**非**完整 token 语法宏 |
| **声明位 `#macro` 参数替换 MVP** | **部分 GA** | 声明位 freestanding `#macro` 参数文本替换（指纹 `macro-token-v2:decl-param-subst-mvp`）；**非**完整 token 语法宏 |
| **声明位 `#macro` 卫生 + linked/run_file 收口 MVP** | **部分 GA** | 声明位 hygiene（`macro-token-v3:decl-hygiene-mvp`）；`compile_linked` / `run_file_printed` 可复现；**非**完整 token 语法宏 |
| **语句位 `#macro` token subst MVP** | **部分 GA** | 语句位 freestanding `#macro` ident/literal 参数文本替换（指纹 `macro-token-v4:stmt-token-subst-mvp`）；**非**完整 token 语法宏 / token-stream API |
| `@attached` member/peer/accessor/extension/deinit | **部分** | 真 AST `parse_fragment`；非全 Swift 角色面 |
| `@Equatable` / `@Hashable` / `@Codable` L1 | **GA** | stored `data class` |
| **`@Codable` L2 associated-value enum 全自动** | **部分 GA** | 关联值 enum `encode`/`decode` 往返（指纹 `macro-codable-l2:associated-enum-auto`）；**非**泛型关联值全集 / 任意嵌套深度 / 自定义 coding keys DSL |
| `#externalMacro` plugin-prod | **部分** | cdylib JSON 生产路径；Linux seccomp 硬闸 / Darwin `reexec-only` 诚实标签 |
| `@resultBuilder` | **GA** | engine 固定点管线第六 pass；literal+for-in 与 **HostCall+Closure** 混用 **可编译运行**（`macro-pipeline-v6:rb-hostcall-closure-fixed`） |
| 宏包 `[macros].exports` | **GA** | Phase 120 schema 与 `SyxManifest` 对齐 |
| `@Service` / `@Component` + `@Inject` ctor + 字段 `@Autowired` | **GA** | **多包 workspace 扫描**（仅 `resolved_packages`）+ 真实 ctor resolve；singleton 默认（M14 继承） |
| `@Scope("prototype")` / `@Scope("request")` | **GA** | prototype 每次新实例；request 帧内单例（M13 继承） |
| `@Qualifier` / `@Primary` | **GA** | 多绑定消歧（指纹 `macro-service-v5:conditional-qualifier`） |
| `@Profile` | **GA** | 条件注册；`Di.setProfile` / `SYX_DI_PROFILE`（指纹 `macro-service-v5:conditional-profile`） |
| `@PostConstruct` / `@PreDestroy` 钩子 MVP | **GA** | 构造 + 字段注入后 / 容器 destroy·reset·request end 时 |
| Typed resolve：`"Type"` / `Type` ident | **GA** | 字符串字面量或已注册类型标识符；动态变量 → **E010** |
| DI 仪式糖化 + `Di` 包 GA | **GA** | `import Di` → `resolve` / `withRequest` / `setProfile` / …；短糖 `resolve(...)`（M14 继承） |
| **基类型多态注入** | **GA** | 构造/字段 `@Inject` 以**基类/父类型**解析具体实现；registry 绑定 superclass alias（指纹 `macro-service-v6:polymorphic-base-inject`）；**不得**再依赖 init 副作用冒充 dispatch |
| **协议型注入 / witness table** | **GA** | conformance witness 注册协议 ← `@Service` 实现（指纹 `runtime-service-v7:protocol-witness`）；M15 alias binding 继承 |
| **字段 `@Qualifier` 元数据** | **GA** | `@Autowired @Qualifier("…")` 跨包保留消歧元数据（指纹 `macro-service-v6:field-qualifier-metadata`） |
| **`@Around` advice proceed + `@Before`/`@After`** | **部分 GA** | **仅** `@Service`/`@Component` **实例方法**；有序多 `@Around` **proceed 委托**（`__syx_proceed_N`）+ Before/After；可与 `@Pointcut` / `execution` 名模式绑定（指纹 `macro-service-v8:advice-proceed`）；**非** AspectJ / 切面类 |
| **`@Pointcut` 方法名 MVP** | **部分 GA** | `@Pointcut("Type.method")` 注册窄字符串模式；`@Around("…")` 绑定 proceed 链（指纹 `macro-service-v11:pointcut-name-mvp`）；**非** AspectJ 表达式 / 切面类 / 通配 join point |
| **`execution(* T.m(..))` 方法名模式 MVP** | **部分 GA** | `@Pointcut("execution(* Type.method(..))")` / `execution(* Type.*(..))` 名/`*` 模式（指纹 `macro-service-v12:pointcut-execution-name-mvp`）；**非**完整 AspectJ |
| **`@Before`/`@After` pointcut 字符串绑定 MVP** | **部分 GA** | `@Before("…")` / `@After("…")` 绑定已注册 `@Pointcut` / execution 名模式（指纹 `macro-service-v13:before-after-pointcut-bind-mvp`）；裸 Before/After 仍为声明序自包装；**非**完整 AspectJ |
| **`@Pointcut("within(Type)")` 类型名模式 MVP** | **部分 GA** | `@Pointcut("within(Foo)")` 注册窄类型名模式；`@Around`/`@Before`/`@After` 可绑定命中该类型上任意方法（指纹 `macro-service-v14:pointcut-within-name-mvp`）；**非** AspectJ 表达式 / 切面类 |
| **`@Pointcut("args(Type)")` 首参类型名模式 MVP** | **部分 GA** | `@Pointcut("args(Int)")` 注册窄首参类型名模式；`@Around`/`@Before`/`@After` 可绑定首参类型匹配的方法（指纹 `macro-service-v15:pointcut-args-name-mvp`）；**非** AspectJ 全表达式 / 切面类 |
| **`@Pointcut("within(T) && args(U)")` 窄组合 MVP** | **部分 GA** | `@Pointcut("within(Foo) && args(Int)")` 仅 `within` + `args` 名模式 `&&` 组合；两者均命中才匹配（指纹 `macro-service-v16:pointcut-and-compose-mvp`）；**非**否定 / AspectJ 全表达式 / 切面类 |
| **`@Pointcut("within(T) || args(U)")` 窄组合 MVP** | **部分 GA** | `@Pointcut("within(Foo) || args(Int)")` 仅 `within` + `args` 名模式 `||` 组合；任一侧命中即匹配（指纹 `macro-service-v17:pointcut-or-compose-mvp`）；**非**否定 / AspectJ 全表达式 / 切面类 / Spring → **137+** |
| **`@Pointcut("!within(Type)")` 否定 MVP** | **部分 GA** | `@Pointcut("!within(Foo)")` 窄否定类型名模式（指纹 `macro-service-v18:pointcut-not-within-mvp`）；**非** AspectJ 全表达式 / 切面类 |
| **声明位 `#macro` decl-expand MVP** | **部分 GA** | 声明位单 `Ident` token 可 expand 为 rename/wrap 函数 decl（指纹 `macro-token-v8:decl-expand-mvp`）；**非**完整 token-stream / 用户 SYX 展开 → **138+** |
| **`@Configuration` / `@Bean` MVP** | **部分 GA** | 配置类工厂方法注册到 DI；`Di.resolve` 可解析（指纹 `macro-service-v8:configuration-bean-mvp`） |
| **`@Lazy` + 环依赖懒代理 MVP** | **部分 GA** | 延迟工厂至首次 resolve/use；含 `@Lazy` 边的环插入懒代理；无 `@Lazy` 环仍 **E007**（指纹 `macro-service-v9:lazy-cycle-proxy-mvp`）；**非**任意类型代理 / CGLIB / 作用域代理全集 |
| **`@Bean` 参数 `@Autowired` / 类型解析 MVP** | **部分 GA** | `@Configuration` 内 `@Bean` 方法参数按类型注入已注册 `@Service`/`@Bean`（指纹 `macro-service-v10:bean-param-autowire-mvp`）；**非**条件装配全集 / CGLIB / 作用域代理全集 |
| 完整 token 语法宏 / 完整 AspectJ / 完整 Spring | **未实现** | **137+** |

测试：`crates/syx-driver/tests/fixtures/macros/` · `examples/service_di/` · `examples/stmt_token_macro/` · `examples/decl_param_macro/` · `examples/token_expr_macro/` · `within_wildcard_pointcut.sy` · `not_within_pointcut.sy` · `or_pointcut.sy` · `and_pointcut.sy` · `decl_token.sy` · `expr_token.sy` · `within_pointcut.sy` · `stmt_token.sy` · `macro_di_within_wildcard.rs` · `macro_di_not_within.rs` · `macro_di_or_pointcut.rs` · `service_di_and_pointcut.rs` · `token_decl_deepen.rs` · `task_queue_caller_runs.rs` · `bounded_task_queue.rs`

SYX 宏分为 **freestanding**（`#`）与 **attached**（`@`）。由 `engine::expand_to_fixed_point` 驱动（builtin → attached → **wrapper** → user → **external** → **resultBuilder** → **service DI**）。

## `@Service` / `@Component` / `@Inject` / `@Autowired`（Phase 136 · Service DI v17）

Spring 风格**可演示子集**：singleton（默认）+ prototype + **request** scope + **Qualifier/Primary/Profile** + **基类型多态注入** + **协议 witness table** + 字段 `@Qualifier` 元数据 + **`@Around` proceed 委托 / `@Before`/`@After`（含字符串 pointcut 绑定）** + **`@Pointcut("Type.method")` 方法名 MVP** + **`execution(* T.m(..))` 名模式 MVP** + **`@Pointcut("within(Type)")` 类型名模式 MVP** + **`@Pointcut("args(Type)")` 首参类型名模式 MVP** + **`@Pointcut("within(T) && args(U)")` / `within(T) || args(U)` 窄组合 MVP** + **`@Configuration`/`@Bean` MVP** + **`@Bean` 参数注入 MVP** + **`@Lazy` + 环依赖懒代理 MVP** + 构造/字段注入 + 多包 workspace 扫描 + `Di.withRequest`。不得称 Spring 兼容 / 完整 AspectJ AOP / 完整 token 语法宏 / 全自动装配。

```syx
import Di

@Qualifier("fast")
@Service
class FastLogger {
  init() {}
  func tag() -> String { return "fast" }
}

@Primary
@Service
class SlowLogger {
  init() {}
  func tag() -> String { return "slow" }
}

@Service
class Worker {
  let logger: FastLogger
  init(@Inject @Qualifier("fast") logger: FastLogger) {
    self.logger = logger
  }
}

@Profile("prod")
@Service
class ProdOnly {
  init() {}
}

@Scope("request")
@Service
class RequestCtx {
  init() {}
}

func handleReq() {
  let _ctx = Di.resolve("RequestCtx")
}

func main() {
  Di.setProfile("prod")
  let w = Di.resolve("Worker") as Worker
  print(w.logger.tag())
  Di.withRequest("handleReq")
  Di.destroy()
}
```

- **`@Service` / `@Component`** — attached 属性，标记可注册类型；错用在非类型上 → **E006**
- **多包 workspace 扫描** — driver 聚合 `resolved_packages` 内 `@Service`；**跳过**同目录嵌套 `syx.toml` sibling
- **`@Scope("prototype")`** — 每次 `resolve` 创建新实例
- **`@Scope("request")`** — 请求帧内同一实例；跨帧新实例；优先 `Di.withRequest(handler)`，亦可 `Di.beginRequest` / `Di.endRequest`
- **`@Qualifier("name")` / `@Primary`** — 多实现消歧；无 Primary/Qualifier 的歧义 → fail-closed
- **`@Profile("name")`** — 仅当 `Di.setProfile` / `SYX_DI_PROFILE` 匹配时注册
- **`@Inject`** — **构造参数** 注入；缺依赖 fail-closed（**E008**）；无 `@Lazy` 环依赖仍 **E007**
- **字段 `@Autowired` MVP** — stored property；resolve 后在实例上赋值（无反射全集）
- **`@PostConstruct` / `@PreDestroy` MVP** — 生命周期钩子
- **用户面容器** — `import Di`：`Di.resolve` / `Di.withRequest` / `Di.setProfile` / `Di.destroy`；短糖 `resolve(...)`；**应用代码勿直接调用 `__syx_service_*`**
- **基类型多态注入** — `@Inject logger: Logger` 当 `FastLogger: Logger` 注册时解析到具体实现（registry superclass alias）；不再以 init 副作用代替 dispatch
- **字段 `@Qualifier`** — `@Autowired @Qualifier("fast") let logger: Logger` 跨包保留消歧元数据
- **`@Around` proceed** — **仅** `@Service`/`@Component` **实例方法**；展开为有序包裹原方法体的 proceed 委托（`__syx_proceed_N`）+ `print("__around__:method")`；可观察返回值；可叠 `@Around("Type.method")` 绑定已注册 `@Pointcut`
- **`@Pointcut` 方法名 MVP** — `@Pointcut("Type.method")` 注册窄字符串模式；`@Around("…")` 须匹配已注册 pointcut，否则 fail-closed（指纹 `macro-service-v11:pointcut-name-mvp`）；**非** AspectJ 表达式 / 切面类 / 通配 join point
- **Token 表达式位 `#macro` MVP** — 表达式位 `#double(21)` / `#apply(f,x)` 等 ident/literal 参数替换；Phase 134 表达式树 walk 加深（指纹 `macro-token-v5:expr-token-subst-mvp`）；Phase 130 初版（`macro-token-v1:expr-token-mvp`）；**非**完整 token 语法宏 / token-stream API
- **声明位 `#macro` token subst 加深 MVP** — 声明位函数名 / 参数默认值 / enum payload 等 ident/literal 替换（指纹 `macro-token-v6:decl-token-subst-mvp`）；Phase 131 初版（`macro-token-v2:decl-param-subst-mvp`）；**非**完整 token 语法宏 / token-stream API
- **声明位 `#macro` token-stream 枚举 MVP** — 声明位参数可枚举为 `Ident` / `Punct` / `Group`（指纹 `macro-token-v7:token-stream-mvp`）；**非**完整 token-stream / 用户 SYX 展开 → **137+**
- **语句位 `#macro` token subst MVP** — 语句位 `#say("x")` 等 ident/literal 参数文本替换（指纹 `macro-token-v4:stmt-token-subst-mvp`）；**非**完整 token 语法宏 / token-stream API
- **`@Pointcut("within(Type)")` 类型名 MVP** — `@Pointcut("within(Foo)")` 注册窄类型名模式；`@Around`/`@Before`/`@After` 可绑定命中该类型上任意方法（指纹 `macro-service-v14:pointcut-within-name-mvp`）；**非** AspectJ 表达式 / 切面类
- **`@Pointcut("args(Type)")` 首参类型名 MVP** — `@Pointcut("args(Int)")` 注册窄首参类型名模式；`@Around`/`@Before`/`@After` 可绑定首参类型匹配的方法（指纹 `macro-service-v15:pointcut-args-name-mvp`）；**非** AspectJ 全表达式 / 切面类
- **`@Pointcut("within(T) && args(U)")` 窄组合 MVP** — `@Pointcut("within(Foo) && args(Int)")` 仅 `within` + `args` 名模式 `&&` 组合；两者均命中才匹配（指纹 `macro-service-v16:pointcut-and-compose-mvp`）；**非**否定 / AspectJ 全表达式 / 切面类
- **`@Pointcut("within(T) || args(U)")` 窄组合 MVP** — `@Pointcut("within(Foo) || args(Int)")` 仅 `within` + `args` 名模式 `||` 组合；任一侧命中即匹配（指纹 `macro-service-v17:pointcut-or-compose-mvp`）；**非**否定 / AspectJ 全表达式 / 切面类 / Spring → **137+**
- **`@Configuration` / `@Bean` MVP** — 配置类内工厂方法注册绑定；`Di.resolve` 解析返回类型
- **`@Bean` 参数注入 MVP** — `@Bean` 方法参数按类型 / `@Autowired`（可选 `@Qualifier`）从已注册 `@Service`/`@Bean` 图注入（指纹 `macro-service-v10:bean-param-autowire-mvp`）；**非**条件装配全集
- **`@Lazy` + 环依赖懒代理** — 延迟工厂至首次 `Di.resolve`/使用；含 `@Lazy` 边的 A↔B 环插入懒代理句柄（首次方法调用物化）；**非**任意类型代理 / CGLIB / 作用域代理全集
- **137+ 债务** — 完整 token 语法宏 / token-stream API、完整 AspectJ 式 AOP（否定 / 切面类 / 通配 join point / 全表达式与优先级全集）、完整 Spring 容器、ThreadPoolExecutor 全集（DiscardOldest / Actor 共享池等）

```syx
@Service
class Audited {
  init() {}
  @Pointcut("Audited.work")
  @Around("Audited.work")
  func work() -> Int {
    return 42
  }
}

@Configuration
class AppConfig {
  init() {}
  @Bean
  func greeter(@Autowired logger: DepLogger) -> BeanGreeter {
    return BeanGreeter(logger: logger)
  }
}

@Lazy
@Service
class Expensive {
  init() { print("factory") }
}

@Service
class CycleA {
  let b: CycleB
  init(@Inject b: CycleB) { self.b = b }
}

@Lazy
@Service
class CycleB {
  let a: CycleA
  init(@Inject a: CycleA) { self.a = a }
}
```

测试：`platform/service_di_and_pointcut.rs` · `fixtures/macros/`（`token_expr.sy` · `pointcut_around.sy` 等） · `examples/service_di/` · `examples/token_expr_macro/` · `packages/di/mod.sy`

## `@freestanding` 角色强制（Phase 119 · M9 继承）

`expression` / `statement` / `declaration` 附着点错位 → 编译错误 **E003**（driver + LSP E2E）。

测试：`fixtures/lsp/freestanding_decl_role_a.sy`

## Attached wrapper + 卫生 DX

```syx
@attached(wrapper)
class Clamped {
  var wrappedValue: Int
  init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}

class Point {
  @Clamped var x: Int
}
```

- **`@propertyWrapper`** 已 deprecated（W001）；请迁移 `@attached(wrapper)`
- **Hygiene escape**：宏体内 `\name` 保留 outer binding（`SYX_MACRO_HYGIENE=0` 全局关闭一版）
- **Import 宏**：`register_imported_macros` 注册 User + External

测试：`fixtures/macros/attached_wrapper.sy`、`fixtures/macros/hygiene_escape_ok.sy`、`fixtures/macros/`

## External 生产路径（Phase 119 plugin-prod）

```syx
macro ExternalSmoke = #externalMacro(module: "SyxMacros", type: "SmokeMemberMacro")

func main() {
    #ExternalSmoke()   // freestanding statement 展开
}
```

- **跨模块 import**：依赖 crate 的 `MacroDecl` 经 `register_imported_external_macros` 注册
- **WASM**：`SYX_TARGET_WASM=1` 时 external 宏 compile-time E004
- **生产**：cdylib JSON 协议真实类型；子进程沙箱默认 on；**无 seccomp 不得冒充 hard-fail GA**（Linux 有则硬闸，Darwin 诚实 `sandbox-reexec-only`）

测试：`fixtures/lsp/freestanding_decl.sy`、`fixtures/macros/json_plugin_prod.sy`、`fixtures/macros/json_plugin_prod.sy`

## Freestanding 内置宏

| 宏 | 说明 |
|----|------|
| `#assert(expr)` | 断言失败终止 |
| `#if condition` | 条件编译 |
| `#assert_eq` / `#assert_ne` | 相等断言 |
| `#while` / `#for` | 循环展开 |
| `#warning` / `#error` | W001 / E001 |
| `#compileTime` | 常量求值子集（M9 硬化边界） |
| `#stringify(expr)` | 表达式位 → 字符串字面量（M9 builtin pass） |

```syx
#assert(1 + 1 == 2)
#for(3, print("x"))
let s = #stringify(1 + 1)   // "1 + 1"
```

测试：`fixtures/macros/macro_engine.sy`、`fixtures/macros/macro_engine.sy`、`fixtures/lsp/freestanding_decl.sy`

## 用户 `#macro`（Phase 40+，卫生 v2 Phase 50）

```syx
#macro twice(x) {
    print(x)
    print(x)
}

func main() {
    #twice("hello")
}
```

- 仅 **freestanding**（语句位）；参数为标识符/表达式文本替换
- 展开深度 ≤16；循环或超深展开 → **`E002`**
- **Phase 50 卫生 v2**：参数绑定 `MacroParamId`，避免与调用方同名标识符冲突（`SYX_MACRO_HYGIENE=0` 可关闭一版）

测试：`fixtures/macros/macro_engine.sy`、`fixtures/macros/hygiene_escape_ok.sy`

## @propertyWrapper（Phase 40 GA）

```syx
@propertyWrapper
class Twice {
    var wrappedValue: Int
    init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}

class User {
    @Twice var count: Int
}
```

CLI：`--no-property-wrapper` 可关闭。测试：`fixtures/macros/attached_wrapper.sy`

## @attached（Phase 50 真 AST 展开）

**Phase 50** 起 `body:` 经 `parse_fragment` 解析为真实语句/声明，不再仅生成 `print` stub。

```syx
@attached(member, named, greet, "print(\"member:{name}\")")
data class Greeter(name: String)

@attached(peer, names: prefixed(_describe))
data class Point(x: Int, y: Int)
```

- 角色：`member`、`peer`、`custom`
- 源内 `body:` 优先于 `SYX_ATTACHED_*_BODY` 环境变量（测试/模板用）

测试：`fixtures/macros/attached_wrapper.sy`、`fixtures/macros/hygiene_escape_ok.sy`、`fixtures/macros/attached_conformance_dispatch.sy`

## 证明宏（L1 GA · L2 partial，Phase 50–52 / 119）

在 `data class` 上自动生成协议 witness：

| 宏 | 生成内容 | 档位 |
|----|----------|------|
| `@Equatable` | `func ==(other:) -> Bool` | L1 **GA** |
| `@Hashable` | `func hash(into:)` → `__syx_hasher_combine` | L1 **GA** |
| `@Codable` | `encode()` + `static decode(text:)`（JSON Host） | L1 **GA**；**关联值 enum L2 部分 GA**（指纹 `macro-codable-l2:associated-enum-auto`）；generic witness **partial** |

```syx
@Codable
data class User(n: Int, name: String)

@Codable
enum WireStatus {
    ok(code: Int)
    err(message: String)
}

func main() {
    let u = User(n: 42, name: "syx")
    let text = u.encode()
    let back = User.decode(text: text)
    let w = WireStatus.ok(code: 7)
    let wj = w.encode()
    let w2 = WireStatus.decode(text: wj)
}
```

限制：L1 仅 **stored 属性** 的 `data class`；L2 关联值 enum 全自动往返 **GA（窄）**；泛型关联值全集 / 任意嵌套 / 自定义 coding keys → **131+**。测试：`fixtures/parity/dataclass_synth.sy`、`fixtures/parity/dataclass_synth.sy`、`fixtures/macros/codable_associated_enum.sy`、`codable_associated_enum.sy`

## `attached(conformance)` 诚实化（Phase 119）

未知协议 **不得静默忽略**（**E00x** 或显式非 GA 诊断）；Equatable/Hashable/Codable 路径保留。

## `#externalMacro`（Phase 51–52 → 119 plugin-prod）

进程内或插件进程通过 JSON 协议 `MacroExpandRequest` / `MacroExpandResponse` 展开：

```syx
macro ExternalSmoke = #externalMacro(module: "SyxMacros", type: "SmokeMemberMacro")

@ExternalSmoke
data class Point(x: Int, y: Int)
```

包 manifest 可声明 `[macros].exports` 注册外部宏；LSP v44 支持跳转与 expand preview。见 [ADR-112](../../../docs/adr/architecture-decisions.md)

测试：`fixtures/macros/json_plugin_prod.sy`

## @resultBuilder（Phase 119 · engine 管线 · Phase 123 HostCall+Closure 修栈）

`@resultBuilder` 在 **`syx-macro` engine** 固定点管线第六 pass 展开。支持 multi-stmt / nested `if`/`for`、`buildBlock` / `buildEither` / `buildArray`，以及 **`buildOptional`**。

Phase 123：literal + for-in + **for-in 内 HostCall**（如 `__syx_str_length`）经 closure 内联 **可编译运行**（指纹 `macro-pipeline-v6:rb-hostcall-closure-fixed`）；非法模式仍 fail-closed。

见 [架构决议 · 宏系统](../../../docs/adr/architecture-decisions.md#宏系统)。

## @attached 角色（Phase 53+）

| 角色 | 附着点 | 示例 |
|------|--------|------|
| `member` / `peer` / `custom` | 类型 | `@attached(member, named, greet, "...")` |
| **`accessor`** | 属性（extension 方法） | `@attached(extension, named, readValue, "func readValue() ...")` |
| **`deinit`** | class | `@attached(deinit, named, cleanup, "print(...)")` |
| **`extension`** | 类型 | `@attached(extension, named, label, "func label() ...")` |

JSON 协议 `role` 字段：`member` \| `accessor` \| `deinit` \| `extension`。

## 未实现 / 顺延（136+）

- 完整 token 语法宏（当前表达式/声明/语句位 ident/literal MVP + Phase 134 表达式树 / Phase 135 声明位加深；**非** token-stream API / 任意语法改写）
- 完整 AspectJ 式 AOP / 切面类 / 通配 join point（当前 `@Pointcut("Type.method")` + 窄 `execution(* T.m(..))` + `within(Type)` + `args(Type)` + `within && args` 名模式 MVP）
- 完整 Spring 容器
- 完整商业 OT 许可表级 attached 角色
- 用户用 SYX 源码编写展开逻辑
- `@Codable` L2 泛型关联值全集 / 任意嵌套 enum / 自定义 coding keys DSL

## 与 Swift 的差异

内置 + 有限用户宏 + L1 证明宏 + opt-in 外部宏 + M25 Service DI v16 + decl-token deepen / expr-token / stmt-token / decl-hygiene / decl-param MVP + `@Before`/`@After` pointcut 绑定 + `@Pointcut` / `execution` / `within(Type)` / `args(Type)` / `within && args` 名模式 MVP + `@Around` proceed + `@Bean` 参数注入 MVP + Codable L2 associated-enum + 诚实 partial 标注，非 Swift 完整可扩展卫生宏系统。

---

**下一章** → [类型转换](20-type-casting.md)

**参见** → [语言参考：特性](../language-reference/06-attributes.md) · [ADR-127](../../../docs/adr/architecture-decisions.md)
