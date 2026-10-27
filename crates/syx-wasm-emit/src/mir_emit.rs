use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

use syx_diagnostics::{SyxDiagnostic, SyxResult};

thread_local! {
    static AOT_EMIT_HARD_FAIL: RefCell<Option<String>> = RefCell::new(None);
}

fn aot_emit_hard_fail_reset() {
    AOT_EMIT_HARD_FAIL.with(|c| *c.borrow_mut() = None);
}

fn aot_emit_hard_fail(msg: &str) {
    AOT_EMIT_HARD_FAIL.with(|c| {
        let mut g = c.borrow_mut();
        if g.is_none() {
            *g = Some(msg.to_string());
        }
    });
}

fn aot_emit_hard_fail_take() -> Option<String> {
    AOT_EMIT_HARD_FAIL.with(|c| c.borrow_mut().take())
}
use syx_mir::{MirFunction, MirModule, MirOp};
use syx_stdlib::host_fn_name_for_index;
use wasm_encoder::{
    CodeSection, ConstExpr, DataSection, ElementSection, Elements, EntityType, ExportKind,
    ExportSection, Function, FunctionSection, GlobalSection, GlobalType, ImportSection,
    Instruction as WasmInstr, MemorySection, MemoryType, Module, RefType, TableSection,
    TableType, TypeSection, ValType,
};

use crate::exec::{aot, host, op};

struct EmitPlan {
    aot: bool,
    import_count: u32,
    aot_import_map: HashMap<aot::Import, u32>,
    host_import_map: HashMap<u32, u32>,
}

impl EmitPlan {
    fn module_func_index(&self, fn_idx: usize) -> u32 {
        host::module_func_base(self.import_count) + fn_idx as u32
    }

    fn aot_import(&self, kind: aot::Import) -> u32 {
        *self.aot_import_map.get(&kind).expect("aot import registered")
    }
}

fn collect_used_aot_imports(module: &MirModule) -> BTreeSet<aot::Import> {
    let mut used = BTreeSet::new();
    for func in &module.functions {
        for block in &func.blocks {
            for instr in &block.instructions {
                match &instr.op {
                    MirOp::Call(_) => {
                        used.insert(aot::Import::CallFn);
                        // Wave A W18: callee CallHost may leave pending_catch for caller.
                        used.insert(aot::Import::TakeCatchPc);
                    }
                    MirOp::CallHost(_) => {
                        used.insert(aot::Import::CallHost);
                    }
                    MirOp::LoadGlobal(_) => {
                        used.insert(aot::Import::LoadGlobal);
                    }
                    MirOp::LoadLocal(_) => {
                        used.insert(aot::Import::LoadLocal);
                    }
                    MirOp::StoreGlobal(_) => {
                        used.insert(aot::Import::StoreGlobal);
                    }
                    MirOp::StoreLocal(_) => {
                        used.insert(aot::Import::StoreLocal);
                    }
                    MirOp::ConstI32(_) => {
                        used.insert(aot::Import::PushRtI32);
                    }
                    MirOp::ConstBool(_) => {
                        used.insert(aot::Import::PushRtBool);
                    }
                    MirOp::ConstNil => {
                        used.insert(aot::Import::PushRtNil);
                    }
                    MirOp::ConstFnIndex(_) => {
                        used.insert(aot::Import::PushRtI32);
                    }
                    MirOp::ConstNumeric { kind, .. } => {
                        if const_numeric_needs_128_emit(*kind) {
                            used.insert(aot::Import::ConstNumeric128);
                        } else {
                            used.insert(aot::Import::PushRtNumeric);
                        }
                    }
                    MirOp::ConstStr(_) => {
                        used.insert(aot::Import::ConstStr);
                    }
                    MirOp::ArrayBuild(n) if *n <= 16 => {
                        used.insert(aot::Import::ArrayBuild);
                    }
                    MirOp::DictBuild(_) => {
                        used.insert(aot::Import::DictBuild);
                    }
                    MirOp::SetBuild(_) => {
                        used.insert(aot::Import::SetBuild);
                    }
                    MirOp::Print => {
                        used.insert(aot::Import::Print);
                    }
                    MirOp::Eq
                    | MirOp::Ne
                    | MirOp::Lt { .. }
                    | MirOp::Le { .. }
                    | MirOp::Gt { .. }
                    | MirOp::Ge { .. }
                    | MirOp::And
                    | MirOp::Or
                    | MirOp::Not => {
                        used.insert(aot::Import::StackOp);
                        used.insert(aot::Import::TakeRtI32);
                    }
                    MirOp::Add { .. }
                    | MirOp::Sub { .. }
                    | MirOp::Mul { .. }
                    | MirOp::Div { .. }
                    | MirOp::Mod { .. }
                    | MirOp::BitAnd { .. }
                    | MirOp::BitOr { .. }
                    | MirOp::BitXor { .. }
                    | MirOp::Shl { .. }
                    | MirOp::Shr { .. }
                    | MirOp::Neg { .. }
                    | MirOp::BitNot { .. } => {
                        used.insert(aot::Import::StackOp);
                    }
                    MirOp::MakeIterator => {
                        used.insert(aot::Import::MakeIterator);
                    }
                    MirOp::IteratorNext => {
                        used.insert(aot::Import::IteratorNext);
                        used.insert(aot::Import::TakeRtI32);
                    }
                    MirOp::ArrayGet => {
                        used.insert(aot::Import::ArrayGet);
                    }
                    MirOp::ArraySet => {
                        used.insert(aot::Import::ArraySet);
                    }
                    MirOp::ObjectGet => {
                        used.insert(aot::Import::ObjectGet);
                    }
                    MirOp::ObjectSet => {
                        used.insert(aot::Import::ObjectSet);
                    }
                    MirOp::ValueEq | MirOp::OpaqueValueEq => {
                        used.insert(aot::Import::ValueEq);
                    }
                    MirOp::ValueNe => {
                        used.insert(aot::Import::ValueNe);
                    }
                    MirOp::RefEq => {
                        used.insert(aot::Import::RefEq);
                    }
                    MirOp::RefNe => {
                        used.insert(aot::Import::RefNe);
                    }
                    MirOp::CallWitness(_, _) => {
                        used.insert(aot::Import::CallWitness);
                    }
                    MirOp::ArrSorted => {
                        used.insert(aot::Import::ArraySorted);
                    }
                    MirOp::MakeClosure(_, _) => {
                        used.insert(aot::Import::MakeClosure);
                    }
                    MirOp::CallClosure => {
                        used.insert(aot::Import::CallClosure);
                    }
                    MirOp::AwaitPoll => {
                        used.insert(aot::Import::AwaitPoll);
                    }
                    MirOp::AwaitSuspend { .. } => {
                        used.insert(aot::Import::AwaitSuspend);
                    }
                    MirOp::ActorCall(_) => {
                        used.insert(aot::Import::ActorCall);
                        // ADR-631: C009 under try → pending_catch → TakeCatchPc.
                        used.insert(aot::Import::TakeCatchPc);
                    }
                    MirOp::ActorSend(_) => {
                        used.insert(aot::Import::ActorSend);
                        used.insert(aot::Import::TakeCatchPc);
                    }
                    MirOp::ActorRegister => {
                        used.insert(aot::Import::ActorRegister);
                    }
                    MirOp::TaskSpawn(_) => {
                        used.insert(aot::Import::TaskSpawn);
                    }
                    MirOp::TaskJoin => {
                        used.insert(aot::Import::TaskJoin);
                    }
                    MirOp::BeginTry(_) => {
                        used.insert(aot::Import::BeginTry);
                    }
                    MirOp::Catch => {
                        used.insert(aot::Import::Catch);
                    }
                    MirOp::Throw => {
                        used.insert(aot::Import::Throw);
                    }
                    MirOp::EndTry => {
                        used.insert(aot::Import::EndTry);
                    }
                    MirOp::SimdAdd => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdAdd),
                    MirOp::SimdSub => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdSub),
                    MirOp::SimdMul => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdMul),
                    MirOp::SimdDiv => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdDiv),
                    MirOp::SimdEq => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdEq),
                    MirOp::SimdLt => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdLt),
                    MirOp::SimdGt => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdGt),
                    MirOp::SimdNe => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdNe),
                    MirOp::SimdLe => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdLe),
                    MirOp::SimdGe => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdGe),
                    MirOp::SimdAnd => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdAnd),
                    MirOp::SimdOr => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdOr),
                    MirOp::SimdXor => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdXor),
                    MirOp::SimdNot => collect_guest_v9_unary_imports(&mut used, aot::Import::SimdNot, false),
                    MirOp::SimdShl => collect_guest_v9_unary_imports(&mut used, aot::Import::SimdShl, true),
                    MirOp::SimdShr => collect_guest_v9_unary_imports(&mut used, aot::Import::SimdShr, true),
                    MirOp::SimdMin => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdMin),
                    MirOp::SimdMax => collect_guest_v11_cmp_imports(&mut used, aot::Import::SimdMax),
                    MirOp::SimdF32Add => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF32Add),
                    MirOp::SimdF32Sub => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF32Sub),
                    MirOp::SimdF32Mul => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF32Mul),
                    MirOp::SimdF32Div => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF32Div),
                    MirOp::SimdF64Add => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF64Add),
                    MirOp::SimdF64Sub => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF64Sub),
                    MirOp::SimdF64Mul => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF64Mul),
                    MirOp::SimdF64Div => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdF64Div),
                    MirOp::SimdI16Add => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdI16Add),
                    MirOp::SimdI16Sub | MirOp::SimdI16Mul | MirOp::SimdI8Add | MirOp::SimdI8Sub | MirOp::SimdI8Mul | MirOp::SimdI8And | MirOp::SimdI8Or | MirOp::SimdI8Xor | MirOp::SimdI8Not | MirOp::SimdI8Abs | MirOp::SimdI8Neg | MirOp::SimdI8Popcnt => {
                        collect_guest_v11_arith_imports(&mut used);
                    }
                    MirOp::SimdI8Min | MirOp::SimdI8Max | MirOp::SimdI8UMin | MirOp::SimdI8UMax | MirOp::SimdI8Eq | MirOp::SimdI8Ne | MirOp::SimdI8Le | MirOp::SimdI8Ge | MirOp::SimdI8AvgrU | MirOp::SimdI8Lt | MirOp::SimdI8Gt | MirOp::SimdI8AddSat | MirOp::SimdI8SubSat => {
                        collect_guest_v11_arith_imports(&mut used);
                    }
                    MirOp::SimdI8Shuffle { .. } => {
                        collect_guest_v11_arith_imports(&mut used);
                        used.insert(aot::Import::CallHost);
                        used.insert(aot::Import::PushRtI32);
                    }
                    MirOp::SimdI8Bitmask => {
                        used.insert(aot::Import::CallHost);
                        if syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::SimdGuestPopV128);
                            used.insert(aot::Import::PushRtI32);
                        }
                    }
                    MirOp::SimdI8AllTrue | MirOp::SimdI8AnyTrue => {
                        used.insert(aot::Import::CallHost);
                        if syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::SimdGuestPopV128);
                            used.insert(aot::Import::PushRtBool);
                        }
                    }
                    MirOp::SimdI8Extract | MirOp::SimdI8Replace => {
                        used.insert(aot::Import::CallHost);
                        if syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::TakeRtI32);
                            used.insert(aot::Import::SimdGuestPopV128);
                            used.insert(aot::Import::SimdGuestPushV128);
                            if matches!(instr.op, MirOp::SimdI8Extract) {
                                used.insert(aot::Import::PushRtI32);
                            }
                        }
                    }
                    MirOp::SimdI8Shl | MirOp::SimdI8Shr => collect_guest_v11_shift_imports(&mut used),
                    MirOp::SimdI64Add => collect_guest_v8_arith_imports(&mut used, aot::Import::SimdI64Add),
                    MirOp::SimdReduceAdd { element, .. } => {
                        used.insert(aot::Import::CallHost);
                        if syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::SimdGuestPopV128);
                            if *element == syx_stdlib::NumericKind::F64.to_byte() {
                                used.insert(aot::Import::PushRtF64);
                            } else if *element == syx_stdlib::NumericKind::F32.to_byte() {
                                used.insert(aot::Import::PushRtF32);
                            } else if *element == syx_stdlib::NumericKind::I64.to_byte() {
                                used.insert(aot::Import::PushRtI64);
                            } else {
                                used.insert(aot::Import::PushRtI32);
                            }
                        }
                    }
                    MirOp::SimdShuffle { .. } => {
                        collect_guest_v9_unary_imports(&mut used, aot::Import::SimdShuffle, false);
                    }
                    MirOp::SimdLoad { element, .. } => {
                        if !syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::SimdLoad);
                        }
                        if syx_config::wasm_simd128_enabled() {
                            // Phase 117 guest-v10: ArrayGet + lane pack + stack-native push.
                            used.insert(aot::Import::StoreLocal);
                            used.insert(aot::Import::LoadLocal);
                            used.insert(aot::Import::ArrayGet);
                            used.insert(aot::Import::PushRtI32);
                            used.insert(aot::Import::TakeRtI32);
                            if *element == syx_stdlib::NumericKind::F64.to_byte() {
                                used.insert(aot::Import::TakeRtF64);
                            } else if *element == syx_stdlib::NumericKind::F32.to_byte() {
                                used.insert(aot::Import::TakeRtF32);
                            } else if *element == syx_stdlib::NumericKind::I64.to_byte() {
                                used.insert(aot::Import::TakeRtI64);
                            } else if *element == syx_stdlib::NumericKind::I16.to_byte()
                                || *element == syx_stdlib::NumericKind::I8.to_byte()
                            {
                                used.insert(aot::Import::TakeRtI32);
                            }
                            used.insert(aot::Import::SimdGuestPushV128);
                        }
                    }
                    MirOp::SimdStore { element, .. } => {
                        if !syx_config::wasm_simd128_enabled() {
                            used.insert(aot::Import::SimdStore);
                        }
                        if syx_config::wasm_simd128_enabled() {
                            // Phase 117 guest-v10: stack-native pop + lane unpack + ArraySet.
                            used.insert(aot::Import::SimdGuestPopV128);
                            used.insert(aot::Import::StoreLocal);
                            used.insert(aot::Import::LoadLocal);
                            used.insert(aot::Import::ArraySet);
                            used.insert(aot::Import::PushRtI32);
                            if *element == syx_stdlib::NumericKind::F64.to_byte() {
                                used.insert(aot::Import::PushRtF64);
                            } else if *element == syx_stdlib::NumericKind::F32.to_byte() {
                                used.insert(aot::Import::PushRtF32);
                            } else if *element == syx_stdlib::NumericKind::I64.to_byte() {
                                used.insert(aot::Import::PushRtI64);
                            }
                        }
                    }
                    MirOp::ConstNumeric128 { .. } => {
                        used.insert(aot::Import::ConstNumeric128);
                    }
                    // Wave A W544 / ADR-888: MirOp ObjectRef* peeled — no AOT imports.
                    MirOp::HeapCompact => {
                        used.insert(aot::Import::HeapCompact);
                    }
                    MirOp::ConcatStr => {
                        used.insert(aot::Import::ConcatStr);
                    }
                    MirOp::ToStr => {
                        used.insert(aot::Import::ToStr);
                    }
                    _ => {}
                }
            }
        }
    }
    used
}

