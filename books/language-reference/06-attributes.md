# Attributes

**Implementation status:** **M28 + Service DI v19 (SYX 116.0)** — the full chapter is based on Phase 138; M28 multi-token decl rewrite MVP + `@Pointcut("within(*Type)")` type wildcards + DI v19 GA; Phase 132 declaration-position `#macro` hygiene MVP + `@Before`/`@After` pointcut string binding MVP; Phase 131 declaration-position parameter substitution MVP + `execution(* T.m(..))` name/`*` pattern MVP; Phase 130 token expression-position `#macro` MVP + `@Pointcut("Type.method")` method name MVP; Phase 129 `@Codable` L2 fully automatic associated enums + `@Bean` parameter `@Autowired`/type resolution MVP; Phase 128 `@Lazy` + lazy proxies for cyclic dependencies; Phase 127 advice proceed + `@Configuration`/`@Bean`; see [ADR-130](../../docs/adr/architecture-decisions.md) · documentation fingerprint `macro-docs-v20:m28-service-v19-honesty`

Attributes are written with an `@` prefix on declarations, and are expanded or registered by `syx-macro::expand_module` at a fixed point early in compilation (including the sixth pass for **resultBuilder**).

---

## Syntax

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

`Attribute { name, args }` is attached to functions, `data class`, `class` and other declarations.

## @attached

**Implementation status:** **Partially implemented** (Phase 50 real AST `body:` expansion; Phase 119 conformance no-silent-drop)

Generates extension methods or peer declarations; the `body:` string is parsed by `parse_fragment` into statements/declarations.

Roles: `member`, `peer`, `custom`, `accessor`, `extension`, `deinit`. `@freestanding(expression|statement|declaration)` attached at the wrong position → **E003**.

```syx
@attached(member, named, greet, "print(\"hi:{name}\")")
data class Greeter(name: String)
```

See ADR-010, 016, 018, 020; tests `fixtures/macros/attached_wrapper.sy`, `fixtures/macros/attached_conformance_dispatch.sy`.

### @attached Roles (Phase 53+)

| Role | Attachment point | Example |
|------|--------|------|
| `member` / `peer` / `custom` | Type | `@attached(member, named, greet, "...")` |
| `accessor` | Property (extension method) | `@attached(extension, named, readValue, "func readValue() ...")` |
| `deinit` | class | `@attached(deinit, named, cleanup, "print(...)")` |
| `extension` | Type | `@attached(extension, named, label, "func label() ...")` |

JSON protocol `role` field: `member` | `accessor` | `deinit` | `extension`.

## @Equatable / @Hashable / @Codable

**Implementation status:** **L1 GA · L2 partial** (built-in proof macros)

| Attribute | Description |
|------|------|
| `@Equatable` | Synthesizes `==` for a `data class` (Phase 50) — **GA** |
| `@Hashable` | Synthesizes `hash(into:)` (Phase 51–52) — **GA** |
| `@Codable` | Synthesizes `encode()` / `static decode(text:)` JSON (Phase 52 L1; Phase 129 L2 enum + generics **partial GA**) |

L1: `data class` with stored properties only. An unknown `attached(conformance)` protocol must not be silently dropped (Phase 119). See [Language Guide: Macros](../language-guide/19-macros.md) · [ADR-112](../../docs/adr/architecture-decisions.md).

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

**Implementation status:** **GA + partial MVP** (Phase 138 Service DI v19 — `@Pointcut("within(*Type)")` wildcard MVP + multi-token decl rewrite MVP; Phase 137 `!within` negation MVP; Phase 136 `within || args` compose MVP; Phase 135 `within && args` compose MVP; Phase 134 `args(Type)` first-parameter type name MVP; Phase 133 `within(Type)` type name MVP + stmt-token subst MVP; Phase 132 `@Before`/`@After` pointcut string binding MVP; Phase 131 `execution(* T.m(..))` name pattern MVP; Phase 130 `@Pointcut` method name MVP; Phase 129 `@Bean` parameter injection MVP; Phase 128 `@Lazy` + lazy proxy for cyclic dependencies MVP; Phase 127 `@Around` proceed + `@Configuration`/`@Bean` MVP; M16 protocol witness + advice chains; M15 base-type polymorphism + field Qualifier; M14 Qualifier/Primary/Profile + `Di.withRequest` inheritance)

