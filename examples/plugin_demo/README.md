# plugin_demo — dynamic Host plugin end-to-end

Minimal SYX project that loads a **cdylib** plugin at runtime and calls `Crypto.sha256` via the Host table.

## Layout

```
plugin_demo/
  syx.toml              # [plugins].paths + crypto module dependency
  main.sy              # calls __syx_plugin_mycrypto_sha256 directly
  main_with_import.sy  # optional: import Crypto wrapper (needs path dep in syx.toml)
  run.sh                # build plugin + syx run with SYX_HOST_PLUGIN_PATH
  plugins/mycrypto/
    plugin.json         # compile-time + runtime manifest
    mycrypto_plugin.c   # C plugin (SHA-256 Host)
    build.sh            # cc -shared → libmycrypto_plugin.{dylib,so}
    lib/crypto.sy      # thin SYX wrapper
```

## Quick start

```bash
cd examples/plugin_demo
chmod +x run.sh plugins/mycrypto/build.sh
./run.sh
```

Expected stdout (SHA-256 of `"hello"`):

```
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

## Manual steps

```bash
./plugins/mycrypto/build.sh
export SYX_HOST_PLUGINS=1
export SYX_HOST_PLUGIN_PATH="$PWD/plugins/mycrypto/plugin.json"
cargo run -p syx-cli -- run .
```

## How it works

1. **Compile** — `register_compile_plugin_manifests` reads `syx.toml` `[plugins]` and registers `__syx_plugin_mycrypto_sha256` for MIR `CallHost(n)`.
2. **Run** — `load_from_env` dlopens the `.dylib`/`.so`, calls `syx_plugin_init`, registers C callbacks.
3. **C ABI** — the plugin uses `syx_plugin_stack_pop_str` / `syx_plugin_stack_push_str` exported from the `syx` binary.

See [books/language-guide/33-runtime-extensions.md](../../books/language-guide/33-runtime-extensions.md) and [ADR-071](../../docs/adr/architecture-decisions.md).

**Rust cdylib variant:** [`../rust_plugin_demo`](../rust_plugin_demo/) (same C ABI, `crate-type = ["cdylib"]`).
