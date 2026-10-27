//! MIR opcode tags for `syx.mir_exec` host import and tier-1C direct host imports.

pub use syx_mir_run::op;

pub mod host {
    pub const MODULE: &str = "syx";
    pub const MIR_EXEC: &str = "mir_exec";
    /// `(i32, i32, i32) -> i32` — MIR interpreter dispatch.
    pub const TYPE_MIR_EXEC: u32 = 0;
    /// `() -> i32` — module functions, host imports, and `call_indirect` targets.
    pub const TYPE_FUNC_I32: u32 = 1;
    /// `(i32) -> i32` — tier-1F load global / unary helpers.
    pub const TYPE_I32_TO_I32: u32 = 2;
    /// `(i32) -> ()` — tier-1F store global (side effect only).
    pub const TYPE_I32_VOID: u32 = 3;
    /// `(i32, i32) -> i32` — tier-1G const str pair.
    pub const TYPE_I32_I32_TO_I32: u32 = 4;
    /// `() -> ()` — tier-1H print.
    pub const TYPE_VOID: u32 = 5;
    /// `(i32, i32) -> ()` — Phase 104 SIMD wasm spill/fill guest.
    pub const TYPE_I32_I32_VOID: u32 = 6;
    /// `(i32, i32, i32, i32, i32) -> i32` — Phase 104 i128 const guest.
    pub const TYPE_CONST_NUMERIC128: u32 = 7;
    /// `() -> f64` — Phase 115 typed f64 take from RT stack.
    pub const TYPE_FUNC_F64: u32 = 8;
    /// `(f64) -> ()` — Phase 115 typed f64 push to RT stack.
    pub const TYPE_F64_VOID: u32 = 9;
    /// `() -> f32` — Phase 116 typed f32 take from RT stack.
    pub const TYPE_FUNC_F32: u32 = 10;
    /// `(f32) -> ()` — Phase 116 typed f32 push to RT stack.
    pub const TYPE_F32_VOID: u32 = 11;
    /// `() -> v128` — Phase 117 guest-v10 stack-native pop (RT simd → wasm v128).
    pub const TYPE_FUNC_V128: u32 = 12;
    /// `(i32, v128) -> ()` — Phase 117 guest-v10 stack-native push (element + v128 → RT).
    pub const TYPE_I32_V128_VOID: u32 = 13;
    /// `() -> i64` — Phase 117 typed i64 take from RT stack.
    pub const TYPE_FUNC_I64: u32 = 14;
    /// `(i64) -> ()` — Phase 117 typed i64 push to RT stack.
    pub const TYPE_I64_VOID: u32 = 15;
    pub const IMPORT_MIR_EXEC: u32 = 0;
    /// First module-defined function index (after `mir_exec` + dynamic host imports).
    pub fn module_func_base(import_count: u32) -> u32 {
        import_count
    }
}

