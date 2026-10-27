> **Current SSOT:** [books/shared/implementation-status.md](../books/shared/implementation-status.md) (SYX **117.0** / Phase **139** · `syx-cache-v1`). Priority matrices below may lag; trust status and `architecture-decisions.md`.  
> **License:** [Apache-2.0](../LICENSE) · **Native AOT:** LLVM only (Cranelift AOT removed; JIT tier-2 opt-in experimental).

# SYX Language Feature Implementation Priority List

[简体中文](./zh/7_FeatureImplementationPriority.zh-CN.md)

This document prioritizes SYX toolchain features (**Rust/Cargo workspace**) against current implementation and expected demand. Source uses `.sy`; examples are SYX syntax.

> **Compiler pipeline:** `Lexer → Parser → Semantic → MIR → CodeGen → VM`; `--native` is **MIR → LLVM → syx-rt → `app`**; `--wasm` is MIR → WASM emit.

---

## Status legend

| Mark | Meaning |
|------|------|
| ✅ | Done; integration tests or runnable examples |
| ⚠️ MVP | Syntax/basic behavior; incomplete function or performance |
| ❌ | Not implemented or lexer/AST placeholder only |

**Primary crates:** `syx-lexer` · `syx-parser` · `syx-ast` · `syx-semantic` · `syx-codegen` · `syx-vm` · `syx-runtime-core` · `syx-stdlib` · `syx-driver` · `syx-mir` · `syx-mir-run` · `syx-rt` · `syx-llvm` · `syx-wasm-emit` · `syx-fmt` · `syx-lint` · `syx-doc` · `syx-async` · `syx-meta` · `syx-cli` · `syx-macro` · `syx-host-plugin`

---

## Priority levels

| Level | Name | Definition | Timeframe |
|------|------|------|----------|
| P0 | Critical | Language unusable without it | 1–3 months |
| P1 | High | Major usability gain | 3–6 months |
| P2 | Medium | Developer experience | 6–12 months |
| P3 | Low | Nice-to-have | 12+ months |

---

## P0 — Critical features

### 1. Module system ✅

**Status:** Implemented. Supports `import`, module path resolution, `public` / `internal` / `private` visibility, cross-module `public` exports; cross-module `internal` access → **S005**. Multi-module projects validated via `syx.toml` + `examples/deps/`.

**Example:**
```syx
// math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {
    return 0
}

// main.sy
import math

func main() {
    print(math.add(1, 2))
}
```

**Subtasks:**
- [x] Module import syntax (`import moduleName`)
- [x] Visibility (public / internal / private)
- [x] Search paths (same dir, `syx.toml` deps, relative paths)
- [x] Namespace isolation (`module.symbol` calls)

**Location:** `syx-driver` (`compile_module_recursive`, `resolve_module_path`), `syx-semantic` (S005)

---

### 2. String processing ✅

**Status:** Implemented. Interpolation, `split`, `trim`, `hasPrefix`, `lowercased` / `uppercased`, `replacingOccurrences`, `substring`, `length`, `matches` (regex on strings); regex literals `/pattern/` parse.

**Example:**
```syx
func main() {
    let name = "World"
    print("Hello, \(name)!")

    let parts = "a,b,c".split(",")
    for item in parts {
        print(item)
    }

    print("  Hello  ".trim())
    print("abc123".matches("\\d+"))
}
```

**Subtasks:**
- [x] String interpolation
- [x] Regex (`matches` + host `__syx_regex_match`)
- [x] Common string methods (`syx-stdlib` `STRING_METHODS`)

**Location:** `syx-stdlib/registry.rs`, `syx-vm/hosts.rs`

---

### 3. File I/O ✅

**Status:** Implemented. `File` / `Directory` static methods via host functions; use `try/catch` for I/O errors.

**Example:**
```syx
func main() {
    let path = "/tmp/data.txt"
    try {
        File.write(path, "line1\nline2")
        let content = File.read(path)
        print(content)

        let lines = File.readLines(path)
        for line in lines {
            print(line)
        }

        print(File.exists(path))
        Directory.create("/tmp/mydir")
    } catch (err) {
        print("file error")
    }
}
```

