use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{
    BinOp, CallArg, ClosureBody, CompoundAssignOp, Decl, Expr, Literal, Module, Stmt, TypeExpr,
    UnaryOp,
};
use syx_diagnostics::{Span, SyxResult};
use syx_mir::{MirFunction, MirModule, MirOp};
use syx_mono::mangle_name;
use syx_semantic::{
    expr_span_start, normalize_type_arg_name, type_id_collection_base, type_id_to_ir_name,
    type_name_dispatch_base, TypeId,
};
use syx_stdlib::{
    bit_width, default_float_kind, float_literal_bits, from_suffix, from_type_name,
    host_fn_id_by_name, host_fn_index, host_fn_index_by_name, is_equatable_type,
    is_manifest_host_op, is_unsigned, lookup_array_method, lookup_dict_method,
    lookup_http_response_method, lookup_set_method, lookup_sql_row_method, lookup_string_method,
    lookup_type_method, manifest_host_name, method_return_type, resolve_alias, widen_for_binary,
    HostFnId, PLATFORM_INT,
};

mod codegen_diag;
mod syxbc_text;

use codegen_diag::codegen_error_at;
pub use syxbc_text::format_compiled_program;

const MUT_REF_KEY: &str = "__syx_ref";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueType {
    I32,
    F64,
    Bool,
    String,
    Nil,
    Object,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    // Stack / constants
    ConstI32(i32),
    ConstF64(f64),
    ConstNumeric {
        kind: u8,
        bits: u64,
    },
    ConstNumeric128 {
        kind: u8,
        lo: u64,
        hi: u64,
    },
    NumericAdd(u8),
    NumericSub(u8),
    NumericMul(u8),
    NumericDiv(u8),
    NumericMod(u8),
    NumericNeg(u8),
    NumericBitAnd(u8),
    NumericBitOr(u8),
    NumericBitXor(u8),
    NumericBitNot(u8),
    NumericShl(u8),
    NumericShr(u8),
    NumericEq(u8),
    NumericNe(u8),
    NumericLt(u8),
    NumericLe(u8),
    NumericGt(u8),
    NumericGe(u8),
    NumericCast {
        from: u8,
        to: u8,
    },
    SetBuild(u32),
    SimdAdd {
        lanes: u8,
        element: u8,
    },
    SimdSub {
        lanes: u8,
        element: u8,
    },
    SimdMul {
        lanes: u8,
        element: u8,
    },
    SimdDiv {
        lanes: u8,
        element: u8,
    },
    SimdEq {
        lanes: u8,
        element: u8,
    },
    SimdLt {
        lanes: u8,
        element: u8,
    },
    SimdGt {
        lanes: u8,
        element: u8,
    },
    SimdNe {
        lanes: u8,
        element: u8,
    },
    SimdLe {
        lanes: u8,
        element: u8,
    },
    SimdGe {
        lanes: u8,
        element: u8,
    },
    SimdAnd {
        lanes: u8,
        element: u8,
    },
    SimdOr {
        lanes: u8,
        element: u8,
    },
    SimdXor {
        lanes: u8,
        element: u8,
    },
    SimdNot {
        lanes: u8,
        element: u8,
    },
    SimdShl {
        lanes: u8,
        element: u8,
    },
    SimdShr {
        lanes: u8,
        element: u8,
    },
    SimdMin {
        lanes: u8,
        element: u8,
    },
    SimdMax {
        lanes: u8,
        element: u8,
    },
    SimdF32Add {
        lanes: u8,
        element: u8,
    },
    SimdF32Sub {
        lanes: u8,
        element: u8,
    },
    SimdF32Mul {
        lanes: u8,
        element: u8,
    },
    SimdF32Div {
        lanes: u8,
        element: u8,
    },
    SimdF64Add {
        lanes: u8,
        element: u8,
    },
    SimdI16Add {
        lanes: u8,
        element: u8,
    },
    SimdI16Sub {
        lanes: u8,
        element: u8,
    },
    SimdI16Mul {
        lanes: u8,
        element: u8,
    },
    SimdI8Add {
        lanes: u8,
        element: u8,
    },
    SimdI8Sub {
        lanes: u8,
        element: u8,
    },
    SimdI8Mul {
        lanes: u8,
        element: u8,
    },
    SimdI8And {
        lanes: u8,
        element: u8,
    },
    SimdI8Or {
        lanes: u8,
        element: u8,
    },
    SimdI8Xor {
        lanes: u8,
        element: u8,
    },
    SimdI8Not {
        lanes: u8,
        element: u8,
    },
    SimdI8Min {
        lanes: u8,
        element: u8,
    },
    SimdI8Max {
        lanes: u8,
        element: u8,
    },
    SimdI8UMin {
        lanes: u8,
        element: u8,
    },
    SimdI8UMax {
        lanes: u8,
        element: u8,
    },
    SimdI8Abs {
        lanes: u8,
        element: u8,
    },
    SimdI8Neg {
        lanes: u8,
        element: u8,
    },
    SimdI8Extract {
        lanes: u8,
        element: u8,
    },
    SimdI8Replace {
        lanes: u8,
        element: u8,
    },
    SimdI8Eq {
        lanes: u8,
        element: u8,
    },
    SimdI8Ne {
        lanes: u8,
        element: u8,
    },
    SimdI8Le {
        lanes: u8,
        element: u8,
    },
    SimdI8Ge {
        lanes: u8,
        element: u8,
    },
    SimdI8AvgrU {
        lanes: u8,
        element: u8,
    },
    SimdI8Popcnt {
        lanes: u8,
        element: u8,
    },
    SimdI8Bitmask {
        lanes: u8,
        element: u8,
    },
    SimdI8AllTrue {
        lanes: u8,
        element: u8,
    },
    SimdI8AnyTrue {
        lanes: u8,
        element: u8,
    },
    SimdI8Lt {
        lanes: u8,
        element: u8,
    },
    SimdI8Gt {
        lanes: u8,
        element: u8,
    },
    SimdI8AddSat {
        lanes: u8,
        element: u8,
    },
    SimdI8SubSat {
        lanes: u8,
        element: u8,
    },
    SimdI8Shl {
        lanes: u8,
        element: u8,
    },
    SimdI8Shr {
        lanes: u8,
        element: u8,
    },
    SimdI64Add {
        lanes: u8,
        element: u8,
    },
    SimdF64Sub {
        lanes: u8,
        element: u8,
    },
    SimdF64Mul {
        lanes: u8,
        element: u8,
    },
    SimdF64Div {
        lanes: u8,
        element: u8,
    },
    SimdReduceAdd {
        lanes: u8,
        element: u8,
    },
    SimdLoad {
        lanes: u8,
        element: u8,
        offset: u32,
    },
    SimdStore {
        lanes: u8,
        element: u8,
        offset: u32,
    },
    SimdBroadcast {
        lanes: u8,
        element: u8,
    },
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
    // Arithmetic
    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    I32Mod,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    I32Neg,
    F64Neg,
    // Comparison
    I32Eq,
    I32Ne,
    I32Lt,
    I32Le,
    I32Gt,
    I32Ge,
    F64Eq,
    F64Ne,
    BoolAnd,
    BoolOr,
    BoolNot,
    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    Block,
    EndBlock,
    Loop,
    EndLoop,
    Return,
    // Calls
    Call(u32),
    CallHost(u32),
    /// Phase 66: static super-class call (no virtual dispatch).
    CallSuper(u32),
    /// Phase 66: indirect call via fn index on stack (vtable dispatch).
    CallIndirect(u32),
    MakeClosure(u32, u32),
    CallClosure,
    ActorRegister,
    ActorCall(u32),
    CallWitness(u32, u32),
    // Memory / objects
    StructNew(u32),
    MethodCall(u32),
    ArrayNew,
    ArrayGet,
    ArraySet,
    /// Phase 68: homogeneous array sort via Comparable witness compare loop.
    ArrSorted,
    // GC
    GcCollect,
    GcPin,
    GcUnpin,
    /// Phase 75: compact unified managed heap.
    HeapCompact,
    // Exceptions
    BeginTry(usize),
    EndTry,
    Catch,
    JumpIfNil(usize),
    ThrowError,
    BindResult(usize),
    ReturnErr,
    // Iterator
    MakeIterator,
    IteratorNext,
    IteratorDone,
    AwaitPoll,
    AwaitSuspend {
        resume_fn: u32,
        state: u32,
    },
    ActorSend(u32),
    TaskSpawn(u32),
    TaskJoin,
    ArrayBuild(u32),
    DictBuild(u32),
    ObjectGet,
    ObjectSet,
    StrEq,
    StrNe,
    BoolEq,
    BoolNe,
    ValueEq,
    ValueNe,
    RefEq,
    RefNe,
    // Misc
    Print,
    ConcatStr,
    ToStr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionUnit {
    pub name: SmolStr,
    pub param_count: u32,
    pub local_count: u32,
    pub instructions: Vec<Instruction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instruction_spans: Vec<Option<(u32, u32)>>,
    /// Source names for local slots (params + `let`/`var`), index = VM local slot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_names: Vec<SmolStr>,
    /// Static types for local slots (parallel to `local_names`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_types: Vec<SmolStr>,
    /// Source file path/name for `instruction_spans` offsets (debug breakpoint mapping).
    #[serde(default, skip_serializing_if = "SmolStr::is_empty")]
    pub source_name: SmolStr,
    /// Phase 134: PC of first init body stmt (after dict/vtable setup) for `CallSuper` on `_init`.
    #[serde(default)]
    pub init_body_pc: u32,
    /// Phase 134: PC of MutRefCell wrap epilogue (super init stops here).
    #[serde(default)]
    pub init_wrap_pc: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompiledProgram {
    pub functions: Vec<FunctionUnit>,
    pub globals: Vec<SmolStr>,
    pub strings: Vec<String>,
    pub host_fn_names: Vec<String>,
    /// Entry function name; defaults to `main`, or `@Main`-annotated function.
    pub entry_point: SmolStr,
    /// Phase 12: MIR bodies for tier-0.5 JIT fastpath (`SYX_JIT_FASTPATH=1`).
    #[serde(skip)]
    pub mir_functions: indexmap::IndexMap<SmolStr, syx_mir::MirFunction>,
    /// Phase 36: full MIR module for tier-2 Cranelift JIT.
    #[serde(skip)]
    pub mir_module: Option<syx_mir::MirModule>,
    /// Phase 13: opaque return witness table (`func` → protocol name).
    #[serde(default, skip_serializing_if = "indexmap::IndexMap::is_empty")]
    pub opaque_witnesses: indexmap::IndexMap<SmolStr, SmolStr>,
}

impl Default for CompiledProgram {
    fn default() -> Self {
        Self {
            functions: Vec::new(),
            globals: Vec::new(),
            strings: Vec::new(),
            host_fn_names: Vec::new(),
            entry_point: SmolStr::new("main"),
            mir_functions: indexmap::IndexMap::new(),
            mir_module: None,
            opaque_witnesses: indexmap::IndexMap::new(),
        }
    }
}

pub struct CodeGenerator {
    program: CompiledProgram,
    locals: indexmap::IndexMap<SmolStr, u32>,
    local_count: u32,
    imports: indexmap::IndexSet<SmolStr>,
    in_async: bool,
    await_segment: u32,
    compiling_fn: SmolStr,
    dataclass_names: indexmap::IndexSet<SmolStr>,
    dataclass_fields: indexmap::IndexMap<SmolStr, Vec<SmolStr>>,
    /// S1-I2: `(type, field)` → default i32 for omitted memberwise init args.
    dataclass_field_defaults: indexmap::IndexMap<(SmolStr, SmolStr), i32>,
    final_classes: indexmap::IndexMap<SmolStr, Vec<(SmolStr, Vec<SmolStr>)>>,
    enum_classes: indexmap::IndexMap<SmolStr, Vec<(SmolStr, Option<i64>)>>,
    extension_methods: indexmap::IndexSet<(SmolStr, SmolStr)>,
    class_types: indexmap::IndexMap<SmolStr, Vec<SmolStr>>,
    /// Phase 65: single inheritance superclass map (child -> parent).
    class_superclasses: indexmap::IndexMap<SmolStr, SmolStr>,
    /// Phase 65: vtable slot map (class -> method -> mangled fn).
    class_vtables: indexmap::IndexMap<SmolStr, indexmap::IndexMap<SmolStr, SmolStr>>,
    /// Phase 65: methods marked `final` (class, method).
    final_methods: indexmap::IndexSet<(SmolStr, SmolStr)>,
    property_observers: indexmap::IndexMap<
        (SmolStr, SmolStr),
        (Option<Vec<syx_ast::Stmt>>, Option<Vec<syx_ast::Stmt>>),
    >,
    generic_templates: indexmap::IndexSet<SmolStr>,
    generic_enum_templates: indexmap::IndexSet<SmolStr>,
    generic_final_templates: indexmap::IndexSet<SmolStr>,
    function_param_types: indexmap::IndexMap<SmolStr, Vec<Option<TypeExpr>>>,
    mut_ref_aliases: indexmap::IndexMap<SmolStr, SmolStr>,
    mut_ref_params: HashSet<SmolStr>,
    actor_types: indexmap::IndexSet<SmolStr>,
    static_class_methods: indexmap::IndexSet<(SmolStr, SmolStr)>,
    protocol_methods: indexmap::IndexMap<SmolStr, Vec<SmolStr>>,
    var_types: indexmap::IndexMap<SmolStr, SmolStr>,
    expr_types: indexmap::IndexMap<u32, TypeId>,
    instantiations: std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    closure_counter: u32,
    task_body_counter: u32,
    opaque_return_protocol: Option<SmolStr>,
    source_name: SmolStr,
}

impl CodeGenerator {
    // W482 / ADR-826: public AST-only `compile` / `compile_into*` entry points deleted.
    // Prefer [`Self::compile_with_mir`]. Historical inventories (W477–W479) keep
    // `compile_into_api_present=true` / `api_delete_enabled=false` / `retired=false`.

    fn mir_function_instruction_count(func: &MirFunction) -> usize {
        if !func.instructions.is_empty() {
            func.instructions.len()
        } else {
            func.blocks.iter().map(|b| b.instructions.len()).sum()
        }
    }

    fn mir_function_quality(func: &MirFunction) -> u32 {
        let ins = Self::mir_function_instruction_count(func) as u32;
        func.param_count.saturating_mul(1000).saturating_add(ins)
    }

    fn should_replace_bytecode_from_mir(
        existing: &FunctionUnit,
        new_param_count: u32,
        new_instruction_count: usize,
    ) -> bool {
        // Phase 125: MIR `lower_class_init` is the authoritative init body; codegen shells
        // include vtable setup that inflates the quality score and must not block replacement.
        if existing.name.ends_with("_init") && new_instruction_count > 0 {
            return true;
        }
        let old_ins = existing.instructions.len() as u32;
        if old_ins == 0 {
            return true;
        }
        let old_score = existing
            .param_count
            .saturating_mul(1000)
            .saturating_add(old_ins);
        let new_score = new_param_count
            .saturating_mul(1000)
            .saturating_add(new_instruction_count as u32);
        new_score >= old_score
    }

    pub fn compile_from_mir(
        mir: &MirModule,
        module: &Module,
        program: &mut CompiledProgram,
    ) -> SyxResult<()> {
        Self::compile_from_mir_with_source(mir, module, program, "")
    }

    pub fn compile_from_mir_with_source(
        mir: &MirModule,
        module: &Module,
        program: &mut CompiledProgram,
        source_name: &str,
    ) -> SyxResult<()> {
        program.entry_point = entry_point_from_module(module);
        if program.host_fn_names.is_empty() {
            program.host_fn_names = syx_stdlib::all_host_fn_names()
                .into_iter()
                .map(String::from)
                .collect();
        }
        // Apply highest-quality MIR body per function name (extension shells must not clobber class inits).
        let mut best_by_name: indexmap::IndexMap<SmolStr, &MirFunction> = indexmap::IndexMap::new();
        for func in &mir.functions {
            best_by_name
                .entry(func.name.clone())
                .and_modify(|prev| {
                    if Self::mir_function_quality(func) > Self::mir_function_quality(prev) {
                        *prev = func;
                    }
                })
                .or_insert(func);
        }
        for func in best_by_name.values() {
            if !program.functions.iter().any(|f| f.name == func.name) {
                program.functions.push(FunctionUnit {
                    name: func.name.clone(),
                    param_count: func.param_count,
                    local_count: func.local_count,
                    instructions: Vec::new(),
                    instruction_spans: Vec::new(),
                    local_names: Vec::new(),
                    local_types: Vec::new(),
                    source_name: SmolStr::new(source_name),
                    init_body_pc: 0,
                    init_wrap_pc: 0,
                });
            }
        }
        // First pass may bind Call indices before later shells exist. Second pass
        // re-emits from MIR against the final name table (W1059 / ADR-1404).
        for force in [false, true] {
            for func in best_by_name.values() {
                let (instructions, instruction_spans) =
                    Self::mir_function_to_bytecode(func, program)?;
                let new_ins_len = instructions.len();
                if let Some(existing) = program.functions.iter_mut().find(|f| f.name == func.name) {
                    if force
                        || Self::should_replace_bytecode_from_mir(
                            existing,
                            func.param_count,
                            new_ins_len,
                        )
                    {
                        existing.instructions = instructions;
                        existing.instruction_spans = instruction_spans;
                        existing.param_count = func.param_count;
                        existing.local_count = func.local_count;
                        if !func.local_names.is_empty() {
                            existing.local_names = func.local_names.clone();
                        }
                        if !func.local_types.is_empty() {
                            existing.local_types = func.local_types.clone();
                        }
                        if existing.source_name.is_empty() && !source_name.is_empty() {
                            existing.source_name = SmolStr::new(source_name);
                        }
                        if func.init_body_pc > 0 {
                            existing.init_body_pc = func.init_body_pc;
                            existing.init_wrap_pc = func.init_wrap_pc;
                        }
                    }
                } else {
                    program.functions.push(FunctionUnit {
                        name: func.name.clone(),
                        param_count: func.param_count,
                        local_count: func.local_count,
                        instructions,
                        instruction_spans,
                        local_names: func.local_names.clone(),
                        local_types: func.local_types.clone(),
                        source_name: SmolStr::new(source_name),
                        init_body_pc: func.init_body_pc,
                        init_wrap_pc: func.init_wrap_pc,
                    });
                }
            }
        }
        Self::overlay_mod_prefixed_alias_bodies(program);
        Ok(())
    }

    fn mir_function_to_bytecode(
        func: &MirFunction,
        program: &CompiledProgram,
    ) -> SyxResult<(Vec<Instruction>, Vec<Option<(u32, u32)>>)> {
        let mut instructions = Vec::new();
        let mut instruction_spans: Vec<Option<(u32, u32)>> = Vec::new();
        for instr in &func.instructions {
            if let MirOp::SimdShuffle { mode } = &instr.op {
                instructions.push(Instruction::ConstI32(*mode));
                instruction_spans.push(instr.span);
                instructions.push(Instruction::CallHost(host_fn_index(
                    syx_stdlib::HostFnId::SimdShuffle,
                )));
                instruction_spans.push(instr.span);
                continue;
            }
            if let MirOp::SimdI8Shuffle { mode } = &instr.op {
                instructions.push(Instruction::ConstI32(*mode));
                instruction_spans.push(instr.span);
                instructions.push(Instruction::CallHost(host_fn_index(
                    syx_stdlib::HostFnId::SimdI8Shuffle,
                )));
                instruction_spans.push(instr.span);
                continue;
            }
            instructions.push(Self::mir_op_to_instruction(&instr.op, program)?);
            instruction_spans.push(instr.span);
        }
        if !instructions
            .iter()
            .any(|i| matches!(i, Instruction::Return))
        {
            instructions.push(Instruction::Return);
            instruction_spans.push(None);
        }
        Ok((instructions, instruction_spans))
    }

    /// Import re-exports (`Mat_*`) can keep a stale Call index; `mod_Mat_*` is the
    /// package-local body that nested `Mat(...)` actually binds. Overlay twins.
    fn overlay_mod_prefixed_alias_bodies(program: &mut CompiledProgram) {
        let twins: Vec<(SmolStr, usize)> = program
            .functions
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if f.instructions.is_empty() || !f.name.starts_with("mod_") {
                    return None;
                }
                Some((f.name.clone(), i))
            })
            .collect();
        for (mod_name, src_idx) in twins {
            let alias = match mod_name.strip_prefix("mod_") {
                Some(rest) if rest.starts_with("Mat") => rest,
                _ => continue,
            };
            let src = program.functions[src_idx].clone();
            if let Some(dst) = program.functions.iter_mut().find(|f| f.name == alias) {
                if dst.param_count == src.param_count && !src.instructions.is_empty() {
                    dst.instructions = src.instructions;
                    dst.instruction_spans = src.instruction_spans;
                    dst.local_count = src.local_count;
                    if !src.local_names.is_empty() {
                        dst.local_names = src.local_names;
                    }
                    if !src.local_types.is_empty() {
                        dst.local_types = src.local_types;
                    }
                    dst.init_body_pc = src.init_body_pc;
                    dst.init_wrap_pc = src.init_wrap_pc;
                }
            }
        }
    }
    pub fn new() -> Self {
        Self {
            program: CompiledProgram {
                host_fn_names: syx_stdlib::all_host_fn_names()
                    .into_iter()
                    .map(String::from)
                    .collect(),
                ..Default::default()
            },
            locals: indexmap::IndexMap::new(),
            local_count: 0,
            imports: indexmap::IndexSet::new(),
            in_async: false,
            await_segment: 0,
            compiling_fn: SmolStr::new(""),
            dataclass_names: indexmap::IndexSet::new(),
            dataclass_fields: indexmap::IndexMap::new(),
            dataclass_field_defaults: indexmap::IndexMap::new(),
            final_classes: indexmap::IndexMap::new(),
            enum_classes: indexmap::IndexMap::new(),
            extension_methods: indexmap::IndexSet::new(),
            class_types: indexmap::IndexMap::new(),
            class_superclasses: indexmap::IndexMap::new(),
            class_vtables: indexmap::IndexMap::new(),
            final_methods: indexmap::IndexSet::new(),
            property_observers: indexmap::IndexMap::new(),
            generic_templates: indexmap::IndexSet::new(),
            generic_enum_templates: indexmap::IndexSet::new(),
            generic_final_templates: indexmap::IndexSet::new(),
            function_param_types: indexmap::IndexMap::new(),
            mut_ref_aliases: indexmap::IndexMap::new(),
            mut_ref_params: HashSet::new(),
            actor_types: indexmap::IndexSet::new(),
            static_class_methods: indexmap::IndexSet::new(),
            protocol_methods: indexmap::IndexMap::new(),
            var_types: indexmap::IndexMap::new(),
            expr_types: indexmap::IndexMap::new(),
            instantiations: std::collections::HashSet::new(),
            closure_counter: 0,
            task_body_counter: 0,
            opaque_return_protocol: None,
            source_name: SmolStr::new(""),
        }
    }

    /// MIR hybrid path: dataclass/enum shells from MIR (M1-B03); other type decls via AST;
    /// function bodies from MIR.
    pub fn compile_with_mir(
        module: &Module,
        mir: &MirModule,
        program: &mut CompiledProgram,
        semantic: Option<&syx_semantic::SemanticAnalyzer>,
        source_name: Option<&str>,
        prelude_extensions: Option<&indexmap::IndexSet<(SmolStr, SmolStr)>>,
    ) -> SyxResult<()> {
        if program.host_fn_names.is_empty() {
            program.host_fn_names = syx_stdlib::all_host_fn_names()
                .into_iter()
                .map(String::from)
                .collect();
        }
        program.entry_point = entry_point_from_module(module);
        let mut gen = Self::new_with_program(std::mem::take(program));
        gen.register_builtin_protocol_methods();
        if let Some(name) = source_name {
            gen.source_name = SmolStr::new(name);
        }
        if let Some(sem) = semantic {
            gen.expr_types = sem.expr_types.clone();
            gen.instantiations = sem.instantiations.clone();
        }
        ensure_opaque_equatable_witness(&mut gen.program);
        gen.collect_imports(module);
        if let Some(sem) = semantic {
            gen.collect_import_namespaces_from_semantic(sem);
        }
        gen.collect_type_decls(module);
        if let Some(ext) = prelude_extensions {
            gen.extension_methods.extend(ext.iter().cloned());
        }
        // M1-B03: merge MIR dataclass/enum shells before bodies; skip AST shells for those.
        let mut mir_work = mir.clone();
        syx_mir::merge_mir_functions(&mut mir_work, syx_mir::lower_dataclass_shells(module));
        syx_mir::merge_mir_functions(&mut mir_work, syx_mir::lower_enum_shells(module));
        // W468 / ADR-812: FinalClass variant-init shells from MIR.
        syx_mir::merge_mir_functions(&mut mir_work, syx_mir::lower_finalclass_shells(module));
        // W469 / ADR-813: register top-level VarGlobal names + combined `__init_global`.
        for decl in &module.decls {
            if let Decl::VarGlobal { name, .. } = decl {
                gen.program.globals.push(name.clone());
            }
        }
        syx_mir::merge_mir_functions(&mut mir_work, syx_mir::lower_varglobal_inits(module)?);
        // S2-I138 / ADR stage2-s2-03-type-shell-delete: no compile_type_shells hook.
        Self::compile_from_mir_with_source(
            &mir_work,
            module,
            &mut gen.program,
            gen.source_name.as_str(),
        )?;
        let mut best_mir: indexmap::IndexMap<SmolStr, syx_mir::MirFunction> =
            indexmap::IndexMap::new();
        for f in &mir_work.functions {
            let ins_len = Self::mir_function_instruction_count(f);
            if ins_len < 4 {
                continue;
            }
            best_mir
                .entry(f.name.clone())
                .and_modify(|prev| {
                    if Self::mir_function_quality(f) > Self::mir_function_quality(prev) {
                        *prev = f.clone();
                    }
                })
                .or_insert_with(|| f.clone());
        }
        gen.program.mir_functions.extend(best_mir);
        gen.tag_untagged_functions_source();
        *program = gen.program;
        program.mir_module = Some(mir_work);
        Self::collect_opaque_witnesses(module, program);
        Ok(())
    }

    fn collect_opaque_witnesses(module: &Module, program: &mut CompiledProgram) {
        for decl in &module.decls {
            if let Decl::Function {
                name, return_type, ..
            } = decl
            {
                if let Some(TypeExpr::OpaqueSome(proto)) = return_type.as_ref() {
                    if let TypeExpr::Named {
                        name: proto_name, ..
                    } = proto.as_ref()
                    {
                        program
                            .opaque_witnesses
                            .insert(name.clone(), proto_name.clone());
                    }
                }
            }
        }
    }

    fn resolve_fn_index(program: &CompiledProgram, name: &SmolStr) -> SyxResult<u32> {
        program
            .functions
            .iter()
            .position(|f| f.name == *name)
            .map(|i| i as u32)
            .ok_or_else(|| {
                codegen_error_at(
                    format!("Unknown function '{name}'"),
                    None,
                    Span::DUMMY,
                    "C020",
                )
            })
    }

    fn mir_op_to_instruction(op: &MirOp, program: &CompiledProgram) -> SyxResult<Instruction> {
        Ok(match op {
            MirOp::ConstI32(v) => Instruction::ConstI32(*v),
            MirOp::ConstNumeric { kind, bits } => Instruction::ConstNumeric {
                kind: *kind,
                bits: *bits,
            },
            MirOp::ConstNumeric128 { kind, lo, hi } => Instruction::ConstNumeric128 {
                kind: *kind,
                lo: *lo,
                hi: *hi,
            },
            MirOp::ConstStr(s) => Instruction::ConstStr(s.clone()),
            MirOp::ConstBool(v) => Instruction::ConstBool(*v),
            MirOp::ConstNil => Instruction::ConstNil,
            MirOp::LoadLocal(slot) => Instruction::LoadLocal(*slot),
            MirOp::StoreLocal(slot) => Instruction::StoreLocal(*slot),
            MirOp::LoadGlobal(slot) => Instruction::LoadGlobal(*slot),
            MirOp::StoreGlobal(slot) => Instruction::StoreGlobal(*slot),
            MirOp::Add { kind } => Instruction::NumericAdd(*kind),
            MirOp::SimdAdd => Instruction::SimdAdd {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdSub => Instruction::SimdSub {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdMul => Instruction::SimdMul {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdDiv => Instruction::SimdDiv {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdEq => Instruction::SimdEq {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdLt => Instruction::SimdLt {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdGt => Instruction::SimdGt {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdNe => Instruction::SimdNe {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdLe => Instruction::SimdLe {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdGe => Instruction::SimdGe {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdAnd => Instruction::SimdAnd {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdOr => Instruction::SimdOr {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdXor => Instruction::SimdXor {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdNot => Instruction::SimdNot {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdShl => Instruction::SimdShl {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdShr => Instruction::SimdShr {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdMin => Instruction::SimdMin {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdMax => Instruction::SimdMax {
                lanes: 4,
                element: syx_stdlib::NumericKind::I32.to_byte(),
            },
            MirOp::SimdF32Add => Instruction::SimdF32Add {
                lanes: 4,
                element: syx_stdlib::NumericKind::F32.to_byte(),
            },
            MirOp::SimdF32Sub => Instruction::SimdF32Sub {
                lanes: 4,
                element: syx_stdlib::NumericKind::F32.to_byte(),
            },
            MirOp::SimdF32Mul => Instruction::SimdF32Mul {
                lanes: 4,
                element: syx_stdlib::NumericKind::F32.to_byte(),
            },
            MirOp::SimdF32Div => Instruction::SimdF32Div {
                lanes: 4,
                element: syx_stdlib::NumericKind::F32.to_byte(),
            },
            MirOp::SimdI16Add => Instruction::SimdI16Add {
                lanes: 8,
                element: syx_stdlib::NumericKind::I16.to_byte(),
            },
            MirOp::SimdI16Sub => Instruction::SimdI16Sub {
                lanes: 8,
                element: syx_stdlib::NumericKind::I16.to_byte(),
            },
            MirOp::SimdI16Mul => Instruction::SimdI16Mul {
                lanes: 8,
                element: syx_stdlib::NumericKind::I16.to_byte(),
            },
            MirOp::SimdI8Add => Instruction::SimdI8Add {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Sub => Instruction::SimdI8Sub {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Mul => Instruction::SimdI8Mul {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8And => Instruction::SimdI8And {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Or => Instruction::SimdI8Or {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Xor => Instruction::SimdI8Xor {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Not => Instruction::SimdI8Not {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Min => Instruction::SimdI8Min {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Max => Instruction::SimdI8Max {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8UMin => Instruction::SimdI8UMin {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8UMax => Instruction::SimdI8UMax {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Abs => Instruction::SimdI8Abs {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Neg => Instruction::SimdI8Neg {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Extract => Instruction::SimdI8Extract {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Replace => Instruction::SimdI8Replace {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Eq => Instruction::SimdI8Eq {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Ne => Instruction::SimdI8Ne {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Le => Instruction::SimdI8Le {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Ge => Instruction::SimdI8Ge {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8AvgrU => Instruction::SimdI8AvgrU {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Popcnt => Instruction::SimdI8Popcnt {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Bitmask => Instruction::SimdI8Bitmask {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8AllTrue => Instruction::SimdI8AllTrue {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8AnyTrue => Instruction::SimdI8AnyTrue {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Lt => Instruction::SimdI8Lt {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Gt => Instruction::SimdI8Gt {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8AddSat => Instruction::SimdI8AddSat {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8SubSat => Instruction::SimdI8SubSat {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Shl => Instruction::SimdI8Shl {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI8Shr => Instruction::SimdI8Shr {
                lanes: 16,
                element: syx_stdlib::NumericKind::I8.to_byte(),
            },
            MirOp::SimdI64Add => Instruction::SimdI64Add {
                lanes: 2,
                element: syx_stdlib::NumericKind::I64.to_byte(),
            },
            MirOp::SimdF64Add => Instruction::SimdF64Add {
                lanes: 2,
                element: syx_stdlib::NumericKind::F64.to_byte(),
            },
            MirOp::SimdF64Sub => Instruction::SimdF64Sub {
                lanes: 2,
                element: syx_stdlib::NumericKind::F64.to_byte(),
            },
            MirOp::SimdF64Mul => Instruction::SimdF64Mul {
                lanes: 2,
                element: syx_stdlib::NumericKind::F64.to_byte(),
            },
            MirOp::SimdF64Div => Instruction::SimdF64Div {
                lanes: 2,
                element: syx_stdlib::NumericKind::F64.to_byte(),
            },
            MirOp::SimdReduceAdd { lanes, element } => Instruction::SimdReduceAdd {
                lanes: *lanes,
                element: *element,
            },
            MirOp::SimdLoad { lanes, element } => Instruction::SimdLoad {
                lanes: *lanes,
                element: *element,
                offset: 0,
            },
            MirOp::SimdStore { lanes, element } => Instruction::SimdStore {
                lanes: *lanes,
                element: *element,
                offset: 0,
            },
            MirOp::Sub { kind } => Instruction::NumericSub(*kind),
            MirOp::Mul { kind } => Instruction::NumericMul(*kind),
            MirOp::Div { kind } => Instruction::NumericDiv(*kind),
            MirOp::Mod { kind } => Instruction::NumericMod(*kind),
            MirOp::BitAnd { kind } => Instruction::NumericBitAnd(*kind),
            MirOp::BitOr { kind } => Instruction::NumericBitOr(*kind),
            MirOp::BitXor { kind } => Instruction::NumericBitXor(*kind),
            MirOp::Shl { kind } => Instruction::NumericShl(*kind),
            MirOp::Shr { kind } => Instruction::NumericShr(*kind),
            MirOp::Eq => Instruction::NumericEq(PLATFORM_INT.to_byte()),
            MirOp::Ne => Instruction::NumericNe(PLATFORM_INT.to_byte()),
            MirOp::Lt { kind } => Instruction::NumericLt(*kind),
            MirOp::Le { kind } => Instruction::NumericLe(*kind),
            MirOp::Gt { kind } => Instruction::NumericGt(*kind),
            MirOp::Ge { kind } => Instruction::NumericGe(*kind),
            MirOp::And => Instruction::BoolAnd,
            MirOp::Or => Instruction::BoolOr,
            MirOp::Not => Instruction::BoolNot,
            MirOp::Neg { kind } => Instruction::NumericNeg(*kind),
            MirOp::BitNot { kind } => Instruction::NumericBitNot(*kind),
            MirOp::Print => Instruction::Print,
            MirOp::ConcatStr => Instruction::ConcatStr,
            MirOp::ToStr => Instruction::ToStr,
            MirOp::Pop => Instruction::Pop,
            MirOp::Dup => Instruction::Dup,
            MirOp::Jump(target) => Instruction::Jump(*target as usize),
            MirOp::JumpIfFalse(target) => Instruction::JumpIfFalse(*target as usize),
            MirOp::JumpIfNil(target) => Instruction::JumpIfNil(*target as usize),
            MirOp::Call(name) => Instruction::Call(Self::resolve_fn_index(program, name)?),
            MirOp::CallSuper(name) => {
                Instruction::CallSuper(Self::resolve_fn_index(program, name)?)
            }
            MirOp::CallIndirect(n) => Instruction::CallIndirect(*n),
            MirOp::ConstFnIndex(name) => Instruction::ConstI32(
                program
                    .functions
                    .iter()
                    .position(|f| f.name == *name)
                    .unwrap_or(0) as i32,
            ),
            MirOp::CallHost(idx) => Instruction::CallHost(*idx),
            MirOp::MakeClosure(name, cap) => {
                Instruction::MakeClosure(Self::resolve_fn_index(program, name)?, *cap)
            }
            MirOp::CallClosure => Instruction::CallClosure,
            MirOp::ActorCall(name) => {
                Instruction::ActorCall(Self::resolve_fn_index(program, name)?)
            }
            MirOp::ActorRegister => Instruction::ActorRegister,
            MirOp::CallWitness(p, m) => Instruction::CallWitness(*p, *m),
            MirOp::MakeIterator => Instruction::MakeIterator,
            MirOp::IteratorNext => Instruction::IteratorNext,
            MirOp::AwaitPoll => Instruction::AwaitPoll,
            MirOp::AwaitSuspend { resume_fn, state } => Instruction::AwaitSuspend {
                resume_fn: *resume_fn,
                state: *state,
            },
            MirOp::ActorSend(name) => {
                Instruction::ActorSend(Self::resolve_fn_index(program, name)?)
            }
            MirOp::DictBuild(n) => Instruction::DictBuild(*n),
            MirOp::ArrayBuild(n) => Instruction::ArrayBuild(*n),
            MirOp::SetBuild(n) => Instruction::SetBuild(*n),
            MirOp::ObjectGet => Instruction::ObjectGet,
            MirOp::ObjectSet => Instruction::ObjectSet,
            MirOp::HeapCompact => Instruction::HeapCompact,
            MirOp::ArrayGet => Instruction::ArrayGet,
            MirOp::ArraySet => Instruction::ArraySet,
            MirOp::ArrSorted => Instruction::ArrSorted,
            MirOp::ValueEq => Instruction::ValueEq,
            MirOp::OpaqueValueEq => {
                let idx = program
                    .functions
                    .iter()
                    .position(|f| f.name == "__syx_opaque_equatable_eq")
                    .unwrap_or(0) as u32;
                Instruction::CallWitness(0, idx)
            }
            MirOp::ValueNe => Instruction::ValueNe,
            MirOp::RefEq => Instruction::RefEq,
            MirOp::RefNe => Instruction::RefNe,
            MirOp::BeginTry(catch) => Instruction::BeginTry(*catch as usize),
            MirOp::EndTry => Instruction::EndTry,
            MirOp::Catch => Instruction::Catch,
            MirOp::Throw => Instruction::ThrowError,
            MirOp::BindResult(err) => Instruction::BindResult(*err as usize),
            MirOp::ReturnErr => Instruction::ReturnErr,
            MirOp::TaskSpawn(name) => {
                Instruction::TaskSpawn(Self::resolve_fn_index(program, name)?)
            }
            MirOp::TaskJoin => Instruction::TaskJoin,
            MirOp::Return => Instruction::Return,
            MirOp::SimdShuffle { .. } => {
                return Err(codegen_error_at(
                    "SimdShuffle must be expanded before mir_op_to_instruction",
                    None,
                    Span::DUMMY,
                    "C020",
                ));
            }
            MirOp::SimdI8Shuffle { .. } => {
                return Err(codegen_error_at(
                    "SimdI8Shuffle must be expanded before mir_op_to_instruction",
                    None,
                    Span::DUMMY,
                    "C020",
                ));
            }
            MirOp::NumericCast { from, to } => Instruction::NumericCast {
                from: *from,
                to: *to,
            },
        })
    }

    fn register_builtin_protocol_methods(&mut self) {
        self.protocol_methods
            .entry(SmolStr::new("Iterator"))
            .or_insert_with(|| vec![SmolStr::new("next")]);
        self.protocol_methods
            .entry(SmolStr::new("Comparable"))
            .or_insert_with(|| vec![SmolStr::new("compare")]);
        self.protocol_methods
            .entry(SmolStr::new("Equatable"))
            .or_insert_with(|| vec![SmolStr::new("equals")]);
    }

    fn new_with_program(program: CompiledProgram) -> Self {
        Self {
            program,
            locals: indexmap::IndexMap::new(),
            local_count: 0,
            imports: indexmap::IndexSet::new(),
            in_async: false,
            await_segment: 0,
            compiling_fn: SmolStr::new(""),
            dataclass_names: indexmap::IndexSet::new(),
            dataclass_fields: indexmap::IndexMap::new(),
            dataclass_field_defaults: indexmap::IndexMap::new(),
            final_classes: indexmap::IndexMap::new(),
            enum_classes: indexmap::IndexMap::new(),
            extension_methods: indexmap::IndexSet::new(),
            class_types: indexmap::IndexMap::new(),
            class_superclasses: indexmap::IndexMap::new(),
            class_vtables: indexmap::IndexMap::new(),
            final_methods: indexmap::IndexSet::new(),
            property_observers: indexmap::IndexMap::new(),
            generic_templates: indexmap::IndexSet::new(),
            generic_enum_templates: indexmap::IndexSet::new(),
            generic_final_templates: indexmap::IndexSet::new(),
            function_param_types: indexmap::IndexMap::new(),
            mut_ref_aliases: indexmap::IndexMap::new(),
            mut_ref_params: HashSet::new(),
            actor_types: indexmap::IndexSet::new(),
            static_class_methods: indexmap::IndexSet::new(),
            protocol_methods: indexmap::IndexMap::new(),
            var_types: indexmap::IndexMap::new(),
            expr_types: indexmap::IndexMap::new(),
            instantiations: std::collections::HashSet::new(),
            closure_counter: 0,
            task_body_counter: 0,
            opaque_return_protocol: None,
            source_name: SmolStr::new(""),
        }
    }

    fn tag_untagged_functions_source(&mut self) {
        if self.source_name.is_empty() {
            return;
        }
        for f in &mut self.program.functions {
            if f.source_name.is_empty() {
                f.source_name = self.source_name.clone();
            }
        }
    }

    fn collect_type_decls(&mut self, module: &Module) {
        for decl in &module.decls {
            match decl {
                Decl::DataClass { name, fields, .. } => {
                    self.dataclass_names.insert(name.clone());
                    self.dataclass_fields.insert(
                        name.clone(),
                        fields.iter().map(|(f, _)| f.clone()).collect(),
                    );
                }
                Decl::Struct {
                    name,
                    fields,
                    members,
                    ..
                } => {
                    self.dataclass_names.insert(name.clone());
                    self.dataclass_fields.insert(
                        name.clone(),
                        fields.iter().map(|(f, _)| f.clone()).collect(),
                    );
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            init:
                                Some(Expr::Literal {
                                    value: Literal::Integer { value, .. },
                                    ..
                                }),
                            ..
                        } = member
                        {
                            self.dataclass_field_defaults
                                .insert((name.clone(), field.clone()), *value as i32);
                        }
                    }
                }
                Decl::Enum {
                    name,
                    type_params,
                    cases,
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_enum_templates.insert(name.clone());
                    }
                    let mut resolved = Vec::new();
                    let mut next_tag = 0i64;
                    for (case, value, _fields) in cases {
                        let tag = if let Some(expr) = value {
                            match expr {
                                Expr::Literal {
                                    value: Literal::Integer { value: v, .. },
                                    ..
                                } => Some(*v as i64),
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let tag_val = tag.unwrap_or(next_tag);
                        next_tag = tag_val + 1;
                        resolved.push((case.clone(), Some(tag_val)));
                    }
                    self.enum_classes.insert(name.clone(), resolved);
                }
                Decl::FinalClass {
                    name,
                    type_params,
                    variants,
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_final_templates.insert(name.clone());
                    }
                    self.final_classes.insert(
                        name.clone(),
                        variants
                            .iter()
                            .map(|(v, fields)| {
                                (v.clone(), fields.iter().map(|(f, _)| f.clone()).collect())
                            })
                            .collect(),
                    );
                }
                Decl::Extension {
                    type_name, methods, ..
                } => {
                    for method in methods {
                        if let Decl::Function { name, .. } = method {
                            self.extension_methods
                                .insert((type_name.clone(), name.clone()));
                        }
                    }
                }
                Decl::Class {
                    name,
                    superclass,
                    members,
                    ..
                } => {
                    if let Some(base) = superclass {
                        self.class_superclasses.insert(name.clone(), base.clone());
                    }
                    let mut vtable = self
                        .class_superclasses
                        .get(name)
                        .and_then(|base| self.class_vtables.get(base))
                        .cloned()
                        .unwrap_or_default();
                    let fields: Vec<SmolStr> = members
                        .iter()
                        .filter_map(|m| {
                            if let Decl::VarGlobal { name: field, .. } = m {
                                Some(field.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.class_types.insert(name.clone(), fields);
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            will_set,
                            did_set,
                            ..
                        } = member
                        {
                            if will_set.is_some() || did_set.is_some() {
                                self.property_observers.insert(
                                    (name.clone(), field.clone()),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                        if let Decl::Class {
                            name: inner,
                            members: nested,
                            ..
                        } = member
                        {
                            let qualified = SmolStr::new(format!("{name}_{inner}"));
                            let nested_fields: Vec<SmolStr> = nested
                                .iter()
                                .filter_map(|m| {
                                    if let Decl::VarGlobal { name: field, .. } = m {
                                        Some(field.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            self.class_types.insert(qualified, nested_fields);
                        }
                        if let Decl::Function {
                            name: method,
                            type_params,
                            attrs,
                            is_final,
                            ..
                        } = member
                        {
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                self.static_class_methods
                                    .insert((name.clone(), method.clone()));
                            }
                            if !type_params.is_empty()
                                && attrs.iter().any(|a| a.name.as_str() == "static")
                            {
                                self.generic_templates
                                    .insert(SmolStr::new(format!("{name}_{method}")));
                            }
                            let mangled = SmolStr::new(format!("{name}_{method}"));
                            vtable.insert(method.clone(), mangled);
                            if *is_final {
                                self.final_methods.insert((name.clone(), method.clone()));
                            }
                        }
                    }
                    self.class_vtables.insert(name.clone(), vtable);
                }
                Decl::Actor { name, members, .. } => {
                    let fields: Vec<SmolStr> = members
                        .iter()
                        .filter_map(|m| {
                            if let Decl::VarGlobal { name: field, .. } = m {
                                Some(field.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.class_types.insert(name.clone(), fields);
                    self.actor_types.insert(name.clone());
                }
                Decl::Function {
                    name,
                    type_params,
                    params,
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_templates.insert(name.clone());
                    }
                    self.function_param_types
                        .insert(name.clone(), params.iter().map(|p| p.ty.clone()).collect());
                }
                Decl::Protocol { name, methods, .. } => {
                    let method_names: Vec<SmolStr> = methods
                        .iter()
                        .filter_map(|m| {
                            if let Decl::Function { name: method, .. } = m {
                                Some(method.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.protocol_methods.insert(name.clone(), method_names);
                }
                Decl::VarGlobal {
                    attrs,
                    will_set,
                    did_set,
                    ..
                } => {
                    if let Some(attr) = attrs.iter().find(|a| a.name.as_str() == "syx_static_var") {
                        if attr.args.len() >= 2 {
                            let type_key = match &attr.args[0] {
                                syx_ast::AttributeArg::Ident(s) => s.clone(),
                                syx_ast::AttributeArg::String(s) => SmolStr::new(s.as_str()),
                            };
                            let field_key = match &attr.args[1] {
                                syx_ast::AttributeArg::Ident(s) => s.clone(),
                                syx_ast::AttributeArg::String(s) => SmolStr::new(s.as_str()),
                            };
                            if will_set.is_some() || did_set.is_some() {
                                self.property_observers.insert(
                                    (type_key, field_key),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_imports(&mut self, module: &Module) {
        for decl in &module.decls {
            if let Some(symbol) = decl.import_symbol() {
                self.imports.insert(symbol.clone());
            }
        }
    }

    fn collect_import_namespaces_from_semantic(
        &mut self,
        semantic: &syx_semantic::SemanticAnalyzer,
    ) {
        for (mod_name, symbols) in &semantic.import_exports {
            self.imports.insert(mod_name.clone());
            for (name, ty) in symbols {
                if let syx_semantic::TypeId::Named(n) = ty {
                    if name == n {
                        self.imports.insert(name.clone());
                        self.class_types
                            .entry(name.clone())
                            .or_insert_with(Vec::new);
                    }
                }
                if let Some(meta) = syx_meta::reflect_type(name.as_str()) {
                    self.dataclass_names.insert(name.clone());
                    self.dataclass_fields
                        .insert(name.clone(), meta.fields.iter().cloned().collect());
                }
            }
        }
    }

    // S2-I139 / ADR stage2-s2-03-compile-module-delete: `compile_module` trio deleted.
    // S2-I140 / ADR stage2-s2-03-compile-decl-delete: `compile_decl` + dataclass/
    // type/class/enum/`self_param*`/`merged_class_fields` corpse deleted. Live
    // closures still use `compile_function_decl` via `compile_expr`. ≠ S2-03 [x].

    fn register_param_var_type(&mut self, name: &SmolStr, ty: &TypeExpr) {
        match ty {
            TypeExpr::Named { generics, .. } if !generics.is_empty() => {
                self.var_types
                    .insert(name.clone(), Self::type_expr_name(ty));
            }
            TypeExpr::Named {
                name: type_name, ..
            } => {
                self.var_types.insert(name.clone(), type_name.clone());
            }
            TypeExpr::Array(inner) => {
                self.var_types.insert(
                    name.clone(),
                    SmolStr::new(format!("Array<{}>", Self::type_expr_name(inner))),
                );
            }
            TypeExpr::Dictionary { .. } => {
                self.var_types
                    .insert(name.clone(), Self::type_expr_name(ty));
            }
            _ => {}
        }
    }

    fn compile_function_decl(
        &mut self,
        name: &SmolStr,
        params: &[syx_ast::Param],
        body: &[Stmt],
        captures: &[SmolStr],
    ) -> SyxResult<()> {
        let saved_locals = std::mem::take(&mut self.locals);
        let saved_local_count = self.local_count;
        let saved_compiling_fn = self.compiling_fn.clone();
        self.compiling_fn = name.clone();
        self.local_count = 0;
        self.mut_ref_aliases.clear();
        self.mut_ref_params.clear();
        for param in params {
            self.alloc_local(&param.name);
            if let Some(ty) = &param.ty {
                self.register_param_var_type(&param.name, ty);
            }
            if param
                .ty
                .as_ref()
                .is_some_and(|t| matches!(t, TypeExpr::Reference(_, true, _)))
            {
                self.mut_ref_params.insert(param.name.clone());
            }
        }
        if params.first().is_some_and(|p| p.name.as_str() == "self")
            && self.instance_self_mut_ref_method(name)
        {
            self.mut_ref_params.insert(SmolStr::new("self"));
        }
        if let Some((type_name, _)) = name.as_str().split_once('_') {
            if params.first().is_some_and(|p| p.name.as_str() == "self") {
                self.var_types
                    .insert(SmolStr::new("self"), SmolStr::new(type_name));
            }
        }
        for cap in captures {
            self.alloc_local(cap);
        }
        let mut fn_instructions = Vec::new();
        let mut fn_spans = Vec::new();
        let mut span_sink: Option<&mut Vec<Option<(u32, u32)>>> = Some(&mut fn_spans);
        for stmt in body {
            self.compile_stmt(stmt, &mut fn_instructions, &mut span_sink)?;
        }
        self.emit_scope_deinit_locals(&mut fn_instructions)?;
        let has_explicit_return = body.iter().any(|s| matches!(s, Stmt::Return { .. }));
        let returns_self = !has_explicit_return
            && params
                .first()
                .map(|p| p.name.as_str() == "self")
                .unwrap_or(false)
            && self
                .actor_types
                .iter()
                .any(|t| name.starts_with(&format!("{t}_")));
        if returns_self {
            if let Some(&self_slot) = self.locals.get(&SmolStr::new("self")) {
                fn_instructions.push(Instruction::LoadLocal(self_slot));
            }
        }
        fn_instructions.push(Instruction::Return);
        fn_spans.push(None);
        let unit = FunctionUnit {
            name: name.clone(),
            param_count: params.len() as u32,
            local_count: self.local_count,
            instructions: fn_instructions,
            instruction_spans: fn_spans,
            local_names: Self::local_names_for_slots(&self.locals, self.local_count),
            local_types: Self::local_types_for_slots(
                &self.locals,
                &self.var_types,
                params,
                self.local_count,
                name,
            ),
            source_name: self.source_name.clone(),
            init_body_pc: 0,
            init_wrap_pc: 0,
        };
        if let Some(idx) = self.program.functions.iter().position(|f| f.name == *name) {
            self.program.functions[idx] = unit;
        } else {
            self.program.functions.push(unit);
        }
        self.locals = saved_locals;
        self.local_count = saved_local_count;
        self.compiling_fn = saved_compiling_fn;
        Ok(())
    }

    fn compiling_fn_index(&self) -> u32 {
        self.program
            .functions
            .iter()
            .position(|f| f.name == self.compiling_fn)
            .unwrap_or(0) as u32
    }

    fn in_class_init(&self) -> bool {
        self.compiling_fn.ends_with("_init")
    }

    fn member_assign_skip_unwrap_object(&self, object: &Expr) -> bool {
        self.in_class_init()
            && matches!(object, Expr::Ident { name, .. } if name.as_str() == "self")
    }

    fn lookup_property_observer(
        &self,
        ty: &SmolStr,
        field: &SmolStr,
    ) -> Option<(Option<Vec<syx_ast::Stmt>>, Option<Vec<syx_ast::Stmt>>)> {
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if let Some(obs) = self.property_observers.get(&(t.clone(), field.clone())) {
                return Some(obs.clone());
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    fn receiver_class_type(&self, obj_name: &SmolStr) -> Option<SmolStr> {
        if obj_name.as_str() == "self" {
            self.compiling_fn
                .as_str()
                .split_once('_')
                .map(|(ty, _)| SmolStr::new(ty))
        } else {
            self.var_types.get(obj_name).cloned()
        }
    }

    fn is_class_instance_local(&self, name: &SmolStr) -> bool {
        if name.as_str() == "self" {
            return true;
        }
        self.var_types
            .get(name)
            .is_some_and(|t| self.class_types.contains_key(t) || self.actor_types.contains(t))
    }

    fn compile_mut_ref_cell_writeback(
        &mut self,
        name: &SmolStr,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(&slot) = self.locals.get(&resolved) else {
            return Ok(());
        };
        let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
        out.push(Instruction::StoreLocal(temp));
        out.push(Instruction::LoadLocal(slot));
        out.push(Instruction::LoadLocal(temp));
        out.push(Instruction::ConstStr(MUT_REF_KEY.into()));
        out.push(Instruction::ObjectSet);
        out.push(Instruction::StoreLocal(slot));
        Ok(())
    }

    fn emit_unwrap_syx_ref(out: &mut Vec<Instruction>) {
        out.push(Instruction::ConstStr("__syx_ref".into()));
        out.push(Instruction::ObjectGet);
    }

    fn compile_stmt(
        &mut self,
        stmt: &Stmt,
        out: &mut Vec<Instruction>,
        spans: &mut Option<&mut Vec<Option<(u32, u32)>>>,
    ) -> SyxResult<()> {
        let start = out.len();
        match stmt {
            Stmt::Print { expr, .. } => {
                self.compile_expr(expr, out)?;
                out.push(Instruction::Print);
            }
            Stmt::Expr { expr, .. } => {
                self.compile_expr(expr, out)?;
                if !matches!(expr, Expr::Assign { .. }) {
                    out.push(Instruction::Pop);
                }
            }
            Stmt::VarDecl { name, init, ty, .. } => {
                let slot = self.alloc_local(name);
                if let Some(type_expr) = ty {
                    self.register_param_var_type(name, type_expr);
                }
                if let Some(expr) = init {
                    if let Expr::Borrow {
                        expr: inner,
                        mutable: true,
                        ..
                    } = expr
                    {
                        if let Expr::Ident { name: target, .. } = inner.as_ref() {
                            self.mut_ref_aliases.insert(name.clone(), target.clone());
                            return Ok(());
                        }
                    }
                    if let Expr::Call { callee, .. } = expr {
                        if let Expr::Ident {
                            name: type_name, ..
                        } = callee.as_ref()
                        {
                            if self.actor_types.contains(type_name) {
                                self.var_types.insert(name.clone(), type_name.clone());
                            } else if self.dataclass_names.contains(type_name) {
                                self.var_types.insert(name.clone(), type_name.clone());
                            } else if self.class_types.contains_key(type_name) {
                                self.var_types.insert(name.clone(), type_name.clone());
                            }
                        }
                    }
                    self.compile_expr(expr, out)?;
                    if self
                        .var_types
                        .get(name)
                        .is_some_and(|t| Self::type_name_is_set(t.as_str()))
                        || ty.as_ref().is_some_and(|type_expr| {
                            matches!(
                                type_expr,
                                syx_ast::TypeExpr::Named { name: type_name, .. }
                                    if type_name == "Set"
                            )
                        })
                    {
                        if matches!(out.last(), Some(Instruction::ArrayBuild(_))) {
                            let n = if let Some(Instruction::ArrayBuild(n)) = out.pop() {
                                n
                            } else {
                                0
                            };
                            out.push(Instruction::SetBuild(n));
                        }
                    }
                    if let Some(inferred) = self.infer_expr_type(expr) {
                        if !self.var_types.contains_key(name) {
                            self.var_types.insert(name.clone(), inferred);
                        }
                    }
                } else {
                    out.push(Instruction::ConstNil);
                }
                out.push(Instruction::StoreLocal(slot));
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.compile_expr(v, out)?;
                } else {
                    out.push(Instruction::ConstNil);
                }
                if let Some(proto) = &self.opaque_return_protocol {
                    out.push(Instruction::ConstStr("__opaque_payload".into()));
                    out.push(Instruction::ConstStr(proto.to_string()));
                    out.push(Instruction::ConstStr("__opaque_witness".into()));
                    out.push(Instruction::DictBuild(2));
                }
                out.push(Instruction::Return);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.compile_expr(condition, out)?;
                let jump_false = out.len();
                out.push(Instruction::JumpIfFalse(0));
                for s in then_branch {
                    self.compile_stmt(s, out, spans)?;
                }
                if let Some(else_b) = else_branch {
                    let jump_end = out.len();
                    out.push(Instruction::Jump(0));
                    let else_start = out.len();
                    if let Instruction::JumpIfFalse(ref mut target) = out[jump_false] {
                        *target = else_start;
                    }
                    for s in else_b {
                        self.compile_stmt(s, out, spans)?;
                    }
                    let end = out.len();
                    if let Instruction::Jump(ref mut target) = out[jump_end] {
                        *target = end;
                    }
                } else {
                    let end = out.len();
                    if let Instruction::JumpIfFalse(ref mut target) = out[jump_false] {
                        *target = end;
                    }
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                let while_span = stmt_span(stmt);
                let setup_start = out.len();
                let loop_start = out.len();
                out.push(Instruction::Loop);
                self.compile_expr(condition, out)?;
                let jump_false = out.len();
                out.push(Instruction::JumpIfFalse(0));
                let setup_end = out.len();
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                out.push(Instruction::Jump(loop_start + 1));
                out.push(Instruction::EndLoop);
                let end = out.len();
                if let Instruction::JumpIfFalse(ref mut target) = out[jump_false] {
                    *target = end;
                }
                if let Some(spans) = spans.as_mut() {
                    while spans.len() < out.len() {
                        spans.push(None);
                    }
                    for i in setup_start..setup_end {
                        spans[i] = Some(while_span);
                    }
                }
                return Ok(());
            }
            Stmt::ForIn {
                name,
                iterable,
                body,
                ..
            } => {
                let for_span = stmt_span(stmt);
                let setup_start = out.len();
                self.compile_expr(iterable, out)?;
                out.push(Instruction::MakeIterator);
                let iter_slot = self.alloc_local(&SmolStr::new(format!("__iter_{name}")));
                out.push(Instruction::StoreLocal(iter_slot));
                let loop_start = out.len();
                out.push(Instruction::LoadLocal(iter_slot));
                out.push(Instruction::IteratorNext);
                let jump_done = out.len();
                out.push(Instruction::JumpIfFalse(0));
                let slot = self.alloc_local(name);
                out.push(Instruction::StoreLocal(slot));
                out.push(Instruction::StoreLocal(iter_slot));
                let setup_end = out.len();
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                out.push(Instruction::Jump(loop_start));
                let end = out.len();
                if let Instruction::JumpIfFalse(ref mut target) = out[jump_done] {
                    *target = end;
                }
                if let Some(spans) = spans.as_mut() {
                    while spans.len() < out.len() {
                        spans.push(None);
                    }
                    for i in setup_start..setup_end {
                        spans[i] = Some(for_span);
                    }
                }
                return Ok(());
            }
            Stmt::Block { stmts, .. } | Stmt::Seq { stmts, .. } => {
                for s in stmts {
                    self.compile_stmt(s, out, spans)?;
                }
                return Ok(());
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
            Stmt::Try {
                body,
                catch_name,
                catch_body,
                ..
            } => {
                let catch_start = out.len();
                out.push(Instruction::BeginTry(0));
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                out.push(Instruction::EndTry);
                let jump_end = out.len();
                out.push(Instruction::Jump(0));
                let catch_label = out.len();
                if let Instruction::BeginTry(ref mut target) = out[catch_start] {
                    *target = catch_label;
                }
                out.push(Instruction::Catch);
                let slot = self.alloc_local(catch_name);
                out.push(Instruction::StoreLocal(slot));
                for s in catch_body {
                    self.compile_stmt(s, out, spans)?;
                }
                let end = out.len();
                if let Instruction::Jump(ref mut target) = out[jump_end] {
                    *target = end;
                }
            }
            Stmt::Switch {
                subject,
                cases,
                default,
                ..
            } => {
                self.compile_expr(subject, out)?;
                let case_slot = self.alloc_local(&SmolStr::new("__switch_val"));
                out.push(Instruction::StoreLocal(case_slot));
                let mut end_jumps = Vec::new();
                for case in cases {
                    self.compile_switch_pattern_match(&case.pattern, case_slot, out)?;
                    let jump_pattern_fail = out.len();
                    out.push(Instruction::JumpIfFalse(0));
                    if let Some(guard) = &case.guard {
                        self.bind_switch_pattern_bindings(&case.pattern, case_slot, out);
                        self.compile_expr(guard, out)?;
                        let jump_guard_fail = out.len();
                        out.push(Instruction::JumpIfFalse(0));
                        self.bind_switch_pattern_bindings(&case.pattern, case_slot, out);
                        for s in &case.body {
                            self.compile_stmt(s, out, spans)?;
                        }
                        end_jumps.push(out.len());
                        out.push(Instruction::Jump(0));
                        let next = out.len();
                        if let Instruction::JumpIfFalse(ref mut t) = out[jump_guard_fail] {
                            *t = next;
                        }
                        if let Instruction::JumpIfFalse(ref mut t) = out[jump_pattern_fail] {
                            *t = next;
                        }
                        continue;
                    }
                    self.bind_switch_pattern_bindings(&case.pattern, case_slot, out);
                    for s in &case.body {
                        self.compile_stmt(s, out, spans)?;
                    }
                    end_jumps.push(out.len());
                    out.push(Instruction::Jump(0));
                    let next = out.len();
                    if let Instruction::JumpIfFalse(ref mut t) = out[jump_pattern_fail] {
                        *t = next;
                    }
                }
                if let Some(body) = default {
                    for s in body {
                        self.compile_stmt(s, out, spans)?;
                    }
                }
                let end = out.len();
                for j in end_jumps {
                    if let Instruction::Jump(ref mut t) = out[j] {
                        *t = end;
                    }
                }
            }
            Stmt::IfCase {
                pattern,
                subject,
                body,
                else_branch,
                ..
            } => {
                self.compile_expr(subject, out)?;
                out.push(Instruction::Dup);
                let case_slot = self.alloc_local(&SmolStr::new("__ifcase_val"));
                out.push(Instruction::StoreLocal(case_slot));
                self.compile_switch_pattern_match(pattern, case_slot, out)?;
                let jump_fail = out.len();
                out.push(Instruction::JumpIfFalse(0));
                self.bind_switch_pattern_bindings(pattern, case_slot, out);
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                if let Some(else_b) = else_branch {
                    let jump_end = out.len();
                    out.push(Instruction::Jump(0));
                    let else_start = out.len();
                    if let Instruction::JumpIfFalse(ref mut t) = out[jump_fail] {
                        *t = else_start;
                    }
                    for s in else_b {
                        self.compile_stmt(s, out, spans)?;
                    }
                    let end = out.len();
                    if let Instruction::Jump(ref mut t) = out[jump_end] {
                        *t = end;
                    }
                } else {
                    let end = out.len();
                    if let Instruction::JumpIfFalse(ref mut t) = out[jump_fail] {
                        *t = end;
                    }
                }
            }
            Stmt::IfLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                self.compile_expr(pattern, out)?;
                let jump_nil = out.len();
                out.push(Instruction::JumpIfNil(0));
                let slot = self.alloc_local(name);
                out.push(Instruction::StoreLocal(slot));
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                if let Some(else_b) = else_branch {
                    let jump_end = out.len();
                    out.push(Instruction::Jump(0));
                    let else_start = out.len();
                    if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                        *t = else_start;
                    }
                    for s in else_b {
                        self.compile_stmt(s, out, spans)?;
                    }
                    let end = out.len();
                    if let Instruction::Jump(ref mut t) = out[jump_end] {
                        *t = end;
                    }
                } else {
                    let end = out.len();
                    if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                        *t = end;
                    }
                }
            }
            Stmt::GuardLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                self.compile_expr(pattern, out)?;
                let jump_fail = out.len();
                out.push(Instruction::JumpIfFalse(0));
                let slot = self.alloc_local(name);
                out.push(Instruction::StoreLocal(slot));
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                let jump_end = out.len();
                out.push(Instruction::Jump(0));
                let else_start = out.len();
                if let Instruction::JumpIfFalse(ref mut t) = out[jump_fail] {
                    *t = else_start;
                }
                for s in else_branch {
                    self.compile_stmt(s, out, spans)?;
                }
                let end = out.len();
                if let Instruction::Jump(ref mut t) = out[jump_end] {
                    *t = end;
                }
            }
            Stmt::Throw { expr, .. } => {
                self.compile_expr(expr, out)?;
                out.push(Instruction::ThrowError);
            }
            Stmt::RepeatWhile {
                body, condition, ..
            } => {
                let loop_start = out.len();
                out.push(Instruction::Loop);
                for s in body {
                    self.compile_stmt(s, out, spans)?;
                }
                self.compile_expr(condition, out)?;
                let jump_false = out.len();
                out.push(Instruction::JumpIfFalse(0));
                out.push(Instruction::Jump(loop_start + 1));
                out.push(Instruction::EndLoop);
                let end = out.len();
                if let Instruction::JumpIfFalse(ref mut target) = out[jump_false] {
                    *target = end;
                }
            }
            Stmt::MacroInvoke { .. } => {}
        }
        if let Some(spans) = spans.as_mut() {
            let span = stmt_span(stmt);
            while spans.len() < out.len() {
                spans.push(None);
            }
            for i in start..out.len() {
                spans[i] = Some(span);
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr, out: &mut Vec<Instruction>) -> SyxResult<()> {
        match expr {
            Expr::Literal { value, .. } => match value {
                Literal::BigInt(text) => {
                    out.push(Instruction::ConstStr(text.clone()));
                    if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_string") {
                        out.push(Instruction::CallHost(idx));
                    }
                }
                Literal::Integer { value, suffix } => {
                    if suffix.as_deref() == Some("n") {
                        out.push(Instruction::ConstNumeric128 {
                            kind: syx_stdlib::NumericKind::I128.to_byte(),
                            lo: *value as u64,
                            hi: (*value as u128 >> 64) as u64,
                        });
                        if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_i128") {
                            out.push(Instruction::CallHost(idx));
                        }
                        return Ok(());
                    }
                    let kind = suffix
                        .as_ref()
                        .and_then(|s| from_suffix(s))
                        .unwrap_or(PLATFORM_INT);
                    let resolved = resolve_alias(kind);
                    if bit_width(resolved) > 64 {
                        let bits = if is_unsigned(resolved) {
                            *value as u128
                        } else {
                            *value as u128
                        };
                        let lo = bits as u64;
                        let hi = (bits >> 64) as u64;
                        out.push(Instruction::ConstNumeric128 {
                            kind: resolved.to_byte(),
                            lo,
                            hi,
                        });
                    } else {
                        out.push(Instruction::ConstNumeric {
                            kind: resolved.to_byte(),
                            bits: *value as u64,
                        });
                    }
                }
                Literal::Double { value, suffix } => {
                    let kind = suffix
                        .as_ref()
                        .and_then(|s| from_suffix(s))
                        .unwrap_or(default_float_kind());
                    out.push(Instruction::ConstNumeric {
                        kind: kind.to_byte(),
                        bits: float_literal_bits(kind, *value),
                    });
                }
                Literal::String(s) => out.push(Instruction::ConstStr(s.clone())),
                Literal::Regex(p) => out.push(Instruction::ConstStr(p.clone())),
                Literal::Interpolated(parts) => {
                    let mut first = true;
                    for part in parts {
                        match part {
                            syx_ast::StringPart::Text(t) => {
                                out.push(Instruction::ConstStr(t.clone()));
                            }
                            syx_ast::StringPart::Expr(e) => self.compile_expr(e, out)?,
                        }
                        if first {
                            first = false;
                        } else {
                            out.push(Instruction::ConcatStr);
                        }
                    }
                }
                Literal::Bool(v) => out.push(Instruction::ConstBool(*v)),
                Literal::Nil => out.push(Instruction::ConstNil),
                Literal::Dict(entries) => {
                    for (key, val) in entries {
                        self.compile_expr(val, out)?;
                        self.compile_expr(key, out)?;
                    }
                    out.push(Instruction::DictBuild(entries.len() as u32));
                }
            },
            Expr::Ident { name, .. } => {
                self.compile_load_binding(name, out)?;
            }
            Expr::Binary {
                left, op, right, ..
            } => {
                if *op == BinOp::NilCoalesce {
                    // JumpIfNil pops nil / keeps non-nil — avoid Dup+Pop (Some-path leak).
                    self.compile_expr(left, out)?;
                    let jump_right = out.len();
                    out.push(Instruction::JumpIfNil(0));
                    let jump_end = out.len();
                    out.push(Instruction::Jump(0));
                    let right_label = out.len();
                    if let Instruction::JumpIfNil(ref mut t) = out[jump_right] {
                        *t = right_label;
                    }
                    self.compile_expr(right, out)?;
                    let end = out.len();
                    if let Instruction::Jump(ref mut t) = out[jump_end] {
                        *t = end;
                    }
                    return Ok(());
                }
                self.compile_expr(left, out)?;
                self.compile_expr(right, out)?;
                if let (Some((la, ea)), Some((lb, eb))) =
                    (self.expr_simd_type(left), self.expr_simd_type(right))
                {
                    if la == lb && ea == eb {
                        let element = ea.to_byte();
                        out.push(match op {
                            BinOp::Add => Instruction::SimdAdd { lanes: la, element },
                            BinOp::Sub => Instruction::SimdSub { lanes: la, element },
                            BinOp::Mul => Instruction::SimdMul { lanes: la, element },
                            BinOp::Div => Instruction::SimdDiv { lanes: la, element },
                            BinOp::Eq => Instruction::SimdEq { lanes: la, element },
                            BinOp::NotEq => Instruction::SimdNe { lanes: la, element },
                            BinOp::Lt => Instruction::SimdLt { lanes: la, element },
                            BinOp::LtEq => Instruction::SimdLe { lanes: la, element },
                            BinOp::Gt => Instruction::SimdGt { lanes: la, element },
                            BinOp::GtEq => Instruction::SimdGe { lanes: la, element },
                            BinOp::And => Instruction::SimdAnd { lanes: la, element },
                            BinOp::Or => Instruction::SimdOr { lanes: la, element },
                            // No BinOp::Xor — SIMD xor via host; never fall through to Add.
                            _ => Instruction::SimdAdd { lanes: la, element },
                        });
                        return Ok(());
                    }
                }
                if matches!(op, BinOp::IdenticalEq | BinOp::IdenticalNe) {
                    out.push(match op {
                        BinOp::IdenticalEq => Instruction::RefEq,
                        BinOp::IdenticalNe => Instruction::RefNe,
                        _ => unreachable!(),
                    });
                    return Ok(());
                }
                if matches!(op, BinOp::Eq | BinOp::NotEq) {
                    let inst = self.eq_instruction_for_exprs(left, right, *op);
                    let witness_eq = matches!(inst, Instruction::CallWitness(_, _));
                    out.push(inst);
                    if *op == BinOp::NotEq && witness_eq {
                        out.push(Instruction::BoolNot);
                    }
                    return Ok(());
                }
                // Phase 125: `String + String` / string-like Add → ConcatStr (not NumericAdd→0).
                if *op == BinOp::Add
                    && (self.expr_is_string_like(left) || self.expr_is_string_like(right))
                {
                    out.push(Instruction::ConcatStr);
                    return Ok(());
                }
                let nk = self.numeric_kind_for_binary(left, right);
                out.push(match op {
                    BinOp::Add => Instruction::NumericAdd(nk),
                    BinOp::Sub => Instruction::NumericSub(nk),
                    BinOp::Mul => Instruction::NumericMul(nk),
                    BinOp::Div => Instruction::NumericDiv(nk),
                    BinOp::Mod => Instruction::NumericMod(nk),
                    BinOp::BitAnd => Instruction::NumericBitAnd(nk),
                    BinOp::BitOr => Instruction::NumericBitOr(nk),
                    BinOp::BitXor => Instruction::NumericBitXor(nk),
                    BinOp::Shl => Instruction::NumericShl(nk),
                    BinOp::Shr => Instruction::NumericShr(nk),
                    BinOp::Lt => Instruction::NumericLt(nk),
                    BinOp::Gt => Instruction::NumericGt(nk),
                    BinOp::LtEq => Instruction::NumericLe(nk),
                    BinOp::GtEq => Instruction::NumericGe(nk),
                    BinOp::And => Instruction::BoolAnd,
                    BinOp::Or => Instruction::BoolOr,
                    BinOp::NilCoalesce => Instruction::Dup,
                    BinOp::Eq | BinOp::NotEq | BinOp::IdenticalEq | BinOp::IdenticalNe => {
                        Instruction::ValueEq
                    }
                });
            }
            Expr::Unary { op, expr, .. } => {
                if *op == UnaryOp::Try {
                    let catch_start = out.len();
                    out.push(Instruction::BeginTry(0));
                    self.compile_expr(expr, out)?;
                    out.push(Instruction::EndTry);
                    let jump_end = out.len();
                    out.push(Instruction::Jump(0));
                    let catch_label = out.len();
                    if let Instruction::BeginTry(ref mut t) = out[catch_start] {
                        *t = catch_label;
                    }
                    out.push(Instruction::Catch);
                    out.push(Instruction::ConstStr(String::new()));
                    let end = out.len();
                    if let Instruction::Jump(ref mut t) = out[jump_end] {
                        *t = end;
                    }
                    return Ok(());
                }
                self.compile_expr(expr, out)?;
                out.push(match op {
                    UnaryOp::Neg => Instruction::NumericNeg(PLATFORM_INT.to_byte()),
                    UnaryOp::Not => Instruction::BoolNot,
                    UnaryOp::BitNot => Instruction::NumericBitNot(PLATFORM_INT.to_byte()),
                    UnaryOp::Try => Instruction::Pop,
                });
            }
            Expr::Call { callee, args, .. } => {
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if name == "print" {
                        for arg in args {
                            self.compile_expr(&arg.value, out)?;
                        }
                        out.push(Instruction::Print);
                        return Ok(());
                    }
                    if name == "readLine" {
                        out.push(Instruction::CallHost(host_fn_index(HostFnId::ReadLine)));
                        return Ok(());
                    }
                    if name == "reflect" {
                        if let Some(arg) = args.first() {
                            if let Expr::TypeSelf { type_name, .. } = &arg.value {
                                out.push(Instruction::ConstStr(type_name.to_string()));
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ReflectType,
                                )));
                                return Ok(());
                            }
                        }
                    }
                }
                if let Expr::Member {
                    object,
                    name: method,
                    ..
                } = callee.as_ref()
                {
                    if matches!(object.as_ref(), Expr::Super { .. }) {
                        if let Some(class) = Self::fn_enclosing_class(&self.compiling_fn) {
                            if let Some(super_class) = self.class_superclasses.get(&class).cloned()
                            {
                                self.compile_load_binding(&SmolStr::new("self"), out)?;
                                for arg in args {
                                    self.compile_expr(&arg.value, out)?;
                                }
                                let fn_name = SmolStr::new(format!("{super_class}_{method}"));
                                out.push(Instruction::CallSuper(self.function_index(&fn_name)));
                                // Phase 135 A0: `super.init` may return an updated ObjectRef /
                                // MutRefCell; write it back so subclass field readback sees parent stores.
                                if method.as_str() == "init" {
                                    if let Some(&slot) = self.locals.get("self") {
                                        out.push(Instruction::StoreLocal(slot));
                                    }
                                }
                                return Ok(());
                            }
                        }
                    }
                    if method == "contains" && args.len() == 1 {
                        if let Some((start, end, inclusive)) = Self::range_bounds(object.as_ref()) {
                            self.compile_expr(&args[0].value, out)?;
                            out.push(Instruction::Dup);
                            out.push(Instruction::ConstI32(start as i32));
                            out.push(Instruction::I32Ge);
                            out.push(Instruction::Dup);
                            out.push(Instruction::ConstI32(end as i32));
                            if inclusive {
                                out.push(Instruction::I32Le);
                            } else {
                                out.push(Instruction::I32Lt);
                            }
                            out.push(Instruction::BoolAnd);
                            return Ok(());
                        }
                        if matches!(object.as_ref(), Expr::Ident { .. }) {
                            self.compile_range_contains(object, args, out)?;
                            return Ok(());
                        }
                    }
                }
                if let Some((host_id, receiver_first)) = self.resolve_call(callee) {
                    if receiver_first {
                        self.compile_expr(
                            match callee.as_ref() {
                                Expr::Member { object, .. } => object.as_ref(),
                                _ => callee.as_ref(),
                            },
                            out,
                        )?;
                    }
                    for arg in args {
                        self.compile_expr(&arg.value, out)?;
                    }
                    out.push(Instruction::CallHost(host_fn_index(host_id)));
                    return Ok(());
                }
                if let Expr::Member { object, name, .. } = callee.as_ref() {
                    if name == "hash" && args.len() == 1 && args[0].label.as_deref() == Some("into")
                    {
                        if let Some(type_name) = self.infer_expr_type(object) {
                            // Prefer dataclass shell `{T}_hash_into`, then user `func hash` → `{T}_hash`.
                            let hash_into_fn = SmolStr::new(format!("{type_name}_hash_into"));
                            let hash_method_fn = SmolStr::new(format!("{type_name}_hash"));
                            let call_fn = if self.has_function(&hash_into_fn) {
                                Some(hash_into_fn)
                            } else if self.has_function(&hash_method_fn) {
                                Some(hash_method_fn)
                            } else {
                                None
                            };
                            if let Some(hash_fn) = call_fn {
                                self.compile_expr(object, out)?;
                                self.compile_expr(&args[0].value, out)?;
                                out.push(Instruction::Call(self.function_index(&hash_fn)));
                                if let Expr::Ident {
                                    name: hasher_name, ..
                                } = &args[0].value
                                {
                                    if let Some(&slot) = self.locals.get(hasher_name) {
                                        out.push(Instruction::StoreLocal(slot));
                                    }
                                }
                                out.push(Instruction::ConstNil);
                                return Ok(());
                            }
                            let base = type_name_dispatch_base(&type_name);
                            if self
                                .extension_methods
                                .contains(&(base.clone(), name.clone()))
                            {
                                let ext_name = SmolStr::new(format!("{base}_{name}"));
                                let is_type_name_call = matches!(
                                    object.as_ref(),
                                    Expr::Ident { name: recv, .. } if recv == &type_name
                                );
                                if !is_type_name_call {
                                    self.compile_expr(object, out)?;
                                }
                                for arg in args {
                                    self.compile_expr(&arg.value, out)?;
                                }
                                out.push(Instruction::Call(self.function_index(&ext_name)));
                                out.push(Instruction::ConstNil);
                                return Ok(());
                            }
                            // Phase 46: no HasherCombine fallback for arbitrary types.
                            return Err(codegen_error_at(format!(
                                    "type '{type_name}' has no hash(into:) implementation; use a Core Hashable extension, data class + @Hashable, or define func hash(into:)"
                                ), None, Span::DUMMY, "C020"));
                        }
                    }
                    if let Some(type_name) = self.infer_expr_type(object) {
                        if type_name == "Hasher" {
                            if name == "combine" && args.len() == 1 {
                                self.compile_expr(object, out)?;
                                self.compile_expr(&args[0].value, out)?;
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::HasherCombine,
                                )));
                                return Ok(());
                            }
                            if name == "finalize" && args.is_empty() {
                                self.compile_expr(object, out)?;
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::HasherFinalize,
                                )));
                                return Ok(());
                            }
                        }
                    }
                }
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if let Some(host_id) = host_fn_id_by_name(name) {
                        self.compile_call_args(args, out)?;
                        out.push(Instruction::CallHost(host_fn_index(host_id)));
                        return Ok(());
                    }
                    if let Some(idx) = host_fn_index_by_name(name.as_str()) {
                        self.compile_call_args(args, out)?;
                        out.push(Instruction::CallHost(idx));
                        return Ok(());
                    }
                }
                if let Some((root_type, property, optional)) = keypath_from_expr(callee.as_ref()) {
                    if let Some(arg) = args.first() {
                        self.compile_expr(&arg.value, out)?;
                        out.push(Instruction::ConstStr(root_type.to_string()));
                        out.push(Instruction::ConstStr(property.to_string()));
                        out.push(Instruction::ConstBool(optional));
                        out.push(Instruction::CallHost(host_fn_index(HostFnId::KeyPathGet)));
                        return Ok(());
                    }
                }
                if let Expr::KeyPath {
                    root_type,
                    property,
                    optional,
                    ..
                } = callee.as_ref()
                {
                    if let Some(arg) = args.first() {
                        self.compile_expr(&arg.value, out)?;
                        out.push(Instruction::ConstStr(root_type.to_string()));
                        out.push(Instruction::ConstStr(property.to_string()));
                        out.push(Instruction::ConstBool(*optional));
                        out.push(Instruction::CallHost(host_fn_index(HostFnId::KeyPathGet)));
                        return Ok(());
                    }
                }
                if let Expr::OptionalChain { object, name, .. } = callee.as_ref() {
                    self.compile_expr(object, out)?;
                    out.push(Instruction::Dup);
                    let jump_nil = out.len();
                    out.push(Instruction::JumpIfNil(0));
                    for arg in args {
                        self.compile_expr(&arg.value, out)?;
                    }
                    if self.expr_is_string_like(object) {
                        if let Some(method) = lookup_string_method(name) {
                            out.push(Instruction::MethodCall(method.index()));
                        }
                    } else if self.expr_is_array_like(object) {
                        if let Some(method) = lookup_array_method(name) {
                            out.push(Instruction::MethodCall(method.index()));
                        }
                    } else {
                        out.push(Instruction::ConstStr(name.to_string()));
                        out.push(Instruction::ObjectGet);
                    }
                    let end = out.len();
                    out.push(Instruction::Jump(0));
                    let nil_label = out.len();
                    if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                        *t = nil_label;
                    }
                    out.push(Instruction::Pop);
                    out.push(Instruction::ConstNil);
                    let end_label = out.len();
                    if let Instruction::Jump(ref mut t) = out[end] {
                        *t = end_label;
                    }
                    return Ok(());
                }
                if let Expr::Member { object, name, .. } = callee.as_ref() {
                    if let Expr::Ident { name: mod_name, .. } = object.as_ref() {
                        let qualified = SmolStr::new(format!("{mod_name}_{name}"));
                        // User enum cases must win over the builtin Result/BigInt shortcut.
                        if self.final_classes.contains_key(mod_name)
                            || self.enum_classes.contains_key(mod_name)
                        {
                            self.compile_call_args(args, out)?;
                            let init_name = SmolStr::new(format!("{mod_name}_{name}_init"));
                            let idx = self.function_index(&init_name);
                            out.push(Instruction::Call(idx));
                            return Ok(());
                        }
                        if matches!(mod_name.as_str(), "Result" | "BigInt") {
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            out.push(Instruction::Call(self.require_function_index(&qualified)?));
                            return Ok(());
                        }
                        if self.generic_templates.contains(&qualified) {
                            return self.compile_generic_qualified_call(&qualified, args, out);
                        }
                        if self.imports.contains(mod_name)
                            || self.class_types.contains_key(mod_name)
                        {
                            if let Some(mangled) = self.resolve_static_monomorph(&qualified, args) {
                                for arg in args {
                                    self.compile_expr(&arg.value, out)?;
                                }
                                out.push(Instruction::Call(self.require_function_index(&mangled)?));
                                return Ok(());
                            }
                        }
                        if self.imports.contains(mod_name) {
                            if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                                self.compile_call_args(args, out)?;
                                let init_name = SmolStr::new(format!("{mod_name}_{name}_init"));
                                out.push(Instruction::Call(
                                    self.require_function_index(&init_name)?,
                                ));
                                return Ok(());
                            }
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            let idx = if self.has_function(name) {
                                self.require_function_index(name)?
                            } else {
                                self.require_function_index(&qualified)?
                            };
                            out.push(Instruction::Call(idx));
                            return Ok(());
                        }
                    }
                    if let Some(type_name) = self
                        .infer_expr_type(object)
                        .or_else(|| self.infer_expr_type_from_semantic(object))
                    {
                        let base = type_name_dispatch_base(&type_name);
                        let is_type_name_call = matches!(
                            object.as_ref(),
                            Expr::Ident { name: recv, .. } if recv == &type_name
                        );
                        if is_type_name_call
                            && name.chars().next().is_some_and(|c| c.is_uppercase())
                        {
                            let nested_ty = SmolStr::new(format!("{base}_{name}"));
                            if self.class_types.contains_key(&nested_ty) {
                                for arg in args {
                                    self.compile_expr(&arg.value, out)?;
                                }
                                let init_name = SmolStr::new(format!("{nested_ty}_init"));
                                out.push(Instruction::Call(self.function_index(&init_name)));
                                return Ok(());
                            }
                        }
                        let ext_name = SmolStr::new(format!("{base}_{name}"));
                        // ADR-236 / M2-S4-01: named HTTPResponse class must not steal
                        // MethodId accessors (status/text/ok/…) into empty HTTPResponse_* Calls.
                        if (base.as_str() == "HTTPResponse" || type_name.as_str() == "HTTPResponse")
                            && lookup_http_response_method(name.as_str()).is_some()
                        {
                            self.compile_expr(object, out)?;
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            let method =
                                lookup_http_response_method(name.as_str()).expect("checked");
                            out.push(Instruction::MethodCall(method.index()));
                            return Ok(());
                        }
                        // ADR-246 / M2-S8-02b: named SqlRow accessors → MethodCall (throws via host).
                        if (base.as_str() == "SqlRow" || type_name.as_str() == "SqlRow")
                            && lookup_sql_row_method(name.as_str()).is_some()
                        {
                            let is_type_name_call = matches!(
                                object.as_ref(),
                                Expr::Ident { name: recv, .. } if recv == &type_name
                            );
                            if !is_type_name_call {
                                self.compile_expr(object, out)?;
                            }
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            if is_type_name_call {
                                // Static SqlRow.getInt(row:, name:) → CallHost.
                                if let Some(host) =
                                    lookup_type_method("SqlRow", name.as_str()).and_then(|m| m.host)
                                {
                                    out.push(Instruction::CallHost(host_fn_index(host)));
                                    return Ok(());
                                }
                            }
                            let method = lookup_sql_row_method(name.as_str()).expect("checked");
                            out.push(Instruction::MethodCall(method.index()));
                            return Ok(());
                        }
                        if self.class_types.contains_key(&base)
                            || self
                                .extension_methods
                                .contains(&(base.clone(), name.clone()))
                        {
                            let is_type_name_call = matches!(
                                object.as_ref(),
                                Expr::Ident { name: recv, .. } if recv == &type_name
                            );
                            // Phase 125: prefer Call + resolve_virtual_method (__type) over
                            // CallIndirect vtable slots. Unconditional vtable dispatch breaks when
                            // function_index falls back to 0 (wrong host / resolve) before methods
                            // are registered — DI polymorphic inject must not hit that path.
                            if !is_type_name_call {
                                self.compile_expr(object, out)?;
                            }
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            if self.actor_types.contains(&type_name) {
                                out.push(Instruction::ActorCall(self.function_index(&ext_name)));
                                if name.as_str() != "get" {
                                    if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                                        if let Some(&slot) = self.locals.get(obj_name) {
                                            out.push(Instruction::StoreLocal(slot));
                                            out.push(Instruction::LoadLocal(slot));
                                        }
                                    }
                                }
                            } else if name.as_str() == "description" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDescription,
                                )));
                            } else if name.as_str() == "debugDescription" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDebugDescription,
                                )));
                            } else if self
                                .protocol_methods
                                .contains_key(&SmolStr::new(base.as_str()))
                                && !is_type_name_call
                            {
                                self.emit_witness_protocol_call(
                                    object,
                                    base.as_str(),
                                    name.as_str(),
                                    args,
                                    out,
                                )?;
                            } else {
                                out.push(Instruction::Call(self.function_index(&ext_name)));
                            }
                            return Ok(());
                        }
                        if self
                            .extension_methods
                            .contains(&(base.clone(), name.clone()))
                            || self.has_function(&ext_name)
                        {
                            let is_type_name_call = matches!(
                                object.as_ref(),
                                Expr::Ident { name: recv, .. } if recv == &type_name
                            );
                            if !is_type_name_call
                                && self
                                    .protocol_methods
                                    .contains_key(&SmolStr::new(base.as_str()))
                            {
                                self.emit_witness_protocol_call(
                                    object,
                                    base.as_str(),
                                    name.as_str(),
                                    args,
                                    out,
                                )?;
                                return Ok(());
                            }
                            if !is_type_name_call {
                                self.compile_expr(object, out)?;
                            }
                            for arg in args {
                                self.compile_expr(&arg.value, out)?;
                            }
                            if name.as_str() == "description" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDescription,
                                )));
                            } else if name.as_str() == "debugDescription" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDebugDescription,
                                )));
                            } else {
                                out.push(Instruction::Call(self.function_index(&ext_name)));
                            }
                            return Ok(());
                        }
                    }
                    self.compile_expr(object, out)?;
                    for arg in args {
                        self.compile_expr(&arg.value, out)?;
                    }
                    if self.expr_is_http_response_like(object) {
                        if let Some(method) = lookup_http_response_method(name) {
                            out.push(Instruction::MethodCall(method.index()));
                            return Ok(());
                        }
                    }
                    if self.expr_is_string_like(object) {
                        if let Some(method) = lookup_string_method(name) {
                            out.push(Instruction::MethodCall(method.index()));
                            return Ok(());
                        }
                    }
                    if self.expr_is_array_like(object) {
                        if self.try_emit_hosted_collection_call("Array", name.as_str(), out) {
                            return Ok(());
                        }
                        let base = SmolStr::new("Array");
                        if self
                            .extension_methods
                            .contains(&(base.clone(), name.clone()))
                        {
                            if name.as_str() == "description" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDescription,
                                )));
                                return Ok(());
                            }
                            if name.as_str() == "debugDescription" {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDebugDescription,
                                )));
                                return Ok(());
                            }
                            let ext_name = SmolStr::new(format!("Array_{name}"));
                            out.push(Instruction::Call(self.function_index(&ext_name)));
                            return Ok(());
                        }
                        if let Some(method) = lookup_array_method(name) {
                            out.push(Instruction::MethodCall(method.index()));
                            return Ok(());
                        }
                    }
                    if let Some(method) = lookup_array_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                    if let Some(method) = lookup_string_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                    if let Some(method) = lookup_dict_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                    if let Some(method) = lookup_set_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                    if name == "description" {
                        if let Some(type_name) = self.infer_expr_type(object) {
                            if self.dataclass_names.contains(&type_name) {
                                let desc = SmolStr::new(format!("{type_name}_describe"));
                                out.push(Instruction::Call(self.function_index(&desc)));
                                return Ok(());
                            }
                        }
                    }
                    if name == "copy" {
                        if let Some(type_name) = self.infer_expr_type(object) {
                            if self.dataclass_names.contains(&type_name) {
                                let copy_fn = SmolStr::new(format!("{type_name}_copy"));
                                out.push(Instruction::Call(self.function_index(&copy_fn)));
                                return Ok(());
                            }
                        }
                    }
                }
                if let Expr::Ident {
                    name: callee_name, ..
                } = callee.as_ref()
                {
                    if self.class_types.contains_key(callee_name) {
                        // Builtin `Set` is `Value::Set`; empty `Set()` → SetBuild(0).
                        if callee_name.as_str() == "Set" && args.is_empty() {
                            out.push(Instruction::SetBuild(0));
                            return Ok(());
                        }
                        self.compile_call_args(args, out)?;
                        let init_name = SmolStr::new(format!("{callee_name}_init"));
                        out.push(Instruction::Call(self.function_index(&init_name)));
                        return Ok(());
                    }
                    if self.dataclass_names.contains(callee_name) {
                        if let Some(fields) = self.dataclass_fields.get(callee_name).cloned() {
                            self.compile_ordered_call_args(callee_name, args, &fields, out)?;
                        } else {
                            self.compile_call_args(args, out)?;
                        }
                        let init_name = SmolStr::new(format!("{callee_name}_init"));
                        out.push(Instruction::Call(self.function_index(&init_name)));
                        return Ok(());
                    }
                    if self.generic_templates.contains(callee_name)
                        || self.function_param_types.contains_key(callee_name)
                    {
                        return self.compile_call_with_mut_refs(callee, args, out);
                    }
                }
                if let Expr::Member { object, .. } = callee.as_ref() {
                    if let Expr::Member {
                        object,
                        name: method,
                        ..
                    } = callee.as_ref()
                    {
                        if let Expr::Ident {
                            name: type_name, ..
                        } = object.as_ref()
                        {
                            if self
                                .extension_methods
                                .contains(&(type_name.clone(), method.clone()))
                            {
                                self.compile_call_args(args, out)?;
                                let ext_name = SmolStr::new(format!("{type_name}_{method}"));
                                out.push(Instruction::Call(self.function_index(&ext_name)));
                                return Ok(());
                            }
                        }
                    }
                    if let Expr::Ident {
                        name: type_name, ..
                    } = object.as_ref()
                    {
                        if self.enum_classes.contains_key(type_name)
                            || self.generic_enum_templates.contains(type_name)
                            || self.generic_final_templates.contains(type_name)
                        {
                            return self.compile_call_with_mut_refs(callee, args, out);
                        }
                    }
                }
                if let Expr::Member { name, .. } = callee.as_ref() {
                    if let Some(m) = lookup_array_method(name) {
                        out.push(Instruction::MethodCall(m.index()));
                        return Ok(());
                    }
                    if let Some(m) = lookup_string_method(name) {
                        out.push(Instruction::MethodCall(m.index()));
                        return Ok(());
                    }
                }
                self.compile_call_args(args, out)?;
                if let Expr::Ident {
                    name: callee_name, ..
                } = callee.as_ref()
                {
                    // S1-I4: local/param/global closure value — args then closure, CallClosure.
                    if !self.has_function(callee_name) {
                        if let Some(&slot) = self.locals.get(callee_name) {
                            out.push(Instruction::LoadLocal(slot));
                            out.push(Instruction::CallClosure);
                            return Ok(());
                        }
                        if let Some(idx) =
                            self.program.globals.iter().position(|g| g == callee_name)
                        {
                            out.push(Instruction::LoadGlobal(idx as u32));
                            out.push(Instruction::CallClosure);
                            return Ok(());
                        }
                    }
                    let idx = self.require_function_index(callee_name)?;
                    out.push(Instruction::Call(idx));
                } else {
                    let idx = self
                        .program
                        .functions
                        .iter()
                        .position(|f| {
                            if let Expr::Ident {
                                name: callee_name, ..
                            } = callee.as_ref()
                            {
                                f.name == *callee_name
                            } else {
                                false
                            }
                        })
                        .unwrap_or(0);
                    out.push(Instruction::Call(idx as u32));
                }
            }
            Expr::Member { object, name, .. } => {
                if let Expr::Ident {
                    name: type_name, ..
                } = object.as_ref()
                {
                    let nested_ty = SmolStr::new(format!("{type_name}_{name}"));
                    if self.class_types.contains_key(&nested_ty) {
                        let init_name = SmolStr::new(format!("{nested_ty}_init"));
                        out.push(Instruction::Call(self.function_index(&init_name)));
                        return Ok(());
                    }
                    if self.enum_classes.contains_key(type_name)
                        || self.generic_enum_templates.contains(type_name)
                        || self.generic_final_templates.contains(type_name)
                        || self.final_classes.contains_key(type_name)
                    {
                        let init_name = SmolStr::new(format!("{type_name}_{name}_init"));
                        out.push(Instruction::Call(self.function_index(&init_name)));
                        return Ok(());
                    }
                }
                self.compile_expr(object, out)?;
                if name == "description" {
                    if let Some(type_name) = self.infer_expr_type(object) {
                        if self.dataclass_names.contains(&type_name) {
                            let desc = SmolStr::new(format!("{type_name}_describe"));
                            out.push(Instruction::Call(self.function_index(&desc)));
                            return Ok(());
                        }
                        let base = type_name_dispatch_base(&type_name);
                        if self
                            .extension_methods
                            .contains(&(base.clone(), SmolStr::new("description")))
                        {
                            out.push(Instruction::CallHost(host_fn_index(
                                HostFnId::ValueDescription,
                            )));
                            return Ok(());
                        }
                    }
                }
                if name == "debugDescription" {
                    if let Some(type_name) = self.infer_expr_type(object) {
                        let base = type_name_dispatch_base(&type_name);
                        if self
                            .extension_methods
                            .contains(&(base.clone(), SmolStr::new("debugDescription")))
                        {
                            out.push(Instruction::CallHost(host_fn_index(
                                HostFnId::ValueDebugDescription,
                            )));
                            return Ok(());
                        }
                    }
                }
                if name == "copy" {
                    if let Some(type_name) = self.infer_expr_type(object) {
                        if self.dataclass_names.contains(&type_name) {
                            let copy_fn = SmolStr::new(format!("{type_name}_copy"));
                            out.push(Instruction::Call(self.function_index(&copy_fn)));
                            return Ok(());
                        }
                    }
                }
                if self.expr_is_string_like(object) {
                    if let Some(method) = lookup_string_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                }
                if self.expr_is_array_like(object) {
                    if name == "description" {
                        out.push(Instruction::CallHost(host_fn_index(
                            HostFnId::ValueDescription,
                        )));
                        return Ok(());
                    }
                    if name == "debugDescription" {
                        out.push(Instruction::CallHost(host_fn_index(
                            HostFnId::ValueDebugDescription,
                        )));
                        return Ok(());
                    }
                    let base = SmolStr::new("Array");
                    if self
                        .extension_methods
                        .contains(&(base.clone(), name.clone()))
                    {
                        match name.as_str() {
                            "description" => {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDescription,
                                )));
                            }
                            "debugDescription" => {
                                out.push(Instruction::CallHost(host_fn_index(
                                    HostFnId::ValueDebugDescription,
                                )));
                            }
                            _ => {
                                let ext_name = SmolStr::new(format!("Array_{name}"));
                                out.push(Instruction::Call(self.function_index(&ext_name)));
                            }
                        }
                        return Ok(());
                    }
                    if let Some(method) = lookup_array_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                }
                if self.expr_is_dict_like(object) {
                    if name == "description" || name == "debugDescription" {
                        let host = if name == "description" {
                            HostFnId::ValueDescription
                        } else {
                            HostFnId::ValueDebugDescription
                        };
                        out.push(Instruction::CallHost(host_fn_index(host)));
                        return Ok(());
                    }
                    if let Some(method) = lookup_dict_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                }
                if self.expr_is_set_like(object) {
                    if let Some(method) = lookup_set_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        return Ok(());
                    }
                }
                if let Some(method) = lookup_array_method(name) {
                    out.push(Instruction::MethodCall(method.index()));
                    return Ok(());
                }
                if let Some(method) = lookup_string_method(name) {
                    out.push(Instruction::MethodCall(method.index()));
                    return Ok(());
                }
                out.push(Instruction::ConstStr(name.to_string()));
                out.push(Instruction::ObjectGet);
            }
            Expr::Group { expr, .. } => self.compile_expr(expr, out)?,
            Expr::Borrow { expr, .. } => self.compile_expr(expr, out)?,
            Expr::KeyPath { .. } => {
                return Err(codegen_error_at(
                    "Key path must be applied to a value, e.g. (\\Person.name)(obj)",
                    None,
                    Span::DUMMY,
                    "G001",
                ));
            }
            Expr::MacroInvoke { .. } => {
                out.push(Instruction::ConstNil);
            }
            Expr::OptionalCast { expr, target, .. } => {
                self.compile_expr(expr, out)?;
                out.push(Instruction::ConstStr(
                    Self::type_expr_name(target).to_string(),
                ));
                out.push(Instruction::CallHost(host_fn_index(HostFnId::AsOptional)));
            }
            Expr::Is { expr, target, .. } => {
                self.compile_expr(expr, out)?;
                out.push(Instruction::ConstStr(
                    Self::type_expr_name(target).to_string(),
                ));
                out.push(Instruction::CallHost(host_fn_index(HostFnId::TypeIs)));
            }
            Expr::Cast { expr, target, .. } => {
                self.compile_expr(expr, out)?;
                out.push(Instruction::ConstStr(
                    Self::type_expr_name(target).to_string(),
                ));
                out.push(Instruction::CallHost(host_fn_index(HostFnId::TypeCast)));
            }
            Expr::TypeSelf { type_name, .. } => {
                out.push(Instruction::ConstStr(type_name.to_string()));
                out.push(Instruction::CallHost(host_fn_index(HostFnId::ReflectType)));
            }
            Expr::Array { elements, .. } => {
                for el in elements {
                    self.compile_expr(el, out)?;
                }
                out.push(Instruction::ArrayBuild(elements.len() as u32));
            }
            Expr::Tuple { elements, .. } => {
                for (_, el) in elements {
                    self.compile_expr(el, out)?;
                }
                out.push(Instruction::ArrayBuild(elements.len() as u32));
            }
            Expr::Index { object, index, .. } => {
                self.compile_expr(object, out)?;
                self.compile_expr(index, out)?;
                if self.expr_is_string_like(object) {
                    out.push(Instruction::CallHost(host_fn_index(HostFnId::StringCharAt)));
                } else if self.expr_is_dict_like(object) {
                    out.push(Instruction::ObjectGet);
                } else if let Some(type_name) = self.infer_expr_type(object) {
                    let base = type_name_dispatch_base(&type_name);
                    if self
                        .extension_methods
                        .contains(&(base.clone(), smol_str::SmolStr::new("__subscript_get")))
                    {
                        let get_fn = SmolStr::new(format!("{base}___subscript_get"));
                        out.push(Instruction::Call(self.function_index(&get_fn)));
                    } else {
                        out.push(Instruction::ArrayGet);
                    }
                } else {
                    out.push(Instruction::ArrayGet);
                }
            }
            Expr::Assign { name, value, .. } => {
                self.compile_expr(value, out)?;
                if let Some(ty) = self.infer_expr_type(value) {
                    self.var_types.insert(name.clone(), ty);
                }
                self.compile_store_binding(name, out)?;
            }
            Expr::Await { expr, .. } => {
                if self.in_async {
                    self.compile_expr(expr, out)?;
                    let state = self.await_segment;
                    self.await_segment += 1;
                    out.push(Instruction::AwaitSuspend {
                        resume_fn: self.compiling_fn_index(),
                        state,
                    });
                } else {
                    return Err(codegen_error_at(
                        "await is only allowed inside async functions",
                        None,
                        Span::DUMMY,
                        "G002",
                    ));
                }
            }
            Expr::OptionalChain { object, name, .. } => {
                self.compile_expr(object, out)?;
                out.push(Instruction::Dup);
                let jump_nil = out.len();
                out.push(Instruction::JumpIfNil(0));
                if self.expr_is_string_like(object) {
                    if let Some(method) = lookup_string_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        let end = out.len();
                        out.push(Instruction::Jump(0));
                        let nil_label = out.len();
                        if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                            *t = nil_label;
                        }
                        out.push(Instruction::Pop);
                        out.push(Instruction::ConstNil);
                        let end_label = out.len();
                        if let Instruction::Jump(ref mut t) = out[end] {
                            *t = end_label;
                        }
                        return Ok(());
                    }
                }
                if self.expr_is_array_like(object) {
                    if let Some(method) = lookup_array_method(name) {
                        out.push(Instruction::MethodCall(method.index()));
                        let end = out.len();
                        out.push(Instruction::Jump(0));
                        let nil_label = out.len();
                        if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                            *t = nil_label;
                        }
                        out.push(Instruction::Pop);
                        out.push(Instruction::ConstNil);
                        let end_label = out.len();
                        if let Instruction::Jump(ref mut t) = out[end] {
                            *t = end_label;
                        }
                        return Ok(());
                    }
                }
                out.push(Instruction::ConstStr(name.to_string()));
                out.push(Instruction::ObjectGet);
                let end = out.len();
                out.push(Instruction::Jump(0));
                let nil_label = out.len();
                if let Instruction::JumpIfNil(ref mut t) = out[jump_nil] {
                    *t = nil_label;
                }
                out.push(Instruction::Pop);
                out.push(Instruction::ConstNil);
                let end_label = out.len();
                if let Instruction::Jump(ref mut t) = out[end] {
                    *t = end_label;
                }
            }
            Expr::ForceUnwrap { expr, .. } => {
                self.compile_expr(expr, out)?;
                let trap = out.len();
                out.push(Instruction::JumpIfNil(0));
                let end = out.len();
                out.push(Instruction::Jump(0));
                let throw_label = out.len();
                if let Instruction::JumpIfNil(ref mut t) = out[trap] {
                    *t = throw_label;
                }
                out.push(Instruction::ThrowError);
                let end_label = out.len();
                if let Instruction::Jump(ref mut t) = out[end] {
                    *t = end_label;
                }
            }
            Expr::CompoundAssign {
                name,
                op,
                value,
                span,
            } => {
                let bin_op = compound_to_bin(*op);
                let desugared = Expr::Assign {
                    span: *span,
                    name: name.clone(),
                    value: Box::new(Expr::Binary {
                        span: *span,
                        left: Box::new(Expr::Ident {
                            span: *span,
                            name: name.clone(),
                        }),
                        op: bin_op,
                        right: value.clone(),
                    }),
                };
                self.compile_expr(&desugared, out)?;
            }
            Expr::MemberAssign {
                object,
                name,
                value,
                ..
            } => {
                if let Expr::Ident {
                    name: type_name, ..
                } = object.as_ref()
                {
                    if self.class_types.contains_key(type_name) {
                        let global_name = syx_ast::static_var_global_name(type_name, name);
                        if let Some(idx) =
                            self.program.globals.iter().position(|g| g == &global_name)
                        {
                            let observer = self.lookup_property_observer(type_name, name);
                            let saved_locals = self.locals.clone();
                            let saved_local_count = self.local_count;
                            if let Some((will_set, did_set)) = observer {
                                let old_slot = self.alloc_local(&SmolStr::new("oldValue"));
                                let new_slot = self.alloc_local(&SmolStr::new("newValue"));
                                if did_set.is_some() {
                                    out.push(Instruction::LoadGlobal(idx as u32));
                                    out.push(Instruction::StoreLocal(old_slot));
                                }
                                if will_set.is_some() {
                                    self.compile_expr(value, out)?;
                                    out.push(Instruction::StoreLocal(new_slot));
                                    self.locals.insert(SmolStr::new("oldValue"), old_slot);
                                    self.locals.insert(SmolStr::new("newValue"), new_slot);
                                    if let Some(stmts) = will_set {
                                        self.compile_inline_stmts(&stmts, out)?;
                                    }
                                    self.locals.swap_remove("oldValue");
                                    self.locals.swap_remove("newValue");
                                }
                                self.compile_expr(value, out)?;
                                out.push(Instruction::StoreGlobal(idx as u32));
                                if let Some(stmts) = did_set {
                                    out.push(Instruction::LoadGlobal(idx as u32));
                                    out.push(Instruction::StoreLocal(new_slot));
                                    self.locals.insert(SmolStr::new("oldValue"), old_slot);
                                    self.locals.insert(SmolStr::new("newValue"), new_slot);
                                    self.compile_inline_stmts(&stmts, out)?;
                                    self.locals.swap_remove("oldValue");
                                    self.locals.swap_remove("newValue");
                                }
                                self.locals = saved_locals;
                                self.local_count = saved_local_count;
                                return Ok(());
                            }
                            self.compile_expr(value, out)?;
                            out.push(Instruction::StoreGlobal(idx as u32));
                            return Ok(());
                        }
                    }
                }
                let obj_type = if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                    self.receiver_class_type(obj_name)
                } else {
                    None
                };
                if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                    if self.uses_mut_ref_cell(obj_name) {
                        self.compile_load_binding(obj_name, out)?;
                    } else {
                        self.compile_expr(object, out)?;
                    }
                } else {
                    self.compile_expr(object, out)?;
                }
                let observer = obj_type
                    .as_ref()
                    .and_then(|ty| self.lookup_property_observer(ty, name));
                let suppress_observers = self.member_assign_skip_unwrap_object(object.as_ref());
                let saved_locals = self.locals.clone();
                let saved_local_count = self.local_count;
                if observer.is_some() && !suppress_observers {
                    let (will_set, did_set) = observer.unwrap();
                    let old_slot = self.alloc_local(&SmolStr::new("oldValue"));
                    let new_slot = self.alloc_local(&SmolStr::new("newValue"));
                    if did_set.is_some() {
                        out.push(Instruction::Dup);
                        out.push(Instruction::ConstStr(name.to_string()));
                        out.push(Instruction::ObjectGet);
                        out.push(Instruction::StoreLocal(old_slot));
                    }
                    if will_set.is_some() {
                        self.compile_expr(value, out)?;
                        out.push(Instruction::StoreLocal(new_slot));
                        self.locals.insert(SmolStr::new("oldValue"), old_slot);
                        self.locals.insert(SmolStr::new("newValue"), new_slot);
                        if let Some(stmts) = will_set {
                            self.compile_inline_stmts(&stmts, out)?;
                        }
                        self.locals.swap_remove("oldValue");
                        self.locals.swap_remove("newValue");
                    }
                    if !self.member_assign_skip_unwrap_object(object.as_ref())
                        && !self.expr_is_dict_like(object)
                    {
                        Self::emit_unwrap_syx_ref(out);
                    }
                    self.compile_expr(value, out)?;
                    out.push(Instruction::ConstStr(name.to_string()));
                    out.push(Instruction::ObjectSet);
                    if let Some(stmts) = did_set {
                        out.push(Instruction::Dup);
                        out.push(Instruction::ConstStr(name.to_string()));
                        out.push(Instruction::ObjectGet);
                        out.push(Instruction::StoreLocal(new_slot));
                        self.locals.insert(SmolStr::new("oldValue"), old_slot);
                        self.locals.insert(SmolStr::new("newValue"), new_slot);
                        self.compile_inline_stmts(&stmts, out)?;
                        self.locals.swap_remove("oldValue");
                        self.locals.swap_remove("newValue");
                    }
                } else {
                    // Plain Object/Dictionary bags are not MutRefCell wrappers —
                    // unwrapping `__syx_ref` would ObjectGet→Nil and wipe the local.
                    if !self.member_assign_skip_unwrap_object(object.as_ref())
                        && !self.expr_is_dict_like(object)
                    {
                        Self::emit_unwrap_syx_ref(out);
                    }
                    self.compile_expr(value, out)?;
                    out.push(Instruction::ConstStr(name.to_string()));
                    out.push(Instruction::ObjectSet);
                }
                self.locals = saved_locals;
                self.local_count = saved_local_count;
                if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                    if !self.member_assign_skip_unwrap_object(object.as_ref())
                        && self.is_class_instance_local(obj_name)
                    {
                        self.compile_mut_ref_cell_writeback(obj_name, out)?;
                    } else if self.uses_mut_ref_cell(obj_name) {
                        self.compile_store_binding(obj_name, out)?;
                    } else if let Some(&slot) = self.locals.get(obj_name) {
                        out.push(Instruction::StoreLocal(slot));
                    }
                }
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
                if let Expr::Member {
                    object: base,
                    name: field,
                    ..
                } = object.as_ref()
                {
                    if let Expr::Ident { name: recv, .. } = base.as_ref() {
                        if self.locals.contains_key(recv) {
                            self.compile_expr(base, out)?;
                            out.push(Instruction::ConstStr(field.to_string()));
                            out.push(Instruction::ObjectGet);
                            self.compile_expr(index, out)?;
                            self.compile_expr(value, out)?;
                            out.push(Instruction::ArraySet);
                            out.push(Instruction::ConstStr(field.to_string()));
                            out.push(Instruction::ObjectSet);
                            if let Some(&slot) = self.locals.get(recv) {
                                out.push(Instruction::StoreLocal(slot));
                            }
                            return Ok(());
                        }
                    }
                }
                if let Some(type_name) = self.infer_expr_type(object) {
                    let base = type_name_dispatch_base(&type_name);
                    if self
                        .extension_methods
                        .contains(&(base.clone(), smol_str::SmolStr::new("__subscript_set")))
                    {
                        self.compile_expr(object, out)?;
                        self.compile_expr(index, out)?;
                        self.compile_expr(value, out)?;
                        let set_fn = SmolStr::new(format!("{base}___subscript_set"));
                        out.push(Instruction::Call(self.function_index(&set_fn)));
                        if let Expr::Ident { name: recv, .. } = object.as_ref() {
                            if let Some(&slot) = self.locals.get(recv) {
                                out.push(Instruction::StoreLocal(slot));
                            }
                        }
                        return Ok(());
                    }
                }
                // Object / Dictionary use ObjectSet (string keys); arrays use ArraySet.
                let obj_ty = self.infer_expr_type(object);
                let use_object_set = obj_ty.as_ref().is_some_and(|t| {
                    let s = t.as_str();
                    s == "Object" || s == "Dictionary" || s.starts_with("Dictionary<")
                });
                if use_object_set {
                    self.compile_expr(object, out)?;
                    self.compile_expr(value, out)?;
                    self.compile_expr(index, out)?;
                    out.push(Instruction::ObjectSet);
                } else {
                    self.compile_expr(object, out)?;
                    self.compile_expr(index, out)?;
                    self.compile_expr(value, out)?;
                    out.push(Instruction::ArraySet);
                }
                if let Expr::Ident { name: recv, .. } = object.as_ref() {
                    if let Some(&slot) = self.locals.get(recv) {
                        out.push(Instruction::StoreLocal(slot));
                    }
                }
            }
            Expr::Closure {
                params, body, span, ..
            } => {
                let closure_name = SmolStr::new(format!("__closure_{}", self.closure_counter));
                self.closure_counter += 1;
                let body_stmts = match body {
                    ClosureBody::Expr(e) => vec![Stmt::Return {
                        span: *span,
                        value: Some((**e).clone()),
                    }],
                    ClosureBody::Block(stmts) => stmts.clone(),
                };
                let free_vars = collect_closure_free_vars(params, body, &self.locals);
                for name in &free_vars {
                    if let Some(&slot) = self.locals.get(name) {
                        out.push(Instruction::LoadLocal(slot));
                    } else if let Some(idx) = self.program.globals.iter().position(|g| g == name) {
                        out.push(Instruction::LoadGlobal(idx as u32));
                    }
                }
                self.compile_function_decl(&closure_name, params, &body_stmts, &free_vars)?;
                let fn_idx = self.function_index(&closure_name);
                out.push(Instruction::MakeClosure(fn_idx, free_vars.len() as u32));
            }
            Expr::TaskBlock { body, detached, .. } => {
                let fn_name = SmolStr::new(format!("__task_body_{}", self.task_body_counter));
                self.task_body_counter += 1;
                self.compile_function_decl(&fn_name, &[], body, &[])?;
                let fn_idx = self.function_index(&fn_name);
                if *detached {
                    out.push(Instruction::ConstStr(fn_name.to_string()));
                    out.push(Instruction::CallHost(host_fn_index(HostFnId::TaskDetach)));
                } else {
                    out.push(Instruction::TaskSpawn(fn_idx));
                }
            }
            Expr::HalfOpenRange { start, end, .. } => {
                out.push(Instruction::ConstI32(*start as i32));
                out.push(Instruction::ConstStr("start".into()));
                out.push(Instruction::ConstI32(*end as i32));
                out.push(Instruction::ConstStr("end".into()));
                out.push(Instruction::DictBuild(2));
            }
            Expr::ClosedRange { start, end, .. } => {
                out.push(Instruction::ConstI32(*start as i32));
                out.push(Instruction::ConstStr("start".into()));
                out.push(Instruction::ConstI32(*end as i32));
                out.push(Instruction::ConstStr("end".into()));
                out.push(Instruction::ConstBool(true));
                out.push(Instruction::ConstStr("inclusive".into()));
                out.push(Instruction::DictBuild(3));
            }
            Expr::RangeDyn {
                start,
                end,
                inclusive,
                ..
            } => {
                self.compile_expr(start, out)?;
                out.push(Instruction::ConstStr("start".into()));
                self.compile_expr(end, out)?;
                out.push(Instruction::ConstStr("end".into()));
                if *inclusive {
                    out.push(Instruction::ConstBool(true));
                    out.push(Instruction::ConstStr("inclusive".into()));
                    out.push(Instruction::DictBuild(3));
                } else {
                    out.push(Instruction::DictBuild(2));
                }
            }
            Expr::Ternary {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.compile_expr(condition, out)?;
                let jump_else = out.len();
                out.push(Instruction::JumpIfFalse(0));
                self.compile_expr(then_branch, out)?;
                let jump_end = out.len();
                out.push(Instruction::Jump(0));
                let else_label = out.len();
                if let Instruction::JumpIfFalse(ref mut t) = out[jump_else] {
                    *t = else_label;
                }
                self.compile_expr(else_branch, out)?;
                let end = out.len();
                if let Instruction::Jump(ref mut t) = out[jump_end] {
                    *t = end;
                }
            }
            Expr::VariantPattern { .. } | Expr::TuplePattern { .. } => {}
            Expr::Super { .. } => {
                self.compile_load_binding(&SmolStr::new("self"), out)?;
            }
        }
        Ok(())
    }

    fn compile_range_contains(
        &mut self,
        object: &Expr,
        args: &[CallArg],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let slot = self.alloc_local(&SmolStr::new("__range"));
        self.compile_expr(object, out)?;
        out.push(Instruction::StoreLocal(slot));
        let val_slot = self.alloc_local(&SmolStr::new("__range_val"));
        self.compile_expr(&args[0].value, out)?;
        out.push(Instruction::StoreLocal(val_slot));
        let ge_slot = self.alloc_local(&SmolStr::new("__range_ge"));
        out.push(Instruction::LoadLocal(val_slot));
        out.push(Instruction::LoadLocal(slot));
        out.push(Instruction::ConstStr("start".into()));
        out.push(Instruction::ObjectGet);
        out.push(Instruction::I32Ge);
        out.push(Instruction::StoreLocal(ge_slot));
        let end_check_slot = self.alloc_local(&SmolStr::new("__range_end_ok"));
        out.push(Instruction::LoadLocal(slot));
        out.push(Instruction::ConstStr("inclusive".into()));
        out.push(Instruction::ObjectGet);
        let jump_half_open = out.len();
        out.push(Instruction::JumpIfFalse(0));
        out.push(Instruction::LoadLocal(val_slot));
        out.push(Instruction::LoadLocal(slot));
        out.push(Instruction::ConstStr("end".into()));
        out.push(Instruction::ObjectGet);
        out.push(Instruction::I32Le);
        out.push(Instruction::StoreLocal(end_check_slot));
        let jump_merge = out.len();
        out.push(Instruction::Jump(0));
        let half_open_label = out.len();
        if let Instruction::JumpIfFalse(ref mut target) = out[jump_half_open] {
            *target = half_open_label;
        }
        out.push(Instruction::LoadLocal(val_slot));
        out.push(Instruction::LoadLocal(slot));
        out.push(Instruction::ConstStr("end".into()));
        out.push(Instruction::ObjectGet);
        out.push(Instruction::I32Lt);
        out.push(Instruction::StoreLocal(end_check_slot));
        let merge_label = out.len();
        if let Instruction::Jump(ref mut target) = out[jump_merge] {
            *target = merge_label;
        }
        out.push(Instruction::LoadLocal(ge_slot));
        out.push(Instruction::LoadLocal(end_check_slot));
        out.push(Instruction::BoolAnd);
        Ok(())
    }

    fn compile_call_args(&mut self, args: &[CallArg], out: &mut Vec<Instruction>) -> SyxResult<()> {
        for arg in args {
            self.compile_expr(&arg.value, out)?;
        }
        Ok(())
    }

    fn compile_ordered_call_args(
        &mut self,
        type_name: &SmolStr,
        args: &[CallArg],
        fields: &[SmolStr],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        for field in fields {
            let arg = args
                .iter()
                .find(|a| a.label.as_deref() == Some(field.as_str()))
                .or_else(|| {
                    let idx = fields.iter().position(|f| f == field)?;
                    args.get(idx).filter(|a| a.label.is_none())
                });
            if let Some(arg) = arg {
                self.compile_expr(&arg.value, out)?;
            } else if let Some(default) = self
                .dataclass_field_defaults
                .get(&(type_name.clone(), field.clone()))
            {
                // S1-I2: struct memberwise default (Counter value = 0).
                out.push(Instruction::ConstI32(*default));
            } else {
                out.push(Instruction::ConstNil);
            }
        }
        Ok(())
    }

    fn resolve_call(&self, callee: &Expr) -> Option<(HostFnId, bool)> {
        if let Expr::Member { object, name, .. } = callee {
            if let Expr::Ident {
                name: type_name, ..
            } = object.as_ref()
            {
                if matches!(type_name.as_str(), "Result" | "BigInt") {
                    return None;
                }
                if self.enum_classes.contains_key(type_name)
                    || self.final_classes.contains_key(type_name)
                {
                    return None;
                }
            }
            match name.as_str() {
                "json" => return Some((HostFnId::HttpResponseJson, true)),
                "status" => return Some((HostFnId::HttpResponseStatus, true)),
                "text" | "body" => return Some((HostFnId::HttpResponseText, true)),
                "ok" => return Some((HostFnId::HttpResponseOk, true)),
                "header" => return Some((HostFnId::HttpResponseHeader, true)),
                "headers" => return Some((HostFnId::HttpResponseHeaders, true)),
                _ => {}
            }
            if let Expr::Ident {
                name: type_name, ..
            } = object.as_ref()
            {
                if let Some(tm) = lookup_type_method(type_name, name) {
                    return tm.host.map(|h| (h, false));
                }
            }
        }
        None
    }

    fn bind_switch_pattern_bindings(
        &mut self,
        pattern: &Expr,
        case_slot: u32,
        out: &mut Vec<Instruction>,
    ) {
        if let Expr::VariantPattern { bindings, .. } = pattern {
            for (i, (label, bind)) in bindings.iter().enumerate() {
                let bind_slot = self.alloc_local(bind);
                out.push(Instruction::LoadLocal(case_slot));
                let field = label
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("_{i}"));
                out.push(Instruction::ConstStr(field));
                out.push(Instruction::ObjectGet);
                out.push(Instruction::StoreLocal(bind_slot));
            }
        }
        if let Expr::TuplePattern { bindings, .. } = pattern {
            for (i, bind) in bindings.iter().enumerate() {
                let bind_slot = self.alloc_local(bind);
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstI32(i as i32));
                out.push(Instruction::ArrayGet);
                out.push(Instruction::StoreLocal(bind_slot));
            }
        }
    }

    fn compile_switch_pattern_match(
        &mut self,
        pattern: &Expr,
        case_slot: u32,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        match pattern {
            Expr::HalfOpenRange { start, end, .. } => {
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstI32(*start as i32));
                out.push(Instruction::I32Ge);
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstI32(*end as i32));
                out.push(Instruction::I32Lt);
                out.push(Instruction::BoolAnd);
            }
            Expr::ClosedRange { start, end, .. } => {
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstI32(*start as i32));
                out.push(Instruction::I32Ge);
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstI32(*end as i32));
                out.push(Instruction::I32Le);
                out.push(Instruction::BoolAnd);
            }
            Expr::TuplePattern { bindings, .. } => {
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::MethodCall(
                    syx_stdlib::lookup_array_method("count")
                        .map(|m| m.index())
                        .unwrap_or(0),
                ));
                out.push(Instruction::ConstI32(bindings.len() as i32));
                out.push(Instruction::I32Eq);
            }
            Expr::VariantPattern { variant, .. } => {
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstStr("case".into()));
                out.push(Instruction::ObjectGet);
                out.push(Instruction::ConstStr(variant.to_string()));
                out.push(Instruction::StrEq);
            }
            Expr::Is { target, .. } => {
                out.push(Instruction::LoadLocal(case_slot));
                out.push(Instruction::ConstStr(
                    Self::type_expr_name(target).to_string(),
                ));
                out.push(Instruction::CallHost(host_fn_index(HostFnId::TypeIs)));
            }
            other => {
                out.push(Instruction::LoadLocal(case_slot));
                self.compile_expr(other, out)?;
                out.push(Instruction::I32Eq);
            }
        }
        Ok(())
    }

    fn infer_class_method_type_arg_names(&self, args: &[CallArg]) -> Vec<SmolStr> {
        for arg in args {
            if let Some(ty) = self.expr_types.get(&expr_span_start(&arg.value)) {
                if let TypeId::Array(elem) = ty {
                    let elem_name = normalize_type_arg_name(type_id_to_ir_name(elem));
                    if elem_name.as_str() != "Unknown" {
                        return vec![elem_name];
                    }
                }
            }
            if let Some(ty) = self.infer_expr_type(&arg.value) {
                if ty.starts_with("Array<") && ty.ends_with('>') {
                    let inner = ty.trim_start_matches("Array<").trim_end_matches('>');
                    if !inner.is_empty() && inner != "Unknown" {
                        return vec![normalize_type_arg_name(SmolStr::new(inner))];
                    }
                }
            }
        }
        self.infer_call_type_arg_names(args)
    }

    fn resolve_static_monomorph(&self, qualified: &SmolStr, args: &[CallArg]) -> Option<SmolStr> {
        if let Some((_, type_args)) = self
            .instantiations
            .iter()
            .find(|(base, _)| base.as_str() == qualified.as_str())
        {
            let mangled = mangle_name(qualified.as_str(), type_args);
            if self.has_function(&mangled) {
                return Some(mangled);
            }
        }
        let type_args = self.infer_class_method_type_arg_names(args);
        if !type_args.is_empty() && type_args.iter().all(|t| t.as_str() != "Unknown") {
            let mangled = mangle_name(qualified.as_str(), &type_args);
            if self.has_function(&mangled) {
                return Some(mangled);
            }
        }
        let prefix = format!("{qualified}_");
        let matches: Vec<_> = self
            .program
            .functions
            .iter()
            .filter(|f| f.name.as_str().starts_with(prefix.as_str()))
            .map(|f| f.name.clone())
            .collect();
        if matches.len() == 1 {
            return Some(matches[0].clone());
        }
        None
    }

    fn compile_generic_qualified_call(
        &mut self,
        qualified: &SmolStr,
        args: &[CallArg],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        for arg in args {
            self.compile_expr(&arg.value, out)?;
        }
        let mangled = self
            .instantiations
            .iter()
            .find(|(base, _)| base.as_str() == qualified.as_str())
            .map(|(_, type_args)| mangle_name(qualified.as_str(), type_args))
            .unwrap_or_else(|| {
                mangle_name(
                    qualified.as_str(),
                    &self.infer_class_method_type_arg_names(args),
                )
            });
        out.push(Instruction::Call(self.require_function_index(&mangled)?));
        Ok(())
    }

    fn infer_call_type_arg_names(&self, args: &[CallArg]) -> Vec<SmolStr> {
        args.iter()
            .map(|a| {
                self.infer_expr_type(&a.value)
                    .unwrap_or_else(|| SmolStr::new("Unknown"))
            })
            .collect()
    }

    fn infer_expr_type(&self, expr: &Expr) -> Option<SmolStr> {
        let inferred = match expr {
            Expr::Literal { value, .. } => match value {
                Literal::String(_) | Literal::Interpolated(_) | Literal::Regex(_) => {
                    Some(SmolStr::new("String"))
                }
                Literal::Integer { .. } | Literal::Double { .. } => {
                    Some(SmolStr::new(if cfg!(target_pointer_width = "64") {
                        "Int64"
                    } else {
                        "Int32"
                    }))
                }
                Literal::BigInt(_) => Some(SmolStr::new("BigInt")),
                Literal::Bool(_) => Some(SmolStr::new("Bool")),
                Literal::Dict(_) => Some(SmolStr::new("Dictionary")),
                _ => None,
            },
            Expr::Array { elements, .. } => {
                let elem = elements
                    .first()
                    .and_then(|e| self.infer_expr_type(e))
                    .unwrap_or_else(|| SmolStr::new("Unknown"));
                Some(SmolStr::new(format!("Array<{elem}>")))
            }
            Expr::Ident { name, .. } => self.var_types.get(name).cloned(),
            Expr::Call { callee, .. } => {
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if let Some(host_id) = syx_stdlib::host_fn_id_by_name(name.as_str()) {
                        let ret = syx_stdlib::host_fn_return_type(host_id);
                        if ret == "Array" {
                            return Some(SmolStr::new("Array"));
                        }
                        if !ret.is_empty() && ret != "Nil" && ret != "Void" {
                            return Some(SmolStr::new(ret));
                        }
                    }
                    if self.dataclass_names.contains(name) || self.class_types.contains_key(name) {
                        return Some(name.clone());
                    }
                }
                if let Expr::Member { object, name, .. } = callee.as_ref() {
                    if let Expr::Ident { name: sealed, .. } = object.as_ref() {
                        if self.enum_classes.contains_key(sealed)
                            || self.final_classes.contains_key(sealed)
                        {
                            return Some(sealed.clone());
                        }
                    }
                    if name == "map" || name == "filter" || name == "sorted" || name == "reverse" {
                        if self.expr_is_array_like(object) {
                            return Some(SmolStr::new("Array"));
                        }
                    }
                    if name == "split" || name == "matches" || name == "readLines" {
                        return Some(SmolStr::new("Array"));
                    }
                    if let Expr::Ident {
                        name: type_name, ..
                    } = object.as_ref()
                    {
                        if matches!(type_name.as_str(), "Http" | "HTTP")
                            && matches!(
                                name.as_str(),
                                "get" | "post" | "fetch" | "request" | "put" | "patch" | "delete"
                            )
                        {
                            return Some(SmolStr::new("HTTPResponse"));
                        }
                    }
                    if self.expr_is_string_like(object) {
                        if let Some(m) = lookup_string_method(name) {
                            return Some(SmolStr::new(method_return_type(m)));
                        }
                    }
                    if self.expr_is_array_like(object) {
                        if let Some(m) = lookup_array_method(name) {
                            return Some(SmolStr::new(method_return_type(m)));
                        }
                    }
                }
                None
            }
            Expr::Member { object, name, .. } => {
                if self.expr_is_http_response_like(object) {
                    if let Some(m) = lookup_http_response_method(name) {
                        return Some(SmolStr::new(method_return_type(m)));
                    }
                }
                if self.expr_is_string_like(object) {
                    if let Some(m) = lookup_string_method(name) {
                        return Some(SmolStr::new(method_return_type(m)));
                    }
                }
                if self.expr_is_array_like(object) {
                    if let Some(m) = lookup_array_method(name) {
                        return Some(SmolStr::new(method_return_type(m)));
                    }
                }
                self.infer_expr_type(object)
            }
            _ => None,
        };
        inferred.or_else(|| self.infer_expr_type_from_semantic(expr))
    }

    fn infer_expr_type_from_semantic(&self, expr: &Expr) -> Option<SmolStr> {
        self.expr_types.get(&expr_span_start(expr)).map(|ty| {
            type_id_collection_base(ty)
                .map(SmolStr::new)
                .unwrap_or_else(|| type_id_to_ir_name(ty))
        })
    }

    fn type_name_is_dict(name: &str) -> bool {
        name == "Object" || name == "Dictionary" || name.starts_with("Dictionary<")
    }

    fn type_name_is_set(name: &str) -> bool {
        name == "Set" || name.starts_with("Set<")
    }

    fn type_name_is_array(name: &str) -> bool {
        name == "Array" || name.starts_with("Array<")
    }

    fn expr_is_string_like(&self, expr: &Expr) -> bool {
        if let Some(ty) = self.expr_types.get(&expr_span_start(expr)) {
            if matches!(ty, TypeId::String) {
                return true;
            }
            if let TypeId::Named(n) = ty {
                if n.as_str() == "Character" {
                    return true;
                }
            }
        }
        if matches!(
            expr,
            Expr::Literal {
                value: Literal::String(_) | Literal::Interpolated(_),
                ..
            }
        ) || matches!(self.infer_expr_type(expr).as_deref(), Some("String"))
        {
            return true;
        }
        // Chained `a + b + c`: nested Add of string-like operands is ConcatStr.
        if let Expr::Binary {
            left, op, right, ..
        } = expr
        {
            if matches!(op, BinOp::Add)
                && (self.expr_is_string_like(left) || self.expr_is_string_like(right))
            {
                return true;
            }
        }
        false
    }

    fn try_emit_hosted_collection_call(
        &self,
        type_base: &str,
        method: &str,
        out: &mut Vec<Instruction>,
    ) -> bool {
        if type_base == "Array" && method == "sorted" {
            out.push(Instruction::ArrSorted);
            return true;
        }
        let base = SmolStr::new(type_base);
        let name = SmolStr::new(method);
        if self
            .extension_methods
            .contains(&(base.clone(), name.clone()))
        {
            if method == "debugDescription" {
                out.push(Instruction::CallHost(host_fn_index(
                    HostFnId::ValueDebugDescription,
                )));
                return true;
            }
            if method == "description" {
                out.push(Instruction::CallHost(host_fn_index(
                    HostFnId::ValueDescription,
                )));
                return true;
            }
        }
        if !is_manifest_host_op(type_base, method) {
            return false;
        }
        if let Some(host) = manifest_host_name(type_base, method) {
            if let Some(idx) = host_fn_index_by_name(&host) {
                out.push(Instruction::CallHost(idx));
                return true;
            }
        }
        if self
            .extension_methods
            .contains(&(base.clone(), name.clone()))
        {
            out.push(Instruction::Call(
                self.function_index(&SmolStr::new(format!("{type_base}_{method}"))),
            ));
            return true;
        }
        false
    }

    fn expr_is_array_like(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Array { .. })
            || self
                .infer_expr_type(expr)
                .is_some_and(|t| Self::type_name_is_array(t.as_str()))
    }

    fn expr_is_http_response_like(&self, expr: &Expr) -> bool {
        self.infer_expr_type(expr)
            .is_some_and(|t| t.as_str() == "HTTPResponse" || t.as_str() == "Object")
            || matches!(
                expr,
                Expr::Call { callee, .. } if matches!(
                    callee.as_ref(),
                    Expr::Member {
                        object,
                        name,
                        ..
                    } if matches!(
                        object.as_ref(),
                        Expr::Ident { name: tn, .. }
                            if matches!(tn.as_str(), "Http" | "HTTP")
                    ) && matches!(
                        name.as_str(),
                        "get" | "post" | "fetch" | "request"
                    )
                )
            )
    }

    fn expr_is_set_like(&self, expr: &Expr) -> bool {
        self.infer_expr_type(expr)
            .is_some_and(|t| Self::type_name_is_set(t.as_str()))
            || matches!(
                expr,
                Expr::Ident { name, .. } if self
                    .var_types
                    .get(name)
                    .is_some_and(|t| Self::type_name_is_set(t))
            )
    }

    fn expr_is_dict_like(&self, expr: &Expr) -> bool {
        matches!(
            expr,
            Expr::Literal {
                value: Literal::Dict(_),
                ..
            }
        ) || self
            .infer_expr_type(expr)
            .is_some_and(|t| Self::type_name_is_dict(t.as_str()))
            || matches!(
                expr,
                Expr::Ident { name, .. } if self
                    .var_types
                    .get(name)
                    .is_some_and(|t| Self::type_name_is_dict(t))
            )
    }

    fn has_function(&self, name: &SmolStr) -> bool {
        self.program.functions.iter().any(|f| f.name == *name)
    }

    fn function_index(&self, name: &SmolStr) -> u32 {
        self.program
            .functions
            .iter()
            .position(|f| f.name == *name)
            .unwrap_or(0) as u32
    }

    fn require_function_index(&self, name: &SmolStr) -> SyxResult<u32> {
        self.program
            .functions
            .iter()
            .position(|f| f.name == *name)
            .map(|i| i as u32)
            .ok_or_else(|| {
                codegen_error_at(
                    format!("Unknown function '{name}'"),
                    None,
                    Span::DUMMY,
                    "C020",
                )
            })
    }

    fn alloc_local(&mut self, name: &SmolStr) -> u32 {
        if let Some(&slot) = self.locals.get(name) {
            slot
        } else {
            let slot = self.local_count;
            self.local_count += 1;
            self.locals.insert(name.clone(), slot);
            slot
        }
    }

    fn local_names_for_slots(
        locals: &indexmap::IndexMap<SmolStr, u32>,
        local_count: u32,
    ) -> Vec<SmolStr> {
        let mut names = (0..local_count)
            .map(|i| SmolStr::new(format!("local_{i}")))
            .collect::<Vec<_>>();
        for (name, slot) in locals {
            if (*slot as usize) < names.len() {
                names[*slot as usize] = name.clone();
            }
        }
        names
    }

    fn local_types_for_slots(
        locals: &indexmap::IndexMap<SmolStr, u32>,
        var_types: &indexmap::IndexMap<SmolStr, SmolStr>,
        params: &[syx_ast::Param],
        local_count: u32,
        fn_name: &SmolStr,
    ) -> Vec<SmolStr> {
        let mut types = (0..local_count)
            .map(|_| SmolStr::new("Unknown"))
            .collect::<Vec<_>>();
        for param in params {
            if let Some(&slot) = locals.get(&param.name) {
                let ty = param
                    .ty
                    .as_ref()
                    .map(|t| Self::type_expr_name(t))
                    .unwrap_or_else(|| SmolStr::new("Unknown"));
                types[slot as usize] = ty;
            }
        }
        if let Some((type_name, _)) = fn_name.as_str().split_once('_') {
            if let Some(&slot) = locals.get(&SmolStr::new("self")) {
                types[slot as usize] = SmolStr::new(type_name);
            }
        }
        for (name, slot) in locals {
            if let Some(ty) = var_types.get(name) {
                types[*slot as usize] = ty.clone();
            }
        }
        types
    }

    fn type_expr_name(ty: &syx_ast::TypeExpr) -> SmolStr {
        match ty {
            syx_ast::TypeExpr::Named { name, generics } => {
                if generics.is_empty() {
                    name.clone()
                } else {
                    let args = generics
                        .iter()
                        .map(Self::type_expr_name)
                        .collect::<Vec<_>>()
                        .join(", ");
                    SmolStr::new(format!("{name}<{args}>"))
                }
            }
            syx_ast::TypeExpr::Array(inner) => {
                SmolStr::new(format!("Array<{}>", Self::type_expr_name(inner)))
            }
            syx_ast::TypeExpr::Optional(inner) => {
                SmolStr::new(format!("{}?", Self::type_expr_name(inner)))
            }
            syx_ast::TypeExpr::Dictionary { key, value, .. } => SmolStr::new(format!(
                "Dictionary<{}, {}>",
                Self::type_expr_name(key),
                Self::type_expr_name(value)
            )),
            _ => SmolStr::new("Unknown"),
        }
    }

    fn resolve_binding_name<'a>(&'a self, name: &'a SmolStr) -> &'a SmolStr {
        match self.mut_ref_aliases.get(name) {
            Some(alias) => alias,
            None => name,
        }
    }

    fn uses_mut_ref_cell(&self, name: &SmolStr) -> bool {
        self.mut_ref_params.contains(name)
    }

    fn compile_load_binding(
        &mut self,
        name: &SmolStr,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let resolved = self.resolve_binding_name(name).clone();
        if self.uses_mut_ref_cell(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                out.push(Instruction::LoadLocal(slot));
                out.push(Instruction::ConstStr(MUT_REF_KEY.into()));
                out.push(Instruction::ObjectGet);
            }
            return Ok(());
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            out.push(Instruction::LoadLocal(slot));
        } else if let Some(idx) = self.program.globals.iter().position(|g| g == &resolved) {
            out.push(Instruction::LoadGlobal(idx as u32));
        } else {
            return Err(codegen_error_at(
                format!("Unknown variable '{name}'"),
                None,
                Span::DUMMY,
                "G003",
            ));
        }
        Ok(())
    }

    fn compile_inline_stmts(
        &mut self,
        stmts: &[Stmt],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        for stmt in stmts {
            self.compile_stmt(stmt, out, &mut None)?;
        }
        Ok(())
    }

    fn emit_scope_deinit_locals(&mut self, out: &mut Vec<Instruction>) -> SyxResult<()> {
        let locals: Vec<(SmolStr, String)> = self
            .var_types
            .iter()
            .filter(|(name, ty)| {
                self.locals.contains_key(*name) && self.class_types.contains_key(ty.as_str())
            })
            .map(|(n, t)| (n.clone(), t.to_string()))
            .collect();
        for (name, type_name) in locals {
            let deinit_fn = SmolStr::new(format!("{type_name}_deinit"));
            if self.has_function(&deinit_fn) {
                self.compile_load_binding(&name, out)?;
                out.push(Instruction::Call(self.function_index(&deinit_fn)));
            }
        }
        Ok(())
    }

    fn emit_binding_override_deinit(
        &mut self,
        name: &SmolStr,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(type_name) = self.var_types.get(&resolved).cloned() else {
            return Ok(());
        };
        if !self.class_types.contains_key(type_name.as_str()) {
            return Ok(());
        }
        let deinit_fn = SmolStr::new(format!("{type_name}_deinit"));
        if !self.has_function(&deinit_fn) {
            return Ok(());
        }
        let new_temp = self.alloc_local(&SmolStr::new(format!("__p101_store_new_{resolved}")));
        out.push(Instruction::StoreLocal(new_temp));
        self.compile_load_binding(&resolved, out)?;
        out.push(Instruction::Call(self.function_index(&deinit_fn)));
        out.push(Instruction::LoadLocal(new_temp));
        Ok(())
    }

    fn compile_store_binding(
        &mut self,
        name: &SmolStr,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        self.emit_binding_override_deinit(name, out)?;
        let resolved = self.resolve_binding_name(name).clone();
        if self.uses_mut_ref_cell(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
                out.push(Instruction::StoreLocal(temp));
                out.push(Instruction::LoadLocal(slot));
                out.push(Instruction::LoadLocal(temp));
                out.push(Instruction::ConstStr(MUT_REF_KEY.into()));
                out.push(Instruction::ObjectSet);
                out.push(Instruction::StoreLocal(slot));
            }
            return Ok(());
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            out.push(Instruction::StoreLocal(slot));
        } else if let Some(idx) = self.program.globals.iter().position(|g| g == &resolved) {
            out.push(Instruction::StoreGlobal(idx as u32));
        }
        Ok(())
    }

    fn emit_mut_ref_cell_wrap(out: &mut Vec<Instruction>) {
        out.push(Instruction::ConstStr(MUT_REF_KEY.into()));
        out.push(Instruction::ConstStr("MutRefCell".into()));
        out.push(Instruction::ConstStr("__type".into()));
        out.push(Instruction::DictBuild(2));
    }

    fn compile_mut_ref_cell(
        &mut self,
        target: &SmolStr,
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        self.compile_load_binding(target, out)?;
        Self::emit_mut_ref_cell_wrap(out);
        Ok(())
    }

    fn fn_enclosing_class(fn_name: &SmolStr) -> Option<SmolStr> {
        let s = fn_name.as_str();
        let (type_name, _) = s.rsplit_once('_')?;
        Some(SmolStr::new(type_name))
    }

    fn instance_self_mut_ref_method(&self, name: &SmolStr) -> bool {
        let Some((type_name, rest)) = name.split_once('_') else {
            return false;
        };
        if rest == "init" || rest.ends_with("_init") {
            return false;
        }
        let type_smol = SmolStr::new(type_name);
        self.class_types.contains_key(&type_smol) || self.actor_types.contains(&type_smol)
    }

    fn param_is_mut_reference(param_ty: &Option<TypeExpr>) -> bool {
        matches!(param_ty, Some(TypeExpr::Reference(_, true, _)))
    }

    fn compile_call_with_mut_refs(
        &mut self,
        callee: &Expr,
        args: &[CallArg],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let callee_name = match callee {
            Expr::Ident { name, .. } => Some(name.clone()),
            _ => None,
        };
        let param_types = callee_name
            .as_ref()
            .and_then(|n| self.function_param_types.get(n))
            .cloned()
            .unwrap_or_default();

        let mut unbox_targets: Vec<(SmolStr, u32)> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let is_mut_param = param_types.get(i).is_some_and(Self::param_is_mut_reference);
            if is_mut_param {
                if let Expr::Borrow {
                    expr,
                    mutable: true,
                    ..
                } = &arg.value
                {
                    if let Expr::Ident { name: target, .. } = expr.as_ref() {
                        let cell_name = SmolStr::new(format!("__syx_cell_{i}"));
                        let cell_slot = self.alloc_local(&cell_name);
                        self.compile_mut_ref_cell(target, out)?;
                        out.push(Instruction::StoreLocal(cell_slot));
                        out.push(Instruction::LoadLocal(cell_slot));
                        unbox_targets.push((target.clone(), cell_slot));
                        continue;
                    }
                }
            }
            self.compile_expr(&arg.value, out)?;
        }

        if let Some(name) = callee_name {
            if self.generic_templates.contains(&name) {
                // Prefer semantic instantiations (ADR-324: Array<Element> where needs Element).
                let mangled = self
                    .instantiations
                    .iter()
                    .find(|(base, _)| base == &name)
                    .map(|(_, type_args)| mangle_name(name.as_str(), type_args))
                    .unwrap_or_else(|| {
                        mangle_name(name.as_str(), &self.infer_call_type_arg_names(args))
                    });
                out.push(Instruction::Call(self.require_function_index(&mangled)?));
            } else {
                out.push(Instruction::Call(self.require_function_index(&name)?));
            }
        } else if let Expr::Member {
            object,
            name: member,
            ..
        } = callee
        {
            if let Expr::Ident {
                name: type_name, ..
            } = object.as_ref()
            {
                let type_args = if self.generic_enum_templates.contains(type_name)
                    || self.generic_final_templates.contains(type_name)
                {
                    self.infer_call_type_arg_names(args)
                } else {
                    vec![]
                };
                let mangled = if type_args.is_empty() {
                    type_name.clone()
                } else {
                    mangle_name(type_name.as_str(), &type_args)
                };
                let init_name = SmolStr::new(format!("{mangled}_{member}_init"));
                self.compile_call_args(args, out)?;
                out.push(Instruction::Call(self.function_index(&init_name)));
            } else {
                return Err(codegen_error_at(
                    "Unsupported member call callee",
                    None,
                    Span::DUMMY,
                    "G004",
                ));
            }
        } else {
            self.compile_expr(callee, out)?;
            out.push(Instruction::Call(0));
        }

        for (target, cell_slot) in unbox_targets {
            out.push(Instruction::LoadLocal(cell_slot));
            out.push(Instruction::ConstStr(MUT_REF_KEY.into()));
            out.push(Instruction::ObjectGet);
            let resolved = self.resolve_binding_name(&target).clone();
            if let Some(&slot) = self.locals.get(&resolved) {
                out.push(Instruction::StoreLocal(slot));
            }
        }
        Ok(())
    }
}

fn entry_point_from_module(module: &Module) -> SmolStr {
    for decl in &module.decls {
        if let Decl::Function { name, attrs, .. } = decl {
            if attrs.iter().any(|a| a.name == "Main") {
                return name.clone();
            }
        }
    }
    SmolStr::new("main")
}

impl CodeGenerator {
    #[allow(dead_code)]
    fn method_has_vtable_slot(&self, class: &str, method: &str) -> bool {
        self.class_vtables
            .get(&SmolStr::new(class))
            .is_some_and(|vt| vt.contains_key(&SmolStr::new(method)))
    }

    fn emit_witness_protocol_call(
        &mut self,
        object: &Expr,
        protocol: &str,
        method: &str,
        args: &[syx_ast::CallArg],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let recv_slot = self.alloc_local(&SmolStr::new("__witness_recv"));
        self.compile_expr(object, out)?;
        out.push(Instruction::StoreLocal(recv_slot));
        let mut arg_slots = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let slot = self.alloc_local(&SmolStr::new(format!("__witness_arg{i}")));
            self.compile_expr(&arg.value, out)?;
            out.push(Instruction::StoreLocal(slot));
            arg_slots.push(slot);
        }
        out.push(Instruction::LoadLocal(recv_slot));
        for slot in &arg_slots {
            out.push(Instruction::LoadLocal(*slot));
        }
        out.push(Instruction::LoadLocal(recv_slot));
        out.push(Instruction::Dup);
        out.push(Instruction::ConstStr(format!(
            "__witness_{protocol}_{method}"
        )));
        out.push(Instruction::ObjectGet);
        out.push(Instruction::CallIndirect((1 + args.len()) as u32));
        Ok(())
    }

    #[allow(dead_code)]
    fn emit_vtable_instance_call(
        &mut self,
        object: &Expr,
        method: &str,
        args: &[syx_ast::CallArg],
        out: &mut Vec<Instruction>,
    ) -> SyxResult<()> {
        let recv_slot = self.alloc_local(&SmolStr::new("__vtable_recv"));
        self.compile_expr(object, out)?;
        out.push(Instruction::StoreLocal(recv_slot));
        let mut arg_slots = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let slot = self.alloc_local(&SmolStr::new(format!("__vtable_arg{i}")));
            self.compile_expr(&arg.value, out)?;
            out.push(Instruction::StoreLocal(slot));
            arg_slots.push(slot);
        }
        out.push(Instruction::LoadLocal(recv_slot));
        for slot in &arg_slots {
            out.push(Instruction::LoadLocal(*slot));
        }
        out.push(Instruction::LoadLocal(recv_slot));
        out.push(Instruction::Dup);
        out.push(Instruction::ConstStr(format!("__vtable_{method}")));
        out.push(Instruction::ObjectGet);
        out.push(Instruction::CallIndirect((1 + args.len()) as u32));
        Ok(())
    }

    fn range_bounds(expr: &Expr) -> Option<(i64, i64, bool)> {
        match expr {
            Expr::HalfOpenRange { start, end, .. } => Some((*start, *end, false)),
            Expr::ClosedRange { start, end, .. } => Some((*start, *end, true)),
            Expr::Group { expr, .. } => Self::range_bounds(expr),
            _ => None,
        }
    }

    fn expr_is_opaque_equatable(&self, expr: &Expr) -> bool {
        if let Some(ty) = self.expr_types.get(&expr_span_start(expr)) {
            if matches!(ty, TypeId::OpaqueSome(_) | TypeId::OpaqueAny(_)) {
                return true;
            }
        }
        self.infer_expr_type(expr)
            .is_some_and(|t| t.starts_with("some ") || t.starts_with("any "))
    }

    fn opaque_equatable_witness_index(&self) -> u32 {
        self.program
            .functions
            .iter()
            .position(|f| f.name == "__syx_opaque_equatable_eq")
            .unwrap_or(0) as u32
    }

    fn eq_instruction_for_exprs(&self, left: &Expr, right: &Expr, op: BinOp) -> Instruction {
        let ne = op == BinOp::NotEq;
        if self.expr_is_opaque_equatable(left) || self.expr_is_opaque_equatable(right) {
            return Instruction::CallWitness(0, self.opaque_equatable_witness_index());
        }
        let left_type = self
            .infer_expr_type(left)
            .or_else(|| self.infer_expr_type_from_semantic(left));
        let right_type = self
            .infer_expr_type(right)
            .or_else(|| self.infer_expr_type_from_semantic(right));

        if left_type.as_deref() == Some("String") && right_type.as_deref() == Some("String") {
            return if ne {
                Instruction::StrNe
            } else {
                Instruction::StrEq
            };
        }
        if left_type.as_deref() == Some("Bool") && right_type.as_deref() == Some("Bool") {
            return if ne {
                Instruction::BoolNe
            } else {
                Instruction::BoolEq
            };
        }
        if let (Some(lt), Some(rt)) = (&left_type, &right_type) {
            if is_equatable_type(lt.as_str()) && is_equatable_type(rt.as_str()) {
                if let (Some(lk), Some(rk)) =
                    (from_type_name(lt.as_str()), from_type_name(rt.as_str()))
                {
                    if let Some(nk) = widen_for_binary(lk, rk) {
                        return if ne {
                            Instruction::NumericNe(nk.to_byte())
                        } else {
                            Instruction::NumericEq(nk.to_byte())
                        };
                    }
                }
            }
        }
        if ne {
            Instruction::ValueNe
        } else {
            Instruction::ValueEq
        }
    }

    fn numeric_kind_for_binary(&self, left: &Expr, right: &Expr) -> u8 {
        let left_type = self
            .infer_expr_type(left)
            .or_else(|| self.infer_expr_type_from_semantic(left));
        let right_type = self
            .infer_expr_type(right)
            .or_else(|| self.infer_expr_type_from_semantic(right));
        if let (Some(lt), Some(rt)) = (left_type.as_deref(), right_type.as_deref()) {
            if let (Some(lk), Some(rk)) = (from_type_name(lt), from_type_name(rt)) {
                if let Some(nk) = widen_for_binary(lk, rk) {
                    return nk.to_byte();
                }
            }
        }
        PLATFORM_INT.to_byte()
    }

    fn expr_simd_type(&self, expr: &Expr) -> Option<(u8, syx_stdlib::NumericKind)> {
        self.expr_types
            .get(&expr_span_start(expr))
            .and_then(|ty| ty.as_simd())
    }
}

/// Phase 16: builtin witness for opaque `==` via CallWitness (falls back to ValueEq inside).
fn ensure_opaque_equatable_witness(program: &mut CompiledProgram) {
    const WITNESS: &str = "__syx_opaque_equatable_eq";
    if program.functions.iter().any(|f| f.name == WITNESS) {
        return;
    }
    program.functions.push(FunctionUnit {
        name: SmolStr::new(WITNESS),
        param_count: 2,
        local_count: 2,
        instructions: vec![
            Instruction::LoadLocal(0),
            Instruction::LoadLocal(1),
            Instruction::ValueEq,
            Instruction::Return,
        ],
        instruction_spans: vec![],
        local_names: vec![SmolStr::new("left"), SmolStr::new("right")],
        local_types: vec![SmolStr::new("Unknown"), SmolStr::new("Unknown")],
        source_name: SmolStr::new(""),
        init_body_pc: 0,
        init_wrap_pc: 0,
    });
}

fn compound_to_bin(op: CompoundAssignOp) -> BinOp {
    match op {
        CompoundAssignOp::Add => BinOp::Add,
        CompoundAssignOp::Sub => BinOp::Sub,
        CompoundAssignOp::Mul => BinOp::Mul,
        CompoundAssignOp::Div => BinOp::Div,
        CompoundAssignOp::Mod => BinOp::Mod,
        CompoundAssignOp::BitAnd => BinOp::BitAnd,
        CompoundAssignOp::BitOr => BinOp::BitOr,
        CompoundAssignOp::BitXor => BinOp::BitXor,
        CompoundAssignOp::Shl => BinOp::Shl,
        CompoundAssignOp::Shr => BinOp::Shr,
    }
}

fn collect_closure_free_vars(
    params: &[syx_ast::Param],
    body: &ClosureBody,
    outer_locals: &indexmap::IndexMap<SmolStr, u32>,
) -> Vec<SmolStr> {
    use std::collections::HashSet;
    let mut bound: HashSet<SmolStr> = params.iter().map(|p| p.name.clone()).collect();
    let mut seen = HashSet::new();
    let mut free = Vec::new();

    fn visit_expr(
        expr: &Expr,
        bound: &HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
    ) {
        match expr {
            Expr::Ident { name, .. } => {
                if bound.contains(name) {
                    return;
                }
                if outer_locals.contains_key(name) && seen.insert(name.clone()) {
                    free.push(name.clone());
                }
            }
            Expr::Binary { left, right, .. } => {
                visit_expr(left, bound, outer_locals, seen, free);
                visit_expr(right, bound, outer_locals, seen, free);
            }
            Expr::Unary { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Call { callee, args, .. } => {
                visit_expr(callee, bound, outer_locals, seen, free);
                for arg in args {
                    visit_expr(&arg.value, bound, outer_locals, seen, free);
                }
            }
            Expr::Member { object, .. } => visit_expr(object, bound, outer_locals, seen, free),
            Expr::Index { object, index, .. } => {
                visit_expr(object, bound, outer_locals, seen, free);
                visit_expr(index, bound, outer_locals, seen, free);
            }
            Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
                visit_expr(value, bound, outer_locals, seen, free);
            }
            Expr::MemberAssign { object, value, .. } | Expr::IndexAssign { object, value, .. } => {
                visit_expr(object, bound, outer_locals, seen, free);
                visit_expr(value, bound, outer_locals, seen, free);
            }
            Expr::Group { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Array { elements, .. } => {
                for el in elements {
                    visit_expr(el, bound, outer_locals, seen, free);
                }
            }
            Expr::Await { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::OptionalChain { object, .. } => {
                visit_expr(object, bound, outer_locals, seen, free)
            }
            Expr::ForceUnwrap { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Closure { params, body, .. } => {
                let mut nested_bound = bound.clone();
                for p in params {
                    nested_bound.insert(p.name.clone());
                }
                match body {
                    ClosureBody::Expr(e) => {
                        visit_expr(e, &nested_bound, outer_locals, seen, free);
                    }
                    ClosureBody::Block(stmts) => {
                        visit_stmts(stmts, &mut nested_bound, outer_locals, seen, free);
                    }
                }
            }
            Expr::TaskBlock { body, .. } => {
                visit_stmts(body, &mut bound.clone(), outer_locals, seen, free);
            }
            _ => {}
        }
    }

    fn visit_stmts(
        stmts: &[Stmt],
        bound: &mut HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
    ) {
        for stmt in stmts {
            match stmt {
                Stmt::VarDecl { name, init, .. } => {
                    if let Some(expr) = init {
                        visit_expr(expr, bound, outer_locals, seen, free);
                    }
                    bound.insert(name.clone());
                }
                Stmt::ForIn {
                    name,
                    iterable,
                    body,
                    ..
                } => {
                    visit_expr(iterable, bound, outer_locals, seen, free);
                    bound.insert(name.clone());
                    visit_stmts(body, bound, outer_locals, seen, free);
                    bound.remove(name);
                }
                Stmt::Return { value: Some(v), .. } => {
                    visit_expr(v, bound, outer_locals, seen, free)
                }
                Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                    visit_expr(expr, bound, outer_locals, seen, free);
                }
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    visit_expr(condition, bound, outer_locals, seen, free);
                    visit_stmts(then_branch, bound, outer_locals, seen, free);
                    if let Some(e) = else_branch {
                        visit_stmts(e, bound, outer_locals, seen, free);
                    }
                }
                Stmt::While {
                    condition, body, ..
                } => {
                    visit_expr(condition, bound, outer_locals, seen, free);
                    visit_stmts(body, bound, outer_locals, seen, free);
                }
                Stmt::Block { stmts, .. } => visit_stmts(stmts, bound, outer_locals, seen, free),
                Stmt::Try {
                    body,
                    catch_name,
                    catch_body,
                    ..
                } => {
                    visit_stmts(body, bound, outer_locals, seen, free);
                    bound.insert(catch_name.clone());
                    visit_stmts(catch_body, bound, outer_locals, seen, free);
                    bound.remove(catch_name);
                }
                Stmt::Switch {
                    subject,
                    cases,
                    default,
                    ..
                } => {
                    visit_expr(subject, bound, outer_locals, seen, free);
                    for case in cases {
                        if let Some(g) = &case.guard {
                            visit_expr(g, bound, outer_locals, seen, free);
                        }
                        visit_stmts(&case.body, bound, outer_locals, seen, free);
                    }
                    if let Some(d) = default {
                        visit_stmts(d, bound, outer_locals, seen, free);
                    }
                }
                Stmt::IfCase {
                    pattern,
                    subject,
                    body,
                    else_branch,
                    ..
                } => {
                    visit_expr(pattern, bound, outer_locals, seen, free);
                    visit_expr(subject, bound, outer_locals, seen, free);
                    visit_stmts(body, bound, outer_locals, seen, free);
                    if let Some(e) = else_branch {
                        visit_stmts(e, bound, outer_locals, seen, free);
                    }
                }
                Stmt::IfLet {
                    pattern,
                    body,
                    else_branch,
                    ..
                } => {
                    visit_expr(pattern, bound, outer_locals, seen, free);
                    visit_stmts(body, bound, outer_locals, seen, free);
                    if let Some(e) = else_branch {
                        visit_stmts(e, bound, outer_locals, seen, free);
                    }
                }
                Stmt::GuardLet {
                    body, else_branch, ..
                } => {
                    visit_stmts(body, bound, outer_locals, seen, free);
                    visit_stmts(else_branch, bound, outer_locals, seen, free);
                }
                Stmt::RepeatWhile {
                    body, condition, ..
                } => {
                    visit_stmts(body, bound, outer_locals, seen, free);
                    visit_expr(condition, bound, outer_locals, seen, free);
                }
                Stmt::Throw { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
                _ => {}
            }
        }
    }

    match body {
        ClosureBody::Expr(e) => visit_expr(e, &bound, outer_locals, &mut seen, &mut free),
        ClosureBody::Block(stmts) => {
            visit_stmts(stmts, &mut bound, outer_locals, &mut seen, &mut free)
        }
    }
    free
}

fn stmt_span(stmt: &syx_ast::Stmt) -> (u32, u32) {
    use syx_ast::Stmt;
    let span = match stmt {
        Stmt::Print { span, .. }
        | Stmt::Expr { span, .. }
        | Stmt::VarDecl { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::If { span, .. }
        | Stmt::While { span, .. }
        | Stmt::ForIn { span, .. }
        | Stmt::Block { span, .. }
        | Stmt::Seq { span, .. }
        | Stmt::Break { span, .. }
        | Stmt::Continue { span, .. }
        | Stmt::Try { span, .. }
        | Stmt::Throw { span, .. }
        | Stmt::Switch { span, .. }
        | Stmt::IfCase { span, .. }
        | Stmt::IfLet { span, .. }
        | Stmt::GuardLet { span, .. }
        | Stmt::RepeatWhile { span, .. }
        | Stmt::MacroInvoke { span, .. } => *span,
    };
    (span.start, span.len)
}

fn keypath_from_expr(expr: &Expr) -> Option<(&SmolStr, &SmolStr, bool)> {
    match expr {
        Expr::KeyPath {
            root_type,
            property,
            optional,
            ..
        } => Some((root_type, property, *optional)),
        Expr::Group { expr, .. } => keypath_from_expr(expr),
        _ => None,
    }
}

#[allow(dead_code)]
fn append_with_span(
    out: &mut Vec<Instruction>,
    spans: &mut Vec<Option<(u32, u32)>>,
    instructions: &[Instruction],
    span: (u32, u32),
) {
    for instr in instructions {
        out.push(instr.clone());
        spans.push(Some(span));
    }
}