| Attribute | Description |
|------|------|
| `@Service` / `@Component` | Marks a registrable type; **multi-package** scanning (only `resolved_packages`) + registry; superclass/protocol act as binding aliases; attaching to a non-type → **E006** |
| `@Inject` | **Constructor parameter** injection; base types/protocol names may also serve as dependency keys; missing or cyclic dependencies fail-closed |
| `@Autowired` | Field injection MVP (stored property, assigned after construction); may be combined with `@Qualifier("…")` so metadata survives across packages |
| `@Scope("prototype")` | A new instance per `resolve`; singleton by default |
| `@Scope("request")` | Singleton within a request frame, new instance across frames; prefer `Di.withRequest` |
| `@Qualifier("name")` | Disambiguating alias for multiple bindings (type level or field level) |
| `@Primary` | The default implementation when no Qualifier is given |
| `@Profile("name")` | Conditional registration; `Di.setProfile` / `SYX_DI_PROFILE` |
| `@PostConstruct` / `@PreDestroy` | Hooks after construction / at teardown · request end |
| `@Pointcut("Type.method")` | **partial GA (Phase 130 · DI v11)** — narrow string method-name pattern registration; fingerprint `macro-service-v11:pointcut-name-mvp` |
| `@Pointcut("execution(* T.m(..))")` | **partial GA (Phase 131 · DI v12)** — narrow method-name/`*` execution pattern; fingerprint `macro-service-v12:pointcut-execution-name-mvp` |
| `@Pointcut("within(Type)")` | **partial GA (Phase 133 · DI v14)** — narrow type-name pattern registration; fingerprint `macro-service-v14:pointcut-within-name-mvp` |
| `@Pointcut("within(*Type)")` / `within(T*)` | **partial GA (Phase 138 · DI v19)** — type-name wildcards; fingerprint `macro-service-v19:pointcut-within-wildcard-mvp` |
| `@Pointcut("args(Type)")` | **partial GA (Phase 134 · DI v15)** — narrow first-parameter type-name pattern; fingerprint `macro-service-v15:pointcut-args-name-mvp` |
| `@Pointcut("within(T) && args(U)")` | **partial GA (Phase 135 · DI v16)** — narrow `&&` composition |
| `@Pointcut("within(T) \|\| args(U)")` | **partial GA (Phase 136 · DI v17)** — narrow `\|\|` composition; fingerprint `macro-service-v17:pointcut-or-compose-mvp` |
| `@Pointcut("!within(Type)")` | **partial GA (Phase 137 · DI v18)** — negation; fingerprint `macro-service-v18:pointcut-not-within-mvp` |
| `@Around` / `@Around("…")` | **partial GA** — **only** for **instance methods** of `@Service`/`@Component`; ordered multi-interceptor chains; the string form binds a registered `@Pointcut` / execution pattern; **not** full AspectJ AOP |
| `@Before("…")` / `@After("…")` | **partial GA (Phase 132 · DI v13)** — binds a registered `@Pointcut` / execution name pattern; fingerprint `macro-service-v13:before-after-pointcut-bind-mvp` |
| `@Before` / `@After` (no string) | **MVP** — same scope as `@Around`; self-wrapping in declaration order |

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

> Full AspectJ AOP / a complete token syntax macro / full Spring → **139+**. See [19-macros.md](../language-guide/19-macros.md) · [ADR-130](../../docs/adr/architecture-decisions.md).

## @Lazy

**Implementation status:** **partial GA (Phase 128 · Service DI v9)**

Marks `@Service` / `@Component` / `@Bean` for lazy instantiation: the factory runs only when the materialization point — the first `Di.resolve` / injection site — is reached. A dependency cycle containing a `@Lazy` edge gets a lazy proxy handle inserted; a cycle without `@Lazy` still fails with **E007**. Fingerprint `macro-service-v9:lazy-cycle-proxy-mvp`. **Not** arbitrary type proxies / CGLIB / the full set of scoped proxies.

```syx
@Lazy
@Service
class Expensive {
    init() { print("factory") }
}
```

## @Bean Parameter Injection

**Implementation status:** **partial GA (Phase 129 · Service DI v10)**

`@Bean` method parameters inside a `@Configuration` are injected by type / `@Autowired` (optionally `@Qualifier`) from the graph of registered `@Service`/`@Bean`s. Fingerprint `macro-service-v10:bean-param-autowire-mvp`.

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

**Implementation status:** **partial GA (Phase 138 · Service DI v19)**

`@Pointcut("Type.method")`, `@Pointcut("execution(* Type.method(..))")` / `execution(* Type.*(..))`, `@Pointcut("within(Type)")` / `within(*Type)` / `within(T*)`, `@Pointcut("args(Type)")`, or `@Pointcut("within(T) && args(U)")` / `within(T) || args(U)` / `!within(T)` register a narrow name/type pattern on a `@Service`/`@Component` method; `@Around("…")` / `@Before("…")` / `@After("…")` bind proceed / hook chains. Full AspectJ → **139+**.

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

## Token Expression-Position `#macro` MVP

**Implementation status:** **partial GA (Phase 134 · M21+)**

