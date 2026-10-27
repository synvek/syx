# 运行时扩展（动态 Host 插件）

**实现状态：** 静态 `HostFnId` 注册 **已实现**；`dlopen` 插件 opt-in **部分实现**（Phase 49–52）；`plugin.json` + lock 文件 **部分实现**（Phase 49+）；`ed25519` 验签 **部分实现**（Phase 50–51）；Embed API 路径 **已实现**（Phase 51+）

SYX 的特殊能力（I/O、JSON、哈希、SIMD 等）最终通过 **Host 函数** 进入 runtime。维护者可在编译器仓库内静态注册 `HostFnId`；**应用开发者** 还可通过 **dlopen 插件** 在运行期追加 Host，无需 fork 编译器。

嵌入包（`import Linalg` / `Os` / `Log` / `IO` …）的用户面叙事见 [标准库指南](../library-guide/00-preface.md)；Linalg 范例见 [examples/linalg/](../../../examples/linalg/)。

## 能力分层

```
SYX 源码（import / __syx_plugin_*）
    → 语义分析 + MIR: MirOp::CallHost(n)
    → call_host(n) → 静态 Host 表 或 插件槽
    → 插件 .so 内 C/Rust 回调
```

| 层 | 说明 |
|----|------|
| `lib/*.sy` | 用户可见 API |
| Host 注册表 | 名字 → 索引 → 返回类型 |
| 静态 `hosts.rs` / 插件 `.so` | 实际操作 VM/MIR 栈 |

> **纯 SYX 代码** 不能凭空调用任意 C 库；须已有 Host 或通过插件注册。

