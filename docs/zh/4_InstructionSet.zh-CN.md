# SYX 编译器 - 指令集文档

[English](../4_InstructionSet.md)


> **现状对齐：** SYX **117.0** · Phase **139** · `syx-cache-v1` · 许可证 [Apache-2.0](../../LICENSE)  
> 原生 AOT：**仅 LLVM**（`syx build --native`）；VM 默认路径为 `syx run`。  
> 实现状态 SSOT：[books/shared/implementation-status.md](../../books/shared/implementation-status.md) · 文档索引：[README.md](./README.zh-CN.md)

## 1. 概述

SYX 采用**栈式字节码**虚拟机。指令定义在 `crates/syx-codegen/src/lib.rs`，运行时由 `crates/syx-vm/src/lib.rs` 中的 `VmInstance`（历史类型别名 `WasmVM`）解释执行。

编译产物 `CompiledProgram` 包含：

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

## 2. 运行时值模型（Value）

VM 栈与局部变量均使用 `Value`：

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

辅助方法：`as_i32()`、`as_bool()`、`as_str()`。

Codegen 侧另有序列化用的 `ValueType`（`I32` / `F64` / `Bool` / `String` / `Nil` / `Object`），用于 MIR 路径，与 VM `Value` 对应但不完全相同。

---

## 3. 完整 Instruction 枚举

```rust
// crates/syx-codegen/src/lib.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    // 栈操作
    ConstI32(i32),
    ConstF64(f64),
    ConstStr(String),
    ConstBool(bool),
    ConstNil,
    Pop,
    Dup,

    // 局部变量
    LoadLocal(u32),
    StoreLocal(u32),

    // 全局变量
    LoadGlobal(u32),
    StoreGlobal(u32),

    // 整数算术
    I32Add, I32Sub, I32Mul, I32Div, I32Mod,
    I32Neg,

    // 浮点算术（codegen 可生成，VM 部分未实现）
    F64Add, F64Sub, F64Mul, F64Div,
    F64Neg,

    // 比较
    I32Eq, I32Ne, I32Lt, I32Le, I32Gt, I32Ge,
    F64Eq, F64Ne,
    BoolAnd, BoolOr, BoolNot,

    // 控制流
    Jump(usize),           // 绝对 PC
    JumpIfFalse(usize),
    Block,
    EndBlock,
    Loop,
    EndLoop,
    Return,

    // 调用
    Call(u32),             // 函数索引
    CallHost(u32),         // Host 函数索引

    // 对象 / 集合
    StructNew(u32),        // 预留
    MethodCall(u32),       // MethodId 索引
    ArrayNew,
    ArrayGet,
    ArraySet,
    ArrayBuild(u32),
    DictBuild(u32),
    ObjectGet,
    StrEq,

    // 垃圾回收
    GcCollect,
    GcPin,
    GcUnpin,

    // 异常
    BeginTry(usize),       // catch 入口 PC
    EndTry,
    Catch,
    JumpIfNil(usize),
    ThrowError,

    // 迭代器
    MakeIterator,
    IteratorNext,
    IteratorDone,

    // 杂项
    Print,
    ConcatStr,
}
```

---

## 4. 执行模型

