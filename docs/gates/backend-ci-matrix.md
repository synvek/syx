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

### C-3h (done · MIR ModuleId thin gate)

`ModuleId` + `MirModIndex::build/require` + `collect_module_names_from_paths` — dense module-name→id table; duplicate name / unknown require / same-stem different path → **MIR008**. `MirModule.module_names` populated from `ModuleGraph.order` stems (empty OK). Hooked after nil-branch floor on unify + codegen. Census: [`c-3h-mir-mod-index-census`](../census/c-3h-mir-mod-index-census.md). C-3 remains `[~]` (≠ `[x]`). **≠** DefId / Call(ModuleId) IR / full MirOp / exception.

### C-4a (done · git rev pin + locked git install)

Manifest git dependencies require explicit `rev` (missing → **P043**); never default to `main`. Under `SYX_INSTALL_LOCKED`, Git pins are read from `syx.lock` (URL/rev; missing/wrong → **P014**); lock bytes not rewritten. Census: [`c-4a-git-rev-pin-census`](../census/c-4a-git-rev-pin-census.md). **≠** default-locked install / publish-requires-lock / lock v16 trim (C-4b+).

### C-4b (done · locked-by-default install)

`InstallMode::{Auto,Locked,Unlocked}`: Auto locks when `syx.lock` exists (no rewrite); first install without lock still writes; `syx update` / `add` use Unlocked; CLI `syx install --locked` requires lock (**P014**). Env `SYX_INSTALL_LOCKED` / `SYX_INSTALL_UNLOCKED` retained. Census: [`c-4b-locked-default-census`](../census/c-4b-locked-default-census.md). **≠** publish-requires-lock / lock v16 trim / path content hash (C-4c+).

### C-4c (done · publish requires usable lock)

`require_publish_lock` / `publish_package`: missing `syx.lock` → **P014** (C-4c); present → `enforce_lock_v16`. CLI `syx publish --dry-run` uses the same gate. Empty `packages` OK for zero-dep libs after install. Census: [`c-4c-publish-lock-census`](../census/c-4c-publish-lock-census.md). **≠** lock v16 field trim / path content hash (C-4d+).

### C-4d (done · lock v16 infra trim)

`graph_to_lockfile` no longer stamps CDN/farm/billing/marketplace env snapshots (or hard-coded phase8 CDN regions) into fresh `syx.lock`. Schema + `enforce_lock_v12…v16` retained for old/manual locks. Runtime CDN still reads process env. Census: [`c-4d-lock-trim-census`](../census/c-4d-lock-trim-census.md). **≠** path content hash / delete SyxLockfile fields (C-4e+).

### C-4e (done · path content hash)

`PackageSource::Path` records `checksum = sha256:…` (directory tree hash; excludes `.git` / `target` / `.DS_Store`). Unlocked resolve writes it; Locked install recomputes and fail-closes on missing/mismatch (**P014**, C-4e). Legacy path locks without checksum require Unlocked rewrite. Census: [`c-4e-path-content-hash-census`](../census/c-4e-path-content-hash-census.md). **C-4 `[x]`**. **≠** manifest↔lock full reconcile / delete SyxLockfile infra fields / C-5.

### C-5a (done · threat-model honesty)

ADR [`macro-embed-threat-model.md`](../adr/macro-embed-threat-model.md): macro = reexec-only; embed flags = soft/HTTP deny; permission-sandbox = HostFn S024; `*SECCOMP*` = label/probe. Public claim scrub + `macro_sandbox_mode` → `seccomp-label-unwired`. Census: [`c-5a-threat-model-census`](../census/c-5a-threat-model-census.md). C-5 remains `[~]` (≠ `[x]`). **≠** embed wasm fail-closed / real seccomp (C-5b+).

### C-5b (done · embed wasm fail-closed)

When `SYX_EMBED_WASM_SANDBOX=1`, `syx_embed_v2_run` returns **null** if wasmtime is unavailable or the wasm run fails — **no** silent `compile_and_run_embedded` fallback. Unset env keeps default in-process. Census: [`c-5b-embed-wasm-fail-closed-census`](../census/c-5b-embed-wasm-fail-closed-census.md). C-5 remains `[~]`. **≠** timeout wire / real seccomp (C-5c+) / C-5 `[x]`.

### C-5c-1 (done · embed/macro timeout hard wire)

Opt-in hard wall: `SYX_EMBED_TIMEOUT_MS` (env wins) ∪ CLI `--embed-timeout-ms`; in-process `recv_timeout` → null; wasmtime subprocess **killed** on deadline → null; `SYX_MACRO_TIMEOUT_MS` kills expand child → **M040**. Unset = no wall-clock default. Census: [`c-5c-1-embed-macro-timeout-census`](../census/c-5c-1-embed-macro-timeout-census.md). C-5 remains `[~]`. **≠** seccomp BPF / RSS hard kill / C-5 `[x]` (→ **C-5c-2**).

