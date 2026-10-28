# SYX Compiler — Instruction Set

[简体中文](./zh/4_InstructionSet.zh-CN.md)

## 1. Overview

SYX uses a **stack bytecode** virtual machine. Instructions are defined in `crates/syx-codegen/src/lib.rs`; the runtime interprets them in `crates/syx-vm/src/lib.rs` via `VmInstance` (historical type alias `WasmVM`).

Compile output `CompiledProgram` contains:

```rust
// crates/syx-codegen/src/lib.rs
pub struct CompiledProgram {
    pub functions: Vec<FunctionUnit>,
    pub globals: Vec<SmolStr>,
    pub strings: Vec<String>,
    pub host_fn_names: Vec<String>,
}

pub struct FunctionUnit {
    pub name: SmolStr,
    pub param_count: u32,
    pub local_count: u32,
    pub instructions: Vec<Instruction>,
}
```

---

## 2. Runtime value model (Value)

VM stack and locals use `Value`:

```rust
// crates/syx-vm/src/lib.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    F64(f64),
    Bool(bool),
    Str(String),
    Nil,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Iterator {
        items: Vec<Value>,
        index: usize,
    },
}
```

Helpers: `as_i32()`, `as_bool()`, `as_str()`.

Codegen also has serializable `ValueType` (`I32` / `F64` / `Bool` / `String` / `Nil` / `Object`) for the MIR path; it corresponds to VM `Value` but is not identical.

---

## 3. Full Instruction enum

```rust
// crates/syx-codegen/src/lib.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    // Stack
    ConstI32(i32),
    ConstF64(f64),
    ConstStr(String),
    ConstBool(bool),
    ConstNil,
    Pop,
    Dup,

    // Locals
    LoadLocal(u32),
    StoreLocal(u32),

    // Globals
    LoadGlobal(u32),
    StoreGlobal(u32),

    // Integer arithmetic
    I32Add, I32Sub, I32Mul, I32Div, I32Mod,
    I32Neg,

    // Floating-point (codegen may emit; VM partially unimplemented)
    F64Add, F64Sub, F64Mul, F64Div,
    F64Neg,

    // Comparison
    I32Eq, I32Ne, I32Lt, I32Le, I32Gt, I32Ge,
    F64Eq, F64Ne,
    BoolAnd, BoolOr, BoolNot,

    // Control flow
    Jump(usize),           // Absolute PC
    JumpIfFalse(usize),
    Block,
    EndBlock,
    Loop,
    EndLoop,
    Return,

    // Calls
    Call(u32),             // Function index
    CallHost(u32),         // Host function index

    // Objects / collections
    StructNew(u32),        // Reserved
    MethodCall(u32),       // MethodId index
    ArrayNew,
    ArrayGet,
    ArraySet,
    ArrayBuild(u32),
    DictBuild(u32),
    ObjectGet,
    StrEq,

    // Garbage collection
    GcCollect,
    GcPin,
    GcUnpin,

    // Exceptions
    BeginTry(usize),       // Catch entry PC
    EndTry,
    Catch,
    JumpIfNil(usize),
    ThrowError,

    // Iterator
    MakeIterator,
    IteratorNext,
    IteratorDone,

    // Misc
    Print,
    ConcatStr,
}
```

---

## 4. Execution model

### 4.1 VmInstance (alias `WasmVM`)

```rust
// crates/syx-vm/src/lib.rs
pub struct VmInstance { // pub type WasmVM = VmInstance;
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub globals: Vec<Value>,
    pub pool: MemoryPool,
    pub gc: Gc,
    pub host_fns: HashMap<String, HostFn>,
    pub printed: Vec<String>,
    try_handlers: Vec<usize>,      // catch PC pushed by BeginTry
    pending_catch: Option<usize>,  // Host exception jump
}
```

`HostFn` type: `fn(&mut Vec<Value>) -> SyxResult<()>`.

### 4.2 Program startup

```rust
pub fn run_program(&mut self, program: &CompiledProgram) -> SyxResult<Value> {
    // 1. Initialize globals
    // 2. Run __init_global (global variable init)
    // 3. Find and run main
}
```

### 4.3 Function calls

`Call(idx)` pops `param_count` arguments from the stack, invokes `run_function_with_args`, pushes the return value.

`Return` sets PC to `code.len()` to end the frame; top of stack is the result.

### 4.4 Jump semantics

| Instruction | Behavior |
|------|------|
| `Jump(n)` | `pc = n` |
| `JumpIfFalse(n)` | Pop; if false `pc = n`, else `pc += 1` |
| `JumpIfNil(n)` | Pop; if `Nil` then `pc = n`, else push value back |

