# SYX MIR Text Format v1

Human-readable dump of unified MIR ([`MirModule`](../../crates/syx-mir/src/lib.rs)).

## Header

```text
; syx-mir-text v1
```

## Function block

```text
[function main]
params 0 locals 5 entry 0
block 0:
  successors 1 2
  r0 = ConstStr "Hello"
  Print
  Return
block 1:
  ...
flat:
  ...                        ; optional flattened instruction list
```

## Operations

- `MirOp` names match the `MirOp` enum (e.g. `Call Greeter_greet`, `Jump block 2`).
- Register assignments: `rN = <op>`.
- Side-effect-only ops appear without `rN =`.

## CLI

See [9_UsageGuide.md — 字节码与 IR 导出](9_UsageGuide.md#字节码与-ir-导出--syx-emit).

```bash
syx emit examples/hello.sy --format mir
syx emit examples --format all -o emit/
```

Also available: `wasm`, `wat`, `llvm`, `clif` (see `syx emit --help`).

Produced by `syx-mir::format_mir_module`. v1 is **write-only**.
