# SYX SDK

Prebuilt toolchain for running SYX and linking **native** executables as a
**single file** (static `syx_rt`, no `syx_rt.dll`).

`full-native` ships LLVM/LLD inside `bin/syx`. Platform **import libs** live under
`lib/sysroot/<triple>/` so end-user machines do **not** need Visual Studio or
Xcode to link. The resulting binary may still load OS-provided DLLs (`kernel32`,
`libSystem`, …), same idea as Go.

## Layout

```text
syx/                      # archive root (e.g. syx-sdk-0.1.0-macos-aarch64.tar.gz)
  README.md
  LICENSE
  bin/
    syx                 # CLI (syx.exe on Windows)
  lib/
    libsyx_rt.a         # default runtime (same as profiles/full; syx_rt.lib on Windows)
    profiles/
      full/...
      minimal/...
      async-only/...
      async-http/...
      async-file/...
    sysroot/
      <host-triple>/    # link import libs / markers (see below)
  packages/             # standard library sources (reference)
  examples/             # sample programs
  tutorial/             # guided `.sy` lessons
```

## Quick start

```bash
export PATH="$PWD/bin:$PATH"
syx --version
syx doctor --native-sdk
syx run examples/hello.sy
syx build --native examples/hello.sy   # → single executable next to the project
```

Keep `bin/` and `lib/` as siblings. The linker resolves `../lib/libsyx_rt.a` (or
`syx_rt.lib`) and `../lib/sysroot/<triple>/` relative to `bin/syx`.

Override with:

- `SYX_RT_STATIC_LIB` — absolute path to a `.a` / `.lib`
- `SYX_RT_PROFILE=<id>` — `../lib/profiles/<id>/…`
- `SYX_SYSROOT` — absolute path to a sysroot directory (replaces `../lib/sysroot/<triple>`)

## Runtime profiles

| Profile id   | Contents                                      |
|--------------|-----------------------------------------------|
| `full`       | async + HTTP + file hosts (default flat lib)  |
| `minimal`    | no optional host features                     |
| `async-only` | async runtime only                            |
| `async-http` | async + HTTP                                  |
| `async-file` | async + file                                  |

## Sysroot (no local VS / Xcode)

| Platform | What ships in `lib/sysroot/<triple>/` | User machine |
|----------|----------------------------------------|--------------|
| Windows  | MSVC/Windows SDK **import** `.lib` files (`kernel32`, `ucrt`, …) | No VS required to link |
| macOS    | Marker + notes; link uses host `libSystem` / CLT when present | Command Line Tools recommended |
| Linux    | Marker; link uses host glibc (`-lc` …) | Normal libc is enough |

Single-file native builds **statically** link `syx_rt` into your executable. They
do **not** ship or require a separate `syx_rt` DLL/dylib.

## License

See `LICENSE` in this directory (Apache-2.0).
