# Backend / CI support matrix (C-2a SSOT)

**Role:** Single source of truth for which backends are **default-hard** vs **soft / experimental**.  
**Live backlog:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2  
**Status snapshot:** [`plan/01-project-status.md`](../../plan/01-project-status.md) · Native 2.0 / CI soft  
**Census:** [`docs/census/c-2a-backend-ci-matrix-census.md`](../census/c-2a-backend-ci-matrix-census.md)

**Date:** 2026-11-24 · **C-2a…C-2d**（`-N` = N×历史 `+`；C-2 轴收口）

## Contract

| Tier | Meaning |
|------|---------|
| **default_hard** | Merge / census KPI floor. Soft-skip must **not** count as “supported”. |
| **soft_non_default** | May run with `continue-on-error` or L006 skip; **≠ product green / ≠ default KPI**. |
| **experimental** | Opt-in pilot only; never default KPI. |

### Hard rule (C-2a)

**L006 skip / soft native job green ≠ native supported.**  
LLVM refuse-without-`full-native` uses diagnostic code **L006** (toolchain gate; ≠ book “illegal char” L006).

---

## default_hard

| Surface | Floor | Evidence (today) |
|---------|-------|------------------|
| **VM** | runnable / check smoke | required `lint` + driver / census |
| **MIR** | runnable path | required CI + driver |
| **Wasm** | **46/46** | `--test wasm` (census); required `compiler-wasm` / `wasm-aot-strict` / `wasm-aot-full` |
| **Browser** | **0** failed | `--test browser` (census); note: CI job `browser-e2e` is still **soft** |
| **Book output** | `sy,output` **12/12** | `scripts/book_pass_rate.py` (census) |
| **Object / simd residual** | **≤180/≤10** · **≥159** | packages scan (census) |
| **Native paid floor** | required `full-native-linux` hello + path_file + tcp + sql + http GA + pool/row llvm + http.types/server/stream/timeout llvm + file raii/stream/watch llvm + tls.client aot | `.github/workflows/ci.yml` · LLVM 21.1.6 · `--features full-native` · **no** `continue-on-error` (C-2b-2…13) |

These are the **default support claims** for SYX as research beta. Do not advertise product-wide Native AOT GA or JIT as default-supported on soft CI alone; paid raises ≠ bulk `native=real`; JIT remains experimental (**C-2c**).

---

## soft_non_default

| Job / path | CI / code | Why soft |
|------------|-----------|----------|
| `native-linux` | `.github/workflows/ci.yml` · `continue-on-error: true` · L006 → **step fail** (C-2b+; ≠ soft-skip exit 0) | Default CLI lacks `--features full-native` |
| `native-macos` | same · L006 → **exit 1** (C-2b+) | same |
| `browser-e2e` | `continue-on-error: true` | Soft e2e wall; hard browser floor is census `browser_failed=0` |
| Extended / differential soft walls | `continue-on-error` | Unpaid vs default_hard |
| `scripts/s0_backend_smoke.sh` | L006 **FAIL-CLOSED** counts as pass on required lint | Toolchain honesty, not native GA |
| ~~`run_native_strict_or_mirror_vm`~~ | **C-2b exited** — helper **removed**; GA uses `try_run_native_strict` + **skip** (≠ VM mirror) | Forbidden: mirror VM stdout as native GA |

**Capability:** C-2b-1 demoted unpaid → `partial`. Raised: path×2 (C-2b-3) · tcp×2 (C-2b-4) · `data.sqlite` (C-2b-5) · `net.http` (C-2b-6) · pool+row (C-2b-7) · `net.http.types` (C-2b-8) · `net.http.server` (C-2b-9) · `net.http.stream` (C-2b-10) · `net.http.timeout_tls` (C-2b-11) · io.file.raii·stream·watch (C-2b-12) · `net.tls.client` (C-2b-13). Remaining product rows stay `partial` (incl. `net.tls.server`). JIT labels scrubbed (**C-2c**). `ci.job.*` surfaces cite live yaml job ids (**C-2d**). C-2 axis **closed**.

---

## experimental

| Surface | Label | Notes |
|---------|-------|-------|
| **JIT** (Suite-1 / Cranelift) | experimental / unpaid | CI job `jit` · `continue-on-error`; **non_default_kpi** |
| SyxHost / CST deep / true implicit `CallSuper` | park | Not default next |

**C-2c (done):** public FeaturePriority / env-registry / AnalysisReport wording unified to **experimental-real-subset · default off · non_default_kpi** (≠ Stub / ≠ default-on).

---

## Non-claims

### C-2a (done)
- Documented hard/soft/experimental split; L006 skip ≠ native supported

### C-2b (done · mirror honesty)
- **Removed** `run_native_strict_or_mirror_vm`; GA nails skip without full-native AOT
- **Hard rule:** mirror VM stdout as native parity = **forbidden**

