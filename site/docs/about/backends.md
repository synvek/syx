---
title: SYX backends (VM, LLVM, Wasm)
description: SYX has three backends from one MIR pipeline — stack bytecode VM (syx run), LLVM native AOT (syx build --native), and WebAssembly (syx build --wasm / syx run --wasm).
lastUpdated: 2026-11-13
---
# SYX backends (VM, LLVM, Wasm)

SYX feeds **three** execution / emit backends from **one MIR pipeline**. LLVM is the only **native AOT** backend.

| Backend | Typical command | Role |
|---------|-----------------|------|
| Stack bytecode VM | `syx run` | Fast iteration / default interpret path |
| LLVM native AOT | `syx build --native` | Native machine code |
| WebAssembly | `syx build --wasm` / `syx run --wasm` | Wasm emit and run |

```text
.sy → … → MIR ─┬→ VM bytecode
               ├→ LLVM object / native link
               └→ Wasm
```

## Choosing a backend

- **Day-to-day learning and scripts:** VM (`syx run`)
- **Native binaries:** LLVM AOT (needs the native/LLVM toolchain — see [Install](./install))
- **Browser / Wasm hosts:** `--wasm`

Backend parity and debugging notes live under [Compiler Internals](/compiler/) ([Backend Parity](/compiler/backend-parity), [Backend Debugging](/compiler/backend-debugging)).

## Related

- [What is SYX?](./what-is-syx)
- [Capabilities](./capabilities) — honest feature boundaries
- [Quick Guide](/quick-guide/) — build flags and projects