### C-5c-2 (done · Linux seccomp net-deny wire)

Opt-in `SYX_MACRO_SECCOMP` / `SYX_EMBED_SECCOMP`: Linux installs **net-deny seccomp-BPF** (deny socket/connect/bind/listen; default-allow) in macro child / embed worker thread; mode `seccomp-netdeny-wired`. Non-Linux + env → **M040** / **null**. Census: [`c-5c-2-seccomp-wire-census`](../census/c-5c-2-seccomp-wire-census.md). C-5 remains `[~]`. **≠** Job Object / full filter GA / RSS hard kill / C-5 `[x]` (→ **C-5d**).

### C-5d (done · embed/macro RSS hard wall)

Opt-in `SYX_EMBED_RSS_MB` / `SYX_MACRO_RSS_MB`: Linux **`prlimit(RLIMIT_AS)`** in embed worker / wasmtime child pid / macro child; over-AS → **null** / **M040**. Non-Linux + env → fail-closed. MEMORY: env∪CLI stdout-sum soft (honest). Census: [`c-5d-embed-rss-hard-census`](../census/c-5d-embed-rss-hard-census.md). C-5 remains `[~]`. **≠** Job Object / macOS sandbox / C-5 `[x]` (→ **C-5e**).

### C-5e (done · Windows Job Object thin gate)

Same `SYX_*_RSS_MB` env: **Windows** Job Object **`PROCESS_MEMORY_LIMIT`** on embed worker / wasmtime child / macro child (parent assign); over-limit → **null** / **M040**. Linux keeps prlimit. macOS/other + env → fail-closed. Census: [`c-5e-jobobject-thin-census`](../census/c-5e-jobobject-thin-census.md). C-5 remains `[~]`. **≠** full Job Object GA / macOS sandbox / C-5 `[x]` (→ **C-5f**).

### C-5f (done · macOS Seatbelt net-deny thin gate)

Opt-in `SYX_EMBED_MACOS_SANDBOX` / `SYX_MACRO_MACOS_SANDBOX`: **macOS** Seatbelt **net-deny** via `sandbox_init(SANDBOX_NAMED, "no-network")` in macro child and `sandbox-exec -n no-network` around wasmtime; **in-process embed → null**. Non-macOS + env → **null** / **M040**. Census: [`c-5f-macos-sandbox-thin-census`](../census/c-5f-macos-sandbox-thin-census.md). C-5 remains `[~]`. **≠** App Sandbox GA / full filter / C-5 `[x]` (→ **C-5g**).

### C-5g (done · Linux seccomp allowlist thin gate)

Opt-in `SYX_EMBED_SECCOMP_ALLOWLIST` / `SYX_MACRO_SECCOMP_ALLOWLIST`: **Linux** **default-deny** BPF allowlist (v11 base + futex/brk/openat/close/mprotect[/arch_prctl]); mutually exclusive with `SYX_*_SECCOMP` net-deny. Non-Linux + env → **null** / **M040**. Census: [`c-5g-seccomp-allowlist-census`](../census/c-5g-seccomp-allowlist-census.md). C-5 remains `[~]`. **≠** full syscall GA / permission-sandbox default-on / C-5 `[x]` (→ **C-5h**).

### C-6a (done · version SSOT align)

Public semver SSOT = root `Cargo.toml` `[workspace.package].version` (currently **0.4.0**). Aligned: docs/books/`current.md` banners, `plan/01`, three capability TOML `syx_version` (+ generated), credibility nails, R-49/S2-07 probes. Census: [`c-6a-version-ssot-census`](../census/c-6a-version-ssot-census.md).

### C-6b (done · gen-from-Cargo)

`gen_capabilities.py` / `gen_ir_capabilities.py` parse Cargo `V`, force `meta.syx_version = V`, rewrite stale TOML on write, `--check` fail-closed when TOML≠`V` or outputs stale; IR `"117.0"` fallback removed. Shared helper [`scripts/syx_cargo_ssot.py`](../../scripts/syx_cargo_ssot.py). Census: [`c-6b-gen-from-cargo-census`](../census/c-6b-gen-from-cargo-census.md). **C-6 `[x]`**.

### C-7a (done · fuzz-signal honesty)

`syx_smoke` consumes UTF-8 via `syx-lexer::Lexer::scan_all` (invalid UTF-8 early-return; no deliberate panic). Required CI job **`fuzz-smoke`**: `cargo +nightly fuzz run syx_smoke -- -runs=1`. UsageGuide CI **met** aligns with yaml. Census: [`c-7a-fuzz-honesty-census`](../census/c-7a-fuzz-honesty-census.md).

### C-7b (done · soft-step fail-closed)