/// Phase 66 tier-1F–I: fixed AOT host imports (direct `call`, not `mir_exec`).
pub mod aot {
    pub const LOAD_GLOBAL: &str = "__syx_aot_load_global";
    pub const LOAD_LOCAL: &str = "__syx_aot_load_local";
    pub const STORE_GLOBAL: &str = "__syx_aot_store_global";
    pub const STORE_LOCAL: &str = "__syx_aot_store_local";
    /// Phase 110: push literal i32 onto RT stack (ConstI32 dest None under AOT).
    pub const PUSH_RT_I32: &str = "__syx_aot_push_rt_i32";
    pub const PUSH_RT_BOOL: &str = "__syx_aot_push_rt_bool";
    /// Wave A W203 / ADR-547: ConstNil → RT Nil (Print / void parity).
    pub const PUSH_RT_NIL: &str = "__syx_aot_push_rt_nil";
    pub const PUSH_RT_NUMERIC: &str = "__syx_aot_push_rt_numeric";
    /// Phase 110: pop one RT i32 for guest SIMD shift scalar operand.
    pub const TAKE_RT_I32: &str = "__syx_aot_take_rt_i32";
    /// Phase 115: pop one RT f64 for typed guest SIMD lane pack.
    pub const TAKE_RT_F64: &str = "__syx_aot_take_rt_f64";
    /// Phase 115: push one RT f64 from guest f64 operand.
    pub const PUSH_RT_F64: &str = "__syx_aot_push_rt_f64";
    /// Phase 116: pop one RT f32 for typed guest SIMD lane pack.
    pub const TAKE_RT_F32: &str = "__syx_aot_take_rt_f32";
    /// Phase 116: push one RT f32 from guest f32 operand.
    pub const PUSH_RT_F32: &str = "__syx_aot_push_rt_f32";
    /// Phase 117: pop one RT i64 for typed guest SIMD lane pack.
    pub const TAKE_RT_I64: &str = "__syx_aot_take_rt_i64";
    /// Phase 117: push one RT i64 from guest i64 operand.
    pub const PUSH_RT_I64: &str = "__syx_aot_push_rt_i64";
    /// Phase 117: lane-wise `i16x8` add guest pilot (SIMD8).
    pub const SIMD_I16_ADD: &str = "__syx_aot_simd_i16_add";
    /// Phase 117: lane-wise `i64x2` add guest pilot.
    pub const SIMD_I64_ADD: &str = "__syx_aot_simd_i64_add";
    pub const CONST_STR: &str = "__syx_aot_const_str";
    pub const ARRAY_BUILD: &str = "__syx_aot_array_build";
    pub const DICT_BUILD: &str = "__syx_aot_dict_build";
    pub const SET_BUILD: &str = "__syx_aot_set_build";
    pub const PRINT: &str = "__syx_aot_print";
    /// M0-W4: scalar MIR stack op by opcode (ADD/SUB/LT/…); operands stay on RT stack.
    pub const STACK_OP: &str = "__syx_aot_stack_op";
    pub const MAKE_ITERATOR: &str = "__syx_aot_make_iterator";
    pub const ITERATOR_NEXT: &str = "__syx_aot_iterator_next";
    pub const ARRAY_GET: &str = "__syx_aot_array_get";
    pub const ARRAY_SET: &str = "__syx_aot_array_set";
    pub const OBJECT_GET: &str = "__syx_aot_object_get";
    pub const OBJECT_SET: &str = "__syx_aot_object_set";
    pub const VALUE_EQ: &str = "__syx_aot_value_eq";
    pub const VALUE_NE: &str = "__syx_aot_value_ne";
    pub const REF_EQ: &str = "__syx_aot_ref_eq";
    pub const REF_NE: &str = "__syx_aot_ref_ne";
    pub const CALL_WITNESS: &str = "__syx_aot_call_witness";
    pub const ARRAY_SORTED: &str = "__syx_aot_array_sorted";
    pub const MAKE_CLOSURE: &str = "__syx_aot_make_closure";
    pub const CALL_CLOSURE: &str = "__syx_aot_call_closure";
    pub const AWAIT_POLL: &str = "__syx_aot_await_poll";
    pub const AWAIT_SUSPEND: &str = "__syx_aot_await_suspend";
    pub const DICT_GET: &str = "__syx_aot_dict_get";
    pub const ACTOR_CALL: &str = "__syx_aot_actor_call";
    pub const ACTOR_SEND: &str = "__syx_aot_actor_send";
    pub const ACTOR_REGISTER: &str = "__syx_aot_actor_register";
    pub const TASK_SPAWN: &str = "__syx_aot_task_spawn";
    pub const TASK_JOIN: &str = "__syx_aot_task_join";
    pub const BEGIN_TRY: &str = "__syx_aot_begin_try";
    pub const CATCH: &str = "__syx_aot_catch";
    pub const THROW: &str = "__syx_aot_throw";
    pub const END_TRY: &str = "__syx_aot_end_try";
    pub const SIMD_ADD: &str = "__syx_aot_simd_add";
    pub const SIMD_SUB: &str = "__syx_aot_simd_sub";
    pub const SIMD_MUL: &str = "__syx_aot_simd_mul";
    pub const SIMD_DIV: &str = "__syx_aot_simd_div";
    pub const SIMD_EQ: &str = "__syx_aot_simd_eq";
    pub const SIMD_LT: &str = "__syx_aot_simd_lt";
    pub const SIMD_GT: &str = "__syx_aot_simd_gt";
    pub const SIMD_NE: &str = "__syx_aot_simd_ne";
    pub const SIMD_LE: &str = "__syx_aot_simd_le";
    pub const SIMD_GE: &str = "__syx_aot_simd_ge";
    pub const SIMD_AND: &str = "__syx_aot_simd_and";
    pub const SIMD_OR: &str = "__syx_aot_simd_or";
    pub const SIMD_XOR: &str = "__syx_aot_simd_xor";
    pub const SIMD_NOT: &str = "__syx_aot_simd_not";
    pub const SIMD_SHL: &str = "__syx_aot_simd_shl";
    pub const SIMD_SHR: &str = "__syx_aot_simd_shr";
    pub const SIMD_MIN: &str = "__syx_aot_simd_min";
    pub const SIMD_MAX: &str = "__syx_aot_simd_max";
    pub const SIMD_F32_ADD: &str = "__syx_aot_simd_f32_add";
    pub const SIMD_F32_SUB: &str = "__syx_aot_simd_f32_sub";
    pub const SIMD_F32_MUL: &str = "__syx_aot_simd_f32_mul";
    pub const SIMD_F32_DIV: &str = "__syx_aot_simd_f32_div";
    pub const SIMD_F64_ADD: &str = "__syx_aot_simd_f64_add";
    pub const SIMD_F64_SUB: &str = "__syx_aot_simd_f64_sub";
    pub const SIMD_F64_MUL: &str = "__syx_aot_simd_f64_mul";
    pub const SIMD_F64_DIV: &str = "__syx_aot_simd_f64_div";
    pub const SIMD_LOAD: &str = "__syx_aot_simd_load";
    pub const SIMD_STORE: &str = "__syx_aot_simd_store";
    pub const SIMD_SHUFFLE: &str = "__syx_aot_simd_shuffle";
    /// Phase 104: SIMD stack ↔ linear memory (no mir_exec spill/fill).
    pub const SIMD_WASM_SPILL: &str = "__syx_aot_simd_wasm_spill";
    pub const SIMD_WASM_FILL: &str = "__syx_aot_simd_wasm_fill";
    /// Phase 117 guest-v10: push wasm v128 onto RT simd stack (stack-native; no scratch ABI).
    pub const SIMD_GUEST_PUSH_V128: &str = "__syx_aot_simd_guest_push_v128";
    /// Phase 117 guest-v10: pop RT simd as wasm v128 (stack-native; no scratch ABI).
    pub const SIMD_GUEST_POP_V128: &str = "__syx_aot_simd_guest_pop_v128";
    /// Phase 104: ConstNumeric128 guest (no mir_exec SET_CONST128_*).
    pub const CONST_NUMERIC128: &str = "__syx_aot_const_numeric128";
    pub const HEAP_COLLECT: &str = "__syx_aot_heap_collect";
    pub const HEAP_COMPACT: &str = "__syx_aot_heap_compact";
    pub const EXEC_OP: &str = "__syx_aot_exec_op";
    /// Phase 97: tier-1 string concat (eliminates sync hot-path ExecOp).
    pub const CONCAT_STR: &str = "__syx_aot_concat_str";
    /// Phase 97: tier-1 to-string (eliminates sync hot-path ExecOp).
    pub const TO_STR: &str = "__syx_aot_to_str";
    /// Phase 101: function prologue / return value guest imports (off mir_exec allowlist).
    pub const BEGIN_FN: &str = "__syx_aot_begin_fn";
    pub const END_FN: &str = "__syx_aot_end_fn";
    pub const GET_RETURN: &str = "__syx_aot_get_return";
    /// Phase 102: user-fn / host calls off mir_exec allowlist.
    pub const CALL_FN: &str = "__syx_aot_call_fn";
    pub const CALL_HOST: &str = "__syx_aot_call_host";
    /// Wave A W18 / ADR-362: after guest `call_indirect`, poll pending catch (passthrough i32).
    pub const TAKE_CATCH_PC: &str = "__syx_aot_take_catch_pc";
    /// Phase 103: function index prologue off mir_exec allowlist.
    pub const SET_FN_INDEX: &str = "__syx_aot_set_fn_index";
    /// Phase 91: MIR fallback counted as tier-2c, not `mir_exec` histogram.
    pub const RUN_OP: &str = "__syx_aot_run_op";

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Import {
        LoadGlobal,
        LoadLocal,
        StoreGlobal,
        StoreLocal,
        PushRtI32,
        PushRtBool,
        PushRtNil,
        PushRtNumeric,
        TakeRtI32,
        TakeRtF64,
        PushRtF64,
        TakeRtF32,
        PushRtF32,
        TakeRtI64,
        PushRtI64,
        ConstStr,
        ArrayBuild,
        DictBuild,
        SetBuild,
        Print,
        StackOp,
        MakeIterator,
        IteratorNext,
        ArrayGet,
        ArraySet,
        ObjectGet,
        ObjectSet,
        ValueEq,
        ValueNe,
        RefEq,
        RefNe,
        CallWitness,
        ArraySorted,
        MakeClosure,
        CallClosure,
        AwaitPoll,
        AwaitSuspend,
        DictGet,
        ActorCall,
        ActorSend,
        ActorRegister,
        TaskSpawn,
        TaskJoin,
        BeginTry,
        Catch,
        Throw,
        EndTry,
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
        SimdShl,
        SimdShr,
        SimdMin,
        SimdMax,
        SimdF32Add,
        SimdF32Sub,
        SimdF32Mul,
        SimdF32Div,
        SimdF64Add,
        SimdF64Sub,
        SimdF64Mul,
        SimdF64Div,
        SimdI16Add,
        SimdI64Add,
        SimdLoad,
        SimdStore,
        SimdShuffle,
        SimdWasmSpill,
        SimdWasmFill,
        SimdGuestPushV128,
        SimdGuestPopV128,
        ConstNumeric128,
        HeapCollect,
        HeapCompact,
        ExecOp,
        ConcatStr,
        ToStr,
        BeginFn,
        EndFn,
        GetReturn,
        CallFn,
        CallHost,
        TakeCatchPc,
        SetFnIndex,
    }

