# SYX Compiler — Complete Technical Documentation (Rust Implementation)

[简体中文](./zh/5_CompleteTechnicalDocumentation.zh-CN.md)

> **Current alignment:** SYX **117.0** · Phase **139** · cache `syx-cache-v1` · audit driver ≥2600 / LSP ≥3050 · License [Apache-2.0](../LICENSE)  
> **Native AOT:** **LLVM only** (`syx build --native`); Cranelift AOT removed (JIT tier-2 remains opt-in experimental).  
> **SSOT:** [implementation-status](../books/shared/implementation-status.md) · [architecture-decisions](adr/architecture-decisions.md) · [docs/README](./README.md)  
> Some sections below retain historical detail; when they conflict with the banner above, the banner wins.

## Table of contents

1. [Project overview](#1-project-overview)
2. [Cargo workspace and layout](#2-cargo-workspace-and-layout)
3. [Architecture and compilation pipelines](#3-architecture-and-compilation-pipelines)
4. [Crate responsibilities](#4-crate-responsibilities)
5. [Compilation driver (`syx-driver`)](#5-compilation-driver-syx-driver)
6. [Command-line tool (`syx-cli`)](#6-command-line-tool-syx-cli)
7. [Module system and dependencies](#7-module-system-and-dependencies)
8. [Incremental compilation cache](#8-incremental-compilation-cache)
9. [Meta and async compile-time passes](#9-meta-and-async-compile-time-passes)
10. [Frontend: lexing, parsing, semantics](#10-frontend-lexing-parsing-semantics)
11. [Code generation and intermediate representation](#11-code-generation-and-intermediate-representation)
12. [Virtual machine and runtime](#12-virtual-machine-and-runtime)
13. [Standard library and host functions](#13-standard-library-and-host-functions)
14. [Toolchain crates (P2/P3)](#14-toolchain-crates-p2p3)
15. [Error handling and diagnostics](#15-error-handling-and-diagnostics)
16. [Testing](#16-testing)
17. [Build and CI](#17-build-and-ci)
18. [Appendix](#18-appendix)

---

## 1. Project overview

### 1.1 Introduction

SYX is a programming language inspired by Swift syntax. The compiler and toolchain are **implemented entirely in Rust**, organized as a Cargo workspace, covering the full path from source to executable bytecode.

Core pipeline:

| Stage | Crate | Role |
|------|-----------|------|
| Lexical analysis | `syx-lexer` | Source → token stream |
| Parsing | `syx-parser` | Tokens → AST (`syx-ast`) |
| Semantic analysis | `syx-semantic` | Type checking, symbol tables, module imports |
| Meta passes | `syx-meta` | Data class registration, pure/IO effect checks |
| Async checks | `syx-async` | `async`/`await` syntax constraints |
| Code generation | `syx-codegen` | AST or MIR → VM instructions |
| Execution | `syx-vm` | Stack VM + host functions |
| Driver orchestration | `syx-driver` | Module graph, cache, multi-backend entry |
| User entry | `syx-cli` | `syx` CLI |

### 1.2 Implementation phases

| Phase | Scope | Rust status |
|------|------|---------------|
| **P0** | Modules, strings, file I/O, HTTP, arrays/iteration | Largely done; integration tests |
| **P1** | Null safety, data classes, `guard let`, effect annotations | Partially done |
| **P2** | MIR, formatting, lint, doc generation, debug sessions | Scaffolding + MVP |
| **P3** | LLVM native, WASM emit, JIT, metaprogramming | Scaffolding + MVP |

---

## 2. Cargo workspace and layout

### 2.1 Workspace configuration

The root `Cargo.toml` defines the workspace:

- **members:** `crates/*` (19 crates)
- **edition:** 2021
- **rust-version:** 1.80
- **Shared dependencies:** `miette`, `thiserror`, `clap`, `serde`, `toml`, `bincode`, `wasm-encoder`, `reqwest`, etc.

### 2.2 Directory tree

```
synswx/
├── Cargo.toml                    # Workspace root
├── rustfmt.toml                  # Rust formatting rules
├── .github/workflows/ci.yml      # CI: fmt / clippy / test
├── crates/
│   ├── syx-cli/                  # CLI entry (`syx` command)
│   ├── syx-driver/             # Driver, module graph, cache, backends
│   ├── syx-diagnostics/        # Diagnostics, Span, SourceFile
│   ├── syx-lexer/                # Lexer
│   ├── syx-ast/                  # AST definitions and visitor traits
│   ├── syx-parser/               # Recursive-descent parser
│   ├── syx-semantic/             # Semantic analysis and type checking
│   ├── syx-codegen/              # VM bytecode generation
│   ├── syx-vm/                   # Stack VM, GC, hosts, JIT stubs
│   ├── syx-stdlib/               # Builtin types and method registry
│   ├── syx-mir/                  # MIR lowering and optimization
│   ├── syx-async/                # async/await compile-time checks and runtime stubs
│   ├── syx-meta/                 # Data classes, effects, reflection metadata
│   ├── syx-fmt/                  # Source formatter
│   ├── syx-lint/                 # Static lint rules
│   ├── syx-doc/                  # Markdown API doc generation
│   ├── syx-debug/                # Debug sessions and DAP stubs
│   ├── syx-mir-run/              # Shared MIR interpreter (WASM + native FFI)
│   ├── syx-runtime-core/         # Value, protocols, Hasher (no codegen)
│   ├── syx-cranelift/            # MIR → native object (JIT/experimental paths)
│   ├── syx-rt/                   # Native C ABI runtime, linking helpers
│   ├── syx-llvm/                 # LLVM IR emission (native AOT)
│   └── syx-wasm-emit/            # MIR → WebAssembly binary
├── examples/                     # Sample .sy programs and syx.toml
│   ├── hello.sy
│   ├── p0_demo.sy
│   ├── main.sy
│   ├── math.sy
│   ├── module_main.sy
│   └── syx.toml
├── docs/                         # Language and compiler documentation
└── README.md
```

### 2.3 Crate dependency graph (simplified)

```
syx-diagnostics
    ↑
syx-lexer → syx-ast → syx-parser
                ↑           ↑
                └── syx-semantic ← syx-stdlib
                        ↑
                   syx-codegen ← syx-mir
                        ↑
                     syx-vm
                        ↑
              syx-async / syx-meta
                        ↑
                   syx-driver ← syx-fmt / syx-lint / syx-doc / syx-debug
                        ↑              syx-llvm / syx-wasm-emit
                     syx-cli
```

---

## 3. Architecture and compilation pipelines

### 3.1 Overall architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     Source (.sy)                                 │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-lexer          Lexical scan → token sequence                │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-parser         Recursive descent → syx-ast::Module          │
└────────────────────────────┬─────────────────────────────────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│ syx-meta        │ │ syx-async    │ │ syx-macro       │
│ data class /    │ │ async rules  │ │ expand_module   │
│ effect checks   │ │              │ │                 │
└────────┬────────┘ └──────┬───────┘ └────────┬────────┘
         └─────────────────┼──────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-semantic       Type check + symbols + import merge          │
└────────────────────────────┬─────────────────────────────────────┘
                             │
              ┌──────────────┴──────────────┐
              ▼                             ▼
   ┌─────────────────────┐       ┌─────────────────────┐
   │ Default (VM) path   │       │ SYX_USE_MIR=1       │
   │ AST → syx-codegen   │       │ AST → syx-mir       │
   │                     │       │   → optimize      │
   │                     │       │   → syx-codegen   │
   └──────────┬──────────┘       └──────────┬──────────┘
              │                             │
              └──────────────┬──────────────┘
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  CompiledProgram (function table + instruction stream)           │
└────────────────────────────┬─────────────────────────────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         ▼                   ▼                   ▼
┌──────────────┐   ┌─────────────────┐   ┌─────────────────┐
│ syx-vm       │   │ syx-llvm        │   │ syx-wasm-emit   │
│ interpret    │   │ MIR → native    │   │ MIR → .wasm     │
│ (default)    │   │ + syx-rt link   │   │                 │
└──────────────┘   └─────────────────┘   └─────────────────┘
         │                   │                   │
         │           syx-mir-run (shared MIR interpreter)
         │           syx-runtime-core (Value / protocols)
         └───────────────────┴───────────────────┘
```

### 3.2 Main compilation pipelines

The default user path uses **MIR** as the shared middle end; `syx run` lowers to VM bytecode; `--native` / `--wasm` start from the same MIR graph.

#### 3.2.1 Default pipeline (`syx run` → VM)

**Triggers:** `syx run`, `syx check` (and most day-to-day development)

```
Source → Lexer → Parser → Macro / Meta / Async Pass → Semantic
       → syx_mir::lower_module → optimize
       → CodeGenerator (MIR → bytecode)
       → VmInstance::run_program (alias WasmVM)
```

- Full standard library and host functions available
- Suitable for daily development and behavior validation

#### 3.2.2 Native pipeline (`syx build --native`, **LLVM only**)

**Triggers:** `syx build --native [path]` (optional `--object`, `--release`; see UsageGuide for `full-native` / local LLVM)

```
Source → … → Semantic → MIR → optimize
       → syx_llvm (MIR → LLVM IR / object)
       → link libsyx_rt.a → app (or --object for .o only)
```

- **Cranelift AOT removed**; CLI no longer offers `--backend=cranelift`
- **`syx-rt`:** C ABI runtime; **`syx-mir-run`:** shared MIR interpretation (WASM host, etc.)
- Parity details: [10_BackendParity.md](10_BackendParity.md)

#### 3.2.3 WebAssembly pipeline

**Triggers:** `syx build --wasm` / `syx run --wasm`

```
Source → … → MIR → optimize
       → syx_wasm_emit::emit_wasm(MIR) → main.wasm
```

- CLI `syx run --wasm`: guest + host `syx.mir_exec` (`MirRuntime`), close to native `syx run`
- Browser `syx-compiler-wasm` / `examples/wasm_web` is a **separate** subset path; do not conflate with CLI WASM

---

## 4. Crate responsibilities

### 4.1 Frontend crates

| Crate | Role | Key types / APIs |
|-------|------|-------------|
| `syx-diagnostics` | Source locations, spans, structured diagnostics | `SourceFile`, `SyxDiagnostic`, `SyxResult` |
| `syx-lexer` | Single-pass scan, keyword recognition | `Token`, `Lexer::scan` |
| `syx-ast` | AST nodes, visitor traits | `Module`, `Decl`, `Expr`, `Stmt`, `ExprVisitor` |
| `syx-parser` | Recursive descent + Pratt expressions | `Parser::parse` |
| `syx-semantic` | Two-pass analysis, scopes, type compatibility | `SemanticAnalyzer`, `TypeId`, `Scope` |

### 4.2 Middle-end and backend crates

| Crate | Role | Key types / APIs |
|-------|------|-------------|
| `syx-mir` | AST lowering, CFG blocks, optimization passes | `MirModule`, `MirOp`, `lower_module`, `optimize` |
| `syx-codegen` | Instruction selection, function table build | `CodeGenerator`, `CompiledProgram`, `Instruction` |
| `syx-vm` | Stack interpreter, managed heap / GC | `VmInstance` (`WasmVM` alias), `Value` |
| `syx-stdlib` | Builtin types/methods/host registration | `HostFnId`, `MethodId`, `BUILTIN_TYPES`, manifest |
| `syx-llvm` | Native AOT (sole backend) | MIR → LLVM / object → `app` |
| `syx-wasm-emit` | MIR → WASM binary | `emit_wasm` |
| `syx-cranelift-jit` | Experimental JIT tier-2 (opt-in) | `SYX_JIT_TIER2` / Suite-1; **not** AOT |

### 4.3 Compile-time extension crates

| Crate | Role | Depth |
|-------|------|----------|
| `syx-meta` | Data class metadata, Pure/IO effect inference and checks, type reflection | P1 usable |
| `syx-async` | `await` site counting in `async` functions, Task runtime stubs | P2 MVP |

### 4.4 Toolchain crates

| Crate | Role | Depth |
|-------|------|----------|
| `syx-fmt` | AST-driven source formatting | P2 MVP |
| `syx-lint` | Naming, duplicate defs, empty bodies, etc. | P2 MVP |
| `syx-doc` | Markdown API docs from AST | P2 MVP |
| `syx-debug` | Debug sessions, breakpoint tables, DAP stubs | P3 scaffolding |

### 4.5 Orchestration and entry crates

| Crate | Role |
|-------|------|
| `syx-driver` | Module graph, cache, multi-backend dispatch, REPL, dependency management |
| `syx-cli` | Clap subcommands calling public `syx-driver` APIs |

---

## 5. Compilation driver (`syx-driver`)

`syx-driver` is the core orchestration layer, exposing project- and file-level APIs.

### 5.1 Core data structures

| Type | Description |
|------|------|
| `SyxManifest` | Parsed `syx.toml`: name, version, main, dependencies |
| `SyxLockfile` | `syx.lock` pins dependency resolution |
| `ModuleUnit` | Single-module artifact: path, source, imports, `CompiledProgram` |
| `ModuleGraph` | Module dependency graph: compile order + module table |
| `CompiledProject` | Full project: manifest + entry module + graph |

### 5.2 Public API (selected)

| Function | Purpose |
|------|------|
| `compile_file` / `compile_linked` | Single-file compile (recursive dependencies) |
| `compile_project` | Compile entire project from `syx.toml` |
| `compile_with_cache` | Compile with `.sy-cache` incremental cache |
| `check_project` | Type-check only, no execution |
| `run_file` / `run_project` | Compile and run on VM |
| `build_native` / `build_native_with_options` | MIR → native `.o` (LLVM via `DriverOptions`) |
| `build_native_executable` | Same + link `syx-rt` → `app` |
| `build_wasm` | MIR → `main.wasm` |
| `repl_eval` | REPL single-line evaluation |
| `format_project` / `lint_project` / `doc_project` | Toolchain operations |
| `debug_project` | Create `DebugSession` (DAP breakpoints in `syx-debug`) |
| `add_dependency` / `update_dependencies` | Write manifest and lockfile |
| `reflect_type_info` | Query `syx-meta` registered type metadata |

Three-backend debugging and linking: [12_BackendDebuggingGuide.md](12_BackendDebuggingGuide.md).

### 5.3 Compile orchestration

`compile_module_recursive` runs:

1. Normalize paths; detect cycles (return if already in graph)
2. Read source → `Parser::parse`
3. `apply_meta_passes` (data classes, effects, pure-function checks)
4. `prepare_async_module` (async/await constraints)
5. Recursively compile modules referenced by `import`
6. `compile_file_with_context`: merge import symbols → semantic analysis → codegen
7. Record in `ModuleGraph`

`compile_to_program` chooses direct AST codegen or MIR based on `SYX_USE_MIR`.

---

## 6. Command-line tool (`syx-cli`)

`syx-cli` uses Clap derive macros to map user commands to `syx-driver` APIs.

### 6.1 Subcommands

| Command | Description | Driver API |
|------|------|-----------------|
| `syx build [path]` | Compile file or project (VM bytecode) | `compile_project` / `compile_file` |
| `syx build --native [path]` | Build `app` (**LLVM** sole; `--object` for `.o` only) | `build_native_executable` / `build_native` |
| `syx build --wasm [path]` | Emit `main.wasm` | `build_wasm` |
| `syx run [path]` | Compile and run | `run_project` / `run_file` |
| `syx check [path]` | Type-check only | `check_project` / `compile_file` |
| `syx test [path]` | Run `tests/**/*.sy` (recursive) or project entry / single file | `run_file` loop / `run_project` |
| `syx format [path]` | Format entry file | `format_project` |
| `syx lint [path]` | Static analysis | `lint_project` |
| `syx debug [path]` | Start debug session summary | `debug_project` |
| `syx repl` | Interactive REPL | `repl_eval` |
| `syx add <name> <path>` | Add dependency | `add_dependency` |
| `syx update [path]` | Refresh lockfile | `update_dependencies` |
| `syx doc [path]` | Generate `docs/api.md` | `doc_project` |

### 6.2 Path resolution

- Default `path` is current directory `.`
- Directory: use `main` from `syx.toml` as entry
- File: compile as a single unit

### 6.3 Build and install

```bash
# Development
cargo run -p syx-cli -- run examples/hello.sy

# Install to PATH
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

---

## 7. Module system and dependencies

### 7.1 Module syntax

```syx
// math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

// main.sy
import math
func main() {
    print(math.add(1, 2))
}
```

### 7.2 Module resolution

`resolve_module` searches in order:

1. **Embedded stdlib** (`enable_std=true`, default): `math`, `io`, etc.
2. **`syx.lock` installed deps:** manifest package name → lock `path` / `.sy/vendor/`
3. **Same directory as importer:** `./foo.sy`
4. **`--module-dir`** extra search paths

Projects with `dependencies` need `syx install` for a lock file, else **P001**. Missing module → **M001**.

### 7.3 Visibility

- `public` declarations are importable from other modules
- Non-`public` cross-module access → **S005** (internal symbol access)
- Semantic analysis merges export tables via `import_exports`

### 7.4 Manifest formats

**syx.toml:**

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies.math]
version = "1.0"
path = "../math"
```

**syx.lock:**

```toml
[packages.local_math]
source = "path"
version = "1.0.0"
path = "../deps_math"
```

Package management is in **`syx-pkg`:** `install_project` resolves path/git/registry deps and writes the lock; `load_resolved_packages` feeds the driver at compile time.

Registry API: `GET /v1/packages/{name}`, `GET /v1/packages/{name}/{version}/download`, `PUT /v1/packages/{name}/{version}` (publish). Local testing: `file:///path/to/registry`.

---

## 8. Incremental compilation cache

### 8.1 Cache layout

| Item | Description |
|------|------|
| Cache directory | `{project_root}/.sy-cache/` |
| Cache file | `{sha256(source + "syx-cache-v1")}.bin` |
| Serialization | `bincode` `CachedModuleUnit` |
| Payload | Path, source, import list, `CompiledProgram` |

### 8.2 Hit flow

```
compile_linked(path)
  → incremental_cache_path(root, path)
  → if .bin exists and bincode deserializes → return ModuleUnit
  → else full compile → write cache
```

### 8.3 Invalidation

Keys are **source content hashes**; edits invalidate automatically. Fine-grained cross-module invalidation is not implemented (dependency changes do not invalidate downstream caches; clear `.sy-cache` manually).

### 8.4 Related APIs

- `module_hash(path)` — SHA-256 hash
- `incremental_cache_path(root, module)` — cache file path
- `compile_with_cache(path, root)` — explicit cached compile entry

---

## 9. Meta and async compile-time passes

### 9.1 Meta pass (`syx-meta`)

Invoked before semantic analysis via `apply_meta_passes`:

| Pass | Function | Behavior |
|------|------|------|
| Data class registration | `register_data_classes` | Scan `data class` decls into global `TYPE_REGISTRY` |
| Effect inference | `check_effects` | Infer Pure/IO; name heuristics (e.g. `fetch`) |
| Pure function validation | `validate_pure_functions` | `requires Pure` bodies must not perform I/O → `E001` |
| Type reflection | `reflect_type` | Driver queries registered field metadata |

**P3 scaffolding APIs** (not wired into main pipeline yet):

- `compile_time_eval` — compile-time eval stub
- `type_level_contains` — type-level constraint stub
- `service_wiring` — Phase 125 Service DI v6: base/protocol polymorphic inject + field `@Qualifier` + `@Primary` + **narrow `@Around` MVP** (`@Service`/`@Component` instance methods, not AspectJ); M14 `Di.withRequest` / Qualifier/Primary/Profile inheritance

### 9.2 Async pass (`syx-async`)

Invoked before semantic analysis via `prepare_async_module`:

| Rule | Code | Notes |
|------|--------|------|
| `await` only in `async` functions | `A002` | Compile-time reject |
| At most one `await` per async function | `A001` | MVP limit |

**Runtime pieces** (P2 scaffolding):

- `Runtime` — `Future`-based task queue
- `Task<T>` — async task handle
- `blocking` / `blocking_poll` — synchronous blocking stubs

---

## 10. Frontend: lexing, parsing, semantics

### 10.1 Lexer (`syx-lexer`)

- **Strategy:** single-pass scan, O(1) keyword lookup
- **Locations:** `SourceFile::line_col` for line/column reporting
- **Token classes:** declarations, control flow, access control, operators, delimiters, literals (including string interpolation)

### 10.2 Parser (`syx-parser`)

- **Strategy:** recursive descent + Pratt precedence climbing
- **Recovery:** sync to statement boundaries; collect multiple syntax errors when possible
- **Features:** functions, class/struct/enum, protocols, generics, closures, pattern matching, property wrappers, try-catch, import, etc.

### 10.3 Semantic analyzer (`syx-semantic`)

**Two-pass analysis:**

1. **Symbol collection:** walk declarations; build global and export tables
2. **Detailed analysis:** expression/statement typing, access control, method resolution

**Type system (`TypeId`):**

```
Void / Numeric(NumericKind) / Bool / String
Optional<T> / Array<T> / Set<T> / Dictionary<K,V>
Named(name) / Unknown
```

Platform `Int`/`UInt` resolve via `syx-stdlib::numeric::PLATFORM_INT` / `PLATFORM_UINT` on the compile host to concrete `NumericKind`.

**Scope chain:** nested `Scope` with `parent` for block and function scopes.

**Imports:** `analyze_with_imports` takes `(module name, export table)` pairs and validates cross-module references.

---

## 11. Code generation and intermediate representation

### 11.1 Direct AST codegen

`CodeGenerator::compile_into` walks the AST and emits VM instructions:

| AST node | Typical instructions |
|----------|----------|
| Binary numeric expr | `NumericAdd/Sub/Mul/Div` (by `NumericKind`) |
| Numeric literal | `ConstNumeric { kind, bits }` |
| Casts | `NumericCast` |
| Calls | `Call`, `MethodCall` |
| Collection literals | `ArrayBuild`, `SetBuild`, `DictBuild` |
| Conditionals | `JumpIfFalse`, `Jump` |
| Locals | `StoreLocal` |
| Return | `Return` |

### 11.2 MIR

`syx-mir` lowers AST to SSA-style register IR:

- **MirOp:** `ConstI32`, `LoadLocal`, `Add`, `Call`, `JumpIfFalse`, `Return`, etc.
- **MirBlock:** basic block + successor edges
- **MirFunction:** per-function ops and entry block

**Optimization (`optimize`):**

- Constant folding
- Dead code elimination
- Peephole (local instruction merge)

### 11.3 Backend comparison

| Backend | Input | Output | Completeness |
|------|------|------|--------|
| VM (codegen) | AST or MIR | `CompiledProgram` | Full (P0/P1) |
| LLVM | MIR | `.o` or IR text | MVP subset |
| WASM | MIR | `.wasm` | MVP subset |

---

## 12. Virtual machine and runtime

### 12.1 Execution model

`VmInstance` (alias `WasmVM`) uses **stack interpretation**:

```
Operand stack + locals + globals + function table
```

Example: `a + b`

```
[] → local_get(0) → [a] → local_get(1) → [a,b] → i32_add → [a+b]
```

### 12.2 `Value` representation

| Variant | Meaning |
|------|------|
| `I32` / `F64` / `Bool` / `Str` | Scalars |
| `Nil` | Null |
| `Array` | Dynamic array |
| `Object` | Key/value (HTTP responses, etc.) |
| `Iterator` | `for-in` state |

### 12.3 Memory management

| Component | Strategy |
|------|-----------|
| `MemoryPool` | Object pool + free-list reuse |
| `Gc` | Mark-sweep (simplified) |
| `WeakReference` | Weak ref stub |

### 12.4 JIT (P3 scaffolding)

`syx-vm::jit::JitCompiler`:

- Hot-path counting (default threshold 1000 calls)
- `maybe_compile` emits JIT stub bytes; not full native codegen yet
- `run_with_jit` still falls back to interpretation after threshold

---

## 13. Standard library and host functions

### 13.1 Builtin types (`syx-stdlib`)

| Type | Example methods |
|------|----------|
| `Int` / `Double` / `Bool` / `String` | `toString`, `length`, `substring` |
| `Array` | `push`, `pop`, `map`, `filter`, `len` |
| `String` | `split`, `trim`, `hasPrefix`, `lowercased`, `matches` |
| `File` / `Directory` | Via host functions |
| `HTTP` / `HTTPResponse` | `get`, `post`, `json` |

### 13.2 Host function registration

Registered at VM startup in `hosts.rs`; semantic analysis validates via `HostFnId`:

| Host Function | SYX API |
|---------------|---------|
| `__syx_file_read` | `File.read(path)` |
| `__syx_file_write` | `File.write(path, content)` |
| `__syx_file_exists` | `File.exists(path)` |
| `__syx_file_read_lines` | `File.readLines(path)` |
| `__syx_file_append` | `File.append(path, content)` |
| `__syx_file_delete` | `File.delete(path)` |
| `__syx_file_copy` | `File.copy(from, to)` |
| `__syx_file_move` | `File.move(from, to)` |
| `__syx_dir_create` | `Directory.create(path)` |
| `__syx_dir_list` | `Directory.list(path)` |
| `__syx_dir_exists` | `Directory.exists(path)` |
| `__syx_http_get` | `HTTP.get(url)` |
| `__syx_http_post` | `HTTP.post(url, body)` |
| `__syx_http_response_json` | `response.json()` |

HTTP uses `reqwest` (blocking + rustls); file I/O uses `std::fs`.

### 13.3 Iterator protocol

`ITERATOR_PROTOCOL` defines `makeIterator` / `next`; VM `for-in` drives `Value::Iterator`.

---

## 14. Toolchain crates (P2/P3)

### 14.1 syx-fmt

- Parse to AST and re-emit with conventional style
- Formats functions, imports, globals, control flow
- `format_project` overwrites entry file in place

### 14.2 syx-lint

Sample rules:

| Code | Rule |
|------|------|
| `L001` | Function names should start lowercase |
| `L002` | Duplicate function names |
| `L003` | Empty function body |
| `L004` | `public` functions should declare return types |
| `L005` | Async naming suggestions |
| `L006` | Unused imports |

### 14.3 syx-doc

- Extract `///` doc comments
- Emit Markdown signature blocks for public functions/structs
- Output to `{project}/docs/api.md`

### 14.4 syx-debug

- `DebugSession`: PC, function index, breakpoints
- Basic `step` / `continue` / `stack_summary`
- DAP JSON message handling stubs

---

## 15. Error handling and diagnostics

### 15.1 Diagnostic stack

Rich reports via `miette`:

```
error[E0308]: type mismatch
  --> main.sy:5:10
   |
 5 |     let x: Int = "hello"
   |                  ^^^^^^^ expected `Int`, found `String`
```

### 15.2 Error categories

| Stage | Typical codes |
|------|-----------|
| Lexer | Invalid character, unterminated string |
| Parser | Expected token, sync recovery |
| Semantic | `S001` duplicate def, `S005` internal access |
| Meta | `E001` pure function with I/O |
| Async | `A001`/`A002` await rules |
| Modules | `M001` module not found |
| Runtime | VM underflow, cast failures |

### 15.3 Recovery

- **Lexer:** skip bad characters and continue
- **Parser:** sync to statement boundaries
- **Semantic:** collect all errors before returning
- **Runtime:** `try-catch`, with `finally` at language level

---

## 16. Testing

### 16.1 Layout

```
crates/syx-driver/tests/
├── integration.rs           # Basic integration
├── p0_integration.rs        # P0 regression
├── p1_integration.rs        # P1 regression
├── p0_strings.sy
├── p0_arrays.sy
├── p0_file_io.sy
├── p0_http.sy
├── p0_modules.sy
├── p1_features.sy
├── p1_null.sy
└── p1_dataclass.sy

Per-crate #[cfg(test)] unit tests
```

### 16.2 P0 integration coverage

| Test | Validates |
|------|--------|
| `p0_demo_compiles` | Example project compiles |
| `p0_strings_run` | String methods and interpolation |
| `p0_arrays_run` | Arrays and for-in |
| `p0_file_io_run` | File host functions |
| `p0_http_compiles` | HTTP compiles |
| `p0_modules_run` | Cross-module `public` import |
| `p0_modules_internal_rejected` | Internal symbol rejected |
| `p0_deps_project_*` | `syx.toml` dependency projects |

### 16.3 P1 integration coverage

| Test | Validates |
|------|--------|
| `p1_features_compile` | guard let, effects, etc. |
| `p1_null_run` | Optional null safety |
| `p1_dataclass_run` | data class and metadata registration |

### 16.4 Benchmarks

`crates/syx-driver/benches/compile_bench.rs` — compile benchmarks (optional criterion).

---

## 17. Build and CI

### 17.1 Local build

```bash
cargo build              # Debug
cargo build --release    # Release
cargo test --all         # All tests
cargo fmt --all          # Format
cargo clippy --all-targets --all-features -- -D warnings
```

### 17.2 Environment variables

| Variable | Effect |
|------|------|
| `SYX_USE_MIR=1` | Enable MIR optimization pipeline |

### 17.3 CI

GitHub Actions (`.github/workflows/ci.yml`):

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features`
3. `cargo test --all`

Uses `Swatinem/rust-cache` for dependency caching.

---

## 18. Appendix

### Appendix A: Language feature matrix (Rust implementation)

| Feature | Status | Notes |
|------|------|------|
| Scalar types | ✅ | Fixed-width numerics, platform `Int`/`UInt`, Bool, String |
| Collections | ✅ | `Array<T>`, `Set<T>`, `[String: V]` dictionary |
| Variables | ✅ | var, let |
| Functions | ✅ | func, params, return types |
| Class/struct/enum | ✅ | Basic OOP |
| Protocols/extensions | ⚠️ | Basic |
| Generics | ⚠️ | Basic |
| Closures | ⚠️ | Basic |
| Property observers | ⚠️ | willSet/didSet parsed |
| Property wrappers | ⚠️ | `@` syntax parsed |
| Access control | ✅ | public/internal, etc. |
| Exceptions | ✅ | try/catch/throw |
| Optionals | ✅ P1 | `?`, `!`, `??`, guard let |
| Arrays/dicts | ✅ | Basic collections |
| String interpolation | ✅ P0 | `\(expr)` |
| Regex | ✅ P0 | `matches` |
| Module import | ✅ P0 | public exports |
| File I/O | ✅ P0 | Host functions |
| HTTP | ✅ P0 | GET/POST |
| for-in | ✅ P0 | Arrays, string split results |
| data class | ✅ P1 | Metadata registration |
| Effects | ✅ P1 | requires Pure/IO |
| async/await | ⚠️ P2 | Syntax + checks; runtime stubs |
| MIR opts | ⚠️ P2 | Env var gated |
| fmt/lint/doc | ⚠️ P2 | MVP |
| LLVM native | ⚠️ P3 | MVP subset (sole AOT) |
| WASM backend | ⚠️ P3 | MVP subset |
| JIT | ❌ P3 | Hot-path counter stub only |
| Macros | ✅ M18 | `syx-macro` fixed-point expand + Service DI v9 (`@Lazy`, cycle lazy-proxy, proceed, `@Bean`, M17 inherited); see [19-macros.md](../books/language-guide/19-macros.md) · [ADR-120](adr/architecture-decisions.md) |
| IDE/LSP | ❌ | Not implemented |

### Appendix B: Performance targets

| Metric | Target | Current |
|------|------|------|
| Compile throughput | 1000+ LOC/s | Pending benchmark |
| Compile memory | < 100 MB | Lightweight Rust implementation |
| VM startup | < 100 ms | Met |
| GC pause | < 10 ms | Simplified GC; small heaps OK |

### Appendix C: Roadmap sketch

**Near term (P1 finish)**

- Null safety and data class runtime polish
- More lint rules and formatter coverage

**Mid term (P2)**

- MIR on by default; more optimization passes
- Full async/await runtime
- DAP debugger polish

**Long term (P3)**

- Full LLVM instruction coverage
- WASM + WASI
- Real JIT
- LSP and IDE plugins

---

**Document version:** v2.0 (Rust implementation)  
**Last updated:** 2026-06-14  
**Maintainer crate:** `syx-driver` (orchestration entry)
