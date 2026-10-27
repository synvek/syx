# Linalg（Mat）

**实现状态：** packed-SYX 外表皮 **已接线**；核心 binop / unary 已接 faer（`*`/`+`/`-`/`hadamard`/`negated`/`scale`）；E-00 **产品 Mat 执行 ADR** 已交；E-00 **产品 Mat GA 收窄验收** 已交；E-00 **产品 Mat GA 收窄面宣称** 已交（；收窄宣称 ≠ 完整产品 Mat GA）；P-SURF **Mat 收窄面翻行章程** 已交；P-SURF **Mat 收窄面 soft-close** 已交（；**`P-SURF` → `[x]`**；薄完成 ≠ 完整产品 Mat GA）；P-FAER **收窄面翻行章程** 已交；P-FAER **收窄面 soft-close** 已交（；**`P-FAER` → `[x]`**；薄完成 ≠ faer 产品 GA）；P-ABI-v2 **收窄面翻行章程** 已交；P-ABI-v2 **收窄面 soft-close** 已交（；**`P-ABI-v2` → `[x]`**；薄完成 ≠ ABI v2 产品 GA）；E-00 **完整产品 Mat GA 章程** 已交（；章程 ≠ 宣称 ≠ E-00 `[x]`）；E-00 **完整产品 Mat GA 宣称** 已交（；双旗 `full_claimed` ∧ 仍 `ga_not_claimed`；宣称 ≠ soft-close ≠ 翻 honesty ≠ E-00 `[x]`）；E-00 **完整产品 Mat GA soft-close** 已交（；证据轨收口；仍 `ga_not_claimed`；soft-close ≠ 翻 honesty ≠ E-00 `[x]`）；E-00 **完整产品 Mat GA honesty flip** 已交（；新 inventory `ga_not_claimed=false`；历史旗不改写；honesty flip ≠ E-00 `[x]` ≠ 产品排序）；E-00 **产品排序执行 ADR** 已交（；仍 `ranking_not_claimed`；执行 ADR ≠ 选定排序 ≠ E-00 `[x]`）；E-00 **选定产品排序 SSOT** 已交（；WS→gRPC→更多 DB；SynUI/Farm 延后；仍 `ranking_not_claimed`；选定 SSOT ≠ soft-close ≠ 翻 honesty ≠ E-00 `[x]`）；E-00 **产品排序 soft-close** 已交（；证据轨收口；仍 `ranking_not_claimed`；soft-close ≠ 翻 honesty ≠ E-00 `[x]`）；E-00 **产品排序 honesty flip** 已交（；新 inventory `ranking_not_claimed=false`；历史旗不改写；honesty flip ≠ E-00 `[x]`）；E-00 **行勾选** 已交（；`e00_roadmap_checked=true`；`| E-00 |` **`[x]`**； **1/6**；行勾选 ≠ E-01 开工）；E-01 **执行 ADR** 已交（；授权后续 scaffold/真 WS；仍 `e01_still_frozen`；执行 ADR ≠ scaffold ≠ 真 WS GA ≠ E-01 `[x]`）；E-01 **scaffold** 已交（；钉既有 facade/mock/`SYX_WS_REAL` 薄入口；仍 `e01_still_frozen`；scaffold ≠ 真 WS GA ≠ E-01 `[x]`）；E-01 **真 WS 施工** 已交（；窄 `ws: / ` RFC6455；仍 `e01_still_frozen`；施工 ≠ 产品 GA ≠ E-01 `[x]`）；E-01 **负例** 已交（；仍 `e01_negative_tests_not_claimed`；负例≠ 全集 ≠ E-01 `[x]`）；**不是** 产品 WebSocket GA；**不是** CORE prelude；`E-00` 行已 `[x]`；`E-01` 行仍 `[ ]`；`P-ABI-v2` / `P-FAER` / `P-SURF` 收窄行已 `[x]`（旁路产品 GA honesty 仍未翻）。

```syx
import Linalg

func main() {
    let a = Mat.from([1.0, 2.0, 3.0, 4.0], rows: 2, cols: 2)
    let c = a * Mat.eye(2)
    print(c.packed()[0])
}
```

范例：[examples/linalg/matmul.sy](../../examples/linalg/matmul.sy) · [shape.sy](../../examples/linalg/shape.sy) · [pad.sy](../../examples/linalg/pad.sy)

## 布局

`Mat` 三个公开字段：`handle`（faer 句柄）、`rows`、`cols`。元素在 Host 侧，行主序 `Float64`。用户代码用 `packed()` / `at` 读；工厂与算子返回 **新** `Mat`，左操作数 packed 不改。

| 工厂 | 说明 |
|------|------|
| `Mat.from(_:rows:cols:)` | 从 packed 构造 |
| `Mat.zeros(rows:cols:)` | 独立零矩阵 |
| `Mat.ones(rows:cols:)` | 全 1 |
| `Mat.filled(rows:cols:_:)` | 填常数 |
| `Mat.eye(_:)` | 单位阵 |
| `Mat.fromDiagonal(_:)` | 对角嵌入 |