    impl Import {
        pub fn name(self) -> &'static str {
            match self {
                Self::LoadGlobal => LOAD_GLOBAL,
                Self::LoadLocal => LOAD_LOCAL,
                Self::StoreGlobal => STORE_GLOBAL,
                Self::StoreLocal => STORE_LOCAL,
                Self::PushRtI32 => PUSH_RT_I32,
                Self::PushRtBool => PUSH_RT_BOOL,
                Self::PushRtNil => PUSH_RT_NIL,
                Self::PushRtNumeric => PUSH_RT_NUMERIC,
                Self::TakeRtI32 => TAKE_RT_I32,
                Self::TakeRtF64 => TAKE_RT_F64,
                Self::PushRtF64 => PUSH_RT_F64,
                Self::TakeRtF32 => TAKE_RT_F32,
                Self::PushRtF32 => PUSH_RT_F32,
                Self::TakeRtI64 => TAKE_RT_I64,
                Self::PushRtI64 => PUSH_RT_I64,
                Self::ConstStr => CONST_STR,
                Self::ArrayBuild => ARRAY_BUILD,
                Self::DictBuild => DICT_BUILD,
                Self::SetBuild => SET_BUILD,
                Self::Print => PRINT,
                Self::StackOp => STACK_OP,
                Self::MakeIterator => MAKE_ITERATOR,
                Self::IteratorNext => ITERATOR_NEXT,
                Self::ArrayGet => ARRAY_GET,
                Self::ArraySet => ARRAY_SET,
                Self::ObjectGet => OBJECT_GET,
                Self::ObjectSet => OBJECT_SET,
                Self::ValueEq => VALUE_EQ,
                Self::ValueNe => VALUE_NE,
                Self::RefEq => REF_EQ,
                Self::RefNe => REF_NE,
                Self::CallWitness => CALL_WITNESS,
                Self::ArraySorted => ARRAY_SORTED,
                Self::MakeClosure => MAKE_CLOSURE,
                Self::CallClosure => CALL_CLOSURE,
                Self::AwaitPoll => AWAIT_POLL,
                Self::AwaitSuspend => AWAIT_SUSPEND,
                Self::DictGet => DICT_GET,
                Self::ActorCall => ACTOR_CALL,
                Self::ActorSend => ACTOR_SEND,
                Self::ActorRegister => ACTOR_REGISTER,
                Self::TaskSpawn => TASK_SPAWN,
                Self::TaskJoin => TASK_JOIN,
                Self::BeginTry => BEGIN_TRY,
                Self::Catch => CATCH,
                Self::Throw => THROW,
                Self::EndTry => END_TRY,
                Self::SimdAdd => SIMD_ADD,
                Self::SimdSub => SIMD_SUB,
                Self::SimdMul => SIMD_MUL,
                Self::SimdDiv => SIMD_DIV,
                Self::SimdEq => SIMD_EQ,
                Self::SimdLt => SIMD_LT,
                Self::SimdGt => SIMD_GT,
                Self::SimdNe => SIMD_NE,
                Self::SimdLe => SIMD_LE,
                Self::SimdGe => SIMD_GE,
                Self::SimdAnd => SIMD_AND,
                Self::SimdOr => SIMD_OR,
                Self::SimdXor => SIMD_XOR,
                Self::SimdNot => SIMD_NOT,
                Self::SimdShl => SIMD_SHL,
                Self::SimdShr => SIMD_SHR,
                Self::SimdMin => SIMD_MIN,
                Self::SimdMax => SIMD_MAX,
                Self::SimdF32Add => SIMD_F32_ADD,
                Self::SimdF32Sub => SIMD_F32_SUB,
                Self::SimdF32Mul => SIMD_F32_MUL,
                Self::SimdF32Div => SIMD_F32_DIV,
                Self::SimdF64Add => SIMD_F64_ADD,
                Self::SimdF64Sub => SIMD_F64_SUB,
                Self::SimdF64Mul => SIMD_F64_MUL,
                Self::SimdF64Div => SIMD_F64_DIV,
                Self::SimdI16Add => SIMD_I16_ADD,
                Self::SimdI64Add => SIMD_I64_ADD,
                Self::SimdLoad => SIMD_LOAD,
                Self::SimdStore => SIMD_STORE,
                Self::SimdShuffle => SIMD_SHUFFLE,
                Self::SimdWasmSpill => SIMD_WASM_SPILL,
                Self::SimdWasmFill => SIMD_WASM_FILL,
                Self::SimdGuestPushV128 => SIMD_GUEST_PUSH_V128,
                Self::SimdGuestPopV128 => SIMD_GUEST_POP_V128,
                Self::ConstNumeric128 => CONST_NUMERIC128,
                Self::HeapCollect => HEAP_COLLECT,
                Self::HeapCompact => HEAP_COMPACT,
                Self::ExecOp => EXEC_OP,
                Self::ConcatStr => CONCAT_STR,
                Self::ToStr => TO_STR,
                Self::BeginFn => BEGIN_FN,
                Self::EndFn => END_FN,
                Self::GetReturn => GET_RETURN,
                Self::CallFn => CALL_FN,
                Self::CallHost => CALL_HOST,
                Self::TakeCatchPc => TAKE_CATCH_PC,
                Self::SetFnIndex => SET_FN_INDEX,
            }
        }

