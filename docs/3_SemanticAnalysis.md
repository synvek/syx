# SYX Compiler — Semantic Analysis

[简体中文](./zh/3_SemanticAnalysis.zh-CN.md)

## 1. Overview

Semantic analysis is the third stage of the SYX compile pipeline, in `crates/syx-semantic/`. This crate type-checks `syx_ast::Module`: type inference, symbol resolution, scope management, and cross-module visibility, supplying type information for MIR and code generation.

Entry API:

```rust
// crates/syx-semantic/src/lib.rs
pub fn analyze(module: &Module) -> SyxResult<SemanticAnalyzer>
pub fn analyze_with_imports(
    module: &Module,
    imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<SemanticAnalyzer>
```

The driver `crates/syx-driver/src/lib.rs` calls `analyze_with_imports` per module and injects `public` exports from dependency modules into the analyzer.

---

## 2. Core data structures

### 2.1 SemanticAnalyzer

```rust
// crates/syx-semantic/src/lib.rs
#[derive(Debug, Default)]
pub struct SemanticAnalyzer {
    pub globals: IndexMap<SmolStr, Symbol>,           // Global symbol table
    pub scope: Scope,                                  // Current scope chain
    pub exports: IndexMap<SmolStr, TypeId>,             // Module exports (reserved)
    pub imported_modules: Vec<SmolStr>,                // Module names from import statements
    pub import_exports: IndexMap<SmolStr, IndexMap<SmolStr, TypeId>>, // Dependency exports
    pub errors: Vec<SyxDiagnostic>,                    // Accumulated diagnostics (currently fail on first error)
}
```

### 2.2 Symbol

```rust
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub ty: TypeId,
    pub mutable: bool,
    pub access: AccessLevel,   // From syx_ast::AccessLevel
}
```

Unlike the Swift implementation, the Rust code uses a single `Symbol` struct rather than layered `SymbolKind` / `TypeSymbol` types.

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

Scopes nest via `parent` for lookup; function bodies, blocks, `try` catch parameters, etc. push new scopes.

### 2.4 TypeId

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Void,
    Numeric(NumericKind),   // Int8…Int64, UInt*, Int/UInt (platform), Float32/64
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

`NumericKind` and platform `Int`/`UInt` are defined in `crates/syx-stdlib/src/numeric.rs`: driven by compile-host `target_pointer_width` (32-bit → `Int32`/`UInt32`, 64-bit → `Int64`/`UInt64`).

Type helpers:

| Function | Location | Purpose |
|------|------|------|
| `type_name_to_id(name: &str)` | `syx-semantic` | Map string type names (stdlib + numeric aliases) to `TypeId` |
| `type_expr_to_id(ty: &TypeExpr)` | `syx-semantic` | Map AST type expressions to `TypeId` |
| `type_id_to_name(ty: &TypeId)` | `syx-semantic` | Type → readable string |
| `expr_types: IndexMap<u32, TypeId>` | `SemanticAnalyzer` | Expression span → type for codegen/MIR |

---

## 3. Analysis flow

### 3.1 analyze_with_imports

```rust
pub fn analyze_with_imports(module, imports) -> SyxResult<Self> {
    let mut analyzer = Self::default();
    analyzer.register_builtins();              // Register stdlib types and Host functions
    // Inject dependency module exports
    for (module_name, symbols) in imports { ... }
    analyzer.collect_decls(&module.decls)?;     // Pass 1: collect top-level symbols
    analyzer.analyze_decls(&module.decls)?;     // Pass 2: analyze bodies/statements
    if let Some(err) = analyzer.errors.first() {
        return Err(err.clone());
    }
    Ok(analyzer)
}
```

### 3.2 Two-pass scan

**Pass 1 `collect_decls`**: Walk `Decl`, register symbols in `globals`, do not enter function bodies.

| Decl kind | Registration |
|----------|----------|
| `Decl::Import` | Record `imported_modules` |
| `Decl::Function` | Global function, `ty = Void` (return type inferred at use sites) |
| `Decl::VarGlobal` | Global variable with annotation or `Unknown` |
| `Decl::Class/Struct/DataClass/SealedClass/Actor` | `TypeId::Named(name)` |
| `Decl::Extension` | Extension methods as global function symbols |
| `Decl::Protocol` | `TypeId::Named(name)`, `access = Internal` |