### 4.1 VmInstance 结构（别名 `WasmVM`）

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
    try_handlers: Vec<usize>,      // BeginTry 压入的 catch PC
    pending_catch: Option<usize>,  // Host 异常跳转
}
```

`HostFn` 类型：`fn(&mut Vec<Value>) -> SyxResult<()>`。

### 4.2 程序启动

```rust
pub fn run_program(&mut self, program: &CompiledProgram) -> SyxResult<Value> {
    // 1. 初始化 globals
    // 2. 执行 __init_global（全局变量初始化）
    // 3. 查找并执行 main
}
```

### 4.3 函数调用

`Call(idx)` 从栈顶弹出 `param_count` 个参数，调用 `run_function_with_args`，将返回值压栈。

`Return` 将 PC 设为 `code.len()`，结束当前帧；栈顶值为函数结果。

### 4.4 跳转语义

| 指令 | 行为 |
|------|------|
| `Jump(n)` | `pc = n` |
| `JumpIfFalse(n)` | 弹栈；若为假则 `pc = n`，否则 `pc += 1` |
| `JumpIfNil(n)` | 弹栈；若为 `Nil` 则 `pc = n`，否则值回压栈 |

Codegen 在生成 `if`/`while`/`??` 等结构时，采用**回填绝对地址**方式 patch 跳转目标。

---

## 5. 指令分类详解

### 5.1 常量与栈

| 指令 | 栈效果 |
|------|--------|
| `ConstI32(v)` | `[] → [I32(v)]` |
| `ConstF64(v)` | `[] → [F64(v)]` |
| `ConstStr(s)` | `[] → [Str(s)]` |
| `ConstBool(v)` | `[] → [Bool(v)]` |
| `ConstNil` | `[] → [Nil]` |
| `Pop` | `[v] → []` |
| `Dup` | `[v] → [v, v]` |

### 5.2 局部 / 全局变量

| 指令 | 栈效果 |
|------|--------|
| `LoadLocal(i)` | `[] → [locals[i]]` |
| `StoreLocal(i)` | `[v] → []` |
| `LoadGlobal(i)` | `[] → [globals[i]]` |
| `StoreGlobal(i)` | `[v] → []` |

局部槽由 `CodeGenerator::alloc_local` 按名称分配。

### 5.3 整数运算

| 指令 | 栈效果 |
|------|--------|
| `I32Add/Sub/Mul/Div/Mod` | `[a, b] → [result]` |
| `I32Neg` | `[a] → [-a]` |
| `I32Eq/Ne/Lt/Le/Gt/Ge` | `[a, b] → [Bool]` |
| `BoolAnd/Or` | `[a, b] → [Bool]` |
| `BoolNot` | `[a] → [!a]` |

### 5.4 控制流块

| 指令 | 用途 |
|------|------|
| `Loop` / `EndLoop` | `while` 循环标记（配合 `Jump` 回跳） |
| `Block` / `EndBlock` | 块标记（预留） |

**if-else 生成模式**（`compile_stmt`）：

```
compile condition
JumpIfFalse → else 起始
then 分支
[Jump → end]        // 有 else 时
else 分支
end:
```

### 5.5 Host 调用 CallHost

标准库 Host 函数通过 `CallHost(idx)` 调用，`idx` 对应 `CompiledProgram.host_fn_names` 中的名称，再映射到 `HostFnId`。

实现位于 `crates/syx-vm/src/hosts.rs`，例如：

| HostFnId | Rust 实现 | 栈参数（自顶向下弹出） |
|----------|-----------|------------------------|
| `FileRead` | `host_file_read` | `path` → `String` |
| `HttpGet` | `host_http_get` | `url` → `HTTPResponse` Object |
| `RegexMatch` | `host_regex_match` | `pattern, text` → `Array` |

Host 函数出错时，若存在活跃 `BeginTry`，错误消息压栈并 `pending_catch = catch_pc`；否则向上返回 `Err`。

Codegen 通过 `resolve_call` + `lookup_type_method` 将 `File.read(...)` 等编译为 `CallHost`。

### 5.6 实例方法 MethodCall

```rust
Instruction::MethodCall(u32)  // u32 = MethodId 索引
```

VM `call_method` 流程：

1. 按 `MethodId` 确定参数个数
2. 弹出参数与 receiver
3. 调用 `hosts::call_string_method` 或 `host_http_response_json`

字符串方法映射见 `crates/syx-stdlib/src/registry.rs` 的 `STRING_METHODS` / `ARRAY_METHODS`。

**栈布局**（以 `StrSplit` 为例）：`[receiver, arg]` → `[Array]`。

### 5.7 对象与集合

| 指令 | 栈效果 | VM 行为 |
|------|--------|---------|
| `ArrayBuild(n)` | `n 个元素 + 逆序` → `[Array]` | 依次 pop 构建 |
| `DictBuild(n)` | `n 组 (value, key)` → `[Object]` | 构建 `HashMap` |
| `ObjectGet` | `[obj, key]` → `[field]` | 从 `Object` 取字段，缺省 `Nil` |
| `ArrayGet` | `[arr, idx]` → `[elem]` | 越界返回 `Nil` |
| `StrEq` | `[a, b]` → `[Bool]` | 字符串相等比较 |
| `ConcatStr` | `[a, b]` → `[Str(a+b)]` | 字符串拼接 |
| `ToStr` | `[v]` → `[Str(...)]` | 值转字符串（data class description 等） |

`DataClass` / `SealedClass` 初始化函数通过 `DictBuild` 构造带 `__type`、`variant` 等字段的对象。

### 5.8 异常处理

| 指令 | 说明 |
|------|------|
| `BeginTry(catch_pc)` | 将 `catch_pc` 压入 `try_handlers` |
| `EndTry` | 弹出 handler |
| `Catch` | catch 块入口标记；错误值已在栈顶 |
| `JumpIfNil(target)` | Optional 链、`??`、force-unwrap 前置检查 |
| `ThrowError` | 强制解包失败，返回 `R007` |

**try/catch 生成**（`Stmt::Try`）：

```
BeginTry(0)          // 回填 catch 地址
try body
EndTry
Jump → end
catch:
Catch
StoreLocal(catch_name)
catch body
end:
```

**force unwrap `!`**：条件为 `Nil` 时跳至 `ThrowError`。

**一元 `try?`**：包裹 `BeginTry`/`EndTry`/`Catch`，失败时压入空字符串。

### 5.9 迭代器

| 指令 | 栈效果 |
|------|--------|
| `MakeIterator` | `[iterable] → [Iterator]` |
| `IteratorNext` | `[iter] → [iter', value, true]` 或 `[false]` |
| `IteratorDone` | `[] → [false]`（占位） |

`MakeIterator` 支持：

- `Array` → 元素迭代
- `Str` → 按字符迭代
- 非负 `I32` → `0..n` 范围
- 其他 → 单元素迭代

**for-in 生成**：

```
compile iterable
MakeIterator
StoreLocal(__iter_slot)
loop:
  LoadLocal(__iter_slot)
  IteratorNext
  JumpIfFalse → end
  StoreLocal(loop_var)
  StoreLocal(__iter_slot)   // 更新迭代器状态
  body
  Jump → loop
end:
```

### 5.10 垃圾回收

```rust
// crates/syx-vm/src/lib.rs
pub struct MemoryPool { objects, free_list }
pub struct Gc { pub pinned: Vec<usize> }
```

| 指令 | VM 行为 |
|------|---------|
| `GcCollect` | 回收 `pool` 中未 pin 且值为 `Nil` 的槽 |
| `GcPin` | 预留（当前为空操作） |
| `GcUnpin` | 预留（当前为空操作） |

GC 为 MVP 实现，主要配合 `MemoryPool::alloc` 复用对象槽。

### 5.11 输出

| 指令 | 行为 |
|------|------|
| `Print` | 弹栈值，`println!`，追加到 `vm.printed` |

---

## 6. CodeGenerator 概览

入口：

```rust
// crates/syx-codegen/src/lib.rs
impl CodeGenerator {
    pub fn compile(module: &Module) -> SyxResult<CompiledProgram>;
    pub fn compile_into(module: &Module, program: &mut CompiledProgram) -> SyxResult<()>;
    pub fn compile_from_mir(mir: &MirModule, program: &mut CompiledProgram) -> SyxResult<()>;
}
```

编译顺序：

1. 先编译 `DataClass` / `SealedClass` / `Extension`
2. 再编译其余声明（函数、全局变量等）

特殊名称：

| 函数名 | 用途 |
|--------|------|
| `__init_global` | 全局变量初始化 |
| `{Type}_init` | DataClass 构造 |
| `{Sealed}_{Variant}_init` | 密封类变体构造 |
| `{Type}_{method}` | Extension 方法 |

---

## 7. 指令生成示例

### 7.1 简单加法函数

SYX 源码：

```syx
func add(a: Int, b: Int) -> Int {
    return a + b
}
```

近似字节码：

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

### 7.3 Optional 链 `obj?.method()`

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

### 7.4 Host 调用 File.read

```
ConstStr("path.txt")
CallHost(host_fn_index(FileRead))
```

### 7.5 字符串方法

```
ConstStr("  hi  ")
MethodCall(MethodId::StrTrim.index())
```

---

## 8. 未完全实现的指令

以下指令已在 `Instruction` 枚举中定义，但 `VmInstance` 执行路径中可能仍有未实现 / catch-all 分支（以源码为准）：

- `F64Add/Sub/Mul/Div/Neg`、`F64Eq/Ne`
- `StructNew`、`ArrayNew`、`ArraySet`
- `Block` / `EndBlock`（标记性指令）
- `IteratorDone`（固定压入 `false`）

Codegen 当前主要生成 `I32` 路径与 `ConstF64`；浮点运算在 VM 侧待补全。

---

## 9. Crate 依赖关系

```
syx_ast::Module
    → syx-codegen::CodeGenerator → CompiledProgram
    → syx-vm::VmInstance::run_program
         ├── syx-vm/src/hosts.rs   (HostFn 实现)
         └── syx-stdlib            (HostFnId, MethodId, 方法表)
```

可选 JIT 路径见 `crates/syx-vm/src/jit.rs` 的 `run_with_jit` / `JitCompiler`。

---

## 10. 测试

VM 单元测试 `crates/syx-vm/src/lib.rs::tests::executes_print` 演示最小程序：

```rust
instructions: vec![
    Instruction::ConstStr("Hello, World".into()),
    Instruction::Print,
    Instruction::Return,
]
```

集成测试见 `crates/syx-driver/tests/p0_integration.rs`。
