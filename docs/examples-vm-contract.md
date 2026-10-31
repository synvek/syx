# Examples VM allowlist contract (S1-10)

**Status:** expanded (S1-I17). **Not** stage-1 exit. **Not** three-backend GA. **Not** “examples runnable rate full”.

## Scope

This slice freezes a **VM-only** allowlist of single-file demos and asserts:

1. `syx check <file>` succeeds
2. `syx run <file>` succeeds on the **default VM** backend

Script: [`scripts/examples_vm_contract.sh`](../scripts/examples_vm_contract.sh)

## Allowlist (frozen, 14 files)

| File | Notes |
|------|--------|
| `examples/hello.sy` | S0 hello path |
| `examples/io_demo.sy` | S0 IO thin green |
| `examples/module_demo.sy` | S0 module thin green |
| `examples/task_demo.sy` | S0 task thin green |
| `examples/basic_demo.sy` | single-file demo |
| `examples/array_demo.sy` | single-file demo |
| `examples/operator_demo.sy` | single-file demo |
| `examples/object_demo.sy` | single-file demo |
| `examples/hot_loop.sy` | S1-I17 expand (S0 smoke cousin) |
| `examples/json_demo.sy` | S1-I17 expand |
| `examples/json_adv_demo.sy` | S1-I17 expand |
| `examples/log_fields.sy` | S1-I17 expand |
| `examples/hello2.sy` | S1-I17 expand |
| `examples/observability_demo.sy` | S1-I17 expand |

## Honest boundaries

- **In:** default VM `check` + `run` for the allowlist above.
- **Out:** native / wasm full matrix; project-dir examples (`examples/deps`, tools, plugins, `gui_demo`); claiming “examples runnable rate full” or stage-1 exit.
- Demos that fail `let`+mutating (`class_demo`, `collection_demo`) stay **off** the list until fixed.
- S0 backend smoke (`scripts/s0_backend_smoke.sh`) still covers hello native/wasm fail-closed separately; it is **not** this contract.

## How to run

```bash
cargo build -p syx-cli
scripts/examples_vm_contract.sh
```

Non-zero exit = contract broken.
