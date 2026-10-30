use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::{MirFunction, MirOp};
use syx_runtime_core::{
    display_numeric, numeric_cmp, platform_int_value, simd_abs, simd_all_true, simd_and,
    simd_any_true, simd_avgr_u, simd_binop, simd_bitmask, simd_eq, simd_ge, simd_gt, simd_le,
    simd_load_from_array, simd_lt, simd_max, simd_min, simd_ne, simd_neg, simd_not, simd_or,
    simd_popcnt, simd_reduce_add, simd_shl, simd_shr, simd_store_to_array, simd_umax, simd_umin,
    simd_xor, try_numeric_cast, value_debug_description, value_description, value_equals,
    value_from_numeric, value_from_numeric128, value_identical, value_kind, IteratorState,
    SimdBinOp, Value, VmRuntime,
};
use syx_stdlib::{method_from_index, HostFnId, NumericKind, PLATFORM_INT};
use syx_vm::{
    call_receiver_method, cooperative_yield, host_fn_for, numeric_add, numeric_bitand,
    numeric_bitnot, numeric_bitor, numeric_bitxor, numeric_div, numeric_mod, numeric_mul,
    numeric_neg, numeric_shl, numeric_shr, numeric_sub,
};

use crate::op;

fn diagnostic_to_catch_value(err: SyxDiagnostic) -> Value {
    let mut map = HashMap::new();
    map.insert("message".into(), Value::Str(err.message));
    if let Some(code) = err.code {
        map.insert("code".into(), Value::Str(code));
    }
    Value::Object(map)
}

static NEXT_ACTOR_ID: AtomicI64 = AtomicI64::new(1);

#[derive(Debug, Clone)]
pub struct MirSuspendFrame {
    pub pc: usize,
    pub locals: Vec<Value>,
    pub stack: Vec<Value>,
    pub resume_fn: u32,
    pub state: u32,
}

pub struct MirRuntime {
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    /// Phase 98: caller locals frames for LLVM Call/Return ABI.
    pub locals_frames: Vec<Vec<Value>>,
    pub globals: Vec<Value>,
    pub host_fn_names: Vec<String>,
    pub functions: Vec<MirFunction>,
    pub fn_name_to_index: HashMap<String, usize>,
    pub runtime: Option<VmRuntime>,
    pub string_pool: Vec<u8>,
    pub pending_catch: Option<Value>,
    pub try_handlers: Vec<u32>,
    /// Wave A W6 / ADR-350: `try_handlers.len()` at each function entry so a
    /// callee rethrow cannot pop the caller's BeginTry landing pad.
    pub try_handler_frames: Vec<usize>,
    pub printed: Vec<String>,
    pub block_pos_by_fn: Vec<HashMap<u32, usize>>,
    pub current_fn: usize,
    /// Phase 14: SSA virtual registers keyed by `MirInstr.dest`.
    pub ssa_regs: Vec<Value>,
    /// Phase 39: low/high halves for pending `ConstNumeric128` literal.
    pub pending_const128_lo: u64,
    pub pending_const128_hi: u64,
    /// Phase 74: managed heap slot indices mirrored from ObjectRef wraps.
    pub heap_slots: Vec<usize>,
    /// Phase 87: native async suspend/resume driver state.
    pub suspended: bool,
    pub async_resume_fn: u32,
    pub async_state: u32,
    pub suspend_frame: Option<MirSuspendFrame>,
    /// Phase 111: element kind for last guest simd128 spill/fill (f32 pilot).
    pub simd_guest_element: Option<syx_stdlib::NumericKind>,
}

