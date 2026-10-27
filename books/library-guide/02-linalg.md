# Linalg (Mat)

**Implementation status:** the packed-SYX outer skin is **wired up**; core binop / unary operators are wired to faer (`*` / `+` / `-` / `hadamard` / `negated` / `scale`); E-00 **product Mat execution ADR** delivered; E-00 **product Mat GA narrowed acceptance** delivered; E-00 **product Mat GA narrowed-surface claim** delivered (a narrowed claim ≠ full product Mat GA); P-SURF **Mat narrowed-surface flip charter** delivered; P-SURF **Mat narrowed-surface soft-close** delivered (**`P-SURF` → `[x]`**; thin completion ≠ full product Mat GA); P-FAER **narrowed-surface flip charter** delivered; P-FAER **narrowed-surface soft-close** delivered (**`P-FAER` → `[x]`**; thin completion ≠ faer product GA); P-ABI-v2 **narrowed-surface flip charter** delivered; P-ABI-v2 **narrowed-surface soft-close** delivered (**`P-ABI-v2` → `[x]`**; thin completion ≠ ABI v2 product GA); E-00 **full product Mat GA charter** delivered (a charter ≠ a claim ≠ E-00 `[x]`); E-00 **full product Mat GA claim** delivered (twin flags `full_claimed` ∧ still `ga_not_claimed`; a claim ≠ soft-close ≠ a flipped honesty ≠ E-00 `[x]`); E-00 **full product Mat GA soft-close** delivered (evidence track closed out; still `ga_not_claimed`; soft-close ≠ a flipped honesty ≠ E-00 `[x]`); E-00 **full product Mat GA honesty flip** delivered (new inventory `ga_not_claimed=false`; historical flags are not rewritten; honesty flip ≠ E-00 `[x]` ≠ product ranking); E-00 **product ranking execution ADR** delivered (still `ranking_not_claimed`; an execution ADR ≠ a chosen ranking ≠ E-00 `[x]`); E-00 **chosen product ranking SSOT** delivered (WS→gRPC→more DBs; SynUI/Farm deferred; still `ranking_not_claimed`; a chosen SSOT ≠ soft-close ≠ a flipped honesty ≠ E-00 `[x]`); E-00 **product ranking soft-close** delivered (evidence track closed out; still `ranking_not_claimed`; soft-close ≠ a flipped honesty ≠ E-00 `[x]`); E-00 **product ranking honesty flip** delivered (new inventory `ranking_not_claimed=false`; historical flags are not rewritten; honesty flip ≠ E-00 `[x]`); E-00 **row check** delivered (`e00_roadmap_checked=true`; `| E-00 |` **`[x]`**; **1/6**; a row check ≠ starting E-01); E-01 **execution ADR** delivered (authorizes later scaffold / real WS; still `e01_still_frozen`; an execution ADR ≠ scaffold ≠ real WS GA ≠ E-01 `[x]`); E-01 **scaffold** delivered (pins the existing facade/mock/`SYX_WS_REAL` thin entry; still `e01_still_frozen`; scaffold ≠ real WS GA ≠ E-01 `[x]`); E-01 **real WS construction** delivered (narrow `ws: / ` RFC6455; still `e01_still_frozen`; construction ≠ product GA ≠ E-01 `[x]`); E-01 **negative cases** delivered (still `e01_negative_tests_not_claimed`; negative cases ≠ the full set ≠ E-01 `[x]`); **not** product WebSocket GA; **not** the CORE prelude; the `E-00` row is already `[x]`; the `E-01` row is still `[ ]`; the `P-ABI-v2` / `P-FAER` / `P-SURF` narrowed rows are already `[x]` (the bypass product GA honesty is still not flipped).

```syx
import Linalg

func main() {
    let a = Mat.from([1.0, 2.0, 3.0, 4.0], rows: 2, cols: 2)
    let c = a * Mat.eye(2)
    print(c.packed()[0])
}
```

Examples: [examples/linalg/matmul.sy](../../examples/linalg/matmul.sy) · [shape.sy](../../examples/linalg/shape.sy) · [pad.sy](../../examples/linalg/pad.sy)

## Layout

`Mat` has three public fields: `handle` (faer handle), `rows`, `cols`. Elements live on the Host side, row-major `Float64`. User code reads through `packed()` / `at`; factories and operators return a **new** `Mat`, and the packed data of the left operand is not mutated.

| Factory | Description |
|------|------|
| `Mat.from(_:rows:cols:)` | Construct from packed data |
| `Mat.zeros(rows:cols:)` | Standalone zero matrix |
| `Mat.ones(rows:cols:)` | All ones |
| `Mat.filled(rows:cols:_:)` | Fill with a constant |
| `Mat.eye(_:)` | Identity matrix |
| `Mat.fromDiagonal(_:)` | Embed along the diagonal |

Do **not** write `Mat([[1.0, 2.0], [3.0, 4.0]])` (a nested literal is not a constructor of this slice).

## Multiplication and Solving

- `a * b`: matrix multiplication (`operator func *` → `__syx_plugin_faer_mul`), **not** Hadamard.
- `a + b` / `a - b`: same-shape add and subtract (→ `__syx_plugin_faer_add` / `_sub`).
- Element-wise multiply: `hadamard` (→ `__syx_plugin_faer_hadamard`).
- Linear solve: `a.solve(b)` (square matrix × right-hand side; faer dense LU internally).
- `negated` / `scale`: faer compose (zeros+`faer_sub` / filled+`faer_hadamard`; no dedicated host).
- `div` / `addScalar` and friends still go through packed loops.