**Pass 2 `analyze_decls`**: Analyze function bodies, global initializers, protocol constraints, etc.

---

## 4. Standard library integration (syx-stdlib)

Semantic analysis uses `crates/syx-stdlib/` for built-in types, Host functions, and method tables.

### 4.1 register_builtins

```rust
// crates/syx-stdlib/src/lib.rs
pub fn register_builtins() -> Vec<StdlibEntry>;

pub struct StdlibEntry {
    pub name: &'static str,
    pub type_name: &'static str,
}
```

`BUILTIN_TYPES` includes all fixed-width numeric types, `Int`/`UInt` (platform aliases), `Float`/`Double`, `Bool`, `String`, `Array`, `Set`, `Dictionary`, `Object`, `Void`, plus `File`, `Directory`, `HTTP`, `HTTPResponse`, `print`, `Iterator`.

`SemanticAnalyzer::register_builtins`:

1. Iterates `register_builtins()` and writes each entry to `globals`
2. Iterates `HostFnId::all()`, infers return types via `host_fn_return_type`, and registers

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

Host functions run in the VM as `CallHost(idx)`; index from `host_fn_index(id)`.

### 4.3 TYPE_METHODS and member inference

```rust
pub struct TypeMethod {
    pub type_name: &'static str,
    pub method: &'static str,
    pub host: Option<HostFnId>,      // Static type method → Host call
    pub instance: Option<MethodId>,  // Instance method → MethodCall
}

pub static TYPE_METHODS: LazyLock<Vec<TypeMethod>>;
pub static STRING_METHODS: LazyLock<IndexMap<&'static str, MethodId>>;
pub static ARRAY_METHODS: LazyLock<IndexMap<&'static str, MethodId>>;
```

`infer_member_type` resolves method return types from receiver type:

```rust
fn infer_member_type(&self, object_ty: &TypeId, method: &str) -> TypeId {
    // String instance methods → lookup_string_method
    // Named types → lookup_type_method → host_fn_return_type / method_return_type
    // Special: method == "json" → Object
    // Otherwise → Unknown
}
```

---

## 5. Expression and statement analysis

### 5.1 Expressions `analyze_expr`

| Expression | Inferred type |
|--------|----------|
| Integer literal (no suffix) | Platform `Int` |
| Integer literal + suffix | Corresponding `NumericKind` (e.g. `42i8` → `Int8`) |
| Float literal (no suffix) | `Float64` / `Double` |
| Float + suffix | `Float32`, etc. |
| String / bool / nil | `String` / `Bool` / nullable |
| Array literal | `Array<unified element type>` |
| Dict literal | Default `Dictionary<K,V>` (homogeneous); heterogeneous bag when annotated `Object` |
| Binary arithmetic | Numeric widening; float involved → `Float64`; signed/unsigned mix → **S013** |
| Explicit narrowing | Requires explicit cast, else **S012** |
| Literal out of range | **S011** |
| Inconsistent collection elements | **S014** |
| Set element not Hashable | **S015** |
| Comparison / logic | `Bool` |
| `??` | Right operand type |
| Identifier | Scope or `globals` lookup; failure → **S002** |
| Member access | `infer_member_type` |
| Assignment | Mutability check → **S003**, compatibility → **S004** |

### 5.2 Cross-module access

In `analyze_call`, for `module.symbol(...)`, if the symbol is missing from `import_exports`, report **S005**:

```
Cannot access internal symbol '{module}.{name}'
```

Only `public func` is exported to dependents via driver `export_symbols`.

### 5.3 Type compatibility `is_compatible`

Supported implicit conversions:

| Expected | Actual | Result |
|------|------|------|
| Any `Numeric` | Wider `Numeric` | Implicit widen |
| `Set<T>` | `Array<T>` (compatible elements) | Compatible (array literal to Set) |
| `Double` / `Float64` | Integer Numeric | Compatible (widen to float) |
| `Optional(T)` | `T` | Compatible |
| `Optional(_)` | `Unknown` | Compatible |
| `Optional(_)` | `Optional(Unknown)` | Compatible |