        pub fn type_index(self) -> u32 {
            use super::host;
            match self {
                Self::ExecOp => host::TYPE_MIR_EXEC,
                Self::ToStr => host::TYPE_I32_TO_I32,
                // Phase 101: GetReturn is () -> i32 (matches wasm-run linker; no dummy arg).
                Self::GetReturn => host::TYPE_FUNC_I32,
                Self::TakeRtI32 => host::TYPE_FUNC_I32,
                Self::TakeRtF64 => host::TYPE_FUNC_F64,
                Self::PushRtF64 => host::TYPE_F64_VOID,
                Self::TakeRtF32 => host::TYPE_FUNC_F32,
                Self::PushRtF32 => host::TYPE_F32_VOID,
                Self::TakeRtI64 => host::TYPE_FUNC_I64,
                Self::PushRtI64 => host::TYPE_I64_VOID,
                // Phase 101: ConcatStr consumes two guest i32 placeholders; real operands are
                // RT stack Values in MIR order [left, right] (pop right then left).
                Self::ConcatStr => host::TYPE_I32_I32_TO_I32,
                Self::LoadGlobal
                | Self::ArrayBuild
                | Self::DictBuild
                | Self::SetBuild
                | Self::CallHost
                | Self::TakeCatchPc => host::TYPE_I32_TO_I32,
                Self::LoadLocal | Self::StoreLocal | Self::SetFnIndex | Self::PushRtI32 | Self::PushRtBool => {
                    host::TYPE_I32_VOID
                }
                Self::MakeIterator
                | Self::IteratorNext
                | Self::ArrayGet
                | Self::ArraySet
                | Self::ObjectGet
                | Self::ObjectSet
                | Self::ValueEq
                | Self::ValueNe
                | Self::RefEq
                | Self::RefNe
                | Self::ArraySorted
                | Self::CallClosure
                | Self::AwaitPoll
                | Self::DictGet
                | Self::ActorRegister
                | Self::TaskJoin
                | Self::Catch
                | Self::Throw
                | Self::EndTry
                | Self::SimdAdd
                | Self::SimdSub
                | Self::SimdMul
                | Self::SimdDiv
                | Self::SimdEq
                | Self::SimdLt
                | Self::SimdGt
                | Self::SimdNe
                | Self::SimdLe
                | Self::SimdGe
                | Self::SimdAnd
                | Self::SimdOr
                | Self::SimdXor
                | Self::SimdNot
                | Self::SimdShl
                | Self::SimdShr
                | Self::SimdMin
                | Self::SimdMax
                | Self::SimdF32Add
                | Self::SimdF32Sub
                | Self::SimdF32Mul
                | Self::SimdF32Div
                | Self::SimdF64Add
                | Self::SimdF64Sub
                | Self::SimdF64Mul
                | Self::SimdF64Div
                | Self::SimdI16Add
                | Self::SimdI64Add => host::TYPE_FUNC_I32,
                Self::SimdLoad | Self::SimdStore | Self::SimdShuffle => host::TYPE_I32_TO_I32,
                Self::PushRtNumeric | Self::SimdWasmSpill | Self::SimdWasmFill => {
                    host::TYPE_I32_I32_VOID
                }
                Self::SimdGuestPopV128 => host::TYPE_FUNC_V128,
                Self::SimdGuestPushV128 => host::TYPE_I32_V128_VOID,
                Self::ConstNumeric128 => host::TYPE_CONST_NUMERIC128,
                Self::HeapCollect | Self::HeapCompact
                | Self::EndFn | Self::PushRtNil => host::TYPE_VOID,
                Self::BeginFn
                | Self::MakeClosure
                | Self::AwaitSuspend
                | Self::BeginTry
                | Self::ActorCall
                | Self::ActorSend
                | Self::TaskSpawn
                | Self::CallFn => host::TYPE_I32_I32_TO_I32,
                Self::CallWitness => host::TYPE_I32_I32_TO_I32,
                Self::StoreGlobal => host::TYPE_I32_VOID,
                Self::ConstStr => host::TYPE_I32_I32_TO_I32, // returns discarded i32; RT owns Value
                Self::Print => host::TYPE_I32_TO_I32,
                Self::StackOp => host::TYPE_I32_VOID,
            }
        }
    }
}