fn collect_used_host_indices(module: &MirModule) -> BTreeSet<u32> {
    let mut used = BTreeSet::new();
    for func in &module.functions {
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::CallHost(idx) = &instr.op {
                    used.insert(*idx);
                }
            }
        }
        for instr in &func.instructions {
            if let MirOp::CallHost(idx) = &instr.op {
                used.insert(*idx);
            }
        }
    }
    used
}

/// Phase 100: JumpIfFalse / JumpIfNil / Pop / ConstStr / ConcatStr are guest-only (no mir_exec).
fn module_needs_guest_control(module: &MirModule) -> bool {
    for func in &module.functions {
        for block in &func.blocks {
            for instr in &block.instructions {
                if matches!(
                    instr.op,
                    MirOp::JumpIfFalse(_)
                        | MirOp::JumpIfNil(_)
                        | MirOp::Pop
                        | MirOp::ConstStr(_)
                        | MirOp::ConcatStr
                ) {
                    return true;
                }
            }
        }
    }
    false
}

fn build_emit_plan(module: &MirModule, aot: bool) -> EmitPlan {
    let mut import_count = 1u32;
    let mut aot_import_map = HashMap::new();
    let mut host_import_map = HashMap::new();
    if aot {
        // Phase 90 tier-3: ExecOp only when tier-3 disabled; otherwise fall back to mir_exec.
        if !syx_config::wasm_aot_tier3_enabled() {
            aot_import_map.insert(aot::Import::ExecOp, 0);
        }
        aot_import_map.insert(aot::Import::SetFnIndex, import_count);
        import_count += 1;
        if module.functions.iter().any(|f| f.param_count > 0) {
            aot_import_map.insert(aot::Import::BeginFn, import_count);
            import_count += 1;
            aot_import_map.insert(aot::Import::EndFn, import_count);
            import_count += 1;
        }
        for kind in collect_used_aot_imports(module) {
            aot_import_map.insert(kind, import_count);
            import_count += 1;
        }
        for host_idx in collect_used_host_indices(module) {
            host_import_map.insert(host_idx, import_count);
            import_count += 1;
        }
    } else {
        // Phase 100/101: guest imports on residual interpreter path (no mir_exec prologue/string).
        aot_import_map.insert(aot::Import::SetFnIndex, import_count);
        import_count += 1;
        aot_import_map.insert(aot::Import::BeginFn, import_count);
        import_count += 1;
        aot_import_map.insert(aot::Import::GetReturn, import_count);
        import_count += 1;
        for kind in collect_used_aot_imports(module) {
            if matches!(
                kind,
                aot::Import::ConstStr
                    | aot::Import::ConcatStr
                    | aot::Import::LoadLocal
                    | aot::Import::StoreLocal
                    | aot::Import::PushRtI32
                    | aot::Import::PushRtBool
                    | aot::Import::PushRtNil
                    | aot::Import::PushRtNumeric
                    | aot::Import::TakeRtI32
                    | aot::Import::MakeClosure
                    | aot::Import::CallFn
                    | aot::Import::CallHost
                    | aot::Import::TakeCatchPc
                    | aot::Import::ActorCall
                    | aot::Import::ActorSend
                    | aot::Import::ActorRegister
                    | aot::Import::TaskSpawn
                    | aot::Import::TaskJoin
                    | aot::Import::ConstNumeric128
                    | aot::Import::SimdWasmSpill
                    | aot::Import::SimdWasmFill
                    | aot::Import::SimdAdd
                    | aot::Import::SimdSub
                    | aot::Import::SimdMul
                    | aot::Import::SimdDiv
                    | aot::Import::SimdEq
                    | aot::Import::SimdLt
                    | aot::Import::SimdGt
                    | aot::Import::SimdNe
                    | aot::Import::SimdLe
                    | aot::Import::SimdGe
                    | aot::Import::SimdAnd
                    | aot::Import::SimdOr
                    | aot::Import::SimdXor
                    | aot::Import::SimdNot
                    | aot::Import::SimdShl
                    | aot::Import::SimdShr
                    | aot::Import::SimdMin
                    | aot::Import::SimdMax
                    | aot::Import::SimdF32Add
                    | aot::Import::SimdF32Sub
                    | aot::Import::SimdF32Mul
                    | aot::Import::SimdF32Div
                    | aot::Import::SimdF64Add
                    | aot::Import::SimdF64Sub
                    | aot::Import::SimdF64Mul
                    | aot::Import::SimdF64Div
                    | aot::Import::SimdI16Add
                    | aot::Import::SimdI64Add
                    | aot::Import::TakeRtF64
                    | aot::Import::PushRtF64
                    | aot::Import::TakeRtF32
                    | aot::Import::PushRtF32
                    | aot::Import::TakeRtI64
                    | aot::Import::PushRtI64
                    | aot::Import::SimdGuestPushV128
                    | aot::Import::SimdGuestPopV128
                    | aot::Import::SimdLoad
                    | aot::Import::SimdStore
                    | aot::Import::SimdShuffle
            ) {
                aot_import_map.insert(kind, import_count);
                import_count += 1;
            }
        }
    }
    EmitPlan {
        aot,
        import_count,
        aot_import_map,
        host_import_map,
    }
}

struct StringPool {
    bytes: Vec<u8>,
}

impl StringPool {
    fn intern(&mut self, s: &str) -> (i32, i32) {
        let offset = self.bytes.len() as i32;
        self.bytes.extend_from_slice(s.as_bytes());
        (offset, s.len() as i32)
    }
}

pub fn emit_wasm(module: &MirModule) -> SyxResult<Vec<u8>> {
    emit_wasm_with_options(module, syx_config::wasm_aot_enabled())
}

pub fn emit_wasm_with_options(module: &MirModule, aot: bool) -> SyxResult<Vec<u8>> {
    aot_emit_hard_fail_reset();
    // Phase 100: guest control/string opcodes require AOT stack typing (no mir_exec POP_BOOL/CONST_STR).
    let wants_guest_control = module_needs_guest_control(module);
    if syx_config::wasm_mir_exec_only() && wants_guest_control {
        return Err(SyxDiagnostic::error(
            "wasm AOT hard-zero emit failed: Phase 100 guest control (JumpIfFalse/ConstStr/Pop) is incompatible with SYX_WASM_MIR_EXEC_ONLY",
        ));
    }
    let aot = (aot || wants_guest_control) && !syx_config::wasm_mir_exec_only();
    let plan = build_emit_plan(module, aot);
    let simd128 = syx_config::wasm_simd128_enabled();
    let mut pool = StringPool { bytes: Vec::new() };
    let mut str_map: HashMap<String, (i32, i32)> = HashMap::new();
    for func in &module.functions {
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::ConstStr(s) = &instr.op {
                    str_map.entry(s.clone()).or_insert_with(|| pool.intern(s));
                }
            }
        }
    }
    let (_, rt_str_map) = syx_mir_run::module_runtime_string_offsets(module);
    let scratch_offset = if simd128 {
        let pad = (32 - (pool.bytes.len() % 32)) % 32;
        pool.bytes.extend(std::iter::repeat_n(0u8, pad));
        let off = pool.bytes.len() as i32;
        // 32 bytes for two v128 slots + 4 for scalar shift temp
        pool.bytes.extend(std::iter::repeat_n(0u8, 48));
        off
    } else {
        -1
    };

    let mut types = TypeSection::new();
    types
        .ty()
        .function([ValType::I32, ValType::I32, ValType::I32], [ValType::I32]);
    types.ty().function([], [ValType::I32]);
    types.ty().function([ValType::I32], [ValType::I32]);
    types.ty().function([ValType::I32], []);
    types.ty().function([ValType::I32, ValType::I32], [ValType::I32]);
    types.ty().function([], []);
    types
        .ty()
        .function(
            [ValType::I32, ValType::I32],
            [],
        );
    types.ty().function(
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
        ],
        [ValType::I32],
    );
    // Phase 115: typed f64 RT stack bridge for guest SIMD reduce / f64 LS.
    types.ty().function([], [ValType::F64]);
    types.ty().function([ValType::F64], []);
    // Phase 116: typed f32 RT stack bridge for guest SIMD f32 LS.
    types.ty().function([], [ValType::F32]);
    types.ty().function([ValType::F32], []);
    // Phase 117 guest-v10: stack-native v128 RT bridge + typed i64 LS.
    types.ty().function([], [ValType::V128]);
    types.ty().function([ValType::I32, ValType::V128], []);
    types.ty().function([], [ValType::I64]);
    types.ty().function([ValType::I64], []);

    let mut imports = ImportSection::new();
    // Phase 99: never rename mir_exec → __syx_aot_run_op (hard-zero policy).
    imports.import(
        host::MODULE,
        host::MIR_EXEC,
        EntityType::Function(host::TYPE_MIR_EXEC),
    );
    // EmitPlan order: aot imports (incl. Phase 100 ConstStr/ConcatStr guest) then host imports.
    let mut aot_by_idx: Vec<(u32, aot::Import)> = plan
        .aot_import_map
        .iter()
        .map(|(k, &idx)| (idx, *k))
        .collect();
    aot_by_idx.sort_by_key(|(idx, _)| *idx);
    for (idx, kind) in aot_by_idx {
        // ExecOp may alias mir_exec at slot 0 when tier-3 is off — do not re-import.
        if idx == 0 {
            continue;
        }
        imports.import(
            host::MODULE,
            kind.name(),
            EntityType::Function(kind.type_index()),
        );
    }
    let mut host_by_idx: Vec<(u32, u32)> = plan
        .host_import_map
        .iter()
        .map(|(h, &idx)| (idx, *h))
        .collect();
    host_by_idx.sort_by_key(|(idx, _)| *idx);
    for (_, host_idx) in host_by_idx {
        let name = host_fn_name_for_index(host_idx).unwrap_or("host_unknown");
        imports.import(
            host::MODULE,
            name,
            EntityType::Function(host::TYPE_FUNC_I32),
        );
    }

    let mut functions = FunctionSection::new();
    for _ in 0..module.functions.len() {
        functions.function(host::TYPE_FUNC_I32);
    }

    let mut tables = TableSection::new();
    if plan.aot && !module.functions.is_empty() {
        tables.table(TableType {
            element_type: RefType::FUNCREF,
            minimum: module.functions.len() as u64,
            maximum: None,
            table64: false,
            shared: false,
        });
    }

    let mut memories = MemorySection::new();
    memories.memory(MemoryType {
        minimum: 1,
        maximum: None,
        memory64: false,
        shared: false,
        page_size_log2: None,
    });

    let mut globals = GlobalSection::new();
    globals.global(
        GlobalType {
            val_type: ValType::I32,
            mutable: true,
            shared: false,
        },
        &ConstExpr::i32_const(0),
    );

    let mut exports = ExportSection::new();
    exports.export("memory", ExportKind::Memory, 0);
    for (i, func) in module.functions.iter().enumerate() {
        exports.export(
            func.name.as_str(),
            ExportKind::Func,
            plan.module_func_index(i),
        );
    }

    let mut codes = CodeSection::new();
    for (func_idx, func) in module.functions.iter().enumerate() {
        let mir_i32_locals = func.local_count.max(1);
        // Guest funcs are () -> i32. Layout: MIR i32 locals, shift-amount i32, v128 scratch.
        let simd_tmp_local = if simd128 {
            Some(mir_i32_locals + 1)
        } else {
            None
        };
        let mut body = if simd128 {
            Function::new([
                (mir_i32_locals, ValType::I32),
                (1, ValType::I32),
                (1, ValType::V128),
            ])
        } else {
            Function::new([(mir_i32_locals, ValType::I32)])
        };
        emit_function_body(
            &mut body,
            func_idx,
            func,
            module,
            &plan,
            &str_map,
            &rt_str_map,
            scratch_offset,
            simd128,
            simd_tmp_local,
        );
        codes.function(&body);
    }

    let mut data = DataSection::new();
    if !pool.bytes.is_empty() {
        data.active(
            0,
            &ConstExpr::i32_const(0),
            pool.bytes.iter().copied(),
        );
    }

    let mut elements = ElementSection::new();
    if plan.aot && !module.functions.is_empty() {
        let func_indices: Vec<u32> = (0..module.functions.len())
            .map(|i| plan.module_func_index(i))
            .collect();
        elements.active(
            Some(0),
            &ConstExpr::i32_const(0),
            Elements::Functions(func_indices.into()),
        );
    }

    let mut wasm_module = Module::new();
    wasm_module.section(&types);
    wasm_module.section(&imports);
    wasm_module.section(&functions);
    if plan.aot && !module.functions.is_empty() {
        wasm_module.section(&tables);
    }
    wasm_module.section(&memories);
    wasm_module.section(&globals);
    wasm_module.section(&exports);
    if plan.aot && !module.functions.is_empty() {
        wasm_module.section(&elements);
    }
    wasm_module.section(&codes);
    if !pool.bytes.is_empty() {
        wasm_module.section(&data);
    }
    if let Some(msg) = aot_emit_hard_fail_take() {
        return Err(SyxDiagnostic::error(format!(
            "wasm AOT hard-zero emit failed: {msg}"
        )));
    }
    Ok(wasm_module.finish())
}

