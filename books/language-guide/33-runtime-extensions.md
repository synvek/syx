# Runtime Extensions (Dynamic Host Plugins)

**Implementation status:** static `HostFnId` registration **Implemented**; `dlopen` plugins opt-in **Partially implemented** (Phase 49–52); `plugin.json` + lock file **Partially implemented** (Phase 49+); `ed25519` signature verification **Partially implemented** (Phase 50–51); Embed API path **Implemented** (Phase 51+)

SYX's special capabilities (I/O, JSON, hashing, SIMD, …) ultimately enter the runtime through **Host functions**. Maintainers may register `HostFnId` statically inside the compiler repository; **application developers** can also add Hosts at runtime through **dlopen plugins**, without forking the compiler.

For the user-facing narrative of embedded packages (`import Linalg` / `Os` / `Log` / `IO` …) see the [Standard Library Guide](../library-guide/00-preface.md); for Linalg examples see [examples/linalg/](../../examples/linalg/).

## Capability Layers

```
SYX source (import / __syx_plugin_*)
    → Semantic analysis + MIR: MirOp::CallHost(n)
    → call_host(n) → static Host table or plugin slot
    → C/Rust callbacks inside the plugin .so
```

| Layer | Description |
|----|------|
| `lib/*.sy` | User-visible API |
| Host registry | name → index → return type |
| Static `hosts.rs` / plugin `.so` | Actually operates on the VM/MIR stack |

> **Pure SYX code** cannot call arbitrary C libraries out of thin air; a Host must already exist or be registered through a plugin.

## Enabling Plugins (Runtime)

```bash
export SYX_HOST_PLUGIN_PATH=/path/to/libecho_plugin.so:/path/to/libmycrypto.so
syx run app.sy
```

Native executables load them during the `syx_rt_init_plugins()` phase (after `syx build --native`, `./app` works the same way).

> **Multi-path separator:** `:` on Unix, `;` on Windows.

Calling a plugin Host from the SYX side (namespace `__syx_plugin_{id}_{fn}`):

```syx
func main() {
    let msg = __syx_plugin_echo_echo("hello")
    print(msg)
}
```

**Complete examples:** [examples/plugin_demo](../../examples/plugin_demo/) (C cdylib SHA-256 + `syx run`); [examples/rust_plugin_demo](../../examples/rust_plugin_demo/) (Rust `cdylib` + the same C ABI); [examples/linalg](../../examples/linalg/) (the system faerlin: `bin/libfaerlin_plugin.*` + `bin/faerlin/plugin.json`; user extensions go in `bin/plugins/`).

**Install-prefix auto-discovery (no `SYX_HOST_PLUGIN_PATH` needed):** `bin/` next to `syx` holds system plugins; `bin/plugins/<id>/plugin.json` holds user extensions. The repository root `./bin` is also recognized. Both `syx run` and `syx build --native` (including single-file builds) register these Hosts before compilation; when a native executable starts, `syx_rt_init_plugins` → `load_from_env` loads the runtime callbacks again.

## Compile-Time Manifests and Lock Files (Phase 49+)

A project `syx.toml` may declare plugin manifests:

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

If the compile-time and runtime plugin sets differ → **`H002`**. The index order is written into **`syx-plugin-lock.json`** (independent of `syx.lock`).

## Plugin Authors (C ABI)

Header: `crates/syx-host-plugin/include/syx_plugin.h`

A plugin exports `syx_plugin_init()`, and the Host callback signature is:

```c
typedef void (*syx_host_fn)(const char* in, char* out, size_t out_cap);
```

The loader takes string arguments from the SYX stack, calls the C function, and writes `out` back as the result.

```c
SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void);
```

Example fixture: `crates/syx-host-plugin/tests/fixtures/echo_plugin`

