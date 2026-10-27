# 特性（Attributes）

**实现状态：** **M28 + Service DI v19（SYX 116.0）** — 完整章节基于 Phase 138；M28 multi-token decl rewrite MVP + `@Pointcut("within(*Type)")` 类型通配 + DI v19 GA；Phase 132 声明位 `#macro` 卫生 MVP + `@Before`/`@After` pointcut 字符串绑定 MVP；Phase 131 声明位参数替换 MVP + `execution(* T.m(..))` 名/`*` 模式 MVP；Phase 130 token 表达式位 `#macro` MVP + `@Pointcut("Type.method")` 方法名 MVP；Phase 129 `@Codable` L2 associated-enum 全自动 + `@Bean` 参数 `@Autowired`/类型解析 MVP；Phase 128 `@Lazy` + 环依赖懒代理；Phase 127 advice proceed + `@Configuration`/`@Bean`；见 [ADR-130](../../../docs/adr/architecture-decisions.md) · 文档指纹 `macro-docs-v20:m28-service-v19-honesty`

特性以 `@` 前缀标注声明，在编译早期由 `syx-macro::expand_module` 固定点展开或登记（含 **resultBuilder** 第六 pass）。

---

## 语法

```syx
@attached(peer, names: prefixed(_))
@Equatable
@unchecked Sendable
data class User(name: String)
```

```
attribute-decl ::= '@' identifier (attribute-args)?
attribute-args ::= '(' (attribute-arg (',' attribute-arg)*)? ')'
attribute-arg  ::= identifier (':' expr | 'as' type)?
                |  (':' expr)            -- positional
                |  ('as' type)            -- type cast
```

`Attribute { name, args }` 附着于函数、`data class`、`class` 等声明。

## @attached

**实现状态：** **部分实现**（Phase 50 真 AST `body:` 展开；Phase 119 conformance no-silent-drop）

生成 extension 方法或 peer 声明；`body:` 字符串经 `parse_fragment` 解析为语句/声明。

角色（role）：`member`、`peer`、`custom`、`accessor`、`extension`、`deinit`。`@freestanding(expression|statement|declaration)` 附着点错位 → **E003**。

```syx
@attached(member, named, greet, "print(\"hi:{name}\")")
data class Greeter(name: String)
```

参见 ADR-010、016、018、020；测试 `fixtures/macros/attached_wrapper.sy`、`fixtures/macros/attached_conformance_dispatch.sy`。

### @attached 角色（Phase 53+）

| 角色 | 附着点 | 示例 |
|------|--------|------|
| `member` / `peer` / `custom` | 类型 | `@attached(member, named, greet, "...")` |
| `accessor` | 属性（extension 方法） | `@attached(extension, named, readValue, "func readValue() ...")` |
| `deinit` | class | `@attached(deinit, named, cleanup, "print(...)")` |
| `extension` | 类型 | `@attached(extension, named, label, "func label() ...")` |

JSON 协议 `role` 字段：`member` | `accessor` | `deinit` | `extension`。

## @Equatable / @Hashable / @Codable

**实现状态：** **L1 GA · L2 partial**（builtin 证明宏）

| 特性 | 说明 |
|------|------|
| `@Equatable` | 为 `data class` 合成 `==`（Phase 50）— **GA** |
| `@Hashable` | 合成 `hash(into:)`（Phase 51–52）— **GA** |
| `@Codable` | 合成 `encode()` / `static decode(text:)` JSON（Phase 52 L1；Phase 129 L2 enum + generic **partial GA**） |

L1：仅 stored 属性的 `data class`。未知 `attached(conformance)` 协议不得静默忽略（Phase 119）。见 [语言指南：宏](../language-guide/19-macros.md) · [ADR-112](../../../docs/adr/architecture-decisions.md)。

```syx
@Equatable
@Hashable
@Codable
data class User(n: Int, name: String)

@Codable
enum WireStatus {
    case ok(code: Int)
    case err(message: String)
}
```

## @Service / @Component / @Inject / @Autowired / @Scope / @PreDestroy / @Qualifier / @Primary / @Profile / @Around / @Before / @After / @Pointcut

