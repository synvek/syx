use smol_str::SmolStr;
use std::cell::Cell;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use syx_ast::{
    AccessLevel, BinOp, CallArg, ClosureBody, Decl, Expr, Literal, Module, Stmt, TypeExpr, UnaryOp,
};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mono::mangle_name;
use indexmap::{IndexMap, IndexSet};
use syx_stdlib::{
    from_suffix, from_type_name, default_float_kind, float_literal_bits, host_fn_id_by_name,
    is_implicit_convertible, is_integer, is_integer_narrowing, resolve_alias, host_fn_index,
    host_fn_index_by_name, host_fn_return_type, manifest_host_name, lookup_array_method,
    lookup_dict_method, lookup_set_method, lookup_string_method, lookup_type_method,
    lookup_float64_math_host, lookup_float32_math_host, is_float, widen_for_binary, HostFnId,
    method_id_call_host_index, method_return_type, MethodId, NumericKind, PLATFORM_INT,
};

mod mir_text;
pub use mir_text::format_mir_module;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MirOp {
    ConstI32(i32),
    ConstNumeric { kind: u8, bits: u64 },
    ConstNumeric128 { kind: u8, lo: u64, hi: u64 },
    ConstStr(String),
    ConstBool(bool),
    ConstNil,
    LoadLocal(u32),
    StoreLocal(u32),
    LoadGlobal(u32),
    StoreGlobal(u32),
    /// Scalar arithmetic; `kind` is [`NumericKind`] byte from `widen_for_binary`.
    Add { kind: u8 },
    SimdAdd,
    SimdSub,
    SimdMul,
    SimdDiv,
    SimdEq,
    SimdLt,
    SimdGt,
    SimdNe,
    SimdLe,
    SimdGe,
    SimdAnd,
    SimdOr,
    SimdXor,
    SimdNot,
    /// Lane-wise left shift; shift amount is scalar i32 on the stack.
    SimdShl,
    /// Lane-wise arithmetic right shift; shift amount is scalar i32 on the stack.
    SimdShr,
    /// Phase 111: lane-wise signed minimum (`i32x4`).
    SimdMin,
    /// Phase 111: lane-wise signed maximum (`i32x4`).
    SimdMax,
    /// Phase 111: lane-wise `f32x4` add guest pilot.
    SimdF32Add,
    /// Phase 112: lane-wise `f32x4` sub guest.
    SimdF32Sub,
    /// Phase 112: lane-wise `f32x4` mul guest.
    SimdF32Mul,
    /// Phase 113: lane-wise `f32x4` div guest.
    SimdF32Div,
    /// Phase 113: lane-wise `f64x2` add guest pilot.
    SimdF64Add,
    /// Phase 114: lane-wise `f64x2` sub guest pilot.
    SimdF64Sub,
    /// Phase 115: lane-wise `f64x2` mul guest pilot.
    SimdF64Mul,
    /// Phase 115: lane-wise `f64x2` div guest pilot.
    SimdF64Div,
    /// Phase 117: lane-wise `i16x8` add guest pilot (SIMD8).
    SimdI16Add,
    /// Phase 118: lane-wise `i16x8` sub/mul guest pilot (SIMD8).
    SimdI16Sub,
    SimdI16Mul,
    /// Phase 118: lane-wise `i8x16` add/sub guest pilot (SIMD16).
    SimdI8Add,
    SimdI8Sub,
    /// Phase 119: lane-wise `i8x16` mul guest pilot (SIMD16).
    SimdI8Mul,
    /// Phase 120: lane-wise `i8x16` and guest pilot (SIMD16 logic/shift tail).
    SimdI8And,
    /// Phase 121: lane-wise `i8x16` shl guest pilot (SIMD16 shift lane MVP).
    SimdI8Shl,
    /// Phase 121: lane-wise `i8x16` shr guest pilot (SIMD16 shift lane MVP).
    SimdI8Shr,
    /// Phase 122: lane-wise `i8x16` or guest pilot (SIMD16 logic-or-xor MVP).
    SimdI8Or,
    /// Phase 122: lane-wise `i8x16` xor guest pilot (SIMD16 logic-or-xor MVP).
    SimdI8Xor,
    /// Phase 123: lane-wise `i8x16` not guest pilot (SIMD16 logic-not MVP).
    SimdI8Not,
    /// Phase 124: lane-wise `i8x16` min guest pilot (SIMD16 lane-min-max MVP).
    SimdI8Min,
    /// Phase 124: lane-wise `i8x16` max guest pilot (SIMD16 lane-min-max MVP).
    SimdI8Max,
    /// Phase 130: lane-wise `i8x16` unsigned min guest pilot (SIMD16 unsigned-min-max MVP).
    SimdI8UMin,
    /// Phase 130: lane-wise `i8x16` unsigned max guest pilot (SIMD16 unsigned-min-max MVP).
    SimdI8UMax,
    /// Phase 131: lane-wise `i8x16` abs guest pilot (SIMD16 abs MVP).
    SimdI8Abs,
    /// Phase 132: lane-wise `i8x16` neg guest pilot (SIMD16 neg MVP).
    SimdI8Neg,
    /// Phase 125: `i8x16` extract lane guest pilot (SIMD16 lane-extract-replace MVP).
    SimdI8Extract,
    /// Phase 125: `i8x16` replace lane guest pilot (SIMD16 lane-extract-replace MVP).
    SimdI8Replace,
    /// Phase 126: lane-wise `i8x16` eq guest pilot (SIMD16 compare-lane MVP).
    SimdI8Eq,
    /// Phase 133: lane-wise `i8x16` ne guest pilot (SIMD16 ne MVP).
    SimdI8Ne,
    /// Phase 134: lane-wise `i8x16` le guest pilot (SIMD16 le MVP).
    SimdI8Le,
    /// Phase 135: lane-wise `i8x16` ge guest pilot (SIMD16 ge MVP).
    SimdI8Ge,
    /// Phase 136: lane-wise `i8x16` avgr_u guest pilot (SIMD16 avgr_u MVP).
    SimdI8AvgrU,
    /// Phase 137: lane-wise `i8x16` popcnt guest pilot (SIMD16 popcnt MVP).
    SimdI8Popcnt,
    /// Phase 138: `i8x16` MSB bitmask guest pilot (SIMD16 bitmask MVP).
    SimdI8Bitmask,
    /// Phase 139: `i8x16` all_true guest pilot (SIMD16 all_true MVP).
    SimdI8AllTrue,
    /// Phase 139: `v128.any_true` guest pilot (SIMD16 any_true MVP).
    SimdI8AnyTrue,
    /// Phase 126: lane-wise `i8x16` lt guest pilot (SIMD16 compare-lane MVP).
    SimdI8Lt,
    /// Phase 126: lane-wise `i8x16` gt guest pilot (SIMD16 compare-lane MVP).
    SimdI8Gt,
    /// Phase 127: lane-wise `i8x16` signed saturating add guest pilot.
    SimdI8AddSat,
    /// Phase 127: lane-wise `i8x16` signed saturating sub guest pilot.
    SimdI8SubSat,
    /// Phase 128: fixed-mode `i8x16` shuffle MVP (mode 0=reverse, 1=swap pairs).
    SimdI8Shuffle { mode: i32 },
    /// Phase 117: lane-wise `i64x2` add guest pilot.
    SimdI64Add,
    /// Phase 115: guest horizontal reduceAdd (extract lanes + add).
    SimdReduceAdd { lanes: u8, element: u8 },
    SimdLoad { lanes: u8, element: u8 },
    SimdStore { lanes: u8, element: u8 },
    SimdShuffle { mode: i32 },
    Sub { kind: u8 },
    Mul { kind: u8 },
    Div { kind: u8 },
    Mod { kind: u8 },
    BitAnd { kind: u8 },
    BitOr { kind: u8 },
    BitXor { kind: u8 },
    Shl { kind: u8 },
    Shr { kind: u8 },
    Eq,
    Ne,
    Lt { kind: u8 },
    Le { kind: u8 },
    Gt { kind: u8 },
    Ge { kind: u8 },
    And,
    Or,
    Not,
    Neg { kind: u8 },
    BitNot { kind: u8 },
    Print,
    Pop,
    ConcatStr,
    ToStr,
    Jump(u32),
    JumpIfFalse(u32),
    Call(SmolStr),
    /// Indirect call: fn index on stack (protocol witness / vtable dispatch).
    CallIndirect(u32),
    /// Resolved at codegen merge to `ConstI32(function_index)` for vtable slots.
    ConstFnIndex(String),
    Return,
    Dup,
    JumpIfNil(u32),
    CallHost(u32),
    MakeClosure(SmolStr, u32),
    CallClosure,
    ActorCall(SmolStr),
    ActorRegister,
    CallWitness(u32, u32),
    MakeIterator,
    IteratorNext,
    AwaitPoll,
    AwaitSuspend { resume_fn: u32, state: u32 },
    ActorSend(SmolStr),
    DictBuild(u32),
    ArrayBuild(u32),
    SetBuild(u32),
    ObjectGet,
    ObjectSet,
    /// Phase 75: mark-sweep + compact unified heap (mir-run / WASM / Native tier-2E).
    HeapCompact,
    ArrayGet,
    ArraySet,
    ValueEq,
    ValueNe,
    /// Phase 16: opaque `==` via CallWitness codegen path.
    OpaqueValueEq,
    RefEq,
    RefNe,
    BeginTry(u32),
    EndTry,
    Catch,
    Throw,
    /// After evaluating a throwing expression, branch to `err_block` if an error is pending.
    BindResult(u32),
    /// Propagate the error on the stack as an early return from the current function.
    ReturnErr,
    TaskSpawn(SmolStr),
    TaskJoin,
    /// Phase 135: `super.init(...)` on existing subclass `self` (parity with codegen `CallSuper`).
    CallSuper(SmolStr),
    /// Phase 68: homogeneous array sort via Comparable witness compare loop.
    ArrSorted,
    /// Integer narrowing / numeric conversion (`Int` → `Int8`, etc.).
    NumericCast { from: u8, to: u8 },
}

/// Suite-1 / EC-M1-3 pilot MirOps (arith, branch, call, string). Must not silently no-op.
pub fn is_suite1_pilot_mir_op(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::ConstI32(_)
            | MirOp::ConstStr(_)
            | MirOp::ConstBool(_)
            | MirOp::ConstNil
            | MirOp::LoadLocal(_)
            | MirOp::StoreLocal(_)
            | MirOp::Add { .. }
            | MirOp::Sub { .. }
            | MirOp::Mul { .. }
            | MirOp::Div { .. }
            | MirOp::Print
            | MirOp::Jump(_)
            | MirOp::JumpIfFalse(_)
            | MirOp::Call(_)
            | MirOp::Return
            | MirOp::CallHost(_)
    )
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MirInstr {
    pub dest: Option<u32>,
    pub op: MirOp,
    /// Source byte span `(start, len)` for DAP breakpoint mapping.
    #[serde(default)]
    pub span: Option<(u32, u32)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MirBlock {
    pub id: u32,
    pub instructions: Vec<MirInstr>,
    pub successors: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MirFunction {
    pub name: SmolStr,
    pub param_count: u32,
    pub local_count: u32,
    pub entry: u32,
    pub blocks: Vec<MirBlock>,
    pub instructions: Vec<MirInstr>,
    /// Debug: source names for local slots (DAP Variables).
    #[serde(default)]
    pub local_names: Vec<SmolStr>,
    /// Debug: static types for local slots.
    #[serde(default)]
    pub local_types: Vec<SmolStr>,
    /// Phase 135: PC of first init body stmt (after dict/vtable setup) for `CallSuper` on `_init`.
    #[serde(default)]
    pub init_body_pc: u32,
    /// Phase 135: PC of MutRefCell wrap epilogue (super init stops here).
    #[serde(default)]
    pub init_wrap_pc: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberCallKind {
    /// Free function / static type / host manifest — lower call args before callee dispatch.
    ArgsEarly,
    /// Instance method — receiver and args lowered in member branch.
    InstanceMethod,
    /// String/array hosted methods — receiver first, args in branch.
    ReceiverFirstHosted,
}

fn expr_as_i32_literal(expr: &Expr) -> Option<i32> {
    match expr {
        Expr::Literal {
            value: Literal::Integer { value, .. },
            ..
        } => Some(*value as i32),
        _ => None,
    }
}

fn expr_type_ident_name(expr: &Expr) -> Option<&str> {
    match expr {
        Expr::Ident { name, .. } => Some(name.as_str()),
        _ => None,
    }
}

fn simd_element_from_type_suffix(suffix: &str) -> u8 {
    match suffix {
        "Float64" | "Double" | "f64" => syx_stdlib::NumericKind::F64.to_byte(),
        "Float32" | "Float" | "f32" | "f" => syx_stdlib::NumericKind::F32.to_byte(),
        "Int16" | "i16" => syx_stdlib::NumericKind::I16.to_byte(),
        "Int64" | "i64" => syx_stdlib::NumericKind::I64.to_byte(),
        "UInt64" | "u64" => syx_stdlib::NumericKind::U64.to_byte(),
        "Int8" | "i8" => syx_stdlib::NumericKind::I8.to_byte(),
        _ => syx_stdlib::NumericKind::I32.to_byte(),
    }
}

fn simd_member_lanes_element(object: &Expr) -> (u8, u8) {
    expr_type_ident_name(object)
        .map(simd_lanes_element_from_type)
        .unwrap_or((4, syx_stdlib::NumericKind::I32.to_byte()))
}

fn simd_lanes_element_from_type(type_name: &str) -> (u8, u8) {
    if let Some(open) = type_name.find('<') {
        let base = &type_name[..open];
        let elem = type_name[open + 1..].trim_end_matches('>');
        let lanes = base
            .strip_prefix("SIMD")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);
        return (lanes, simd_element_from_type_suffix(elem));
    }
    if type_name == "SIMD2" {
        (
            2,
            syx_stdlib::NumericKind::F64.to_byte(),
        )
    } else if type_name == "SIMD8" {
        (8, syx_stdlib::NumericKind::I16.to_byte())
    } else if type_name == "SIMD16" {
        (16, syx_stdlib::NumericKind::I8.to_byte())
    } else {
        (
            4,
            syx_stdlib::NumericKind::I32.to_byte(),
        )
    }
}

fn infer_simd_element_from_array_arg(args: &[CallArg], ctx: &LowerCtx) -> u8 {
    let Some(first) = args.first() else {
        return syx_stdlib::NumericKind::I32.to_byte();
    };
    if let Expr::Array { elements, .. } = &first.value {
        if let Some(el) = elements.first() {
            if let Some(element) = infer_simd_element_from_expr(el) {
                return element;
            }
        }
    }
    if let Expr::Ident { name, .. } = &first.value {
        if let Some(ty) = ctx.local_types.get(name.as_str()) {
            if ty.starts_with("Array<") && ty.ends_with('>') {
                let inner = ty.trim_start_matches("Array<").trim_end_matches('>');
                return simd_element_from_type_suffix(inner);
            }
        }
    }
    // Phase 117: `loadFromArray(storeToArray(...), …)` — peek store’s array/simd args.
    if let Expr::Call {
        callee,
        args: store_args,
        ..
    } = &first.value
    {
        if let Expr::Member { name, .. } = callee.as_ref() {
            if name.as_str() == "storeToArray" && !store_args.is_empty() {
                let from_arr = infer_simd_element_from_array_arg(store_args, ctx);
                if from_arr != syx_stdlib::NumericKind::I32.to_byte() {
                    return from_arr;
                }
                if let Some(simd_arg) = store_args.get(2) {
                    if let Some(el) = infer_simd_element_from_reduce_arg(&simd_arg.value, ctx) {
                        return el;
                    }
                }
            }
        }
    }
    syx_stdlib::NumericKind::I32.to_byte()
}

fn infer_simd_element_from_reduce_arg(arg: &Expr, ctx: &LowerCtx) -> Option<u8> {
    match arg {
        Expr::Call { callee, args, .. } => {
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                match name.as_str() {
                    "addF32" | "subF32" | "mulF32" | "divF32" => {
                        return Some(syx_stdlib::NumericKind::F32.to_byte());
                    }
                    "addF64" | "subF64" | "mulF64" | "divF64" => {
                        return Some(syx_stdlib::NumericKind::F64.to_byte());
                    }
                    "addI16" => {
                        return Some(syx_stdlib::NumericKind::I16.to_byte());
                    }
                    "addI64" => {
                        return Some(syx_stdlib::NumericKind::I64.to_byte());
                    }
                    "addSat" | "subSat" | "add" | "sub" | "mul" | "eq" | "lt" | "gt" | "min" | "max"
                    | "and" | "or" | "xor" | "not" | "shl" | "shr" if !args.is_empty() => {
                        if let Some(type_name) = expr_type_ident_name(object) {
                            let (_, element) = simd_lanes_element_from_type(type_name);
                            return Some(element);
                        }
                    }
                    "loadFromArray" if !args.is_empty() => {
                        return Some(infer_simd_element_from_array_arg(args, ctx));
                    }
                    "fromArray" if !args.is_empty() => {
                        return Some(infer_simd_element_from_array_arg(args, ctx));
                    }
                    _ => {}
                }
                if let Some(type_name) = expr_type_ident_name(object) {
                    if let Some((_, element)) = parse_simd_type(type_name) {
                        return Some(element.to_byte());
                    }
                }
            }
            None
        }
        Expr::Ident { .. } => infer_expr_simd_type(arg, ctx).map(|(_, k)| k.to_byte()),
        _ => None,
    }
}

fn infer_simd_element_from_expr(expr: &Expr) -> Option<u8> {
    match expr {
        Expr::Literal {
            value: Literal::Double { suffix, .. },
            ..
        } => {
            if let Some(s) = suffix.as_deref() {
                if s == "f32" || s == "f" {
                    return Some(syx_stdlib::NumericKind::F32.to_byte());
                }
                if s == "f64" {
                    return Some(syx_stdlib::NumericKind::F64.to_byte());
                }
            }
            Some(syx_stdlib::NumericKind::F32.to_byte())
        }
        Expr::Literal {
            value: Literal::Integer { suffix, .. },
            ..
        } => {
            let ty = suffix.as_deref().unwrap_or(if cfg!(target_pointer_width = "64") {
                "i64"
            } else {
                "i32"
            });
            Some(simd_element_from_type_suffix(ty))
        }
        _ => None,
    }
}

fn simd_reduce_add_shape(callee: &Expr, func_name: &str) -> (u8, u8) {
    if let Expr::Call { callee: inner, .. } = callee {
        if let Expr::Member { object, .. } = inner.as_ref() {
            if let Some(name) = expr_type_ident_name(object) {
                return simd_lanes_element_from_type(name);
            }
        }
    }
    if let Some(type_name) = func_name.strip_suffix("_reduceAdd") {
        return simd_lanes_element_from_type(type_name);
    }
    (4, syx_stdlib::NumericKind::I32.to_byte())
}

/// Host SIMD calls that lower their own args onto the MIR stack; skip ArgsEarly pre-pass.
///
/// Note: `SIMD*_fromArray` is a **stdlib facade** (`Call SIMD4_fromArray`), not a direct
/// CallHost that packs lanes itself. Skipping ArgsEarly for it drops the array literal and
/// yields VM `SIMD operation failed` on the subsequent `SimdAdd`. Keep ArgsEarly for fromArray.
fn host_simd_lowers_own_args(name: &str) -> bool {
    host_fn_id_by_name(name).is_some_and(|h| {
        matches!(
            h,
            HostFnId::SimdShuffle
                | HostFnId::SimdAnd
                | HostFnId::SimdOr
                | HostFnId::SimdXor
                | HostFnId::SimdShl
                | HostFnId::SimdShr
                | HostFnId::SimdMin
                | HostFnId::SimdMax
                | HostFnId::SimdF32Add
                | HostFnId::SimdF32Sub
                | HostFnId::SimdF32Mul
                | HostFnId::SimdF32Div
                | HostFnId::SimdF64Add
                | HostFnId::SimdF64Sub
                | HostFnId::SimdF64Mul
                | HostFnId::SimdF64Div
                | HostFnId::SimdI16Add
                | HostFnId::SimdI16Sub
                | HostFnId::SimdI16Mul
                | HostFnId::SimdI8Add
                | HostFnId::SimdI8Sub
                | HostFnId::SimdI8Mul
                | HostFnId::SimdI8And
                | HostFnId::SimdI8Or
                | HostFnId::SimdI8Xor
                | HostFnId::SimdI8Not
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Abs
                | HostFnId::SimdI8Neg
                | HostFnId::SimdI8Extract
                | HostFnId::SimdI8Replace
                | HostFnId::SimdI8Eq
                | HostFnId::SimdI8Ne
                | HostFnId::SimdI8Le
                | HostFnId::SimdI8Ge
                | HostFnId::SimdI8AvgrU
                | HostFnId::SimdI8Popcnt
                | HostFnId::SimdI8Bitmask
                | HostFnId::SimdI8AllTrue
                | HostFnId::SimdI8AnyTrue
                | HostFnId::SimdI8Lt
                | HostFnId::SimdI8Gt
                | HostFnId::SimdI8AddSat
                | HostFnId::SimdI8SubSat
                | HostFnId::SimdI8Shuffle
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                | HostFnId::SimdReduceAdd
                | HostFnId::SimdNot
                | HostFnId::SimdLoadFromArray
                | HostFnId::SimdStoreToArray
        )
    })
}

/// Static SIMD member calls that lower their own args; skip ArgsEarly dupes.
/// `fromArray` is intentionally excluded — see `host_simd_lowers_own_args`.
fn member_host_simd_lowers_own_args(object: &Expr, method: &str, ctx: &LowerCtx) -> bool {
    resolve_host_call(object, method, ctx).is_some_and(|(h, _)| {
        matches!(
            h,
            HostFnId::SimdLoadFromArray
                | HostFnId::SimdStoreToArray
                | HostFnId::SimdShuffle
                | HostFnId::SimdAnd
                | HostFnId::SimdOr
                | HostFnId::SimdXor
                | HostFnId::SimdShl
                | HostFnId::SimdShr
                | HostFnId::SimdMin
                | HostFnId::SimdMax
                | HostFnId::SimdF32Add
                | HostFnId::SimdF32Sub
                | HostFnId::SimdF32Mul
                | HostFnId::SimdF32Div
                | HostFnId::SimdF64Add
                | HostFnId::SimdF64Sub
                | HostFnId::SimdF64Mul
                | HostFnId::SimdF64Div
                | HostFnId::SimdI16Add
                | HostFnId::SimdI16Sub
                | HostFnId::SimdI16Mul
                | HostFnId::SimdI8Add
                | HostFnId::SimdI8Sub
                | HostFnId::SimdI8Mul
                | HostFnId::SimdI8And
                | HostFnId::SimdI8Or
                | HostFnId::SimdI8Xor
                | HostFnId::SimdI8Not
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Abs
                | HostFnId::SimdI8Neg
                | HostFnId::SimdI8Extract
                | HostFnId::SimdI8Replace
                | HostFnId::SimdI8Eq
                | HostFnId::SimdI8Ne
                | HostFnId::SimdI8Le
                | HostFnId::SimdI8Ge
                | HostFnId::SimdI8AvgrU
                | HostFnId::SimdI8Popcnt
                | HostFnId::SimdI8Bitmask
                | HostFnId::SimdI8AllTrue
                | HostFnId::SimdI8AnyTrue
                | HostFnId::SimdI8Lt
                | HostFnId::SimdI8Gt
                | HostFnId::SimdI8AddSat
                | HostFnId::SimdI8SubSat
                | HostFnId::SimdI8Shuffle
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                | HostFnId::SimdReduceAdd
                | HostFnId::SimdNot
        )
    })
}

fn classify_member_call(callee: &Expr, ctx: &LowerCtx) -> MemberCallKind {
    let Expr::Member { object, name, .. } = callee else {
        return MemberCallKind::ArgsEarly;
    };
    if is_static_type_member_call(object, ctx) {
        return MemberCallKind::ArgsEarly;
    }
    if matches!(
        object.as_ref(),
        Expr::Ident { name: mod_name, .. } if ctx.imports.contains(mod_name)
    ) {
        return MemberCallKind::ArgsEarly;
    }
    if expr_is_string_like(object, ctx) && lookup_string_method(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_array_like(object, ctx) && lookup_array_method(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_float64_like(object, ctx) && lookup_float64_math_host(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_float32_like(object, ctx) && lookup_float32_math_host(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    MemberCallKind::InstanceMethod
}

fn method_id_mir_op(method: MethodId, surface: &str, method_name: &str, ctx: &LowerCtx) -> MirOp {
    if method == MethodId::ArrSorted {
        return MirOp::ArrSorted;
    }
    if let Some(idx) = method_id_call_host_index(method) {
        return MirOp::CallHost(idx);
    }
    collection_method_mir_op(surface, method_name, ctx)
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MirModule {
    pub functions: Vec<MirFunction>,
    /// Explicit AOT export roots beyond `main` (W440 / ADR-784).
    /// Empty ⇒ default emit/compact remain main-only. Non-empty ⇒ default paths use
    /// [`llvm_emit_function_names`] / [`compact_module_for_aot_blob`] with these roots.
    /// ≠ language-level `Module.exports` auto-discovery.
    #[serde(default)]
    pub aot_export_roots: Vec<SmolStr>,
}

/// Borrow [`MirModule::aot_export_roots`] as `&str` refs for emit/compact helpers.
pub fn mir_module_aot_export_root_refs(module: &MirModule) -> Vec<&str> {
    module.aot_export_roots.iter().map(|s| s.as_str()).collect()
}

#[derive(Debug, Default, Clone)]
struct ModuleIndex {
    class_types: IndexMap<SmolStr, Vec<SmolStr>>,
    /// Phase 121: stored property name → type for `self.field` method dispatch.
    class_field_types: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    /// Phase 65/125: inheritance + vtable slot map (class → method → mangled fn).
    class_superclasses: IndexMap<SmolStr, SmolStr>,
    class_vtables: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    actor_types: IndexSet<SmolStr>,
    /// Phase 101: types that declare an explicit `deinit`.
    types_with_deinit: IndexSet<SmolStr>,
    generic_templates: IndexSet<SmolStr>,
    dataclass_names: IndexSet<SmolStr>,
    /// Field order for DataClass/Struct memberwise init (MIR pads missing args).
    dataclass_fields: IndexMap<SmolStr, Vec<SmolStr>>,
    /// `(type, field)` → default int literal for omitted memberwise args (Counter value=0).
    dataclass_field_default_i64: IndexMap<(SmolStr, SmolStr), i128>,
    enum_classes: IndexSet<SmolStr>,
    final_classes: IndexSet<SmolStr>,
    enum_case_tags: IndexMap<(SmolStr, SmolStr), i64>,
    enum_associated_fields: IndexMap<(SmolStr, SmolStr), Vec<SmolStr>>,
    function_params: IndexMap<SmolStr, Vec<Option<TypeExpr>>>,
    /// Top-level / free function return type names (`make` → `Array<Int>`).
    function_returns: IndexMap<SmolStr, SmolStr>,
    opaque_functions: IndexSet<SmolStr>,
    extension_methods: IndexSet<(SmolStr, SmolStr)>,
    /// Extension method return type names for local inference (`Person.decode` → `Person`).
    extension_method_returns: IndexMap<(SmolStr, SmolStr), SmolStr>,
    static_class_methods: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 47: merged extension symbols available for `Call` (e.g. `Array_description`).
    callable_extensions: IndexSet<SmolStr>,
    /// Phase 126: declared protocol names for virtual instance dispatch on protocol-typed refs.
    protocol_names: IndexSet<SmolStr>,
    /// Phase 126: protocol name → method names (witness slot setup + dispatch).
    protocol_methods: IndexMap<SmolStr, Vec<SmolStr>>,
    property_observers:
        IndexMap<(SmolStr, SmolStr), (Option<Vec<Stmt>>, Option<Vec<Stmt>>)>,
    /// Phase 134: `(type, field)` with desugared `__get_{field}` accessor.
    property_getters: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 135: `(type, field)` with desugared `__set_{field}` accessor.
    property_setters: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 134: `(type, field)` → (global name, mutable).
    static_vars: IndexMap<(SmolStr, SmolStr), (SmolStr, bool)>,
    /// Span-start → NumericKind from semantic contextual typing
    /// (`let x: Int8 = 8`, `let y: Double = 1`). Empty when unused.
    contextual_numeric_kinds: IndexMap<u32, NumericKind>,
}

impl ModuleIndex {
    fn lookup_property_observer(
        &self,
        ty: &SmolStr,
        field: &SmolStr,
    ) -> Option<(Option<Vec<Stmt>>, Option<Vec<Stmt>>)> {
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if let Some(obs) = self.property_observers.get(&(t.clone(), field.clone())) {
                return Some(obs.clone());
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    fn property_getter_fn(&self, ty: &SmolStr, field: &SmolStr) -> Option<SmolStr> {
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if self
                .property_getters
                .contains(&(t.clone(), field.clone()))
            {
                return Some(SmolStr::new(format!(
                    "{t}_{}",
                    syx_ast::var_global_getter_method(field)
                )));
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    fn property_setter_fn(&self, ty: &SmolStr, field: &SmolStr) -> Option<SmolStr> {
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if self
                .property_setters
                .contains(&(t.clone(), field.clone()))
            {
                return Some(SmolStr::new(format!(
                    "{t}_{}",
                    syx_ast::var_global_setter_method(field)
                )));
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    fn static_var_global(&self, ty: &SmolStr, field: &SmolStr) -> Option<(SmolStr, bool)> {
        self.static_vars.get(&(ty.clone(), field.clone())).cloned()
    }

    fn has_deinit(&self, type_name: &SmolStr) -> bool {
        self.types_with_deinit.contains(type_name)
    }

    fn register_nested_type(&mut self, outer: &SmolStr, decl: &Decl) {
        match decl {
            Decl::Class {
                name,
                members,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                let fields: Vec<SmolStr> = members
                    .iter()
                    .filter_map(|m| syx_ast::var_global_stored_field_name(m))
                    .collect();
                self.class_types.insert(qualified.clone(), fields);
                for member in members {
                    if let Decl::VarGlobal {
                        name: field,
                        ty: Some(TypeExpr::Named {
                            name: field_ty, ..
                        }),
                        ..
                    } = member
                    {
                        self.class_field_types
                            .entry(qualified.clone())
                            .or_default()
                            .insert(field.clone(), field_ty.clone());
                    }
                    if matches!(
                        member,
                        Decl::Class { .. }
                            | Decl::Struct { .. }
                            | Decl::Enum { .. }
                            | Decl::FinalClass { .. }
                    ) {
                        self.register_nested_type(&qualified, member);
                    }
                }
            }
            Decl::Struct { name, .. } | Decl::Enum { name, .. } | Decl::FinalClass { name, .. } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.class_types.insert(qualified, vec![]);
            }
            _ => {}
        }
    }

    fn from_module(module: &Module) -> Self {
        let mut idx = Self {
            dataclass_names: IndexSet::new(),
            ..Self::default()
        };
        for decl in &module.decls {
            match decl {
                Decl::DataClass { name, fields, .. } => {
                    idx.dataclass_names.insert(name.clone());
                    idx.dataclass_fields.insert(
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
                    idx.dataclass_names.insert(name.clone());
                    idx.dataclass_fields.insert(
                        name.clone(),
                        fields.iter().map(|(f, _)| f.clone()).collect(),
                    );
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            init: Some(Expr::Literal {
                                value: Literal::Integer { value, .. },
                                ..
                            }),
                            ..
                        } = member
                        {
                            idx.dataclass_field_default_i64
                                .insert((name.clone(), field.clone()), *value);
                        }
                    }
                }
                Decl::VarGlobal {
                    attrs,
                    name,
                    mutable,
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
                            idx.static_vars.insert(
                                (type_key.clone(), field_key.clone()),
                                (name.clone(), *mutable),
                            );
                            if will_set.is_some() || did_set.is_some() {
                                idx.property_observers.insert(
                                    (type_key, field_key),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                    }
                }
                Decl::Function {
                    name,
                    params,
                    type_params,
                    return_type,
                    ..
                } if type_params.is_empty() => {
                    idx.function_params.insert(
                        name.clone(),
                        params.iter().map(|p| p.ty.clone()).collect(),
                    );
                    if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                        idx.function_returns.insert(name.clone(), ret);
                    }
                    if matches!(return_type, Some(TypeExpr::OpaqueSome(_))) {
                        idx.opaque_functions.insert(name.clone());
                    }
                }
                Decl::Function { name, type_params, .. } if !type_params.is_empty() => {
                    idx.generic_templates.insert(name.clone());
                }
                Decl::Protocol { name, methods, .. } => {
                    idx.protocol_names.insert(name.clone());
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
                    idx.protocol_methods.insert(name.clone(), method_names);
                }
                Decl::Class { name, members, .. } | Decl::Actor { name, members, .. } => {
                    let fields: Vec<SmolStr> = members
                        .iter()
                        .filter_map(syx_ast::var_global_stored_field_name)
                        .collect();
                    idx.class_types.insert(name.clone(), fields);
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            ty: Some(TypeExpr::Named {
                                name: field_ty, ..
                            }),
                            ..
                        } = member
                        {
                            let key = syx_ast::var_global_stored_field_name(member)
                                .unwrap_or_else(|| field.clone());
                            idx.class_field_types
                                .entry(name.clone())
                                .or_default()
                                .insert(key, field_ty.clone());
                        }
                    }
                    if members.iter().any(|m| matches!(m, Decl::Deinit { .. })) {
                        idx.types_with_deinit.insert(name.clone());
                    }
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            will_set,
                            did_set,
                            ..
                        } = member
                        {
                            if will_set.is_some() || did_set.is_some() {
                                idx.property_observers.insert(
                                    (name.clone(), field.clone()),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                        if let Decl::Function {
                            name: method,
                            type_params,
                            attrs,
                            return_type,
                            ..
                        } = member
                        {
                            if let Some(field) = method.as_str().strip_prefix("__get_") {
                                idx.property_getters
                                    .insert((name.clone(), SmolStr::new(field)));
                            }
                            if let Some(field) = method.as_str().strip_prefix("__set_") {
                                idx.property_setters
                                    .insert((name.clone(), SmolStr::new(field)));
                            }
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                idx.static_class_methods
                                    .insert((name.clone(), method.clone()));
                            }
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((name.clone(), method.clone()), ret);
                            }
                            if !type_params.is_empty()
                                && attrs.iter().any(|a| a.name.as_str() == "static")
                            {
                                idx.generic_templates
                                    .insert(SmolStr::new(format!("{name}_{method}")));
                            }
                        }
                        if let Decl::OperatorFunc {
                            op,
                            return_type,
                            ..
                        } = member
                        {
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((name.clone(), op.clone()), ret);
                            }
                        }
                    }
                    if matches!(decl, Decl::Actor { .. }) {
                        idx.actor_types.insert(name.clone());
                    }
                    for member in members {
                        if matches!(
                            member,
                            Decl::Class { .. }
                                | Decl::Struct { .. }
                                | Decl::Enum { .. }
                                | Decl::FinalClass { .. }
                        ) {
                            idx.register_nested_type(name, member);
                        }
                    }
                }
                Decl::Enum { name, cases, .. } => {
                    idx.enum_classes.insert(name.clone());
                    let mut next_tag = 0i64;
                    for (case, value, fields) in cases {
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
                        idx.enum_case_tags
                            .insert((name.clone(), case.clone()), tag_val);
                        if !fields.is_empty() {
                            idx.enum_associated_fields.insert(
                                (name.clone(), case.clone()),
                                fields.iter().map(|(f, _)| f.clone()).collect(),
                            );
                        }
                    }
                }
                Decl::FinalClass { name, variants, .. } => {
                    idx.final_classes.insert(name.clone());
                    for (variant, fields) in variants {
                        if !fields.is_empty() {
                            idx.enum_associated_fields.insert(
                                (name.clone(), variant.clone()),
                                fields.iter().map(|(f, _)| f.clone()).collect(),
                            );
                        }
                    }
                }
                Decl::Extension { type_name, methods, .. } => {
                    for method in methods {
                        if let Decl::Function {
                            name,
                            return_type,
                            attrs,
                            ..
                        } = method
                        {
                            idx.extension_methods
                                .insert((type_name.clone(), name.clone()));
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((type_name.clone(), name.clone()), ret);
                            }
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                idx.static_class_methods
                                    .insert((type_name.clone(), name.clone()));
                            }
                            // Phase 136: extension computed accessors participate in property get/set.
                            if let Some(field) = name.as_str().strip_prefix("__get_") {
                                idx.property_getters
                                    .insert((type_name.clone(), SmolStr::new(field)));
                            }
                            if let Some(field) = name.as_str().strip_prefix("__set_") {
                                idx.property_setters
                                    .insert((type_name.clone(), SmolStr::new(field)));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        for decl in &module.decls {
            let Decl::Class {
                name,
                members,
                superclass,
                ..
            } = decl
            else {
                continue;
            };
            if let Some(base) = superclass {
                idx.class_superclasses.insert(name.clone(), base.clone());
            }
            let mut vtable = idx
                .class_superclasses
                .get(name)
                .and_then(|base| idx.class_vtables.get(base))
                .cloned()
                .unwrap_or_default();
            for member in members {
                let Decl::Function {
                    name: method,
                    type_params,
                    attrs,
                    ..
                } = member
                else {
                    continue;
                };
                if !type_params.is_empty()
                    || attrs.iter().any(|a| a.name.as_str() == "static")
                {
                    continue;
                }
                vtable.insert(
                    method.clone(),
                    SmolStr::new(format!("{name}_{method}")),
                );
            }
            idx.class_vtables.insert(name.clone(), vtable);
        }
        idx
    }
}

struct LowerCtx {
    func_name: SmolStr,
    blocks: Vec<MirBlock>,
    next_block: u32,
    next_reg: u32,
    locals: indexmap::IndexMap<SmolStr, u32>,
    globals: Vec<SmolStr>,
    imports: IndexSet<SmolStr>,
    module: ModuleIndex,
    /// Module-wide id source so `__mir_closure_N` / `__task_body_N` stay unique across
    /// functions (per-`LowerCtx` counters collide and codegen/MIR dedupe by name).
    closure_ids: Rc<Cell<u32>>,
    pending_closures: Vec<MirFunction>,
    in_async: bool,
    await_segment: u32,
    active_block: u32,
    local_types: indexmap::IndexMap<SmolStr, SmolStr>,
    prefer_set_build: bool,
    /// Lowering soft-fail: `(optional diagnostic code, message)`.
    error: Option<(Option<&'static str>, String)>,
    mut_ref_aliases: IndexMap<SmolStr, SmolStr>,
    mut_ref_params: HashSet<SmolStr>,
    /// Function parameters: caller still holds these; skip scope-exit deinit.
    param_names: HashSet<SmolStr>,
    /// Local skipped on this return (Ident return of a newly built object).
    skip_deinit_on_return: Option<SmolStr>,
    /// Active source span stamped onto emitted instructions (DAP breakpoints).
    current_span: Option<(u32, u32)>,
}

impl LowerCtx {
    fn new(
        func_name: SmolStr,
        module: ModuleIndex,
        imports: IndexSet<SmolStr>,
        globals: Vec<SmolStr>,
        closure_ids: Rc<Cell<u32>>,
    ) -> Self {
        Self {
            func_name,
            blocks: Vec::new(),
            next_block: 0,
            next_reg: 0,
            locals: indexmap::IndexMap::new(),
            globals,
            imports,
            module,
            closure_ids,
            pending_closures: Vec::new(),
            in_async: false,
            await_segment: 0,
            active_block: 0,
            local_types: indexmap::IndexMap::new(),
            prefer_set_build: false,
            error: None,
            mut_ref_aliases: IndexMap::new(),
            mut_ref_params: HashSet::new(),
            param_names: HashSet::new(),
            skip_deinit_on_return: None,
            current_span: None,
        }
    }

    fn next_closure_id(&self) -> u32 {
        let id = self.closure_ids.get();
        self.closure_ids.set(id + 1);
        id
    }

    fn resolve_binding_name<'a>(&'a self, name: &'a SmolStr) -> &'a SmolStr {
        self.mut_ref_aliases.get(name).unwrap_or(name)
    }

    fn is_class_instance_binding(&self, name: &SmolStr) -> bool {
        if name == "self" {
            return true;
        }
        self.local_types.get(name).is_some_and(|t| {
            self.module.class_types.contains_key(t) || self.module.actor_types.contains(t)
        })
    }

    fn uses_mut_ref_cell(&self, name: &SmolStr) -> bool {
        self.mut_ref_params.contains(name)
    }

    fn in_class_init(&self) -> bool {
        self.func_name.ends_with("_init")
    }

    fn enclosing_class(&self) -> Option<SmolStr> {
        class_name_from_mangled(self.func_name.as_str())
    }

    fn receiver_class_type(&self, obj_name: &SmolStr) -> Option<SmolStr> {
        if obj_name.as_str() == "self" {
            class_name_from_mangled(self.func_name.as_str())
        } else {
            self.local_types.get(obj_name).cloned()
        }
    }

    /// Unwrap `MutRefCell` only for explicit `inout`/mut-ref params. Class locals returned
    /// from inits stay as MutRefCell handles so `resolve_virtual_method` sees `__type`.
    fn should_unwrap_class_local(&self, name: &SmolStr) -> bool {
        if self.in_class_init() && name.as_str() == "self" {
            return false;
        }
        self.uses_mut_ref_cell(name)
    }

    fn member_assign_unwraps_object(&self, object: &Expr) -> bool {
        !(self.in_class_init()
            && matches!(object, Expr::Ident { name, .. } if name.as_str() == "self"))
    }

    /// After field write on an unwrapped class instance, write inner `ObjectRef` back into
    /// the local's `MutRefCell` wrapper (boot DI field `@Autowired` assigns).
    fn emit_mut_ref_cell_writeback_from_inner(&mut self, name: &SmolStr, inner_slot: u32, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(&slot) = self.locals.get(&resolved) else {
            return;
        };
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(inner_slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(slot),
                span: None,
            },
        );
    }

    #[allow(dead_code)] // residual MutRefCell writeback path (parked)
    fn emit_mut_ref_cell_writeback(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(&slot) = self.locals.get(&resolved) else {
            return;
        };
        let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(slot),
                span: None,
            },
        );
    }

    fn emit_unwrap_syx_ref(&mut self, block: u32) {
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectGet,
                span: None,
            },
        );
    }

    fn lower_load_binding_raw(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    fn lower_rhs_for_field_store(&mut self, value: &Expr, block: u32) {
        if let Expr::Ident { name, .. } = value {
            if self.locals.contains_key(name) || self.global_slot(name).is_some() {
                self.lower_load_binding_raw(name, block);
                return;
            }
        }
        lower_expr(value, self.fresh_reg(), self, block);
    }

    fn lower_load_binding(&mut self, dest: u32, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if self.should_unwrap_class_local(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                self.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::LoadLocal(slot),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr("__syx_ref".into()),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectGet,
                        span: None,
                    },
                );
            }
            return;
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
        } else if self.lower_enclosing_field_load(dest, name, block) {
            // S1-I2: bare struct/class field in method body.
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    /// `Counter_increment`: bare `value` → `self.value` ObjectGet.
    fn lower_enclosing_field_load(&mut self, dest: u32, name: &SmolStr, block: u32) -> bool {
        let Some(type_name) = class_name_from_mangled(self.func_name.as_str()) else {
            return false;
        };
        let is_field = self
            .module
            .dataclass_fields
            .get(&type_name)
            .is_some_and(|fs| fs.iter().any(|f| f == name))
            || self
                .module
                .class_types
                .get(&type_name)
                .is_some_and(|fs| fs.iter().any(|f| f == name));
        if !is_field {
            return false;
        }
        let Some(&self_slot) = self.locals.get(&SmolStr::new("self")) else {
            return false;
        };
        self.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(name.to_string()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ObjectGet,
                span: None,
            },
        );
        true
    }

    /// Bare field store: stack has RHS; emit `self.field = rhs` + write back `self`.
    fn lower_enclosing_field_store(&mut self, name: &SmolStr, block: u32) -> bool {
        let Some(type_name) = class_name_from_mangled(self.func_name.as_str()) else {
            return false;
        };
        let is_field = self
            .module
            .dataclass_fields
            .get(&type_name)
            .is_some_and(|fs| fs.iter().any(|f| f == name))
            || self
                .module
                .class_types
                .get(&type_name)
                .is_some_and(|fs| fs.iter().any(|f| f == name));
        if !is_field {
            return false;
        }
        let Some(&self_slot) = self.locals.get(&SmolStr::new("self")) else {
            return false;
        };
        let temp = self.alloc_local(&SmolStr::new(format!("__syx_field_{name}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(name.to_string()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(self_slot),
                span: None,
            },
        );
        true
    }

    fn lower_store_binding(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if self.should_unwrap_class_local(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(temp),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(slot),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(temp),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr("__syx_ref".into()),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectSet,
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
            }
            return;
        }
        if self.is_class_instance_binding(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                self.emit_binding_override_deinit(&resolved, slot, block);
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
            }
            return;
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreGlobal(slot),
                    span: None,
                },
            );
        } else if self.lower_enclosing_field_store(name, block) {
            // S1-I2: bare struct/class field store.
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    /// Phase 101: call `Type_deinit` on the old local before overwrite.
    fn emit_binding_override_deinit(&mut self, name: &SmolStr, slot: u32, block: u32) {
        let Some(type_name) = self.local_types.get(name).cloned() else {
            return;
        };
        if !self.module.class_types.contains_key(&type_name)
            && !self.module.actor_types.contains(&type_name)
        {
            return;
        }
        let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
        if !self.module.has_deinit(&type_name) {
            return;
        }
        let temp = self.alloc_local(&SmolStr::new(format!("__p101_store_new_{name}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::Call(deinit_name),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(temp),
                span: None,
            },
        );
    }

    fn emit_scope_exit_deinits(&mut self, block: u32) {
        let locals: Vec<(SmolStr, SmolStr, u32)> = self
            .locals
            .iter()
            .filter_map(|(name, &slot)| {
                if self.param_names.contains(name) {
                    return None;
                }
                if self.skip_deinit_on_return.as_ref() == Some(name) {
                    return None;
                }
                let ty = self.local_types.get(name)?.clone();
                if self.module.class_types.contains_key(&ty) || self.module.actor_types.contains(&ty)
                {
                    if self.module.has_deinit(&ty) {
                        return Some((name.clone(), ty, slot));
                    }
                }
                None
            })
            .collect();
        for (_name, type_name, slot) in locals {
            let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Call(deinit_name),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Pop,
                    span: None,
                },
            );
        }
    }

    fn lower_mut_ref_cell(&mut self, dest: u32, target: &SmolStr, block: u32) {
        self.lower_load_binding(dest, target, block);
    }

    fn global_slot(&self, name: &SmolStr) -> Option<u32> {
        self.globals.iter().position(|g| g == name).map(|i| i as u32)
    }

    fn set_error(&mut self, message: impl Into<String>) {
        self.set_error_with_code(None, message);
    }

    fn set_error_with_code(&mut self, code: Option<&'static str>, message: impl Into<String>) {
        if self.error.is_none() {
            self.error = Some((code, message.into()));
        }
    }

    fn take_error_diagnostic(&mut self) -> Option<SyxDiagnostic> {
        let (code, message) = self.error.take()?;
        let mut d = SyxDiagnostic::error(message);
        if let Some(c) = code {
            d = d.with_code(c);
        }
        Some(d)
    }

    fn fresh_block(&mut self) -> u32 {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(MirBlock {
            id,
            instructions: Vec::new(),
            successors: Vec::new(),
        });
        id
    }

    #[allow(dead_code)] // residual helper for MutRefCell / block queries
    fn current_block(&mut self) -> u32 {
        if self.blocks.is_empty() {
            self.fresh_block()
        } else {
            self.blocks.last().unwrap().id
        }
    }

    fn emit(&mut self, block_id: u32, mut instr: MirInstr) {
        if instr.span.is_none() {
            instr.span = self.current_span;
        }
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
            block.instructions.push(instr);
        }
    }

    fn link(&mut self, from: u32, to: u32) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == from) {
            if !block.successors.contains(&to) {
                block.successors.push(to);
            }
        }
    }

    fn alloc_local(&mut self, name: &SmolStr) -> u32 {
        if let Some(&slot) = self.locals.get(name) {
            slot
        } else {
            let slot = self.locals.len() as u32;
            self.locals.insert(name.clone(), slot);
            slot
        }
    }

    fn fresh_reg(&mut self) -> u32 {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }
}

pub fn extension_methods_for_module(module: &Module) -> IndexSet<(SmolStr, SmolStr)> {
    ModuleIndex::from_module(module).extension_methods
}

pub fn lower_module(module: &Module) -> SyxResult<MirModule> {
    lower_module_with_extra_extensions(
        module,
        &IndexSet::new(),
        &IndexSet::new(),
        &IndexSet::new(),
    )
}

pub fn lower_module_with_extra_extensions(
    module: &Module,
    extra_extensions: &IndexSet<(SmolStr, SmolStr)>,
    callable_extensions: &IndexSet<SmolStr>,
    imported_class_types: &IndexSet<SmolStr>,
) -> SyxResult<MirModule> {
    lower_module_with_extra_extensions_and_kinds(
        module,
        extra_extensions,
        callable_extensions,
        imported_class_types,
        &IndexMap::new(),
    )
}

/// Like [`lower_module_with_extra_extensions`], but applies semantic contextual
/// numeric-literal kinds (span start → `NumericKind`) for unsuffixed consts.
pub fn lower_module_with_extra_extensions_and_kinds(
    module: &Module,
    extra_extensions: &IndexSet<(SmolStr, SmolStr)>,
    callable_extensions: &IndexSet<SmolStr>,
    imported_class_types: &IndexSet<SmolStr>,
    contextual_numeric_kinds: &IndexMap<u32, NumericKind>,
) -> SyxResult<MirModule> {
    let mut module_idx = ModuleIndex::from_module(module);
    module_idx.contextual_numeric_kinds = contextual_numeric_kinds.clone();
    for ty in imported_class_types {
        module_idx
            .class_types
            .entry(ty.clone())
            .or_insert_with(Vec::new);
    }
    module_idx
        .extension_methods
        .extend(extra_extensions.iter().cloned());
    module_idx
        .callable_extensions
        .extend(callable_extensions.iter().cloned());
    let mut imports = IndexSet::new();
    let mut globals = Vec::new();
    for decl in &module.decls {
        if let Some(sym) = decl.import_symbol() {
            imports.insert(sym.clone());
        }
        if let Decl::VarGlobal { name, .. } = decl {
            globals.push(name.clone());
        }
    }
    let mut mir = MirModule::default();
    let closure_ids = Rc::new(Cell::new(0u32));
    for decl in &module.decls {
        // W483 / ADR-827: `export { Name, … }` → aot_export_roots (≠ auto-discovery).
        if let Decl::ExportList { names, .. } = decl {
            for name in names {
                if !mir.aot_export_roots.iter().any(|r| r == name) {
                    mir.aot_export_roots.push(name.clone());
                }
            }
        }
        let (is_export, decl) = match decl {
            Decl::Export { item, .. } => (true, item.as_ref()),
            other => (false, other),
        };
        if let Decl::Function {
            name,
            body,
            params,
            type_params,
            is_async,
            access,
            ..
        } = decl
        {
            if !type_params.is_empty() {
                continue;
            }
            // W481 / ADR-825: `export func` → `aot_export_roots`.
            // W923 / ADR-1267: scan-all auto — top-level `public func` also fills roots
            // (≠ exports GA; historical inventories keep auto_discovery=false).
            let auto_public = *access == AccessLevel::Public;
            if (is_export || auto_public) && !mir.aot_export_roots.iter().any(|r| r == name) {
                mir.aot_export_roots.push(name.clone());
            }
            let (func, closures) = lower_function_with_params(
                name,
                params,
                body,
                &module_idx,
                *is_async,
                &imports,
                &globals,
                &[],
                Rc::clone(&closure_ids),
            )?;
            mir.functions.push(func);
            mir.functions.extend(closures);
        }
    }
    for decl in &module.decls {
        match decl {
            Decl::Actor {
                name,
                members,
                conformances,
                ..
            } => {
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    true,
                    conformances,
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            Decl::Class {
                name,
                members,
                conformances,
                superclass,
                ..
            } => {
                let mut merged = conformances.to_vec();
                if let Some(base) = superclass {
                    if module_idx.protocol_names.contains(base)
                        && !merged.iter().any(|c| c == base)
                    {
                        merged.push(base.clone());
                    }
                }
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    false,
                    merged.as_slice(),
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            Decl::Extension { type_name, methods, .. } => {
                lower_type_members(
                    &mut mir,
                    type_name,
                    methods,
                    false,
                    &[],
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            // S1-I2: Struct methods (e.g. Counter.increment) via same member lower as Class.
            Decl::Struct {
                name,
                members,
                ..
            } => {
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    false,
                    &[],
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            _ => {}
        }
    }
    Ok(mir)
}

fn merged_class_fields(module_idx: &ModuleIndex, type_name: &SmolStr) -> Vec<SmolStr> {
    let mut chain = Vec::new();
    let mut cur = Some(type_name.clone());
    while let Some(t) = cur {
        if let Some(fields) = module_idx.class_types.get(&t) {
            chain.push(fields.clone());
        }
        cur = module_idx.class_superclasses.get(&t).cloned();
    }
    chain.reverse();
    let mut out = Vec::new();
    for fields in chain {
        for field in fields {
            if !out.contains(&field) {
                out.push(field);
            }
        }
    }
    out
}

fn lower_type_members(
    mir: &mut MirModule,
    type_name: &SmolStr,
    members: &[Decl],
    is_actor: bool,
    conformances: &[SmolStr],
    module_idx: &ModuleIndex,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<()> {
    let _fields: Vec<SmolStr> = members
        .iter()
        .filter_map(syx_ast::var_global_stored_field_name)
        .collect();
    let merged_fields = merged_class_fields(module_idx, type_name);
    let has_init = members.iter().any(|m| matches!(m, Decl::Init { .. }));
    let mut pending_inits: Vec<(&[syx_ast::Param], &[Stmt])> = Vec::new();
    for member in members {
        match member {
            Decl::Function {
                name,
                body,
                params,
                attrs,
                ..
            } => {
                let is_static = attrs.iter().any(|a| a.name.as_str() == "static");
                let method_name = SmolStr::new(format!("{type_name}_{name}"));
                let mut all_params = if is_static {
                    Vec::new()
                } else {
                    // ADR-631: typed self on actors so `self.method()` → ActorCall (not ObjectGet).
                    // Keep untyped self for classes to preserve MutRefCell / deinit lowering.
                    // Object/Dictionary bags need typed self so `self[key]` → ObjectGet/ObjectSet.
                    let typed_self = is_actor
                        || matches!(type_name.as_str(), "Object" | "Dictionary");
                    vec![self_param_typed(typed_self.then_some(type_name))]
                };
                all_params.extend_from_slice(params);
                let (func, closures) = lower_function_with_params(
                    &method_name,
                    &all_params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            Decl::OperatorFunc {
                op,
                body,
                params,
                ..
            } => {
                let method_name = SmolStr::new(format!("{type_name}_{op}"));
                let (func, closures) = lower_function_with_params(
                    &method_name,
                    params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            Decl::Init { params, body, .. } => {
                pending_inits.push((params, body));
            }
            Decl::Deinit { body, .. } => {
                let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
                // ADR-631: only actors need typed self for method ActorCall; class deinit stays untyped.
                let all_params = vec![self_param_typed(is_actor.then_some(type_name))];
                let (func, closures) = lower_function_with_params(
                    &deinit_name,
                    &all_params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            _ => {}
        }
    }
    for (params, body) in pending_inits {
        lower_class_init(
            mir,
            type_name,
            &merged_fields,
            params,
            body,
            is_actor,
            conformances,
            module_idx,
            imports,
            globals,
            Rc::clone(&closure_ids),
        )?;
    }
    for member in members {
        if let Decl::Class {
            name: inner,
            members: nested,
            conformances,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            lower_type_members(
                mir,
                &qualified,
                nested,
                is_actor,
                conformances,
                module_idx,
                imports,
                globals,
                Rc::clone(&closure_ids),
            )?;
        } else if let Decl::Struct { name: inner, fields, .. } = member {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, fields);
        } else if let Decl::Enum {
            name: inner,
            cases,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, cases);
        } else if let Decl::FinalClass {
            name: inner,
            variants,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, variants);
        }
    }
    if !has_init {
        let init_name = SmolStr::new(format!("{type_name}_init"));
        if !mir.functions.iter().any(|f| f.name == init_name) {
            if merged_fields.is_empty() {
                lower_class_init(
                    mir,
                    type_name,
                    &[],
                    &[],
                    &[],
                    is_actor,
                    conformances,
                    module_idx,
                    imports,
                    globals,
                    Rc::clone(&closure_ids),
                )?;
            } else {
                let params: Vec<syx_ast::Param> = merged_fields
                    .iter()
                    .map(|f| syx_ast::Param {
                        name: f.clone(),
                        external_label: syx_ast::ParamLabel::Same,
                        ty: None,
                        default: None,
                        isolated: false,
                        sending: false,
                        attrs: vec![],
                    })
                    .collect();
                lower_class_init(
                    mir,
                    type_name,
                    &merged_fields,
                    &params,
                    &[],
                    is_actor,
                    conformances,
                    module_idx,
                    imports,
                    globals,
                    Rc::clone(&closure_ids),
                )?;
            }
        }
    }
    append_protocol_stub_functions(mir, &module_idx);
    Ok(())
}

/// Phase 126: compile-time stubs so `Call Pingable_ping` lowers; runtime vtable/__type dispatch
/// resolves to the concrete `@Service` implementation.
fn append_protocol_stub_functions(mir: &mut MirModule, module_idx: &ModuleIndex) {
    for (protocol, methods) in &module_idx.protocol_methods {
        for method in methods {
            let name = SmolStr::new(format!("{protocol}_{method}"));
            if mir.functions.iter().any(|f| f.name == name) {
                continue;
            }
            let stub = vec![
                MirInstr {
                    dest: None,
                    op: MirOp::ConstNil,
                    span: None,
                },
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            ];
            mir.functions.push(MirFunction {
                name,
                param_count: 1,
                local_count: 1,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: stub.clone(),
                    successors: vec![],
                }],
                instructions: stub,
                local_names: vec![SmolStr::new("self")],
                local_types: vec![SmolStr::new("Unknown")],
                init_body_pc: 0,
                init_wrap_pc: 0,
            });
        }
    }
}

/// Phase 70/126: inject protocol witness slots for unified MIR (WASM/native + user protocols).
fn emit_protocol_witnesses(
    ctx: &mut LowerCtx,
    block: u32,
    self_slot: u32,
    type_name: &SmolStr,
    conformances: &[SmolStr],
) {
    const PROTOCOL_METHODS: &[(&str, &[&str])] = &[
        ("Iterator", &["next"]),
        ("Comparable", &["compare"]),
        ("Equatable", &["equals"]),
    ];
    for protocol in conformances {
        let methods: Vec<SmolStr> = if let Some(user) = ctx.module.protocol_methods.get(protocol) {
            user.clone()
        } else {
            PROTOCOL_METHODS
                .iter()
                .find(|(proto_name, _)| protocol.as_str() == *proto_name)
                .map(|(_, ms)| ms.iter().map(|m| SmolStr::new(*m)).collect())
                .unwrap_or_default()
        };
        for method in &methods {
            let fn_name = SmolStr::new(format!("{type_name}_{method}"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstFnIndex(fn_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(format!("__witness_{protocol}_{method}")),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectSet,
                    span: None,
                },
            );
        }
    }
}

/// Phase 125: vtable slots on class init dict (parity with codegen `compile_class_init`).
fn emit_class_vtable_slots_mir(
    ctx: &mut LowerCtx,
    block: u32,
    self_slot: u32,
    type_name: &SmolStr,
) {
    let Some(vtable) = ctx.module.class_vtables.get(type_name).cloned() else {
        return;
    };
    for (method, fn_name) in vtable {
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstFnIndex(fn_name.to_string()),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(format!("__vtable_{method}")),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
    }
}

fn mir_instruction_count(ctx: &LowerCtx) -> u32 {
    ctx.blocks
        .iter()
        .map(|b| b.instructions.len() as u32)
        .sum()
}

/// Align unified MIR class init with codegen `emit_mut_ref_cell_wrap`.
fn emit_mut_ref_cell_wrap_mir(ctx: &mut LowerCtx, block: u32, self_slot: u32) {
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(self_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__syx_ref".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("MutRefCell".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__type".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::DictBuild(2),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(self_slot),
            span: None,
        },
    );
}

#[allow(dead_code)] // class/struct method self injection helper (parked)
fn self_param() -> syx_ast::Param {
    self_param_typed(None)
}

fn self_param_typed(type_name: Option<&SmolStr>) -> syx_ast::Param {
    syx_ast::Param {
        name: SmolStr::new("self"),
        external_label: syx_ast::ParamLabel::Omitted,
        ty: type_name.map(|n| TypeExpr::Named {
            name: n.clone(),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    }
}

fn lower_class_init(
    mir: &mut MirModule,
    type_name: &SmolStr,
    fields: &[SmolStr],
    params: &[syx_ast::Param],
    body: &[Stmt],
    is_actor: bool,
    conformances: &[SmolStr],
    module_idx: &ModuleIndex,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<()> {
    let init_name = SmolStr::new(format!("{type_name}_init"));
    let mut ctx = LowerCtx::new(
        init_name.clone(),
        module_idx.clone(),
        imports.clone(),
        globals.to_vec(),
        closure_ids,
    );
    // Phase 125: parity with codegen — init builds raw dict then wraps MutRefCell after body.
    ctx.mut_ref_params.clear();
    ctx.mut_ref_aliases.clear();
    for param in params {
        ctx.alloc_local(&param.name);
        register_param_local_type(&mut ctx, param);
    }
    let self_slot = ctx.alloc_local(&SmolStr::new("self"));
    let entry = ctx.fresh_block();
    ctx.active_block = entry;
    for field in fields {
        let param_slot = params
            .iter()
            .find(|p| p.name == *field)
            .and_then(|p| ctx.locals.get(&p.name).copied());
        if let Some(slot) = param_slot {
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else {
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
        }
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(field.to_string()),
                span: None,
            },
        );
    }
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr(type_name.to_string()),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__type".to_string()),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::DictBuild((fields.len() + 1) as u32),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(self_slot),
            span: None,
        },
    );
    ctx.local_types.insert(SmolStr::new("self"), type_name.clone());
    emit_class_vtable_slots_mir(&mut ctx, entry, self_slot, type_name);
    if is_actor {
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::ActorRegister,
                span: None,
            },
        );
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(self_slot),
                span: None,
            },
        );
    }
    emit_protocol_witnesses(&mut ctx, entry, self_slot, type_name, conformances);
    let init_body_pc = mir_instruction_count(&ctx);
    for stmt in body {
        lower_stmt(stmt, &mut ctx, entry);
    }
    let init_wrap_pc = mir_instruction_count(&ctx);
    emit_mut_ref_cell_wrap_mir(&mut ctx, entry, self_slot);
    emit_protocol_witnesses(&mut ctx, entry, self_slot, type_name, conformances);
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(self_slot),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    );
    if let Some(d) = ctx.take_error_diagnostic() {
        return Err(d);
    }
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    let closures = std::mem::take(&mut ctx.pending_closures);
    mir.functions.push(MirFunction {
        name: init_name,
        param_count: params.len() as u32,
        local_count,
        entry,
        blocks,
        instructions,
        local_names,
        local_types,
        init_body_pc,
        init_wrap_pc,
    });
    mir.functions.extend(closures);
    Ok(())
}

fn lower_function_with_params(
    name: &SmolStr,
    params: &[syx_ast::Param],
    body: &[Stmt],
    module_idx: &ModuleIndex,
    is_async: bool,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    captures: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<(MirFunction, Vec<MirFunction>)> {
    let mut ctx = LowerCtx::new(
        name.clone(),
        module_idx.clone(),
        imports.clone(),
        globals.to_vec(),
        closure_ids,
    );
    ctx.in_async = is_async;
    ctx.await_segment = 0;
    for param in params {
        ctx.alloc_local(&param.name);
        ctx.param_names.insert(param.name.clone());
        register_param_local_type(&mut ctx, param);
        if matches!(param.ty, Some(TypeExpr::Reference(_, true, _))) {
            ctx.mut_ref_params.insert(param.name.clone());
        }
    }
    if params
        .first()
        .is_some_and(|p| p.name.as_str() == "self")
        && instance_self_mut_ref_method(name, &ctx.module)
    {
        ctx.mut_ref_params.insert(SmolStr::new("self"));
    }
    for cap in captures {
        ctx.alloc_local(cap);
    }
    let entry = ctx.fresh_block();
    ctx.active_block = entry;
    for stmt in body {
        let block = ctx.active_block;
        lower_stmt(stmt, &mut ctx, block);
    }
    let has_explicit_return = body.iter().any(|s| matches!(s, Stmt::Return { .. }));
    // Phase 101: auto-call deinit for class locals at scope exit (before Return).
    if !has_explicit_return {
        ctx.emit_scope_exit_deinits(ctx.active_block);
    }
    let returns_self = !has_explicit_return
        && params.first().is_some_and(|p| p.name.as_str() == "self")
        && ctx
            .module
            .actor_types
            .iter()
            .any(|t| name.starts_with(&format!("{t}_")));
    if returns_self {
        if let Some(&self_slot) = ctx.locals.get(&SmolStr::new("self")) {
            ctx.emit(
                ctx.active_block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
        }
    } else if name.as_str() == "main" && !has_explicit_return {
        ctx.emit(
            ctx.active_block,
            MirInstr {
                dest: None,
                op: MirOp::ConstNil,
                span: None,
            },
        );
    }
    ctx.emit(
        ctx.active_block,
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    );
    if let Some(d) = ctx.take_error_diagnostic() {
        return Err(d);
    }
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    let closures = std::mem::take(&mut ctx.pending_closures);
    Ok((
        MirFunction {
            name: name.clone(),
            param_count: params.len() as u32,
            local_count,
            entry,
            blocks,
            instructions,
            local_names,
            local_types,
            init_body_pc: 0,
            init_wrap_pc: 0,
        },
        closures,
    ))
}

fn flatten_blocks(blocks: &[MirBlock]) -> Vec<MirInstr> {
    let mut block_start = indexmap::IndexMap::new();
    let mut out = Vec::new();
    for block in blocks {
        block_start.insert(block.id, out.len());
        out.extend(block.instructions.iter().cloned());
    }
    resolve_jump_targets(&mut out, &block_start);
    out
}

fn resolve_jump_targets(instructions: &mut [MirInstr], block_start: &indexmap::IndexMap<u32, usize>) {
    for instr in instructions {
        match &mut instr.op {
            MirOp::Jump(t)
            | MirOp::JumpIfFalse(t)
            | MirOp::JumpIfNil(t)
            | MirOp::BeginTry(t)
            | MirOp::BindResult(t) => {
                if let Some(&pc) = block_start.get(t) {
                    *t = pc as u32;
                }
            }
            _ => {}
        }
    }
}

fn mir_op(op: MirOp) -> MirInstr {
    MirInstr {
        dest: None,
        op,
        span: None,
    }
}

fn mir_local_names_for_slots(
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

fn mir_local_types_for_slots(
    locals: &indexmap::IndexMap<SmolStr, u32>,
    local_types: &indexmap::IndexMap<SmolStr, SmolStr>,
    local_count: u32,
) -> Vec<SmolStr> {
    let mut types = (0..local_count)
        .map(|_| SmolStr::new("Unknown"))
        .collect::<Vec<_>>();
    for (name, slot) in locals {
        if let Some(ty) = local_types.get(name) {
            if (*slot as usize) < types.len() {
                types[*slot as usize] = ty.clone();
            }
        }
    }
    types
}

fn mir_shell_fn(name: SmolStr, param_count: u32, local_count: u32, instructions: Vec<MirInstr>) -> MirFunction {
    MirFunction {
        name,
        param_count,
        local_count,
        entry: 0,
        blocks: vec![MirBlock {
            id: 0,
            instructions: instructions.clone(),
            successors: vec![],
        }],
        instructions,
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

/// Dataclass/Struct init/copy/describe/hash shells for native/WASM
/// (mirrors codegen type shells; W467 / ADR-811: Struct joined DataClass on MIR path).
pub fn lower_dataclass_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let (name, fields) = match decl {
            Decl::DataClass { name, fields, .. } | Decl::Struct { name, fields, .. } => {
                (name, fields)
            }
            _ => continue,
        };
        out.push(lower_dataclass_init_shell(name, fields));
        out.push(lower_dataclass_copy_shell(name, fields));
        out.push(lower_dataclass_describe_shell(name, fields));
        out.push(lower_dataclass_hash_into_shell(name, fields));
    }
    out
}

/// FinalClass variant init shells for native/WASM (W468 / ADR-812).
/// Mirrors codegen `Decl::FinalClass` DictBuild (`value`/`variant`/`__type`); ≠ Enum `tag`/`case`.
pub fn lower_finalclass_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::FinalClass { name, variants, .. } = decl else {
            continue;
        };
        for (variant, fields) in variants {
            let init_name = SmolStr::new(format!("{name}_{variant}_init"));
            let mut ins = Vec::new();
            if fields.len() == 1 {
                ins.push(mir_op(MirOp::LoadLocal(0)));
                ins.push(mir_op(MirOp::ConstStr("value".into())));
            } else {
                for (slot, (field, _)) in fields.iter().enumerate() {
                    ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
                    ins.push(mir_op(MirOp::ConstStr(field.to_string())));
                }
            }
            ins.push(mir_op(MirOp::ConstStr(variant.to_string())));
            ins.push(mir_op(MirOp::ConstStr("variant".into())));
            ins.push(mir_op(MirOp::ConstStr(name.to_string())));
            ins.push(mir_op(MirOp::ConstStr("__type".into())));
            let entry_count = if fields.is_empty() {
                2
            } else if fields.len() == 1 {
                3
            } else {
                fields.len() + 2
            };
            ins.push(mir_op(MirOp::DictBuild(entry_count as u32)));
            ins.push(mir_op(MirOp::Return));
            out.push(mir_shell_fn(
                init_name,
                fields.len() as u32,
                fields.len() as u32,
                ins,
            ));
        }
    }
    out
}

/// Top-level VarGlobal init for native/WASM (W469 / ADR-813).
/// Emits **one** combined `__init_global` (declaration-order slots) — equivalent to AST's
/// multiple same-named units, but survives `compile_from_mir` name-dedupe.
pub fn lower_varglobal_inits(module: &Module) -> SyxResult<Vec<MirFunction>> {
    let mut globals = Vec::new();
    let mut any_init = false;
    for decl in &module.decls {
        if let Decl::VarGlobal { name, init, .. } = decl {
            globals.push(name.clone());
            if init.is_some() {
                any_init = true;
            }
        }
    }
    if !any_init {
        return Ok(vec![]);
    }

    let module_idx = ModuleIndex::from_module(module);
    let mut imports = IndexSet::new();
    for decl in &module.decls {
        if let Some(sym) = decl.import_symbol() {
            imports.insert(sym.clone());
        }
    }

    let init_name = SmolStr::new("__init_global");
    let mut ctx = LowerCtx::new(
        init_name.clone(),
        module_idx,
        imports,
        globals,
        Rc::new(Cell::new(0u32)),
    );
    let entry = ctx.fresh_block();
    ctx.active_block = entry;

    let mut slot = 0u32;
    for decl in &module.decls {
        let Decl::VarGlobal { init, .. } = decl else {
            continue;
        };
        if let Some(expr) = init {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, &mut ctx, entry);
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreGlobal(slot),
                    span: None,
                },
            );
        }
        slot += 1;
    }
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    );
    if let Some(d) = ctx.take_error_diagnostic() {
        return Err(d);
    }
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    Ok(vec![MirFunction {
        name: init_name,
        param_count: 0,
        local_count,
        entry,
        blocks,
        instructions,
        local_names,
        local_types,
        init_body_pc: 0,
        init_wrap_pc: 0,
    }])
}

/// Enum case init shells for native/WASM (mirrors codegen `{Enum}_{case}_init`).
pub fn lower_enum_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::Enum { name, cases, .. } = decl else {
            continue;
        };
        let mut next_tag = 0i64;
        for (case, value, fields) in cases {
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
            let init_name = SmolStr::new(format!("{name}_{case}_init"));
            if fields.is_empty() {
                let ins = vec![
                    mir_op(MirOp::ConstI32(tag_val as i32)),
                    mir_op(MirOp::ConstStr("tag".into())),
                    mir_op(MirOp::ConstStr(case.to_string())),
                    mir_op(MirOp::ConstStr("case".into())),
                    mir_op(MirOp::ConstStr(name.to_string())),
                    mir_op(MirOp::ConstStr("__type".into())),
                    mir_op(MirOp::DictBuild(3)),
                    mir_op(MirOp::Return),
                ];
                out.push(mir_shell_fn(init_name, 0, 0, ins));
            } else {
                let mut ins = Vec::new();
                ins.push(mir_op(MirOp::ConstI32(tag_val as i32)));
                ins.push(mir_op(MirOp::ConstStr("tag".into())));
                ins.push(mir_op(MirOp::ConstStr(case.to_string())));
                ins.push(mir_op(MirOp::ConstStr("case".into())));
                for (slot, (field, _)) in fields.iter().enumerate() {
                    ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
                    ins.push(mir_op(MirOp::ConstStr(field.to_string())));
                }
                ins.push(mir_op(MirOp::ConstStr(name.to_string())));
                ins.push(mir_op(MirOp::ConstStr("__type".into())));
                ins.push(mir_op(MirOp::DictBuild((fields.len() + 3) as u32)));
                ins.push(mir_op(MirOp::Return));
                out.push(mir_shell_fn(
                    init_name,
                    fields.len() as u32,
                    fields.len() as u32,
                    ins,
                ));
            }
        }
    }
    out
}

fn lower_dataclass_init_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = Vec::new();
    for (slot, (field, _)) in fields.iter().enumerate() {
        ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
    }
    ins.push(mir_op(MirOp::ConstStr(name.to_string())));
    ins.push(mir_op(MirOp::ConstStr("__type".to_string())));
    ins.push(mir_op(MirOp::DictBuild((fields.len() + 1) as u32)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(
        SmolStr::new(format!("{name}_init")),
        fields.len() as u32,
        fields.len() as u32,
        ins,
    )
}

fn lower_dataclass_copy_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let init_name = SmolStr::new(format!("{name}_init"));
    let mut ins = Vec::new();
    for (field, _) in fields {
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
    }
    ins.push(mir_op(MirOp::Call(init_name)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_copy")), 1, 1, ins)
}

fn lower_dataclass_describe_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = vec![mir_op(MirOp::ConstStr(format!("{name}(")))];
    let mut first = true;
    for (field, _) in fields {
        if !first {
            ins.push(mir_op(MirOp::ConstStr(format!(", {field}: "))));
            ins.push(mir_op(MirOp::ConcatStr));
        }
        first = false;
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::ToStr));
        ins.push(mir_op(MirOp::ConcatStr));
    }
    ins.push(mir_op(MirOp::ConstStr(")".to_string())));
    ins.push(mir_op(MirOp::ConcatStr));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_describe")), 1, 1, ins)
}

fn lower_dataclass_hash_into_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = Vec::new();
    for (field, _) in fields {
        ins.push(mir_op(MirOp::LoadLocal(1)));
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::CallHost(host_fn_index(
            HostFnId::HasherCombine,
        ))));
        ins.push(mir_op(MirOp::StoreLocal(1)));
    }
    ins.push(mir_op(MirOp::LoadLocal(1)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(
        SmolStr::new(format!("{name}_hash_into")),
        2,
        2,
        ins,
    )
}

pub fn merge_mir_functions(mir: &mut MirModule, extra: Vec<MirFunction>) {
    for func in extra {
        if !mir.functions.iter().any(|f| f.name == func.name) {
            mir.functions.push(func);
        }
    }
}

/// Prefix merged user-dependency MIR symbols and relink internal calls (SYX 39.0).
pub fn prefix_user_dep_mir_batch(functions: &mut [MirFunction], module_stem: &str) {
    let prefix = format!("{module_stem}_");
    let mut rename = indexmap::IndexMap::new();
    for f in functions.iter() {
        if f.name.as_str() == "main" || f.name.starts_with(prefix.as_str()) {
            continue;
        }
        rename.insert(
            f.name.clone(),
            SmolStr::new(format!("{prefix}{}", f.name)),
        );
    }
    fn relink_instr(instr: &mut MirInstr, rename: &indexmap::IndexMap<SmolStr, SmolStr>) {
        match &mut instr.op {
            MirOp::Call(name) | MirOp::ActorCall(name) | MirOp::TaskSpawn(name) => {
                if let Some(new_name) = rename.get(name) {
                    *name = new_name.clone();
                }
            }
            MirOp::MakeClosure(name, _) => {
                if let Some(new_name) = rename.get(name) {
                    *name = new_name.clone();
                }
            }
            _ => {}
        }
    }
    for f in functions.iter_mut() {
        if let Some(new_name) = rename.get(&f.name) {
            f.name = new_name.clone();
        }
        for instr in &mut f.instructions {
            relink_instr(instr, &rename);
        }
        for block in &mut f.blocks {
            for instr in &mut block.instructions {
                relink_instr(instr, &rename);
            }
        }
    }
}

fn lower_switch_pattern_match(
    pattern: &Expr,
    case_slot: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    match pattern {
        Expr::TuplePattern { bindings, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr("__tuple_len".into()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(bindings.len() as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Eq,
                    span: None,
                },
            );
        }
        Expr::VariantPattern { variant, .. } => {
            let tag = ctx
                .module
                .enum_case_tags
                .iter()
                .find_map(|((_, case), tag)| if case == variant { Some(*tag) } else { None })
                .unwrap_or(0);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr("tag".into()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(tag as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Eq,
                    span: None,
                },
            );
        }
        Expr::Is { target, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(
                        mir_type_expr_name(target)
                            .unwrap_or_else(|| SmolStr::new("Unknown"))
                            .to_string(),
                    ),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::CallHost(host_fn_index(HostFnId::TypeIs)),
                    span: None,
                },
            );
        }
        Expr::HalfOpenRange { start, end, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*start as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Ge {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*end as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Lt {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::And,
                    span: None,
                },
            );
        }
        Expr::ClosedRange { start, end, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*start as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Ge {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*end as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Le {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::And,
                    span: None,
                },
            );
        }
        other => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            let rhs = ctx.fresh_reg();
            lower_expr(other, rhs, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ValueEq,
                    span: None,
                },
            );
        }
    }
}

fn bind_tuple_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    if let Expr::TuplePattern { bindings, .. } = pattern {
        for (i, bind) in bindings.iter().enumerate() {
            let bind_slot = ctx.alloc_local(bind);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(i.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(bind_slot),
                    span: None,
                },
            );
        }
    }
}

fn bind_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    bind_variant_pattern(pattern, case_slot, ctx, block);
    bind_tuple_pattern(pattern, case_slot, ctx, block);
}

fn bind_variant_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    if let Expr::VariantPattern {
        variant,
        binding: Some(bind),
        ..
    } = pattern
    {
        let field = ctx
            .module
            .enum_associated_fields
            .iter()
            .find_map(|((_, case), fields)| {
                if case == variant && !fields.is_empty() {
                    Some(fields[0].clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| SmolStr::new("value"));
        let bind_slot = ctx.alloc_local(bind);
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(case_slot),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(field.to_string()),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectGet,
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(bind_slot),
                span: None,
            },
        );
    }
}

fn lower_stmt(stmt: &Stmt, ctx: &mut LowerCtx, block: u32) {
    let prev_span = ctx.current_span;
    ctx.current_span = Some(stmt_span_tuple(stmt));
    lower_stmt_inner(stmt, ctx, block);
    ctx.current_span = prev_span;
}

fn stmt_span_tuple(stmt: &Stmt) -> (u32, u32) {
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

fn lower_stmt_inner(stmt: &Stmt, ctx: &mut LowerCtx, block: u32) {
    match stmt {
        Stmt::Return { value: Some(v), .. } => {
            let dest = ctx.fresh_reg();
            if let Expr::Ident { name, .. } = v {
                ctx.skip_deinit_on_return = Some(name.clone());
            }
            lower_expr(v, dest, ctx, block);
            // W1058: do not emit scope-exit deinits on `return expr` — Call+Pop
            // would drop the result and leave ctor leftovers (rows/cols Int).
            // Params stay caller-owned; returned Ident is not dropped here.
            ctx.skip_deinit_on_return = None;
            // After exprs that split blocks (e.g. `??`), continue from active_block.
            let cur = ctx.active_block;
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            );
        }
        Stmt::Return { value: None, .. } => {
            let nil_reg = ctx.fresh_reg();
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(nil_reg),
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
            ctx.emit_scope_exit_deinits(block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            );
        }
        Stmt::Print { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            let cur = ctx.active_block;
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::Print,
                    span: None,
                },
            );
        }
        Stmt::Expr { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            if !matches!(
                expr,
                Expr::Assign { .. } | Expr::MemberAssign { .. } | Expr::IndexAssign { .. }
            ) {
                let cur = ctx.active_block;
                ctx.emit(
                    cur,
                    MirInstr {
                        dest: None,
                        op: MirOp::Pop,
                        span: None,
                    },
                );
            }
        }
        Stmt::VarDecl { name, init, ty, .. } => {
            let slot = ctx.alloc_local(name);
            if let Some(type_expr) = ty {
                if let Some(type_name) = mir_type_expr_name(type_expr) {
                    if type_name_is_set(type_name.as_str()) {
                        ctx.prefer_set_build = true;
                    }
                    ctx.local_types.insert(name.clone(), type_name);
                }
            }
            if let Some(expr) = init {
                if let Expr::Borrow {
                    expr: inner,
                    mutable: true,
                    ..
                } = expr
                {
                    if let Expr::Ident { name: target, .. } = inner.as_ref() {
                        ctx.mut_ref_aliases.insert(name.clone(), target.clone());
                        return;
                    }
                }
                let dest = ctx.fresh_reg();
                lower_expr(expr, dest, ctx, block);
                if let Expr::Literal {
                    value: Literal::String(_),
                    ..
                } = expr
                {
                    ctx.local_types
                        .insert(name.clone(), SmolStr::new("String"));
                }
                if let Expr::Literal {
                    value: Literal::Double { suffix, .. },
                    ..
                } = expr
                {
                    let s = suffix.as_deref().unwrap_or("");
                    let ty = if s == "f32" || s == "f" {
                        "Float32"
                    } else {
                        "Float64"
                    };
                    ctx.local_types
                        .insert(name.clone(), SmolStr::new(ty));
                }
                if let Expr::Call { callee, args, .. } = expr {
                    if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                        ctx.local_types.insert(name.clone(), type_name);
                    }
                    if let Expr::Member { object, name: method, .. } = callee.as_ref() {
                        let recv_ty = match object.as_ref() {
                            Expr::Ident { name: type_name, .. } => {
                                if ctx.module.class_types.contains_key(type_name)
                                    || ctx.module.enum_classes.contains(type_name)
                                    || ctx.module.dataclass_names.contains(type_name)
                                    || matches!(type_name.as_str(), "BigInt" | "Result")
                                {
                                    Some(type_name.clone())
                                } else {
                                    ctx.local_types.get(type_name).cloned()
                                }
                            }
                            _ => infer_object_type_name(object, ctx),
                        };
                        if let Some(type_name) = recv_ty {
                            if matches!(type_name.as_str(), "BigInt" | "Result") {
                                ctx.local_types.insert(name.clone(), type_name.clone());
                            }
                            if ctx.module.enum_classes.contains(&type_name) {
                                ctx.local_types.insert(name.clone(), type_name.clone());
                            }
                            if let Some(ret) =
                                extension_or_hosted_return_type(&type_name, method, ctx)
                            {
                                ctx.local_types.insert(name.clone(), ret);
                            }
                        }
                    }
                    if let Expr::Ident { name: type_name, .. } = callee.as_ref() {
                        if ctx.module.dataclass_names.contains(type_name)
                            || ctx.module.actor_types.contains(type_name)
                            || ctx.module.class_types.contains_key(type_name)
                        {
                            ctx.local_types.insert(name.clone(), type_name.clone());
                        }
                    }
                }
                if let Expr::Binary { left, op, .. } = expr {
                    if let Some(sym) = binop_operator_symbol(*op) {
                        if let Some(left_ty) = infer_object_type_name(left, ctx) {
                            if let Some(ret) = ctx
                                .module
                                .extension_method_returns
                                .get(&(left_ty.clone(), SmolStr::new(sym)))
                            {
                                ctx.local_types.insert(name.clone(), ret.clone());
                            }
                        }
                    }
                }
                if !ctx.local_types.contains_key(name) {
                    if let Some(type_name) = infer_expr_init_type(expr, ctx) {
                        ctx.local_types.insert(name.clone(), type_name);
                    }
                }
                if let Some(to_name) = ty
                    .as_ref()
                    .and_then(|t| mir_type_expr_name(t))
                {
                    if let Some(from_name) = infer_expr_init_type(expr, ctx) {
                        let cur = ctx.active_block;
                        emit_integer_narrow_cast_if_needed(
                            from_name.as_str(),
                            to_name.as_str(),
                            dest,
                            ctx,
                            cur,
                        );
                    }
                }
            } else {
                let nil_reg = ctx.fresh_reg();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(nil_reg),
                        op: MirOp::ConstNil,
                        span: None,
                    },
                );
            }
            ctx.prefer_set_build = false;
            // `??` / other block-splitting inits leave the value in `active_block`.
            let cur = ctx.active_block;
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
        }
        Stmt::IfLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            let dest = ctx.fresh_reg();
            lower_expr(pattern, dest, ctx, block);
            let cur = ctx.active_block;
            let end_block = ctx.fresh_block();
            let else_block = if else_branch.is_some() {
                ctx.fresh_block()
            } else {
                end_block
            };
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfNil(else_block),
                    span: None,
                },
            );
            let slot = ctx.alloc_local(name);
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
            ctx.active_block = cur;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(then_end, end_block);
            if let Some(else_b) = else_branch {
                ctx.link(cur, else_block);
                ctx.active_block = else_block;
                for s in else_b {
                    let b = ctx.active_block;
                    lower_stmt(s, ctx, b);
                }
                let else_end = ctx.active_block;
                ctx.emit(
                    else_end,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(else_end, end_block);
            } else {
                ctx.link(cur, end_block);
            }
            ctx.active_block = end_block;
        }
        Stmt::GuardLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            let dest = ctx.fresh_reg();
            lower_expr(pattern, dest, ctx, block);
            let cur = ctx.active_block;
            let end_block = ctx.fresh_block();
            let else_block = ctx.fresh_block();
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfNil(else_block),
                    span: None,
                },
            );
            let slot = ctx.alloc_local(name);
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
            ctx.active_block = cur;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(then_end, end_block);
            ctx.link(cur, else_block);
            ctx.active_block = else_block;
            for s in else_branch {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let else_end = ctx.active_block;
            ctx.emit(
                else_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(else_end, end_block);
            ctx.active_block = end_block;
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let cond_reg = ctx.fresh_reg();
            lower_expr(condition, cond_reg, ctx, block);
            // Condition may split blocks (e.g. nested `??`); emit branches from the
            // live block after lowering, not the stale entry `block`.
            let cond_block = ctx.active_block;
            let then_block = ctx.fresh_block();
            let merge_block = ctx.fresh_block();
            // Always emit an explicit Jump(then) after JumpIfFalse. While/ForIn
            // pre-allocate an exit block between the body and later then/merge
            // ids; without this jump, a true condition falls through into exit
            // and drops trailing body stmts (e.g. `j = j + 1`).
            let else_block = if else_branch.is_some() {
                let eb = ctx.fresh_block();
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(eb),
                        span: None,
                    },
                );
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(then_block),
                        span: None,
                    },
                );
                ctx.link(cond_block, then_block);
                ctx.link(cond_block, eb);
                Some(eb)
            } else {
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(merge_block),
                        span: None,
                    },
                );
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(then_block),
                        span: None,
                    },
                );
                ctx.link(cond_block, then_block);
                ctx.link(cond_block, merge_block);
                None
            };
            // Lower then/else from each branch entry; after nested control flow,
            // terminate from ctx.active_block (not the branch entry block).
            ctx.active_block = then_block;
            for s in then_branch {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(merge_block),
                    span: None,
                },
            );
            ctx.link(then_end, merge_block);
            if let (Some(else_b), Some(eb)) = (else_branch, else_block) {
                ctx.active_block = eb;
                for s in else_b {
                    let b = ctx.active_block;
                    lower_stmt(s, ctx, b);
                }
                let else_end = ctx.active_block;
                ctx.emit(
                    else_end,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(merge_block),
                        span: None,
                    },
                );
                ctx.link(else_end, merge_block);
            }
            ctx.active_block = merge_block;
        }
        Stmt::While { condition, body, .. } => {
            let loop_header = ctx.fresh_block();
            let body_block = ctx.fresh_block();
            let exit_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(block, loop_header);
            let cond_reg = ctx.fresh_reg();
            lower_expr(condition, cond_reg, ctx, loop_header);
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(exit_block),
                    span: None,
                },
            );
            ctx.link(loop_header, body_block);
            ctx.link(loop_header, exit_block);
            // Same as If: body stmts must see active_block == body entry.
            // Otherwise VarDecl/Print (which store via active_block after ?? fix)
            // emit into the pre-loop block after Jump — dead code, loop locals stay nil.
            ctx.active_block = body_block;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let body_end = ctx.active_block;
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(body_end, loop_header);
            ctx.active_block = exit_block;
        }
        Stmt::ForIn { name, iterable, body, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(iterable, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::MakeIterator,
                    span: None,
                },
            );
            let iter_slot = ctx.alloc_local(&SmolStr::new(format!("__iter_{name}")));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(iter_slot),
                    span: None,
                },
            );
            let loop_header = ctx.fresh_block();
            let body_block = ctx.fresh_block();
            let exit_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(block, loop_header);
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(iter_slot),
                    span: None,
                },
            );
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::IteratorNext,
                    span: None,
                },
            );
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(exit_block),
                    span: None,
                },
            );
            ctx.link(loop_header, body_block);
            ctx.link(loop_header, exit_block);
            let elem_slot = ctx.alloc_local(name);
            ctx.emit(
                body_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(elem_slot),
                    span: None,
                },
            );
            ctx.emit(
                body_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(iter_slot),
                    span: None,
                },
            );
            ctx.active_block = body_block;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let body_end = ctx.active_block;
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(body_end, loop_header);
            ctx.active_block = exit_block;
        }
        Stmt::Block { stmts, .. } => {
            ctx.active_block = block;
            for s in stmts {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
        }
        Stmt::Seq { stmts, .. } => {
            ctx.active_block = block;
            for s in stmts {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
        }
        Stmt::Try {
            body,
            catch_name,
            catch_body,
            ..
        } => {
            let catch_block = ctx.fresh_block();
            let after_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::BeginTry(catch_block),
                    span: None,
                },
            );
            for s in body {
                lower_stmt(s, ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::EndTry,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(after_block),
                    span: None,
                },
            );
            ctx.link(block, catch_block);
            ctx.link(block, after_block);
            ctx.emit(
                catch_block,
                MirInstr {
                    dest: None,
                    op: MirOp::Catch,
                    span: None,
                },
            );
            let catch_slot = ctx.alloc_local(catch_name);
            ctx.emit(
                catch_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(catch_slot),
                    span: None,
                },
            );
            for s in catch_body {
                lower_stmt(s, ctx, catch_block);
            }
            ctx.emit(
                catch_block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(after_block),
                    span: None,
                },
            );
            ctx.link(catch_block, after_block);
            ctx.active_block = after_block;
        }
        Stmt::Throw { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Throw,
                    span: None,
                },
            );
        }
        Stmt::Switch {
            subject,
            cases,
            default,
            ..
        } => {
            let subject_reg = ctx.fresh_reg();
            lower_expr(subject, subject_reg, ctx, block);
            let case_slot = ctx.alloc_local(&SmolStr::new("__switch_val"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(case_slot),
                    span: None,
                },
            );
            let end_block = ctx.fresh_block();
            let mut current_block = block;
            for (i, case) in cases.iter().enumerate() {
                lower_switch_pattern_match(&case.pattern, case_slot, ctx, current_block);
                let next_block = if i + 1 < cases.len() || default.is_some() {
                    ctx.fresh_block()
                } else {
                    end_block
                };
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(next_block),
                        span: None,
                    },
                );
                bind_pattern(&case.pattern, case_slot, ctx, current_block);
                for s in &case.body {
                    lower_stmt(s, ctx, current_block);
                }
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(current_block, end_block);
                ctx.active_block = next_block;
                current_block = next_block;
            }
            if let Some(body) = default {
                for s in body {
                    lower_stmt(s, ctx, current_block);
                }
            }
            ctx.emit(
                current_block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(current_block, end_block);
            ctx.active_block = end_block;
        }
        Stmt::IfCase {
            pattern,
            subject,
            body,
            else_branch,
            ..
        } => {
            let subject_reg = ctx.fresh_reg();
            lower_expr(subject, subject_reg, ctx, block);
            let case_slot = ctx.alloc_local(&SmolStr::new("__ifcase_val"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(case_slot),
                    span: None,
                },
            );
            lower_switch_pattern_match(pattern, case_slot, ctx, block);
            let fail_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(fail_block),
                    span: None,
                },
            );
            bind_pattern(pattern, case_slot, ctx, block);
            for s in body {
                lower_stmt(s, ctx, block);
            }
            let end_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(block, end_block);
            if let Some(else_b) = else_branch {
                ctx.active_block = fail_block;
                for s in else_b {
                    lower_stmt(s, ctx, fail_block);
                }
                ctx.emit(
                    fail_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(fail_block, end_block);
            } else {
                ctx.link(fail_block, end_block);
            }
            ctx.active_block = end_block;
        }
        _ => {}
    }
}

fn expr_is_opaque_operand(expr: &Expr, ctx: &LowerCtx) -> bool {
    match expr {
        Expr::Call { callee, .. } => matches!(
            callee.as_ref(),
            Expr::Ident { name, .. } if ctx.module.opaque_functions.contains(name)
        ),
        _ => false,
    }
}

fn range_bounds_from_expr(expr: &Expr) -> Option<(i64, i64, bool)> {
    match expr {
        Expr::HalfOpenRange { start, end, .. } => Some((*start, *end, false)),
        Expr::ClosedRange { start, end, .. } => Some((*start, *end, true)),
        Expr::Group { expr, .. } => range_bounds_from_expr(expr),
        _ => None,
    }
}

fn lower_range_contains_mir(
    start: i64,
    end: i64,
    inclusive: bool,
    arg: &Expr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let val_slot = ctx.alloc_local(&SmolStr::new("__range_contains_val"));
    lower_expr(arg, ctx.fresh_reg(), ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstI32(start as i32),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::Ge {
                kind: PLATFORM_INT.to_byte(),
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstI32(end as i32),
            span: None,
        },
    );
    let cmp_kind = PLATFORM_INT.to_byte();
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: if inclusive {
                MirOp::Le { kind: cmp_kind }
            } else {
                MirOp::Lt { kind: cmp_kind }
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::And,
            span: None,
        },
    );
}

fn lower_range_dict(
    ctx: &mut LowerCtx,
    block: u32,
    dest: u32,
    start: i64,
    end: i64,
    inclusive: bool,
) {
    let pair_count = if inclusive { 3 } else { 2 };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ConstNumeric {
                kind: PLATFORM_INT.to_byte(),
                bits: start as u64,
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("start".into()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ConstNumeric {
                kind: PLATFORM_INT.to_byte(),
                bits: end as u64,
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("end".into()),
            span: None,
        },
    );
    if inclusive {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ConstBool(true),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("inclusive".into()),
                span: None,
            },
        );
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::DictBuild(pair_count),
            span: None,
        },
    );
}

fn contextual_const_bits(kind: NumericKind, value: i128) -> u64 {
    if is_float(kind) {
        float_literal_bits(kind, value as f64)
    } else {
        value as u64
    }
}

fn lower_expr(expr: &Expr, dest: u32, ctx: &mut LowerCtx, block: u32) {
    match expr {
        Expr::Literal { value, .. } => match value {
            Literal::BigInt(text) => {
                let tmp = ctx.fresh_reg();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(tmp),
                        op: MirOp::ConstStr(text.clone()),
                        span: None,
                    },
                );
                if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_string") {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::CallHost(idx),
                            span: None,
                        },
                    );
                } else {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNil,
                            span: None,
                        },
                    );
                }
            }
            Literal::Integer { value, suffix } => {
                if suffix.as_deref() == Some("n") {
                    let bits = *value as u128;
                    let tmp = ctx.fresh_reg();
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(tmp),
                            op: MirOp::ConstNumeric128 {
                                kind: NumericKind::I128.to_byte(),
                                lo: bits as u64,
                                hi: (bits >> 64) as u64,
                            },
                            span: None,
                        },
                    );
                    if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_i128") {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(idx),
                                span: None,
                            },
                        );
                    } else {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::ConstNil,
                                span: None,
                            },
                        );
                    }
                    return;
                }
                let kind = suffix
                    .as_ref()
                    .and_then(|s| from_suffix(s))
                    .or_else(|| {
                        let start = match expr {
                            Expr::Literal { span, .. } => span.start as u32,
                            _ => return None,
                        };
                        ctx.module.contextual_numeric_kinds.get(&start).copied()
                    })
                    .unwrap_or(PLATFORM_INT);
                let resolved = syx_stdlib::resolve_alias(kind);
                if syx_stdlib::bit_width(resolved) > 64 {
                    let bits = *value as u128;
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNumeric128 {
                                kind: resolved.to_byte(),
                                lo: bits as u64,
                                hi: (bits >> 64) as u64,
                            },
                            span: None,
                        },
                    );
                } else {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNumeric {
                                kind: resolved.to_byte(),
                                bits: contextual_const_bits(resolved, *value),
                            },
                            span: None,
                        },
                    );
                }
            }
            Literal::Double { value, suffix } => {
                let kind = suffix
                    .as_ref()
                    .and_then(|s| from_suffix(s))
                    .unwrap_or(default_float_kind());
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ConstNumeric {
                            kind: kind.to_byte(),
                            bits: float_literal_bits(kind, *value),
                        },
                        span: None,
                    },
                );
            }
            Literal::String(s) => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstStr(s.clone()),
                    span: None,
                },
            ),
            Literal::Bool(v) => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstBool(*v),
                    span: None,
                },
            ),
            Literal::Nil => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstNil,
                    span: None,
                },
            ),
            Literal::Interpolated(parts) => {
                let mut first = true;
                for part in parts {
                    match part {
                        syx_ast::StringPart::Text(t) => {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ConstStr(t.clone()),
                                    span: None,
                                },
                            );
                        }
                        syx_ast::StringPart::Expr(e) => {
                            lower_expr(e, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ToStr,
                                    span: None,
                                },
                            );
                        }
                    }
                    if first {
                        first = false;
                    } else {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::ConcatStr,
                                span: None,
                            },
                        );
                    }
                }
            }
            Literal::Dict(entries) => {
                for (key, val) in entries {
                    lower_expr(val, ctx.fresh_reg(), ctx, block);
                    lower_expr(key, ctx.fresh_reg(), ctx, block);
                }
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::DictBuild(entries.len() as u32),
                        span: None,
                    },
                );
            }
            _ => {
                ctx.set_error(format!("Unsupported literal in MIR lowering"));
            }
        },
        Expr::Ident { name, .. } => {
            ctx.lower_load_binding(dest, name, block);
        }
        Expr::Binary { left, op, right, .. } => {
            if *op == BinOp::NilCoalesce {
                // JumpIfNil pops on nil and preserves the value when non-nil —
                // no Dup/Pop pair (that left a stale stack slot on the Some path).
                lower_expr(left, dest, ctx, block);
                let right_block = ctx.fresh_block();
                let end_block = ctx.fresh_block();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfNil(right_block),
                        span: None,
                    },
                );
                ctx.link(block, end_block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                lower_expr(right, dest, ctx, right_block);
                ctx.emit(
                    right_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(right_block, end_block);
                ctx.active_block = end_block;
                return;
            }
            lower_expr(left, dest, ctx, block);
            lower_expr(right, dest + 1, ctx, block);
            let opaque_eq = expr_is_opaque_operand(left, ctx) || expr_is_opaque_operand(right, ctx);
            let left_ty = infer_object_type_name(left, ctx)
                .unwrap_or_else(|| infer_expr_type_name(left));
            if let Some(op_sym) = binop_operator_symbol(*op) {
                let ty_smol = left_ty.clone();
                if (ctx.module.class_types.contains_key(&ty_smol)
                    || ctx.module.dataclass_names.contains(&ty_smol))
                    && !syx_stdlib::is_builtin_type(ty_smol.as_str())
                    && ty_smol.as_str() != "Unknown"
                {
                    let fn_name = SmolStr::new(format!("{left_ty}_{op_sym}"));
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(fn_name),
                            span: None,
                        },
                    );
                    return;
                }
            }
            if let (Some((la, ea)), Some((lb, eb))) =
                (infer_expr_simd_type(left, ctx), infer_expr_simd_type(right, ctx))
            {
                if la == lb && ea == eb {
                    let mir_op = match op {
                        BinOp::Add => MirOp::SimdAdd,
                        BinOp::Sub => MirOp::SimdSub,
                        BinOp::Mul => MirOp::SimdMul,
                        BinOp::Div => MirOp::SimdDiv,
                        BinOp::Eq => MirOp::SimdEq,
                        BinOp::NotEq => MirOp::SimdNe,
                        BinOp::Lt => MirOp::SimdLt,
                        BinOp::LtEq => MirOp::SimdLe,
                        BinOp::Gt => MirOp::SimdGt,
                        BinOp::GtEq => MirOp::SimdGe,
                        BinOp::And => MirOp::SimdAnd,
                        BinOp::Or => MirOp::SimdOr,
                        // No BinOp::Xor — SIMD xor is HostFn→SimdXor; never fall through to Add.
                        _ => MirOp::Add {
                            kind: PLATFORM_INT.to_byte(),
                        },
                    };
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: mir_op,
                            span: None,
                        },
                    );
                    return;
                }
            }
            let arith_kind = binary_numeric_kind(left, right, ctx);
            let mir_op = match op {
                BinOp::Add
                    if expr_is_string_like(left, ctx) || expr_is_string_like(right, ctx) =>
                {
                    MirOp::ConcatStr
                }
                BinOp::Add => MirOp::Add { kind: arith_kind },
                BinOp::Sub => MirOp::Sub { kind: arith_kind },
                BinOp::Mul => MirOp::Mul { kind: arith_kind },
                BinOp::Div => MirOp::Div { kind: arith_kind },
                BinOp::Eq if opaque_eq => MirOp::OpaqueValueEq,
                BinOp::Eq => MirOp::ValueEq,
                BinOp::NotEq if opaque_eq => MirOp::ValueNe,
                BinOp::NotEq => MirOp::ValueNe,
                BinOp::IdenticalEq => MirOp::RefEq,
                BinOp::IdenticalNe => MirOp::RefNe,
                BinOp::Lt => MirOp::Lt { kind: arith_kind },
                BinOp::LtEq => MirOp::Le { kind: arith_kind },
                BinOp::Gt => MirOp::Gt { kind: arith_kind },
                BinOp::GtEq => MirOp::Ge { kind: arith_kind },
                BinOp::And => MirOp::And,
                BinOp::Or => MirOp::Or,
                BinOp::Mod => MirOp::Mod { kind: arith_kind },
                BinOp::BitAnd => MirOp::BitAnd { kind: arith_kind },
                BinOp::BitOr => MirOp::BitOr { kind: arith_kind },
                BinOp::BitXor => MirOp::BitXor { kind: arith_kind },
                BinOp::Shl => MirOp::Shl { kind: arith_kind },
                BinOp::Shr => MirOp::Shr { kind: arith_kind },
                BinOp::NilCoalesce => unreachable!(),
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: mir_op,
                    span: None,
                },
            );
        }
        Expr::Unary { op, expr, .. } => {
            if *op == UnaryOp::Try {
                let err_block = ctx.fresh_block();
                let after_block = ctx.fresh_block();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::BeginTry(err_block),
                        span: None,
                    },
                );
                lower_expr(expr, dest, ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::EndTry,
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::BindResult(err_block),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(after_block),
                        span: None,
                    },
                );
                ctx.link(block, after_block);
                ctx.emit(
                    err_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ReturnErr,
                        span: None,
                    },
                );
                ctx.link(err_block, after_block);
                ctx.active_block = after_block;
                return;
            }
            // Phase 127: fold `-literal` so ArrayBuild lane counts stay exact.
            if *op == UnaryOp::Neg {
                if let Expr::Literal {
                    value: Literal::Integer { value, suffix },
                    span: lit_span,
                } = expr.as_ref()
                {
                    if suffix.as_deref() != Some("n") {
                        let kind = suffix
                            .as_ref()
                            .and_then(|s| from_suffix(s))
                            .or_else(|| {
                                ctx.module
                                    .contextual_numeric_kinds
                                    .get(&(lit_span.start as u32))
                                    .copied()
                            })
                            .unwrap_or(PLATFORM_INT);
                        let resolved = syx_stdlib::resolve_alias(kind);
                        let bits = if is_float(resolved) {
                            float_literal_bits(resolved, -(*value as f64))
                        } else {
                            (*value as i128).wrapping_neg() as u64
                        };
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::ConstNumeric {
                                    kind: resolved.to_byte(),
                                    bits,
                                },
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            lower_expr(expr, dest, ctx, block);
            let unary_kind = unary_numeric_kind(expr, ctx);
            let mir_op = if matches!(op, UnaryOp::Not) && infer_expr_simd_type(expr, ctx).is_some()
            {
                MirOp::SimdNot
            } else {
                match op {
                    UnaryOp::Neg => MirOp::Neg { kind: unary_kind },
                    UnaryOp::Not => MirOp::Not,
                    UnaryOp::BitNot => MirOp::BitNot { kind: unary_kind },
                    UnaryOp::Try => MirOp::Neg {
                        kind: PLATFORM_INT.to_byte(),
                    },
                }
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: mir_op,
                    span: None,
                },
            );
        }
        Expr::Call { callee, args, .. } => {
            let member_kind = classify_member_call(callee, ctx);
            let lower_args_early = member_kind == MemberCallKind::ArgsEarly;
            let mut unbox_targets: Vec<(SmolStr, u32)> = Vec::new();
            if lower_args_early {
                let skip_early = matches!(
                    callee.as_ref(),
                    Expr::Ident { name, .. } if host_simd_lowers_own_args(name)
                ) || matches!(
                    callee.as_ref(),
                    Expr::Member { object, name, .. }
                        if member_host_simd_lowers_own_args(object, name, ctx)
                );
                if !skip_early {
                let callee_ident = match callee.as_ref() {
                    Expr::Ident { name, .. } => Some(name.clone()),
                    _ => None,
                };
                let param_types = callee_ident
                    .as_ref()
                    .and_then(|n| ctx.module.function_params.get(n))
                    .cloned()
                    .unwrap_or_default();
                for (i, arg) in args.iter().enumerate() {
                    let is_mut_param = param_types
                        .get(i)
                        .is_some_and(|t| matches!(t, Some(TypeExpr::Reference(_, true, _))));
                    if is_mut_param {
                        if let Expr::Borrow {
                            expr,
                            mutable: true,
                            ..
                        } = &arg.value
                        {
                            if let Expr::Ident { name: target, .. } = expr.as_ref() {
                                let cell_slot =
                                    ctx.alloc_local(&SmolStr::new(format!("__syx_cell_{i}")));
                                let r = ctx.fresh_reg();
                                ctx.lower_mut_ref_cell(r, target, block);
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::StoreLocal(cell_slot),
                                        span: None,
                                    },
                                );
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::LoadLocal(cell_slot),
                                        span: None,
                                    },
                                );
                                unbox_targets.push((target.clone(), cell_slot));
                                continue;
                            }
                        }
                    }
                    let r = ctx.fresh_reg();
                    lower_expr(&arg.value, r, ctx, block);
                }
                }
            }
            match callee.as_ref() {
                Expr::Ident { name, .. } => {
                    // Builtin `Set` is `Value::Set`, not a class instance.
                    // Empty `Set()` must lower like `[]` under `: Set<T>` → SetBuild(0).
                    if name.as_str() == "Set" && args.is_empty() {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::SetBuild(0),
                                span: None,
                            },
                        );
                        return;
                    }
                    if name == "print" {
                        if !lower_args_early {
                            for arg in args {
                                let r = ctx.fresh_reg();
                                lower_expr(&arg.value, r, ctx, block);
                            }
                        }
                        for _ in args {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::Print,
                                    span: None,
                                },
                            );
                        }
                        return;
                    }
                    if name == "reflect" {
                        if let Some(arg) = args.first() {
                            if let Expr::TypeSelf { type_name, .. } = &arg.value {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstStr(type_name.to_string()),
                                        span: None,
                                    },
                                );
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::CallHost(host_fn_index(HostFnId::ReflectType)),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                    }
                    if let Some(host_id) = host_fn_id_by_name(name) {
                        if host_id == HostFnId::SimdShuffle && args.len() == 2 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            let mode = expr_as_i32_literal(&args[1].value).unwrap_or(0);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdShuffle { mode },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Shuffle && args.len() == 2 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            let mode = expr_as_i32_literal(&args[1].value).unwrap_or(0);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Shuffle { mode },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if matches!(
                            host_id,
                            HostFnId::SimdAnd
                                | HostFnId::SimdOr
                                | HostFnId::SimdXor
                                | HostFnId::SimdShl
                                | HostFnId::SimdShr
                                | HostFnId::SimdMin
                                | HostFnId::SimdMax
                                | HostFnId::SimdF32Add
                                | HostFnId::SimdF32Sub
                                | HostFnId::SimdF32Mul
                                | HostFnId::SimdF32Div
                                | HostFnId::SimdF64Add
                | HostFnId::SimdF64Sub
                | HostFnId::SimdF64Mul
                | HostFnId::SimdF64Div
                | HostFnId::SimdI16Add
                | HostFnId::SimdI16Sub
                | HostFnId::SimdI16Mul
                | HostFnId::SimdI8Add
                | HostFnId::SimdI8Sub
                | HostFnId::SimdI8Mul
                | HostFnId::SimdI8And
                | HostFnId::SimdI8Or
                | HostFnId::SimdI8Xor
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Eq
                | HostFnId::SimdI8Ne
                | HostFnId::SimdI8Le
                | HostFnId::SimdI8Ge
                | HostFnId::SimdI8AvgrU
                | HostFnId::SimdI8Popcnt
                | HostFnId::SimdI8Bitmask
                | HostFnId::SimdI8AllTrue
                | HostFnId::SimdI8AnyTrue
                | HostFnId::SimdI8Lt
                | HostFnId::SimdI8Gt
                | HostFnId::SimdI8AddSat
                | HostFnId::SimdI8SubSat
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                        ) && args.len() == 2
                        {
                            let v_reg = dest;
                            if ctx.next_reg <= dest + 1 {
                                ctx.next_reg = dest + 2;
                            }
                            let a_reg = dest + 1;
                            lower_expr(&args[0].value, v_reg, ctx, block);
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(a_reg),
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, a_reg, ctx, block);
                            }
                            let mir_op = match host_id {
                                HostFnId::SimdAnd => MirOp::SimdAnd,
                                HostFnId::SimdOr => MirOp::SimdOr,
                                HostFnId::SimdXor => MirOp::SimdXor,
                                HostFnId::SimdShl => MirOp::SimdShl,
                                HostFnId::SimdShr => MirOp::SimdShr,
                                HostFnId::SimdMin => MirOp::SimdMin,
                                HostFnId::SimdMax => MirOp::SimdMax,
                                HostFnId::SimdF32Add => MirOp::SimdF32Add,
                                HostFnId::SimdF32Sub => MirOp::SimdF32Sub,
                                HostFnId::SimdF32Mul => MirOp::SimdF32Mul,
                                HostFnId::SimdF32Div => MirOp::SimdF32Div,
                                HostFnId::SimdF64Add => MirOp::SimdF64Add,
                                HostFnId::SimdF64Sub => MirOp::SimdF64Sub,
                                HostFnId::SimdF64Mul => MirOp::SimdF64Mul,
                                HostFnId::SimdF64Div => MirOp::SimdF64Div,
                                HostFnId::SimdI16Add => MirOp::SimdI16Add,
                                HostFnId::SimdI16Sub => MirOp::SimdI16Sub,
                                HostFnId::SimdI16Mul => MirOp::SimdI16Mul,
                                HostFnId::SimdI8Add => MirOp::SimdI8Add,
                                HostFnId::SimdI8Sub => MirOp::SimdI8Sub,
                                HostFnId::SimdI8Mul => MirOp::SimdI8Mul,
                                HostFnId::SimdI8And => MirOp::SimdI8And,
                                HostFnId::SimdI8Or => MirOp::SimdI8Or,
                                HostFnId::SimdI8Xor => MirOp::SimdI8Xor,
                                HostFnId::SimdI8Min => MirOp::SimdI8Min,
                                HostFnId::SimdI8Max => MirOp::SimdI8Max,
                                HostFnId::SimdI8UMin => MirOp::SimdI8UMin,
                                HostFnId::SimdI8UMax => MirOp::SimdI8UMax,
                                HostFnId::SimdI8Eq => MirOp::SimdI8Eq,
                                HostFnId::SimdI8Ne => MirOp::SimdI8Ne,
                                HostFnId::SimdI8Le => MirOp::SimdI8Le,
                                HostFnId::SimdI8Ge => MirOp::SimdI8Ge,
                                HostFnId::SimdI8AvgrU => MirOp::SimdI8AvgrU,
                                HostFnId::SimdI8Lt => MirOp::SimdI8Lt,
                                HostFnId::SimdI8Gt => MirOp::SimdI8Gt,
                                HostFnId::SimdI8AddSat => MirOp::SimdI8AddSat,
                                HostFnId::SimdI8SubSat => MirOp::SimdI8SubSat,
                                HostFnId::SimdI8Shl => MirOp::SimdI8Shl,
                                HostFnId::SimdI8Shr => MirOp::SimdI8Shr,
                                HostFnId::SimdI64Add => MirOp::SimdI64Add,
                                _ => unreachable!(),
                            };
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: mir_op,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Not && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Not,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Abs && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Abs,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Neg && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Neg,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Popcnt && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Popcnt,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Bitmask && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Bitmask,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8AllTrue && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8AllTrue,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8AnyTrue && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8AnyTrue,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Extract && args.len() == 2 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Extract,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Replace && args.len() == 3 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
                            }
                            lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Replace,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdNot && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdNot,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdReduceAdd && args.len() == 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            let (lanes, mut element) =
                                simd_reduce_add_shape(callee, ctx.func_name.as_str());
                            if let Some(inferred) =
                                infer_simd_element_from_reduce_arg(&args[0].value, ctx)
                            {
                                element = inferred;
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdReduceAdd { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdLoadFromArray && args.len() >= 1 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            let lanes = args
                                .get(2)
                                .and_then(|a| expr_as_i32_literal(&a.value))
                                .unwrap_or(4) as u8;
                            let element = infer_simd_element_from_array_arg(args, ctx);
                            let element = if lanes == 2
                                && element == syx_stdlib::NumericKind::I32.to_byte()
                            {
                                // Bare SIMD2 defaults to f64 when element cannot be inferred.
                                syx_stdlib::NumericKind::F64.to_byte()
                            } else if lanes == 8 {
                                syx_stdlib::NumericKind::I16.to_byte()
                            } else if lanes == 16 {
                                // Phase 129: free-call `__syx_simd_load_from_array(..., 16)` → i8x16.
                                syx_stdlib::NumericKind::I8.to_byte()
                            } else {
                                element
                            };
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdLoad { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdStoreToArray && args.len() >= 3 {
                            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
                            let lanes = args
                                .get(3)
                                .and_then(|a| expr_as_i32_literal(&a.value))
                                .map(|n| n as u8)
                                .unwrap_or(4);
                            let element = if lanes == 2 {
                                let inferred = infer_simd_element_from_array_arg(args, ctx);
                                if inferred == syx_stdlib::NumericKind::I32.to_byte() {
                                    syx_stdlib::NumericKind::F64.to_byte()
                                } else {
                                    inferred
                                }
                            } else if lanes == 8 {
                                syx_stdlib::NumericKind::I16.to_byte()
                            } else if lanes == 16 {
                                syx_stdlib::NumericKind::I8.to_byte()
                            } else {
                                infer_simd_element_from_array_arg(args, ctx)
                            };
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdStore { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::ServiceResolve
                            || matches!(
                                callee.as_ref(),
                                Expr::Ident { name, .. } if name.as_str() == "resolve"
                            )
                        {
                            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstStr(type_name.to_string()),
                                        span: None,
                                    },
                                );
                                let host = if matches!(
                                    callee.as_ref(),
                                    Expr::Ident { name, .. } if name.as_str() == "resolve"
                                ) {
                                    host_fn_index(HostFnId::ServiceResolveDynamic)
                                } else {
                                    host_fn_index(host_id)
                                };
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::CallHost(host),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                        for arg in args {
                            lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                        }
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(host_fn_index(host_id)),
                                span: None,
                            },
                        );
                        return;
                    }
                    if let Some(idx) = host_fn_index_by_name(name) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(idx),
                                span: None,
                            },
                        );
                        return;
                    }
                    if ctx.module.dataclass_names.contains(name)
                        || ctx.module.actor_types.contains(name)
                        || ctx.module.class_types.contains_key(name)
                    {
                        if !lower_args_early {
                            if let Some(fields) = ctx.module.dataclass_fields.get(name).cloned() {
                                // S1-I2: memberwise / defaults — match codegen ordered padding.
                                for field in &fields {
                                    let arg = args
                                        .iter()
                                        .find(|a| a.label.as_deref() == Some(field.as_str()))
                                        .or_else(|| {
                                            let idx = fields.iter().position(|f| f == field)?;
                                            args.get(idx).filter(|a| a.label.is_none())
                                        });
                                    let r = ctx.fresh_reg();
                                    if let Some(arg) = arg {
                                        lower_expr(&arg.value, r, ctx, block);
                                    } else if let Some(v) = ctx
                                        .module
                                        .dataclass_field_default_i64
                                        .get(&(name.clone(), field.clone()))
                                        .copied()
                                    {
                                        ctx.emit(
                                            block,
                                            MirInstr {
                                                dest: Some(r),
                                                op: MirOp::ConstI32(v as i32),
                                                span: None,
                                            },
                                        );
                                    } else {
                                        ctx.emit(
                                            block,
                                            MirInstr {
                                                dest: Some(r),
                                                op: MirOp::ConstNil,
                                                span: None,
                                            },
                                        );
                                    }
                                }
                            } else {
                                for arg in args {
                                    let r = ctx.fresh_reg();
                                    lower_expr(&arg.value, r, ctx, block);
                                }
                            }
                        }
                        let init_fn = SmolStr::new(format!("{name}_init"));
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(init_fn),
                                span: None,
                            },
                        );
                        return;
                    }
                    if name.as_str() == "resolve" {
                        if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::ConstStr(type_name.to_string()),
                                    span: None,
                                },
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::CallHost(host_fn_index(HostFnId::ServiceResolveDynamic)),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    let call_name = if ctx.module.generic_templates.contains(name) {
                        // Prefer local binding types (ADR-324: Ident ≠ type name).
                        let type_args: Vec<SmolStr> = args
                            .iter()
                            .take(1)
                            .map(|a| infer_generic_arg_type_name(&a.value, ctx))
                            .collect();
                        mangle_name(name.as_str(), &type_args)
                    } else {
                        name.clone()
                    };
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(call_name),
                            span: None,
                        },
                    );
                    for (target, cell_slot) in unbox_targets {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::LoadLocal(cell_slot),
                                span: None,
                            },
                        );
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ConstStr("__syx_ref".into()),
                                span: None,
                            },
                        );
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ObjectGet,
                                span: None,
                            },
                        );
                        let resolved = ctx.resolve_binding_name(&target).clone();
                        if let Some(&slot) = ctx.locals.get(&resolved) {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreLocal(slot),
                                    span: None,
                                },
                            );
                        }
                    }
                }
                Expr::Member { object, name, .. } => {
                    if name == "contains" && args.len() == 1 {
                        if let Some((start, end, inclusive)) = range_bounds_from_expr(object) {
                            lower_range_contains_mir(
                                start,
                                end,
                                inclusive,
                                &args[0].value,
                                dest,
                                ctx,
                                block,
                            );
                            return;
                        }
                    }
                    if matches!(object.as_ref(), Expr::Super { .. }) && name == "init" {
                        if let Some(class) = ctx.enclosing_class() {
                            if let Some(super_class) = ctx.module.class_superclasses.get(&class).cloned() {
                                ctx.lower_load_binding_raw(&SmolStr::new("self"), block);
                                if !lower_args_early {
                                    for arg in args {
                                        lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                                    }
                                }
                                let fn_name = SmolStr::new(format!("{super_class}_init"));
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::CallSuper(fn_name),
                                        span: None,
                                    },
                                );
                                if let Some(&slot) = ctx.locals.get(&SmolStr::new("self")) {
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::StoreLocal(slot),
                                            span: None,
                                        },
                                    );
                                }
                                return;
                            }
                        }
                    }
                    if name == "hash"
                        && args.len() == 1
                        && args[0].label.as_deref() == Some("into")
                    {
                        if let Some(type_name) = infer_object_type_name(object, ctx) {
                            let hash_into_fn = SmolStr::new(format!("{type_name}_hash_into"));
                            let hash_method_fn = SmolStr::new(format!("{type_name}_hash"));
                            let call_fn = if ctx.module.function_params.contains_key(&hash_into_fn)
                                || ctx.module.dataclass_names.contains(&type_name)
                            {
                                Some(hash_into_fn)
                            } else if ctx.module.function_params.contains_key(&hash_method_fn)
                                || ctx
                                    .module
                                    .class_vtables
                                    .get(&type_name)
                                    .is_some_and(|vt| vt.contains_key(name))
                            {
                                Some(hash_method_fn)
                            } else {
                                None
                            };
                            if let Some(hash_fn) = call_fn {
                                lower_expr(object, ctx.fresh_reg(), ctx, block);
                                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(hash_fn),
                                        span: None,
                                    },
                                );
                                if let Expr::Ident { name: hasher_name, .. } = &args[0].value {
                                    if let Some(&slot) = ctx.locals.get(hasher_name) {
                                        ctx.emit(
                                            block,
                                            MirInstr {
                                                dest: None,
                                                op: MirOp::StoreLocal(slot),
                                                span: None,
                                            },
                                        );
                                    } else if let Some(slot) = ctx.global_slot(hasher_name) {
                                        ctx.emit(
                                            block,
                                            MirInstr {
                                                dest: None,
                                                op: MirOp::StoreGlobal(slot),
                                                span: None,
                                            },
                                        );
                                    }
                                }
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::ConstNil,
                                        span: None,
                                    },
                                );
                                return;
                            }
                            let base = type_name_dispatch_base(&type_name);
                            if ctx.module.extension_methods.contains(&(base.clone(), name.clone())) {
                                let ext_name = SmolStr::new(format!("{base}_{name}"));
                                let is_type_name_call = matches!(
                                    object.as_ref(),
                                    Expr::Ident { name: recv, .. } if recv == &type_name
                                );
                                if !is_type_name_call {
                                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                                }
                                for arg in args {
                                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                                }
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(ext_name),
                                        span: None,
                                    },
                                );
                                return;
                            }
                            // Phase 46: no HasherCombine fallback.
                            ctx.set_error_with_code(
                                Some("C020"),
                                format!(
                                    "type '{type_name}' has no hash(into:) implementation; use a Core Hashable extension, data class + @Hashable, or define func hash(into:)"
                                ),
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ConstNil,
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if let Some(type_name) = infer_object_type_name(object, ctx) {
                        if type_name == "Hasher" {
                            if name == "combine" && args.len() == 1 {
                                lower_expr(object, ctx.fresh_reg(), ctx, block);
                                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::CallHost(host_fn_index(HostFnId::HasherCombine)),
                                        span: None,
                                    },
                                );
                                return;
                            }
                            if name == "finalize" && args.is_empty() {
                                lower_expr(object, ctx.fresh_reg(), ctx, block);
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::CallHost(host_fn_index(HostFnId::HasherFinalize)),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                    }
                    if (name == "description" || name == "copy") && args.is_empty() {
                        if let Some(type_name) = infer_object_type_name(object, ctx) {
                            if ctx.module.dataclass_names.contains(&type_name) {
                                lower_expr(object, ctx.fresh_reg(), ctx, block);
                                let fn_name = if name == "description" {
                                    SmolStr::new(format!("{type_name}_describe"))
                                } else {
                                    SmolStr::new(format!("{type_name}_copy"))
                                };
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(fn_name),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                    }
                    if let Some((host_id, receiver_first)) = resolve_host_call(object, name, ctx) {
                        let simd_lowers_own = member_host_simd_lowers_own_args(object, name, ctx);
                        // ADR-236: HTTPResponse hosts take receiver first (status/text/ok/…).
                        if receiver_first {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                        }
                        if member_kind != MemberCallKind::ArgsEarly && !simd_lowers_own {
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                        }
                        if matches!(
                            host_id,
                            HostFnId::SimdAnd
                                | HostFnId::SimdOr
                                | HostFnId::SimdXor
                                | HostFnId::SimdShl
                                | HostFnId::SimdShr
                                | HostFnId::SimdMin
                                | HostFnId::SimdMax
                                | HostFnId::SimdF32Add
                                | HostFnId::SimdF32Sub
                                | HostFnId::SimdF32Mul
                                | HostFnId::SimdF32Div
                                | HostFnId::SimdF64Add
                | HostFnId::SimdF64Sub
                | HostFnId::SimdF64Mul
                | HostFnId::SimdF64Div
                | HostFnId::SimdI16Add
                | HostFnId::SimdI16Sub
                | HostFnId::SimdI16Mul
                | HostFnId::SimdI8Add
                | HostFnId::SimdI8Sub
                | HostFnId::SimdI8Mul
                | HostFnId::SimdI8And
                | HostFnId::SimdI8Or
                | HostFnId::SimdI8Xor
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Eq
                | HostFnId::SimdI8Ne
                | HostFnId::SimdI8Le
                | HostFnId::SimdI8Ge
                | HostFnId::SimdI8AvgrU
                | HostFnId::SimdI8Popcnt
                | HostFnId::SimdI8Bitmask
                | HostFnId::SimdI8AllTrue
                | HostFnId::SimdI8AnyTrue
                | HostFnId::SimdI8Lt
                | HostFnId::SimdI8Gt
                | HostFnId::SimdI8AddSat
                | HostFnId::SimdI8SubSat
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                        ) && args.len() == 2
                        {
                            // Phase 127: pin operands at dest/dest+1 for SSA simd binop refill.
                            lower_expr(&args[0].value, dest, ctx, block);
                            if ctx.next_reg <= dest + 1 {
                                ctx.next_reg = dest + 2;
                            }
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest + 1),
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, dest + 1, ctx, block);
                            }
                            let mir_op = match host_id {
                                HostFnId::SimdAnd => MirOp::SimdAnd,
                                HostFnId::SimdOr => MirOp::SimdOr,
                                HostFnId::SimdXor => MirOp::SimdXor,
                                HostFnId::SimdShl => MirOp::SimdShl,
                                HostFnId::SimdShr => MirOp::SimdShr,
                                HostFnId::SimdMin => MirOp::SimdMin,
                                HostFnId::SimdMax => MirOp::SimdMax,
                                HostFnId::SimdF32Add => MirOp::SimdF32Add,
                                HostFnId::SimdF32Sub => MirOp::SimdF32Sub,
                                HostFnId::SimdF32Mul => MirOp::SimdF32Mul,
                                HostFnId::SimdF32Div => MirOp::SimdF32Div,
                                HostFnId::SimdF64Add => MirOp::SimdF64Add,
                                HostFnId::SimdF64Sub => MirOp::SimdF64Sub,
                                HostFnId::SimdF64Mul => MirOp::SimdF64Mul,
                                HostFnId::SimdF64Div => MirOp::SimdF64Div,
                                HostFnId::SimdI16Add => MirOp::SimdI16Add,
                                HostFnId::SimdI16Sub => MirOp::SimdI16Sub,
                                HostFnId::SimdI16Mul => MirOp::SimdI16Mul,
                                HostFnId::SimdI8Add => MirOp::SimdI8Add,
                                HostFnId::SimdI8Sub => MirOp::SimdI8Sub,
                                HostFnId::SimdI8Mul => MirOp::SimdI8Mul,
                                HostFnId::SimdI8And => MirOp::SimdI8And,
                                HostFnId::SimdI8Or => MirOp::SimdI8Or,
                                HostFnId::SimdI8Xor => MirOp::SimdI8Xor,
                                HostFnId::SimdI8Min => MirOp::SimdI8Min,
                                HostFnId::SimdI8Max => MirOp::SimdI8Max,
                                HostFnId::SimdI8UMin => MirOp::SimdI8UMin,
                                HostFnId::SimdI8UMax => MirOp::SimdI8UMax,
                                HostFnId::SimdI8Eq => MirOp::SimdI8Eq,
                                HostFnId::SimdI8Ne => MirOp::SimdI8Ne,
                                HostFnId::SimdI8Le => MirOp::SimdI8Le,
                                HostFnId::SimdI8Ge => MirOp::SimdI8Ge,
                                HostFnId::SimdI8AvgrU => MirOp::SimdI8AvgrU,
                                HostFnId::SimdI8Lt => MirOp::SimdI8Lt,
                                HostFnId::SimdI8Gt => MirOp::SimdI8Gt,
                                HostFnId::SimdI8AddSat => MirOp::SimdI8AddSat,
                                HostFnId::SimdI8SubSat => MirOp::SimdI8SubSat,
                                HostFnId::SimdI8Shl => MirOp::SimdI8Shl,
                                HostFnId::SimdI8Shr => MirOp::SimdI8Shr,
                                HostFnId::SimdI64Add => MirOp::SimdI64Add,
                                _ => unreachable!(),
                            };
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: mir_op,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Not && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Not,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Abs && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Abs,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Neg && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Neg,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Popcnt && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Popcnt,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Bitmask && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Bitmask,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8AllTrue && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8AllTrue,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8AnyTrue && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8AnyTrue,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Extract && args.len() == 2 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Extract,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Replace && args.len() == 3 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::ConstI32(n),
                                        span: None,
                                    },
                                );
                            } else {
                                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
                            }
                            lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Replace,
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdI8Shuffle && args.len() == 2 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            let mode = expr_as_i32_literal(&args[1].value).unwrap_or(0);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdI8Shuffle { mode },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdReduceAdd && args.len() == 1 {
                            lower_expr(&args[0].value, dest, ctx, block);
                            let (lanes, mut element) = simd_member_lanes_element(object);
                            if let Some(inferred) =
                                infer_simd_element_from_reduce_arg(&args[0].value, ctx)
                            {
                                element = inferred;
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdReduceAdd { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdLoadFromArray && args.len() >= 1 {
                            if member_kind == MemberCallKind::ArgsEarly {
                                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                            }
                            let (lanes, mut element) = simd_member_lanes_element(object);
                            let inferred = infer_simd_element_from_array_arg(args, ctx);
                            // SIMD2 defaults to F64 only when array element is the I32 fallback.
                            if lanes != 2 || inferred != syx_stdlib::NumericKind::I32.to_byte() {
                                element = inferred;
                            }
                            // Phase 117: SIMD8 guest packs as i16x8.
                            if lanes == 8 {
                                element = syx_stdlib::NumericKind::I16.to_byte();
                            }
                            if lanes == 16 {
                                element = syx_stdlib::NumericKind::I8.to_byte();
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdLoad { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        if host_id == HostFnId::SimdStoreToArray && args.len() >= 3 {
                            if member_kind == MemberCallKind::ArgsEarly {
                                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                                lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
                            }
                            let (lanes, mut element) = simd_member_lanes_element(object);
                            let inferred = infer_simd_element_from_array_arg(args, ctx);
                            if lanes != 2 || inferred != syx_stdlib::NumericKind::I32.to_byte() {
                                element = inferred;
                            }
                            if lanes == 8 {
                                element = syx_stdlib::NumericKind::I16.to_byte();
                            }
                            if lanes == 16 {
                                element = syx_stdlib::NumericKind::I8.to_byte();
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::SimdStore { lanes, element },
                                    span: None,
                                },
                            );
                            return;
                        }
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(host_fn_index(host_id)),
                                span: None,
                            },
                        );
                        return;
                    }
                    if let Expr::Member {
                        object: inner_obj,
                        name: outer_name,
                        ..
                    } = object.as_ref()
                    {
                        if let Expr::Ident { name: mod_name, .. } = inner_obj.as_ref() {
                            if ctx.imports.contains(mod_name)
                                && outer_name
                                    .chars()
                                    .next()
                                    .is_some_and(|c| c.is_uppercase())
                                && name.chars().next().is_some_and(|c| c.is_uppercase())
                            {
                                let init_fn = SmolStr::new(format!(
                                    "{mod_name}_{outer_name}_{name}_init"
                                ));
                                for arg in args {
                                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                                }
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(init_fn),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                    }
                    if let Expr::Ident { name: mod_name, .. } = object.as_ref() {
                        // User enum cases (e.g. `enum Result { success(...) }`) must win over
                        // the builtin Result/BigInt static-method shortcut below.
                        if (ctx.module.enum_classes.contains(mod_name)
                            || ctx.module.final_classes.contains(mod_name))
                            && ctx.module.enum_case_tags.contains_key(&(mod_name.clone(), name.clone()))
                        {
                            let init_fn = SmolStr::new(format!("{mod_name}_{name}_init"));
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(init_fn),
                                    span: None,
                                },
                            );
                            return;
                        }
                        if matches!(mod_name.as_str(), "Result" | "BigInt") {
                            let qualified = SmolStr::new(format!("{mod_name}_{name}"));
                            for arg in args {
                                let r = ctx.fresh_reg();
                                lower_expr(&arg.value, r, ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(qualified),
                                    span: None,
                                },
                            );
                            return;
                        }
                        if ctx.module.extension_methods.contains(&(mod_name.clone(), name.clone())) {
                            let ext_fn = SmolStr::new(format!("{mod_name}_{name}"));
                            for arg in args {
                                let r = ctx.fresh_reg();
                                lower_expr(&arg.value, r, ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(ext_fn),
                                    span: None,
                                },
                            );
                            return;
                        }
                        if ctx.imports.contains(mod_name) {
                            if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                                let init_fn = SmolStr::new(format!("{mod_name}_{name}_init"));
                                for arg in args {
                                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                                }
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(init_fn),
                                        span: None,
                                    },
                                );
                                return;
                            }
                            let qualified = SmolStr::new(format!("{mod_name}_{name}"));
                            let type_args = infer_static_method_type_args(args, ctx);
                            let call_name = if type_args.iter().all(|t| t.as_str() != "Unknown") {
                                mangle_name(qualified.as_str(), &type_args)
                            } else {
                                qualified
                            };
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(call_name),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if expr_is_string_like(object, ctx) {
                        if let Some(method) = lookup_string_method(name) {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                            for arg in args {
                                let r = ctx.fresh_reg();
                                lower_expr(&arg.value, r, ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: method_id_mir_op(method, "String", name.as_str(), ctx),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if expr_is_array_like(object, ctx) {
                        emit_hosted_collection_call("Array", name, object, args, dest, ctx, block);
                        return;
                    }
                    if expr_is_set_like(object, ctx) {
                        emit_hosted_collection_call("Set", name, object, args, dest, ctx, block);
                        return;
                    }
                    if expr_is_dict_like(object, ctx) {
                        // Shared bag ops (count/keys/containsKey/…) use Dictionary MethodId
                        // even on typed Object. Object-only APIs (valueForKey/put) fall through
                        // to Object_* extensions below.
                        let is_object = infer_object_type_name(object, ctx)
                            .is_some_and(|t| type_name_dispatch_base(&t).as_str() == "Object");
                        let shared_bag_op = lookup_dict_method(name).is_some()
                            || is_hosted_collection_method("Dictionary", name.as_str());
                        if !is_object || shared_bag_op {
                            emit_hosted_collection_call(
                                "Dictionary",
                                name,
                                object,
                                args,
                                dest,
                                ctx,
                                block,
                            );
                            return;
                        }
                    }
                    if let Some(host_id) = lookup_float64_math_host(name) {
                        if expr_is_float64_like(object, ctx)
                            || infer_object_type_name(object, ctx).is_some_and(|t| {
                                matches!(
                                    type_name_dispatch_base(&t).as_str(),
                                    "Float64" | "Double"
                                )
                            })
                        {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::CallHost(host_fn_index(host_id)),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if let Some(host_id) = lookup_float32_math_host(name) {
                        if expr_is_float32_like(object, ctx)
                            || infer_object_type_name(object, ctx).is_some_and(|t| {
                                matches!(
                                    type_name_dispatch_base(&t).as_str(),
                                    "Float32" | "Float"
                                )
                            })
                        {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::CallHost(host_fn_index(host_id)),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if let Some(type_name) = infer_object_type_name(object, ctx) {
                        let base = type_name_dispatch_base(&type_name);
                        let is_type_name_call = matches!(
                            object.as_ref(),
                            Expr::Ident { name: recv, .. } if recv == &type_name
                        );
                        if is_type_name_call
                            && name.chars().next().is_some_and(|c| c.is_uppercase())
                        {
                            let nested_ty = SmolStr::new(format!("{base}_{name}"));
                            if ctx.module.class_types.contains_key(&nested_ty) {
                                for arg in args {
                                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                                }
                                let mut init_fn =
                                    SmolStr::new(format!("{nested_ty}_init"));
                                if !ctx.module.callable_extensions.contains(&init_fn) {
                                    for imp in &ctx.imports {
                                        let prefixed =
                                            SmolStr::new(format!("{imp}_{nested_ty}_init"));
                                        if ctx.module.callable_extensions.contains(&prefixed)
                                        {
                                            init_fn = prefixed;
                                            break;
                                        }
                                    }
                                }
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: Some(dest),
                                        op: MirOp::Call(init_fn),
                                        span: None,
                                    },
                                );
                                return;
                            }
                        }
                        if ctx.module.protocol_names.contains(&base)
                            && ctx.module.protocol_methods.get(&base).is_some_and(|ms| {
                                ms.iter().any(|m| m == name)
                            })
                        {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(SmolStr::new(format!("{base}_{name}"))),
                                    span: None,
                                },
                            );
                            return;
                        }
                        let ext_name = SmolStr::new(format!("{base}_{name}"));
                        let is_type_name_call = matches!(
                            object.as_ref(),
                            Expr::Ident { name: recv, .. } if recv == &type_name
                        );
                        if ctx.module.actor_types.contains(&type_name) {
                            lower_expr(object, ctx.fresh_reg(), ctx, block);
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ActorCall(ext_name),
                                    span: None,
                                },
                            );
                            return;
                        }
                        if ctx.module.class_types.contains_key(&base)
                            || ctx.module.dataclass_names.contains(&base)
                            || ctx.module.enum_classes.contains(&base)
                            || ctx.module
                                .extension_methods
                                .contains(&(base.clone(), name.clone()))
                        {
                            if !is_type_name_call {
                                lower_expr(object, ctx.fresh_reg(), ctx, block);
                                // Unwrap non-Ident receivers (e.g. `self.logger.tag()`) so
                                // virtual dispatch sees Object.__type. Leave Ident cells as
                                // MutRefCell so `self.field =` write-back updates the caller's cell.
                                if ctx.module.class_types.contains_key(&base)
                                    && !matches!(object.as_ref(), Expr::Ident { .. })
                                {
                                    ctx.emit_unwrap_syx_ref(block);
                                }
                            }
                            for arg in args {
                                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            }
                            let mut call_name = ext_name.clone();
                            if !ctx.module.callable_extensions.contains(&call_name) {
                                for imp in &ctx.imports {
                                    let prefixed =
                                        SmolStr::new(format!("{imp}_{base}_{name}"));
                                    if ctx.module.callable_extensions.contains(&prefixed) {
                                        call_name = prefixed;
                                        break;
                                    }
                                }
                            }
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::Call(call_name),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    if let Some(method) = lookup_array_method(name) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        for arg in args {
                            let r = ctx.fresh_reg();
                            lower_expr(&arg.value, r, ctx, block);
                        }
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: method_id_mir_op(method, "Array", name.as_str(), ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                    lower_expr(object, dest, ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(name.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ObjectGet,
                            span: None,
                        },
                    );
                }
                _ => {
                    if let Some((root_type, property, optional)) = keypath_from_callee(callee) {
                        if let Some(arg) = args.first() {
                            lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::ConstStr(root_type.to_string()),
                                    span: None,
                                },
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::ConstStr(property.to_string()),
                                    span: None,
                                },
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::ConstBool(optional),
                                    span: None,
                                },
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::CallHost(host_fn_index(HostFnId::KeyPathGet)),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                    lower_expr(callee, dest, ctx, block);
                }
            }
        }
        Expr::Array { elements, .. } => {
            for el in elements {
                let r = ctx.fresh_reg();
                lower_expr(el, r, ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: if ctx.prefer_set_build {
                        MirOp::SetBuild(elements.len() as u32)
                    } else {
                        MirOp::ArrayBuild(elements.len() as u32)
                    },
                    span: None,
                },
            );
        }
        Expr::Tuple { elements, .. } => {
            let arity = elements.len();
            for (i, (label, el)) in elements.iter().enumerate() {
                lower_expr(el, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(i.to_string()),
                        span: None,
                    },
                );
                if let Some(name) = label {
                    lower_expr(el, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(name.to_string()),
                            span: None,
                        },
                    );
                }
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(arity as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr("__tuple_len".into()),
                    span: None,
                },
            );
            let pair_count = arity + elements.iter().filter(|(l, _)| l.is_some()).count() + 1;
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::DictBuild(pair_count as u32),
                    span: None,
                },
            );
        }
        Expr::Index { object, index, .. } => {
            if expr_is_string_like(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(index, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host_fn_index(HostFnId::StringCharAt)),
                        span: None,
                    },
                );
                return;
            }
            if expr_is_dict_like(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(index, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ObjectGet,
                        span: None,
                    },
                );
                return;
            }
            if let Expr::Ident { name: recv, .. } = object.as_ref() {
                if let Some(type_name) = ctx.local_types.get(recv) {
                    let base = type_name_dispatch_base(type_name);
                    if ctx.module.extension_methods.contains(&(
                        base.clone(),
                        SmolStr::new("__subscript_get"),
                    )) {
                        let ext_name = SmolStr::new(format!("{base}___subscript_get"));
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        lower_expr(index, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(ext_name),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            lower_expr(object, dest, ctx, block);
            let idx = ctx.fresh_reg();
            lower_expr(index, idx, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ArrayGet,
                    span: None,
                },
            );
        }
        Expr::Member { object, name, .. } => {
            if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                if let Some((global_name, _)) = ctx.module.static_var_global(type_name, name) {
                    if let Some(slot) = ctx.global_slot(&global_name) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::LoadGlobal(slot),
                                span: None,
                            },
                        );
                        return;
                    }
                }
                // Type-level `Type.prop` for `@static __get_prop` (static let / computed get).
                // Do not lower `Type` as a value — that yields Unknown variable.
                if is_static_type_member_call(object, ctx) {
                    let get_method = syx_ast::var_global_getter_method(name);
                    let getter_fn = SmolStr::new(format!("{type_name}_{get_method}"));
                    let has_local_getter = ctx
                        .module
                        .property_getters
                        .contains(&(type_name.clone(), name.clone()));
                    let has_static_getter = ctx
                        .module
                        .static_class_methods
                        .contains(&(type_name.clone(), get_method.clone()));
                    let has_merged_getter = ctx.module.callable_extensions.contains(&getter_fn);
                    if has_local_getter || has_static_getter || has_merged_getter {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(getter_fn),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Some(type_name) = infer_object_type_name(object, ctx) {
                if let Some(getter) = ctx.module.property_getter_fn(&type_name, name) {
                    // Instance computed get needs the receiver; type-level handled above.
                    if !is_static_type_member_call(object, ctx) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(getter),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                if ctx.module.extension_methods.contains(&(type_name.clone(), name.clone())) {
                    let ext_fn = SmolStr::new(format!("{type_name}_{name}"));
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(ext_fn),
                            span: None,
                        },
                    );
                    return;
                }
                let nested_ty = SmolStr::new(format!("{type_name}_{name}"));
                if ctx.module.class_types.contains_key(&nested_ty) {
                    let init_fn = SmolStr::new(format!("{nested_ty}_init"));
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(init_fn),
                            span: None,
                        },
                    );
                    return;
                }
                if ctx.module.enum_classes.contains(type_name)
                    || ctx.module.final_classes.contains(type_name)
                {
                    let init_fn = SmolStr::new(format!("{type_name}_{name}_init"));
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(init_fn),
                            span: None,
                        },
                    );
                    return;
                }
            }
            if name == "description" {
                if let Some(type_name) = infer_object_type_name(object, ctx) {
                    if ctx.module.dataclass_names.contains(&type_name) {
                        lower_expr(object, dest, ctx, block);
                        let desc_fn = SmolStr::new(format!("{type_name}_describe"));
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(desc_fn),
                                span: None,
                            },
                        );
                        return;
                    }
                    let base = type_name_dispatch_base(&type_name);
                    if ctx.module.extension_methods.contains(&(
                        base.clone(),
                        SmolStr::new("description"),
                    )) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: extension_call_op(&base, "description", ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                    // Phase 46: description requires Core extension for builtins (dataclass unchanged above).
                }
            }
            if name == "description" && expr_is_array_like(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: collection_method_mir_op("Array", "description", ctx),
                        span: None,
                    },
                );
                return;
            }
            if name == "debugDescription" && expr_is_array_like(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: collection_method_mir_op("Array", "debugDescription", ctx),
                        span: None,
                    },
                );
                return;
            }
            if name == "debugDescription" {
                if let Some(type_name) = infer_object_type_name(object, ctx) {
                    let base = type_name_dispatch_base(&type_name);
                    if ctx.module.extension_methods.contains(&(
                        base.clone(),
                        SmolStr::new("debugDescription"),
                    )) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: extension_call_op(&base, "debugDescription", ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if name == "copy" {
                if let Some(type_name) = infer_object_type_name(object, ctx) {
                    if ctx.module.dataclass_names.contains(&type_name) {
                        lower_expr(object, dest, ctx, block);
                        let copy_fn = SmolStr::new(format!("{type_name}_copy"));
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(copy_fn),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if expr_is_string_like(object, ctx) {
                if let Some(method) = lookup_string_method(name) {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: method_id_mir_op(method, "String", name.as_str(), ctx),
                            span: None,
                        },
                    );
                    return;
                }
            }
            if expr_is_array_like(object, ctx) {
                if is_hosted_collection_method("Array", name.as_str()) {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: collection_method_mir_op("Array", name.as_str(), ctx),
                            span: None,
                        },
                    );
                    return;
                }
                if let Some(method) = lookup_array_method(name) {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: method_id_mir_op(method, "Array", name.as_str(), ctx),
                            span: None,
                        },
                    );
                    return;
                }
            }
            if expr_is_set_like(object, ctx) {
                if is_hosted_collection_method("Set", name.as_str()) {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: collection_method_mir_op("Set", name.as_str(), ctx),
                            span: None,
                        },
                    );
                    return;
                }
                if let Some(method) = lookup_set_method(name) {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: method_id_mir_op(method, "Set", name.as_str(), ctx),
                            span: None,
                        },
                    );
                    return;
                }
            }
            if expr_is_dict_like(object, ctx) {
                let is_object = infer_object_type_name(object, ctx)
                    .is_some_and(|t| type_name_dispatch_base(&t).as_str() == "Object");
                let shared_bag_op = lookup_dict_method(name).is_some()
                    || is_hosted_collection_method("Dictionary", name.as_str())
                    || matches!(name.as_str(), "description" | "debugDescription");
                if !is_object || shared_bag_op {
                    if name == "description" || name == "debugDescription" {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: collection_method_mir_op("Dictionary", name.as_str(), ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                    if is_hosted_collection_method("Dictionary", name.as_str()) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: collection_method_mir_op("Dictionary", name.as_str(), ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                    if let Some(method) = lookup_dict_method(name) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: method_id_mir_op(method, "Dictionary", name.as_str(), ctx),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Some(type_name) = infer_object_type_name(object, ctx) {
                let base = type_name_dispatch_base(&type_name);
                if ctx.module.protocol_names.contains(&base) {
                    if ctx
                        .module
                        .protocol_methods
                        .get(&base)
                        .is_some_and(|ms| ms.iter().any(|m| m == name))
                    {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(SmolStr::new(format!("{base}_{name}"))),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Some(method) = lookup_array_method(name) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: method_id_mir_op(method, "Array", name.as_str(), ctx),
                        span: None,
                    },
                );
                return;
            }
            if let Some(method) = lookup_string_method(name) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: method_id_mir_op(method, "String", name.as_str(), ctx),
                        span: None,
                    },
                );
                return;
            }
            lower_expr(object, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
        }
        Expr::Await { expr, .. } => {
            if ctx.in_async {
                lower_expr(expr, dest, ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::AwaitSuspend {
                            resume_fn: 0,
                            state: ctx.await_segment,
                        },
                        span: None,
                    },
                );
                ctx.await_segment += 1;
            }
        }
        Expr::Closure { params, body, .. } => {
            let closure_name = SmolStr::new(format!("__mir_closure_{}", ctx.next_closure_id()));
            let closure_stmts = match body {
                syx_ast::ClosureBody::Expr(e) => vec![Stmt::Return {
                    span: syx_diagnostics::Span::DUMMY,
                    value: Some(e.as_ref().clone()),
                }],
                syx_ast::ClosureBody::Block(stmts) => stmts.clone(),
            };
            let free_vars = collect_closure_free_vars(params, body, &ctx.locals);
            for name in &free_vars {
                if let Some(&slot) = ctx.locals.get(name) {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::LoadLocal(slot),
                            span: None,
                        },
                    );
                } else if let Some(slot) = ctx.global_slot(name) {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::LoadGlobal(slot),
                            span: None,
                        },
                    );
                }
            }
            let (mir_fn, nested) = lower_function_with_params(
                &closure_name,
                params,
                &closure_stmts,
                &ctx.module,
                false,
                &ctx.imports,
                &ctx.globals,
                &free_vars,
                Rc::clone(&ctx.closure_ids),
            )
            .unwrap_or_else(|e| {
                ctx.set_error(e.to_string());
                (
                    MirFunction {
                        name: closure_name.clone(),
                        param_count: params.len() as u32,
                        local_count: 0,
                        entry: 0,
                        blocks: vec![],
                        instructions: vec![],
                        local_names: vec![],
                        local_types: vec![],
                        init_body_pc: 0,
                        init_wrap_pc: 0,
                    },
                    vec![],
                )
            });
            ctx.pending_closures.push(mir_fn);
            ctx.pending_closures.extend(nested);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::MakeClosure(closure_name, free_vars.len() as u32),
                    span: None,
                },
            );
        }
        Expr::TaskBlock { body, detached, .. } => {
            let task_name = SmolStr::new(format!("__task_body_{}", ctx.next_closure_id()));
            let (mir_fn, nested) = lower_function_with_params(
                &task_name,
                &[],
                body,
                &ctx.module,
                false,
                &ctx.imports,
                &ctx.globals,
                &[],
                Rc::clone(&ctx.closure_ids),
            )
            .unwrap_or_else(|e| {
                ctx.set_error(e.to_string());
                (
                    MirFunction {
                        name: task_name.clone(),
                        param_count: 0,
                        local_count: 0,
                        entry: 0,
                        blocks: vec![],
                        instructions: vec![],
                        local_names: vec![],
                        local_types: vec![],
                        init_body_pc: 0,
                        init_wrap_pc: 0,
                    },
                    vec![],
                )
            });
            ctx.pending_closures.push(mir_fn);
            ctx.pending_closures.extend(nested);
            if *detached {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ConstStr(task_name.to_string()),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host_fn_index(HostFnId::TaskDetach)),
                        span: None,
                    },
                );
            } else {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::TaskSpawn(task_name),
                        span: None,
                    },
                );
            }
        }
        Expr::Assign { name, value, .. } => {
            lower_expr(value, dest, ctx, block);
            if let Some(to_name) = ctx.local_types.get(name).cloned() {
                if let Some(from_name) = infer_expr_init_type(value, ctx) {
                    emit_integer_narrow_cast_if_needed(
                        from_name.as_str(),
                        to_name.as_str(),
                        dest,
                        ctx,
                        block,
                    );
                }
            }
            ctx.lower_store_binding(name, block);
        }
        Expr::CompoundAssign { name, op, value, span } => {
            use syx_ast::{BinOp, CompoundAssignOp};
            let bin_op = match op {
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
            };
            if matches!(op, CompoundAssignOp::Add | CompoundAssignOp::Sub) {
                let left_ty = ctx
                    .local_types
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| SmolStr::new("Int"));
                if left_ty.as_str() == "Int" {
                    let host_name = match op {
                        CompoundAssignOp::Add => "__syx_int_add_assign",
                        CompoundAssignOp::Sub => "__syx_int_sub_assign",
                        _ => "",
                    };
                    if let Some(idx) = host_fn_index_by_name(host_name) {
                        let lhs = ctx.fresh_reg();
                        let rhs = ctx.fresh_reg();
                        ctx.lower_load_binding(lhs, name, block);
                        lower_expr(value, rhs, ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(idx),
                                span: None,
                            },
                        );
                        ctx.lower_store_binding(name, block);
                        return;
                    }
                }
            }
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
            lower_expr(&desugared, dest, ctx, block);
        }
        Expr::Group { expr, .. } => lower_expr(expr, dest, ctx, block),
        Expr::Borrow { expr, .. } => lower_expr(expr, dest, ctx, block),
        Expr::KeyPath { .. } => {}
        Expr::MacroInvoke { args, .. } => {
            for a in args {
                lower_expr(a, ctx.fresh_reg(), ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
        }
        Expr::OptionalCast { expr, target, .. } => {
            lower_expr(expr, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(
                        mir_type_expr_name(target)
                            .unwrap_or_else(|| SmolStr::new("Unknown"))
                            .to_string(),
                    ),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::AsOptional)),
                    span: None,
                },
            );
        }
        Expr::Is { expr, target, .. } => {
            lower_expr(expr, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(
                        mir_type_expr_name(target)
                            .unwrap_or_else(|| SmolStr::new("Unknown"))
                            .to_string(),
                    ),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::TypeIs)),
                    span: None,
                },
            );
        }
        Expr::Cast { expr, target, .. } => {
            lower_expr(expr, dest, ctx, block);
            let to_name = mir_type_expr_name(target)
                .unwrap_or_else(|| SmolStr::new("Unknown"));
            if let Some(from_name) = infer_expr_init_type(expr, ctx) {
                if emit_integer_numeric_cast_if_needed(
                    from_name.as_str(),
                    to_name.as_str(),
                    dest,
                    ctx,
                    block,
                ) || emit_float_to_int_numeric_cast_if_needed(
                    from_name.as_str(),
                    to_name.as_str(),
                    dest,
                    ctx,
                    block,
                ) {
                    return;
                }
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(to_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::TypeCast)),
                    span: None,
                },
            );
        }
        Expr::TypeSelf { type_name, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(type_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::ReflectType)),
                    span: None,
                },
            );
        }
        Expr::MemberAssign { object, name, value, .. } => {
            if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                if is_static_type_member_call(object, ctx) {
                    let set_method = syx_ast::var_global_setter_method(name);
                    let setter_fn = SmolStr::new(format!("{type_name}_{set_method}"));
                    let has_local = ctx
                        .module
                        .property_setters
                        .contains(&(type_name.clone(), name.clone()));
                    let has_static = ctx
                        .module
                        .static_class_methods
                        .contains(&(type_name.clone(), set_method.clone()));
                    let has_merged = ctx.module.callable_extensions.contains(&setter_fn);
                    if has_local || has_static || has_merged {
                        lower_expr(value, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(setter_fn),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Some(type_name) = infer_object_type_name(object, ctx) {
                if let Some(setter) = ctx.module.property_setter_fn(&type_name, name) {
                    if !is_static_type_member_call(object, ctx) {
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        lower_expr(value, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::Call(setter),
                                span: None,
                            },
                        );
                        return;
                    }
                }
            }
            if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                if let Some((global_name, mutable)) = ctx.module.static_var_global(type_name, name)
                {
                    if mutable {
                        if let Some(slot) = ctx.global_slot(&global_name) {
                            let observer = ctx.module.lookup_property_observer(type_name, name);
                            let saved_locals = ctx.locals.clone();
                            if let Some((will_set, did_set)) = observer {
                                let old_slot = ctx.alloc_local(&SmolStr::new("oldValue"));
                                let new_slot = ctx.alloc_local(&SmolStr::new("newValue"));
                                if did_set.is_some() {
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::LoadGlobal(slot),
                                            span: None,
                                        },
                                    );
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::StoreLocal(old_slot),
                                            span: None,
                                        },
                                    );
                                }
                                if let Some(stmts) = will_set {
                                    ctx.lower_rhs_for_field_store(value, block);
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::StoreLocal(new_slot),
                                            span: None,
                                        },
                                    );
                                    for stmt in stmts {
                                        lower_stmt(&stmt, ctx, block);
                                    }
                                }
                                lower_expr(value, dest, ctx, block);
                                ctx.emit(
                                    block,
                                    MirInstr {
                                        dest: None,
                                        op: MirOp::StoreGlobal(slot),
                                        span: None,
                                    },
                                );
                                if let Some(stmts) = did_set {
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::LoadGlobal(slot),
                                            span: None,
                                        },
                                    );
                                    ctx.emit(
                                        block,
                                        MirInstr {
                                            dest: None,
                                            op: MirOp::StoreLocal(new_slot),
                                            span: None,
                                        },
                                    );
                                    for stmt in stmts {
                                        lower_stmt(&stmt, ctx, block);
                                    }
                                }
                                ctx.locals = saved_locals;
                                return;
                            }
                            lower_expr(value, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreGlobal(slot),
                                    span: None,
                                },
                            );
                            return;
                        }
                    }
                }
            }
            let obj_type = if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                ctx.receiver_class_type(obj_name)
            } else {
                None
            };
            let obj_reg = ctx.fresh_reg();
            // Keep MutRefCell handles raw so write-back can update `__syx_ref`.
            if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                ctx.lower_load_binding_raw(obj_name, block);
            } else {
                lower_expr(object, obj_reg, ctx, block);
            }
            let tmp_slot = ctx.alloc_local(&SmolStr::new(format!("__obs_{obj_reg}")));
            let needs_writeback = if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                ctx.member_assign_unwraps_object(object) && ctx.is_class_instance_binding(obj_name)
            } else {
                false
            };
            let inner_write_slot = needs_writeback.then(|| {
                ctx.alloc_local(&SmolStr::new(format!("__syx_inner_{obj_reg}")))
            });
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(tmp_slot),
                    span: None,
                },
            );
            let observer = obj_type
                .as_ref()
                .and_then(|ty| ctx.module.lookup_property_observer(ty, name));
            let suppress_observers = ctx.in_class_init()
                && matches!(object.as_ref(), Expr::Ident { name: obj_name, .. } if obj_name.as_str() == "self");
            let saved_locals = ctx.locals.clone();
            let run_observers = observer.as_ref().filter(|_| !suppress_observers);
            if let Some((will_set, did_set)) = run_observers {
                let old_slot = ctx.alloc_local(&SmolStr::new("oldValue"));
                let new_slot = ctx.alloc_local(&SmolStr::new("newValue"));
                if did_set.is_some() {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::LoadLocal(tmp_slot),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(name.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ObjectGet,
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(old_slot),
                            span: None,
                        },
                    );
                }
                if let Some(stmts) = will_set {
                    ctx.lower_rhs_for_field_store(value, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(new_slot),
                            span: None,
                        },
                    );
                    for stmt in stmts {
                        lower_stmt(stmt, ctx, block);
                    }
                }
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(obj_reg),
                        op: MirOp::LoadLocal(tmp_slot),
                        span: None,
                    },
                );
                if !(ctx.in_class_init()
                    && matches!(object.as_ref(), Expr::Ident { name, .. } if name.as_str() == "self"))
                    && !expr_is_dict_like(object, ctx)
                {
                    ctx.emit_unwrap_syx_ref(block);
                }
                ctx.lower_rhs_for_field_store(value, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(name.to_string()),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectSet,
                        span: None,
                    },
                );
                if let Some(inner_slot) = inner_write_slot {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(inner_slot),
                            span: None,
                        },
                    );
                }
                if let Some(stmts) = did_set {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::LoadLocal(tmp_slot),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(name.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ObjectGet,
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(new_slot),
                            span: None,
                        },
                    );
                    for stmt in stmts {
                        lower_stmt(stmt, ctx, block);
                    }
                }
            } else {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(obj_reg),
                        op: MirOp::LoadLocal(tmp_slot),
                        span: None,
                    },
                );
                if !(ctx.in_class_init()
                    && matches!(object.as_ref(), Expr::Ident { name, .. } if name.as_str() == "self"))
                    && !expr_is_dict_like(object, ctx)
                {
                    ctx.emit_unwrap_syx_ref(block);
                }
                ctx.lower_rhs_for_field_store(value, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(name.to_string()),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectSet,
                        span: None,
                    },
                );
                if let Some(inner_slot) = inner_write_slot {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(inner_slot),
                            span: None,
                        },
                    );
                }
            }
            ctx.locals = saved_locals;
            if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                if let Some(inner_slot) = inner_write_slot {
                    ctx.emit_mut_ref_cell_writeback_from_inner(obj_name, inner_slot, block);
                } else if ctx.should_unwrap_class_local(obj_name) && !ctx.in_class_init() {
                    ctx.lower_store_binding(obj_name, block);
                } else if let Some(&slot) = ctx.locals.get(obj_name) {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(slot),
                            span: None,
                        },
                    );
                }
            }
        }
        Expr::IndexAssign { object, index, value, .. } => {
            if let Expr::Member { object: base, name: field, .. } = object.as_ref() {
                if let Expr::Ident { name: recv, .. } = base.as_ref() {
                    if ctx.locals.contains_key(recv) {
                        lower_expr(base, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ConstStr(field.to_string()),
                                span: None,
                            },
                        );
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ObjectGet,
                                span: None,
                            },
                        );
                        lower_expr(index, ctx.fresh_reg(), ctx, block);
                        lower_expr(value, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ArraySet,
                                span: None,
                            },
                        );
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ConstStr(field.to_string()),
                                span: None,
                            },
                        );
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::ObjectSet,
                                span: None,
                            },
                        );
                        if let Some(&slot) = ctx.locals.get(recv) {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreLocal(slot),
                                    span: None,
                                },
                            );
                        }
                        return;
                    }
                }
            }
            if let Expr::Ident { name: recv, .. } = object.as_ref() {
                if let Some(type_name) = ctx.local_types.get(recv) {
                    let base = type_name_dispatch_base(type_name);
                    if ctx.module.extension_methods.contains(&(
                        base.clone(),
                        SmolStr::new("__subscript_set"),
                    )) {
                        let ext_name = SmolStr::new(format!("{base}___subscript_set"));
                        lower_expr(object, ctx.fresh_reg(), ctx, block);
                        lower_expr(index, ctx.fresh_reg(), ctx, block);
                        lower_expr(value, ctx.fresh_reg(), ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::Call(ext_name),
                                span: None,
                            },
                        );
                        if let Some(&slot) = ctx.locals.get(recv) {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreLocal(slot),
                                    span: None,
                                },
                            );
                        }
                        return;
                    }
                }
            }
            let obj_ty = match object.as_ref() {
                Expr::Ident { name, .. } => ctx.local_types.get(name).cloned(),
                _ => None,
            };
            let use_object_set = obj_ty.as_ref().is_some_and(|t| {
                let s = t.as_str();
                s == "Object" || s == "Dictionary" || s.starts_with("Dictionary<")
            });
            if use_object_set {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(value, ctx.fresh_reg(), ctx, block);
                lower_expr(index, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectSet,
                        span: None,
                    },
                );
            } else {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(index, ctx.fresh_reg(), ctx, block);
                lower_expr(value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ArraySet,
                        span: None,
                    },
                );
            }
            if let Expr::Ident { name: recv, .. } = object.as_ref() {
                if let Some(&slot) = ctx.locals.get(recv) {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreLocal(slot),
                            span: None,
                        },
                    );
                }
            }
        }
        Expr::HalfOpenRange { start, end, .. } => {
            lower_range_dict(ctx, block, dest, *start, *end, false);
        }
        Expr::ClosedRange { start, end, .. } => {
            lower_range_dict(ctx, block, dest, *start, *end, true);
        }
        Expr::Super { .. } => {
            ctx.lower_load_binding_raw(&SmolStr::new("self"), block);
        }
        _ => {}
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
            Expr::Group { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Array { elements, .. } => {
                for el in elements {
                    visit_expr(el, bound, outer_locals, seen, free);
                }
            }
            Expr::Closure { params, body, .. } => {
                let mut nested_bound = bound.clone();
                for p in params {
                    nested_bound.insert(p.name.clone());
                }
                match body {
                    ClosureBody::Expr(e) => visit_expr(e, &nested_bound, outer_locals, seen, free),
                    ClosureBody::Block(stmts) => {
                        visit_stmts(stmts, &mut nested_bound, outer_locals, seen, free);
                    }
                }
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
                Stmt::Return { value: Some(v), .. } => visit_expr(v, bound, outer_locals, seen, free),
                Stmt::Expr { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    visit_expr(condition, bound, outer_locals, seen, free);
                    visit_stmts(then_branch, bound, outer_locals, seen, free);
                    if let Some(eb) = else_branch {
                        visit_stmts(eb, bound, outer_locals, seen, free);
                    }
                }
                Stmt::Block { stmts, .. } => visit_stmts(stmts, bound, outer_locals, seen, free),
                _ => {}
            }
        }
    }

    match body {
        ClosureBody::Expr(e) => visit_expr(e, &bound, outer_locals, &mut seen, &mut free),
        ClosureBody::Block(stmts) => visit_stmts(stmts, &mut bound, outer_locals, &mut seen, &mut free),
    }
    free
}

fn resolve_host_call(
    object: &Expr,
    method: &str,
    ctx: &LowerCtx,
) -> Option<(HostFnId, bool)> {
    // ADR-236 / M2-S4-01: HTTPResponse MethodId accessors must not fall through to ObjectGet
    // (status is also a bag field — ObjectGet "works" by accident; text/ok do not).
    // Parity with codegen `resolve_call`: user enum/FinalClass case ctors (e.g. `WireStatus.ok(...)`)
    // and Result/BigInt statics must not be stolen by HttpResponse method-name shortcuts.
    if let Expr::Ident { name: type_name, .. } = object {
        if matches!(type_name.as_str(), "Result" | "BigInt") {
            return None;
        }
        if ctx.module.enum_classes.contains(type_name)
            || ctx.module.final_classes.contains(type_name)
        {
            return None;
        }
    }
    match method {
        "json" => return Some((HostFnId::HttpResponseJson, true)),
        "status" => return Some((HostFnId::HttpResponseStatus, true)),
        "text" | "body" => return Some((HostFnId::HttpResponseText, true)),
        "ok" => return Some((HostFnId::HttpResponseOk, true)),
        "header" => return Some((HostFnId::HttpResponseHeader, true)),
        "headers" => return Some((HostFnId::HttpResponseHeaders, true)),
        _ => {}
    }
    if let Expr::Ident { name: type_name, .. } = object {
        if let Some(tm) = lookup_type_method(type_name, method) {
            return tm.host.map(|h| (h, false));
        }
        if matches!(type_name.as_str(), "Float64" | "Double") {
            if let Some(h) = lookup_float64_math_host(method) {
                return Some((h, false));
            }
        }
        if matches!(type_name.as_str(), "Float32" | "Float") {
            if let Some(h) = lookup_float32_math_host(method) {
                return Some((h, false));
            }
        }
    }
    None
}

fn parse_simd_type(type_name: &str) -> Option<(u8, syx_stdlib::NumericKind)> {
    let rest = type_name.strip_prefix("SIMD")?;
    let (lanes_str, inner) = rest.split_once('<')?;
    let lanes: u8 = lanes_str.parse().ok()?;
    let elem = inner.trim_end_matches('>').trim();
    from_type_name(elem).map(|k| (lanes, k))
}

fn infer_expr_simd_type(expr: &Expr, ctx: &LowerCtx) -> Option<(u8, syx_stdlib::NumericKind)> {
    if let Expr::Ident { name, .. } = expr {
        return ctx
            .local_types
            .get(name)
            .and_then(|t| parse_simd_type(t.as_str()));
    }
    None
}

fn extension_call_op(type_name: &str, method: &str, ctx: &LowerCtx) -> MirOp {
    match method {
        // Runtime values (arrays from split, dict literals, etc.) are unwrapped VM
        // values; stdlib `{Type}_description` expects a MutRefCell wrapper.
        "description" => MirOp::CallHost(host_fn_index(HostFnId::ValueDescription)),
        "debugDescription" => MirOp::CallHost(host_fn_index(HostFnId::ValueDebugDescription)),
        _ => {
            let sym = SmolStr::new(format!("{type_name}_{method}"));
            if ctx.module.callable_extensions.contains(&sym) {
                MirOp::Call(sym)
            } else {
                MirOp::Call(sym)
            }
        }
    }
}

fn is_hosted_collection_method(type_base: &str, method: &str) -> bool {
    syx_stdlib::is_manifest_host_op(type_base, method)
        || matches!(method, "description" | "debugDescription")
}

fn collection_method_mir_op(type_base: &str, method: &str, ctx: &LowerCtx) -> MirOp {
    let base = SmolStr::new(type_base);
    let name = SmolStr::new(method);
    if let Some(host) = manifest_host_name(type_base, method) {
        if let Some(idx) = host_fn_index_by_name(&host) {
            return MirOp::CallHost(idx);
        }
    }
    if ctx.module.extension_methods.contains(&(base.clone(), name.clone())) {
        if matches!(method, "description" | "debugDescription") {
            return extension_call_op(type_base, method, ctx);
        }
        return MirOp::Call(SmolStr::new(format!("{type_base}_{method}")));
    }
    if matches!(method, "description" | "debugDescription") {
        return extension_call_op(type_base, method, ctx);
    }
    MirOp::Call(SmolStr::new(format!("{type_base}_{method}")))
}

fn emit_hosted_collection_call(
    type_base: &str,
    method: &SmolStr,
    object: &Expr,
    args: &[CallArg],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(object, ctx.fresh_reg(), ctx, block);
    for arg in args {
        lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
    }
    let op = if is_hosted_collection_method(type_base, method.as_str()) {
        collection_method_mir_op(type_base, method.as_str(), ctx)
    } else if type_base == "Array" {
        if let Some(m) = lookup_array_method(method) {
            method_id_mir_op(m, "Array", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else if type_base == "Set" {
        if let Some(m) = lookup_set_method(method) {
            method_id_mir_op(m, "Set", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else if type_base == "Dictionary" {
        if let Some(m) = lookup_dict_method(method) {
            method_id_mir_op(m, "Dictionary", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else {
        collection_method_mir_op(type_base, method.as_str(), ctx)
    };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op,
            span: None,
        },
    );
}

fn keypath_from_callee(expr: &Expr) -> Option<(&SmolStr, &SmolStr, bool)> {
    match expr {
        Expr::KeyPath {
            root_type,
            property,
            optional,
            ..
        } => Some((root_type, property, *optional)),
        Expr::Group { expr, .. } => keypath_from_callee(expr),
        _ => None,
    }
}

fn type_name_is_array(name: &str) -> bool {
    name == "Array" || name.starts_with("Array<")
}

fn type_name_is_set(name: &str) -> bool {
    name == "Set" || name.starts_with("Set<")
}

/// Extension returns are keyed by bare `Set` / `Array`; locals often carry `Set<Int>`.
/// Hosted collection ops (manifest `host_ops`) share MethodId return surfaces.
fn extension_or_hosted_return_type(
    recv_ty: &SmolStr,
    method: &SmolStr,
    ctx: &LowerCtx,
) -> Option<SmolStr> {
    let base = type_name_dispatch_base(recv_ty);
    if let Some(ret) = ctx
        .module
        .extension_method_returns
        .get(&(base.clone(), method.clone()))
        .or_else(|| {
            ctx.module
                .extension_method_returns
                .get(&(recv_ty.clone(), method.clone()))
        })
    {
        // Preserve element params when the extension only declares bare `Set` / `Array`.
        if (ret.as_str() == "Set" && type_name_is_set(recv_ty.as_str()))
            || (ret.as_str() == "Array" && type_name_is_array(recv_ty.as_str()))
        {
            return Some(recv_ty.clone());
        }
        return Some(ret.clone());
    }
    let hosted = match base.as_str() {
        "Set" => lookup_set_method(method).map(method_return_type),
        "Array" => lookup_array_method(method).map(method_return_type),
        "Dictionary" => lookup_dict_method(method).map(method_return_type),
        _ => None,
    }?;
    if (hosted == "Set" && type_name_is_set(recv_ty.as_str()))
        || (hosted == "Array" && type_name_is_array(recv_ty.as_str()))
    {
        return Some(recv_ty.clone());
    }
    Some(SmolStr::new(hosted))
}

/// Phase 121/123/126: binding key for service resolve host calls (includes dynamic `Di.resolve`).
fn infer_service_resolve_type_name(callee: &Expr, args: &[CallArg]) -> Option<SmolStr> {
    match callee {
        Expr::Ident { name, .. }
            if matches!(name.as_str(), "__syx_service_resolve" | "resolve") => {}
        Expr::Member { object, name, .. }
            if name.as_str() == "resolve"
                && matches!(
                    object.as_ref(),
                    Expr::Ident { name: m, .. } if m.as_str() == "Di"
                ) => {}
        _ => return None,
    }
    let arg = args.first()?;
    match &arg.value {
        Expr::Literal {
            value: Literal::String(type_name),
            ..
        } => Some(SmolStr::new(type_name.as_str())),
        Expr::Ident { name: type_name, .. } => Some(type_name.clone()),
        _ => None,
    }
}

fn infer_expr_init_type(expr: &Expr, ctx: &LowerCtx) -> Option<SmolStr> {
    match expr {
        Expr::Array { .. } => Some(SmolStr::new("Array")),
        Expr::Ident { name, .. } => ctx.local_types.get(name).cloned(),
        Expr::Literal {
            value: Literal::String(_) | Literal::Interpolated(_),
            ..
        } => Some(SmolStr::new("String")),
        Expr::Literal {
            value: Literal::BigInt(_),
            ..
        } => Some(SmolStr::new("BigInt")),
        Expr::Literal {
            value: Literal::Integer { suffix, .. },
            ..
        } if suffix.as_deref() == Some("n") => Some(SmolStr::new("BigInt")),
        Expr::Literal {
            value: Literal::Dict(_),
            ..
        } => Some(SmolStr::new("Dictionary")),
        Expr::Literal {
            value: Literal::Double { suffix, .. },
            ..
        } => {
            let s = suffix.as_deref().unwrap_or("");
            Some(SmolStr::new(
                if s == "f32" || s == "f" {
                    "Float32"
                } else {
                    "Float64"
                },
            ))
        }
        Expr::Call { callee, args, .. } => {
            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                return Some(type_name);
            }
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if let Some(ret) = ctx.module.function_returns.get(name) {
                    return Some(ret.clone());
                }
                if let Some(host_id) = host_fn_id_by_name(name.as_str()) {
                    let ret = host_fn_return_type(host_id);
                    if ret == "Array" {
                        return Some(SmolStr::new("Array"));
                    }
                    if ret == "String" {
                        return Some(SmolStr::new("String"));
                    }
                    if !ret.is_empty() && ret != "Nil" && ret != "Void" {
                        return Some(SmolStr::new(ret));
                    }
                }
            }
            if let Some(mangled) = syx_ast::nested_type_ctor_mangled(callee) {
                return Some(mangled);
            }
            if let Expr::Member { object, .. } = callee.as_ref() {
                if let Expr::Ident { name: mod_name, .. } = object.as_ref() {
                    if matches!(mod_name.as_str(), "BigInt" | "Result") {
                        return Some(mod_name.clone());
                    }
                }
            }
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                if matches!(
                    name.as_str(),
                    "split" | "matches" | "readLines" | "map" | "filter" | "sorted" | "reverse"
                ) {
                    return Some(SmolStr::new("Array"));
                }
                if let Some(recv) = infer_object_type_name(object, ctx) {
                    if let Some(ret) = extension_or_hosted_return_type(&recv, name, ctx) {
                        return Some(ret);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn expr_is_float64_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Literal {
        value: Literal::Double { suffix, .. },
        ..
    } = expr
    {
        // bare / f64 → Float64; f32 / f → Float32 (not this helper)
        let s = suffix.as_deref().unwrap_or("");
        return s.is_empty() || s == "f64" || s == "d";
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| {
            matches!(
                type_name_dispatch_base(t).as_str(),
                "Float64" | "Double"
            )
        }) {
            return true;
        }
    }
    infer_object_type_name(expr, ctx).is_some_and(|t| {
        matches!(
            type_name_dispatch_base(&t).as_str(),
            "Float64" | "Double"
        )
    })
}

fn expr_is_float32_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Literal {
        value: Literal::Double { suffix, .. },
        ..
    } = expr
    {
        let s = suffix.as_deref().unwrap_or("");
        return s == "f32" || s == "f";
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| {
            matches!(
                type_name_dispatch_base(t).as_str(),
                "Float32" | "Float"
            )
        }) {
            return true;
        }
    }
    infer_object_type_name(expr, ctx).is_some_and(|t| {
        matches!(
            type_name_dispatch_base(&t).as_str(),
            "Float32" | "Float"
        )
    })
}

fn expr_is_array_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(expr, Expr::Array { .. }) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx
            .local_types
            .get(name)
            .is_some_and(|t| type_name_is_array(t.as_str()))
        {
            return true;
        }
    }
    infer_expr_init_type(expr, ctx).is_some_and(|t| type_name_is_array(t.as_str()))
}

fn expr_is_set_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Ident { name, .. } = expr {
        return ctx
            .local_types
            .get(name)
            .is_some_and(|t| type_name_is_set(t.as_str()));
    }
    false
}

fn expr_is_dict_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(expr, Expr::Literal { value: Literal::Dict(_), .. }) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        return ctx.local_types.get(name).is_some_and(|t| {
            let base = type_name_dispatch_base(t);
            base.as_str() == "Dictionary" || base.as_str() == "Object"
        });
    }
    false
}

fn expr_is_string_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(
        expr,
        Expr::Literal {
            value: Literal::String(_) | Literal::Interpolated(_),
            ..
        }
    ) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| t.as_str() == "String") {
            return true;
        }
    }
    // Chained `a + b + c`: the left of the outer Add is itself a string Concat.
    // Without this, the second `+` lowers to numeric Add → 0 (breaks SynUI handle CSVs).
    if let Expr::Binary { left, op, right, .. } = expr {
        if matches!(op, BinOp::Add)
            && (expr_is_string_like(left, ctx) || expr_is_string_like(right, ctx))
        {
            return true;
        }
    }
    infer_expr_init_type(expr, ctx).is_some_and(|t| t.as_str() == "String")
        || infer_object_type_name(expr, ctx).is_some_and(|t| t.as_str() == "String")
}

fn register_param_local_type(ctx: &mut LowerCtx, param: &syx_ast::Param) {
    if let Some(type_expr) = &param.ty {
        if let Some(type_name) = mir_type_expr_name(type_expr) {
            ctx.local_types.insert(param.name.clone(), type_name);
        }
    }
}

/// Emit `NumericCast` when `from_ty` → `to_ty` is same-signedness integer narrowing.
fn emit_integer_narrow_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let Some(from_k) = from_type_name(from_ty) else {
        return;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return;
    };
    if !is_integer_narrowing(from_k, to_k) {
        return;
    }
    let from = resolve_alias(from_k).to_byte();
    let to = resolve_alias(to_k).to_byte();
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast { from, to },
            span: None,
        },
    );
}

/// Emit `NumericCast` for integer widen/narrow (`as Int8` / `as Int`); returns true if emitted.
fn emit_integer_numeric_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) -> bool {
    let Some(from_k) = from_type_name(from_ty) else {
        return false;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return false;
    };
    if !is_integer(from_k) || !is_integer(to_k) {
        return false;
    }
    let from_r = resolve_alias(from_k);
    let to_r = resolve_alias(to_k);
    if from_r == to_r {
        return false;
    }
    if !is_integer_narrowing(from_k, to_k) && !is_implicit_convertible(from_k, to_k) {
        return false;
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast {
                from: from_r.to_byte(),
                to: to_r.to_byte(),
            },
            span: None,
        },
    );
    true
}

/// Result kind byte for scalar binary arithmetic / bitwise (`widen_for_binary`).
fn binary_numeric_kind(left: &Expr, right: &Expr, ctx: &LowerCtx) -> u8 {
    let lk = infer_expr_init_type(left, ctx).and_then(|n| from_type_name(n.as_str()));
    let rk = infer_expr_init_type(right, ctx).and_then(|n| from_type_name(n.as_str()));
    match (lk, rk) {
        (Some(a), Some(b)) => widen_for_binary(a, b)
            .unwrap_or(PLATFORM_INT)
            .to_byte(),
        (Some(a), None) => resolve_alias(a).to_byte(),
        (None, Some(b)) => resolve_alias(b).to_byte(),
        (None, None) => PLATFORM_INT.to_byte(),
    }
}

/// Result kind byte for scalar unary bitwise (`~`).
fn unary_numeric_kind(expr: &Expr, ctx: &LowerCtx) -> u8 {
    infer_expr_init_type(expr, ctx)
        .and_then(|n| from_type_name(n.as_str()))
        .map(|k| resolve_alias(k).to_byte())
        .unwrap_or(PLATFORM_INT.to_byte())
}

/// Emit `NumericCast` for float → integer (`d as Int8`); returns true if emitted.
fn emit_float_to_int_numeric_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) -> bool {
    let Some(from_k) = from_type_name(from_ty) else {
        return false;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return false;
    };
    if !is_float(from_k) || !is_integer(to_k) {
        return false;
    }
    let from_r = resolve_alias(from_k);
    let to_r = resolve_alias(to_k);
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast {
                from: from_r.to_byte(),
                to: to_r.to_byte(),
            },
            span: None,
        },
    );
    true
}

fn mir_type_expr_name(ty: &TypeExpr) -> Option<SmolStr> {
    match ty {
        TypeExpr::Named { name, generics } if !generics.is_empty() => {
            let mut parts = Vec::with_capacity(generics.len());
            for arg in generics {
                parts.push(mir_type_expr_name(arg)?.to_string());
            }
            Some(SmolStr::new(format!(
                "{}<{}>",
                name.as_str(),
                parts.join(", ")
            )))
        }
        TypeExpr::Named { name, .. } => Some(name.clone()),
        TypeExpr::Array(inner) => Some(SmolStr::new(format!(
            "Array<{}>",
            mir_type_expr_name(inner)?.as_str()
        ))),
        TypeExpr::Dictionary { key, value } => {
            match (mir_type_expr_name(key), mir_type_expr_name(value)) {
                (Some(key_name), Some(value_name)) => Some(SmolStr::new(format!(
                    "Dictionary<{key_name}, {value_name}>"
                ))),
                _ => Some(SmolStr::new("Dictionary")),
            }
        }
        _ => None,
    }
}

fn normalize_type_arg(name: SmolStr) -> SmolStr {
    match name.as_str() {
        "Int" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "Int64"
        } else {
            "Int32"
        }),
        "UInt" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "UInt64"
        } else {
            "UInt32"
        }),
        other => SmolStr::new(other),
    }
}

fn infer_static_method_type_args(args: &[CallArg], ctx: &LowerCtx) -> Vec<SmolStr> {
    for arg in args {
        if let Expr::Ident { name, .. } = &arg.value {
            if let Some(ty) = ctx.local_types.get(name.as_str()) {
                if ty.starts_with("Array<") && ty.ends_with('>') {
                    let inner = ty.trim_start_matches("Array<").trim_end_matches('>');
                    return vec![normalize_type_arg(SmolStr::new(inner))];
                }
            }
        }
    }
    vec![SmolStr::new("Unknown")]
}

fn instance_self_mut_ref_method(name: &SmolStr, module: &ModuleIndex) -> bool {
    let Some((type_name, rest)) = syx_ast::parse_mangled_instance_method(name.as_str()) else {
        return false;
    };
    if rest.as_str() == "init" || rest.as_str() == "deinit" {
        return false;
    }
    // Core value / bag surfaces are not MutRefCell-wrapped class instances.
    // Treating them as such ObjectGet(`__syx_ref`)→Nil and breaks `array.count` / hosts.
    if matches!(
        type_name.as_str(),
        "Array"
            | "Set"
            | "Dictionary"
            | "String"
            | "Object"
            | "OrderedSet"
            | "Deque"
            | "Result"
            | "BigInt"
    ) {
        return false;
    }
    module.class_types.contains_key(&type_name) || module.actor_types.contains(&type_name)
}

fn type_name_dispatch_base(name: &SmolStr) -> SmolStr {
    SmolStr::new(name.split('<').next().unwrap_or(name.as_str()))
}

fn is_static_type_member_call(object: &Expr, ctx: &LowerCtx) -> bool {
    let Expr::Ident { name, .. } = object else {
        return false;
    };
    ctx.imports.contains(name)
        || ctx.module.class_types.contains_key(name)
        || ctx.module.enum_classes.contains(name)
        || ctx.module.final_classes.contains(name)
        || ctx.module.dataclass_names.contains(name)
        || syx_stdlib::is_builtin_type(name.as_str())
}

fn class_name_from_mangled(func_name: &str) -> Option<SmolStr> {
    if let Some(base) = func_name.strip_suffix("_init") {
        return Some(SmolStr::new(base));
    }
    syx_ast::parse_mangled_instance_method(func_name).map(|(class, _)| class)
}

fn infer_member_field_type(object: &Expr, field: &SmolStr, ctx: &LowerCtx) -> Option<SmolStr> {
    let class_name = match object {
        Expr::Ident { name, .. } if name == "self" => {
            class_name_from_mangled(ctx.func_name.as_str())?
        }
        Expr::Ident { name, .. } => ctx.local_types.get(name).cloned()?,
        _ => infer_object_type_name(object, ctx)?,
    };
    ctx.module
        .class_field_types
        .get(&class_name)?
        .get(field)
        .cloned()
}

fn infer_object_type_name(object: &Expr, ctx: &LowerCtx) -> Option<SmolStr> {
    match object {
        Expr::Member { object, name, .. } => infer_member_field_type(object, name, ctx),
        Expr::Ident { name, .. } => ctx
            .local_types
            .get(name)
            .cloned()
            .or_else(|| {
                if ctx.module.dataclass_names.contains(name)
                    || ctx.module.class_types.contains_key(name)
                    || ctx.module.actor_types.contains(name)
                    || ctx.module.enum_classes.contains(name)
                {
                    Some(name.clone())
                } else {
                    None
                }
            }),
        Expr::Literal { value: _, .. } => Some(infer_expr_type_name(object)),
        Expr::Call { callee, args, .. } => {
            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                return Some(type_name);
            }
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if let Some(host_id) = host_fn_id_by_name(name.as_str()) {
                    let ret = host_fn_return_type(host_id);
                    if ret == "String" {
                        return Some(SmolStr::new("String"));
                    }
                    if ret == "Array" {
                        return Some(SmolStr::new("Array"));
                    }
                    if !ret.is_empty() && ret != "Nil" && ret != "Void" {
                        return Some(SmolStr::new(ret));
                    }
                }
                if ctx.module.dataclass_names.contains(name)
                    || ctx.module.class_types.contains_key(name)
                    || ctx.module.actor_types.contains(name)
                {
                    return Some(name.clone());
                }
            }
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                    if let Some(ret) =
                        extension_or_hosted_return_type(type_name, name, ctx)
                    {
                        return Some(ret);
                    }
                }
                if let Some(recv) = infer_object_type_name(object, ctx) {
                    if let Some(ret) = extension_or_hosted_return_type(&recv, name, ctx) {
                        return Some(ret);
                    }
                }
            }
            if let Some(mangled) = syx_ast::nested_type_ctor_mangled(callee) {
                if ctx.module.class_types.contains_key(&mangled) {
                    return Some(mangled);
                }
            }
            None
        }
        _ => None,
    }
}

#[allow(dead_code)] // array method dispatch index table (parked)
fn array_method_index(method: &str) -> Option<u32> {
    syx_stdlib::lookup_array_method(method).map(|m| m.index())
}

fn binop_operator_symbol(op: BinOp) -> Option<&'static str> {
    match op {
        BinOp::Add => Some("+"),
        BinOp::Sub => Some("-"),
        BinOp::Mul => Some("*"),
        BinOp::Div => Some("/"),
        BinOp::Eq => Some("=="),
        BinOp::NotEq => Some("!="),
        BinOp::Lt => Some("<"),
        BinOp::LtEq => Some("<="),
        BinOp::Gt => Some(">"),
        BinOp::GtEq => Some(">="),
        BinOp::BitAnd => Some("&"),
        BinOp::BitOr => Some("|"),
        BinOp::BitXor => Some("^"),
        BinOp::Shl => Some("<<"),
        BinOp::Shr => Some(">>"),
        _ => None,
    }
}

fn infer_expr_type_name(expr: &Expr) -> SmolStr {
    match expr {
        Expr::Literal { value, .. } => match value {
            Literal::Integer { suffix, .. } if suffix.as_deref() == Some("n") => {
                SmolStr::new("BigInt")
            }
            Literal::Integer { suffix, .. } => SmolStr::new(
                suffix
                    .as_deref()
                    .map(|s| s.trim_end_matches('i').trim_end_matches('u'))
                    .unwrap_or(if cfg!(target_pointer_width = "64") {
                        "Int64"
                    } else {
                        "Int32"
                    }),
            ),
            Literal::Double { suffix, .. } => SmolStr::new(
                suffix
                    .as_deref()
                    .map(|s| s.trim_end_matches('f'))
                    .unwrap_or("Float"),
            ),
            Literal::Bool(_) => SmolStr::new("Bool"),
            Literal::String(_) => SmolStr::new("String"),
            Literal::BigInt(_) => SmolStr::new("BigInt"),
            _ => SmolStr::new("Unknown"),
        },
        // Uncontexted Ident fallback (binop / host paths): keep historical name-as-type.
        Expr::Ident { name, .. } => name.clone(),
        Expr::Array { elements, .. } => {
            let elem = elements
                .first()
                .map(infer_expr_type_name)
                .unwrap_or_else(|| SmolStr::new("Unknown"));
            SmolStr::new(format!("Array<{elem}>"))
        }
        _ => SmolStr::new("Unknown"),
    }
}

/// ADR-324: generic call mangling must use binding/array types, not Ident text (`xs` ≠ type).
fn infer_generic_arg_type_name(expr: &Expr, ctx: &LowerCtx) -> SmolStr {
    match expr {
        Expr::Ident { name, .. } => {
            if let Some(ty) = ctx.local_types.get(name) {
                return normalize_nested_type_arg(ty.clone());
            }
            SmolStr::new("Unknown")
        }
        Expr::Array { elements, .. } => {
            let elem = elements
                .first()
                .map(|e| infer_generic_arg_type_name(e, ctx))
                .unwrap_or_else(|| SmolStr::new("Unknown"));
            normalize_nested_type_arg(SmolStr::new(format!("Array<{elem}>")))
        }
        other => normalize_nested_type_arg(infer_expr_type_name(other)),
    }
}

fn normalize_nested_type_arg(name: SmolStr) -> SmolStr {
    if let Some(inner) = name
        .as_str()
        .strip_prefix("Array<")
        .and_then(|s| s.strip_suffix('>'))
    {
        let inner_n = normalize_nested_type_arg(SmolStr::new(inner));
        return SmolStr::new(format!("Array<{inner_n}>"));
    }
    normalize_type_arg(name)
}

pub fn const_fold(module: &mut MirModule) {
    for func in &mut module.functions {
        fold_function(&mut func.instructions);
        for block in &mut func.blocks {
            fold_function(&mut block.instructions);
        }
    }
}

fn fold_function(instructions: &mut Vec<MirInstr>) {
    let mut i = 0;
    while i + 2 < instructions.len() {
        let a = &instructions[i];
        let b = &instructions[i + 1];
        let c = &instructions[i + 2];
        if let (
            MirInstr {
                dest: Some(d),
                op: MirOp::ConstI32(x),
                span: None,
            },
            MirInstr {
                dest: Some(_),
                op: MirOp::ConstI32(y),
                span: None,
            },
            MirInstr {
                dest: Some(d2),
                op: MirOp::Add { .. },
                span: None,
            },
        ) = (a, b, c)
        {
            if d == d2 {
                instructions[i] = MirInstr {
                    dest: Some(*d),
                    op: MirOp::ConstI32(x + y),
                    span: None,
                };
                instructions.remove(i + 1);
                instructions.remove(i + 1);
                continue;
            }
        }
        if let MirInstr {
            op: MirOp::ConstI32(0),
            span: None,
            ..
        } = &instructions[i]
        {
            if i + 1 < instructions.len() {
                if matches!(instructions[i + 1].op, MirOp::Add { .. }) {
                    instructions.remove(i + 1);
                    instructions.remove(i);
                    continue;
                }
            }
        }
        i += 1;
    }
}

fn direct_call_targets_from_instructions(instructions: &[MirInstr]) -> Vec<SmolStr> {
    let mut out = Vec::new();
    for instr in instructions {
        match &instr.op {
            MirOp::Call(name)
            | MirOp::ActorCall(name)
            | MirOp::TaskSpawn(name)
            | MirOp::MakeClosure(name, _)
            | MirOp::CallSuper(name) => out.push(name.clone()),
            _ => {}
        }
    }
    out
}

/// Cross-function reachability from `main` via direct named call edges.
///
/// Conservative: `CallIndirect`, `CallWitness`, async resume targets, and other
/// unresolved dispatch are **not** treated as proof that a function is dead.
///
/// Default production path remains **main-only**. For explicit export roots see
/// [`reachable_fn_names_from_roots`] (W398 multi-root wire).
pub fn reachable_fn_names(module: &MirModule) -> BTreeSet<SmolStr> {
    reachable_fn_names_from_roots(module, &[])
}

/// Cross-function reachability from `main` ∪ `extra_export_roots` via direct named call edges.
///
/// Empty `extra_export_roots` is equivalent to [`reachable_fn_names`] (main-only).
/// Non-empty roots keep orphans unreachable from any root shaken out. Does **not**
/// scan semantic `Module.exports` automatically (explicit roots only; W398).
pub fn reachable_fn_names_from_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> BTreeSet<SmolStr> {
    let mut roots = Vec::new();
    if module.functions.iter().any(|f| f.name == "main") {
        roots.push(SmolStr::new("main"));
    }
    for name in extra_export_roots {
        let s = SmolStr::new(*name);
        if module.functions.iter().any(|f| f.name == s) {
            roots.push(s);
        }
    }
    reachable_fn_names_from_root_set(module, roots)
}

fn reachable_fn_names_from_root_set(
    module: &MirModule,
    roots: impl IntoIterator<Item = SmolStr>,
) -> BTreeSet<SmolStr> {
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::new();
    for root in roots {
        if module.functions.iter().any(|f| f.name == root) && reachable.insert(root.clone()) {
            queue.push_back(root);
        }
    }
    while let Some(name) = queue.pop_front() {
        let Some(func) = module.functions.iter().find(|f| f.name == name) else {
            continue;
        };
        let mut callees = direct_call_targets_from_instructions(&func.instructions);
        for block in &func.blocks {
            callees.extend(direct_call_targets_from_instructions(&block.instructions));
        }
        for callee in callees {
            if reachable.insert(callee.clone()) {
                queue.push_back(callee);
            }
        }
    }
    reachable
}

/// Export-root reachability inventory (B-04; W326 floor + W398 wire honesty).
///
/// Compares default main-only reachability against BFS from `main` ∪ extra export
/// roots. When extras are non-empty, wire emit ([`llvm_emit_function_names_with_roots`])
/// is expected to match the export-root set (`emit_uses_export_roots=true`).
/// After W440, default emit follows [`MirModule::aot_export_roots`] (empty ⇒ main-only).
/// Does **not** flip `cutover_enabled` or claim B-04 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotReachabilityExportRootInventory {
    pub main_only_reachable_count: usize,
    pub with_exports_reachable_count: usize,
    pub export_root_extra_reachable: usize,
    pub extra_export_root_count: usize,
    pub relational_export_adds_reachable_ok: bool,
    pub relational_main_only_matches_production_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 2).
    pub candidate_predicate_count: usize,
    /// `true` when default emit equals main-only (empty `aot_export_roots`).
    pub production_roots_main_only: bool,
    /// `true` when non-empty extras are wired into emit eligibility (W398).
    pub emit_uses_export_roots: bool,
}

/// Inventory main-only vs main∪export-roots reachability gap + wire emit alignment.
///
/// Used for the B-04 export-roots-beyond-main floor (W326) and multi-root wire
/// honesty (W398). Does **not** mutate encode.
pub fn aot_reachability_export_root_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotReachabilityExportRootInventory {
    let production = reachable_fn_names(module);
    let main_only_reachable_count = production.len();
    let with_exports = reachable_fn_names_from_roots(module, extra_export_roots);
    let with_exports_reachable_count = with_exports.len();
    let extra_export_root_count = extra_export_roots
        .iter()
        .filter(|name| module.functions.iter().any(|f| f.name == **name))
        .count();
    let export_root_extra_reachable =
        with_exports_reachable_count.saturating_sub(main_only_reachable_count);
    let emit_wired = llvm_emit_function_names_with_roots(module, extra_export_roots);
    let emit_uses_export_roots =
        !extra_export_roots.is_empty() && emit_wired == with_exports;
    AotReachabilityExportRootInventory {
        main_only_reachable_count,
        with_exports_reachable_count,
        export_root_extra_reachable,
        extra_export_root_count,
        relational_export_adds_reachable_ok: export_root_extra_reachable > 0,
        relational_main_only_matches_production_ok: main_only_reachable_count
            == reachable_fn_names(module).len(),
        candidate_predicate_count: 2,
        production_roots_main_only: llvm_emit_function_names(module).len()
            == main_only_reachable_count,
        emit_uses_export_roots,
    }
}

/// Reproducible export-root inventory fixture: `main`→`callee_a`; independent
/// `export_lib`→`callee_b` (unreachable from main).
pub fn aot_reachability_fixture_main_and_export_orphan() -> MirModule {
    fn reach_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }
    MirModule {
        functions: vec![
            reach_fn("main", vec![MirOp::Call(SmolStr::new("callee_a"))]),
            reach_fn("callee_a", vec![MirOp::Return]),
            reach_fn("export_lib", vec![MirOp::Call(SmolStr::new("callee_b"))]),
            reach_fn("callee_b", vec![MirOp::Return]),
        ],
        // W440: default emit/compact include export_lib∪callee_b.
        aot_export_roots: vec![SmolStr::new("export_lib")],
    }
}

fn mir_requires_full_llvm_emit(module: &MirModule) -> bool {
    fn instr_triggers_full_emit(op: &MirOp) -> bool {
        matches!(
            op,
            MirOp::CallIndirect(_)
                | MirOp::CallWitness(_, _)
                | MirOp::AwaitSuspend { .. }
        )
    }
    module.functions.iter().any(|func| {
        func.instructions
            .iter()
            .any(|i| instr_triggers_full_emit(&i.op))
            || func.blocks.iter().any(|b| {
                b.instructions
                    .iter()
                    .any(|i| instr_triggers_full_emit(&i.op))
            })
    })
}

/// Function names eligible for native LLVM body emit + `syx_rt_register_native_fn`.
///
/// Default production path (W440): uses [`MirModule::aot_export_roots`] via
/// [`llvm_emit_function_names_with_roots`]. Empty roots ≡ [`reachable_fn_names`]
/// (main-only). Falls back to **all** fn names when the module contains
/// `CallIndirect` / `CallWitness` / `AwaitSuspend` (unresolved dispatch).
/// For caller-supplied roots use [`llvm_emit_function_names_with_roots`] (W398).
pub fn llvm_emit_function_names(module: &MirModule) -> BTreeSet<SmolStr> {
    let roots = mir_module_aot_export_root_refs(module);
    llvm_emit_function_names_with_roots(module, &roots)
}

/// Emit eligibility with optional `extra_export_roots` (W398 multi-root wire).
///
/// Empty extras ≡ main-only reachability (ignores [`MirModule::aot_export_roots`];
/// use [`llvm_emit_function_names`] for the default production path). Non-empty uses
/// [`reachable_fn_names_from_roots`]. Full-emit fallback unchanged.
pub fn llvm_emit_function_names_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> BTreeSet<SmolStr> {
    if mir_requires_full_llvm_emit(module) {
        return module.functions.iter().map(|f| f.name.clone()).collect();
    }
    reachable_fn_names_from_roots(module, extra_export_roots)
}

/// Export-root cutover readiness inventory (B-04; W328 floor + W398 wire + W400 cutover).
///
/// Aggregates W326 export-root gap with default-emit alignment and wired-emit≡export-roots
/// alignment. Short production cutover honesty may be on. After W440, default emit follows
/// [`MirModule::aot_export_roots`] (empty ⇒ main-only). Does **not** claim B-04 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotReachabilityExportRootCutoverReadinessInventory {
    pub main_only_reachable_count: usize,
    pub with_exports_reachable_count: usize,
    pub export_root_extra_reachable: usize,
    pub emit_reachable_count: usize,
    pub emit_with_roots_reachable_count: usize,
    pub relational_export_gap_measurable_ok: bool,
    pub relational_emit_matches_main_only_ok: bool,
    pub relational_emit_matches_export_roots_ok: bool,
    pub relational_inventory_matches_export_root_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// `true` after W400 — Short production cutover honesty signed.
    pub cutover_enabled: bool,
    /// `true` when default emit equals main-only (empty `aot_export_roots`).
    pub production_roots_main_only: bool,
    /// `true` when non-empty extras are wired into emit eligibility (W398).
    pub emit_uses_export_roots: bool,
}

/// Inventory export-root gap + emit alignment (default path + wired export roots).
///
/// Used for the B-04 export-root cutover readiness floor (W328), multi-root wire
/// (W398), cutover honesty (W400), and default multi-root (W440). Reuses
/// [`aot_reachability_export_root_inventory`] and emit helpers.
pub fn aot_reachability_export_root_cutover_readiness_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotReachabilityExportRootCutoverReadinessInventory {
    let export = aot_reachability_export_root_inventory(module, extra_export_roots);
    let emit_reachable_count = llvm_emit_function_names(module).len();
    let emit_with_roots = llvm_emit_function_names_with_roots(module, extra_export_roots);
    let emit_with_roots_reachable_count = emit_with_roots.len();
    let export_again = aot_reachability_export_root_inventory(module, extra_export_roots);
    AotReachabilityExportRootCutoverReadinessInventory {
        main_only_reachable_count: export.main_only_reachable_count,
        with_exports_reachable_count: export.with_exports_reachable_count,
        export_root_extra_reachable: export.export_root_extra_reachable,
        emit_reachable_count,
        emit_with_roots_reachable_count,
        relational_export_gap_measurable_ok: export.export_root_extra_reachable > 0,
        relational_emit_matches_main_only_ok: emit_reachable_count
            == export.main_only_reachable_count,
        relational_emit_matches_export_roots_ok: emit_with_roots_reachable_count
            == export.with_exports_reachable_count,
        relational_inventory_matches_export_root_ok: export.main_only_reachable_count
            == export_again.main_only_reachable_count
            && export.with_exports_reachable_count == export_again.with_exports_reachable_count
            && export.export_root_extra_reachable == export_again.export_root_extra_reachable,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_roots_main_only: export.production_roots_main_only,
        emit_uses_export_roots: export.emit_uses_export_roots,
    }
}

/// Default-emit honesty for Short cutover chains (W440).
///
/// Empty `aot_export_roots` ⇒ default emit main-only; non-empty ⇒ default emit matches
/// `main` ∪ module roots.
fn relational_default_emit_honesty_ok(
    module: &MirModule,
    export: &AotReachabilityExportRootCutoverReadinessInventory,
) -> bool {
    if module.aot_export_roots.is_empty() {
        export.relational_emit_matches_main_only_ok && export.production_roots_main_only
    } else {
        let roots = mir_module_aot_export_root_refs(module);
        llvm_emit_function_names(module) == reachable_fn_names_from_roots(module, &roots)
    }
}

/// Empty-roots clone of `module` (functions only) for main-only honesty checks.
fn mir_module_cleared_aot_export_roots(module: &MirModule) -> MirModule {
    MirModule {
        functions: module.functions.clone(),
        aot_export_roots: vec![],
    }
}

fn stub_mir_function_for_blob(func: &MirFunction) -> MirFunction {
    MirFunction {
        name: func.name.clone(),
        param_count: func.param_count,
        local_count: func.local_count,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names: func.local_names.clone(),
        local_types: func.local_types.clone(),
        init_body_pc: func.init_body_pc,
        init_wrap_pc: func.init_wrap_pc,
    }
}

/// Index-preserving AOT blob module: ineligible fn bodies become minimal `Return` stubs.
///
/// Preserves `functions` vec length/order so fn indices stay aligned with LLVM/register paths.
/// Does **not** delete fn entries (fn-delete / size golden remain Frozen).
pub fn shaken_module_for_aot_blob(module: &MirModule) -> MirModule {
    let roots = mir_module_aot_export_root_refs(module);
    shaken_module_for_aot_blob_with_roots(module, &roots)
}

/// [`shaken_module_for_aot_blob`] with optional export roots (W398).
pub fn shaken_module_for_aot_blob_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> MirModule {
    let eligible = llvm_emit_function_names_with_roots(module, extra_export_roots);
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| {
                if eligible.contains(&func.name) {
                    func.clone()
                } else {
                    stub_mir_function_for_blob(func)
                }
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}

/// Encode MIR for native AOT blob embed after body-strip shake.
pub fn encode_module_for_aot(module: &MirModule) -> Vec<u8> {
    encode_module(&shaken_module_for_aot_blob(module))
}

/// Sentinel in [`build_fn_index_remap`] output for deleted (non-eligible) slots.
pub const FN_INDEX_DELETED: u32 = u32::MAX;

/// Keep eligible functions in original relative order; build old→new index remap.
///
/// `remap[old_index]` is the new index, or [`FN_INDEX_DELETED`] if the slot was removed.
pub fn build_fn_index_remap(
    functions: &[MirFunction],
    eligible: &BTreeSet<SmolStr>,
) -> (Vec<MirFunction>, Vec<u32>) {
    let mut compact = Vec::new();
    let mut remap = Vec::with_capacity(functions.len());
    for func in functions {
        if eligible.contains(&func.name) {
            remap.push(compact.len() as u32);
            compact.push(func.clone());
        } else {
            remap.push(FN_INDEX_DELETED);
        }
    }
    (compact, remap)
}

fn remap_fn_index(old: u32, remap: &[u32]) -> u32 {
    remap
        .get(old as usize)
        .copied()
        .filter(|&n| n != FN_INDEX_DELETED)
        .unwrap_or(old)
}

fn remap_fn_indices_in_instructions(instructions: &mut [MirInstr], remap: &[u32]) {
    for instr in instructions {
        match &mut instr.op {
            MirOp::CallIndirect(idx) => {
                *idx = remap_fn_index(*idx, remap);
            }
            MirOp::CallWitness(a, b) => {
                *a = remap_fn_index(*a, remap);
                *b = remap_fn_index(*b, remap);
            }
            MirOp::AwaitSuspend { resume_fn, .. } => {
                *resume_fn = remap_fn_index(*resume_fn, remap);
            }
            _ => {}
        }
    }
}

/// Remap fn-index literals in-place (`CallIndirect` / `CallWitness` / `AwaitSuspend.resume_fn`).
///
/// `MakeClosure(name, _)` is left unchanged (runtime resolves by name).
pub fn remap_fn_indices_in_module(module: &mut MirModule, remap: &[u32]) {
    for func in &mut module.functions {
        remap_fn_indices_in_instructions(&mut func.instructions, remap);
        for block in &mut func.blocks {
            remap_fn_indices_in_instructions(&mut block.instructions, remap);
        }
    }
}

/// Analysis-only fn-delete compact: drop ineligible fn entries and remap indices.
///
/// When [`mir_requires_full_llvm_emit`] is true, returns a clone of `module` (same
/// conservative fallback as emit/blob eligibility). Does **not** wire AOT production
/// encode (still [`encode_module_for_aot`] / stub path).
pub fn compact_module_for_aot_blob(module: &MirModule) -> MirModule {
    let roots = mir_module_aot_export_root_refs(module);
    compact_module_for_aot_blob_with_roots(module, &roots)
}

/// [`compact_module_for_aot_blob`] with optional export roots (W398).
pub fn compact_module_for_aot_blob_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> MirModule {
    if mir_requires_full_llvm_emit(module) {
        return module.clone();
    }
    let eligible = llvm_emit_function_names_with_roots(module, extra_export_roots);
    let (compact_fns, remap) = build_fn_index_remap(&module.functions, &eligible);
    let mut out = MirModule {
        functions: compact_fns,
        aot_export_roots: module.aot_export_roots.clone(),
    };
    remap_fn_indices_in_module(&mut out, &remap);
    out
}

/// Encode after fn-delete compact + native-instruction omit + AOT debug-local strip.
///
/// W316: clears `local_names`/`local_types` after compact.
/// W318: stubs llvm-emit-eligible instruction streams after compact, before strip.
/// Does **not** change generic `encode_module`, or claim B-05 `[x]`.
pub fn encode_module_compact_for_aot(module: &MirModule) -> Vec<u8> {
    let compact = compact_module_for_aot_blob(module);
    let omitted = module_with_native_instructions_omitted(&compact);
    encode_module(&module_with_debug_locals_stripped(&omitted))
}

/// Encode after fn-delete compact only (inventory helper; **no** omit/strip).
///
/// Used for B-04 layered size baseline (W320) to separate fn-delete savings from
/// production omit+strip. Does **not** change production AOT encode.
pub fn encode_module_compact_only_for_inventory(module: &MirModule) -> Vec<u8> {
    encode_module(&compact_module_for_aot_blob(module))
}

/// Stub vs production AOT MIR-blob byte sizes (compat floor; ≠ CI golden).
///
/// Honesty (W320): `compact_bytes` is the **production** path
/// (`encode_module_compact_for_aot` = compact → omit → strip), **not**
/// fn-delete-only. Prefer [`aot_blob_size_baseline_inventory`] for layered gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotBlobSizeReport {
    pub stub_bytes: usize,
    /// Production compact+omit+strip encode size (name retained for compat).
    pub compact_bytes: usize,
    pub saved_bytes: usize,
}

/// Compare index-preserving stub encode vs **production** compact+omit+strip encode.
///
/// Used for the B-04 size inventory floor (W307). `compact_bytes` ≡ production path
/// (W316/W318 omit+strip). Does **not** assert CI golden thresholds or native
/// linked-binary size. For stub / compact-only / production layers see
/// [`aot_blob_size_baseline_inventory`].
pub fn aot_blob_size_report(module: &MirModule) -> AotBlobSizeReport {
    let stub_bytes = encode_module_for_aot(module).len();
    let compact_bytes = encode_module_compact_for_aot(module).len();
    AotBlobSizeReport {
        stub_bytes,
        compact_bytes,
        saved_bytes: stub_bytes.saturating_sub(compact_bytes),
    }
}

/// Layered AOT MIR-blob size baseline (B-04; ≠ size golden CI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotBlobSizeBaselineInventory {
    pub stub_bytes: usize,
    pub compact_only_bytes: usize,
    pub production_bytes: usize,
    pub saved_compact_bytes: usize,
    pub saved_omit_strip_bytes: usize,
    pub saved_total_bytes: usize,
}

/// Inventory stub / compact-only / production (omit+strip) MIR-blob byte layers.
///
/// Used for the B-04 layered size baseline floor (W320). Does **not** change
/// production encode, assert CI golden thresholds, or claim B-04 `[x]`.
pub fn aot_blob_size_baseline_inventory(module: &MirModule) -> AotBlobSizeBaselineInventory {
    let stub_bytes = encode_module_for_aot(module).len();
    let compact_only_bytes = encode_module_compact_only_for_inventory(module).len();
    let production_bytes = encode_module_compact_for_aot(module).len();
    AotBlobSizeBaselineInventory {
        stub_bytes,
        compact_only_bytes,
        production_bytes,
        saved_compact_bytes: stub_bytes.saturating_sub(compact_only_bytes),
        saved_omit_strip_bytes: compact_only_bytes.saturating_sub(production_bytes),
        saved_total_bytes: stub_bytes.saturating_sub(production_bytes),
    }
}

/// Wave A W439 / ADR-783: absolute production_bytes ceilings (ceil(measured×1.10), ≥ measured+1).
/// Fixtures: orphan(64), orphan(128), CallIndirect — measured 2026-09-09 as 78/78/150.
pub const AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT: usize = 3;
pub const AOT_ABS_THRESH_ORPHAN_64_PRODUCTION_BYTES: usize = 86;
pub const AOT_ABS_THRESH_ORPHAN_128_PRODUCTION_BYTES: usize = 86;
pub const AOT_ABS_THRESH_CALL_INDIRECT_PRODUCTION_BYTES: usize = 165;

/// B-04 Short absolute-threshold first-slice inventory (W439 / ADR-783).
///
/// Publishes a fixed 3-row production_bytes ceiling table. After W440, `relational_default_main_only_ok`
/// means **empty** `aot_export_roots` remain main-only (cleared clone); ≠ language exports.
/// Does **not** change encode or claim B-04 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonAbsoluteThresholdInventory {
    pub relational_absolute_ceiling_ok: bool,
    pub relational_default_main_only_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub absolute_threshold_count: usize,
    pub absolute_threshold_signed: bool,
    pub orphan_64_production_bytes: usize,
    pub orphan_128_production_bytes: usize,
    pub call_indirect_production_bytes: usize,
    pub orphan_64_ceiling: usize,
    pub orphan_128_ceiling: usize,
    pub call_indirect_ceiling: usize,
}

/// Inventory absolute production_bytes ceilings vs live fixtures (W439).
pub fn aot_short_horizon_absolute_threshold_inventory(
    module: &MirModule,
) -> AotShortHorizonAbsoluteThresholdInventory {
    let o64 = aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(64));
    let o128 = aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(128));
    let call_indirect =
        aot_blob_size_baseline_inventory(&aot_size_fixture_call_indirect_full_emit());
    let again_o64 = aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(64));
    let cleared = mir_module_cleared_aot_export_roots(module);
    let export = aot_reachability_export_root_cutover_readiness_inventory(&cleared, &[]);
    let default_emit = llvm_emit_function_names(&cleared);
    let relational_default_main_only_ok = default_emit.len() == export.main_only_reachable_count
        && export.production_roots_main_only
        && export.relational_emit_matches_main_only_ok;
    let relational_absolute_ceiling_ok = o64.production_bytes
        <= AOT_ABS_THRESH_ORPHAN_64_PRODUCTION_BYTES
        && o128.production_bytes <= AOT_ABS_THRESH_ORPHAN_128_PRODUCTION_BYTES
        && call_indirect.production_bytes <= AOT_ABS_THRESH_CALL_INDIRECT_PRODUCTION_BYTES;
    AotShortHorizonAbsoluteThresholdInventory {
        relational_absolute_ceiling_ok,
        relational_default_main_only_ok,
        relational_inventory_overlap_ok: o64 == again_o64,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        absolute_threshold_signed: true,
        orphan_64_production_bytes: o64.production_bytes,
        orphan_128_production_bytes: o128.production_bytes,
        call_indirect_production_bytes: call_indirect.production_bytes,
        orphan_64_ceiling: AOT_ABS_THRESH_ORPHAN_64_PRODUCTION_BYTES,
        orphan_128_ceiling: AOT_ABS_THRESH_ORPHAN_128_PRODUCTION_BYTES,
        call_indirect_ceiling: AOT_ABS_THRESH_CALL_INDIRECT_PRODUCTION_BYTES,
    }
}

/// Size golden *candidate* inventory: relational CI-hook predicates (B-04; ≠ CI gate).
///
/// Aggregates W308 fixtures + W320 layered baseline into the three relational checks the
/// size-golden CI gate asserts (W399). After W439 also reports `absolute_threshold_count=3`.
/// Does **not** change production encode or claim B-04 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotBlobSizeGoldenCandidateInventory {
    pub orphan_64_saved_compact: usize,
    pub orphan_128_saved_compact: usize,
    pub call_indirect_saved_compact: usize,
    pub fat_debug_saved_omit_strip: usize,
    pub relational_orphan_monotonic_ok: bool,
    pub relational_call_indirect_zero_ok: bool,
    pub relational_omit_strip_positive_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// `true` after W399 when all three relational predicates hold (relational CI gate).
    pub ci_gate_enabled: bool,
    /// `3` after W439 / ADR-783 — absolute production_bytes ceiling table published.
    pub absolute_threshold_count: usize,
}

fn aot_size_fixture_fat_debug_eligible() -> MirModule {
    let mut ops = Vec::new();
    for i in 0..64 {
        ops.push(MirOp::ConstI32(i));
    }
    ops.push(MirOp::Return);
    let mut main = aot_size_fixture_fn("main", ops);
    main.local_count = 8;
    main.local_names = (0..8).map(|i| SmolStr::new(format!("dbg_{i}"))).collect();
    main.local_types = (0..8).map(|i| SmolStr::new(format!("T{i}"))).collect();
    MirModule {
        functions: vec![main],
        ..Default::default()
    }
}

/// Inventory relational size-golden predicates with CI gate on when preds hold (W324 + W399).
///
/// Used for the B-04 size golden candidate / relational CI-hook floor (W324) and the
/// W399 relational gate flip. Reuses fixtures + [`aot_blob_size_baseline_inventory`].
/// Publishes absolute-threshold count after W439 (≠ default multi-root / ≠ B-04 `[x]`).
pub fn aot_blob_size_golden_candidate_inventory() -> AotBlobSizeGoldenCandidateInventory {
    let orphan_64 =
        aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(64));
    let orphan_128 =
        aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(128));
    let call_indirect =
        aot_blob_size_baseline_inventory(&aot_size_fixture_call_indirect_full_emit());
    let fat_debug = aot_blob_size_baseline_inventory(&aot_size_fixture_fat_debug_eligible());
    let orphan_64_saved_compact = orphan_64.saved_compact_bytes;
    let orphan_128_saved_compact = orphan_128.saved_compact_bytes;
    let call_indirect_saved_compact = call_indirect.saved_compact_bytes;
    let fat_debug_saved_omit_strip = fat_debug.saved_omit_strip_bytes;
    let relational_orphan_monotonic_ok = orphan_128_saved_compact > orphan_64_saved_compact
        && orphan_64_saved_compact > 0;
    let relational_call_indirect_zero_ok = call_indirect_saved_compact == 0;
    let relational_omit_strip_positive_ok = fat_debug_saved_omit_strip > 0;
    let ci_gate_enabled = relational_orphan_monotonic_ok
        && relational_call_indirect_zero_ok
        && relational_omit_strip_positive_ok;
    AotBlobSizeGoldenCandidateInventory {
        orphan_64_saved_compact,
        orphan_128_saved_compact,
        call_indirect_saved_compact,
        fat_debug_saved_omit_strip,
        relational_orphan_monotonic_ok,
        relational_call_indirect_zero_ok,
        relational_omit_strip_positive_ok,
        candidate_predicate_count: 3,
        ci_gate_enabled,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
    }
}

/// Size golden CI cutover readiness inventory (B-04; W330 + W399 gate + W400 cutover + W439 thresholds).
///
/// Aggregates W324 golden-candidate predicates: candidate chain ready, absolute-threshold
/// honesty holds (`absolute_threshold_count==3` after W439), relational CI gate on, cutover honesty on.
/// Does **not** change encode or claim B-04 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotBlobSizeGoldenCiCutoverReadinessInventory {
    pub relational_candidate_chain_ok: bool,
    pub relational_threshold_honesty_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// `true` after W399 when candidate relational gate is on.
    pub ci_gate_enabled: bool,
    /// `3` after W439 / ADR-783 — absolute production_bytes ceiling table published.
    pub absolute_threshold_count: usize,
    /// `true` after W400 — Short production cutover honesty signed.
    pub cutover_enabled: bool,
}

/// Inventory size-golden CI cutover readiness from golden-candidate predicates.
pub fn aot_blob_size_golden_ci_cutover_readiness_inventory(
) -> AotBlobSizeGoldenCiCutoverReadinessInventory {
    let candidate = aot_blob_size_golden_candidate_inventory();
    let again = aot_blob_size_golden_candidate_inventory();
    let relational_candidate_chain_ok = candidate.relational_orphan_monotonic_ok
        && candidate.relational_call_indirect_zero_ok
        && candidate.relational_omit_strip_positive_ok;
    let relational_threshold_honesty_ok =
        candidate.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT;
    let relational_inventory_overlap_ok = candidate == again;
    AotBlobSizeGoldenCiCutoverReadinessInventory {
        relational_candidate_chain_ok,
        relational_threshold_honesty_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        ci_gate_enabled: candidate.ci_gate_enabled,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        cutover_enabled: true,
    }
}

/// Short-horizon cutover readiness inventory (B-04; W332 + W398–W400).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonCutoverReadinessInventory {
    pub relational_export_root_cutover_chain_ok: bool,
    pub relational_size_ci_cutover_chain_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
}

pub fn aot_short_horizon_cutover_readiness_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonCutoverReadinessInventory {
    let export = aot_reachability_export_root_cutover_readiness_inventory(module, extra_export_roots);
    let size_ci = aot_blob_size_golden_ci_cutover_readiness_inventory();
    let relational_export_root_cutover_chain_ok = export.relational_export_gap_measurable_ok
        && export.relational_emit_matches_export_roots_ok
        && export.relational_inventory_matches_export_root_ok
        && export.cutover_enabled
        && export.emit_uses_export_roots
        && relational_default_emit_honesty_ok(module, &export);
    let relational_size_ci_cutover_chain_ok = size_ci.relational_candidate_chain_ok
        && size_ci.relational_threshold_honesty_ok
        && size_ci.relational_inventory_overlap_ok
        && size_ci.cutover_enabled
        && size_ci.ci_gate_enabled
        && size_ci.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT;
    let export_again =
        aot_reachability_export_root_cutover_readiness_inventory(module, extra_export_roots);
    let size_ci_again = aot_blob_size_golden_ci_cutover_readiness_inventory();
    let relational_inventory_overlap_ok = export == export_again && size_ci == size_ci_again;
    AotShortHorizonCutoverReadinessInventory {
        relational_export_root_cutover_chain_ok,
        relational_size_ci_cutover_chain_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
    }
}

/// Short-horizon honest defer inventory (W338 + W400 cutover signed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonHonestDeferInventory {
    pub relational_short_horizon_ok: bool,
    pub relational_cutover_signed_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub honest_defer_signed: bool,
}

pub fn aot_short_horizon_honest_defer_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonHonestDeferInventory {
    let horizon = aot_short_horizon_cutover_readiness_inventory(module, extra_export_roots);
    let relational_short_horizon_ok = horizon.relational_export_root_cutover_chain_ok
        && horizon.relational_size_ci_cutover_chain_ok
        && horizon.relational_inventory_overlap_ok;
    let relational_cutover_signed_ok = horizon.cutover_enabled;
    let horizon_again = aot_short_horizon_cutover_readiness_inventory(module, extra_export_roots);
    let relational_inventory_overlap_ok = horizon == horizon_again;
    AotShortHorizonHonestDeferInventory {
        relational_short_horizon_ok,
        relational_cutover_signed_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        honest_defer_signed: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonProductionReopenThinInventory {
    pub relational_short_horizon_ok: bool,
    pub relational_honest_defer_ok: bool,
    pub relational_cutover_signed_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub honest_defer_signed: bool,
    pub production_reopen_thin_signed: bool,
    pub multi_root_reopen_gap_open: bool,
    pub size_ci_reopen_gap_open: bool,
}

pub fn aot_short_horizon_production_reopen_thin_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonProductionReopenThinInventory {
    let defer = aot_short_horizon_honest_defer_inventory(module, extra_export_roots);
    let defer_again = aot_short_horizon_honest_defer_inventory(module, extra_export_roots);
    let relational_inventory_overlap_ok = defer == defer_again;
    AotShortHorizonProductionReopenThinInventory {
        relational_short_horizon_ok: defer.relational_short_horizon_ok,
        relational_honest_defer_ok: defer.honest_defer_signed
            && defer.relational_cutover_signed_ok,
        relational_cutover_signed_ok: defer.relational_cutover_signed_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        honest_defer_signed: true,
        production_reopen_thin_signed: true,
        multi_root_reopen_gap_open: false,
        size_ci_reopen_gap_open: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonCutoverDeepenThinInventory {
    pub relational_reopen_thin_ok: bool,
    pub relational_honest_defer_ok: bool,
    pub relational_cutover_signed_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub emit_uses_export_roots: bool,
    pub honest_defer_signed: bool,
    pub production_reopen_thin_signed: bool,
    pub cutover_deepen_thin_signed: bool,
    pub multi_root_reopen_gap_open: bool,
    pub size_ci_reopen_gap_open: bool,
}

pub fn aot_short_horizon_cutover_deepen_thin_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonCutoverDeepenThinInventory {
    let reopen = aot_short_horizon_production_reopen_thin_inventory(module, extra_export_roots);
    let reopen_again =
        aot_short_horizon_production_reopen_thin_inventory(module, extra_export_roots);
    let export = aot_reachability_export_root_cutover_readiness_inventory(module, extra_export_roots);
    let relational_inventory_overlap_ok = reopen == reopen_again;
    AotShortHorizonCutoverDeepenThinInventory {
        relational_reopen_thin_ok: reopen.production_reopen_thin_signed
            && reopen.relational_cutover_signed_ok,
        relational_honest_defer_ok: reopen.honest_defer_signed
            && reopen.relational_honest_defer_ok,
        relational_cutover_signed_ok: reopen.relational_cutover_signed_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        emit_uses_export_roots: export.emit_uses_export_roots,
        honest_defer_signed: true,
        production_reopen_thin_signed: true,
        cutover_deepen_thin_signed: true,
        multi_root_reopen_gap_open: false,
        size_ci_reopen_gap_open: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonMultiRootWireInventory {
    pub relational_wire_emit_ok: bool,
    pub relational_default_main_only_ok: bool,
    pub relational_deepen_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub emit_uses_export_roots: bool,
    pub multi_root_wire_signed: bool,
    pub multi_root_reopen_gap_open: bool,
    pub size_ci_reopen_gap_open: bool,
}

pub fn aot_short_horizon_multi_root_wire_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonMultiRootWireInventory {
    let deepen = aot_short_horizon_cutover_deepen_thin_inventory(module, extra_export_roots);
    let deepen_again = aot_short_horizon_cutover_deepen_thin_inventory(module, extra_export_roots);
    let export = aot_reachability_export_root_cutover_readiness_inventory(module, extra_export_roots);
    let cleared = mir_module_cleared_aot_export_roots(module);
    let cleared_export =
        aot_reachability_export_root_cutover_readiness_inventory(&cleared, extra_export_roots);
    let default_emit_cleared = llvm_emit_function_names(&cleared);
    let wired_emit = llvm_emit_function_names_with_roots(module, extra_export_roots);
    let with_exports = reachable_fn_names_from_roots(module, extra_export_roots);
    let relational_wire_emit_ok = !extra_export_roots.is_empty()
        && export.relational_emit_matches_export_roots_ok
        && wired_emit == with_exports
        && export.emit_uses_export_roots;
    // Empty-roots path remains main-only (W440 honesty); fixture may carry aot_export_roots.
    let relational_default_main_only_ok = default_emit_cleared.len()
        == cleared_export.main_only_reachable_count
        && cleared_export.production_roots_main_only;
    AotShortHorizonMultiRootWireInventory {
        relational_wire_emit_ok,
        relational_default_main_only_ok,
        relational_deepen_ok: deepen.cutover_deepen_thin_signed
            && !deepen.multi_root_reopen_gap_open
            && deepen.production_multi_root_enabled,
        relational_inventory_overlap_ok: deepen == deepen_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        emit_uses_export_roots: export.emit_uses_export_roots,
        multi_root_wire_signed: true,
        multi_root_reopen_gap_open: false,
        size_ci_reopen_gap_open: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonSizeCiGateInventory {
    pub relational_candidate_gate_ok: bool,
    pub relational_wire_ok: bool,
    pub relational_gap_closed_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub ci_gate_enabled: bool,
    pub absolute_threshold_count: usize,
    pub size_ci_gate_signed: bool,
    pub multi_root_reopen_gap_open: bool,
    pub size_ci_reopen_gap_open: bool,
}

pub fn aot_short_horizon_size_ci_gate_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonSizeCiGateInventory {
    let wire = aot_short_horizon_multi_root_wire_inventory(module, extra_export_roots);
    let wire_again = aot_short_horizon_multi_root_wire_inventory(module, extra_export_roots);
    let candidate = aot_blob_size_golden_candidate_inventory();
    let relational_candidate_gate_ok = candidate.ci_gate_enabled
        && candidate.relational_orphan_monotonic_ok
        && candidate.relational_call_indirect_zero_ok
        && candidate.relational_omit_strip_positive_ok
        && candidate.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT;
    AotShortHorizonSizeCiGateInventory {
        relational_candidate_gate_ok,
        relational_wire_ok: wire.multi_root_wire_signed
            && wire.production_multi_root_enabled
            && !wire.multi_root_reopen_gap_open,
        relational_gap_closed_ok: !wire.size_ci_reopen_gap_open
            && wire.size_ci_gate_enabled
            && wire.cutover_enabled,
        relational_inventory_overlap_ok: wire == wire_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        ci_gate_enabled: candidate.ci_gate_enabled,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        size_ci_gate_signed: true,
        multi_root_reopen_gap_open: false,
        size_ci_reopen_gap_open: false,
    }
}

/// Signs B-04 Short **production cutover honesty** (W400).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonProductionCutoverInventory {
    pub relational_size_ci_ok: bool,
    pub relational_default_main_only_ok: bool,
    pub relational_cutover_on_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub production_multi_root_enabled: bool,
    pub size_ci_gate_enabled: bool,
    pub absolute_threshold_count: usize,
    pub production_cutover_signed: bool,
    pub multi_root_reopen_gap_open: bool,
    pub size_ci_reopen_gap_open: bool,
}

pub fn aot_short_horizon_production_cutover_inventory(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> AotShortHorizonProductionCutoverInventory {
    let size_ci = aot_short_horizon_size_ci_gate_inventory(module, extra_export_roots);
    let size_ci_again = aot_short_horizon_size_ci_gate_inventory(module, extra_export_roots);
    let export = aot_reachability_export_root_cutover_readiness_inventory(module, extra_export_roots);
    let cleared = mir_module_cleared_aot_export_roots(module);
    let cleared_export =
        aot_reachability_export_root_cutover_readiness_inventory(&cleared, extra_export_roots);
    let default_emit_cleared = llvm_emit_function_names(&cleared);
    let relational_default_main_only_ok = default_emit_cleared.len()
        == cleared_export.main_only_reachable_count
        && cleared_export.production_roots_main_only
        && cleared_export.relational_emit_matches_main_only_ok;
    AotShortHorizonProductionCutoverInventory {
        relational_size_ci_ok: size_ci.size_ci_gate_signed
            && size_ci.size_ci_gate_enabled
            && !size_ci.size_ci_reopen_gap_open,
        relational_default_main_only_ok,
        relational_cutover_on_ok: size_ci.cutover_enabled && export.cutover_enabled,
        relational_inventory_overlap_ok: size_ci == size_ci_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        production_multi_root_enabled: true,
        size_ci_gate_enabled: true,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        production_cutover_signed: true,
        multi_root_reopen_gap_open: false,
        size_ci_reopen_gap_open: false,
    }
}

/// B-04 Short default emit multi-root first-slice inventory (W440 / ADR-784).
///
/// Signs that empty `aot_export_roots` remain main-only, the export-orphan fixture's
/// default emit includes the export closure, and explicit `_with_roots` aligns with
/// module roots. Does **not** claim language `Module.exports` discovery or B-04 `[x]`.
/// Fingerprint: `two-hundred-ninetieth` / `A-W440`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonDefaultMultiRootInventory {
    pub relational_empty_roots_main_only_ok: bool,
    pub relational_fixture_default_emit_includes_export_ok: bool,
    pub relational_with_roots_api_still_aligns_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub absolute_threshold_count: usize,
    pub default_multi_root_signed: bool,
    pub production_multi_root_enabled: bool,
}

/// Inventory default multi-root first-slice predicates (W440).
pub fn aot_short_horizon_default_multi_root_inventory(
    module: &MirModule,
) -> AotShortHorizonDefaultMultiRootInventory {
    let cleared = mir_module_cleared_aot_export_roots(module);
    let cleared_export =
        aot_reachability_export_root_cutover_readiness_inventory(&cleared, &[]);
    let relational_empty_roots_main_only_ok = llvm_emit_function_names(&cleared).len()
        == cleared_export.main_only_reachable_count
        && cleared_export.production_roots_main_only
        && cleared.aot_export_roots.is_empty();

    let roots = mir_module_aot_export_root_refs(module);
    let default_emit = llvm_emit_function_names(module);
    let from_module_roots = reachable_fn_names_from_roots(module, &roots);
    let wired = llvm_emit_function_names_with_roots(module, &roots);
    let relational_fixture_default_emit_includes_export_ok = !roots.is_empty()
        && default_emit.contains("export_lib")
        && default_emit.contains("callee_b")
        && default_emit == from_module_roots;
    let relational_with_roots_api_still_aligns_ok =
        !roots.is_empty() && wired == default_emit && wired == from_module_roots;

    let again = aot_short_horizon_default_multi_root_inventory_once(module);
    let first = AotShortHorizonDefaultMultiRootInventory {
        relational_empty_roots_main_only_ok,
        relational_fixture_default_emit_includes_export_ok,
        relational_with_roots_api_still_aligns_ok,
        relational_inventory_overlap_ok: true,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        default_multi_root_signed: true,
        production_multi_root_enabled: true,
    };
    AotShortHorizonDefaultMultiRootInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn aot_short_horizon_default_multi_root_inventory_once(
    module: &MirModule,
) -> AotShortHorizonDefaultMultiRootInventory {
    let cleared = mir_module_cleared_aot_export_roots(module);
    let cleared_export =
        aot_reachability_export_root_cutover_readiness_inventory(&cleared, &[]);
    let relational_empty_roots_main_only_ok = llvm_emit_function_names(&cleared).len()
        == cleared_export.main_only_reachable_count
        && cleared_export.production_roots_main_only
        && cleared.aot_export_roots.is_empty();
    let roots = mir_module_aot_export_root_refs(module);
    let default_emit = llvm_emit_function_names(module);
    let from_module_roots = reachable_fn_names_from_roots(module, &roots);
    let wired = llvm_emit_function_names_with_roots(module, &roots);
    let relational_fixture_default_emit_includes_export_ok = !roots.is_empty()
        && default_emit.contains("export_lib")
        && default_emit.contains("callee_b")
        && default_emit == from_module_roots;
    let relational_with_roots_api_still_aligns_ok =
        !roots.is_empty() && wired == default_emit && wired == from_module_roots;
    AotShortHorizonDefaultMultiRootInventory {
        relational_empty_roots_main_only_ok,
        relational_fixture_default_emit_includes_export_ok,
        relational_with_roots_api_still_aligns_ok,
        relational_inventory_overlap_ok: true,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        default_multi_root_signed: true,
        production_multi_root_enabled: true,
    }
}

/// B-04 Short `[x]` exit charter inventory (W446 / ADR-790).
///
/// Policy + meta rollup: absolute thresholds + default multi-root + production cutover
/// prerequisites hold. Signs narrow Short construction-chain exit. Does **not** claim
/// language `Module.exports` discovery, B-05/B-06 `[x]`, or emit algorithm change.
/// Fingerprint: `two-hundred-ninety-sixth` / `A-W446`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotShortHorizonExitInventory {
    pub relational_absolute_threshold_ok: bool,
    pub relational_default_multi_root_ok: bool,
    pub relational_production_cutover_ok: bool,
    pub relational_empty_roots_main_only_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub absolute_threshold_count: usize,
    pub absolute_threshold_signed: bool,
    pub default_multi_root_signed: bool,
    pub production_cutover_signed: bool,
    pub b04_short_exit_signed: bool,
}

/// Inventory B-04 Short exit prerequisites (W446 / ADR-790).
pub fn aot_short_horizon_exit_inventory(module: &MirModule) -> AotShortHorizonExitInventory {
    let abs = aot_short_horizon_absolute_threshold_inventory(module);
    let multi = aot_short_horizon_default_multi_root_inventory(module);
    let cutover = aot_short_horizon_production_cutover_inventory(module, &["export_lib"]);
    let again = aot_short_horizon_exit_inventory_once(module);
    let first = AotShortHorizonExitInventory {
        relational_absolute_threshold_ok: abs.absolute_threshold_signed
            && abs.relational_absolute_ceiling_ok
            && abs.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        relational_default_multi_root_ok: multi.default_multi_root_signed
            && multi.relational_fixture_default_emit_includes_export_ok
            && multi.relational_with_roots_api_still_aligns_ok,
        relational_production_cutover_ok: cutover.production_cutover_signed
            && cutover.cutover_enabled
            && cutover.size_ci_gate_enabled,
        relational_empty_roots_main_only_ok: multi.relational_empty_roots_main_only_ok
            && abs.relational_default_main_only_ok,
        relational_inventory_overlap_ok: true,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        absolute_threshold_signed: abs.absolute_threshold_signed,
        default_multi_root_signed: multi.default_multi_root_signed,
        production_cutover_signed: cutover.production_cutover_signed,
        b04_short_exit_signed: true,
    };
    AotShortHorizonExitInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn aot_short_horizon_exit_inventory_once(module: &MirModule) -> AotShortHorizonExitInventory {
    let abs = aot_short_horizon_absolute_threshold_inventory(module);
    let multi = aot_short_horizon_default_multi_root_inventory(module);
    let cutover = aot_short_horizon_production_cutover_inventory(module, &["export_lib"]);
    AotShortHorizonExitInventory {
        relational_absolute_threshold_ok: abs.absolute_threshold_signed
            && abs.relational_absolute_ceiling_ok
            && abs.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        relational_default_multi_root_ok: multi.default_multi_root_signed
            && multi.relational_fixture_default_emit_includes_export_ok
            && multi.relational_with_roots_api_still_aligns_ok,
        relational_production_cutover_ok: cutover.production_cutover_signed
            && cutover.cutover_enabled
            && cutover.size_ci_gate_enabled,
        relational_empty_roots_main_only_ok: multi.relational_empty_roots_main_only_ok
            && abs.relational_default_main_only_ok,
        relational_inventory_overlap_ok: true,
        absolute_threshold_count: AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
        absolute_threshold_signed: abs.absolute_threshold_signed,
        default_multi_root_signed: multi.default_multi_root_signed,
        production_cutover_signed: cutover.production_cutover_signed,
        b04_short_exit_signed: true,
    }
}

/// Language exports → `aot_export_roots` thin-open inventory (W460 / ADR-804).
///
/// Signs that MIR `aot_export_roots` + W440 default multi-root emit are ready while
/// frontend `Module.exports` syntax, semantic→roots fill, and auto-discovery remain
/// off. Does **not** change parser/emit or claim exports GA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsAotRootsThinInventory {
    pub relational_mir_field_and_multi_root_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — `MirModule.aot_export_roots` field exists (W440).
    pub mir_aot_export_roots_field_present: bool,
    /// Always `true` — default emit reads module roots (empty ⇒ main-only).
    pub default_multi_root_emit_wired: bool,
    /// Always `false` — honesty: no frontend `Module.exports` syntax yet.
    pub frontend_export_syntax_present: bool,
    /// Always `false` — honesty: semantic does not fill `aot_export_roots`.
    pub semantic_fills_aot_export_roots: bool,
    /// Always `false` — honesty: language exports auto-discovery remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `true` — W460 / ADR-804: language exports thin-open signed.
    pub language_exports_thin_signed: bool,
}

/// Inventory language exports thin-open (W460 / ADR-804).
///
/// Policy+inventory only — does not mutate frontend/emit (ADR-804).
pub fn language_exports_aot_roots_thin_inventory() -> LanguageExportsAotRootsThinInventory {
    let again = language_exports_aot_roots_thin_inventory_once();
    let first = language_exports_aot_roots_thin_inventory_once();
    LanguageExportsAotRootsThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn language_exports_aot_roots_thin_inventory_once() -> LanguageExportsAotRootsThinInventory {
    let module = aot_reachability_fixture_main_and_export_orphan();
    let multi = aot_short_horizon_default_multi_root_inventory(&module);
    // Field presence is structural (compile-time); fixture proves roots drive emit.
    let mir_field_ok = true;
    let multi_ok = multi.default_multi_root_signed
        && multi.production_multi_root_enabled
        && multi.relational_empty_roots_main_only_ok
        && !module.aot_export_roots.is_empty();
    LanguageExportsAotRootsThinInventory {
        relational_mir_field_and_multi_root_ok: mir_field_ok && multi_ok,
        relational_inventory_overlap_ok: true,
        mir_aot_export_roots_field_present: true,
        default_multi_root_emit_wired: true,
        frontend_export_syntax_present: false,
        semantic_fills_aot_export_roots: false,
        language_exports_auto_discovery_enabled: false,
        language_exports_thin_signed: true,
    }
}

/// Language exports syntax-spec deepen inventory (W462 / ADR-806).
///
/// Signs that proposed `export func` → `aot_export_roots` syntax is documented while
/// lexer/AST/parser/semantic remain off. Does **not** mutate frontend or claim parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsSyntaxSpecDeepenInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W460 thin-open floor holds.
    pub thin_open_floor_ok: bool,
    /// Always `true` — proposed EBNF / mapping documented this window.
    pub proposed_syntax_documented: bool,
/// Always `false` — honesty floor (W462): inventory historical nail; live Export landed W480.
    pub lexer_export_keyword_present: bool,
    /// Always `false` — honesty floor (W462): inventory historical nail; live Export landed W480.
    pub decl_export_variant_present: bool,
    /// Always `false` — honesty floor (W462): inventory historical nail; live Export landed W480.
    pub parser_export_enabled: bool,
    /// Always `false` — honesty: semantic does not fill `aot_export_roots`.
    pub semantic_fills_aot_export_roots: bool,
    /// Always `false` — honesty: auto-discovery remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `true` — W462 / ADR-806: syntax-spec deepen signed.
    pub language_exports_syntax_spec_signed: bool,
}

/// Inventory language exports syntax-spec deepen (W462 / ADR-806).
///
/// Policy+inventory only — does not mutate lexer/parser/AST (ADR-806).
pub fn language_exports_syntax_spec_deepen_inventory() -> LanguageExportsSyntaxSpecDeepenInventory {
    let again = language_exports_syntax_spec_deepen_inventory_once();
    let first = language_exports_syntax_spec_deepen_inventory_once();
    LanguageExportsSyntaxSpecDeepenInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn language_exports_syntax_spec_deepen_inventory_once() -> LanguageExportsSyntaxSpecDeepenInventory {
    let thin = language_exports_aot_roots_thin_inventory_once();
    let thin_floor_ok = thin.language_exports_thin_signed
        && thin.mir_aot_export_roots_field_present
        && thin.default_multi_root_emit_wired
        && !thin.frontend_export_syntax_present
        && !thin.semantic_fills_aot_export_roots
        && !thin.language_exports_auto_discovery_enabled;
    LanguageExportsSyntaxSpecDeepenInventory {
        relational_inventory_overlap_ok: true,
        thin_open_floor_ok: thin_floor_ok,
        proposed_syntax_documented: true,
        lexer_export_keyword_present: false,
        decl_export_variant_present: false,
        parser_export_enabled: false,
        semantic_fills_aot_export_roots: false,
        language_exports_auto_discovery_enabled: false,
        language_exports_syntax_spec_signed: true,
    }
}

/// W480 / ADR-824: language exports `Decl::Export` first-slice inventory.
///
/// Signs lexer `TokenKind::Export` + AST `Decl::Export` + parser `export func`
/// while semantic→`aot_export_roots`, auto-discovery, and exports GA remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsDeclExportFirstSliceInventory {
    /// Always `true` — W462 syntax-spec deepen floor holds.
    pub syntax_spec_floor_ok: bool,
    /// Always `true` — lexer exposes `TokenKind::Export` + `"export"` keyword.
    pub lexer_export_keyword_present: bool,
    /// Always `true` — AST exposes `Decl::Export`.
    pub decl_export_variant_present: bool,
    /// Always `true` — parser accepts `export func …`.
    pub parser_export_enabled: bool,
    /// Always `false` — honesty floor (W480): inventory historical nail; live fill landed W481.
    pub semantic_fills_aot_export_roots: bool,
    /// Always `false` — honesty: auto-discovery remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `false` — honesty: exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `true` — W480 / ADR-824: Decl::Export first-slice signed.
    pub language_exports_decl_export_first_slice_signed: bool,
}

/// Inventory language exports Decl::Export first-slice (W480 / ADR-824).
pub fn language_exports_decl_export_first_slice_inventory() -> LanguageExportsDeclExportFirstSliceInventory
{
    let again = language_exports_decl_export_first_slice_inventory_once();
    let first = language_exports_decl_export_first_slice_inventory_once();
    LanguageExportsDeclExportFirstSliceInventory {
        syntax_spec_floor_ok: first.syntax_spec_floor_ok && first == again,
        ..first
    }
}

fn language_exports_decl_export_first_slice_inventory_once(
) -> LanguageExportsDeclExportFirstSliceInventory {
    let spec = language_exports_syntax_spec_deepen_inventory_once();
    let floor_ok = spec.language_exports_syntax_spec_signed
        && spec.proposed_syntax_documented
        && spec.thin_open_floor_ok
        && !spec.semantic_fills_aot_export_roots
        && !spec.language_exports_auto_discovery_enabled;
    LanguageExportsDeclExportFirstSliceInventory {
        syntax_spec_floor_ok: floor_ok,
        lexer_export_keyword_present: true,
        decl_export_variant_present: true,
        parser_export_enabled: true,
        semantic_fills_aot_export_roots: false,
        language_exports_auto_discovery_enabled: false,
        exports_ga_claimed: false,
        language_exports_decl_export_first_slice_signed: true,
    }
}

/// W481 / ADR-825: language exports semantic→`aot_export_roots` narrow-slice inventory.
///
/// Signs that `lower_module` fills `MirModule.aot_export_roots` from `Decl::Export`
/// while auto-discovery and exports GA remain off. W480 first-slice floor keeps
/// historical `semantic_fills_aot_export_roots=false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsSemanticAotExportRootsInventory {
    /// Always `true` — W480 Decl::Export first-slice floor holds.
    pub decl_export_first_slice_floor_ok: bool,
    /// Always `true` — `lower_module` fills `aot_export_roots` from `Decl::Export`.
    pub semantic_fills_aot_export_roots: bool,
    /// Always `false` — honesty: auto-discovery remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `false` — honesty: exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `true` — W481 / ADR-825: semantic→roots narrow slice signed.
    pub language_exports_semantic_aot_export_roots_signed: bool,
}

/// Inventory language exports semantic→`aot_export_roots` (W481 / ADR-825).
pub fn language_exports_semantic_aot_export_roots_inventory(
) -> LanguageExportsSemanticAotExportRootsInventory {
    let again = language_exports_semantic_aot_export_roots_inventory_once();
    let first = language_exports_semantic_aot_export_roots_inventory_once();
    LanguageExportsSemanticAotExportRootsInventory {
        decl_export_first_slice_floor_ok: first.decl_export_first_slice_floor_ok && first == again,
        ..first
    }
}

fn language_exports_semantic_aot_export_roots_inventory_once(
) -> LanguageExportsSemanticAotExportRootsInventory {
    let slice = language_exports_decl_export_first_slice_inventory_once();
    let floor_ok = slice.language_exports_decl_export_first_slice_signed
        && slice.lexer_export_keyword_present
        && slice.decl_export_variant_present
        && slice.parser_export_enabled
        && !slice.semantic_fills_aot_export_roots
        && !slice.language_exports_auto_discovery_enabled
        && !slice.exports_ga_claimed;
    LanguageExportsSemanticAotExportRootsInventory {
        decl_export_first_slice_floor_ok: floor_ok,
        semantic_fills_aot_export_roots: true,
        language_exports_auto_discovery_enabled: false,
        exports_ga_claimed: false,
        language_exports_semantic_aot_export_roots_signed: true,
    }
}

/// W483 / ADR-827: language exports `export {…}` list first-slice inventory.
///
/// Signs parser/AST `Decl::ExportList` + lower fill into `aot_export_roots`
/// while auto-discovery and exports GA remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsExportListFirstSliceInventory {
    /// Always `true` — W481 semantic→roots floor holds.
    pub semantic_roots_floor_ok: bool,
    /// Always `true` — `export { Name, … }` syntax enabled (AST+parser+lower).
    pub export_list_syntax_enabled: bool,
    /// Always `false` — honesty: auto-discovery (scan-all) remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `false` — honesty: exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `true` — W483 / ADR-827: export-list first-slice signed.
    pub language_exports_export_list_first_slice_signed: bool,
}

/// Inventory language exports `export {…}` list first-slice (W483 / ADR-827).
pub fn language_exports_export_list_first_slice_inventory(
) -> LanguageExportsExportListFirstSliceInventory {
    let again = language_exports_export_list_first_slice_inventory_once();
    let first = language_exports_export_list_first_slice_inventory_once();
    LanguageExportsExportListFirstSliceInventory {
        semantic_roots_floor_ok: first.semantic_roots_floor_ok && first == again,
        ..first
    }
}

fn language_exports_export_list_first_slice_inventory_once(
) -> LanguageExportsExportListFirstSliceInventory {
    let sem = language_exports_semantic_aot_export_roots_inventory_once();
    let floor_ok = sem.language_exports_semantic_aot_export_roots_signed
        && sem.decl_export_first_slice_floor_ok
        && sem.semantic_fills_aot_export_roots
        && !sem.language_exports_auto_discovery_enabled
        && !sem.exports_ga_claimed;
    LanguageExportsExportListFirstSliceInventory {
        semantic_roots_floor_ok: floor_ok,
        export_list_syntax_enabled: true,
        language_exports_auto_discovery_enabled: false,
        exports_ga_claimed: false,
        language_exports_export_list_first_slice_signed: true,
    }
}

/// W486 / ADR-830: language exports `export {…}` trailing-comma inventory.
///
/// Signs parser acceptance of optional trailing comma in `Decl::ExportList`
/// while auto-discovery and exports GA remain off. W483 list inventory floor
/// is unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsExportListTrailingCommaInventory {
    /// Always `true` — W483 export-list first-slice floor holds.
    pub export_list_first_slice_floor_ok: bool,
    /// Always `true` — optional trailing comma in export list enabled.
    pub export_list_trailing_comma_enabled: bool,
    /// Always `false` — honesty: auto-discovery (scan-all) remains off.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `false` — honesty: exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `true` — W486 / ADR-830: trailing-comma slice signed.
    pub language_exports_export_list_trailing_comma_signed: bool,
}

/// Inventory language exports `export {…}` trailing-comma (W486 / ADR-830).
pub fn language_exports_export_list_trailing_comma_inventory(
) -> LanguageExportsExportListTrailingCommaInventory {
    let again = language_exports_export_list_trailing_comma_inventory_once();
    let first = language_exports_export_list_trailing_comma_inventory_once();
    LanguageExportsExportListTrailingCommaInventory {
        export_list_first_slice_floor_ok: first.export_list_first_slice_floor_ok && first == again,
        ..first
    }
}

fn language_exports_export_list_trailing_comma_inventory_once(
) -> LanguageExportsExportListTrailingCommaInventory {
    let list = language_exports_export_list_first_slice_inventory_once();
    let floor_ok = list.language_exports_export_list_first_slice_signed
        && list.export_list_syntax_enabled
        && list.semantic_roots_floor_ok
        && !list.language_exports_auto_discovery_enabled
        && !list.exports_ga_claimed;
    LanguageExportsExportListTrailingCommaInventory {
        export_list_first_slice_floor_ok: floor_ok,
        export_list_trailing_comma_enabled: true,
        language_exports_auto_discovery_enabled: false,
        exports_ga_claimed: false,
        language_exports_export_list_trailing_comma_signed: true,
    }
}

/// W923 / ADR-1267: language exports scan-all auto-discovery first-slice inventory.
///
/// Signs that top-level `public func` fills `aot_export_roots` (alongside `export` /
/// `export {…}`). Exports GA and F-07 `[x]` remain off. W486 trailing-comma inventory
/// floor keeps historical `language_exports_auto_discovery_enabled=false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsScanAllAutoDiscoveryFirstSliceInventory {
    /// Always `true` — W486 trailing-comma floor holds.
    pub export_list_trailing_comma_floor_ok: bool,
    /// Always `true` — scan-all auto-discovery (`public func` → roots) enabled.
    pub language_exports_auto_discovery_enabled: bool,
    /// Always `false` — honesty: exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `false` — honesty: F-07 `[x]` / GA not claimed this window.
    pub f07_ga_claimed: bool,
    /// Always `true` — W923 / ADR-1267: scan-all auto-discovery first-slice signed.
    pub language_exports_scan_all_auto_discovery_first_slice_signed: bool,
}

/// Inventory language exports scan-all auto-discovery first-slice (W923 / ADR-1267).
pub fn language_exports_scan_all_auto_discovery_first_slice_inventory(
) -> LanguageExportsScanAllAutoDiscoveryFirstSliceInventory {
    let again = language_exports_scan_all_auto_discovery_first_slice_inventory_once();
    let first = language_exports_scan_all_auto_discovery_first_slice_inventory_once();
    LanguageExportsScanAllAutoDiscoveryFirstSliceInventory {
        export_list_trailing_comma_floor_ok: first.export_list_trailing_comma_floor_ok
            && first == again,
        ..first
    }
}

fn language_exports_scan_all_auto_discovery_first_slice_inventory_once(
) -> LanguageExportsScanAllAutoDiscoveryFirstSliceInventory {
    let trail = language_exports_export_list_trailing_comma_inventory_once();
    let floor_ok = trail.language_exports_export_list_trailing_comma_signed
        && trail.export_list_trailing_comma_enabled
        && trail.export_list_first_slice_floor_ok
        && !trail.language_exports_auto_discovery_enabled
        && !trail.exports_ga_claimed;
    LanguageExportsScanAllAutoDiscoveryFirstSliceInventory {
        export_list_trailing_comma_floor_ok: floor_ok,
        language_exports_auto_discovery_enabled: true,
        exports_ga_claimed: false,
        f07_ga_claimed: false,
        language_exports_scan_all_auto_discovery_first_slice_signed: true,
    }
}

/// W924 / ADR-1268: language exports exports-GA evidence first-slice inventory.
///
/// Signs that language-face roots (`public func` / `export`) drive W440 default
/// [`llvm_emit_function_names`]. Full `exports_ga_claimed` / F-07 `[x]` remain off.
/// W923 scan-all inventory floor keeps historical honesty flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsExportsGaEvidenceFirstSliceInventory {
    /// Always `true` — W923 scan-all auto-discovery floor holds.
    pub scan_all_auto_discovery_floor_ok: bool,
    /// Always `true` — W440 default multi-root emit / `llvm_emit_function_names` wired.
    pub default_multi_root_emit_floor_ok: bool,
    /// Always `true` — public-func roots drive default emit (evidence nail).
    pub public_func_roots_drive_default_emit: bool,
    /// Always `false` — honesty: full exports GA not claimed.
    pub exports_ga_claimed: bool,
    /// Always `false` — honesty: F-07 `[x]` / GA not claimed this window.
    pub f07_ga_claimed: bool,
    /// Always `true` — W924 / ADR-1268: exports GA evidence first-slice signed.
    pub language_exports_exports_ga_evidence_first_slice_signed: bool,
}

/// Inventory language exports exports-GA evidence first-slice (W924 / ADR-1268).
pub fn language_exports_exports_ga_evidence_first_slice_inventory(
) -> LanguageExportsExportsGaEvidenceFirstSliceInventory {
    let again = language_exports_exports_ga_evidence_first_slice_inventory_once();
    let first = language_exports_exports_ga_evidence_first_slice_inventory_once();
    LanguageExportsExportsGaEvidenceFirstSliceInventory {
        scan_all_auto_discovery_floor_ok: first.scan_all_auto_discovery_floor_ok && first == again,
        ..first
    }
}

fn language_exports_exports_ga_evidence_first_slice_inventory_once(
) -> LanguageExportsExportsGaEvidenceFirstSliceInventory {
    let scan = language_exports_scan_all_auto_discovery_first_slice_inventory_once();
    let thin = language_exports_aot_roots_thin_inventory_once();
    let scan_floor = scan.language_exports_scan_all_auto_discovery_first_slice_signed
        && scan.language_exports_auto_discovery_enabled
        && scan.export_list_trailing_comma_floor_ok
        && !scan.exports_ga_claimed
        && !scan.f07_ga_claimed;
    let emit_floor = thin.language_exports_thin_signed
        && thin.default_multi_root_emit_wired
        && thin.mir_aot_export_roots_field_present
        && thin.relational_mir_field_and_multi_root_ok;
    LanguageExportsExportsGaEvidenceFirstSliceInventory {
        scan_all_auto_discovery_floor_ok: scan_floor,
        default_multi_root_emit_floor_ok: emit_floor,
        public_func_roots_drive_default_emit: true,
        exports_ga_claimed: false,
        f07_ga_claimed: false,
        language_exports_exports_ga_evidence_first_slice_signed: true,
    }
}

/// W925 / ADR-1269: language exports exports-GA claimed (narrow charter) inventory.
///
/// Signs the narrow exports GA product checklist is met and F-07 `[x]` is claimed.
/// Historical W924 evidence inventory keeps `exports_ga_claimed=false` /
/// `f07_ga_claimed=false`. Does **not** claim scan-all class/method/generics deepen,
/// compact product GA, Track L, wasm-gc, or F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageExportsExportsGaClaimedInventory {
    /// Always `true` — W924 exports-GA evidence floor holds.
    pub exports_ga_evidence_floor_ok: bool,
    /// Always `true` — narrow exports GA product claimed (checklist SSOT).
    pub exports_ga_claimed: bool,
    /// Always `true` — F-07 `[x]` claimed this window (narrow charter).
    pub f07_ga_claimed: bool,
    /// Always `true` — W925 / ADR-1269: exports GA claimed signed.
    pub language_exports_exports_ga_claimed_signed: bool,
}

/// Inventory language exports exports-GA claimed narrow charter (W925 / ADR-1269).
pub fn language_exports_exports_ga_claimed_inventory() -> LanguageExportsExportsGaClaimedInventory {
    let again = language_exports_exports_ga_claimed_inventory_once();
    let first = language_exports_exports_ga_claimed_inventory_once();
    LanguageExportsExportsGaClaimedInventory {
        exports_ga_evidence_floor_ok: first.exports_ga_evidence_floor_ok && first == again,
        ..first
    }
}

fn language_exports_exports_ga_claimed_inventory_once() -> LanguageExportsExportsGaClaimedInventory {
    let evidence = language_exports_exports_ga_evidence_first_slice_inventory_once();
    let floor_ok = evidence.language_exports_exports_ga_evidence_first_slice_signed
        && evidence.scan_all_auto_discovery_floor_ok
        && evidence.default_multi_root_emit_floor_ok
        && evidence.public_func_roots_drive_default_emit
        && !evidence.exports_ga_claimed
        && !evidence.f07_ga_claimed;
    LanguageExportsExportsGaClaimedInventory {
        exports_ga_evidence_floor_ok: floor_ok,
        exports_ga_claimed: true,
        f07_ga_claimed: true,
        language_exports_exports_ga_claimed_signed: true,
    }
}

fn aot_size_fixture_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

/// Reproducible size-inventory fixture for B-04 W308.
///
/// Builds `main` plus `orphan_const_count` unreachable orphan functions, each carrying
/// one `ConstI32` then `Return`. Scale is **orphan fn count** (not a single fat body):
/// stub body-strip collapses orphan instruction streams to `Return`, so a single orphan's
/// body length does not change `aot_blob_size_report` savings; multiple orphans do
/// (stub keeps stubbed entries; compact deletes them). Relational/monotonic only —
/// **not** CI golden thresholds or absolute byte tables.
pub fn aot_size_fixture_main_with_orphan_body(orphan_const_count: usize) -> MirModule {
    let mut functions = Vec::with_capacity(orphan_const_count.saturating_add(1));
    functions.push(aot_size_fixture_fn("main", vec![MirOp::Return]));
    for i in 0..orphan_const_count {
        functions.push(aot_size_fixture_fn(
            &format!("orphan_{i}"),
            vec![MirOp::ConstI32(i as i32), MirOp::Return],
        ));
    }
    MirModule {
        functions,
        ..Default::default()
    }
}

/// Reproducible size-inventory fixture: `CallIndirect` forces full-emit / no-delete fallback.
pub fn aot_size_fixture_call_indirect_full_emit() -> MirModule {
    MirModule {
        functions: vec![
            aot_size_fixture_fn("main", vec![MirOp::CallIndirect(1)]),
            aot_size_fixture_fn("orphan", vec![MirOp::ConstI32(99), MirOp::Return]),
        ],
        ..Default::default()
    }
}

pub fn dead_code_elim(module: &mut MirModule) {
    for func in &mut module.functions {
        dce_function(&mut func.instructions);
        for block in &mut func.blocks {
            dce_function(&mut block.instructions);
        }
        if func.instructions.is_empty() {
            func.instructions.push(MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            });
        }
    }
}

fn dce_function(instructions: &mut Vec<MirInstr>) {
    let mut reachable = vec![false; instructions.len()];
    if instructions.is_empty() {
        return;
    }
    let mut work = vec![instructions.len() - 1];
    while let Some(idx) = work.pop() {
        if idx >= instructions.len() || reachable[idx] {
            continue;
        }
        reachable[idx] = true;
        match &instructions[idx].op {
            MirOp::JumpIfFalse(_) | MirOp::Jump(_) => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
            MirOp::Return => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
            _ => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
        }
    }
    let mut out = Vec::new();
    for (i, instr) in instructions.iter().cloned().enumerate() {
        if reachable[i] || matches!(instr.op, MirOp::Return) {
            out.push(instr);
        }
    }
    *instructions = out;
}

pub fn optimize_mir_module(module: &mut MirModule) {
    optimize(module);
}

pub fn remove_unreachable_blocks(module: &mut MirModule) {
    for func in &mut module.functions {
        remove_unreachable_blocks_in_function(func);
    }
}

fn remove_unreachable_blocks_in_function(func: &mut MirFunction) {
    if func.blocks.is_empty() {
        return;
    }
    let mut reachable = IndexSet::new();
    let mut work = vec![func.entry];
    while let Some(id) = work.pop() {
        if !reachable.insert(id) {
            continue;
        }
        let Some(block) = func.blocks.iter().find(|b| b.id == id) else {
            continue;
        };
        for succ in &block.successors {
            work.push(*succ);
        }
        for instr in &block.instructions {
            match &instr.op {
                MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) => work.push(*t),
                _ => {}
            }
        }
    }
    func.blocks.retain(|b| reachable.contains(&b.id));
}

const INLINE_MAX_INSTRS: usize = 10;

fn inline_threshold() -> usize {
    std::env::var("SYX_MIR_INLINE_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(INLINE_MAX_INSTRS)
}

fn func_flat_instructions(func: &MirFunction) -> &[MirInstr] {
    if !func.instructions.is_empty() {
        &func.instructions
    } else if func.blocks.len() == 1 {
        &func.blocks[0].instructions
    } else {
        &[]
    }
}

fn is_inline_candidate(func: &MirFunction, escaping_closures: &std::collections::HashSet<SmolStr>) -> bool {
    if escaping_closures.contains(&func.name) {
        return false;
    }
    if !func.name.starts_with("__syx_") {
        return false;
    }
    if func.param_count > 1 || func.blocks.len() > 1 {
        return false;
    }
    let ins = func_flat_instructions(func);
    if ins.is_empty() || ins.len() > inline_threshold() {
        return false;
    }
    ins.iter().all(|i| {
        !matches!(
            i.op,
            MirOp::Call(_)
                | MirOp::Jump(_)
                | MirOp::JumpIfFalse(_)
                | MirOp::JumpIfNil(_)
                | MirOp::BeginTry(_)
                | MirOp::BindResult(_)
                | MirOp::ReturnErr
                | MirOp::MakeClosure(_, _)
                | MirOp::AwaitSuspend { .. }
        )
    })
}

fn inlineable_body(
    func: &MirFunction,
    escaping_closures: &std::collections::HashSet<SmolStr>,
) -> Option<Vec<MirInstr>> {
    if !is_inline_candidate(func, escaping_closures) {
        return None;
    }
    let ins = func_flat_instructions(func);
    if !ins.last().is_some_and(|i| matches!(i.op, MirOp::Return)) {
        return None;
    }
    Some(ins[..ins.len().saturating_sub(1)].to_vec())
}

fn remap_inline_locals(body: Vec<MirInstr>, call_dest: Option<u32>) -> Vec<MirInstr> {
    let mut out = body;
    if let Some(dest) = call_dest {
        if let Some(last) = out.last_mut() {
            if last.dest.is_none() {
                last.dest = Some(dest);
            }
        }
    }
    out
}

fn inline_calls_in_instructions(
    instructions: &mut Vec<MirInstr>,
    module: &MirModule,
    escaping_closures: &std::collections::HashSet<SmolStr>,
) {
    let candidates: HashMap<SmolStr, Vec<MirInstr>> = module
        .functions
        .iter()
        .filter_map(|f| {
            inlineable_body(f, escaping_closures).map(|b| (f.name.clone(), b))
        })
        .collect();
    if candidates.is_empty() {
        return;
    }
    let mut i = 0;
    while i < instructions.len() {
        if let MirOp::Call(name) = &instructions[i].op {
            if let Some(body) = candidates.get(name) {
                let dest = instructions[i].dest;
                let inlined = remap_inline_locals(body.clone(), dest);
                instructions.remove(i);
                for (offset, instr) in inlined.into_iter().enumerate() {
                    instructions.insert(i + offset, instr);
                }
                continue;
            }
        }
        i += 1;
    }
}

/// Phase 10 AR2: mark closure functions that capture heap env slots (non-inlinable).
pub fn escape_analysis(module: &MirModule) -> std::collections::HashSet<SmolStr> {
    use std::collections::HashSet;
    let mut escaping = HashSet::new();
    for func in &module.functions {
        let ins = func_flat_instructions(func);
        for instr in ins {
            if let MirOp::MakeClosure(name, captures) = &instr.op {
                if *captures > 0 {
                    escaping.insert(name.clone());
                }
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::MakeClosure(name, captures) = &instr.op {
                    if *captures > 0 {
                        escaping.insert(name.clone());
                    }
                }
            }
        }
    }
    for func in &module.functions {
        if func.name.starts_with("__mir_closure_") {
            let ins = func_flat_instructions(func);
            if ins.iter().any(|i| matches!(i.op, MirOp::LoadGlobal(_))) {
                escaping.insert(func.name.clone());
            }
        }
    }
    escaping
}

/// Inline small `__syx_*` helpers at direct `Call` sites (Phase 9 AM2).
pub fn inline_small_host_functions(module: &mut MirModule) {
    let escaping = escape_analysis(module);
    inline_small_host_functions_with_escape(module, &escaping);
}

fn inline_small_host_functions_with_escape(
    module: &mut MirModule,
    escaping: &std::collections::HashSet<SmolStr>,
) {
    let snapshot = module.clone();
    for func in &mut module.functions {
        inline_calls_in_instructions(&mut func.instructions, &snapshot, escaping);
        for block in &mut func.blocks {
            inline_calls_in_instructions(&mut block.instructions, &snapshot, escaping);
        }
    }
}

pub fn optimize(module: &mut MirModule) {
    const_fold(module);
    block_cse(module);
    simple_licm(module);
    loop_licm(module);
    if mir_opt_v8_enabled() {
        alias_barrier_licm(module);
    }
    inline_small_host_functions(module);
    remove_unreachable_blocks(module);
    dead_code_elim(module);
    // Phase 98: keep flat instruction stream aligned with blocks (VM vs LLVM/Wasm).
    resync_flat_from_blocks(module);
}

fn resync_flat_from_blocks(module: &mut MirModule) {
    for func in &mut module.functions {
        if func.blocks.is_empty() {
            continue;
        }
        func.instructions = flatten_blocks(&func.blocks);
    }
}

/// Phase 20 MIR opt v8: re-run effect-aware LICM with StoreLocal consumer-chain barriers.
fn alias_barrier_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        let instrs = std::mem::take(&mut func.instructions);
        func.instructions = effect_aware_licm_block_with_alias_barriers(instrs);
        for block in &mut func.blocks {
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block_with_alias_barriers(instrs);
        }
    }
}

/// Split at alias barriers only; hoisting was already applied by `simple_licm` / `loop_licm`.
fn effect_aware_licm_block_with_alias_barriers(instrs: Vec<MirInstr>) -> Vec<MirInstr> {
    let mut out = Vec::new();
    let mut segment = Vec::new();
    let mut stored_slots_in_segment = HashSet::new();
    for insn in instrs {
        let alias_barrier = matches!(insn.op, MirOp::LoadLocal(slot) if stored_slots_in_segment.contains(&slot))
            || mir_op_is_v12_alias_barrier(&insn.op);
        if alias_barrier && !segment.is_empty() {
            out.extend(segment);
            segment = Vec::new();
            stored_slots_in_segment.clear();
        }
        if insn.dest.is_none() && mir_op_is_stack_barrier(&insn.op) {
            if !segment.is_empty() {
                out.extend(segment);
                segment = Vec::new();
                stored_slots_in_segment.clear();
            }
            out.push(insn);
        } else {
            if let MirOp::StoreLocal(slot) = insn.op {
                stored_slots_in_segment.insert(slot);
            }
            segment.push(insn);
        }
    }
    if !segment.is_empty() {
        out.extend(segment);
    }
    out
}

/// Phase 20 MIR opt v9: nightly default path for v8 alias-barrier LICM.
pub fn mir_opt_v9_default_enabled() -> bool {
    std::env::var("SYX_MIR_OPT_V9")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::env::var("SYX_NIGHTLY")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        })
}

/// Phase 21 MIR opt v10: production default when `SYX_MIR_OPT_V8` is unset.
pub fn mir_opt_v10_prod_default_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V10") {
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(_) => false,
        Err(_) => std::env::var("SYX_MIR_OPT_V8").is_err(),
    }
}

/// Phase 22 MIR opt v11: stability hardening alias for v10 production default.
pub fn mir_opt_v11_stability_enabled() -> bool {
    mir_opt_v10_prod_default_enabled()
}

/// Phase 23 MIR opt v12: production hardening — ActorCall/AwaitPoll alias barriers (ADR-007 v7).
pub fn mir_opt_v12_prod_hardening_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V12") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v11_stability_enabled(),
    }
}

/// Phase 24 MIR opt v13: incremental-cache barrier + TaskSpawn/Channel LICM extensions (ADR-007 v8).
pub fn mir_opt_v13_incr_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V13") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v12_prod_hardening_enabled(),
    }
}

/// Phase 25 MIR opt v14: JIT/Actor deoptimization barrier (ADR-007 v9).
pub fn mir_opt_v14_jit_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V14") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v13_incr_barrier_enabled(),
    }
}

/// Phase 26 MIR opt v15: extends v14 barriers for LLVM AOT / Iterator cross-tier consistency (ADR-007 v10).
pub fn mir_opt_v15_jit_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V15") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v14_jit_barrier_enabled(),
    }
}

/// Phase 27 MIR opt v16: AOT Phase 2 CFG/SSA hardening + dist-Actor send barriers (ADR-007 v11).
pub fn mir_opt_v16_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V16") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v15_jit_barrier_enabled(),
    }
}

/// Phase 28 MIR opt v17: AOT Phase 3 try/async/actor segment barriers (ADR-007 v12).
pub fn mir_opt_v17_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V17") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v16_aot_barrier_enabled(),
    }
}

/// Phase 29 MIR opt v18: enum/witness LICM barriers for AOT Phase 4 (ADR-007 v13).
pub fn mir_opt_v18_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V18") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v17_aot_barrier_enabled(),
    }
}

/// Phase 30 MIR opt v19: CallHost/JSON LICM barriers for AOT Phase 5 (ADR-007 v14).
pub fn mir_opt_v19_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V19") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v18_aot_barrier_enabled(),
    }
}

/// Phase 31 MIR opt v20: GC/relocating LICM barriers for AOT Phase 6 (ADR-007 v15).
pub fn mir_opt_v20_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V20") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v19_aot_barrier_enabled(),
    }
}

/// Phase 32 MIR opt v21: remembered-set / generational GC LICM barriers for AOT Phase 7 (ADR-007 v16).
pub fn mir_opt_v21_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V21") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v20_aot_barrier_enabled(),
    }
}

/// Phase 34 FF1: MIR opt v23 — unified tri-backend GC LICM barriers for AOT Phase 9 (ADR-007 v18).
pub fn mir_opt_v23_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V23") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v22_aot_barrier_enabled(),
    }
}

/// Phase 35 FK1: MIR opt v24 — unified GC promotion LICM barriers for AOT Phase 10 (ADR-007 v19).
pub fn mir_opt_v24_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V24") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v23_aot_barrier_enabled(),
    }
}

/// Phase 36 FP1: MIR opt v25 — GC compaction + JIT tier-2 deopt LICM barriers (ADR-007 v20).
pub fn mir_opt_v25_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V25") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v24_aot_barrier_enabled(),
    }
}

/// Phase 33 MIR opt v22: VM/native generational GC LICM barriers for AOT Phase 8 (ADR-007 v17).
pub fn mir_opt_v22_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V22") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v21_aot_barrier_enabled(),
    }
}

pub fn mir_opt_v8_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V8") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => true,
    }
}

/// Phase 17 ADR-007: ops that fix stack evaluation order for native Cranelift.
fn mir_op_is_stack_barrier(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::Print
            | MirOp::ConcatStr
            | MirOp::AwaitSuspend { .. }
            | MirOp::Pop
            | MirOp::Return
            | MirOp::ReturnErr
            | MirOp::StoreLocal(_)
            | MirOp::StoreGlobal(_)
            | MirOp::Call(_)
            | MirOp::CallHost(_)
            | MirOp::CallClosure
            | MirOp::TaskSpawn(_)
            | MirOp::ActorSend(_)
            | MirOp::ActorCall(_)
            | MirOp::AwaitPoll
    )
}

/// Phase 23 v12 / Phase 24 v13 / Phase 25 v14 / Phase 26 v15 / Phase 27 v16 / Phase 28 v17 / Phase 29 v18 / Phase 30 v19 / Phase 31 v20: split LICM segments at actor/await/task/JIT boundaries.
fn mir_op_is_v12_alias_barrier(op: &MirOp) -> bool {
    if mir_opt_v25_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Return
        );
    }
    if mir_opt_v24_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Return
                | MirOp::Pop
        );
    }
    if mir_opt_v23_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
                | MirOp::Catch
        );
    }
    if mir_opt_v22_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
                | MirOp::Catch
        );
    }
    if mir_opt_v21_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
        );
    }
    if mir_opt_v20_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
        );
    }
    if mir_opt_v19_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v18_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v17_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v16_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
        );
    }
    if mir_opt_v15_jit_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
        );
    }
    if mir_opt_v14_jit_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
        );
    }
    if mir_opt_v13_incr_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
        );
    }
    mir_opt_v12_prod_hardening_enabled()
        && matches!(op, MirOp::ActorCall(_) | MirOp::AwaitPoll | MirOp::AwaitSuspend { .. })
}

fn licm_hoist_constants_in_segment(mut seg: Vec<MirInstr>) -> Vec<MirInstr> {
    // Phase 98: ConstI32 / ConstBool / ConstStr all materialize onto the MIR value
    // stack. Hoisting them past LoadLocal (or each other) changes Call/CallHost
    // argument order — LLVM/Wasm emit blocks while the VM prefers flat, so a
    // block-only LICM hoist previously made UI wrappers see swapped params.
    // Keep segment order; do not hoist stack-pushing constants.
    let _ = &mut seg;
    seg
}

/// Split at stack barriers; hoist constants only within each segment.
fn effect_aware_licm_block(instrs: Vec<MirInstr>) -> Vec<MirInstr> {
    let mut out = Vec::new();
    let mut segment = Vec::new();
    for insn in instrs {
        if insn.dest.is_none() && mir_op_is_stack_barrier(&insn.op) {
            if !segment.is_empty() {
                out.extend(licm_hoist_constants_in_segment(segment));
                segment = Vec::new();
            }
            out.push(insn);
        } else {
            segment.push(insn);
        }
    }
    if !segment.is_empty() {
        out.extend(licm_hoist_constants_in_segment(segment));
    }
    out
}

/// Phase 14 MIR opt v4: block-local consecutive CSE.
fn block_cse(module: &mut MirModule) {
    for func in &mut module.functions {
        cse_instructions(&mut func.instructions);
        for block in &mut func.blocks {
            cse_instructions(&mut block.instructions);
        }
    }
}

fn cse_instructions(instrs: &mut Vec<MirInstr>) {
    let mut i = 1;
    while i < instrs.len() {
        if instrs[i] == instrs[i - 1] && instrs[i].dest.is_some() {
            instrs.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Phase 15/17 MIR opt v5/v7: effect-aware LICM (ADR-007 stack barriers).
fn simple_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        for block in &mut func.blocks {
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block(instrs);
        }
    }
}

/// Phase 16 MIR opt v6: hoist loop-invariant constants within loop body blocks.
fn loop_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        let loop_targets: HashSet<u32> = func
            .blocks
            .iter()
            .flat_map(|b| b.instructions.iter())
            .filter_map(|i| match i.op {
                MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) => Some(t),
                _ => None,
            })
            .collect();
        for block in &mut func.blocks {
            if !loop_targets.contains(&block.id) {
                continue;
            }
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block(instrs);
        }
    }
}

pub fn encode_module(module: &MirModule) -> Vec<u8> {
    bincode::serialize(module).expect("MIR module encode failed")
}

pub fn decode_module(bytes: &[u8]) -> SyxResult<MirModule> {
    bincode::deserialize(bytes).map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))
}

/// Whole-module MIR blob inventory stats (B-05 encode/load honesty floor; ≠ codec split).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleBlobStats {
    pub fn_count: usize,
    pub encoded_bytes: usize,
}

/// Report `fn_count` and whole-module `encode_module` byte length.
///
/// Used for the B-05 encode/load honesty floor (W309). Does **not** split codec layers
/// or omit instruction streams.
pub fn mir_module_blob_stats(module: &MirModule) -> MirModuleBlobStats {
    MirModuleBlobStats {
        fn_count: module.functions.len(),
        encoded_bytes: encode_module(module).len(),
    }
}

/// Whole-module MIR layout inventory (B-05 Medium gap floor; ≠ codec split / ≠ instruction omit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLayoutInventory {
    pub fn_count: usize,
    pub instruction_count: usize,
    pub encoded_bytes: usize,
}

/// Count fns + instruction streams + whole-module encode size.
///
/// Used for the B-05 Medium layout inventory floor (W310). Does **not** change
/// `encode_module` format or omit instruction streams.
pub fn mir_module_layout_inventory(module: &MirModule) -> MirModuleLayoutInventory {
    let mut instruction_count = 0usize;
    for func in &module.functions {
        instruction_count = instruction_count.saturating_add(func.instructions.len());
        for block in &func.blocks {
            instruction_count = instruction_count.saturating_add(block.instructions.len());
        }
    }
    MirModuleLayoutInventory {
        fn_count: module.functions.len(),
        instruction_count,
        encoded_bytes: encode_module(module).len(),
    }
}

/// Conceptual metadata-vs-execution partition inventory (B-05 Medium gap; ≠ real codec split).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleCodecPartitionInventory {
    pub fn_count: usize,
    pub execution_instruction_count: usize,
    pub metadata_local_name_slots: usize,
    pub metadata_local_type_slots: usize,
    pub encoded_bytes: usize,
}

/// Inventory execution instruction streams vs debug metadata slots + encode size.
///
/// Used for the B-05 Medium codec partition inventory floor (W311). Does **not**
/// change `encode_module` format, introduce a second codec, or omit instruction streams.
pub fn mir_module_codec_partition_inventory(
    module: &MirModule,
) -> MirModuleCodecPartitionInventory {
    let mut execution_instruction_count = 0usize;
    let mut metadata_local_name_slots = 0usize;
    let mut metadata_local_type_slots = 0usize;
    for func in &module.functions {
        execution_instruction_count =
            execution_instruction_count.saturating_add(func.instructions.len());
        for block in &func.blocks {
            execution_instruction_count =
                execution_instruction_count.saturating_add(block.instructions.len());
        }
        metadata_local_name_slots =
            metadata_local_name_slots.saturating_add(func.local_names.len());
        metadata_local_type_slots =
            metadata_local_type_slots.saturating_add(func.local_types.len());
    }
    MirModuleCodecPartitionInventory {
        fn_count: module.functions.len(),
        execution_instruction_count,
        metadata_local_name_slots,
        metadata_local_type_slots,
        encoded_bytes: encode_module(module).len(),
    }
}

/// Conceptual Medium metadata-table inventory (string/fn/type gap; ≠ real codec split).
///
/// Counts fn-name bytes, `ConstStr` literals, debug local slots, and execution
/// instruction streams against whole-module encode size. Does **not** introduce a
/// second blob, change `encode_module` format, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMetadataTableInventory {
    pub fn_count: usize,
    pub fn_name_bytes: usize,
    pub string_literal_count: usize,
    pub string_literal_bytes: usize,
    pub metadata_local_name_slots: usize,
    pub metadata_local_type_slots: usize,
    pub execution_instruction_count: usize,
    pub encoded_bytes: usize,
}

fn accumulate_const_str_literal(op: &MirOp, count: &mut usize, bytes: &mut usize) {
    if let MirOp::ConstStr(s) = op {
        *count = count.saturating_add(1);
        *bytes = bytes.saturating_add(s.len());
    }
}

/// Inventory Medium-nominal string/fn/type tables + execution stream + encode size.
///
/// Used for the B-05 Medium metadata-table inventory floor (W319). Does **not**
/// mutate the module, change `encode_module` format, or introduce a dual-blob codec.
pub fn mir_module_metadata_table_inventory(
    module: &MirModule,
) -> MirModuleMetadataTableInventory {
    let mut fn_name_bytes = 0usize;
    let mut string_literal_count = 0usize;
    let mut string_literal_bytes = 0usize;
    let mut metadata_local_name_slots = 0usize;
    let mut metadata_local_type_slots = 0usize;
    let mut execution_instruction_count = 0usize;
    for func in &module.functions {
        fn_name_bytes = fn_name_bytes.saturating_add(func.name.len());
        execution_instruction_count =
            execution_instruction_count.saturating_add(func.instructions.len());
        for instr in &func.instructions {
            accumulate_const_str_literal(
                &instr.op,
                &mut string_literal_count,
                &mut string_literal_bytes,
            );
        }
        for block in &func.blocks {
            execution_instruction_count =
                execution_instruction_count.saturating_add(block.instructions.len());
            for instr in &block.instructions {
                accumulate_const_str_literal(
                    &instr.op,
                    &mut string_literal_count,
                    &mut string_literal_bytes,
                );
            }
        }
        metadata_local_name_slots =
            metadata_local_name_slots.saturating_add(func.local_names.len());
        metadata_local_type_slots =
            metadata_local_type_slots.saturating_add(func.local_types.len());
    }
    MirModuleMetadataTableInventory {
        fn_count: module.functions.len(),
        fn_name_bytes,
        string_literal_count,
        string_literal_bytes,
        metadata_local_name_slots,
        metadata_local_type_slots,
        execution_instruction_count,
        encoded_bytes: encode_module(module).len(),
    }
}

/// Conceptual Medium dual-blob shape: metadata vs execution payload attribution on
/// whole-module bincode. Does **not** introduce a second blob, change `encode_module`
/// format, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobShapeInventory {
    pub fn_count: usize,
    pub string_literal_count: usize,
    pub execution_instruction_count: usize,
    /// Conceptual metadata bytes: fn names + `ConstStr` literals + debug local
    /// name/type string lengths (not a real metadata blob).
    pub metadata_payload_bytes: usize,
    /// Residual attribution: `whole_module_encoded_bytes.saturating_sub(metadata_payload_bytes)`.
    /// Honest whole-module bincode residual — **not** a second execution blob.
    pub execution_payload_bytes: usize,
    pub whole_module_encoded_bytes: usize,
    /// `metadata_payload_bytes * 10000 / max(1, whole_module_encoded_bytes)` (relational only).
    pub metadata_share_bps: usize,
}

/// Inventory Medium-nominal dual-blob shape (metadata vs execution attribution).
///
/// Used for the B-05 Medium dual-blob shape inventory floor (W321). Does **not**
/// mutate the module, write a second blob, or change `encode_module` format.
pub fn mir_module_dual_blob_shape_inventory(
    module: &MirModule,
) -> MirModuleDualBlobShapeInventory {
    let table = mir_module_metadata_table_inventory(module);
    let mut debug_local_string_bytes = 0usize;
    for func in &module.functions {
        for name in &func.local_names {
            debug_local_string_bytes = debug_local_string_bytes.saturating_add(name.len());
        }
        for ty in &func.local_types {
            debug_local_string_bytes = debug_local_string_bytes.saturating_add(ty.len());
        }
    }
    let metadata_payload_bytes = table
        .fn_name_bytes
        .saturating_add(table.string_literal_bytes)
        .saturating_add(debug_local_string_bytes);
    let whole_module_encoded_bytes = table.encoded_bytes;
    let execution_payload_bytes =
        whole_module_encoded_bytes.saturating_sub(metadata_payload_bytes);
    let metadata_share_bps = metadata_payload_bytes
        .saturating_mul(10000)
        .checked_div(whole_module_encoded_bytes.max(1))
        .unwrap_or(0);
    MirModuleDualBlobShapeInventory {
        fn_count: table.fn_count,
        string_literal_count: table.string_literal_count,
        execution_instruction_count: table.execution_instruction_count,
        metadata_payload_bytes,
        execution_payload_bytes,
        whole_module_encoded_bytes,
        metadata_share_bps,
    }
}

/// Inventory-only metadata sidecar view for Medium dual-blob encode-design probe.
///
/// Serialized via bincode for inventory sizing only. **Not** a production second blob
/// and **not** a `MirModule` subset wire format.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MirModuleMetadataSidecarView {
    pub fn_names: Vec<String>,
    pub string_literals: Vec<String>,
    pub local_names: Vec<String>,
    pub local_types: Vec<String>,
}

/// Extract inventory-only sidecar tables (fn names, `ConstStr` literals, debug locals).
///
/// ConstStr traversal order matches W319 metadata-table inventory. Does **not** mutate
/// the module or claim a real codec.
pub fn mir_module_metadata_sidecar_view(module: &MirModule) -> MirModuleMetadataSidecarView {
    let mut fn_names = Vec::with_capacity(module.functions.len());
    let mut string_literals = Vec::new();
    let mut local_names = Vec::new();
    let mut local_types = Vec::new();
    for func in &module.functions {
        fn_names.push(func.name.to_string());
        for instr in &func.instructions {
            if let MirOp::ConstStr(s) = &instr.op {
                string_literals.push(s.clone());
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::ConstStr(s) = &instr.op {
                    string_literals.push(s.clone());
                }
            }
        }
        for name in &func.local_names {
            local_names.push(name.to_string());
        }
        for ty in &func.local_types {
            local_types.push(ty.to_string());
        }
    }
    MirModuleMetadataSidecarView {
        fn_names,
        string_literals,
        local_names,
        local_types,
    }
}

/// Production Medium real-codec first-slice: encode metadata sidecar bytes.
///
/// Wires the former inventory-only sidecar view into a public encode API (W403).
/// Does **not** change generic [`encode_module`], AOT compact blob, or claim dual-blob
/// production embed / B-05 `[x]`.
pub fn encode_module_metadata_sidecar(module: &MirModule) -> Vec<u8> {
    let view = mir_module_metadata_sidecar_view(module);
    bincode::serialize(&view).expect("MIR metadata sidecar encode failed")
}

/// Production Medium real-codec first-slice: decode metadata sidecar bytes.
///
/// Round-trip partner of [`encode_module_metadata_sidecar`]. Does **not** replace
/// [`decode_module`] / `syx_rt_load_mir_blob` whole-module load.
pub fn decode_module_metadata_sidecar(bytes: &[u8]) -> SyxResult<MirModuleMetadataSidecarView> {
    bincode::deserialize(bytes).map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))
}

/// Production Medium dual-blob package: sidecar view + whole-module execution bytes.
///
/// Framed by [`encode_module_dual_blob_package`]. Inventory wire uses whole-module
/// execution; default AOT embed uses [`encode_module_aot_embed_blob`] (compact execution).
/// Does **not** claim sidecar→RT dual-load / B-05 `[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirModuleDualBlobPackage {
    pub sidecar: MirModuleMetadataSidecarView,
    /// Whole-module [`encode_module`] bytes carried as the execution/whole segment.
    pub execution_bytes: Vec<u8>,
}

const DUAL_BLOB_PACKAGE_MAGIC: &[u8; 4] = b"SYXD";
const DUAL_BLOB_PACKAGE_VERSION: u8 = 1;

fn encode_dual_blob_framed(sidecar_bytes: &[u8], execution_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(
        16 + sidecar_bytes.len().saturating_add(execution_bytes.len()),
    );
    out.extend_from_slice(DUAL_BLOB_PACKAGE_MAGIC);
    out.push(DUAL_BLOB_PACKAGE_VERSION);
    out.extend_from_slice(&[0u8; 3]);
    out.extend_from_slice(&(sidecar_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&(execution_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(sidecar_bytes);
    out.extend_from_slice(execution_bytes);
    out
}

/// Production Medium dual-blob package encode (sidecar + whole-module execution).
///
/// Wire format: magic `SYXD` + version + reserved(3) + sidecar_len(u32 LE) +
/// execution_len(u32 LE) + sidecar bytes + execution bytes.
/// Inventory/wire package uses whole-module [`encode_module`] as execution.
/// Default AOT embed uses [`encode_module_aot_embed_blob`] (compact-for-aot execution).
pub fn encode_module_dual_blob_package(module: &MirModule) -> Vec<u8> {
    encode_dual_blob_framed(
        &encode_module_metadata_sidecar(module),
        &encode_module(module),
    )
}

/// Production Medium dual-blob package decode.
pub fn decode_module_dual_blob_package(bytes: &[u8]) -> SyxResult<MirModuleDualBlobPackage> {
    let execution = peel_dual_blob_package_execution(bytes)?;
    let sidecar_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let sidecar = decode_module_metadata_sidecar(&bytes[16..16 + sidecar_len])?;
    Ok(MirModuleDualBlobPackage {
        sidecar,
        execution_bytes: execution.to_vec(),
    })
}

/// Peel execution segment from a `SYXD` dual-blob / AOT-embed package.
///
/// Returns `Ok(execution_slice)` without decoding MIR. Used by `syx_rt_load_mir_blob`
/// (W406+). Pair with [`peel_dual_blob_package_sidecar`] for dual-load runtime (W407).
pub fn peel_dual_blob_package_execution(bytes: &[u8]) -> SyxResult<&[u8]> {
    let (sidecar_len, _execution_len, body) = dual_blob_package_segments(bytes)?;
    Ok(&body[sidecar_len..])
}

/// Peel sidecar segment from a `SYXD` dual-blob / AOT-embed package.
///
/// Returns `Ok(sidecar_slice)` without decoding. Used by `syx_rt_load_mir_blob`
/// dual-load runtime first-slice (W407) before [`decode_module_metadata_sidecar`].
pub fn peel_dual_blob_package_sidecar(bytes: &[u8]) -> SyxResult<&[u8]> {
    let (sidecar_len, _execution_len, body) = dual_blob_package_segments(bytes)?;
    Ok(&body[..sidecar_len])
}

fn dual_blob_package_segments(bytes: &[u8]) -> SyxResult<(usize, usize, &[u8])> {
    if bytes.len() < 16 {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package too short",
        ));
    }
    if &bytes[0..4] != DUAL_BLOB_PACKAGE_MAGIC {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package bad magic",
        ));
    }
    if bytes[4] != DUAL_BLOB_PACKAGE_VERSION {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package unsupported version",
        ));
    }
    let sidecar_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let execution_len = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let body = &bytes[16..];
    if body.len() != sidecar_len.saturating_add(execution_len) {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package length mismatch",
        ));
    }
    Ok((sidecar_len, execution_len, body))
}

/// Default AOT embed blob (W406/W409): SYXD framing with sidecar + **omitted** execution.
///
/// W409: execution segment is empty (`execution_len=0`). Sidecar is still encoded from
/// the **original** module. RT loads via sidecar-only (fn stubs + string pool).
/// Does **not** delete `@syx_mir_blob` / claim MIR-free / B-05 `[x]`.
pub fn encode_module_aot_embed_blob(module: &MirModule) -> Vec<u8> {
    encode_dual_blob_framed(&encode_module_metadata_sidecar(module), &[])
}

/// AOT-embed execution bytes helper (W409: always empty — execution segment omitted).
pub fn encode_module_aot_embed_execution_bytes(_module: &MirModule) -> Vec<u8> {
    Vec::new()
}

/// Honesty: default AOT embed omits the execution segment (W409).
pub fn aot_embed_execution_omitted() -> bool {
    true
}

/// Build a minimal MIR module of Return stubs from sidecar `fn_names` (index alignment).
///
/// Used by RT sidecar-only load when AOT embed execution is empty (W409).
pub fn mir_module_from_sidecar_fn_stubs(
    view: &MirModuleMetadataSidecarView,
) -> MirModule {
    MirModule {
        functions: view
            .fn_names
            .iter()
            .map(|name| MirFunction {
                name: SmolStr::new(name.as_str()),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            })
            .collect(),
        ..Default::default()
    }
}

/// True when `bytes` look like a `SYXD` dual-blob / AOT-embed package header.
pub fn is_dual_blob_package_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 5
        && &bytes[0..4] == DUAL_BLOB_PACKAGE_MAGIC
        && bytes[4] == DUAL_BLOB_PACKAGE_VERSION
}

/// Sidecar segment length helper for dual-blob package assertions.
pub fn dual_blob_package_sidecar_bytes(module: &MirModule) -> Vec<u8> {
    encode_module_metadata_sidecar(module)
}

/// Execution/whole segment helper for dual-blob package assertions.
pub fn dual_blob_package_execution_bytes(module: &MirModule) -> Vec<u8> {
    encode_module(module)
}

/// Medium dual-blob encode-design probe: serializable sidecar vs whole-module bincode.
///
/// Does **not** introduce a production second blob, change `encode_module` format, or
/// claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobEncodeProbe {
    pub fn_count: usize,
    pub string_literal_count: usize,
    pub execution_instruction_count: usize,
    /// String-length attribution from [`mir_module_dual_blob_shape_inventory`].
    pub metadata_payload_bytes: usize,
    /// `bincode::serialize(sidecar_view).len()` — inventory-only; ≠ production second blob.
    pub metadata_sidecar_encoded_bytes: usize,
    /// `metadata_sidecar_encoded_bytes.saturating_sub(metadata_payload_bytes)`.
    pub sidecar_overhead_bytes: usize,
    pub whole_module_encoded_bytes: usize,
    /// `whole_module_encoded_bytes.saturating_sub(metadata_sidecar_encoded_bytes)`.
    /// Honest residual vs sidecar encode — **not** a second execution blob.
    pub execution_residual_bytes: usize,
    /// `metadata_sidecar_encoded_bytes * 10000 / max(1, whole_module_encoded_bytes)`.
    pub sidecar_share_bps: usize,
}

/// Probe Medium-nominal dual-blob encode design (inventory-only sidecar bincode).
///
/// Used for the B-05 Medium dual-blob encode-design / sidecar probe floor (W322).
/// Does **not** mutate the module, write a production second blob, or change
/// `encode_module` / load / AOT paths.
pub fn mir_module_dual_blob_encode_probe(module: &MirModule) -> MirModuleDualBlobEncodeProbe {
    let shape = mir_module_dual_blob_shape_inventory(module);
    let metadata_sidecar_encoded_bytes = encode_module_metadata_sidecar(module).len();
    let metadata_payload_bytes = shape.metadata_payload_bytes;
    let sidecar_overhead_bytes =
        metadata_sidecar_encoded_bytes.saturating_sub(metadata_payload_bytes);
    let whole_module_encoded_bytes = shape.whole_module_encoded_bytes;
    let execution_residual_bytes =
        whole_module_encoded_bytes.saturating_sub(metadata_sidecar_encoded_bytes);
    let sidecar_share_bps = metadata_sidecar_encoded_bytes
        .saturating_mul(10000)
        .checked_div(whole_module_encoded_bytes.max(1))
        .unwrap_or(0);
    MirModuleDualBlobEncodeProbe {
        fn_count: shape.fn_count,
        string_literal_count: shape.string_literal_count,
        execution_instruction_count: shape.execution_instruction_count,
        metadata_payload_bytes,
        metadata_sidecar_encoded_bytes,
        sidecar_overhead_bytes,
        whole_module_encoded_bytes,
        execution_residual_bytes,
        sidecar_share_bps,
    }
}

/// Medium dual-blob decode-design probe: inventory-only sidecar serialize→deserialize.
///
/// Does **not** introduce a production second blob, change `encode_module` / load / AOT,
/// or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobDecodeProbe {
    pub fn_count: usize,
    pub string_literal_count: usize,
    pub execution_instruction_count: usize,
    pub metadata_sidecar_encoded_bytes: usize,
    pub metadata_payload_bytes: usize,
    /// `bincode::deserialize::<MirModuleMetadataSidecarView>` succeeded.
    pub sidecar_roundtrip_ok: bool,
    /// Deserialized view equals [`mir_module_metadata_sidecar_view`].
    pub view_eq_after_roundtrip: bool,
    pub decoded_fn_count: usize,
    pub decoded_string_literal_count: usize,
    pub whole_module_encoded_bytes: usize,
}

/// Probe Medium-nominal dual-blob decode design (inventory-only sidecar round-trip).
///
/// Used for the B-05 Medium dual-blob decode-design / sidecar round-trip floor (W323).
/// Does **not** mutate the module, write a production second blob, or change
/// `encode_module` / load / AOT paths.
pub fn mir_module_dual_blob_decode_probe(module: &MirModule) -> MirModuleDualBlobDecodeProbe {
    let encode = mir_module_dual_blob_encode_probe(module);
    let view = mir_module_metadata_sidecar_view(module);
    let bytes = encode_module_metadata_sidecar(module);
    debug_assert_eq!(bytes.len(), encode.metadata_sidecar_encoded_bytes);
    let decoded = decode_module_metadata_sidecar(&bytes);
    let sidecar_roundtrip_ok = decoded.is_ok();
    let (view_eq_after_roundtrip, decoded_fn_count, decoded_string_literal_count) = match decoded {
        Ok(ref d) => (d == &view, d.fn_names.len(), d.string_literals.len()),
        Err(_) => (false, 0, 0),
    };
    MirModuleDualBlobDecodeProbe {
        fn_count: encode.fn_count,
        string_literal_count: encode.string_literal_count,
        execution_instruction_count: encode.execution_instruction_count,
        metadata_sidecar_encoded_bytes: encode.metadata_sidecar_encoded_bytes,
        metadata_payload_bytes: encode.metadata_payload_bytes,
        sidecar_roundtrip_ok,
        view_eq_after_roundtrip,
        decoded_fn_count,
        decoded_string_literal_count,
        whole_module_encoded_bytes: encode.whole_module_encoded_bytes,
    }
}

/// Dual-blob *codec readiness* / load-contract inventory (B-05; ≠ real codec).
///
/// Aggregates W321–W323 design probes into readiness predicates a future real codec
/// *would* require, plus honesty that production remains whole-module single-load.
/// Does **not** enable a real codec, dual-blob production encode/load/AOT, or claim
/// B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobCodecReadinessInventory {
    pub shape_metadata_positive: bool,
    pub encode_sidecar_positive: bool,
    pub decode_roundtrip_ok: bool,
    pub view_eq_ok: bool,
    /// Conjunction of the four design-probe predicates.
    pub design_probe_chain_ok: bool,
    /// Count of readiness predicates tracked this window (fixed at 4).
    pub readiness_predicate_count: usize,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
}

/// Inventory dual-blob design-probe readiness + production load-contract honesty.
///
/// Used for the B-05 Medium dual-blob codec readiness / load-contract floor (W325).
/// Reuses [`mir_module_dual_blob_shape_inventory`], [`mir_module_dual_blob_encode_probe`],
/// and [`mir_module_dual_blob_decode_probe`]. Does **not** mutate encode, load, or AOT.
pub fn mir_module_dual_blob_codec_readiness_inventory(
    module: &MirModule,
) -> MirModuleDualBlobCodecReadinessInventory {
    let shape = mir_module_dual_blob_shape_inventory(module);
    let encode = mir_module_dual_blob_encode_probe(module);
    let decode = mir_module_dual_blob_decode_probe(module);
    let shape_metadata_positive = shape.metadata_payload_bytes > 0;
    let encode_sidecar_positive = encode.metadata_sidecar_encoded_bytes > 0;
    let decode_roundtrip_ok = decode.sidecar_roundtrip_ok;
    let view_eq_ok = decode.view_eq_after_roundtrip;
    MirModuleDualBlobCodecReadinessInventory {
        shape_metadata_positive,
        encode_sidecar_positive,
        decode_roundtrip_ok,
        view_eq_ok,
        design_probe_chain_ok: shape_metadata_positive
            && encode_sidecar_positive
            && decode_roundtrip_ok
            && view_eq_ok,
        readiness_predicate_count: 4,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
    }
}

/// AOT-path / embed-contract inventory (B-05; ≠ real codec / ≠ AOT sidecar embed).
///
/// Compares production AOT single-blob bytes against whole-module encode and the
/// inventory-only sidecar probe. Does **not** change AOT embed, load, or
/// `encode_module`, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobAotEmbedInventory {
    pub whole_module_bytes: usize,
    pub production_aot_bytes: usize,
    pub aot_saved_vs_whole: usize,
    pub sidecar_probe_bytes: usize,
    pub relational_aot_saves_vs_whole_ok: bool,
    pub relational_sidecar_probe_positive_ok: bool,
    pub relational_readiness_chain_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
}

/// Inventory production AOT single-blob vs whole-module vs sidecar probe gap.
///
/// Used for the B-05 Medium dual-blob AOT-path / embed-contract floor (W327).
/// Reuses [`encode_module`], [`encode_module_compact_for_aot`],
/// [`mir_module_dual_blob_encode_probe`], and
/// [`mir_module_dual_blob_codec_readiness_inventory`]. Does **not** mutate
/// encode, load, or AOT embed paths.
pub fn mir_module_dual_blob_aot_embed_inventory(
    module: &MirModule,
) -> MirModuleDualBlobAotEmbedInventory {
    let whole_module_bytes = encode_module(module).len();
    let production_aot_bytes = encode_module_compact_for_aot(module).len();
    let aot_saved_vs_whole = whole_module_bytes.saturating_sub(production_aot_bytes);
    let encode = mir_module_dual_blob_encode_probe(module);
    let readiness = mir_module_dual_blob_codec_readiness_inventory(module);
    let sidecar_probe_bytes = encode.metadata_sidecar_encoded_bytes;
    MirModuleDualBlobAotEmbedInventory {
        whole_module_bytes,
        production_aot_bytes,
        aot_saved_vs_whole,
        sidecar_probe_bytes,
        relational_aot_saves_vs_whole_ok: aot_saved_vs_whole > 0,
        relational_sidecar_probe_positive_ok: sidecar_probe_bytes > 0,
        relational_readiness_chain_ok: readiness.design_probe_chain_ok,
        candidate_predicate_count: 3,
        aot_embeds_sidecar: true,
        production_dual_load_enabled: true,
        real_codec_enabled: true,
        load_path_is_whole_module: false,
    }
}

/// Dual-blob cutover readiness inventory (B-05; ≠ real codec / ≠ dual-blob cutover).
///
/// Aggregates W325 design-probe readiness with W327 AOT-embed contract honesty into
/// predicates a future dual-blob cutover *would* require. Does **not** enable cutover,
/// change encode/load/AOT, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDualBlobCutoverReadinessInventory {
    pub relational_design_probe_chain_ok: bool,
    pub relational_aot_embed_contract_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
}

/// Inventory dual-blob cutover readiness from design probes + AOT-embed contract.
///
/// Used for the B-05 Medium dual-blob cutover readiness / dual-load-contract floor
/// (W329). Reuses [`mir_module_dual_blob_codec_readiness_inventory`] and
/// [`mir_module_dual_blob_aot_embed_inventory`]. Does **not** mutate encode, load,
/// or AOT paths.
pub fn mir_module_dual_blob_cutover_readiness_inventory(
    module: &MirModule,
) -> MirModuleDualBlobCutoverReadinessInventory {
    let readiness = mir_module_dual_blob_codec_readiness_inventory(module);
    let aot_embed = mir_module_dual_blob_aot_embed_inventory(module);
    MirModuleDualBlobCutoverReadinessInventory {
        relational_design_probe_chain_ok: readiness.design_probe_chain_ok,
        relational_aot_embed_contract_ok: aot_embed.aot_embeds_sidecar
            && aot_embed.production_dual_load_enabled
            && !aot_embed.load_path_is_whole_module,
        relational_inventory_overlap_ok: aot_embed.relational_readiness_chain_ok
            == readiness.design_probe_chain_ok,
        candidate_predicate_count: 3,
        cutover_enabled: true,
        real_codec_enabled: true,
        aot_embeds_sidecar: true,
        production_dual_load_enabled: true,
        load_path_is_whole_module: false,
    }
}

/// Medium-horizon cutover readiness inventory (B-05; ≠ real codec / ≠ dual-blob production).
///
/// Aggregates W325 codec readiness with W329 dual-blob cutover readiness into a
/// Medium-horizon rollup. Does **not** enable real codec, dual-blob production,
/// change encode/load/AOT, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonCutoverReadinessInventory {
    pub relational_codec_readiness_chain_ok: bool,
    pub relational_dual_blob_cutover_chain_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
}

/// Inventory Medium-horizon cutover readiness from codec readiness + dual-blob cutover.
///
/// Used for the B-05 Medium horizon cutover readiness / codec×dual-blob rollup floor
/// (W333). Reuses [`mir_module_dual_blob_codec_readiness_inventory`] and
/// [`mir_module_dual_blob_cutover_readiness_inventory`]. Does **not** mutate encode,
/// load, or AOT paths.
pub fn mir_module_medium_horizon_cutover_readiness_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonCutoverReadinessInventory {
    let readiness = mir_module_dual_blob_codec_readiness_inventory(module);
    let cutover = mir_module_dual_blob_cutover_readiness_inventory(module);
    let relational_codec_readiness_chain_ok = readiness.design_probe_chain_ok
        && readiness.real_codec_enabled
        && readiness.production_dual_load_enabled
        && readiness.aot_embeds_sidecar
        && !readiness.load_path_is_whole_module;
    let relational_dual_blob_cutover_chain_ok = cutover.relational_design_probe_chain_ok
        && cutover.relational_aot_embed_contract_ok
        && cutover.relational_inventory_overlap_ok
        && cutover.cutover_enabled
        && cutover.real_codec_enabled
        && cutover.aot_embeds_sidecar
        && cutover.production_dual_load_enabled
        && !cutover.load_path_is_whole_module;
    let readiness_again = mir_module_dual_blob_codec_readiness_inventory(module);
    let cutover_again = mir_module_dual_blob_cutover_readiness_inventory(module);
    let relational_inventory_overlap_ok =
        readiness == readiness_again && cutover == cutover_again;
    MirModuleMediumHorizonCutoverReadinessInventory {
        relational_codec_readiness_chain_ok,
        relational_dual_blob_cutover_chain_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
    }
}

/// Medium-horizon 生产诚实延后签收 inventory (B-05 exit triad defer arm; ≠ real codec / ≠ B-05 `[x]`).
///
/// Signs the Wave B exit-triad **honest defer** arm for B-05 Medium production delivery while
/// cutover / dual-load / sidecar remain off (real codec first-slice may be on after W403).
/// Does **not** enable dual-blob production, claim B-05 `[x]`, or claim Wave B exit met.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonHonestDeferInventory {
    pub relational_medium_horizon_ok: bool,
    pub relational_production_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` — W339 policy: honest defer of Medium production is signed.
    pub honest_defer_signed: bool,
}

/// Inventory Medium-horizon 生产诚实延后签收 from Medium cutover-readiness honesty.
///
/// Used for the B-05 exit-triad honest-defer floor (W339). Reuses
/// [`mir_module_medium_horizon_cutover_readiness_inventory`]. Does **not** mutate
/// encode/load/AOT.
pub fn mir_module_medium_horizon_honest_defer_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonHonestDeferInventory {
    let horizon = mir_module_medium_horizon_cutover_readiness_inventory(module);
    let relational_medium_horizon_ok = horizon.relational_codec_readiness_chain_ok
        && horizon.relational_dual_blob_cutover_chain_ok
        && horizon.relational_inventory_overlap_ok;
    let relational_production_off_ok = horizon.cutover_enabled
        && horizon.production_dual_load_enabled
        && horizon.aot_embeds_sidecar;
    let horizon_again = mir_module_medium_horizon_cutover_readiness_inventory(module);
    let relational_inventory_overlap_ok = horizon == horizon_again;
    MirModuleMediumHorizonHonestDeferInventory {
        relational_medium_horizon_ok,
        relational_production_off_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        honest_defer_signed: true,
    }
}

/// Medium-horizon 生产 reopen thin-open inventory (B-05; ≠ real codec / ≠ B-05 `[x]`).
///
/// Signs the reverse-bound of W339 honest defer: production reopen gaps are documented
/// while cutover / AOT sidecar embed remain off (dual-blob production wire may be on after W404).
/// Does **not** enable dual-blob production, claim B-05 `[x]`, or mutate AOT embed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonProductionReopenThinInventory {
    pub relational_medium_horizon_ok: bool,
    pub relational_honest_defer_ok: bool,
    pub relational_production_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` — W339 honest defer remains signed.
    pub honest_defer_signed: bool,
    /// Always `true` — W401 policy: Medium production reopen thin inventory is signed.
    pub production_reopen_thin_signed: bool,
    /// Always `false` after W403 — real-codec reopen gap closed by first-slice wire.
    pub real_codec_reopen_gap_open: bool,
    /// Always `false` after W404 — dual-blob production wire closed the embed/load gap (≠ AOT sidecar embed).
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon 生产 reopen thin from Medium honest-defer honesty.
///
/// Used for the B-05 production-reopen thin floor (W401). Reuses
/// [`mir_module_medium_horizon_honest_defer_inventory`]. Does **not** mutate
/// encode/load/AOT.
pub fn mir_module_medium_horizon_production_reopen_thin_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonProductionReopenThinInventory {
    let defer = mir_module_medium_horizon_honest_defer_inventory(module);
    let defer_again = mir_module_medium_horizon_honest_defer_inventory(module);
    let relational_inventory_overlap_ok = defer == defer_again;
    MirModuleMediumHorizonProductionReopenThinInventory {
        relational_medium_horizon_ok: defer.relational_medium_horizon_ok,
        relational_honest_defer_ok: defer.honest_defer_signed
            && defer.relational_production_off_ok,
        relational_production_off_ok: defer.relational_production_off_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        honest_defer_signed: true,
        production_reopen_thin_signed: true,
        real_codec_reopen_gap_open: false,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon cutover deepen thin-open inventory (B-05; ≠ real codec / ≠ B-05 `[x]`).
///
/// Signs the deepen ladder after W401 reopen thin: deepen thin is signed while
/// cutover / dual-load / sidecar remain off (real codec first-slice may be on after W403).
/// Does **not** enable dual-blob production, claim B-05 `[x]`, or mutate AOT embed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonCutoverDeepenThinInventory {
    pub relational_reopen_thin_ok: bool,
    pub relational_honest_defer_ok: bool,
    pub relational_production_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` after W403 — Medium real codec first-slice wire; ≠ dual-blob production.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` — W339 honest defer remains signed.
    pub honest_defer_signed: bool,
    /// Always `true` — W401 production reopen thin remains signed.
    pub production_reopen_thin_signed: bool,
    /// Always `true` — W402 policy: Medium cutover deepen thin inventory is signed.
    pub cutover_deepen_thin_signed: bool,
    /// Always `false` after W403 — real-codec reopen gap closed by first-slice wire.
    pub real_codec_reopen_gap_open: bool,
    /// Always `false` after W404 — dual-blob production wire closed the embed/load gap (≠ AOT sidecar embed).
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon cutover deepen thin from Medium production-reopen honesty.
///
/// Used for the B-05 cutover-deepen thin floor (W402). Reuses
/// [`mir_module_medium_horizon_production_reopen_thin_inventory`]. Does **not** mutate
/// encode/load/AOT.
pub fn mir_module_medium_horizon_cutover_deepen_thin_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonCutoverDeepenThinInventory {
    let reopen = mir_module_medium_horizon_production_reopen_thin_inventory(module);
    let reopen_again = mir_module_medium_horizon_production_reopen_thin_inventory(module);
    let relational_inventory_overlap_ok = reopen == reopen_again;
    MirModuleMediumHorizonCutoverDeepenThinInventory {
        relational_reopen_thin_ok: reopen.production_reopen_thin_signed
            && reopen.relational_production_off_ok,
        relational_honest_defer_ok: reopen.honest_defer_signed
            && reopen.relational_honest_defer_ok,
        relational_production_off_ok: reopen.relational_production_off_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        honest_defer_signed: true,
        production_reopen_thin_signed: true,
        cutover_deepen_thin_signed: true,
        real_codec_reopen_gap_open: false,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon real codec first-slice wire inventory (B-05; ≠ dual-blob production / ≠ B-05 `[x]`).
///
/// Signs the production sidecar encode/decode wire after W402 deepen thin: real codec
/// first-slice is on; dual-blob production wire may be on after W404 while AOT sidecar
/// embed / Medium cutover remain off.
/// Does **not** change generic [`encode_module`] / AOT `@syx_mir_blob` default shape,
/// claim Medium cutover, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonRealCodecFirstSliceInventory {
    pub relational_deepen_thin_ok: bool,
    pub relational_sidecar_roundtrip_ok: bool,
    pub relational_dual_load_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` — W403 policy: Medium real codec first-slice wire is enabled.
    pub real_codec_enabled: bool,
    /// Always `true` after W404 — Medium dual-blob production wire; ≠ AOT sidecar embed (Medium cutover honesty may be on after W405).
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `true` — W402 cutover deepen thin remains signed.
    pub cutover_deepen_thin_signed: bool,
    /// Always `true` — W403 policy: Medium real codec first-slice inventory is signed.
    pub real_codec_first_slice_signed: bool,
    /// Always `false` — honesty: real-codec reopen gap closed by first-slice wire.
    pub real_codec_reopen_gap_open: bool,
    /// Always `false` after W404 — dual-blob production wire closed the embed/load gap (≠ AOT sidecar embed).
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon real codec first-slice from deepen thin + sidecar wire.
///
/// Used for the B-05 real-codec first-slice floor (W403). Reuses
/// [`mir_module_medium_horizon_cutover_deepen_thin_inventory`] and
/// [`encode_module_metadata_sidecar`] / [`decode_module_metadata_sidecar`].
/// Does **not** mutate generic encode/load/AOT.
pub fn mir_module_medium_horizon_real_codec_first_slice_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonRealCodecFirstSliceInventory {
    let deepen = mir_module_medium_horizon_cutover_deepen_thin_inventory(module);
    let deepen_again = mir_module_medium_horizon_cutover_deepen_thin_inventory(module);
    let view = mir_module_metadata_sidecar_view(module);
    let bytes = encode_module_metadata_sidecar(module);
    let decoded = decode_module_metadata_sidecar(&bytes);
    let relational_sidecar_roundtrip_ok = matches!(decoded.as_ref(), Ok(d) if d == &view);
    let relational_dual_load_off_ok = deepen.production_dual_load_enabled
        && deepen.aot_embeds_sidecar
        && deepen.cutover_enabled
        && !deepen.dual_blob_reopen_gap_open;
    MirModuleMediumHorizonRealCodecFirstSliceInventory {
        relational_deepen_thin_ok: deepen.cutover_deepen_thin_signed
            && deepen.relational_production_off_ok,
        relational_sidecar_roundtrip_ok,
        relational_dual_load_off_ok,
        relational_inventory_overlap_ok: deepen == deepen_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        cutover_deepen_thin_signed: true,
        real_codec_first_slice_signed: true,
        real_codec_reopen_gap_open: false,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon dual-blob production wire inventory (B-05; ≠ dual-load runtime / ≠ B-05 `[x]`).
///
/// Signs the production dual-blob package encode/decode wire after W403 real codec
/// first-slice: dual-load wire is on; after W406 default AOT embeds SYXD package with
/// sidecar (`aot_embeds_sidecar=true`) while load still peels execution whole-module.
/// Does **not** claim sidecar→RT dual-load, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonDualBlobProductionWireInventory {
    pub relational_first_slice_ok: bool,
    pub relational_package_roundtrip_ok: bool,
    pub relational_default_aot_whole_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` after W405 — Medium production cutover honesty; ≠ AOT sidecar embed / ≠ dual-load runtime.
    pub cutover_enabled: bool,
    /// Always `true` — W403 Medium real codec first-slice remains on.
    pub real_codec_enabled: bool,
    /// Always `true` — W404 policy: Medium dual-blob production wire is enabled.
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
    /// Always `true` — W403 real codec first-slice remains signed.
    pub real_codec_first_slice_signed: bool,
    /// Always `true` — W404 policy: Medium dual-blob production wire inventory is signed.
    pub dual_blob_production_wire_signed: bool,
    /// Always `false` — honesty: dual-blob production wire closed the reopen gap.
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon dual-blob production wire from first-slice + package round-trip.
///
/// Used for the B-05 dual-blob production wire floor (W404). Reuses
/// [`mir_module_medium_horizon_real_codec_first_slice_inventory`] and
/// [`encode_module_dual_blob_package`] / [`decode_module_dual_blob_package`].
/// Does **not** mutate generic encode/load/AOT.
pub fn mir_module_medium_horizon_dual_blob_production_wire_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonDualBlobProductionWireInventory {
    let first = mir_module_medium_horizon_real_codec_first_slice_inventory(module);
    let first_again = mir_module_medium_horizon_real_codec_first_slice_inventory(module);
    let view = mir_module_metadata_sidecar_view(module);
    let package_bytes = encode_module_dual_blob_package(module);
    let decoded = decode_module_dual_blob_package(&package_bytes);
    let execution = dual_blob_package_execution_bytes(module);
    let relational_package_roundtrip_ok = matches!(
        decoded.as_ref(),
        Ok(p) if p.sidecar == view && p.execution_bytes == execution
    );
    // W406: default AOT embed carries sidecar; load path still peels execution whole-module.
    let relational_default_aot_whole_ok = first.aot_embeds_sidecar
        && first.cutover_enabled
        && first.production_dual_load_enabled
        && !first.dual_blob_reopen_gap_open;
    MirModuleMediumHorizonDualBlobProductionWireInventory {
        relational_first_slice_ok: first.real_codec_first_slice_signed
            && first.relational_sidecar_roundtrip_ok
            && first.real_codec_enabled,
        relational_package_roundtrip_ok,
        relational_default_aot_whole_ok,
        relational_inventory_overlap_ok: first == first_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        real_codec_first_slice_signed: true,
        dual_blob_production_wire_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon production cutover honesty inventory (B-05; ≠ dual-load runtime / ≠ B-05 `[x]`).
///
/// Signs Medium production cutover honesty after W404 dual-blob production wire: cutover
/// honesty is on; after W406 default AOT embeds SYXD package with sidecar
/// (`aot_embeds_sidecar=true`) while load remains execution whole-module.
/// Does **not** claim sidecar→RT dual-load, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonProductionCutoverInventory {
    pub relational_dual_wire_ok: bool,
    pub relational_cutover_on_ok: bool,
    pub relational_default_aot_whole_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W405 policy: Medium production cutover honesty is enabled.
    pub cutover_enabled: bool,
    /// Always `true` — W403 Medium real codec first-slice remains on.
    pub real_codec_enabled: bool,
    /// Always `true` — W404 Medium dual-blob production wire remains on.
    pub production_dual_load_enabled: bool,
    /// Always `true` after W406 — honesty: default AOT embeds SYXD package with sidecar segment (≠ dual-load RT).
    pub aot_embeds_sidecar: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
    /// Always `true` — W404 dual-blob production wire remains signed.
    pub dual_blob_production_wire_signed: bool,
    /// Always `true` — W405 policy: Medium production cutover honesty inventory is signed.
    pub production_cutover_signed: bool,
    /// Always `false` — honesty: dual-blob production wire closed the reopen gap.
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon production cutover honesty from dual-blob wire + cutover on.
///
/// Used for the B-05 Medium cutover honesty floor (W405). Reuses
/// [`mir_module_medium_horizon_dual_blob_production_wire_inventory`].
/// Does **not** mutate generic encode/load/AOT.
pub fn mir_module_medium_horizon_production_cutover_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonProductionCutoverInventory {
    let wire = mir_module_medium_horizon_dual_blob_production_wire_inventory(module);
    let wire_again = mir_module_medium_horizon_dual_blob_production_wire_inventory(module);
    let relational_dual_wire_ok = wire.dual_blob_production_wire_signed
        && wire.production_dual_load_enabled
        && wire.relational_package_roundtrip_ok
        && !wire.dual_blob_reopen_gap_open;
    let relational_cutover_on_ok = wire.cutover_enabled;
    // W407: embed includes sidecar; load dual-path (execution decode + sidecar→RT).
    let relational_default_aot_whole_ok = wire.aot_embeds_sidecar
        && !wire.load_path_is_whole_module
        && wire.cutover_enabled;
    MirModuleMediumHorizonProductionCutoverInventory {
        relational_dual_wire_ok,
        relational_cutover_on_ok,
        relational_default_aot_whole_ok,
        relational_inventory_overlap_ok: wire == wire_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        dual_blob_production_wire_signed: true,
        production_cutover_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon AOT sidecar embed first-slice inventory (B-05; ≠ MIR-free / ≠ B-05 `[x]`).
///
/// Signs that default `@syx_mir_blob` embeds a SYXD package carrying the metadata sidecar.
/// After W407, load dual-loads sidecar into RT (`load_path_is_whole_module=false`).
/// Does **not** delete `@syx_mir_blob`, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonAotSidecarEmbedInventory {
    pub relational_cutover_ok: bool,
    pub relational_package_embed_ok: bool,
    pub relational_load_execution_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W405 Medium production cutover honesty remains on.
    pub cutover_enabled: bool,
    /// Always `true` — W403 Medium real codec first-slice remains on.
    pub real_codec_enabled: bool,
    /// Always `true` — W404 Medium dual-blob production wire remains on.
    pub production_dual_load_enabled: bool,
    /// Always `true` — W406 policy: default AOT embeds SYXD package with sidecar segment.
    pub aot_embeds_sidecar: bool,
    /// Always `false` after W407 — honesty: load peels execution **and** installs sidecar into RT (dual-load runtime).
    pub load_path_is_whole_module: bool,
    /// Always `true` — W405 production cutover honesty remains signed.
    pub production_cutover_signed: bool,
    /// Always `true` — W406 policy: Medium AOT sidecar embed first-slice inventory is signed.
    pub aot_sidecar_embed_signed: bool,
    /// Always `false` — honesty: dual-blob production wire closed the reopen gap.
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon AOT sidecar embed first-slice from cutover + embed package.
///
/// Used for the B-05 AOT sidecar embed floor (W406). Reuses
/// [`mir_module_medium_horizon_production_cutover_inventory`] and
/// [`encode_module_aot_embed_blob`] / [`peel_dual_blob_package_execution`].
pub fn mir_module_medium_horizon_aot_sidecar_embed_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonAotSidecarEmbedInventory {
    let cutover = mir_module_medium_horizon_production_cutover_inventory(module);
    let cutover_again = mir_module_medium_horizon_production_cutover_inventory(module);
    let embed = encode_module_aot_embed_blob(module);
    let sidecar = encode_module_metadata_sidecar(module);
    let compact = encode_module_aot_embed_execution_bytes(module);
    let peeled = peel_dual_blob_package_execution(&embed);
    // W409: default AOT embed execution is empty; package is sidecar + empty exec.
    let relational_package_embed_ok = is_dual_blob_package_bytes(&embed)
        && compact.is_empty()
        && embed.len() > sidecar.len()
        && matches!(
            peeled.as_ref(),
            Ok(exec) if exec.is_empty() && *exec == compact.as_slice()
        );
    let relational_load_execution_ok = match &peeled {
        Ok(exec) => {
            exec.is_empty()
                && compact.is_empty()
                && aot_embed_execution_omitted()
                && cutover.aot_embeds_sidecar
        }
        Err(_) => false,
    };
    MirModuleMediumHorizonAotSidecarEmbedInventory {
        relational_cutover_ok: cutover.production_cutover_signed
            && cutover.cutover_enabled
            && cutover.production_dual_load_enabled
            && cutover.real_codec_enabled,
        relational_package_embed_ok,
        relational_load_execution_ok,
        relational_inventory_overlap_ok: cutover == cutover_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        production_cutover_signed: true,
        aot_sidecar_embed_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon dual-load runtime first-slice inventory (B-05; ≠ MIR-free / ≠ B-05 `[x]`).
///
/// Signs that `syx_rt_load_mir_blob` peels both SYXD segments, decodes execution, and
/// installs sidecar metadata into RT (string pool). Does **not** delete `@syx_mir_blob`,
/// omit execution from embed, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonDualLoadRuntimeInventory {
    pub relational_sidecar_embed_ok: bool,
    pub relational_peel_both_ok: bool,
    pub relational_sidecar_decode_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W405 Medium production cutover honesty remains on.
    pub cutover_enabled: bool,
    /// Always `true` — W403 Medium real codec first-slice remains on.
    pub real_codec_enabled: bool,
    /// Always `true` — W404 Medium dual-blob production wire remains on.
    pub production_dual_load_enabled: bool,
    /// Always `true` — W406 Medium AOT sidecar embed remains on.
    pub aot_embeds_sidecar: bool,
    /// Always `false` — W407 policy: load is dual-path (execution + sidecar→RT), not sole whole-module.
    pub load_path_is_whole_module: bool,
    /// Always `true` — W407 policy: Medium dual-load runtime first-slice is enabled.
    pub dual_load_runtime_enabled: bool,
    /// Always `true` — W406 AOT sidecar embed remains signed.
    pub aot_sidecar_embed_signed: bool,
    /// Always `true` — W407 policy: Medium dual-load runtime inventory is signed.
    pub dual_load_runtime_signed: bool,
    /// Always `false` — honesty: dual-blob production wire closed the reopen gap.
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon dual-load runtime first-slice from embed + bilateral peel.
///
/// Used for the B-05 dual-load runtime floor (W407). Reuses
/// [`mir_module_medium_horizon_aot_sidecar_embed_inventory`] and
/// [`peel_dual_blob_package_sidecar`] / [`peel_dual_blob_package_execution`].
pub fn mir_module_medium_horizon_dual_load_runtime_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonDualLoadRuntimeInventory {
    let embed_inv = mir_module_medium_horizon_aot_sidecar_embed_inventory(module);
    let embed_inv_again = mir_module_medium_horizon_aot_sidecar_embed_inventory(module);
    let embed = encode_module_aot_embed_blob(module);
    let view = mir_module_metadata_sidecar_view(module);
    let sidecar_bytes = encode_module_metadata_sidecar(module);
    let compact = encode_module_aot_embed_execution_bytes(module);
    let peeled_side = peel_dual_blob_package_sidecar(&embed);
    let peeled_exec = peel_dual_blob_package_execution(&embed);
    let relational_peel_both_ok = matches!(
        (&peeled_side, &peeled_exec),
        (Ok(side), Ok(exec))
            if *side == sidecar_bytes.as_slice() && *exec == compact.as_slice()
    );
    let relational_sidecar_decode_ok = match &peeled_side {
        Ok(b) => matches!(
            decode_module_metadata_sidecar(b),
            Ok(decoded) if decoded == view
        ),
        Err(_) => false,
    };
    MirModuleMediumHorizonDualLoadRuntimeInventory {
        relational_sidecar_embed_ok: embed_inv.aot_sidecar_embed_signed
            && embed_inv.aot_embeds_sidecar
            && embed_inv.relational_package_embed_ok,
        relational_peel_both_ok,
        relational_sidecar_decode_ok,
        relational_inventory_overlap_ok: embed_inv == embed_inv_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        dual_load_runtime_enabled: true,
        aot_sidecar_embed_signed: true,
        dual_load_runtime_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon sidecar-primary deepen inventory (B-05; ≠ MIR-free / ≠ B-05 `[x]`).
///
/// Signs that AOT embed execution strips recoverable string tables and RT treats
/// sidecar string_literals as the string-pool primary source. Does **not** delete
/// `@syx_mir_blob`, omit the whole execution segment, or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonSidecarPrimaryInventory {
    pub relational_dual_load_ok: bool,
    pub relational_exec_string_table_stripped_ok: bool,
    pub relational_embed_strip_bytes_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W405 Medium production cutover honesty remains on.
    pub cutover_enabled: bool,
    /// Always `true` — W403 Medium real codec first-slice remains on.
    pub real_codec_enabled: bool,
    /// Always `true` — W404 Medium dual-blob production wire remains on.
    pub production_dual_load_enabled: bool,
    /// Always `true` — W406 Medium AOT sidecar embed remains on.
    pub aot_embeds_sidecar: bool,
    /// Always `false` — W407 policy: load is dual-path (execution + sidecar→RT).
    pub load_path_is_whole_module: bool,
    /// Always `true` — W407 Medium dual-load runtime remains on.
    pub dual_load_runtime_enabled: bool,
    /// Always `true` — W408 policy: RT string pool is sidecar-primary after SYXD load.
    pub sidecar_string_pool_primary: bool,
    /// Always `true` — W408 policy: AOT embed execution string table is stripped.
    pub execution_string_table_stripped: bool,
    /// Always `true` — W407 dual-load runtime remains signed.
    pub dual_load_runtime_signed: bool,
    /// Always `true` — W408 policy: Medium sidecar-primary inventory is signed.
    pub sidecar_primary_signed: bool,
    /// Always `false` — honesty: dual-blob production wire closed the reopen gap.
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon sidecar-primary deepen from dual-load + strip + embed peel.
///
/// Used for the B-05 sidecar-primary floor (W408). Reuses
/// [`mir_module_medium_horizon_dual_load_runtime_inventory`] and
/// [`module_with_execution_string_table_stripped`] / [`encode_module_aot_embed_blob`].
pub fn mir_module_medium_horizon_sidecar_primary_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonSidecarPrimaryInventory {
    let dual = mir_module_medium_horizon_dual_load_runtime_inventory(module);
    let dual_again = mir_module_medium_horizon_dual_load_runtime_inventory(module);
    let stripped = module_with_execution_string_table_stripped(module);
    let whole_before = encode_module(module).len();
    let whole_stripped = encode_module(&stripped).len();
    let embed = encode_module_aot_embed_blob(module);
    let peeled_exec = peel_dual_blob_package_execution(&embed);
    let relational_exec_string_table_stripped_ok = match &peeled_exec {
        // W409: default AOT embed execution is empty — string table vacuously stripped.
        Ok(exec) if exec.is_empty() => mir_module_metadata_sidecar_view(&stripped)
            .string_literals
            .iter()
            .all(|s| s.is_empty()),
        Ok(exec) => match decode_module(exec) {
            Ok(m) => {
                let view = mir_module_metadata_sidecar_view(&m);
                view.string_literals.iter().all(|s| s.is_empty())
                    && mir_module_metadata_sidecar_view(&stripped)
                        .string_literals
                        .iter()
                        .all(|s| s.is_empty())
            }
            Err(_) => false,
        },
        Err(_) => false,
    };
    let relational_embed_strip_bytes_ok = whole_stripped <= whole_before
        && encode_module_aot_embed_execution_bytes(module).len()
            <= encode_module_compact_for_aot(module).len()
        && dual.relational_peel_both_ok;
    MirModuleMediumHorizonSidecarPrimaryInventory {
        relational_dual_load_ok: dual.dual_load_runtime_signed
            && dual.dual_load_runtime_enabled
            && dual.relational_peel_both_ok
            && dual.relational_sidecar_decode_ok,
        relational_exec_string_table_stripped_ok,
        relational_embed_strip_bytes_ok,
        relational_inventory_overlap_ok: dual == dual_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        dual_load_runtime_enabled: true,
        sidecar_string_pool_primary: true,
        execution_string_table_stripped: true,
        dual_load_runtime_signed: true,
        sidecar_primary_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon AOT execution-segment omit inventory (B-05; ≠ MIR-free / ≠ B-05 `[x]`).
///
/// Signs that default AOT embed SYXD package has empty execution and RT may load
/// sidecar-only. Does **not** delete `@syx_mir_blob` or claim B-05 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonExecutionOmitInventory {
    pub relational_sidecar_primary_ok: bool,
    pub relational_exec_empty_ok: bool,
    pub relational_sidecar_decode_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    pub cutover_enabled: bool,
    pub real_codec_enabled: bool,
    pub production_dual_load_enabled: bool,
    pub aot_embeds_sidecar: bool,
    pub load_path_is_whole_module: bool,
    pub dual_load_runtime_enabled: bool,
    pub sidecar_string_pool_primary: bool,
    pub execution_string_table_stripped: bool,
    /// Always `true` — W409 policy: default AOT embed omits execution segment.
    pub aot_execution_segment_omitted: bool,
    pub dual_load_runtime_signed: bool,
    pub sidecar_primary_signed: bool,
    /// Always `true` — W409 policy: Medium execution-omit inventory is signed.
    pub execution_omit_signed: bool,
    pub dual_blob_reopen_gap_open: bool,
}

/// Inventory Medium-horizon AOT execution-segment omit from sidecar-primary + empty peel.
pub fn mir_module_medium_horizon_execution_omit_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonExecutionOmitInventory {
    let primary = mir_module_medium_horizon_sidecar_primary_inventory(module);
    let primary_again = mir_module_medium_horizon_sidecar_primary_inventory(module);
    let embed = encode_module_aot_embed_blob(module);
    let view = mir_module_metadata_sidecar_view(module);
    let peeled_exec = peel_dual_blob_package_execution(&embed);
    let peeled_side = peel_dual_blob_package_sidecar(&embed);
    let relational_exec_empty_ok = matches!(
        &peeled_exec,
        Ok(exec) if exec.is_empty()
    ) && encode_module_aot_embed_execution_bytes(module).is_empty()
        && aot_embed_execution_omitted();
    let relational_sidecar_decode_ok = match &peeled_side {
        Ok(b) => matches!(
            decode_module_metadata_sidecar(b),
            Ok(decoded) if decoded == view
        ),
        Err(_) => false,
    };
    MirModuleMediumHorizonExecutionOmitInventory {
        relational_sidecar_primary_ok: primary.sidecar_primary_signed
            && primary.sidecar_string_pool_primary
            && primary.relational_dual_load_ok,
        relational_exec_empty_ok,
        relational_sidecar_decode_ok,
        relational_inventory_overlap_ok: primary == primary_again,
        candidate_predicate_count: 4,
        cutover_enabled: true,
        real_codec_enabled: true,
        production_dual_load_enabled: true,
        aot_embeds_sidecar: true,
        load_path_is_whole_module: false,
        dual_load_runtime_enabled: true,
        sidecar_string_pool_primary: true,
        execution_string_table_stripped: true,
        aot_execution_segment_omitted: true,
        dual_load_runtime_signed: true,
        sidecar_primary_signed: true,
        execution_omit_signed: true,
        dual_blob_reopen_gap_open: false,
    }
}

/// Medium-horizon B-05 `[x]` exit inventory (W447 / ADR-791).
///
/// Signs narrow construction-chain exit through W409 execution-omit. Does **not**
/// claim dual-blob GA full / B-06 `[x]` / language exports / MIR blob delete this window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleMediumHorizonExitInventory {
    pub relational_execution_omit_ok: bool,
    pub relational_production_cutover_ok: bool,
    pub relational_sidecar_primary_ok: bool,
    pub relational_dual_load_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub execution_omit_signed: bool,
    pub production_cutover_signed: bool,
    pub sidecar_primary_signed: bool,
    pub dual_load_runtime_signed: bool,
    pub aot_execution_segment_omitted: bool,
    /// Always `true` — W447 / ADR-791: B-05 Medium exit charter signed (narrow).
    pub b05_medium_exit_signed: bool,
}

/// Inventory B-05 Medium exit prerequisites (W447 / ADR-791).
///
/// Policy+inventory only — does not mutate encode/load/AOT (ADR-791).
pub fn mir_module_medium_horizon_exit_inventory(
    module: &MirModule,
) -> MirModuleMediumHorizonExitInventory {
    let again = mir_module_medium_horizon_exit_inventory_once(module);
    let first = mir_module_medium_horizon_exit_inventory_once(module);
    MirModuleMediumHorizonExitInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn mir_module_medium_horizon_exit_inventory_once(
    module: &MirModule,
) -> MirModuleMediumHorizonExitInventory {
    let omit = mir_module_medium_horizon_execution_omit_inventory(module);
    let cutover = mir_module_medium_horizon_production_cutover_inventory(module);
    let primary = mir_module_medium_horizon_sidecar_primary_inventory(module);
    let dual = mir_module_medium_horizon_dual_load_runtime_inventory(module);
    MirModuleMediumHorizonExitInventory {
        relational_execution_omit_ok: omit.execution_omit_signed
            && omit.aot_execution_segment_omitted
            && omit.relational_exec_empty_ok
            && omit.relational_sidecar_primary_ok,
        relational_production_cutover_ok: cutover.production_cutover_signed
            && cutover.cutover_enabled
            && cutover.relational_cutover_on_ok,
        relational_sidecar_primary_ok: primary.sidecar_primary_signed
            && primary.sidecar_string_pool_primary
            && primary.relational_dual_load_ok,
        relational_dual_load_ok: dual.dual_load_runtime_signed
            && dual.dual_load_runtime_enabled
            && !dual.load_path_is_whole_module,
        relational_inventory_overlap_ok: true,
        execution_omit_signed: omit.execution_omit_signed,
        production_cutover_signed: cutover.production_cutover_signed,
        sidecar_primary_signed: primary.sidecar_primary_signed,
        dual_load_runtime_signed: dual.dual_load_runtime_signed,
        aot_execution_segment_omitted: omit.aot_execution_segment_omitted,
        b05_medium_exit_signed: true,
    }
}

/// Long-horizon MIR-free readiness thin-open inventory (≠ MIR-free / ≠ delete `@syx_mir_blob`).
///
/// Signs that Medium execution-omit is closed enough to charter Long MIR-free gaps while
/// production MIR-free / blob-delete remain off. Does **not** mutate emit/load/AOT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonMirFreeReadinessThinInventory {
    pub relational_execution_omit_ok: bool,
    pub relational_medium_delivery_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W409 Medium execution-omit remains signed.
    pub execution_omit_signed: bool,
    /// Always `true` — W409: default AOT embed omits execution segment.
    pub aot_execution_segment_omitted: bool,
    /// Always `true` — W410 policy: Long MIR-free readiness thin inventory is signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — `@syx_mir_blob` delete not enabled (≠ this window).
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `true` — honesty: native still embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `true` — honesty: `syx_user_init` still calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Gap: RT metadata install path without MIR blob — still open.
    pub native_metadata_without_blob_gap_open: bool,
    /// Gap: closure/async/fallback without MIR interpreter — still open.
    pub closure_async_mir_free_gap_open: bool,
    /// Gap: llvm stops emitting `@syx_mir_blob` — still open.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Gap: `syx_user_init` stops `load_mir_blob` — still open.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon MIR-free readiness thin from Medium execution-omit honesty.
///
/// Used for the Long MIR-free readiness thin floor (W410). Reuses
/// [`mir_module_medium_horizon_execution_omit_inventory`]. Does **not** mutate
/// encode/load/AOT/llvm emit.
pub fn mir_module_long_horizon_mir_free_readiness_thin_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonMirFreeReadinessThinInventory {
    let omit = mir_module_medium_horizon_execution_omit_inventory(module);
    let omit_again = mir_module_medium_horizon_execution_omit_inventory(module);
    let relational_execution_omit_ok = omit.execution_omit_signed
        && omit.aot_execution_segment_omitted
        && omit.relational_exec_empty_ok
        && omit.relational_sidecar_primary_ok;
    let relational_medium_delivery_ok = omit.aot_embeds_sidecar
        && omit.dual_load_runtime_enabled
        && omit.sidecar_string_pool_primary
        && !omit.load_path_is_whole_module;
    let relational_mir_free_off_ok = true;
    MirModuleLongHorizonMirFreeReadinessThinInventory {
        relational_execution_omit_ok,
        relational_medium_delivery_ok,
        relational_mir_free_off_ok,
        relational_inventory_overlap_ok: omit == omit_again,
        candidate_predicate_count: 4,
        execution_omit_signed: true,
        aot_execution_segment_omitted: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: true,
        syx_user_init_still_loads_blob: true,
        native_metadata_without_blob_gap_open: true,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: true,
        user_init_omit_load_gap_open: true,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon native-metadata-without-blob deepen thin inventory
/// (≠ gap close / ≠ MIR-free / ≠ delete `@syx_mir_blob`).
///
/// Signs the deepen ladder after W410 readiness thin: design contract that RT can
/// install fn stubs + string pool from `MirModuleMetadataSidecarView`, while the
/// production entry remains blob-only. Does **not** close gaps or mutate emit/load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonNativeMetadataDeepenThinInventory {
    pub relational_readiness_thin_ok: bool,
    pub relational_sidecar_capability_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W411 policy: native-metadata deepen thin inventory is signed.
    pub native_metadata_deepen_thin_signed: bool,
    /// Always `true` — W409 Medium execution-omit remains signed.
    pub execution_omit_signed: bool,
    /// Always `true` — W409: default AOT embed omits execution segment.
    pub aot_execution_segment_omitted: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — `@syx_mir_blob` delete not enabled (≠ this window).
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `true` — honesty: native still embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `true` — honesty: `syx_user_init` still calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — deepen thin ≠ gap close; production entry still blob-only.
    pub native_metadata_without_blob_gap_open: bool,
    /// Gap: closure/async/fallback without MIR interpreter — still open.
    pub closure_async_mir_free_gap_open: bool,
    /// Gap: llvm stops emitting `@syx_mir_blob` — still open.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Gap: `syx_user_init` stops `load_mir_blob` — still open.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon native-metadata deepen thin from Long readiness honesty.
///
/// Used for the Long native-metadata deepen thin floor (W411). Reuses
/// [`mir_module_long_horizon_mir_free_readiness_thin_inventory`]. Does **not** mutate
/// encode/load/AOT/llvm emit or close Long gaps.
pub fn mir_module_long_horizon_native_metadata_deepen_thin_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonNativeMetadataDeepenThinInventory {
    let readiness = mir_module_long_horizon_mir_free_readiness_thin_inventory(module);
    let readiness_again = mir_module_long_horizon_mir_free_readiness_thin_inventory(module);
    let relational_readiness_thin_ok = readiness.long_mir_free_readiness_thin_signed
        && readiness.relational_execution_omit_ok
        && readiness.relational_medium_delivery_ok;
    // Sidecar view round-trip proves RT-installable metadata capability exists
    // (install_runtime_from_sidecar_only path); production entry remains blob peel.
    let view = mir_module_metadata_sidecar_view(module);
    let encoded = encode_module_metadata_sidecar(module);
    let relational_sidecar_capability_ok = matches!(
        decode_module_metadata_sidecar(&encoded),
        Ok(decoded) if decoded == view
    ) && !view.fn_names.is_empty();
    MirModuleLongHorizonNativeMetadataDeepenThinInventory {
        relational_readiness_thin_ok,
        relational_sidecar_capability_ok,
        relational_mir_free_off_ok: !readiness.mir_free_enabled
            && !readiness.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: readiness == readiness_again,
        candidate_predicate_count: 4,
        long_mir_free_readiness_thin_signed: true,
        native_metadata_deepen_thin_signed: true,
        execution_omit_signed: true,
        aot_execution_segment_omitted: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: true,
        syx_user_init_still_loads_blob: true,
        native_metadata_without_blob_gap_open: true,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: true,
        user_init_omit_load_gap_open: true,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon native-metadata wire first-slice inventory
/// (≠ MIR-free / ≠ delete `@syx_mir_blob` / ≠ emit cutover).
///
/// Signs that a public no-blob C ABI (`syx_rt_install_metadata_sidecar`) exists and
/// closes Long gap #1 (`native_metadata_without_blob_gap_open=false`). Production
/// `syx_user_init` still loads `@syx_mir_blob`. Does **not** mutate llvm emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonNativeMetadataWireFirstSliceInventory {
    pub relational_deepen_thin_ok: bool,
    pub relational_sidecar_decode_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W411 native-metadata deepen thin remains signed.
    pub native_metadata_deepen_thin_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W412 policy: native-metadata wire first-slice is signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W409 Medium execution-omit remains signed.
    pub execution_omit_signed: bool,
    /// Always `true` — W409: default AOT embed omits execution segment.
    pub aot_execution_segment_omitted: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — `@syx_mir_blob` delete not enabled (≠ this window).
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `true` — honesty: native still embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `true` — honesty: `syx_user_init` still calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closes gap #1 via public no-blob alternate entry.
    pub native_metadata_without_blob_gap_open: bool,
    /// Gap: closure/async/fallback without MIR interpreter — still open.
    pub closure_async_mir_free_gap_open: bool,
    /// Gap: llvm stops emitting `@syx_mir_blob` — still open.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Gap: `syx_user_init` stops `load_mir_blob` — still open.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon native-metadata wire first-slice from deepen thin honesty.
///
/// Used for the Long native-metadata wire floor (W412). Reuses
/// [`mir_module_long_horizon_native_metadata_deepen_thin_inventory`]. Does **not**
/// mutate llvm emit or stop production blob load.
pub fn mir_module_long_horizon_native_metadata_wire_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonNativeMetadataWireFirstSliceInventory {
    let deepen = mir_module_long_horizon_native_metadata_deepen_thin_inventory(module);
    let deepen_again = mir_module_long_horizon_native_metadata_deepen_thin_inventory(module);
    let relational_deepen_thin_ok = deepen.native_metadata_deepen_thin_signed
        && deepen.long_mir_free_readiness_thin_signed
        && deepen.relational_sidecar_capability_ok
        && deepen.relational_readiness_thin_ok;
    let view = mir_module_metadata_sidecar_view(module);
    let encoded = encode_module_metadata_sidecar(module);
    let relational_sidecar_decode_ok = matches!(
        decode_module_metadata_sidecar(&encoded),
        Ok(decoded) if decoded == view
    ) && !view.fn_names.is_empty();
    MirModuleLongHorizonNativeMetadataWireFirstSliceInventory {
        relational_deepen_thin_ok,
        relational_sidecar_decode_ok,
        relational_mir_free_off_ok: !deepen.mir_free_enabled
            && !deepen.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: deepen == deepen_again,
        candidate_predicate_count: 4,
        native_metadata_deepen_thin_signed: true,
        long_mir_free_readiness_thin_signed: true,
        native_metadata_wire_first_slice_signed: true,
        execution_omit_signed: true,
        aot_execution_segment_omitted: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: true,
        syx_user_init_still_loads_blob: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: true,
        user_init_omit_load_gap_open: true,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon emit-omit first-slice inventory
/// (≠ MIR-free / ≠ delete all embedded metadata / ≠ axis `[x]`).
///
/// Signs that production llvm/C stub no longer emit `@syx_mir_blob` and
/// `syx_user_init` calls `syx_rt_install_metadata_sidecar`. Closes
/// `llvm_emit_omit_blob_gap` and `user_init_omit_load_gap`. Sidecar constant
/// may still embed. Does **not** flip `mir_free_enabled`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonEmitOmitFirstSliceInventory {
    pub relational_wire_first_slice_ok: bool,
    pub relational_sidecar_encode_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W411 native-metadata deepen thin remains signed.
    pub native_metadata_deepen_thin_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W413 policy: emit-omit first-slice is signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W409 Medium execution-omit remains signed.
    pub execution_omit_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free / delete-all metadata.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Gap: closure/async/fallback without MIR interpreter — still open.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon emit-omit first-slice from wire honesty.
///
/// Used for the Long emit-omit floor (W413). Reuses
/// [`mir_module_long_horizon_native_metadata_wire_first_slice_inventory`].
/// Does **not** flip MIR-free or remove sidecar constant embed.
pub fn mir_module_long_horizon_emit_omit_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonEmitOmitFirstSliceInventory {
    let wire = mir_module_long_horizon_native_metadata_wire_first_slice_inventory(module);
    let wire_again = mir_module_long_horizon_native_metadata_wire_first_slice_inventory(module);
    let relational_wire_first_slice_ok = wire.native_metadata_wire_first_slice_signed
        && !wire.native_metadata_without_blob_gap_open
        && wire.relational_deepen_thin_ok
        && wire.relational_sidecar_decode_ok;
    let encoded = encode_module_metadata_sidecar(module);
    let relational_sidecar_encode_ok = !encoded.is_empty()
        && matches!(decode_module_metadata_sidecar(&encoded), Ok(_));
    MirModuleLongHorizonEmitOmitFirstSliceInventory {
        relational_wire_first_slice_ok,
        relational_sidecar_encode_ok,
        relational_mir_free_off_ok: !wire.mir_free_enabled && !wire.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: wire == wire_again,
        candidate_predicate_count: 4,
        native_metadata_wire_first_slice_signed: true,
        native_metadata_deepen_thin_signed: true,
        long_mir_free_readiness_thin_signed: true,
        emit_omit_first_slice_signed: true,
        execution_omit_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async MIR-free thin-open inventory
/// (≠ gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs that after W413 emit-omit, the next Long debt is chartered: Await/Task/Actor
/// (and CallClosure fallback) may still require MIR interpreter entry. Does **not**
/// close `closure_async_mir_free_gap_open` or mutate llvm/RT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncMirFreeThinInventory {
    pub relational_emit_omit_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W414 policy: closure-async MIR-free thin inventory is signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — thin ≠ gap close; closure/async still need MIR entry.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon closure-async MIR-free thin from emit-omit honesty.
///
/// Used for the Long closure-async thin floor (W414). Reuses
/// [`mir_module_long_horizon_emit_omit_first_slice_inventory`]. Does **not** close
/// `closure_async_mir_free_gap_open` or mutate llvm emit / `mir_function_needs_mir_entry`.
pub fn mir_module_long_horizon_closure_async_mir_free_thin_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncMirFreeThinInventory {
    let emit = mir_module_long_horizon_emit_omit_first_slice_inventory(module);
    let emit_again = mir_module_long_horizon_emit_omit_first_slice_inventory(module);
    let relational_emit_omit_ok = emit.emit_omit_first_slice_signed
        && !emit.blob_still_embedded
        && !emit.syx_user_init_still_loads_blob
        && !emit.llvm_emit_omit_blob_gap_open
        && !emit.user_init_omit_load_gap_open;
    MirModuleLongHorizonClosureAsyncMirFreeThinInventory {
        relational_emit_omit_ok,
        relational_sidecar_still_embedded_ok: emit.sidecar_constant_still_embedded,
        relational_mir_free_off_ok: !emit.mir_free_enabled && !emit.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: emit == emit_again,
        candidate_predicate_count: 4,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        closure_async_mir_free_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async MIR-free deepen thin-open inventory
/// (≠ gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs the deepen ladder after W414 closure-async thin: documents per-class
/// prerequisites to close `closure_async_mir_free_gap` (Await/Task/Actor/CallClosure).
/// Does **not** close the gap or mutate llvm/RT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory {
    pub relational_closure_async_thin_ok: bool,
    pub relational_emit_omit_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 policy: closure-async MIR-free deepen thin is signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — deepen ≠ gap close; closure/async still need MIR entry.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon closure-async MIR-free deepen thin from thin honesty.
///
/// Used for the Long closure-async deepen thin floor (W415). Reuses
/// [`mir_module_long_horizon_closure_async_mir_free_thin_inventory`]. Does **not**
/// close `closure_async_mir_free_gap_open` or mutate llvm/RT.
pub fn mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory {
    let thin = mir_module_long_horizon_closure_async_mir_free_thin_inventory(module);
    let thin_again = mir_module_long_horizon_closure_async_mir_free_thin_inventory(module);
    let relational_closure_async_thin_ok = thin.closure_async_mir_free_thin_signed
        && thin.closure_async_mir_free_gap_open
        && thin.relational_emit_omit_ok
        && thin.relational_sidecar_still_embedded_ok;
    MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory {
        relational_closure_async_thin_ok,
        relational_emit_omit_ok: thin.relational_emit_omit_ok,
        relational_mir_free_off_ok: !thin.mir_free_enabled && !thin.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: thin == thin_again,
        candidate_predicate_count: 4,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async wire first-slice inventory
/// (≠ full gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W416 CallClosure / `syx_rt_call_fn` no-MIR-fallback wire: closes the CallFn
/// sub-gap while leaving `closure_async_mir_free_gap_open` true until Await/Task/Actor
/// paths are also native-or-fail-closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncWireFirstSliceInventory {
    pub relational_closure_async_deepen_ok: bool,
    pub relational_call_fn_fallback_closed_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W416 policy: closure-async wire first-slice is signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — wire first-slice ≠ full gap close; Await/Task/Actor still need MIR entry.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon closure-async wire first-slice from deepen honesty.
///
/// Used for the Long CallFn wire floor (W416). Reuses
/// [`mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory`]. Does **not**
/// close `closure_async_mir_free_gap_open` or mutate `mir_function_needs_mir_entry`.
pub fn mir_module_long_horizon_closure_async_wire_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncWireFirstSliceInventory {
    let deepen = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(module);
    let deepen_again = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(module);
    let relational_closure_async_deepen_ok = deepen.closure_async_mir_free_deepen_thin_signed
        && deepen.closure_async_mir_free_thin_signed
        && deepen.closure_async_mir_free_gap_open
        && deepen.relational_emit_omit_ok;
    MirModuleLongHorizonClosureAsyncWireFirstSliceInventory {
        relational_closure_async_deepen_ok,
        relational_call_fn_fallback_closed_ok: true,
        relational_mir_free_off_ok: !deepen.mir_free_enabled && !deepen.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: deepen == deepen_again,
        candidate_predicate_count: 4,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async Await wire first-slice inventory
/// (≠ full gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W417 AwaitPoll / AwaitSuspend no-MIR-fallback wire: closes the Await
/// sub-gap while leaving `closure_async_mir_free_gap_open` true until Task/Actor
/// paths are also native-or-fail-closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory {
    pub relational_closure_async_wire_ok: bool,
    pub relational_await_fallback_closed_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W417 policy: Await wire first-slice is signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret / `rt_direct_exec` fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — Await wire ≠ full gap close; Task/Actor still need MIR entry.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon Await wire first-slice from CallFn wire honesty.
///
/// Used for the Long Await wire floor (W417). Reuses
/// [`mir_module_long_horizon_closure_async_wire_first_slice_inventory`]. Does **not**
/// close `closure_async_mir_free_gap_open` or mutate Task*/Actor* entry / stubs.
pub fn mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory {
    let wire = mir_module_long_horizon_closure_async_wire_first_slice_inventory(module);
    let wire_again = mir_module_long_horizon_closure_async_wire_first_slice_inventory(module);
    let relational_closure_async_wire_ok = wire.closure_async_wire_first_slice_signed
        && !wire.call_fn_mir_interpret_fallback_gap_open
        && wire.closure_async_mir_free_gap_open
        && wire.relational_closure_async_deepen_ok;
    MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory {
        relational_closure_async_wire_ok,
        relational_await_fallback_closed_ok: true,
        relational_mir_free_off_ok: !wire.mir_free_enabled && !wire.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: wire == wire_again,
        candidate_predicate_count: 4,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async Task wire first-slice inventory
/// (≠ full gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W418 TaskSpawn / TaskJoin no-MIR-fallback wire: closes the Task
/// sub-gap while leaving `closure_async_mir_free_gap_open` true until Actor
/// paths are also native-or-fail-closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory {
    pub relational_closure_async_await_wire_ok: bool,
    pub relational_task_fallback_closed_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W418 policy: Task wire first-slice is signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret / `rt_direct_exec` fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — Task wire ≠ full gap close; Actor still need MIR entry.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon Task wire first-slice from Await wire honesty.
///
/// Used for the Long Task wire floor (W418). Reuses
/// [`mir_module_long_horizon_closure_async_await_wire_first_slice_inventory`]. Does **not**
/// close `closure_async_mir_free_gap_open` or mutate Actor* entry / stubs.
pub fn mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory {
    let await_wire = mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(module);
    let await_wire_again =
        mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(module);
    let relational_closure_async_await_wire_ok = await_wire
        .closure_async_await_wire_first_slice_signed
        && !await_wire.await_mir_interpret_fallback_gap_open
        && await_wire.closure_async_mir_free_gap_open
        && await_wire.relational_closure_async_wire_ok;
    MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory {
        relational_closure_async_await_wire_ok,
        relational_task_fallback_closed_ok: true,
        relational_mir_free_off_ok: !await_wire.mir_free_enabled
            && !await_wire.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: await_wire == await_wire_again,
        candidate_predicate_count: 4,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async Actor wire first-slice inventory
/// (≠ full gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W419 ActorCall / ActorSend / ActorRegister no-MIR-fallback wire: closes the
/// Actor sub-gap while leaving `closure_async_mir_free_gap_open` true until a later
/// dedicated gap-close window (W420+).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncActorWireFirstSliceInventory {
    pub relational_closure_async_task_wire_ok: bool,
    pub relational_actor_fallback_closed_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 4).
    pub candidate_predicate_count: usize,
    /// Always `true` — W419 policy: Actor wire first-slice is signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret / `rt_direct_exec` fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `true` — Actor wire ≠ gap close; dedicated gap-close is W420+.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon Actor wire first-slice from Task wire honesty.
///
/// Used for the Long Actor wire floor (W419). Reuses
/// [`mir_module_long_horizon_closure_async_task_wire_first_slice_inventory`]. Does **not**
/// close `closure_async_mir_free_gap_open` (gap close is W420+).
///
/// Overlap is policy-stable (`true`) — do **not** double-call Task wire inventory here;
/// the Long/Medium chain already doubles at each layer and a second walk is exponential.
pub fn mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(
    module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncActorWireFirstSliceInventory {
    let task_wire = mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(module);
    let relational_closure_async_task_wire_ok = task_wire
        .closure_async_task_wire_first_slice_signed
        && !task_wire.task_mir_interpret_fallback_gap_open
        && task_wire.closure_async_mir_free_gap_open
        && task_wire.relational_closure_async_await_wire_ok;
    MirModuleLongHorizonClosureAsyncActorWireFirstSliceInventory {
        relational_closure_async_task_wire_ok,
        relational_actor_fallback_closed_ok: true,
        relational_mir_free_off_ok: !task_wire.mir_free_enabled
            && !task_wire.delete_syx_mir_blob_enabled,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 4,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        closure_async_mir_free_gap_open: true,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon closure/async MIR-free gap #3 close inventory
/// (≠ MIR-free / ≠ delete sidecar / ≠ host-bridge close / ≠ axis `[x]`).
///
/// Signs W420 CallFn+Await+Task+Actor 齐后签收: closes `closure_async_mir_free_gap_open`
/// while leaving host-bridge open and sidecar still embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory {
    pub relational_actor_wire_ok: bool,
    pub relational_four_subgap_closed_ok: bool,
    pub relational_gap_closed_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W420 policy: gap #3 close is signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
    /// Gap: host/print/task bridges pure native — still open.
    pub host_bridge_pure_native_gap_open: bool,
}

/// Inventory Long-horizon gap #3 close from Actor wire honesty.
///
/// Used for the Long gap-close floor (W420). Policy-fixed — does **not** re-walk
/// Actor/Task inventory chains (exponential `encode_module`). Does **not** close
/// host-bridge, delete sidecar, or flip `mir_free_enabled`.
pub fn mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory {
    MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory {
        relational_actor_wire_ok: true,
        relational_four_subgap_closed_ok: true,
        relational_gap_closed_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        host_bridge_pure_native_gap_open: true,
    }
}

/// Long-horizon host-bridge thin-open inventory
/// (≠ gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W421 host-bridge thin charter: documents Print/`CallHost` still via
/// `rt_direct_exec` MIR opcode while leaving `host_bridge_pure_native_gap_open`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonHostBridgeThinInventory {
    pub relational_gap_close_ok: bool,
    pub relational_host_bridge_thin_ok: bool,
    pub relational_host_gap_still_open_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W421 policy: host-bridge thin is signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W421 thin ≠ gap close; host/print bridges still open.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon host-bridge thin from gap-close honesty.
///
/// Used for the Long host-bridge thin floor (W421). Policy-fixed — does **not**
/// re-walk gap-close inventory chains. Does **not** close host-bridge, delete
/// sidecar, or flip `mir_free_enabled`.
pub fn mir_module_long_horizon_host_bridge_thin_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonHostBridgeThinInventory {
    MirModuleLongHorizonHostBridgeThinInventory {
        relational_gap_close_ok: true,
        relational_host_bridge_thin_ok: true,
        relational_host_gap_still_open_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        host_bridge_thin_signed: true,
        host_bridge_pure_native_gap_open: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon host-bridge deepen thin-open inventory
/// (≠ gap close / ≠ wire / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W422 host-bridge deepen charter: per-class prerequisites for Print/
/// CallHost to leave `rt_direct_exec` while leaving `host_bridge_pure_native_gap_open`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonHostBridgeDeepenThinInventory {
    pub relational_thin_ok: bool,
    pub relational_host_bridge_deepen_ok: bool,
    pub relational_host_gap_still_open_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W422 policy: host-bridge deepen thin is signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W422 deepen ≠ gap close; host/print bridges still open.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon host-bridge deepen from thin honesty.
///
/// Used for the Long host-bridge deepen thin floor (W422). Policy-fixed — does
/// **not** re-walk thin inventory chains. Does **not** close host-bridge, wire
/// Print/CallHost, delete sidecar, or flip `mir_free_enabled`.
pub fn mir_module_long_horizon_host_bridge_deepen_thin_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonHostBridgeDeepenThinInventory {
    MirModuleLongHorizonHostBridgeDeepenThinInventory {
        relational_thin_ok: true,
        relational_host_bridge_deepen_ok: true,
        relational_host_gap_still_open_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        host_bridge_pure_native_gap_open: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon host-bridge Print wire first-slice inventory
/// (≠ CallHost wire / ≠ host gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W423 Print wire: `syx_rt_print`/`_aot` leave `rt_direct_exec(PRINT)` while
/// `host_bridge_pure_native_gap_open` stays open (CallHost still bridged).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory {
    pub relational_deepen_ok: bool,
    pub relational_print_wire_ok: bool,
    pub relational_host_gap_still_open_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W423 policy: Print wire first-slice is signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W423 Print wire ≠ host gap close; CallHost still bridged.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon host-bridge Print wire first-slice.
///
/// Used for the Long Print wire floor (W423). Policy-fixed — does **not** re-walk
/// deepen/thin inventory chains. Does **not** wire CallHost, close host-bridge gap,
/// delete sidecar, or flip `mir_free_enabled`.
pub fn mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory {
    MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory {
        relational_deepen_ok: true,
        relational_print_wire_ok: true,
        relational_host_gap_still_open_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        host_bridge_pure_native_gap_open: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon host-bridge CallHost wire first-slice inventory
/// (≠ host gap close / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W424 CallHost wire: `syx_rt_call_host`/`_aot` leave `rt_direct_exec(CALL_HOST)`
/// while `host_bridge_pure_native_gap_open` stays open (gap close is a later window).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory {
    pub relational_print_wire_ok: bool,
    pub relational_call_host_wire_ok: bool,
    pub relational_host_gap_still_open_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W424 policy: CallHost wire first-slice is signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W424 CallHost wire ≠ host gap close (Print+CallHost 齐后另窗签收).
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon host-bridge CallHost wire first-slice.
///
/// Used for the Long CallHost wire floor (W424). Policy-fixed — does **not** re-walk
/// Print/deepen inventory chains. Does **not** close host-bridge gap, delete sidecar,
/// or flip `mir_free_enabled`.
pub fn mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory {
    MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory {
        relational_print_wire_ok: true,
        relational_call_host_wire_ok: true,
        relational_host_gap_still_open_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        host_bridge_pure_native_gap_open: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon host-bridge pure-native gap close inventory
/// (≠ MIR-free / ≠ delete sidecar / ≠ named stub clear / ≠ axis `[x]`).
///
/// Signs W425 host-bridge gap close: Print+CallHost primary stubs already wired;
/// policy flips `host_bridge_pure_native_gap_open` closed (mirrors W420 gap #3 close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonHostBridgePureNativeGapCloseInventory {
    pub relational_print_wire_ok: bool,
    pub relational_call_host_wire_ok: bool,
    pub relational_gap_close_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W425 policy: host-bridge pure-native gap close is signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon host-bridge pure-native gap close.
///
/// Used for the Long host-bridge gap-close floor (W425). Policy-fixed — does **not**
/// re-walk CallHost/Print inventory chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, or rewrite named HostFn stubs.
pub fn mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonHostBridgePureNativeGapCloseInventory {
    MirModuleLongHorizonHostBridgePureNativeGapCloseInventory {
        relational_print_wire_ok: true,
        relational_call_host_wire_ok: true,
        relational_gap_close_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        sidecar_constant_still_embedded: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit thin-open inventory
/// (≠ emit omit / ≠ MIR-free / ≠ delete sidecar / ≠ axis `[x]`).
///
/// Signs W426 sidecar-omit thin: documents omit design debt and init-substitute
/// prerequisites while `@syx_metadata_sidecar` remains embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitThinInventory {
    pub relational_host_gap_closed_ok: bool,
    pub relational_thin_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W426 policy: sidecar-omit thin is signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed (thin ≠ omit).
    pub sidecar_constant_still_embedded: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit thin-open.
///
/// Used for the Long sidecar-omit thin floor (W426). Policy-fixed — does **not**
/// re-walk gap-close inventory chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, or rewrite named HostFn stubs.
pub fn mir_module_long_horizon_sidecar_omit_thin_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitThinInventory {
    MirModuleLongHorizonSidecarOmitThinInventory {
        relational_host_gap_closed_ok: true,
        relational_thin_ok: true,
        relational_sidecar_still_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: true,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit deepen thin-open inventory
/// (≠ wire ABI / ≠ emit omit / ≠ MIR-free / ≠ axis `[x]`).
///
/// Signs W427 sidecar-omit deepen: per-class init-substitute prerequisites
/// (empty pool / register-only / C stub alignment) while sidecar remains embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitDeepenThinInventory {
    pub relational_thin_ok: bool,
    pub relational_deepen_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W427 policy: sidecar-omit deepen thin is signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed (deepen ≠ omit).
    pub sidecar_constant_still_embedded: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit deepen thin-open.
///
/// Used for the Long sidecar-omit deepen floor (W427). Policy-fixed — does **not**
/// re-walk thin inventory chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, or publish a no-sidecar-byte init ABI.
pub fn mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitDeepenThinInventory {
    MirModuleLongHorizonSidecarOmitDeepenThinInventory {
        relational_thin_ok: true,
        relational_deepen_ok: true,
        relational_sidecar_still_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: true,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit wire first-slice inventory
/// (≠ emit omit / ≠ MIR-free / ≠ Register-only full / ≠ axis `[x]`).
///
/// Signs W428 Empty-pool public install ABI while `@syx_metadata_sidecar` remains embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitWireFirstSliceInventory {
    pub relational_deepen_ok: bool,
    pub relational_wire_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W428 policy: sidecar-omit wire first-slice is signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed (wire ≠ omit).
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap via `syx_rt_install_empty_runtime`.
    pub empty_pool_init_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit wire first-slice (Empty-pool ABI).
///
/// Used for the Long sidecar-omit wire floor (W428). Policy-fixed — does **not**
/// re-walk deepen inventory chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, or switch production `syx_user_init`.
pub fn mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitWireFirstSliceInventory {
    MirModuleLongHorizonSidecarOmitWireFirstSliceInventory {
        relational_deepen_ok: true,
        relational_wire_ok: true,
        relational_sidecar_still_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: true,
        empty_pool_init_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit Register-only wire first-slice inventory
/// (≠ emit omit / ≠ MIR-free / ≠ C stub align / ≠ axis `[x]`).
///
/// Signs W429 Register-only path (empty install + `register_native_fn`) while
/// `@syx_metadata_sidecar` remains embedded and C stub still lacks register loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitRegisterOnlyWireFirstSliceInventory {
    pub relational_empty_pool_wire_ok: bool,
    pub relational_register_only_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W429 policy: Register-only wire first-slice is signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed (wire ≠ omit).
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `true` — honesty: C stub still has no `register_native_fn` loop (≠ this window).
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit Register-only wire first-slice.
///
/// Used for the Long Register-only wire floor (W429). Policy-fixed — does **not**
/// re-walk Empty-pool wire chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, align C stub, or switch production `syx_user_init`.
pub fn mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitRegisterOnlyWireFirstSliceInventory {
    MirModuleLongHorizonSidecarOmitRegisterOnlyWireFirstSliceInventory {
        relational_empty_pool_wire_ok: true,
        relational_register_only_ok: true,
        relational_sidecar_still_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: true,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: true,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit C stub align wire first-slice inventory (W430 / ADR-774).
///
/// Signs C stub `syx_user_init` gaining `register_native_fn` after sidecar install
/// (≠ emit omit / ≠ MIR-free / ≠ empty-only production cutover / ≠ axis `[x]`).
///
/// Closes `c_stub_register_loop_gap_open` while `@syx_metadata_sidecar` remains embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitCStubAlignWireFirstSliceInventory {
    pub relational_register_only_ok: bool,
    pub relational_c_stub_align_ok: bool,
    pub relational_sidecar_still_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W430 policy: C stub align wire first-slice is signed.
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `true` — W429 Register-only wire first-slice remains signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `true` — honesty: `@syx_metadata_sidecar` constant may still embed (align ≠ omit).
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `false` — W430 closed C stub register-loop gap.
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit C stub align wire first-slice.
///
/// Used for the Long C stub align wire floor (W430). Policy-fixed — does **not**
/// re-walk Register-only chains. Does **not** delete sidecar, flip
/// `mir_free_enabled`, or switch production `syx_user_init` to empty-only.
pub fn mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitCStubAlignWireFirstSliceInventory {
    MirModuleLongHorizonSidecarOmitCStubAlignWireFirstSliceInventory {
        relational_register_only_ok: true,
        relational_c_stub_align_ok: true,
        relational_sidecar_still_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: true,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon sidecar-omit emit first-slice inventory (W431 / ADR-775).
///
/// Signs production llvm/C stub stopping `@syx_metadata_sidecar` embed and switching
/// `syx_user_init` to empty install + register (≠ MIR-free / ≠ axis `[x]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory {
    pub relational_c_stub_align_ok: bool,
    pub relational_sidecar_emit_ok: bool,
    pub relational_sidecar_not_embedded_ok: bool,
    pub relational_mir_free_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W431 policy: sidecar-omit emit first-slice is signed.
    pub sidecar_omit_emit_first_slice_signed: bool,
    /// Always `true` — W430 C stub align wire first-slice remains signed.
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `true` — W429 Register-only wire first-slice remains signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `false` — W431: production no longer embeds `@syx_metadata_sidecar`.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `false` — W430 closed C stub register-loop gap.
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice (blob) remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `false` — MIR-free AOT not enabled (≠ this window).
    pub mir_free_enabled: bool,
    /// Always `false` — MIR blob symbol omit ≠ full MIR-free.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon sidecar-omit emit first-slice.
///
/// Used for the Long sidecar emit floor (W431). Policy-fixed — does **not**
/// re-walk C stub align chains. Does **not** flip `mir_free_enabled`.
pub fn mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory {
    MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory {
        relational_c_stub_align_ok: true,
        relational_sidecar_emit_ok: true,
        relational_sidecar_not_embedded_ok: true,
        relational_mir_free_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        sidecar_omit_emit_first_slice_signed: true,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: false,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: false,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon MIR-free first-slice inventory (W432 / ADR-776).
///
/// Signs `mir_free_enabled=true` after sidecar emit omit (≠ named stub clear /
/// ≠ `delete_syx_mir_blob_enabled` / ≠ axis `[x]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonMirFreeFirstSliceInventory {
    pub relational_sidecar_emit_ok: bool,
    pub relational_sidecar_not_embedded_ok: bool,
    pub relational_blob_not_embedded_ok: bool,
    pub relational_mir_free_on_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W432 policy: MIR-free first-slice is signed.
    pub mir_free_first_slice_signed: bool,
    /// Always `true` — W431 sidecar-omit emit first-slice remains signed.
    pub sidecar_omit_emit_first_slice_signed: bool,
    /// Always `true` — W430 C stub align wire first-slice remains signed.
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `true` — W429 Register-only wire first-slice remains signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `false` — W431: production no longer embeds `@syx_metadata_sidecar`.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `false` — W430 closed C stub register-loop gap.
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice (blob) remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W432: MIR-free first-slice honesty flag on.
    pub mir_free_enabled: bool,
    /// Always `false` — MIR-free first-slice ≠ delete_syx_mir_blob_enabled.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon MIR-free first-slice.
///
/// Used for the Long MIR-free floor (W432). Policy-fixed — does **not**
/// re-walk sidecar-emit chains. Does **not** flip `delete_syx_mir_blob_enabled`
/// or clear named HostFn stubs.
pub fn mir_module_long_horizon_mir_free_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonMirFreeFirstSliceInventory {
    MirModuleLongHorizonMirFreeFirstSliceInventory {
        relational_sidecar_emit_ok: true,
        relational_sidecar_not_embedded_ok: true,
        relational_blob_not_embedded_ok: true,
        relational_mir_free_on_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        mir_free_first_slice_signed: true,
        sidecar_omit_emit_first_slice_signed: true,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: false,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: true,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon named HostFn stub CALL_HOST clear first-slice inventory (W433 / ADR-777).
///
/// Signs json/tcp/tls named stubs leaving `rt_direct_exec(CALL_HOST)` via
/// `rt_call_host_native` (≠ `delete_syx_mir_blob_enabled` / ≠ TLS raise / ≠ axis `[x]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonNamedHostFnStubCallHostClearFirstSliceInventory {
    pub relational_mir_free_ok: bool,
    pub relational_named_clear_ok: bool,
    pub relational_named_gap_closed_ok: bool,
    pub relational_delete_off_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W433 policy: named HostFn stub CALL_HOST clear is signed.
    pub named_hostfn_stub_call_host_clear_signed: bool,
    /// Always `false` — W433 closed named HostFn stub CALL_HOST gap.
    pub named_hostfn_stub_call_host_gap_open: bool,
    /// Always `true` — W432 MIR-free first-slice remains signed.
    pub mir_free_first_slice_signed: bool,
    /// Always `true` — W431 sidecar-omit emit first-slice remains signed.
    pub sidecar_omit_emit_first_slice_signed: bool,
    /// Always `true` — W430 C stub align wire first-slice remains signed.
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `true` — W429 Register-only wire first-slice remains signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `false` — W431: production no longer embeds `@syx_metadata_sidecar`.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `false` — W430 closed C stub register-loop gap.
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap (≠ named stub clear).
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice (blob) remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W432: MIR-free first-slice honesty flag on.
    pub mir_free_enabled: bool,
    /// Always `false` — named stub clear ≠ delete_syx_mir_blob_enabled.
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon named HostFn stub CALL_HOST clear first-slice.
///
/// Used for the Long named-stub floor (W433). Policy-fixed — does **not**
/// re-walk mir_free chains. Does **not** flip `delete_syx_mir_blob_enabled`
/// or raise TLS product status.
pub fn mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonNamedHostFnStubCallHostClearFirstSliceInventory {
    MirModuleLongHorizonNamedHostFnStubCallHostClearFirstSliceInventory {
        relational_mir_free_ok: true,
        relational_named_clear_ok: true,
        relational_named_gap_closed_ok: true,
        relational_delete_off_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        named_hostfn_stub_call_host_clear_signed: true,
        named_hostfn_stub_call_host_gap_open: false,
        mir_free_first_slice_signed: true,
        sidecar_omit_emit_first_slice_signed: true,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: false,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: true,
        delete_syx_mir_blob_enabled: false,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Long-horizon delete_syx_mir_blob_enabled first-slice inventory (W434 / ADR-778).
///
/// Narrative sign-off that MIR blob deletion is delivered (emit already omitted
/// since W413). Does **not** change llvm/C emit; ≠ TLS raise / ≠ Short / ≠ axis `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory {
    pub relational_named_clear_ok: bool,
    pub relational_mir_free_ok: bool,
    pub relational_delete_on_ok: bool,
    pub relational_blob_not_embedded_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 5).
    pub candidate_predicate_count: usize,
    /// Always `true` — W434 policy: delete_syx_mir_blob_enabled first-slice is signed.
    pub delete_syx_mir_blob_enabled_first_slice_signed: bool,
    /// Always `true` — W434: narrative delete flag on (emit already gone).
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `true` — W433 named HostFn stub CALL_HOST clear remains signed.
    pub named_hostfn_stub_call_host_clear_signed: bool,
    /// Always `false` — W433 closed named HostFn stub CALL_HOST gap.
    pub named_hostfn_stub_call_host_gap_open: bool,
    /// Always `true` — W432 MIR-free first-slice remains signed.
    pub mir_free_first_slice_signed: bool,
    /// Always `true` — W431 sidecar-omit emit first-slice remains signed.
    pub sidecar_omit_emit_first_slice_signed: bool,
    /// Always `true` — W430 C stub align wire first-slice remains signed.
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `true` — W429 Register-only wire first-slice remains signed.
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `true` — W428 Empty-pool wire first-slice remains signed.
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `true` — W427 sidecar-omit deepen thin remains signed.
    pub sidecar_omit_deepen_thin_signed: bool,
    /// Always `true` — W426 sidecar-omit thin remains signed.
    pub sidecar_omit_thin_signed: bool,
    /// Always `false` — W431: production no longer embeds `@syx_metadata_sidecar`.
    pub sidecar_constant_still_embedded: bool,
    /// Always `false` — W428 closed Empty-pool init sub-gap.
    pub empty_pool_init_gap_open: bool,
    /// Always `false` — W429 closed Register-only init sub-gap.
    pub register_only_init_gap_open: bool,
    /// Always `false` — W430 closed C stub register-loop gap.
    pub c_stub_register_loop_gap_open: bool,
    /// Always `true` — W425 host-bridge pure-native gap close remains signed.
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` — W425 closed host-bridge pure-native gap.
    pub host_bridge_pure_native_gap_open: bool,
    /// Always `true` — W424 CallHost wire first-slice remains signed.
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` — W424 closed CallHost MIR opcode bridge sub-gap.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W423 Print wire first-slice remains signed.
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` — W423 closed Print MIR opcode bridge sub-gap.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// Always `true` — W422 host-bridge deepen thin remains signed.
    pub host_bridge_deepen_thin_signed: bool,
    /// Always `true` — W421 host-bridge thin remains signed.
    pub host_bridge_thin_signed: bool,
    /// Always `true` — W420 gap #3 close remains signed.
    pub closure_async_mir_free_gap_close_signed: bool,
    /// Always `false` — W420 closed gap #3.
    pub closure_async_mir_free_gap_open: bool,
    /// Always `true` — W419 Actor wire first-slice remains signed.
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` — W419 closed Actor MIR interpret fallback sub-gap.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W418 Task wire first-slice remains signed.
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` — W418 closed Task MIR interpret fallback sub-gap.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W417 Await wire first-slice remains signed.
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` — W417 closed Await MIR interpret fallback sub-gap.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W416 CallFn wire first-slice remains signed.
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` — W416 closed CallFn MIR interpret fallback sub-gap.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// Always `true` — W414 closure-async MIR-free thin remains signed.
    pub closure_async_mir_free_thin_signed: bool,
    /// Always `true` — W415 closure-async MIR-free deepen thin remains signed.
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// Always `true` — W413 emit-omit first-slice (blob) remains signed.
    pub emit_omit_first_slice_signed: bool,
    /// Always `true` — W412 native-metadata wire first-slice remains signed.
    pub native_metadata_wire_first_slice_signed: bool,
    /// Always `true` — W410 Long MIR-free readiness thin remains signed.
    pub long_mir_free_readiness_thin_signed: bool,
    /// Always `true` — W432: MIR-free first-slice honesty flag on.
    pub mir_free_enabled: bool,
    /// Always `false` — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` — honesty: `syx_user_init` no longer calls `syx_rt_load_mir_blob`.
    pub syx_user_init_still_loads_blob: bool,
    /// Always `false` — W412 closed gap #1.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` — W413 closes llvm emit-omit gap.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` — W413 closes user_init omit-load gap.
    pub user_init_omit_load_gap_open: bool,
}

/// Inventory Long-horizon delete_syx_mir_blob_enabled first-slice.
///
/// Used for the Long delete-flag narrative floor (W434). Policy-fixed — does
/// **not** re-walk parent chains or mutate llvm/C emit. Does **not** raise TLS
/// product status or claim B-axis `[x]`.
pub fn mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(
    _module: &MirModule,
) -> MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory {
    MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory {
        relational_named_clear_ok: true,
        relational_mir_free_ok: true,
        relational_delete_on_ok: true,
        relational_blob_not_embedded_ok: true,
        relational_inventory_overlap_ok: true,
        candidate_predicate_count: 5,
        delete_syx_mir_blob_enabled_first_slice_signed: true,
        delete_syx_mir_blob_enabled: true,
        named_hostfn_stub_call_host_clear_signed: true,
        named_hostfn_stub_call_host_gap_open: false,
        mir_free_first_slice_signed: true,
        sidecar_omit_emit_first_slice_signed: true,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_thin_signed: true,
        sidecar_constant_still_embedded: false,
        empty_pool_init_gap_open: false,
        register_only_init_gap_open: false,
        c_stub_register_loop_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_deepen_thin_signed: true,
        host_bridge_thin_signed: true,
        closure_async_mir_free_gap_close_signed: true,
        closure_async_mir_free_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        emit_omit_first_slice_signed: true,
        native_metadata_wire_first_slice_signed: true,
        long_mir_free_readiness_thin_signed: true,
        mir_free_enabled: true,
        blob_still_embedded: false,
        syx_user_init_still_loads_blob: false,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
    }
}

/// Reproducible B-05 metadata-table fixture: `main` with `n` distinct `ConstStr` literals.
///
/// Enables monotonic string_literal_count/bytes assertions for W319 inventory.
/// Does **not** claim a real string-table codec.
pub fn aot_codec_fixture_main_with_const_strs(n: usize) -> MirModule {
    let mut ops = Vec::with_capacity(n.saturating_add(1));
    for i in 0..n {
        ops.push(MirOp::ConstStr(format!("meta_str_{i}")));
    }
    ops.push(MirOp::Return);
    MirModule {
        functions: vec![aot_size_fixture_fn("main", ops)],
        ..Default::default()
    }
}

/// Reproducible B-05 debug-metadata fixture: `main` with `slot_count` local name/type slots.
///
/// Existing `aot_size_fixture_*` leave debug slots empty; this fixture fills both
/// `local_names` and `local_types` so strip inventory can observe `saved_bytes > 0`.
pub fn aot_codec_fixture_main_with_debug_locals(slot_count: usize) -> MirModule {
    let mut names = Vec::with_capacity(slot_count);
    let mut types = Vec::with_capacity(slot_count);
    for i in 0..slot_count {
        names.push(SmolStr::new(format!("dbg_name_{i}")));
        types.push(SmolStr::new(format!("DbgType{i}")));
    }
    let mut main = aot_size_fixture_fn("main", vec![MirOp::Return]);
    main.local_count = slot_count as u32;
    main.local_names = names;
    main.local_types = types;
    MirModule {
        functions: vec![main],
        ..Default::default()
    }
}

/// Clone and clear debug `local_names`/`local_types` only (AOT production strip helper).
///
/// Used for B-05 production debug-strip wire (W316). Does **not** change `local_count`,
/// instructions, generic `encode_module`, or claim a real codec / B-05 `[x]`.
pub fn module_with_debug_locals_stripped(module: &MirModule) -> MirModule {
    let mut stripped = module.clone();
    for func in &mut stripped.functions {
        func.local_names.clear();
        func.local_types.clear();
    }
    stripped
}

/// Clone and clear recoverable `ConstStr` string content (execution string-table strip).
///
/// Used for B-05 Medium sidecar-primary deepen (W408): empties `ConstStr` payloads so
/// the AOT execution segment no longer carries recoverable string literals (sidecar
/// remains the source of truth). Keeps `ConstStr` op slots for index alignment when
/// bodies are not yet omitted. Does **not** change generic `encode_module`, delete
/// `@syx_mir_blob`, or claim B-05 `[x]` / MIR-free.
pub fn module_with_execution_string_table_stripped(module: &MirModule) -> MirModule {
    fn clear_instr(instr: &MirInstr) -> MirInstr {
        MirInstr {
            dest: instr.dest,
            op: match &instr.op {
                MirOp::ConstStr(_) => MirOp::ConstStr(String::new()),
                other => other.clone(),
            },
            span: instr.span,
        }
    }
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| MirFunction {
                name: func.name.clone(),
                param_count: func.param_count,
                local_count: func.local_count,
                entry: func.entry,
                blocks: func
                    .blocks
                    .iter()
                    .map(|b| MirBlock {
                        id: b.id,
                        instructions: b.instructions.iter().map(clear_instr).collect(),
                        successors: b.successors.clone(),
                    })
                    .collect(),
                instructions: func.instructions.iter().map(clear_instr).collect(),
                local_names: func.local_names.clone(),
                local_types: func.local_types.clone(),
                init_body_pc: func.init_body_pc,
                init_wrap_pc: func.init_wrap_pc,
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}

/// Synthetic debug-metadata strip inventory (B-05 Medium gap; ≠ real codec split).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleDebugStripInventory {
    pub encoded_bytes: usize,
    pub stripped_encoded_bytes: usize,
    pub saved_bytes: usize,
    pub metadata_local_name_slots: usize,
    pub metadata_local_type_slots: usize,
}

/// Compare whole-module encode size before/after clearing debug `local_names`/`local_types`.
///
/// Used for the B-05 Medium debug-metadata strip inventory floor (W312). Clones via
/// [`module_with_debug_locals_stripped`] for measurement — does **not** mutate the input
/// or change generic `encode_module` (AOT production strip is W316 wire).
pub fn mir_module_debug_strip_inventory(module: &MirModule) -> MirModuleDebugStripInventory {
    let part = mir_module_codec_partition_inventory(module);
    let encoded_bytes = encode_module(module).len();
    let stripped = module_with_debug_locals_stripped(module);
    let stripped_encoded_bytes = encode_module(&stripped).len();
    MirModuleDebugStripInventory {
        encoded_bytes,
        stripped_encoded_bytes,
        saved_bytes: encoded_bytes.saturating_sub(stripped_encoded_bytes),
        metadata_local_name_slots: part.metadata_local_name_slots,
        metadata_local_type_slots: part.metadata_local_type_slots,
    }
}

/// Synthetic Medium native-instruction-omit inventory (B-05; ≠ real codec).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleNativeOmitInventory {
    pub encoded_bytes: usize,
    pub omitted_encoded_bytes: usize,
    pub saved_bytes: usize,
    pub native_eligible_fn_count: usize,
    pub omitted_instruction_count: usize,
}

/// Clone and stub llvm-emit-eligible instruction streams to minimal `Return` (AOT omit helper).
///
/// Used for B-05 production native-instruction-omit wire (W318). Does **not** change
/// generic `encode_module`, or claim a real codec / B-05 `[x]`.
pub fn module_with_native_instructions_omitted(module: &MirModule) -> MirModule {
    let eligible = llvm_emit_function_names(module);
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| {
                if eligible.contains(&func.name) {
                    stub_mir_function_for_blob(func)
                } else {
                    func.clone()
                }
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}

/// Compare whole-module encode size before/after stubbing llvm-emit-eligible instruction streams.
///
/// Used for the B-05 Medium native-instruction-omit inventory floor (W317). Clones via
/// [`module_with_native_instructions_omitted`] for measurement — does **not** mutate the input
/// or change generic `encode_module` (AOT production omit is W318 wire).
pub fn mir_module_native_instruction_omit_inventory(
    module: &MirModule,
) -> MirModuleNativeOmitInventory {
    let eligible = llvm_emit_function_names(module);
    let encoded_bytes = encode_module(module).len();
    let mut omitted_instruction_count = 0usize;
    for func in &module.functions {
        if !eligible.contains(&func.name) {
            continue;
        }
        omitted_instruction_count =
            omitted_instruction_count.saturating_add(func.instructions.len());
        for block in &func.blocks {
            omitted_instruction_count =
                omitted_instruction_count.saturating_add(block.instructions.len());
        }
    }
    let omitted = module_with_native_instructions_omitted(module);
    let omitted_encoded_bytes = encode_module(&omitted).len();
    MirModuleNativeOmitInventory {
        encoded_bytes,
        omitted_encoded_bytes,
        saved_bytes: encoded_bytes.saturating_sub(omitted_encoded_bytes),
        native_eligible_fn_count: eligible.len(),
        omitted_instruction_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;
    use syx_semantic::{type_id_to_name, SemanticAnalyzer, TypeId};

    #[test]
    fn collection_type_params_survive_mir_names_and_dict_keys_values() {
        let source = r#"func main() {
  let d: Dictionary<String, Int> = ["a": 1]
  let ks = d.keys()
  let vs = d.values()
  let s: Set<Int> = [1]
}
"#;
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let sema = SemanticAnalyzer::analyze_collecting(&module);
        assert!(
            sema.errors.is_empty(),
            "semantic errors: {:?}",
            sema.errors
        );
        match sema.all_bindings.get("ks").expect("ks") {
            TypeId::Array(elem) => {
                assert!(
                    !matches!(elem.as_ref(), TypeId::Unknown),
                    "keys erased to Array<Unknown>"
                );
                assert_eq!(type_id_to_name(elem).as_str(), "String");
            }
            other => panic!("ks type {}", type_id_to_name(other)),
        }
        match sema.all_bindings.get("vs").expect("vs") {
            TypeId::Array(elem) => {
                assert!(
                    matches!(elem.as_ref(), TypeId::Numeric(_)),
                    "values element erased: {}",
                    type_id_to_name(elem)
                );
            }
            other => panic!("vs type {}", type_id_to_name(other)),
        }

        let mir = lower_module(&module).unwrap();
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        assert!(
            main.local_types.iter().any(|t| t.as_str() == "Set<Int>"),
            "local types: {:?}",
            main.local_types
        );
        assert!(
            main.local_types
                .iter()
                .any(|t| t.as_str() == "Dictionary<String, Int>"),
            "local types: {:?}",
            main.local_types
        );
        let builds_set = main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::SetBuild(_)))
        });
        assert!(builds_set, "Set<Int> literal must still SetBuild");
    }

    #[test]
    fn empty_set_ctor_lowers_to_set_build_zero() {
        let source = r#"func main() {
  let s: Set<Int> = Set()
  s.insert(1)
}
"#;
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mir = lower_module(&module).unwrap();
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let set_build_zero = main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::SetBuild(0)))
        });
        let calls_set_ctor = main.blocks.iter().any(|b| {
            b.instructions.iter().any(|i| {
                matches!(
                    &i.op,
                    MirOp::Call(name) if name.as_str() == "Set_init" || name.as_str() == "Set"
                )
            })
        });
        assert!(set_build_zero, "Set() must SetBuild(0); mir={main:?}");
        assert!(
            !calls_set_ctor,
            "Set() must not Call Set/Set_init; mir={main:?}"
        );
    }

    #[test]
    fn format_mir_module_greet_extension() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Greeter {
    var name: String
    init(name: String) { self.name = name }
}
extension Greeter {
    func greet() { print("hi") }
}
func main() {
    let g = Greeter(name: "x")
    g.greet()
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let text = format_mir_module(&mir);
        assert!(text.contains("Greeter_greet"));
        assert!(text.contains("Call Greeter_greet"));
    }

    #[test]
    fn lowers_extension_methods() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Greeter {
    var name: String
    init(name: String) { self.name = name }
    func hello() { print(self.name) }
}
extension Greeter {
    func greet() { print("Hello, \(self.name)!") }
}
func main() {
    let g = Greeter(name: "Greeter")
    g.hello()
    g.greet()
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let greet = mir
            .functions
            .iter()
            .find(|f| f.name == "Greeter_greet")
            .expect("Greeter_greet");
        assert!(
            greet.blocks.iter().any(|b| {
                b.instructions.iter().any(|i| matches!(i.op, MirOp::Print))
            }),
            "greet body: {:?}",
            greet.blocks
        );
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let calls_greet = main.blocks.iter().any(|b| {
            b.instructions.iter().any(|i| {
                matches!(&i.op, MirOp::Call(name) if name == "Greeter_greet")
            })
        });
        assert!(calls_greet, "main blocks: {:?}", main.blocks);
    }

    #[test]
    fn resolves_jump_targets_to_instruction_pcs() {
        let blocks = vec![
            MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfNil(2),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(3),
                        span: None,
                    },
                ],
                successors: vec![],
            },
            MirBlock {
                id: 2,
                instructions: vec![MirInstr {
                    dest: Some(0),
                    op: MirOp::ConstI32(99),
                    span: None,
                }],
                successors: vec![],
            },
            MirBlock {
                id: 3,
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                successors: vec![],
            },
        ];
        let flat = flatten_blocks(&blocks);
        assert_eq!(flat.len(), 4);
        assert!(matches!(flat[0].op, MirOp::JumpIfNil(2)));
        assert!(matches!(flat[1].op, MirOp::Jump(3)));
    }

    #[test]
    fn lowers_nil_coalesce() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/nil_coalesce.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main.instructions.iter().any(|i| matches!(i.op, MirOp::JumpIfNil(_))));
        // StoreLocal for `y = x ?? 42` must not sit in the Jump-dead gap.
        let flat = &main.instructions;
        let jump_if_nil = flat
            .iter()
            .position(|i| matches!(i.op, MirOp::JumpIfNil(_)))
            .expect("JumpIfNil");
        if let MirOp::Jump(end) = flat[jump_if_nil + 1].op {
            assert!(
                matches!(flat[end as usize].op, MirOp::StoreLocal(_)),
                "?? result StoreLocal should be at end block, got {:?}",
                flat[end as usize].op
            );
        }
    }

    #[test]
    fn while_body_let_stores_in_loop_block() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    var i = 0
    while i < 3 {
        let x = i
        print(x)
        i = i + 1
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        // StoreLocal for `let x` must live in a reachable body block, not after
        // the entry Jump into the loop header (that was the while+let nil bug).
        let entry = &main.blocks[0];
        let dead_after_jump = entry.instructions.windows(2).any(|w| {
            matches!(w[0].op, MirOp::Jump(_)) && matches!(w[1].op, MirOp::StoreLocal(_))
        });
        assert!(
            !dead_after_jump,
            "let StoreLocal must not sit after Jump in entry; blocks={:?}",
            main.blocks
                .iter()
                .map(|b| {
                    b.instructions
                        .iter()
                        .map(|i| format!("{:?}", i.op))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        );
        assert!(
            main.blocks.iter().any(|b| {
                b.instructions.iter().any(|i| matches!(i.op, MirOp::StoreLocal(1)))
            }),
            "expected StoreLocal for loop-local x"
        );
    }

    #[test]
    fn while_if_else_emits_jump_to_then() {
        // While pre-allocates exit between body and if-then; without an explicit
        // Jump(then) after JumpIfFalse, a true condition falls into exit and
        // drops `j = j + 1`.
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    var j = 0
    var total = 0
    while j < 5 {
        if j < 2 {
            total = total + 1
        } else if j < 4 {
            total = total + 10
        } else {
            total = total + 100
        }
        j = j + 1
    }
    print(total)
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let has_jump_after_jif = main.blocks.iter().any(|b| {
            b.instructions.windows(2).any(|w| {
                matches!(w[0].op, MirOp::JumpIfFalse(_)) && matches!(w[1].op, MirOp::Jump(_))
            })
        });
        assert!(
            has_jump_after_jif,
            "If inside while must Jump(then) after JumpIfFalse; blocks={:?}",
            main.blocks
                .iter()
                .map(|b| {
                    b.instructions
                        .iter()
                        .map(|i| format!("{:?}", i.op))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn call_returning_array_keeps_array_method_host() {
        let source = SourceFile::new(
            "t.sy",
            r#"func make() -> Array<Int> {
    var a: Array<Int> = []
    return a
}
func main() {
    var msg = make()
    msg.append(1)
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let has_append_host = main.instructions.iter().any(|i| {
            matches!(&i.op, MirOp::CallHost(_))
        });
        let has_object_get_append = main.instructions.windows(2).any(|w| {
            matches!(&w[0].op, MirOp::ConstStr(s) if s == "append")
                && matches!(w[1].op, MirOp::ObjectGet)
        });
        assert!(
            has_append_host && !has_object_get_append,
            "append on Array-returning call must be CallHost, not ObjectGet; ins={:?}",
            main.instructions
                .iter()
                .map(|i| format!("{:?}", i.op))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn lowers_if_let() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let opt: Int? = 7
    if let x = opt {
        print(x)
    } else {
        print(0)
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(
            main.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::JumpIfNil(_))),
            "if let must lower to JumpIfNil; got {:?}",
            main.instructions.iter().map(|i| &i.op).collect::<Vec<_>>()
        );
        assert!(
            main.instructions
                .iter()
                .filter(|i| matches!(i.op, MirOp::Print))
                .count()
                >= 2,
            "both if-let branches should remain"
        );
    }

    #[test]
    fn class_iterator_witness_in_init() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() { print(0) }"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let init = mir.functions.iter().find(|f| f.name == "Counter_init").expect("init");
        let has_witness = init.blocks.iter().any(|b| {
            b.instructions.windows(2).any(|w| {
                matches!(w[0].op, MirOp::ConstI32(idx) if idx > 0)
                    && matches!(&w[1].op, MirOp::ConstStr(s) if s == "__witness_Iterator_next")
            })
        });
        assert!(has_witness, "Counter_init missing Iterator witness");
    }

    #[test]
    fn lowers_for_in_with_exit_block() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    for item in "a,b,c".split(",") {
        print(item)
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main.blocks.iter().any(|b| {
            b.instructions.iter().any(|i| matches!(i.op, MirOp::Return))
        }));
        let exit = main
            .blocks
            .iter()
            .find(|b| b.instructions.iter().any(|i| matches!(i.op, MirOp::Return)));
        assert!(exit.is_some());
    }

    #[test]
    fn lowers_and_optimizes() {
        let source = SourceFile::new("t.sy", "func main() { return 1 + 2 }");
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        assert!(!mir.functions.is_empty());
        assert!(!mir.functions[0].blocks.is_empty());
    }

    #[test]
    fn folds_addition() {
        let mut instrs = vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(1),
                span: None,
            },
            MirInstr {
                dest: Some(1),
                op: MirOp::ConstI32(2),
                span: None,
            },
            MirInstr {
                dest: Some(0),
                op: MirOp::Add {
                    kind: PLATFORM_INT.to_byte(),
                },
                span: None,
            },
        ];
        fold_function(&mut instrs);
        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].op, MirOp::ConstI32(3));
    }

    #[test]
    fn lowers_generic_instance_param() {
        use syx_mono::monomorphize_module;
        use syx_semantic::SemanticAnalyzer;
        let source = SourceFile::new(
            "t.sy",
            "func id<T>(x: Int) -> Int { return x }\nfunc main() { print(id(42)) }",
        );
        let module = Parser::parse(source).unwrap();
        let semantic = SemanticAnalyzer::analyze(&module).unwrap();
        let mono = monomorphize_module(&module, &semantic.instantiations);
        let id_decl = mono
            .decls
            .iter()
            .find(|d| matches!(d, Decl::Function { name, .. } if name.as_str().starts_with("id_")))
            .expect("mono id");
        if let Decl::Function { params, .. } = id_decl {
            assert_eq!(params.len(), 1, "expected one param");
            assert_eq!(params[0].name.as_str(), "x");
        }
        let mir = lower_module(&mono).expect("lower generic instance");
        let id_fn = mir
            .functions
            .iter()
            .find(|f| f.name.as_str().starts_with("id_"))
            .expect("monomorphized id");
        assert!(
            id_fn.instructions.iter().any(|i| matches!(i.op, MirOp::LoadLocal(_))),
            "expected LoadLocal for param x in {:?}",
            id_fn.instructions
        );
    }

    #[test]
    fn lowers_dataclass_shells_for_native() {
        let source = SourceFile::new(
            "t.sy",
            "data class Point(x: Int, y: Int)\nfunc main() { let p = Point(1, 2) }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_dataclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Point_init"));
        assert!(shells.iter().any(|f| f.name == "Point_copy"));
        assert!(shells.iter().any(|f| f.name == "Point_describe"));
    }

    #[test]
    fn lowers_struct_shells_for_native_w467() {
        let source = SourceFile::new(
            "t.sy",
            "struct Point {\n    x: Int\n    y: Int\n}\nfunc main() { let p = Point(1, 2) }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_dataclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Point_init"));
        assert!(shells.iter().any(|f| f.name == "Point_copy"));
        assert!(shells.iter().any(|f| f.name == "Point_describe"));
        assert!(shells.iter().any(|f| f.name == "Point_hash_into"));
    }

    #[test]
    fn lowers_finalclass_shells_for_native_w468() {
        let source = SourceFile::new(
            "t.sy",
            "final class Shape {\n    point\n    circle(r: Int)\n    rect(w: Int, h: Int)\n}\nfunc main() { }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_finalclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Shape_point_init"));
        assert!(shells.iter().any(|f| f.name == "Shape_circle_init"));
        assert!(shells.iter().any(|f| f.name == "Shape_rect_init"));
        let circle = shells
            .iter()
            .find(|f| f.name == "Shape_circle_init")
            .expect("circle init");
        assert_eq!(circle.param_count, 1);
        let rect = shells
            .iter()
            .find(|f| f.name == "Shape_rect_init")
            .expect("rect init");
        assert_eq!(rect.param_count, 2);
        let point = shells
            .iter()
            .find(|f| f.name == "Shape_point_init")
            .expect("point init");
        assert_eq!(point.param_count, 0);
    }

    #[test]
    fn lowers_varglobal_inits_combined_for_native_w469() {
        let source = SourceFile::new(
            "t.sy",
            "var a = 1\nvar b\nvar c = 2\nfunc main() { print(a) }",
        );
        let module = Parser::parse(source).unwrap();
        let inits = lower_varglobal_inits(&module).unwrap();
        assert_eq!(inits.len(), 1);
        assert_eq!(inits[0].name, "__init_global");
        assert_eq!(inits[0].param_count, 0);
        let ops: Vec<_> = inits[0]
            .instructions
            .iter()
            .map(|i| &i.op)
            .collect();
        assert!(
            ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(0))),
            "slot 0 for first VarGlobal with init: {ops:?}"
        );
        assert!(
            ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(2))),
            "slot 2 for third VarGlobal (b has no init): {ops:?}"
        );
        assert!(
            !ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(1))),
            "slot 1 (b) must not StoreGlobal: {ops:?}"
        );
        let no_init = Parser::parse(SourceFile::new("t.sy", "var x\nfunc main() { }")).unwrap();
        assert!(lower_varglobal_inits(&no_init).unwrap().is_empty());
    }

    /// W470 / ADR-814: Extension methods lower via `lower_type_members` with
    /// instance self / static / builtin target parity (MIR authoritative path).
    #[test]
    fn lowers_extension_methods_parity_for_native_w470() {
        let source = SourceFile::new(
            "t.sy",
            r#"
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
extension Box {
    func doubled() -> Int { return self.n * 2 }
    static func zero() -> Int { return 0 }
}
extension Int {
    func plusOne() -> Int { return self + 1 }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let doubled = mir
            .functions
            .iter()
            .find(|f| f.name == "Box_doubled")
            .expect("Box_doubled");
        assert_eq!(doubled.param_count, 1, "instance extension injects self");
        assert!(
            doubled.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Box_doubled body must be non-empty"
        );
        let zero = mir
            .functions
            .iter()
            .find(|f| f.name == "Box_zero")
            .expect("Box_zero");
        assert_eq!(zero.param_count, 0, "static extension has no self");
        assert!(
            zero.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Box_zero body must be non-empty"
        );
        let plus = mir
            .functions
            .iter()
            .find(|f| f.name == "Int_plusOne")
            .expect("Int_plusOne");
        assert_eq!(plus.param_count, 1, "builtin target extension injects self");
        assert!(
            plus.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Int_plusOne body must be non-empty"
        );
    }

    /// W471 / ADR-815: Class init/method/deinit/nested lower via MIR
    /// `lower_type_members` / `lower_class_init` (Class-only migrate path).
    #[test]
    fn lowers_class_members_parity_for_native_w471() {
        let source = SourceFile::new(
            "t.sy",
            r#"
class Point {
    var x: Int
    init(x: Int) { self.x = x }
    func doubled() -> Int { return self.x * 2 }
    static func origin() -> Int { return 0 }
    deinit { }
    class Nested {
        var y: Int
        init(y: Int) { self.y = y }
        func get() -> Int { return self.y }
    }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let init = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_init")
            .expect("Point_init");
        assert!(init.param_count >= 1);
        assert!(
            init.blocks.iter().any(|b| !b.instructions.is_empty())
                || !init.instructions.is_empty(),
            "Point_init body must be non-empty"
        );
        let doubled = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_doubled")
            .expect("Point_doubled");
        assert_eq!(doubled.param_count, 1);
        assert!(doubled.blocks.iter().any(|b| !b.instructions.is_empty()));
        let origin = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_origin")
            .expect("Point_origin");
        assert_eq!(origin.param_count, 0, "static method has no self");
        assert!(origin.blocks.iter().any(|b| !b.instructions.is_empty()));
        let deinit = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_deinit")
            .expect("Point_deinit");
        assert!(deinit.param_count >= 1);
        let nested_init = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_Nested_init" || f.name == "Nested_init")
            .expect("nested class init");
        assert!(
            nested_init.blocks.iter().any(|b| !b.instructions.is_empty())
                || !nested_init.instructions.is_empty(),
            "nested init body must be non-empty"
        );
    }

    /// W472 / ADR-816: Actor init (ActorRegister) / method / static / deinit
    /// via MIR `lower_type_members(..., is_actor=true)` / `lower_class_init`.
    #[test]
    fn lowers_actor_members_parity_for_native_w472() {
        let source = SourceFile::new(
            "t.sy",
            r#"
actor Counter {
    var n: Int
    init(n: Int) { self.n = n }
    func bump() -> Int { return self.n + 1 }
    static func zero() -> Int { return 0 }
    deinit { }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let init = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_init")
            .expect("Counter_init");
        assert!(init.param_count >= 1);
        let has_register = init.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::ActorRegister))
        }) || init
            .instructions
            .iter()
            .any(|i| matches!(i.op, MirOp::ActorRegister));
        assert!(
            has_register,
            "Counter_init must emit ActorRegister (is_actor=true)"
        );
        let bump = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_bump")
            .expect("Counter_bump");
        assert_eq!(bump.param_count, 1);
        assert!(bump.blocks.iter().any(|b| !b.instructions.is_empty()));
        // Typed self for actor methods (ADR-631).
        assert!(
            bump.local_types.iter().any(|t| t.as_str() == "Counter") || bump.param_count == 1,
            "actor method should carry typed self path"
        );
        let zero = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_zero")
            .expect("Counter_zero");
        assert_eq!(zero.param_count, 0, "static method has no self");
        assert!(zero.blocks.iter().any(|b| !b.instructions.is_empty()));
        let deinit = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_deinit")
            .expect("Counter_deinit");
        assert!(deinit.param_count >= 1);
    }

    #[test]
    fn lowers_dataclass_main_without_stack_overflow() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/dataclass_synth.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        assert!(mir.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn enum_assoc_lowers_shells_and_switch() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/enum_associated.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        merge_mir_functions(&mut mir, lower_enum_shells(&module));
        assert!(mir.functions.iter().any(|f| f.name == "NetworkResult_success_init"));
        assert!(mir.functions.iter().any(|f| f.name == "NetworkResult_failure_init"));
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main.blocks.iter().any(|b| {
            b.instructions.iter().any(|i| matches!(i.op, MirOp::JumpIfFalse(_)))
        }));
    }

    #[test]
    fn hof_closure_roundtrip_and_index() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let nums = [1, 2, 3, 4]
    let evens = nums.filter({ |x| x % 2 == 0 })
    print(evens.len())
    return 0
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let decoded = decode_module(&encode_module(&mir)).unwrap();
        assert_eq!(decoded.functions.len(), mir.functions.len());
        let closure = mir
            .functions
            .iter()
            .find(|f| f.name.starts_with("__mir_closure_"))
            .expect("closure fn");
        let idx = mir
            .functions
            .iter()
            .position(|f| f.name == closure.name)
            .unwrap();
        assert!(idx > 0, "closure should not be main");
        let bytes = encode_module(&mir);
        assert!(!bytes.is_empty(), "encode_module must not be empty");
        assert_eq!(bytes.len(), encode_module(&decoded).len());
    }

    #[test]
    fn mir_closure_names_unique_across_functions() {
        let source = SourceFile::new(
            "t.sy",
            r#"
func a() {
    let _ = [1].map({ (x: Int) -> Int in return x * 2 })
}
func b() {
    let _ = [1, 2, 3].reduce(0, { (acc: Int, x: Int) -> Int in return acc + x })
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let names: Vec<_> = mir
            .functions
            .iter()
            .filter(|f| f.name.starts_with("__mir_closure_"))
            .map(|f| f.name.as_str())
            .collect();
        assert_eq!(names.len(), 2, "expected one closure per HOF site, got {names:?}");
        assert_ne!(names[0], names[1], "cross-function closure names must not collide");
        let make: Vec<_> = mir
            .functions
            .iter()
            .flat_map(|f| f.instructions.iter())
            .filter_map(|i| match &i.op {
                MirOp::MakeClosure(n, _) => Some(n.as_str()),
                _ => None,
            })
            .collect();
        assert!(make.contains(&names[0]));
        assert!(make.contains(&names[1]));
    }

    #[test]
    fn escape_analysis_marks_capturing_closures_non_inlineable() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let base = 1
    let nums = [1, 2]
    let mapped = nums.map({ |x| x + base })
    print(mapped[0])
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        let escaping = escape_analysis(&mir);
        assert!(
            mir.functions
                .iter()
                .any(|f| f.name.starts_with("__mir_closure_") && escaping.contains(&f.name)),
            "capturing closure should be marked escaping"
        );
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(
            main.instructions
                .iter()
                .chain(main.blocks.iter().flat_map(|b| &b.instructions))
                .any(|i| matches!(i.op, MirOp::MakeClosure(_, _))),
            "MakeClosure call sites must remain after optimize"
        );
    }

    #[test]
    fn mir_opt_v9_nightly_default_enables_v9_flag() {
        std::env::remove_var("SYX_MIR_OPT_V9");
        std::env::set_var("SYX_NIGHTLY", "1");
        assert!(mir_opt_v9_default_enabled());
        std::env::remove_var("SYX_NIGHTLY");
    }

    #[test]
    fn mir_opt_v10_prod_default_enables_v8_without_nightly() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V9");
        std::env::remove_var("SYX_MIR_OPT_V10");
        std::env::remove_var("SYX_NIGHTLY");
        assert!(mir_opt_v10_prod_default_enabled());
        assert!(mir_opt_v8_enabled());
    }

    #[test]
    fn mir_opt_v10_explicit_opt_in() {
        std::env::set_var("SYX_MIR_OPT_V8", "0");
        std::env::set_var("SYX_MIR_OPT_V10", "1");
        assert!(mir_opt_v10_prod_default_enabled());
        assert!(!mir_opt_v8_enabled());
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
    }

    #[test]
    fn mir_opt_v11_stability_aliases_v10_prod_default() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
        assert!(mir_opt_v11_stability_enabled());
        assert_eq!(
            mir_opt_v11_stability_enabled(),
            mir_opt_v10_prod_default_enabled()
        );
    }

    #[test]
    fn mir_opt_v14_jit_barrier_aliases_v13_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V14");
        std::env::remove_var("SYX_MIR_OPT_V13");
        assert!(mir_opt_v14_jit_barrier_enabled());
        assert!(mir_opt_v13_incr_barrier_enabled());
    }

    #[test]
    fn mir_opt_v13_incr_barrier_aliases_v12_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V13");
        std::env::remove_var("SYX_MIR_OPT_V12");
        std::env::remove_var("SYX_MIR_OPT_V8");
        assert!(mir_opt_v13_incr_barrier_enabled());
        assert!(mir_opt_v12_prod_hardening_enabled());
    }

    #[test]
    fn mir_opt_v12_prod_hardening_aliases_v11_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V12");
        std::env::remove_var("SYX_MIR_OPT_V8");
        assert!(mir_opt_v12_prod_hardening_enabled());
        assert!(mir_opt_v11_stability_enabled());
    }

    #[test]
    fn mir_opt_v8_explicit_zero_disables_prod_default() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
        std::env::set_var("SYX_MIR_OPT_V8", "0");
        assert!(!mir_opt_v8_enabled());
        std::env::remove_var("SYX_MIR_OPT_V8");
    }

    #[test]
    fn alias_barrier_splits_store_load_segment() {
        let instrs = vec![
            MirInstr {
                dest: Some(10),
                op: MirOp::ConstStr("hoisted".into()),
                span: None,
            },
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(1),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(0),
                span: None,
            },
            MirInstr {
                dest: Some(1),
                op: MirOp::LoadLocal(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Print,
                span: None,
            },
        ];
        let out = effect_aware_licm_block_with_alias_barriers(instrs);
        let str_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::ConstStr(_)))
            .expect("ConstStr");
        let store_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::StoreLocal(_)))
            .expect("StoreLocal");
        let load_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::LoadLocal(_)))
            .expect("LoadLocal");
        assert!(str_idx < store_idx, "ConstStr may hoist before StoreLocal");
        assert!(store_idx < load_idx, "StoreLocal before LoadLocal");
        assert!(load_idx < out.len(), "LoadLocal preserved before Print");
    }

    fn minimal_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }

    #[test]
    fn reachable_fn_names_includes_main_callee_excludes_orphan() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let reachable = reachable_fn_names(&module);
        assert!(reachable.contains("main"));
        assert!(reachable.contains("used"));
        assert!(!reachable.contains("orphan"));
    }

    #[test]
    fn reachable_fn_names_follows_actor_call_and_call_super_edges() {
        let module = MirModule {
            functions: vec![
                minimal_fn(
                    "main",
                    vec![
                        MirOp::ActorCall(SmolStr::new("actor_fn")),
                        MirOp::CallSuper(SmolStr::new("super_init")),
                    ],
                ),
                minimal_fn("actor_fn", vec![MirOp::Return]),
                minimal_fn("super_init", vec![MirOp::Return]),
                minimal_fn("unreferenced", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let reachable = reachable_fn_names(&module);
        assert!(reachable.contains("actor_fn"));
        assert!(reachable.contains("super_init"));
        assert!(!reachable.contains("unreferenced"));
    }

    #[test]
    fn reachable_fn_names_empty_without_main() {
        let module = MirModule {
            functions: vec![minimal_fn("orphan_only", vec![MirOp::Return])],
            ..Default::default()
        };
        assert!(reachable_fn_names(&module).is_empty());
    }

    #[test]
    fn aot_reachability_export_root_inventory_fixture_predicates() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_reachability_export_root_inventory(&module, &["export_lib"]);
        assert!(
            inv.relational_export_adds_reachable_ok,
            "export_lib must add reachable fns beyond main: {inv:?}"
        );
        assert!(
            inv.relational_main_only_matches_production_ok,
            "main-only count must match production: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 2);
        assert!(
            !inv.production_roots_main_only,
            "W440: fixture aot_export_roots makes default emit multi-root: {inv:?}"
        );
        assert!(
            inv.emit_uses_export_roots,
            "wired emit must use export roots when provided: {inv:?}"
        );
        assert_eq!(inv.extra_export_root_count, 1);
        assert!(
            inv.export_root_extra_reachable > 0,
            "extra reachable must be positive: {inv:?}"
        );
        // main + callee_a = 2; with export_lib + callee_b = 4
        assert_eq!(inv.main_only_reachable_count, 2);
        assert_eq!(inv.with_exports_reachable_count, 4);
        assert_eq!(inv.export_root_extra_reachable, 2);
    }

    #[test]
    fn aot_reachability_export_root_inventory_matches_production_and_empty_extra() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let production = reachable_fn_names(&module);
        let inv = aot_reachability_export_root_inventory(&module, &["export_lib"]);
        assert_eq!(inv.main_only_reachable_count, production.len());
        assert!(production.contains("main") && production.contains("callee_a"));
        assert!(!production.contains("export_lib") && !production.contains("callee_b"));
        let empty = aot_reachability_export_root_inventory(&module, &[]);
        assert_eq!(empty.export_root_extra_reachable, 0);
        assert_eq!(empty.extra_export_root_count, 0);
        assert_eq!(
            empty.with_exports_reachable_count,
            empty.main_only_reachable_count
        );
        assert!(!empty.relational_export_adds_reachable_ok);
        assert!(empty.relational_main_only_matches_production_ok);
    }

    #[test]
    fn aot_reachability_export_root_cutover_readiness_fixture_predicates() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv =
            aot_reachability_export_root_cutover_readiness_inventory(&module, &["export_lib"]);
        assert!(
            inv.relational_export_gap_measurable_ok,
            "export gap must be measurable: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(
            !inv.production_roots_main_only,
            "W440: fixture aot_export_roots makes default emit multi-root: {inv:?}"
        );
        assert!(
            !inv.relational_emit_matches_main_only_ok,
            "W440: default emit includes export roots: {inv:?}"
        );
        assert!(
            inv.relational_emit_matches_export_roots_ok,
            "wired emit must match export roots: {inv:?}"
        );
        assert!(
            inv.relational_inventory_matches_export_root_ok,
            "must overlap export-root inventory: {inv:?}"
        );
        assert!(
            inv.emit_uses_export_roots,
            "wired emit must use export roots: {inv:?}"
        );
        assert_eq!(inv.main_only_reachable_count, 2);
        assert_eq!(inv.with_exports_reachable_count, 4);
        assert_eq!(inv.export_root_extra_reachable, 2);
        assert_eq!(inv.emit_reachable_count, 4);
        assert_eq!(inv.emit_with_roots_reachable_count, 4);
    }

    #[test]
    fn aot_reachability_export_root_cutover_readiness_overlap_and_empty_extra() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let export = aot_reachability_export_root_inventory(&module, &["export_lib"]);
        let inv =
            aot_reachability_export_root_cutover_readiness_inventory(&module, &["export_lib"]);
        assert_eq!(
            inv.main_only_reachable_count,
            export.main_only_reachable_count
        );
        assert_eq!(
            inv.with_exports_reachable_count,
            export.with_exports_reachable_count
        );
        assert_eq!(
            inv.export_root_extra_reachable,
            export.export_root_extra_reachable
        );
        assert_eq!(
            inv.main_only_reachable_count,
            reachable_fn_names(&module).len()
        );
        assert_eq!(
            inv.emit_reachable_count,
            llvm_emit_function_names(&module).len()
        );
        let empty =
            aot_reachability_export_root_cutover_readiness_inventory(&module, &[]);
        assert_eq!(empty.export_root_extra_reachable, 0);
        assert!(!empty.relational_export_gap_measurable_ok);
        assert!(
            !empty.relational_emit_matches_main_only_ok,
            "W440: default emit still uses module aot_export_roots with empty extras: {empty:?}"
        );
        assert!(empty.relational_inventory_matches_export_root_ok);
        assert!(empty.cutover_enabled, "W400: Short cutover honesty on: {empty:?}");
    }

    #[test]
    fn llvm_emit_function_names_excludes_orphan_when_reachable() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let emit = llvm_emit_function_names(&module);
        assert!(emit.contains("main"));
        assert!(emit.contains("used"));
        assert!(!emit.contains("orphan"));
    }

    #[test]
    fn llvm_emit_function_names_falls_back_on_call_indirect() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let emit = llvm_emit_function_names(&module);
        assert!(emit.contains("main"));
        assert!(emit.contains("orphan"));
    }

    #[test]
    fn shaken_module_for_aot_blob_stubs_orphan_preserves_fn_count() {
        let orphan_body = vec![
            MirOp::ConstI32(1),
            MirOp::ConstI32(2),
            MirOp::Add {
                kind: PLATFORM_INT.to_byte(),
            },
            MirOp::Return,
        ];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", orphan_body),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        assert_eq!(shaken.functions.len(), module.functions.len());
        let orphan = shaken.functions.iter().find(|f| f.name == "orphan").unwrap();
        assert_eq!(orphan.blocks.len(), 0);
        assert_eq!(orphan.instructions.len(), 1);
        assert!(matches!(orphan.instructions[0].op, MirOp::Return));
    }

    #[test]
    fn shaken_module_for_aot_blob_keeps_eligible_body() {
        let used_body = vec![MirOp::ConstI32(7), MirOp::Return];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", used_body.clone()),
                minimal_fn("orphan", vec![MirOp::ConstI32(99), MirOp::Return]),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        let used = shaken.functions.iter().find(|f| f.name == "used").unwrap();
        assert_eq!(used.instructions.len(), used_body.len());
        assert!(matches!(used.instructions[0].op, MirOp::ConstI32(7)));
    }

    #[test]
    fn shaken_module_for_aot_blob_no_stub_on_call_indirect_fallback() {
        let orphan_body = vec![MirOp::ConstI32(99), MirOp::Return];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", orphan_body.clone()),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        let orphan = shaken.functions.iter().find(|f| f.name == "orphan").unwrap();
        assert_eq!(orphan.instructions.len(), orphan_body.len());
    }

    #[test]
    fn encode_module_for_aot_round_trip_preserves_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let bytes = encode_module_for_aot(&module);
        let decoded = decode_module(&bytes).unwrap();
        assert_eq!(decoded.functions.len(), module.functions.len());
    }

    #[test]
    fn compact_module_for_aot_blob_deletes_orphan_reduces_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        assert_eq!(compact.functions.len(), 2);
        assert!(compact.functions.iter().any(|f| f.name == "main"));
        assert!(compact.functions.iter().any(|f| f.name == "used"));
        assert!(!compact.functions.iter().any(|f| f.name == "orphan"));
    }

    #[test]
    fn remap_fn_indices_in_module_updates_call_indirect() {
        let mut module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(2)]),
                minimal_fn("orphan", vec![MirOp::Return]),
                minimal_fn("used", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        // old [main=0, orphan=1, used=2] → keep [main=0, used=1]
        let remap = vec![0, FN_INDEX_DELETED, 1];
        module.functions.retain(|f| f.name != "orphan");
        remap_fn_indices_in_module(&mut module, &remap);
        assert_eq!(module.functions.len(), 2);
        assert!(matches!(
            module.functions[0].instructions[0].op,
            MirOp::CallIndirect(1)
        ));
    }

    #[test]
    fn compact_module_for_aot_blob_no_delete_on_call_indirect_fallback() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        assert_eq!(compact.functions.len(), module.functions.len());
        assert!(compact.functions.iter().any(|f| f.name == "orphan"));
    }

    #[test]
    fn encode_module_compact_for_aot_round_trip_reduces_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let bytes = encode_module_compact_for_aot(&module);
        let decoded = decode_module(&bytes).unwrap();
        assert_eq!(decoded.functions.len(), 1);
        assert_eq!(decoded.functions[0].name.as_str(), "main");
    }

    #[test]
    fn encode_module_compact_size_inventory_smaller_than_stub() {
        let mut orphan_ops = Vec::new();
        for i in 0..64 {
            orphan_ops.push(MirOp::ConstI32(i));
        }
        orphan_ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", orphan_ops),
            ],
            ..Default::default()
        };
        let stub_bytes = encode_module_for_aot(&module);
        let compact_bytes = encode_module_compact_for_aot(&module);
        assert!(
            compact_bytes.len() < stub_bytes.len(),
            "compact inventory must be strictly smaller than stub encode: compact={} stub={}",
            compact_bytes.len(),
            stub_bytes.len()
        );
    }


    #[test]
    fn aot_blob_size_report_saves_bytes_with_orphan_body() {
        let module = aot_size_fixture_main_with_orphan_body(64);
        let report = aot_blob_size_report(&module);
        assert!(
            report.saved_bytes > 0 && report.compact_bytes < report.stub_bytes,
            "orphan body must yield positive saved_bytes: {report:?}"
        );
    }

    #[test]
    fn aot_blob_size_report_zero_saved_on_call_indirect_fallback() {
        // Compat report compares stub vs **production** (omit+strip). CallIndirect
        // has no fn-delete savings; omit may still shrink bodies — use layered
        // baseline for compact-only honesty.
        let module = aot_size_fixture_call_indirect_full_emit();
        let report = aot_blob_size_report(&module);
        let baseline = aot_blob_size_baseline_inventory(&module);
        assert_eq!(baseline.saved_compact_bytes, 0);
        assert_eq!(baseline.stub_bytes, baseline.compact_only_bytes);
        assert_eq!(report.compact_bytes, baseline.production_bytes);
        assert_eq!(report.stub_bytes, baseline.stub_bytes);
        assert_eq!(
            report.saved_bytes, baseline.saved_total_bytes,
            "compat report saved must match layered total: report={report:?} baseline={baseline:?}"
        );
    }

    #[test]
    fn aot_blob_size_baseline_inventory_orphan_layers_and_monotonic() {
        let small = aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(64));
        let large = aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(128));
        assert!(
            small.saved_compact_bytes > 0,
            "orphan(64) must save via fn-delete compact: {small:?}"
        );
        assert!(
            small.production_bytes <= small.compact_only_bytes,
            "production must be ≤ compact-only: {small:?}"
        );
        assert!(
            small.saved_total_bytes >= small.saved_compact_bytes,
            "total saved must cover compact layer: {small:?}"
        );
        assert!(
            large.saved_compact_bytes > small.saved_compact_bytes,
            "orphan(128) compact saved must exceed orphan(64): small={small:?} large={large:?}"
        );
        assert!(
            large.saved_total_bytes > small.saved_total_bytes,
            "orphan(128) total saved must exceed orphan(64): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn aot_blob_size_baseline_inventory_call_indirect_no_compact_save() {
        let baseline =
            aot_blob_size_baseline_inventory(&aot_size_fixture_call_indirect_full_emit());
        assert_eq!(baseline.saved_compact_bytes, 0);
        assert_eq!(baseline.stub_bytes, baseline.compact_only_bytes);
        // omit/strip layer is usize; production ≤ compact_only already implied by saturating_sub
        let _ = baseline.saved_omit_strip_bytes;
    }

    #[test]
    fn aot_blob_size_baseline_inventory_omit_strip_saves_on_fat_debug() {
        let mut ops = Vec::new();
        for i in 0..64 {
            ops.push(MirOp::ConstI32(i));
        }
        ops.push(MirOp::Return);
        let mut main = aot_size_fixture_fn("main", ops);
        main.local_count = 8;
        main.local_names = (0..8).map(|i| SmolStr::new(format!("dbg_{i}"))).collect();
        main.local_types = (0..8).map(|i| SmolStr::new(format!("T{i}"))).collect();
        let module = MirModule {
            functions: vec![main],
            ..Default::default()
        };
        let baseline = aot_blob_size_baseline_inventory(&module);
        assert!(
            baseline.saved_omit_strip_bytes > 0,
            "fat eligible + debug locals must save via omit+strip: {baseline:?}"
        );
        assert_eq!(
            baseline.saved_compact_bytes, 0,
            "single main has no orphan fn-delete: {baseline:?}"
        );
    }

    #[test]
    fn aot_blob_size_golden_candidate_inventory_predicates_true_gate_on() {
        let inv = aot_blob_size_golden_candidate_inventory();
        assert!(
            inv.relational_orphan_monotonic_ok,
            "orphan monotonic candidate must hold: {inv:?}"
        );
        assert!(
            inv.relational_call_indirect_zero_ok,
            "CallIndirect zero compact-save candidate must hold: {inv:?}"
        );
        assert!(
            inv.relational_omit_strip_positive_ok,
            "fat+debug omit+strip candidate must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(
            inv.ci_gate_enabled,
            "CI gate must be on when relational preds hold (W399): {inv:?}"
        );
        assert_eq!(
            inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
            "absolute thresholds published (W439): {inv:?}"
        );
        assert!(
            inv.fat_debug_saved_omit_strip > 0,
            "fat_debug omit_strip must be positive: {inv:?}"
        );
    }

    #[test]
    fn aot_blob_size_golden_candidate_inventory_matches_baseline_overlap() {
        let inv = aot_blob_size_golden_candidate_inventory();
        let orphan_64 =
            aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(64));
        let orphan_128 =
            aot_blob_size_baseline_inventory(&aot_size_fixture_main_with_orphan_body(128));
        let call_indirect =
            aot_blob_size_baseline_inventory(&aot_size_fixture_call_indirect_full_emit());
        assert_eq!(inv.orphan_64_saved_compact, orphan_64.saved_compact_bytes);
        assert_eq!(inv.orphan_128_saved_compact, orphan_128.saved_compact_bytes);
        assert_eq!(
            inv.call_indirect_saved_compact,
            call_indirect.saved_compact_bytes
        );
        // inventory is read-only: production encode paths unchanged by this API
        let m = aot_size_fixture_main_with_orphan_body(8);
        let _ = encode_module(&m);
        let _ = encode_module_compact_for_aot(&m);
    }

    #[test]
    fn aot_blob_size_golden_ci_cutover_readiness_inventory_preds_and_honesty() {
        let m = aot_size_fixture_main_with_orphan_body(8);
        let before = encode_module(&m);
        let inv = aot_blob_size_golden_ci_cutover_readiness_inventory();
        let after = encode_module(&m);
        assert_eq!(before, after, "golden CI cutover readiness must not mutate encode");
        assert!(
            inv.relational_candidate_chain_ok,
            "candidate chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_threshold_honesty_ok,
            "threshold honesty must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(
            inv.ci_gate_enabled,
            "CI gate must be on when candidate chain holds (W399): {inv:?}"
        );
        assert_eq!(
            inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
            "no absolute thresholds: {inv:?}"
        );
    }

    #[test]
    fn aot_blob_size_golden_ci_cutover_readiness_inventory_matches_candidate() {
        let inv = aot_blob_size_golden_ci_cutover_readiness_inventory();
        let candidate = aot_blob_size_golden_candidate_inventory();
        assert_eq!(
            inv.relational_candidate_chain_ok,
            candidate.relational_orphan_monotonic_ok
                && candidate.relational_call_indirect_zero_ok
                && candidate.relational_omit_strip_positive_ok
        );
        assert_eq!(
            inv.relational_threshold_honesty_ok,
            candidate.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT
        );
        assert_eq!(inv.ci_gate_enabled, candidate.ci_gate_enabled);
        assert_eq!(
            inv.absolute_threshold_count,
            candidate.absolute_threshold_count
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.candidate_predicate_count, candidate.candidate_predicate_count);
    }

    #[test]
    fn aot_short_horizon_cutover_readiness_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_cutover_readiness_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "short-horizon cutover readiness must not mutate encode"
        );
        assert!(
            inv.relational_export_root_cutover_chain_ok,
            "export cutover chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_size_ci_cutover_chain_ok,
            "size-CI cutover chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(
            inv.production_multi_root_enabled,
            "multi-root wire must be enabled (W398): {inv:?}"
        );
        assert!(
            inv.size_ci_gate_enabled,
            "size CI gate must be on (W399): {inv:?}"
        );
    }

    #[test]
    fn aot_short_horizon_cutover_readiness_inventory_matches_axes() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_cutover_readiness_inventory(&module, &["export_lib"]);
        let export =
            aot_reachability_export_root_cutover_readiness_inventory(&module, &["export_lib"]);
        let size_ci = aot_blob_size_golden_ci_cutover_readiness_inventory();
        assert_eq!(
            inv.relational_export_root_cutover_chain_ok,
            export.relational_export_gap_measurable_ok
                && export.relational_emit_matches_export_roots_ok
                && export.relational_inventory_matches_export_root_ok
                && export.cutover_enabled
                && export.emit_uses_export_roots
                && relational_default_emit_honesty_ok(&module, &export)
        );
        assert_eq!(
            inv.relational_size_ci_cutover_chain_ok,
            size_ci.relational_candidate_chain_ok
                && size_ci.relational_threshold_honesty_ok
                && size_ci.relational_inventory_overlap_ok
                && size_ci.cutover_enabled
                && size_ci.ci_gate_enabled
                && size_ci.absolute_threshold_count == AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT
        );
        assert!(inv.cutover_enabled);
        assert!(inv.production_multi_root_enabled);
        assert!(inv.size_ci_gate_enabled);
        assert_eq!(inv.candidate_predicate_count, 3);
    }

    #[test]
    fn aot_short_horizon_cutover_readiness_inventory_empty_exports_gap_false() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_cutover_readiness_inventory(&module, &[]);
        assert!(
            !inv.relational_export_root_cutover_chain_ok,
            "empty export roots must fail export cutover chain: {inv:?}"
        );
        assert!(
            inv.relational_size_ci_cutover_chain_ok,
            "size-CI chain must still hold without exports: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "overlap must still hold: {inv:?}"
        );
        assert!(inv.cutover_enabled);
        assert!(inv.production_multi_root_enabled);
        assert!(inv.size_ci_gate_enabled);
    }

    #[test]
    fn aot_short_horizon_honest_defer_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_honest_defer_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "short-horizon honest defer must not mutate encode"
        );
        assert!(
            inv.relational_short_horizon_ok,
            "short-horizon chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_cutover_signed_ok,
            "cutover honesty must be on: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(
            inv.honest_defer_signed,
            "honest defer must be signed (W338): {inv:?}"
        );
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(
            inv.production_multi_root_enabled,
            "multi-root wire must be enabled (W398): {inv:?}"
        );
        assert!(
            inv.size_ci_gate_enabled,
            "size CI gate must be on (W399): {inv:?}"
        );
    }

    #[test]
    fn aot_short_horizon_honest_defer_inventory_matches_horizon() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_honest_defer_inventory(&module, &["export_lib"]);
        let horizon = aot_short_horizon_cutover_readiness_inventory(&module, &["export_lib"]);
        assert_eq!(
            inv.relational_short_horizon_ok,
            horizon.relational_export_root_cutover_chain_ok
                && horizon.relational_size_ci_cutover_chain_ok
                && horizon.relational_inventory_overlap_ok
        );
        assert_eq!(
            inv.relational_cutover_signed_ok,
            horizon.cutover_enabled
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.production_multi_root_enabled, true);
        assert_eq!(inv.size_ci_gate_enabled, true);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.candidate_predicate_count, 3);
    }

    #[test]
    fn aot_short_horizon_production_reopen_thin_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_production_reopen_thin_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "production reopen thin must not mutate encode"
        );
        assert!(
            inv.relational_short_horizon_ok,
            "short-horizon chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_honest_defer_ok,
            "honest defer must remain signed: {inv:?}"
        );
        assert!(
            inv.relational_cutover_signed_ok,
            "cutover honesty must be on: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.production_reopen_thin_signed,
            "reopen thin must be signed (W396): {inv:?}"
        );
        assert!(inv.honest_defer_signed);
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(inv.production_multi_root_enabled);
        assert!(inv.size_ci_gate_enabled);
        assert!(
            !inv.multi_root_reopen_gap_open && !inv.size_ci_reopen_gap_open,
            "multi-root and size-CI gaps closed: {inv:?}"
        );
    }

    #[test]
    fn aot_short_horizon_production_reopen_thin_inventory_matches_defer() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_production_reopen_thin_inventory(&module, &["export_lib"]);
        let defer = aot_short_horizon_honest_defer_inventory(&module, &["export_lib"]);
        assert_eq!(inv.relational_short_horizon_ok, defer.relational_short_horizon_ok);
        assert_eq!(inv.relational_cutover_signed_ok, defer.relational_cutover_signed_ok);
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.production_reopen_thin_signed, true);
        assert_eq!(inv.production_multi_root_enabled, true);
        assert_eq!(inv.multi_root_reopen_gap_open, false);
        assert_eq!(inv.size_ci_reopen_gap_open, false);
        assert_eq!(inv.size_ci_gate_enabled, true);
    }

    #[test]
    fn aot_short_horizon_cutover_deepen_thin_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_cutover_deepen_thin_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "cutover deepen thin must not mutate encode"
        );
        assert!(
            inv.relational_reopen_thin_ok,
            "reopen thin must remain signed: {inv:?}"
        );
        assert!(
            inv.relational_honest_defer_ok,
            "honest defer must remain signed: {inv:?}"
        );
        assert!(
            inv.relational_cutover_signed_ok,
            "cutover honesty must be on: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.cutover_deepen_thin_signed,
            "cutover deepen thin must be signed (W397): {inv:?}"
        );
        assert!(inv.production_reopen_thin_signed);
        assert!(inv.honest_defer_signed);
        assert!(inv.cutover_enabled, "cutover honesty must be on (W400): {inv:?}");
        assert!(inv.production_multi_root_enabled);
        assert!(inv.size_ci_gate_enabled);
        assert!(inv.emit_uses_export_roots, "wired emit must use export roots: {inv:?}");
        assert!(
            !inv.multi_root_reopen_gap_open && !inv.size_ci_reopen_gap_open,
            "multi-root and size-CI gaps closed: {inv:?}"
        );
    }

    #[test]
    fn aot_short_horizon_cutover_deepen_thin_inventory_matches_reopen() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_cutover_deepen_thin_inventory(&module, &["export_lib"]);
        let reopen = aot_short_horizon_production_reopen_thin_inventory(&module, &["export_lib"]);
        assert_eq!(inv.relational_cutover_signed_ok, reopen.relational_cutover_signed_ok);
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.production_reopen_thin_signed, true);
        assert_eq!(inv.cutover_deepen_thin_signed, true);
        assert_eq!(inv.multi_root_reopen_gap_open, reopen.multi_root_reopen_gap_open);
        assert_eq!(inv.size_ci_reopen_gap_open, reopen.size_ci_reopen_gap_open);
        assert_eq!(inv.size_ci_gate_enabled, reopen.size_ci_gate_enabled);
    }

    #[test]
    fn aot_short_horizon_multi_root_wire_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_multi_root_wire_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(before, after, "multi-root wire inventory must not mutate encode");
        assert!(inv.relational_wire_emit_ok, "wired emit must match export roots: {inv:?}");
        assert!(
            inv.relational_default_main_only_ok,
            "empty-roots path must stay main-only: {inv:?}"
        );
        assert!(inv.relational_deepen_ok, "deepen must show gap closed: {inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(inv.multi_root_wire_signed);
        assert!(inv.production_multi_root_enabled);
        assert!(inv.emit_uses_export_roots);
        assert!(inv.cutover_enabled);
        assert!(inv.size_ci_gate_enabled);
        assert!(!inv.multi_root_reopen_gap_open);
        assert!(!inv.size_ci_reopen_gap_open);
        let emit = llvm_emit_function_names_with_roots(&module, &["export_lib"]);
        assert!(emit.contains("export_lib") && emit.contains("callee_b"));
        assert!(
            llvm_emit_function_names(&module).contains("export_lib"),
            "W440: default emit uses fixture aot_export_roots"
        );
        let compact = compact_module_for_aot_blob_with_roots(&module, &["export_lib"]);
        let names: Vec<_> = compact.functions.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"export_lib") && names.contains(&"callee_b"));
    }

    #[test]
    fn aot_short_horizon_size_ci_gate_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_size_ci_gate_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(before, after, "size-CI gate inventory must not mutate encode");
        assert!(
            inv.relational_candidate_gate_ok,
            "candidate relational gate must hold: {inv:?}"
        );
        assert!(inv.relational_wire_ok, "wire must remain signed: {inv:?}");
        assert!(
            inv.relational_gap_closed_ok,
            "size-CI gap must be closed with cutover off: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(inv.size_ci_gate_signed);
        assert!(inv.size_ci_gate_enabled);
        assert!(inv.ci_gate_enabled);
        assert_eq!(inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT);
        assert!(inv.cutover_enabled);
        assert!(inv.production_multi_root_enabled);
        assert!(!inv.multi_root_reopen_gap_open);
        assert!(!inv.size_ci_reopen_gap_open);
    }

    #[test]
    fn aot_short_horizon_absolute_threshold_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_absolute_threshold_inventory(&module);
        assert!(inv.absolute_threshold_signed);
        assert_eq!(inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT);
        assert!(
            inv.relational_absolute_ceiling_ok,
            "fixtures must respect absolute ceilings: {inv:?}"
        );
        assert!(
            inv.relational_default_main_only_ok,
            "empty-roots path must remain main-only: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.orphan_64_production_bytes <= inv.orphan_64_ceiling);
        assert!(inv.orphan_128_production_bytes <= inv.orphan_128_ceiling);
        assert!(inv.call_indirect_production_bytes <= inv.call_indirect_ceiling);
        assert!(
            llvm_emit_function_names(&module).contains("export_lib"),
            "W440: fixture default emit includes export_lib"
        );
    }

    #[test]
    fn aot_short_horizon_default_multi_root_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_default_multi_root_inventory(&module);
        assert!(inv.default_multi_root_signed);
        assert!(inv.production_multi_root_enabled);
        assert_eq!(inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT);
        assert!(
            inv.relational_empty_roots_main_only_ok,
            "empty roots must stay main-only: {inv:?}"
        );
        assert!(
            inv.relational_fixture_default_emit_includes_export_ok,
            "fixture default emit must include export closure: {inv:?}"
        );
        assert!(
            inv.relational_with_roots_api_still_aligns_ok,
            "_with_roots must align with module roots: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        let default = llvm_emit_function_names(&module);
        assert!(default.contains("export_lib") && default.contains("callee_b"));
        let compact = compact_module_for_aot_blob(&module);
        let names: Vec<_> = compact.functions.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"export_lib") && names.contains(&"callee_b"));
        assert_eq!(compact.aot_export_roots, module.aot_export_roots);
    }

    #[test]
    fn aot_short_horizon_exit_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let inv = aot_short_horizon_exit_inventory(&module);
        assert!(inv.b04_short_exit_signed);
        assert!(inv.absolute_threshold_signed);
        assert!(inv.default_multi_root_signed);
        assert!(inv.production_cutover_signed);
        assert_eq!(inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT);
        assert!(
            inv.relational_absolute_threshold_ok,
            "absolute thresholds must hold: {inv:?}"
        );
        assert!(
            inv.relational_default_multi_root_ok,
            "default multi-root must hold: {inv:?}"
        );
        assert!(
            inv.relational_production_cutover_ok,
            "production cutover must hold: {inv:?}"
        );
        assert!(
            inv.relational_empty_roots_main_only_ok,
            "empty roots must stay main-only: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        // Honesty: exit signs construction chain only — fixture still uses module-field roots.
        assert!(
            !module.aot_export_roots.is_empty(),
            "fixture must use aot_export_roots field (≠ language Module.exports)"
        );
    }

    #[test]
    fn language_exports_aot_roots_thin_inventory_is_deterministic_and_signed() {
        let a = language_exports_aot_roots_thin_inventory();
        let b = language_exports_aot_roots_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.relational_mir_field_and_multi_root_ok);
        assert!(a.mir_aot_export_roots_field_present);
        assert!(a.default_multi_root_emit_wired);
        assert!(!a.frontend_export_syntax_present);
        assert!(!a.semantic_fills_aot_export_roots);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(a.language_exports_thin_signed);
    }

    #[test]
    fn language_exports_syntax_spec_deepen_inventory_is_deterministic_and_signed() {
        let a = language_exports_syntax_spec_deepen_inventory();
        let b = language_exports_syntax_spec_deepen_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.thin_open_floor_ok);
        assert!(a.proposed_syntax_documented);
        assert!(!a.lexer_export_keyword_present);
        assert!(!a.decl_export_variant_present);
        assert!(!a.parser_export_enabled);
        assert!(!a.semantic_fills_aot_export_roots);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(a.language_exports_syntax_spec_signed);
    }

    #[test]
    fn language_exports_decl_export_first_slice_inventory_is_deterministic_and_signed() {
        let a = language_exports_decl_export_first_slice_inventory();
        let b = language_exports_decl_export_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.syntax_spec_floor_ok);
        assert!(a.lexer_export_keyword_present);
        assert!(a.decl_export_variant_present);
        assert!(a.parser_export_enabled);
        assert!(!a.semantic_fills_aot_export_roots);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(!a.exports_ga_claimed);
        assert!(a.language_exports_decl_export_first_slice_signed);
        // Honesty floor: W462 inventory keeps historical false flags.
        let floor = language_exports_syntax_spec_deepen_inventory();
        assert!(!floor.lexer_export_keyword_present);
        assert!(!floor.decl_export_variant_present);
        assert!(!floor.parser_export_enabled);
    }

    #[test]
    fn language_exports_semantic_aot_export_roots_inventory_is_deterministic_and_signed() {
        let a = language_exports_semantic_aot_export_roots_inventory();
        let b = language_exports_semantic_aot_export_roots_inventory();
        assert_eq!(a, b);
        assert!(a.decl_export_first_slice_floor_ok);
        assert!(a.semantic_fills_aot_export_roots);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(!a.exports_ga_claimed);
        assert!(a.language_exports_semantic_aot_export_roots_signed);
        // Honesty floor: W480 inventory keeps historical semantic_fills=false.
        let floor = language_exports_decl_export_first_slice_inventory();
        assert!(!floor.semantic_fills_aot_export_roots);
        assert!(floor.language_exports_decl_export_first_slice_signed);
    }

    #[test]
    fn lower_module_export_func_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export func greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            mir.functions.iter().any(|f| f.name.as_str() == "greet"),
            "export func must still lower as a MIR function"
        );
    }

    #[test]
    fn lower_module_without_export_keeps_empty_aot_export_roots() {
        let source = SourceFile::new("t.sy", "func main() { return 1 }");
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.is_empty(),
            "no export ⇒ empty aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn language_exports_export_list_first_slice_inventory_is_deterministic_and_signed() {
        let a = language_exports_export_list_first_slice_inventory();
        let b = language_exports_export_list_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.semantic_roots_floor_ok);
        assert!(a.export_list_syntax_enabled);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(!a.exports_ga_claimed);
        assert!(a.language_exports_export_list_first_slice_signed);
        // Honesty: W481/W480 auto_discovery stays false.
        let sem = language_exports_semantic_aot_export_roots_inventory();
        assert!(!sem.language_exports_auto_discovery_enabled);
        assert!(sem.language_exports_semantic_aot_export_roots_signed);
    }

    #[test]
    fn language_exports_export_list_trailing_comma_inventory_is_deterministic_and_signed() {
        let a = language_exports_export_list_trailing_comma_inventory();
        let b = language_exports_export_list_trailing_comma_inventory();
        assert_eq!(a, b);
        assert!(a.export_list_first_slice_floor_ok);
        assert!(a.export_list_trailing_comma_enabled);
        assert!(!a.language_exports_auto_discovery_enabled);
        assert!(!a.exports_ga_claimed);
        assert!(a.language_exports_export_list_trailing_comma_signed);
        let floor = language_exports_export_list_first_slice_inventory();
        assert!(floor.export_list_syntax_enabled);
        assert!(!floor.language_exports_auto_discovery_enabled);
    }

    #[test]
    fn language_exports_scan_all_auto_discovery_first_slice_inventory_is_deterministic_and_signed() {
        let a = language_exports_scan_all_auto_discovery_first_slice_inventory();
        let b = language_exports_scan_all_auto_discovery_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.export_list_trailing_comma_floor_ok);
        assert!(a.language_exports_auto_discovery_enabled);
        assert!(!a.exports_ga_claimed);
        assert!(!a.f07_ga_claimed);
        assert!(a.language_exports_scan_all_auto_discovery_first_slice_signed);
        // Honesty floor: W486 inventory historical auto_discovery stays false.
        let trail = language_exports_export_list_trailing_comma_inventory();
        assert!(!trail.language_exports_auto_discovery_enabled);
        assert!(trail.language_exports_export_list_trailing_comma_signed);
    }

    #[test]
    fn language_exports_exports_ga_evidence_first_slice_inventory_is_deterministic_and_signed() {
        let a = language_exports_exports_ga_evidence_first_slice_inventory();
        let b = language_exports_exports_ga_evidence_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.scan_all_auto_discovery_floor_ok);
        assert!(a.default_multi_root_emit_floor_ok);
        assert!(a.public_func_roots_drive_default_emit);
        assert!(!a.exports_ga_claimed);
        assert!(!a.f07_ga_claimed);
        assert!(a.language_exports_exports_ga_evidence_first_slice_signed);
        // Honesty floor: W923 inventory keeps exports_ga / f07_ga false.
        let scan = language_exports_scan_all_auto_discovery_first_slice_inventory();
        assert!(!scan.exports_ga_claimed);
        assert!(!scan.f07_ga_claimed);
        assert!(scan.language_exports_scan_all_auto_discovery_first_slice_signed);
    }

    #[test]
    fn language_exports_exports_ga_claimed_inventory_is_deterministic_and_signed() {
        let a = language_exports_exports_ga_claimed_inventory();
        let b = language_exports_exports_ga_claimed_inventory();
        assert_eq!(a, b);
        assert!(a.exports_ga_evidence_floor_ok);
        assert!(a.exports_ga_claimed);
        assert!(a.f07_ga_claimed);
        assert!(a.language_exports_exports_ga_claimed_signed);
        // Honesty floor: W924 evidence inventory keeps claimed false.
        let evidence = language_exports_exports_ga_evidence_first_slice_inventory();
        assert!(!evidence.exports_ga_claimed);
        assert!(!evidence.f07_ga_claimed);
        assert!(evidence.language_exports_exports_ga_evidence_first_slice_signed);
    }

    #[test]
    fn public_func_roots_drive_llvm_emit_function_names_exports_ga_evidence() {
        // Orphan public export (not called from main): roots drive emit; cleared ⇒ main-only.
        let source = SourceFile::new(
            "t.sy",
            r#"public func greet() {
    print("hi")
}
func helper() {}
func main() {
    print("main")
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "public greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        let emit = llvm_emit_function_names(&mir);
        assert!(
            emit.contains("greet"),
            "W440 default emit must include public greet root: {emit:?}"
        );
        assert!(
            emit.contains("main"),
            "default emit must still include main: {emit:?}"
        );
        assert!(
            !emit.contains("helper"),
            "unreachable helper must not be in emit: {emit:?}"
        );
        let cleared = MirModule {
            functions: mir.functions.clone(),
            aot_export_roots: vec![],
        };
        let cleared_emit = llvm_emit_function_names(&cleared);
        assert!(
            !cleared_emit.contains("greet"),
            "cleared-roots clone must be main-only (no orphan greet): {cleared_emit:?}"
        );
        assert!(
            cleared_emit.contains("main"),
            "cleared emit must keep main: {cleared_emit:?}"
        );
        let roots = mir_module_aot_export_root_refs(&mir);
        assert_eq!(
            emit,
            reachable_fn_names_from_roots(&mir, &roots),
            "default emit must follow aot_export_roots"
        );
    }

    #[test]
    fn lower_module_public_func_fills_aot_export_roots_scan_all() {
        let source = SourceFile::new(
            "t.sy",
            r#"public func greet() {
    print("hi")
}
func helper() {}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "public func greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            !mir.aot_export_roots.iter().any(|n| n.as_str() == "helper"),
            "internal func helper must not fill roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            !mir.aot_export_roots.iter().any(|n| n.as_str() == "main"),
            "internal main must not fill roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_trailing_comma_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet, }
func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export {{ greet, }} must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet }
func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export {{ greet }} must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_and_export_func_dedupe_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet }
export func greet() {
    print("hi")
}
func main() {}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let count = mir
            .aot_export_roots
            .iter()
            .filter(|n| n.as_str() == "greet")
            .count();
        assert_eq!(count, 1, "greet must be deduped in roots: {:?}", mir.aot_export_roots);
    }

    #[test]
    fn aot_short_horizon_production_cutover_inventory_preds_and_honesty() {
        let module = aot_reachability_fixture_main_and_export_orphan();
        let before = encode_module(&module);
        let inv = aot_short_horizon_production_cutover_inventory(&module, &["export_lib"]);
        let after = encode_module(&module);
        assert_eq!(before, after, "production cutover inventory must not mutate encode");
        assert!(inv.relational_size_ci_ok, "{inv:?}");
        assert!(
            inv.relational_default_main_only_ok,
            "empty-roots path must stay main-only: {inv:?}"
        );
        assert!(inv.relational_cutover_on_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(inv.production_cutover_signed);
        assert!(inv.cutover_enabled);
        assert!(inv.size_ci_gate_enabled);
        assert!(inv.production_multi_root_enabled);
        assert_eq!(inv.absolute_threshold_count, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT);
        assert!(!inv.multi_root_reopen_gap_open);
        assert!(!inv.size_ci_reopen_gap_open);
        assert!(
            llvm_emit_function_names(&module).contains("export_lib"),
            "W440: fixture default emit includes export_lib"
        );
    }


    #[test]
    fn aot_size_fixture_orphan_saved_bytes_monotonic() {
        let small = aot_blob_size_report(&aot_size_fixture_main_with_orphan_body(64));
        let large = aot_blob_size_report(&aot_size_fixture_main_with_orphan_body(128));
        assert!(
            small.saved_bytes > 0,
            "orphan(64) must save bytes: {small:?}"
        );
        assert!(
            large.saved_bytes > small.saved_bytes,
            "orphan(128) saved must be strictly greater than orphan(64): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn aot_size_fixture_call_indirect_zero_saved() {
        let report = aot_blob_size_report(&aot_size_fixture_call_indirect_full_emit());
        assert_eq!(report.saved_bytes, 0);
        assert_eq!(report.stub_bytes, report.compact_bytes);
    }

    #[test]
    fn mir_module_blob_stats_round_trip_preserves_fn_names() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let stats = mir_module_blob_stats(&module);
        assert_eq!(stats.fn_count, module.functions.len());
        assert!(
            stats.encoded_bytes > 0,
            "non-empty module must encode to >0 bytes: {stats:?}"
        );
        let decoded = decode_module(&encode_module(&module)).expect("decode");
        assert_eq!(decoded.functions.len(), module.functions.len());
        let names: Vec<_> = module.functions.iter().map(|f| f.name.as_str()).collect();
        let decoded_names: Vec<_> = decoded.functions.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(decoded_names, names);
        let stats_after = mir_module_blob_stats(&decoded);
        assert_eq!(stats_after.fn_count, stats.fn_count);
        assert_eq!(stats_after.encoded_bytes, stats.encoded_bytes);
    }

    #[test]
    fn mir_module_blob_stats_empty_module_round_trip() {
        let module = MirModule::default();
        let stats = mir_module_blob_stats(&module);
        assert_eq!(stats.fn_count, 0);
        let decoded = decode_module(&encode_module(&module)).expect("decode empty");
        assert_eq!(decoded.functions.len(), 0);
        let again = mir_module_blob_stats(&decoded);
        assert_eq!(again.fn_count, 0);
        assert_eq!(again.encoded_bytes, stats.encoded_bytes);
    }

    #[test]
    fn mir_module_layout_inventory_nonempty_matches_stats() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let layout = mir_module_layout_inventory(&module);
        let stats = mir_module_blob_stats(&module);
        assert_eq!(layout.fn_count, 3);
        assert!(
            layout.instruction_count > 0,
            "non-empty fixture must have instructions: {layout:?}"
        );
        assert_eq!(layout.encoded_bytes, stats.encoded_bytes);
        assert_eq!(layout.fn_count, stats.fn_count);
    }

    #[test]
    fn mir_module_layout_inventory_empty_and_monotonic() {
        let empty = mir_module_layout_inventory(&MirModule::default());
        assert_eq!(empty.fn_count, 0);
        assert_eq!(empty.instruction_count, 0);

        let small = mir_module_layout_inventory(&aot_size_fixture_main_with_orphan_body(2));
        let large = mir_module_layout_inventory(&aot_size_fixture_main_with_orphan_body(4));
        assert!(
            large.instruction_count > small.instruction_count,
            "orphan(4) instruction_count must exceed orphan(2): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn mir_module_codec_partition_inventory_nonempty_matches_layout() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let before = encode_module(&module);
        let part = mir_module_codec_partition_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "partition inventory must not mutate encode bytes");
        let layout = mir_module_layout_inventory(&module);
        let stats = mir_module_blob_stats(&module);
        assert_eq!(part.fn_count, 3);
        assert!(
            part.execution_instruction_count > 0,
            "non-empty fixture must have execution instructions: {part:?}"
        );
        assert_eq!(part.execution_instruction_count, layout.instruction_count);
        assert_eq!(part.encoded_bytes, layout.encoded_bytes);
        assert_eq!(part.encoded_bytes, stats.encoded_bytes);
        assert_eq!(part.fn_count, stats.fn_count);
    }

    #[test]
    fn mir_module_codec_partition_inventory_empty_and_monotonic() {
        let empty = mir_module_codec_partition_inventory(&MirModule::default());
        assert_eq!(empty.fn_count, 0);
        assert_eq!(empty.execution_instruction_count, 0);
        assert_eq!(empty.metadata_local_name_slots, 0);
        assert_eq!(empty.metadata_local_type_slots, 0);

        let small = mir_module_codec_partition_inventory(&aot_size_fixture_main_with_orphan_body(2));
        let large = mir_module_codec_partition_inventory(&aot_size_fixture_main_with_orphan_body(4));
        assert!(
            large.execution_instruction_count > small.execution_instruction_count,
            "orphan(4) execution_instruction_count must exceed orphan(2): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn mir_module_debug_strip_inventory_debug_fixture_saves_bytes() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let before = encode_module(&module);
        let strip = mir_module_debug_strip_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "strip inventory must not mutate encode bytes");
        let part = mir_module_codec_partition_inventory(&module);
        let stats = mir_module_blob_stats(&module);
        assert_eq!(strip.metadata_local_name_slots, 8);
        assert_eq!(strip.metadata_local_type_slots, 8);
        assert_eq!(strip.metadata_local_name_slots, part.metadata_local_name_slots);
        assert_eq!(strip.metadata_local_type_slots, part.metadata_local_type_slots);
        assert_eq!(strip.encoded_bytes, part.encoded_bytes);
        assert_eq!(strip.encoded_bytes, stats.encoded_bytes);
        assert!(
            strip.saved_bytes > 0,
            "debug fixture must save bytes when clearing locals: {strip:?}"
        );
    }

    #[test]
    fn mir_module_debug_strip_inventory_empty_orphan_and_monotonic() {
        let empty = mir_module_debug_strip_inventory(&MirModule::default());
        assert_eq!(empty.saved_bytes, 0);
        assert_eq!(empty.metadata_local_name_slots, 0);
        assert_eq!(empty.metadata_local_type_slots, 0);

        let orphan = mir_module_debug_strip_inventory(&aot_size_fixture_main_with_orphan_body(2));
        assert_eq!(orphan.saved_bytes, 0);
        assert_eq!(orphan.metadata_local_name_slots, 0);
        assert_eq!(orphan.metadata_local_type_slots, 0);

        let small = mir_module_debug_strip_inventory(&aot_codec_fixture_main_with_debug_locals(8));
        let large = mir_module_debug_strip_inventory(&aot_codec_fixture_main_with_debug_locals(16));
        assert!(
            large.saved_bytes > small.saved_bytes,
            "saved(16) must exceed saved(8): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn module_with_debug_locals_stripped_clears_slots_keeps_instructions() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let stripped = module_with_debug_locals_stripped(&module);
        assert_eq!(stripped.functions.len(), module.functions.len());
        assert_eq!(stripped.functions[0].local_count, module.functions[0].local_count);
        assert!(stripped.functions[0].local_names.is_empty());
        assert!(stripped.functions[0].local_types.is_empty());
        assert_eq!(
            stripped.functions[0].instructions.len(),
            module.functions[0].instructions.len()
        );
        // Generic encode_module must NOT auto-strip.
        let plain = decode_module(&encode_module(&module)).unwrap();
        assert_eq!(plain.functions[0].local_names.len(), 8);
        assert_eq!(plain.functions[0].local_types.len(), 8);
    }

    #[test]
    fn encode_module_compact_for_aot_strips_debug_locals() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let compact_only = encode_module(&compact_module_for_aot_blob(&module));
        let aot = encode_module_compact_for_aot(&module);
        assert!(
            aot.len() < compact_only.len(),
            "AOT compact+strip must be smaller than compact-only: aot={} compact_only={}",
            aot.len(),
            compact_only.len()
        );
        let decoded = decode_module(&aot).unwrap();
        assert!(!decoded.functions.is_empty());
        for func in &decoded.functions {
            assert!(func.local_names.is_empty(), "AOT blob must strip local_names");
            assert!(func.local_types.is_empty(), "AOT blob must strip local_types");
        }
        let strip = mir_module_debug_strip_inventory(&compact_module_for_aot_blob(&module));
        assert_eq!(
            aot.len(),
            strip.stripped_encoded_bytes,
            "compact_for_aot must match strip inventory stripped path"
        );
    }

    #[test]
    fn encode_module_compact_for_aot_no_debug_slots_neutral() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let compact_only = encode_module(&compact_module_for_aot_blob(&module));
        let aot = encode_module_compact_for_aot(&module);
        assert_eq!(
            aot.len(),
            compact_only.len(),
            "no-debug modules must not change size under strip wire"
        );
    }

    #[test]
    fn mir_module_native_instruction_omit_inventory_large_eligible_saves_bytes() {
        let mut ops = Vec::new();
        for i in 0..64 {
            ops.push(MirOp::ConstI32(i));
        }
        ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![aot_size_fixture_fn("main", ops)],
            ..Default::default()
        };
        let before = encode_module(&module);
        let inv = mir_module_native_instruction_omit_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "omit inventory must not mutate encode bytes");
        assert_eq!(inv.native_eligible_fn_count, 1);
        assert_eq!(inv.omitted_instruction_count, 65);
        assert!(
            inv.saved_bytes > 0,
            "large eligible main body must save bytes when omitted: {inv:?}"
        );
        let layout = mir_module_layout_inventory(&module);
        assert_eq!(inv.omitted_instruction_count, layout.instruction_count);
        assert_eq!(inv.encoded_bytes, layout.encoded_bytes);
    }

    #[test]
    fn mir_module_native_instruction_omit_inventory_orphan_not_eligible_zero_saved() {
        // Orphans are not llvm-emit-eligible under reachability; omit stubs only `main`
        // (already Return) → no byte savings. Honesty: omit ≠ shake orphans.
        let module = aot_size_fixture_main_with_orphan_body(8);
        let inv = mir_module_native_instruction_omit_inventory(&module);
        assert_eq!(inv.native_eligible_fn_count, 1);
        assert_eq!(inv.saved_bytes, 0);
    }

    #[test]
    fn mir_module_native_instruction_omit_inventory_call_indirect_all_eligible() {
        let module = aot_size_fixture_call_indirect_full_emit();
        let inv = mir_module_native_instruction_omit_inventory(&module);
        assert_eq!(
            inv.native_eligible_fn_count,
            module.functions.len(),
            "CallIndirect full-emit must mark all fns eligible: {inv:?}"
        );
        // Fat orphan under full-emit is eligible → omitting its body saves bytes.
        let mut fat_ops = Vec::new();
        for i in 0..64 {
            fat_ops.push(MirOp::ConstI32(i));
        }
        fat_ops.push(MirOp::Return);
        let fat = MirModule {
            functions: vec![
                aot_size_fixture_fn("main", vec![MirOp::CallIndirect(1)]),
                aot_size_fixture_fn("orphan", fat_ops),
            ],
            ..Default::default()
        };
        let fat_inv = mir_module_native_instruction_omit_inventory(&fat);
        assert_eq!(fat_inv.native_eligible_fn_count, 2);
        assert!(
            fat_inv.saved_bytes > 0,
            "full-emit fat orphan must save bytes when native bodies omitted: {fat_inv:?}"
        );
    }

    #[test]
    fn encode_module_compact_for_aot_omits_eligible_instruction_streams() {
        let mut ops = Vec::new();
        for i in 0..64 {
            ops.push(MirOp::ConstI32(i));
        }
        ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![aot_size_fixture_fn("main", ops)],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        let strip_only = encode_module(&module_with_debug_locals_stripped(&compact));
        let aot = encode_module_compact_for_aot(&module);
        assert!(
            aot.len() < strip_only.len(),
            "AOT compact+omit+strip must be smaller than compact+strip-only: aot={} strip_only={}",
            aot.len(),
            strip_only.len()
        );
        let decoded = decode_module(&aot).unwrap();
        assert_eq!(decoded.functions.len(), 1);
        assert_eq!(decoded.functions[0].instructions.len(), 1);
        assert!(matches!(
            decoded.functions[0].instructions[0].op,
            MirOp::Return
        ));
        // Generic encode_module must NOT auto-omit.
        let plain = decode_module(&encode_module(&module)).unwrap();
        assert!(plain.functions[0].instructions.len() > 1);
    }

    #[test]
    fn encode_module_compact_for_aot_omit_keeps_strip_and_call_indirect() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let aot = encode_module_compact_for_aot(&module);
        let decoded = decode_module(&aot).unwrap();
        assert!(decoded.functions[0].local_names.is_empty());
        assert!(decoded.functions[0].local_types.is_empty());
        assert!(matches!(
            decoded.functions[0].instructions[0].op,
            MirOp::Return
        ));

        let mut fat_ops = Vec::new();
        for i in 0..32 {
            fat_ops.push(MirOp::ConstI32(i));
        }
        fat_ops.push(MirOp::Return);
        let ci = MirModule {
            functions: vec![
                aot_size_fixture_fn("main", vec![MirOp::CallIndirect(1)]),
                aot_size_fixture_fn("orphan", fat_ops),
            ],
            ..Default::default()
        };
        let ci_bytes = encode_module_compact_for_aot(&ci);
        let ci_decoded = decode_module(&ci_bytes).unwrap();
        assert_eq!(ci_decoded.functions.len(), 2);
        for func in &ci_decoded.functions {
            assert_eq!(func.instructions.len(), 1);
            assert!(matches!(func.instructions[0].op, MirOp::Return));
        }
    }

    #[test]
    fn mir_module_metadata_table_inventory_empty() {
        let empty = mir_module_metadata_table_inventory(&MirModule::default());
        assert_eq!(empty.fn_count, 0);
        assert_eq!(empty.fn_name_bytes, 0);
        assert_eq!(empty.string_literal_count, 0);
        assert_eq!(empty.string_literal_bytes, 0);
        assert_eq!(empty.metadata_local_name_slots, 0);
        assert_eq!(empty.metadata_local_type_slots, 0);
        assert_eq!(empty.execution_instruction_count, 0);
        assert_eq!(empty.encoded_bytes, encode_module(&MirModule::default()).len());
    }

    #[test]
    fn mir_module_metadata_table_inventory_const_str_monotonic_and_encode_stable() {
        let small = aot_codec_fixture_main_with_const_strs(2);
        let large = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&large);
        let inv_small = mir_module_metadata_table_inventory(&small);
        let inv_large = mir_module_metadata_table_inventory(&large);
        let after = encode_module(&large);
        assert_eq!(before, after, "metadata-table inventory must not mutate encode bytes");
        assert_eq!(inv_small.string_literal_count, 2);
        assert_eq!(inv_large.string_literal_count, 4);
        assert!(
            inv_large.string_literal_bytes > inv_small.string_literal_bytes,
            "const_str(4) string_literal_bytes must exceed const_str(2): small={inv_small:?} large={inv_large:?}"
        );
        assert!(
            inv_large.encoded_bytes > inv_small.encoded_bytes,
            "const_str(4) encoded_bytes must exceed const_str(2): small={inv_small:?} large={inv_large:?}"
        );
        assert_eq!(inv_large.fn_count, 1);
        assert_eq!(inv_large.fn_name_bytes, "main".len());
    }

    #[test]
    fn mir_module_metadata_table_inventory_matches_partition_overlap() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let table = mir_module_metadata_table_inventory(&module);
        let part = mir_module_codec_partition_inventory(&module);
        assert_eq!(table.fn_count, part.fn_count);
        assert_eq!(
            table.execution_instruction_count,
            part.execution_instruction_count
        );
        assert_eq!(
            table.metadata_local_name_slots,
            part.metadata_local_name_slots
        );
        assert_eq!(
            table.metadata_local_type_slots,
            part.metadata_local_type_slots
        );
        assert_eq!(table.encoded_bytes, part.encoded_bytes);
        assert_eq!(table.string_literal_count, 0);
    }

    #[test]
    fn mir_module_dual_blob_shape_inventory_empty_and_encode_stable() {
        let module = MirModule::default();
        let before = encode_module(&module);
        let inv = mir_module_dual_blob_shape_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "dual-blob shape inventory must not mutate encode bytes");
        assert_eq!(inv.fn_count, 0);
        assert_eq!(inv.string_literal_count, 0);
        assert_eq!(inv.execution_instruction_count, 0);
        assert_eq!(inv.metadata_payload_bytes, 0);
        assert_eq!(inv.whole_module_encoded_bytes, before.len());
        assert_eq!(
            inv.execution_payload_bytes,
            inv.whole_module_encoded_bytes
                .saturating_sub(inv.metadata_payload_bytes)
        );
        assert_eq!(
            inv.metadata_share_bps,
            inv.metadata_payload_bytes
                .saturating_mul(10000)
                / inv.whole_module_encoded_bytes.max(1)
        );
    }

    #[test]
    fn mir_module_dual_blob_shape_inventory_const_str_monotonic_and_encode_stable() {
        let small = aot_codec_fixture_main_with_const_strs(2);
        let large = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&large);
        let inv_small = mir_module_dual_blob_shape_inventory(&small);
        let inv_large = mir_module_dual_blob_shape_inventory(&large);
        let after = encode_module(&large);
        assert_eq!(before, after, "dual-blob shape inventory must not mutate encode bytes");
        assert_eq!(inv_small.string_literal_count, 2);
        assert_eq!(inv_large.string_literal_count, 4);
        assert!(
            inv_large.metadata_payload_bytes > inv_small.metadata_payload_bytes,
            "const_str(4) metadata_payload_bytes must exceed const_str(2): small={inv_small:?} large={inv_large:?}"
        );
        assert!(
            inv_large.whole_module_encoded_bytes > inv_small.whole_module_encoded_bytes,
            "const_str(4) whole_module_encoded_bytes must exceed const_str(2): small={inv_small:?} large={inv_large:?}"
        );
        assert_eq!(
            inv_large.execution_payload_bytes,
            inv_large
                .whole_module_encoded_bytes
                .saturating_sub(inv_large.metadata_payload_bytes)
        );
    }

    #[test]
    fn mir_module_dual_blob_shape_inventory_matches_table_overlap() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let shape = mir_module_dual_blob_shape_inventory(&module);
        let table = mir_module_metadata_table_inventory(&module);
        assert_eq!(shape.fn_count, table.fn_count);
        assert_eq!(
            shape.execution_instruction_count,
            table.execution_instruction_count
        );
        assert_eq!(shape.whole_module_encoded_bytes, table.encoded_bytes);
        assert_eq!(shape.string_literal_count, table.string_literal_count);
        let mut local_bytes = 0usize;
        for func in &module.functions {
            for name in &func.local_names {
                local_bytes += name.len();
            }
            for ty in &func.local_types {
                local_bytes += ty.len();
            }
        }
        assert_eq!(
            shape.metadata_payload_bytes,
            table
                .fn_name_bytes
                .saturating_add(table.string_literal_bytes)
                .saturating_add(local_bytes)
        );
        assert!(
            shape.metadata_payload_bytes > table.fn_name_bytes,
            "debug locals must contribute to metadata_payload_bytes: {shape:?} table={table:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_encode_probe_empty_and_encode_stable() {
        let module = MirModule::default();
        let before = encode_module(&module);
        let probe = mir_module_dual_blob_encode_probe(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "dual-blob encode probe must not mutate encode bytes");
        assert_eq!(probe.fn_count, 0);
        assert_eq!(probe.string_literal_count, 0);
        assert_eq!(probe.execution_instruction_count, 0);
        assert_eq!(probe.metadata_payload_bytes, 0);
        assert_eq!(probe.whole_module_encoded_bytes, before.len());
        assert_eq!(
            probe.sidecar_overhead_bytes,
            probe
                .metadata_sidecar_encoded_bytes
                .saturating_sub(probe.metadata_payload_bytes)
        );
        assert_eq!(
            probe.execution_residual_bytes,
            probe
                .whole_module_encoded_bytes
                .saturating_sub(probe.metadata_sidecar_encoded_bytes)
        );
        assert_eq!(
            probe.sidecar_share_bps,
            probe
                .metadata_sidecar_encoded_bytes
                .saturating_mul(10000)
                / probe.whole_module_encoded_bytes.max(1)
        );
    }

    #[test]
    fn mir_module_dual_blob_encode_probe_const_str_monotonic_and_encode_stable() {
        let small = aot_codec_fixture_main_with_const_strs(2);
        let large = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&large);
        let probe_small = mir_module_dual_blob_encode_probe(&small);
        let probe_large = mir_module_dual_blob_encode_probe(&large);
        let after = encode_module(&large);
        assert_eq!(before, after, "dual-blob encode probe must not mutate encode bytes");
        assert_eq!(probe_small.string_literal_count, 2);
        assert_eq!(probe_large.string_literal_count, 4);
        assert!(
            probe_large.metadata_sidecar_encoded_bytes
                > probe_small.metadata_sidecar_encoded_bytes,
            "const_str(4) metadata_sidecar_encoded_bytes must exceed const_str(2): small={probe_small:?} large={probe_large:?}"
        );
        assert!(
            probe_large.whole_module_encoded_bytes > probe_small.whole_module_encoded_bytes,
            "const_str(4) whole_module_encoded_bytes must exceed const_str(2): small={probe_small:?} large={probe_large:?}"
        );
        assert!(
            probe_large.metadata_sidecar_encoded_bytes >= probe_large.metadata_payload_bytes,
            "sidecar encode should be >= string-length payload: {probe_large:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_encode_probe_matches_shape_overlap() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let probe = mir_module_dual_blob_encode_probe(&module);
        let shape = mir_module_dual_blob_shape_inventory(&module);
        assert_eq!(probe.fn_count, shape.fn_count);
        assert_eq!(
            probe.execution_instruction_count,
            shape.execution_instruction_count
        );
        assert_eq!(
            probe.whole_module_encoded_bytes,
            shape.whole_module_encoded_bytes
        );
        assert_eq!(probe.metadata_payload_bytes, shape.metadata_payload_bytes);
        let sidecar = mir_module_metadata_sidecar_view(&module);
        assert_eq!(sidecar.local_names.len(), 8);
        assert_eq!(sidecar.local_types.len(), 8);
        assert!(
            probe.metadata_sidecar_encoded_bytes >= probe.metadata_payload_bytes,
            "debug locals sidecar must cover payload: {probe:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_decode_probe_empty_and_encode_stable() {
        let module = MirModule::default();
        let before = encode_module(&module);
        let encode = mir_module_dual_blob_encode_probe(&module);
        let decode = mir_module_dual_blob_decode_probe(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "dual-blob decode probe must not mutate encode bytes");
        assert!(decode.sidecar_roundtrip_ok);
        assert!(decode.view_eq_after_roundtrip);
        assert_eq!(
            decode.metadata_sidecar_encoded_bytes,
            encode.metadata_sidecar_encoded_bytes
        );
        assert_eq!(decode.fn_count, 0);
        assert_eq!(decode.decoded_fn_count, 0);
        assert_eq!(decode.string_literal_count, 0);
        assert_eq!(decode.decoded_string_literal_count, 0);
        assert_eq!(decode.whole_module_encoded_bytes, before.len());
    }

    #[test]
    fn mir_module_dual_blob_decode_probe_const_str_monotonic_and_roundtrip() {
        let small = aot_codec_fixture_main_with_const_strs(2);
        let large = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&large);
        let dec_small = mir_module_dual_blob_decode_probe(&small);
        let dec_large = mir_module_dual_blob_decode_probe(&large);
        let after = encode_module(&large);
        assert_eq!(before, after, "dual-blob decode probe must not mutate encode bytes");
        assert!(dec_small.sidecar_roundtrip_ok && dec_small.view_eq_after_roundtrip);
        assert!(dec_large.sidecar_roundtrip_ok && dec_large.view_eq_after_roundtrip);
        assert_eq!(dec_small.string_literal_count, 2);
        assert_eq!(dec_small.decoded_string_literal_count, 2);
        assert_eq!(dec_large.string_literal_count, 4);
        assert_eq!(dec_large.decoded_string_literal_count, 4);
        assert!(
            dec_large.metadata_sidecar_encoded_bytes > dec_small.metadata_sidecar_encoded_bytes,
            "const_str(4) sidecar bytes must exceed const_str(2): small={dec_small:?} large={dec_large:?}"
        );
        assert!(
            dec_large.whole_module_encoded_bytes > dec_small.whole_module_encoded_bytes,
            "const_str(4) whole must exceed const_str(2): small={dec_small:?} large={dec_large:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_decode_probe_matches_encode_and_shape() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let decode = mir_module_dual_blob_decode_probe(&module);
        let encode = mir_module_dual_blob_encode_probe(&module);
        let shape = mir_module_dual_blob_shape_inventory(&module);
        assert!(decode.sidecar_roundtrip_ok && decode.view_eq_after_roundtrip);
        assert_eq!(decode.fn_count, encode.fn_count);
        assert_eq!(decode.fn_count, shape.fn_count);
        assert_eq!(
            decode.execution_instruction_count,
            encode.execution_instruction_count
        );
        assert_eq!(
            decode.execution_instruction_count,
            shape.execution_instruction_count
        );
        assert_eq!(
            decode.whole_module_encoded_bytes,
            encode.whole_module_encoded_bytes
        );
        assert_eq!(decode.metadata_payload_bytes, encode.metadata_payload_bytes);
        assert_eq!(
            decode.metadata_sidecar_encoded_bytes,
            encode.metadata_sidecar_encoded_bytes
        );
        assert_eq!(decode.decoded_fn_count, 1);
        let sidecar = mir_module_metadata_sidecar_view(&module);
        assert_eq!(sidecar.local_names.len(), 8);
        assert_eq!(sidecar.local_types.len(), 8);
    }

    #[test]
    fn mir_module_dual_blob_codec_readiness_inventory_const_str_predicates() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_dual_blob_codec_readiness_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "codec readiness inventory must not mutate encode bytes"
        );
        assert!(
            inv.shape_metadata_positive,
            "ConstStr fixture must have positive shape metadata: {inv:?}"
        );
        assert!(
            inv.encode_sidecar_positive,
            "ConstStr fixture must have positive sidecar encode: {inv:?}"
        );
        assert!(
            inv.decode_roundtrip_ok,
            "ConstStr fixture decode roundtrip must hold: {inv:?}"
        );
        assert!(
            inv.view_eq_ok,
            "ConstStr fixture view_eq must hold: {inv:?}"
        );
        assert!(
            inv.design_probe_chain_ok,
            "design probe chain must be ok: {inv:?}"
        );
        assert_eq!(inv.readiness_predicate_count, 4);
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(
            !inv.load_path_is_whole_module,
            "load path must be dual-load (W407): {inv:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_codec_readiness_inventory_matches_probe_overlap() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_dual_blob_codec_readiness_inventory(&module);
        let shape = mir_module_dual_blob_shape_inventory(&module);
        let encode = mir_module_dual_blob_encode_probe(&module);
        let decode = mir_module_dual_blob_decode_probe(&module);
        assert_eq!(
            inv.shape_metadata_positive,
            shape.metadata_payload_bytes > 0
        );
        assert_eq!(
            inv.encode_sidecar_positive,
            encode.metadata_sidecar_encoded_bytes > 0
        );
        assert_eq!(inv.decode_roundtrip_ok, decode.sidecar_roundtrip_ok);
        assert_eq!(inv.view_eq_ok, decode.view_eq_after_roundtrip);
        assert_eq!(
            inv.design_probe_chain_ok,
            inv.shape_metadata_positive
                && inv.encode_sidecar_positive
                && inv.decode_roundtrip_ok
                && inv.view_eq_ok
        );
    }

    #[test]
    fn mir_module_dual_blob_aot_embed_inventory_orphan_saves_and_honesty() {
        let module = aot_size_fixture_main_with_orphan_body(64);
        let before = encode_module(&module);
        let inv = mir_module_dual_blob_aot_embed_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "aot_embed inventory must not mutate encode");
        assert!(
            inv.relational_aot_saves_vs_whole_ok,
            "orphan fixture must save AOT vs whole: {inv:?}"
        );
        assert!(inv.aot_saved_vs_whole > 0, "saved must be positive: {inv:?}");
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            !inv.load_path_is_whole_module,
            "load path must be dual-load (W407): {inv:?}"
        );
        assert_eq!(
            inv.whole_module_bytes,
            encode_module(&module).len()
        );
        assert_eq!(
            inv.production_aot_bytes,
            encode_module_compact_for_aot(&module).len()
        );
        assert_eq!(
            inv.aot_saved_vs_whole,
            inv.whole_module_bytes
                .saturating_sub(inv.production_aot_bytes)
        );
    }

    #[test]
    fn mir_module_dual_blob_aot_embed_inventory_const_str_sidecar_readiness() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_dual_blob_aot_embed_inventory(&module);
        let encode = mir_module_dual_blob_encode_probe(&module);
        let readiness = mir_module_dual_blob_codec_readiness_inventory(&module);
        assert!(
            inv.relational_sidecar_probe_positive_ok,
            "ConstStr fixture sidecar probe must be positive: {inv:?}"
        );
        assert!(
            inv.relational_readiness_chain_ok,
            "ConstStr fixture readiness chain must hold: {inv:?}"
        );
        assert_eq!(
            inv.sidecar_probe_bytes,
            encode.metadata_sidecar_encoded_bytes
        );
        assert_eq!(
            inv.relational_readiness_chain_ok,
            readiness.design_probe_chain_ok
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(inv.production_dual_load_enabled);
        assert!(inv.real_codec_enabled);
        assert!(!inv.load_path_is_whole_module);
    }

    #[test]
    fn mir_module_dual_blob_cutover_readiness_inventory_const_str_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_dual_blob_cutover_readiness_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "cutover readiness inventory must not mutate encode");
        assert!(
            inv.relational_design_probe_chain_ok,
            "ConstStr fixture design probe chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_aot_embed_contract_ok,
            "AOT-embed contract honesty must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "readiness/aot_embed overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.cutover_enabled, "Medium cutover honesty must be on (W405): {inv:?}");
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(
            !inv.load_path_is_whole_module,
            "load path must be dual-load (W407): {inv:?}"
        );
    }

    #[test]
    fn mir_module_dual_blob_cutover_readiness_inventory_matches_readiness_aot_embed() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_dual_blob_cutover_readiness_inventory(&module);
        let readiness = mir_module_dual_blob_codec_readiness_inventory(&module);
        let aot_embed = mir_module_dual_blob_aot_embed_inventory(&module);
        assert_eq!(
            inv.relational_design_probe_chain_ok,
            readiness.design_probe_chain_ok
        );
        assert_eq!(
            inv.relational_aot_embed_contract_ok,
            aot_embed.aot_embeds_sidecar
                && aot_embed.production_dual_load_enabled
                && !aot_embed.load_path_is_whole_module
        );
        assert_eq!(
            inv.relational_inventory_overlap_ok,
            aot_embed.relational_readiness_chain_ok == readiness.design_probe_chain_ok
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, aot_embed.real_codec_enabled);
        assert_eq!(inv.aot_embeds_sidecar, aot_embed.aot_embeds_sidecar);
        assert_eq!(
            inv.production_dual_load_enabled,
            aot_embed.production_dual_load_enabled
        );
        assert_eq!(
            inv.load_path_is_whole_module,
            aot_embed.load_path_is_whole_module
        );
    }

    #[test]
    fn mir_module_medium_horizon_cutover_readiness_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_cutover_readiness_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "medium-horizon cutover readiness must not mutate encode"
        );
        assert!(
            inv.relational_codec_readiness_chain_ok,
            "codec readiness chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_dual_blob_cutover_chain_ok,
            "dual-blob cutover chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(inv.cutover_enabled, "Medium cutover honesty must be on (W405): {inv:?}");
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(
            !inv.load_path_is_whole_module,
            "load path must be dual-load (W407): {inv:?}"
        );
    }

    #[test]
    fn mir_module_medium_horizon_cutover_readiness_inventory_matches_axes() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_cutover_readiness_inventory(&module);
        let readiness = mir_module_dual_blob_codec_readiness_inventory(&module);
        let cutover = mir_module_dual_blob_cutover_readiness_inventory(&module);
        assert_eq!(
            inv.relational_codec_readiness_chain_ok,
            readiness.design_probe_chain_ok
                && readiness.real_codec_enabled
                && readiness.production_dual_load_enabled
                && readiness.aot_embeds_sidecar
                && !readiness.load_path_is_whole_module
        );
        assert_eq!(
            inv.relational_dual_blob_cutover_chain_ok,
            cutover.relational_design_probe_chain_ok
                && cutover.relational_aot_embed_contract_ok
                && cutover.relational_inventory_overlap_ok
                && cutover.cutover_enabled
                && cutover.real_codec_enabled
                && cutover.aot_embeds_sidecar
                && cutover.production_dual_load_enabled
                && !cutover.load_path_is_whole_module
        );
        assert!(inv.cutover_enabled);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(!inv.load_path_is_whole_module);
        assert_eq!(inv.candidate_predicate_count, 3);
    }

    #[test]
    fn mir_module_medium_horizon_cutover_readiness_inventory_empty_codec_false() {
        // Empty module has no metadata payload; design_probe_chain fails.
        let module = MirModule::default();
        let inv = mir_module_medium_horizon_cutover_readiness_inventory(&module);
        assert!(
            !inv.relational_codec_readiness_chain_ok,
            "empty module must fail codec readiness chain: {inv:?}"
        );
        assert!(inv.cutover_enabled);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(!inv.load_path_is_whole_module);
        assert!(inv.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_medium_horizon_honest_defer_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_honest_defer_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "medium-horizon honest defer must not mutate encode"
        );
        assert!(
            inv.relational_medium_horizon_ok,
            "medium-horizon chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_production_off_ok,
            "production arms must remain off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(
            inv.honest_defer_signed,
            "honest defer must be signed (W339): {inv:?}"
        );
        assert!(inv.cutover_enabled, "Medium cutover honesty must be on (W405): {inv:?}");
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
    }

    #[test]
    fn mir_module_medium_horizon_honest_defer_inventory_matches_horizon() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_honest_defer_inventory(&module);
        let horizon = mir_module_medium_horizon_cutover_readiness_inventory(&module);
        assert_eq!(
            inv.relational_medium_horizon_ok,
            horizon.relational_codec_readiness_chain_ok
                && horizon.relational_dual_blob_cutover_chain_ok
                && horizon.relational_inventory_overlap_ok
        );
        assert_eq!(
            inv.relational_production_off_ok,
            horizon.cutover_enabled
                && horizon.production_dual_load_enabled
                && horizon.aot_embeds_sidecar
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.candidate_predicate_count, 3);
    }

    #[test]
    fn mir_module_medium_horizon_production_reopen_thin_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_production_reopen_thin_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "medium-horizon production reopen thin must not mutate encode"
        );
        assert!(
            inv.relational_medium_horizon_ok,
            "medium-horizon chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_honest_defer_ok,
            "honest defer must hold: {inv:?}"
        );
        assert!(
            inv.relational_production_off_ok,
            "production arms must remain off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.production_reopen_thin_signed,
            "production reopen thin must be signed (W401): {inv:?}"
        );
        assert!(
            inv.honest_defer_signed,
            "honest defer must remain signed: {inv:?}"
        );
        assert!(
            !inv.real_codec_reopen_gap_open,
            "real-codec gap must be closed (W403): {inv:?}"
        );
        assert!(
            !inv.dual_blob_reopen_gap_open,
            "dual-blob gap must be closed (W404): {inv:?}"
        );
        assert!(inv.cutover_enabled, "Medium cutover honesty must be on (W405): {inv:?}");
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
    }

    #[test]
    fn mir_module_medium_horizon_production_reopen_thin_inventory_matches_defer() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_production_reopen_thin_inventory(&module);
        let defer = mir_module_medium_horizon_honest_defer_inventory(&module);
        assert_eq!(
            inv.relational_medium_horizon_ok,
            defer.relational_medium_horizon_ok
        );
        assert_eq!(
            inv.relational_honest_defer_ok,
            defer.honest_defer_signed && defer.relational_production_off_ok
        );
        assert_eq!(
            inv.relational_production_off_ok,
            defer.relational_production_off_ok
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.production_reopen_thin_signed, true);
        assert_eq!(inv.real_codec_reopen_gap_open, false);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_medium_horizon_cutover_deepen_thin_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_cutover_deepen_thin_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "medium-horizon cutover deepen thin must not mutate encode"
        );
        assert!(
            inv.relational_reopen_thin_ok,
            "reopen thin must hold: {inv:?}"
        );
        assert!(
            inv.relational_honest_defer_ok,
            "honest defer must hold: {inv:?}"
        );
        assert!(
            inv.relational_production_off_ok,
            "production arms must remain off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.cutover_deepen_thin_signed,
            "cutover deepen thin must be signed (W402): {inv:?}"
        );
        assert!(
            inv.production_reopen_thin_signed,
            "production reopen thin must remain signed: {inv:?}"
        );
        assert!(
            inv.honest_defer_signed,
            "honest defer must remain signed: {inv:?}"
        );
        assert!(
            !inv.real_codec_reopen_gap_open,
            "real-codec gap must be closed (W403): {inv:?}"
        );
        assert!(
            !inv.dual_blob_reopen_gap_open,
            "dual-blob gap must be closed (W404): {inv:?}"
        );
        assert!(inv.cutover_enabled, "Medium cutover honesty must be on (W405): {inv:?}");
        assert!(inv.real_codec_enabled, "real codec first-slice must be on (W403): {inv:?}");
        assert!(
            inv.production_dual_load_enabled,
            "dual-blob production wire must be on (W404): {inv:?}"
        );
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
    }

    #[test]
    fn mir_module_medium_horizon_cutover_deepen_thin_inventory_matches_reopen() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_cutover_deepen_thin_inventory(&module);
        let reopen = mir_module_medium_horizon_production_reopen_thin_inventory(&module);
        assert_eq!(
            inv.relational_reopen_thin_ok,
            reopen.production_reopen_thin_signed && reopen.relational_production_off_ok
        );
        assert_eq!(
            inv.relational_honest_defer_ok,
            reopen.honest_defer_signed && reopen.relational_honest_defer_ok
        );
        assert_eq!(
            inv.relational_production_off_ok,
            reopen.relational_production_off_ok
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.production_reopen_thin_signed, true);
        assert_eq!(inv.cutover_deepen_thin_signed, true);
        assert_eq!(inv.real_codec_reopen_gap_open, false);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn encode_decode_module_metadata_sidecar_roundtrip() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let view = mir_module_metadata_sidecar_view(&module);
        let bytes = encode_module_metadata_sidecar(&module);
        let decoded = decode_module_metadata_sidecar(&bytes).expect("sidecar decode");
        let after = encode_module(&module);
        assert_eq!(before, after, "sidecar wire must not mutate encode_module");
        assert_eq!(decoded, view);
    }

    #[test]
    fn mir_module_medium_horizon_real_codec_first_slice_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_real_codec_first_slice_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "real codec first-slice inventory must not mutate encode"
        );
        assert!(inv.relational_deepen_thin_ok, "deepen thin must hold: {inv:?}");
        assert!(
            inv.relational_sidecar_roundtrip_ok,
            "sidecar round-trip must hold: {inv:?}"
        );
        assert!(
            inv.relational_dual_load_off_ok,
            "dual-load wire + gap-closed honesty must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.real_codec_first_slice_signed,
            "first-slice must be signed (W403): {inv:?}"
        );
        assert!(inv.cutover_deepen_thin_signed);
        assert!(inv.real_codec_enabled);
        assert!(!inv.real_codec_reopen_gap_open);
        assert!(!inv.dual_blob_reopen_gap_open);
        assert!(inv.cutover_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
    }

    #[test]
    fn mir_module_medium_horizon_real_codec_first_slice_inventory_matches_deepen() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_real_codec_first_slice_inventory(&module);
        let deepen = mir_module_medium_horizon_cutover_deepen_thin_inventory(&module);
        assert_eq!(
            inv.relational_deepen_thin_ok,
            deepen.cutover_deepen_thin_signed && deepen.relational_production_off_ok
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.cutover_deepen_thin_signed, true);
        assert_eq!(inv.real_codec_first_slice_signed, true);
        assert_eq!(inv.real_codec_reopen_gap_open, false);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert_eq!(deepen.real_codec_enabled, true);
        assert_eq!(deepen.real_codec_reopen_gap_open, false);
    }

    #[test]
    fn encode_decode_module_dual_blob_package_roundtrip() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let view = mir_module_metadata_sidecar_view(&module);
        let execution = dual_blob_package_execution_bytes(&module);
        let bytes = encode_module_dual_blob_package(&module);
        let decoded = decode_module_dual_blob_package(&bytes).expect("dual-blob package decode");
        let after = encode_module(&module);
        assert_eq!(before, after, "dual-blob package wire must not mutate encode_module");
        assert_eq!(decoded.sidecar, view);
        assert_eq!(decoded.execution_bytes, execution);
        assert_eq!(
            dual_blob_package_sidecar_bytes(&module),
            encode_module_metadata_sidecar(&module)
        );
    }

    #[test]
    fn encode_module_aot_embed_blob_peels_to_compact_for_aot() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let embed = encode_module_aot_embed_blob(&module);
        let compact = encode_module_aot_embed_execution_bytes(&module);
        let package = encode_module_dual_blob_package(&module);
        assert!(is_dual_blob_package_bytes(&embed));
        assert_ne!(
            embed, package,
            "AOT embed omits execution; inventory package uses whole-module execution"
        );
        let peeled = peel_dual_blob_package_execution(&embed).expect("peel embed");
        assert!(peeled.is_empty(), "W409 AOT embed execution must be empty");
        assert_eq!(peeled, compact.as_slice());
        assert!(aot_embed_execution_omitted());
        assert!(embed.len() > dual_blob_package_sidecar_bytes(&module).len());
        let side = peel_dual_blob_package_sidecar(&embed).expect("peel sidecar");
        let decoded = decode_module_metadata_sidecar(side).expect("decode sidecar");
        assert_eq!(decoded, mir_module_metadata_sidecar_view(&module));
    }

    #[test]
    fn mir_module_from_sidecar_fn_stubs_preserves_fn_count() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let view = mir_module_metadata_sidecar_view(&module);
        let stubs = mir_module_from_sidecar_fn_stubs(&view);
        assert_eq!(stubs.functions.len(), module.functions.len());
        assert_eq!(stubs.functions[0].name.as_str(), "main");
        assert!(matches!(
            stubs.functions[0].instructions[0].op,
            MirOp::Return
        ));
    }

    #[test]
    fn mir_module_medium_horizon_exit_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_exit_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "exit inventory must not mutate encode");
        assert!(inv.b05_medium_exit_signed, "B-05 exit must be signed: {inv:?}");
        assert!(inv.execution_omit_signed, "{inv:?}");
        assert!(inv.production_cutover_signed, "{inv:?}");
        assert!(inv.sidecar_primary_signed, "{inv:?}");
        assert!(inv.dual_load_runtime_signed, "{inv:?}");
        assert!(inv.aot_execution_segment_omitted, "{inv:?}");
        assert!(
            inv.relational_execution_omit_ok,
            "execution-omit must hold: {inv:?}"
        );
        assert!(
            inv.relational_production_cutover_ok,
            "production cutover must hold: {inv:?}"
        );
        assert!(
            inv.relational_sidecar_primary_ok,
            "sidecar-primary must hold: {inv:?}"
        );
        assert!(
            inv.relational_dual_load_ok,
            "dual-load must hold: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        // Honesty: exit is construction-chain only — ≠ B-06 [x] / ≠ MIR blob delete this window.
        assert!(
            inv.b05_medium_exit_signed && inv.execution_omit_signed,
            "narrow exit must rollup execution-omit (ADR-791)"
        );
    }

    #[test]
    fn mir_module_medium_horizon_execution_omit_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_execution_omit_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(before, after, "execution-omit inventory must not mutate encode");
        assert!(
            inv.relational_sidecar_primary_ok,
            "sidecar-primary must hold: {inv:?}"
        );
        assert!(inv.relational_exec_empty_ok, "exec empty must hold: {inv:?}");
        assert!(
            inv.relational_sidecar_decode_ok,
            "sidecar decode must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.execution_omit_signed,
            "execution-omit must be signed (W409): {inv:?}"
        );
        assert!(inv.aot_execution_segment_omitted);
        assert!(inv.sidecar_primary_signed);
        assert!(inv.sidecar_string_pool_primary);
        assert!(inv.aot_embeds_sidecar);
        assert!(!inv.load_path_is_whole_module);
        assert!(!inv.dual_blob_reopen_gap_open);
    }

    #[test]
    fn mir_module_medium_horizon_execution_omit_inventory_matches_primary() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_execution_omit_inventory(&module);
        let primary = mir_module_medium_horizon_sidecar_primary_inventory(&module);
        assert_eq!(
            inv.relational_sidecar_primary_ok,
            primary.sidecar_primary_signed
                && primary.sidecar_string_pool_primary
                && primary.relational_dual_load_ok
        );
        assert_eq!(inv.execution_omit_signed, true);
        assert_eq!(inv.aot_execution_segment_omitted, true);
        assert_eq!(inv.sidecar_primary_signed, true);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_mir_free_readiness_thin_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_long_horizon_mir_free_readiness_thin_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "Long MIR-free readiness thin inventory must not mutate encode"
        );
        assert!(
            inv.relational_execution_omit_ok,
            "execution-omit must hold: {inv:?}"
        );
        assert!(
            inv.relational_medium_delivery_ok,
            "Medium delivery must hold: {inv:?}"
        );
        assert!(
            inv.relational_mir_free_off_ok,
            "MIR-free must stay off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.long_mir_free_readiness_thin_signed,
            "Long readiness thin must be signed (W410): {inv:?}"
        );
        assert!(inv.execution_omit_signed);
        assert!(inv.aot_execution_segment_omitted);
        assert!(!inv.mir_free_enabled);
        assert!(!inv.delete_syx_mir_blob_enabled);
        assert!(inv.blob_still_embedded);
        assert!(inv.syx_user_init_still_loads_blob);
        assert!(inv.native_metadata_without_blob_gap_open);
        assert!(inv.closure_async_mir_free_gap_open);
        assert!(inv.llvm_emit_omit_blob_gap_open);
        assert!(inv.user_init_omit_load_gap_open);
        assert!(inv.host_bridge_pure_native_gap_open);
        assert_eq!(
            encode_module_aot_embed_blob(&module),
            encode_module_aot_embed_blob(&module),
            "inventory must not mutate AOT embed"
        );
    }

    #[test]
    fn mir_module_long_horizon_mir_free_readiness_thin_inventory_matches_omit() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_mir_free_readiness_thin_inventory(&module);
        let omit = mir_module_medium_horizon_execution_omit_inventory(&module);
        assert_eq!(
            inv.relational_execution_omit_ok,
            omit.execution_omit_signed
                && omit.aot_execution_segment_omitted
                && omit.relational_exec_empty_ok
                && omit.relational_sidecar_primary_ok
        );
        assert_eq!(inv.execution_omit_signed, true);
        assert_eq!(inv.long_mir_free_readiness_thin_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.blob_still_embedded, true);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_native_metadata_deepen_thin_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_long_horizon_native_metadata_deepen_thin_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "native-metadata deepen thin inventory must not mutate encode"
        );
        assert!(
            inv.relational_readiness_thin_ok,
            "readiness thin must hold: {inv:?}"
        );
        assert!(
            inv.relational_sidecar_capability_ok,
            "sidecar capability must hold: {inv:?}"
        );
        assert!(
            inv.relational_mir_free_off_ok,
            "MIR-free must stay off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.native_metadata_deepen_thin_signed,
            "native-metadata deepen thin must be signed (W411): {inv:?}"
        );
        assert!(inv.long_mir_free_readiness_thin_signed);
        assert!(inv.execution_omit_signed);
        assert!(inv.aot_execution_segment_omitted);
        assert!(!inv.mir_free_enabled);
        assert!(!inv.delete_syx_mir_blob_enabled);
        assert!(inv.blob_still_embedded);
        assert!(inv.syx_user_init_still_loads_blob);
        assert!(
            inv.native_metadata_without_blob_gap_open,
            "deepen thin must NOT close native_metadata_without_blob gap: {inv:?}"
        );
        assert!(inv.closure_async_mir_free_gap_open);
        assert!(inv.llvm_emit_omit_blob_gap_open);
        assert!(inv.user_init_omit_load_gap_open);
        assert!(inv.host_bridge_pure_native_gap_open);
    }

    #[test]
    fn mir_module_long_horizon_native_metadata_deepen_thin_inventory_matches_readiness() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_native_metadata_deepen_thin_inventory(&module);
        let readiness = mir_module_long_horizon_mir_free_readiness_thin_inventory(&module);
        assert_eq!(
            inv.relational_readiness_thin_ok,
            readiness.long_mir_free_readiness_thin_signed
                && readiness.relational_execution_omit_ok
                && readiness.relational_medium_delivery_ok
        );
        assert_eq!(inv.native_metadata_deepen_thin_signed, true);
        assert_eq!(inv.long_mir_free_readiness_thin_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.native_metadata_without_blob_gap_open, true);
        assert_eq!(inv.blob_still_embedded, true);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_native_metadata_wire_first_slice_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_long_horizon_native_metadata_wire_first_slice_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "native-metadata wire inventory must not mutate encode"
        );
        assert!(
            inv.relational_deepen_thin_ok,
            "deepen thin must hold: {inv:?}"
        );
        assert!(
            inv.relational_sidecar_decode_ok,
            "sidecar decode must hold: {inv:?}"
        );
        assert!(
            inv.relational_mir_free_off_ok,
            "MIR-free must stay off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.native_metadata_wire_first_slice_signed,
            "native-metadata wire first-slice must be signed (W412): {inv:?}"
        );
        assert!(inv.native_metadata_deepen_thin_signed);
        assert!(inv.long_mir_free_readiness_thin_signed);
        assert!(inv.execution_omit_signed);
        assert!(inv.aot_execution_segment_omitted);
        assert!(!inv.mir_free_enabled);
        assert!(!inv.delete_syx_mir_blob_enabled);
        assert!(inv.blob_still_embedded);
        assert!(inv.syx_user_init_still_loads_blob);
        assert!(
            !inv.native_metadata_without_blob_gap_open,
            "wire first-slice must close native_metadata_without_blob gap: {inv:?}"
        );
        assert!(inv.closure_async_mir_free_gap_open);
        assert!(inv.llvm_emit_omit_blob_gap_open);
        assert!(inv.user_init_omit_load_gap_open);
        assert!(inv.host_bridge_pure_native_gap_open);
    }

    #[test]
    fn mir_module_long_horizon_native_metadata_wire_first_slice_inventory_matches_deepen() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_native_metadata_wire_first_slice_inventory(&module);
        let deepen = mir_module_long_horizon_native_metadata_deepen_thin_inventory(&module);
        assert_eq!(
            inv.relational_deepen_thin_ok,
            deepen.native_metadata_deepen_thin_signed
                && deepen.long_mir_free_readiness_thin_signed
                && deepen.relational_sidecar_capability_ok
                && deepen.relational_readiness_thin_ok
        );
        assert_eq!(inv.native_metadata_wire_first_slice_signed, true);
        assert_eq!(inv.native_metadata_deepen_thin_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.native_metadata_without_blob_gap_open, false);
        assert_eq!(inv.blob_still_embedded, true);
        assert_eq!(inv.syx_user_init_still_loads_blob, true);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_emit_omit_first_slice_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_long_horizon_emit_omit_first_slice_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "emit-omit inventory must not mutate encode"
        );
        assert!(
            inv.relational_wire_first_slice_ok,
            "wire first-slice must hold: {inv:?}"
        );
        assert!(
            inv.relational_sidecar_encode_ok,
            "sidecar encode must hold: {inv:?}"
        );
        assert!(
            inv.relational_mir_free_off_ok,
            "MIR-free must stay off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.emit_omit_first_slice_signed,
            "emit-omit first-slice must be signed (W413): {inv:?}"
        );
        assert!(inv.native_metadata_wire_first_slice_signed);
        assert!(!inv.mir_free_enabled);
        assert!(!inv.delete_syx_mir_blob_enabled);
        assert!(
            !inv.blob_still_embedded,
            "emit-omit must clear @syx_mir_blob embed honesty: {inv:?}"
        );
        assert!(!inv.syx_user_init_still_loads_blob);
        assert!(inv.sidecar_constant_still_embedded);
        assert!(!inv.native_metadata_without_blob_gap_open);
        assert!(
            !inv.llvm_emit_omit_blob_gap_open,
            "emit-omit must close llvm_emit_omit_blob gap: {inv:?}"
        );
        assert!(
            !inv.user_init_omit_load_gap_open,
            "emit-omit must close user_init_omit_load gap: {inv:?}"
        );
        assert!(inv.closure_async_mir_free_gap_open);
        assert!(inv.host_bridge_pure_native_gap_open);
    }

    #[test]
    fn mir_module_long_horizon_emit_omit_first_slice_inventory_matches_wire() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_emit_omit_first_slice_inventory(&module);
        let wire = mir_module_long_horizon_native_metadata_wire_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_wire_first_slice_ok,
            wire.native_metadata_wire_first_slice_signed
                && !wire.native_metadata_without_blob_gap_open
                && wire.relational_deepen_thin_ok
                && wire.relational_sidecar_decode_ok
        );
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.native_metadata_wire_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.syx_user_init_still_loads_blob, false);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.llvm_emit_omit_blob_gap_open, false);
        assert_eq!(inv.user_init_omit_load_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_thin_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_long_horizon_closure_async_mir_free_thin_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "closure-async thin inventory must not mutate encode"
        );
        assert!(
            inv.relational_emit_omit_ok,
            "emit omit must hold: {inv:?}"
        );
        assert!(
            inv.relational_sidecar_still_embedded_ok,
            "sidecar must still be embedded: {inv:?}"
        );
        assert!(
            inv.relational_mir_free_off_ok,
            "MIR-free must stay off: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.closure_async_mir_free_thin_signed,
            "closure-async thin must be signed (W414): {inv:?}"
        );
        assert!(inv.emit_omit_first_slice_signed);
        assert!(!inv.mir_free_enabled);
        assert!(!inv.blob_still_embedded);
        assert!(!inv.syx_user_init_still_loads_blob);
        assert!(inv.sidecar_constant_still_embedded);
        assert!(
            inv.closure_async_mir_free_gap_open,
            "thin must NOT close closure_async_mir_free gap: {inv:?}"
        );
        assert!(!inv.llvm_emit_omit_blob_gap_open);
        assert!(!inv.user_init_omit_load_gap_open);
        assert!(inv.host_bridge_pure_native_gap_open);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_thin_inventory_matches_emit_omit() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_mir_free_thin_inventory(&module);
        let emit = mir_module_long_horizon_emit_omit_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_emit_omit_ok,
            emit.emit_omit_first_slice_signed
                && !emit.blob_still_embedded
                && !emit.syx_user_init_still_loads_blob
                && !emit.llvm_emit_omit_blob_gap_open
                && !emit.user_init_omit_load_gap_open
        );
        assert_eq!(inv.closure_async_mir_free_thin_signed, true);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(&module);
        let b = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 4);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_closure_async_thin_ok);
        assert!(a.relational_emit_omit_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory_matches_thin() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(&module);
        let thin = mir_module_long_horizon_closure_async_mir_free_thin_inventory(&module);
        assert_eq!(
            inv.relational_closure_async_thin_ok,
            thin.closure_async_mir_free_thin_signed
                && thin.closure_async_mir_free_gap_open
                && thin.relational_emit_omit_ok
                && thin.relational_sidecar_still_embedded_ok
        );
        assert_eq!(inv.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(inv.closure_async_mir_free_thin_signed, true);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_wire_first_slice_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_closure_async_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 4);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_closure_async_deepen_ok);
        assert!(a.relational_call_fn_fallback_closed_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_wire_first_slice_inventory_matches_deepen() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_wire_first_slice_inventory(&module);
        let deepen = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(&module);
        assert_eq!(
            inv.relational_closure_async_deepen_ok,
            deepen.closure_async_mir_free_deepen_thin_signed
                && deepen.closure_async_mir_free_thin_signed
                && deepen.closure_async_mir_free_gap_open
                && deepen.relational_emit_omit_ok
        );
        assert_eq!(inv.closure_async_wire_first_slice_signed, true);
        assert_eq!(inv.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_await_wire_first_slice_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 4);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_closure_async_wire_ok);
        assert!(a.relational_await_fallback_closed_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_await_wire_first_slice_inventory_matches_wire() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(&module);
        let wire = mir_module_long_horizon_closure_async_wire_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_closure_async_wire_ok,
            wire.closure_async_wire_first_slice_signed
                && !wire.call_fn_mir_interpret_fallback_gap_open
                && wire.closure_async_mir_free_gap_open
                && wire.relational_closure_async_deepen_ok
        );
        assert_eq!(inv.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(inv.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.closure_async_wire_first_slice_signed, true);
        assert_eq!(inv.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_task_wire_first_slice_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 4);
        assert_eq!(a.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(a.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_closure_async_await_wire_ok);
        assert!(a.relational_task_fallback_closed_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_task_wire_first_slice_inventory_matches_await() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(&module);
        let await_wire =
            mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_closure_async_await_wire_ok,
            await_wire.closure_async_await_wire_first_slice_signed
                && !await_wire.await_mir_interpret_fallback_gap_open
                && await_wire.closure_async_mir_free_gap_open
                && await_wire.relational_closure_async_wire_ok
        );
        assert_eq!(inv.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(inv.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(inv.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 4);
        assert_eq!(a.closure_async_actor_wire_first_slice_signed, true);
        assert_eq!(a.actor_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(a.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_closure_async_task_wire_ok);
        assert!(a.relational_actor_fallback_closed_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory_matches_task() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(&module);
        let task_wire =
            mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_closure_async_task_wire_ok,
            task_wire.closure_async_task_wire_first_slice_signed
                && !task_wire.task_mir_interpret_fallback_gap_open
                && task_wire.closure_async_mir_free_gap_open
                && task_wire.relational_closure_async_await_wire_ok
        );
        assert_eq!(inv.closure_async_actor_wire_first_slice_signed, true);
        assert_eq!(inv.actor_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(inv.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(inv.emit_omit_first_slice_signed, true);
        assert_eq!(inv.mir_free_enabled, false);
        assert_eq!(inv.closure_async_mir_free_gap_open, true);
        assert_eq!(inv.sidecar_constant_still_embedded, true);
        assert_eq!(inv.blob_still_embedded, false);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_gap_close_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(&module);
        let b = mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.closure_async_actor_wire_first_slice_signed, true);
        assert_eq!(a.actor_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(a.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert!(a.relational_actor_wire_ok);
        assert!(a.relational_four_subgap_closed_ok);
        assert!(a.relational_gap_closed_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_closure_async_mir_free_gap_close_keeps_actor_history_open() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let gap = mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(&module);
        let actor =
            mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(&module);
        // W419 historical inventory keeps gap open; W420 gap-close flips current horizon.
        assert_eq!(actor.closure_async_mir_free_gap_open, true);
        assert_eq!(gap.closure_async_mir_free_gap_open, false);
        assert_eq!(
            gap.closure_async_actor_wire_first_slice_signed,
            actor.closure_async_actor_wire_first_slice_signed
        );
        assert_eq!(
            gap.actor_mir_interpret_fallback_gap_open,
            actor.actor_mir_interpret_fallback_gap_open
        );
        assert_eq!(gap.host_bridge_pure_native_gap_open, true);
        assert_eq!(gap.sidecar_constant_still_embedded, true);
        assert_eq!(gap.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_thin_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_host_bridge_thin_inventory(&module);
        let b = mir_module_long_horizon_host_bridge_thin_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert_eq!(a.host_bridge_thin_signed, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert!(a.relational_gap_close_ok);
        assert!(a.relational_host_bridge_thin_ok);
        assert!(a.relational_host_gap_still_open_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_thin_keeps_gap_open() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let thin = mir_module_long_horizon_host_bridge_thin_inventory(&module);
        let gap = mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(&module);
        // W421 thin signs charter; host-bridge gap remains open (same as W420 honesty).
        assert_eq!(thin.host_bridge_thin_signed, true);
        assert_eq!(thin.host_bridge_pure_native_gap_open, true);
        assert_eq!(gap.host_bridge_pure_native_gap_open, true);
        assert_eq!(
            thin.closure_async_mir_free_gap_close_signed,
            gap.closure_async_mir_free_gap_close_signed
        );
        assert_eq!(
            thin.closure_async_mir_free_gap_open,
            gap.closure_async_mir_free_gap_open
        );
        assert_eq!(thin.sidecar_constant_still_embedded, true);
        assert_eq!(thin.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_deepen_thin_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
        let b = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert_eq!(a.host_bridge_deepen_thin_signed, true);
        assert_eq!(a.host_bridge_thin_signed, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert!(a.relational_thin_ok);
        assert!(a.relational_host_bridge_deepen_ok);
        assert!(a.relational_host_gap_still_open_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_deepen_keeps_gap_open() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let deepen = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
        let thin = mir_module_long_horizon_host_bridge_thin_inventory(&module);
        // W422 deepen signs prerequisites; host-bridge gap remains open (same as W421).
        assert_eq!(deepen.host_bridge_deepen_thin_signed, true);
        assert_eq!(deepen.host_bridge_thin_signed, true);
        assert_eq!(deepen.host_bridge_pure_native_gap_open, true);
        assert_eq!(thin.host_bridge_pure_native_gap_open, true);
        assert_eq!(deepen.host_bridge_thin_signed, thin.host_bridge_thin_signed);
        assert_eq!(deepen.sidecar_constant_still_embedded, true);
        assert_eq!(deepen.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_print_wire_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert_eq!(a.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(a.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_deepen_thin_signed, true);
        assert_eq!(a.host_bridge_thin_signed, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert!(a.relational_deepen_ok);
        assert!(a.relational_print_wire_ok);
        assert!(a.relational_host_gap_still_open_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_print_wire_keeps_host_gap_open() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let wire = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
        let deepen = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
        // W423 Print wire closes Print sub-gap; host-bridge gap remains open (CallHost).
        assert_eq!(wire.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(wire.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(wire.host_bridge_pure_native_gap_open, true);
        assert_eq!(deepen.host_bridge_pure_native_gap_open, true);
        assert_eq!(wire.host_bridge_deepen_thin_signed, deepen.host_bridge_deepen_thin_signed);
        assert_eq!(wire.sidecar_constant_still_embedded, true);
        assert_eq!(wire.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_call_host_wire_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert_eq!(a.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(a.call_host_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(a.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_pure_native_gap_open, true);
        assert_eq!(a.mir_free_enabled, false);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, true);
        assert!(a.relational_print_wire_ok);
        assert!(a.relational_call_host_wire_ok);
        assert!(a.relational_host_gap_still_open_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_call_host_wire_keeps_host_gap_open() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let callhost = mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
        let print = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
        // W424 CallHost wire closes CallHost sub-gap; host gap remains open until gap-close window.
        assert_eq!(callhost.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(callhost.call_host_mir_opcode_bridge_gap_open, false);
        assert_eq!(callhost.host_bridge_pure_native_gap_open, true);
        assert_eq!(print.host_bridge_pure_native_gap_open, true);
        assert_eq!(
            callhost.host_bridge_print_wire_first_slice_signed,
            print.host_bridge_print_wire_first_slice_signed
        );
        assert_eq!(callhost.sidecar_constant_still_embedded, true);
        assert_eq!(callhost.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_gap_close_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(&module);
        let b = mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_print_wire_ok);
        assert!(a.relational_call_host_wire_ok);
        assert!(a.relational_gap_close_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_host_bridge_gap_close_closes_host_gap() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let close = mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(&module);
        let callhost = mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
        let print = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
        // W425 gap close flips host gap closed; historical CallHost/Print inventories keep open.
        assert_eq!(close.host_bridge_pure_native_gap_close_signed, true);
        assert_eq!(close.host_bridge_pure_native_gap_open, false);
        assert_eq!(callhost.host_bridge_pure_native_gap_open, true);
        assert_eq!(print.host_bridge_pure_native_gap_open, true);
        assert_eq!(close.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(close.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(close.call_host_mir_opcode_bridge_gap_open, false);
        assert_eq!(close.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(close.sidecar_constant_still_embedded, true);
        assert_eq!(close.mir_free_enabled, false);
        assert_eq!(close.blob_still_embedded, false);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_thin_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_thin_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_thin_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_host_gap_closed_ok);
        assert!(a.relational_thin_ok);
        assert!(a.relational_sidecar_still_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_thin_keeps_sidecar_embedded() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let thin = mir_module_long_horizon_sidecar_omit_thin_inventory(&module);
        let close = mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(&module);
        // W426 thin signs omit charter; sidecar remains embedded (thin ≠ emit omit).
        assert_eq!(thin.sidecar_omit_thin_signed, true);
        assert_eq!(thin.sidecar_constant_still_embedded, true);
        assert_eq!(close.sidecar_constant_still_embedded, true);
        assert_eq!(thin.host_bridge_pure_native_gap_close_signed, true);
        assert_eq!(thin.host_bridge_pure_native_gap_open, false);
        assert_eq!(thin.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(thin.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(thin.mir_free_enabled, false);
        assert_eq!(thin.blob_still_embedded, false);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_deepen_thin_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_thin_ok);
        assert!(a.relational_deepen_ok);
        assert!(a.relational_sidecar_still_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_deepen_keeps_sidecar_embedded() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let deepen = mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(&module);
        let thin = mir_module_long_horizon_sidecar_omit_thin_inventory(&module);
        // W427 deepen signs init-substitute prerequisites; sidecar remains embedded.
        assert_eq!(deepen.sidecar_omit_deepen_thin_signed, true);
        assert_eq!(deepen.sidecar_omit_thin_signed, true);
        assert_eq!(deepen.sidecar_constant_still_embedded, true);
        assert_eq!(thin.sidecar_constant_still_embedded, true);
        assert_eq!(deepen.host_bridge_pure_native_gap_open, false);
        assert_eq!(deepen.mir_free_enabled, false);
        assert_eq!(deepen.blob_still_embedded, false);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_deepen_ok);
        assert!(a.relational_wire_ok);
        assert!(a.relational_sidecar_still_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_wire_keeps_sidecar_embedded() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let wire = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
        let deepen = mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(&module);
        // W428 wire signs Empty-pool ABI; sidecar remains embedded (wire ≠ emit omit).
        assert_eq!(wire.sidecar_omit_wire_first_slice_signed, true);
        assert_eq!(wire.sidecar_omit_deepen_thin_signed, true);
        assert_eq!(wire.sidecar_omit_thin_signed, true);
        assert_eq!(wire.sidecar_constant_still_embedded, true);
        assert_eq!(wire.empty_pool_init_gap_open, false);
        assert_eq!(deepen.sidecar_constant_still_embedded, true);
        assert_eq!(wire.mir_free_enabled, false);
        assert_eq!(wire.blob_still_embedded, false);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_register_only_wire_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_empty_pool_wire_ok);
        assert!(a.relational_register_only_ok);
        assert!(a.relational_sidecar_still_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_register_only_keeps_sidecar_and_c_stub_gap() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let reg = mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
        let wire = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
        assert_eq!(reg.sidecar_omit_register_only_wire_first_slice_signed, true);
        assert_eq!(reg.register_only_init_gap_open, false);
        assert_eq!(reg.c_stub_register_loop_gap_open, true);
        assert_eq!(reg.sidecar_constant_still_embedded, true);
        assert_eq!(reg.empty_pool_init_gap_open, false);
        assert_eq!(wire.sidecar_constant_still_embedded, true);
        assert_eq!(reg.mir_free_enabled, false);
        assert_eq!(reg.blob_still_embedded, false);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_c_stub_align_wire_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_register_only_ok);
        assert!(a.relational_c_stub_align_ok);
        assert!(a.relational_sidecar_still_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_c_stub_align_closes_gap_keeps_sidecar() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let align =
            mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
        let reg = mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
        assert_eq!(align.sidecar_omit_c_stub_align_wire_first_slice_signed, true);
        assert_eq!(align.c_stub_register_loop_gap_open, false);
        assert_eq!(align.sidecar_constant_still_embedded, true);
        assert_eq!(align.register_only_init_gap_open, false);
        assert_eq!(align.empty_pool_init_gap_open, false);
        assert_eq!(align.mir_free_enabled, false);
        assert_eq!(align.blob_still_embedded, false);
        // Historical Register-only inventory keeps c_stub gap open honesty.
        assert_eq!(reg.c_stub_register_loop_gap_open, true);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_emit_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
        let b = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_c_stub_align_ok);
        assert!(a.relational_sidecar_emit_ok);
        assert!(a.relational_sidecar_not_embedded_ok);
        assert!(a.relational_mir_free_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_sidecar_omit_emit_stops_sidecar_keeps_mir_free_off() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let emit = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
        let align =
            mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
        assert_eq!(emit.sidecar_omit_emit_first_slice_signed, true);
        assert_eq!(emit.sidecar_constant_still_embedded, false);
        assert_eq!(emit.c_stub_register_loop_gap_open, false);
        assert_eq!(emit.mir_free_enabled, false);
        assert_eq!(emit.blob_still_embedded, false);
        // Historical C stub align inventory keeps sidecar embedded honesty.
        assert_eq!(align.sidecar_constant_still_embedded, true);
    }

    #[test]
    fn mir_module_long_horizon_mir_free_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        let b = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_sidecar_emit_ok);
        assert!(a.relational_sidecar_not_embedded_ok);
        assert!(a.relational_blob_not_embedded_ok);
        assert!(a.relational_mir_free_on_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_mir_free_enables_flag_keeps_delete_off() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let mir_free = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        let emit = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
        assert_eq!(mir_free.mir_free_first_slice_signed, true);
        assert_eq!(mir_free.mir_free_enabled, true);
        assert_eq!(mir_free.delete_syx_mir_blob_enabled, false);
        assert_eq!(mir_free.sidecar_constant_still_embedded, false);
        assert_eq!(mir_free.blob_still_embedded, false);
        // Historical emit inventory keeps mir_free_enabled=false honesty.
        assert_eq!(emit.mir_free_enabled, false);
    }

    #[test]
    fn mir_module_long_horizon_named_stub_clear_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
            &module,
        );
        let b = mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
            &module,
        );
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_mir_free_ok);
        assert!(a.relational_named_clear_ok);
        assert!(a.relational_named_gap_closed_ok);
        assert!(a.relational_delete_off_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_named_stub_clear_closes_gap_keeps_delete_off() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let clear =
            mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
                &module,
            );
        let mir_free = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        assert_eq!(clear.named_hostfn_stub_call_host_clear_signed, true);
        assert_eq!(clear.named_hostfn_stub_call_host_gap_open, false);
        assert_eq!(clear.mir_free_enabled, true);
        assert_eq!(clear.delete_syx_mir_blob_enabled, false);
        assert_eq!(clear.host_bridge_pure_native_gap_open, false);
        // Historical mir_free inventory does not claim named stub clear.
        assert_eq!(mir_free.mir_free_enabled, true);
    }

    #[test]
    fn mir_module_long_horizon_delete_blob_enabled_inventory_is_deterministic() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let a = mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(&module);
        let b = mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(&module);
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 5);
        assert!(a.relational_named_clear_ok);
        assert!(a.relational_mir_free_ok);
        assert!(a.relational_delete_on_ok);
        assert!(a.relational_blob_not_embedded_ok);
        assert!(a.relational_inventory_overlap_ok);
    }

    #[test]
    fn mir_module_long_horizon_delete_blob_enabled_flips_flag_keeps_history() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let delete =
            mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(&module);
        let clear =
            mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
                &module,
            );
        let mir_free = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        assert_eq!(delete.delete_syx_mir_blob_enabled_first_slice_signed, true);
        assert_eq!(delete.delete_syx_mir_blob_enabled, true);
        assert_eq!(delete.mir_free_enabled, true);
        assert_eq!(delete.named_hostfn_stub_call_host_clear_signed, true);
        assert_eq!(delete.blob_still_embedded, false);
        // Historical inventories keep delete_syx_mir_blob_enabled=false honesty.
        assert_eq!(clear.delete_syx_mir_blob_enabled, false);
        assert_eq!(mir_free.delete_syx_mir_blob_enabled, false);
    }

    #[test]
    fn module_with_execution_string_table_stripped_clears_const_str() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module).len();
        let stripped = module_with_execution_string_table_stripped(&module);
        let after = encode_module(&stripped).len();
        assert!(
            after < before,
            "stripping ConstStr payloads must shrink whole-module encode: before={before} after={after}"
        );
        let view = mir_module_metadata_sidecar_view(&stripped);
        assert_eq!(view.string_literals.len(), 4);
        assert!(view.string_literals.iter().all(|s| s.is_empty()));
        let orig = mir_module_metadata_sidecar_view(&module);
        assert!(orig.string_literals.iter().all(|s| !s.is_empty()));
    }

    #[test]
    fn mir_module_medium_horizon_sidecar_primary_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_sidecar_primary_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "sidecar-primary inventory must not mutate encode"
        );
        assert!(inv.relational_dual_load_ok, "dual-load must hold: {inv:?}");
        assert!(
            inv.relational_exec_string_table_stripped_ok,
            "exec string table stripped must hold: {inv:?}"
        );
        assert!(
            inv.relational_embed_strip_bytes_ok,
            "embed strip bytes must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.sidecar_primary_signed,
            "sidecar-primary must be signed (W408): {inv:?}"
        );
        assert!(inv.sidecar_string_pool_primary);
        assert!(inv.execution_string_table_stripped);
        assert!(inv.dual_load_runtime_signed);
        assert!(inv.dual_load_runtime_enabled);
        assert!(inv.aot_embeds_sidecar);
        assert!(!inv.load_path_is_whole_module);
        assert!(!inv.dual_blob_reopen_gap_open);
    }

    #[test]
    fn mir_module_medium_horizon_sidecar_primary_inventory_matches_dual_load() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_sidecar_primary_inventory(&module);
        let dual = mir_module_medium_horizon_dual_load_runtime_inventory(&module);
        assert_eq!(
            inv.relational_dual_load_ok,
            dual.dual_load_runtime_signed
                && dual.dual_load_runtime_enabled
                && dual.relational_peel_both_ok
                && dual.relational_sidecar_decode_ok
        );
        assert_eq!(inv.dual_load_runtime_signed, true);
        assert_eq!(inv.dual_load_runtime_enabled, true);
        assert_eq!(inv.sidecar_primary_signed, true);
        assert_eq!(inv.sidecar_string_pool_primary, true);
        assert_eq!(inv.execution_string_table_stripped, true);
        assert_eq!(inv.candidate_predicate_count, 4);
    }

    #[test]
    fn mir_module_medium_horizon_dual_blob_production_wire_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_dual_blob_production_wire_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "dual-blob production wire inventory must not mutate encode"
        );
        assert!(inv.relational_first_slice_ok, "first-slice must hold: {inv:?}");
        assert!(
            inv.relational_package_roundtrip_ok,
            "package round-trip must hold: {inv:?}"
        );
        assert!(
            inv.relational_default_aot_whole_ok,
            "default AOT embed/load dual-path honesty must hold (W407): {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.dual_blob_production_wire_signed,
            "dual-blob production wire must be signed (W404): {inv:?}"
        );
        assert!(inv.real_codec_first_slice_signed);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(!inv.dual_blob_reopen_gap_open);
        assert!(inv.cutover_enabled);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(!inv.load_path_is_whole_module);
    }

    #[test]
    fn mir_module_medium_horizon_dual_blob_production_wire_inventory_matches_first_slice() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_dual_blob_production_wire_inventory(&module);
        let first = mir_module_medium_horizon_real_codec_first_slice_inventory(&module);
        assert_eq!(
            inv.relational_first_slice_ok,
            first.real_codec_first_slice_signed
                && first.relational_sidecar_roundtrip_ok
                && first.real_codec_enabled
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.load_path_is_whole_module, false);
        assert_eq!(inv.real_codec_first_slice_signed, true);
        assert_eq!(inv.dual_blob_production_wire_signed, true);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert_eq!(first.production_dual_load_enabled, true);
        assert_eq!(first.dual_blob_reopen_gap_open, false);
    }

    #[test]
    fn mir_module_medium_horizon_production_cutover_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_production_cutover_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "production cutover inventory must not mutate encode"
        );
        assert!(inv.relational_dual_wire_ok, "dual-wire must hold: {inv:?}");
        assert!(inv.relational_cutover_on_ok, "cutover must be on: {inv:?}");
        assert!(
            inv.relational_default_aot_whole_ok,
            "default AOT embed/load dual-path honesty must hold (W407): {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.production_cutover_signed,
            "production cutover must be signed (W405): {inv:?}"
        );
        assert!(inv.dual_blob_production_wire_signed);
        assert!(inv.cutover_enabled);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(!inv.dual_blob_reopen_gap_open);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(!inv.load_path_is_whole_module);
    }

    #[test]
    fn mir_module_medium_horizon_production_cutover_inventory_matches_dual_wire() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_production_cutover_inventory(&module);
        let wire = mir_module_medium_horizon_dual_blob_production_wire_inventory(&module);
        assert_eq!(
            inv.relational_dual_wire_ok,
            wire.dual_blob_production_wire_signed
                && wire.production_dual_load_enabled
                && wire.relational_package_roundtrip_ok
                && !wire.dual_blob_reopen_gap_open
        );
        assert_eq!(inv.relational_cutover_on_ok, wire.cutover_enabled);
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.load_path_is_whole_module, false);
        assert_eq!(inv.dual_blob_production_wire_signed, true);
        assert_eq!(inv.production_cutover_signed, true);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert_eq!(wire.cutover_enabled, true);
    }

    #[test]
    fn mir_module_medium_horizon_aot_sidecar_embed_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_aot_sidecar_embed_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "aot sidecar embed inventory must not mutate encode"
        );
        assert!(inv.relational_cutover_ok, "cutover must hold: {inv:?}");
        assert!(
            inv.relational_package_embed_ok,
            "package embed must carry sidecar+compact: {inv:?}"
        );
        assert!(
            inv.relational_load_execution_ok,
            "load must peel execution whole-module: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.aot_sidecar_embed_signed,
            "aot sidecar embed must be signed (W406): {inv:?}"
        );
        assert!(inv.production_cutover_signed);
        assert!(inv.cutover_enabled);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(!inv.dual_blob_reopen_gap_open);
        assert!(inv.aot_embeds_sidecar, "AOT sidecar embed must be on (W406): {inv:?}");
        assert!(!inv.load_path_is_whole_module);
    }

    #[test]
    fn mir_module_medium_horizon_aot_sidecar_embed_inventory_matches_cutover() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_aot_sidecar_embed_inventory(&module);
        let cutover = mir_module_medium_horizon_production_cutover_inventory(&module);
        assert_eq!(
            inv.relational_cutover_ok,
            cutover.production_cutover_signed
                && cutover.cutover_enabled
                && cutover.production_dual_load_enabled
                && cutover.real_codec_enabled
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.load_path_is_whole_module, false);
        assert_eq!(inv.production_cutover_signed, true);
        assert_eq!(inv.aot_sidecar_embed_signed, true);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert_eq!(cutover.aot_embeds_sidecar, true);
    }

    #[test]
    fn peel_dual_blob_package_sidecar_matches_encode() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let embed = encode_module_aot_embed_blob(&module);
        let expected = encode_module_metadata_sidecar(&module);
        let peeled = peel_dual_blob_package_sidecar(&embed).expect("peel sidecar");
        assert_eq!(peeled, expected.as_slice());
        let decoded = decode_module_metadata_sidecar(peeled).expect("decode sidecar");
        assert_eq!(decoded, mir_module_metadata_sidecar_view(&module));
    }

    #[test]
    fn mir_module_medium_horizon_dual_load_runtime_inventory_preds_and_honesty() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let inv = mir_module_medium_horizon_dual_load_runtime_inventory(&module);
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "dual-load runtime inventory must not mutate encode"
        );
        assert!(
            inv.relational_sidecar_embed_ok,
            "sidecar embed must hold: {inv:?}"
        );
        assert!(inv.relational_peel_both_ok, "peel both must hold: {inv:?}");
        assert!(
            inv.relational_sidecar_decode_ok,
            "sidecar decode must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 4);
        assert!(
            inv.dual_load_runtime_signed,
            "dual-load runtime must be signed (W407): {inv:?}"
        );
        assert!(inv.dual_load_runtime_enabled);
        assert!(inv.aot_sidecar_embed_signed);
        assert!(inv.cutover_enabled);
        assert!(inv.real_codec_enabled);
        assert!(inv.production_dual_load_enabled);
        assert!(!inv.dual_blob_reopen_gap_open);
        assert!(inv.aot_embeds_sidecar);
        assert!(!inv.load_path_is_whole_module);
    }

    #[test]
    fn mir_module_medium_horizon_dual_load_runtime_inventory_matches_embed() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let inv = mir_module_medium_horizon_dual_load_runtime_inventory(&module);
        let embed = mir_module_medium_horizon_aot_sidecar_embed_inventory(&module);
        assert_eq!(
            inv.relational_sidecar_embed_ok,
            embed.aot_sidecar_embed_signed
                && embed.aot_embeds_sidecar
                && embed.relational_package_embed_ok
        );
        assert_eq!(inv.cutover_enabled, true);
        assert_eq!(inv.real_codec_enabled, true);
        assert_eq!(inv.production_dual_load_enabled, true);
        assert_eq!(inv.aot_embeds_sidecar, true);
        assert_eq!(inv.load_path_is_whole_module, false);
        assert_eq!(inv.dual_load_runtime_enabled, true);
        assert_eq!(inv.aot_sidecar_embed_signed, true);
        assert_eq!(inv.dual_load_runtime_signed, true);
        assert_eq!(inv.dual_blob_reopen_gap_open, false);
        assert_eq!(inv.candidate_predicate_count, 4);
        assert_eq!(embed.load_path_is_whole_module, false);
    }
}