**Subtasks:**
- [x] Read/write/append
- [x] Exists, delete, copy, move
- [x] Directory create, list, exists

**Location:** `syx-stdlib` (`HostFnId::File*` / `Dir*`), `syx-vm/hosts.rs`

---

### 4. Basic networking ✅

**Status:** Implemented. `HTTP.get` / `HTTP.post` via hosts; response `.json()`. Network tests often `#[ignore]` (needs external network).

**Example:**
```syx
func main() {
    try {
        let response = HTTP.get("https://httpbin.org/get")
        let json = response.json()
        print(json)
    } catch (err) {
        print("http error")
    }
}
```

**Subtasks:**
- [x] HTTP GET / POST
- [x] JSON response parsing
- [ ] Timeouts, retries, custom headers (future)

**Location:** `syx-stdlib`, `syx-vm/hosts.rs`

---

### 5. Iterator protocol ⚠️ MVP

**Status:** `for-in` works on `Array`, per-character `String`, and arrays from `File.readLines`. `protocol Iterator { func next() }` declares and **S006** checks; user `Sequence` / custom iterators incomplete.

**Example:**
```syx
func main() {
    for n in [1, 2, 3] {
        print(n)
    }
    for ch in "hi" {
        print(ch)
    }
}
```

**Subtasks:**
- [x] Array / String / readLines iteration
- [x] Iterator protocol declaration and semantic check
- [ ] User-defined iterators and `makeIterator`
- [ ] Streaming line-by-line files (today: readLines array)

**Location:** `syx-codegen` (MakeIterator), `syx-semantic` (S006)

---

## P1 — High-priority features

### 6. Structured concurrency ⚠️ MVP

**Status:** `async func` / `await` parse; `syx-async` validates await usage; **single await site, blocking poll**—no TaskGroup / multithreaded scheduler.

**Example:**
```syx
async func fetchLabel() -> String {
    let resp = await HTTP.get("https://example.com")
    return "done"
}
```

**Subtasks:**
- [x] async/await parsing
- [x] await in non-async → compile error
- [ ] Task / TaskGroup
- [ ] Scheduler + full state-machine lowering

**Location:** `syx-async`, `syx-codegen`

---

### 7. Null safety ✅

**Status:** Implemented. `T?`, `nil`, `??`, force unwrap `!` (nil → **R007**), optional chain `?.`, `if let` / `guard let`.

**Example:**
```syx
func main() {
    let a: Int? = nil
    print(a ?? 99)

    let b: Int? = 7
    print(b!)

    let s: String? = "hello"
    print(s?.hasPrefix("he"))

    if let y = b {
        print(y)
    }
}
```

**Subtasks:**
- [x] Nullable types and nil
- [x] `??` / `!` / `?.`
- [x] `if let` / `guard let`

**Location:** `syx-semantic`, `syx-codegen`, `syx-vm` (R007)

---

### 8. Data class ⚠️ MVP

**Status:** `data class Point(x: Int, y: Int)` compiles; ctor, field access (`p.x`), **`.description()`** auto string (e.g. `Point(x: 3, y: 4)`). No auto `copy()`, `==`, `hashCode` yet.

**Example:**
```syx
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    print(p.x)
    print(p.y)
}
```

**Subtasks:**
- [x] Syntax and constructor
- [x] Field access
- [ ] Auto copy / equals / hashCode / description

**Location:** `syx-codegen`, `syx-meta` (`register_data_classes`)

---

### 9. Extension functions ⚠️ MVP

**Status:** `extension Type { func ... }` implemented; instance calls desugar to module-level functions. Functions mostly; computed properties limited.

**Example:**
```syx
extension Int {
    func doubled() -> Int {
        return self + self
    }
}

func main() {
    print(5.doubled())
}
```

**Subtasks:**
- [x] extension decl and desugar
- [ ] Computed properties, generic extensions

