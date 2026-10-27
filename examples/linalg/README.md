# Linalg 范例

`import Linalg` 提供 packed-SYX `Mat`（句柄 + 行主序 `Array<Float64>`）。**不是** CORE prelude，**不是** 产品 Mat GA（需 E-00）。

| 文件 | 内容 |
|------|------|
| [matmul.sy](matmul.sy) | `from` / `zeros` / `eye` / `*`（矩阵乘）/ `solve` / `packed` |
| [shape.sy](shape.sy) | `hstack` / `vstack` / `takeRows` / `dropCols` / `transposed` |
| [pad.sy](pad.sy) | 零 pad、常数 pad、`pad*Reflect`（numpy `mode='reflect'`，n=1） |

## 插件安装布局

`syx` 会从**可执行文件旁**自动发现插件（也识别仓库根下的 `./bin`）：

```
bin/
  syx                              # CLI
  libfaerlin_plugin.dylib|.so|.dll # 系统插件动态库
  faerlin/plugin.json              # 系统插件清单
  plugins/                         # 用户扩展插件（每插件一子目录 + plugin.json）
```

构建并安装系统 faerlin：

```bash
./plugins/faerlin/build.sh
# 或：cargo build -p syx-cli --release && cp target/release/syx-cli bin/syx
```

## 运行

```bash
./bin/syx run ./examples/linalg/pad.sy
./bin/syx run ./examples/linalg/matmul.sy
```

`Mat` 走 `__syx_plugin_faer_*` 句柄 Host：当 `bin/` 下检测到系统 faerlin 时，CLI 会注册进程内 Mat Host，并 dlopen `libfaerlin_plugin.*`（cdylib 面）。

也可用环境变量显式指定：

```bash
export SYX_HOST_PLUGINS=1
export SYX_HOST_PLUGIN_PATH="$PWD/bin/faerlin/plugin.json"
./bin/syx run ./examples/linalg/pad.sy
```

Evergreen 烟测（进程内注册，不依赖 bin/）：

```bash
cargo test -p syx-driver --test evergreen plugin_mat_mul_vm -- --test-threads=1 --nocapture
```

构造一律 `Mat.from(packed, rows:, cols:)`；**不要**写 `Mat([[1.0, 2.0], …])`。访问用 `.at(i, col: j)` / `.packed()`，没有 `A[i, j]`。

指南：[books/library-guide/02-linalg.md](../../books/library-guide/02-linalg.md)
