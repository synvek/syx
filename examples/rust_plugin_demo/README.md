# rust_plugin_demo — Rust Host plugin (cdylib)

Minimal SYX project that loads a **Rust `cdylib`** at runtime and calls
`__syx_plugin_mygreet_greet` through the Host table (same C ABI as the C
example in [`../plugin_demo`](../plugin_demo/)).

## Layout

```
rust_plugin_demo/
  syx.toml
  main.sy
  run.sh
  plugins/mygreet/
    Cargo.toml          # crate-type = ["cdylib"]
    src/lib.rs          # syx_plugin_init + greet Host
    plugin.json
    build.sh            # cargo build --release + copy dylib/so
```

## Quick start

```bash
cd examples/rust_plugin_demo
chmod +x run.sh plugins/mygreet/build.sh
./run.sh
```

Expected stdout:

```
Hello, SYX!
```

## Manual steps

```bash
./plugins/mygreet/build.sh
export SYX_HOST_PLUGINS=1
export SYX_HOST_PLUGIN_PATH="$PWD/plugins/mygreet/plugin.json"
cargo run -p syx-cli -- run .
```

## How it works

1. Rust exports `syx_plugin_init` with `#[no_mangle] extern "C"` and a
   `#[repr(C)]` `syx_plugin_info` (see `syx_plugin.h`).
2. `build.sh` builds a release cdylib and updates `plugin.json` → `library`.
3. Compile-time: `[plugins].paths` registers `__syx_plugin_mygreet_greet`.
4. Runtime: `dlopen` + `syx_plugin_init`, then CallHost runs the Rust callback.

Compared with the C demo (`plugin_demo/plugins/mycrypto`), this shows the same
ABI can be implemented in Rust without depending on the SYX workspace crates.

See [books/language-guide/33-runtime-extensions.md](../../books/language-guide/33-runtime-extensions.md).
