# SYX 编译器 - 语义分析文档

[English](../3_SemanticAnalysis.md)


## 1. 概述

语义分析是 SYX 编译流水线的第三阶段，位于 `crates/syx-semantic/`。该 crate 对 `syx_ast::Module` 进行类型推断、符号解析、作用域管理与跨模块可见性检查，为后续 MIR 与代码生成提供类型信息。

入口 API：

```rust
// crates/syx-semantic/src/lib.rs
pub fn analyze(module: &Module) -> SyxResult<SemanticAnalyzer>
pub fn analyze_with_imports(
    module: &Module,
    imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<SemanticAnalyzer>
```

驱动层 `crates/syx-driver/src/lib.rs` 在编译每个模块时调用 `analyze_with_imports`，将依赖模块的 `public` 导出符号注入分析器。

---

## 2. 核心数据结构

### 2.1 SemanticAnalyzer

```rust
// crates/syx-semantic/src/lib.rs
#[derive(Debug, Default)]
pub struct SemanticAnalyzer {
    pub globals: IndexMap<SmolStr, Symbol>,           // 全局符号表
    pub scope: Scope,                                  // 当前作用域链
    pub exports: IndexMap<SmolStr, TypeId>,             // 本模块导出（预留）
    pub imported_modules: Vec<SmolStr>,                // import 语句记录的模块名
    pub import_exports: IndexMap<SmolStr, IndexMap<SmolStr, TypeId>>, // 依赖模块导出
    pub errors: Vec<SyxDiagnostic>,                    // 累积诊断（当前遇首错即返回）
}
```

### 2.2 Symbol

```rust
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub ty: TypeId,
    pub mutable: bool,
    pub access: AccessLevel,   // 来自 syx_ast::AccessLevel
}
```

与 Swift 版不同，Rust 实现将符号简化为单一 `Symbol` 结构，不再区分 `SymbolKind` / `TypeSymbol` 等层次类型。

### 2.3 Scope

```rust
#[derive(Debug, Default)]
pub struct Scope {
    pub symbols: IndexMap<SmolStr, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn define(&mut self, symbol: Symbol) -> SyxResult<()>;
    pub fn lookup(&self, name: &str) -> Option<&Symbol>;
}
```

作用域通过 `parent` 指针链实现嵌套查找；函数体、块语句、`try` 的 `catch` 参数等都会压入新作用域。

### 2.4 TypeId

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Void,
    Numeric(NumericKind),   // Int8…Int64, UInt*, Int/UInt(平台), Float32/64
    Bool,
    String,
    Optional(Box<TypeId>),
    Array(Box<TypeId>),
    Set(Box<TypeId>),
    Dictionary { key: Box<TypeId>, value: Box<TypeId> },
    Named(SmolStr),
    Unknown,
}
```

`NumericKind` 与平台 `Int`/`UInt` 定义于 `crates/syx-stdlib/src/numeric.rs`：由编译 host 的 `target_pointer_width` 决定（32 位 → `Int32`/`UInt32`，64 位 → `Int64`/`UInt64`）。

类型转换辅助函数：

| 函数 | 位置 | 用途 |
|------|------|------|
| `type_name_to_id(name: &str)` | `syx-semantic` | 将字符串类型名（含标准库与数值别名）映射为 `TypeId` |
| `type_expr_to_id(ty: &TypeExpr)` | `syx-semantic` | 将 AST 类型表达式映射为 `TypeId` |
| `type_id_to_name(ty: &TypeId)` | `syx-semantic` | 类型 → 可读字符串 |
| `expr_types: IndexMap<u32, TypeId>` | `SemanticAnalyzer` | 表达式 span → 类型，供 codegen/MIR 使用 |

---

## 3. 分析流程

### 3.1 analyze_with_imports

```rust
pub fn analyze_with_imports(module, imports) -> SyxResult<Self> {
    let mut analyzer = Self::default();
    analyzer.register_builtins();              // 注册标准库类型与 Host 函数
    // 注入依赖模块导出符号
    for (module_name, symbols) in imports { ... }
    analyzer.collect_decls(&module.decls)?;     // 第一遍：收集顶层符号
    analyzer.analyze_decls(&module.decls)?;     // 第二遍：分析函数体/语句
    if let Some(err) = analyzer.errors.first() {
        return Err(err.clone());
    }
    Ok(analyzer)
}
```

### 3.2 两遍扫描

**第一遍 `collect_decls`**：遍历 `Decl`，向 `globals` 注册符号，不进入函数体。

| 声明类型 | 注册行为 |
|----------|----------|
| `Decl::Import` | 记录 `imported_modules` |
| `Decl::Function` | 全局函数，`ty = Void`（返回类型在调用处推断） |
| `Decl::VarGlobal` | 全局变量，带类型注解或 `Unknown` |
| `Decl::Class/Struct/DataClass/SealedClass/Actor` | `TypeId::Named(name)` |
| `Decl::Extension` | 扩展方法作为全局函数符号 |
| `Decl::Protocol` | `TypeId::Named(name)`，`access = Internal` |

**第二遍 `analyze_decls`**：分析函数体、全局变量初始化、协议约束等。

---

## 4. 标准库集成（syx-stdlib）

语义分析通过 `crates/syx-stdlib/` 获取内置类型、Host 函数与方法表。

### 4.1 register_builtins

```rust
// crates/syx-stdlib/src/lib.rs
pub fn register_builtins() -> Vec<StdlibEntry>;

