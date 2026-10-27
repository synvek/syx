# Macros

**Implementation status:** **M28 + Service DI v19 (SYX 116.0)** — see [ADR-130](../../docs/adr/architecture-decisions.md) · [Architecture Decisions](../../docs/adr/architecture-decisions.md#macro-system) · documentation fingerprint `macro-docs-v20:m28-service-v19-honesty`

## SYX 116.0: Macro M28 + Service DI v19 (GA vs partial)

| Capability | Tier | Description |
|------|------|------|
| **Declaration-position `#macro` multi-token decl rewrite MVP** | **partial GA** | A declaration-position `Ident`+`Punct`+`Group` token stream can be rewritten into a rename/wrap function decl (fingerprint `macro-token-v9:token-rewrite-mvp`); **not** a complete token-stream / user SYX expansion logic → **139+** |
| **`@Pointcut("within(*Service)")` / `within(Foo*)` type-name wildcard MVP** | **partial GA** | `@Pointcut("within(*Service)")` suffix / `within(Foo*)` prefix narrow wildcards (fingerprint `macro-service-v19:pointcut-within-wildcard-mvp`); **not** full AspectJ expressions / aspect classes → **139+** |

## SYX 115.0: Macro M27 + Service DI v18 (GA vs partial)

| Capability | Tier | Description |
|------|------|------|
| **Declaration-position `#macro` decl-expand MVP** | **partial GA** | A single declaration-position `Ident` token can expand into a rename/wrap function decl (fingerprint `macro-token-v8:decl-expand-mvp`); **not** a complete token-stream / user SYX expansion logic → **138+** |
| **`@Pointcut("!within(Type)")` negation MVP** | **partial GA** | `@Pointcut("!within(Foo)")` narrow negated type-name pattern (fingerprint `macro-service-v18:pointcut-not-within-mvp`); **not** full AspectJ expressions / aspect classes → **138+** |

## SYX 114.0: Macro M26 + Service DI v17 (GA vs partial)

| Capability | Tier | Description |
|------|------|------|
| Freestanding builtin (`#assert` `#if` `#for` `#stringify` `#compileTime`) | **GA** | `syx-macro` builtin pass; role misplacement → **E003** (inherited from M9) |
| User `#macro` + Hygiene v2 | **partial** | Statement-position text substitution; expression/declaration-position parameter substitution + declaration-position hygiene MVP below |
| **Token expression-position `#macro` MVP** | **partial GA** | Expression-position `#macro` ident/literal parameter substitution; Phase 134 deepened the expression tree (fingerprint `macro-token-v5:expr-token-subst-mvp`); Phase 130 initial version (`macro-token-v1:expr-token-mvp`); **not** a complete token syntax macro |
| **Declaration-position `#macro` token-stream enumeration MVP** | **partial GA** | Declaration-position `#macro` parameters can be enumerated as an `Ident` / `Punct` / `Group` tri-state token list (fingerprint `macro-token-v7:token-stream-mvp`); **not** a complete token-stream / user SYX expansion logic → **137+** |
| **Declaration-position `#macro` token subst deepening MVP** | **partial GA** | Deepened ident/literal substitution of declaration-position freestanding `#macro` function names / parameter defaults / enum payloads (fingerprint `macro-token-v6:decl-token-subst-mvp`); Phase 131 initial version (`macro-token-v2:decl-param-subst-mvp`); **not** a complete token syntax macro |
| **Declaration-position `#macro` parameter substitution MVP** | **partial GA** | Declaration-position freestanding `#macro` parameter text substitution (fingerprint `macro-token-v2:decl-param-subst-mvp`); **not** a complete token syntax macro |
| **Declaration-position `#macro` hygiene + linked/run_file closeout MVP** | **partial GA** | Declaration-position hygiene (`macro-token-v3:decl-hygiene-mvp`); `compile_linked` / `run_file_printed` are reproducible; **not** a complete token syntax macro |
| **Statement-position `#macro` token subst MVP** | **partial GA** | Statement-position freestanding `#macro` ident/literal parameter text substitution (fingerprint `macro-token-v4:stmt-token-subst-mvp`); **not** a complete token syntax macro / token-stream API |
| `@attached` member/peer/accessor/extension/deinit | **partial** | Real AST `parse_fragment`; not the full Swift role surface |
| `@Equatable` / `@Hashable` / `@Codable` L1 | **GA** | stored `data class` |
| **`@Codable` L2 associated-value enum fully automatic** | **partial GA** | Associated-value enum `encode`/`decode` round-tripping (fingerprint `macro-codable-l2:associated-enum-auto`); **not** the full set of generic associated values / arbitrary nesting depth / a custom coding keys DSL |
| `#externalMacro` plugin-prod | **partial** | cdylib JSON production path; Linux seccomp hard gate / Darwin `reexec-only` honest label |
| `@resultBuilder` | **GA** | Sixth pass of the engine fixed-point pipeline; literal+for-in mixed with **HostCall+Closure** **compiles and runs** (`macro-pipeline-v6:rb-hostcall-closure-fixed`) |
| Macro package `[macros].exports` | **GA** | Phase 120 schema aligned with `SyxManifest` |
| `@Service` / `@Component` + `@Inject` ctor + field `@Autowired` | **GA** | **Multi-package workspace scanning** (only `resolved_packages`) + real ctor resolve; singleton by default (inherited from M14) |
| `@Scope("prototype")` / `@Scope("request")` | **GA** | prototype creates a new instance each time; request is a singleton within a frame (inherited from M13) |
| `@Qualifier` / `@Primary` | **GA** | Disambiguating multiple bindings (fingerprint `macro-service-v5:conditional-qualifier`) |
| `@Profile` | **GA** | Conditional registration; `Di.setProfile` / `SYX_DI_PROFILE` (fingerprint `macro-service-v5:conditional-profile`) |
| `@PostConstruct` / `@PreDestroy` hook MVP | **GA** | After construction + field injection / at container destroy·reset·request end |
| Typed resolve: `"Type"` / `Type` ident | **GA** | A string literal or a registered type identifier; a dynamic variable → **E010** |
| DI ceremony sugar + `Di` package GA | **GA** | `import Di` → `resolve` / `withRequest` / `setProfile` / …; short sugar `resolve(...)` (inherited from M14) |
| **Base-type polymorphic injection** | **GA** | Constructor/field `@Inject` resolves the concrete implementation by **base/parent type**; registry binds a superclass alias (fingerprint `macro-service-v6:polymorphic-base-inject`); **must not** rely on init side effects to fake dispatch |
| **Protocol-typed injection / witness table** | **GA** | conformance witness registers the protocol ← `@Service` implementation (fingerprint `runtime-service-v7:protocol-witness`); M15 alias binding inherited |
| **Field `@Qualifier` metadata** | **GA** | `@Autowired @Qualifier("…")` preserves disambiguating metadata across packages (fingerprint `macro-service-v6:field-qualifier-metadata`) |
| **`@Around` advice proceed + `@Before`/`@After`** | **partial GA** | **Only** **instance methods** of `@Service`/`@Component`; ordered multi-`@Around` **proceed delegation** (`__syx_proceed_N`) + Before/After; may bind to `@Pointcut` / `execution` name patterns (fingerprint `macro-service-v8:advice-proceed`); **not** AspectJ / aspect classes |
| **`@Pointcut` method name MVP** | **partial GA** | `@Pointcut("Type.method")` registers a narrow string pattern; `@Around("…")` binds the proceed chain (fingerprint `macro-service-v11:pointcut-name-mvp`); **not** AspectJ expressions / aspect classes / wildcard join points |
| **`execution(* T.m(..))` method-name pattern MVP** | **partial GA** | `@Pointcut("execution(* Type.method(..))")` / `execution(* Type.*(..))` name/`*` patterns (fingerprint `macro-service-v12:pointcut-execution-name-mvp`); **not** complete AspectJ |
| **`@Before`/`@After` pointcut string binding MVP** | **partial GA** | `@Before("…")` / `@After("…")` bind a registered `@Pointcut` / execution name pattern (fingerprint `macro-service-v13:before-after-pointcut-bind-mvp`); bare Before/After still self-wrap in declaration order; **not** complete AspectJ |
| **`@Pointcut("within(Type)")` type-name pattern MVP** | **partial GA** | `@Pointcut("within(Foo)")` registers a narrow type-name pattern; `@Around`/`@Before`/`@After` may bind any method on that type (fingerprint `macro-service-v14:pointcut-within-name-mvp`); **not** AspectJ expressions / aspect classes |
| **`@Pointcut("args(Type)")` first-parameter type-name pattern MVP** | **partial GA** | `@Pointcut("args(Int)")` registers a narrow first-parameter type-name pattern; `@Around`/`@Before`/`@After` may bind methods whose first parameter type matches (fingerprint `macro-service-v15:pointcut-args-name-mvp`); **not** full AspectJ expressions / aspect classes |
| **`@Pointcut("within(T) && args(U)")` narrow composition MVP** | **partial GA** | `@Pointcut("within(Foo) && args(Int)")` only `within` + `args` name patterns composed with `&&`; it matches only when both hit (fingerprint `macro-service-v16:pointcut-and-compose-mvp`); **not** negation / full AspectJ expressions / aspect classes |
| **`@Pointcut("within(T) \|\| args(U)")` narrow composition MVP** | **partial GA** | `@Pointcut("within(Foo) \|\| args(Int)")` only `within` + `args` name patterns composed with `\|\|`; it matches when either side hits (fingerprint `macro-service-v17:pointcut-or-compose-mvp`); **not** negation / full AspectJ expressions / aspect classes / Spring → **137+** |
| **`@Pointcut("!within(Type)")` negation MVP** | **partial GA** | `@Pointcut("!within(Foo)")` narrow negated type-name pattern (fingerprint `macro-service-v18:pointcut-not-within-mvp`); **not** full AspectJ expressions / aspect classes |
| **Declaration-position `#macro` decl-expand MVP** | **partial GA** | A single declaration-position `Ident` token can expand into a rename/wrap function decl (fingerprint `macro-token-v8:decl-expand-mvp`); **not** a complete token-stream / user SYX expansion → **138+** |
| **`@Configuration` / `@Bean` MVP** | **partial GA** | Configuration-class factory methods register with DI; `Di.resolve` can resolve them (fingerprint `macro-service-v8:configuration-bean-mvp`) |
| **`@Lazy` + cyclic-dependency lazy proxy MVP** | **partial GA** | Defers the factory until the first resolve/use; a cycle containing a `@Lazy` edge gets a lazy proxy inserted; a cycle without `@Lazy` is still **E007** (fingerprint `macro-service-v9:lazy-cycle-proxy-mvp`); **not** arbitrary type proxies / CGLIB / the full set of scoped proxies |
| **`@Bean` parameter `@Autowired` / type resolution MVP** | **partial GA** | `@Bean` method parameters inside a `@Configuration` are injected by type from the registered `@Service`/`@Bean` graph (fingerprint `macro-service-v10:bean-param-autowire-mvp`); **not** the full set of conditional assembly / CGLIB / the full set of scoped proxies |
| Complete token syntax macros / complete AspectJ / complete Spring | **Not implemented** | **137+** |

Tests: `crates/syx-driver/tests/fixtures/macros/` · `examples/service_di/` · `examples/stmt_token_macro/` · `examples/decl_param_macro/` · `examples/token_expr_macro/` · `within_wildcard_pointcut.sy` · `not_within_pointcut.sy` · `or_pointcut.sy` · `and_pointcut.sy` · `decl_token.sy` · `expr_token.sy` · `within_pointcut.sy` · `stmt_token.sy` · `macro_di_within_wildcard.rs` · `macro_di_not_within.rs` · `macro_di_or_pointcut.rs` · `service_di_and_pointcut.rs` · `token_decl_deepen.rs` · `task_queue_caller_runs.rs` · `bounded_task_queue.rs`

SYX macros come in two kinds: **freestanding** (`#`) and **attached** (`@`). Driven by `engine::expand_to_fixed_point` (builtin → attached → **wrapper** → user → **external** → **resultBuilder** → **service DI**).

## `@Service` / `@Component` / `@Inject` / `@Autowired` (Phase 136 · Service DI v17)

A Spring-style **demonstrable subset**: singleton (default) + prototype + **request** scope + **Qualifier/Primary/Profile** + **base-type polymorphic injection** + **protocol witness table** + field `@Qualifier` metadata + **`@Around` proceed delegation / `@Before`/`@After` (including string pointcut binding)** + **`@Pointcut("Type.method")` method-name MVP** + **`execution(* T.m(..))` name-pattern MVP** + **`@Pointcut("within(Type)")` type-name-pattern MVP** + **`@Pointcut("args(Type)")` first-parameter type-name-pattern MVP** + **`@Pointcut("within(T) && args(U)")` / `within(T) || args(U)` narrow composition MVP** + **`@Configuration`/`@Bean` MVP** + **`@Bean` parameter injection MVP** + **`@Lazy` + cyclic-dependency lazy proxy MVP** + constructor/field injection + multi-package workspace scanning + `Di.withRequest`. Do not claim Spring compatibility / complete AspectJ AOP / complete token syntax macros / fully automatic wiring.

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

- **`@Service` / `@Component`** — attached attributes marking a registrable type; used on a non-type → **E006**
- **Multi-package workspace scanning** — the driver aggregates `@Service`s inside `resolved_packages`; **skips** sibling nested `syx.toml` directories
- **`@Scope("prototype")`** — creates a new instance on each `resolve`
- **`@Scope("request")`** — the same instance within a request frame, a new instance across frames; prefer `Di.withRequest(handler)`, or `Di.beginRequest` / `Di.endRequest`
- **`@Qualifier("name")` / `@Primary`** — disambiguates multiple implementations; ambiguity without Primary/Qualifier → fail-closed
- **`@Profile("name")`** — registered only when `Di.setProfile` / `SYX_DI_PROFILE` matches
- **`@Inject`** — **constructor parameter** injection; a missing dependency fails closed (**E008**); a dependency cycle without `@Lazy` is still **E007**
- **Field `@Autowired` MVP** — stored property; assigned on the instance after resolve (no full reflection set)
- **`@PostConstruct` / `@PreDestroy` MVP** — lifecycle hooks
- **User-facing container** — `import Di`: `Di.resolve` / `Di.withRequest` / `Di.setProfile` / `Di.destroy`; short sugar `resolve(...)`; **application code must not call `__syx_service_*` directly**
- **Base-type polymorphic injection** — `@Inject logger: Logger` resolves to the concrete implementation when `FastLogger: Logger` is registered (registry superclass alias); init side effects no longer stand in for dispatch
- **Field `@Qualifier`** — `@Autowired @Qualifier("fast") let logger: Logger` preserves disambiguating metadata across packages
- **`@Around` proceed** — **only** **instance methods** of `@Service`/`@Component`; expands into ordered proceed delegation (`__syx_proceed_N`) wrapping the original method body + `print("__around__:method")`; the return value is observable; may stack `@Around("Type.method")` bound to a registered `@Pointcut`
- **`@Pointcut` method-name MVP** — `@Pointcut("Type.method")` registers a narrow string pattern; `@Around("…")` must match a registered pointcut, otherwise fail-closed (fingerprint `macro-service-v11:pointcut-name-mvp`); **not** AspectJ expressions / aspect classes / wildcard join points
- **Token expression-position `#macro` MVP** — expression-position `#double(21)` / `#apply(f,x)` and other ident/literal parameter substitutions; Phase 134 deepened the expression-tree walk (fingerprint `macro-token-v5:expr-token-subst-mvp`); Phase 130 initial version (`macro-token-v1:expr-token-mvp`); **not** a complete token syntax macro / token-stream API
- **Declaration-position `#macro` token subst deepening MVP** — declaration-position function names / parameter defaults / enum payloads and other ident/literal substitutions (fingerprint `macro-token-v6:decl-token-subst-mvp`); Phase 131 initial version (`macro-token-v2:decl-param-subst-mvp`); **not** a complete token syntax macro / token-stream API
- **Declaration-position `#macro` token-stream enumeration MVP** — declaration-position parameters can be enumerated as `Ident` / `Punct` / `Group` (fingerprint `macro-token-v7:token-stream-mvp`); **not** a complete token-stream / user SYX expansion → **137+**
- **Statement-position `#macro` token subst MVP** — statement-position `#say("x")` and other ident/literal parameter text substitutions (fingerprint `macro-token-v4:stmt-token-subst-mvp`); **not** a complete token syntax macro / token-stream API
- **`@Pointcut("within(Type)")` type-name MVP** — `@Pointcut("within(Foo)")` registers a narrow type-name pattern; `@Around`/`@Before`/`@After` may bind any method on that type (fingerprint `macro-service-v14:pointcut-within-name-mvp`); **not** AspectJ expressions / aspect classes
- **`@Pointcut("args(Type)")` first-parameter type-name MVP** — `@Pointcut("args(Int)")` registers a narrow first-parameter type-name pattern; `@Around`/`@Before`/`@After` may bind methods whose first parameter type matches (fingerprint `macro-service-v15:pointcut-args-name-mvp`); **not** full AspectJ expressions / aspect classes
- **`@Pointcut("within(T) && args(U)")` narrow composition MVP** — `@Pointcut("within(Foo) && args(Int)")` only `within` + `args` name patterns composed with `&&`; it matches only when both hit (fingerprint `macro-service-v16:pointcut-and-compose-mvp`); **not** negation / full AspectJ expressions / aspect classes
- **`@Pointcut("within(T) || args(U)")` narrow composition MVP** — `@Pointcut("within(Foo) || args(Int)")` only `within` + `args` name patterns composed with `||`; it matches when either side hits (fingerprint `macro-service-v17:pointcut-or-compose-mvp`); **not** negation / full AspectJ expressions / aspect classes / Spring → **137+**
- **`@Configuration` / `@Bean` MVP** — factory methods inside a configuration class register bindings; `Di.resolve` resolves the return type
- **`@Bean` parameter injection MVP** — `@Bean` method parameters are injected by type / `@Autowired` (optionally `@Qualifier`) from the registered `@Service`/`@Bean` graph (fingerprint `macro-service-v10:bean-param-autowire-mvp`); **not** the full set of conditional assembly
- **`@Lazy` + cyclic-dependency lazy proxy** — defers the factory until the first `Di.resolve`/use; an A↔B cycle containing a `@Lazy` edge gets a lazy proxy handle inserted (materialized on the first method call); **not** arbitrary type proxies / CGLIB / the full set of scoped proxies
- **137+ debt** — complete token syntax macros / token-stream API, complete AspectJ-style AOP (negation / aspect classes / wildcard join points / the full expression and precedence set), a complete Spring container, the full ThreadPoolExecutor set (DiscardOldest / Actor shared pool, …)

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

Tests: `platform/service_di_and_pointcut.rs` · `fixtures/macros/` (`token_expr.sy` · `pointcut_around.sy`, …) · `examples/service_di/` · `examples/token_expr_macro/` · `packages/di/mod.sy`

## `@freestanding` Role Enforcement (Phase 119 · inherited from M9)

`expression` / `statement` / `declaration` attachment-point misplacement → compile error **E003** (driver + LSP E2E).

Test: `fixtures/lsp/freestanding_decl_role_a.sy`

## Attached wrapper + Hygiene DX

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

- **`@propertyWrapper`** is deprecated (W001); migrate to `@attached(wrapper)`
- **Hygiene escape**: `\name` inside a macro body preserves the outer binding (`SYX_MACRO_HYGIENE=0` turns it off globally for one build)
- **Import macros**: `register_imported_macros` registers User + External

Tests: `fixtures/macros/attached_wrapper.sy`, `fixtures/macros/hygiene_escape_ok.sy`, `fixtures/macros/`

## External Production Path (Phase 119 plugin-prod)

```syx
macro ExternalSmoke = #externalMacro(module: "SyxMacros", type: "SmokeMemberMacro")

func main() {
    #ExternalSmoke()   // freestanding statement expansion
}
```

- **Cross-module import**: a dependency crate's `MacroDecl` is registered through `register_imported_external_macros`
- **WASM**: with `SYX_TARGET_WASM=1`, external macros raise compile-time E004
- **Production**: cdylib JSON protocol with real types; the subprocess sandbox is on by default; **without seccomp you must not claim hard-fail GA** (Linux has a hard gate when available, Darwin is honestly `sandbox-reexec-only`)

Tests: `fixtures/lsp/freestanding_decl.sy`, `fixtures/macros/json_plugin_prod.sy`, `fixtures/macros/json_plugin_prod.sy`

## Freestanding Builtin Macros

| Macro | Description |
|----|------|
| `#assert(expr)` | Terminates when the assertion fails |
| `#if condition` | Conditional compilation |
| `#assert_eq` / `#assert_ne` | Equality assertions |
| `#while` / `#for` | Loop unrolling |
| `#warning` / `#error` | W001 / E001 |
| `#compileTime` | Constant-evaluation subset (M9 hardened boundary) |
| `#stringify(expr)` | Expression position → string literal (M9 builtin pass) |

```syx
#assert(1 + 1 == 2)
#for(3, print("x"))
let s = #stringify(1 + 1)   // "1 + 1"
```

Tests: `fixtures/macros/macro_engine.sy`, `fixtures/macros/macro_engine.sy`, `fixtures/lsp/freestanding_decl.sy`

## User `#macro` (Phase 40+, hygiene v2 Phase 50)

```syx
#macro twice(x) {
    print(x)
    print(x)
}

func main() {
    #twice("hello")
}
```

- **Freestanding only** (statement position); parameters are identifier/expression text substitution
- Expansion depth ≤16; cyclic or overly deep expansion → **`E002`**
- **Phase 50 hygiene v2**: parameter bindings use `MacroParamId`, avoiding collisions with same-named identifiers at the call site (`SYX_MACRO_HYGIENE=0` disables it for one build)

Tests: `fixtures/macros/macro_engine.sy`, `fixtures/macros/hygiene_escape_ok.sy`

## @propertyWrapper (Phase 40 GA)

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

CLI: `--no-property-wrapper` disables it. Test: `fixtures/macros/attached_wrapper.sy`

## @attached (Phase 50 Real AST Expansion)

**Since Phase 50**, `body:` is parsed by `parse_fragment` into real statements/declarations, no longer just generating a `print` stub.

```syx
@attached(member, named, greet, "print(\"member:{name}\")")
data class Greeter(name: String)

@attached(peer, names: prefixed(_describe))
data class Point(x: Int, y: Int)
```

- Roles: `member`, `peer`, `custom`
- An in-source `body:` takes precedence over the `SYX_ATTACHED_*_BODY` environment variables (for tests/templates)

Tests: `fixtures/macros/attached_wrapper.sy`, `fixtures/macros/hygiene_escape_ok.sy`, `fixtures/macros/attached_conformance_dispatch.sy`

## Proof Macros (L1 GA · L2 partial, Phase 50–52 / 119)

Automatically generate protocol witnesses on a `data class`:

| Macro | Generated content | Tier |
|----|----------|------|
| `@Equatable` | `func ==(other:) -> Bool` | L1 **GA** |
| `@Hashable` | `func hash(into:)` → `__syx_hasher_combine` | L1 **GA** |
| `@Codable` | `encode()` + `static decode(text:)` (JSON Host) | L1 **GA**; **associated-value enum L2 partial GA** (fingerprint `macro-codable-l2:associated-enum-auto`); generic witness **partial** |

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

Limits: L1 applies only to a `data class` with **stored properties**; L2 associated-value enum full automatic round-tripping is **GA (narrow)**; the full set of generic associated values / arbitrary nesting / custom coding keys → **131+**. Tests: `fixtures/parity/dataclass_synth.sy`, `fixtures/parity/dataclass_synth.sy`, `fixtures/macros/codable_associated_enum.sy`, `codable_associated_enum.sy`

## `attached(conformance)` Honesty (Phase 119)

An unknown protocol **must not be silently ignored** (**E00x** or an explicit non-GA diagnostic); the Equatable/Hashable/Codable paths remain.

## `#externalMacro` (Phase 51–52 → 119 plugin-prod)

Expansion happens in-process or in a plugin process through the JSON protocol `MacroExpandRequest` / `MacroExpandResponse`:

```syx
macro ExternalSmoke = #externalMacro(module: "SyxMacros", type: "SmokeMemberMacro")

@ExternalSmoke
data class Point(x: Int, y: Int)
```

A package manifest may declare `[macros].exports` to register external macros; LSP v44 supports jump-to-definition and expand preview. See [ADR-112](../../docs/adr/architecture-decisions.md)

Test: `fixtures/macros/json_plugin_prod.sy`

## @resultBuilder (Phase 119 · engine pipeline · Phase 123 HostCall+Closure stack fix)

`@resultBuilder` expands in the sixth pass of the **`syx-macro` engine** fixed-point pipeline. It supports multi-stmt / nested `if`/`for`, `buildBlock` / `buildEither` / `buildArray`, and **`buildOptional`**.

Phase 123: literal + for-in + **HostCall inside for-in** (such as `__syx_str_length`) via closure inlining **compiles and runs** (fingerprint `macro-pipeline-v6:rb-hostcall-closure-fixed`); illegal patterns still fail-closed.

See [Architecture Decisions · Macro System](../../docs/adr/architecture-decisions.md#macro-system).

## @attached Roles (Phase 53+)

| Role | Attachment point | Example |
|------|--------|------|
| `member` / `peer` / `custom` | Type | `@attached(member, named, greet, "...")` |
| **`accessor`** | Property (extension method) | `@attached(extension, named, readValue, "func readValue() ...")` |
| **`deinit`** | class | `@attached(deinit, named, cleanup, "print(...)")` |
| **`extension`** | Type | `@attached(extension, named, label, "func label() ...")` |

JSON protocol `role` field: `member` | `accessor` | `deinit` | `extension`.

## Not Implemented / Deferred (136+)

- Complete token syntax macros (currently expression/declaration/statement-position ident/literal MVP + Phase 134 expression tree / Phase 135 declaration-position deepening; **not** a token-stream API / arbitrary syntax rewriting)
- Complete AspectJ-style AOP / aspect classes / wildcard join points (currently `@Pointcut("Type.method")` + narrow `execution(* T.m(..))` + `within(Type)` + `args(Type)` + `within && args` name-pattern MVP)
- A complete Spring container
- The complete commercial OT license table-level attached roles
- Writing expansion logic in user SYX source
- `@Codable` L2 full set of generic associated values / arbitrary nesting enums / a custom coding keys DSL

## Differences from Swift

Built-in + limited user macros + L1 proof macros + opt-in external macros + M25 Service DI v16 + decl-token deepen / expr-token / stmt-token / decl-hygiene / decl-param MVP + `@Before`/`@After` pointcut binding + `@Pointcut` / `execution` / `within(Type)` / `args(Type)` / `within && args` name-pattern MVP + `@Around` proceed + `@Bean` parameter injection MVP + Codable L2 associated-enum + honest partial annotations — **not** Swift's complete extensible hygienic macro system.

---

**Next chapter** → [Type Casting](20-type-casting.md)

**See also** → [Language Reference: Attributes](../language-reference/06-attributes.md) · [ADR-127](../../docs/adr/architecture-decisions.md)