**Location:** `syx-codegen` (`extension_methods`)

---

### 10. Pattern matching ⚠️ Partial

**Status:** `switch` supports int cases, half-open ranges `0..<13`, sealed variants `.ok(let x)`. `where` guards and complex tuple patterns incomplete.

**Example:**
```syx
func main() {
    let age = 25
    switch age {
    case 0..<13:
        print("child")
    case 13..<20:
        print("teen")
    default:
        print("adult")
    }
}
```

**Subtasks:**
- [x] Int / range cases
- [x] Sealed variant cases
- [ ] `where` guards
- [ ] Tuple / nested patterns

**Location:** `syx-parser` (`parse_switch_pattern`), `syx-codegen`

---

### 11. Sealed class ⚠️ MVP

**Status:** `sealed class`, variants, switch patterns; exhaustiveness MVP (hard with complex generics).

**Example:**
```syx
sealed class Result {
    case ok(value: Int)
    case err(message: String)
}

func main() {
    let r = Result.ok(42)
    switch r {
    case .ok(let x):
        print(x)
    case .err(let msg):
        print(msg)
    }
}
```

**Subtasks:**
- [x] Syntax and codegen
- [x] Variant construction and switch
- [ ] Compile-time exhaustiveness (all cases)

**Location:** `syx-codegen` (`sealed_classes`)

---

### 12. Package manager ✅

**Status:** `syx-pkg` supports path / git / HTTP registry; `syx install` writes structured `syx.lock`; compile resolves via lock (**P001** if lock missing).

**Example:**
```toml
[dependencies.local_math]
version = "1.0.0"
path = "../deps_math"
```

**Subtasks:**
- [x] Local path dependencies
- [x] Lockfile + `syx install` / `syx add` / `syx update`
- [x] SemVer resolution
- [x] Git URL → `.sy/vendor/`
- [x] HTTP / file registry client + `syx publish`

**Location:** `syx-pkg`; `syx-driver` (lock-driven `resolve_module`)

---

## P2 — Medium-priority features

### 13. Macro system ✅ M15 (honest layering)

**Status (Phase 125):** `syx-macro::expand_module` fixed-point for freestanding / attached / external / resultBuilder / service DI; Service DI v6: **base/protocol polymorphic inject**, field `@Qualifier`, **narrow `@Around` MVP** (**only** `@Service`/`@Component` instance methods, **not** AspectJ); M14 **`Di.withRequest`** / `@Qualifier` / `@Primary` / `@Profile` inheritance. Doc fingerprint `macro-docs-v7:m15-service-v6-honesty`. Token macros / AspectJ / full Spring → **128+**. See [19-macros.md](../books/language-guide/19-macros.md) · [ADR-119](adr/architecture-decisions.md).

**Location:** `syx-macro` (engine), `syx-driver` (`expand_module` wrapper), `syx-parser` (`Stmt::MacroInvoke`)

---

### 14. Actor model ✅

**Status:** `actor` decl, mailbox isolation, `ActorCall`/`ActorSend` (VM / MIR / WASM / LLVM); parity in behavior suites. WASM optional `SYX_WASM_MIR_FALLBACK=1`.

**Location:** `syx-codegen`, `syx-mir-run`, `syx-cranelift`, `syx-rt/bridge.rs`

---

### 15. Borrow checker (simplified) ⚠️ MVP

**Status (Phase 7):** `&T` shared, `&mut T` exclusive (**S011** single active rule), move; fixtures `p6_borrow_*`, `p7_borrow_mut*.sy`.

**Location:** `syx-semantic` (`TypeId::Reference`, `moved_bindings`), `syx-parser` (`&` types/exprs)

---

### 16. Compiler optimizations ⚠️ Partial

**Status:** `syx-mir` lowering + basic `optimize`; `SYX_USE_MIR=1` enables MIR→CodeGen. Constant folding, DCE, inlining are early—not production-grade.

**Subtasks:**
- [x] MIR IR
- [x] Basic optimize pass
- [ ] Full pipeline + benchmark-driven tuning