pub struct StdlibEntry {
    pub name: &'static str,
    pub type_name: &'static str,
}
```

`BUILTIN_TYPES` 包含全部固定宽度数值类型、`Int`/`UInt`（平台别名）、`Float`/`Double`、`Bool`、`String`、`Array`、`Set`、`Dictionary`、`Object`、`Void`，以及 `File`、`Directory`、`HTTP`、`HTTPResponse`、`print`、`Iterator`。

`SemanticAnalyzer::register_builtins` 会：

1. 遍历 `register_builtins()`，将每个条目写入 `globals`
2. 遍历 `HostFnId::all()`，以 `host_fn_return_type` 推断返回类型并注册

### 4.2 HostFnId

```rust
// crates/syx-stdlib/src/registry.rs
pub enum HostFnId {
    FileRead, FileReadLines, FileWrite, FileAppend,
    FileExists, FileDelete, FileCopy, FileMove,
    DirCreate, DirList, DirExists,
    HttpGet, HttpPost, HttpResponseJson,
    RegexMatch,
}
```

Host 函数在 VM 中以 `CallHost(idx)` 调用，索引由 `host_fn_index(id)` 决定。

### 4.3 TYPE_METHODS 与成员推断

```rust
pub struct TypeMethod {
    pub type_name: &'static str,
    pub method: &'static str,
    pub host: Option<HostFnId>,      // 静态类型方法 → Host 调用
    pub instance: Option<MethodId>,  // 实例方法 → MethodCall
}