fn emit_function_body(
    body: &mut Function,
    func_idx: usize,
    func: &MirFunction,
    module: &MirModule,
    plan: &EmitPlan,
    str_map: &HashMap<String, (i32, i32)>,
    rt_str_map: &HashMap<String, (i32, i32)>,
    scratch_offset: i32,
    simd128: bool,
    simd_tmp_local: Option<u32>,
) {
    body.instruction(&WasmInstr::I32Const(func_idx as i32));
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SetFnIndex)));
    if !plan.aot || func.param_count > 0 {
        body.instruction(&WasmInstr::I32Const(func.param_count as i32));
        body.instruction(&WasmInstr::I32Const(func.local_count as i32));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::BeginFn)));
        body.instruction(&WasmInstr::Drop);
    }

    let block_pos: HashMap<u32, usize> = func
        .blocks
        .iter()
        .enumerate()
        .map(|(i, b)| (b.id, i))
        .collect();

    let exit_returns_i32 = func.name == "main";
    let simd_export_shim = func.name.starts_with("SIMD") && func.name.contains('_');
    if needs_cfg_dispatch(func) {
        emit_cfg_body(
            body,
            func,
            module,
            plan,
            str_map,
            rt_str_map,
            &block_pos,
            exit_returns_i32,
            func.param_count,
            scratch_offset,
            simd128,
            simd_tmp_local,
            simd_export_shim,
        );
    } else {
        emit_linear_body(
            body,
            func,
            module,
            plan,
            str_map,
            rt_str_map,
            exit_returns_i32,
            func.param_count,
            scratch_offset,
            simd128,
            simd_tmp_local,
            simd_export_shim,
        );
    }
    body.instruction(&WasmInstr::End);
}

fn needs_cfg_dispatch(func: &MirFunction) -> bool {
    func.blocks.iter().any(|b| {
        b.instructions.iter().any(|i| {
            matches!(
                i.op,
                MirOp::Jump(_)
                    | MirOp::JumpIfFalse(_)
                    | MirOp::JumpIfNil(_)
                    | MirOp::BeginTry(_)
            )
        })
    })
}

fn emit_linear_body(
    body: &mut Function,
    func: &MirFunction,
    module: &MirModule,
    plan: &EmitPlan,
    str_map: &HashMap<String, (i32, i32)>,
    rt_str_map: &HashMap<String, (i32, i32)>,
    exit_returns_i32: bool,
    func_param_count: u32,
    scratch_offset: i32,
    simd128: bool,
    simd_tmp_local: Option<u32>,
    simd_export_shim: bool,
) {
    let mut returned = false;
    'linear: for block in &func.blocks {
        for instr in &block.instructions {
            if matches!(instr.op, MirOp::Return) {
                returned = true;
            }
            emit_mir_instr(
                body,
                instr,
                module,
                plan,
                str_map,
                rt_str_map,
                &HashMap::new(),
                0,
                0,
                exit_returns_i32,
                func_param_count,
                scratch_offset,
                simd128,
                simd_tmp_local,
                simd_export_shim,
            );
            if matches!(instr.op, MirOp::Return) {
                break 'linear;
            }
        }
    }
    if !returned {
        body.instruction(&WasmInstr::I32Const(0));
        body.instruction(&WasmInstr::Return);
    }
}

fn emit_cfg_body(
    body: &mut Function,
    func: &MirFunction,
    module: &MirModule,
    plan: &EmitPlan,
    str_map: &HashMap<String, (i32, i32)>,
    rt_str_map: &HashMap<String, (i32, i32)>,
    block_pos: &HashMap<u32, usize>,
    exit_returns_i32: bool,
    func_param_count: u32,
    scratch_offset: i32,
    simd128: bool,
    simd_tmp_local: Option<u32>,
    simd_export_shim: bool,
) {
    let entry_pc = *block_pos.get(&func.entry).unwrap_or(&0) as i32;

    body.instruction(&WasmInstr::I32Const(entry_pc));
    body.instruction(&WasmInstr::GlobalSet(0));

    body.instruction(&WasmInstr::Loop(wasm_encoder::BlockType::Empty));

    for (block_idx, block) in func.blocks.iter().enumerate() {
        let start = block_idx as i32;
        body.instruction(&WasmInstr::Block(wasm_encoder::BlockType::Empty));
        body.instruction(&WasmInstr::GlobalGet(0));
        body.instruction(&WasmInstr::I32Const(start));
        body.instruction(&WasmInstr::I32Eq);
        body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));

        let mut ends_with_transfer = false;
        let mut idx = 0;
        while idx < block.instructions.len() {
            if matches!(block.instructions[idx].op, MirOp::JumpIfFalse(_)) {
                if idx + 1 < block.instructions.len() {
                    let target = match block.instructions[idx].op {
                        MirOp::JumpIfFalse(t) => t,
                        _ => unreachable!(),
                    };
                    // Phase 100: JumpIfFalse is true guest (i32.eqz) — never POP_BOOL via mir_exec.
                    body.instruction(&WasmInstr::I32Eqz);
                    body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
                    let t = *block_pos.get(&target).unwrap_or(&(block_idx + 1)) as i32;
                    body.instruction(&WasmInstr::I32Const(t));
                    body.instruction(&WasmInstr::GlobalSet(0));
                    body.instruction(&WasmInstr::Br(3));
                    body.instruction(&WasmInstr::Else);
                    idx += 1;
                    while idx < block.instructions.len() {
                        if matches!(
                            block.instructions[idx].op,
                            MirOp::Jump(_) | MirOp::Return
                        ) {
                            ends_with_transfer = true;
                        }
                        emit_mir_instr(
                            body,
                            &block.instructions[idx],
                            module,
                            plan,
                            str_map,
                            rt_str_map,
                            block_pos,
                            block_idx,
                            2,
                            exit_returns_i32,
                            func_param_count,
                            scratch_offset,
                            simd128,
                            simd_tmp_local,
                            simd_export_shim,
                        );
                        idx += 1;
                    }
                    body.instruction(&WasmInstr::End);
                } else {
                    emit_mir_instr(
                        body,
                        &block.instructions[idx],
                        module,
                        plan,
                        str_map,
                        rt_str_map,
                        block_pos,
                        block_idx,
                        2,
                        exit_returns_i32,
                        func_param_count,
                        scratch_offset,
                        simd128,
                        simd_tmp_local,
                        simd_export_shim,
                    );
                    idx += 1;
                }
            } else {
                if matches!(
                    block.instructions[idx].op,
                    MirOp::Jump(_) | MirOp::Return
                ) {
                    ends_with_transfer = true;
                }
                emit_mir_instr(
                    body,
                    &block.instructions[idx],
                    module,
                    plan,
                    str_map,
                    rt_str_map,
                    block_pos,
                    block_idx,
                    2,
                    exit_returns_i32,
                    func_param_count,
                    scratch_offset,
                    simd128,
                    simd_tmp_local,
                    simd_export_shim,
                );
                idx += 1;
            }
        }

        if !ends_with_transfer {
            let next_pc = block_idx + 1;
            body.instruction(&WasmInstr::I32Const(next_pc as i32));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(2));
        }

        body.instruction(&WasmInstr::End);
        body.instruction(&WasmInstr::End);
    }

    body.instruction(&WasmInstr::End);

    body.instruction(&WasmInstr::I32Const(0));
    body.instruction(&WasmInstr::Return);
}
fn catch_br(br_to_loop: u32) -> Option<u32> {
    if br_to_loop > 0 {
        Some(br_to_loop)
    } else {
        None
    }
}

/// Wave A W18 / ADR-362: after CallHost (i32 result on stack), dispatch pending catch
/// using the same encoding as `emit_mir_exec` / `mir_exec_dispatch` (`-(pc+2)`).
/// Fallthrough restores the host i32 (same as `emit_mir_exec(..., keep_result: true)`).
fn emit_call_host_take_catch(body: &mut Function, br_to_loop: u32) {
    body.instruction(&WasmInstr::LocalSet(0));
    body.instruction(&WasmInstr::LocalGet(0));
    body.instruction(&WasmInstr::I32Const(-2));
    body.instruction(&WasmInstr::I32LtS);
    body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
    body.instruction(&WasmInstr::I32Const(-2));
    body.instruction(&WasmInstr::LocalGet(0));
    body.instruction(&WasmInstr::I32Sub);
    body.instruction(&WasmInstr::GlobalSet(0));
    if br_to_loop > 0 {
        body.instruction(&WasmInstr::Br(br_to_loop + 1));
    }
    body.instruction(&WasmInstr::End);
    body.instruction(&WasmInstr::LocalGet(0));
}

/// Phase 110: one host spill call for both binop SIMD operands (slot `-1`).
/// Retained only for non-simd128 residual host paths; guest-v11 GA pilots do not use spill/fill.
#[allow(dead_code)]
const SIMD_SPILL_BINOP_PAIR: i32 = -1;

fn collect_guest_v8_arith_imports(used: &mut BTreeSet<aot::Import>, fallback: aot::Import) {
    if !syx_config::wasm_simd128_enabled() {
        used.insert(fallback);
        used.insert(aot::Import::SimdWasmSpill);
        used.insert(aot::Import::SimdWasmFill);
    } else {
        used.insert(aot::Import::SimdGuestPushV128);
        used.insert(aot::Import::SimdGuestPopV128);
        used.insert(aot::Import::CallHost);
        let f64_fallback = matches!(
            fallback,
            aot::Import::SimdF64Add
                | aot::Import::SimdF64Sub
                | aot::Import::SimdF64Mul
                | aot::Import::SimdF64Div
        );
        let guest_v11_pilot = matches!(
            fallback,
            aot::Import::SimdI16Add | aot::Import::SimdI64Add
        );
        if !f64_fallback && !guest_v11_pilot {
            used.insert(fallback);
        }
    }
}

/// Phase 116 guest-v9 / Phase 117 guest-v10: stack-native unary/shift/shuffle (no spill/fill).
/// Fingerprint: `wasm-simd128-guest-v11:zero-scratch-simd16`
fn collect_guest_v9_unary_imports(
    used: &mut BTreeSet<aot::Import>,
    fallback: aot::Import,
    shift: bool,
) {
    if !syx_config::wasm_simd128_enabled() {
        used.insert(fallback);
        used.insert(aot::Import::SimdWasmSpill);
        used.insert(aot::Import::SimdWasmFill);
    } else {
        used.insert(aot::Import::SimdGuestPushV128);
        used.insert(aot::Import::SimdGuestPopV128);
        if shift {
            used.insert(aot::Import::TakeRtI32);
        }
        used.insert(fallback);
    }
}

/// Phase 118 guest-v11: stack-native guest binops (no spill/fill; no extra aot Import).
fn collect_guest_v11_arith_imports(used: &mut BTreeSet<aot::Import>) {
    if !syx_config::wasm_simd128_enabled() {
        used.insert(aot::Import::CallHost);
    } else {
        used.insert(aot::Import::SimdGuestPushV128);
        used.insert(aot::Import::SimdGuestPopV128);
    }
}

/// Phase 121 guest-v11: unary shift import collect without stale spill/fill.
fn collect_guest_v11_shift_imports(used: &mut BTreeSet<aot::Import>) {
    if !syx_config::wasm_simd128_enabled() {
        used.insert(aot::Import::CallHost);
    } else {
        used.insert(aot::Import::SimdGuestPushV128);
        used.insert(aot::Import::SimdGuestPopV128);
        used.insert(aot::Import::TakeRtI32);
    }
}

/// Phase 118 guest-v11: cmp/bitwise/minmax import collect without stale spill/fill.
fn collect_guest_v11_cmp_imports(used: &mut BTreeSet<aot::Import>, fallback: aot::Import) {
    if !syx_config::wasm_simd128_enabled() {
        used.insert(fallback);
        used.insert(aot::Import::SimdWasmSpill);
        used.insert(aot::Import::SimdWasmFill);
    } else {
        used.insert(aot::Import::SimdGuestPushV128);
        used.insert(aot::Import::SimdGuestPopV128);
        used.insert(fallback);
    }
}

/// RT local slot used to hold the Array receiver across guest Load/Store lane loops.
const SIMD_LS_ARRAY_LOCAL: i32 = 240;

/// Phase 117 guest-v10: push wasm v128 (on stack) into RT simd — stack-native ABI.
/// Fingerprint: `wasm-simd128-guest-v11:zero-scratch-simd16`
fn emit_simd_guest_push_v128(
    body: &mut Function,
    plan: &EmitPlan,
    tmp_local: u32,
    element: u8,
) {
    body.instruction(&WasmInstr::LocalSet(tmp_local));
    body.instruction(&WasmInstr::I32Const(element as i32));
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    body.instruction(&WasmInstr::Call(
        plan.aot_import(aot::Import::SimdGuestPushV128),
    ));
}

/// Phase 117 guest-v10: pop RT simd onto wasm v128 stack — stack-native ABI.
fn emit_simd_guest_pop_v128(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(
        plan.aot_import(aot::Import::SimdGuestPopV128),
    ));
}

fn scratch_v128_mem() -> wasm_encoder::MemArg {
    wasm_encoder::MemArg {
        offset: 0,
        align: 4,
        memory_index: 0,
    }
}

fn scratch_i32_mem_at(offset: i32) -> wasm_encoder::MemArg {
    wasm_encoder::MemArg {
        offset: offset as u64,
        align: 2,
        memory_index: 0,
    }
}

fn scratch_f64_mem() -> wasm_encoder::MemArg {
    wasm_encoder::MemArg {
        offset: 0,
        align: 3,
        memory_index: 0,
    }
}

fn emit_take_rt_f64(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtF64)));
}

fn emit_push_rt_f64(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtF64)));
}

fn emit_take_rt_f32(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtF32)));
}

fn emit_push_rt_f32(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtF32)));
}

fn emit_take_rt_i64(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI64)));
}

fn emit_push_rt_i64(body: &mut Function, plan: &EmitPlan) {
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI64)));
}

/// Phase 117 guest-v10: stack-native v128 binop (pop→v128 stack→op→push; no scratch ABI).
/// Fingerprint: `wasm-simd128-guest-v11:zero-scratch-simd16`
fn emit_simd_export_host_call(body: &mut Function, plan: &EmitPlan, host: syx_stdlib::HostFnId) {
    let idx = syx_stdlib::host_fn_index(host);
    body.instruction(&WasmInstr::I32Const(idx as i32));
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallHost)));
}

fn emit_simd_binop_v128_nospill(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    element: u8,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
    wasm_op: fn(&mut Function),
) {
    let _ = simd_export_shim;
    // RT top is rhs; pop rhs then lhs so wasm stack is [lhs, rhs].
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalSet(tmp_local));
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    wasm_op(body);
    emit_simd_guest_push_v128(body, plan, tmp_local, element);
    let _ = br_to_loop;
}

