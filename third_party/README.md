# Third-party patches

## `llvm-sys`

Vendored from crates.io `llvm-sys` 211.1.1 with two SYX fixes:

1. **Windows / MSVC:** Official Windows LLVM builds use a static CRT and embed
   rpmalloc. The full-native **CLI** build therefore passes `+crt-static` via
   `--config` (see `scripts/build-full-native`); it must not sit in
   `.cargo/config.toml` or `syx_rt.lib` breaks `syx build --native`. Upstream
   `llvm-sys` treats `+crt-static` as “link every `llvm-config --system-libs`
   entry as `static=`”, which is wrong for MSVC import libraries (`psapi.lib`,
   …). Our `build.rs` always uses `dylib=` for system libs on `msvc`.

2. **macOS:** Sibling / GitHub LLVM prefixes often ship `libc++` under
   `--libdir` with `@rpath`. Upstream’s `-L$libdir -lc++` then picks that copy
   and the final `syx` link fails with undefined `operator new` / `__cxa_*`.
   We stage a link-search directory that omits `libc++` / `libunwind` so
   `-lc++` resolves to the system/SDK library instead.