pub static TYPE_METHODS: LazyLock<Vec<TypeMethod>>;
pub static STRING_METHODS: LazyLock<IndexMap<&'static str, MethodId>>;
pub static ARRAY_METHODS: LazyLock<IndexMap<&'static str, MethodId>>;
```

`infer_member_type` 根据接收者类型查找方法返回类型：

```rust
fn infer_member_type(&self, object_ty: &TypeId, method: &str) -> TypeId {
    // String 实例方法 → lookup_string_method
    // Named 类型 → lookup_type_method → host_fn_return_type / method_return_type
    // 特殊：method == "json" → Object
    // 否则 → Unknown
}
```

---

## 5. 表达式与语句分析

### 5.1 表达式 `analyze_expr`

| 表达式 | 推断类型 |
|--------|----------|
| 整数字面量（无后缀） | 平台 `Int` |
| 整数字面量 + 后缀 | 对应 `NumericKind`（如 `42i8` → `Int8`） |
| 浮点字面量（无后缀） | `Float64` / `Double` |
| 浮点 + 后缀 | `Float32` 等 |
| 字符串 / 布尔 / nil | `String` / `Bool` / 可空 |
| 数组字面量 | `Array<元素统一类型>` |
| 字典字面量 | 默认 `Dictionary<K,V>`（同质）；显式标注 `Object` 时为异质袋 |
| 二元算术 | 数值变宽；浮点参与 → `Float64`；signed/unsigned 混用 → **S013** |
| 显式窄化 | 须显式 cast，否则 **S012** |
| 字面量超范围 | **S011** |
| 集合元素不一致 | **S014** |
| Set 元素非 Hashable | **S015** |
| 比较 / 逻辑 | `Bool` |
| `??` | 右操作数类型 |
| 标识符 | 作用域或 `globals` 查找，失败 → **S002** |
| 成员访问 | `infer_member_type` |
| 赋值 | 检查可变性 → **S003**，类型兼容 → **S004** |

### 5.2 跨模块访问

`analyze_call` 在处理 `module.symbol(...)` 时，若 `import_exports` 中不存在该符号，报错 **S005**：

```
Cannot access internal symbol '{module}.{name}'
```

仅 `public func` 会被驱动层 `export_symbols` 导出到依赖模块。

### 5.3 类型兼容性 `is_compatible`

当前支持的隐式转换：

| 期望 | 实际 | 结果 |
|------|------|------|
| 任意 `Numeric` | 更宽 `Numeric` | 隐式变宽 |
| `Set<T>` | `Array<T>`（元素兼容） | 兼容（数组字面量赋给 Set） |
| `Double` / `Float64` | 整数 Numeric | 兼容（变宽到浮点） |
| `Optional(T)` | `T` | 兼容 |
| `Optional(_)` | `Unknown` | 兼容 |
| `Optional(_)` | `Optional(Unknown)` | 兼容 |

不兼容时通过 `check_assignable` 抛出 **S004**。

### 5.4 语句要点

- `for-in`：循环变量类型固定为 `Int`（MVP）
- `try/catch`：`catch` 参数类型为 `String`
- `if let` / `guard let`：绑定变量类型为 `Unknown`
- `Decl::Protocol`：必须声明 `func next()`，否则 **S006**

---

## 6. 访问控制（AccessLevel）

定义于 `crates/syx-ast/src/lib.rs`：

```rust
pub enum AccessLevel {
    Public,
    Internal,
    Private,
}
```

| 级别 | 语义 |
|------|------|
| `Public` | 可被其他模块 import 并调用（当前仅 `public func` 参与导出） |
| `Internal` | 模块内可见；跨模块访问非导出符号触发 **S005** |
| `Private` | 解析器可标注，语义层尚未做细粒度校验 |

顶层声明默认 `Internal`（除非源码写 `public`）。内置符号与 Host 函数一律 `Public`。

---

## 7. 语义错误码 S001–S015

| 代码 | 触发条件 | 示例消息 |
|------|----------|----------|
| **S001** | 同一作用域重复定义符号 | `Symbol 'x' already defined in this scope` |
| **S002** | 未定义标识符 | `Undefined identifier 'foo'` |
| **S003** | 对不可变变量赋值 | `Cannot assign to immutable 'x'` |
| **S004** | 类型不匹配 | `Type mismatch for 'x': expected Int, found String` |
| **S005** | 访问未导出的模块符号 | `Cannot access internal symbol 'mod.fn'` |
| **S006** | 协议缺少 `next()` | `Protocol 'Iter' must declare func next()` |
| **S011** | 字面量超出目标数值类型范围 | `Literal out of range for Int8` |
| **S012** | 非法窄化转换 | `Narrowing conversion requires explicit cast` |
| **S013** | signed/unsigned 数值混用 | `Mixed signed and unsigned arithmetic` |
| **S014** | 集合字面量元素类型不一致 | `Array elements have inconsistent types` |
| **S015** | Set 元素不满足 Hashable | `Set element type does not satisfy Hashable` |

错误通过 `syx_diagnostics::SyxDiagnostic::with_code("S00x")` 附加代码，由 `analyze_with_imports` 在首错处返回 `Err`。

---

## 8. 作用域层次示例

```
globals（内置 + 顶层声明 + import 导出）
└── 函数 foo 作用域
    ├── 参数 a, b
    └── 块作用域
        └── 局部变量 x
```

函数进入时：`Scope { parent: Some(当前 scope), .. }`，退出时恢复 `parent`。

---

## 9. 与下游 crate 的关系

```
syx_ast::Module
    → syx-semantic::SemanticAnalyzer::analyze_with_imports
    → syx-codegen::CodeGenerator::compile（类型推断较浅，主要依赖 codegen 自身 var_types）
    → syx-vm::VmInstance::run_program（类型别名 `WasmVM`）
```

语义分析当前为 **MVP**：不做循环引用检测、未使用声明警告、类属性初始化检查等高级分析；这些在 Swift 版文档中存在，Rust 版尚未实现。

---

## 10. 协议 Conformance（Equatable / Hashable）

`syx-stdlib::protocols` 维护内置类型的协议表；`type_satisfies_protocol` 与 `type_id_satisfies_protocol` 供语义与 codegen 共用。

| 协议 | 成员 / 行为 |
|------|-------------|
| `Equatable` | `==` / `!=`；两侧须可 Equatable（**S016**） |
| `Hashable` | `hash(into:)`（MVP：Host witness） |
| `CustomStringConvertible` | `.description` → `String` |
| `CustomDebugStringConvertible` | `.debugDescription` → `String` |

`===` / `!==` 结果类型为 `Bool`；值类型与 `==` 等价，class / Object 比较实例标识。

---

## 11. 使用示例

```rust
use syx_ast::Module;
use syx_semantic::SemanticAnalyzer;

let module: Module = /* 解析得到 */;
let analyzer = SemanticAnalyzer::analyze(&module)?;

// 多模块编译
let imports = vec![
    ("utils".into(), exported_symbols_from_utils),
];
let analyzer = SemanticAnalyzer::analyze_with_imports(&module, &imports)?;
```

单元测试见 `crates/syx-semantic/src/lib.rs` 模块 `tests::analyzes_simple_main`。
