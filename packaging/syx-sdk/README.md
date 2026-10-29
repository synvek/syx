# SYX SDK

Prebuilt toolchain for running SYX and linking **native** executables as a
**single file** (static `syx_rt`, no `syx_rt.dll`).

`full-native` **statically embeds LLD** into `bin/syx` (macOS/Linux `.a`, Windows
MSVC `lld*.lib`) so the SDK is a single CLI binary for the linker driver. If
static embed fails at build time, the CLI may fall back to a prefix `lld` /
`lld-link` (optionally shipped next to `bin/syx`). Native link still needs host
CRT/import libs (Visual Studio Build Tools / Xcode / Command Line Tools). A
bundled `lib/sysroot` is **not** shipped yet.

The resulting binary may still load OS-provided DLLs (`kernel32`, `libSystem`, …),
same idea as Go.

## Layout

```text
syx/                      # archive root (e.g. syx-sdk-0.1.0-macos-aarch64.tar.gz)
  README.md
  LICENSE
  bin/
    syx                 # CLI (syx.exe on Windows); LLD normally linked in-process
  lib/
    libsyx_rt.a         # default runtime (same as profiles/full; syx_rt.lib on Windows)
    profiles/
      full/...
      minimal/...
      async-only/...
      async-http/...
      async-file/...
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
`syx_rt.lib`) relative to `bin/syx`.

Override with:

- `SYX_RT_STATIC_LIB` — absolute path to a `.a` / `.lib`
- `SYX_RT_PROFILE=<id>` — `../lib/profiles/<id>/…`
- `SYX_SYSROOT` — optional override if you maintain a local link sysroot (not shipped)

## Runtime profiles

| Profile id   | Contents                                      |
|--------------|-----------------------------------------------|
| `full`       | async + HTTP + file hosts (default flat lib)  |
| `minimal`    | no optional host features                     |
| `async-only` | async runtime only                            |
| `async-http` | async + HTTP                                  |
| `async-file` | async + file                                  |

## Native link requirements (temporary)

| Platform | Host requirement |
|----------|------------------|
| Windows  | Visual Studio Build Tools / MSVC `link.exe` (or usable `SYX_SYSROOT`) |
| macOS    | Xcode or Command Line Tools |
| Linux    | Normal libc toolchain (`cc` / LLD) |

Single-file native builds **statically** link `syx_rt` into your executable. They
do **not** ship or require a separate `syx_rt` DLL/dylib.

## License

See `LICENSE` in this directory (Apache-2.0).
