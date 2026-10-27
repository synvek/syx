# 模块与包

**实现状态：** 单文件 / 多文件项目 / `import` 路径依赖 / git 依赖 / registry 依赖 / `[plugins].paths` / `[macros].exports`（Phase 120 schema GA） / `syx install` / `syx add` / `syx update` / `syx lock` 全部 **已实现**

SYX 的模块系统包括三个层级：**单文件**（最小单元）、**多文件项目**（`syx.toml`）、**包管理**（`syx install` / `syx.lock`）。本章覆盖所有路径与 `syx.toml` 完整字段。

## 单文件模块

每个 `.sy` 文件为一个编译单元；`func main()` 为入口：

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

`import` 路径对应：

| 形式 | 解析 |
|------|------|
| `import math` | 找同目录 `math.sy` 或 `syx.toml` 依赖中键 `math` |
| `import local.deps` | 嵌套命名空间（`local/deps.sy` 或子目录 `local/deps/`） |
| `import utils.helpers as H` | 别名 `H` 引用 |

## 多文件项目

```
myapp/
├── syx.toml
├── syx.lock
├── syx-plugin-lock.json    # 可选：插件 Host 索引锁（Phase 49+）
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

## syx.toml 完整字段

```toml
# === 包元信息 ===
name = "myapp"                              # 项目名
version = "0.1.0"                            # 版本（SemVer）
main = "main.sy"                            # 入口 SYX 文件

# === 依赖 ===
[dependencies]
# 简单路径依赖
utils = "../utils/main.sy"

# 带 SemVer 的 path 依赖
[dependencies.local_math]
version = "1.0.0"
path = "../deps_math"

# git 依赖
[dependencies.lib]
version = "^0.2"
git = "https://github.com/org/lib.git"
rev = "v0.2.1"

# registry 依赖（需 [registry] 配置）
[registry]
default = "https://packages.sy.dev"

[dependencies.http_client]
version = "^1.0"

# === 插件 ===
[plugins]
paths = [
    "./plugins/mycrypto/plugin.json",
    "./plugins/echo/plugin.json"
]

# === 宏包导出 ===
[macros]
exports = ["Codable", "Hashable", "Equatable"]

# === 编译配置（可选） ===
[build]
target = "native"                           # "vm" | "native" | "wasm"
backend = "llvm"                            # "llvm"（Phase 77+ GA；Cranelift 已移除）
opt-level = 2                               # 0 / 1 / 2 / 3
lto = false                                 # Phase 94 opt-in
strip = true

# === 启用/禁用功能 ===
[features]
simd = true
gc = true

# === Profile（Service DI v5+） ===
[profiles]
default = ["dev"]
prod = ["prod"]
```

## 依赖类型

### 路径依赖

```toml
[dependencies]
utils = "../utils/main.sy"                 # 简单形式
[dependencies.complex]
version = "1.0.0"
path = "../complex"                          # 目录（需 syx.toml）
```

### git 依赖

```toml
[dependencies.lib]
version = "^0.2"
git = "https://github.com/org/lib.git"
rev = "v0.2.1"                               # tag / commit SHA
```

### registry 依赖

```toml
[registry]
default = "https://packages.sy.dev"

[dependencies.http-client]
version = "^1.0"
checksum = "sha256:..."                     # 可选，强制锁
```

> **registry：** 需先设置 `[registry].default`；否则使用环境变量 `SYX_REGISTRY`。

## syx.lock 锁文件

执行 `syx install` / `syx add` / `syx update` 后生成。**编译带依赖的项目前必须先 install**（否则报 **P001**）：

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

Git / registry 包安装到 `.sy/vendor/`（已加入 `.gitignore`）。Path 依赖仅记录路径，不复制源码。

## CLI 命令

| 命令 | 说明 |
|------|------|
| `syx install [path]` | 解析并安装依赖，写入 `syx.lock` |
| `syx add <name> --path PATH` | 添加 path 依赖并 install |
| `syx add <name> --git URL [--rev TAG]` | 添加 git 依赖 |
| `syx update [path]` | 重新解析并安装全部依赖 |
| `syx publish [path] [--registry URL]` | 发布包到 registry |

```bash
syx install examples/deps                  # 生成 syx.lock
syx run examples/deps                      # 输出 42（local_math.add(10, 32)）
```

## 跨模块访问

```syx
// module: math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {             // 模块内
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

详见 [第 28 章 访问控制](28-access-control.md)。

## 多模块项目示例

```bash
# examples/deps/
syx install examples/deps
syx run examples/deps                      # 42
```

## 插件 manifest（Phase 49+）

`syx.toml` 可声明插件 manifest：

```toml
[plugins]
paths = ["./plugins/mycrypto/plugin.json"]
```

`plugin.json` 示例：

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

详见 [第 33 章 运行时扩展](33-runtime-extensions.md)。

## 宏包 schema（Phase 120 GA）

```toml
[macros]
exports = ["Codable", "Hashable"]
```

Phase 120 起 `[macros].exports` schema 与 `SyxManifest` 对齐；`examples/macro_package/` 演示。

## 增量编译缓存

编译产物缓存在项目根 `target/cache/`（MIR/module bincode，指纹 `syx-cache-v110`）。编译行为异常时：

```bash
rm -rf target/cache
syx check .
```

## 完整示例

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

`syx.toml`：

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

`lib/math.sy`：

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}
```

`main.sy`：

```syx
import local_math

func main() {
    print("3 + 4 = \(local_math.add(a: 3, b: 4))")
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **P001** | 编译时缺 `syx.lock` | `syx install` |
| **S005** | 跨模块访问 internal | 改 public |
| **H002** | 插件 manifest 不一致 | 同步 `syx-plugin-lock.json` |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| SPM | `syx-pkg` + `syx.toml` |
| `Package.swift` | `syx.toml`（TOML 格式） |
| `import Module` | `import module`（大小写不敏感） |
| `import` 嵌套 | 支持 `import a.b.c` + `as` 别名 |
| `target` / `product` | `syx.toml` 单 target |
| git 依赖 | ✅ tag / rev |
| registry | ✅ 私有 registry 支持 |

---

**下一章** → [高级运算符](30-advanced-operators.md)

**参见** → [完整使用指南](../../../docs/9_UsageGuide.md) · [标准库指南](../library-guide/00-preface.md) · [语言参考：声明](../language-reference/05-declarations.md) · [第 28 章 访问控制](28-access-control.md) · [第 33 章 运行时扩展](33-runtime-extensions.md)