## 启用插件（运行时）

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libecho_plugin.so:/path/to/libmycrypto.so
syx run app.sy
```

Native 可执行文件在 `syx_rt_init_plugins()` 阶段加载（`syx build --native` 后 `./app` 同样生效）。

> **多路径分隔符：** Unix `:`，Windows `;`。

SYX 侧调用插件 Host（命名空间 `__syx_plugin_{id}_{fn}`）：

```syx
func main() {
    let msg = __syx_plugin_echo_echo("hello")
    print(msg)
}
```

**完整示例：** [examples/plugin_demo](../../../examples/plugin_demo/)（C cdylib SHA-256 + `syx run`）；[examples/rust_plugin_demo](../../../examples/rust_plugin_demo/)（Rust `cdylib` + 同一 C ABI）；[examples/linalg](../../../examples/linalg/)（系统 faerlin：`bin/libfaerlin_plugin.*` + `bin/faerlin/plugin.json`，用户扩展放 `bin/plugins/`）。

**安装前缀自动发现（无需 `SYX_HOST_PLUGIN_PATH`）：** 与 `syx` 同目录的 `bin/` 为系统插件；`bin/plugins/<id>/plugin.json` 为用户扩展。也识别仓库根下的 `./bin`。`syx run` 与 `syx build --native`（含单文件）在编译前都会登记这些 Host；native 可执行文件启动时经 `syx_rt_init_plugins` → `load_from_env` 再加载运行时回调。

## 编译期 manifest 与锁文件（Phase 49+）

项目 `syx.toml` 可声明插件 manifest：

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

编译期与运行期插件集合不一致 → **`H002`**。索引顺序写入 **`syx-plugin-lock.json`**（与 `syx.lock` 独立）。

## 插件作者（C ABI）

头文件：`crates/syx-host-plugin/include/syx_plugin.h`

插件导出 `syx_plugin_init()`，Host 回调签名为：

```c
typedef void (*syx_host_fn)(const char* in, char* out, size_t out_cap);
```

Loader 从 SYX 栈取出字符串参数，调用 C 函数，再将 `out` 写回结果。

```c
SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void);
```

示例 fixture：`crates/syx-host-plugin/tests/fixtures/echo_plugin`

**P-SURF packed-SYX padReflect 四向第一片（W1100 / ADR-1446）：** `padRowsBeforeReflect` / `padRowsAfterReflect` / `padColsBeforeReflect` / `padColsAfterReflect`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX padConstant 四向第一片（W1099 / ADR-1445）：** `padRowsBeforeConstant` / `padRowsAfterConstant` / `padColsBeforeConstant` / `padColsAfterConstant`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX pad 边框第一片（W1098 / ADR-1444）：** `padRowsBefore` / `padRowsAfter` / `padColsBefore` / `padColsAfter`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 取舍/丢弃第一片（W1097 / ADR-1443）：** `takeRows` / `dropRows` / `takeCols` / `dropCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 标量掩码第二片（W1096 / ADR-1442）：** `lessThanScalar` / `notEqualToScalar` / `lessEqualToScalar` / `greaterEqualToScalar`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 差分加深/标量掩码第一片（W1095 / ADR-1441）：** `maxAbsDiff` / `hamming` / `equalToScalar` / `greaterThanScalar`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 差分/整数损失第一片（W1094 / ADR-1440）：** `absDiff` / `squaredDiff` / `manhattan` / `sumSquaredDiff`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 硬激活加深第一片（W1093 / ADR-1439）：** `leakyRelu` / `hardTanh` / `heaviside` / `relu6`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX clip/threshold 第一片（W1092 / ADR-1438）：** `clampMin` / `clampMax` / `relu` / `threshold`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴前缀第三片（W1091 / ADR-1437）：** `cummaxRows` / `cummaxCols` / `cummin` / `cummax`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴前缀第二片（W1090 / ADR-1436）：** `diffRows` / `diffCols` / `cumminRows` / `cumminCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴前缀第一片（W1089 / ADR-1435）：** `cumsumRows` / `cumsumCols` / `cumprodRows` / `cumprodCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴归约第五片（W1088 / ADR-1434）：** `varRows` / `varCols` / `rangeRows` / `rangeCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴归约第四片（W1087 / ADR-1433）：** `argminCols` / `argmaxRows` / `stdRows` / `stdCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴归约第三片（W1086 / ADR-1432）：** `meanCols` / `prodRows` / `argminRows` / `argmaxCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴归约第二片（W1085 / ADR-1431）：** `meanRows` / `prodCols` / `maxRows` / `minCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 轴归约第一片（W1084 / ADR-1430）：** `sumRows` / `sumCols` / `minRows` / `maxCols`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 掩码逻辑第三片（W1083 / ADR-1429）：** `logicalXor` / `countTrue` / `noneTrue` / `fillWhere`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 掩码逻辑第二片（W1082 / ADR-1428）：** `logicalAnd` / `logicalOr` / `anyTrue` / `allTrue`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 掩码选择/同形填充第一片（W1081 / ADR-1427）：** `zerosLike` / `filledLike` / `choose` / `logicalNot`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素比较第二片（W1080 / ADR-1426）：** `notEqual` / `less` / `lessEqual` / `greaterEqual`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 外积/同形/比较第一片（W1079 / ADR-1425）：** `outer` / `onesLike` / `equal` / `greater`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 前缀累加/差分第一片（W1078 / ADR-1424）：** `cumsum` / `cumprod` / `diff` / `countNonzero`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素 log10/Γ/lerp/dot 第一片（W1077 / ADR-1423）：** `log10` / `tgamma` / `lerp` / `dot`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素熔合/指数对数第一片（W1076 / ADR-1422）：** `mulAdd` / `exp2` / `log2` / `cbrt`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素浮点二元第一片（W1075 / ADR-1421）：** `hypot` / `pow` / `fmod` / `remainder`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素二元极值/符号第一片（W1074 / ADR-1420）：** `maximum` / `minimum` / `copysign` / `reciprocal`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 元素一元第二片（W1073 / ADR-1419）：** `floor` / `ceil` / `round` / `trunc`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** NaN GA。**P-SURF packed-SYX 元素除/标量广播第一片（W1072 / ADR-1418）：** `div` / `addScalar` / `subScalar` / `divScalar`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** `operator /`。**P-SURF packed-SYX 元素一元第一片（W1071 / ADR-1417）：** `sign` / `sqrt` / `squared` / `clamp`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 几何置换第一片（W1070 / ADR-1416）：** `rot180` / `swapRows` / `swapCols` / `roll`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 几何翻转第一片（W1069 / ADR-1415）：** `flipud` / `fliplr` / `rot90` / `antidiagonal`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 结构复制第一片（W1068 / ADR-1414）：** `tile` / `kron` / `tril` / `triu`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 归约/范数第一片（W1067 / ADR-1413）：** `min` / `max` / `argmin` / `argmax` / `frobeniusNorm`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** kron/tile。**P-SURF packed-SYX 形状拼装第一片（W1066 / ADR-1412）：** `reshape` / `flatten` / `hstack` / `vstack` / `fromDiagonal`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** kron/tile。**P-SURF packed-SYX 切片/访问第一片（W1065 / ADR-1411）：** `withAt` / `row` / `col` / `diagonal` / `trace` / `abs`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** `A[i,j]` / **非** 原地 setAt。**P-SURF packed-SYX 元素算术第一片（W1064 / ADR-1410）：** `+` / `-` / `negated` / `hadamard` / `filled` / `mean`。**非** 产品 Mat GA / **非** E-00 / **非** prelude / **非** faer add/sub host。**P-SURF packed-SYX 形状通用化第一片（W1063 / ADR-1409）：** 任意形 `eye` / `ones` / `sum` / `transposed` / `copy` + `scale`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF packed-SYX 便利/stats 第一片（W1062 / ADR-1408）：** `at` / `transposed` / `copy` / `eye` / `ones` / `sum`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF solve/packed/shape 方法族第一片（W1061 / ADR-1407）：** `Mat.from` / `packed()` / `rows`/`cols` / `solve`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF zeros 语言面第一片（W1060 / ADR-1406）：** `Mat.zeros(rows, cols)` 得到独立零矩阵。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF 独立新 Mat 第一片（W1059 / ADR-1404）：** `let c = a * b` 得到独立 `Mat` 且左右 packed 不改。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-FAER cdylib packed zeros 第一片（W1059 / ADR-1405）：** `__syx_plugin_faer_cdylib_zeros`。**非** Mat 语言面 / **非** faer 产品 GA。**P-SURF operator 返回值第一片（W1058 / ADR-1403）：** `let c = a * b` 得到新矩阵且右操作数不改。**非** 产品 Mat GA / **非** E-00 / **非** prelude。**P-SURF Mat 独立包落地第一片（W1057 / ADR-1402）：** `import Linalg` / `class Mat` HANDLE wrap + `operator func *`。**非** 产品 Mat GA / **非** E-00 / **非** prelude。
**P-FAER cdylib F64LE packed solve 第一片（W1056 / ADR-1401）：** `__syx_plugin_faer_cdylib_solve`。**非** 句柄 GA / **非** faer 产品 GA / **非** 产品 `Mat`。
**P-FAER HANDLE×cdylib drop 第一片（W1055 / ADR-1400）：** `__syx_plugin_faer_cdylib_hdrop`。**非** 句柄 GA / **非** faer 产品 GA / **非** `Mat`。