fn emit_simd_unary_v128_nospill(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    element: u8,
    br_to_loop: Option<u32>,
    wasm_op: fn(&mut Function),
) {
    emit_simd_guest_pop_v128(body, plan);
    wasm_op(body);
    emit_simd_guest_push_v128(body, plan, tmp_local, element);
    let _ = br_to_loop;
}

fn emit_simd_shift_i32x4_nospill(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    element: u8,
    br_to_loop: Option<u32>,
    wasm_op: fn(&mut Function),
) {
    let i32_tmp = tmp_local - 1;
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
    body.instruction(&WasmInstr::LocalSet(i32_tmp));
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalGet(i32_tmp));
    wasm_op(body);
    emit_simd_guest_push_v128(body, plan, tmp_local, element);
    let _ = br_to_loop;
}

fn emit_v128_load_from_scratch(body: &mut Function, scratch_offset: i32) {
    body.instruction(&WasmInstr::I32Const(0));
    body.instruction(&WasmInstr::V128Load(wasm_encoder::MemArg {
        offset: scratch_offset as u64,
        align: 4,
        memory_index: 0,
    }));
}

fn emit_v128_store_to_scratch(body: &mut Function, tmp_local: u32, scratch_offset: i32) {
    body.instruction(&WasmInstr::LocalSet(tmp_local));
    body.instruction(&WasmInstr::I32Const(0));
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    body.instruction(&WasmInstr::V128Store(wasm_encoder::MemArg {
        offset: scratch_offset as u64,
        align: 4,
        memory_index: 0,
    }));
}

/// Phase 114: F64/I64/U64 literals need full 64-bit guest push (PushRtNumeric truncates to i32).
fn const_numeric_needs_128_emit(kind: u8) -> bool {
    syx_stdlib::NumericKind::from_byte(kind)
        .map(|k| syx_stdlib::bit_width(syx_stdlib::resolve_alias(k)) > 32)
        .unwrap_or(false)
}

fn emit_const_numeric_guest(
    body: &mut Function,
    plan: &EmitPlan,
    kind: u8,
    bits: u64,
) {
    if const_numeric_needs_128_emit(kind) {
        emit_const_numeric128_guest(body, plan, kind, bits, 0);
    } else {
        body.instruction(&WasmInstr::I32Const(kind as i32));
        body.instruction(&WasmInstr::I32Const(bits as i32));
        body.instruction(&WasmInstr::Call(
            plan.aot_import(aot::Import::PushRtNumeric),
        ));
    }
}

fn emit_const_numeric128_guest(
    body: &mut Function,
    plan: &EmitPlan,
    kind: u8,
    lo: u64,
    hi: u64,
) {
    body.instruction(&WasmInstr::I32Const(kind as i32));
    body.instruction(&WasmInstr::I32Const((lo & 0xffff_ffff) as i32));
    body.instruction(&WasmInstr::I32Const((lo >> 32) as i32));
    body.instruction(&WasmInstr::I32Const((hi & 0xffff_ffff) as i32));
    body.instruction(&WasmInstr::I32Const((hi >> 32) as i32));
    body.instruction(&WasmInstr::Call(
        plan.aot_import(aot::Import::ConstNumeric128),
    ));
    body.instruction(&WasmInstr::Drop);
}

/// Phase 111: Add/Sub/Mul (and Div operand load) guest-v8 nospill binops.
/// Fingerprint: wasm-simd128-guest-v8:nospill-binop-f64muldiv-reduce
fn emit_simd_binop_i32x4_thin(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
    wasm_op: fn(&mut Function),
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
        simd_export_shim,
        br_to_loop,
        wasm_op,
    );
}

fn emit_simd_add_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4Add);
    });
}

fn emit_simd_sub_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4Sub);
    });
}

fn emit_simd_mul_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4Mul);
    });
}

/// Phase 109 honest path: Wasm SIMD has **no** `i32x4.div_*` opcode.
/// Guest emit uses lane-wise `i32.div_s` via extract/replace (not host Call).
/// Phase 117: stack-native pop/push; scratch only for lane scratchpad.
fn emit_simd_div_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    let lane_tmp = tmp_local - 1;
    emit_simd_guest_pop_v128(body, plan);
    emit_v128_store_to_scratch(body, tmp_local, scratch_offset + 16);
    emit_simd_guest_pop_v128(body, plan);
    emit_v128_store_to_scratch(body, tmp_local, scratch_offset);
    for lane in 0u8..4 {
        emit_v128_load_from_scratch(body, scratch_offset);
        body.instruction(&WasmInstr::I32x4ExtractLane(lane));
        emit_v128_load_from_scratch(body, scratch_offset + 16);
        body.instruction(&WasmInstr::I32x4ExtractLane(lane));
        body.instruction(&WasmInstr::I32DivS);
        body.instruction(&WasmInstr::LocalSet(lane_tmp));
        emit_v128_load_from_scratch(body, scratch_offset);
        body.instruction(&WasmInstr::LocalGet(lane_tmp));
        body.instruction(&WasmInstr::I32x4ReplaceLane(lane));
        emit_v128_store_to_scratch(body, tmp_local, scratch_offset);
    }
    emit_v128_load_from_scratch(body, scratch_offset);
    emit_simd_guest_push_v128(
        body,
        plan,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
    );
    let _ = br_to_loop;
}

fn emit_simd_min_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4MinS);
    });
}

fn emit_simd_max_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4MaxS);
    });
}

/// Phase 111: `f32x4` add guest pilot (honest float lane semantics).
fn emit_simd_add_f32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F32.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F32x4Add);
        },
    );
}

/// Phase 112: `f32x4` sub guest pilot.
fn emit_simd_sub_f32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F32.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F32x4Sub);
        },
    );
}

/// Phase 112: `f32x4` mul guest pilot.
fn emit_simd_mul_f32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F32.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F32x4Mul);
        },
    );
}

/// Phase 113: `f32x4` div guest pilot (`wasm-simd128-guest-v6:f32div-guest-ls-f64`).
fn emit_simd_div_f32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F32.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F32x4Div);
        },
    );
}

/// Phase 114: `f64x2` sub guest pilot (`wasm-simd128-guest-v7:f64reduce-guest-ls-f64sub`).
fn emit_simd_sub_f64x2_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F64.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F64x2Sub);
        },
    );
}

fn emit_simd_add_f64x2_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F64.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F64x2Add);
        },
    );
}

/// Phase 117: `i16x8` add guest pilot (SIMD8 equivalent of i32×8).
/// Fingerprint: `wasm-simd128-guest-v11:zero-scratch-simd16`
fn emit_simd_add_i16x8_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I16.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I16x8Add);
        },
    );
}

fn emit_simd_sub_i16x8_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I16.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I16x8Sub);
        },
    );
}

fn emit_simd_mul_i16x8_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I16.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I16x8Mul);
        },
    );
}

fn emit_simd_add_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Add);
        },
    );
}

fn emit_simd_sub_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Sub);
        },
    );
}

/// Phase 119: `i8x16` mul guest pilot (matrix-expand; host lane mul — no WASM `i8x16.mul` opcode).
/// Fingerprint: `wasm-simd16-guest-v2:matrix-expand`
fn emit_simd_mul_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    _tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = br_to_loop;
    let _ = simd_export_shim;
    // WASM SIMD has no `i8x16.mul`; guest matrix-expand uses host lane mul (native/wasm parity).
    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Mul);
}

/// Phase 120: `i8x16` and guest pilot (logic-or-shift tail; native wasm `i8x16.and`).
/// Fingerprint: `wasm-simd16-guest-v3:logic-or-shift`
fn emit_simd_and_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::V128And);
        },
    );
}

/// Phase 122: `i8x16` or guest pilot (logic-or-xor MVP; native wasm `v128.or`).
/// Fingerprint: `wasm-simd16-guest-v5:logic-or-xor`
fn emit_simd_or_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::V128Or);
        },
    );
}

/// Phase 122: `i8x16` xor guest pilot (logic-or-xor MVP; native wasm `v128.xor`).
/// Fingerprint: `wasm-simd16-guest-v5:logic-or-xor`
fn emit_simd_xor_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::V128Xor);
        },
    );
}

/// Phase 123: `i8x16` not guest pilot (logic-not MVP; native wasm `v128.not`).
/// Fingerprint: `wasm-simd16-guest-v6:logic-not`
fn emit_simd_not_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_unary_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::V128Not);
        },
    );
}

/// Phase 124: `i8x16` min guest pilot (lane-min-max MVP; native wasm `i8x16.min_s`).
/// Fingerprint: `wasm-simd16-guest-v7:lane-min-max`
fn emit_simd_min_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16MinS);
        },
    );
}

/// Phase 124: `i8x16` max guest pilot (lane-min-max MVP; native wasm `i8x16.max_s`).
/// Fingerprint: `wasm-simd16-guest-v7:lane-min-max`
fn emit_simd_max_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16MaxS);
        },
    );
}

/// Phase 130: `i8x16` unsigned min guest pilot (unsigned-min-max MVP; native wasm `i8x16.min_u`).
/// Fingerprint: `wasm-simd16-guest-v13:unsigned-min-max-mvp`
fn emit_simd_umin_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16MinU);
        },
    );
}

/// Phase 130: `i8x16` unsigned max guest pilot (unsigned-min-max MVP; native wasm `i8x16.max_u`).
/// Fingerprint: `wasm-simd16-guest-v13:unsigned-min-max-mvp`
fn emit_simd_umax_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16MaxU);
        },
    );
}

/// Phase 131: `i8x16` abs guest pilot (abs MVP; native wasm `i8x16.abs`).
/// Fingerprint: `wasm-simd16-guest-v14:abs-mvp`
fn emit_simd_abs_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_unary_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Abs);
        },
    );
}

/// Phase 132: `i8x16` neg guest pilot (neg MVP; native wasm `i8x16.neg`).
/// Fingerprint: `wasm-simd16-guest-v15:neg-mvp`
fn emit_simd_neg_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_unary_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Neg);
        },
    );
}

/// Phase 126: `i8x16` eq guest pilot (compare-lane MVP; native wasm `i8x16.eq`).
/// Fingerprint: `wasm-simd16-guest-v9:compare-lane-mvp`
fn emit_simd_eq_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Eq);
        },
    );
}

/// Phase 133: `i8x16` ne guest pilot (ne MVP; native wasm `i8x16.ne`).
/// Fingerprint: `wasm-simd16-guest-v16:ne-mvp`
fn emit_simd_ne_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Ne);
        },
    );
}

/// Phase 134: `i8x16` le guest pilot (le MVP; native wasm `i8x16.le_s`).
/// Fingerprint: `wasm-simd16-guest-v17:le-mvp`
fn emit_simd_le_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16LeS);
        },
    );
}

/// Phase 135: `i8x16` ge guest pilot (ge MVP; native wasm `i8x16.ge_s`).
/// Fingerprint: `wasm-simd16-guest-v18:ge-mvp`
fn emit_simd_ge_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16GeS);
        },
    );
}

/// Phase 136: `i8x16` avgr_u guest pilot (avgr_u MVP; native wasm `i8x16.avgr_u`).
/// Fingerprint: `wasm-simd16-guest-v19:avgr-u-mvp`
fn emit_simd_avgr_u_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16AvgrU);
        },
    );
}

/// Phase 137: `i8x16` popcnt guest pilot (popcnt MVP; native wasm `i8x16.popcnt`).
/// Fingerprint: `wasm-simd16-guest-v20:popcnt-mvp`
fn emit_simd_popcnt_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_unary_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Popcnt);
        },
    );
}

/// Phase 138: `i8x16` bitmask guest pilot (bitmask MVP; native wasm `i8x16.bitmask`).
/// Fingerprint: `wasm-simd16-guest-v21:bitmask-mvp`
fn emit_simd_bitmask_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    _tmp_local: u32,
    br_to_loop: Option<u32>,
) {
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::I8x16Bitmask);
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
    let _ = br_to_loop;
}

/// Phase 139: `i8x16.all_true` guest pilot (all-any-true MVP; native wasm `i8x16.all_true`).
/// Fingerprint: `wasm-simd16-guest-v22:all-any-true-mvp`
fn emit_simd_all_true_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    _tmp_local: u32,
    br_to_loop: Option<u32>,
) {
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::I8x16AllTrue);
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtBool)));
    let _ = br_to_loop;
}

/// Phase 139: `v128.any_true` guest pilot (all-any-true MVP; native wasm `v128.any_true`).
/// Fingerprint: `wasm-simd16-guest-v22:all-any-true-mvp`
fn emit_simd_any_true_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    _tmp_local: u32,
    br_to_loop: Option<u32>,
) {
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::V128AnyTrue);
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtBool)));
    let _ = br_to_loop;
}

/// Phase 126: `i8x16` lt guest pilot (compare-lane MVP; native wasm `i8x16.lt_s`).
/// Fingerprint: `wasm-simd16-guest-v9:compare-lane-mvp`
fn emit_simd_lt_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16LtS);
        },
    );
}

/// Phase 126: `i8x16` gt guest pilot (compare-lane MVP; native wasm `i8x16.gt_s`).
/// Fingerprint: `wasm-simd16-guest-v9:compare-lane-mvp`
fn emit_simd_gt_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16GtS);
        },
    );
}

/// Phase 127: `i8x16` signed saturating add guest pilot (native wasm `i8x16.add_sat_s`).
/// Fingerprint: `wasm-simd16-guest-v10:saturating-add-sub-mvp`
fn emit_simd_add_sat_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16AddSatS);
        },
    );
}

/// Phase 127: `i8x16` signed saturating sub guest pilot (native wasm `i8x16.sub_sat_s`).
/// Fingerprint: `wasm-simd16-guest-v10:saturating-add-sub-mvp`
fn emit_simd_sub_sat_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16SubSatS);
        },
    );
}

/// Phase 128: fixed-mode `i8x16` shuffle guest pilot (native wasm `i8x16.shuffle`).
/// Fingerprint: `wasm-simd16-guest-v11:shuffle-mvp`
/// mode 0 = reverse, mode 1 = swap pairs, else identity (matches host `simd_shuffle_i8`).
fn emit_simd_shuffle_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    mode: i32,
    br_to_loop: Option<u32>,
) {
    let lanes: [u8; 16] = match mode {
        0 => [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        1 => [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14],
        _ => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    };
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalTee(tmp_local));
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    body.instruction(&WasmInstr::I8x16Shuffle(lanes));
    emit_simd_guest_push_v128(
        body,
        plan,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
    );
    let _ = br_to_loop;
}

