# R-20 已知缺口 census

**Date:** 2026-11-10
**Scope:** honest accounting of two gaps that are out of scope for R-20 but must not be forgotten.

---

## 1. wasm emit W001（R-23…R-25：套件收口）

**Status:** R-23/R-24 fixed emit W001 for real programs. **R-25** closed the remaining
wasm-suite failures as harness/fixture/isolation work (plus non-AOT `Const*` arms).
Object 198/14 still parked. Fail-closed unchanged.

### R-25 delivery

1. **RUNTIME_CONFIG guard** — [`tests/wasm/runtime_config_guard.rs`](../../crates/syx-driver/tests/wasm/runtime_config_guard.rs):
   Mutex + snapshot/`Drop` restore; `with_aot` / `with_simd128_tier3` /
   `with_runtime_config` shared by full/hybrid/track tests.
2. **Fixtures** — track_b/c `import SIMD4`; `parity/generics.sy` → `id(x: 42)`.
3. **Const\*** — non-AOT `match` arms for `ConstI32/Bool/Nil/Numeric`; p98 WAT installs AOT.
4. **Phase-100 contract** — `aot_mir_exec_only_incompatible_with_guest_control` expects
   W001 (no `#[ignore]`).

| Suite | After R-24 | After R-25 |
|-------|------------|------------|
| `cargo test -p syx-driver --test wasm` | 40 / 6 | **46 passed / 0 failed** |
| credibility `r25_*` | n/a | **5/5** |

R-25 residual for this track: **none** in the wasm harness. Object 198/14 remains parked (out of scope).

### R-24 delivery (retained)

1. **Hello fixture** — `hello.sy` → [`examples/hello.sy`](../../examples/hello.sy).
2. **Drop vs Pop** — `next_is_pop` early-return; Print in stub cleanup.
3. **ToStr / ConcatStr** — guest `I32Const(0)` placeholders.
4. **SimdI8Extract** — host Drop + StackTracker `Block(Result)` / Br dead-code.

| Suite | After R-23 | After R-24 |
|-------|------------|------------|
| `cargo test -p syx-driver --test wasm` | 26 / 20 | **40 passed / 6 failed** (gate ≥34 ✓) |
| greeter / dataclass_synth AOT | W001 | **green** |
| credibility `r24_*` | n/a | **5/5** |

### R-23 delivery (retained)

1. **StackTracker** — [`crates/syx-wasm-emit/src/stack_tracker.rs`](../../crates/syx-wasm-emit/src/stack_tracker.rs)
2. **Evidence** — `wasm_validate_probe` on `print(1)`: CallIndirect + take_catch stub
3. **Fix** — `leaves_guest_call_stub` Drop unless next is `Pop`

| Suite | Before (R-20 census) | After R-23 |
|-------|----------------------|------------|
| `cargo test -p syx-driver --test wasm` | 14 passed / 32 failed | **26 passed / 20 failed** |
| minimal `print(1)` emit | W001 | **validates** |
| `syx-wasm-emit --lib` | green | green (incl. stack_tracker tests) |

### Seven falsified hypotheses

Recorded so nobody repeats them.

| # | Hypothesis | How it was falsified |
|---|------------|----------------------|
| 1 | `Br(3)` depth wrong, change to `Br(2)` | failure offset `0xdb13` unchanged |
| 2 | `CallClosure` missing `Drop` | offset unchanged |
| 3 | AOT-specific path | `SYX_WASM_AOT=0` fails identically |
| 4 | `JumpIfFalse` expansion doesn't `break` on `Return` | **emitted bytes MD5-identical** (`eb46f446…`), branch never executed |
| 5 | Hand-written stack simulator identified the leak | simulator omitted `I32Sub` / `Else` / `Select`, measurement unreliable |
| 6 | `emit_call_host_take_catch` nets +1 | manual per-instruction sum is net 0 |
| 7 | Gate `keep_result` on `instr.dest` | **`dest` is never `None` for `Call`** (probed); bytes MD5-identical |

### Retained assets

**Forensic tool:**

```bash
cargo run -p syx-wasm-emit --example wasm_validate_probe -- <file.wasm>
cargo run -p syx-wasm-emit --example wasm_validate_probe -- <file.wasm> --dump-ops 875
```

Handles `ValidPayload::Func` (wasmparser 0.219 deferred body validation).

**Stack-effect table** — [`crates/syx-wasm-emit/src/stack_effect.rs`](../../crates/syx-wasm-emit/src/stack_effect.rs).

**Fail-closed is non-negotiable.** `wasmparser::validate` is never downgraded; no
`SYX_SKIP_WASM_VALIDATE`; failing cases are not `#[ignore]`d.

### What the next attempt needs

Wasm harness is green after R-25. **R-26** closed the Swift closure statement-body
P001 hole (`parse_swift_closure` + `looks_like_closure_stmt_body`); see
[`r-26-frontend-red-census.md`](r-26-frontend-red-census.md). **R-27** sliced
Array/Dict/Set public `Object` → `Sendable` (inventory **186/11**; simd **159**
still parked); see [`r-27-collections-object-census.md`](r-27-collections-object-census.md).
Further Object unpark needs an honest SIMD / OrderedSet / json plan. Do not retry
the seven falsified hypotheses.

---

