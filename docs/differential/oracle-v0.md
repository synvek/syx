# Differential oracle format v0 (M0-D01 / ADR-133/134)

Fixtures live under `crates/syx-driver/tests/fixtures/differential/suite{1,2,3,4}/`.

Each `name.sy` has a sibling `name.oracle.json`:

```json
{
  "exit": 0,
  "stdout": ["line1", "line2"],
  "stderr": [],
  "skip": {
    "vm": false,
    "mir": false,
    "wasm": true,
    "native": false,
    "reason": {
      "wasm": "WASM file hosts incomplete"
    }
  },
  "expect_error": false,
  "http_loopback": false
}
```

- **exit**: process / embed exit code
- **stdout**: exact printed lines (no trailing empties; no `contains`)
- **skip**: set a backend to `true` when unsupported; **`skip.reason.<backend>` is required** when that backend is skipped (SSOT; harness panics if missing)
- **expect_error**: compile/run must fail (Suite-3 connect-refused)
- **http_loopback**: replace `SYX_DIFF_HTTP_PORT` with harness TCP server
- **Suite-2 paths**: fixtures use placeholder `SYX_DIFF_TMP_PATH`; harness substitutes a per-run temp file
- Runner: `cargo test -p syx-driver --test differential -- --test-threads=1`
- Backends: VM embed, MIR, WASM, native (`SYX_NATIVE_STRICT=1` via child `Command` env only)

## Harness hardening

| Knob | Default | Meaning |
|------|---------|---------|
| `SYX_DIFF_TRACE=1` | off | Print backend start/end |
| `SYX_DIFF_BACKEND_TIMEOUT_SECS` | 120 | Wall timeout for vm/mir/wasm/native-build |
| `SYX_DIFF_NATIVE_RUN_TIMEOUT_SECS` | 60 | Kill hung native child |

On backend `Err` or assert mismatch, artifacts are copied to `target/differential-failures/<fixture>/<backend>/` (CI uploads on failure).

**Must** run with `--test-threads=1`. Process-global rusqlite / plugin registry / MIR pin remain shared; Suite-4 calls `reset_rusqlite_store` per run.

## Skip summary (details live in each `*.oracle.json` `skip.reason`)

| Suite | VM | MIR | WASM | native |
|-------|----|-----|------|--------|
| Suite-1 arith/branch/call/string | exact | exact | exact | exact |
| Suite-1 array/dict | exact | exact | skip | skip |
| Suite-2 file_text / file_exists / file_overwrite | exact | exact | skip | exact |
| Suite-2 file_handle_* | exact | exact | skip | skip |
| Suite-3 HTTP | exact | exact | skip | skip |
| Suite-4 SQLite | exact | exact | skip | skip |

Never silent-ignore: skipped backends must stay listed in `skip` with `reason`.