### C-2b+ (done · native honesty thin)
- **Demoted** unpaid product `native="real"` → `"partial"` (38 rows)
- **Banned** L006 soft-skip → step success (`return`/`exit` 0); jobs stay `continue-on-error`
- **Hard rule:** unpaid `native=real` = **forbidden** until paid full-native evidence

### C-2b++ / C-2b-2 (done · paid full-native floor)
- **Required** job `full-native-linux`: LLVM 21.1.6 → `cargo build -p syx-cli --features full-native` → `syx build --native examples/hello.sy`
- **No** `continue-on-error`; **no** `full-native || release` fallback on that job

### C-2b-3 (done · path_file paid raise)
- `full-native-linux` also runs `path_file_native_wasm_ga` with `--features full-native`
- **Raised** `io.path` + `io.file.read_all_bytes` → `native=real` (notes cite C-2b-3)

### C-2b-4 (done · tcp paid raise)
- `full-native-linux` also runs `tcp_native_wasm_ga` with `--features full-native`
- **Raised** `net.tcp` + `net.tcp.listener` → `native=real` (notes cite C-2b-4)

### C-2b-5 (done · sql paid raise)
- `full-native-linux` also runs `sql_native_wasm_ga` with `--features full-native`
- **Raised** `data.sqlite` → `native=real` (notes cite C-2b-5；≠ pool/row)
- Product `native=real` total = **5**

### C-2b-6 (done · http paid raise)
- `full-native-linux` also runs `http_native_wasm_ga` with `--features full-native`
- **Raised** `net.http` → `native=real` (notes cite C-2b-6；≠ server / types / stream / timeout_tls)
- Product `native=real` total = **6**

### C-2b-7 (done · pool/row paid raise)
- `full-native-linux` also runs `hostfn_sql_pool_lifecycle_native_llvm` + `hostfn_sql_row_accessors_native_llvm` (`--test stdlib`)
- **Raised** `data.sqlite.pool` + `data.sqlite.row` → `native=real` (notes cite C-2b-7)
- Product `native=real` total = **8**

### C-2b-8 (done · http.types paid raise)
- `full-native-linux` also runs `m2_s4_01_response_accessors_native_llvm` (`--test stdlib`)
- **Raised** `net.http.types` → `native=real` (notes cite C-2b-8；≠ server / stream / timeout_tls)
- Product `native=real` total = **9**

### C-2b-9 (done · http.server paid raise)
- `full-native-linux` also runs `m2_s4_03b_graceful_shutdown_native_llvm` (`--test stdlib`)
- **Raised** `net.http.server` → `native=real` (notes cite C-2b-9；≠ stream / timeout_tls)
- Product `native=real` total = **10**

### C-2b-10 (done · http.stream paid raise)
- `full-native-linux` also runs `m2_s4_04_http_body_stream_native_llvm` (`--test stdlib`)
- **Raised** `net.http.stream` → `native=real` (notes cite C-2b-10；≠ timeout_tls)
- Product `native=real` total = **11**

### C-2b-11 (done · timeout_tls paid raise)
- `full-native-linux` also runs `m2_s4_02b_timeout_hang_loopback_native_aot` (`--test stdlib`)
- **Raised** `net.http.timeout_tls` → `native=real` (notes cite C-2b-11；≠ net.tls.client / skip-verify / server TLS)
- Product `native=real` total = **12**

### C-2b-12 (done · io.file.raii·stream·watch paid raise)
- `full-native-linux` also runs `m2_s3_02b_with_open_native_llvm` + `m2_s3_03_file_stream_read_chunk_native_llvm` + `m2_s3_03_file_watcher_native_llvm` (`--test stdlib`)
- **Raised** `io.file.raii` + `io.file.stream` + `io.file.watch` → `native=real` (notes cite C-2b-12；≠ net.tls.* / recursive watch GA / File.watch one-shot)
- Product `native=real` total = **15**

### C-2b-13 (done · tls.client paid raise)
- `full-native-linux` also runs `m2_s4_05_https_loopback_ok_with_extra_root_native_aot` + `m2_s4_05_self_signed_fail_closed_native_aot` (`--test stdlib`)
- **Raised** `net.tls.client` → `native=real` (notes cite C-2b-13；≠ skip-verify / server TLS / daemon / `net.tls.server`)
- Product `native=real` total = **16**

### C-2c (done · JIT experimental label scrub)
- Public FeaturePriority §23 / env-registry `SYX_JIT_TIER2` / AnalysisReport JIT rows → **experimental-real-subset · default off · non_default_kpi**
- CI job `jit` stays `continue-on-error`; product `native=real` count unchanged (**16**)
- ≠ JIT Suite-1 expand / ≠ default-on JIT / ≠ hard KPI

### C-2d (done · ci.job.* surface align)
- 8× `ci.job.*` `surface`/`notes` cite live yaml ids: `native-linux` · `native-macos` · `extended-suites` · `jit` · `gui-xvfb` · `windows-smoke` · `book-doctest`
- Capability `id`/`feature` unchanged; yaml job names unchanged; soft `continue-on-error` unchanged
- ≠ soft→hard · ≠ yaml rename · ≠ R-37/R-39 fossil script rewrite