**P-SURF packed-SYX padReflect four-direction first slice (W1100 / ADR-1446):** `padRowsBeforeReflect` / `padRowsAfterReflect` / `padColsBeforeReflect` / `padColsAfterReflect`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX padConstant four-direction first slice (W1099 / ADR-1445):** `padRowsBeforeConstant` / `padRowsAfterConstant` / `padColsBeforeConstant` / `padColsAfterConstant`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX pad border first slice (W1098 / ADR-1444):** `padRowsBefore` / `padRowsAfter` / `padColsBefore` / `padColsAfter`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX take/drop first slice (W1097 / ADR-1443):** `takeRows` / `dropRows` / `takeCols` / `dropCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX scalar mask second slice (W1096 / ADR-1442):** `lessThanScalar` / `notEqualToScalar` / `lessEqualToScalar` / `greaterEqualToScalar`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX difference deepening / scalar mask first slice (W1095 / ADR-1441):** `maxAbsDiff` / `hamming` / `equalToScalar` / `greaterThanScalar`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX difference / integer loss first slice (W1094 / ADR-1440):** `absDiff` / `squaredDiff` / `manhattan` / `sumSquaredDiff`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX hard activation deepening first slice (W1093 / ADR-1439):** `leakyRelu` / `hardTanh` / `heaviside` / `relu6`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX clip/threshold first slice (W1092 / ADR-1438):** `clampMin` / `clampMax` / `relu` / `threshold`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis prefix third slice (W1091 / ADR-1437):** `cummaxRows` / `cummaxCols` / `cummin` / `cummax`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis prefix second slice (W1090 / ADR-1436):** `diffRows` / `diffCols` / `cumminRows` / `cumminCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis prefix first slice (W1089 / ADR-1435):** `cumsumRows` / `cumsumCols` / `cumprodRows` / `cumprodCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis reduction fifth slice (W1088 / ADR-1434):** `varRows` / `varCols` / `rangeRows` / `rangeCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis reduction fourth slice (W1087 / ADR-1433):** `argminCols` / `argmaxRows` / `stdRows` / `stdCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis reduction third slice (W1086 / ADR-1432):** `meanCols` / `prodRows` / `argminRows` / `argmaxCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis reduction second slice (W1085 / ADR-1431):** `meanRows` / `prodCols` / `maxRows` / `minCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX axis reduction first slice (W1084 / ADR-1430):** `sumRows` / `sumCols` / `minRows` / `maxCols`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX mask logic third slice (W1083 / ADR-1429):** `logicalXor` / `countTrue` / `noneTrue` / `fillWhere`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX mask logic second slice (W1082 / ADR-1428):** `logicalAnd` / `logicalOr` / `anyTrue` / `allTrue`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX mask selection / same-shape fill first slice (W1081 / ADR-1427):** `zerosLike` / `filledLike` / `choose` / `logicalNot`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element comparison second slice (W1080 / ADR-1426):** `notEqual` / `less` / `lessEqual` / `greaterEqual`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX outer product / same shape / comparison first slice (W1079 / ADR-1425):** `outer` / `onesLike` / `equal` / `greater`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX prefix accumulation / difference first slice (W1078 / ADR-1424):** `cumsum` / `cumprod` / `diff` / `countNonzero`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element log10/Γ/lerp/dot first slice (W1077 / ADR-1423):** `log10` / `tgamma` / `lerp` / `dot`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element fused multiply-add / exponential-logarithm first slice (W1076 / ADR-1422):** `mulAdd` / `exp2` / `log2` / `cbrt`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element floating-point binary first slice (W1075 / ADR-1421):** `hypot` / `pow` / `fmod` / `remainder`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element binary extremum / sign first slice (W1074 / ADR-1420):** `maximum` / `minimum` / `copysign` / `reciprocal`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX element unary second slice (W1073 / ADR-1419):** `floor` / `ceil` / `round` / `trunc`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** NaN GA. **P-SURF packed-SYX element division / scalar broadcast first slice (W1072 / ADR-1418):** `div` / `addScalar` / `subScalar` / `divScalar`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** `operator /`. **P-SURF packed-SYX element unary first slice (W1071 / ADR-1417):** `sign` / `sqrt` / `squared` / `clamp`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX geometry permutation first slice (W1070 / ADR-1416):** `rot180` / `swapRows` / `swapCols` / `roll`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX geometry flip first slice (W1069 / ADR-1415):** `flipud` / `fliplr` / `rot90` / `antidiagonal`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX structural replication first slice (W1068 / ADR-1414):** `tile` / `kron` / `tril` / `triu`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX reduction / norm first slice (W1067 / ADR-1413):** `min` / `max` / `argmin` / `argmax` / `frobeniusNorm`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** kron/tile. **P-SURF packed-SYX shape assembly first slice (W1066 / ADR-1412):** `reshape` / `flatten` / `hstack` / `vstack` / `fromDiagonal`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** kron/tile. **P-SURF packed-SYX slicing / access first slice (W1065 / ADR-1411):** `withAt` / `row` / `col` / `diagonal` / `trace` / `abs`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** `A[i,j]` / **not** in-place setAt. **P-SURF packed-SYX element arithmetic first slice (W1064 / ADR-1410):** `+` / `-` / `negated` / `hadamard` / `filled` / `mean`. **Not** product Mat GA / **not** E-00 / **not** prelude / **not** faer add/sub host. **P-SURF packed-SYX shape generalization first slice (W1063 / ADR-1409):** arbitrary-shape `eye` / `ones` / `sum` / `transposed` / `copy` + `scale`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF packed-SYX convenience/stats first slice (W1062 / ADR-1408):** `at` / `transposed` / `copy` / `eye` / `ones` / `sum`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF solve/packed/shape method family first slice (W1061 / ADR-1407):** `Mat.from` / `packed()` / `rows`/`cols` / `solve`. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF zeros language surface first slice (W1060 / ADR-1406):** `Mat.zeros(rows, cols)` yields an independent zero matrix. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF independent new Mat first slice (W1059 / ADR-1404):** `let c = a * b` yields an independent `Mat` and does not modify the packed data of either operand. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-FAER cdylib packed zeros first slice (W1059 / ADR-1405):** `__syx_plugin_faer_cdylib_zeros`. **Not** the Mat language surface / **not** faer product GA. **P-SURF operator return value first slice (W1058 / ADR-1403):** `let c = a * b` yields a new matrix and does not modify the right operand. **Not** product Mat GA / **not** E-00 / **not** prelude. **P-SURF Mat standalone package landing first slice (W1057 / ADR-1402):** `import Linalg` / `class Mat` HANDLE wrap + `operator func *`. **Not** product Mat GA / **not** E-00 / **not** prelude.

**P-FAER cdylib F64LE packed solve first slice (W1056 / ADR-1401):** `__syx_plugin_faer_cdylib_solve`. **Not** handle GA / **not** faer product GA / **not** product `Mat`.

**P-FAER HANDLE×cdylib drop first slice (W1055 / ADR-1400):** `__syx_plugin_faer_cdylib_hdrop`. **Not** handle GA / **not** faer product GA / **not** `Mat`.

**P-SURF written Mat constraints first slice (W1054 / ADR-1399):** HANDLE wrapper / `operator func` / forbidden to merge into `math.sy`. **Not** `Mat` landing / **not** product GA.

**P-FAER HANDLE×cdylib to/from first slice (W1053 / ADR-1398):** `__syx_plugin_faer_cdylib_hfrom` / `_hto`. **Not** faer product GA / **not** `Mat` / **not** handle GA.

**P-FAER HANDLE×cdylib solve first slice (W1052 / ADR-1397):** `__syx_plugin_faer_cdylib_hsolve`. **Not** faer product GA / **not** `Mat` / **not** handle GA.

**P-ABI-v2 handle borrow first slice (W1051 / ADR-1396):** alloc `borrow_fn`/`pin_fn` + `__syx_plugin_faer_cdylib_hmul`. **Not** handle GA / **not** faer product GA / **not** `Mat`.

**P-FAER cdylib loading first slice (W1050 / ADR-1395):** real dlopen of the out-of-tree `plugins/faerlin` + v2 F64LE `__syx_plugin_faer_cdylib_mul`. **Not** HANDLE×cdylib / **not** faer product GA / **not** `Mat`.

**P-FAER solve first slice (W1049 / ADR-1394):** in-process `__syx_plugin_faer_solve` (square A × b; internal faer dense LU). **Not** faer product GA / **not** `Mat` / **not** ABI v2 completion.

**P-FAER construction first slice (W1048 / ADR-1393):** in-process `__syx_plugin_faer_from` / `__syx_plugin_faer_zeros` / `__syx_plugin_faer_mul` / `__syx_plugin_faer_to`; the handle carries nrows/ncols; internal faer `Mat<f64>` multiplication. **Not** faer product GA / **not** `Mat` / **not** ABI v2 completion.

**ABI v2 handle table first slice (W1047 / ADR-1392):** opaque `u64 → Vec<f64>` inside the Host process; `SYX_PLUGIN_BUF_HANDLE` kind=2 (8-byte LE u64, **not** disguised as f64). Smoke tests `__syx_plugin_v2probe_pin` / `__syx_plugin_v2probe_sumh` / `__syx_plugin_v2probe_drop`. Handles do not enter the SYX GC; unknown / double drop fails. **Not** handle GC GA / faer product GA / four-back-end plugins.

**ABI v2 deepening first slice (W1046 / ADR-1391):** the Host accepts `api_version` 1, 2 and 3. The v1/v2 paths are unchanged. v2m is a multi-slot `syx_plugin_iov` + host `realloc_fn` (initial cap 8, growable; failures are not silently truncated). Smoke tests `__syx_plugin_v2probe_dot` / `__syx_plugin_v2probe_grow`. **Not** handle table GA / faer / four-back-end plugins.

## Security and Signature Verification (Phase 50–51)

| Variable | Description |
|------|------|
| `SYX_HOST_PLUGIN_VERIFY=1` | Enforces ed25519 signature verification |
| `SYX_HOST_PLUGINS=disabled` | Disables loading (enterprise policy, Phase 51+) |

Plugins run in the **same process** as `syx`, which is equivalent to running local native code; they only register Host callbacks and have **no** arbitrary syscall API. Malicious plugins must be controlled through signatures and path allow-lists.

## Differences Across the Three Back Ends

| Back end | Plugin location |
|------|----------|
| VM (`syx run`) | dlopen by the CLI process |
| Native | Same process as `app`, `syx_rt_init_plugins` |
| WASM | Loaded by the **host process**; there is no dlopen inside the guest `.wasm` |

## Embed Integration (Phase 51+)

A host application can set plugin paths through the Embed API:

```c
syx_embed_v2_set_plugin_paths(
    "/path/to/lib1.so:/path/to/lib2.so"
);
```

This works together with the [Embed API](../../docs/9_UsageGuide.md#embed-apiphase-1415) sandbox policy. If encryption is done on the host side and SYX only orchestrates, Embed + HTTP/file may still be simpler than a plugin.

## Relationship to Static stdlib Extensions

| Approach | Applies to |
|------|------|
| Merging into `HostFnId` + `lib/*.sy` | Official core capabilities, hostless WASM scenarios |
| dlopen plugins | Third-party SDKs, private crypto, fast iteration |
| Embed peripheral integration | Host controls all native logic |

For the maintainer static-extension checklist, see [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md). Dense linear algebra should **not** add a static `HostFnId`; the bypass charter (parked): [plugin ABI v2 / faer](../../plan/topics/plugin-abi-v2-linalg.md) (ADR-1389).

## Complete Example: A Custom echo Plugin

### 1. Write the C Plugin

`echo_plugin.c`:

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

### 2. Compile

```bash
clang -shared -fPIC -o libecho_plugin.so echo_plugin.c \
    -I crates/syx-host-plugin/include
```

### 3. Write `plugin.json`

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

### 4. Call It from the SYX Side

`syx.toml`:

```toml
[plugins]
paths = ["./echo_plugin.json"]
```

`main.sy`:

```syx
func main() {
    let msg = __syx_plugin_echo_echo("hello")
    print(msg)                              // "echo: hello"
}
```

### 5. Run

```bash
export SYX_HOST_PLUGIN_PATH=./libecho_plugin.so
syx run .
```

## Diagnostic Codes

| Code | Meaning | Fix |
|------|------|------|
| H001 | Runtime plugin not loaded or Host missing | Check `SYX_HOST_PLUGIN_PATH` |
| H002 | Compile-time manifest inconsistent with runtime / lock file | Re-run `syx install` |
| H003 | Plugin ABI version mismatch | Upgrade the plugin / compiler |
| H004 | Plugin `min_syx_version` not satisfied | Upgrade the compiler |

## Differences from Swift

| Swift | SYX |
|-------|-----|
| Static stdlib | **Implemented** |
| Dynamic plugins (dlopen) | **Implemented** (Phase 49+, opt-in) |
| Signature verification | **Partially implemented** (ed25519) |
| Embed API | **Implemented** (`syx_embed_v1` / `syx_embed_v2`) |
| WASM plugins | **Not applicable** (loaded by the host process) |

## Further Reading

- [Complete Usage Guide § Runtime Extensions](../../docs/9_UsageGuide.md#runtime-host-pluginsopt-in)
- [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md) — ADR-071 / ADR-072 / ADR-073
- [plan/topics/hostfn-extension.md](../../plan/topics/hostfn-extension.md) — maintainer extension checklist

---

**Previous chapter** ← [Operators vs Core Host](32-operators-vs-host.md)

**See also** → [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md) · [Chapter 29, Modules and Packages](29-modules-and-packages.md) · [Chapter 32, Operators vs Core Host](32-operators-vs-host.md) · [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md)