**Location:** `syx-mir`, `syx-driver` (`SYX_USE_MIR`; stdlib modules still codegen—see `docs/12_BackendDebuggingGuide.md`)

---

### 17. Incremental compilation ⚠️ MVP

**Status:** Project `.sy-cache/` per-module SHA256 + **v8** fingerprint (rustc version, dependency tree hash, MIR op version) caches `ModuleUnit` (bincode); `purge_stale_incremental_caches` on dependency changes.

**Location:** `syx-driver` (`compile_with_cache`, `incremental_cache_path`)

---

### 18. Debugging ✅

**Status:** `syx debug` / `syx debug --dap` line breakpoints, `continue`/`stepIn`/`stepOut`, structured `variables` (`value_description`), `stackTrace`. See [docs/12](12_BackendDebuggingGuide.md).

**Location:** `syx-debug`, `syx-cli`

---

### 19. Standard library expansion ⚠️ Partial

**Status:** Array **`length`**, **`contains(item)`**, **`sorted()`**, **`reverse()`**, **`joined(separator)`**; **`map`/`filter`/`reduce`** via closure HOF (VM/MIR/native parity, `p9_hof_closure.sy`). Global **`readLine()`** from stdin; range **`(1..<10).contains(n)`**. No DateTime yet.

**Example:**
```syx
let nums = [3, 1, 2]
print(nums.contains(2))
print(nums.sorted().length)
print(nums.reverse().joined(","))
print((1..<10).contains(5))

let name = readLine()
print(name)
```

**Location:** `syx-stdlib` (`ARRAY_METHODS`, `HostFnId::ReadLine`), `syx-vm/hosts.rs`

---

### 20. Toolchain polish ✅

**Status:** CLI: `run` · `build` · `check` · `test` · `format` · `lint` · `doc` · `debug` · `repl` · `add` · `update`.

```bash
cargo build --release          # Build syx CLI
syx run examples/p0_demo.sy
syx format .
syx lint .
syx doc .                      # → docs/api.md
syx repl
syx test .
```

**Subtasks:**
- [x] format (`syx-fmt`)
- [x] lint (`syx-lint`, L001–L012)
- [x] doc (`syx-doc`)
- [x] test / check / repl
- [x] LSP (SemanticAnalyzer: hover, completion, definition, rename, references)

**Location:** `syx-cli`, `syx-driver`

---

## P3 — Lower-priority features

### 21. Native backend (LLVM sole) ✅ / ongoing

**Value:** Native performance, full host surface (print / HTTP / task, etc.)  
**Status:** `syx build --native` → MIR → **`syx-llvm`** → `user.o` → link **`syx-rt`** → **`app`**. Cranelift AOT removed; LLVM is the sole native AOT backend. `--object` emits `.o` only. MIR semantics align with VM (`ValueEq`, `ToDescription`, float ops, try/catch). Release binary size varies with `syx-rt` features (default `async-runtime`; `full` includes HTTP).  
**Effort:** Ongoing · **Difficulty:** ⭐⭐⭐⭐

---

### 22. WASM target ⚠️ MVP

**Value:** Browser / embedded wasmtime  
**Status:** `syx build --wasm` compiles linked MIR to WASM (guest calls `syx.mir_exec`); `syx run --wasm` uses wasmtime + `MirRuntime`. SYXP container path removed.  
**Effort:** 1–2 months (hardening) · **Difficulty:** ⭐⭐⭐

---

### 23. JIT compilation ❌ / ⚠️ Stub

**Status:** `syx-vm` `JitCompiler` hot-path counter stub; tiered compilation off by default.

---

### 24. Type-level programming ❌

**Status:** No conditional/map types (TypeScript-style type computation).

---

### 25. Effect system ⚠️ MVP

**Status:** `requires Pure` / `requires IO` on functions; `syx-meta` rejects I/O inside Pure bodies (`print`, File calls, etc.).

**Example:**
```syx
func compute(x: Int) -> Int requires Pure {
    return x * 2
}

func load() -> String requires IO {
    return File.read("data.txt")
}
```

