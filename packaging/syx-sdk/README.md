# SYX SDK

Prebuilt toolchain layout for running SYX and linking **native** executables.

Native `syx build --native` links against `libsyx_rt` next to the CLI. A CLI-only
binary is not enough: without the runtime archive under `lib/`, native builds fail.

## Layout

```text
syx-sdk/
  README.md
  LICENSE
  bin/
    syx                 # CLI (syx.exe on Windows)
  lib/
    libsyx_rt.a         # default runtime (same as profiles/full)
    profiles/
      full/libsyx_rt.a
      minimal/libsyx_rt.a
      async-only/libsyx_rt.a
      async-http/libsyx_rt.a
      async-file/libsyx_rt.a
```

On Windows MSVC installs, the archive is named `syx_rt.lib` in the same places.

## Quick start

```bash
# Unpack, then put bin/ on PATH (or invoke bin/syx directly)
export PATH="$PWD/bin:$PATH"
syx --version
syx run examples/hello.sy
syx build --native examples/hello.sy
```

Keep `bin/` and `lib/` as siblings. The linker resolves `../lib/libsyx_rt.a`
relative to `bin/syx`.

## Runtime profiles

| Profile id   | Contents                                      |
|--------------|-----------------------------------------------|
| `full`       | async + HTTP + file hosts (default flat lib)  |
| `minimal`    | no optional host features                     |
| `async-only` | async runtime only                            |
| `async-http` | async + HTTP                                  |
| `async-file` | async + file                                  |

Select a non-default archive with:

```bash
export SYX_RT_PROFILE=minimal   # uses ../lib/profiles/minimal/...
# or
export SYX_RT_STATIC_LIB=/absolute/path/to/libsyx_rt.a
```

### Link lookup order

1. **`SYX_RT_STATIC_LIB`** — absolute path to a `.a` / `.lib` (or a deps directory)
2. **`SYX_RT_PROFILE=<id>`** — relative to `bin/syx`:
   - `../lib/profiles/<id>/libsyx_rt.a` (preferred)
   - `../lib/<id>/libsyx_rt.a` (fallback)
3. Flat **`../lib/libsyx_rt.a`**, then cargo `target/{debug,release}` when developing from source

`LinkOptions.rt_profile` (`debug` | `release`) is the Cargo build profile, not a feature profile id.

## License

See `LICENSE` in this directory (Apache-2.0).
