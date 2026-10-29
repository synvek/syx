# SYX Quick Usage Guide

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
| clang / LLVM | Optional: with `--features full-native`, use sibling `../llvm` or `SYX_LLVM_PREFIX` (inkwell emits `.o` + **static embedded LLD**); see [plan/topics/native-llvm.md](../plan/topics/native-llvm.md). Official **syx-sdk** packages ship LLD + `lib/sysroot/<triple>` so end users do **not** need VS/Xcode to link a single-file native exe. |

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

Examples: `examples/http_tool/` (client); `examples/http_server_tool/` (server; default port 18765). Also see `examples/httpclient_demo.sy` / `httpserver_demo.sy`.

### Sql（SQLite）

`import Sql` after importmulti-connection facade (`throws SqlError`): 

| Method | Description |
|------|------|
| `Sql.open(dsn:)` | Open connection (`":memory:"` or file path)→ `Int` connId |
| `Sql.execOn(connId:, sql:)` | Execute on connection |
| `Sql.queryOn(connId:, sql:)` | Query → **`Array` of `SqlRow`** (May use `.count`) |
| `Sql.close(connId:)` | Close explicitly opened connection |

Example: `examples/sql_tool/` (EC-M2-1 SQLite service slice). Live count `sql_conn_live_count` counts explicit connections only (id≥1), in test/diagnostic layer (EC-M2-3 first pin), **not** application API.

### String three views (`M2-S2-01`)

Default character model is **grapheme (grapheme cluster)** `Character`: 

| view | API | Description |
|------|-----|------|
| Grapheme | `.count` / `s[i]` / `.substring` / `for-in` | Int subscript is grapheme offset (see below) |
| Unicode scalar | `String.unicodeScalarCount(text:)` / `String.unicodeScalars(text:)` | may diverge from grapheme (e.g. ZWJ family emoji) |
| UTF-8 bytes | `String.utf8Count(text:)` / `String.utf8Bytes(text:)` | byte length and `Array<Int>` (0–255) |

### String subscript complexity (`M2-S2-02`)

`s[i]` with `.substring(start, end)` uses **Int grapheme offset** (not byte offset, nor Swift opaque `String.Index`).**single random access is O(n)** (implemented via grapheme iteration `.nth` / `.skip`, no hidden O(1) cache).for sequential scan use `for-in` (by grapheme). Pin tests: `m2_s2_02_string_index_honesty`.≠ opaque Index GA.

### String normalization / case folding（`M2-S2-03`）

| API | Description |
|-----|------|
| `String.normalizedNFC(text:)` | Unicode **NFC** |
| `String.normalizedNFD(text:)` | Unicode **NFD** |
| `String.caseFolded(text:)` | Unicode **default/full case folding** ( e.g. `ß`→`ss`) |
| `.lowercased()` / `.uppercased()` | Unicode **simple case mapping**（≠ `caseFolded`；≠ locale/Turkish） |

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

### collection HOF feature-off (`M2-S1-04`)

`Array.map` / `filter` / `reduce` (includes `__syx_array_*` CallHost) at illegal receiver, missing Closure, or interpreter feature-off **fail-closed**: diagnostic code **`U001` / unsupported**, **must not** use empty array `[]` or silent `Nil` as success.

```syx
// Illegal: second arg not Closure → runtime failure (U001), will not print []
__syx_array_map([1, 2, 3], 0)
```

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

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libplugin.so
syx run app.sy
```

`syx.toml` optional `[plugins].paths`; index lock `syx-plugin-lock.json`.verify signature: `SYX_HOST_PLUGIN_VERIFY=1`.


---

## Toolchain

### Embed API（Phase 14–15）

Phase 14+ Embed API (`syx_embed_v2_*`): sandbox / timeout / memory & RSS budget / capability allowlist.See [architecture decision](adr/architecture-decisions.md).

### Formatting — `syx format`

Format project entry file (`syx.toml` in `main` specify): 

```bash
syx format .
```

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

Default should generate a **single-file** `app` (static `syx_rt`, no `syx_rt.dll`). If only `.o`, **`--object`** was used.

If link fails on a clean machine, prefer the official **syx-sdk** zip/tarball (`bin/syx` + `lib/syx_rt*` + `lib/sysroot/<triple>`). Check layout with:

```bash
syx doctor --native-sdk
```

Source trees may still use a local MSVC/Xcode toolchain when no sysroot is present. See [`packaging/syx-sdk/README.md`](../packaging/syx-sdk/README.md).

When no source tree is available, use environment variables to specify runtime archives / sysroot (C-11 narrow `[x]` contract; ≠ Release full-matrix CI upload):

- **`SYX_RT_STATIC_LIB`**: points to an existing `libsyx_rt.a` / `syx_rt.lib` (or includes hashed archive deps)
- **`SYX_RT_PROFILE=<id>`**: lookup per SDK install layout `bin/../lib/profiles/<id>/libsyx_rt.a` (fallback `../lib/<id>/`)
- **`SYX_SYSROOT`**: absolute path to link import libs (replaces `bin/../lib/sysroot/<triple>`)

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