**Location:** `syx-meta` (`check_effects`, `validate_pure_functions`)

---

### 26. Dependency injection ⚠️ MVP

**Status (Phase 125):** `@Service` / `@Component` multi-package scan + registry; `@Inject` ctor + field `@Autowired`; **base/protocol polymorphic inject GA**; field `@Qualifier`; **`@Around` method intercept MVP** (**only** `@Service`/`@Component` instance methods, **not** AspectJ); prototype + **request scope** + Qualifier/Primary/Profile + `Di.withRequest`. E2E: `examples/service_di/` · `p125_service_di_v6.rs` · `lsp_phase125_real`. Token macros / full AOP → **128+**. See [19-macros.md](../books/language-guide/19-macros.md) · [ADR-119](adr/architecture-decisions.md).

---

### 27. Reflection ⚠️ MVP

**Status:** Rust-side `syx_driver::reflect_type_info` / `syx-meta::reflect_type` metadata stub; source `reflect(T.self)` is scaffolding.

---

### 28. Metaprogramming ❌ / ⚠️ Scaffolding

**Status:** `#compileTime { }` parses; no compile-time codegen execution.

---

## Feature dependency graph

```
Core language (P0 largely ✅)
├── Module system ✅ ──────┬──→ Package manager ⚠️
├── String enhancements ✅ ─┤
├── File I/O ✅ ────────────┼──→ Stdlib expansion ⚠️
├── Networking ✅ ──────────┤
└── Iterator protocol ⚠️ ───┘

Type system (P1 partial ✅)
├── Null safety ✅ ─────────┬──→ Data class ⚠️
├── Data class ⚠️ ──────────┤
├── Sealed class ⚠️ ────────┤
└── Extension functions ⚠️ ─┘

Advanced
├── Structured concurrency ✅ ──┬──→ Actor ✅
├── Borrow checking ❌ ─────────┤
└── Macro system ❌ ────────────┘

Toolchain & backends
├── Package manager ✅ ────────┬──→ LSP ✅
├── fmt/lint/doc ✅ ───────────┤
├── Incremental compile ✅ ──┤
├── MIR/optimize ✅ ───────────┤
├── LLVM/WASM ✅ ──────────────┤
├── unified MIR ✅ ────────────┤
└── Debug DAP ✅ ──────────────┘
```

---

## Progress and next steps

### Completed milestones (Rust toolchain)

| Area | Status |
|------|------|
| P0 modules / strings / File / HTTP / URL | ✅ |
| P0 script stdlib (Env/Path/Time) | ✅ |
| P1 null safety / try-catch | ✅ |
| P1 data class / extension / sealed | ⚠️ MVP (copy/eq/hash synthesis) |
| P1 async/await + Task syntax | ⚠️ MVP (state-machine suspend) |
| P1 packages (path + git + registry) | ✅ MVP |
| P2 incremental cache (v8 dep-tree invalidation) | ✅ |
| P2 LSP / debug DAP | ✅ GA (Phase 11: ≥45 LSP tests, code actions/semantic tokens, DAP async frames) |
| P3 native/wasm backends | ✅ parity-gate |
| P3 unified MIR + registry fixture CI | ✅ |
| P3 effects Pure/IO | ⚠️ MVP |

### Next phase suggestions

**Short term (Phase 6+):**
- Borrow checking (simplified)
- Macro expansion and metaprogramming
- LLVM as default native backend (AOT sole backend today)
- GC intrinsics across three backends

**Done (Phase 5 / SYX 1.0):**
- Actor / Hasher / try-catch three-backend without default MIR fallback
- enum associated values + dataclass eq/copy/hash parity
- async fair scheduling fixture (`p10_async_interleave`)
- SemanticAnalyzer-driven LSP; DAP stepOut + structured locals
- `build_unified_mir` convergence + HTTP mock / registry PR CI

**Long term:**
- Borrow checking (simplified)
- Macro expansion and metaprogramming
- JIT / full optimization pipeline

