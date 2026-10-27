# SYXBC Text Format v1

Human-readable dump of SYX VM bytecode ([`CompiledProgram`](../../crates/syx-codegen/src/lib.rs)).

## Header

```text
; syxbc-text v1
entry <function_name>
```

## Sections

| Section | Description |
|---------|-------------|
| `[strings]` | String pool: `index "literal"` |
| `[globals]` | Global names: `index name` |
| `[host_fns]` | Host function names: `index name` |
| `[opaque_witnesses]` | Optional: `function protocol` |
| `[functions]` | Function index table: `index name` |
| `[function N name]` | Per-function body |

## Function block

```text
[function 0 main]
params <count>
locals <count>
source <file.sy>          ; optional
local_names self name      ; optional
local_types String String  ; optional
 0 ConstStr 0
 1 Print
 2 Return
spans                      ; optional debug spans
 0 12 5
 1 -
```

## Instructions

Opcode names match the `Instruction` enum in `syx-codegen` (e.g. `Call`, `Jump`, `AwaitSuspend resume_fn=1 state=0`).

- Jump targets use **PC indices** (`Jump 5`).
- `Call` / `ActorCall` use indices from `[functions]`.
- `ConstStr` uses indices from `[strings]`.
- `CallHost` uses indices from `[host_fns]`.

## CLI

See [9_UsageGuide.md — 字节码与 IR 导出](9_UsageGuide.md#字节码与-ir-导出--syx-emit).

```bash
syx emit examples/hello.sy --format syxbc
syx emit examples --format syxbc -o out.syxbc
```

Produced by `syx-codegen::format_compiled_program`. v1 is **write-only** (no text parser).