/// Phase 125: `i8x16` extract lane guest pilot (lane-extract-replace MVP; native wasm `i8x16.extract_lane_s`).
/// Fingerprint: `wasm-simd16-guest-v8:lane-extract-replace`
fn emit_simd_extract_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    br_to_loop: Option<u32>,
) {
    let lane_local = tmp_local - 1;
    let v128_local = tmp_local;
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
    body.instruction(&WasmInstr::LocalSet(lane_local));
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalSet(v128_local));
    body.instruction(&WasmInstr::Block(wasm_encoder::BlockType::Result(ValType::I32)));
    for lane in 0u8..16 {
        body.instruction(&WasmInstr::LocalGet(lane_local));
        body.instruction(&WasmInstr::I32Const(lane as i32));
        body.instruction(&WasmInstr::I32Eq);
        body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
        body.instruction(&WasmInstr::LocalGet(v128_local));
        body.instruction(&WasmInstr::I8x16ExtractLaneS(lane));
        body.instruction(&WasmInstr::Br(1));
        body.instruction(&WasmInstr::End);
    }
    body.instruction(&WasmInstr::LocalGet(v128_local));
    body.instruction(&WasmInstr::I8x16ExtractLaneS(0));
    body.instruction(&WasmInstr::End);
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
    let _ = br_to_loop;
}

/// Phase 125: `i8x16` replace lane guest pilot (lane-extract-replace MVP; native wasm `i8x16.replace_lane`).
/// Fingerprint: `wasm-simd16-guest-v8:lane-extract-replace`
fn emit_simd_replace_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    br_to_loop: Option<u32>,
) {
    let value_local = tmp_local - 2;
    let lane_local = tmp_local - 1;
    let v128_local = tmp_local;
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
    body.instruction(&WasmInstr::LocalSet(value_local));
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
    body.instruction(&WasmInstr::LocalSet(lane_local));
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalSet(v128_local));
    body.instruction(&WasmInstr::Block(wasm_encoder::BlockType::Empty));
    for lane in 0u8..16 {
        body.instruction(&WasmInstr::LocalGet(lane_local));
        body.instruction(&WasmInstr::I32Const(lane as i32));
        body.instruction(&WasmInstr::I32Eq);
        body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
        body.instruction(&WasmInstr::LocalGet(v128_local));
        body.instruction(&WasmInstr::LocalGet(value_local));
        body.instruction(&WasmInstr::I8x16ReplaceLane(lane));
        body.instruction(&WasmInstr::LocalSet(v128_local));
        body.instruction(&WasmInstr::Br(1));
        body.instruction(&WasmInstr::End);
    }
    body.instruction(&WasmInstr::LocalGet(v128_local));
    body.instruction(&WasmInstr::LocalGet(value_local));
    body.instruction(&WasmInstr::I8x16ReplaceLane(0));
    body.instruction(&WasmInstr::LocalSet(v128_local));
    body.instruction(&WasmInstr::End);
    body.instruction(&WasmInstr::LocalGet(v128_local));
    emit_simd_guest_push_v128(
        body,
        plan,
        v128_local,
        syx_stdlib::NumericKind::I8.to_byte(),
    );
    let _ = br_to_loop;
}

/// Phase 121: `i8x16` shl guest pilot (shift lane MVP; native wasm `i8x16.shl`).
/// Fingerprint: `wasm-simd16-guest-v4:shift-lane-mvp`
fn emit_simd_shl_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_shift_i32x4_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16Shl);
        },
    );
}

/// Phase 121: `i8x16` shr guest pilot (shift lane MVP; native wasm `i8x16.shr_s`).
/// Fingerprint: `wasm-simd16-guest-v4:shift-lane-mvp`
fn emit_simd_shr_i8x16_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_shift_i32x4_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I8.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I8x16ShrS);
        },
    );
}

/// Phase 117: `i64x2` add guest pilot.
fn emit_simd_add_i64x2_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I64.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I64x2Add);
        },
    );
}

fn emit_simd_mul_f64x2_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F64.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F64x2Mul);
        },
    );
}

fn emit_simd_div_f64x2_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::F64.to_byte(),
        simd_export_shim,
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::F64x2Div);
        },
    );
}

fn emit_simd_eq_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4Eq);
    });
}

fn emit_simd_lt_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4LtS);
    });
}

fn emit_simd_gt_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4GtS);
    });
}

fn emit_simd_and_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::V128And);
    });
}

fn emit_simd_or_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::V128Or);
    });
}

fn emit_simd_xor_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::V128Xor);
    });
}

fn emit_simd_ne_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4Ne);
    });
}

fn emit_simd_le_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4LeS);
    });
}

fn emit_simd_ge_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    emit_simd_binop_i32x4_thin(body, plan, scratch_offset, tmp_local, simd_export_shim, br_to_loop, |body| {
        body.instruction(&WasmInstr::I32x4GeS);
    });
}

fn emit_simd_not_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_unary_v128_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::V128Not);
        },
    );
}

fn emit_simd_shl_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_shift_i32x4_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I32x4Shl);
        },
    );
}

fn emit_simd_shr_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    scratch_offset: i32,
    tmp_local: u32,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    emit_simd_shift_i32x4_nospill(
        body,
        plan,
        scratch_offset,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
        br_to_loop,
        |body| {
            body.instruction(&WasmInstr::I32x4ShrS);
        },
    );
}

fn emit_v128_splat_zero(
    body: &mut Function,
    f64_element: bool,
    f32_element: bool,
    i64_element: bool,
    i16_element: bool,
    i8_element: bool,
) {
    if f64_element {
        body.instruction(&WasmInstr::F64Const(0.0));
        body.instruction(&WasmInstr::F64x2Splat);
    } else if f32_element {
        body.instruction(&WasmInstr::F32Const(0.0));
        body.instruction(&WasmInstr::F32x4Splat);
    } else if i64_element {
        body.instruction(&WasmInstr::I64Const(0));
        body.instruction(&WasmInstr::I64x2Splat);
    } else if i16_element {
        body.instruction(&WasmInstr::I32Const(0));
        body.instruction(&WasmInstr::I16x8Splat);
    } else if i8_element {
        body.instruction(&WasmInstr::I32Const(0));
        body.instruction(&WasmInstr::I8x16Splat);
    } else {
        body.instruction(&WasmInstr::I32Const(0));
        body.instruction(&WasmInstr::I32x4Splat);
    }
}

fn simd_guest_lane_count(lanes: u8, element: u8) -> u8 {
    let f64_element = element == syx_stdlib::NumericKind::F64.to_byte();
    let i64_element = element == syx_stdlib::NumericKind::I64.to_byte();
    let i16_element = element == syx_stdlib::NumericKind::I16.to_byte();
    let i8_element = element == syx_stdlib::NumericKind::I8.to_byte();
    if f64_element || i64_element {
        lanes.max(1).min(2)
    } else if i16_element {
        lanes.max(1).min(8)
    } else if i8_element {
        lanes.max(1).min(16)
    } else {
        lanes.max(1).min(4)
    }
}

/// Phase 118 guest-v11: stack-native ArrayGet lane pack (no V128Load/Store scratch).
/// Phase 129: i8x16 load MVP via same pilot (`lanes=16`, `element=I8`) — no spill/fill.
/// Fingerprint: `wasm-simd128-guest-v11:zero-scratch-simd16`
/// Fingerprint: `wasm-simd16-guest-v12:load-store-mvp`
fn emit_simd_load_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    lanes: u8,
    element: u8,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let f64_element = element == syx_stdlib::NumericKind::F64.to_byte();
    let f32_element = element == syx_stdlib::NumericKind::F32.to_byte();
    let i64_element = element == syx_stdlib::NumericKind::I64.to_byte();
    let i16_element = element == syx_stdlib::NumericKind::I16.to_byte();
    let i8_element = element == syx_stdlib::NumericKind::I8.to_byte();
    let lane_count = simd_guest_lane_count(lanes, element);
    let i32_tmp = tmp_local - 1;
    if simd_export_shim {
        body.instruction(&WasmInstr::Drop);
        body.instruction(&WasmInstr::I32Const(0));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadLocal)));
        body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreLocal)));
    } else {
        body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreLocal)));
    }
    emit_v128_splat_zero(body, f64_element, f32_element, i64_element, i16_element, i8_element);
    body.instruction(&WasmInstr::LocalSet(tmp_local));
    for lane in 0..lane_count {
        body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadLocal)));
        body.instruction(&WasmInstr::I32Const(lane as i32));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArrayGet)));
        body.instruction(&WasmInstr::Drop);
        if f64_element {
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            emit_take_rt_f64(body, plan);
            body.instruction(&WasmInstr::F64x2ReplaceLane(lane));
        } else if f32_element {
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            emit_take_rt_f32(body, plan);
            body.instruction(&WasmInstr::F32x4ReplaceLane(lane));
        } else if i64_element {
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            emit_take_rt_i64(body, plan);
            body.instruction(&WasmInstr::I64x2ReplaceLane(lane));
        } else if i16_element {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
            body.instruction(&WasmInstr::LocalSet(i32_tmp));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::LocalGet(i32_tmp));
            body.instruction(&WasmInstr::I16x8ReplaceLane(lane));
        } else if i8_element {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
            body.instruction(&WasmInstr::LocalSet(i32_tmp));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::LocalGet(i32_tmp));
            body.instruction(&WasmInstr::I8x16ReplaceLane(lane));
        } else {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
            body.instruction(&WasmInstr::LocalSet(i32_tmp));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::LocalGet(i32_tmp));
            body.instruction(&WasmInstr::I32x4ReplaceLane(lane));
        }
        body.instruction(&WasmInstr::LocalSet(tmp_local));
    }
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    emit_simd_guest_push_v128(body, plan, tmp_local, element);
    let _ = br_to_loop;
}

/// Phase 118 guest-v11: stack-native pop + extract lanes + ArraySet (no scratch pad).
/// Phase 129: i8x16 store MVP — leave updated array on RT stack (LoadLocal) for dest/chains.
/// Fingerprint: `wasm-simd16-guest-v12:load-store-mvp`
fn emit_simd_store_v128_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    lanes: u8,
    element: u8,
    br_to_loop: Option<u32>,
) {
    let f64_element = element == syx_stdlib::NumericKind::F64.to_byte();
    let f32_element = element == syx_stdlib::NumericKind::F32.to_byte();
    let i64_element = element == syx_stdlib::NumericKind::I64.to_byte();
    let i16_element = element == syx_stdlib::NumericKind::I16.to_byte();
    let i8_element = element == syx_stdlib::NumericKind::I8.to_byte();
    let lane_count = simd_guest_lane_count(lanes, element);
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalSet(tmp_local));
    body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreLocal)));
    for lane in 0..lane_count {
        body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadLocal)));
        body.instruction(&WasmInstr::I32Const(lane as i32));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
        body.instruction(&WasmInstr::LocalGet(tmp_local));
        if f64_element {
            body.instruction(&WasmInstr::F64x2ExtractLane(lane));
            emit_push_rt_f64(body, plan);
        } else if f32_element {
            body.instruction(&WasmInstr::F32x4ExtractLane(lane));
            emit_push_rt_f32(body, plan);
        } else if i64_element {
            body.instruction(&WasmInstr::I64x2ExtractLane(lane));
            emit_push_rt_i64(body, plan);
        } else if i16_element {
            body.instruction(&WasmInstr::I16x8ExtractLaneS(lane));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
        } else if i8_element {
            body.instruction(&WasmInstr::I8x16ExtractLaneS(lane));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
        } else {
            body.instruction(&WasmInstr::I32x4ExtractLane(lane));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
        }
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArraySet)));
        body.instruction(&WasmInstr::Drop);
        body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreLocal)));
    }
    // Return the updated array on the RT stack (dest / loadFromArray chains).
    body.instruction(&WasmInstr::I32Const(SIMD_LS_ARRAY_LOCAL));
    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadLocal)));
    let _ = br_to_loop;
}

/// Phase 118 guest-v11: horizontal reduceAdd via extract lanes + add (stack-native pop).
fn emit_simd_reduce_add_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    lanes: u8,
    element: u8,
    simd_export_shim: bool,
    br_to_loop: Option<u32>,
) {
    let _ = simd_export_shim;
    let f64_element = element == syx_stdlib::NumericKind::F64.to_byte();
    let f32_element = element == syx_stdlib::NumericKind::F32.to_byte();
    let i64_element = element == syx_stdlib::NumericKind::I64.to_byte();
    let i16_element = element == syx_stdlib::NumericKind::I16.to_byte();
    let i8_element = element == syx_stdlib::NumericKind::I8.to_byte();
    emit_simd_guest_pop_v128(body, plan);
    if f64_element {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        body.instruction(&WasmInstr::F64x2ExtractLane(0));
        body.instruction(&WasmInstr::LocalGet(tmp_local));
        body.instruction(&WasmInstr::F64x2ExtractLane(1));
        body.instruction(&WasmInstr::F64Add);
        emit_push_rt_f64(body, plan);
    } else if f32_element {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        let lane_count = lanes.max(1).min(4);
        body.instruction(&WasmInstr::F32x4ExtractLane(0));
        for lane in 1..lane_count {
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::F32x4ExtractLane(lane));
            body.instruction(&WasmInstr::F32Add);
        }
        emit_push_rt_f32(body, plan);
    } else if i64_element {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        body.instruction(&WasmInstr::I64x2ExtractLane(0));
        body.instruction(&WasmInstr::LocalGet(tmp_local));
        body.instruction(&WasmInstr::I64x2ExtractLane(1));
        body.instruction(&WasmInstr::I64Add);
        emit_push_rt_i64(body, plan);
    } else if i16_element {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        let lane_count = lanes.max(1).min(8);
        body.instruction(&WasmInstr::I16x8ExtractLaneS(0));
        for lane in 1..lane_count {
            body.instruction(&WasmInstr::LocalSet(tmp_local - 1));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::I16x8ExtractLaneS(lane));
            body.instruction(&WasmInstr::LocalGet(tmp_local - 1));
            body.instruction(&WasmInstr::I32Add);
        }
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
    } else if i8_element {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        let lane_count = lanes.max(1).min(16);
        body.instruction(&WasmInstr::I8x16ExtractLaneS(0));
        for lane in 1..lane_count {
            body.instruction(&WasmInstr::LocalSet(tmp_local - 1));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::I8x16ExtractLaneS(lane));
            body.instruction(&WasmInstr::LocalGet(tmp_local - 1));
            body.instruction(&WasmInstr::I32Add);
        }
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
    } else {
        body.instruction(&WasmInstr::LocalTee(tmp_local));
        let lane_count = lanes.max(1).min(4);
        body.instruction(&WasmInstr::I32x4ExtractLane(0));
        for lane in 1..lane_count {
            body.instruction(&WasmInstr::LocalSet(tmp_local - 1));
            body.instruction(&WasmInstr::LocalGet(tmp_local));
            body.instruction(&WasmInstr::I32x4ExtractLane(lane));
            body.instruction(&WasmInstr::LocalGet(tmp_local - 1));
            body.instruction(&WasmInstr::I32Add);
        }
        body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
    }
    let _ = br_to_loop;
}