**实现状态：** **GA + partial MVP**（Phase 138 Service DI v19 — `@Pointcut("within(*Type)")` 通配 MVP + multi-token decl rewrite MVP；Phase 137 `!within` 否定 MVP；Phase 136 `within || args` compose MVP；Phase 135 `within && args` compose MVP；Phase 134 `args(Type)` 首参类型名 MVP；Phase 133 `within(Type)` 类型名 MVP + stmt-token subst MVP；Phase 132 `@Before`/`@After` pointcut 字符串绑定 MVP；Phase 131 `execution(* T.m(..))` 名模式 MVP；Phase 130 `@Pointcut` 方法名 MVP；Phase 129 `@Bean` 参数注入 MVP；Phase 128 `@Lazy` + 环依赖懒代理 MVP；Phase 127 `@Around` proceed + `@Configuration`/`@Bean` MVP；M16 协议 witness + advice 链；M15 基类型多态 + 字段 Qualifier；M14 Qualifier/Primary/Profile + `Di.withRequest` 继承）

| 特性 | 说明 |
|------|------|
| `@Service` / `@Component` | 标记可注册类型；**多包**扫描（仅 `resolved_packages`）+ registry；superclass/protocol 作 binding alias；非类型附着 → **E006** |
| `@Inject` | **构造参数** 注入；基类型/协议名亦可作依赖键；缺依赖 / 环依赖 fail-closed |
| `@Autowired` | 字段注入 MVP（stored property，构造后赋值）；可叠 `@Qualifier("…")` 跨包保留元数据 |
| `@Scope("prototype")` | 每次 `resolve` 新实例；默认 singleton |
| `@Scope("request")` | 请求帧内单例、跨帧新实例；优先 `Di.withRequest` |
| `@Qualifier("name")` | 多绑定消歧别名（类型级或字段级） |
| `@Primary` | 无 Qualifier 时的默认实现 |
| `@Profile("name")` | 条件注册；`Di.setProfile` / `SYX_DI_PROFILE` |
| `@PostConstruct` / `@PreDestroy` | 构造后 / teardown·request end 钩子 |
| `@Pointcut("Type.method")` | **部分 GA（Phase 130 · DI v11）** — 窄字符串方法名模式注册；指纹 `macro-service-v11:pointcut-name-mvp` |
| `@Pointcut("execution(* T.m(..))")` | **部分 GA（Phase 131 · DI v12）** — 方法名/`*` 窄 execution 模式；指纹 `macro-service-v12:pointcut-execution-name-mvp` |
| `@Pointcut("within(Type)")` | **部分 GA（Phase 133 · DI v14）** — 窄类型名模式注册；指纹 `macro-service-v14:pointcut-within-name-mvp` |
| `@Pointcut("within(*Type)")` / `within(T*)` | **部分 GA（Phase 138 · DI v19）** — 类型名通配；指纹 `macro-service-v19:pointcut-within-wildcard-mvp` |
| `@Pointcut("args(Type)")` | **部分 GA（Phase 134 · DI v15）** — 窄首参类型名模式；指纹 `macro-service-v15:pointcut-args-name-mvp` |
| `@Pointcut("within(T) && args(U)")` | **部分 GA（Phase 135 · DI v16）** — `&&` 窄组合 |
| `@Pointcut("within(T) \|\| args(U)")` | **部分 GA（Phase 136 · DI v17）** — `\|\|` 窄组合；指纹 `macro-service-v17:pointcut-or-compose-mvp` |
| `@Pointcut("!within(Type)")` | **部分 GA（Phase 137 · DI v18）** — 否定；指纹 `macro-service-v18:pointcut-not-within-mvp` |
| `@Around` / `@Around("…")` | **部分 GA** — **仅** `@Service`/`@Component` **实例方法**；有序多 interceptor 链；字符串形式绑定已注册 `@Pointcut` / execution 模式；**非**完整 AspectJ AOP |
| `@Before("…")` / `@After("…")` | **部分 GA（Phase 132 · DI v13）** — 绑定已注册 `@Pointcut` / execution 名模式；指纹 `macro-service-v13:before-after-pointcut-bind-mvp` |
| `@Before` / `@After`（无字符串） | **MVP** — 同 `@Around` 作用域；声明序自包装 |