Codegen patches jump targets with **absolute addresses** for `if`/`while`/`??`, etc.

---

## 5. Instruction categories

### 5.1 Constants and stack

| Instruction | Stack effect |
|------|--------|
| `ConstI32(v)` | `[] → [I32(v)]` |
| `ConstF64(v)` | `[] → [F64(v)]` |
| `ConstStr(s)` | `[] → [Str(s)]` |
| `ConstBool(v)` | `[] → [Bool(v)]` |
| `ConstNil` | `[] → [Nil]` |
| `Pop` | `[v] → []` |
| `Dup` | `[v] → [v, v]` |

### 5.2 Locals / globals

| Instruction | Stack effect |
|------|--------|
| `LoadLocal(i)` | `[] → [locals[i]]` |
| `StoreLocal(i)` | `[v] → []` |
| `LoadGlobal(i)` | `[] → [globals[i]]` |
| `StoreGlobal(i)` | `[v] → []` |

Locals are allocated by name via `CodeGenerator::alloc_local`.

### 5.3 Integer operations

| Instruction | Stack effect |
|------|--------|
| `I32Add/Sub/Mul/Div/Mod` | `[a, b] → [result]` |
| `I32Neg` | `[a] → [-a]` |
| `I32Eq/Ne/Lt/Le/Gt/Ge` | `[a, b] → [Bool]` |
| `BoolAnd/Or` | `[a, b] → [Bool]` |
| `BoolNot` | `[a] → [!a]` |

### 5.4 Control-flow blocks

| Instruction | Purpose |
|------|------|
| `Loop` / `EndLoop` | `while` loop markers (with `Jump` back) |
| `Block` / `EndBlock` | Block markers (reserved) |

**if-else pattern** (`compile_stmt`):

```
compile condition
JumpIfFalse → else start
then branch
[Jump → end]        // when else present
else branch
end:
```

### 5.5 Host calls CallHost

Stdlib Host functions use `CallHost(idx)`; `idx` maps to `CompiledProgram.host_fn_names`, then to `HostFnId`.

Implementations in `crates/syx-vm/src/hosts.rs`, e.g.:

| HostFnId | Rust impl | Stack args (pop top-down) |
|----------|-----------|------------------------|
| `FileRead` | `host_file_read` | `path` → `String` |
| `HttpGet` | `host_http_get` | `url` → `HTTPResponse` Object |
| `RegexMatch` | `host_regex_match` | `pattern, text` → `Array` |

On Host error with active `BeginTry`, error message is pushed and `pending_catch = catch_pc`; otherwise propagate `Err`.

Codegen uses `resolve_call` + `lookup_type_method` to compile `File.read(...)` etc. to `CallHost`.

### 5.6 Instance methods MethodCall

```rust
Instruction::MethodCall(u32)  // u32 = MethodId index
```

VM `call_method` flow:

1. Determine argument count from `MethodId`
2. Pop args and receiver
3. Call `hosts::call_string_method` or `host_http_response_json`

String method mapping: `crates/syx-stdlib/src/registry.rs` `STRING_METHODS` / `ARRAY_METHODS`.

**Stack layout** (e.g. `StrSplit`): `[receiver, arg]` → `[Array]`.

### 5.7 Objects and collections

| Instruction | Stack effect | VM behavior |
|------|--------|---------|
| `ArrayBuild(n)` | `n elements + reverse order` → `[Array]` | Pop and build |
| `DictBuild(n)` | `n (value, key) pairs` → `[Object]` | Build `HashMap` |
| `ObjectGet` | `[obj, key]` → `[field]` | Field from `Object`; missing → `Nil` |
| `ArrayGet` | `[arr, idx]` → `[elem]` | Out of bounds → `Nil` |
| `StrEq` | `[a, b]` → `[Bool]` | String equality |
| `ConcatStr` | `[a, b]` → `[Str(a+b)]` | String concat |
| `ToStr` | `[v]` → `[Str(...)]` | Value to string (data class description, etc.) |

`DataClass` / `SealedClass` init functions use `DictBuild` for objects with `__type`, `variant`, etc.

### 5.8 Exception handling

| Instruction | Notes |
|------|------|
| `BeginTry(catch_pc)` | Push `catch_pc` onto `try_handlers` |
| `EndTry` | Pop handler |
| `Catch` | Catch block entry; error value on stack |
| `JumpIfNil(target)` | Optional chain, `??`, force-unwrap checks |
| `ThrowError` | Force unwrap failed → `R007` |

**try/catch codegen** (`Stmt::Try`):

```
BeginTry(0)          // backpatch catch address
try body
EndTry
Jump → end
catch:
Catch
StoreLocal(catch_name)
catch body
end:
```