/// Fixed 4-lane i32 shuffle modes (matches host `host_simd_shuffle`):
/// mode 0 = reverse, mode 1 = swap pairs, else identity.
/// Phase 117 guest-v10: stack-native shuffle (no spill/fill).
fn emit_simd_shuffle_i32x4_pilot(
    body: &mut Function,
    plan: &EmitPlan,
    _scratch_offset: i32,
    tmp_local: u32,
    mode: i32,
    br_to_loop: Option<u32>,
) {
    let lanes: [u8; 16] = match mode {
        0 => [
            12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3,
        ],
        1 => [
            4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11,
        ],
        _ => [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
    };
    emit_simd_guest_pop_v128(body, plan);
    body.instruction(&WasmInstr::LocalTee(tmp_local));
    body.instruction(&WasmInstr::LocalGet(tmp_local));
    body.instruction(&WasmInstr::I8x16Shuffle(lanes));
    emit_simd_guest_push_v128(
        body,
        plan,
        tmp_local,
        syx_stdlib::NumericKind::I32.to_byte(),
    );
    let _ = br_to_loop;
}

fn emit_mir_instr(
    body: &mut Function,
    instr: &syx_mir::MirInstr,
    module: &MirModule,
    plan: &EmitPlan,
    str_map: &HashMap<String, (i32, i32)>,
    rt_str_map: &HashMap<String, (i32, i32)>,
    block_pos: &HashMap<u32, usize>,
    current_block: usize,
    br_to_loop: u32,
    exit_returns_i32: bool,
    func_param_count: u32,
    scratch_offset: i32,
    simd128: bool,
    simd_tmp_local: Option<u32>,
    simd_export_shim: bool,
) {
    let _ = simd_tmp_local;
    if plan.aot {
        if let MirOp::ConstI32(v) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*v));
            body.instruction(&WasmInstr::Call(
                plan.aot_import(aot::Import::PushRtI32),
            ));
            return;
        }
        // Phase 126: protocol/class witness slots emit ConstFnIndex before ObjectSet.
        if let MirOp::ConstFnIndex(name) = &instr.op {
            let idx = module
                .functions
                .iter()
                .position(|f| f.name == *name)
                .unwrap_or(0) as i32;
            body.instruction(&WasmInstr::I32Const(idx));
            body.instruction(&WasmInstr::Call(
                plan.aot_import(aot::Import::PushRtI32),
            ));
            return;
        }
        if let MirOp::ConstBool(b) = &instr.op {
            // Wave A W203 / ADR-547: RT Bool for Print; leave guest i32 for JumpIfFalse.
            let n = if *b { 1 } else { 0 };
            body.instruction(&WasmInstr::I32Const(n));
            body.instruction(&WasmInstr::Call(
                plan.aot_import(aot::Import::PushRtBool),
            ));
            body.instruction(&WasmInstr::I32Const(n));
            return;
        }
        if matches!(instr.op, MirOp::ConstNil) {
            // Wave A W203 / ADR-547: RT Nil for Print; guest 0 for JumpIfNil.
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtNil)));
            body.instruction(&WasmInstr::I32Const(0));
            return;
        }
        if let MirOp::ConstNumeric { kind, bits } = &instr.op {
            emit_const_numeric_guest(body, plan, *kind, *bits);
            return;
        }
        // M0-W4: scalar binops run on RT stack (ConstNumeric leaves operands on RT).
        let stack_op = match &instr.op {
            MirOp::Add { .. } => Some(syx_mir_run::ADD),
            MirOp::Sub { .. } => Some(syx_mir_run::SUB),
            MirOp::Mul { .. } => Some(syx_mir_run::MUL),
            MirOp::Div { .. } => Some(syx_mir_run::DIV),
            MirOp::Mod { .. } => Some(syx_mir_run::MOD),
            MirOp::BitAnd { .. } => Some(syx_mir_run::BIT_AND),
            MirOp::BitOr { .. } => Some(syx_mir_run::BIT_OR),
            MirOp::BitXor { .. } => Some(syx_mir_run::BIT_XOR),
            MirOp::Shl { .. } => Some(syx_mir_run::SHL),
            MirOp::Shr { .. } => Some(syx_mir_run::SHR),
            MirOp::Eq => Some(syx_mir_run::EQ),
            MirOp::Ne => Some(syx_mir_run::NE),
            MirOp::Lt { .. } => Some(syx_mir_run::LT),
            MirOp::Le { .. } => Some(syx_mir_run::LE),
            MirOp::Gt { .. } => Some(syx_mir_run::GT),
            MirOp::Ge { .. } => Some(syx_mir_run::GE),
            MirOp::And => Some(syx_mir_run::AND),
            MirOp::Or => Some(syx_mir_run::OR),
            MirOp::Not => Some(syx_mir_run::NOT),
            MirOp::Neg { .. } => Some(syx_mir_run::NEG),
            MirOp::BitNot { .. } => Some(syx_mir_run::BIT_NOT),
            _ => None,
        };
        if let Some(op_code) = stack_op {
            body.instruction(&WasmInstr::I32Const(op_code as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StackOp)));
            // Compare / bool results stay on RT; JumpIfFalse takes via TakeRtI32.
            if matches!(
                instr.op,
                MirOp::Eq
                    | MirOp::Ne
                    | MirOp::Lt { .. }
                    | MirOp::Le { .. }
                    | MirOp::Gt { .. }
                    | MirOp::Ge { .. }
                    | MirOp::And
                    | MirOp::Or
                    | MirOp::Not
            ) {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
            }
            return;
        }
        if let MirOp::LoadLocal(slot) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*slot as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadLocal)));
            return;
        }
        if let MirOp::StoreLocal(slot) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*slot as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreLocal)));
            return;
        }
        if let MirOp::Return = &instr.op {
            if plan.aot && func_param_count > 0 {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::EndFn)));
            }
            if exit_returns_i32 && !plan.aot {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::GetReturn)));
            } else if exit_returns_i32 && plan.aot {
                // Wave A W18: aot main export needs an i32; RT may hold leftover values
                // (e.g. print args). Match CFG epilogue — wasm exit code is 0.
                body.instruction(&WasmInstr::I32Const(0));
            } else if !exit_returns_i32 {
                body.instruction(&WasmInstr::I32Const(0));
            }
            body.instruction(&WasmInstr::Return);
            return;
        }
        if let MirOp::Jump(target) = &instr.op {
            let t = *block_pos.get(target).unwrap_or(&current_block) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop));
            return;
        }
        if let MirOp::JumpIfFalse(target) = &instr.op {
            // Phase 100: guest control — condition already on wasm stack as i32.
            body.instruction(&WasmInstr::I32Eqz);
            body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
            let t = *block_pos.get(target).unwrap_or(&(current_block + 1)) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop + 1));
            body.instruction(&WasmInstr::End);
            return;
        }
        if let MirOp::CallHost(idx) = &instr.op {
            if let Some(&import_idx) = plan.host_import_map.get(idx) {
                body.instruction(&WasmInstr::Call(import_idx));
                emit_call_host_take_catch(body, br_to_loop);
            } else {
                // Phase 99: missing host import is an emit hard-fail (no mir_exec escape).
                aot_emit_hard_fail(&format!("CallHost({idx}) has no WASM host import"));
                body.instruction(&WasmInstr::Unreachable);
            }
            return;
        }
        if let MirOp::Call(name) = &instr.op {
            if let Some(idx) = module.functions.iter().position(|f| f.name == *name) {
                body.instruction(&WasmInstr::I32Const(idx as i32));
                body.instruction(&WasmInstr::CallIndirect {
                    type_index: host::TYPE_FUNC_I32,
                    table_index: 0,
                });
                // Wave A W18 / ADR-362: llvm W11 parity — poll pending catch after Call.
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
                emit_call_host_take_catch(body, br_to_loop);
                return;
            }
            aot_emit_hard_fail(&format!("Call({name}) has no module function"));
            body.instruction(&WasmInstr::Unreachable);
            return;
        }
        if let MirOp::LoadGlobal(slot) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*slot as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::LoadGlobal)));
            return;
        }
        if let MirOp::StoreGlobal(slot) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*slot as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::StoreGlobal)));
            return;
        }
        if let MirOp::ConstStr(s) = &instr.op {
            if let Some((off, len)) = str_map.get(s) {
                // Phase 100: data-section off/len + dedicated ConstStr import (not mir_exec).
                body.instruction(&WasmInstr::I32Const(*off));
                body.instruction(&WasmInstr::I32Const(*len));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ConstStr)));
                return;
            }
            aot_emit_hard_fail(&format!("ConstStr({s:?}) missing from wasm data section"));
            body.instruction(&WasmInstr::Unreachable);
            return;
        }
        if let MirOp::ArrayBuild(n) = &instr.op {
            if *n <= 16 {
                body.instruction(&WasmInstr::I32Const(*n as i32));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArrayBuild)));
                return;
            }
        }
        if let MirOp::DictBuild(n) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*n as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::DictBuild)));
            return;
        }
        if let MirOp::SetBuild(n) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*n as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SetBuild)));
            return;
        }
        if matches!(instr.op, MirOp::Print) {
            body.instruction(&WasmInstr::I32Const(0));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::Print)));
            return;
        }
        if matches!(instr.op, MirOp::MakeIterator) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::MakeIterator)));
            return;
        }
        if matches!(instr.op, MirOp::IteratorNext) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::IteratorNext)));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeRtI32)));
            return;
        }
        if matches!(instr.op, MirOp::ArrayGet) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArrayGet)));
            return;
        }
        if matches!(instr.op, MirOp::ArraySet) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArraySet)));
            return;
        }
        if matches!(instr.op, MirOp::ObjectGet) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ObjectGet)));
            return;
        }
        if matches!(instr.op, MirOp::ObjectSet) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ObjectSet)));
            return;
        }
        if matches!(instr.op, MirOp::ValueEq | MirOp::OpaqueValueEq) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ValueEq)));
            return;
        }
        if matches!(instr.op, MirOp::ValueNe) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ValueNe)));
            return;
        }
        if matches!(instr.op, MirOp::RefEq) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::RefEq)));
            return;
        }
        if matches!(instr.op, MirOp::RefNe) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::RefNe)));
            return;
        }
        if matches!(instr.op, MirOp::ArrSorted) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ArraySorted)));
            return;
        }
        if let MirOp::MakeClosure(name, cap) = &instr.op {
            let idx = module
                .functions
                .iter()
                .position(|f| f.name == *name)
                .unwrap_or(0);
            body.instruction(&WasmInstr::I32Const(idx as i32));
            body.instruction(&WasmInstr::I32Const(*cap as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::MakeClosure)));
            body.instruction(&WasmInstr::Drop);
            return;
        }
        if matches!(instr.op, MirOp::CallClosure) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallClosure)));
            return;
        }
        if matches!(instr.op, MirOp::AwaitPoll) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::AwaitPoll)));
            return;
        }
        if let MirOp::AwaitSuspend { resume_fn, state } = &instr.op {
            body.instruction(&WasmInstr::I32Const(*resume_fn as i32));
            body.instruction(&WasmInstr::I32Const(*state as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::AwaitSuspend)));
            return;
        }
        if let MirOp::CallWitness(a, b) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*a as i32));
            body.instruction(&WasmInstr::I32Const(*b as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallWitness)));
            return;
        }
        if let MirOp::Div { .. } = &instr.op {
            body.instruction(&WasmInstr::I32DivS);
            return;
        }
        if let MirOp::Mod { .. } = &instr.op {
            body.instruction(&WasmInstr::I32RemS);
            return;
        }
        if matches!(instr.op, MirOp::Le { .. }) {
            body.instruction(&WasmInstr::I32LeS);
            return;
        }
        if matches!(instr.op, MirOp::Ge { .. }) {
            body.instruction(&WasmInstr::I32GeS);
            return;
        }
        if matches!(instr.op, MirOp::Gt { .. }) {
            body.instruction(&WasmInstr::I32GtS);
            return;
        }
        if matches!(instr.op, MirOp::Eq) {
            body.instruction(&WasmInstr::I32Eq);
            return;
        }
        if matches!(instr.op, MirOp::Ne) {
            body.instruction(&WasmInstr::I32Ne);
            return;
        }
        if matches!(instr.op, MirOp::Lt { .. }) {
            body.instruction(&WasmInstr::I32LtS);
            return;
        }
        if matches!(instr.op, MirOp::And) {
            body.instruction(&WasmInstr::I32And);
            return;
        }
        if matches!(instr.op, MirOp::Or) {
            body.instruction(&WasmInstr::I32Or);
            return;
        }
        if matches!(instr.op, MirOp::Not) {
            body.instruction(&WasmInstr::I32Eqz);
            return;
        }
        if matches!(instr.op, MirOp::Neg { .. }) {
            body.instruction(&WasmInstr::I32Const(0));
            body.instruction(&WasmInstr::I32Sub);
            return;
        }
        if matches!(instr.op, MirOp::Pop) {
            body.instruction(&WasmInstr::Drop);
            return;
        }
        if let MirOp::JumpIfNil(target) = &instr.op {
            body.instruction(&WasmInstr::I32Eqz);
            body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
            let t = *block_pos.get(target).unwrap_or(&(current_block + 1)) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop + 1));
            body.instruction(&WasmInstr::End);
            return;
        }
        if let MirOp::DictBuild(n) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*n as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::DictBuild)));
            return;
        }
        if let MirOp::SetBuild(n) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*n as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SetBuild)));
            return;
        }
        // Wave A W544 / ADR-888: MirOp ObjectRefWrap/Retain/Release peeled
        // (ADR-858/ADR-855 AOT nops retired with the variants).
        if matches!(instr.op, MirOp::HeapCompact) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::HeapCompact)));
            return;
        }
        if matches!(instr.op, MirOp::SimdAdd) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAdd)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAdd)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdSub) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdSub)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdSub)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdMul) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_mul_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMul)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMul)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdDiv) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_div_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdDiv)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdDiv)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdEq) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_eq_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdEq)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdEq)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdLt) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_lt_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLt)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLt)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdGt) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_gt_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGt)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGt)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdAnd) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_and_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAnd)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAnd)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdOr) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_or_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdOr)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdOr)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdXor) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_xor_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdXor)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdXor)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdNe) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_ne_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNe)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNe)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdLe) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_le_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLe)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLe)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdGe) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_ge_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGe)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGe)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdNot) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_not_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNot)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNot)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdShl) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shl_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShl)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShl)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdShr) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shr_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShr)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShr)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdMin) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_min_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMin)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMin)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdMax) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_max_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMax)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMax)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF32Add) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Add)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Add)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF32Sub) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Sub)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Sub)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF32Mul) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_mul_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Mul)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Mul)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF32Div) {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_div_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Div)));
                }
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Div)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF64Add) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Add);
            } else if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Add);
                }
            } else if simd128 {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Add);
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Add)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF64Sub) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Sub);
            } else if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Sub);
                }
            } else if simd128 {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Sub);
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Sub)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF64Mul) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Mul);
            } else if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_mul_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Mul);
                }
            } else if simd128 {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Mul);
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Mul)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdF64Div) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Div);
            } else if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_div_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Div);
                }
            } else if simd128 {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdF64Div);
            } else {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Div)));
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI16Add) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Add);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Add);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Add);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI16Sub) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Sub);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Sub);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Sub);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI16Mul) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Mul);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_mul_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Mul);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI16Mul);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Add) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Add);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Add);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Add);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Sub) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Sub);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Sub);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Sub);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Mul) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Mul);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_mul_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Mul);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Mul);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8And) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8And);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_and_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8And);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8And);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Or) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Or);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_or_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Or);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Or);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Xor) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Xor);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_xor_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Xor);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Xor);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Not) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Not);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_not_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Not);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Not);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Min) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Min);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_min_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Min);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Min);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Max) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Max);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_max_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Max);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Max);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8UMin) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMin);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_umin_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMin);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMin);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8UMax) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMax);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_umax_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMax);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8UMax);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Abs) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Abs);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_abs_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Abs);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Abs);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Neg) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Neg);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_neg_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Neg);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Neg);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Eq) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Eq);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_eq_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Eq);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Eq);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Ne) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ne);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_ne_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ne);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ne);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Le) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Le);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_le_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Le);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Le);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Ge) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ge);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_ge_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ge);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Ge);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8AvgrU) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AvgrU);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_avgr_u_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AvgrU);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AvgrU);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Popcnt) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Popcnt);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_popcnt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Popcnt);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Popcnt);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Bitmask) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Bitmask);
            } else if simd128 && scratch_offset >= 0 {
                if simd_tmp_local.is_some() {
                    emit_simd_bitmask_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        simd_tmp_local.unwrap_or(0),
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Bitmask);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Bitmask);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8AllTrue) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AllTrue);
            } else if simd128 && scratch_offset >= 0 {
                if simd_tmp_local.is_some() {
                    emit_simd_all_true_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        simd_tmp_local.unwrap_or(0),
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AllTrue);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AllTrue);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8AnyTrue) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AnyTrue);
            } else if simd128 && scratch_offset >= 0 {
                if simd_tmp_local.is_some() {
                    emit_simd_any_true_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        simd_tmp_local.unwrap_or(0),
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AnyTrue);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AnyTrue);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Lt) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Lt);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_lt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Lt);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Lt);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Gt) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Gt);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_gt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Gt);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Gt);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8AddSat) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AddSat);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_sat_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AddSat);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8AddSat);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8SubSat) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8SubSat);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_sub_sat_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8SubSat);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8SubSat);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Extract) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Extract);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_extract_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Extract);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Extract);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Replace) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Replace);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_replace_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Replace);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Replace);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Shl) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shl);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shl_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shl);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shl);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI8Shr) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shr);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shr_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shr);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shr);
            }
            return;
        }
        if matches!(instr.op, MirOp::SimdI64Add) {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI64Add);
            } else if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_add_i64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI64Add);
                }
            } else {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI64Add);
            }
            return;
        }
        if let MirOp::SimdReduceAdd { lanes, element } = &instr.op {
            if simd_export_shim {
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdReduceAdd);
            } else if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_reduce_add_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *lanes,
                        *element,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    let idx = syx_stdlib::host_fn_index(syx_stdlib::HostFnId::SimdReduceAdd);
                    body.instruction(&WasmInstr::I32Const(idx as i32));
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallHost)));
                }
            } else {
                let idx = syx_stdlib::host_fn_index(syx_stdlib::HostFnId::SimdReduceAdd);
                body.instruction(&WasmInstr::I32Const(idx as i32));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallHost)));
            }
            return;
        }
        if let MirOp::SimdShuffle { mode } = &instr.op {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shuffle_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *mode,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::I32Const(*mode));
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShuffle)));
                }
            } else {
                body.instruction(&WasmInstr::I32Const(*mode));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShuffle)));
            }
            return;
        }
        if let MirOp::SimdI8Shuffle { mode } = &instr.op {
            if simd128 && scratch_offset >= 0 && !simd_export_shim {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_shuffle_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *mode,
                        catch_br(br_to_loop),
                    );
                } else {
                    body.instruction(&WasmInstr::I32Const(*mode));
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
                    emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shuffle);
                }
            } else {
                body.instruction(&WasmInstr::I32Const(*mode));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
                emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shuffle);
            }
            return;
        }
        if let MirOp::SimdLoad { lanes, element } = &instr.op {
            if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_load_v128_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *lanes,
                        *element,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                } else {
                    aot_emit_hard_fail("SimdLoad guest-v8 pilot requires simd tmp local");
                    body.instruction(&WasmInstr::Unreachable);
                }
            } else {
                let tag = (*lanes as i32) | ((*element as i32) << 8);
                body.instruction(&WasmInstr::I32Const(tag));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLoad)));
            }
            return;
        }
        if let MirOp::SimdStore { lanes, element } = &instr.op {
            if simd128 && scratch_offset >= 0 {
                if let Some(tmp) = simd_tmp_local {
                    emit_simd_store_v128_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *lanes,
                        *element,
                        catch_br(br_to_loop),
                    );
                } else {
                    aot_emit_hard_fail("SimdStore guest-v8 pilot requires simd tmp local");
                    body.instruction(&WasmInstr::Unreachable);
                }
            } else {
                let tag = (*lanes as i32) | ((*element as i32) << 8);
                body.instruction(&WasmInstr::I32Const(tag));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdStore)));
            }
            return;
        }
        if let MirOp::ActorCall(name) = &instr.op {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorCall)));
            // ADR-631: poll pending_catch after ActorCall (C009 under try).
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
            emit_call_host_take_catch(body, br_to_loop);
            return;
        }
        if let MirOp::ActorSend(name) = &instr.op {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorSend)));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
            emit_call_host_take_catch(body, br_to_loop);
            return;
        }
        if matches!(instr.op, MirOp::ActorRegister) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorRegister)));
            return;
        }
        if let MirOp::TaskSpawn(name) = &instr.op {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TaskSpawn)));
            return;
        }
        if matches!(instr.op, MirOp::TaskJoin) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TaskJoin)));
            return;
        }
        if let MirOp::BeginTry(catch) = &instr.op {
            body.instruction(&WasmInstr::I32Const(*catch as i32));
            body.instruction(&WasmInstr::I32Const(0));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::BeginTry)));
            return;
        }
        if matches!(instr.op, MirOp::Catch) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::Catch)));
            return;
        }
        if matches!(instr.op, MirOp::Throw) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::Throw)));
            return;
        }
        if matches!(instr.op, MirOp::EndTry) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::EndTry)));
            return;
        }
        if matches!(instr.op, MirOp::ConcatStr) {
            // Guest placeholders (ConstStr ret / LoadLocal 0) satisfy (i32,i32)->i32;
            // host pops RT [left,right] so concat is exact (e.g. "Hello, SYX!").
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ConcatStr)));
            return;
        }
        if matches!(instr.op, MirOp::ToStr) {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ToStr)));
            return;
        }
        if let MirOp::ConstNumeric128 { kind, lo, hi } = &instr.op {
            emit_const_numeric128_guest(body, plan, *kind, *lo, *hi);
            return;
        }
        // Phase 99: ban silent catch-all emit_mir_exec on sync AOT hot path.
        aot_emit_hard_fail(&format!("unmapped AOT sync opcode: {:?}", instr.op));
        body.instruction(&WasmInstr::Unreachable);
        return;
    }
    if simd128 && scratch_offset >= 0 && !simd_export_shim {
        if let Some(tmp) = simd_tmp_local {
            match instr.op {
                MirOp::SimdAdd => {
                    emit_simd_add_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdSub => {
                    emit_simd_sub_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdMul => {
                    emit_simd_mul_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdDiv => {
                    emit_simd_div_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdEq => {
                    emit_simd_eq_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdLt => {
                    emit_simd_lt_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdGt => {
                    emit_simd_gt_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdAnd => {
                    emit_simd_and_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdOr => {
                    emit_simd_or_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdXor => {
                    emit_simd_xor_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdNe => {
                    emit_simd_ne_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdLe => {
                    emit_simd_le_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdGe => {
                    emit_simd_ge_i32x4_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdNot => {
                    emit_simd_not_v128_pilot(body, plan, scratch_offset, tmp, simd_export_shim, catch_br(br_to_loop));
                    return;
                }
                MirOp::SimdShl => {
                    emit_simd_shl_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdShr => {
                    emit_simd_shr_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdMin => {
                    emit_simd_min_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdMax => {
                    emit_simd_max_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF32Add => {
                    emit_simd_add_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF32Sub => {
                    emit_simd_sub_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF32Mul => {
                    emit_simd_mul_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF32Div => {
                    emit_simd_div_f32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF64Add => {
                    emit_simd_add_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF64Sub => {
                    emit_simd_sub_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF64Mul => {
                    emit_simd_mul_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdF64Div => {
                    emit_simd_div_f64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI16Add => {
                    emit_simd_add_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI16Sub => {
                    emit_simd_sub_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI16Mul => {
                    emit_simd_mul_i16x8_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Add => {
                    emit_simd_add_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Sub => {
                    emit_simd_sub_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Mul => {
                    emit_simd_mul_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8And => {
                    emit_simd_and_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Or => {
                    emit_simd_or_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Xor => {
                    emit_simd_xor_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Not => {
                    emit_simd_not_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Min => {
                    emit_simd_min_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Max => {
                    emit_simd_max_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8UMin => {
                    emit_simd_umin_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8UMax => {
                    emit_simd_umax_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Abs => {
                    emit_simd_abs_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Neg => {
                    emit_simd_neg_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Eq => {
                    emit_simd_eq_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Ne => {
                    emit_simd_ne_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Le => {
                    emit_simd_le_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Ge => {
                    emit_simd_ge_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8AvgrU => {
                    emit_simd_avgr_u_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Popcnt => {
                    emit_simd_popcnt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Bitmask => {
                    emit_simd_bitmask_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8AllTrue => {
                    emit_simd_all_true_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8AnyTrue => {
                    emit_simd_any_true_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Lt => {
                    emit_simd_lt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Gt => {
                    emit_simd_gt_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8AddSat => {
                    emit_simd_add_sat_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8SubSat => {
                    emit_simd_sub_sat_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Shuffle { mode } => {
                    emit_simd_shuffle_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        mode,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Extract => {
                    emit_simd_extract_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Replace => {
                    emit_simd_replace_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Shl => {
                    emit_simd_shl_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI8Shr => {
                    emit_simd_shr_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdI64Add => {
                    emit_simd_add_i64x2_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdReduceAdd { lanes, element } => {
                    emit_simd_reduce_add_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        lanes,
                        element,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdShuffle { mode } => {
                    emit_simd_shuffle_i32x4_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        mode,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdLoad { lanes, element } => {
                    emit_simd_load_v128_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        lanes,
                        element,
                        simd_export_shim,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                MirOp::SimdStore { lanes, element } => {
                    emit_simd_store_v128_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        lanes,
                        element,
                        catch_br(br_to_loop),
                    );
                    return;
                }
                _ => {}
            }
        }
    }
    match &instr.op {
        MirOp::Jump(target) => {
            let t = *block_pos.get(target).unwrap_or(&current_block) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop));
        }
        MirOp::JumpIfFalse(target) => {
            // Phase 100: true guest — no POP_BOOL mir_exec.
            body.instruction(&WasmInstr::I32Eqz);
            body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
            let t = *block_pos.get(target).unwrap_or(&(current_block + 1)) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop + 1));
            body.instruction(&WasmInstr::End);
        }
        MirOp::JumpIfNil(target) => {
            // Phase 100: nil sentinel is i32 0 on the guest stack (no PEEK_IS_NIL / POP mir_exec).
            body.instruction(&WasmInstr::I32Eqz);
            body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
            let t = *block_pos.get(target).unwrap_or(&(current_block + 1)) as i32;
            body.instruction(&WasmInstr::I32Const(t));
            body.instruction(&WasmInstr::GlobalSet(0));
            body.instruction(&WasmInstr::Br(br_to_loop + 1));
            body.instruction(&WasmInstr::End);
        }
        MirOp::Pop => {
            body.instruction(&WasmInstr::Drop);
        }
        MirOp::Return => {
            if plan.aot && func_param_count > 0 {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::EndFn)));
            }
            if exit_returns_i32 && !plan.aot {
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::GetReturn)));
            } else if exit_returns_i32 && plan.aot {
                body.instruction(&WasmInstr::I32Const(0));
            } else if !exit_returns_i32 {
                body.instruction(&WasmInstr::I32Const(0));
            }
            body.instruction(&WasmInstr::Return);
        }
        MirOp::Call(name) => {
            if plan.aot {
                if let Some(idx) = module.functions.iter().position(|f| f.name == *name) {
                    body.instruction(&WasmInstr::I32Const(idx as i32));
                    body.instruction(&WasmInstr::CallIndirect {
                        type_index: host::TYPE_FUNC_I32,
                        table_index: 0,
                    });
                    body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
                    emit_call_host_take_catch(body, br_to_loop);
                    return;
                }
            }
            if let Some(idx) = module.functions.iter().position(|f| f.name == *name) {
                let param_count = module.functions[idx].param_count;
                body.instruction(&WasmInstr::I32Const(idx as i32));
                body.instruction(&WasmInstr::I32Const(param_count as i32));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallFn)));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
                emit_call_host_take_catch(body, br_to_loop);
            }
        }
        MirOp::CallHost(idx) => {
            body.instruction(&WasmInstr::I32Const(*idx as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallHost)));
            emit_call_host_take_catch(body, br_to_loop);
        }
        MirOp::ConstStr(s) => {
            if let Some((off, len)) = str_map.get(s) {
                body.instruction(&WasmInstr::I32Const(*off));
                body.instruction(&WasmInstr::I32Const(*len));
                body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ConstStr)));
            } else {
                aot_emit_hard_fail(&format!("ConstStr({s:?}) missing from wasm data section"));
                body.instruction(&WasmInstr::Unreachable);
            }
        }
        MirOp::ConcatStr => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ConcatStr)));
        }
        MirOp::MakeClosure(name, cap) => {
            let idx = module
                .functions
                .iter()
                .position(|f| f.name == *name)
                .unwrap_or(0);
            body.instruction(&WasmInstr::I32Const(idx as i32));
            body.instruction(&WasmInstr::I32Const(*cap as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::MakeClosure)));
            body.instruction(&WasmInstr::Drop);
        }
        MirOp::CallClosure => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallClosure)));
        }
        MirOp::ActorCall(name) => {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorCall)));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
            emit_call_host_take_catch(body, br_to_loop);
        }
        MirOp::ActorSend(name) => {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorSend)));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TakeCatchPc)));
            emit_call_host_take_catch(body, br_to_loop);
        }
        MirOp::ActorRegister => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::ActorRegister)));
        }
        MirOp::TaskSpawn(name) => {
            let (off, len) = rt_str_map.get(name.as_str()).copied().unwrap_or((0, 0));
            body.instruction(&WasmInstr::I32Const(off));
            body.instruction(&WasmInstr::I32Const(len));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TaskSpawn)));
        }
        MirOp::TaskJoin => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::TaskJoin)));
        }
        MirOp::ConstNumeric128 { kind, lo, hi } => {
            emit_const_numeric128_guest(body, plan, *kind, *lo, *hi);
        }
        MirOp::SimdAdd => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAdd)));
        }
        MirOp::SimdSub => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdSub)));
        }
        MirOp::SimdMul => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMul)));
        }
        MirOp::SimdDiv => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdDiv)));
        }
        MirOp::SimdEq => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdEq)));
        }
        MirOp::SimdLt => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLt)));
        }
        MirOp::SimdGt => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGt)));
        }
        MirOp::SimdAnd => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdAnd)));
        }
        MirOp::SimdOr => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdOr)));
        }
        MirOp::SimdXor => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdXor)));
        }
        MirOp::SimdNe => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNe)));
        }
        MirOp::SimdLe => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLe)));
        }
        MirOp::SimdGe => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdGe)));
        }
        MirOp::SimdNot => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdNot)));
        }
        MirOp::SimdShl => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShl)));
        }
        MirOp::SimdShr => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShr)));
        }
        MirOp::SimdMin => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMin)));
        }
        MirOp::SimdMax => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdMax)));
        }
        MirOp::SimdF32Add => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Add)));
        }
        MirOp::SimdF32Sub => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Sub)));
        }
        MirOp::SimdF32Mul => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Mul)));
        }
        MirOp::SimdF32Div => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF32Div)));
        }
        MirOp::SimdF64Add => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Add)));
        }
        MirOp::SimdF64Sub => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Sub)));
        }
        MirOp::SimdF64Mul => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Mul)));
        }
        MirOp::SimdF64Div => {
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdF64Div)));
        }
        MirOp::SimdReduceAdd { .. } => {
            let idx = syx_stdlib::host_fn_index(syx_stdlib::HostFnId::SimdReduceAdd);
            body.instruction(&WasmInstr::I32Const(idx as i32));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::CallHost)));
        }
        MirOp::SimdShuffle { mode } => {
            body.instruction(&WasmInstr::I32Const(*mode));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdShuffle)));
        }
        MirOp::SimdI8Shuffle { mode } => {
            if let Some(tmp) = simd_tmp_local {
                if simd128 && scratch_offset >= 0 && !simd_export_shim {
                    emit_simd_shuffle_i8x16_pilot(
                        body,
                        plan,
                        scratch_offset,
                        tmp,
                        *mode,
                        catch_br(br_to_loop),
                    );
                    return;
                }
            }
            body.instruction(&WasmInstr::I32Const(*mode));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::PushRtI32)));
            emit_simd_export_host_call(body, plan, syx_stdlib::HostFnId::SimdI8Shuffle);
        }
        MirOp::SimdLoad { lanes, element } => {
            let tag = (*lanes as i32) | ((*element as i32) << 8);
            body.instruction(&WasmInstr::I32Const(tag));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdLoad)));
        }
        MirOp::SimdStore { lanes, element } => {
            let tag = (*lanes as i32) | ((*element as i32) << 8);
            body.instruction(&WasmInstr::I32Const(tag));
            body.instruction(&WasmInstr::Call(plan.aot_import(aot::Import::SimdStore)));
        }
        other => {
            if syx_config::wasm_mir_exec_only() {
                let mut rt = syx_mir_run::MirRuntime::new();
                if !rt_str_map.is_empty() {
                    let (pool, _) = syx_mir_run::module_runtime_string_offsets(module);
                    rt.string_pool = pool;
                }
                let (o, a, b) = syx_mir_run::mir_op_immediates(other, &mut [], &mut rt);
                emit_mir_exec(body, o, a, b, false, catch_br(br_to_loop));
            } else {
                aot_emit_hard_fail(&format!(
                    "Phase 105: unmapped sync opcode (no mir_exec catch-all): {:?}",
                    instr.op
                ));
                body.instruction(&WasmInstr::Unreachable);
            }
        }
    }
}