## Shape and Slicing

`reshape` / `flatten` / `hstack` / `vstack` / `row` / `col` / `diagonal` / `takeRows` / `dropRows` / `takeCols` / `dropCols` / `transposed` / `copy`.

There is **no** `A[i, j]` and **no** in-place `setAt`. To change one cell, use `withAt(row, col:, _)` to obtain a new matrix.

## Padding

Zero / constant / mirror padding around the border. Pinned to `n = 1` with source dimensions `≥ 2`. Mirror aligns with numpy's `mode='reflect'` (edge elements are **not** duplicated; replicate/circular are **not** delivered).

For `a = [[1, 2], [3, 4]]` (row-major packed `1 2 3 4`):

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

Circular (numpy `wrap`) needs L≥3 to be distinguishable from reflect. Rows use the 3×2 `b=[[1,2],[3,4],[5,6]]`, columns use the 2×3 `c=[[1,2,3],[4,5,6]]`:

| API | packed |
|-----|--------|
| `padRowsBeforeCircular(1)` | `5 6 1 2 3 4 5 6` |
| `padRowsAfterCircular(1)` | `1 2 3 4 5 6 1 2` |
| `padColsBeforeCircular(1)` | `3 1 2 3 6 4 5 6` |
| `padColsAfterCircular(1)` | `1 2 3 1 4 5 6 4` |

`a` itself is still `1 2 3 4`. Zero pad / constant pad use the same-direction APIs: `padRowsBefore`, `padRowsBeforeConstant(_:_:)` and so on. Replicate aligns with numpy's `mode='edge'`; circular aligns with `mode='wrap'`.

Implementation constraints (when writing extensions): use `__syx_plugin_faer_to` in method bodies, not `self.packed()`; do not put a MathFloat HostFn in an `ArraySet` right-hand side (write copies as `0.0 + p[k]`); prefer subscript lvalues of the form `q[i * nc + c]` / `q[k + skip]` that carry an operator.

## Reductions and Masks

`sum` / `mean` / `min` / `max` / `argmin` / `argmax` / `frobeniusNorm`; along an axis `sumRows` / `meanCols` and so on; comparisons and masks `equal` / `greaterThanScalar` / `choose` / `logicalAnd` and so on. Same-shape losses and distances: `manhattan` / `sumSquaredDiff` / `mae` / `mse` / `euclidean` / `rmse` (relative to `ones`: mae `1.5`, mse `3.5`, euclidean `√14`, rmse `√3.5`); cosine: `cosineSimilarity` / `cosineDistance` (self-similarity `1`/`0`; against `ones` `10/√120` / `1-10/√120`). Scaling: `normalize` (÷Frobenius), `center` (subtract mean), `standardize` (z-score, ddof=0), `minMaxScale` (`(x-min)/(max-min)`). Natural exponential/logarithm and soft activations: `exp` / `log` / `softplus` (`log1p(exp(x))`) / `sigmoid` (`1/(1+exp(-x))`); base 2/10 variants too: `exp2` / `log2` / `log10`. Trigonometric and special functions: `sin` / `cos` / `erf` / `erfc`. Smooth activations: `elu(_:)`, `gelu`, `tanh`, `swish` (`x·sigmoid(x)`; `silu`≡`swish`). Advanced activations: `selu` (fixed Klambauer λ/α), `celu(_:)`, `mish` (`x·tanh(softplus(x))`), `softsign`. Shrinkage/threshold activations: `softshrink(_:)`, `hardshrink(_:)`, `prelu(_:)` (same as `leakyRelu`), `hardSigmoid` (`clamp(0.2x+0.5,0,1)`). Median/quantile: `median` (average of the two middle values for even length), `quantile(_:)` (linear interpolation at `q*(n-1)` on a sorted copy), `medianRows` (**nr×1**), `medianCols` (**1×nc**), `quantileRows(_:)` (**nr×1**), `quantileCols(_:)` (**1×nc**; same interpolation as `quantile` after sorting the axis window). Gated linear: `glu` (halves the last dimension: left half × `sigmoid(right half)` → **nr×(nc/2)**; requires even `cols`).

## Frozen

- `Mat` requires `import Linalg`; the prelude has **no** `Mat`
- Not merged into `packages/core/math.sy`
- Do not claim full product Mat GA / faer product GA / ABI v2 product GA / P-ABI-v2 `[x]` / E-00 `[x]` (the `P-SURF` / `P-FAER` narrowed rows are already `[x]`)
- Do not invent `mat_ga` / `p_surf_ga` / `faer_ga` / `plugin_abi_v2_ga`
- HostFn count stays **629** (no new plugin host)

Manifests and ADRs: the living SSOT for the P-ABI-v2 / P-FAER / P-SURF milestone chain is [`plan/topics/plugin-abi-v2-linalg.md`](../../plan/topics/plugin-abi-v2-linalg.md); architecture decisions live in [`docs/adr/architecture-decisions.md`](../../docs/adr/architecture-decisions.md). Per-slice `docs/lang/qs1-*` charters referenced by older notes have been retired — follow the plan topic and ADR series instead.