**不要**写 `Mat([[1.0, 2.0], [3.0, 4.0]])`（嵌套字面量不是本切片构造器）。

## 乘与求解

- `a * b`：矩阵乘（`operator func *` → `__syx_plugin_faer_mul`），**不是** Hadamard。
- `a + b` / `a - b`：同形加减（→ `__syx_plugin_faer_add` / `_sub`；）。
- 逐元乘：`hadamard`（→ `__syx_plugin_faer_hadamard`；）。
- 线性求解：`a.solve(b)`（方阵 × 右端；内部 faer dense LU）。
- `negated` / `scale`：faer compose（zeros+`faer_sub` / filled+`faer_hadamard`；无专用 host）。
- `div` / `addScalar` 等仍走 packed 循环。

## 形状与切片

`reshape` / `flatten` / `hstack` / `vstack` / `row` / `col` / `diagonal` / `takeRows` / `dropRows` / `takeCols` / `dropCols` / `transposed` / `copy`。

**没有** `A[i, j]`，**没有** 原地 `setAt`。改一个格子用 `withAt(row, col:, _)` 得到新矩阵。

## Pad

边框补零 / 常数 / 镜像。钉 `n = 1`、源维 `≥ 2`。镜像对齐 numpy `mode='reflect'`（**不**重复边元；**不**交 replicate/circular）。

对 `a = [[1, 2], [3, 4]]`（行主序 packed `1 2 3 4`）：

| API | packed |
|-----|--------|
| `padRowsBeforeReflect(1)` | `3 4 1 2 3 4` |
| `padRowsAfterReflect(1)` | `1 2 3 4 1 2` |
| `padColsBeforeReflect(1)` | `2 1 2 4 3 4` |
| `padColsAfterReflect(1)` | `1 2 1 3 4 3` |
| `padRowsBeforeReplicate(1)` | `1 2 1 2 3 4` |
| `padRowsAfterReplicate(1)` | `1 2 3 4 3 4` |
| `padColsBeforeReplicate(1)` | `1 1 2 3 3 4` |
| `padColsAfterReplicate(1)` | `1 2 2 3 4 4` |

Circular（numpy `wrap`）需 L≥3 才能与 reflect 区分。行向用 3×2 `b=[[1,2],[3,4],[5,6]]`，列向用 2×3 `c=[[1,2,3],[4,5,6]]`：

| API | packed |
|-----|--------|
| `padRowsBeforeCircular(1)` | `5 6 1 2 3 4 5 6` |
| `padRowsAfterCircular(1)` | `1 2 3 4 5 6 1 2` |
| `padColsBeforeCircular(1)` | `3 1 2 3 6 4 5 6` |
| `padColsAfterCircular(1)` | `1 2 3 1 4 5 6 4` |

`a` 本身仍是 `1 2 3 4`。零 pad / 常数 pad 同向 API：`padRowsBefore`、`padRowsBeforeConstant(_:_:)` 等。Replicate 对齐 numpy `mode='edge'`；Circular 对齐 `mode='wrap'`。

实现约束（写扩展时）：方法体用 `__syx_plugin_faer_to`，不要 `self.packed()`；`ArraySet` 右值不要塞 MathFloat HostFn（拷贝写成 `0.0 + p[k]`）；下标左值优先 `q[i * nc + c]` / `q[k + skip]` 这种带运算符的形式。

## 归约与掩码

`sum` / `mean` / `min` / `max` / `argmin` / `argmax` / `frobeniusNorm`；轴上 `sumRows` / `meanCols` 等；比较与掩码 `equal` / `greaterThanScalar` / `choose` / `logicalAnd` 等。同形损失/距离：`manhattan` / `sumSquaredDiff` / `mae` / `mse` / `euclidean` / `rmse`（相对 `ones`：mae `1.5`、mse `3.5`、euclidean `√14`、rmse `√3.5`）；余弦：`cosineSimilarity` / `cosineDistance`（自相似 `1`/`0`；对 `ones` 为 `10/√120` / `1-10/√120`）。缩放：`normalize`（÷Frobenius）、`center`（减均值）、`standardize`（z-score，ddof=0）、`minMaxScale`（`(x-min)/(max-min)`）。自然指数/对数与软激活：`exp` / `log` / `softplus`（`log1p(exp(x))`）/ `sigmoid`（`1/(1+exp(-x))`）；另有基 2/10：`exp2` / `log2` / `log10`。三角与特殊函数：`sin` / `cos` / `erf` / `erfc`。平滑激活：`elu(_:)`、`gelu`、`tanh`、`swish`（`x·sigmoid(x)`；`silu`≡`swish`）。高级激活：`selu`（固定 Klambauer λ/α）、`celu(_:)`、`mish`（`x·tanh(softplus(x))`）、`softsign`。收缩/阈值激活：`softshrink(_:)`、`hardshrink(_:)`、`prelu(_:)`（同 `leakyRelu`）、`hardSigmoid`（`clamp(0.2x+0.5,0,1)`）。中位数/分位：`median`（偶数长度两中位平均）、`quantile(_:)`（排序副本上 `q*(n-1)` 线性插值）、`medianRows`（**nr×1**）、`medianCols`（**1×nc**）、`quantileRows(_:)`（**nr×1**）、`quantileCols(_:)`（**1×nc**；轴窗排序后同 `quantile` 插值）。门控线性：`glu`（末维半切：左半 × `sigmoid(右半)` → **nr×(nc/2)**；偶数 `cols`）。