**P-SURF 书面 Mat 约束第一片（W1054 / ADR-1399）：** HANDLE 包一层 / `operator func` / 禁并入 `math.sy`。**非** `Mat` 落地 / **非** 产品 GA。

**P-FAER HANDLE×cdylib to/from 第一片（W1053 / ADR-1398）：** `__syx_plugin_faer_cdylib_hfrom` / `_hto`。**非** faer 产品 GA / **非** `Mat` / **非** 句柄 GA。

**P-FAER HANDLE×cdylib solve 第一片（W1052 / ADR-1397）：** `__syx_plugin_faer_cdylib_hsolve`。**非** faer 产品 GA / **非** `Mat` / **非** 句柄 GA。

**P-ABI-v2 句柄 borrow 第一片（W1051 / ADR-1396）：** alloc `borrow_fn`/`pin_fn` + `__syx_plugin_faer_cdylib_hmul`。**非** 句柄 GA / **非** faer 产品 GA / **非** `Mat`。

**P-FAER cdylib 装载第一片（W1050 / ADR-1395）：** 站外 `plugins/faerlin` 真实 dlopen + v2 F64LE `__syx_plugin_faer_cdylib_mul`。**非** HANDLE×cdylib / **非** faer 产品 GA / **非** `Mat`。

**P-FAER solve 第一片（W1049 / ADR-1394）：** in-process `__syx_plugin_faer_solve`（A 方阵 × b；内部 faer dense LU）。**非** faer 产品 GA / **非** `Mat` / **非** ABI v2 完成。

**P-FAER 施工第一片（W1048 / ADR-1393）：** in-process `__syx_plugin_faer_from` / `__syx_plugin_faer_zeros` / `__syx_plugin_faer_mul` / `__syx_plugin_faer_to`；句柄带 nrows/ncols；内部 faer `Mat<f64>` 乘。**非** faer 产品 GA / **非** `Mat` / **非** ABI v2 完成。

**ABI v2 句柄表第一片（W1047 / ADR-1392）：** Host 进程内 opaque `u64 → Vec<f64>`；`SYX_PLUGIN_BUF_HANDLE` kind=2（8-byte LE u64，**非** f64 伪装）。烟测 `__syx_plugin_v2probe_pin` / `__syx_plugin_v2probe_sumh` / `__syx_plugin_v2probe_drop`。句柄不进 SYX GC；未知/二次 drop 失败。**非** 句柄 GC GA / faer 产品 GA / 四后端插件。

**ABI v2 加深第一片（W1046 / ADR-1391）：** Host 接受 `api_version` 1、2 与 3。v1/v2 路径不变。v2m 为多槽 `syx_plugin_iov` + host `realloc_fn`（初始 cap 8 可增长；失败非静默截断）。烟测 `__syx_plugin_v2probe_dot` / `__syx_plugin_v2probe_grow`。**非** 句柄表 GA / faer / 四后端插件。

## 安全与验签（Phase 50–51）

| 变量 | 说明 |
|------|------|
| `SYX_HOST_PLUGIN_VERIFY=1` | 强制 ed25519 验签 |
| `SYX_HOST_PLUGINS=disabled` | 禁用加载（企业策略，Phase 51+） |

插件与 `syx` **同进程**，等价于运行本地原生代码；仅注册 Host 回调，**无**任意 syscall API。恶意插件须靠签名与路径白名单管控。

