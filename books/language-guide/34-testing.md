# Unit Testing (@Test)

**Implementation status:** `@Test` discovery and execution **Implemented**; JUnit-shaped hooks (`@TestSetup` / `@TestTeardown` / `@TestBefore` / `@TestAfter`) **Implemented**; recursive `tests/**/*.sy` and single-file `syx test path/to/file.sy` **Implemented**; `import Testing` + `assert` / `assertEq` **Implemented** (explicit import, **not** auto-injected); `@Test async` plus `await` that can complete inside the body (via the `syx test` JSON pass) **Implemented**. For a file without `@Test`, if the entry calls `test(name:fn:)`, those closures run before that entry finishes, and the JSON case name is `{file}.{name}`. **Not implemented:** `Testing` auto-import.

`syx test` compiles the test files into the VM and runs the cases carrying `@Test`; a failure exits non-zero. For a product example see [examples/test_demo.sy](../../examples/test_demo.sy); for CLI details see [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md); for the narrow charter and follow-up slices see [plan/topics/syx-test.md](../../plan/topics/syx-test.md).

## Minimal Working Example

```syx
import Testing

@Test
func adds() {
    assertEq(1 + 1, 2)
    assert(true)
}
```

Put the file under the project's `tests/` directory (nesting is allowed), then:

```bash
syx test .
```

## Discovery Rules

| Input | Behavior |
|------|------|
| `syx test .` (or the project root) | Collects `tests/**/*.sy` recursively; case names include the relative path (e.g. `nested/math.add`) |
| `syx test path/to/file.sy` | Runs only the `@Test`s in that file |
| `--filter substr` | Filters by case-name substring; non-matches are recorded as `skipped` |
| `--format human\|json\|tap\|junit` | Report format (default human) |

Each test file is compiled and run independently.

## Hooks (PascalCase)

| Attribute | Timing |
|------|------|
| `@TestSetup` | Once before all cases in the file |
| `@TestBefore` | Before each `@Test` |
| `@Test` | The case itself |
| `@TestAfter` | After each `@Test` |
| `@TestTeardown` | Once after all cases in the file |

```syx
import Log
import Testing

@TestSetup
func boot() {
    Log.info("once before all")
}

@TestBefore
func prep() {
    Log.info("before each")
}

@Test
func first() {
    assertEq(1 + 1, 2)
}

@TestAfter
func clean() {
    Log.info("after each")
}

@TestTeardown
func done() {
    Log.info("once after all")
}
```

## `import Testing`

[`packages/testing/mod.sy`](../../packages/testing/mod.sy) provides:

| API | Purpose |
|-----|------|
| `assert(condition: Bool)` | If the condition is false → the case fails (Host `Err`, not a process panic) |
| `assertEq(actual: Object, expected: Object)` | If the values are unequal → failure (`value_equals`) |

You **must** write `import Testing`; `syx test` does **not** auto-inject the module. When there is no `@Test`, the closures registered by `test(name:fn:)` run and appear in the JSON (`{file}.{name}`). A file with `@Test` does not open additional sibling cases; if a registered closure fails, the failure text is merged into that `@Test`. `expectEqual` is still not a discovery path. `@Test async func …` may `await` inside its body (same drain shape as `async func main`); an explicit `import Testing` is still required. **Not implemented:** auto-import.

## Relationship to `main`

A test file usually does **not** need `func main()`: the driver discovers by attribute and calls the hooks and cases. A normal `syx run` entry still requires `main`; do not mix the two narratives in the same file.

## Honest Boundaries

- In the Host capability matrix, `TestingAssert` / `TestingAssertEq` are **partial**, **VM `real` only** (the `syx test` product path); no claim of mir/llvm/wasm GA.
- ≠ auto-import, ≠ all HostFns with `status=implemented`, ≠ a complete async test matrix.
