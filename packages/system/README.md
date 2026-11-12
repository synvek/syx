# System (SA-3-01 / R-47)

Official companion module: `import System`.

Merges former import names (destructive; no aliases):

- `Os` · `Process` · `Env` · `Path` · `IO` · `Time` · `DateTime`

## Domain sources (build-time gen only)

`packages/os`, `packages/process`, `packages/io`, `packages/core/{env,path,time}.sy`, and `packages/datetime` are **build-time domain sources**. Edit those files, then regenerate the aggregate with:

```bash
python3 scripts/gen_system_mod.py
```

Keep `crates/syx-driver/src/embedded_std.rs` `SYSTEM_SOURCE` file list in sync. Users should `import System` — domain dirs are not a supported public import surface (todo-005 F3 honesty; trees not deleted this window).

Runtime/workspace load prepends `packages/internal/system_host.sy` (SA-4-04a); `__syx_*` stay internal.

## R-47 API shape (slice)

| Type | Shape |
|------|--------|
| `Env` | Caseless `enum` namespace (`get` / `set`) |
| `FilePath` | Value `struct` (`rawValue`) + instance methods; String static bridges kept |
| `ProcessInfo` | Caseless `enum` (unchanged) |