**Force unwrap `!`**: jump to `ThrowError` when value is `Nil`.

**Unary `try?`**: wrap `BeginTry`/`EndTry`/`Catch`; on failure push empty string.

### 5.9 Iterator

| Instruction | Stack effect |
|------|--------|
| `MakeIterator` | `[iterable] → [Iterator]` |
| `IteratorNext` | `[iter] → [iter', value, true]` or `[false]` |
| `IteratorDone` | `[] → [false]` (placeholder) |

`MakeIterator` supports:

- `Array` → element iteration
- `Str` → per-character iteration
- Non-negative `I32` → `0..n` range
- Other → single-element iteration

**for-in codegen**:

```
compile iterable
MakeIterator
StoreLocal(__iter_slot)
loop:
  LoadLocal(__iter_slot)
  IteratorNext
  JumpIfFalse → end
  StoreLocal(loop_var)
  StoreLocal(__iter_slot)   // update iterator state
  body
  Jump → loop
end:
```

### 5.10 Garbage collection

```rust
// crates/syx-vm/src/lib.rs
pub struct MemoryPool { objects, free_list }
pub struct Gc { pub pinned: Vec<usize> }
```

| Instruction | VM behavior |
|------|---------|
| `GcCollect` | Reclaim unpinned pool slots whose value is `Nil` |
| `GcPin` | Reserved (no-op today) |
| `GcUnpin` | Reserved (no-op today) |

GC is MVP, mainly with `MemoryPool::alloc` for object slot reuse.

### 5.11 Output

| Instruction | Behavior |
|------|------|
| `Print` | Pop value, `println!`, append to `vm.printed` |

---

## 6. CodeGenerator overview

Entry:

```rust
// crates/syx-codegen/src/lib.rs
impl CodeGenerator {
    pub fn compile(module: &Module) -> SyxResult<CompiledProgram>;
    pub fn compile_into(module: &Module, program: &mut CompiledProgram) -> SyxResult<()>;
    pub fn compile_from_mir(mir: &MirModule, program: &mut CompiledProgram) -> SyxResult<()>;
}
```

Compile order:

1. `DataClass` / `SealedClass` / `Extension` first
2. Then other decls (functions, globals, etc.)

Special names:

| Function name | Purpose |
|--------|------|
| `__init_global` | Global variable initialization |
| `{Type}_init` | DataClass constructor |
| `{Sealed}_{Variant}_init` | Sealed class variant constructor |
| `{Type}_{method}` | Extension method |

---

## 7. Codegen examples

### 7.1 Simple add function

SYX source:

```syx
func add(a: Int, b: Int) -> Int {
    return a + b
}
```

Approximate bytecode:

```
LoadLocal(0)    // a
LoadLocal(1)    // b
I32Add
Return
```

### 7.2 if-else

```
// condition
JumpIfFalse → else_pc
// then
Jump → end_pc
else_pc:
// else
end_pc:
```

### 7.3 Optional chain `obj?.method()`

```
LoadLocal(obj)
Dup
JumpIfNil → nil_branch
// method call
Jump → end
nil_branch:
Pop
ConstNil
end:
```

### 7.4 Host call File.read

```
ConstStr("path.txt")
CallHost(host_fn_index(FileRead))
```

### 7.5 String method

```
ConstStr("  hi  ")
MethodCall(MethodId::StrTrim.index())
```

---

## 8. Partially implemented instructions

These exist on `Instruction` but `VmInstance` may still hit unimplemented / catch-all paths (source is authoritative):

- `F64Add/Sub/Mul/Div/Neg`, `F64Eq/Ne`
- `StructNew`, `ArrayNew`, `ArraySet`
- `Block` / `EndBlock` (marker instructions)
- `IteratorDone` (always pushes `false`)

Codegen mainly emits the `I32` path and `ConstF64`; float ops in the VM are still incomplete.

---

## 9. Crate dependencies

```
syx_ast::Module
    → syx-codegen::CodeGenerator → CompiledProgram
    → syx-vm::VmInstance::run_program
         ├── syx-vm/src/hosts.rs   (HostFn implementations)
         └── syx-stdlib            (HostFnId, MethodId, method tables)
```

Optional JIT: `crates/syx-vm/src/jit.rs` `run_with_jit` / `JitCompiler`.

---

## 10. Tests

VM unit test `crates/syx-vm/src/lib.rs::tests::executes_print` runs a minimal program:

```rust
instructions: vec![
    Instruction::ConstStr("Hello, World".into()),
    Instruction::Print,
    Instruction::Return,
]
```

Integration tests: `crates/syx-driver/tests/p0_integration.rs`.