An expression-position `#macro` performs textual substitution of ident/literal parameters; Phase 134 deepens the expression-tree walk. Fingerprints `macro-token-v1:expr-token-mvp` · `macro-token-v5:expr-token-subst-mvp`. **Not** a complete token syntax macro. See [19-macros.md](../language-guide/19-macros.md).

## Statement-Position `#macro` token subst MVP

**Implementation status:** **partial GA (Phase 133 · M23)**

A statement-position freestanding `#macro` performs textual substitution of ident/literal parameters. Fingerprint `macro-token-v4:stmt-token-subst-mvp`. **Not** a complete token syntax macro / token-stream API → **139+**. See [19-macros.md](../language-guide/19-macros.md) · `examples/stmt_token_macro/`.

## Declaration-Position `#macro` Parameter Substitution MVP

**Implementation status:** **partial GA (Phase 131 · M21)**

A declaration-position freestanding `#macro` performs textual substitution of its parameters. Fingerprint `macro-token-v2:decl-param-subst-mvp`. **Not** a complete token syntax macro → **139+**.

## Declaration-Position `#macro` Hygiene MVP

**Implementation status:** **partial GA (Phase 132 · M22)**

Declaration-position freestanding `#macro` hygiene, plus closure of linked `run_file` / `compile_linked`. Fingerprint `macro-token-v3:decl-hygiene-mvp`. **Not** a complete token syntax macro / token-stream API → **139+**.

## Declaration-Position `#macro` decl-expand MVP

**Implementation status:** **partial GA (Phase 137 · M27)**

A single declaration-position `Ident` token can expand into a rename/wrap function decl. Fingerprint `macro-token-v8:decl-expand-mvp`. **Not** a full token stream / user SYX expansion → **138+**.

## Declaration-Position `#macro` Multi-Token decl rewrite MVP

**Implementation status:** **partial GA (Phase 138 · M28)**

The declaration-position `Ident` + `Punct` + `Group` token stream can be rewritten into a rename/wrap function decl. Fingerprint `macro-token-v9:token-rewrite-mvp`. **Not** a full token stream / user SYX expansion → **139+**.

## @Codable (L2 associated enums)

**Implementation status:** **partial GA (Phase 129 · M19)**

An `enum` with associated values automatically synthesizes `encode`/`decode` JSON round-tripping. Fingerprint `macro-codable-l2:associated-enum-auto`. **Not** the full set of generic associated values / arbitrary nesting / custom coding keys.

## @propertyWrapper

**Implementation status:** **Implemented** (Phase 40 GA, **deprecated W001**; migrate to `@attached(wrapper)`)

```syx
@propertyWrapper
class Clamped {
    var wrappedValue: Int
    init(wrappedValue: Int) { self.wrappedValue = wrappedValue }
}
```

CLI: `--no-property-wrapper` turns it off.

## @resultBuilder

**Implementation status:** **GA** (Phase 119: sixth pass of the `syx-macro` engine fixed-point pipeline; `buildOptional` + multi-statement + literal+for-in+HostCall+closure mixtures compile and run as of Phase 123)

A statement block is desugared into container construction (e.g. `Array`). The earlier driver-only `desugar_result_builders` has moved into the engine.

## @unchecked Sendable

**Implementation status:** **Partially implemented**

Suppresses part of the Sendable checking; misuse may cause data races. **C003** in strict mode E2E.

## requires Pure / IO

Function effect annotations (not `@` attributes, but declaration modifiers):

```syx
func calc(x: Int) -> Int requires Pure { … }
func load() -> String requires IO { … }
```

**Implementation status:** **Partially implemented** (`syx-meta` MVP)

## Compile-Time Macros (`#` prefix, not `@`)

Freestanding macros are invoked in statement or expression position:

| Macro | Description |
|----|------|
| `#assert` / `#assert_eq` / `#assert_ne` | Assertions |
| `#if` | Conditional compilation |
| `#while` / `#for` | Loop unrolling |
| `#stringify(expr)` | Expression → string literal (Phase 50) |
| `#warning` / `#error` | W001 / E001 |
| `#compileTime` | Constant-evaluation subset |
| `#macro name(params) { … }` | User freestanding macro |

External macro declarations (Phase 51+):

```syx
macro MyMacro = #externalMacro(module: "Pkg", type: "MyMacro")
```

An unknown `#name` (not a registered macro) → compile error. Exceeding the expansion depth → **E002**.

---

**Diagnostics:** E002 / E003 / E004 / E006 / E007 / E008 / E010 / W001

**Usage examples** → [Language Guide: Macros](../language-guide/19-macros.md)

**See also** → [shared/implementation-status.md](../shared/implementation-status.md) · [ADR-130](../../docs/adr/architecture-decisions.md)
