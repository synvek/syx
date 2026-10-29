# Third-party patches

## `llvm-sys`

Vendored from crates.io `llvm-sys` 211.1.1 with a one-line Windows fix:

Official Windows LLVM builds are linked with a static CRT and embed rpmalloc.
The full-native **CLI** build therefore passes `+crt-static` via `--config`
(see `scripts/build-full-native`); it must not sit in `.cargo/config.toml` or
`syx_rt.lib` breaks `syx build --native`. Upstream `llvm-sys` treats
`+crt-static` as “link every `llvm-config --system-libs` entry as `static=`”,
which is wrong for MSVC import libraries (`psapi.lib`, …).

Our `build.rs` always uses `dylib=` for system libs on `msvc`.