## 三后端差异

| 后端 | 插件位置 |
|------|----------|
| VM（`syx run`） | CLI 进程 dlopen |
| Native | 与 `app` 同进程，`syx_rt_init_plugins` |
| WASM | **宿主进程**加载；guest `.wasm` 内无 dlopen |

## Embed 集成（Phase 51+）

宿主应用可通过 Embed API 设置插件路径：

```c
syx_embed_v2_set_plugin_paths(
    "/path/to/lib1.so:/path/to/lib2.so"
);
```

与 [Embed API](../../../docs/9_UsageGuide.md#embed-apiphase-1415) 沙箱策略联动。若加密在宿主侧完成、SYX 只做编排，Embed + HTTP/文件 仍可能比插件更简单。

## 与静态 stdlib 扩展的关系

| 方式 | 适用 |
|------|------|
| 合入 `HostFnId` + `lib/*.sy` | 官方核心能力、WASM 无宿主场景 |
| dlopen 插件 | 第三方 SDK、私有 crypto、快速迭代 |
| Embed 外围集成 | 宿主掌控全部原生逻辑 |

维护者静态扩展清单见 [plan/topics/hostfn-extension.md](../../../plan/topics/hostfn-extension.md)。稠密线性代数 **不要** 加静态 `HostFnId`；旁路章程（停车）：[plugin ABI v2 / faer](../../../plan/topics/plugin-abi-v2-linalg.md)（ADR-1389）。

## 完整示例：自定义 echo 插件

### 1. 写 C 插件

`echo_plugin.c`：

```c
#include "syx_plugin.h"
#include <string.h>
#include <stdio.h>

static void echo_host(const char* in, char* out, size_t out_cap) {
    snprintf(out, out_cap, "echo: %s", in);
}

SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void) {
    static syx_plugin_info info = {
        .id = "echo",
        .api_version = 1,
        .library = "libecho_plugin.so",
        .hosts = (syx_host_entry[]){
            {
                .name = "__syx_plugin_echo_echo",
                .fn = echo_host,
                .return_type = "String",
                .params = (const char*[]){"data: String"},
                .params_count = 1
            }
        },
        .hosts_count = 1
    };
    return &info;
}
```

### 2. 编译

```bash
clang -shared -fPIC -o libecho_plugin.so echo_plugin.c \
    -I crates/syx-host-plugin/include
```

### 3. 写 `plugin.json`

```json
{
  "id": "echo",
  "api_version": 1,
  "library": "libecho_plugin.so",
  "hosts": [
    {
      "name": "__syx_plugin_echo_echo",
      "return_type": "String",
      "params": ["data: String"]
    }
  ],
  "capabilities": ["example"],
  "min_syx_version": "27.0"
}
```

### 4. SYX 端调用

`syx.toml`：

```toml
[plugins]
paths = ["./echo_plugin.json"]
```

`main.sy`：

```syx
func main() {
    let msg = __syx_plugin_echo_echo("hello")
    print(msg)                              // "echo: hello"
}
```

### 5. 运行

```bash
export SYX_HOST_PLUGIN_PATH=./libecho_plugin.so
syx run .
```

## 诊断码

| 代码 | 含义 | 修复 |
|------|------|------|
| H001 | 运行期插件未加载或 Host 缺失 | 检查 `SYX_HOST_PLUGIN_PATH` |
| H002 | 编译期 manifest 与运行期/锁文件不一致 | 重新 `syx install` |
| H003 | 插件 ABI 版本不匹配 | 升级插件 / 编译器 |
| H004 | 插件 `min_syx_version` 不满足 | 升级编译器 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| 静态 stdlib | **已实现** |
| 动态插件（dlopen） | **已实现**（Phase 49+，opt-in） |
| 验签 | **部分实现**（ed25519） |
| Embed API | **已实现**（`syx_embed_v1` / `syx_embed_v2`） |
| WASM 插件 | **不适用**（宿主进程加载） |

## 进一步阅读

- [完整使用指南 § 运行时扩展](../../../docs/9_UsageGuide.md#运行时-host-插件opt-in)
- [docs/adr/architecture-decisions.md](../../../docs/adr/architecture-decisions.md) — ADR-071 / ADR-072 / ADR-073
- [plan/topics/hostfn-extension.md](../../../plan/topics/hostfn-extension.md) — 维护者扩展清单

---

**上一章** ← [运算符 vs Core Host](32-operators-vs-host.md)

**参见** → [docs/9_UsageGuide.md](../../../docs/9_UsageGuide.md) · [第 29 章 模块与包](29-modules-and-packages.md) · [第 32 章 运算符 vs Core Host](32-operators-vs-host.md) · [docs/adr/architecture-decisions.md](../../../docs/adr/architecture-decisions.md)