/// Phase 104 Track B cache / doctor fingerprint strings.
pub const PHASE104_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd-guest-v1:no-mir-exec-simd",
    "wasm-i128-guest-v1:no-mir-exec-const128",
    "wasm-allowlist-empty-v1:phase104",
];

/// Phase 105 Track B: full AOT SIMD guest ops; no non-AOT mir_exec catch-all.
pub const PHASE105_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd-full-v1:aot-ops-complete",
    "wasm-no-mir-exec-catchall-v1:phase105",
];

/// Phase 106 Track B: SimdLt/Shuffle AOT guest; GA forbids MIR_EXEC_ONLY / tier3-off zero gates.
pub const PHASE106_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd-lt-shuffle-v1:aot-complete",
    "wasm-ga-no-mir-exec-env-v1:phase106",
];

/// Phase 107 Track B: true simd128 Div/Eq/Lt/Shuffle + LLVM declare↔call hygiene.
pub const PHASE107_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-expand-v1:div-eq-lt-shuffle",
    "llvm-declare-call-v1:rt-hygiene",
];

/// Phase 108 Track B: simd128 Gt + bitwise And/Or/Xor.
pub const PHASE108_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-bitwise-v1:gt-and-or-xor",
];

/// Phase 109 Track B: simd128 Div/Load/Store guest + Ne/Le/Ge/Not/Shl.
pub const PHASE109_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v2:div-load-store-ext",
];

