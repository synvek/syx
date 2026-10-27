# SYX Complete Usage Guide

> **Phase 139 (SYX 117.0):** Language/toolchain under Truth M0–M4; SIMD16 guest-v22; SynUI deferred; Farm/Dist frozen v19; audit **2600/3050**; `syx-cache-v1`; native AOT **LLVM sole**; license [Apache-2.0](../LICENSE).  
> See [architecture-decisions.md](adr/architecture-decisions.md) and [implementation-status](../books/shared/implementation-status.md).

[简体中文](./zh/9_UsageGuide.zh-CN.md)

This document is for **SYX language users and toolchain users**. It covers installation, CLI, project configuration, language features, the standard library, toolchain, and advanced compile options. For compiler implementation details, see the compiler documentation in [docs/README.md](./README.md).

---

## Table of Contents

1. [Introduction](#introduction)
2. [Installation and build](#installation-and-build)
3. [Quick start](#quick-start)
4. [CLI command reference](#cli-command-reference)
5. [Project configuration](#project-configuration)
6. [Language basics](#language-basics)
7. [Modules and dependencies](#modules-and-dependencies)
8. [Standard library API](#standard-library-api)
9. [Error handling](#error-handling)
10. [Null safety and Optional](#null-safety-and-optional)
11. [Advanced language features](#advanced-language-features)
12. [Toolchain](#toolchain)
13. [Compilation and backends](#compilation-and-backends)
14. [Testing](#testing)
15. [Example projects](#example-projects)
16. [Environment variables](#environment-variables)
17. [Diagnostic codes](#diagnostic-codes)
18. [FAQ](#faq)
19. [Related documentation](#related-documentation)

---

## Introduction

SYX is a Swift-inspired programming language, currently implemented as a **full toolchain written in Rust**:

```
Source (.sy) → Lexer → Parser → Semantic → CodeGen → VM
                      ↘ (optional) MIR → optimize ↗
                      ↘ (optional) LLVM / WASM backends
```

**Default execution:** compile to stack bytecode, interpret in **VmInstance** (alias `WasmVM`), with file I/O, HTTP, and other capabilities via **Host Functions**.

**System requirements**

| Component | Version |
|------|------|
| Rust | 1.80+ |
| Cargo | Bundled with Rust |
| clang / LLVM | Optional: with `--features full-native`, use sibling `../llvm` or `SYX_LLVM_PREFIX` (inkwell emits `.o` + **static embedded LLD**); see [plan/topics/native-llvm.md](../plan/topics/native-llvm.md) |

---

## Installation and build

### Build from source

```bash
git clone <repo-url> synswx
cd synswx

# Debug build
cargo build

# Release build (recommended for daily use)
cargo build --release
```

After building, invoke the CLI via Cargo:

```bash
cargo run -p syx-cli -- run examples/hello.sy
```

Add the release binary to PATH (optional):

```bash
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

### Verify installation

```bash
cargo test
```

When all pass, the toolchain is ready to use.

---

## Quick start

### Single-file program

Create `hello.sy`:

```syx
func main() {
    print("Hello, SYX!")
}
```

Run:

```bash
syx run hello.sy
# or
cargo run -p syx-cli -- run hello.sy
```

### Project program

Directory layout:

```
my-app/
├── syx.toml
├── build.syb      # optional: build config (Sy Build, not source)
└── main.sy
```

`syx.toml`:

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"
build = "build.syb"   # optional; when omitted, defaults to build.syb if present
```

`main.sy`:

```syx
func main() {
    print("Hello from project!")
}
```

Run:

```bash
cd my-app
syx run .
# or
syx init my-app          # generates main.sy + build.syb + syx.toml
```

---

## CLI command reference

| Command | Description | Example |
|------|------|------|
| `syx run [path]` | Compile and run a file or project | `syx run examples/file_tool` |
| `syx run --gui-visible [path]` | **retired** — legacy `libsyx_gui`/egui path; use SynUI `packages/ui` + `SYX_SYNUI_RUN=1` opt-in `run()` | See SynUI v2 notes below |
| `syx build [path]` | Compile (do not run) | `syx build examples/` |
| `syx build --native [path]` | MIR → native `app` (**LLVM sole**; `--object` only `.o`) | `syx build --native .` |
| `syx build --wasm [path]` | MIR → WASM → `main.wasm` | `syx build --wasm .` |
| `syx emit [path] --format FMT` | Export bytecode / IR (see table below) | `syx emit examples/hello.sy --format syxbc` |
| `syx run --wasm [path]` | Compile and run WASM (project / `.sy` / `.wasm`; supports `print` constants) | `syx run --wasm examples/hello.sy` |
| `syx check [path]` | Type-check only | `syx check main.sy` |
| `syx test [path]` | Run `tests/**/*.sy` (recursive; `--filter` / `--format human\|json\|tap\|junit`); or a single `.sy` | `syx test . --format junit` |
| `syx bench` / `cargo bench -p syx-driver --bench …` | C-09 narrow `[x]`: thin allowlist CLI + Criterion / `compile_bench` (≠ coverage gate / fuzz / SLA) | `syx bench --list` · `syx bench path_join_criterion` |
| `syx cov` / `cargo llvm-cov -p syx-driver --test native --no-report` | C-09 coverage narrow claimed (docs + CLI; ≠ % gate / ≠ CI fail / ≠ fuzz) | `syx cov` (requires local `cargo-llvm-cov`) |
| `syx fuzz` / `cargo fuzz run syx_smoke` | C-09 fuzz narrow claimed + scaffold + CI `-runs=1` + C-09/platform maintenance floor (≠ scaffold GA / ≠ unbounded fuzz) | `syx fuzz` (requires cargo-fuzz; CI **met**; maintenance **met**) |
| `syx prop` / `cargo test -p syx-driver --test property_smoke` | C-09 property narrow claimed + `property_smoke` + CI first-slice + C-09/platform maintenance floor (≠ proptestest GA / ≠ `c09_property_ci_ga`) | `syx prop` |
| `syx format [path]` | Format the entry file | `syx format .` |
| `syx format --check [path]` | dry-run: recursively check `.sy` idempotence (no disk writes; C-06) | `syx format --check .` |
| `syx lint [path]` | Static analysis | `syx lint .` |
| `syx doc [path]` | Generate API docs | `syx doc .` → `docs/api.md` |
| `syx debug [path]` | Start debug session summary | `syx debug .` |
| `syx repl` | Interactive REPL | `syx repl` |
| `syx add <name> --path PATH` | Add path dependency and install | `syx add utils --path ../utils` |
| `syx add <name> --git URL [--rev TAG]` | Add git dependency | `syx add lib --git https://... --rev v1.0` |
| `syx install [path]` | Resolve and install dependencies; write `syx.lock` | `syx install .` |
| `syx update [path]` | Re-resolve and install all dependencies | `syx update .` |
| `syx publish [path] [--registry URL]` | Publish package to registry | `syx publish .` |

### Ed25519 publish → verify (C-05)

Publish and install share one Ed25519 key pair (single publisher pin; ties to C-02 trust-root):

1. **Publish:** set `SYX_PUBLISHER_SIGNING_KEY` to 64 hex chars of a 32-byte seed → `syx publish` → signature form `syx1:ed25519:<pk>:<sig>` (signed over checksum). Without key, lock v8 policy rejects signing with **P012**.
2. **Install:** set `SYX_REGISTRY_VERIFY_KEY` to 64 hex matching publisher verifying key → verify on install; embedded public key must match pin (C-02); mismatch or verify failure → **P011**.
3. **Honest boundary:** ≠ trust-root GA / multi-publisher / transparency log / new crypto; cross-evidence see `c02_*` (syx-pkg) and `registry_signed_*` (evergreen).

**Path argument:** when omitted, defaults to current directory `.`. If the path is a directory, reads `syx.toml` in that directory; if it is a `.sy` file, compiles as a single file.

**Global options:**

| Option | Description | Example |
|------|------|------|
| `--module-dir PATHS` | Extra module search directories; separate paths with `;` | `syx --module-dir "./vendor" run main.sy` |
| `--enable-std` | Enable embedded standard library (default `true`) | `syx --enable-std=false run main.sy` |
| `--backend BACKEND` | Legacy flag; native AOT **LLVM sole** (Cranelift AOT removed) | `syx build --native .` |

Standard library modules (`math`, `io`, etc.) are embedded in the `syx` binary at **highest priority by default**. With `--enable-std` off, `import math` resolves from the local directory or `--module-dir`.

Core `Math` enhancement track (Soft / Special / F32·F64 instances) **closeout→maintenance sign-off** (HostFn **629**). Math llvm: **84/84** + narrow-surface GA; QS1 Math **84/84 + narrow `qs1_math_ga=true`**; Math status **84/84 `status=implemented`**; Fmt status pilot **FmtPrint/FmtPrintLine×2**; Path status pilot **PathJoin/Normalize/Exists/Extension×4**; intermediate first-slice meta F-10 shrink **×33**; File status pilot **FileOpen/Close/Read/Write×4**; File rest **×11 / full File\* `status=implemented`**; Http rest **×16 / full Http\* `status=implemented`**; Sql core **×4 `status=implemented`**; Sql rest **×23 / full Sql\* `status=implemented`**; Tcp **×7 / full Tcp\* `status=implemented`**; Task **×8 / full Task\* `status=implemented`**; Time/DateTime **×8 `status=implemented`**; Json **×11 / full Json\* `status=implemented`**; Str/Unicode **×11 `status=implemented`**; Process/Child wasm-unsupported honesty **×14**; Log core ×4 mir+llvm **real**; Log rest ×8 / family ×12 mir+l … ≠ full HostFn status; still 14 partial). C-09 / platform / Math maintenance chain at **closeout**. See [docs/lang/qs1-d01-execute-adr-first-slice.md](lang/qs1-d01-execute-adr-first-slice.md).

## Project configuration

### `syx.toml` manifest

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies]
# Simple path dependency
utils = "../utils/main.sy"

# Path dependency with SemVer
[dependencies.local_math]
version = "1.0.0"
path = "../deps_math"

# git dependency
[dependencies.lib]
version = "^0.2"
git = "https://github.com/org/lib.git"
rev = "v0.2.1"

# registry dependency (requires `[registry]`)
[registry]
default = "https://packages.sy.dev"

[dependencies.http-client]
version = "^1.0"
```

| Field | Description | Default |
|------|------|--------|
| `name` | Project name | `"app"` |
| `version` | Project version | `"0.1.0"` |
| `main` | Entry SYX source (`.sy`) | `"main.sy"` |
| `build` | Build config file path (`.syb`) | When omitted, default `build.syb` (if present) |
| `dependencies` | Module dependency map | empty |
| `[plugins].paths` | Host plugin manifest paths (Phase 49+) | empty |
| `[macros].exports` | Package exported macro names (Phase 51+) | empty |
| `[registry].default` | Default package registry URL | `SYX_REGISTRY` or production default `https://packages.sy.dev` |

### `build.syb` build configuration (Sy Build)

**Extension `.syb` = Sy Build** is a **declarative build DSL**, **not** Sy source code. Do not write `func` / `class` / business logic in it; use `.sy` for source.

- Priority: `native` / `plugins` / `features` / `compileOptions` in `build.syb` **override** same-named settings in `syx.toml`; `syx.toml` mainly holds `name` / `version` / `main` / `dependencies` / `build` path.
- Generated by `syx init`; use `build = "path/to/foo.syb"` for a custom path.
- Parser is a line/block DSL (not the full Sy compiler); task graph is a DAG, not a Turing-complete script.

Minimal example:

```syb
build {
    compileOptions {
 optLevel = release # or debug
    }
}
```

Common blocks:

| Block | Role | Example |
|----|------|------|
| `compileOptions` | Optimization level | `optLevel = release` |
| `native` | Link libraries and flags | `libs = ["z"]` · `link_flags = ["-pthread"]` |
| `plugins` | Host plugin list | `"mycrypto"` |
| `features` | Feature flags | `wasm_aot = true` |
| `task <name>` | Task DAG | `target = "wasm"` · `dependsOn = ["mir"]` |

Task `target` values (executor supports): `mir` · `wasm` · `native` · `link` · `test`, plus farm-related `remote` / `farm_queue` / `farm_worker` (requires URL or environment variables).

Example with task graph:

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

`syx build` / `syx build --native` / `syx build --wasm` read this config under a project directory and merge it into driver options; if `task` entries exist, they run in dependency order.

### `syx.lock` lockfile

Generated after `syx install` / `syx add` / `syx update`. **Run install before compiling a project with dependencies** (otherwise **P001**).

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

Git / registry packages install to `.sy/vendor/` (listed in `.gitignore`). Path dependencies record the path only; source is not copied.

### Incremental compile cache

Compile artifacts are cached at `target/cache/` under the project root (MIR/module bincode, fingerprint `syx-cache-v1`). If behavior looks wrong, delete the cache and retry:

```bash
rm -rf target/cache
```

---

## Language basics

### Types

| Type | Example | Description |
|------|------|------|
| `Int` | `42` | Integer |
| `Double` | `3.14` | Floating point |
| `Bool` | `true` / `false` | Boolean |
| `String` | `"hello"` | String |
| `Array<T>` | `[1, 2, 3]` | Array literal |
| `T?` | `nil` / `42` | Nullable type |
| Dictionary | `{ "k": "v" }` | Key-value object literal |

### Variables

```syx
var count = 0 // Mutable
let name = "SYX" // Immutable
var x: Int = 10 // Explicit type
```

### Control flow

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

### Functions

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

### Strings

```syx
let s = "Hello, \(name)!" // interpolation
let parts = "a,b,c".split(",") // split
let lower = "ABC".lowercased() // lowercase
let ok = "file.txt".hasPrefix("file")
let sub = "hello".substring(0, 3)
let replaced = "foo".replacingOccurrences("bar", "foo")
let hits = "abc123".matches("\\d+") // regex match, returns array
let pattern = /\\d+/ // regex literal (some contexts)
```

### Printing

```syx
print("message")
print(value)
```

---

## Modules and dependencies

### Importing modules

```syx
import math

func main() {
    let sum = math.add(1, 2)
    print(sum)
}
```

Dependency modules must export **`public`** functions; `internal` / `private` symbols cannot be accessed across modules.

**Example module** `math.sy`:

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {
    return 0
}
```

Cross-module access to `math.secret()` fails at semantic analysis with **S005**.

### Dependency path resolution

- Relative path: `../math.sy` points to a specific SYX file
- Directory path: points to a subproject directory containing `syx.toml`
- Use `syx add math ../math.sy` to write the manifest and update the lockfile

### Multi-module project example

See [examples/deps/](../examples/deps/) and [examples/deps_math/](../examples/deps_math/):

```bash
syx install examples/deps # generate syx.lock
syx run examples/deps # outputs 42 (local_math.add(10, 32))
```

---

## Standard library API

The standard library is exposed via **static type methods** and **instance methods**, managed by the `syx-stdlib` registry.

### File

| Method | Description |
|------|------|
| `File.read(path)` | Read entire file → `String` |
| `File.readLines(path)` | Read by line → `Array<String>` |
| `File.write(path, content)` | Write file |
| `File.append(path, content)` | Append content |
| `File.exists(path)` | Whether file exists → `Bool` |
| `File.delete(path)` | Delete file |
| `File.copy(from, to)` | Copy file |
| `File.move(from, to)` | Move file |

`FileHandle` (`import IO`)provides with `throws IOError` open/read/write/close/seek/readAllBytes.Path assembly uses prelude `Path.join` / `normalize` / `exists` / `fileExtension` (OS separator contract: / `ec_m2_1_path_contract`; ≠ Path GA).Path/File HostFn availability honest write-back in capability matrix; native/WASM column honest deepen pinned (`ec_m2_4_path_file_native_wasm_matrix`); PathJoin llvm GA first tier pinned (`ec_m2_4_path_file_native_wasm_ga`; `PathJoin` `llvm=real` + native AOT execution evidence); PathExtension llvm GA second tier pinned (`ec_m2_4_path_file_native_wasm_ga2`; `PathExtension` `llvm=real` + native AOT `"a/b.txt"` → `"txt"`); PathNormalize llvm GA third tier pinned (`ec_m2_4_path_file_native_wasm_ga3`; `PathNormalize` `llvm=real` + native AOT `"a/b/../c"` → `"a/c"`; PathExists/File* still `llvm=partial` / `wasm=host-assisted` / product `native=partial`; ≠ full Path/File GA ≠ wasm GA ≠ EC-M2-4 full exit).`Path.join`×N Instant microbench in test layer (`ec_m2_4_path_file_microbench`; ≠ perf SLA ≠ EC-M2-4 full exit).`Path.join`×N Criterion formal benchmark first tier in `cargo bench` (`ec_m2_4_path_file_criterion`; ≠ absolute SLA gate ≠ EC-M2-4 full exit).Example: `examples/file_tool/` ( EC-M2-1 slice).live count `file_handle_live_count` test/diagnostic layer only ( EC-M2-3 first pin), **not** application API.

**EC-M2-3-Z: ** post-Sx four production paths (File `withOpen` / Sql `poolClose` / ProcessBuilder+`Child.wait` / Tcp host-release)live_count live_count baseline pin signed; ≠ EC-M2-3 total gate ≠ Q-S5. Pin tests: `ec_m2_3_z_closeout`.

**EC-M2-4-Z: ** A–H eight-axis deepen track (availability/µbench/Criterion/matrix/GA1–3 closeout/Tcp GA4)evidence chaincloseout signed; ≠ absolute SLA ≠ product `native=real`/`wasm=real` ≠ full HostFn GA. Pin tests: `ec_m2_4_z_closeout`.

**EC-M2-4 total gate: ** deepen-track exit criteria A–H + Z signed (honest narrowing); four product slicesstill `native=partial` / `wasm=host-assisted`; ≠ absolute SLA ≠ backends=`real`. Pin tests: `ec_m2_4_total_closeout`.

**M2 exit: ** four EC total gate signedthin exit under signed total gates; open debt (SLA / backends=`real` / full HostFn GA / Path GA / TLS server·accept)**does not block exit**. Pin tests: `m2_exit_closeout`.

**M3-A01 task tree first pin:** `Concurrency.taskGroupCancel` cancels in-group child tasks; `taskGroupWait` returns promptly after cancel; child `join` surfaces **C004**; ≠ silent-leak GA ≠ EC-M3-* ≠ deadline. Pin tests: `m3_a01_task_tree`.

**M3-A01 silent-leak deepen: ** `Concurrency.liveTaskCount` / `__syx_task_live_count`; structured `taskGroupWait` (includes cancel+wait) off, live_count returns to baseline; not in group unreaped detach observable `LEAKED`; ≠ EC-M3-2 ≠ A01 leaf full checkmark (this window)≠ deadline. Pin tests: `m3_a01_silent_leak`.

**M3-A01 leaf thin closeout: ** task tree + silent-leak dual-axis evidence chain closeout; roadmap `M3-A01` `[x]`; ≠ EC-M3-2 ≠ full structured-concurrency GA ≠ A02–A03 ≠ M3 exit. Pin tests: `m3_a01_closeout`.

**M3-A02 deadline first pin:** `Concurrency.setDeadlineMs`; child tasks inherit; deadline cooperative cancel; long sleep returns promptly; join **C004**; ≠ TaskLocal GA ≠ EC-M3 ≠ A02 full checkmark. Pin tests: `m3_a02_deadline`.

**M3-A02 task-local deepen: ** `Concurrency.taskLocalSet` / `taskLocalGet`; child tasks clone-inherit; child writes do not propagate to parent; ≠ `@TaskLocal` ≠ EC-M3 ≠ A02 full checkmark (this window). Pin tests: `m3_a02_task_local`.

**M3-A02 leaf thin closeout: ** deadline + task-local dual-axis evidence chain closeout; roadmap `M3-A02` `[x]`; ≠ EC-M3 ≠ `@TaskLocal` GA ≠ A03 ≠ M3 exit. Pin tests: `m3_a02_closeout`.

**M3-A03 actor reentrancy first pin:** `Concurrency.actorReentrancyPolicy`; same-instance synchronous re-entry → **C009**; ≠ Swift await-reentrancy GA ≠ EC-M3 ≠ A03 full checkmark. Pin tests: `m3_a03_reentrancy`.

**M3-A03 await-hold deepen: ** policy → `serial-no-reenter;await-hold`; no mid-await mailbox interleaving during handler sleep/await; ≠ Swift await-reentrancy GA ≠ EC-M3 ≠ A03 full checkmark (this window). Pin tests: `m3_a03_await_reentrancy`.

**M3-A03 leaf thin closeout: ** reentrancy + await-hold dual-axis evidence chain closeout; roadmap `M3-A03` `[x]`; ≠ EC-M3 ≠ Swift await-reentrancy GA ≠ B01 ≠ M3 exit. Pin tests: `m3_a03_closeout`.

**M3-B01 isolation thin first pin:** separate `blocking_pool`; `Concurrency.blockingSleep` / `isolationPolicy`; Task pool size=1 long blocks must not starve short Tasks; did not routed to `__syx_task_sleep` / HTTP/File; ≠ epoll reactor ≠ B02 ≠ EC-M3 ≠ B01 leaf `[x]`. Pin tests: `m3_b01_isolation`.

**M3-B01 isolation deepen: ** `Concurrency.setBlockingPoolSize` / `blockingPoolSize`; fingerprint `task-pool+blocking-pool-v1-sized`; size=2 parallel `blockingSleep` + size=1 queue must not starve short Tasks; ≠ B01 leaf `[x]` (at that time) ≠ B02 ≠ EC-M3 ≠ reactor GA. Pin tests: `m3_b01_pool_sizing`.

**M3-B01 leaf thin closeout: ** isolation + sizing dual-axis evidence chain closeout; roadmap `M3-B01` `[x]`; ≠ EC-M3 ≠ epoll reactor GA ≠ B02 ≠ M3 exit. Pin tests: `m3_b01_closeout`.

**M3-B02 async adapter thin first pin:** `File.readAsync` / `__syx_file_read_async` + `HttpRequestAsync` routed to `blocking_pool`; fingerprint was `v1-thin` (after deepen tier bump see); ≠ epoll reactor GA ≠ sync File·Http reroute ≠ EC-M3. Pin tests: `m3_b02_async_adapter`.

**M3-B02 async adapter deepen: ** `TcpStream.readAsync` / `__syx_tcp_read_async` via `blocking_pool`; fingerprint `blocking-pool-async-adapter-v1-tcp`; ≠ TcpWrite/Connect async ≠ accept GA ≠ epoll reactor GA ≠ HttpRequestAsync capability TOML ≠ EC-M3. Pin tests: `m3_b02_tcp_async`.

**M3-B02 leaf thin closeout: ** File/HTTP thin + Tcp deepen dual-axis evidence chain closeout; roadmap `M3-B02` `[x]`; ≠ EC-M3 ≠ epoll reactor GA ≠ TcpWrite·Connect async GA ≠ M3 exit. Pin tests: `m3_b02_closeout`.

**M3-C01 Mutex RAII thin first pin:** `Mutex.lockGuard` → `MutexGuard` (`deinit` quiet unlock); fingerprint was `v1-thin` (after deepen tier bump see); ≠ C02 ≠ EC-M3. Pin tests: `m3_c01_mutex_raii`.

**M3-C01 Mutex blocking deepen: ** `__syx_mutex_lock_wait` (Condvar wait when held); product `Mutex.lock` / `lockGuard` routed to wait; fingerprint `mutex-raii-guard-v1-blocking`; cross-Task contention evidence; try-lock HostFn retained; ≠ C02 ≠ EC-M3. Pin tests: `m3_c01_mutex_blocking`.

**M3-C01 leaf thin closeout: ** RAII thin + blocking deepen dual-axis evidence chain closeout; roadmap `M3-C01` `[x]`; ≠ C02 ≠ EC-M3 ≠ M3 exit. Pin tests: `m3_c01_closeout`.

**M3-C02 Semaphore thin first pin:** `Semaphore.create/wait/tryWait/signal`; fingerprintwas `semaphore-v1-thin`; Pin tests: `m3_c02_semaphore`.

**M3-C02 SemaphoreGuard RAII deepen: ** `SemaphoreGuard` + `Semaphore.waitGuard`; fingerprint `semaphore-v1-raii`; Pin tests: `m3_c02_semaphore_raii`.

**M3-C02 leaf thin closeout: ** dual-axis evidence chain (thin + RAII)→ leaf `[x]` (Semaphore slice); ≠ RWLock ≠ Condvar product ≠ EC-M3 ≠ C03. Pin tests: `m3_c02_closeout`.

**M3-C03 memory ordering / Atomic docs first pin:** cross-Task shared mutation → `Mutex` / `Semaphore` / `Channel` establish happens-before; no product `Atomic*` / `memory_order` enum; Wasm atomics **unsupported**; race/stress → C04. Pin tests: `m3_c03_memory_ordering`.

**M3-C03 leaf thin closeout: ** docs evidence chaincloseout → leaf `[x]` (memory ordering/Atomic honest documentation slice); ≠product Atomic API ≠ Wasm atomics full checkmark ≠ C04 ≠ EC-M3. Pin tests: `m3_c03_closeout`.

**M3-C04 race/stress CI samplefirst pin:** machine-readable sample library `race_stress_samples.toml` + cross-Task Mutex sample smoke; Pin tests: `m3_c04_race_stress`.

**M3-C04 chaos sampledeepen: ** TaskGroup cancel interrupt long sleep → C004; sample library `kind=chaos`; Pin tests: `m3_c04_chaos`.

**M3-C04 leaf closeout: ** stress + chaos dual-axis evidence chain → `[x]`; ≠ TSAN ≠ kill-process/network-partition chaos GA ≠ EC-M3. Pin tests: `m3_c04_closeout`.

**M3-D01 structured logging thin first pin:** `Log.setMinLevel` / `infoFields` / fingerprint `log-v1-thin`; Pin tests: `m3_d01_structured_logging`.

**M3-D01 all levels `*Fields` deepen: ** `debugFields` / `warnFields` / `errorFields`; fingerprint `log-v1-fields`; Pin tests: `m3_d01_log_fields`.

**M3-D01 leaf closeout: ** thin + fields dual-axis evidence chain → `[x]`; fingerprint `log-v1-fields`; ≠ JSON GA ≠ D02–D04 ≠ EC-M3. Pin tests: `m3_d01_closeout`.

**M3-D02 Trace/Span thin first pin:** `Trace.start` / `end` / `current` / `contextLine` (includes `task_id`/`actor_id`)/ fingerprint `trace-v1-thin`; ≠ OTel/W3C propagation GA ≠ D02 `[x]` ≠ D03–D04 ≠ EC-M3. Pin tests: `m3_d02_trace_span`.

**M3-D02 nested + Log context deepen: ** nested LIFO; `Log.*Fields` auto-inject `span`/`task_id`/`actor_id`; fingerprint `trace-v1-logctx`; ≠ OTel GA ≠ cross-Task span inheritance ≠ D02 `[x]` ≠ EC-M3. Pin tests: `m3_d02_trace_logctx`.

**M3-D02 leaf closeout: ** thin + logctx dual-axis evidence chain → `[x]`; fingerprint `trace-v1-logctx`; ≠ OTel GA ≠ cross-Task inherit ≠ D03–D04 ≠ EC-M3. Pin tests: `m3_d02_closeout`.

**M3-D03 Metrics Counter thin first pin:** `Metrics.inc` / `get` / `reset` / `snapshot` / fingerprint `metrics-v1-thin`; ≠ OTel/Prometheus export ≠ Gauge/Histogram/labels ≠ D03 `[x]` ≠ D04 ≠ EC-M3. Pin tests: `m3_d03_metrics_counter`.

**M3-D03 Gauge + labeled counter deepen: ** `Metrics.gaugeSet` / `gaugeGet` / `incLabels` / `getLabels`; snapshot includes `g:name=v` with `name{k=v}=n`; fingerprint `metrics-v1-gauge-labels`; ≠ OTel/Prometheus export ≠ Histogram ≠ D04 ≠ EC-M3. Pin tests: `m3_d03_metrics_gauge_labels`.

**M3-D03 leaf thin closeout: ** thin + deepen dual-axis evidence chain → `M3-D03` `[x]`; fingerprintkeep `metrics-v1-gauge-labels`; ≠ OTel/Prometheus export ≠ Histogram ≠ D04 ≠ EC-M3. Pin tests: `m3_d03_closeout`.

**M3-D04 Profiler section wall-clockthin first pin:** `Profiler.start` / `stop` / `elapsed` / `reset` / `snapshot` / fingerprint `profiler-v1-thin`; ≠ sampling GA ≠ CPU-time GA ≠ pprof export ≠ D04 `[x]` ≠ EC-M3. Pin tests: `m3_d04_profiler_sections`.

**M3-D04 process CPU sectiondeepen: ** `Profiler.cpuStart` / `cpuStop` / `cpuElapsed`; snapshot includes `c:name=ms`; fingerprint `profiler-v1-cpu`; ≠ sampling GA ≠ pprof export ≠ EC-M3. Pin tests: `m3_d04_profiler_cpu`.

**M3-D04 leaf thin closeout: ** thin + deepen dual-axis evidence chain → `M3-D04` `[x]`; fingerprintkeep `profiler-v1-cpu`; ≠ sampling GA ≠ pprof export ≠ EC-M3. Pin tests: `m3_d04_closeout`.

**EC-M3-4 Observability evidence chainthin first pin:** D01–D04 four-leaf closeout + fingerprintevidence chain (`log-v1-fields` / `trace-v1-logctx` / `metrics-v1-gauge-labels` / `profiler-v1-cpu`); roadmap footnote thin delivered; ≠ sampling ≠ pprof ≠ OTel ≠ M3 exit. Pin tests: `ec_m3_4_observability`.

**EC-M3-4 leaf thin closeout: ** thin evidence chain → `EC-M3-4` total gate `[x]` (honest narrowing); ≠ EC-M3-1～3 ≠ sampling ≠ pprof ≠ OTel ≠ M3 exit. Pin tests: `ec_m3_4_closeout`.

**EC-M3-2 Silent-leak evidence chainthin first pin:** A01 closeout + task-tree/silent-leak dual-axis + `liveTaskCount` / `TaskLiveCount`; roadmap footnote thin delivered; ≠ async-let GA ≠ non-group reclamation complete ≠ EC-M3-1/3 ≠ M3 exit. Pin tests: `ec_m3_2_silent_leak`.

**EC-M3-2 leaf thin closeout: ** thin evidence chain → `EC-M3-2` total gate `[x]` (honest narrowing); ≠ async-let GA ≠ non-group reclamation complete ≠ EC-M3-1/3 ≠ M3 exit. Pin tests: `ec_m3_2_closeout`.

**M3-E01 permission model docsthin first pin:** five domains default-deny intent (`docs/capability-sandbox-policy-v0.md`); fingerprint `permission-policy-v0:default-deny-intent`; roadmap `M3-E01` `[~]`; ≠ runtime enforcement ≠ CLI grant ≠ HostFn gate ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e01_permission_model`.

**M3-E01 leaf thin closeout: ** policyDocumentevidence chaincloseout; roadmap `M3-E01` `[x]`; ≠ runtime enforcement ≠ E02–E03 ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e01_closeout`.

**M3-E02 permission grants thin first pin:** opt-in `--permission-sandbox` + `--allow-net` / `SYX_ALLOW_NET`; fingerprint `permission-grants-v0:allow-net-cli`; unauthorized net HostFn → `S024` (since W52; formerly `S023`); roadmap `M3-E02` `[~]`; ≠ five domains GA ≠ E03 ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e02_permission_grants`.

**M3-E02 permission grantsdeepen: ** `--allow-fs` / `SYX_ALLOW_FS` + `syx.toml` `[permissions]`; fingerprint `permission-grants-v0:allow-net-fs-toml`; `FileRead`/`FileWrite` unauthorized → `S024`; roadmap `M3-E02` still `[~]`; ≠ five domains GA ≠ path allowlist ≠ E03 ≠ EC-M3-3. Pin tests: `m3_e02_permission_fs`.

** D-04 env+process thin gate: ** opt-in `--allow-env` / `--allow-process`; unauthorized `EnvGet`/`EnvSet`/`ProcessExec`/`ProcessSpawn` → `S024`; ≠ five domains GA ≠ default-on ≠ plugin gate ≠ D-04 `[x]`. Pin tests: `qs1_d04_permission_env` / `qs1_d04_permission_process`.

** D-04 env+process thin gate: ** opt-in `--allow-env` / `--allow-process`; unauthorized `EnvGet`/`EnvSet`/`ProcessExec`/`ProcessSpawn` → `S024`; ≠ five domains GA ≠ default-on ≠ plugin gate ≠ D-04 `[x]`. Pin tests: `qs1_d04_permission_env` / `qs1_d04_permission_process`.

** D-04 plugin thin gate: ** opt-in `--allow-plugin`; unauthorized `__syx_plugin_*` → `S024`; ≠ five domains GA ≠ default-on ≠ plugin-id allowlist ≠ D-04 `[x]`. Pin tests: `qs1_d04_permission_plugin`.

** HostFn intermediate meta F-10: ** removed construction-chain intermediate first-slice meta ×21; retained status closeout / Process honesty / D-04/F-08 live floor; ≠ D-04 `[x]` ≠ F-08 `[x]` ≠ full HostFn status. Pin tests: `qs1_hostfn_intermediate_meta_f10_first_slice`.

** D-04 default-on productdecision ADR: ** written decision = **keep opt-in**; sandbox **not** default-on; default-on **deferred to**explicit execution window; ≠ D-04 `[x]` ≠ five domains GA ≠ sandbox on by default. Pin tests: `qs1_d04_default_on_adr_first_slice`.

** D-04 opt-in written narrowed scope: ** D-04 **`[x]`** = opt-in + five domainsthin gates aligned; ≠ sandbox on by default ≠ five domains default-on GA ≠ default-on After. Pin tests: `qs1_d04_optin_soft_close_first_slice`.

** D-01 productAfter ADR: ** D-01 **product execution ADR met**; row **still frozen**; ≠ load-farm GA ≠ load-test scaffold landed ≠ D-01 `[x]`. Pin tests: `qs1_d01_execute_adr_first_slice`.

** D-01 scaffold: ** existing HTTP Criterion pinned as reproducible report thin entry; row **still frozen**; ≠ load-farm GA ≠ new Criterion ≠ D-01 `[x]`.  Reproduce: `cargo bench -p syx-driver --bench http_response_criterion -- --sample-size 10 --warm-up-time 1 --measurement-time 2`. Pin tests: `qs1_d01_scaffold_first_slice`.

** D-01 deepen: ** existing path/sql/tcp Criterion pinned as reproducible report; row **still frozen**; ≠ load-farm GA ≠ actor Criterion ≠ D-01 `[x]`.  Reproduce: `cargo bench -p syx-driver --bench path_join_criterion` / `--bench sql_exec_criterion` / `--bench tcp_connect_criterion` (same `--sample-size 10 --warm-up-time 1 --measurement-time 2`). Pin tests: `qs1_d01_deepen_first_slice`.

** D-01 load-farm GA charter: ** written freeze future honest claim load-farm GA in-scope/OUT (HTTP + path/sql/tcp Criterion reproducible report chain); row **still frozen**; ≠ load-farm GA ≠ actor Criterion ≠ D-01 `[x]`. Pin tests: `qs1_d01_load_farm_ga_charter_first_slice`.

** D-01 load-farm GA claim: ** charter in-scope completion surface claim + soft-close; `| D-01 |` **`[x]`**; this-window inventory `d01_load_farm_ga_not_claimed=false`; ≠ actor Criterion ≠ absolute SLA ≠ default PR CI hard gate. Pin tests: `qs1_d01_load_farm_ga_claim_first_slice`.

** D-05/D-06 productAfter ADR: ** written freeze Actor shared pool + DiscardOldest productcompletion surface (Same as ADR); row **still frozen**; ≠ Actor pool GA ≠ DiscardOldest product GA ≠ D-05·06 `[x]`. Pin tests: `qs1_d05_d06_execute_adr_first_slice`.

** D-05/D-06 construction: ** pin existing Actor shared pool MVP + DiscardOldest MVP thin entry; row **still frozen**; ≠ Actor pool GA ≠ DiscardOldest product GA ≠ D-05·06 `[x]`. Pin tests: `qs1_d05_d06_construction_first_slice`.

** D-05/D-06 deepen: ** Pin Actor shared pool race-census + DiscardOldest policydeepen SSOT; row **still frozen**; ≠ Actor pool GA ≠ DiscardOldest product GA ≠ soft-close ≠ D-05·06 `[x]`. Pin tests: `qs1_d05_d06_deepen_first_slice`.

** D-05/D-06 narrow soft-close: ** deepen SSOT completion surfaceflip row; `| D-05 |` / `| D-06 |` **`[x]`**; Wave D **6/6**; **still** umbrella Actor pool / DiscardOldest product GA **not claimed**. Pin tests: `qs1_d05_d06_soft_close_first_slice`.

** D-02 productAfter ADR: ** D-02 **product execution ADR met**; row **still frozen**; ≠ TSAN GA ≠ loom GA ≠ known-issues listlanded ≠ D-02 `[x]`. Pin tests: `qs1_d02_execute_adr_first_slice`.

** D-02 known-issues list: ** existing M3-C04 corpus pinned aslist table body; row **still frozen**; ≠ list complete ≠ TSAN GA ≠ loom GA ≠ D-02 `[x]`. Pin tests: `qs1_d02_known_issues_first_slice`.

** D-02 known-issues listdeepen: ** written freezestill-open residual classification (TSAN CI / loom / kill·network chaos / wasm atomics / Actor shared-pool races); row **still frozen**; ≠ list complete ≠ TSAN CI ≠ TSAN GA ≠ D-02 `[x]`. Pin tests: `qs1_d02_known_issues_deepen_first_slice`.

** D-02 TSAN CI sampling: ** nightly/opt-in `tsan-sample.yml` (`-Zsanitizer=thread` + narrow race_stress mutex smoke); row **still frozen**; ≠ TSAN GA ≠ default PR CI hard gate ≠ list complete ≠ D-02 `[x]`. Pin tests: `qs1_d02_tsan_ci_sampling_first_slice`.

** D-02 known-issues list complete: ** residual gap disposition→`deferred` (no `open`); row **still frozen**; ≠ TSAN GA ≠ harden complete ≠ D-02 `[x]`. Pin tests: `qs1_d02_known_issues_complete_first_slice`.

** D-02 TSAN sample harden: ** `tsan-sample.yml` filter `m3_c04_race_stress` prefix + fail-closed; `tsan_ci=sampling_hardened`; row **still frozen**; ≠ TSAN GA ≠ default PR CI hard gate ≠ D-02 `[x]`. Pin tests: `qs1_d02_tsan_harden_first_slice`.

** D-02 TSAN GA charter: ** written freeze future honest claim TSAN GA in-scope/OUT (`tsan-sample.yml` + race_stress fail-closed + list complete + `sampling_hardened`); row **still frozen**; ≠ TSAN GA ≠ default PR CI hard gate ≠ D-02 `[x]`. Pin tests: `qs1_d02_tsan_ga_charter_first_slice`.

** D-02 TSAN GA claim: ** charter in-scope completion surface claim + soft-close; `| D-02 |` **`[x]`**; this-window inventory `d02_tsan_ga_not_claimed=false`; ≠ default PR CI hard gate ≠ loom GA ≠ full stress. Pin tests: `qs1_d02_tsan_ga_claim_first_slice`.

** D-03 productAfter ADR: ** D-03 **product execution ADR met**; row **still frozen**; ≠ OTel GA ≠ pprof GA ≠ end-to-end examplelanded ≠ D-03 `[x]`. Pin tests: `qs1_d03_execute_adr_first_slice`.

** D-03 end-to-end example: ** `examples/observability_demo.sy` chains Log+Trace+Metrics+Profiler (fingerprint `obs-demo-v1`); `cargo run -p syx-cli -- run./examples/observability_demo.sy`; in-process thin observability; row **still frozen**; ≠ OTel GA ≠ pprof export ≠ D-03 `[x]`. Pin tests: `qs1_d03_e2e_example_first_slice`.

** D-03 narrow soft-close: ** `| D-03 |` **`[x]`** = ADR-1604 in-process e2e completion surface; ≠ OTel GA ≠ pprof product GA. Pin tests: `qs1_d03_soft_close_first_slice`.

** E-02 negative-case full set charter: ** written freeze loopback fail-closed negative-case full set in-scope/OUT; still `e02_negative_tests_not_claimed`; ≠ negative-case full setClaimed ≠ streaming GA ≠ product gRPC GA. Pin tests: `qs1_e02_negative_tests_complete_charter_first_slice`.

** E-02 negative-case full set claim: ** charter in-scope completion surface claim + soft-close; this-window inventory `e02_negative_tests_not_claimed=false`; ≠ streaming GA ≠ product gRPC GA. Pin tests: `qs1_e02_negative_tests_complete_claim_first_slice`.

** E-02 streaming GA charter: ** written freeze loopback streaming completion surface in-scope/OUT (W1158–W1166); still `e02_streaming_not_claimed`; ≠ streaming GA Claimed ≠ product gRPC GA. Pin tests: `qs1_e02_streaming_ga_charter_first_slice`.

** E-02 streaming GA claim: ** charter in-scope completion surface claim + soft-close; this-window inventory `e02_streaming_not_claimed=false`; ≠ product gRPC GA ≠ interop full set. Pin tests: `qs1_e02_streaming_ga_claim_first_slice`.

** E-02 product gRPC·interop GA charter: ** written freeze future honest claim product gRPC GA + interop GA in-scope/OUT (Same as ADR dual umbrella); still `e02_grpc_not_claimed` ∧ `e02_interop_not_claimed`; ≠ product gRPC GA ≠ interop full set ≠ flip both umbrellas. Pin tests: `qs1_e02_product_grpc_interop_ga_charter_first_slice`.

** E-02 product gRPC·interop GA claim: ** charter in-scope completion surface claim + soft-close; this-window inventory `e02_grpc_not_claimed=false` ∧ `e02_interop_not_claimed=false`; ≠ tonic full set ≠ promote `net.grpc` ≠ rewrite W1273 historical dualumbrella. Pin tests: `qs1_e02_product_grpc_interop_ga_claim_first_slice`.

** E-03 productAfter ADR: ** written freeze"more DB"completion surface in-scope/OUT; row **still frozen**; ≠ more DB product GA ≠ SQLite deepen ≠ E-03 `[x]`. Pin tests: `qs1_e03_execute_adr_first_slice`.

** E-03 scaffold: ** pin existing `sql.sy` / `:memory:` / Sql.* / `data.sqlite` partial as thin entry; row **still frozen**; ≠ SQLite deepen ≠ more DB product GA ≠ E-03 `[x]`. Pin tests: `qs1_e03_scaffold_first_slice`.
** E-03 SQLite deepen: ** writtenresidual honest census SSOT (file DSN vs `:memory:`; `data.sqlite` partial; Orm OUT; second driver OUT); row **still frozen**; ≠ Sql construction deepen ≠ more DB product GA ≠ E-03 `[x]`. Pin tests: `qs1_e03_sqlite_deepen_first_slice`.
** E-03 soft-close: ** narrow deepen completion surface flip `| E-03 |` → `[x]`; **still** `e03_more_db_ga_not_claimed`; ≠ more DB product GA ≠ Sql construction deepen. Pin tests: `qs1_e03_soft_close_first_slice`.
** E-04 execute ADR: ** written freeze SynUI real renderer/event/a11y completion surface in-scope/OUT; row **still frozen**; ≠ SynUI product GA ≠ v13 construction ≠ E-04 `[x]`. Pin tests: `qs1_e04_execute_adr_first_slice`.
** E-04 scaffold: ** pin existing `packages/ui` / `__syx_synui_*` as thin entry; row **still frozen**; ≠ SynUI deepen ≠ SynUI product GA ≠ E-04 `[x]`. Pin tests: `qs1_e04_scaffold_first_slice`.
** E-04 SynUI deepen: ** writtenresidual honest census SSOT (Renderer structural-only; Event opt-in; a11y OUT; pixel/v13/Farm OUT); row **still frozen**; ≠ SynUI runtime construction ≠ SynUI product GA ≠ E-04 `[x]`. Pin tests: `qs1_e04_synui_deepen_first_slice`.
** E-04 soft-close: ** narrow deepen completion surface flip `| E-04 |` → `[x]`; **still** `e04_synui_ga_not_claimed`; ≠ SynUI product GA ≠ SynUI runtime construction. Pin tests: `qs1_e04_soft_close_first_slice`.
** E-05 execute ADR: ** written freeze Farm/Dist v20+ completion surface in-scope/OUT; row **still frozen**; ≠ Farm/Dist product GA ≠ v20 construction ≠ E-05 `[x]`. Pin tests: `qs1_e05_execute_adr_first_slice`.
** E-05 scaffold: ** pin existing `build.farm` / farm-lease v19 / dist peer-announce-hold as thin entry; row **still frozen**; ≠ Farm/Dist deepen ≠ Farm/Dist product GA ≠ E-05 `[x]`. Pin tests: `qs1_e05_scaffold_first_slice`.
** E-05 Farm/Dist deepen: ** writtenresidual honest census SSOT (lease/billing/TCP; peer-announce/sticky/multihost; `build.farm` frozen; HSM/multi-region OUT); row **still frozen**; ≠ Farm/Dist runtime construction ≠ Farm/Dist product GA ≠ E-05 `[x]`. Pin tests: `qs1_e05_farm_dist_deepen_first_slice`.
** E-05 soft-close: ** narrow deepen completion surface flip `| E-05 |` → `[x]`; **still** `e05_farm_dist_ga_not_claimed`; ≠ Farm/Dist product GA ≠ Farm/Dist runtime construction. Pin tests: `qs1_e05_soft_close_first_slice`.
** Wave soft-complete + F-08 park honesty: ** written Wave soft-complete @ **52/53** (A–E full; F **10/11**); reiterate ADR-1375 default-no; `| F-08 |` **still `[ ]`**; ≠ F-08 `[x]` ≠ RT ABI fully cleared ≠ Wave **53/53**. Pin tests: `qs1_wave_soft_complete_park_honesty_first_slice`.
** Post-wave product-debt parked-charter: ** separatemaintenance/product debt census SSOT (F-08; D/E/P umbrella; SharedCell/wasm-gc; HostFn status residual); `| F-08 |` **still `[ ]`**; ≠ debt cleared ≠ F-08 `[x]` ≠ RT ABI fully cleared ≠ Wave **53/53**. Pin tests: `qs1_post_wave_product_debt_parked_charter_first_slice`.
** HostFn status residual honesty census: ** deepen Process×14 `partial`/`wasm=unsupported` SSOT; **still** `hostfn_status_batch_all_implemented=false`; ≠ promote `implemented` ≠ full HostFn status ≠ HostFn+1 ≠ F-08 `[x]`. Pin tests: `qs1_hostfn_status_residual_honesty_census_first_slice`.
** SharedCell / wasm-gc residual honesty census: ** deepen Arc still present ∧ wasm-gc unmet ∧ guest SYX GC not claimed SSOT; ≠ Arc clear ≠ wasm-gc complete ≠ F-08 `[x]` ≠ Wave **53/53**. Pin tests: `qs1_sharedcell_wasm_gc_residual_honesty_census_first_slice`.
** D/E/P umbrella residual honesty census: ** deepenstill park product-GA umbrella SSOT (P-star: P-ABI-v2 / P-FAER / P-SURF; D/E sample); **still** `*_ga_not_claimed` / `p_surf_mat_not_claimed`; ≠ flip not_claimed ≠ product GA ≠ F-08 `[x]` ≠ Wave **53/53**. Pin tests: `qs1_dep_umbrella_residual_honesty_census_first_slice`.
** Post-wave residual honesty census-chain closeout: ** written pin W1287–W1291 census stack aligned ∧ still park ∧ defaultno longer census deepen; ≠ debt cleared ≠ product GA ≠ F-08 `[x]` ≠ Wave **53/53**. Pin tests: `qs1_post_wave_residual_honesty_census_chain_closeout_first_slice`.

**Plugin ABI v2 / faer side path (parked charter):** Plan B written to roadmap; P-SURF / P-FAER / P-ABI-v2 milestone pins documented in [`plan/topics/plugin-abi-v2-linalg.md`](../plan/topics/plugin-abi-v2-linalg.md). `P-FAER` / `P-ABI-v2` / `P-SURF` still `[ ]`; ≠ faer product GA ≠ Mat ≠ ABI v2 complete ≠ handle GA ≠ HostFn+1 ≠ F-08. See [`plan/topics/plugin-abi-v2-linalg.md`](../plan/topics/plugin-abi-v2-linalg.md).

**M3-E02 leaf thin closeout: ** thin+deepen evidence chaincloseout; roadmap `M3-E02` `[x]`; ≠ five domains GA ≠ E03 ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e02_closeout`.

**M3-E03 HostFn capability gatethin first pin:** opt-in `--ir-capability-gate` / `SYX_IR_CAPABILITY_GATE`; fingerprint `hostfn-capability-gate-v0:compile-unsupported`; table `status=unsupported` → `S026`; roadmap `M3-E03` `[~]`; ≠ defaultglobalgate ≠ backend column GA ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e03_capability_gate`.

**M3-E03 leaf thin closeout: ** thin evidence chaincloseout; roadmap `M3-E03` `[x]`; ≠ defaultglobalgate ≠ EC-M3-3 ≠ M3 exit. Pin tests: `m3_e03_closeout`.

**EC-M3-3 Capability-sandbox evidence chainthin first pin:** E01–E03 three leaves closeout + fingerprintevidence chain (`permission-policy-v0:default-deny-intent` / `permission-grants-v0:allow-net-fs-toml` / `hostfn-capability-gate-v0:compile-unsupported`); roadmap footnote thin delivered; total gatestill `[ ]`; ≠ five domains GA ≠ defaultglobalgate ≠ EC-M3-1 ≠ M3 exit. Pin tests: `ec_m3_3_capability_sandbox`.

**EC-M3-3 leaf thin closeout: ** thin evidence chain → `EC-M3-3` total gate `[x]` (honest narrowing); ≠ five domains GA ≠ defaultglobalgate ≠ EC-M3-1 ≠ M3 exit. Pin tests: `ec_m3_3_closeout`.

**EC-M3-1 high-concurrency stress reproducibleevidence chainthin first pin:** HTTP Criterion (`ec_m2_4_http_criterion`)+ A03/A04/C04 closeout + C04 samplecorpus; diagnostic surface inherits `liveTaskCount` / `profiler-v1-cpu`; roadmap footnote thin delivered; total gatestill `[ ]`; ≠ load-farm GA ≠ absolute SLA hard gate ≠ TSAN ≠ M3 exit. Pin tests: `ec_m3_1_stress_repro`.

**EC-M3-1 leaf thin closeout: ** thin evidence chain → `EC-M3-1` total gate `[x]` (honest narrowing); four EC aligned; ≠ load-farm GA ≠ absolute SLA hard gate ≠ TSAN ≠ M3 exit. Pin tests: `ec_m3_1_closeout`.

**M3 exit: ** four EC total gate signedthin exit under signed total gates; open debt (load-farm / SLA / TSAN / five domains GA / async-let / sampling·OTel etc.)**does not block exit**; **stop**default M3 EC closeout train. Pin tests: `m3_exit_closeout`.

**M4-L01 first-order where about bind thin first pin:** genericFunctions `where T: Equatable`; `Int` compile-pass; not Equatable UserType → `S010`; fingerprint `generic-where-v0:first-order-s010`; roadmap `M4-L01` `[~]`; ≠ EC-M4-L ≠ L02 ≠ `where T == U` ≠ M4 exit. Pin tests: `m4_l01_first_order_where`.

**M4-L01 leaf thin closeout: ** thin evidence chain → `M4-L01` `[x]` (honest narrowing); ≠ EC-M4-L ≠ L02 ≠ `where T == U` ≠ M4 exit. Pin tests: `m4_l01_closeout`.

**M4-L02 associatedType typealias thin first pin:** protocol `associatedtype Item: Equatable` + conforming side `typealias` see witness; `Item = Int` compile-pass; `Item = Box` (not Equatable)→ `S012`; fingerprint `assoc-type-v0:typealias-s012`; roadmap `M4-L02` `[~]` (**assoc-only**); ≠ conditional conformance GA ≠ EC-M4-L ≠ cross-moduleBlockmatrix ≠ M4 exit. Pin tests: `m4_l02_assoc_type`.

**M4-L02 leaf thin closeout: ** thin evidence chain → `M4-L02` `[x]` (honest **assoc-only**); ≠ conditional conformance GA ≠ EC-M4-L ≠ cross-moduleBlockmatrix ≠ M4 exit. Pin tests: `m4_l02_closeout`.

**conditional conformancethin first pin:** `extension Array: Marker where Element: Equatable` → generic `where T: Marker` compile-pass (`[Int]`)/ compile-fail (not Equatable element → **S010**); fingerprint `conditional-conform-v0:array-element-s010`; L02 still `[x]` (deepen, ≠re-open `[~]`); ≠ conditional GA ≠ EC-M4-L ≠ L03 ≠ M4 exit. Pin tests: `m4_conditional_conformance`.

**conditional conformanceleaf thin closeout: ** thin evidence chain meta closeout; L02 still `[x]`; ≠ conditional GA ≠ EC-M4-L ≠ L03 ≠ M4 exit. Pin tests: `m4_conditional_closeout`.

**M4-L03 existential `any` bindthin first pin:** `let x: any Equatable = 1` compile-pass; not Equatable `Box` → **S014**; fingerprint `existential-any-v0:binding-s014`; roadmap `M4-L03` `[~]`; ≠ full existential opening ≠ `-> some/any` returnsgate ≠ EC-M4-L ≠ M4 exit. Pin tests: `m4_l03_existential_any`.

**M4-L03 leaf thin closeout: ** thin evidence chain → `M4-L03` `[x]` (honest **binding-only**); ≠ full existential opening ≠ EC-M4-L ≠ M4 exit. Pin tests: `m4_l03_closeout`.

**EC-M4-L protocol-layer evidencethin first pin:** L02 assoc/conditional + L03 binding evidence chain meta; fingerprint `assoc-type-v0:typealias-s012` / `conditional-conform-v0:array-element-s010` / `existential-any-v0:binding-s014`; **EC-M4-L total gatestillopen**; ≠ full opening GA ≠ conditional GA ≠ M4 exit. Pin tests: `ec_m4_l_protocol_layer`.

**EC-M4-L leaf thin closeout: ** thin evidence chain → `EC-M4-L` **total gate signed**; ≠ full opening GA ≠ conditional GA ≠ remaining EC-M4-* ≠ M4 exit. Pin tests: `ec_m4_l_closeout`.

**M4-B01 static exclusive borrow thin first pin:** single `&mut` compile-pass; Same asregion dual `&mut` → **S011**; fingerprint `borrow-exclusive-v0:double-mut-s011`; roadmap `M4-B01` `[~]`; ≠ NLL ≠ noncopyable ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b01_exclusive_borrow`.

**M4-B01 leaf thin closeout: ** thin evidence chain → `M4-B01` `[x]`; ≠ NLL ≠ noncopyable ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b01_closeout`.

**M4-B02 move-consume ownershipthin first pin:** `String` by-value consume once compile-pass; use againuses → **S011** (Use of moved value); fingerprint `move-consume-v0:string-s011`; roadmap `M4-B02` `[~]` (**move-consume-only**); ≠ `~Copyable` ≠ assignment-move ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b02_move_consume`.

**M4-B02 leaf thin closeout: ** thin evidence chain → `M4-B02` `[x]` (honest **move-consume-only**); ≠ `~Copyable` ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b02_closeout`.

**M4-B03 language-complete nottargetthin first pin:** M4 leaf explicitlyWith language-complete borrow as **nottarget**; fingerprint `borrow-lang-complete-v0:nongoal`; roadmap `M4-B03` `[~]` (**nongoal**); complete/NLL deepen → EC-M4-B; ≠ NLL GA ≠ language-complete GA ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b03_nongoal`.

**M4-B03 leaf thin closeout: ** thin evidence chain → `M4-B03` `[x]` (honest **nongoal**); ≠ NLL GA ≠ language-complete GA ≠ EC-M4-B ≠ M4 exit. Pin tests: `m4_b03_closeout`.

**EC-M4-B Named move-consume thin first pin:** `class`/Named by-value consume once compile-pass; use againuses → **S011**; fingerprint `move-consume-v0:named-s011`; roadmap EC-M4-B thin (total gatestill `[ ]`; **named-move-layer**); ≠ NLL ≠ `~Copyable` GA ≠ EC total gate ≠ M4 exit. Pin tests: `ec_m4_b_named_move`.

**EC-M4-B leaf thin closeout: ** thin evidence chain → `EC-M4-B` **total gate signed** (honest **named-move-layer**; String + Named S011); ≠ NLL GA ≠ `~Copyable` GA ≠ remaining EC-M4-* ≠ M4 exit. Pin tests: `ec_m4_b_closeout`.

**EC-M4-M macro sandbox thin first pin:** external macrodefault reexec sandbox (not in-process); `macro_sandbox_mode` ∈ `{sandbox-reexec-only, seccomp}`; WASM external expansion → **E004**; fingerprint `macro-sandbox-v0:reexec-default`; roadmap EC-M4-M thin (total gatestill `[ ]`; **sandbox-reexec**); ≠ source-map GA ≠ incremental-binary GA ≠ EC total gate ≠ M4 exit. Pin tests: `ec_m4_m_macro_sandbox`.

**EC-M4-M leaf thin closeout: ** thin evidence chain → `EC-M4-M` **total gate signed** (honest **sandbox-reexec-layer**); ≠ source-map GA ≠ incremental-binary GA ≠ remaining EC-M4-P/X ≠ M4 exit. Pin tests: `ec_m4_m_closeout`.

**EC-M4-P Signatureverifythin first pin:** ed25519 checksum `syx1:ed25519:` verify pass; tamper checksum / bad signature → fail; install **P011**; fingerprint `pkg-sig-v0:ed25519-checksum`; roadmap EC-M4-P thin (total gatestill `[ ]`; **signature-verify**); ≠ trust-root GA ≠ solver GA ≠ frozen GA ≠ EC total gate ≠ M4 exit. Pin tests: `ec_m4_p_signature_verify`.

**EC-M4-P leaf thin closeout: ** thin evidence chain → `EC-M4-P` **total gate signed** (honest **signature-verify-layer**); ≠ trust-root GA ≠ solver GA ≠ frozen GA ≠ EC-M4-X ≠ M4 exit. Pin tests: `ec_m4_p_closeout`.

**EC-M4-X product gate thin first pin:** this milestone **product-gate defer** (does not single-select WS/gRPC/UI); fingerprint `ec-m4-x-v0:product-gate`; roadmap EC-M4-X thin (total gate still `[ ]`); ≠ real WS ≠ real gRPC ≠ SynUI unfreeze ≠ EC total gate ≠ M4 exit. Pin tests: `ec_m4_x_product_gate`.

**EC-M4-X leaf thin closeout: ** thin evidence chain → `EC-M4-X` **total gate signed** (honest **product-gate-layer**); ≠ real WS GA ≠ real gRPC GA ≠ SynUI GA ≠ M4-X01–X04 full checkmark ≠ M4 exit. Pin tests: `ec_m4_x_closeout`.

**M4 exit: ** five EC total gate signedthin exit under signed total gates; open debt (real WS/gRPC/SynUI; trust-root / solver / frozen; NLL; source-map etc.)**does not block exit**; **stop**default M4 EC closeout train; **Truth M0–M4 train closeout**. Pin tests: `m4_exit_closeout`.

**M3-A04 Channel close first pin:** `Channel.close` hard disconnect sender; drain off, recv **Nil**; send again → **C010**; ≠ backpressure/fairness full checkmark ≠ EC-M3 ≠ A04 full checkmark. Pin tests: `m3_a04_channel_close`.

**M3-A04 Channel backpressure deepen: ** bounded `create(N)`; fill off, `trySend` → **false**; `count`/`capacity` observable; ≠ fairness full checkmark ≠ EC-M3 ≠ A04 full checkmark. Pin tests: `m3_a04_backpressure`.

**M3-A04 Channel fairness productpin: ** multi-armWhen ready when `__syx_channel_select` round-robin (`select-fair-v1:round-robin`); seed=0 → 20 times 10/10; ≠cross-thread RR GA ≠ EC-M3. Pin tests: `m3_a04_fairness`.

**M3-A04 leaf thin closeout: ** close + backpressure + fairness three-axisevidence chaincloseout; roadmap `M3-A04` `[x]`; ≠cross-thread RR GA ≠ EC-M3 ≠ A05 ≠ M3 exit. Pin tests: `m3_a04_closeout`.

**M3-A05 pool layering docs first pin:** structured concurrency as primary application API; `Concurrency` pool/Actor capacity knobs are **advanced runtime configuration**; ≠ remove pool API ≠ EC-M3 ≠ A05 full checkmark (this window). Pin tests: `m3_a05_pool_layering`.

**M3-A05 leaf thin closeout: ** layeringDocumentevidence chaincloseout; roadmap `M3-A05` `[x]`; ≠remove pool API API ≠ EC-M3 ≠ ThreadPoolExecutor full set ≠ B01 ≠ M3 exit. Pin tests: `m3_a05_closeout`.

**EC-M2-1-Z: ** A–F sixslicedeliver (file+Path / HTTP client / SQLite / HTTP server / Path deep / five-slice closeout)evidence chaincloseout signed; ≠ full Path GA. Pin tests: `ec_m2_1_z_closeout`.

**EC-M2-1 total gate: ** official stdlib deliverable A–F + Z evidence chainsigned (honest narrowing); `io.path` still partial; Http no typed throws full migration; ≠ M2 exit. Pin tests: `ec_m2_1_total_closeout`.

**Q-S5: ** four kinds handle multi-handle stress (File/Sql/Tcp/Process; N=4)live_count back to baselinesigned; ≠ FileStream·Watcher·Pipe session ≠ Listener·TLS ≠ EC-M2-3 total gate. Pin tests: `q_s5_closeout`.

**EC-M2-3-F: ** FileStream / FileWatcher / ChildPipe session `*_live_count` first pinsigned (onlyTesting/diagnostic layer); ≠ HTTP body stream ≠ Listener·TLS ≠ EC-M2-3 total gate. Pin tests: `ec_m2_3_session_leak_closeout`.

**EC-M2-3-G: ** HttpBodyStream session `http_body_stream_live_count` first pinsigned (onlyTesting/diagnostic layer); ≠ Listener·TLS ≠ EC-M2-3 total gate. Pin tests: `ec_m2_3_http_body_stream_leak_closeout`.

**EC-M2-3-H: ** TcpListener bind/close session `tcp_listener_live_count` first pinsigned (onlyTesting/diagnostic layer); ≠ accept ≠ TLS ≠ EC-M2-3 total gate (at that time). Pin tests: `ec_m2_3_tcp_listener_leak_closeout`.

**EC-M2-3 total gate: ** existingproduct handle/session leak pins alignedsigned (honest narrowing); `net.tls.server` still unsupported; no accept GA; ≠ M2 exit. Pin tests: `ec_m2_3_total_closeout`.

**FileHandle RAII / `withOpen` (`M2-S3-02b`): ** protocol surface `Reader` / `Writer` / `Seekable` / `Closeable`; `FileHandle.openResource` → `FileResource` instance (read/write/seek/close + `deinit` quiet-close); `FileHandle.withOpen(path:mode:body:)` for `body(handle: Int)` **exitmust close** (resource contractabout; body successmust returns `>= 0`).≠language keywords `with`/`defer`; ≠ `EC-M2-3` total gate. Pin tests: `m2_s3_02b_filehandle_raii_with`.

**`readAllBytes` (`M2-S3-04`): ** `FileHandle.readAllBytes` as full buffer `read_to_end` (public surface no longer has `readMmap`).**≠ OS mmap**; streaming chunkBlocksee `FileStream`.`io.file.read_all_bytes` implemented. Pin tests: `m2_s3_04_mmap_rename`.≠ `memmap2`; ≠ `EC-M2-4-Z`.

**FileStream / FileWatcher (`M2-S3-03`): ** `FileStream.open` → `readChunk(handle, maxBytes)` (emptyarray=EOF)→ `close`; `FileWatcher.open` → `poll` → `close`.**With off,further operations must fail**.default `File.read` stillfull buffer; `File.watch` still is one-shot (≠ lifecycle).`io.file.stream` implemented; `io.file.watch` partial (mock default). Pin tests: `m2_s3_03_file_stream_watcher`.≠ recursive/async notify GA; ≠ `EC-M2-4-Z`.

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

| Method | Description |
|------|------|
| `Directory.create(path)` | Create directory |
| `Directory.list(path)` | List directory → `Array` |
| `Directory.exists(path)` | Directory exists → `Bool` |

### DateTime / Clock / Instant

`import DateTime` off,clocksemanticsseparate (roadmap `M2-S5-01`): 

| API | semantics |
|-----|------|
| `Clock.nowMs()` / prelude `Time.nowMs()` | **wall clock** UNIX epoch milliseconds (may shift with NTP) |
| `Instant.now()` / `Instant.elapsedMs(start:)` | **monotonic**in-process tick (opaque Int; ****not** epoch ms) |
| `Duration.ms/seconds/minutes/hours` | Int-ms span constructor (compat helper) |
| `DateTime.parse` / `parseIso8601` / `format` / `now` | narrow Gregorian + RFC3339 UTC/fixed-offset core **implemented** (M0-B01 + /222) |
| `DateTime.formatWithTimezone(ms, pattern, timezone)` | IANA wall-clock formatting (`M2-S5-02`, chrono-tz); `pattern` at least `"iso"` |
| `DateTime.timezoneOffset(ms, timezone)` | offset from UTC at instant (**minutes, east positive**; e.g. NY winter=-300 / summer=-240); unknown zone **U001** |

```syx
func main() {
  let winter = DateTime.parseIso8601(text: "2024-01-15T12:00:00.000Z").ms
  let summer = DateTime.parseIso8601(text: "2024-07-15T12:00:00.000Z").ms
  print(try DateTime.timezoneOffset(ms: winter, timezone: "America/New_York")) // -300
  print(try DateTime.timezoneOffset(ms: summer, timezone: "America/New_York")) // -240
}
```

For timing use `Instant`; For calendar/wall clock use `Clock`/`DateTime`. Pin tests: `m2_s5_01_instant_duration`, `m2_s5_02_iana_tzdb_dst` (IANA/DST), `m2_s5_03_iso_rfc3339_roundtrip`, `m2_s5_04_retire_unsupported_preview`.≠ leap second / ≠ timezone full GA / ≠ llvm DateTime=`real` / ≠ M2 exit.

### HTTP

`import Net` after import `Http` (not `HTTP`): 

| Method | Description |
|------|------|
| `Http.get(url)` | GET → `HTTPResponse` (transport failure may `try`/`catch`; no typed throws migration surface) |
| `Http.post(url, body)` | POST → `HTTPResponse`; `body` string or dict body |
| `Http.fetch(url, options)` | Generic request → `HTTPResponse` (options includes `method`, `timeout_ms` etc.) |
| `Http.request(url, options)` | Same as `fetch` → `HTTPResponse` |
| `resp.status()` / `resp.text()` / `resp.ok()` | status / body / 2xx |
| `Url.parse(text)` | → `Url`（`href`/`host`/`port`/`path`/`query`） |
| `Http.serve(addr, port, handler)` | Basic HTTP server (blocking; `Http.shutdown()` graceful shutdown; tests may use `SYX_HTTP_SERVE_MAX_REQUESTS`) |
| `Http.shutdown` | stop accept (hyper+tokio watch cancel); after current request `serve` returns |
| `Http.response(status, body)` | Build handler response → `HTTPResponse` (`body` prefer String) |
| `HttpBodyStream.open/readChunk/close` | client response body chunked read session (; default get/fetch still buffered) |

**HTTP Named types (`M2-S4-01`): ** facade returns `HTTPResponse` / `Url` ( and Type surface `HttpRequest`/`HttpHeaders`/`HttpBody`), ****not** opaque `Object`.runtime may still be Object bag + MethodId accessors. Pin tests: `m2_s4_01_http_types`.streaming read see.≠ `EC-M2-4-Z`.

**TLS client real path (`M2-S4-05`): ** HTTPS uses reqwest+rustls **default cert verify**; CI/diagnosticMay useEnvironment variables `SYX_HTTP_EXTRA_ROOT_PEM` (PEM filepath)**append** trust anchor (≠ skip-verify / ≠ `danger_accept_invalid_certs`). Pin tests: `m2_s4_05_client_tls` (loopback HTTPS positive case).≠ skip-verify product API; ≠ server TLS GA; ≠ `EC-M2-4-Z`.

**HTTP body stream (`M2-S4-04`): ** `HttpBodyStream.open` → `readChunk(handle, maxBytes)` (empty string=EOF)→ `close`; session + `Read` (≠ `response.bytes`).default `Http.get`/`fetch` **stillfull buffer**. Pin tests: `m2_s4_04_http_body_stream`.≠ server streaming; ≠ wire chunked TE product API; ≠ File S3-03; ≠ JSONStream; ≠ `EC-M2-4-Z`.

**HTTP server graceful shutdown (`M2-S4-03b`): ** `Http.shutdown` → watch cancel (stop accept); after current request `Http.serve` returns. Pin tests: `m2_s4_03b_server_graceful_shutdown` (**not uses** `SYX_HTTP_SERVE_MAX_REQUESTS` impersonate graceful).`MAX_REQUESTS` still is separateTesting hook.≠ server TLS GA; ≠ `EC-M2-4-Z`.

**HTTP server backend: default** `Http.serve` / `Http.shutdown` / `upgradeUpstream` byte tunnelbuilt on **hyper + tokio** on ([`crates/syx-vm/src/http.rs`](../crates/syx-vm/src/http.rs); plain HTTP via hyper HTTP/1; Upgrade header probe off,usesside pathbyte tunnel; handler via isolated thread to preserve `ProgramCtx` / native MIR).**not done** (in-process frame termination / serve computed Accept).≠ axum/ntex/warp; ≠ `net.http.server` product GA; ≠ HostFn+1; ≠ flip row.

**in-process HTTPS: ** diagnostic env `SYX_HTTP_SERVE_TLS_CERT` + `SYX_HTTP_SERVE_TLS_KEY` (PEM path)when both set, after accept **rustls terminates**then use hyper HTTP/1; clientMay use `SYX_HTTP_EXTRA_ROOT_PEM` Appendtrust anchor (≠ skip-verify).Pin `http_serve_https_loopback_get_with_extra_root`.**Claim:** HTTPS GET met; **Not claimed:** WSS product GA (HMR-over-HTTPS see); **Not claimed:** sgate productionremove sidecar; **Not claimed:** `net.http.server` / `net.tls.server` product GA.

**overlapping connections: ** after accept `JoinSet` spawn (multi-thread runtime); plain / Upgrade may overlap handling; Pin `http_serve_overlap_slow_fast_plain_get` (fast request must complete while slow request holds lock).**Claim:** multiple connections mayoverlap accept/handlemet; **Not claimed:** high concurrency SLA / connection pool / backpressure / fulltuning.

**overlap×HMR cross: ** slow plain GET while holding lock, same port HMR Upgrade (101 + Accept relay + masked text-frame)must complete; Pin `http_serve_overlap_slow_plain_and_hmr_upgrade`.**Claim:** combined crossmet; **Not claimed:** fullconcurrency SLA.

**Upgrade-over-TLS / HMR-over-HTTPS: ** diagnostic PEM when both set, rustls terminates off, Upgrade useswith plain Same assemantics `upgradeUpstream` byte tunnel (includes head off, leftover; async `copy_bidirectional`); same port Vite-like path: 101 + Accept relay + masked text-frame.Pin `http_serve_https_hmr_path_same_port_roundtrip`.**Claim:** Upgrade-over-TLS / same port HMR-over-HTTPS (automated pin)met; **Not claimed:** WSS product GA; **Not claimed:** sgate productionremove sidecar; **Not claimed:** browser Playwright HMR GA.

**in-process frame termination / serve computed Accept: ** handler returnsexplicit `upgradeTerminate: true` (**≠** missing `upgradeUpstream`——missing field still 502) when, serve from client `Sec-WebSocket-Key` **computed locally** `Sec-WebSocket-Accept` → 101 → decode one masked text frame and echo.with `upgradeUpstream` **relay**pathsplit: HMR still uses upstream; terminate does not dial.Pin `http_serve_upgrade_terminate_accept_and_text_echo`.**Claim:** frame termination / computed locally Accept met; **Not claimed:** WSS / WebSocket product GA; **Not claimed:** rewrite HMR relay.

**upgradeTerminate close-path: ** text echo then off,read client close (opcode 8)→ reply unmasked close; Pin `http_serve_upgrade_terminate_accept_text_and_close`.**Claim:** close-path met; **Not claimed:** full RFC6455 (binary/fragment/extension/ping); **Not claimed:** WSS product GA.

**upgradeTerminate binary echo: ** text echo then off,read one more frameclient binary (opcode 2)→ reply unmasked binary (`0x82`)→ again close exchange; Pin `http_serve_upgrade_terminate_accept_text_binary_and_close`.**Claim:** binary echo met; **Not claimed:** full RFC6455 (fragment/extension/ping); **Not claimed:** WSS product GA.

**upgradeTerminate ping→pong: ** text→binary then off,read client ping (opcode 9)→ reply unmasked pong (`0x8A` + Same as payload)→ again close exchange; Pin `http_serve_upgrade_terminate_accept_text_binary_ping_and_close`.**Claim:** ping→pong met; **Not claimed:** full RFC6455 (fragment/extension/full keepalive); **Not claimed:** WSS product GA.

**upgradeTerminate fragmented text: ** clienttwo framesfragmented text (`FIN=0`+opcode 1 → `FIN=1`+continuation)→ server-sidereassemble tosingleframe unmasked text; then off, binary→ping/pong→close; Pin `http_serve_upgrade_terminate_accept_fragmented_text_binary_ping_and_close`.**Claim:** fragmented text met; **Not claimed:** binary fragmented / multi-segment continuation / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate binary fragmented: ** fragmented text then off,two more framesfragmented binary (`FIN=0`+opcode 2 → `FIN=1`+continuation)→ reassemble tosingleframe unmasked binary (`0x82`)→ ping/pong→close; Pin `http_serve_upgrade_terminate_accept_fragmented_text_fragmented_binary_ping_and_close`.**Claim:** binary fragmentedmet; **Not claimed:** multi-segment continuation / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate multi-segment continuation N=3: ** frag-text/binary all three-frame (`FIN=0` data + `FIN=0` mid cont + `FIN=1` final cont)reassemble tosingleframe; Pin `http_serve_upgrade_terminate_accept_multi_continuation_n3_ping_and_close`.**Claim:** multi-segment continuation (N=3)met; **Not claimed:** arbitrary segment count / control-during-fragment / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate control-during-fragment: ** N=3 text at frag1 with mid continuation may insert between client ping → server-sideimmediate pong, **does not abort**assemble → singleframe `0x81` echo; binary still N=3 no mid-control; Pin `http_serve_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close`.**Claim:** control-during-fragment (N=3 text / frag1→mid insert ping)met; **Not claimed:** arbitrary segment count / binary mid-control / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate binary mid-control: ** N=3 binary at frag1 with mid continuation may insert between client ping → immediate pong → reassemble tosingleframe `0x82`; Pin `http_serve_upgrade_terminate_accept_control_during_fragment_n3_binary_ping_and_close`.**Claim:** control-during-fragment (N=3 binary / frag1→mid insert ping)met; **Not claimed:** arbitrary segment count / mid↔final control / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate arbitrary segment count N=4 text: ** fragmented text four frames (`FIN=0` data + 2× `FIN=0` cont + `FIN=1` cont)reassemble tosingleframe `0x81`; binary lifecyclestill N=3; Pin `http_serve_upgrade_terminate_accept_multi_continuation_n4_text_ping_and_close`.**Claim:** arbitrary segment count (N=4 text)met; **Not claimed:** unbounded N / binary arbitrary-N / mid↔final control / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate binary arbitrary segment count N=4: ** fragmented binary four framesreassemble tosingleframe `0x82`; text Same aslifecycleretained N=4; Pin `http_serve_upgrade_terminate_accept_multi_continuation_n4_text_and_binary_ping_and_close`.**Claim:** binary arbitrary segment count (N=4)met; **Not claimed:** unbounded N / mid↔final control / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate mid↔final control: ** N=4 text at cont1 with cont2 may insert between client ping → immediate pong → reassemble tosingleframe `0x81`; Pin `http_serve_upgrade_terminate_accept_mid_final_control_n4_text_ping_and_close`.**Claim:** mid↔final control (N=4 text / cont insert between ping)met; **Not claimed:** unbounded N / binary mid↔final / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate binary mid↔final control: ** N=4 binary at cont1 with cont2 may insert between client ping → immediate pong → reassemble tosingleframe `0x82`; Pin `http_serve_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close`.**Claim:** binary mid↔final control (N=4 binary / cont insert between ping)met; **Not claimed:** unbounded N / extension / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate unbounded N (N=5 text): ** fragmented text five frames (`FIN=0` data + 3× `FIN=0` cont + `FIN=1` cont)reassemble tosingleframe `0x81`; Pin `http_serve_upgrade_terminate_accept_multi_continuation_n5_text_ping_and_close`.**Claim:** unbounded N (N=5 text)met; **Not claimed:** unboundedmatrixfull set / N=5 binary / extension / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate unbounded N (N=5 binary): ** fragmented binary five framesreassemble tosingleframe `0x82`; text Same aslifecycleretained N=5; Pin `http_serve_upgrade_terminate_accept_multi_continuation_n5_text_and_binary_ping_and_close`.**Claim:** unbounded N (N=5 binary)met; **Not claimed:** unboundedmatrixfull set / extension / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate close status+reason echo: ** client close with 2 byte status + UTF-8 reason when, server-side unmasked echoSame as payload; empty close still reply empty close; Pin `http_serve_upgrade_terminate_accept_text_close_status_reason_echo`.**Claim:** close status+reason echo met; **Not claimed:** RSV / extension / close negative-case full set / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate RSV fail-closed: ** clientframe RSV1/2/3 anynot 0 when, server-side fail-closed (not echo / disconnect); RSV=0 positive-pathpathunchanged; Pin `http_serve_upgrade_terminate_accept_rsv_text_fail_closed`.**Claim:** RSV fail-closed met; **Not claimed:** extension negotiation / protocol close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate close body len=1 fail-closed: ** client close payload exactly 1 when bytes, server-side fail-closed (no write-back close / disconnect); empty close with len≥2 echo unchanged; Pin `http_serve_upgrade_terminate_accept_close_len1_fail_closed`.**Claim:** close body len=1 fail-closed met; **Not claimed:** unmasked client / extension / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate unmasked-client fail-closed: ** clientframe MASK=0 when, server-side fail-closed (not echo / disconnect); masked positive-pathpathunchanged; Pin `http_serve_upgrade_terminate_accept_unmasked_text_fail_closed`.**Claim:** unmasked-client fail-closed met; **Not claimed:** fragmented control / extension / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate fragmented-control (FIN=0 ping)fail-closed: ** client ping FIN=0 when, server-side fail-closed (not pong / disconnect); FIN=1 ping positive-pathpathunchanged; Pin `http_serve_upgrade_terminate_accept_fin0_ping_fail_closed`.**Claim:** fragmented-control (FIN=0 ping)fail-closed met; **Not claimed:** extension / reserved opcode / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate reserved-control-opcode (0xB–0xF)fail-closed: ** clientframe opcode ∈ 0xB..=0xF when, server-side fail-closed (not echo / disconnect); valid opcode positive-pathpathunchanged; Pin `http_serve_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed`.**Claim:** reserved-control-opcode (0xB–0xF)fail-closed met; **Not claimed:** extension / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate data-reserved-opcode (0x3–0x7)fail-closed: ** clientframe opcode ∈ 0x3..=0x7 when, server-side fail-closed (not echo / disconnect); 0xB..=0xF withvalid opcode behaviorunchanged; Pin `http_serve_upgrade_terminate_accept_reserved_opcode_0x3_fail_closed`.**Claim:** data-reserved-opcode (0x3–0x7)fail-closed met; **Not claimed:** extension / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate oversized-control-payload (>125)fail-closed: ** control frame (opcode 0x8/0x9/0xA) and len7>125 when, server-side fail-closed (not pong / disconnect; reject before extended-length parsing); ≤125 control with data frameunchanged; Pin `http_serve_upgrade_terminate_accept_oversized_ping_fail_closed`.**Claim:** oversized-control-payload (>125)fail-closed met; **Not claimed:** extension / close(1002) productsemantics / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate illegal-close-status fail-closed: ** client close payload len≥2 and status ∈ 0..=999 / 1005 / 1006 / 1015 when, server-side fail-closed (not echo close / disconnect); empty close withvalid status ( e.g. 1000+reason)unchanged; Pin `http_serve_upgrade_terminate_accept_close_status_1005_fail_closed`.**Claim:** illegal-close-status fail-closed met; **Not claimed:** extension / close(1002) product response frame / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate close-reason-utf8 fail-closed: ** client close payload `len>2` and reason (`payload[2..]`)notvalid UTF-8 when, server-side fail-closed (not echo close / disconnect); `len∈{0,2}` withvalid UTF-8 reason ( e.g. 1000+"bye")unchanged; Pin `http_serve_upgrade_terminate_accept_close_reason_non_utf8_fail_closed`.**Claim:** close-reason-utf8 fail-closed met; **Not claimed:** UTF-8 text / extension / close(1002) product response frame / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate UTF-8 text + close-1004 fail-closed three-cluster: ** (1)FIN=1 text payloadnotvalid UTF-8 → fail-closed (Pin `http_serve_upgrade_terminate_accept_text_non_utf8_fail_closed`); (2)fragmented text assemble off,notvalid UTF-8 → fail-closed (Pin `…_fragmented_text_non_utf8_fail_closed`); (3)close status 1004 → fail-closed (Pin `…_close_status_1004_fail_closed`).valid UTF-8 text / binary / empty close / 1000+reason unchanged.**Claim:** UTF-8 text fail-closed first cluster + close-1004 fail-closed met; **Not claimed:** UTF-8 text product GA / extension / close(1002) product response frame / full codec; **Not claimed:** WSS product GA.

**upgradeTerminate framing negative-case fail-closed four-cluster: ** (1)FIN=0 close → fail-closed (Pin `http_serve_upgrade_terminate_accept_fin0_close_fail_closed`); (2)when no message in progress continuation (opcode 0x0)→ fail-closed (Pin `…_unexpected_continuation_fail_closed`); (3)unsolicited client pong as first frame (opcode 0xA)→ fail-closed (Pin `…_unsolicited_pong_fail_closed`); (4)`read_client_ws_frame` for close/ping/pong centralized `require_control_fin`; mid-fragment FIN=0 close → fail-closed (Pin `…_fin0_close_during_fragment_fail_closed`).valid FIN=1 close / ping→pong / fragmented data unchanged.**Claim:** framing negative case fail-closed first cluster met; **Not claimed:** extension / close(1002) product response frame / full RFC6455; **Not claimed:** WSS product GA.

**upgradeTerminate close(1002) product response framefour-cluster: ** protocol error pathGenerated by abort-only promoted to **send unmasked close first(status=1002) then close connection** (`write_protocol_close_1002`): (1)RSV≠0 → close(1002) (Pin `…_accept_rsv_text_close_1002`); (2)reserved opcode (0xB)→ close(1002) (Pin `…_accept_reserved_opcode_0xb_close_1002`); (3)FIN=0 control (ping)→ close(1002) (Pin `…_accept_fin0_ping_close_1002`); (4)unexpected continuation → close(1002) (Pin `…_accept_unexpected_continuation_close_1002`).existing framing/RSV/reserved/UTF-8 etc. abort pin updated expectations 1002 response frame.valid lifecycle / empty close / 1000+reason unchanged.**Claim:** close(1002) product response framefirst cluster (RSV / reserved opcode / FIN=0 control / unexpected continuation)met; **Not claimed:** close codefull-set GA / extension / full RFC6455; **Not claimed:** WSS product GA.

**upgradeTerminate close(1002) product response framesecond cluster: ** unified exit hooked; add product dedicated pin matrix: (1)unmasked client → close(1002) (Pin `…_accept_unmasked_text_close_1002`); (2)oversized ping (len7=126)→ close(1002) (Pin `…_accept_oversized_ping_close_1002`); (3)close body len=1 → close(1002) (Pin `…_accept_close_len1_close_1002`); (4)illegal close status 1005 → close(1002) (Pin `…_accept_close_status_1005_close_1002`).existing `*_fail_closed` retained.**Claim:** close(1002) product response framesecond cluster (unmasked / oversized ping / close len=1 / status 1005)met; **Not claimed:** close codefull-set GA / UTF-8 text product GA / extension; **Not claimed:** WSS product GA.

**upgradeTerminate-over-TLS product cluster: ** HTTPS on `upgradeTerminate` with plain Same assemantics: (1)computed locally Accept + text echo (Pin `http_serve_https_upgrade_terminate_accept_and_text_echo`); (2)RSV≠0 → close(1002) (Pin `…_accept_rsv_text_close_1002`); (3)text→binary→ping/pong→close (Pin `…_accept_text_binary_ping_and_close`); (4)HMR-over-HTTPS Accept stillrelayupstream, ≠ serve computed locally (Pin `http_serve_https_hmr_path_accept_still_relayed_not_serve_computed`).**Claim:** upgradeTerminate-over-TLS / WSS terminate product cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate close(1002) product response framethird cluster: ** unified `InvalidData`→close(1002) exit hooked; add product dedicated pin matrix: (1)text not UTF-8 → close(1002) (Pin `…_accept_text_non_utf8_close_1002`); (2)close reason not UTF-8 → close(1002) (Pin `…_accept_close_reason_non_utf8_close_1002`); (3)unsolicited pong → close(1002) (Pin `…_accept_unsolicited_pong_close_1002`); (4)illegal close status 1004 → close(1002) (Pin `…_accept_close_status_1004_close_1002`).existing `*_fail_closed` retained.**Claim:** close(1002) product response framethird cluster (text not UTF-8 / close-reason not UTF-8 / unsolicited pong / status 1004)met; **Not claimed:** close codefull-set GA / UTF-8 text product GA / extension / WSS product GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate close(1002) product response framefourth cluster: ** unified exit hooked; add product dedicated pin matrix: (1)fragmented text not UTF-8 → close(1002) (Pin `…_accept_fragmented_text_non_utf8_close_1002`); (2)FIN=0 close → close(1002) (Pin `…_accept_fin0_close_close_1002`); (3)FIN=0 close mid-fragment → close(1002) (Pin `…_accept_fin0_close_during_fragment_close_1002`); (4)reserved data opcode 0x3 → close(1002) (Pin `…_accept_reserved_opcode_0x3_close_1002`).existing `*_fail_closed` retained; plain dedicated pin matrix 16/16.**Claim:** close(1002) product response framefourth cluster (frag UTF-8 / FIN=0 close / FIN=0 close-during-fragment / reserved 0x3)met; **Not claimed:** close codefull-set GA / UTF-8 text product GA / extension / WSS product GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS close(1002) product reply frame first cluster:** on HTTPS reuse the same `InvalidData`→close(1002) exit; add TLS product-specific pins: (1) reserved control opcode 0xB → close(1002) (pin `http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_close_1002`); (2) FIN=0 ping → close(1002) (pin `…_accept_fin0_ping_close_1002`); (3) unexpected continuation → close(1002) (pin `…_accept_unexpected_continuation_close_1002`); (4) unmasked client text → close(1002) (pin `…_accept_unmasked_text_close_1002`). **Claim:** upgradeTerminate-over-TLS close(1002) product reply frame first cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** TLS close(1002) matrix full set / remove sidecar / Playwright.

**upgradeTerminate-over-TLS close(1002) product response framesecond cluster: ** unified exit hooked; add TLS product dedicated pin: (1)oversized ping (len7=126)→ close(1002) (Pin `http_serve_https_upgrade_terminate_accept_oversized_ping_close_1002`); (2)close body len=1 → close(1002) (Pin `…_accept_close_len1_close_1002`); (3)illegal close status 1005 → close(1002) (Pin `…_accept_close_status_1005_close_1002`); (4)text not UTF-8 → close(1002) (Pin `…_accept_text_non_utf8_close_1002`).**Claim:** upgradeTerminate-over-TLS close(1002) product response framesecond cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** TLS close(1002) matrixfull set / UTF-8 text product GA / remove sidecar / Playwright.

**upgradeTerminate-over-TLS close(1002) product response framethird cluster: ** unified exit hooked; add TLS product dedicated pin: (1)close reason not UTF-8 → close(1002) (Pin `http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_close_1002`); (2)unsolicited pong → close(1002) (Pin `…_accept_unsolicited_pong_close_1002`); (3)illegal close status 1004 → close(1002) (Pin `…_accept_close_status_1004_close_1002`); (4)fragmented text not UTF-8 → close(1002) (Pin `…_accept_fragmented_text_non_utf8_close_1002`).**Claim:** upgradeTerminate-over-TLS close(1002) product response framethird cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** TLS close(1002) matrixfull set / UTF-8 text product GA / remove sidecar / Playwright.

**upgradeTerminate-over-TLS close(1002) product response framefourth cluster: ** unified exit hooked; add TLS product dedicated pin: (1)FIN=0 close → close(1002) (Pin `http_serve_https_upgrade_terminate_accept_fin0_close_close_1002`); (2)FIN=0 close mid-fragment → close(1002) (Pin `…_accept_fin0_close_during_fragment_close_1002`); (3)reserved data opcode 0x3 → close(1002) (Pin `…_accept_reserved_opcode_0x3_close_1002`).**Claim:** upgradeTerminate-over-TLS close(1002) product response framefourth cluster met; TLS close(1002) dedicated pin matrixrelative plain **16/16** met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** close codefull-set GA / UTF-8 text product GA / remove sidecar / Playwright.

**upgradeTerminate illegal close-status deepenfirst cluster: ** unified exit hooked; add product dedicated pin: (1)plain close status 1006 → close(1002) (Pin `http_serve_upgrade_terminate_accept_close_status_1006_close_1002`); (2)plain close status 1015 → close(1002) (Pin `…_accept_close_status_1015_close_1002`); (3)plain close status 999 (0–999 representative)→ close(1002) (Pin `…_accept_close_status_999_close_1002`); (4)TLS close status 1006 → close(1002) (Pin `http_serve_https_upgrade_terminate_accept_close_status_1006_close_1002`).**Claim:** illegal close-status deepenfirst cluster met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate illegal close-status deepensecond cluster: ** unified exit hooked; add product dedicated pin: (1)TLS close status 1015 → close(1002) (Pin `http_serve_https_upgrade_terminate_accept_close_status_1015_close_1002`); (2)TLS close status 999 → close(1002) (Pin `…_accept_close_status_999_close_1002`); (3)plain close status 0 → close(1002) (Pin `http_serve_upgrade_terminate_accept_close_status_0_close_1002`); (4)plain close status 500 → close(1002) (Pin `…_accept_close_status_500_close_1002`).**Claim:** illegal close-status deepensecond cluster met; TLS illegal five codes (1004/1005/1006/1015/999)aligned with plain; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS positive-path codec deepenfirst cluster: ** no new runtime; HTTPS terminate aligned with plain positive-pathfour pins: (1)close status+reason echo (Pin `http_serve_https_upgrade_terminate_accept_text_close_status_reason_echo`); (2)dual frag text+binary + ping + close (Pin `…_accept_fragmented_text_fragmented_binary_ping_and_close`); (3)multi-continuation N=3 + ping + close (Pin `…_accept_multi_continuation_n3_ping_and_close`); (4)mid↔final control N=4 text + ping + close (Pin `…_accept_mid_final_control_n4_text_ping_and_close`).**Claim:** upgradeTerminate-over-TLS positive-path codec deepenfirst cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA / TLS positive-path codec full set / close codefull-set GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS positive-path codec deepensecond cluster: ** no new runtime; HTTPS terminate aligned with plain: (1)control-during-fragment N=3 text (Pin `http_serve_https_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close`); (2)control-during-fragment N=3 binary (Pin `…_accept_control_during_fragment_n3_binary_ping_and_close`); (3)multi-continuation N=4 text (Pin `…_accept_multi_continuation_n4_text_ping_and_close`); (4)multi-continuation N=4 text+binary (Pin `…_accept_multi_continuation_n4_text_and_binary_ping_and_close`).**Claim:** upgradeTerminate-over-TLS positive-path codec deepensecond cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA / TLS positive-path codec full set / close codefull-set GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS positive-path codec deepenthird cluster: ** no new runtime; HTTPS terminate aligned with plain /: (1)mid↔final control N=4 binary (Pin `http_serve_https_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close`); (2)multi-continuation N=5 text (Pin `…_accept_multi_continuation_n5_text_ping_and_close`); (3)multi-continuation N=5 text+binary (Pin `…_accept_multi_continuation_n5_text_and_binary_ping_and_close`); (4)fragmented text + FIN=1 binary (Pin `…_accept_fragmented_text_binary_ping_and_close`).**Claim:** upgradeTerminate-over-TLS positive-path codec deepenthird cluster met; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA / TLS positive-path codec full set / N5 unboundedfull-set GA / close codefull-set GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS positive-path twin closeout + illegal-status TLS complete: ** no new runtime; HTTPS terminate aligned with plain /: (1)text + close (Pin `http_serve_https_upgrade_terminate_accept_text_and_close`); (2)text + binary + close (Pin `…_accept_text_binary_and_close`); (3)illegal close status 0 → close(1002) (Pin `…_accept_close_status_0_close_1002`); (4)illegal close status 500 → close(1002) (Pin `…_accept_close_status_500_close_1002`).**Claim:** upgradeTerminate-over-TLS positive-path twin closeout + illegal-status TLS 0/500 complete met / plain↔HTTPS positive-path twin name tablealigned; **Not claimed:** WSS / WebSocket / `net.tls.server` product GA / TLS positive-path codec full set / close codefull-set GA; **Not claimed:** remove sidecar / Playwright.

**upgradeTerminate-over-TLS fail_closed name tablefour-linkfirst cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); aligned with plain `*_fail_closed` + scenario): (1)reserved opcode 0xB (Pin `http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed`); (2)FIN=0 ping (Pin `…_accept_fin0_ping_fail_closed`); (3)unexpected continuation (Pin `…_accept_unexpected_continuation_fail_closed`); (4)unmasked text (Pin `…_accept_unmasked_text_fail_closed`).**Claim:** fail_closed name tablefour-linkfirst cluster met / TLS fail_closed **4/16**; **Not claimed:** fail_closed TLS name tablefull set / WSS / WebSocket / `net.tls.server` product GA / close codefull-set GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**todo 8 overall soft-close / item closeout:** no new runtime / no HostFn+1; written freeze of item 8 completion surface SSOT: hyper+tokio default stack; same-port `upgradeUpstream`/HMR (including over-TLS); overlapping accept + overlap×HMR; HTTPS + sgate native; `upgradeTerminate` plain/over-TLS + close(1002) main matrix + illegal near-boundary soft-close; WSS marketing claim boundary soft-close. **OUT (item-level permanent):** Playwright HMR GA; full concurrency SLA; `ws_routes` remove websocat; RFC6455 extension; close code full set; `net.tls.server` HostFn GA; "full WSS/WebSocket product GA". Pin `todo8_overall_soft_close_first_slice`. **Claim:** todo 8 soft-close / item closeout met; **item 8 no further slices**; **Not claimed:** any OUT item "delivered"; **Not claimed:** new progress.

**todo 4 Object soft-close / item closeout: ** no new runtime / no HostFn+1; written freeze: defaultliterals → `Dictionary`; explicit `: Object` heterogeneous bag; Method surface `count`/`isEmpty`/`valueForKey`/`put`/`containsKey`/`keys`/`values`/`remove` (subscript read/writeSame assemantics).**OUT (item-level permanent): ** `.merge`; real `HashMap<Value,Value>`; Object generic deepening; `object_bag_ga`.Pin `todo4_object_soft_close_first_slice`.**Claim:** Object soft-close / item closeout met; ** item 4 no further slices**; **Not claimed:** OUT any OUT item"delivered"; **Not claimed:** Wave new progress.

**WSS marketing product soft-close (todo 8):** no new runtime / no HostFn+1; written freeze of product floor SSOT: (1) same-port `upgradeUpstream` relay (includes HMR-over-TLS automated pin); (2) `upgradeTerminate` plain + over-TLS (positive representative cluster + close(1002) main matrix + illegal near-boundary soft-close); (3) sgate native HTTPS can carry the paths above. **OUT:** RFC6455 extension; close code full-set GA; `net.tls.server` / `__syx_tls_serve` HostFn GA; real browser Playwright HMR; full concurrency SLA; `ws_routes` websocat remove sidecar; "full WSS/WebSocket product GA" marketing slogan. Pin `wss_marketing_product_soft_close_first_slice` (indexes existing representative tests). **Claim:** WSS marketing product soft-close met / this sub-axis no longer opens slices chasing "WSS GA"; **Not claimed:** full WSS/WebSocket product GA / Playwright / SLA / delete websocat / item 8 overall closeout.

**sgate TLS native (todo 8):** no new HostFn; `mode: native` → `tls_native.sy` sets `Env.set(SYX_HTTP_SERVE_TLS_CERT/KEY)` from configured PEM, `main.sy` runs `Http.serve` on `tls.port` (in-process rustls, same semantics as stdlib HTTPS pins); config may use `SYX_SGATE_CONFIG`; retains `tls.sy` `__syx_tls_serve`/`if false` honest placeholder. Pin `sgate_tls_native_first_slice` / test.sh native. **Claim:** sgate TLS native met / production HTTPS without socat; **Not claimed:** delete sidecar code / `ws_routes` remove sidecar / Playwright / `net.tls.server` HostFn GA / item 8 overall closeout (WSS marketing claim boundary see soft-close).

**upgradeTerminate illegal close-status near-boundary sampling axis soft-close (todo 8):** no new runtime; close soft debt + TLS edges aligned to 9/990; near-boundary `N/999−N` further deepen **OUT**: (1) TLS 8/991 fail_closed (pin `http_serve_https_upgrade_terminate_accept_close_status_8_fail_closed` / `…_991_fail_closed`); (2) plain 9/990 fail_closed (pin `http_serve_upgrade_terminate_accept_close_status_9_fail_closed` / `…_990_fail_closed`); (3) TLS 9/990 close(1002) aligned (pin `…_9_close_1002` / `…_990_close_1002`). **Claim:** illegal near-boundary sampling axis soft-close met / soft **4/4** TLS↔plain edges aligned to 9/990; this sub-axis no further slices; **Not claimed:** close code full-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / item 8 overall closeout / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 8/991 closeout + plain 9/990 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 8 (Pin `http_serve_https_upgrade_terminate_accept_close_status_8_close_1002`); (2)TLS close status 991 (Pin `…_991_close_1002`); (3)plain close status 9 (Pin `http_serve_upgrade_terminate_accept_close_status_9_close_1002`); (4)plain close status 990 (Pin `…_990_close_1002`).**Claim:** TLS 8/991 closeout + plain 9/990 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 7 (Pin `http_serve_https_upgrade_terminate_accept_close_status_7_fail_closed`); (2)TLS close status 992 (Pin `…_992_fail_closed`); (3)plain close status 8 (Pin `http_serve_upgrade_terminate_accept_close_status_8_fail_closed`); (4)plain close status 991 (Pin `…_991_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 7/992 closeout + plain 8/991 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 7 (Pin `http_serve_https_upgrade_terminate_accept_close_status_7_close_1002`); (2)TLS close status 992 (Pin `…_992_close_1002`); (3)plain close status 8 (Pin `http_serve_upgrade_terminate_accept_close_status_8_close_1002`); (4)plain close status 991 (Pin `…_991_close_1002`).**Claim:** TLS 7/992 closeout + plain 8/991 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 6 (Pin `http_serve_https_upgrade_terminate_accept_close_status_6_fail_closed`); (2)TLS close status 993 (Pin `…_993_fail_closed`); (3)plain close status 7 (Pin `http_serve_upgrade_terminate_accept_close_status_7_fail_closed`); (4)plain close status 992 (Pin `…_992_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 6/993 closeout + plain 7/992 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 6 (Pin `http_serve_https_upgrade_terminate_accept_close_status_6_close_1002`); (2)TLS close status 993 (Pin `…_993_close_1002`); (3)plain close status 7 (Pin `http_serve_upgrade_terminate_accept_close_status_7_close_1002`); (4)plain close status 992 (Pin `…_992_close_1002`).**Claim:** TLS 6/993 closeout + plain 7/992 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 5 (Pin `http_serve_https_upgrade_terminate_accept_close_status_5_fail_closed`); (2)TLS close status 994 (Pin `…_994_fail_closed`); (3)plain close status 6 (Pin `http_serve_upgrade_terminate_accept_close_status_6_fail_closed`); (4)plain close status 993 (Pin `…_993_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 5/994 closeout + plain 6/993 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 5 (Pin `http_serve_https_upgrade_terminate_accept_close_status_5_close_1002`); (2)TLS close status 994 (Pin `…_994_close_1002`); (3)plain close status 6 (Pin `http_serve_upgrade_terminate_accept_close_status_6_close_1002`); (4)plain close status 993 (Pin `…_993_close_1002`).**Claim:** TLS 5/994 closeout + plain 6/993 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 4 (Pin `http_serve_https_upgrade_terminate_accept_close_status_4_fail_closed`); (2)TLS close status 995 (Pin `…_995_fail_closed`); (3)plain close status 5 (Pin `http_serve_upgrade_terminate_accept_close_status_5_fail_closed`); (4)plain close status 994 (Pin `…_994_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 4/995 closeout + plain 5/994 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 4 (Pin `http_serve_https_upgrade_terminate_accept_close_status_4_close_1002`); (2)TLS close status 995 (Pin `…_995_close_1002`); (3)plain close status 5 (Pin `http_serve_upgrade_terminate_accept_close_status_5_close_1002`); (4)plain close status 994 (Pin `…_994_close_1002`).**Claim:** TLS 4/995 closeout + plain 5/994 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 3 (Pin `http_serve_https_upgrade_terminate_accept_close_status_3_fail_closed`); (2)TLS close status 996 (Pin `…_996_fail_closed`); (3)plain close status 4 (Pin `http_serve_upgrade_terminate_accept_close_status_4_fail_closed`); (4)plain close status 995 (Pin `…_995_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 3/996 closeout + plain 4/995 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 3 (Pin `http_serve_https_upgrade_terminate_accept_close_status_3_close_1002`); (2)TLS close status 996 (Pin `…_996_close_1002`); (3)plain close status 4 (Pin `http_serve_upgrade_terminate_accept_close_status_4_close_1002`); (4)plain close status 995 (Pin `…_995_close_1002`).**Claim:** TLS 3/996 closeout + plain 4/995 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 2 (Pin `http_serve_https_upgrade_terminate_accept_close_status_2_fail_closed`); (2)TLS close status 997 (Pin `…_997_fail_closed`); (3)plain close status 3 (Pin `http_serve_upgrade_terminate_accept_close_status_3_fail_closed`); (4)plain close status 996 (Pin `…_996_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 2/997 closeout + plain 3/996 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 2 (Pin `http_serve_https_upgrade_terminate_accept_close_status_2_close_1002`); (2)TLS close status 997 (Pin `…_997_close_1002`); (3)plain close status 3 (Pin `http_serve_upgrade_terminate_accept_close_status_3_close_1002`); (4)plain close status 996 (Pin `…_996_close_1002`).**Claim:** TLS 2/997 closeout + plain 3/996 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)TLS close status 100 (Pin `http_serve_https_upgrade_terminate_accept_close_status_100_fail_closed`); (2)TLS close status 998 (Pin `…_998_fail_closed`); (3)plain close status 2 (Pin `http_serve_upgrade_terminate_accept_close_status_2_fail_closed`); (4)plain close status 997 (Pin `…_997_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status TLS 100/998 closeout + plain 2/997 deepen: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); product `*_close_1002`: (1)TLS close status 100 (Pin `http_serve_https_upgrade_terminate_accept_close_status_100_close_1002`); (2)TLS close status 998 (Pin `…_998_close_1002`); (3)plain close status 2 (Pin `http_serve_upgrade_terminate_accept_close_status_2_close_1002`); (4)plain close status 997 (Pin `…_997_close_1002`).**Claim:** TLS 100/998 closeout + plain 2/997 deepen met; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin (aligned): ** no new runtime; name-table soft-twin (behavior still close(1002); aligned `*_close_1002`): (1)plain close status 1 (Pin `http_serve_upgrade_terminate_accept_close_status_1_fail_closed`); (2)plain close status 100 (Pin `…_100_fail_closed`); (3)plain close status 998 (Pin `…_998_fail_closed`); (4)TLS close status 1 (Pin `http_serve_https_upgrade_terminate_accept_close_status_1_fail_closed`).**Claim:** illegal-status fail_closed soft-twin (aligned)met / four codes soft-twin **4/4**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status deepenthird cluster: ** no new runtime (`reject_illegal_close_status` already rejects `0..=999`); productsamplingrepresentative point → close(1002): (1)plain close status 1 (Pin `http_serve_upgrade_terminate_accept_close_status_1_close_1002`); (2)plain close status 100 (Pin `…_100_close_1002`); (3)plain close status 998 (Pin `…_998_close_1002`); (4)TLS close status 1 (Pin `http_serve_https_upgrade_terminate_accept_close_status_1_close_1002`).**Claim:** illegal close-status deepenthird cluster met / sampling 1/100/998 (+ TLS 1); **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin closeout: ** no new runtime; name-table soft-twin (behavior still close(1002); aligned TLS `*_close_1002`): (1)TLS close status 0 (Pin `http_serve_https_upgrade_terminate_accept_close_status_0_fail_closed`); (2)TLS close status 500 (Pin `…_500_fail_closed`).**Claim:** illegal-status fail_closed soft-twin closeout met / illegal fail_closed **10/10**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin second cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); mirror): (1)TLS close status 1015 (Pin `http_serve_https_upgrade_terminate_accept_close_status_1015_fail_closed`); (2)TLS close status 999 (Pin `…_999_fail_closed`); (3)plain close status 0 (Pin `http_serve_upgrade_terminate_accept_close_status_0_fail_closed`); (4)plain close status 500 (Pin `…_500_fail_closed`).**Claim:** illegal-status fail_closed soft-twin second cluster met / illegal fail_closed **8/10**; **Not claimed:** close codefull-set GA / illegal fail_closed full set / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate illegal close-status fail_closed soft-twin first cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); mirror): (1)plain close status 1006 (Pin `http_serve_upgrade_terminate_accept_close_status_1006_fail_closed`); (2)plain close status 1015 (Pin `…_1015_fail_closed`); (3)plain close status 999 (Pin `…_999_fail_closed`); (4)TLS close status 1006 (Pin `http_serve_https_upgrade_terminate_accept_close_status_1006_fail_closed`).**Claim:** illegal-status fail_closed soft-twin first cluster met / illegal fail_closed **4/10**; **Not claimed:** close codefull-set GA / WSS / WebSocket / `net.tls.server` product GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate-over-TLS fail_closed name tablefourth/final cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); aligned with plain `*_fail_closed` + / RSV scenario): (1)FIN=0 close (Pin `http_serve_https_upgrade_terminate_accept_fin0_close_fail_closed`); (2)FIN=0 close mid-fragment (Pin `…_accept_fin0_close_during_fragment_fail_closed`); (3)reserved opcode 0x3 (Pin `…_accept_reserved_opcode_0x3_fail_closed`); (4)RSV text (Pin `…_accept_rsv_text_fail_closed`).**Claim:** fail_closed name tablefourth/final cluster met / TLS fail_closed **16/16** (soft-twin); **Not claimed:** WSS / WebSocket / `net.tls.server` product GA / close codefull-set GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate-over-TLS fail_closed name tablethird cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); aligned with plain `*_fail_closed` + scenario): (1)close reason non-UTF-8 (Pin `http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_fail_closed`); (2)unsolicited pong (Pin `…_accept_unsolicited_pong_fail_closed`); (3)illegal status 1004 (Pin `…_accept_close_status_1004_fail_closed`); (4)fragmented text non-UTF-8 (Pin `…_accept_fragmented_text_non_utf8_fail_closed`).**Claim:** fail_closed name tablethird cluster met / TLS fail_closed **12/16**; **Not claimed:** fail_closed TLS name tablefull set / WSS / WebSocket / `net.tls.server` product GA / close codefull-set GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".

**upgradeTerminate-over-TLS fail_closed name tablesecond cluster: ** no new runtime; name-table soft-twin (behavior still close(1002); aligned with plain `*_fail_closed` + scenario): (1)oversized ping (Pin `http_serve_https_upgrade_terminate_accept_oversized_ping_fail_closed`); (2)close len=1 (Pin `…_accept_close_len1_fail_closed`); (3)illegal status 1005 (Pin `…_accept_close_status_1005_fail_closed`); (4)text non-UTF-8 (Pin `…_accept_text_non_utf8_fail_closed`).**Claim:** fail_closed name tablesecond cluster met / TLS fail_closed **8/16**; **Not claimed:** fail_closed TLS name tablefull set / WSS / WebSocket / `net.tls.server` product GA / close codefull-set GA; **Not claimed:** remove sidecar / Playwright / "new abort-only semantics".




**Same-port HMR path:** client Upgrade **only hits the serve port**; handler returns `upgradeUpstream` pointing at local Vite-like WS upstream; 101 + `Sec-WebSocket-Accept` relay + masked text-frame round-trip pinned (`http_serve_upgrade_proxy_hmr_path_same_port_roundtrip`). **Claim:** same-port HMR **product path** (automated pin) met; **Not claimed:** real browser / Playwright HMR GA; **Not claimed:** frame termination / serve-computed Accept. Product usage see [tools/sgate/README.md](../tools/sgate/README.md) (`:8080`→`:9999`).

**HTTP Upgrade → `upgradeUpstream` (in-process byte tunnel):** handler may return an Object with `upgradeUpstream` (upstream `http://host:port/path`); `Http.serve` dials upstream, forwards handshake, then bidirectional byte copy after `101`. WebSocket frames (including client masked text frames) **relay verbatim**; upstream 101 response headers (including `Sec-WebSocket-Accept`) **forward verbatim** (serve **does not** compute Accept from Key). When upstream is not 101, write back to client `status.max(400)` (e.g. 500→500, 200→400) and **do not open** tunnel. Missing `upgradeUpstream`, empty `upgradeUpstream` (treated as missing field), handler failure, dial/write request/read header failure (before tunnel) → client **502** (body includes `Bad Gateway` or field name). Pin tests: `http_serve_upgrade_proxy_roundtrip` / `http_serve_upgrade_proxy_masked_text_frame_roundtrip` / `http_serve_upgrade_proxy_non_101_maps_status_max_400` / `http_serve_upgrade_proxy_relays_sec_websocket_accept` / `http_serve_upgrade_proxy_missing_upgrade_upstream_returns_502` / `http_serve_upgrade_proxy_dial_fail_returns_502` / `http_serve_upgrade_proxy_empty_upgrade_upstream_returns_502` / `http_serve_upgrade_proxy_handler_err_returns_502` / `http_serve_upgrade_proxy_hmr_path_same_port_roundtrip`. Product usage see [tools/sgate/README.md](../tools/sgate/README.md). ≠ WSS product GA; ≠ real browser HMR GA (HMR-over-HTTPS see above; in-process termination see `upgradeTerminate`).

**HTTP timeout / TLS negative case (`M2-S4-02b`): ** `Http.fetch`/`request` `timeout_ms` real timeout → **R012**; self-signed / bad cert fail-closed, forbidden `status=0` success.`Http.get` **no** per-call timeout. Pin tests: `m2_s4_02b_client_timeout_tls`.≠ skip-verify API; ≠ server TLS GA; ≠ `EC-M2-4-Z`.

Http* HostFn availability honest write-back in capability matrix (`ec_m2_4_http_availability`; ≠ EC-M2-4 full ≠ TLS server GA).native/WASM column honest deepen pinned (`ec_m2_4_http_native_wasm_matrix`); HttpMakeResponse llvm GA first tier pinned (`ec_m2_4_http_native_wasm_ga`; `HttpMakeResponse` `llvm=real` + native AOT execution evidence); HttpResponseStatus llvm GA second tier pinned (`ec_m2_4_http_native_wasm_ga2`; `HttpResponseStatus` `llvm=real` + native AOT MakeResponse+status → `"200"`); HttpResponseText llvm GA third tier pinned (`ec_m2_4_http_native_wasm_ga3`; `HttpResponseText` `llvm=real` + native AOT MakeResponse+text → `"ok"`; remaining Http* still `llvm=partial` / `wasm=host-assisted` / product `native=partial`; ≠ full HTTP GA ≠ wasm GA ≠ TLS server GA ≠ EC-M2-4 full exit).`Http.response`×N Instant microbench in test layer (`ec_m2_4_http_microbench`; ≠ perf SLA ≠ EC-M2-4 full exit).`Http.response`×N Criterion formal benchmark first tier in `cargo bench` (`ec_m2_4_http_criterion`; ≠ absolute SLA gate ≠ EC-M2-4 full exit).

`TcpStream.connect` / `read` / `write` throw `NetworkError`. `host_tcp_handle_release` / `TcpStream.closeHandle` really drop from VM `TCP_STREAMS`; live count `tcp_stream_live_count` is test/diagnostic layer only, **not** application API. Tcp* HostFn availability honestly written back in capability matrix (`ec_m2_4_tcp_availability`; ≠ EC-M2-4 full ≠ TLS GA). native/WASM columns honestly deepened (pins `ec_m2_4_tcp_native_wasm_matrix`); TcpConnect llvm GA first tier pinned (`ec_m2_4_tcp_native_wasm_ga`; `TcpConnect` `llvm=real` + native AOT refuse → `-1` execution evidence); TcpHandleRelease llvm GA second tier pinned (`ec_m2_4_tcp_native_wasm_ga2`; `TcpHandleRelease` `llvm=real` + native AOT loopback connect+release → `"true"`); TcpWrite llvm GA third tier pinned (`ec_m2_4_tcp_native_wasm_ga3`; `TcpWrite` `llvm=real` + native AOT loopback connect+write `"hi"` → `"2"`); four-slice native/WASM GA third-tier closeout assessment signed (`ec_m2_4_four_slice_native_wasm_ga3_closeout` / roadmap `EC-M2-4-G`; ≠ full EC-M2-4 ≠ `EC-M2-4-Z`); TcpRead llvm GA fourth tier pinned (`ec_m2_4_tcp_native_wasm_ga4` / roadmap `EC-M2-4-H`; `TcpRead` `llvm=real` + native AOT loopback connect+read → `"hi"`; all four HostFns aligned); `wasm=host-assisted` / product `native=partial`; ≠ full Tcp GA ≠ TLS GA ≠ wasm GA ≠ `EC-M2-4-Z` ≠ EC-M2-4 full exit). `TcpStream.connect` refuse×N Instant microbench in test layer (`ec_m2_4_tcp_microbench`; ≠ perf SLA ≠ EC-M2-4 full exit). `TcpStream.connect` refuse×N Criterion formal benchmark first tier in `cargo bench` (`ec_m2_4_tcp_criterion`; 16MB stack; ≠ absolute SLA gate ≠ EC-M2-4 full exit).

`Process.spawn` / `Process.wait` manage child-process handle lifecycle (`wait` removes from VM `CHILD_TABLE`; failure sentinel `-1`). **ProcessBuilder / Child (`M2-S6-01`):** `ProcessBuilder.create` → `cwd` / `env` (overlay) → `spawn` → `Child.wait` / `kill` / `waitTimeout`; **operations after wait/kill must fail**; timeout `waitTimeout` kills and throws. `os.process.builder` implemented. Pin tests: `m2_s6_01_process_builder`. **ChildPipe (`M2-S6-02b`):** `Child.stdout`/`stderr`/`stdin` take → `ChildPipe.readChunk` (empty=EOF)/`write`/`close`; **read/write after close must fail**; untaken ends dropped internally on wait (prevent blocking). `os.process.stdio` implemented. Pin tests: `m2_s6_02b_child_stdio_pipe`. **Child.signal (`M2-S6-03`):** `Child.signal(handle, name)` accepts `TERM`/`INT`/`KILL` (optional `SIG` prefix, case-insensitive); **after signal handle may still `Child.wait`** (≠ `Child.kill`: force kill + remove + wait). Unix: `libc::kill`; Windows: only `KILL` (`TERM`/`INT` throw unsupported). `os.process.signal` implemented. Pin tests: `m2_s6_03_child_signal`. ≠ full POSIX signal GA; ≠ `Process.signalSelf`; ≠ async select Pipe GA; ≠ `EC-M2-4-Z`. Live count `process_child_live_count` test/diagnostic layer only (EC-M2-3 first pin), **not** application API. EC-M2-3 four-class live-count first-pin closeout assessment signed; total gate "leak=0" still open, **not** full EC-M2-3 / M2 exit. EC-M2-4 four-slice availability + Instant microbench first tier closeout assessment signed (`ec_m2_4_four_slice_closeout`); full total gate still open, **not** EC-M2-4 full / M2 exit. EC-M2-4 four-slice Criterion formal benchmark first tier closeout assessment signed (`ec_m2_4_four_slice_criterion_closeout`; ≠ absolute SLA gate); full total gate still open, **not** EC-M2-4 full / M2 exit. EC-M2-4 four-slice native/WASM matrix first tier closeout assessment signed (`ec_m2_4_four_slice_native_wasm_matrix_closeout`; ≠ native/WASM GA); full total gate still open, **not** EC-M2-4 full / M2 exit. EC-M2-4 four-slice native/WASM GA first tier closeout assessment signed (`ec_m2_4_four_slice_native_wasm_ga_closeout`; PathJoin / HttpMakeResponse / SqlOpen / TcpConnect llvm; ≠ full native/WASM GA); full total gate still open, **not** EC-M2-4 full / M2 exit. EC-M2-4 four-slice native/WASM GA second tier closeout assessment signed (`ec_m2_4_four_slice_native_wasm_ga2_closeout`; PathExtension / HttpResponseStatus / SqlClose / TcpHandleRelease llvm; ≠ full native/WASM GA); full total gate still open, **not** EC-M2-4 full / M2 exit. EC-M2-1 five-slice (file+Path / HTTP client/server / SQLite / Path deep) closeout assessment signed (`ec_m2_1_five_slice_closeout`); full total gate still open, **not** Path GA / M2 exit.

Examples: `examples/http_tool/` (client); `examples/http_server_tool/` (server; default port 18765). Also see `examples/httpclient_demo.sy` / `httpserver_demo.sy`.

### Sql（SQLite）

`import Sql` after importmulti-connection facade (`throws SqlError`): 

| Method | Description |
|------|------|
| `Sql.open(dsn:)` | Open connection (`":memory:"` or file path)→ `Int` connId |
| `Sql.execOn(connId:, sql:)` | Execute on connection |
| `Sql.queryOn(connId:, sql:)` | Query → **`Array` of `SqlRow`** (May use `.count`) |
| `Sql.close(connId:)` | Close explicitly opened connection |

**Typed Row (`M2-S8-02b`): ** Named types `SqlRow` (no opaque Object success).`SqlRow.getInt`/`getString`/`getFloat`/`getBlob`/`isNull`/`columnNames` (missing column or type error throws `SqlError`).`data.sqlite.row` implemented. Pin tests: `m2_s8_02b_typed_row`.**raw SQL hatch: ** `Sql.query`/`queryOn` **still accepts raw SQL strings** (injection risk yours); ≠ typed query builder; ≠ Orm GA.≠ `EC-M2-4-Z`.

**Connection pool (`M2-S8-01c` + `M2-S8-03b`): ** `Sql.poolCreate(dsn, maxSize)` → `poolAcquire` → uses `*On` → `poolRelease` (return to free list, **not ** close)→ `poolClose` (close all pool connections).`maxSize` when exhausted `poolAcquire` **throws immediately** (non-blocking wait).Coexists with bare `Sql.open`/`close` coexist; connections outside pool not in pool.`data.sqlite.pool` implemented. Pin tests: `m2_s8_01c_connection_pool`, `m2_s8_03b_pool_txn`.**Transaction gate: ** `poolRelease` when connection has open txn (`beginTransactionOn` without commit/rollback) when **throws** (facade code 20); same-file dual checkout concurrent `BEGIN IMMEDIATE` at most one succeeds, second begin → busy (code 15); `poolClose` hard-close pool conns and clear txn accounting, `sql_conn_live_count` does not retain pool conn.≠ blocking/timeout acquire; ≠ `EC-M2-4-Z`.

Sql* HostFn availability honest write-back in capability matrix (`ec_m2_4_sql_availability`; ≠ EC-M2-4 full ≠ Orm GA).native/WASM column honest deepen pinned (`ec_m2_4_sql_native_wasm_matrix`); SqlOpen llvm GA first tier pinned (`ec_m2_4_sql_native_wasm_ga`; `SqlOpen` `llvm=real` + native AOT `:memory:` execution evidence); SqlClose llvm GA second tier pinned (`ec_m2_4_sql_native_wasm_ga2`; `SqlClose` `llvm=real` + native AOT open+close → `"ok"`); SqlExecOn llvm GA third tier pinned (`ec_m2_4_sql_native_wasm_ga3`; `SqlExecOn` `llvm=real` + native AOT `:memory:` CREATE+INSERT → `"1"`; remaining Sql* still `llvm=partial` / `wasm=host-assisted` / product `native=partial`; ≠ full Sql GA ≠ Orm GA ≠ wasm GA ≠ EC-M2-4 full exit).`:memory:` + `Sql.execOn` INSERT×N Instant microbench in test layer (`ec_m2_4_sql_microbench`; ≠ perf SLA ≠ EC-M2-4 full exit).`:memory:` + `Sql.execOn` INSERT×N Criterion formal benchmark first tier in `cargo bench` (`ec_m2_4_sql_criterion`; 16MB stack; ≠ absolute SLA gate ≠ EC-M2-4 full exit).

Example: `examples/sql_tool/` (EC-M2-1 SQLite service slice). Live count `sql_conn_live_count` counts explicit connections only (id≥1), in test/diagnostic layer (EC-M2-3 first pin), **not** application API.

### String three views (`M2-S2-01`)

Default character model is **grapheme (grapheme cluster)** `Character`: 

| view | API | Description |
|------|-----|------|
| Grapheme | `.count` / `s[i]` / `.substring` / `for-in` | Int subscript is grapheme offset (see below) |
| Unicode scalar | `String.unicodeScalarCount(text:)` / `String.unicodeScalars(text:)` | may diverge from grapheme (e.g. ZWJ family emoji) |
| UTF-8 bytes | `String.utf8Count(text:)` / `String.utf8Bytes(text:)` | byte length and `Array<Int>` (0–255) |

`Character.isSingleScalar` judged by **scalar count == 1** (not grapheme `.count`). Pin tests: `m2_s2_01_string_three_view`.≠ normalization / XID / IANA.

### String subscript complexity (`M2-S2-02`)

`s[i]` with `.substring(start, end)` uses **Int grapheme offset** (not byte offset, nor Swift opaque `String.Index`).**single random access is O(n)** (implemented via grapheme iteration `.nth` / `.skip`, no hidden O(1) cache).for sequential scan use `for-in` (by grapheme). Pin tests: `m2_s2_02_string_index_honesty`.≠ opaque Index GA.

### String normalization / case folding（`M2-S2-03`）

| API | Description |
|-----|------|
| `String.normalizedNFC(text:)` | Unicode **NFC** |
| `String.normalizedNFD(text:)` | Unicode **NFD** |
| `String.caseFolded(text:)` | Unicode **default/full case folding** ( e.g. `ß`→`ss`) |
| `.lowercased()` / `.uppercased()` | Unicode **simple case mapping**（≠ `caseFolded`；≠ locale/Turkish） |

≠ NFKC/NFKD / IANA. Pin tests: `m2_s2_03_string_normalization`. Identifier XID see next section.

### identifier Unicode XID (`M2-S2-04`)

Identifiers follow UAX #31: **XID_Start** (or `_`)start, **XID_Continue** (or `_`)continue.Greek/CJK etc. valid; emoji etc.not XID_Start is a lexical error.identifiers **do not** NFC-normalize; ≠ UTS #39 confusables / IANA. Pin tests: `m2_s2_04_unicode_xid_identifiers`.

### String instanceMethod

| Method | Description |
|------|------|
| `.split(sep)` | Split to array |
| `.trim()` | Trim whitespace |
| `.hasPrefix(s)` | prefix check |
| `.lowercased()` / `.uppercased()` | Unicode simple case mapping（≠ `caseFolded`） |
| `.substring(start, end)` | Substring (Int grapheme offset; single times O(n), see) |
| `.replacingOccurrences(with, of)` | Replace |
| `.matches(pattern)` | Regex match |
| `.count` | grapheme length |
| `String.unicodeScalarCount` / `unicodeScalars` | Unicode scalar view (static) |
| `String.utf8Count` / `utf8Bytes` | UTF-8 byte view (static) |
| `String.normalizedNFC` / `normalizedNFD` / `caseFolded` | NFC/NFD + full case fold |

### Array instanceMethod

| Method | Description |
|------|------|
| `.count` | Element count |
| `.contains(item)` | Contains element |
| `.sorted()` | Sort ascending (new array) |
| `.reverse()` | Reverse (new array) |
| `.joined(sep)` | Join with separator to string |
| `.map(f)` / `.filter(f)` / `.reduce(f)` | ✅ closure HOF (`p9_hof_closure.sy`, three-backend parity) |

### rangewithinput

```syx
print((1..<10).contains(5))   // true
let line = readLine() // Read one line from stdin
```

### iterator (`M2-S1-01`)

`for x in iterable` Stable subset supports: 

- `Array` element-order traversal
- `String` traverse by **grapheme `Character`** (; ≠ Unicode scalar)
- Custom `class C: Iterator` (`next() -> Int?`, `nil` terminates)
- `File.readLines()` returned array

**Protocol declaration** (semantic check; stable subset return type is `Int?`): 

```syx
protocol Iterator {
    func next() -> Int?
}
```

Protocol must include `func next`, otherwise error **S006**.`Sequence.makeIterator` / full `Collection` Index **still partial**. Pin tests: `m2_s1_01_collection_sequence_iterator`.Deque → `collections.deque`.

### collection mutation (`M2-S1-02`)

`var` narrow mutation on `var` persisted via HeapSlot write-back: 

| API | semantics |
|-----|------|
| `d.remove(key)` | `[String:Int]` real delete; returns `Bool` |
| `s.insert(x)` / `s.remove(x)` | `Set<Int>`; returnswhether newly inserted / existed |
| `a[i] = v` / `__syx_array_set_at` | Array subscript and setAt write-back |

Pin tests: `m2_s1_02_collection_mutation`.≠ Dict subscript insert GA / genericfull set.Deque → `collections.deque`.

### Deque（`M2-S1-03`）

`Deque` Int narrow MVP slice (both-end push/pop + count/isEmpty; HeapSlot write-back): 

```syx
var d = Deque.new()
Deque.pushBack(items: d, item: 1)
Deque.pushFront(items: d, item: 0)
print(Deque.popFront(items: d))  // 0
print(Deque.popBack(items: d))   // 1
print(Deque.isEmpty(items: d)) // true; empty pop → nil
```

`data`/`collections.deque` implemented. Pin tests: `m2_s1_03_deque`.≠ Heap/PriorityQueue; ≠ lazy sequence; ≠ `Deque<T>` generic; ≠ for-in / makeIterator GA; ≠ `EC-M2-4-Z`.

### collection HOF feature-off (`M2-S1-04`)

`Array.map` / `filter` / `reduce` (includes `__syx_array_*` CallHost) at illegal receiver, missing Closure, or interpreter feature-off **fail-closed**: diagnostic code **`U001` / unsupported**, **must not** use empty array `[]` or silent `Nil` as success.

```syx
// Illegal: second arg not Closure → runtime failure (U001), will not print []
__syx_array_map([1, 2, 3], 0)
```

Pin tests: `m2_s1_04_native_feature_off_no_silent`.≠ String three views GA / llvm HOF GA.Deque → `collections.deque`.

---

## Error handling

### try / catch Block

```syx
try {
    File.write("/tmp/test.txt", "data")
    let content = File.read("/tmp/test.txt")
    print(content)
} catch (err) {
    print("failed")
}
```

Host functions (File, HTTP etc.)on failure throw into VM exception mechanism; caught by `catch` catch.

### expression-level try

```syx
let content = try File.read(path)
// equivalent postfix form (withform abovesame AST: UnaryOp::Try): 
let content = File.read(path)?
```

parser accepts prefix `try expr` and postfix `expr?` (same `UnaryOp::Try` node).`syx format` canonical form is **postfix** `expr?`; prefix rewritten to postfix.Statement-level `try { } catch` unchanged.failure handled as runtime error.

---

## Null safety and Optional

```syx
let a: Int? = nil
print(a ?? 99) // Nil coalescing: non-nil left else 99

let b: Int? = 7
print(b!) // Force unwrap; nil traps (R007)

let s: String? = "hello"
print(s?.hasPrefix("he")) // Optional chaining: nil short-circuits to nil

if let y = optionalValue {
    print(y)
}
```

| operator | behavior |
|--------|------|
| `??` | Nil merge |
| `!` | Force unwrap, nil → Runtime error |
| `?.` | Optional chainingstyle tuninguses |

---

## Advanced language features

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

Compiler auto-generates `_init` with `_describe`; `@Equatable`/`@Hashable`/`@Codable` as L1 proof macros (stored properties `data class`).See [books/language-guide/19-macros.md](../books/language-guide/19-macros.md).

### Encoder / Decoder protocol (`M2-S7-01`)

Language provides extensible encoding protocol surface: `Encoder` / `Decoder` / `Encodable` / `Decodable`, and `Codable: Encodable, Decodable`.Default still uses `JSON.encode` / `@Codable` dynamic JSON path. Pin tests: `m2_s7_01_encoder_decoder_protocol`.

### JSONEncoder / JSONDecoder + Fieldpath (`M2-S7-02`)

Nested subset uses `typeName` static API (not metatype `T.Type`): 

```syx
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  let u = User(name: "a", address: Address(city: "x", zip: "1"))
  let s = JSONEncoder.encode(value: u, typeName: "User")
  let back = try JSONDecoder.decode(text: s, typeName: "User")
 // missing required key throws; diagnostic includes codingPath=address (or address.zip)
}
```

Optional fields (`T?`)missing key may decode.≠ CodingKeys DSL. Pin tests: `m2_s7_02_json_encoder_decoder_field_paths`.

### JSONStream NDJSON（`M2-S7-03`）

Optional streaming MVP: session writes NDJSON lines, then chunk `feed` off, `decodeNext`: 

```syx
func main() {
  let w = JSONStream.openWriter()
  try JSONStream.write(handle: w, value: u, typeName: "User")
  let text = JSONStream.finishWriter(handle: w)
  let r = JSONStream.openReader()
  try JSONStream.feed(handle: r, chunk: text)
  let back = try JSONStream.decodeNext(handle: r, typeName: "User")
 // missing key diagnostic includes codingPath=line[0].address
  JSONStream.closeReader(handle: r)
}
```

≠ CodingKeys DSL; ≠ File real stream (→ `M2-S3-03`); ≠ HTTP streaming body (→ `M2-S4-04`); ≠ IANA (→ `M2-S5-02`). Pin tests: `m2_s7_03_json_stream_ndjson`.

### Extension

```syx
extension Int {
    func doubled() -> Int {
        return self + self
    }
}
```

Instance call `n.doubled()` desugars to `Int_doubled(n)`.

### Sealed Class with Switch

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

Supports **range case** (`0..<13`)and **variant patterns**.

### Async / Await

```syx
async func fetch() -> String {
    let data = await HTTP.get("https://example.com")
    return data
}
```

Currently **MVP**: single `await` point, blocking poll.Using inside non-async function `await` compile fails.

### Effectannotation (Effects)

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

`requires Pure` I/O disallowed in function; checked by `syx-meta` at compile time.

### macro withmetaprogramming

 Generated by `syx-macro` unified expansion.See [books/language-guide/19-macros.md](../books/language-guide/19-macros.md) · [](adr/architecture-decisions.md) · Documentfingerprint `macro-docs-v15:m23-service-v14-honesty`.

| capability | Description |
|------|------|
| Built-in `#assert` / `#for` / `#if` | compile-time expansion |
| User `#macro` | freestanding; **statement position** token subst MVP (`macro-token-v4:stmt-token-subst-mvp`)+ **declaration position**param subst MVP (`macro-token-v2:decl-param-subst-mvp`)+ **declaration positionhygiene MVP** (`macro-token-v3:decl-hygiene-mvp`)+ expression position token/ident/literal MVP (`macro-token-v1:expr-token-mvp`); hygiene v2 (`SYX_MACRO_HYGIENE=0` may disable); ≠full token-syntax macro → **134+** |
| `#stringify(expr)` | expression stringify (Phase 50) |
| `@attached` | member/peer; `body:` real AST (Phase 50) |
| `@Equatable` / `@Hashable` / `@Codable` | `data class` L1 synthesis; L2 associated-enum partial GA |
| `#externalMacro` | external macro JSON protocol (Phase 51+) |
| `@propertyWrapper` | Phase 40 GA; `--no-property-wrapper` With |
| `#compileTime` | const-eval subset |
| `reflect(T.self)` | `syx-meta` Rust-side reflection |
| Service DI v14 | `import Di` → `Di.resolve` / `Di.withRequest` / `Di.setProfile` / …; sugar `resolve(...)`; `@Qualifier`/`@Primary`/`@Profile`; **protocol witness + base-type polymorphic inject**; **`@Around` proceed + `@Before`/`@After` pointcut string binding MVP** (`macro-service-v13:before-after-pointcut-bind-mvp`); **`@Pointcut("Type.method")` method-name MVP** + **`execution(* T.m(..))` name/`*` pattern MVP** + **`@Pointcut("within(Type)")` type-name pattern MVP** (`macro-service-v14:pointcut-within-name-mvp`); **`@Configuration`/`@Bean` MVP** (****only** `@Service`/`@Component` instance method advice, ≠full AspectJ / Spring → **134+**); `__syx_service_*` internal only |

```syx
import Di

@Service
@Scope("request")
class Worker {
  init() {}
}

func handle() {
  let a = Di.resolve("Worker")
 let b = Di.resolve("Worker") // in-framesame instance
}

func main() {
  // No hand-written __syx_service_boot; user surface uses Di package
  Di.withRequest("handle")
  Di.destroy()
}
```

Example: `examples/service_di/` · `examples/stmt_token_macro/`.internal host still as `__syx_service_*` (do not call directly from app code).Token macro / AspectJ AOP / full Spring → **134+**.

### Os Run when Query (Phase 133–136 · partial GA)

`import Os` provides `Process` (args/env/exec)** **separate** runtime query MVP: `Os.cpuCount()`, `Os.memoryTotal()` / `Os.memoryAvailable()` / `Os.uptimeMs()` / `Os.loadAverage()` (throws OsError), `Os.userName()`, `Os.homeDir()`, `Os.hostname()`, `Os.diskTotal(path)`, `Os.diskAvailable(path)`.APIs that throw have no `-1`/empty-string success sentinel.≠full sysinfo / network / cgroup → **138+**.

Example: `examples/os_info/`.

### concurrency control ( · M3 surface ·)

**Primary concurrency API: ** `Task` / `TaskGroup` / `async`/`await` / `Actor` / `Channel` / `choose`, and M3 surface (`taskGroupCancel`, `setDeadlineMs`, `taskLocal*`, `actorReentrancyPolicy`, Channel close/backpressure/fairness, `Mutex`/`Semaphore`).Tutorial narrative see [books/language-guide/18-concurrency.md](../books/language-guide/18-concurrency.md).

**Runtime architecture (summary): **

| Component | Implementation |
|------|------|
| Driver entry | `SyxRuntime` — configurable thread pool + shared globals (`RwLock`) |
| VM instance | per-task isolated `VmInstance`; shared globals via runtime bridge |
| Actor | mailbox + per-actor thread; May use `Concurrency.setMaxActors` rate limit |
| standard library | `packages/core/concurrency.sy` thin wrapper |

**Advanced runtime config (deploy/ops, not default app path): ** `Concurrency.setWorkerCount` / `setCorePoolSize` / `setMaximumPoolSize` / `setKeepAliveMs` / `setTaskQueueCapacity` / `setTaskRejectPolicy` / `setMaxActors` / `setMailboxCapacity` / Actor shared pool; `blockingSleep` / `setBlockingPoolSize` (276).`Channel` default unbounded constructor; bounded via `Channel.create(n)` (returns `Channel`, see `packages/sync`).≠ Java `ThreadPoolExecutor` full compatibility; Prefer structured concurrency; avoid unbounded `detach`.

`syx.toml` optional `[concurrency] strict = true` enable C001–C003.Example: `examples/concurrency_control/`.

### Run when Host plugin (opt-in)

Phase 49+ dynamic extensions see [books/language-guide/33-runtime-extensions.md](../books/language-guide/33-runtime-extensions.md) with [architecture decisions · Host plugins](adr/architecture-decisions.md#host-plugin).

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libplugin.so
syx run app.sy
```

`syx.toml` optional `[plugins].paths`; index lock `syx-plugin-lock.json`.verify signature: `SYX_HOST_PLUGIN_VERIFY=1`.

Current ABI v1: UTF-8 strings in/out (~8 KiB output). P-SURF packed-SYX operator surface (pad/reflect, reductions, masks, element ops, geometry, Mat/solve family, faer cdylib HANDLE paths, etc.) — full milestone list in [`plan/topics/plugin-abi-v2-linalg.md`](../plan/topics/plugin-abi-v2-linalg.md). Numeric core uses plugin protocol + faer; does not add `HostFnId` or change the language. ≠ faer product GA, ≠ Mat, ≠ ABI v2 complete, ≠ handle GA, ≠ F-08, ≠ Math expansion, ≠ four-backend plugin GA.

---

## Toolchain

### Embed API（Phase 14–15）

Phase 14+ Embed API (`syx_embed_v2_*`): sandbox / timeout / memory & RSS budget / capability allowlist.See [architecture decision](adr/architecture-decisions.md).

- **v1 (frozen): ** `syx_embed_v1_compile_run` / `syx_embed_v1_free_result`
- **v2：** `syx_embed_v2_compile` / `syx_embed_v2_run` / `syx_embed_v2_free`

### Formatting — `syx format`

Format project entry file (`syx.toml` in `main` specify): 

```bash
syx format .
```

**C-06 (––1176):** `syx format --check [path]` recursively dry-runs `.sy` — asserts `format(format(x)) == format(x)`; would rewrite or F001 → non-zero exit and **no disk write**. Covers: Index/Dict, Class/Init, Extension/Protocol, Protocol access/inheritance honesty, Actor/`deinit`, Enum/TypeAlias, `data class`, `final class`, `global actor`, Subscript get-only/get+set, stored willSet/didSet, computed get-only, `lazy var`, computed get+set, `override var`, `operator func`, `#macro` (User empty/non-empty decl_body + BuiltIn/External), module/expression/statement `#name(args)`, module top-level statements, non-empty `decl_body`, `throw`, `try`/`catch`, closure `{ (params) in … }` (does not rebuild `-> Ret`), `await`, `<expr>!` force-unwrap, `as`/`as?`/`is`, `T.self`, `?.`, `..<`/`...`, `switch` literal case, `if let`, `guard let` (else before body), `repeat {…} while`, `if case .variant[(let x)]`. **:** `if case (a, b) = subject { … }` (`Expr::TuplePattern`; IfCase accepts VariantPattern|TuplePattern; TaskBlock → F001); `format/tuple_pattern_mvp` green; corpus `would_change=0` (natural Δ≈0; MVP +5); gre … ≠ C-06 `[x]`. Pin tests: `c06_fmt_*` / `c06_fmt_check_*`.

override: Functions, Variables, if/while/for-in, expression, array/subscript, Dictionaryliterals, struct declarations etc. (AST path; unsupported → F001).

### Static analysis — `syx lint`

```bash
syx lint .
```

| code | rule |
|------|------|
| L001 | Function names should start lowercase |
| L002 | Duplicate function name |
| L003 | Empty function body |
| L004 | public Functionsshould declare return type |
| L005 | async Functionsshould use await |
| L006 | Variable name illegal chars |
| L007 | globalVariablesmissing initializer |
| L008 | Empty import name |
| L009 | Bare return |
| L010 | Line over 120 chars |
| L011 | Contains TODO/FIXME |
| L012 | Prefer structured logging over print |

### Documentgenerate — `syx doc`

```bash
syx doc .
# outputs: docs/api.md
```

Generate Markdown API stub from doc comments.

### debug — `syx debug`

```bash
# Interactive terminal debug
syx debug examples/main.sy

# IDE GUI debug (DAP, VS Code extension enablesuses)
syx debug --dap examples/main.sy
```

- **terminalmode**: enter `(debug)` prompt, can list/step/run/breakpoint (see [12_BackendDebuggingGuide.md](12_BackendDebuggingGuide.md))
- **DAP mode**: stdin/stdout uses `Content-Length` framing protocol, Generated by `syx_debug::run_dap_server()` Implementation; VS Code needs `launch.json` in `"type": "syx"`, See [12_BackendDebuggingGuide.md — VS Code GUI debug](12_BackendDebuggingGuide.md#vs-code-GUI debugdap)

### REPL — `syx repl`

```bash
syx repl
syx> print(1 + 2)
syx> let x = 10
```

Single-line input, compile-run immediately.Ctrl-D exit.

### bytecodewith IR exports — `syx emit`

Write compile output for debug, backend diff, or IR reading. Does not run.

```bash
syx emit examples/hello.sy --format syxbc
syx emit examples/hello.sy --format mir
syx emit examples/hello.sy --format wasm -o hello.wasm
syx emit examples/ --format all -o emit/
```

**`--format` Values**: 

| Format | Description | Default output (single file) | Default output (project dir) |
|------|------|-------------------|---------------------|
| `syxbc` | VM bytecode text (same path as `syx run`) | `<stem>.syxbc` (same directory as source) | `main.syxbc` |
| `mir` | MIR text | `<stem>.mir` | `main.mir` |
| `wasm` | WASM binary (MIR direct compile) | `<stem>.wasm` | `main.wasm` |
| `wat` | WASM text (disassembled from binary) | `<stem>.wat` | `main.wat` |
| `llvm` | LLVM IR text | `<stem>.ll` | `main.ll` |
| `clif` | Cranelift IR text | `<stem>.clif` | `main.clif` |
| `all` | all six formats to directory | — | `emit/` Table of Contentsbelow `main.*` |

- **Single format**: Use `-o PATH` for output path; default naming per table when omitted.
- **`all`**: `-o` is output directory (default: projectTable of Contentsorsource file parentTable of Contentsunder `emit/`).
- **`syxbc`** uses CodeGen → VM instruction set; other formats use unified MIR path (`build_unified_mir`).
- **v1 write-only**: no official parser for text formats; human read/diff only.

Format specs see [SYXBC Text v1](syxbc-text-v1.md), [SYX MIR Text v1](syx-mir-text-v1.md).`llvm` same IR emission as `syx build --native`.`clif` experimental JIT / historical compare only (Cranelift AOT removed).

---

## Compilation and backends

SYX Offers **three run/compile modes**; different purposes—do not mix: 

| Mode | Command | produces | Use case |
|------|------|------|----------|
| **VM (default)** | `syx run` | no artifact; run directly | daily dev; supports `print`, File, HTTP, Block full P0 capabilities |
| **Native** | `syx build --native` | `app` executable | **LLVM sole**; `--object` only `.o`; `full-native` seeabove |
| **WASM (CLI native)** | `syx build --wasm` / `syx run --wasm` | MIR direct-compile standard `.wasm` | near-full stdlib (File/HTTP etc.) via wasmtime + `syx.mir_exec` host; **≠** browser `compiler.wasm` subset |
| **IR exports** | `syx emit --format …` | `.syxbc` / `.mir` / `.wasm` / `.wat` / `.ll` / `.clif` | debug and backend compare; no link, no run |

> **Note:** `syx build --wasm` / `syx run --wasm` direct-compile SYX **MIR** to standard WASM: guest runs on `MirRuntime` via host import `syx.mir_exec` (stack, locals, stdlib/File/HTTP, etc.), capabilities near native `syx run`. **No longer** uses SYXP container / `syx.run_program`. In-browser "compile a little then run" uses a separate path (`syx-compiler-wasm` / `examples/wasm_web`), see below [Browser boundary](#browser-boundary-compilerwasm-subset), **far smaller** than native CLI WASM. `syx emit --format wasm` is also MIR direct-compile output (export for debugging, not linked for execution), see [Bytecode and IR export](#bytecode-and-ir-export--syx-emit).

### defaultpath ( stack VM)

```bash
syx run program.sy
syx run. # project directory (reads syx.toml)
```

Pipeline: `Parser → Semantic → MIR → CodeGen → VmInstance`

### MIR optimizepath (default, stilluses VM)

Phase 11 user modules **default** MIR; `SYX_USE_MIR=0` may fall back AST direct codegen (Phase 12 removed): 

```bash
syx run program.sy # default MIR
SYX_USE_MIR=0 syx run program.sy # fallback
```

Pipeline: `Parser → Semantic → MIR → optimize → CodeGen → VM`

---

### Native backend

#### backendchoose

| backend | How to enable | Description |
|------|----------|------|
| **LLVM** (sole) | `syx build --native` | production native path; MIR → LLVM → link `syx-rt` produce `app`; parity see [10_BackendParity.md](10_BackendParity.md) |

Cranelift **AOT removed**.experimental JIT (Suite-1)see `SYX_JIT_TIER2` / [implementation-status](../books/shared/implementation-status.md), unrelated to `--native`.

#### buildexecutable (default)

In project dir (with `syx.toml`) or single `.sy` After: 

```bash
cd examples
syx build --native .
# Built native executable app (llvm)

./app
echo $?
```

Default output is **`app`** (project dir) or same-name executable (single `.sy` outputs to `{project}/target/debug/{source file name}`, e.g. `guidemo.sy` → `target/debug/guidemo`).Build emits temp `.o` and auto-links `libsyx_rt.a`, no manual `clang` link.

**Specify output path**: `-o` / `--output` (works for dir and single-file builds): 

```bash
# project dir → custom executable name
syx build --native -o myapp .

# single file → custom path
syx build --native examples/guidemo.sy -o examples/target/debug/my-gui
```

**Avoid output overwrite**: `examples/` project build `main.sy` yields `examples/target/debug/app`; if you then build `examples/guidemo.sy` without `-o`, now writes to `examples/target/debug/guidemo` (no longer overwrites `app`).

#### onlygenerateobject file

```bash
syx build --native --object .
syx build --native --object .
# Built native object main.o (llvm)
```

`--object` yields `.o` (projectTable of Contentsbelow `main.o`, orsinglefile's `*.o`).needs system `clang`/linker available.

#### dependencywithenvironment

- **clang / systemlink **: for linking `app` or compile LLVM IR to `.o`
- **Release size**: debug build `app` about 30–35MB (includes full runtime + debug symbols); `cargo build --release -p syx-cli` then `syx build --native` about 11–12MB; `strip app` may strip to ~**8–9MB**
- **Size profile (Phase 4 CI)**: `parity-gate` gate `hello --minimal` < 6MB, `examples/main --minimal` < 8MB; optional `--no-http` / `--no-file` / `--minimal` further shrink `syx-rt` link surface
- **`--release`**: `syx build --native --release` enables clang `-O2 -DNDEBUG`, release `syx-rt` link and strip flags; default debug uses `-O0 -g`.outputs go to `target/release/` with `target/debug/`.
- Explicit debug: `syx build --native --debug` (with `--release` mutually exclusive)
- Report: `syx doctor --native-profile-report` prints profile, clang args, rt archive path
- **LTO opt-in (Phase 94)**: `SYX_NATIVE_LTO=1 syx build --native --release` enables thin-LTO (`-flto=thin` + link `-flto`)
- **GUI SynUI v10 (Phase 133 · partial GA): ** `import UI` / `packages/ui` uses `__syx_synui_*` host bridge (stack · scrollView Signal bind · Web/Mobile layoutdeepen · v9 inherit image Signal bind · divider · Web/Mobile widgetsdeepen · textArea/list Signal bind · spacer · picker/progress bind · desktop run Column+Scroll+rekey · toggle/radio Signal bind · Signal↔widget bind · desktop onClick real wiringneeds HOLD/`--gui-visible`/`SYX_SYNUI_RUN=1` · checkbox/slider · `row`/`textField`/`column` · structural `renderFingerprint` CI).needs sibling checkout `../synui` (CI sibling gate).legacy `syx-gui` / egui/skia **retired**.**must not** claim wgpu HeadlessRenderer GA / full widget library / Web/Mobile **SYX language** full-set GA (→ **134+**).
- **GUI Report: ** `syx doctor --gui-backend-report` prints backend fingerprint (SynUI path-dep / host-bridge)
- **Codable L11 (Phase 94)**: pure protobuf wire; incompatible with L10 `L10:` prefix

#### fullExample

```bash
cd examples
syx build --native .
./app
echo "exit code: $?"

syx build --native -o myapp .
./myapp
```

---

### WASM backend: main.wasm (CLI nativepath)

This section describes **CLI** `syx build --wasm` / `syx run --wasm` (wasmtime + MIR host).browser `compiler.wasm` see [browserboundary](#browserboundarycompilerwasm-subset).

#### build

```bash
cd examples
syx build --wasm .
# Built wasm examples/main.wasm
```

Output: **wasm-encoder** standard WASM module exporting SYX function names (entry usually `main`).

#### Run (Built-in wasmtime + MIR host)

`syx build --wasm` SYX **MIR direct compile** to standard WASM: guest exports `main` functions; each MIR insn via host import `syx.mir_exec(op, a, b) -> i32` at `MirRuntime` on (stack, locals, stdlib/File/HTTP, etc.).****No longer** SYXP container or `syx.run_program`.

```bash
# Full program (stdlib, vars, interpolation, import math, etc.)
syx run --wasm examples/main.sy
syx run --wasm examples/

# Build.wasm (cross-function calls need MIR metadata; see syx run --wasm source path)
syx build --wasm .
syx run --wasm main.wasm
echo $? # main i32 return
```

Host Import: 

| Import | Signature | Purpose |
|------|------|------|
| `syx.mir_exec` | `(i32, i32, i32) -> i32` | Execute one MIR opcode (see `syx-wasm-emit::exec::op`) |

WASM backendVia MIR direct compile: `emit_wasm` linear path for no-jump fns; with `Jump`/`JumpIfFalse`/`JumpIfNil` fns use global `pc` + dispatch loop CFG path; matches bytecode VM control flow.

#### optional: external wasmtime / wasmer

For debug/module inspect, may use [wasmtime CLI](https://wasmtime.dev/) or [wasmer](https://wasmer.io/): 

```bash
wasmtime --invoke main main.wasm
wasmer run main.wasm --invoke main
```

#### viewexportssign

```bash
wasmtime objdump main.wasm
# or (if alreadyInstall wabt)
wasm-objdump -x main.wasm | grep Export
```

Should see like `Export[main]` entry.

#### fullExample

```bash
mkdir -p /tmp/syx_wasm_demo && cd /tmp/syx_wasm_demo
echo 'func main() { return 42 }' > main.sy
echo -e 'name = "demo"\nmain = "main.sy"' > syx.toml
syx run --wasm .
echo $?                    # 42
```

---

### three kindsModehow to choose

```bash
# ✅ Daily dev, demos, integration tests — full language + stdlib
syx run examples/file_tool

# 🔬 Verify MIR → Native pipeline (LLVM)
syx build --native . && ./app; echo $?
syx build --native . && ./app; echo $?

# Verify MIR → WASM pipeline (CLI native; near-full host)
syx run --wasm . && echo $?
```

### browserboundary: compiler.wasm (subset)

For in-browser compile-run SYX, use: 

- [`crates/syx-compiler-wasm`](../crates/syx-compiler-wasm): produces `compiler.wasm` (**C ABI cdylib**, **no** wasm-bindgen), exports `syx_compile_source_to_wasm_bytes` (FrontendPipeline → MIR → WASM bytes; inject `browser_prelude`)and guest host bridges `syx_guest_mir_exec` / `syx_guest_end_fn` / `syx_guest_take_catch_pc` / `syx_guest_get_return`
- [`examples/wasm_web/`](../examples/wasm_web/): page `fetch('./compiler.wasm')` then compiles user source; guest `syx.*` AOT imports wired to in-compiler `MirRuntime`; AOT-exported `main` is **exit code 0**, page uses `syx_guest_get_return` for RT return

Capability is **compile subset + AOT**, ****far smaller** than native `syx run` / `syx run --wasm`.**≠** Playwright product GA; **≠** native CLI WASM capabilitydeepen.

**Capability floor (honest; not deepen promise): ** No project / `syx.toml` / package dependency resolution; No full File / HTTP / TCP host; **Also no** SQL/ORM, native TLS/WSS, dynamic plugins, product GUI, peer Task/Actor scheduling.`getrandom` is `custom` deterministic fill (not browser CSPRNG).After compiler changes rebuild and `cp` `compiler.wasm`; old artifacts may error `Import "__wbindgen_placeholder__"` (current path has no wbindgen).

Local steps matching CI (see `.github/workflows/ci.yml`): 

```bash
rustup target add wasm32-unknown-unknown
cargo build -p syx-compiler-wasm --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/syx_compiler_wasm.wasm examples/wasm_web/compiler.wasm
# Open examples/wasm_web/index.html (Playwright uses file://; if browser blocks fetch, serve locally e.g. python3 -m http.server)
```

Optional GUI smoke (only `tier2w`; **≠** Playwright product GA): 

```bash
cd examples/wasm_web && npm i && npm run test:gui
```

More detail see [`examples/wasm_web/README.md`](../examples/wasm_web/README.md) with [`crates/syx-compiler-wasm/README.md`](../crates/syx-compiler-wasm/README.md).

### JIT (experimental)

`syx-vm` provides `JitCompiler` hot-path count stub for future AOT/JIT fallback; off by default.

---

## Testing

### project in-process Testing

 `.sy` put files in `tests/` (**recursive** `tests/**/*.sy`; also supports `syx test path/to/file.sy`): 

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
syx test. --format tap # or SYX_TEST_TAP=1
```

Cases with `@Test` may use `packages/testing` assertions (**requires explicit** `import Testing`, not auto-injected; ≠ auto-import): 

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

Each test file compiled/run separately; failure non-zero exit.C-08 narrow closeout: JSON + JUnit + filter **`[x]`**; supports recursive discovery, `import Testing` asserts, Without `@Test` `Testing.test` registers closures run before entry returns (JSON cases `{file}.{name}`), and `@Test async` + completable `await` → JSON pass. Pin tests: `syx_test_async_await_assert_eq_passes`.Tutorial see [books/language-guide/34-testing.md](../books/language-guide/34-testing.md).**≠** Testing auto-import; **≠** full async test matrix.See [plan/topics/syx-test.md](../plan/topics/syx-test.md).

### benchmark entry (C-09 first-slice ·)

C-09 bench narrow entry `[x]` ( docs + `syx bench` thin delegate + claimed; **≠ ms SLA ≠ coverage / fuzz): 

```bash
syx bench --list
syx bench path_join_criterion

# Equivalent cargo
cargo bench -p syx-driver --bench path_join_criterion
cargo bench -p syx-driver --bench http_response_criterion
cargo bench -p syx-driver --bench sql_exec_criterion
cargo bench -p syx-driver --bench tcp_connect_criterion
cargo bench -p syx-driver --bench compile_bench
```

PR CI short sampling on four Criterion; nightly runs `compile_bench`.C-09 **`[x]`** (narrow bench); ≠ coverage gate / fuzz / property / SLA.See [plan/topics/syx-bench.md](../plan/topics/syx-bench.md).

### overriderate entry (C-09 residual · stub + deepen)

Product CLI and equivalent cargo (requires `cargo-llvm-cov`; narrow entry **claimed**; **≠ CI % gate ≠ coverage % GA): 

```bash
syx cov

# etc. 
cargo llvm-cov -p syx-driver --test native --no-report
```

See [plan/topics/syx-coverage.md](../plan/topics/syx-coverage.md) with [docs/lang/c09-syx-coverage-stub-first-slice.md](lang/c09-syx-coverage-stub-first-slice.md).

### Fuzz entry (C-09 residual · CI + / maintenance)

product CLI withEquivalent cargo (`fuzz/` scaffold **delivered**; PR CI **bounded pin** `-runs=1`; C-09 maintenance + platform maintenance floor **signed**; **≠** scaffold GA **≠** unbounded fuzz): 

```bash
syx fuzz

# etc. (requires local cargo-fuzz)
cargo fuzz run syx_smoke

# bounded smoke (PR CI hard gate;)
cargo fuzz run syx_smoke -- -runs=1
```

See [plan/topics/syx-fuzz.md](../plan/topics/syx-fuzz.md), [docs/lang/c09-syx-fuzz-stub-first-slice.md](lang/c09-syx-fuzz-stub-first-slice.md), [docs/lang/c09-syx-residual-maintenance-first-slice.md](lang/c09-syx-residual-maintenance-first-slice.md) with [docs/lang/platform-residual-maintenance-first-slice.md](lang/platform-residual-maintenance-first-slice.md).

### Property entry (C-09 residual · CI + / maintenance)

product CLI withEquivalent cargo (`property_smoke` harness **delivered**; PR CI **explicit pin**; C-09 maintenance + platform maintenance floor **signed**; **≠** proptestest GA **≠** `c09_property_ci_ga`): 

```bash
syx prop

# etc. 
cargo test -p syx-driver --test property_smoke
```

See [plan/topics/syx-property.md](../plan/topics/syx-property.md), [docs/lang/c09-syx-property-stub-first-slice.md](lang/c09-syx-property-stub-first-slice.md), [docs/lang/c09-syx-residual-maintenance-first-slice.md](lang/c09-syx-residual-maintenance-first-slice.md) with [docs/lang/platform-residual-maintenance-first-slice.md](lang/platform-residual-maintenance-first-slice.md).

### Cargo integrationTesting

Run full test suite in repo: 

```bash
cargo test
cargo test -p syx-driver --test p0_integration
cargo test -p syx-driver --test p1_integration
```

Network tests `p0_http_run` default `#[ignore]`; enable manually: 

```bash
cargo test -p syx-driver p0_http_run -- --ignored
```

---

## Example projects

| path | Description |
|------|------|
| [examples/hello.sy](../examples/hello.sy) | Minimal Hello World |
| [examples/main.sy](../examples/main.sy) | Modules + strings + arrays + regex |
| [examples/file_tool/](../examples/file_tool/) | EC-M2-1 slice: Path + FileHandle write/read roundtrip |
| [examples/http_tool/](../examples/http_tool/) | EC-M2-1 slice: Http.get refuse + loopback |
| [examples/http_server_tool/](../examples/http_server_tool/) | EC-M2-1 slice: Http.serve + /health |
| [examples/sql_tool/](../examples/sql_tool/) | EC-M2-1 slice: Sql `:memory:` CRUD |
| [examples/math.sy](../examples/math.sy) | Exportable module |
| [examples/deps/](../examples/deps/) | dependency math subproject |
| [examples/module_main.sy](../examples/module_main.sy) | multi-modeBlockentryExample |
| [examples/os_info/](../examples/os_info/) | Os system info (try/catch) |
| [examples/linalg/](../examples/linalg/) | packed-SYX `Mat`: mul/solve, shape, pad/padReflect (≠ product Mat GA) |
| [examples/log_fields.sy](../examples/log_fields.sy) | `Log` level filter + `infoFields` |

**Suggested learning order**: `hello.sy` → `main.sy` → `file_tool/` → `http_tool/` → `sql_tool/` → `linalg/` → `deps/`

Stdlib narrative see [books/library-guide/](../books/library-guide/00-preface.md).

---

## Environment variables

| Variables | Value | Effect |
|------|-----|------|
| `SYX_USE_MIR` | `0` or `false` | Disable MIR; fall back AST codegen (MIR on by default) |
| `SYX_JIT_FASTPATH` | `1` | VM tier-0.5: hotspot MIR fastpath stub (experimental) |
| `SYX_REGISTRY_MIRROR` | URL | Mirror URL when primary registry unreachable |
| `SYX_PUBLISHER_SIGNING_KEY` | 64 hex | Publish: Ed25519 32-byte seed; produces `syx1:ed25519:<pk>:<sig>` (C-05) |
| `SYX_REGISTRY_VERIFY_KEY` | 64 hex | Install: publisher verifying-key pin (C-02); mismatch `P011` |
| `SYX_HOST_PLUGIN_PATH` | colon-separated paths | Load Host plugins `.so`/`.dylib` (opt-in) |
| `SYX_HOST_PLUGIN_VERIFY` | `1` | Force plugin ed25519 verify |
| `SYX_HOST_PLUGINS` | `0` / `disabled` | Disable plugin loading (CI control) |
| `SYX_MACRO_HYGIENE` | `0` | Disable user `#macro` hygiene v2 (temporary) |
| `SYX_MACRO_TRACE` | `1` | Print macro expansion trace (debug) |
| `SYX_EC_M2_1_HTTP_BASE` | URL origin | `examples/http_tool` positive base URL (e.g. `http://127.0.0.1:PORT`) |
| `SYX_HTTP_SERVE_MAX_REQUESTS` | positive integer N | `Http.serve` return after N responses (test/ops hook; ≠ `Http.shutdown` graceful; if unset needs shutdown or process exit) |

---

## Diagnostic codes

### semanticsanalysis (S series)

| code | Meaning |
|------|------|
| S001 | Duplicate symbol definition |
| S002 | Undefined identifier |
| S003 | Assign to immutable variable |
| S004 | Type mismatch |
| S005 | Cross-module access to internal symbol |
| S006 | Protocol missing `next()` |
| S024 | Collection hosted op forbids legacy MethodCall |
| S025 | hosted methods require extension/CallHost |

### Host plugin (H series)

| code | Meaning |
|------|------|
| H001 | Plugin not loaded or Host missing |
| H002 | Plugin manifest/lock runtime mismatch |

### macro (E series)

| code | Meaning |
|------|------|
| E002 | Macro expansion depth exceeded or cycle |

### Run when (R series)

| code | Meaning |
|------|------|
| R007 | Force unwrap nil (`!`) |
| R008 | `readLine` readLine stdin failure |

### Lint (L series)

see [Toolchain — syx lint](#Static analysis--syx-lint).

---

## FAQ

### `syx: command not found`

CLI not installed.use uses `cargo run -p syx-cli --` prefix, orAfter `cargo install --path crates/syx-cli`.

### Module not found

1. confirm `syx.toml` in `dependencies` configuration is correct
2. Run `syx install .` to generate/update `syx.lock` (missing lock reports **P001**)
3. dependency Block inFunctionsneedsmark `public`

### HTTP tests fail

when the network is unreachable, use uses compile-only Testingorignore networkcases.for local dev you mayuses mock orskip HTTP segment.

### Compile cache issues

```bash
rm -rf target/cache
syx check .
```

### How to view bytecode or IR?

use uses `syx emit` (see [bytecodewith IR exports](#bytecodewith-ir-exports--syx-emit)): 

```bash
syx emit examples/hello.sy --format syxbc # VM instruction
syx emit examples/hello.sy --format mir       # MIR
syx emit examples/hello.sy --format llvm      # LLVM IR
syx emit examples/ --format all -o emit/ # six formatsFormatonceexports
```

### `--native` How to run?

defaultgenerate **`app` executable**, run directly: 

```bash
syx build --native .
./app
echo $? # exitcode
```

onlywhen producing object files use `--object`: 

```bash
syx build --native --object .
file main.o # confirm object file
```

See [Native backend](#native-backend).

### Native whether stillsupports Cranelift？

**not supports AOT.** `syx build --native` **only**uses LLVM.historical Cranelift AOT / `--backend=cranelift` removed.

Experimental **JIT tier-2** (`syx-cranelift-jit`, `SYX_JIT_TIER2=1`) is a separate opt-in path, default off, **not equal to** `syx build --native`.


### `--wasm` produces `main.wasm`, How to run?

use usesBuilt-inRun (recommended): 

```bash
syx run --wasm .              # compile project and run
syx run --wasm main.wasm # Runexisting.wasm
echo $? # view return value
```

Or external wasmtime: `wasmtime --invoke main main.wasm`.

See [WASM backend: main.wasm (CLI nativepath)](#wasm-backendmainwasmcli-nativepath).browserin-compilersubsetsee [browserboundary](#browserboundarycompilerwasm-subset) (≠ this FAQ CLI `main.wasm`).

### `--native` did not produce executable

Default should generate **`app`**. If only `.o`, **`--object`** was used. If link fails, confirm **clang** is installed and `libsyx_rt.a` is available (see [plan/topics/profiles-sdk.md](../plan/topics/profiles-sdk.md)).

When no source tree is available, use environment variables to specify runtime archives (C-11 narrow `[x]` contract; ≠ Release full-matrix CI upload):

- **`SYX_RT_STATIC_LIB`**: points to an existing `libsyx_rt.a` / `syx_rt.lib` (orincludes hashed archive deps Table of Contents)
- **`SYX_RT_PROFILE=<id>`**: lookup per SDK install layout `bin/../lib/profiles/<id>/libsyx_rt.a` (fallback `../lib/<id>/`); convention see [`packaging/syx-sdk/README.md`](../packaging/syx-sdk/README.md)

these take precedence over compile-time embed with cargo `target/` search.
### Differences from Swift

SYX has similar syntax but is not a Swift superset. Missing or MVP-only features include: full concurrency, generic constraints, property wrappers, full macro system, etc. See [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md).

---

## Related documentation

| Document | Content |
|------|------|
| [8_QuickReference.md](./8_QuickReference.md) | Syntax quick reference |
| [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) | P0–P3 Feature checklist |
| [5_CompleteTechnicalDocumentation.md](./5_CompleteTechnicalDocumentation.md) | Compiler architecture |
| [4_InstructionSet.md](./4_InstructionSet.md) | VM Instruction set |

---

**Maintainers:** SYX Core Team  
**Last updated:** 2026-10-27  
**Version:** SYX 117.0 / Phase 139
