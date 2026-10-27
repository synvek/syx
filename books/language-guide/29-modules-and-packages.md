# Modules and Packages

**Implementation status:** single-file / multi-file projects / `import` path dependencies / git dependencies / registry dependencies / `[plugins].paths` / `[macros].exports` (Phase 120 schema GA) / `syx install` / `syx add` / `syx update` / `syx lock` — **all Implemented**

The SYX module system has three layers: **single file** (the smallest unit), **multi-file project** (`syx.toml`), and **package management** (`syx install` / `syx.lock`). This chapter covers all paths and the complete `syx.toml` field set.

## Single-File Modules

Every `.sy` file is a compilation unit; `func main()` is the entry point:

```syx
// hello.sy
func main() {
    print("Hello, World")
}
```

```bash
syx run hello.sy
```

## import

```syx
import Math
import local.deps as Deps
import utils.helpers as H
```

`import` path resolution:

| Form | Resolution |
|------|------|
| `import math` | Finds `math.sy` in the same directory or the key `math` in `syx.toml` dependencies |
| `import local.deps` | Nested namespace (`local/deps.sy` or the subdirectory `local/deps/`) |
| `import utils.helpers as H` | Aliased as `H` |

## Multi-File Projects

```
myapp/
├── syx.toml
├── syx.lock
├── syx-plugin-lock.json    # Optional: plugin host index lock (Phase 49+)
├── main.sy
├── lib/
│   ├── math.sy
│   └── text.sy
└── plugins/
    └── mycrypto/
        ├── plugin.json
        └── lib/
            └── libmycrypto.so
```

## Complete syx.toml Field Set

```toml
# === Package metadata ===
name = "myapp"                              # Project name
version = "0.1.0"                            # Version (SemVer)
main = "main.sy"                            # Entry SYX file

# === Dependencies ===
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

# registry dependency (requires [registry] configuration)
[registry]
default = "https://packages.sy.dev"

[dependencies.http_client]
version = "^1.0"

# === Plugins ===
[plugins]
paths = [
    "./plugins/mycrypto/plugin.json",
    "./plugins/echo/plugin.json"
]

# === Macro package exports ===
[macros]
exports = ["Codable", "Hashable", "Equatable"]

# === Compilation configuration (optional) ===
[build]
target = "native"                           # "vm" | "native" | "wasm"
backend = "llvm"                            # "llvm" (Phase 77+ GA; Cranelift removed)
opt-level = 2                               # 0 / 1 / 2 / 3
lto = false                                 # Phase 94 opt-in
strip = true

# === Enable/disable features ===
[features]
simd = true
gc = true

# === Profile (Service DI v5+) ===
[profiles]
default = ["dev"]
prod = ["prod"]
```

## Dependency Types

### Path Dependencies

```toml
[dependencies]
utils = "../utils/main.sy"                 # Simple form
[dependencies.complex]
version = "1.0.0"
path = "../complex"                          # Directory (needs syx.toml)
```

### git Dependencies

```toml
[dependencies.lib]
version = "^0.2"
git = "https://github.com/org/lib.git"
rev = "v0.2.1"                               # tag / commit SHA
```

### registry Dependencies

```toml
[registry]
default = "https://packages.sy.dev"

[dependencies.http-client]
version = "^1.0"
checksum = "sha256:..."                     # Optional, force lock
```

> **registry:** you must set `[registry].default` first; otherwise the environment variable `SYX_REGISTRY` is used.

## The syx.lock Lock File

Generated after `syx install` / `syx add` / `syx update`. **You must install before compiling a project with dependencies** (otherwise **P001**):

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

Git / registry packages are installed into `.sy/vendor/` (already in `.gitignore`). Path dependencies only record the path; the source is not copied.

## CLI Commands

| Command | Description |
|------|------|
| `syx install [path]` | Resolves and installs dependencies, writes `syx.lock` |
| `syx add <name> --path PATH` | Adds a path dependency and installs |
| `syx add <name> --git URL [--rev TAG]` | Adds a git dependency |
| `syx update [path]` | Re-resolves and installs all dependencies |
| `syx publish [path] [--registry URL]` | Publishes a package to the registry |

```bash
syx install examples/deps                  # Generates syx.lock
syx run examples/deps                      # Outputs 42 (local_math.add(10, 32))
```

## Cross-Module Access

```syx
// module: math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {             // Within module
    return 0
}
```

```syx
// module: main.sy
import math

func main() {
    print(math.add(a: 1, b: 2))              // ✅ public
    // print(math.secret())                  // ❌ S005
}
```

See [Chapter 28, Access Control](28-access-control.md).

## Multi-Module Project Example

```bash
# examples/deps/
syx install examples/deps
syx run examples/deps                      # 42
```

## Plugin Manifests (Phase 49+)

`syx.toml` may declare plugin manifests:

```toml
[plugins]
paths = ["./plugins/mycrypto/plugin.json"]
```

`plugin.json` example:

```json
{
  "id": "mycrypto",
  "api_version": 1,
  "library": "libmycrypto_plugin.so",
  "hosts": [
    {
      "name": "__syx_plugin_mycrypto_sha256",
      "return_type": "String",
      "params": ["data: String"]
    }
  ],
  "capabilities": ["crypto"],
  "min_syx_version": "27.0"
}
```

See [Chapter 33, Runtime Extensions](33-runtime-extensions.md).

## Macro Package Schema (Phase 120 GA)

```toml
[macros]
exports = ["Codable", "Hashable"]
```

Since Phase 120, the `[macros].exports` schema is aligned with `SyxManifest`; see `examples/macro_package/`.

## Incremental Compilation Cache

Compilation artifacts are cached in the project root `target/cache/` (MIR/module bincode, fingerprint `syx-cache-v110`). When compilation behaves abnormally:

```bash
rm -rf target/cache
syx check .
```

## Complete Example

```
my-app/
├── syx.toml
├── syx.lock
├── main.sy
├── lib/
│   ├── math.sy
│   └── helpers.sy
└── tests/
    ├── case_a.sy
    └── case_b.sy
```

`syx.toml`:

```toml
name = "my-app"
version = "0.2.0"
main = "main.sy"

[dependencies.local_math]
version = "1.0.0"
path = "lib/math.sy"

[build]
target = "native"
backend = "llvm"
opt-level = 2
```

`lib/math.sy`:

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}
```

`main.sy`:

```syx
import local_math

func main() {
    print("3 + 4 = \(local_math.add(a: 3, b: 4))")
}
```

## Diagnostics

| Code | Trigger | Fix |
|------|------|------|
| **P001** | `syx.lock` missing at compile time | `syx install` |
| **S005** | Accessing internal across modules | Change it to `public` |
| **H002** | Plugin manifest inconsistent | Sync `syx-plugin-lock.json` |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| SPM | `syx-pkg` + `syx.toml` |
| `Package.swift` | `syx.toml` (TOML format) |
| `import Module` | `import module` (case-insensitive) |
| Nested `import` | Supports `import a.b.c` + `as` aliases |
| `target` / `product` | `syx.toml` single target |
| git dependencies | ✅ tag / rev |
| registry | ✅ private registry support |

---

**Next chapter** → [Advanced Operators](30-advanced-operators.md)

**See also** → [Complete Usage Guide](../../docs/9_UsageGuide.md) · [Standard Library Guide](../library-guide/00-preface.md) · [Language Reference: Declarations](../language-reference/05-declarations.md) · [Chapter 28, Access Control](28-access-control.md) · [Chapter 33, Runtime Extensions](33-runtime-extensions.md)