```syx
import Di

@Qualifier("fast")
@Service
class FastLogger {
    init() {}
}

@Primary
@Service
class SlowLogger {
    init() {}
}

@Profile("prod")
@Service
class ProdOnly {
    init() {}
}

@Scope("request")
@Service
class Worker {
    init() {}
    @Pointcut("Worker.handle")
    @Around("Worker.handle")
    func handle() -> Int {
        return 1
    }
    @PreDestroy
    func cleanup() { print("bye") }
}

@Around("within(*Service)")
class Aspect {
    init() {}
    func audit() { print("audit") }
}

func handle() {
    let _ = Di.resolve("Worker")
}

func main() {
    Di.setProfile("prod")
    Di.withRequest("handle")
    Di.destroy()
}
```

> 完整 AspectJ AOP / 完整 token 语法宏 / 完整 Spring → **139+**。见 [19-macros.md](../language-guide/19-macros.md) · [ADR-130](../../../docs/adr/architecture-decisions.md)。

## @Lazy

**实现状态：** **部分 GA（Phase 128 · Service DI v9）**

标记 `@Service` / `@Component` / `@Bean` 延迟实例化：工厂在首次 `Di.resolve` / 注入点物化时才运行。含 `@Lazy` 边的环依赖插入懒代理句柄；无 `@Lazy` 环仍 **E007**。指纹 `macro-service-v9:lazy-cycle-proxy-mvp`。**非**任意类型代理 / CGLIB / 作用域代理全集。

```syx
@Lazy
@Service
class Expensive {
    init() { print("factory") }
}
```

## @Bean 参数注入

**实现状态：** **部分 GA（Phase 129 · Service DI v10）**

`@Configuration` 内 `@Bean` 方法参数按类型 / `@Autowired`（可选 `@Qualifier`）从已注册 `@Service`/`@Bean` 图注入。指纹 `macro-service-v10:bean-param-autowire-mvp`。

```syx
@Configuration
class AppConfig {
    init() {}
    @Bean
    func greeter(@Autowired logger: DepLogger) -> BeanGreeter {
        return BeanGreeter(logger: logger)
    }
}
```

## @Pointcut

**实现状态：** **部分 GA（Phase 138 · Service DI v19）**

`@Pointcut("Type.method")` 或 `@Pointcut("execution(* Type.method(..))")` / `execution(* Type.*(..))` 或 `@Pointcut("within(Type)")` / `within(*Type)` / `within(T*)` 或 `@Pointcut("args(Type)")` 或 `@Pointcut("within(T) && args(U)")` / `within(T) || args(U)` / `!within(T)` 在 `@Service`/`@Component` 方法上注册窄名/类型模式；`@Around("…")` / `@Before("…")` / `@After("…")` 绑定 proceed / hook 链。完整 AspectJ → **139+**。

```syx
@Service
class Greeter {
    init() {}
    @Pointcut("execution(* Greeter.greet(..))")
    @Around("execution(* Greeter.greet(..))")
    func greet(name: String) -> String {
        return "hello " + name
    }
}
```

## Token 表达式位 `#macro` MVP

**实现状态：** **部分 GA（Phase 134 · M21+）**

表达式位 `#macro` 对 ident/literal 参数做文本替换；Phase 134 表达式树 walk 加深。指纹 `macro-token-v1:expr-token-mvp` · `macro-token-v5:expr-token-subst-mvp`。**非**完整 token 语法宏。见 [19-macros.md](../language-guide/19-macros.md)。

## 语句位 `#macro` token subst MVP

**实现状态：** **部分 GA（Phase 133 · M23）**

语句位 freestanding `#macro` 对 ident/literal 参数做文本替换。指纹 `macro-token-v4:stmt-token-subst-mvp`。**非**完整 token 语法宏 / token-stream API → **139+**。见 [19-macros.md](../language-guide/19-macros.md) · `examples/stmt_token_macro/`。

