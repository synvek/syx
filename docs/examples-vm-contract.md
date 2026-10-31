# Examples VM allowlist contract (S1-10)

**Status:** expanded (S1-I19, **25** files). **Not** stage-1 exit. **Not** three-backend GA. **Not** “examples runnable rate full”.

## Scope

This slice freezes a **VM-only** allowlist and asserts:

1. `syx check <file>` succeeds
2. `syx run <file>` succeeds on the **default VM** backend

Script: [`scripts/examples_vm_contract.sh`](../scripts/examples_vm_contract.sh)

## VM single-file runnable inventory

**In-inventory (this allowlist):** top-level demos that `check`+`run` on VM, plus selected single-file `examples/*/main.sy` tools/macros already green locally.

**Out of inventory (explicit):**

| Category | Examples | Why out |
|----------|----------|---------|
| GUI / SynUI | `gui_demo.sy`, `service_di/` | feature-off / non-VM product surface |
| HTTP server / client | `httpserver_demo.sy`, `httpclient_demo.sy`, `http_*_tool/` | blocking serve / network flaky |
| Network demos | `io_net_demo.sy` | external HTTP |
| Plugins / HostFn | `plugin_demo/`, `rust_plugin_demo/` | HostFn+1 frozen; not VM-contract |
| WASM backends | `wasm_web/`, `wasm_wasi/` | not default VM |
| Multi-module | `deps/`, `macro_package/` | project install / import path |
| Testing shell | `test_demo.sy` | no runnable `main` for contract |

Claiming “examples runnable rate full” requires clearing Out + three-backend matrix — **still owed**.

## Allowlist (frozen, 25 files)

| File | Notes |
|------|--------|
| `examples/hello.sy` | S0 hello |
| `examples/io_demo.sy` | S0 IO |
| `examples/module_demo.sy` | S0 module |
| `examples/task_demo.sy` | S0 task |
| `examples/basic_demo.sy` | demo |
| `examples/array_demo.sy` | demo |
| `examples/operator_demo.sy` | demo |
| `examples/object_demo.sy` | demo |
| `examples/hot_loop.sy` | S1-I17 |
| `examples/json_demo.sy` | S1-I17 |
| `examples/json_adv_demo.sy` | S1-I17 |
| `examples/log_fields.sy` | S1-I17 |
| `examples/hello2.sy` | S1-I17 |
| `examples/observability_demo.sy` | S1-I17 |
| `examples/class_demo.sy` | S1-I18 S060 |
| `examples/collection_demo.sy` | S1-I18 S060 |
| `examples/test.sy` | S1-I19 (`async main`, ~2s sleep) |
| `examples/path_tool/main.sy` | S1-I19 |
| `examples/file_tool/main.sy` | S1-I19 |
| `examples/path_microbench/main.sy` | S1-I19 |
| `examples/concurrency_control/main.sy` | S1-I19 |
| `examples/os_info/main.sy` | S1-I19 |
| `examples/sql_tool/main.sy` | S1-I19 |
| `examples/stmt_token_macro/main.sy` | S1-I19 |
| `examples/token_expr_macro/main.sy` | S1-I19 |

## Honest boundaries

- **In:** default VM `check` + `run` for the allowlist above.
- **Out:** native / wasm full matrix; Out-of-inventory table; stage-1 exit claim.
- S0 backend smoke (`scripts/s0_backend_smoke.sh`) covers hello native/wasm fail-closed separately; it is **not** this contract.

## How to run

```bash
cargo build -p syx-cli
scripts/examples_vm_contract.sh
```

Non-zero exit = contract broken.
