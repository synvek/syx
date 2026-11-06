//! SYX MIR IR data surface — ops, instructions, functions, modules.
//! Extracted in S2-I166 (zero semantics; re-exported from crate root).

use smol_str::SmolStr;

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