Incompatible assignments throw **S004** via `check_assignable`.

### 5.4 Statement notes

- `for-in`: loop variable type is fixed to `Int` (MVP)
- `try/catch`: catch parameter type is `String`
- `if let` / `guard let`: bound variable type is `Unknown`
- `Decl::Protocol`: must declare `func next()`, else **S006**

---

## 6. Access control (AccessLevel)

Defined in `crates/syx-ast/src/lib.rs`:

```rust
pub enum AccessLevel {
    Public,
    Internal,
    Private,
}
```

| Level | Semantics |
|------|------|
| `Public` | Importable and callable from other modules (currently only `public func` is exported) |
| `Internal` | Module-visible; cross-module use of non-exported symbols → **S005** |
| `Private` | Parser may annotate; semantic layer does not yet enforce fine-grained checks |

Top-level declarations default to `Internal` unless `public` is written. Built-ins and Host functions are always `Public`.

---

## 7. Semantic error codes S001–S015

| Code | Condition | Example message |
|------|----------|----------|
| **S001** | Duplicate symbol in same scope | `Symbol 'x' already defined in this scope` |
| **S002** | Undefined identifier | `Undefined identifier 'foo'` |
| **S003** | Assign to immutable variable | `Cannot assign to immutable 'x'` |
| **S004** | Type mismatch | `Type mismatch for 'x': expected Int, found String` |
| **S005** | Access non-exported module symbol | `Cannot access internal symbol 'mod.fn'` |
| **S006** | Protocol missing `next()` | `Protocol 'Iter' must declare func next()` |
| **S011** | Literal out of target numeric range | `Literal out of range for Int8` |
| **S012** | Illegal narrowing conversion | `Narrowing conversion requires explicit cast` |
| **S013** | Mixed signed/unsigned arithmetic | `Mixed signed and unsigned arithmetic` |
| **S014** | Inconsistent collection literal element types | `Array elements have inconsistent types` |
| **S015** | Set element not Hashable | `Set element type does not satisfy Hashable` |

Errors attach codes via `syx_diagnostics::SyxDiagnostic::with_code("S00x")`; `analyze_with_imports` returns `Err` on the first error.

---

## 8. Scope hierarchy example

```
globals (built-ins + top-level decls + import exports)
└── function foo scope
    ├── parameters a, b
    └── block scope
        └── local x
```

On function entry: `Scope { parent: Some(current scope), .. }`; on exit, restore `parent`.

---

## 9. Downstream crates

```
syx_ast::Module
    → syx-semantic::SemanticAnalyzer::analyze_with_imports
    → syx-codegen::CodeGenerator::compile (shallow inference; codegen relies mainly on its own var_types)
    → syx-vm::VmInstance::run_program (type alias `WasmVM`)
```

Semantic analysis is currently **MVP**: no cycle detection, unused-declaration warnings, or class property initialization checks; those exist in Swift docs but not yet in Rust.

---

## 10. Protocol conformance (Equatable / Hashable)

`syx-stdlib::protocols` maintains built-in protocol tables; `type_satisfies_protocol` and `type_id_satisfies_protocol` are shared by semantic and codegen.

| Protocol | Members / behavior |
|------|-------------|
| `Equatable` | `==` / `!=`; both sides must be Equatable (**S016**) |
| `Hashable` | `hash(into:)` (MVP: Host witness) |
| `CustomStringConvertible` | `.description` → `String` |
| `CustomDebugStringConvertible` | `.debugDescription` → `String` |

`===` / `!==` result type is `Bool`; value types match `==`; class / Object compare instance identity.

---

## 11. Usage example

```rust
use syx_ast::Module;
use syx_semantic::SemanticAnalyzer;

let module: Module = /* from parser */;
let analyzer = SemanticAnalyzer::analyze(&module)?;

// Multi-module compile
let imports = vec![
    ("utils".into(), exported_symbols_from_utils),
];
let analyzer = SemanticAnalyzer::analyze_with_imports(&module, &imports)?;
```

Unit tests: `crates/syx-semantic/src/lib.rs` module `tests::analyzes_simple_main`.