---

## Resource estimates

### Staffing (remaining work)

| Phase | Focus | Estimate |
|------|------|------|
| MVP polish | data class, Array HOF, async | 2–3 person-months |
| Production ready | Actor, LSP, backend host bridges | 6–9 person-months |
| Advanced | borrow, macros, JIT | 12+ person-months |

### Skills

- Compiler frontend (Rust, types, MIR)
- Runtime (VM, hosts, async)
- Toolchain (CLI, LSP, debug protocols)
- Backends (LLVM, WASM emit)

---

## Phase 33 → Phase 34 scaffold vs production

| Area | Phase 33 | Phase 34 target | Env gate |
|------|----------|---------------|----------|
| LLVM / inkwell | ADR-042 default GA; text IR bytes | **ADR-044 inkwell Prod GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥91` |
| LLVM AOT | Phase 8 VM/native GC | Phase 9 **unified** VM/WASM/native GC **direct IR** | `SYX_LLVM_AOT_PHASE9` |
| Unified GC | VM/native atomic scaffold | **ADR-045 three-backend unified generational GC** | `SYX_UNIFIED_GC_LTS=1` |
| LLVM perf | `p33_llvm_perf_gate` parity-gate formal | **`p34_llvm_perf_gate` perf release GA** | `llvm_perf_green_seasons()≥5` |
| dist Win pipe | WinAPI scaffold | **Named Pipe prod default** | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| dist TLS | prefix/chain checks | **rustls crate prod default on** | `SYX_DIST_ACTOR_RUSTLS_CRATE_PROD=1` |
| Registry CDN | Phase 14 edge read opt-in | Phase 15 **global CDN multi-active write** E2E | `registry_cdn_phase15_enabled()` |
| GlobalActor | v13 cross-org federated LB scaffold | v14 **global multi-active CDN routing** | `syx_rt_global_actor_v14` |
| Net | setsockopt unix stub | **all-platform setsockopt prod** | ADR-032 Phase 34 |
| borrow | Coq/Lean string export scaffold | **machine-checked proofs prod** | `borrow_regions_coq_lean_proof_prod_enabled()` |
| DateTime | MVP 3 fn | **DateTime v2** ISO8601 + time zones | `lib/datetime.sy` |
| typealias | not implemented | **typealias MVP** (optional) | `p34_typealias_mvp.sy` |
| IDE route graph | multi-org federated live v4 | **multi-region federated live v5** | VS Code 2.7.0 |

## Phase 32 → Phase 33 scaffold vs production

| Area | Phase 32 | Phase 33 target | Env gate |
|------|----------|---------------|----------|
| LLVM / inkwell | inkwell candidate opt-in | **ADR-042 inkwell default GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥78` |
| LLVM AOT | Phase 7 guest/WASM GC | Phase 8 VM/native GC **direct IR** | `SYX_LLVM_AOT_PHASE8` |
| Cranelift GC | **⚡** | **GC intrinsics ✅** | `syx_rt_cranelift_gc_parity` |
| LLVM perf | `p32_llvm_perf_gate` CI candidate | **`p33_llvm_perf_gate` parity-gate formal** | `llvm_perf_green_seasons()≥4` |
| WASI GC | remembered set guest prod opt-in | **guest prod default on** + VM `--run` linkage | `SYX_WASI_GC_PHASE33=1` (ADR-043) |
| VM/native GC | **none** | **remembered set prod** | `SYX_VM_NATIVE_GC_PHASE33=1` |
| Net stream | socket window memory counts | **setsockopt real path** prod | ADR-032 Phase 33 |
| Registry CDN | Phase 13 offline install scaffold | Phase 14 **CDN edge cache** E2E | `registry_cdn_phase14_enabled()` |
| GlobalActor | v12 dynamic weights scaffold | v13 **cross-org federated LB** + endpoint drain | `syx_rt_global_actor_v13` |
| borrow | machine proof harness | **Coq/Lean external proof scaffold** | `borrow_regions_coq_lean_export_enabled()` |
| dist Actor | file pipe + pin prefix | **real Named Pipe API** + **rustls crate** E2E | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| IDE route graph | multi-repo federated live v3 | **multi-org federated live v4** | VS Code 2.6.0 |