## Frozen

- `import Linalg` 才能用 `Mat`；prelude **无** `Mat`
- 不并入 `packages/core/math.sy`
- 不宣称完整产品 Mat GA / faer 产品 GA / ABI v2 产品 GA / P-ABI-v2 `[x]` / E-00 `[x]`（`P-SURF` / `P-FAER` 收窄行已 `[x]`）
- 不发明 `mat_ga` / `p_surf_ga` / `faer_ga` / `plugin_abi_v2_ga`
- HostFn 计数仍 **629**（无新 plugin host）

清单与 ADR：P-ABI-v2 翻行章程：[docs/lang/qs1-p-abi-v2-narrowed-flip-charter-first-slice.md](../../docs/lang/qs1-p-abi-v2-narrowed-flip-charter-first-slice.md) · ；P-FAER soft-close：[docs/lang/qs1-p-faer-narrowed-soft-close-first-slice.md](../../docs/lang/qs1-p-faer-narrowed-soft-close-first-slice.md) · ；P-FAER 翻行章程：[docs/lang/qs1-p-faer-narrowed-flip-charter-first-slice.md](../../docs/lang/qs1-p-faer-narrowed-flip-charter-first-slice.md) · ；P-SURF soft-close：[docs/lang/qs1-p-surf-mat-narrowed-soft-close-first-slice.md](../../docs/lang/qs1-p-surf-mat-narrowed-soft-close-first-slice.md) · ；翻行章程：[docs/lang/qs1-p-surf-mat-narrowed-flip-charter-first-slice.md](../../docs/lang/qs1-p-surf-mat-narrowed-flip-charter-first-slice.md) · ；收窄面宣称：[docs/lang/qs1-e00-product-mat-narrowed-claim-first-slice.md](../../docs/lang/qs1-e00-product-mat-narrowed-claim-first-slice.md) · ；scale faer：[docs/lang/qs1-p-surf-mat-scale-faer-first-slice.md](../../docs/lang/qs1-p-surf-mat-scale-faer-first-slice.md) · ；negated faer：[docs/lang/qs1-p-surf-mat-negated-faer-first-slice.md](../../docs/lang/qs1-p-surf-mat-negated-faer-first-slice.md) · ；收窄验收：[docs/lang/qs1-e00-product-mat-narrowed-acceptance-first-slice.md](../../docs/lang/qs1-e00-product-mat-narrowed-acceptance-first-slice.md) · ；执行 ADR：[docs/lang/qs1-e00-product-mat-execute-adr-first-slice.md](../../docs/lang/qs1-e00-product-mat-execute-adr-first-slice.md) · ；hadamard faer 地板：[qs1-p-surf-mat-hadamard-faer-first-slice.md](../../docs/lang/qs1-p-surf-mat-hadamard-faer-first-slice.md) · ；Mat +/- faer：[qs1-p-surf-mat-binop-faer-first-slice.md](../../docs/lang/qs1-p-surf-mat-binop-faer-first-slice.md) · ；glu：[qs1-p-surf-packed-glu-first-slice.md](../../docs/lang/qs1-p-surf-packed-glu-first-slice.md) · ；quantile-axis：[qs1-p-surf-packed-quantile-axis-first-slice.md](../../docs/lang/qs1-p-surf-packed-quantile-axis-first-slice.md) · ；median/quantile：[qs1-p-surf-packed-median-quantile-first-slice.md](../../docs/lang/qs1-p-surf-packed-median-quantile-first-slice.md) · ；act-shrink：[qs1-p-surf-packed-act-shrink-first-slice.md](../../docs/lang/qs1-p-surf-packed-act-shrink-first-slice.md) · ；act-advanced：[qs1-p-surf-packed-act-advanced-first-slice.md](../../docs/lang/qs1-p-surf-packed-act-advanced-first-slice.md) · ；act-smooth：[qs1-p-surf-packed-act-smooth-first-slice.md](../../docs/lang/qs1-p-surf-packed-act-smooth-first-slice.md) · ；trig：[qs1-p-surf-packed-trig-special-first-slice.md](../../docs/lang/qs1-p-surf-packed-trig-special-first-slice.md) ·。