/// Phase 100/104: control/string/actor/task/call/SIMD/i128 opcodes must never go through `syx.mir_exec`.
fn mir_exec_op_banned(op: u32) -> bool {
    matches!(
        op,
        op::POP_BOOL
            | op::PEEK_IS_NIL
            | op::POP
            | op::CONST_STR
            | op::CONCAT_STR
            | op::ACTOR_CALL
            | op::ACTOR_SEND
            | op::ACTOR_REGISTER
            | op::TASK_SPAWN
            | op::TASK_JOIN
            | op::CALL_FN
            | op::CALL_HOST
            | op::SET_FN_INDEX
            | op::CALL_CLOSURE
            | op::SIMD_WASM_SPILL
            | op::SIMD_WASM_FILL
            | op::SIMD_ADD
            | op::SIMD_SUB
            | op::SIMD_MUL
            | op::SIMD_DIV
            | op::SIMD_EQ
            | op::SIMD_LT
            | op::SIMD_GT
            | op::SIMD_NE
            | op::SIMD_LE
            | op::SIMD_GE
            | op::SIMD_AND
            | op::SIMD_OR
            | op::SIMD_XOR
            | op::SIMD_NOT
            | op::SIMD_SHL
            | op::SIMD_F32_ADD
            | op::SIMD_F32_SUB
            | op::SIMD_F32_MUL
            | op::SIMD_F32_DIV
            | op::SIMD_F64_ADD
            | op::SIMD_LOAD
            | op::SIMD_STORE
            | op::SIMD_SHUFFLE
            | op::SET_CONST128_LO
            | op::SET_CONST128_HI
            | op::CONST_NUMERIC128
    )
}