## Phase 31 → Phase 32 scaffold vs production

| Area | Phase 31 | Phase 32 target | Env gate |
|------|----------|---------------|----------|
| LLVM release | ADR-038 perf candidate | **ADR-040 release blocker GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥65` |
| LLVM AOT | Phase 6 + exec_op **→ 0%** | Phase 7 GC intrinsics all paths | `SYX_LLVM_AOT_PHASE7` |
| LLVM perf | nightly bench not gated | **`p32_llvm_perf_gate` CI gate candidate** | `llvm_perf_green_seasons()≥3` |
| WASI GC | generational relocating | **remembered set full generational** | `SYX_WASI_GC_PHASE32=1` (ADR-041) |
| Net stream | flow control counts | **true socket window** prod | ADR-032 Phase 32 |
| Registry CDN | Phase 12 install signing | Phase 13 **offline install** E2E | `registry_cdn_phase13_enabled()` |
| GlobalActor | v11 weighted LB | v12 **dynamic weights** + cross-region probes | `syx_rt_global_actor_v12` |
| borrow | formal verification scaffold | **machine proof harness** | `borrow_regions_machine_proof_enabled()` |
| dist Actor | file pipe + pin prefix | **real Named Pipe API** + **rustls crate** E2E | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| IDE route graph | cross-workspace federated v2 | **multi-repo federated live v3** | VS Code 2.5.0 |

## Phase 30 → Phase 31 scaffold vs production

| Area | Phase 30 | Phase 31 target | Env gate |
|------|----------|---------------|----------|
| LLVM AOT | Phase 5; GC intrinsics ❌ | Phase 6 + exec_op **→ 0%** | `SYX_LLVM_AOT_PHASE6` |
| LLVM perf | nightly bench not gated | ADR-038 **perf release candidate** | `SYX_LLVM_STRICT_GREEN_WEEKS≥52` |
| WASI GC | relocating prod | **generational relocating** | `SYX_WASI_GC_PHASE31=1` (ADR-039) |
| Net stream | ring buffer counts | socket flow control prod | ADR-032 Phase 31 |
| Registry CDN | Phase 11 auto URL | Phase 12 install signing E2E | `registry_cdn_phase12_enabled()` |
| GlobalActor | v10 three regions | v11 weighted LB + failover | `syx_rt_global_actor_v11` |
| borrow | language complete | formal verification scaffold | `borrow_regions_formal_verify_enabled()` |
| IDE route graph | multi-process live | cross-workspace federation | VS Code 2.4.0 |

## Phase 29 → Phase 30 scaffold vs production

| Area | Phase 29 | Phase 30 target | Env gate |
|------|----------|---------------|----------|
| LLVM AOT | Phase 4; Host I/O/JSON ❌ | Phase 5 + exec_op **< 1%** | `SYX_LLVM_AOT_PHASE5` |
| LLVM CLI default | ADR-034 parity-gate only | ADR-036 CLI **default llvm** | `SYX_LLVM_STRICT_GREEN_WEEKS≥39` |
| WASI GC | relocating MVP | **relocating prod** | `SYX_WASI_GC_PHASE30=1` (ADR-037) |
| Net stream | handle + string backpressure | ring buffer real backpressure prod | ADR-032 Phase 30 |
| Registry CDN | Phase 10 signed URL | Phase 11 auto failover | `registry_cdn_phase11_enabled()` |
| GlobalActor | v9 dual region | v10 global multi-active LB | `syx_rt_global_actor_v10` |
| borrow | SE-0461 100% | language-complete borrow | — |
| IDE route graph | bidirectional sync | multi-process live | VS Code 2.3.0 |

## Phase 28 → Phase 29 scaffold vs production

| Area | Phase 28 | Phase 29 target | Env gate |
|------|----------|---------------|----------|
| LLVM AOT | Phase 3; enum/witness ❌ | Phase 4 + exec_op **< 5%** | `SYX_LLVM_AOT_PHASE4` |
| LLVM default backend | ADR-033 candidate | ADR-034 parity-gate **formally on** | `SYX_LLVM_STRICT_GREEN_WEEKS≥26` |
| WASI GC | incremental sweep | **relocating MVP** | `SYX_WASI_GC_PHASE29=1` |
| Net stream | backpressure/handle scaffold | handle + real backpressure prod | ADR-032 Phase 29 |
| Registry CDN | Phase 9 failover | Phase 10 signing E2E | `registry_cdn_phase10_enabled()` |
| GlobalActor | v8 single region | v9 cross-region multi-active | `syx_rt_global_actor_v9` |
| Result | generic MVP | full mono | — |
| borrow | SE-0461 subset | SE-0461 100% | — |
| IDE route graph | LSP live | bidirectional sync | VS Code 2.2.0 |

## Phase 27 → Phase 28 scaffold vs production

| Area | Phase 27 | Phase 28 target | Env gate |
|------|----------|---------------|----------|
| dist Actor | UDS gate + TLS in-memory queue | UDS prod + real TLS connections | `SYX_DIST_ACTOR_UDS=1` / `SYX_DIST_ACTOR_TLS=1` |
| WASI GC | Phase 27 evaluation gate | guest **default on** prod | `SYX_WASI_GC_PHASE28=1` |
| Registry CDN | Phase 8 static URL list | Phase 9 failover + health probe | `registry_cdn_phase9_enabled()` |
| LLVM AOT | AOT Phase 2 + mixed exec_op | AOT Phase 3 + ADR-033 default backend | `SYX_LLVM_AOT_PHASE3` |
| Net/WSS | stream smoke + WSS mock | backpressure + TLS + WSS prod | `SYX_NET_STREAM_TLS=1` |
| Result | ADR-031 docs + LSP | parser + mono real generics | — |
| IDE route graph | mock webview | LSP live + navigate | `syx/actorRoutes` |

## Phase 26 → Phase 27 scaffold vs production

| Area | Phase 26 | Phase 27 target | Env gate |
|------|----------|---------------|----------|
| dist Actor | in-process `DIST_PROCESS_ROUTES` scaffold | real UDS/pipe + TLS MVP | `dist_actor_uds_enabled()` / `dist_actor_tls_enabled()` |
| WASI GC | Phase 26 guest default off | default on **evaluation** | `SYX_WASI_GC_PHASE27=1` |
| Registry CDN | Phase 7 single region | Phase 8 multi-region failover | `SYX_REGISTRY_CDN_PHASE8=1` |
| LLVM AOT | ADR-028 MVP | AOT Phase 2 + ADR-029 release default on | `SYX_LLVM_AOT_PHASE2` |
| Net | read/write smoke | stream MVP (ADR-030) | `SYX_NET_TCP_MVP=1` |
| Result | wire API | generic syntax MVP or ADR-031 docs | — |
| IDE route graph | no webview | VS Code panel spike | `editors/vscode-syx` |

---

## Risk assessment

| Risk | Probability | Impact | Mitigation |
|------|------|------|----------|
| MVP vs docs drift | Medium | Medium | Trust `crates/syx-driver/tests/*.sy` and integration tests |
| Native/WASM vs VM capability split | High | Medium | Document `--native`/`--wasm` subsets; daily dev on `syx run` |
| async semantics too simplified | Medium | High | Document MVP first; iterate scheduler |
| Borrow checker too heavy | High | High | Swift ownership inspiration, not full Rust borrow |
| Low community participation | Medium | Medium | Improve UsageGuide / QuickReference for contributors |

---

**Document version:** 2.8 (Phase 34 Unified GC LTS + Inkwell Prod GA narrative)  
**Last updated:** 2026-06-22  
**Verification:** `cargo test -p syx-driver` (P0/P1 integration + `p34_*` parity-gate)
