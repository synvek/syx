# comprehensive_demo — syx comprehensive test example

A standalone syx project covering the language guide, standard library, and host capabilities
with many small cases. The same source targets three backends (VM / native / wasm); each case
prints `[PASS]` / `[FAIL]` / `[SKIP]`, with a summary at the end.

syx is in development: **FAIL and SKIP are valid coverage** (they expose compiler/stdlib gaps);
the default exit code is 0. Only compile-time errors need handling (they must never block the
whole project).

## Running

```bash
# From the repo root (the FilePath.exists case in stdlib.time_path assumes this)
./target/release/syx run examples/comprehensive_demo

# One-shot verification across all three backends
# (full VM report + native staging build verification + wasm failure record)
bash examples/comprehensive_demo/run_all.sh
```

### CLI arguments (`syx run . -- <args>`)

| Argument | Description |
|---|---|
| (none) | Run all 25 suites |
| `--only a,b` | Run only the given suites; accepts full names `lang.basics`, short names `basics`, group names `lang` |
| `--skip a,b` | Skip the given suites (prints `[SKIP] … (user skip)`) |
| `--list` | List suites with backend annotations, then exit |
| `--backend vm\|native\|wasm` | Declare the current backend (default vm); suites whose annotation does not match are SKIPped with a reason |
| `--force` | Ignore backend annotations and force a run |
| `--strict` | Exit with code 1 when any FAIL exists (throw SuiteError.strictFailure) |

Examples:

```bash
syx run examples/comprehensive_demo -- --only basics,strings
syx run examples/comprehensive_demo -- --skip host.net
syx run examples/comprehensive_demo -- --backend native   # all SKIP (see backend matrix)
syx run examples/comprehensive_demo -- --list
```

## Directory layout

```
comprehensive_demo/
├── syx.toml              # name/version/main, no external dependencies
├── main.sy               # the only func main(): arg parsing, suite registry, dispatch, summary
├── harness.sy            # global counters + printing assertions (beginSuite/check/checkMsg/skipCase/endSuite)
├── ac_helper.sy          # cross-file helper module for the access_control suite
├── run_all.sh            # three-backend convenience script (includes native staging logic)
└── suite_*.sy            # one suite per file, entry runXxx() -> (Int, Int, Int), no main
```

Note: `syx run <dir>` only collects `.sy` files in the same directory as main.sy (non-recursive),
and all files in a package share one namespace. Suites are therefore flat `suite_*.sy` files;
no imports are needed between files, and entry function names are globally unique.

## Suite and backend matrix (annotated from observed behavior)

The current compiler build (0.4.0 development) **runs fully on VM only**; native/wasm control-flow
lowering has fundamental defects, so every suite is annotated `vm`. Annotations are data in the
main.sy registry (`backendsFor`/`reasonFor`); once the compiler is fixed, change the corresponding
strings back — no dispatch logic changes needed.

| Group | Suites (17 + 3 + 5 = 25) | vm | native | wasm |
|---|---|---|---|---|
| lang | basics / operators / strings / control_flow / functions / enums / structs_classes / properties / inheritance / errors / generics / protocols / extensions / macros / access_control / memory / collections | ✅ | ⛔ runtime | ⛔ runtime |
| stdlib | math / text_json / time_path | ✅ | ⛔ compile+runtime | ⛔ runtime |
| host | concurrency / simd / io / net / services | ✅ | ⛔ compile+runtime | ⛔ runtime |

Full VM baseline: **25 suites, 371 cases (296 pass / 19 fail / 56 skip)**.

### native blockers (observed)

- Compile time (whole-project build fails; run_all.sh staging removes the blockers so the 17 lang suites build):
  - `stdlib.math`: MIR001 Double lowering not supported
  - `stdlib.text_json`: C020 Int `hash(into:)` not generated
  - `stdlib.time_path`: MIR001 FilePath lowering not supported
  - `host.concurrency`: MIR001 Channel; the other host suites depend on the VM host (host.io host functions return nil)
- Runtime (all suites, including the buildable lang group):
  - `if Bool-var` / `if String==String` conditions: the body is not executed (silently skipped)
  - `if Bool==Bool`, `while Bool`: infinite loops (break has no effect)
  - `switch Bool`: returns the scrutinee instead of the branch value
  - `String !=` returns nil (`==` works); the scheduler already uses the equivalent `!(x == y)` form
  - Only `if true/false` literals and `if Int-comparison` work

### wasm blockers (observed)

- Compile time: MIR001 Unknown variable 'Channel' (whole project)
- Runtime: W001 If underflow; a minimal probe confirmed if statement bodies are not executed

## Harness conventions (harness.sy)

Uses module-level global counters + functions (instead of class instances): the current compiler
has a defect assigning init parameters to multi-stored-property classes, and passing class
instances across functions by value triggers move (S011).

```syx
func runXxx() -> (Int, Int, Int) {
    beginSuite("lang.xxx")          // reset counters and print the suite header
    check("case_name", expr)        // single-expression assertion
    checkMsg("case2", ok, "expected …, actual …")  // detail attached on FAIL
    skipCase("case3", "feature not GA")  // proactively skip a single case
    return endSuite()               // print the subtotal and return (pass, fail, skip)
}
```

Case-writing notes (development-stage compiler constraints):

- Wrap multi-statement cases in a suite-private `func caseXxx() -> Bool` (internal do/catch returns false on error).
- String/class/enum instances are move types: the original variable cannot be reused after passing; copy with `"\(s)"`.
- Generic return values/fields are not Equatable: bind with `let` first, then compare via interpolation-to-string;
  do not call generic functions directly inside interpolation (mis-inferred as Int64) or nest string literals (L001).
- A String argument to `check` may be polluted by an adjacent String variable: when suspicious, bind the Bool result first.
- Do not rely on `import Testing` / `@Test` (that is the VM-only `syx test` path).

## Extension guide: adding a suite

1. Create `suite_xxx.sy` with a unique entry `func runXxx() -> (Int, Int, Int)` (following the harness conventions above).
2. Register in three places in `main.sy` (one line each, so run_all.sh can filter by line):
   - add `"group.xxx"` to the `allSuites()` array;
   - add `if name == "group.xxx" { return runXxx() }` to `dispatch()`;
   - if not supported on all backends, register in `backendsFor()` / `reasonFor()` (currently everything defaults to `vm`).
3. Verify: `syx run examples/comprehensive_demo -- --only xxx`.
4. If the suite breaks the whole-project native build, add its file name to `NATIVE_EXCLUDE_FILES`
   and its suite name to `NATIVE_EXCLUDE_NAMES` in run_all.sh (staging removes it and filters the registry lines automatically).

## Known FAIL list (VM; compiler/stdlib gaps, to be corrected as the language evolves)

basics scientific notation / block-level shadowing, operators short-circuit evaluation and Int8 overflow,
functions closures capturing mutable variables, access_control cross-file private not enforced,
memory weak not auto-nil'd, stdlib Random always 0 / SHA-512 empty / Hasher combine ineffective /
DateTime.format returns ms, concurrency await result not propagated back, simd reduction always 0,
net Object response body not serialized / Url.build drops query, services Metrics.inc does not count /
gauge truncates decimals. See each suite's header comments and the `[FAIL]` output details.