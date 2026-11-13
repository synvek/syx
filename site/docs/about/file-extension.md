---
title: SYX file extension (.sy)
description: SYX source files use the .sy extension (UTF-8). Projects are declared with a syx.toml manifest (name, version, main) and modules are imported with import.
lastUpdated: 2026-11-13
---
# SYX file extension (`.sy`)

SYX source files use the **`.sy`** extension and are UTF-8 encoded. A multi-file project is declared with a **`syx.toml`** manifest at the project root.

## Minimal layout

```text
my-app/
  syx.toml
  src/
    main.sy
```

Example `syx.toml` fields (see the [Quick Guide](/quick-guide/) for the full schema):

```toml
name = "my-app"
version = "0.1.0"
main = "src/main.sy"
```

Modules are pulled in with `import`. The toolchain binary is `syx` (`syx-cli`).

## Association checklist

| Item | Value |
|------|-------|
| Source extension | `.sy` |
| Manifest | `syx.toml` |
| Run | `syx run file.sy` |
| Language server | `syx lsp` |

## Related

- [Install SYX](./install)
- [What is SYX?](./what-is-syx)
- [Quick Guide](/quick-guide/) — CLI, projects, Native / Wasm