## 2. struct / data class operators (R-20 target)

**Status:** fixed in R-20 (`==` / `!=`), with the underlying pipeline defect found and
fixed in R-21.

MIR lowering emits `Call("{Type}_{op}")` for any struct or data class operand:

```66:82:crates/syx-mir/src/mir_lower_expr_binary.rs
        if let Some(op_sym) = binop_operator_symbol(*op) {
            let ty_smol = left_ty.clone();
            if (ctx.module.class_types.contains_key(&ty_smol)
                || ctx.module.dataclass_names.contains(&ty_smol))
                && !syx_stdlib::is_builtin_type(ty_smol.as_str())
                && ty_smol.as_str() != "Unknown"
            {
                let fn_name = SmolStr::new(format!("{left_ty}_{op_sym}"));
```

[`binop_operator_symbol`](../../crates/syx-mir/src/mir_type_infer_helpers.rs) covers 13
operators, but [`lower_dataclass_shells`](../../crates/syx-mir/src/mir_lower_shells.rs)
synthesizes only four functions.

| Operator | Emitted name | Synthesized | Rationale |
|----------|--------------|-------------|-----------|
| `==` | `{T}_==` | **yes (R-20)** | structural equality holds for any value type |
| `!=` | `{T}_!=` | **yes (R-20)** | negation of `==` |
| `+` `-` `*` `/` | `{T}_+` … | no | no natural semantics for an arbitrary value type |
| `<` `<=` `>` `>=` | `{T}_<` … | no | only meaningful under `Comparable` |
| `&` `\|` `^` `<<` `>>` | `{T}_&` … | no | integer-only |

Unsynthesized operators must produce an actionable diagnostic instead of a bare
`[C020] Unknown function 'Point_+'` — see R-20-03.

### Synthesized shells (before R-20)

| Shell | Function |
|-------|----------|
| `_init` | memberwise constructor → `Dictionary` bag |
| `_copy` | field-by-field clone |
| `_describe` | `Type(field: value, ...)` |
| `_hash_into` | accumulate fields into a `Hasher` |

`_==` / `_!=` were missing, so **every** `struct` and `data class` comparison failed at
`check` time with `[C020] Unknown function 'Point_=='` — on the VM path too, not just wasm.
---

## 3. Execution-path divergence (found in R-20, fixed in R-21)

**Status:** fixed. Recorded because it silently invalidated struct-field assertions on the
embedded path.

`compile_and_run_embedded` (used by ~204 test files) and `run_file_printed` (used by ~67)
disagreed on the same program:

| Probe | embedded | file path |
|-------|----------|-----------|
| `print(p.x)` for `Point(x: 3, y: 4)` | `nil` | `3` |
| `print(p)` | `Point {}` | `{x: 3, y: 4}` |
| `p == r` with differing fields | `true` | `false` |

### Root cause

`lower_class_init` runs before shell synthesis and emits a generic `{Type}_init` with
`param_count = 0` that builds only `{__type: "Type"}`. The memberwise shell from
`lower_dataclass_init_shell` (`param_count = fields.len()`, builds every field) was then
handed to `merge_mir_functions`, which **only appends when the name is absent** — so the
correct shell was silently dropped and the zero-arg version won. Every struct instance was
constructed with empty fields; `p.x` read back `nil`, and `p == r` compared `nil == nil`
into `true`.

### Fix

`merge_mir_shell_functions` in `mir_lower_shells.rs` replaces same-named functions instead
of skipping them; `compile_to_program` uses it for dataclass and enum shells.

### Consequence for earlier windows

Any assertion of a **struct / data class field value** running on the embedded path before
R-21 was vacuous — it compared `nil` against `nil`. Conclusions that did not touch struct
fields (host-function round-trips, scalar printing, task behaviour) are unaffected. The
R-20 `==`/`!=` shells themselves were sound; only their *embedded-path field inputs* were
empty, which is why the R-20 contract was split across two paths at the time and has been
merged back into one now.

`r21_path_parity_contract` pins both paths to identical output so this cannot regress
silently.

---

## 4. Copy equality asymmetry (found in R-21/R-22, fixed in R-22)

**Status:** fixed.

Symptom: `p == p.copy()` printed `true` but `p.copy() == p` / `c == p` printed `false`,
and `c.description` was `nil`.

### Root cause

MIR binary ops dispatch on the **left** operand type only (`Call("{T}_==")`).
`T.copy()` had no return-type inference, so the left side of `p.copy() == p` was
`Unknown` and fell through to `ValueEq` (identity). Bound copies never entered
`local_types` as `T`, so `.description` degraded to `ObjectGet("description")` → `nil`.

A related false negative during forensics: incremental cache keys on source hash +
`cache_version_tag`, so compiler-only shell fixes (e.g. `_describe` field-0 label) did
not invalidate `/tmp/.syx-cache` hits until the cache was cleared.

### Fix

`dataclass_shell_method_return_type` in `mir_type_infer_helpers.rs` treats
dataclass/struct `.copy()` as returning the receiver type. `mir_unify` now uses
`merge_mir_shell_functions` for all six dataclass/enum shell merge sites (aligned with
the VM `compile_with_mir` path). `r22_copy_eq_symmetry_contract` pins symmetry +
describe labels on both paths.
