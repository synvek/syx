# syx-sdk install layout (W930 / ADR-1274 · claimed W931 / ADR-1275)

Thin packaging contract for a deployable SYX SDK tree. This documents directory
layout and env overrides used by [`rt_archive_path`](../../crates/syx-rt/src/link.rs).

**C-11 narrow `[x]`** (W931): install/env/packaging contract claimed. This does
**not** claim multi-platform CI artifact upload or marketplace SDK GA.

## Directory tree

```text
syx-sdk/
  bin/
    syx                 # CLI
  lib/
    libsyx_rt.a         # default / full install archive (flat)
    profiles/
      full/libsyx_rt.a
      minimal/libsyx_rt.a
      async-only/libsyx_rt.a
      async-http/libsyx_rt.a
      async-file/libsyx_rt.a
```

MSVC-style installs may use `syx_rt.lib` in the same locations.

## Env overrides (link lookup order)

1. **`SYX_RT_STATIC_LIB`** — absolute path to a `.a` / `.lib` (or deps dir). Highest priority.
2. **`SYX_RT_PROFILE=<id>`** — feature profile id (`full` / `minimal` / `async-only` / `async-http` / `async-file`).
   Relative to `bin/syx`, looks up:
   - `../lib/profiles/<id>/libsyx_rt.a` (preferred)
   - `../lib/<id>/libsyx_rt.a` (fallback)
3. Compile-time embed / flat `../lib/libsyx_rt.a` / cargo `target/{debug,release}`.

Note: `LinkOptions.rt_profile` (`debug`|`release`) is the **Cargo build profile**, not a feature profile id.

## Honesty

- C-11 **`[x]`** after W931 is **narrow** (docs + lookup + packaging contract).
- Shipping this README ≠ Release full profile matrix CI ≠ marketplace SDK KPI.