## 声明位 `#macro` 参数替换 MVP

**实现状态：** **部分 GA（Phase 131 · M21）**

声明位 freestanding `#macro` 对参数做文本替换。指纹 `macro-token-v2:decl-param-subst-mvp`。**非**完整 token 语法宏 → **139+**。

## 声明位 `#macro` 卫生 MVP

**实现状态：** **部分 GA（Phase 132 · M22）**

声明位 freestanding `#macro` 卫生 + linked `run_file` / `compile_linked` 收口。指纹 `macro-token-v3:decl-hygiene-mvp`。**非**完整 token 语法宏 / token-stream API → **139+**。

## 声明位 `#macro` decl-expand MVP

**实现状态：** **部分 GA（Phase 137 · M27）**

声明位单 `Ident` token 可 expand 为 rename/wrap 函数 decl。指纹 `macro-token-v8:decl-expand-mvp`。**非**完整 token-stream / 用户 SYX 展开 → **138+**。

## 声明位 `#macro` multi-token decl rewrite MVP

**实现状态：** **部分 GA（Phase 138 · M28）**

声明位 `Ident` + `Punct` + `Group` token 流可 rewrite 为 rename/wrap 函数 decl。指纹 `macro-token-v9:token-rewrite-mvp`。**非**完整 token-stream / 用户 SYX 展开 → **139+**。

## @Codable（L2 associated-enum）

**实现状态：** **部分 GA（Phase 129 · M19）**

带关联值的 `enum` 自动合成 `encode`/`decode` JSON 往返。指纹 `macro-codable-l2:associated-enum-auto`。**非**泛型关联值全集 / 任意嵌套 / 自定义 coding keys。

## @propertyWrapper

**实现状态：** **已实现**（Phase 40 GA，**deprecated W001**；请迁移 `@attached(wrapper)`）

```syx
@propertyWrapper
class Clamped {
    var wrappedValue: Int
    init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}
```

CLI：`--no-property-wrapper` 关闭。

## @resultBuilder

**实现状态：** **GA**（Phase 119：`syx-macro` engine 固定点管线第六 pass；`buildOptional` + multi-stmt + literal+for-in+HostCall+closure 混用可编译运行 Phase 123）

语句块 desugar 为容器构建（如 `Array`）。此前 driver-only `desugar_result_builders` 已迁入 engine。

## @unchecked Sendable

**实现状态：** **部分实现**

抑制部分 Sendable 检查；滥用可能导致数据竞争。**C003** 在 strict 模式 E2E。

## requires Pure / IO

函数效果注解（非 `@` 特性，但属声明修饰）：

```syx
func calc(x: Int) -> Int requires Pure { … }
func load() -> String requires IO { … }
```

**实现状态：** **部分实现**（`syx-meta` MVP）

## 编译期宏（`#` 前缀，非 `@`）

Freestanding 宏在语句或表达式位置调用：

| 宏 | 说明 |
|----|------|
| `#assert` / `#assert_eq` / `#assert_ne` | 断言 |
| `#if` | 条件编译 |
| `#while` / `#for` | 循环展开 |
| `#stringify(expr)` | 表达式 → 字符串字面量（Phase 50） |
| `#warning` / `#error` | W001 / E001 |
| `#compileTime` | 常量求值子集 |
| `#macro name(params) { … }` | 用户 freestanding 宏 |

外部宏声明（Phase 51+）：

```syx
macro MyMacro = #externalMacro(module: "Pkg", type: "MyMacro")
```

未知 `#name`（非已注册宏）→ 编译错误。展开深度超限 → **E002**。

---

**诊断：** E002 / E003 / E004 / E006 / E007 / E008 / E010 / W001

**用法示例** → [语言指南：宏](../language-guide/19-macros.md)

**参见** → [shared/implementation-status.md](../../shared/implementation-status.md) · [ADR-130](../../../docs/adr/architecture-decisions.md)