/// Phase 110 Track B: spill/fill thin + SimdShr; honest Div; guest-primary Load/Store.
pub const PHASE110_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v3:spill-thin-shr",
    "wasm-simd128-div-assembled-v1:honest",
];

/// Phase 111 Track B: thin arith spill + Min/Max + f32x4 Add guest pilot.
pub const PHASE111_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v4:thin-arith-minmax-f32",
];

/// Phase 112 Track B: f32 Sub/Mul + guest-primary Load/Store + Div hard gate.
pub const PHASE112_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v5:f32-ops-guest-ls",
];

/// Phase 113 Track B: f32 Div + guest LS without -4/-5 spill + f64x2 Add.
pub const PHASE113_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v6:f32div-guest-ls-f64",
];

/// Phase 114 Track B: f64 reduce hard gate + guest LS without spill0/fill + f64x2 Sub.
pub const PHASE114_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v7:f64reduce-guest-ls-f64sub",
];

/// Phase 115 Track B: spill-free binops + f64 Mul/Div + guest reduce + typed f64 LS.
pub const PHASE115_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v8:nospill-binop-f64muldiv-reduce",
];

/// Phase 116 Track B: spill-free unary/shift/shuffle + typed f32 LS + i32 reduce hard gate.
pub const PHASE116_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v9:nospill-unary-shift-shuffle-f32ls",
];