/// Phase 104: mir_exec residual allowlist is empty (`wasm-allowlist-empty-v1:phase104`).
fn mir_exec_op_allowlisted(_op: u32) -> bool {
    false
}

fn emit_mir_exec(
    body: &mut Function,
    opcode: u32,
    a: i32,
    b: i32,
    keep_result: bool,
    br_to_loop: Option<u32>,
) {
    if !syx_config::wasm_mir_exec_only() {
        aot_emit_hard_fail(&format!(
            "Phase 105: emit_mir_exec is debug-only (SYX_WASM_MIR_EXEC_ONLY); opcode {opcode}"
        ));
        body.instruction(&WasmInstr::Unreachable);
        return;
    }
    if mir_exec_op_banned(opcode) {
        aot_emit_hard_fail(&format!(
            "Phase 100: banned mir_exec opcode {opcode} (guest control/string only)"
        ));
        body.instruction(&WasmInstr::Unreachable);
        return;
    }
    // Phase 104: SIMD/i128 are guest-only; mir_exec allowlist is empty.
    if matches!(
        opcode,
        op::SIMD_WASM_SPILL
            | op::SIMD_WASM_FILL
            | op::SIMD_ADD
            | op::SIMD_SUB
            | op::SIMD_MUL
            | op::SET_CONST128_LO
            | op::SET_CONST128_HI
            | op::CONST_NUMERIC128
    ) && !mir_exec_op_allowlisted(opcode)
    {
        aot_emit_hard_fail(&format!(
            "Phase 104: mir_exec opcode {opcode} is guest-only (SIMD/i128 allowlist empty)"
        ));
        body.instruction(&WasmInstr::Unreachable);
        return;
    }
    body.instruction(&WasmInstr::I32Const(opcode as i32));
    body.instruction(&WasmInstr::I32Const(a));
    body.instruction(&WasmInstr::I32Const(b));
    body.instruction(&WasmInstr::Call(host::IMPORT_MIR_EXEC));
    if !keep_result && br_to_loop.is_none() {
        body.instruction(&WasmInstr::Drop);
        return;
    }
    body.instruction(&WasmInstr::LocalSet(0));
    body.instruction(&WasmInstr::LocalGet(0));
    body.instruction(&WasmInstr::I32Const(-2));
    body.instruction(&WasmInstr::I32LtS);
    body.instruction(&WasmInstr::If(wasm_encoder::BlockType::Empty));
    body.instruction(&WasmInstr::I32Const(-2));
    body.instruction(&WasmInstr::LocalGet(0));
    body.instruction(&WasmInstr::I32Sub);
    body.instruction(&WasmInstr::GlobalSet(0));
    if let Some(br) = br_to_loop {
        body.instruction(&WasmInstr::Br(br + 1));
    }
    body.instruction(&WasmInstr::End);
    if keep_result {
        body.instruction(&WasmInstr::LocalGet(0));
    }
}

fn mir_op_immediates(op: &MirOp) -> (u32, i32, i32) {
    let mut rt = syx_mir_run::MirRuntime::new();
    syx_mir_run::mir_op_immediates(op, &mut [], &mut rt)
}

/// Emit WebAssembly text (WAT) for a MIR module by printing the binary module.
pub fn emit_wat(module: &MirModule) -> SyxResult<String> {
    let bytes = emit_wasm(module)?;
    Ok(wasmprinter::print_bytes(&bytes).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("WAT print failed: {e}"))
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_mir::{MirBlock, MirInstr, MirModule};

    #[test]
    fn emitted_module_is_valid_wasm() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        MirInstr {
                            dest: Some(0),
                            op: MirOp::ConstI32(42),
                            span: None,
                        },
                        MirInstr {
                            dest: None,
                            op: MirOp::Return,
                            span: None,
                        },
                    ],
                    successors: vec![],
                }],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
};
        validate_wasm(&emit_wasm(&module).unwrap());
    }

    #[test]
    fn emit_wat_from_hello_mir() {
        use syx_diagnostics::SourceFile;
        use syx_mir::lower_module;
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            "func main() { print(\"Hello\") }",
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let wat = emit_wat(&mir).unwrap();
        assert!(wat.contains("module"));
        validate_wasm(&emit_wasm(&mir).unwrap());
    }

    #[test]
    fn enum_assoc_emits_valid_wasm() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_enum_shells, lower_module, merge_mir_functions, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            include_str!("../../syx-driver/tests/fixtures/lang/enum_associated.sy"),
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        merge_mir_functions(&mut mir, lower_enum_shells(&module));
        optimize(&mut mir);
        validate_wasm(&emit_wasm(&mir).unwrap());
    }

    #[test]
    fn cfg_loop_emits_valid_wasm() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 1,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(1),
                                span: None,
                            },
                        ],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![
                            MirInstr {
                                dest: Some(0),
                                op: MirOp::ConstI32(1),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::JumpIfFalse(2),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Print,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(1),
                                span: None,
                            },
                        ],
                        successors: vec![2, 1],
                    },
                    MirBlock {
                        id: 2,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::Return,
                                span: None,
                            },
                        ],
                        successors: vec![],
                    },
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
};
        validate_wasm(&emit_wasm(&module).unwrap());
    }

    fn validate_wasm(wasm: &[u8]) {
        let mut validator =
            wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all());
        for payload in wasmparser::Parser::new(0).parse_all(wasm) {
            let payload = payload.expect("wasm parse error");
            validator.payload(&payload).expect("wasm validation error");
        }
    }

    #[test]
    fn p104_mir_exec_allowlist_empty() {
        assert!(!mir_exec_op_allowlisted(op::SIMD_ADD));
        assert!(!mir_exec_op_allowlisted(op::CONST_NUMERIC128));
        for fp in crate::exec::PHASE104_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
        for fp in crate::exec::PHASE105_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
        for fp in crate::exec::PHASE106_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
        for fp in crate::exec::PHASE107_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
        for fp in crate::exec::PHASE108_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
        for fp in crate::exec::PHASE109_TRACK_B_WASM_FINGERPRINTS {
            assert!(include_str!("exec.rs").contains(fp));
        }
    }

    #[test]
    fn try_catch_emits_valid_wasm() {
        let catch_block = 2u32;
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 1,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::BeginTry(catch_block),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::ConstStr("boom".into()),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Throw,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::EndTry,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(3),
                                span: None,
                            },
                        ],
                        successors: vec![3],
                    },
                    MirBlock {
                        id: catch_block,
                        instructions: vec![
                            MirInstr {
                                dest: Some(0),
                                op: MirOp::Catch,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::ConstStr("caught".into()),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Print,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(3),
                                span: None,
                            },
                        ],
                        successors: vec![3],
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
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
};
        validate_wasm(&emit_wasm(&module).unwrap());
        let wasm = emit_wasm(&module).unwrap();
        syx_wasm_run::run_wasm_with_module(&wasm, "main", Some(&module)).expect("run");
    }
}
