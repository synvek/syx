# A-04 Edge Semantics (Wave A W277 + W281 + W283 + W285 + W286 + W287 + W288 exit charter)

> **Status:** **Q-L4 floor signed** (ADR-632 / W288) — seven thin slices: **deinit** + **nil** (ADR-621 / W277) + **exception** (ADR-625 / W281) + **overflow** (ADR-627 / W283) + **NaN** (ADR-629 / W285) + **Unicode** (ADR-630 / W286) + **actor ordering** (ADR-631 / W287).  
> **Not** Wave A exit. Q-L4 `[x]` means the seven-domain thin-slice evidence floor is signed; remaining Frozen items stay out of scope.

## Scope

| Domain | Status | Evidence |
|--------|--------|----------|
| **deinit ordering** | **in scope** | scope-exit + arc-overwrite overwrite-before-new |
| **nil sentinel** | **in scope** | ConstNil print; Deque empty-pop; TaskLocalGet missing |
| **exception** | **in scope** | typed throw/catch; catch continues; File IOError catch |
| **overflow** | **in scope** | i64 add/sub/mul wrapping floor (native mir + wasm) |
| **NaN** | **in scope** | special Display + IEEE eq (native mir + wasm) |
| **Unicode** | **in scope** | UTF-8≠NFC eq; combining three-view; isSingleScalar; case-map (native mir + wasm) |
| **actor ordering** | **in scope** | isolation counter; C009 sync self-call; policy fingerprint; serial mutate (native mir + wasm) |

## deinit ordering

When a class instance is replaced or goes out of scope, `deinit` runs **before** the binding is released to the new value or end of scope.

**Canonical program:** [`heap_deinit_order.sy`](../../crates/syx-driver/tests/fixtures/memory/heap_deinit_order.sy)

**Expected stdout (exact):**

```
deinit:
1
done
deinit:
2
```

1. `var n = Node(tag: 1)` then `n = Node(tag: 2)` — old node deinit (tag 1) before assignment completes; new node lives until scope end (tag 2).
2. `print("done")` between the two deinit blocks.

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_deinit_nil.rs` (native llvm strict + wasm AOT); driver_gate `p98_driver_lang_deinit_auto_scope_exit_exact_exec` (VM fold W161).

## nil sentinel

- **ConstNil:** `print(nil)` → stdout `nil` (MirOp ConstNil; W203 inventory real).
- **Deque empty pop:** `Deque.popFront` / `popBack` on empty deque → `nil` (W77 product exact).
- **TaskLocalGet missing:** unknown key → `nil` / `Nil` printed (W275 wasm bridge).

**Evidence nails:** `edge_deinit_nil.rs`; `pilot_const_bool_nil.rs`; `deque.rs`; `task_hosts_wasm.rs`.

## exception

Language-level `try` / `catch` restores control after a throw. The catch binding exposes readable fields (at least `.message` / `.code` for typed errors and stdlib IOError).

1. **Typed throw/catch:** user `throw AppError(code: 42, message: "boom")` → catch prints `boom` then `42`.
2. **Catch continues:** after the catch block, subsequent statements run (e.g. `print("after")`).
3. **Stdlib throws:** `FileHandle.open` on a missing path throws; catch may print `err.code` (exact `1` on File open-missing).

**Canonical typed program:** [`typed_catch.sy`](../../crates/syx-driver/tests/fixtures/errors/typed_catch.sy)

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_exception.rs` (native llvm strict + wasm AOT). Distinct from W127–W132 product HostFn ObjectGet batches — this is an A-04 language edge charter only.

**Not claimed:** full exception taxonomy GA; rethrow / finally GA; actor-ordered throws; Wave A exit.

## overflow

Integer arithmetic on `Int` (i64) uses **two's-complement wrapping** on the observed VM / mir-run / wasm paths (`wrapping_add` / `wrapping_sub` / `wrapping_mul` in runtime-core).