### Still park (post C-2)
- ≠ `net.tls.server` paid raise (park；≠ daemon GA)
- ≠ macOS required full-native
- ≠ R-50

---

### C-3a (done · MIR quality demote)

Codegen no longer silently picks a higher-`mir_function_quality` body for duplicate MIR names. Conflicting fingerprints → **MIR001** fail-closed; identical fingerprints keep first. Same-type overloads get arity-qualified MIR names upstream (`__a{N}`). Census: [`c-3a-mir-quality-demote-census`](../census/c-3a-mir-quality-demote-census.md). **≠** C-3 full exit / DefId / full verifier.

### C-3b (done · MIR CFG / terminator floor)

`verify_mir_cfg_floor` fail-closes on bad entry, OOB jump/successor, or missing terminator (**MIR002**). Hooked at `build_unified_mir` and `compile_from_mir_with_source`. Census: [`c-3b-mir-cfg-floor-census`](../census/c-3b-mir-cfg-floor-census.md). **≠** C-3 full exit / DefId / full MirOp stack·type verifier.

### C-3c (done · MirFnIndex / FuncId)

`FuncId` + `MirFnIndex::build/require` — dense name→id table; duplicate name / unknown Call callee → **MIR003**. Hooked after CFG floor on unify + codegen; Call resolve fail-closed (no fn-0 bind). Census: [`c-3c-mir-fn-index-census`](../census/c-3c-mir-fn-index-census.md). **≠** ModuleId graph / Call(FuncId) IR rewrite / full MirOp verifier.

### C-3d (done · MIR stack-height floor)

`verify_mir_stack_floor` / `verify_mir_stack_floor_functions` — suite1/pilot value-stack height; underflow → **MIR004**. Hooked after CFG + FnIndex on unify + codegen (post-unique bodies). CallHost / non-pilot abandon stream; Return empty = mir-run Nil pad. Census: [`c-3d-mir-stack-floor-census`](../census/c-3d-mir-stack-floor-census.md). **≠** cross-block join φ / full type / exception verifier.

### C-3e (done · MIR type-tag floor)

`verify_mir_type_floor` / `verify_mir_type_floor_functions` — suite1/pilot coarse tags; Nil on Add / non-numeric on Sub·Mul·Div → **MIR005**. Hooked after stack floor on unify + codegen. Add string concat allowed (mir-run). Census: [`c-3e-mir-type-floor-census`](../census/c-3e-mir-type-floor-census.md). **≠** locals slot types / cross-block φ / full semantic types.

### C-3f (done · MIR JumpIfFalse condition tag)

`verify_mir_branch_floor` / `verify_mir_branch_floor_functions` — JumpIfFalse condition must be Bool|Unknown; concrete I32|Str|Nil → **MIR006**. Cmp/logic ops treated as Bool producers (without expanding suite1 pilot predicate). Hooked after type floor. Census: [`c-3f-mir-branch-floor-census`](../census/c-3f-mir-branch-floor-census.md). **≠** JumpIfNil / dominators / exception pairing.

### C-3g (done · MIR JumpIfNil condition tag)

`verify_mir_nil_branch_floor` / `verify_mir_nil_branch_floor_functions` — JumpIfNil condition must be Nil|Unknown; concrete Bool|Str|I32 → **MIR007**. Nil pops; Unknown left on stack (mir-run non-Nil path). Hooked after branch floor. Census: [`c-3g-mir-nil-branch-floor-census`](../census/c-3g-mir-nil-branch-floor-census.md). **≠** BeginTry/Throw pairing / ModuleId / full MirOp verifier.

### C-4a (done · git rev pin + locked git install)

Manifest git dependencies require explicit `rev` (missing → **P043**); never default to `main`. Under `SYX_INSTALL_LOCKED`, Git pins are read from `syx.lock` (URL/rev; missing/wrong → **P014**); lock bytes not rewritten. Census: [`c-4a-git-rev-pin-census`](../census/c-4a-git-rev-pin-census.md). **≠** default-locked install / publish-requires-lock / lock v16 trim (C-4b+).

### C-4b (done · locked-by-default install)

`InstallMode::{Auto,Locked,Unlocked}`: Auto locks when `syx.lock` exists (no rewrite); first install without lock still writes; `syx update` / `add` use Unlocked; CLI `syx install --locked` requires lock (**P014**). Env `SYX_INSTALL_LOCKED` / `SYX_INSTALL_UNLOCKED` retained. Census: [`c-4b-locked-default-census`](../census/c-4b-locked-default-census.md). **≠** publish-requires-lock / lock v16 trim / path content hash (C-4c+).

## Next

Default next after C-4b: **C-4c** — `syx publish` requires a usable `syx.lock` (fail-closed). C-4 remains `[~]` (≠ `[x]`). C-3 remains `[~]` (ModuleId / full MirOp / exception park).
