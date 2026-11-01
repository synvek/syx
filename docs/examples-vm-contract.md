# Examples VM allowlist contract (S1-10)

**Status:** CI-gated (S1-I20/I21, **26** files). **S1-10 scoped `[x]`** per [ADR: VM inventory scope](adr/stage1-s1-10-vm-inventory-scope.md). **Not** stage-1 exit. **Not** three-backend GA. **Not** “examples runnable rate full”.

## Scope

This slice freezes a **VM-only** allowlist and asserts:

1. `syx check <file>` succeeds
2. `syx run <file>` succeeds on the **default VM** backend

Script: [`scripts/examples_vm_contract.sh`](../scripts/examples_vm_contract.sh)  
CI: `.github/workflows/ci.yml` runs the script after `cargo build -p syx-cli` (alongside S0 smoke).

## VM single-file runnable inventory

**In-inventory (this allowlist):** tracked single-file demos / `examples/*/main.sy` that `check`+`run` on VM in a clean clone.

**Out of inventory (explicit):**

| Category | Examples | Why out |
|----------|----------|---------|
| GUI / SynUI | `gui_demo.sy`, `service_di/` | feature-off / non-VM product surface |
| HTTP server / client | `httpserver_demo.sy`, `httpclient_demo.sy`, `http_*_tool/` | blocking serve / network flaky |
| Network demos | `io_net_demo.sy` | external HTTP |
| TCP microbench | `tcp_microbench/` | noisy localhost reject loops; not contract material |
| SQL microbench | `sql_microbench/` | bench harness; `sql_tool` already covers SQL thin path |
| LinAlg / faerlin | `linalg/*` | HostFn / plugin path; HostFn+1 frozen |
| Plugins / HostFn | `plugin_demo/`, `rust_plugin_demo/` | HostFn+1 frozen; not VM-contract |
| WASM backends | `wasm_web/`, `wasm_wasi/` | not default VM |
| Multi-module | `deps/`, `macro_package/` | project install / import path |
| Testing shell | `test_demo.sy` | no runnable `main` for contract |
| Gitignored local | `hello2.sy` | listed in `.gitignore`; removed from allowlist (S1-I20) |

Claiming “examples runnable rate full” requires clearing Out + three-backend matrix — **still owed**.

## Allowlist (frozen, 26 files)

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
| `examples/observability_demo.sy` | S1-I17 |
| `examples/class_demo.sy` | S1-I18 S060 |
| `examples/collection_demo.sy` | S1-I18 S060 |
| `examples/test.sy` | S1-I19 |
| `examples/path_tool/main.sy` | S1-I19 |
| `examples/file_tool/main.sy` | S1-I19 |
| `examples/path_microbench/main.sy` | S1-I19 |
| `examples/concurrency_control/main.sy` | S1-I19 |
| `examples/os_info/main.sy` | S1-I19 |
| `examples/sql_tool/main.sy` | S1-I19 |
| `examples/stmt_token_macro/main.sy` | S1-I19 |
| `examples/token_expr_macro/main.sy` | S1-I19 |
| `examples/decl_param_macro/main.sy` | S1-I20 |
| `examples/codable_associated_enum/main.sy` | S1-I20 |

## Honest boundaries

- **In (S1-10 scoped):** default VM `check` + `run` for the allowlist above; CI enforcement of that script; OUT table. See [ADR](adr/stage1-s1-10-vm-inventory-scope.md).
- **Out / defer:** native / wasm full matrix; Out-of-inventory table promotion; stage-1 exit claim; `book_pass_rate.py` CI hard gate (still draft).
- S0 backend smoke (`scripts/s0_backend_smoke.sh`) covers hello native/wasm fail-closed separately.

## How to run

```bash
cargo build -p syx-cli
scripts/examples_vm_contract.sh
```

Non-zero exit = contract broken.