1. **Add wrap:** `9223372036854775807 + 1` → `-9223372036854775808`
2. **Sub wrap:** `-9223372036854775808 - 1` → `9223372036854775807`
3. **Mul wrap:** `4611686018427387904 * 2` → `-9223372036854775808`

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_overflow.rs` (native mir + wasm AOT). Use decimal literals (no `Int.max` boundary property).

**Not claimed:** language-guide debug-panic **R011 overflow** Display GA (runtime always wraps today; R011 also used by HTTP elsewhere); unary `-Int.min` trap GA; llvm `native_strict` i64 large-literal parity (literal truncation debt); Wave A exit.

## NaN

Floating-point specials on the observed VM / mir-run / wasm paths display via `f64::to_string()` and compare with IEEE `==`.

1. **NaN display:** `__syx_f64_sqrt(-1.0)` → `NaN`
2. **inf display:** `__syx_f64_pow(10.0, 400.0)` → `inf`
3. **-inf display:** `__syx_f64_log(0.0)` → `-inf`
4. **IEEE eq:** for `n = sqrt(-1)`, `n == n` → `false`, `n != n` → `true`, `n == 1.0` → `false`

Specials are produced via Math HostFns (not language `0.0/0.0`, which currently traps).

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_nan.rs` (native mir + wasm AOT).

**Not claimed:** language-guide float `/0` → inf/nan GA; relational `<`/`>` NaN semantics GA; `Float.nan` / `.infinity` static API; Wave A exit.

## Unicode

Language-edge Unicode floor on the observed VM / mir-run / wasm paths (HostFn-backed text ops). This is an **A-04/Q-L4 charter** slice — **not** a `text.*` product capability raise and **not** a claim that A-03 product nails are re-credited.

1. **UTF-8 equality ≠ NFC equality:** `d = __syx_str_normalized_nfd("é")` then `d == "é"` → `false`; `__syx_str_normalized_nfc(d) == "é"` → `true`
2. **Combining three-view:** `s = NFD("é")` → `String.length(s)` / `__syx_str_unicode_scalar_count(s)` / `__syx_str_utf8_count(s)` → `1\n2\n3`
3. **isSingleScalar honesty:** `__syx_character_is_single_scalar("a")` / ZWJ family / `"ab"` → `true\nfalse\nfalse`
4. **case-map honesty:** `__syx_str_lowercased("ß")` / `__syx_str_case_folded("ß")` → `ß\nss`

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_unicode.rs` (native mir + wasm AOT). Distinct from product `text.normalization` / `text.three_view` dual-backend cells — those remain A-02/A-03 product evidence.

**Not claimed:** NFKC/NFKD GA; locale/Turkish case mapping; identifier NFC; UTS#39; `text.*` product row raise; HostFn `status=implemented`; Wave A exit.

## actor ordering

Actor serial mailbox floor on the observed **native_mir + wasm AOT** dual-backend paths (ADR-631 plumbing). VM/LLVM concurrency nails remain behavioral oracles and are **not** counted as A-04 dual-backend evidence.

1. **Isolation counter:** `Counter(0); increment; increment; get` → `2`
2. **C009 sync self-call:** `bump→inc` under try → `DENIED\nC009` (must not print `OK`)
3. **Policy fingerprint:** `print(__syx_actor_reentrancy_policy())` → contains `serial-no-reenter` (full string `serial-no-reenter;await-hold`)
4. **Serial mutate:** `set(1); set(2); get` → `2`

**Evidence nails:** `crates/syx-driver/tests/stdlib/edge_actor.rs` (native mir + wasm AOT).

**Not claimed:** await-hold thin slice GA; Swift await-reentrancy GA; distributed actor GA; Wave A exit.

## Q-L4 exit charter (ADR-632)

Q-L4 `[x]` is signed when the seven thin-slice domains above each have dual-backend nails plus this document's sections. This window **does not** add an eighth behavioral domain.

**Evidence nails:** six behavior modules `edge_{deinit_nil,exception,overflow,nan,unicode,actor}.rs` + roadmap A-04/Q-L4 `[x]`（F-10 W577：`gate_a04_ql4_exit` meta retired；edge-semantics 仍活）.

## Frozen

- Wave A exit (A-02/A-05/A-06–A-08/A-10 still open)
- await-hold / Swift await-reentrancy GA
- float `/0`→inf/nan book GA; relational NaN; Float.nan API
- R011 overflow Display; unary `-Int.min` trap; llvm native_strict large Int literals
- NFKC/NFKD; locale/Turkish; text.* / websocket product raise
- native=partial→real hard raises for remaining rows (separate charter; **net.tcp stream native=real** signed ADR-635 / W291; **data.sqlite native=real** signed ADR-636 / W292; **net.http client native=real** signed ADR-637 / W293; **net.http.server native=real** signed ADR-638 / W294; status still partial)
- spawn inherit GA; Deadline sleep→C004 GA; process status=implemented

**Next:** W342+ — native hang/HTTPS AOT nails → A-06/Q-S2 exit charter.