/// Phase 117 Track B: stack-native guest-v10 + SIMD8 i16x8 + i64x2 pilots.
pub const PHASE117_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v10:stack-native-simd8-i64",
];

/// Phase 118 Track B: guest-v11 zero-scratch + SIMD16 i8x16 + SIMD8 mul pilots.
pub const PHASE118_TRACK_B_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 119 Track C: SIMD16 i8x16 matrix expand (Mul) + inherited guest-v11 zero-scratch.
pub const PHASE119_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 120 Track C: SIMD16 i8x16 logic/shift (And) + inherited guest-v11 / matrix-expand.
pub const PHASE120_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 121 Track C: SIMD16 i8x16 shift lane (Shl/Shr) + inherited guest-v11 / matrix-expand / logic-or-shift.
pub const PHASE121_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 122 Track C: SIMD16 i8x16 logic or/xor + inherited guest-v4 shift / guest-v11 / matrix-expand.
pub const PHASE122_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 123 Track C: SIMD16 i8x16 logic not + inherited guest-v5 or/xor / guest-v4 shift / guest-v11 / matrix-expand.
pub const PHASE123_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 124 Track C: SIMD16 i8x16 lane min/max + inherited guest-v6 not / guest-v5 or/xor / guest-v4 shift / guest-v11 / matrix-expand.
pub const PHASE124_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 125 Track C: SIMD16 i8x16 lane extract/replace + inherited guest-v7 min/max / guest-v6 not / guest-v5 or/xor / guest-v4 shift / guest-v11 / matrix-expand.
pub const PHASE125_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 126 Track C: SIMD16 i8x16 compare-lane MVP + inherited Phase 125 guest-v8 extract/replace / guest-v7 min/max / guest-v11.
pub const PHASE126_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 127 Track C: SIMD16 i8x16 saturating add/sub MVP + inherited Phase 126 guest-v9 compare / guest-v8 extract/replace.
pub const PHASE127_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 128 Track C: SIMD16 i8x16 shuffle MVP + inherited Phase 127 guest-v10 sat / guest-v9 compare.
pub const PHASE128_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 129 Track C: SIMD16 i8x16 load/store MVP + inherited Phase 128 guest-v11 shuffle.
pub const PHASE129_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 130 Track C: SIMD16 i8x16 unsigned min/max MVP + inherited Phase 129 guest-v12 load/store.
pub const PHASE130_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 131 Track C: SIMD16 i8x16 abs MVP + inherited Phase 130 guest-v13 unsigned min/max.
pub const PHASE131_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 136 Track C: SIMD16 i8x16 avgr_u MVP + inherited Phase 135 guest-v18 ge.
pub const PHASE136_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v19:avgr-u-mvp",
    "wasm-simd16-guest-v18:ge-mvp",
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 139 Track C: SIMD16 i8x16 all_true / any_true MVP + inherited Phase 138 guest-v21 bitmask.
pub const PHASE139_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v22:all-any-true-mvp",
    "wasm-simd16-guest-v21:bitmask-mvp",
    "wasm-simd16-guest-v20:popcnt-mvp",
    "wasm-simd16-guest-v19:avgr-u-mvp",
    "wasm-simd16-guest-v18:ge-mvp",
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 138 Track C: SIMD16 i8x16 bitmask MVP + inherited Phase 137 guest-v20 popcnt.
pub const PHASE138_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v21:bitmask-mvp",
    "wasm-simd16-guest-v20:popcnt-mvp",
    "wasm-simd16-guest-v19:avgr-u-mvp",
    "wasm-simd16-guest-v18:ge-mvp",
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 137 Track C: SIMD16 i8x16 popcnt MVP + inherited Phase 136 guest-v19 avgr_u.
pub const PHASE137_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v20:popcnt-mvp",
    "wasm-simd16-guest-v19:avgr-u-mvp",
    "wasm-simd16-guest-v18:ge-mvp",
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 135 Track C: SIMD16 i8x16 ge MVP + inherited Phase 134 guest-v17 le.
pub const PHASE135_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v18:ge-mvp",
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 134 Track C: SIMD16 i8x16 le MVP + inherited Phase 133 guest-v16 ne.
pub const PHASE134_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v17:le-mvp",
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 133 Track C: SIMD16 i8x16 ne MVP + inherited Phase 132 guest-v15 neg.
pub const PHASE133_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v16:ne-mvp",
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];

/// Phase 132 Track C: SIMD16 i8x16 neg MVP + inherited Phase 131 guest-v14 abs.
pub const PHASE132_TRACK_C_WASM_FINGERPRINTS: &[&str] = &[
    "wasm-simd16-guest-v15:neg-mvp",
    "wasm-simd16-guest-v14:abs-mvp",
    "wasm-simd16-guest-v13:unsigned-min-max-mvp",
    "wasm-simd16-guest-v12:load-store-mvp",
    "wasm-simd16-guest-v11:shuffle-mvp",
    "wasm-simd16-guest-v10:saturating-add-sub-mvp",
    "wasm-simd16-guest-v9:compare-lane-mvp",
    "wasm-simd16-guest-v8:lane-extract-replace",
    "wasm-simd16-guest-v7:lane-min-max",
    "wasm-simd16-guest-v6:logic-not",
    "wasm-simd16-guest-v5:logic-or-xor",
    "wasm-simd16-guest-v4:shift-lane-mvp",
    "wasm-simd16-guest-v3:logic-or-shift",
    "wasm-simd16-guest-v2:matrix-expand",
    "wasm-simd128-guest-v11:zero-scratch-simd16",
];