impl Default for MirRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl MirRuntime {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
            locals_frames: Vec::new(),
            globals: Vec::new(),
            host_fn_names: syx_stdlib::all_host_fn_names()
                .into_iter()
                .map(String::from)
                .collect(),
            functions: Vec::new(),
            fn_name_to_index: HashMap::new(),
            runtime: None,
            string_pool: Vec::new(),
            pending_catch: None,
            try_handlers: Vec::new(),
            try_handler_frames: Vec::new(),
            printed: Vec::new(),
            block_pos_by_fn: Vec::new(),
            current_fn: 0,
            ssa_regs: Vec::new(),
            pending_const128_lo: 0,
            pending_const128_hi: 0,
            heap_slots: Vec::new(),
            suspended: false,
            async_resume_fn: 0,
            async_state: 0,
            suspend_frame: None,
            simd_guest_element: None,
        }
    }

    /// Wave A W487 / ADR-831 + W489 / ADR-833: gather managed-heap roots for collect/compact.
    ///
    /// Roots include stack / locals / globals / ssa_regs / heap_slots, plus
    /// async `suspend_frame`, caller `locals_frames`, and actor mailbox Value
    /// queues (`syx_vm::actor_mailbox`). Does **not** claim unified-heap enable.
    pub fn collect_live_roots(&self) -> Vec<Value> {
        let mut roots = self.stack.clone();
        roots.extend(self.locals.clone());
        roots.extend(self.globals.clone());
        roots.extend(self.ssa_regs.clone());
        for frame in &self.locals_frames {
            roots.extend(frame.iter().cloned());
        }
        if let Some(frame) = &self.suspend_frame {
            roots.extend(frame.stack.iter().cloned());
            roots.extend(frame.locals.iter().cloned());
        }
        for slot in &self.heap_slots {
            if let Ok(heap) = syx_runtime_core::global_heap().lock() {
                if let Some(v) = heap.get(*slot) {
                    roots.push(v.clone());
                }
            }
        }
        // Wave A W489 / ADR-833: shared VM actor mailbox Value queues.
        roots.extend(syx_vm::actor_mailbox::mailbox_collect_roots());
        roots
    }

    /// Wave A W550 / ADR-894: ObjectRef re-entry `maybe_track_objectref_*`
    /// residual peeled (CallHost/ObjectGet/Dup/LoadLocal/…). MutRefCell
    /// `maybe_track_*` kept. ≠ migrate / ≠ cleanup / ≠ B-09 `[x]`.

    /// Wave A W525 / ADR-869: StoreLocal ObjectRef dual-track death-path —
    /// prune matching `heap_slots` + `ManagedHeap::free_slot` (VM parity).
    /// Keeps ObjectRef RC + Type_deinit. ≠ RC retire / ≠ deinit strip /
    /// ≠ migrate_enabled / ≠ B-09 `[x]`.
    fn maybe_untrack_objectref_value(&mut self, v: &Value) {
        if !v.is_object_ref() {
            return;
        }
        if !syx_runtime_core::ManagedHeap::enabled() {
            return;
        }
        let Some(id) = Value::object_ref_identity(v) else {
            return;
        };
        if let Ok(mut heap) = syx_runtime_core::global_heap().lock() {
            let mut remaining = Vec::with_capacity(self.heap_slots.len());
            for s in self.heap_slots.drain(..) {
                if heap.get(s).and_then(Value::object_ref_identity) == Some(id) {
                    heap.free_slot(s);
                } else {
                    remaining.push(s);
                }
            }
            self.heap_slots = remaining;
        }
    }

    // Wave A W553 / ADR-897: MutRefCell dual-register track helpers peeled
    // (true-delete). ObjectRef untrack retained. ≠ migrate flip ≠ enable ≠ B-09 `[x]`.

    /// Wave A W487 / ADR-831 + W490 / ADR-834 + W491 / ADR-835: managed heap
    /// roots for mark-sweep.
    ///
    /// MirRuntime is **ManagedHeap-only** (`collect_global`). W490 dual-heap
    /// first-slice aligned VM primary path; W491 retires VM MemoryPool
    /// mark-sweep from `run_gc_collect` so GcCollect path is
    /// `vm_mir_runtime_heap_fully_converged` (narrow) — **not**
    /// `unified_mir_runtime_heap_enabled` / B-09 `[x]`.
    /// Wave A W527 / ADR-871: swept ObjectRef → Type_deinit (skip weak-dead).
    /// StoreLocal / run_class_deinits ObjectRef RC still live.
    pub fn heap_collect(&mut self) {
        let roots = self.collect_live_roots();
        let swept = syx_runtime_core::collect_global(&roots);
        let mut memory: [u8; 0] = [];
        self.run_sweep_objectref_type_deinits(swept, &mut memory);
    }

    fn run_sweep_objectref_type_deinits(&mut self, swept: Vec<Value>, memory: &mut [u8]) {
        for v in swept {
            if Value::object_ref_is_weak_dead(&v) {
                continue;
            }
            let _ = self.run_deinit_for_value(&v, memory);
        }
    }

    /// Phase 75 ADR-051 + W487 / ADR-831: collect + compact + rewrite live root slot indices.
    pub fn heap_compact(&mut self) {
        let mut roots = self.collect_live_roots();
        // Compact path also tracks HeapSlot indices as roots (rewrite targets).
        for &slot in &self.heap_slots {
            roots.push(Value::HeapSlot {
                idx: slot,
                kind: syx_runtime_core::HeapSlotKind::Object,
            });
        }
        let (swept, forwarding) = if let Ok(mut h) = syx_runtime_core::global_heap().lock() {
            let swept = h.collect(&roots);
            let forwarding = h.compact();
            (swept, forwarding)
        } else {
            return;
        };
        syx_runtime_core::ManagedHeap::rewrite_slot_indices(&mut roots, &forwarding);
        Self::sync_roots_from_rewrite(&mut self.stack, &forwarding);
        Self::sync_roots_from_rewrite(&mut self.locals, &forwarding);
        Self::sync_roots_from_rewrite(&mut self.globals, &forwarding);
        Self::sync_roots_from_rewrite(&mut self.ssa_regs, &forwarding);
        for frame in &mut self.locals_frames {
            Self::sync_roots_from_rewrite(frame, &forwarding);
        }
        if let Some(frame) = &mut self.suspend_frame {
            Self::sync_roots_from_rewrite(&mut frame.stack, &forwarding);
            Self::sync_roots_from_rewrite(&mut frame.locals, &forwarding);
        }
        for slot in &mut self.heap_slots {
            if let Some(new_idx) = forwarding
                .iter()
                .find_map(|&(old, new)| (*slot == old).then_some(new))
            {
                *slot = new_idx;
            }
        }
        // Wave A W489 / ADR-833: rewrite in-flight mailbox Value queues.
        syx_vm::actor_mailbox::mailbox_rewrite_slots(&forwarding);
        let mut memory: [u8; 0] = [];
        self.run_sweep_objectref_type_deinits(swept, &mut memory);
    }

    fn sync_roots_from_rewrite(target: &mut [Value], forwarding: &[(usize, usize)]) {
        syx_runtime_core::ManagedHeap::rewrite_slot_indices(target, forwarding);
    }

    fn ensure_ssa_reg(&mut self, idx: u32) {
        let i = idx as usize;
        if self.ssa_regs.len() <= i {
            self.ssa_regs.resize(i + 1, Value::Nil);
        }
    }

    fn set_ssa_reg(&mut self, idx: u32, v: Value) {
        self.ensure_ssa_reg(idx);
        self.ssa_regs[idx as usize] = v;
    }

    fn get_ssa_reg(&self, idx: u32) -> Value {
        self.ssa_regs
            .get(idx as usize)
            .cloned()
            .unwrap_or(Value::Nil)
    }

    fn is_ssa_binop(op: &MirOp) -> bool {
        matches!(
            op,
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
                | MirOp::Eq
                | MirOp::Ne
                | MirOp::Lt { .. }
                | MirOp::Le { .. }
                | MirOp::Gt { .. }
                | MirOp::Ge { .. }
                | MirOp::And
                | MirOp::Or
                | MirOp::ValueEq
                | MirOp::ValueNe
                | MirOp::RefEq
                | MirOp::RefNe
        )
    }

    /// Phase 127: SIMD lane binops always take operands from SSA `dest`/`dest+1`
    /// (MIR lowers args into those regs). Do not trust stack depth — ArrayBuild /
    /// nested calls may leave scalar junk below the intended operands.
    fn is_ssa_simd_binop(op: &MirOp) -> bool {
        matches!(
            op,
            MirOp::SimdAnd
                | MirOp::SimdOr
                | MirOp::SimdXor
                | MirOp::SimdShl
                | MirOp::SimdShr
                | MirOp::SimdMin
                | MirOp::SimdMax
                | MirOp::SimdF32Add
                | MirOp::SimdF32Sub
                | MirOp::SimdF32Mul
                | MirOp::SimdF32Div
                | MirOp::SimdF64Add
                | MirOp::SimdF64Sub
                | MirOp::SimdF64Mul
                | MirOp::SimdF64Div
                | MirOp::SimdI16Add
                | MirOp::SimdI16Sub
                | MirOp::SimdI16Mul
                | MirOp::SimdI8Add
                | MirOp::SimdI8Sub
                | MirOp::SimdI8Mul
                | MirOp::SimdI8And
                | MirOp::SimdI8Or
                | MirOp::SimdI8Xor
                | MirOp::SimdI8Min
                | MirOp::SimdI8Max
                | MirOp::SimdI8UMin
                | MirOp::SimdI8UMax
                | MirOp::SimdI8Eq
                | MirOp::SimdI8Ne
                | MirOp::SimdI8Le
                | MirOp::SimdI8Ge
                | MirOp::SimdI8AvgrU
                | MirOp::SimdI8Lt
                | MirOp::SimdI8Gt
                | MirOp::SimdI8AddSat
                | MirOp::SimdI8SubSat
                | MirOp::SimdI8Shl
                | MirOp::SimdI8Shr
                | MirOp::SimdI64Add
                | MirOp::SimdAdd
                | MirOp::SimdSub
                | MirOp::SimdMul
                | MirOp::SimdDiv
                | MirOp::SimdEq
                | MirOp::SimdLt
                | MirOp::SimdGt
        )
    }

    fn is_ssa_unary_op(op: &MirOp) -> bool {
        matches!(
            op,
            MirOp::Neg { .. } | MirOp::Not | MirOp::BitNot { .. } | MirOp::NumericCast { .. }
        )
    }

    fn exec_mir_instr(&mut self, instr: &syx_mir::MirInstr, memory: &mut [u8]) -> SyxResult<()> {
        let dest = instr.dest;
        let op = &instr.op;
        if let Some(d) = dest {
            match op {
                MirOp::LoadLocal(slot) => {
                    // Wave A W517 / ADR-861: SSA LoadLocal push → dual-track
                    // ManagedHeap + heap_slots. No retain call (bytecode/VM
                    // W510 floor unchanged). ≠ migrate / cleanup / enable.
                    let v = self
                        .locals
                        .get(*slot as usize)
                        .cloned()
                        .unwrap_or(Value::Nil);
                    self.set_ssa_reg(d, v.clone());
                    self.stack.push(v);
                    return Ok(());
                }
                MirOp::LoadGlobal(slot) => {
                    // Wave A W516 / ADR-860: SSA LoadGlobal push → dual-track
                    // ManagedHeap + heap_slots. No retain call.
                    // ≠ migrate / cleanup / enable.
                    let v = self
                        .globals
                        .get(*slot as usize)
                        .cloned()
                        .unwrap_or(Value::Nil);
                    self.set_ssa_reg(d, v.clone());
                    self.stack.push(v);
                    return Ok(());
                }
                MirOp::ConstI32(v) => {
                    let val = Value::I32(*v);
                    self.set_ssa_reg(d, val.clone());
                    self.stack.push(val);
                    return Ok(());
                }
                MirOp::ConstNumeric { kind, bits } => {
                    let nk = syx_stdlib::NumericKind::from_byte(*kind)
                        .unwrap_or(syx_stdlib::PLATFORM_INT);
                    let val = value_from_numeric(nk, *bits);
                    self.set_ssa_reg(d, val.clone());
                    self.stack.push(val);
                    return Ok(());
                }
                MirOp::ConstStr(s) => {
                    let val = Value::Str(s.clone());
                    self.set_ssa_reg(d, val.clone());
                    self.stack.push(val);
                    return Ok(());
                }
                MirOp::ConstBool(v) => {
                    let val = Value::Bool(*v);
                    self.set_ssa_reg(d, val.clone());
                    self.stack.push(val);
                    return Ok(());
                }
                MirOp::ConstNil => {
                    self.set_ssa_reg(d, Value::Nil);
                    self.stack.push(Value::Nil);
                    return Ok(());
                }
                _ if Self::is_ssa_binop(op) => {
                    if self.stack.len() < 2 {
                        let a = self.get_ssa_reg(d);
                        let b = self.get_ssa_reg(d + 1);
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    self.exec_mir_op(op, memory)?;
                    let result = self.stack.pop().unwrap_or(Value::Nil);
                    self.set_ssa_reg(d, result.clone());
                    self.stack.push(result);
                    return Ok(());
                }
                _ if Self::is_ssa_simd_binop(op) => {
                    let a = self.get_ssa_reg(d);
                    let b = self.get_ssa_reg(d + 1);
                    let restore = self.stack.len();
                    self.stack.push(a);
                    self.stack.push(b);
                    self.exec_mir_op(op, memory)?;
                    let result = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.truncate(restore);
                    self.set_ssa_reg(d, result.clone());
                    self.stack.push(result);
                    return Ok(());
                }
                _ if Self::is_ssa_unary_op(op) => {
                    // Same-dest fold: drop prior push of this dest (ConstNumeric/LoadLocal)
                    // before re-pushing from SSA so ArrayBuild lane counts stay exact.
                    if !self.stack.is_empty() {
                        self.stack.pop();
                    }
                    self.stack.push(self.get_ssa_reg(d));
                    self.exec_mir_op(op, memory)?;
                    let result = self.stack.pop().unwrap_or(Value::Nil);
                    self.set_ssa_reg(d, result.clone());
                    self.stack.push(result);
                    return Ok(());
                }
                _ => {
                    self.exec_mir_op(op, memory)?;
                    if let Some(v) = self.stack.last().cloned() {
                        self.set_ssa_reg(d, v);
                    }
                    return Ok(());
                }
            }
        }
        self.exec_mir_op(op, memory)
    }

    pub fn install_functions(&mut self, functions: Vec<MirFunction>) {
        self.block_pos_by_fn = functions
            .iter()
            .map(|f| {
                f.blocks
                    .iter()
                    .enumerate()
                    .map(|(i, b)| (b.id, i))
                    .collect()
            })
            .collect();
        self.fn_name_to_index = functions
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.to_string(), i))
            .collect();
        self.functions = functions;
    }

    pub fn set_runtime(&mut self, runtime: VmRuntime) {
        self.runtime = Some(runtime);
    }

    pub fn intern_string(&mut self, s: &str) -> (i32, i32) {
        let offset = self.string_pool.len() as i32;
        self.string_pool.extend_from_slice(s.as_bytes());
        (offset, s.len() as i32)
    }

    pub fn begin_function(&mut self, local_count: u32) {
        self.try_handler_frames.push(self.try_handlers.len());
        self.locals.resize(local_count as usize, Value::Nil);
        self.ssa_regs.clear();
    }

    /// Pop `param_count` call arguments from the value stack into locals, then resize locals.
    /// Phase 98: push caller locals onto `locals_frames` before clobbering (LLVM Call/Return ABI).
    pub fn begin_function_from_call_stack(&mut self, param_count: u32, local_count: u32) {
        let saved = std::mem::take(&mut self.locals);
        self.locals_frames.push(saved);
        self.begin_function(local_count);
        if param_count == 0 {
            return;
        }
        let n = param_count as usize;
        let start = self.stack.len().saturating_sub(n);
        let args: Vec<Value> = self.stack.drain(start..).collect();
        for (i, v) in args.into_iter().enumerate() {
            if i < self.locals.len() {
                self.locals[i] = v;
            }
        }
    }

    /// Phase 98: restore caller locals and publish callee return value onto the MIR stack.
    pub fn end_function(&mut self) {
        let result = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(saved) = self.locals_frames.pop() {
            self.locals = saved;
        }
        if let Some(base) = self.try_handler_frames.pop() {
            self.try_handlers.truncate(base);
        }
        self.stack.push(result);
    }

    pub fn take_catch_pc(&mut self) -> i32 {
        if let Some(err) = self.pending_catch.take() {
            let base = self.try_handler_frames.last().copied().unwrap_or(0);
            if self.try_handlers.len() > base {
                let catch_block = self.try_handlers.pop().unwrap();
                self.stack.push(err);
                let pc = self
                    .block_pos_by_fn
                    .get(self.current_fn)
                    .and_then(|m| m.get(&catch_block))
                    .copied()
                    .unwrap_or(catch_block as usize);
                return pc as i32;
            }
            self.pending_catch = Some(err);
        }
        -1
    }

    pub fn take_catch(&mut self) -> i32 {
        // Wave A W6 / ADR-350: only pop handlers pushed in this function frame so a
        // catch-block rethrow propagates to the caller instead of stealing its pad.
        if let Some(err) = self.pending_catch.take() {
            let base = self.try_handler_frames.last().copied().unwrap_or(0);
            if self.try_handlers.len() > base {
                let catch_block = self.try_handlers.pop().unwrap();
                self.stack.push(err);
                return catch_block as i32;
            }
            self.pending_catch = Some(err);
        }
        -1
    }

    fn pop_try_handler_frame(&mut self) {
        if let Some(base) = self.try_handler_frames.pop() {
            self.try_handlers.truncate(base);
        }
    }

    pub fn exec(
        &mut self,
        op_code: u32,
        a: i32,
        b: i32,
        _c: i32,
        memory: &mut [u8],
    ) -> SyxResult<i32> {
        let mut pool_owned: Vec<u8>;
        let pool: &mut [u8] = if memory.is_empty() {
            pool_owned = self.string_pool.clone();
            &mut pool_owned
        } else {
            memory
        };
        match op_code {
            op::BEGIN_FN => {
                self.begin_function_from_call_stack(a as u32, b as u32);
                return Ok(0);
            }
            op::SET_FN_INDEX => {
                self.current_fn = a.max(0) as usize;
                return Ok(0);
            }
            op::GET_RETURN => return Ok(self.return_value()),
            op::POP_BOOL => return Ok(i32::from(self.pop_bool())),
            op::PEEK_IS_NIL => {
                let is_nil = self
                    .stack
                    .last()
                    .map(|v| matches!(v, Value::Nil))
                    .unwrap_or(true);
                return Ok(i32::from(is_nil));
            }
            op::CALL_FN => {
                self.call_fn(a as usize, &mut [])?;
                return Ok(0);
            }
            op::CONST_I32 => self.stack.push(Value::I32(a)),
            op::CONST_NUMERIC => {
                let v = syx_stdlib::NumericKind::from_byte(a as u8)
                    .map(|k| value_from_numeric(k, b as u64))
                    .unwrap_or(Value::I32(0));
                self.stack.push(v);
            }
            op::SET_CONST128_LO => {
                self.pending_const128_lo = (a as u64 & 0xffff_ffff) | ((b as u64) << 32);
            }
            op::SET_CONST128_HI => {
                self.pending_const128_hi = (a as u64 & 0xffff_ffff) | ((b as u64) << 32);
            }
            op::CONST_NUMERIC128 => {
                let lo = self.pending_const128_lo;
                let hi = self.pending_const128_hi;
                self.pending_const128_lo = 0;
                self.pending_const128_hi = 0;
                if let Some(k) = syx_stdlib::NumericKind::from_byte(a as u8) {
                    self.stack.push(value_from_numeric128(k, lo, hi));
                } else {
                    self.stack.push(Value::Nil);
                }
            }
            op::SIMD_ADD | op::SIMD_SUB | op::SIMD_MUL | op::SIMD_DIV => {
                self.exec_simd_stack_op(op_code)?;
            }
            op::SIMD_EQ => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_eq(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_LT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_lt(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_GT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_gt(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_AND => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_and(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_OR => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_or(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_XOR => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_xor(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_NE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_ne(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_LE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_le(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_GE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_ge(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_NOT => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_not(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_SHL => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_shl(&v, amount) {
                    self.stack.push(r);
                }
            }
            op::SIMD_SHR => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_shr(&v, amount) {
                    self.stack.push(r);
                }
            }
            op::SIMD_MIN => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_min(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_MAX => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_max(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_F32_ADD => {
                self.exec_simd_stack_op(op::SIMD_F32_ADD)?;
            }
            op::SIMD_F32_SUB => {
                self.exec_simd_stack_op(op::SIMD_F32_SUB)?;
            }
            op::SIMD_F32_MUL => {
                self.exec_simd_stack_op(op::SIMD_F32_MUL)?;
            }
            op::SIMD_F32_DIV => {
                self.exec_simd_stack_op(op::SIMD_F32_DIV)?;
            }
            op::SIMD_F64_ADD => {
                self.exec_simd_stack_op(op::SIMD_F64_ADD)?;
            }
            op::SIMD_I16_ADD => {
                self.exec_simd_stack_op(op::SIMD_I16_ADD)?;
            }
            op::SIMD_I16_SUB => {
                self.exec_simd_stack_op(op::SIMD_I16_SUB)?;
            }
            op::SIMD_I16_MUL => {
                self.exec_simd_stack_op(op::SIMD_I16_MUL)?;
            }
            op::SIMD_I8_ADD => {
                self.exec_simd_stack_op(op::SIMD_I8_ADD)?;
            }
            op::SIMD_I8_SUB => {
                self.exec_simd_stack_op(op::SIMD_I8_SUB)?;
            }
            op::SIMD_I8_MUL => {
                self.exec_simd_stack_op(op::SIMD_I8_MUL)?;
            }
            op::SIMD_I8_AND => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_and(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_OR => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_or(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_XOR => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_xor(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_NOT => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_not(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_MIN => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_min(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_MAX => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_max(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_UMIN => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_umin(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_UMAX => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_umax(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_ABS => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_abs(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_NEG => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_neg(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_EXTRACT => {
                let lane = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_extract_lane(&v, lane) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_REPLACE => {
                let scalar = self.stack.pop().unwrap_or(Value::I32(0));
                let lane = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_replace_lane(&v, lane, &scalar) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_EQ => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_eq(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_NE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_ne(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_LE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_le(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_GE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_ge(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_AVGR_U => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_avgr_u(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_POPCNT => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_popcnt(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_BITMASK => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_bitmask(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_ALL_TRUE => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_all_true(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_ANY_TRUE => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_any_true(&v) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_LT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_lt(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_GT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_gt(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_ADD_SAT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_add_sat(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_SUB_SAT => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_sub_sat(&a, &b) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_SHUFFLE => {
                let mode = a;
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_shuffle_i8(&v, mode) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_SHL => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_shl(&v, amount) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I8_SHR => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = simd_shr(&v, amount) {
                    self.stack.push(r);
                }
            }
            op::SIMD_I64_ADD => {
                self.exec_simd_stack_op(op::SIMD_I64_ADD)?;
            }
            op::SIMD_F64_SUB => {
                self.exec_simd_stack_op(op::SIMD_F64_SUB)?;
            }
            op::SIMD_F64_MUL => {
                self.exec_simd_stack_op(op::SIMD_F64_MUL)?;
            }
            op::SIMD_F64_DIV => {
                self.exec_simd_stack_op(op::SIMD_F64_DIV)?;
            }
            op::SIMD_GUEST_PUSH_V128 => {
                // Phase 117: a=element; stack-native path uses wasm V128 arg (see wasm-run linker).
                // Residual mir_exec still accepts scratch_base in b when memory is provided.
                self.simd_guest_push_v128(a, b, pool)?;
            }
            op::SIMD_GUEST_POP_V128 => {
                self.simd_guest_pop_v128(b, pool)?;
            }
            op::SIMD_LOAD => {
                let (lanes, element) = simd_tag_decode(a);
                let offset = b.max(0) as usize;
                let array = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(k) = syx_stdlib::NumericKind::from_byte(element) {
                    if let Some(v) = simd_load_from_array(&array, lanes, k, offset) {
                        self.stack.push(v);
                    }
                }
            }
            op::SIMD_STORE => {
                let (lanes, element) = simd_tag_decode(a);
                let offset = b.max(0) as usize;
                let simd = self.stack.pop().unwrap_or(Value::Nil);
                let mut array = self.stack.pop().unwrap_or(Value::Array(vec![]));
                let _ = (lanes, element);
                if simd_store_to_array(&mut array, offset, &simd).is_some() {
                    self.stack.push(array);
                }
            }
            op::SIMD_SHUFFLE => {
                let mode = a;
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Value::Simd {
                    lanes,
                    element,
                    mut bits,
                } = v
                {
                    if lanes == 4 {
                        let mut lane_vals = [0i32; 4];
                        for i in 0..4 {
                            let off = i * 4;
                            lane_vals[i] =
                                i32::from_le_bytes(bits[off..off + 4].try_into().unwrap_or([0; 4]));
                        }
                        let shuffled = match mode {
                            0 => [lane_vals[3], lane_vals[2], lane_vals[1], lane_vals[0]],
                            1 => [lane_vals[1], lane_vals[0], lane_vals[3], lane_vals[2]],
                            _ => lane_vals,
                        };
                        for (i, val) in shuffled.iter().enumerate() {
                            let off = i * 4;
                            bits[off..off + 4].copy_from_slice(&val.to_le_bytes());
                        }
                        self.stack.push(Value::Simd {
                            lanes,
                            element,
                            bits,
                        });
                    }
                }
            }
            op::CONST_STR => {
                let start = a.max(0) as usize;
                let end = start.saturating_add(b.max(0) as usize);
                let s = if end <= pool.len() {
                    std::str::from_utf8(&pool[start..end])
                        .unwrap_or("")
                        .to_string()
                } else {
                    String::new()
                };
                self.stack.push(Value::Str(s));
            }
            op::CONST_BOOL => self.stack.push(Value::Bool(a != 0)),
            op::CONST_NIL => self.stack.push(Value::Nil),
            op::LOAD_LOCAL => {
                // Wave A W523 / ADR-867: LOAD_LOCAL ObjectRef RC bump retired
                // (true-retire first-slice; parity with VM). Keep push + W510
                // dual-track. SSA LoadLocal already no-retain (W517).
                // ≠ Value RC full delete / Release strip / migrate / enable.
                let v = self.locals.get(a as usize).cloned().unwrap_or(Value::Nil);
                self.stack.push(v);
            }
            op::STORE_LOCAL => {
                // Wave A W525 / ADR-869: STORE_LOCAL ObjectRef ManagedHeap +
                // heap_slots unroot (death-path first-slice; VM parity).
                // Wave A W528 / ADR-872: ObjectRef RC true-retire — no
                // object_ref_release / eager Type_deinit; finalizer via W527
                // collect sweep. Keep maybe_untrack. ≠ Value API delete /
                // ≠ migrate / ≠ enable.
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if self.locals.len() <= a as usize {
                    self.locals.resize(a as usize + 1, Value::Nil);
                }
                let should_unroot = self
                    .locals
                    .get(a as usize)
                    .map(|o| o != &v)
                    .unwrap_or(true);
                let old = std::mem::replace(&mut self.locals[a as usize], v);
                if should_unroot {
                    self.maybe_untrack_objectref_value(&old);
                }
            }
            op::ADD => {
                // String `+` is concatenation (Swift-like), including HeapSlot-backed
                // short strings (heap v9 inline). Numeric Add remains for non-strings.
                // Without this, host handles like `label:1` + "," become `0` and SynUI
                // row/column CSVs silently empty.
                let kind_byte = a as u8;
                let kind_valid = b != 0;
                let rhs = self.stack.pop().unwrap_or(Value::Nil);
                let lhs = self.stack.pop().unwrap_or(Value::Nil);
                let lhs_str = matches!(lhs, Value::Str(_))
                    || matches!(
                        lhs,
                        Value::HeapSlot {
                            kind: syx_runtime_core::HeapSlotKind::String,
                            ..
                        }
                    );
                let rhs_str = matches!(rhs, Value::Str(_))
                    || matches!(
                        rhs,
                        Value::HeapSlot {
                            kind: syx_runtime_core::HeapSlotKind::String,
                            ..
                        }
                    );
                if lhs_str || rhs_str {
                    self.stack
                        .push(Value::Str(format!("{}{}", lhs.as_str(), rhs.as_str())));
                } else {
                    self.stack.push(lhs);
                    self.stack.push(rhs);
                    self.binop_numeric_kind(kind_byte, kind_valid, numeric_add);
                }
            }
            op::SUB => self.binop_numeric_kind(a as u8, b != 0, numeric_sub),
            op::MUL => self.binop_numeric_kind(a as u8, b != 0, numeric_mul),
            op::DIV => self.binop_numeric_kind(a as u8, b != 0, numeric_div),
            op::MOD => self.binop_numeric_kind(a as u8, b != 0, numeric_mod),
            op::BIT_AND => self.binop_numeric_kind(a as u8, b != 0, numeric_bitand),
            op::BIT_OR => self.binop_numeric_kind(a as u8, b != 0, numeric_bitor),
            op::BIT_XOR => self.binop_numeric_kind(a as u8, b != 0, numeric_bitxor),
            op::SHL => self.binop_numeric_kind(a as u8, b != 0, numeric_shl),
            op::SHR => self.binop_numeric_kind(a as u8, b != 0, numeric_shr),
            op::EQ => self.binop_bool(|x, y| x == y),
            op::NE => self.binop_bool(|x, y| x != y),
            op::LT => self.binop_cmp_kind(a as u8, b != 0, |o| o == std::cmp::Ordering::Less),
            op::GT => self.binop_cmp_kind(a as u8, b != 0, |o| o == std::cmp::Ordering::Greater),
            op::LE => self.binop_cmp_kind(a as u8, b != 0, |o| {
                matches!(o, std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
            }),
            op::GE => self.binop_cmp_kind(a as u8, b != 0, |o| {
                matches!(o, std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
            }),
            op::LOAD_GLOBAL => {
                // Wave A W516 / ADR-860: after LOAD_GLOBAL push, dual-register
                // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots.
                // No retain call. ≠ migrate / cleanup / enable / SSA LoadLocal.
                let v = self.globals.get(a as usize).cloned().unwrap_or(Value::Nil);
                self.stack.push(v);
            }
            op::STORE_GLOBAL => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if self.globals.len() <= a as usize {
                    self.globals.resize(a as usize + 1, Value::Nil);
                }
                self.globals[a as usize] = v;
            }
            op::AND => self.binop_logic(|x, y| x && y),
            op::OR => self.binop_logic(|x, y| x || y),
            op::NOT => {
                let v = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(!v.as_bool()));
            }
            op::BIT_NOT => {
                let v = self.stack.pop().unwrap_or(Value::I32(0));
                let kind = if b != 0 {
                    NumericKind::from_byte(a as u8).unwrap_or(PLATFORM_INT)
                } else {
                    value_kind(&v).unwrap_or(PLATFORM_INT)
                };
                if let Some(r) = numeric_bitnot(kind, &v) {
                    self.stack.push(r);
                } else {
                    self.stack.push(Value::I32(0));
                }
            }
            op::NEG => {
                let v = self.stack.pop().unwrap_or(Value::I32(0));
                let kind = if b != 0 {
                    NumericKind::from_byte(a as u8).unwrap_or(PLATFORM_INT)
                } else {
                    value_kind(&v).unwrap_or(PLATFORM_INT)
                };
                if let Some(r) = numeric_neg(kind, &v) {
                    self.stack.push(r);
                } else {
                    self.stack.push(Value::I32(-v.as_i32()));
                }
            }
            op::NUMERIC_CAST => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                let fk = NumericKind::from_byte(a as u8).unwrap_or(PLATFORM_INT);
                let tk = NumericKind::from_byte(b as u8).unwrap_or(PLATFORM_INT);
                match try_numeric_cast(fk, tk, &v) {
                    Ok(casted) => self.stack.push(casted),
                    Err(msg) => {
                        return Err(SyxDiagnostic::error(msg).with_code("R003"));
                    }
                }
            }
            op::PRINT => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                let text = value_to_display(&v);
                self.printed.push(text.clone());
                if let Some(rt) = &self.runtime {
                    rt.record_print(text.clone());
                }
                println!("{text}");
                // Phase 86: keep printed value on stack so AOT `main` can GET_RETURN after print(expr).
                self.stack.push(v);
            }
            op::CONCAT_STR => {
                let b_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let a_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
                self.stack
                    .push(Value::Str(format!("{}{}", a_val.as_str(), b_val.as_str())));
            }
            op::TO_STR => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Str(value_to_display(&v)));
            }
            op::POP => {
                self.stack.pop();
            }
            op::DUP => {
                // Wave A W523 / ADR-867: DUP ObjectRef RC bump retired
                // (true-retire first-slice; parity with VM). Keep push + W510
                // dual-track. Release/StoreLocal/deinit still live.
                // ≠ Value RC full delete / migrate / cleanup / enable.
                let v = self.stack.last().cloned().unwrap_or(Value::Nil);
                self.stack.push(v);
            }
            op::CALL_HOST => {
                // Wave A W501 / ADR-845: after successful CallHost, dual-register
                // stack-top ObjectRef into ManagedHeap + heap_slots (parity with
                // VM W500 / ADR-844); identity-dedup for reused handles.
                // Wave A W505 / ADR-849: also track stack-top MutRefCell (covers
                // host paths such as `host_ordered_set_new`); ≠ migrate / cleanup.
                self.call_host(a as u32, memory)?;
            }
            op::METHOD_CALL => {
                // Wave A W509 / ADR-853: after successful METHOD_CALL dispatch,
                // dual-register stack-top ObjectRef+MutRefCell into ManagedHeap +
                // heap_slots (parity with CALL_HOST / VM MethodCall). ≠ migrate.
                self.method_call(a as u32, memory)?;
            }
            op::MAKE_ITERATOR => self.make_iterator(),
            op::ITERATOR_NEXT => self.iterator_next(pool),
            op::ARRAY_BUILD => self.array_build(a as u32),
            op::ARRAY_GET => self.array_get()?,
            op::ARRAY_SET => self.array_set()?,
            op::ARRAY_SORTED => self.array_sorted(pool)?,
            op::SIMD_WASM_SPILL => self.simd_wasm_spill(a, b, pool)?,
            op::SIMD_WASM_FILL => self.simd_wasm_fill(a, b, pool)?,
            op::OBJECT_GET => self.object_get(),
            op::AWAIT_POLL => cooperative_yield(),
            op::AWAIT_SUSPEND => self.await_suspend(a as u32, b as u32)?,
            op::ACTOR_SEND => self.actor_send(a, b, pool)?,
            op::ACTOR_CALL => self.actor_call(a, b, pool)?,
            op::ACTOR_REGISTER => {
                let obj = self.stack.pop().unwrap_or(Value::Nil);
                let id = NEXT_ACTOR_ID.fetch_add(1, Ordering::Relaxed);
                // ADR-631: stamp __actor_id on Object and ObjectRef (heap-v4).
                self.stack.push(obj.register_actor_id(id));
            }
            op::CALL_WITNESS => self.call_witness(a as u32, b as u32)?,
            op::DICT_BUILD => self.dict_build(a as u32),
            op::SET_BUILD => self.set_build(a as u32),
            op::OBJECT_SET => self.object_set(),
            // Wave A W541 / ADR-885: mir-run OBJECT_REF_* opcode arms peeled.
            // MirOp::ObjectRef* handled in exec_mir_op (wrap without handle table).
            op::HEAP_COMPACT => self.heap_compact(),
            op::MAKE_CLOSURE => self.make_closure(a as u32, b as u32),
            op::CALL_CLOSURE => self.call_closure(pool)?,
            op::TASK_SPAWN => self.task_spawn_by_name(a, b, pool)?,
            op::TASK_JOIN => self.task_join()?,
            op::TO_DESCRIPTION => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Str(value_description(&v)));
            }
            op::TO_DEBUG_DESCRIPTION => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Str(value_debug_description(&v)));
            }
            op::VALUE_EQ => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(value_equals(&a, &b)));
            }
            op::VALUE_NE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(!value_equals(&a, &b)));
            }
            op::REF_EQ => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(value_identical(&a, &b)));
            }
            op::REF_NE => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(!value_identical(&a, &b)));
            }
            op::BEGIN_TRY => {
                self.try_handlers.push(a as u32);
            }
            op::END_TRY => {
                self.try_handlers.pop();
            }
            op::CATCH => {}
            op::THROW => {
                self.pending_catch = Some(self.stack.pop().unwrap_or(Value::Str("error".into())));
            }
            op::BIND_RESULT => {
                if self.pending_catch.is_some() {
                    return Ok(a);
                }
            }
            op::RETURN_ERR => {
                let err = self.stack.pop().unwrap_or(Value::Str("error".into()));
                self.pending_catch = Some(err);
                return Ok(-3);
            }
            _ => {}
        }
        Ok(0)
    }

    pub fn exec_mir_op(&mut self, mir_op: &MirOp, memory: &mut [u8]) -> SyxResult<()> {
        if let MirOp::SimdReduceAdd { .. } = mir_op {
            let v = self.stack.pop().unwrap_or(Value::Nil);
            if let Some(sum) = simd_reduce_add(&v) {
                self.stack.push(sum);
            }
            return Ok(());
        }
        // Wave A W544 / ADR-888: MirOp::ObjectRefWrap/Retain/Release peeled.
        // Product ObjectRef birth remains DictBuild + object_ref_from_map + maybe_track_*.
        // EC-M1-3: control-flow Suite-1 pilots have no immediates opcode (CONST_I32 is also 0).
        // They must go through run_mir_function, not silent no-op via opcode 0.
        if matches!(
            mir_op,
            MirOp::Call(_) | MirOp::Jump(_) | MirOp::JumpIfFalse(_) | MirOp::Return
        ) {
            return Err(SyxDiagnostic::error(format!(
                "EC-M1-3: Suite-1 pilot MirOp is control-flow; use run_mir_function, not exec_mir_op: {mir_op:?}"
            ))
            .with_code("E_MIR_PILOT"));
        }
        let (op_code, a, b) = mir_op_immediates(mir_op, memory, self);
        self.exec(op_code, a, b, 0, memory)?;
        Ok(())
    }

    pub fn pop_bool(&mut self) -> bool {
        self.stack.pop().unwrap_or(Value::Bool(false)).as_bool()
    }

    pub fn return_value(&mut self) -> i32 {
        self.stack.pop().unwrap_or(Value::Nil).as_i32()
    }

    pub fn call_fn(&mut self, fn_idx: usize, memory: &mut [u8]) -> SyxResult<()> {
        self.current_fn = fn_idx;
        let Some(func) = self.functions.get(fn_idx).cloned() else {
            return Ok(());
        };
        let arg_count = func.param_count as usize;
        let start = self.stack.len().saturating_sub(arg_count);
        let mut args: Vec<Value> = self.stack.drain(start..).collect();
        // Phase 128: materialize `@Lazy` proxy receivers before instance methods
        // (not init/deinit — first arg is a ctor parameter, not `self`).
        let is_ctor = func.name.ends_with("_init") || func.name.ends_with("_deinit");
        if func.name.ends_with("_deinit") {
            if let Some(recv) = args.first() {
                Value::mark_object_weak_dead(recv);
            }
        }
        if !is_ctor {
            if let Some(recv) = args.first_mut() {
                *recv = syx_vm::materialize_lazy_receiver(recv.clone())?;
            }
        }
        let fn_idx = self
            .resolve_virtual_mir_fn(&func.name, &args)
            .unwrap_or(fn_idx);
        let Some(func) = self.functions.get(fn_idx).cloned() else {
            return Ok(());
        };
        let saved_locals = std::mem::take(&mut self.locals);
        let result = run_mir_function_with_args(&func, self, memory, &args)?;
        self.locals = saved_locals;
        self.stack.push(result);
        // Wave A W507 / ADR-851: after successful call_fn return, dual-register
        // stack-top ObjectRef and MutRefCell into ManagedHeap + heap_slots
        // (parity with VM finish_call W506 / ADR-850). ≠ migrate / cleanup.
        Ok(())
    }

    /// Phase 125/126: MIR Call virtual dispatch (parity with VM `resolve_virtual_method`).
    fn resolve_virtual_mir_fn(&self, static_name: &str, args: &[Value]) -> Option<usize> {
        let (static_class, method) = syx_ast::parse_mangled_instance_method(static_name)?;
        let static_class = static_class.as_str();
        let method = method.as_str();
        if method.is_empty() || static_class.is_empty() {
            return None;
        }
        if method == "init" || method == "deinit" {
            return None;
        }
        let receiver = args.first()?;
        let current = Value::dispatch_receiver_for_method(receiver.clone());
        let witness_key = Value::Str(format!("__witness_{static_class}_{method}"));
        if let Value::I32(idx) = current.get_by_key(&witness_key) {
            if let Some(slot_fn) = self.functions.get(idx as usize) {
                if slot_fn.name != static_name {
                    return Some(idx as usize);
                }
            }
        }
        let vtable_key = Value::Str(format!("__vtable_{method}"));
        if let Value::I32(idx) = current.get_by_key(&vtable_key) {
            if let Some(slot_fn) = self.functions.get(idx as usize) {
                if slot_fn.name != static_name {
                    return Some(idx as usize);
                }
            }
        }
        let runtime_ty = current.object_ref_type_name()?;
        if runtime_ty == static_class {
            return None;
        }
        let dynamic = format!("{runtime_ty}_{method}");
        self.functions.iter().position(|f| f.name == dynamic)
    }

    fn binop_numeric(&mut self, f: fn(NumericKind, &Value, &Value) -> Option<Value>) {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        let kind = value_kind(&a)
            .or_else(|| value_kind(&b))
            .unwrap_or(PLATFORM_INT);
        if let Some(r) = f(kind, &a, &b) {
            self.stack.push(r);
        } else {
            self.stack.push(Value::I32(0));
        }
    }

    fn binop_numeric_kind(
        &mut self,
        kind_byte: u8,
        kind_valid: bool,
        f: fn(NumericKind, &Value, &Value) -> Option<Value>,
    ) {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        let kind = if kind_valid {
            NumericKind::from_byte(kind_byte).unwrap_or(PLATFORM_INT)
        } else {
            value_kind(&a)
                .or_else(|| value_kind(&b))
                .unwrap_or(PLATFORM_INT)
        };
        if let Some(r) = f(kind, &a, &b) {
            self.stack.push(r);
        } else {
            self.stack.push(Value::I32(0));
        }
    }

    fn binop_bool(&mut self, f: fn(i32, i32) -> bool) {
        let b = self.stack.pop().unwrap_or(Value::I32(0));
        let a = self.stack.pop().unwrap_or(Value::I32(0));
        self.stack.push(Value::Bool(f(a.as_i32(), b.as_i32())));
    }

    fn binop_cmp_kind(
        &mut self,
        kind_byte: u8,
        kind_valid: bool,
        pred: impl Fn(std::cmp::Ordering) -> bool,
    ) {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        let kind = if kind_valid {
            NumericKind::from_byte(kind_byte).unwrap_or(PLATFORM_INT)
        } else {
            value_kind(&a)
                .or_else(|| value_kind(&b))
                .unwrap_or(PLATFORM_INT)
        };
        let ok = numeric_cmp(kind, &a, &b).is_some_and(pred);
        self.stack.push(Value::Bool(ok));
    }

    fn exec_simd_stack_op(&mut self, op_code: u32) -> SyxResult<()> {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        let op = match op_code {
            op::SIMD_ADD
            | op::SIMD_F32_ADD
            | op::SIMD_F64_ADD
            | op::SIMD_I16_ADD
            | op::SIMD_I8_ADD
            | op::SIMD_I64_ADD => SimdBinOp::Add,
            op::SIMD_SUB
            | op::SIMD_F32_SUB
            | op::SIMD_F64_SUB
            | op::SIMD_I16_SUB
            | op::SIMD_I8_SUB => SimdBinOp::Sub,
            op::SIMD_MUL
            | op::SIMD_F32_MUL
            | op::SIMD_F64_MUL
            | op::SIMD_I16_MUL
            | op::SIMD_I8_MUL => SimdBinOp::Mul,
            op::SIMD_DIV | op::SIMD_F32_DIV | op::SIMD_F64_DIV => SimdBinOp::Div,
            _ => SimdBinOp::Add,
        };
        if let Some(r) = simd_binop(op, &a, &b) {
            self.stack.push(r);
        }
        Ok(())
    }

    fn binop_logic(&mut self, f: fn(bool, bool) -> bool) {
        let b = self.stack.pop().unwrap_or(Value::Bool(false));
        let a = self.stack.pop().unwrap_or(Value::Bool(false));
        self.stack.push(Value::Bool(f(a.as_bool(), b.as_bool())));
    }

    fn dispatch_array_hof_host(&mut self, name: &str, memory: &mut [u8]) -> SyxResult<bool> {
        use syx_stdlib::{lookup_collection_host, CollectionHostOp};
        let Some(entry) = lookup_collection_host(name) else {
            return Ok(false);
        };
        let op = entry.op;
        if !matches!(
            op,
            CollectionHostOp::Map | CollectionHostOp::Filter | CollectionHostOp::Reduce
        ) {
            return Ok(false);
        }
        let (args, receiver) = match op {
            CollectionHostOp::Reduce => {
                let closure = self.stack.pop().unwrap_or(Value::Nil);
                let initial = self.stack.pop().unwrap_or(Value::Nil);
                let receiver = self.stack.pop().unwrap_or(Value::Nil);
                (vec![initial, closure], receiver)
            }
            _ => {
                let closure = self.stack.pop().unwrap_or(Value::Nil);
                let receiver = self.stack.pop().unwrap_or(Value::Nil);
                (vec![closure], receiver)
            }
        };
        // ADR-225 / M2-S1-04: illegal receiver/closure → U001 (never `[]`/`Nil` success).
        let hof_err = |op: &str| {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "unsupported array HOF '{op}': requires Array receiver and Closure (feature-off or illegal shape)"
            ))
            .with_code("U001")
        };
        let Value::Array(items) = receiver else {
            let op = match op {
                CollectionHostOp::Map => "map",
                CollectionHostOp::Filter => "filter",
                CollectionHostOp::Reduce => "reduce",
                _ => "hof",
            };
            return Err(hof_err(op));
        };
        let closure_arg = |args: &[Value]| {
            args.iter().find_map(|v| {
                if let Value::Closure { fn_index, captures } = v {
                    Some((*fn_index, captures.clone()))
                } else {
                    None
                }
            })
        };
        match op {
            CollectionHostOp::Map => {
                let Some((fn_index, captures)) = closure_arg(&args) else {
                    return Err(hof_err("map"));
                };
                let mut mapped = Vec::with_capacity(items.len());
                for item in items {
                    mapped.push(self.run_closure_with_captures(
                        fn_index,
                        &captures,
                        &[item],
                        memory,
                    )?);
                }
                self.stack.push(Value::Array(mapped));
            }
            CollectionHostOp::Filter => {
                let Some((fn_index, captures)) = closure_arg(&args) else {
                    return Err(hof_err("filter"));
                };
                let mut filtered = Vec::new();
                for item in items {
                    let keep = self.run_closure_with_captures(
                        fn_index,
                        &captures,
                        &[item.clone()],
                        memory,
                    )?;
                    if keep.as_bool() {
                        filtered.push(item);
                    }
                }
                self.stack.push(Value::Array(filtered));
            }
            CollectionHostOp::Reduce => {
                if args.len() >= 2 {
                    if let Some((fn_index, captures)) = closure_arg(&args[1..]) {
                        let mut acc = args[0].clone();
                        for item in items {
                            acc = self.run_closure_with_captures(
                                fn_index,
                                &captures,
                                &[acc, item],
                                memory,
                            )?;
                        }
                        self.stack.push(acc);
                        return Ok(true);
                    }
                }
                let Some((fn_index, captures)) = closure_arg(&args) else {
                    return Err(hof_err("reduce"));
                };
                if items.is_empty() {
                    self.stack.push(Value::Nil);
                    return Ok(true);
                }
                let mut acc = items[0].clone();
                for item in items.iter().skip(1) {
                    acc = self.run_closure_with_captures(
                        fn_index,
                        &captures,
                        &[acc, item.clone()],
                        memory,
                    )?;
                }
                self.stack.push(acc);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn call_host(&mut self, idx: u32, memory: &mut [u8]) -> SyxResult<()> {
        let name = self
            .host_fn_names
            .get(idx as usize)
            .map(|s| s.as_str())
            .or_else(|| syx_stdlib::host_fn_name_for_index(idx))
            .unwrap_or("")
            .to_string();
        if self.dispatch_array_hof_host(&name, memory)? {
            return Ok(());
        }
        if syx_vm::dispatch_numeric_host(&name, &mut self.stack).unwrap_or(false) {
            return Ok(());
        }
        if syx_vm::dispatch_collection_host(&name, &mut self.stack).unwrap_or(false) {
            return Ok(());
        }
        if syx_vm::dispatch_bigint_host(&name, &mut self.stack).unwrap_or(false) {
            return Ok(());
        }
        #[cfg(not(target_arch = "wasm32"))]
        if syx_host_plugin::dispatch_plugin_host(&name, &mut self.stack) {
            return Ok(());
        }
        if let Some(id) = HostFnId::all().iter().find(|h| h.name() == name).copied() {
            // Wave A W267 / ADR-611: FmtPrint* must aggregate into MIR/wasm `printed`
            // (parity with Print MirOp) so HostFn wasm=real nails can exact-capture.
            match id {
                HostFnId::FmtPrint | HostFnId::FmtPrintLine => {
                    let v = self.stack.pop().unwrap_or(Value::Nil);
                    let text = value_description(&v);
                    if id == HostFnId::FmtPrint {
                        print!("{text}");
                        let _ = std::io::Write::flush(&mut std::io::stdout());
                    } else {
                        println!("{text}");
                    }
                    self.printed.push(text.clone());
                    if let Some(rt) = &self.runtime {
                        rt.record_print(text);
                    }
                    self.stack.push(Value::Nil);
                    return Ok(());
                }
                _ => {}
            }
            if let Some(rt) = &self.runtime {
                match id {
                    HostFnId::TaskDetach => {
                        let (name, args) = parse_task_detach_args(&mut self.stack);
                        let handle = rt.spawn_detached_fn(&name, args)?;
                        self.stack.push(handle);
                        return Ok(());
                    }
                    HostFnId::TaskJoin => {
                        let task = self.stack.pop().unwrap_or(Value::Nil);
                        // LLVM AOT: rt_direct_exec unwrap_or(0) swallows Err — must set
                        // pending_catch under try (parity with host_fn_for path).
                        match rt.join_task_value(&task) {
                            Ok(value) => {
                                let mut map = std::collections::HashMap::new();
                                map.insert("__done".into(), Value::Bool(true));
                                map.insert("__value".into(), value);
                                self.stack.push(Value::Object(map));
                            }
                            Err(e) => {
                                if self.try_handlers.is_empty() {
                                    return Err(e);
                                }
                                self.pending_catch = Some(diagnostic_to_catch_value(e));
                            }
                        }
                        return Ok(());
                    }
                    HostFnId::TaskCancel => {
                        let task = self.stack.pop().unwrap_or(Value::Nil);
                        match rt.cancel_task_value(&task) {
                            Ok(()) => {
                                self.stack.push(Value::Bool(true));
                            }
                            Err(e) => {
                                if self.try_handlers.is_empty() {
                                    return Err(e);
                                }
                                self.pending_catch = Some(diagnostic_to_catch_value(e));
                            }
                        }
                        return Ok(());
                    }
                    HostFnId::TaskSleep => {
                        let ms = self.stack.pop().unwrap_or(Value::I32(0)).as_i32() as i64;
                        // ADR-607: cooperative cancel → C004 (align VM host_task_sleep).
                        match rt.task_sleep_ms(ms) {
                            Ok(()) => {}
                            Err(e) => {
                                if self.try_handlers.is_empty() {
                                    return Err(e);
                                }
                                self.pending_catch = Some(diagnostic_to_catch_value(e));
                            }
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
            if let Some(host) = host_fn_for(id) {
                if let Err(e) = host(&mut self.stack) {
                    // M0-W4 / M0-B06: without an active try, transport/host failures must
                    // fail the run (not leave the URL arg on the stack for print).
                    if self.try_handlers.is_empty() {
                        return Err(e);
                    }
                    self.pending_catch = Some(diagnostic_to_catch_value(e));
                }
            }
        }
        Ok(())
    }

    fn method_call(&mut self, idx: u32, memory: &mut [u8]) -> SyxResult<()> {
        let Some(method) = method_from_index(idx) else {
            return Ok(());
        };
        let arg_count = match method {
            syx_stdlib::MethodId::StrReplacingOccurrences | syx_stdlib::MethodId::StrSubstring => 2,
            syx_stdlib::MethodId::StrSplit
            | syx_stdlib::MethodId::StrHasPrefix
            | syx_stdlib::MethodId::StrMatches => 1,
            syx_stdlib::MethodId::ArrContains | syx_stdlib::MethodId::ArrJoined => 1,
            syx_stdlib::MethodId::ArrMap
            | syx_stdlib::MethodId::ArrFilter
            | syx_stdlib::MethodId::ArrReduce => 1,
            syx_stdlib::MethodId::HttpResponseHeader => 1,
            syx_stdlib::MethodId::HttpResponseJson
            | syx_stdlib::MethodId::HttpResponseStatus
            | syx_stdlib::MethodId::HttpResponseText
            | syx_stdlib::MethodId::HttpResponseBody
            | syx_stdlib::MethodId::HttpResponseOk
            | syx_stdlib::MethodId::HttpResponseHeaders => 0,
            _ => 0,
        };
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(self.stack.pop().unwrap_or(Value::Nil));
        }
        args.reverse();
        let receiver = self.stack.pop().unwrap_or(Value::Nil);
        let closure_arg = args.iter().find_map(|v| {
            if matches!(v, Value::Closure { .. }) {
                Some(v)
            } else {
                None
            }
        });
        if method == syx_stdlib::MethodId::ArrFilter {
            if let (Value::Array(items), Some(Value::Closure { fn_index, captures })) =
                (&receiver, closure_arg)
            {
                let mut filtered = Vec::new();
                for item in items {
                    let keep = self.run_closure_with_captures(
                        *fn_index,
                        captures,
                        &[item.clone()],
                        memory,
                    )?;
                    if keep.as_bool() {
                        filtered.push(item.clone());
                    }
                }
                self.stack.push(Value::Array(filtered));
                return Ok(());
            }
            // ADR-225 / M2-S1-04: no fallthrough to empty-array success.
            return Err(syx_diagnostics::SyxDiagnostic::error(
                "unsupported array HOF 'filter': requires Array receiver and Closure (feature-off or illegal shape)",
            )
            .with_code("U001"));
        }
        if method == syx_stdlib::MethodId::ArrMap {
            if let (Value::Array(items), Some(Value::Closure { fn_index, captures })) =
                (&receiver, closure_arg)
            {
                let mut mapped = Vec::with_capacity(items.len());
                for item in items {
                    mapped.push(self.run_closure_with_captures(
                        *fn_index,
                        captures,
                        &[item.clone()],
                        memory,
                    )?);
                }
                self.stack.push(Value::Array(mapped));
                return Ok(());
            }
            return Err(syx_diagnostics::SyxDiagnostic::error(
                "unsupported array HOF 'map': requires Array receiver and Closure (feature-off or illegal shape)",
            )
            .with_code("U001"));
        }
        if method == syx_stdlib::MethodId::ArrReduce {
            let result = syx_vm::try_call_array_method_for_mir(
                syx_stdlib::MethodId::ArrReduce,
                &receiver,
                &args,
            )?;
            self.stack.push(result);
            return Ok(());
        }
        let result = call_receiver_method(method, &receiver, &args);
        self.stack.push(result);
        Ok(())
    }

    pub fn run_closure_with_captures(
        &mut self,
        fn_index: u32,
        captures: &[Value],
        args: &[Value],
        memory: &mut [u8],
    ) -> SyxResult<Value> {
        let Some(func) = self.functions.get(fn_index as usize).cloned() else {
            return Ok(Value::Nil);
        };
        let saved_locals = std::mem::take(&mut self.locals);
        let saved_stack = std::mem::take(&mut self.stack);
        self.begin_function(func.local_count);
        for (i, arg) in args.iter().enumerate() {
            if i < self.locals.len() {
                self.locals[i] = arg.clone();
            }
        }
        let capture_start = func.param_count as usize;
        for (i, cap) in captures.iter().enumerate() {
            let slot = capture_start + i;
            if slot < self.locals.len() {
                self.locals[slot] = cap.clone();
            }
        }
        let mut pool = if memory.is_empty() {
            self.string_pool.clone()
        } else {
            memory.to_vec()
        };
        let result = run_mir_function_body(&func, self, &mut pool)?;
        self.pop_try_handler_frame();
        self.stack = saved_stack;
        self.locals = saved_locals;
        Ok(result)
    }

    fn class_type_name(&self, obj: &Value) -> Option<String> {
        obj.object_ref_type_name()
    }

    fn resolve_iterator_witness_next_fn(&self, obj: &Value, raw: u32) -> u32 {
        if let Some(ty) = self.class_type_name(obj) {
            let name = format!("{ty}_next");
            if let Some(&idx) = self.fn_name_to_index.get(&name) {
                return idx as u32;
            }
        }
        if let Some(f) = self.functions.get(raw as usize) {
            if f.name.ends_with("_init") {
                if let Some(ty) = self.class_type_name(obj) {
                    let name = format!("{ty}_next");
                    if let Some(&idx) = self.fn_name_to_index.get(&name) {
                        return idx as u32;
                    }
                }
            }
        }
        raw
    }

    fn make_iterator(&mut self) {
        let top = self.stack.pop().unwrap_or(Value::Nil);
        if top.is_mut_ref_cell() {
            if let Some(ty) = self.class_type_name(&top) {
                let name = format!("{ty}_next");
                if let Some(&idx) = self.fn_name_to_index.get(name.as_str()) {
                    self.stack
                        .push(Value::Iterator(Box::new(IteratorState::Witness {
                            obj: Box::new(top),
                            next_fn: idx as u32,
                        })));
                    return;
                }
            }
        }
        let witness_next = match top.mut_ref_cell_get_field("__witness_Iterator_next") {
            Value::I32(fn_idx) => Some(self.resolve_iterator_witness_next_fn(&top, fn_idx as u32)),
            _ => None,
        };
        if let Some(next_fn) = witness_next {
            self.stack
                .push(Value::Iterator(Box::new(IteratorState::Witness {
                    obj: Box::new(top),
                    next_fn,
                })));
            return;
        }
        let resolved = Value::resolve_heap_slot(&top);
        let items = match &resolved {
            Value::Object(map) | Value::Dict(map) => {
                if let (Some(start), Some(end)) = (map.get("start"), map.get("end")) {
                    let s = start.as_i64();
                    let e = end.as_i64();
                    let inclusive = map.get("inclusive").map(|v| v.as_bool()).unwrap_or(false);
                    if inclusive {
                        if s <= e {
                            (s..=e).map(platform_int_value).collect()
                        } else {
                            Vec::new()
                        }
                    } else if s < e {
                        (s..e).map(platform_int_value).collect()
                    } else {
                        Vec::new()
                    }
                } else {
                    vec![top]
                }
            }
            Value::Array(a) => a.clone(),
            // ADR-226 / M2-S2-01: String for-in = grapheme Character (align .length / s[i]).
            Value::Str(s) => syx_vm::graphemes_vec(&s)
                .into_iter()
                .map(Value::Character)
                .collect(),
            other if other.is_mut_ref_cell() || matches!(other, Value::ObjectRef { .. }) => {
                vec![top.clone()]
            }
            other if other.as_i64() >= 0 => (0..other.as_i64()).map(platform_int_value).collect(),
            other => vec![other.clone()],
        };
        self.stack
            .push(Value::Iterator(Box::new(IteratorState::Static {
                items,
                index: 0,
            })));
    }

    fn iterator_next(&mut self, _memory: &mut [u8]) {
        let iter = self
            .stack
            .pop()
            .unwrap_or(Value::Iterator(Box::new(IteratorState::Static {
                items: vec![],
                index: 0,
            })));
        match iter {
            Value::Iterator(state) => match *state {
                IteratorState::Static { items, index } => {
                    if index < items.len() {
                        let val = items[index].clone();
                        self.stack
                            .push(Value::Iterator(Box::new(IteratorState::Static {
                                items,
                                index: index + 1,
                            })));
                        self.stack.push(val);
                        self.stack.push(Value::Bool(true));
                    } else {
                        self.stack.push(Value::Bool(false));
                    }
                }
                IteratorState::Witness { obj, next_fn } => {
                    let next_fn = self.resolve_iterator_witness_next_fn(&(*obj), next_fn);
                    self.stack.push((*obj).clone());
                    match self.call_fn(next_fn as usize, &mut []) {
                        Ok(()) => {
                            let result = self.stack.pop().unwrap_or(Value::Nil);
                            if matches!(result, Value::Nil) {
                                self.stack.push(Value::Bool(false));
                            } else {
                                self.stack.push(Value::Iterator(Box::new(
                                    IteratorState::Witness { obj, next_fn },
                                )));
                                self.stack.push(result);
                                self.stack.push(Value::Bool(true));
                            }
                        }
                        Err(_) => self.stack.push(Value::Bool(false)),
                    }
                }
            },
            _ => self.stack.push(Value::Bool(false)),
        }
    }

    fn array_build(&mut self, count: u32) {
        let mut items = Vec::new();
        for _ in 0..count {
            items.push(self.stack.pop().unwrap_or(Value::Nil));
        }
        items.reverse();
        let array = Value::Array(items);
        self.stack.push(Value::alloc_heap_slot(
            array,
            syx_runtime_core::HeapSlotKind::Array,
        ));
    }

    fn materialize_array(v: Value) -> Value {
        match v {
            Value::HeapSlot { .. } => Value::resolve_heap_slot(&v),
            other => other,
        }
    }

    fn array_get(&mut self) -> SyxResult<()> {
        // Wave A W513 / ADR-857: after ARRAY_GET re-entry, dual-register
        // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots
        // (VM ArrayGet parity). ≠ migrate / cleanup / wrap emit retire.
        let idx = self.stack.pop().unwrap_or(Value::I32(0));
        let arr = Self::materialize_array(self.stack.pop().unwrap_or(Value::Array(vec![])));
        let val = match arr {
            Value::Array(items) => {
                let i = idx.as_i32();
                if i < 0 || (i as usize) >= items.len() {
                    return Err(SyxDiagnostic::error(format!(
                        "Array index {i} out of bounds (count {})",
                        items.len()
                    ))
                    .with_code("R003"));
                }
                items[i as usize].clone()
            }
            Value::Str(s) => {
                let i = idx.as_i32();
                match syx_vm::try_grapheme_at(&s, i) {
                    Ok(g) => Value::Character(g),
                    Err((i, count)) => {
                        return Err(SyxDiagnostic::error(format!(
                            "String index {i} out of bounds (count {count})"
                        ))
                        .with_code("R003"));
                    }
                }
            }
            _ => Value::Nil,
        };
        self.stack.push(val);
        Ok(())
    }

    fn array_set(&mut self) -> SyxResult<()> {
        let val = self.stack.pop().unwrap_or(Value::Nil);
        let idx = self.stack.pop().unwrap_or(Value::I32(0));
        let arr_outer = self.stack.pop().unwrap_or(Value::Array(vec![]));
        let arr = Value::materialize_receiver(arr_outer.clone());
        if let Value::Array(mut items) = arr {
            let i = idx.as_i32();
            if i < 0 || (i as usize) >= items.len() {
                return Err(SyxDiagnostic::error(format!(
                    "Array index {i} out of bounds (count {})",
                    items.len()
                ))
                .with_code("R003"));
            }
            items[i as usize] = val;
            let updated = Value::Array(items);
            self.stack
                .push(Value::write_back_receiver(arr_outer, updated));
        } else {
            self.stack.push(arr_outer);
        }
        Ok(())
    }

    fn compare_with_witness(
        &mut self,
        a: &Value,
        b: &Value,
        witness_idx: u32,
        memory: &mut [u8],
    ) -> SyxResult<std::cmp::Ordering> {
        let Some(func) = self.functions.get(witness_idx as usize).cloned() else {
            return Ok(std::cmp::Ordering::Equal);
        };
        let cmp = run_mir_function_with_args(&func, self, memory, &[a.clone(), b.clone()])?;
        Ok(match cmp.as_i64().cmp(&0) {
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        })
    }

    fn array_sorted(&mut self, memory: &mut [u8]) -> SyxResult<()> {
        let arr = self.stack.pop().unwrap_or(Value::Array(vec![]));
        let mut items = match arr {
            Value::Array(items) => items,
            other => {
                self.stack.push(other);
                return Ok(());
            }
        };
        let witness_idx = items.first().and_then(|v| match v {
            Value::Object(map) | Value::Dict(map) => map
                .get("__witness_Comparable_compare")
                .and_then(|w| match w {
                    Value::I32(idx) => Some(*idx as u32),
                    _ => None,
                }),
            _ => None,
        });
        if let Some(idx) = witness_idx {
            let len = items.len();
            for i in 0..len {
                for j in (i + 1)..len {
                    let ord = self.compare_with_witness(&items[i], &items[j], idx, memory)?;
                    if ord == std::cmp::Ordering::Greater {
                        items.swap(i, j);
                    }
                }
            }
        } else {
            items.sort_by(syx_runtime_core::value_compare);
        }
        self.stack.push(Value::Array(items));
        Ok(())
    }

    fn simd_spillable_vector(lanes: u8, element: syx_stdlib::NumericKind) -> bool {
        let element = syx_stdlib::resolve_alias(element);
        matches!(
            (lanes, element),
            (4, syx_stdlib::NumericKind::I32)
                | (4, syx_stdlib::NumericKind::F32)
                | (2, syx_stdlib::NumericKind::F64)
                | (2, syx_stdlib::NumericKind::I64)
                | (8, syx_stdlib::NumericKind::I16)
                | (16, syx_stdlib::NumericKind::I8)
        )
    }

    fn simd_wasm_spill(
        &mut self,
        slot: i32,
        scratch_base: i32,
        memory: &mut [u8],
    ) -> SyxResult<()> {
        // Phase 113: slots -4/-5 (Array pack/unpack) are non-GA; guest LS uses ArrayGet/Set.
        if slot == -2 {
            let v = self.stack.pop().unwrap_or(Value::Nil);
            let n = v.as_i32();
            let offset = scratch_base.max(0) as usize;
            if offset + 4 <= memory.len() {
                memory[offset..offset + 4].copy_from_slice(&n.to_le_bytes());
            }
            return Ok(());
        }
        if slot == -3 {
            // Phase 110: unary shift — pop amount then vector (host SimdShr order).
            let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
            let v = self.stack.pop().unwrap_or(Value::Nil);
            let base = scratch_base.max(0) as usize;
            if base + 36 <= memory.len() {
                memory[base + 32..base + 36].copy_from_slice(&amount.to_le_bytes());
            }
            let Value::Simd {
                lanes,
                element,
                bits,
            } = v
            else {
                return Ok(());
            };
            if lanes != 4 || syx_stdlib::resolve_alias(element) != syx_stdlib::NumericKind::I32 {
                return Ok(());
            }
            if base + 16 <= memory.len() {
                memory[base..base + 16].copy_from_slice(&bits[..16]);
            }
            return Ok(());
        }
        if slot == -1 {
            // Phase 110: thin binop spill — pop rhs then lhs into scratch+16 / scratch+0.
            let rhs = self.stack.pop().unwrap_or(Value::Nil);
            let lhs = self.stack.pop().unwrap_or(Value::Nil);
            for (v, slot_idx) in [(lhs, 0i32), (rhs, 1i32)] {
                let Value::Simd {
                    lanes,
                    element,
                    bits,
                } = v
                else {
                    continue;
                };
                if !Self::simd_spillable_vector(lanes, element) {
                    continue;
                }
                self.simd_guest_element = Some(syx_stdlib::resolve_alias(element));
                let offset = scratch_base.max(0) as usize + slot_idx.max(0) as usize * 16;
                if offset + 16 <= memory.len() {
                    memory[offset..offset + 16].copy_from_slice(&bits[..16]);
                }
            }
            return Ok(());
        }
        let offset = scratch_base.max(0) as usize + slot.max(0) as usize * 16;
        let v = self.stack.pop().unwrap_or(Value::Nil);
        let Value::Simd {
            lanes,
            element,
            bits,
        } = v
        else {
            return Ok(());
        };
        if !Self::simd_spillable_vector(lanes, element) {
            return Ok(());
        }
        self.simd_guest_element = Some(syx_stdlib::resolve_alias(element));
        if offset + 16 <= memory.len() {
            memory[offset..offset + 16].copy_from_slice(&bits[..16]);
        }
        Ok(())
    }

    fn simd_wasm_fill(&mut self, slot: i32, scratch_base: i32, memory: &mut [u8]) -> SyxResult<()> {
        // Phase 113: slot >= 100 encodes element kind as `100 + element_byte` (guest LS typed fill).
        let (fill_slot, typed_element) = if slot >= 100 {
            (
                0,
                syx_stdlib::NumericKind::from_byte((slot - 100) as u8)
                    .map(syx_stdlib::resolve_alias),
            )
        } else {
            (slot, None)
        };
        let offset = scratch_base.max(0) as usize + fill_slot.max(0) as usize * 16;
        if offset + 16 > memory.len() {
            return Ok(());
        }
        let mut bits = [0u8; 64];
        bits[..16].copy_from_slice(&memory[offset..offset + 16]);
        let element = typed_element
            .or(self.simd_guest_element)
            .unwrap_or(syx_stdlib::NumericKind::I32);
        if let Some(e) = typed_element {
            self.simd_guest_element = Some(e);
        }
        let lanes = match element {
            syx_stdlib::NumericKind::F64 | syx_stdlib::NumericKind::I64 => 2,
            syx_stdlib::NumericKind::I16 => 8,
            syx_stdlib::NumericKind::I8 => 16,
            _ => 4,
        };
        self.stack.push(Value::Simd {
            lanes,
            element,
            bits,
        });
        Ok(())
    }

    /// Phase 117 guest-v10: push v128 bits onto RT (stack-native AOT) or from scratch (residual).
    fn simd_guest_push_v128(
        &mut self,
        element_byte: i32,
        scratch_base: i32,
        memory: &mut [u8],
    ) -> SyxResult<()> {
        let element = syx_stdlib::NumericKind::from_byte(element_byte as u8)
            .map(syx_stdlib::resolve_alias)
            .unwrap_or(syx_stdlib::NumericKind::I32);
        let mut bits = [0u8; 64];
        // Negative scratch_base marks stack-native: bits already written by caller into memory[0..16]
        // is unused; wasm-run linker passes bits via V128 and writes them here via dedicated path.
        // For residual mir_exec, scratch_base >= 0 reads linear memory.
        if scratch_base >= 0 {
            let offset = scratch_base as usize;
            if offset + 16 > memory.len() {
                return Ok(());
            }
            bits[..16].copy_from_slice(&memory[offset..offset + 16]);
        } else if memory.len() >= 16 {
            // Linker stack-native path: first 16 bytes of the shared scratch hold the v128 payload.
            bits[..16].copy_from_slice(&memory[..16]);
        }
        let lanes = match element {
            syx_stdlib::NumericKind::F64 | syx_stdlib::NumericKind::I64 => 2,
            syx_stdlib::NumericKind::I16 => 8,
            syx_stdlib::NumericKind::I8 => 16,
            _ => 4,
        };
        self.simd_guest_element = Some(element);
        self.stack.push(Value::Simd {
            lanes,
            element,
            bits,
        });
        Ok(())
    }

    /// Phase 117 guest-v10: pop RT simd; optionally write to scratch (residual) or leave for V128 return.
    fn simd_guest_pop_v128(&mut self, scratch_base: i32, memory: &mut [u8]) -> SyxResult<()> {
        let v = self.stack.pop().unwrap_or(Value::Nil);
        let Value::Simd {
            lanes,
            element,
            bits,
        } = v
        else {
            return Ok(());
        };
        if !Self::simd_spillable_vector(lanes, element) {
            return Ok(());
        }
        self.simd_guest_element = Some(syx_stdlib::resolve_alias(element));
        if scratch_base >= 0 {
            let offset = scratch_base as usize;
            if offset + 16 <= memory.len() {
                memory[offset..offset + 16].copy_from_slice(&bits[..16]);
            }
        } else if memory.len() >= 16 {
            memory[..16].copy_from_slice(&bits[..16]);
        }
        Ok(())
    }

    /// Phase 117: pop RT simd and return first 16 bytes as u128 (stack-native wasm V128).
    pub fn simd_guest_pop_v128_bits(&mut self) -> u128 {
        let v = self.stack.pop().unwrap_or(Value::Nil);
        let Value::Simd {
            lanes,
            element,
            bits,
        } = v
        else {
            return 0;
        };
        if !Self::simd_spillable_vector(lanes, element) {
            return 0;
        }
        self.simd_guest_element = Some(syx_stdlib::resolve_alias(element));
        u128::from_le_bytes(bits[..16].try_into().unwrap_or([0; 16]))
    }

    /// Phase 117: push u128 bits as RT simd (stack-native wasm V128).
    pub fn simd_guest_push_v128_bits(&mut self, element_byte: i32, bits128: u128) -> SyxResult<()> {
        let element = syx_stdlib::NumericKind::from_byte(element_byte as u8)
            .map(syx_stdlib::resolve_alias)
            .unwrap_or(syx_stdlib::NumericKind::I32);
        let mut bits = [0u8; 64];
        bits[..16].copy_from_slice(&bits128.to_le_bytes());
        let lanes = match element {
            syx_stdlib::NumericKind::F64 | syx_stdlib::NumericKind::I64 => 2,
            syx_stdlib::NumericKind::I16 => 8,
            syx_stdlib::NumericKind::I8 => 16,
            _ => 4,
        };
        self.simd_guest_element = Some(element);
        self.stack.push(Value::Simd {
            lanes,
            element,
            bits,
        });
        Ok(())
    }

    fn dict_build(&mut self, pair_count: u32) {
        let mut pairs = Vec::new();
        for _ in 0..pair_count {
            let key_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
            let value = self.stack.pop().unwrap_or(Value::Nil);
            pairs.push((key_val, value));
        }
        pairs.reverse();
        let mut map = std::collections::HashMap::new();
        for (key, value) in pairs {
            syx_runtime_core::dict_insert(&mut map, key, value);
        }
        if map
            .get("__type")
            .and_then(|v| match v {
                Value::Str(s) if s == "MutRefCell" => Some(()),
                _ => None,
            })
            .is_some()
        {
            // Wave A W503 / ADR-847: MutRefCell mir-run DictBuild ManagedHeap +
            // heap_slots align (VM W502 parity); ≠ MutRefCell full migrate /
            // ≠ migrate_enabled / ≠ cleanup.
            let v = Value::mut_ref_cell_from_map(map);
            self.stack.push(v);
        } else if map.contains_key("__type") {
            syx_vm::ensure_object_id(&mut map);
            // Wave A W547 / ADR-891: DictBuild ObjectRef dual-track birth
            // collapsed — heap-primary single-track via alloc_heap_slot when
            // ManagedHeap enabled (no DictBuild-site maybe_track_objectref_value).
            // Historical W499/W501 dual-register path retired at this site.
            // Wave A W550 / ADR-894: re-entry ObjectRef maybe_track_* peeled.
            // MutRefCell maybe_track_* kept. ≠ migrate / ≠ cleanup.
            // Historical cite: ADR-843 DictBuild align.
            let v = Value::object_ref_from_map(map, true);
            if syx_runtime_core::ManagedHeap::enabled() {
                self.stack.push(Value::alloc_heap_slot(
                    v,
                    syx_runtime_core::HeapSlotKind::Object,
                ));
            } else {
                self.stack.push(v);
            }
        } else if Value::heap_v4_enabled() {
            // ADR-545 / W201: HeapSlot so Dict.remove write-back matches VM (M0-B03 / ADR-224).
            let dict = Value::Dict(map);
            self.stack.push(Value::alloc_heap_slot(
                dict,
                syx_runtime_core::HeapSlotKind::Object,
            ));
        } else {
            self.stack.push(Value::Dict(map));
        }
    }

    fn object_get(&mut self) {
        // Wave A W512 / ADR-856: after OBJECT_GET re-entry, dual-register
        // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots
        // (residual birth/re-entry; VM ObjectGet parity). ≠ migrate / cleanup.
        let key_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
        let obj = self
            .stack
            .pop()
            .unwrap_or(Value::Object(Default::default()));
        let obj = match &obj {
            Value::HeapSlot { .. } => Value::resolve_heap_slot(&obj),
            other => other.clone(),
        };
        let val = obj.get_by_key(&key_val);
        self.stack.push(val);
    }

    fn set_build(&mut self, count: u32) {
        let mut items = Vec::new();
        for _ in 0..count {
            items.push(self.stack.pop().unwrap_or(Value::Nil));
        }
        items.reverse();
        // ADR-224 / M2-S1-02: HeapSlot so Set mutation write-back matches VM.
        if Value::heap_v4_enabled() {
            let set = Value::Set(items);
            self.stack.push(Value::alloc_heap_slot(
                set,
                syx_runtime_core::HeapSlotKind::Set,
            ));
        } else {
            self.stack.push(Value::Set(items));
        }
    }

    fn object_set(&mut self) {
        // Wave A W515 / ADR-859: after OBJECT_SET MutRefCell write-back,
        // dual-register stack-top ObjectRef+MutRefCell into ManagedHeap
        // + heap_slots. Wave A W532 / ADR-876: also dual-track nested
        // write-path `value` before set. ≠ migrate / ≠ ArraySet / ≠ deep
        // field-graph GC walk / ≠ enable / ≠ B-09 [x].
        let key_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
        let value = self.stack.pop().unwrap_or(Value::Nil);
        let obj = self
            .stack
            .pop()
            .unwrap_or(Value::Object(Default::default()));
        if let Value::Str(key) = key_val {
            self.stack
                .push(Value::mut_ref_cell_set_field(obj, &key, value));
        } else {
            self.stack.push(obj);
        }
    }

    fn make_closure(&mut self, fn_idx: u32, capture_count: u32) {
        let mut captures = Vec::new();
        for _ in 0..capture_count {
            captures.push(self.stack.pop().unwrap_or(Value::Nil));
        }
        captures.reverse();
        self.stack.push(Value::Closure {
            fn_index: fn_idx,
            captures,
        });
    }

    fn call_closure(&mut self, memory: &mut [u8]) -> SyxResult<()> {
        let closure = self.stack.pop().unwrap_or(Value::Nil);
        let (fn_index, captures) = match closure {
            Value::Closure { fn_index, captures } => (fn_index, captures),
            _ => return Ok(()),
        };
        let Some(func) = self.functions.get(fn_index as usize).cloned() else {
            return Ok(());
        };
        let arg_count = func.param_count as usize;
        let start = self.stack.len().saturating_sub(arg_count);
        let mut args: Vec<Value> = self.stack.drain(start..).collect();
        args.reverse();
        let saved_locals = std::mem::take(&mut self.locals);
        let mut combined = args;
        combined.extend(captures);
        let result = run_mir_function_with_args(&func, self, memory, &combined)?;
        self.locals = saved_locals;
        self.stack.push(result);
        // Wave A W507 / ADR-851: after successful call_closure return, dual-register
        // stack-top ObjectRef and MutRefCell into ManagedHeap + heap_slots
        // (parity with VM CallClosure → finish_call W506). ≠ migrate / cleanup.
        Ok(())
    }

    fn call_witness(&mut self, next_fn: u32, _state: u32) -> SyxResult<()> {
        let obj = self.stack.pop().unwrap_or(Value::Nil);
        self.stack
            .push(Value::Iterator(Box::new(IteratorState::Witness {
                obj: Box::new(obj),
                next_fn,
            })));
        Ok(())
    }

    fn await_suspend(&mut self, resume_fn: u32, state: u32) -> SyxResult<()> {
        let value = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(rt) = &self.runtime {
            if let Some(result) = rt.try_await_task_value(&value) {
                match result {
                    Ok(v) => {
                        self.stack.push(v);
                        self.suspended = false;
                    }
                    Err(e) => return Err(e),
                }
                return Ok(());
            }
            if let Ok(v) = rt.await_task_value(&value) {
                self.stack.push(v);
                self.suspended = false;
                return Ok(());
            }
        }
        cooperative_yield();
        self.suspended = true;
        self.async_resume_fn = resume_fn;
        self.async_state = state;
        self.stack.push(value);
        Ok(())
    }

    pub fn save_suspend_frame(&mut self, pc: usize, resume_fn: u32, state: u32) {
        self.suspend_frame = Some(MirSuspendFrame {
            pc,
            locals: self.locals.clone(),
            stack: self.stack.clone(),
            resume_fn,
            state,
        });
        self.async_state = state;
        self.suspended = true;
        self.async_resume_fn = resume_fn;
    }

    fn actor_send(&mut self, name_off: i32, name_len: i32, pool: &mut [u8]) -> SyxResult<()> {
        let method = read_pool_string(name_off, name_len, pool);
        let arg_count = self
            .fn_name_to_index
            .get(&method)
            .and_then(|&idx| self.functions.get(idx))
            .map(|f| f.param_count as usize)
            .unwrap_or(1)
            .max(1);
        let start = self.stack.len().saturating_sub(arg_count);
        let args: Vec<Value> = self.stack.drain(start..).collect();
        let receiver = args.first().cloned().unwrap_or(Value::Nil);
        if let Some(rt) = &self.runtime {
            match rt.actor_send_method(&receiver, &method, args) {
                Ok(result) => self.stack.push(result),
                Err(e) => {
                    // ADR-631: under try, surface C009 via pending_catch (wasm/native AOT).
                    if self.try_handlers.is_empty() {
                        return Err(e);
                    }
                    self.pending_catch = Some(diagnostic_to_catch_value(e));
                }
            }
        }
        Ok(())
    }

    fn actor_call(&mut self, name_off: i32, name_len: i32, pool: &mut [u8]) -> SyxResult<()> {
        let method = read_pool_string(name_off, name_len, pool);
        // ADR-631: drain full param_count (self + args), matching VM ActorCall.
        let arg_count = self
            .fn_name_to_index
            .get(&method)
            .and_then(|&idx| self.functions.get(idx))
            .map(|f| f.param_count as usize)
            .unwrap_or(1)
            .max(1);
        let start = self.stack.len().saturating_sub(arg_count);
        let args: Vec<Value> = self.stack.drain(start..).collect();
        let receiver = args.first().cloned().unwrap_or(Value::Nil);
        if let Some(rt) = &self.runtime {
            match rt.actor_send_method(&receiver, &method, args) {
                // Wave A W508 / ADR-852: after successful runtime ActorCall, dual-register
                // stack-top ObjectRef and MutRefCell into ManagedHeap + heap_slots
                // (parity with VM ActorCall → finish_call). ≠ MethodCall / ≠ migrate.
                Ok(result) => {
                    self.stack.push(result);
                }
                Err(e) => {
                    // ADR-631: under try, surface C009 via pending_catch (wasm/native AOT).
                    if self.try_handlers.is_empty() {
                        return Err(e);
                    }
                    self.pending_catch = Some(diagnostic_to_catch_value(e));
                }
            }
        } else if let Some(fn_idx) = self.fn_name_to_index.get(&method).copied() {
            for a in args {
                self.stack.push(a);
            }
            self.call_fn(fn_idx, pool)?;
        } else {
            self.stack.push(Value::Nil);
        }
        Ok(())
    }

    fn task_spawn_by_name(
        &mut self,
        name_off: i32,
        name_len: i32,
        pool: &mut [u8],
    ) -> SyxResult<()> {
        let start = name_off.max(0) as usize;
        let end = start.saturating_add(name_len.max(0) as usize);
        let name = if end <= pool.len() {
            std::str::from_utf8(&pool[start..end])
                .unwrap_or("")
                .to_string()
        } else {
            String::new()
        };
        let fn_idx = self.fn_name_to_index.get(&name).copied().unwrap_or(0) as u32;
        self.task_spawn(fn_idx)
    }

    fn task_spawn(&mut self, fn_idx: u32) -> SyxResult<()> {
        let name = self
            .functions
            .get(fn_idx as usize)
            .map(|f| f.name.to_string())
            .unwrap_or_else(|| format!("task_{fn_idx}"));
        let arg_count = self
            .functions
            .get(fn_idx as usize)
            .map(|f| f.param_count as usize)
            .unwrap_or(0);
        let start = self.stack.len().saturating_sub(arg_count);
        let args: Vec<Value> = self.stack.drain(start..).collect();
        if let Some(rt) = &self.runtime {
            let task = rt.spawn_detached_fn(&name, args)?;
            self.stack.push(task);
        } else {
            self.call_fn(fn_idx as usize, &mut [])?;
        }
        Ok(())
    }

    fn task_join(&mut self) -> SyxResult<()> {
        let task = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(rt) = &self.runtime {
            let result = rt.join_task_value(&task)?;
            self.stack.push(result);
        }
        Ok(())
    }

    /// Phase 55: class deinit on function return (default on).
    pub fn run_class_deinits(&mut self, _memory: &mut [u8]) -> SyxResult<()> {
        // Wave A W526 / ADR-870: run_class_deinits ObjectRef ManagedHeap +
        // heap_slots unroot (death-path first-slice; VM parity).
        // Wave A W528 / ADR-872: ObjectRef RC true-retire — no
        // object_ref_release / eager Type_deinit; finalizer via W527 collect
        // sweep. Keep maybe_untrack. ≠ Value API delete / ≠ migrate / ≠ enable.
        let locals: Vec<Value> = self.locals.clone();
        for local in locals {
            self.maybe_untrack_objectref_value(&local);
        }
        // Wave A W541 / ADR-885: handle-table drain peeled (no register/drop API).
        Ok(())
    }

    /// Phase 101: invoke `Type_deinit` (W527 collect sweep; StoreLocal/deinits
    /// no longer call this via RC — W528 / ADR-872).
    fn run_deinit_for_value(&mut self, value: &Value, memory: &mut [u8]) -> SyxResult<()> {
        let Some(type_name) = value.object_ref_type_name() else {
            return Ok(());
        };
        if matches!(
            type_name.as_str(),
            "ObjectIdentifier" | "Hasher" | "Result" | "MutRefCell"
        ) {
            return Ok(());
        }
        Value::mark_object_weak_dead(value);
        let deinit_name = format!("{type_name}_deinit");
        if let Some(&pos) = self.fn_name_to_index.get(&deinit_name) {
            self.stack.push(value.clone());
            self.call_fn(pos, memory)?;
            let _ = self.stack.pop();
        }
        Ok(())
    }
}

fn parse_task_detach_args(stack: &mut Vec<Value>) -> (String, Vec<Value>) {
    let top = stack.pop().unwrap_or(Value::Nil);
    let bottom = stack.pop();
    match (&top, bottom.as_ref()) {
        (Value::Str(n), Some(Value::Array(items))) => (n.clone(), items.clone()),
        (Value::Str(n), Some(Value::Str(n2))) => (n.clone(), vec![Value::Str(n2.clone())]),
        (Value::Array(items), Some(Value::Str(n))) => (n.clone(), items.clone()),
        (Value::Str(n), _) => (n.clone(), vec![]),
        (_, Some(Value::Str(n))) => (n.clone(), vec![]),
        _ => (String::new(), vec![]),
    }
}

fn read_pool_string(name_off: i32, name_len: i32, pool: &[u8]) -> String {
    let start = name_off.max(0) as usize;
    let end = start.saturating_add(name_len.max(0) as usize);
    if end <= pool.len() {
        std::str::from_utf8(&pool[start..end])
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

fn value_to_display(v: &Value) -> String {
    let v = opaque_unwrap(v);
    if let Some(s) = display_numeric(v) {
        return s;
    }
    match v {
        Value::Bool(b) => b.to_string(),
        Value::Str(s) => s.clone(),
        Value::Nil => "nil".into(),
        _ => value_description(v),
    }
}

fn opaque_unwrap(v: &Value) -> &Value {
    if let Value::Object(map) | Value::Dict(map) = v {
        if let Some(payload) = map.get("__opaque_payload") {
            return payload;
        }
    }
    v
}

pub fn run_mir_function(
    func: &MirFunction,
    rt: &mut MirRuntime,
    memory: &mut [u8],
) -> SyxResult<i32> {
    run_mir_function_with_args(func, rt, memory, &[]).map(mir_main_exit_code)
}

/// Normalize async/void `main` return values to process exit codes.
pub fn mir_main_exit_code(v: Value) -> i32 {
    match v {
        Value::I32(n) => n,
        Value::Bool(b) => i32::from(b),
        _ => 0,
    }
}

pub fn run_mir_function_with_args(
    func: &MirFunction,
    rt: &mut MirRuntime,
    memory: &mut [u8],
    args: &[Value],
) -> SyxResult<Value> {
    rt.begin_function(func.local_count);
    for (i, v) in args.iter().enumerate() {
        if rt.locals.len() <= i {
            rt.locals.resize(i + 1, Value::Nil);
        }
        rt.locals[i] = v.clone();
    }
    let result = run_mir_function_body(func, rt, memory);
    rt.pop_try_handler_frame();
    result
}

fn run_mir_function_body(
    func: &MirFunction,
    rt: &mut MirRuntime,
    memory: &mut [u8],
) -> SyxResult<Value> {
    if !func.instructions.is_empty() {
        run_mir_flat(func, rt, memory)
    } else {
        run_mir_blocks(func, rt, memory)
    }
}

fn run_mir_flat(func: &MirFunction, rt: &mut MirRuntime, memory: &mut [u8]) -> SyxResult<Value> {
    let ins = &func.instructions;
    let mut pc = if let Some(frame) = rt.suspend_frame.take() {
        rt.locals = frame.locals;
        rt.stack = frame.stack;
        rt.async_state = frame.state;
        rt.async_resume_fn = frame.resume_fn;
        frame.pc
    } else {
        0
    };
    rt.suspended = false;
    while pc < ins.len() {
        let instr = &ins[pc];
        pc += 1;
        match &instr.op {
            MirOp::Jump(target) => {
                pc = *target as usize;
            }
            MirOp::JumpIfFalse(target) => {
                if !rt.pop_bool() {
                    pc = *target as usize;
                }
            }
            MirOp::JumpIfNil(target) => {
                let top = rt.stack.last().cloned().unwrap_or(Value::Nil);
                if matches!(top, Value::Nil) {
                    rt.stack.pop();
                    pc = *target as usize;
                }
            }
            MirOp::Return => {
                rt.run_class_deinits(memory)?;
                return Ok(rt.stack.pop().unwrap_or(Value::Nil));
            }
            MirOp::BindResult(target) => {
                if let Some(err) = rt.pending_catch.take() {
                    rt.stack.push(err);
                    pc = *target as usize;
                }
            }
            MirOp::ReturnErr => {
                let err = rt.stack.pop().unwrap_or(Value::Str("error".into()));
                rt.pending_catch = Some(err);
                return Ok(Value::Nil);
            }
            MirOp::Call(name) => {
                if let Some(pos) = rt.functions.iter().position(|f| f.name == *name) {
                    rt.call_fn(pos, memory)?;
                    let catch = rt.take_catch_pc();
                    if catch >= 0 {
                        pc = catch as usize;
                    }
                }
            }
            MirOp::AwaitSuspend { resume_fn, state } => {
                rt.exec_mir_instr(instr, memory)?;
                if rt.suspended {
                    rt.save_suspend_frame(pc, *resume_fn, *state);
                    return Ok(Value::Nil);
                }
            }
            _ => {
                rt.exec_mir_instr(instr, memory)?;
                if rt.suspended {
                    return Ok(Value::Nil);
                }
                // Wave A W202 / ADR-546: pop handler (take_catch_pc) so Throw in catch
                // cannot re-enter the same pad (Catch→Throw infinite loop).
                let catch = rt.take_catch_pc();
                if catch >= 0 {
                    pc = catch as usize;
                }
            }
        }
    }
    Ok(rt.stack.pop().unwrap_or(Value::Nil))
}

fn run_mir_blocks(func: &MirFunction, rt: &mut MirRuntime, memory: &mut [u8]) -> SyxResult<Value> {
    let block_ids: Vec<u32> = func.blocks.iter().map(|b| b.id).collect();
    let block_index: HashMap<u32, usize> = block_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();
    let mut pc = block_index.get(&func.entry).copied().unwrap_or(0);
    loop {
        let Some(block) = func.blocks.get(pc) else {
            break;
        };
        let mut jumped = false;
        for instr in &block.instructions {
            match &instr.op {
                MirOp::Jump(target) => {
                    pc = block_index.get(target).copied().unwrap_or(pc + 1);
                    jumped = true;
                    break;
                }
                MirOp::JumpIfFalse(target) => {
                    if !rt.pop_bool() {
                        pc = block_index.get(target).copied().unwrap_or(pc + 1);
                        jumped = true;
                        break;
                    }
                }
                MirOp::JumpIfNil(target) => {
                    let top = rt.stack.last().cloned().unwrap_or(Value::Nil);
                    if matches!(top, Value::Nil) {
                        rt.stack.pop();
                        pc = block_index.get(target).copied().unwrap_or(pc + 1);
                        jumped = true;
                        break;
                    }
                }
                MirOp::Return => {
                    rt.run_class_deinits(memory)?;
                    return Ok(rt.stack.pop().unwrap_or(Value::Nil));
                }
                MirOp::BindResult(target) => {
                    if let Some(err) = rt.pending_catch.take() {
                        rt.stack.push(err);
                        pc = block_index.get(target).copied().unwrap_or(pc + 1);
                        jumped = true;
                        break;
                    }
                }
                MirOp::ReturnErr => {
                    let err = rt.stack.pop().unwrap_or(Value::Str("error".into()));
                    rt.pending_catch = Some(err);
                    return Ok(Value::Nil);
                }
                MirOp::Call(name) => {
                    if let Some(pos) = rt.functions.iter().position(|f| f.name == *name) {
                        rt.call_fn(pos, memory)?;
                        let catch = rt.take_catch();
                        if catch >= 0 {
                            pc = block_index.get(&(catch as u32)).copied().unwrap_or(pc + 1);
                            jumped = true;
                            break;
                        }
                    }
                }
                _other => {
                    rt.exec_mir_instr(instr, memory)?;
                    // Wave A W202 / ADR-546: pop handler via take_catch (parity with flat/wasm).
                    let catch = rt.take_catch();
                    if catch >= 0 {
                        pc = block_index.get(&(catch as u32)).copied().unwrap_or(pc + 1);
                        jumped = true;
                        break;
                    }
                }
            }
        }
        if jumped {
            continue;
        }
        if pc + 1 >= func.blocks.len() {
            break;
        }
        pc += 1;
    }
    Ok(rt.stack.pop().unwrap_or(Value::Nil))
}

/// Build runtime string pool offsets for ops that read names via `exec` (ConstStr, Actor*, TaskSpawn).
pub fn module_runtime_string_offsets(
    module: &syx_mir::MirModule,
) -> (Vec<u8>, HashMap<String, (i32, i32)>) {
    let mut rt = MirRuntime::new();
    let mut offsets = HashMap::new();
    for func in &module.functions {
        for block in &func.blocks {
            for instr in &block.instructions {
                let key = match &instr.op {
                    syx_mir::MirOp::ConstStr(s) => Some(s.to_string()),
                    syx_mir::MirOp::ActorCall(n)
                    | syx_mir::MirOp::ActorSend(n)
                    | syx_mir::MirOp::TaskSpawn(n) => Some(n.to_string()),
                    _ => None,
                };
                if let Some(s) = key {
                    offsets
                        .entry(s.clone())
                        .or_insert_with(|| rt.intern_string(&s));
                }
            }
        }
    }
    (rt.string_pool, offsets)
}

pub fn install_module_string_pool(rt: &mut MirRuntime, module: &syx_mir::MirModule) {
    let (pool, _) = module_runtime_string_offsets(module);
    rt.string_pool = pool;
}

pub fn mir_op_immediates(op: &MirOp, memory: &mut [u8], rt: &mut MirRuntime) -> (u32, i32, i32) {
    match op {
        MirOp::ConstI32(v) => (op::CONST_I32, *v, 0),
        MirOp::ConstNumeric { kind, bits } => (op::CONST_NUMERIC, *kind as i32, *bits as i32),
        MirOp::ConstStr(s) => {
            let (off, len) = rt.intern_string(s);
            let _ = memory;
            (op::CONST_STR, off, len)
        }
        MirOp::ConstBool(v) => (op::CONST_BOOL, i32::from(*v), 0),
        MirOp::ConstNil => (op::CONST_NIL, 0, 0),
        MirOp::LoadLocal(i) => (op::LOAD_LOCAL, *i as i32, 0),
        MirOp::StoreLocal(i) => (op::STORE_LOCAL, *i as i32, 0),
        MirOp::LoadGlobal(i) => (op::LOAD_GLOBAL, *i as i32, 0),
        MirOp::StoreGlobal(i) => (op::STORE_GLOBAL, *i as i32, 0),
        // `b=1` marks kind immediate as valid (WASM StackOp passes a=0,b=0 → infer).
        MirOp::Add { kind } => (op::ADD, *kind as i32, 1),
        MirOp::Sub { kind } => (op::SUB, *kind as i32, 1),
        MirOp::Mul { kind } => (op::MUL, *kind as i32, 1),
        MirOp::Div { kind } => (op::DIV, *kind as i32, 1),
        MirOp::Mod { kind } => (op::MOD, *kind as i32, 1),
        MirOp::BitAnd { kind } => (op::BIT_AND, *kind as i32, 1),
        MirOp::BitOr { kind } => (op::BIT_OR, *kind as i32, 1),
        MirOp::BitXor { kind } => (op::BIT_XOR, *kind as i32, 1),
        MirOp::Shl { kind } => (op::SHL, *kind as i32, 1),
        MirOp::Shr { kind } => (op::SHR, *kind as i32, 1),
        MirOp::Eq => (op::EQ, 0, 0),
        MirOp::Ne => (op::NE, 0, 0),
        MirOp::Lt { kind } => (op::LT, *kind as i32, 1),
        MirOp::Le { kind } => (op::LE, *kind as i32, 1),
        MirOp::Gt { kind } => (op::GT, *kind as i32, 1),
        MirOp::Ge { kind } => (op::GE, *kind as i32, 1),
        MirOp::And => (op::AND, 0, 0),
        MirOp::Or => (op::OR, 0, 0),
        MirOp::Not => (op::NOT, 0, 0),
        MirOp::Neg { kind } => (op::NEG, *kind as i32, 1),
        MirOp::BitNot { kind } => (op::BIT_NOT, *kind as i32, 1),
        MirOp::Print => (op::PRINT, 0, 0),
        MirOp::ConcatStr => (op::CONCAT_STR, 0, 0),
        MirOp::ToStr => (op::TO_STR, 0, 0),
        MirOp::Pop => (op::POP, 0, 0),
        MirOp::Dup => (op::DUP, 0, 0),
        MirOp::CallHost(i) => (op::CALL_HOST, *i as i32, 0),
        MirOp::MakeIterator => (op::MAKE_ITERATOR, 0, 0),
        MirOp::IteratorNext => (op::ITERATOR_NEXT, 0, 0),
        MirOp::ArrayBuild(n) => (op::ARRAY_BUILD, *n as i32, 0),
        MirOp::SetBuild(n) => (op::SET_BUILD, *n as i32, 0),
        MirOp::ArrayGet => (op::ARRAY_GET, 0, 0),
        MirOp::ArraySet => (op::ARRAY_SET, 0, 0),
        MirOp::ObjectGet => (op::OBJECT_GET, 0, 0),
        MirOp::ObjectSet => (op::OBJECT_SET, 0, 0),
        MirOp::HeapCompact => (op::HEAP_COMPACT, 0, 0),
        MirOp::DictBuild(n) => (op::DICT_BUILD, *n as i32, 0),
        MirOp::AwaitPoll => (op::AWAIT_POLL, 0, 0),
        MirOp::AwaitSuspend { resume_fn, state } => {
            (op::AWAIT_SUSPEND, *resume_fn as i32, *state as i32)
        }
        MirOp::ActorCall(name) => {
            let (off, len) = rt.intern_string(name);
            (op::ACTOR_CALL, off, len)
        }
        MirOp::ActorRegister => (op::ACTOR_REGISTER, 0, 0),
        MirOp::ActorSend(name) => {
            let (off, len) = rt.intern_string(name);
            (op::ACTOR_SEND, off, len)
        }
        MirOp::CallWitness(a, b) => (op::CALL_WITNESS, *a as i32, *b as i32),
        MirOp::MakeClosure(name, cap) => {
            let idx = rt.fn_name_to_index.get(name.as_str()).copied().unwrap_or(0) as i32;
            (op::MAKE_CLOSURE, idx, *cap as i32)
        }
        MirOp::CallClosure => (op::CALL_CLOSURE, 0, 0),
        MirOp::ValueEq => (op::VALUE_EQ, 0, 0),
        MirOp::OpaqueValueEq => (op::VALUE_EQ, 0, 0),
        MirOp::ValueNe => (op::VALUE_NE, 0, 0),
        MirOp::RefEq => (op::REF_EQ, 0, 0),
        MirOp::RefNe => (op::REF_NE, 0, 0),
        MirOp::ArrSorted => (op::ARRAY_SORTED, 0, 0),
        MirOp::NumericCast { from, to } => (op::NUMERIC_CAST, *from as i32, *to as i32),
        MirOp::BeginTry(catch) => (op::BEGIN_TRY, *catch as i32, 0),
        MirOp::EndTry => (op::END_TRY, 0, 0),
        MirOp::Catch => (op::CATCH, 0, 0),
        MirOp::Throw => (op::THROW, 0, 0),
        MirOp::BindResult(err) => (op::BIND_RESULT, *err as i32, 0),
        MirOp::ReturnErr => (op::RETURN_ERR, 0, 0),
        MirOp::TaskSpawn(name) => {
            let (off, len) = rt.intern_string(name);
            (op::TASK_SPAWN, off, len)
        }
        MirOp::TaskJoin => (op::TASK_JOIN, 0, 0),
        MirOp::SimdAdd => (op::SIMD_ADD, 0, 0),
        MirOp::SimdSub => (op::SIMD_SUB, 0, 0),
        MirOp::SimdMul => (op::SIMD_MUL, 0, 0),
        MirOp::SimdDiv => (op::SIMD_DIV, 0, 0),
        MirOp::SimdEq => (op::SIMD_EQ, 0, 0),
        MirOp::SimdLt => (op::SIMD_LT, 0, 0),
        MirOp::SimdGt => (op::SIMD_GT, 0, 0),
        MirOp::SimdNe => (op::SIMD_NE, 0, 0),
        MirOp::SimdLe => (op::SIMD_LE, 0, 0),
        MirOp::SimdGe => (op::SIMD_GE, 0, 0),
        MirOp::SimdAnd => (op::SIMD_AND, 0, 0),
        MirOp::SimdOr => (op::SIMD_OR, 0, 0),
        MirOp::SimdXor => (op::SIMD_XOR, 0, 0),
        MirOp::SimdNot => (op::SIMD_NOT, 0, 0),
        MirOp::SimdShl => (op::SIMD_SHL, 0, 0),
        MirOp::SimdShr => (op::SIMD_SHR, 0, 0),
        MirOp::SimdMin => (op::SIMD_MIN, 0, 0),
        MirOp::SimdMax => (op::SIMD_MAX, 0, 0),
        MirOp::SimdF32Add => (op::SIMD_F32_ADD, 0, 0),
        MirOp::SimdF32Sub => (op::SIMD_F32_SUB, 0, 0),
        MirOp::SimdF32Mul => (op::SIMD_F32_MUL, 0, 0),
        MirOp::SimdF32Div => (op::SIMD_F32_DIV, 0, 0),
        MirOp::SimdF64Add => (op::SIMD_F64_ADD, 0, 0),
        MirOp::SimdI16Add => (op::SIMD_I16_ADD, 0, 0),
        MirOp::SimdI16Sub => (op::SIMD_I16_SUB, 0, 0),
        MirOp::SimdI16Mul => (op::SIMD_I16_MUL, 0, 0),
        MirOp::SimdI8Add => (op::SIMD_I8_ADD, 0, 0),
        MirOp::SimdI8Sub => (op::SIMD_I8_SUB, 0, 0),
        MirOp::SimdI8Mul => (op::SIMD_I8_MUL, 0, 0),
        MirOp::SimdI8And => (op::SIMD_I8_AND, 0, 0),
        MirOp::SimdI8Or => (op::SIMD_I8_OR, 0, 0),
        MirOp::SimdI8Xor => (op::SIMD_I8_XOR, 0, 0),
        MirOp::SimdI8Not => (op::SIMD_I8_NOT, 0, 0),
        MirOp::SimdI8Min => (op::SIMD_I8_MIN, 0, 0),
        MirOp::SimdI8Max => (op::SIMD_I8_MAX, 0, 0),
        MirOp::SimdI8UMin => (op::SIMD_I8_UMIN, 0, 0),
        MirOp::SimdI8UMax => (op::SIMD_I8_UMAX, 0, 0),
        MirOp::SimdI8Abs => (op::SIMD_I8_ABS, 0, 0),
        MirOp::SimdI8Neg => (op::SIMD_I8_NEG, 0, 0),
        MirOp::SimdI8Extract => (op::SIMD_I8_EXTRACT, 0, 0),
        MirOp::SimdI8Replace => (op::SIMD_I8_REPLACE, 0, 0),
        MirOp::SimdI8Eq => (op::SIMD_I8_EQ, 0, 0),
        MirOp::SimdI8Ne => (op::SIMD_I8_NE, 0, 0),
        MirOp::SimdI8Le => (op::SIMD_I8_LE, 0, 0),
        MirOp::SimdI8Ge => (op::SIMD_I8_GE, 0, 0),
        MirOp::SimdI8AvgrU => (op::SIMD_I8_AVGR_U, 0, 0),
        MirOp::SimdI8Popcnt => (op::SIMD_I8_POPCNT, 0, 0),
        MirOp::SimdI8Bitmask => (op::SIMD_I8_BITMASK, 0, 0),
        MirOp::SimdI8AllTrue => (op::SIMD_I8_ALL_TRUE, 0, 0),
        MirOp::SimdI8AnyTrue => (op::SIMD_I8_ANY_TRUE, 0, 0),
        MirOp::SimdI8Lt => (op::SIMD_I8_LT, 0, 0),
        MirOp::SimdI8Gt => (op::SIMD_I8_GT, 0, 0),
        MirOp::SimdI8AddSat => (op::SIMD_I8_ADD_SAT, 0, 0),
        MirOp::SimdI8SubSat => (op::SIMD_I8_SUB_SAT, 0, 0),
        MirOp::SimdI8Shuffle { mode } => (op::SIMD_I8_SHUFFLE, *mode, 0),
        MirOp::SimdI8Shl => (op::SIMD_I8_SHL, 0, 0),
        MirOp::SimdI8Shr => (op::SIMD_I8_SHR, 0, 0),
        MirOp::SimdI64Add => (op::SIMD_I64_ADD, 0, 0),
        MirOp::SimdF64Sub => (op::SIMD_F64_SUB, 0, 0),
        MirOp::SimdF64Mul => (op::SIMD_F64_MUL, 0, 0),
        MirOp::SimdF64Div => (op::SIMD_F64_DIV, 0, 0),
        MirOp::SimdReduceAdd { .. } => (0, 0, 0),
        MirOp::SimdShuffle { mode } => (op::SIMD_SHUFFLE, *mode, 0),
        MirOp::SimdLoad { lanes, element } => (op::SIMD_LOAD, simd_tag(*lanes, *element), 0),
        MirOp::SimdStore { lanes, element } => (op::SIMD_STORE, simd_tag(*lanes, *element), 0),
        MirOp::ConstNumeric128 { kind, lo, hi } => {
            rt.pending_const128_lo = *lo;
            rt.pending_const128_hi = *hi;
            (op::CONST_NUMERIC128, *kind as i32, 0)
        }
        _ => (0, 0, 0),
    }
}

fn simd_tag(lanes: u8, element: u8) -> i32 {
    (lanes as i32) | ((element as i32) << 8)
}

pub fn simd_tag_decode(tag: i32) -> (u8, u8) {
    ((tag & 0xFF) as u8, ((tag >> 8) & 0xFF) as u8)
}

// Phase 88: thread-local active MIR runtime for HOF host closure execution (driver/MIR/WASM parity).
std::thread_local! {
    static ACTIVE_MIR_RT: std::cell::RefCell<Option<*mut MirRuntime>> = const { std::cell::RefCell::new(None) };
}

/// Register `rt` as the active MIR closure target until [`unpin_active_mir_runtime`].
pub fn pin_active_mir_runtime(rt: &mut MirRuntime) {
    ACTIVE_MIR_RT.with(|cell| {
        *cell.borrow_mut() = Some(rt as *mut MirRuntime);
    });
}

pub fn unpin_active_mir_runtime() {
    ACTIVE_MIR_RT.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

/// Run `f` while `rt` is the active MIR closure target for `syx_vm::execute_closure` fallback.
pub fn with_active_mir_runtime<R>(rt: &mut MirRuntime, f: impl FnOnce() -> R) -> R {
    pin_active_mir_runtime(rt);
    let out = f();
    unpin_active_mir_runtime();
    out
}

/// True when a MIR runtime is pinned for HOF host closure execution.
pub fn mir_closure_runtime_active() -> bool {
    ACTIVE_MIR_RT.with(|cell| cell.borrow().is_some())
}

/// Execute a MIR closure when [`with_active_mir_runtime`] is active.
pub fn execute_mir_closure(fn_index: u32, captures: &[Value], args: &[Value]) -> Value {
    ACTIVE_MIR_RT.with(|cell| {
        let Some(rt_ptr) = *cell.borrow() else {
            return Value::Nil;
        };
        let rt = unsafe { &mut *rt_ptr };
        let mut pool = rt.string_pool.clone();
        rt.run_closure_with_captures(fn_index, captures, args, &mut pool)
            .unwrap_or(Value::Nil)
    })
}

#[cfg(test)]
mod tests {
    use syx_diagnostics::SourceFile;
    use syx_mir::lower_module;
    use syx_parser::Parser;

    use super::*;

    #[test]

    fn actor_string_pool_includes_method_names() {
        let source = SourceFile::new(
            "t.sy",
            r#"actor Counter {
    var value: Int
    init(value: Int) { self.value = value }
    func increment() { self.value = self.value + 1 }
    func get() -> Int { return self.value }
}
func main() {
    let c = Counter(value: 0)
    c.increment()
    print(c.get())
}"#,
        );
        let ast = Parser::parse(source).unwrap();
        let mir = lower_module(&ast).unwrap();
        let (pool, map) = module_runtime_string_offsets(&mir);
        assert!(map.contains_key("Counter_increment"), "map={map:?}");
        assert!(map.contains_key("Counter_get"), "map={map:?}");
        assert!(!pool.is_empty());
    }

    #[test]
    fn hasher_local_slot_order_in_main() {
        let source = SourceFile::new(
            "t.sy",
            r#"
import Hasher
func main() {
    let b: Int = 3
    var h = Hasher.newHasher()
    b.hash(into: h)
    print(Hasher.finalize(h))
    return 0
}
"#,
        );
        let ast = Parser::parse(source).unwrap();
        let mir = lower_module(&ast).unwrap();
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert_eq!(main.local_count, 2);
        assert_eq!(
            main.blocks.len(),
            1,
            "expected single-block main for hasher fixture, got {} blocks",
            main.blocks.len()
        );
        let ops: Vec<_> = main
            .blocks
            .iter()
            .flat_map(|b| b.instructions.iter())
            .map(|i| format!("{:?}", i.op))
            .collect();
        assert!(
            ops.iter().any(|o| o.contains("StoreLocal(1)")),
            "ops={ops:?}"
        );
    }

    #[test]
    fn hasher_hash_into_updates_local_via_mir_interpreter() {
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;
        let source = SourceFile::new(
            "t.sy",
            r#"
import Hasher
func main() {
    let b: Int = 3
    var h = Hasher.newHasher()
    b.hash(into: h)
    print(Hasher.finalize(h))
    return 0
}
"#,
        );
        let ast = Parser::parse(source).unwrap();
        let mut mir = lower_module(&ast).unwrap();
        syx_mir::optimize(&mut mir);
        let mut rt = MirRuntime::new();
        rt.install_functions(mir.functions.clone());
        install_module_string_pool(&mut rt, &mir);
        let main = rt
            .functions
            .iter()
            .find(|f| f.name == "main")
            .unwrap()
            .clone();
        let _ = run_mir_function(&main, &mut rt, &mut []).unwrap();
        assert_eq!(rt.printed.len(), 1);
        let printed = rt.printed[0].parse::<u64>().expect("numeric hash");
        assert_ne!(
            printed, FNV_OFFSET,
            "hash(into:) should update hasher local before finalize"
        );
    }

    #[test]
    fn filter_closure_via_mir_interpreter() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let nums = [1, 2, 3, 4]
    let evens = nums.filter({ |x| x % 2 == 0 })
    print(evens.len())
    return 0
}"#,
        );
        let ast = Parser::parse(source).unwrap();
        let mir = lower_module(&ast).unwrap();
        let mut rt = MirRuntime::new();
        rt.install_functions(mir.functions.clone());
        let main = rt
            .functions
            .iter()
            .find(|f| f.name == "main")
            .unwrap()
            .clone();
        let code = run_mir_function(&main, &mut rt, &mut []).unwrap();
        assert_eq!(code, 0, "main should return 0");
    }

    #[test]
    fn collect_live_roots_includes_suspend_frame_and_locals_frames() {
        // Wave A W487 / ADR-831: suspend_frame + locals_frames must enter root set.
        let mut rt = MirRuntime::new();
        rt.stack.push(Value::I64(1));
        rt.locals.push(Value::I64(2));
        rt.locals_frames
            .push(vec![Value::I64(42), Value::Str("frame".into())]);
        rt.suspend_frame = Some(MirSuspendFrame {
            pc: 0,
            locals: vec![Value::I64(99)],
            stack: vec![Value::Str("suspended".into())],
            resume_fn: 0,
            state: 0,
        });
        let roots = rt.collect_live_roots();
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(42))),
            "locals_frames value must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "frame")),
            "locals_frames str must be a root: {roots:?}"
        );
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(99))),
            "suspend_frame locals must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "suspended")),
            "suspend_frame stack must be a root: {roots:?}"
        );
    }

    #[test]
    fn collect_live_roots_includes_mailbox_value_queue() {
        // Wave A W489 / ADR-833: mailbox queue Values must enter MirRuntime roots.
        let id = 77_501;
        while syx_vm::actor_mailbox::mailbox_dequeue(id).is_some() {}
        syx_vm::actor_mailbox::mailbox_enqueue(
            id,
            &[Value::I64(555), Value::Str("mir-mbox".into())],
        );
        let rt = MirRuntime::new();
        let roots = rt.collect_live_roots();
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(555))),
            "mailbox queue I64 must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "mir-mbox")),
            "mailbox queue Str must be a root: {roots:?}"
        );
        let _ = syx_vm::actor_mailbox::mailbox_dequeue(id);
    }

    #[test]
    fn heap_compact_rewrites_suspend_and_locals_frames_slots() {
        let mut rt = MirRuntime::new();
        let slot = {
            let mut heap = syx_runtime_core::global_heap().lock().unwrap();
            heap.alloc(Value::Str("keep".into()))
        };
        rt.locals_frames.push(vec![Value::HeapSlot {
            idx: slot,
            kind: syx_runtime_core::HeapSlotKind::Object,
        }]);
        rt.suspend_frame = Some(MirSuspendFrame {
            pc: 0,
            locals: vec![Value::HeapSlot {
                idx: slot,
                kind: syx_runtime_core::HeapSlotKind::Object,
            }],
            stack: vec![],
            resume_fn: 0,
            state: 0,
        });
        rt.heap_slots.push(slot);
        rt.heap_compact();
        let frame_slot = match &rt.locals_frames[0][0] {
            Value::HeapSlot { idx, .. } => *idx,
            other => panic!("expected HeapSlot in locals_frames, got {other:?}"),
        };
        let suspend_slot = match &rt.suspend_frame.as_ref().unwrap().locals[0] {
            Value::HeapSlot { idx, .. } => *idx,
            other => panic!("expected HeapSlot in suspend_frame, got {other:?}"),
        };
        assert_eq!(frame_slot, suspend_slot);
        let heap = syx_runtime_core::global_heap().lock().unwrap();
        assert!(
            matches!(heap.get(frame_slot), Some(Value::Str(s)) if s == "keep"),
            "compact must keep suspended/frame-rooted slot live"
        );
    }
}