Soft-job steps that previously swallowed test failures with `exit 0` now propagate `exit "$ec"` / `exit 1` (gui · jit · extended · native host-shake/differential · browser-e2e · windows plugin fail · book rate≠100). Jobs stay `continue-on-error`. Whitelist keep `exit 0`: empty plugin list · missing book JSON · domain/property defer notice. Census: [`c-7b-soft-exit0-census`](../census/c-7b-soft-exit0-census.md).

### C-7c (done · count-gate align)

SSOT = `scripts/audit_behavioral_tests.sh` defaults **2600 / 1700** (anti-delete only ≠ maturity KPI). Aligned: CI lint · nightly · docs/books README · implementation-status · ADR charter · site about. Census: [`c-7c-count-gate-census`](../census/c-7c-count-gate-census.md). C-7 remains `[~]` (≠ `[x]`). **≠** scaffold/multi-fuzz/soft→required (C-7c+ park).

### C-8a (done · Ch15 implicit-super MD honesty)

Ch15 en+zh: omitting `super.init` = **U-38 inherited field defaults ≠ CallSuper**; true implicit `CallSuper` marked **not implemented**. Lang honesty nails + census [`c-8a-ch15-implicit-super-md-census`](../census/c-8a-ch15-implicit-super-md-census.md). C-8 remains `[~]` (≠ `[x]`). **≠** implement CallSuper / formatter `--allow-lossy` (C-8b).

### C-8b (done · formatter refuse-lossy)

Default `syx format` / `--check` / LSP: CST001 → **`F003`** refuse (no silent AST comment-dropping write). Explicit CLI `--allow-lossy` restores CST001→AST auto. LSP hard `allow_lossy=false`. Census [`c-8b-fmt-refuse-lossy-census`](../census/c-8b-fmt-refuse-lossy-census.md). C-8 remains `[~]` (≠ `[x]`). **≠** CST expand / lossless GA / book↔parser fork list (C-8c).

### C-8c (done · book↔parser fork inventory)

Live fork ledger ≥20 rows with closed classes (`parked_honest` · `book_ahead` · `legacy_accept` · `doc_debt`). Census [`c-8c-book-parser-fork-census`](../census/c-8c-book-parser-fork-census.md). C-8 remains `[~]` (≠ `[x]`). **≠** fork converged · parser surface changes · CST lossless (C-8d+ park).

### C-8d (done · book MD honesty)

Cleared inventory `doc_debt`/`book_ahead` (F20–F23) en+zh: labeled break Partial honesty; convenience/required Not; mutating Partial; typed-throws ≠ GA. Census [`c-8d-book-md-honesty-census`](../census/c-8d-book-md-honesty-census.md). C-8 remains `[~]` (≠ `[x]`). **≠** F18/F19 legacy parser remove · CST lossless (C-8e+ park).

### C-8e (done · legacy compat notes)

Ch10 en+zh **Compatibility (C-8e)** for F18 bare fields + F19 unmarked mutating (ADR one-cycle; ≠ remove parser accept). Inventory F18/F19 → `parked_honest`. Census [`c-8e-legacy-compat-notes-census`](../census/c-8e-legacy-compat-notes-census.md). C-8 remains `[~]` (≠ `[x]`). **≠** CST lossless GA · remove legacy parser (C-8f+ park).

### C-8f (done · CST residual note)

UsageGuide en+zh **Honesty (C-8f):** refuse-lossy (`F003`) **≠** format **lossless GA** / full-language trivia formatter; lossless reindent only inside CST subset. Census [`c-8f-cst-residual-note-census`](../census/c-8f-cst-residual-note-census.md). C-8 remains `[~]` (≠ `[x]`). Soft park: CST lossless implementation · CallSuper · remove parser legacy.

## Next

Default next after C-0m: **跨轴** — C-0a+ prep / C-0e / Actor·Module.Type ctor（**不**宣称 C-3/C-5 `[x]`）。C-0 remains `[x]` thin (C-0a…d + **C-0f** + **C-0g** + **C-0h** + **C-0i** + **C-0j** + **C-0k** + **C-0l** + **C-0m** done; C-0a+/C-0e park; Actor/Module.Type ctor/host-extension method GA park). C-3 remains `[~]` (C-3a…**ca** done; full MirOp/Call(ModuleId) IR/HostFn arity/trim remap/global name table·remap/locals type·φ/ConstFnIndex fail-closed(R-33)/CallIndirect arity semantics/JumpIfNil path-sensitive height/CallSuper init self path/CallClosure full arity/AwaitPoll·BindResult/cross-block φ/完整 DefId/完整 exception park; scalar non-pilot thin stack floors exhausted). C-5 remains `[~]` (C-5a…**h** done; true full syscall GA park). C-8 remains `[~]` (CST lossless park). C-7 remains `[~]` (C-7c+ park). **C-6 `[x]`**. **C-4 `[x]`**.
