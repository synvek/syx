//! ADR-140 / M1 W4: Cranelift JIT tier-2 — Suite-1 pilot MirOp true lowering + native invoke.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, AbiParam, Block, InstBuilder, Value as ClifValue};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::{MirFunction, MirInstr, MirModule, MirOp};
use target_lexicon::Triple;

/// Compiled tier-2 native entry metadata + callable pointer.
#[derive(Debug, Clone)]
pub struct JitNativeEntry {
    pub func_name: String,
    pub code_len: usize,
    pub deopt_version: u32,
    pub native: bool,
    /// Finalized code pointer (`*const u8` as usize). Valid while JIT module lives.
    pub code_ptr: usize,
    pub param_count: u32,
}

struct JitState {
    module: JITModule,
    func_ids: HashMap<String, FuncId>,
    entries: HashMap<String, JitNativeEntry>,
}

static JIT_STATE: OnceLock<Mutex<Option<JitState>>> = OnceLock::new();
static DEOPT_COUNT: AtomicU64 = AtomicU64::new(0);
static NATIVE_CALL_COUNT: AtomicU64 = AtomicU64::new(0);
static LAST_DEOPT_REASON: Mutex<Option<String>> = Mutex::new(None);

fn jit_state_lock() -> &'static Mutex<Option<JitState>> {
    JIT_STATE.get_or_init(|| Mutex::new(None))
}

/// ADR-140: tier-2 is opt-in — default **off** unless `SYX_JIT_TIER2=1`.
pub fn cranelift_jit_tier2_enabled() -> bool {
    match std::env::var("SYX_JIT_TIER2") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => false,
    }
}

/// Historical name: only true when explicitly enabled + barriers.
pub fn cranelift_jit_tier2_prod_ready() -> bool {
    cranelift_jit_tier2_enabled()
        && syx_mir::mir_opt_v25_aot_barrier_enabled()
        && cranelift_jit_tier2_smoke_seasons() >= 1
}

pub fn cranelift_jit_tier2_smoke_seasons() -> u32 {
    std::env::var("SYX_JIT_TIER2_SMOKE_SEASONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

pub fn cranelift_jit_deopt_version() -> u32 {
    let mut v: u32 = 140;
    if syx_mir::mir_opt_v25_aot_barrier_enabled() {
        v += 1;
    }
    v
}

pub fn deopt_count() -> u64 {
    DEOPT_COUNT.load(Ordering::Relaxed)
}

pub fn jit_native_call_count() -> u64 {
    NATIVE_CALL_COUNT.load(Ordering::Relaxed)
}

pub fn last_deopt_reason() -> Option<String> {
    LAST_DEOPT_REASON.lock().ok().and_then(|g| g.clone())
}

pub fn record_deopt_reason(reason: &str) {
    DEOPT_COUNT.fetch_add(1, Ordering::Relaxed);
    if let Ok(mut g) = LAST_DEOPT_REASON.lock() {
        *g = Some(reason.to_string());
    }
}

fn ensure_state(state: &mut Option<JitState>) -> SyxResult<&mut JitState> {
    if state.is_none() {
        *state = Some(new_jit_state()?);
    }
    Ok(state.as_mut().expect("jit state"))
}

fn new_jit_state() -> SyxResult<JitState> {
    let triple = Triple::host();
    let isa_builder = cranelift_codegen::isa::lookup(triple)
        .map_err(|e| SyxDiagnostic::error(format!("ISA lookup failed: {e}")))?;
    let mut flag_builder = settings::builder();
    flag_builder
        .set("opt_level", "speed")
        .map_err(|e| SyxDiagnostic::error(e.to_string()))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| SyxDiagnostic::error(format!("ISA build failed: {e}")))?;

    let mut jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    jit_builder.symbol("syx_vm_deopt", syx_vm_deopt_trampoline as *const u8);
    Ok(JitState {
        module: JITModule::new(jit_builder),
        func_ids: HashMap::new(),
        entries: HashMap::new(),
    })
}

fn signature_for(param_count: u32) -> cranelift_codegen::ir::Signature {
    let mut sig = cranelift_codegen::ir::Signature::new(CallConv::SystemV);
    for _ in 0..param_count {
        sig.params.push(AbiParam::new(types::I32));
    }
    sig.returns.push(AbiParam::new(types::I32));
    sig
}

fn mir_ops_iter(func: &MirFunction) -> Vec<&MirInstr> {
    if !func.blocks.is_empty() {
        func.blocks
            .iter()
            .flat_map(|b| b.instructions.iter())
            .collect()
    } else {
        func.instructions.iter().collect()
    }
}

/// Observable deopt/compile-failure reason for unsupported MirOps.
pub fn unsupported_mir_op_reason(op: &MirOp) -> Option<String> {
    match op {
        MirOp::ConstI32(_)
        | MirOp::ConstNumeric { .. }
        | MirOp::ConstBool(_)
        | MirOp::ConstNil
        | MirOp::LoadLocal(_)
        | MirOp::StoreLocal(_)
        | MirOp::Add { .. }
        | MirOp::Sub { .. }
        | MirOp::Mul { .. }
        | MirOp::Div { .. }
        | MirOp::Mod { .. }
        | MirOp::Eq
        | MirOp::Ne
        | MirOp::Lt { .. }
        | MirOp::Le { .. }
        | MirOp::Gt { .. }
        | MirOp::Ge { .. }
        | MirOp::Neg { .. }
        | MirOp::Not
        | MirOp::And
        | MirOp::Or
        | MirOp::Pop
        | MirOp::Dup
        | MirOp::Jump(_)
        | MirOp::JumpIfFalse(_)
        | MirOp::Call(_)
        | MirOp::Return => None,
        MirOp::Print => Some("UnsupportedMirOp(Print)".into()),
        MirOp::CallHost(_) => Some("UnsupportedMirOp(CallHost)".into()),
        MirOp::ConstStr(_) => Some("UnsupportedMirOp(ConstStr)".into()),
        other => Some(format!("UnsupportedMirOp({other:?})")),
    }
}

fn check_function_supported(func: &MirFunction) -> SyxResult<()> {
    for insn in mir_ops_iter(func) {
        if let Some(reason) = unsupported_mir_op_reason(&insn.op) {
            return Err(SyxDiagnostic::error(reason));
        }
    }
    Ok(())
}

/// Compile a MIR function to native code via Cranelift JIT (tier-2).
pub fn compile_hot_function(module: &MirModule, func_name: &str) -> SyxResult<JitNativeEntry> {
    {
        let guard = jit_state_lock()
            .lock()
            .map_err(|_| SyxDiagnostic::error("JIT state lock poisoned"))?;
        if let Some(st) = guard.as_ref() {
            if let Some(entry) = st.entries.get(func_name) {
                return Ok(entry.clone());
            }
        }
    }

    let mir_fn = module
        .functions
        .iter()
        .find(|f| f.name == func_name)
        .ok_or_else(|| SyxDiagnostic::error(format!("JIT: unknown function {func_name}")))?
        .clone();

    check_function_supported(&mir_fn)?;

    for insn in mir_ops_iter(&mir_fn) {
        if let MirOp::Call(callee) = &insn.op {
            let callee = callee.as_str();
            if callee != func_name {
                let _ = compile_hot_function(module, callee)?;
            }
        }
    }

    let mut guard = jit_state_lock()
        .lock()
        .map_err(|_| SyxDiagnostic::error("JIT state lock poisoned"))?;
    let state = ensure_state(&mut guard)?;

    if let Some(entry) = state.entries.get(func_name) {
        return Ok(entry.clone());
    }

    let export_name = format!("syx_jit_{func_name}");
    let sig = signature_for(mir_fn.param_count);
    let func_id = if let Some(id) = state.func_ids.get(func_name).copied() {
        id
    } else {
        let id = state
            .module
            .declare_function(&export_name, Linkage::Export, &sig)
            .map_err(|e| SyxDiagnostic::error(format!("JIT declare: {e}")))?;
        state.func_ids.insert(func_name.to_string(), id);
        id
    };

    for insn in mir_ops_iter(&mir_fn) {
        if let MirOp::Call(callee) = &insn.op {
            let cname = callee.as_str();
            if !state.func_ids.contains_key(cname) {
                let cfn = module
                    .functions
                    .iter()
                    .find(|f| f.name == cname)
                    .ok_or_else(|| SyxDiagnostic::error(format!("JIT: unknown callee {cname}")))?;
                let csig = signature_for(cfn.param_count);
                let cid = state
                    .module
                    .declare_function(&format!("syx_jit_{cname}"), Linkage::Export, &csig)
                    .map_err(|e| SyxDiagnostic::error(format!("JIT declare callee: {e}")))?;
                state.func_ids.insert(cname.to_string(), cid);
            }
        }
    }

    let func_ids = state.func_ids.clone();
    let mut ctx = cranelift_codegen::Context::new();
    ctx.func.signature = sig;
    {
        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);
        if !mir_fn.blocks.is_empty() {
            lower_blocks(&mut state.module, module, &mir_fn, &mut builder, &func_ids)?;
        } else {
            lower_flat(&mut state.module, module, &mir_fn, &mut builder, &func_ids)?;
        }
        builder.finalize();
    }

    state
        .module
        .define_function(func_id, &mut ctx)
        .map_err(|e| SyxDiagnostic::error(format!("JIT define: {e}")))?;
    state.module.clear_context(&mut ctx);
    state
        .module
        .finalize_definitions()
        .map_err(|e| SyxDiagnostic::error(format!("JIT finalize: {e}")))?;

    let code_ptr = state.module.get_finalized_function(func_id);
    let code_len = ctx.compiled_code().map(|c| c.code_buffer().len()).unwrap_or(0);
    if code_ptr.is_null() {
        return Err(SyxDiagnostic::error("JIT: null code pointer"));
    }

    let entry = JitNativeEntry {
        func_name: func_name.to_string(),
        code_len,
        deopt_version: cranelift_jit_deopt_version(),
        native: true,
        code_ptr: code_ptr as usize,
        param_count: mir_fn.param_count,
    };
    state.entries.insert(func_name.to_string(), entry.clone());
    Ok(entry)
}

fn declare_locals(
    builder: &mut FunctionBuilder<'_>,
    entry: Block,
    param_count: u32,
    local_count: u32,
) {
    builder.append_block_params_for_function_params(entry);
    builder.switch_to_block(entry);
    let params: Vec<ClifValue> = builder.block_params(entry).to_vec();
    let n = local_count.max(param_count);
    for i in 0..n {
        let var = Variable::from_u32(i);
        builder.declare_var(var, types::I32);
        if (i as usize) < params.len() && i < param_count {
            builder.def_var(var, params[i as usize]);
        } else {
            let z = builder.ins().iconst(types::I32, 0);
            builder.def_var(var, z);
        }
    }
}

fn lower_blocks(
    jit_module: &mut JITModule,
    module: &MirModule,
    mir_fn: &MirFunction,
    builder: &mut FunctionBuilder<'_>,
    func_ids: &HashMap<String, FuncId>,
) -> SyxResult<()> {
    let mut clif_blocks: HashMap<u32, Block> = HashMap::new();
    for b in &mir_fn.blocks {
        clif_blocks.insert(b.id, builder.create_block());
    }
    let entry = clif_blocks[&mir_fn.entry];
    declare_locals(
        builder,
        entry,
        mir_fn.param_count,
        mir_fn.local_count.max(mir_fn.param_count),
    );

    let mut ordered: Vec<_> = mir_fn.blocks.iter().collect();
    ordered.sort_by_key(|b| b.id);

    for b in ordered {
        let block = clif_blocks[&b.id];
        if b.id != mir_fn.entry {
            builder.switch_to_block(block);
        }
        let mut stack: Vec<ClifValue> = Vec::new();
        let mut terminated = false;
        for insn in &b.instructions {
            if emit_insn(jit_module, module, builder, func_ids, &clif_blocks, &mut stack, insn)? {
                terminated = true;
                break;
            }
        }
        if !terminated {
            if let Some(&succ) = b.successors.first() {
                builder.ins().jump(clif_blocks[&succ], &[]);
            } else {
                let z = builder.ins().iconst(types::I32, 0);
                builder.ins().return_(&[z]);
            }
        }
    }

    for b in &mir_fn.blocks {
        builder.seal_block(clif_blocks[&b.id]);
    }
    Ok(())
}

fn lower_flat(
    jit_module: &mut JITModule,
    module: &MirModule,
    mir_fn: &MirFunction,
    builder: &mut FunctionBuilder<'_>,
    func_ids: &HashMap<String, FuncId>,
) -> SyxResult<()> {
    let entry = builder.create_block();
    declare_locals(
        builder,
        entry,
        mir_fn.param_count,
        mir_fn.local_count.max(mir_fn.param_count),
    );
    builder.seal_block(entry);

    let empty_blocks: HashMap<u32, Block> = HashMap::new();
    let mut stack: Vec<ClifValue> = Vec::new();
    let mut terminated = false;
    for insn in &mir_fn.instructions {
        if matches!(insn.op, MirOp::Jump(_) | MirOp::JumpIfFalse(_)) {
            return Err(SyxDiagnostic::error(
                "UnsupportedMirOp(Jump_flat_without_blocks)",
            ));
        }
        if emit_insn(
            jit_module,
            module,
            builder,
            func_ids,
            &empty_blocks,
            &mut stack,
            insn,
        )? {
            terminated = true;
            break;
        }
    }
    if !terminated {
        let v = stack
            .pop()
            .unwrap_or_else(|| builder.ins().iconst(types::I32, 0));
        builder.ins().return_(&[v]);
    }
    Ok(())
}

fn pop(builder: &mut FunctionBuilder<'_>, stack: &mut Vec<ClifValue>) -> ClifValue {
    stack
        .pop()
        .unwrap_or_else(|| builder.ins().iconst(types::I32, 0))
}

/// Emit one instruction. Returns true if the block was terminated.
fn emit_insn(
    jit_module: &mut JITModule,
    module: &MirModule,
    builder: &mut FunctionBuilder<'_>,
    func_ids: &HashMap<String, FuncId>,
    clif_blocks: &HashMap<u32, Block>,
    stack: &mut Vec<ClifValue>,
    insn: &MirInstr,
) -> SyxResult<bool> {
    match &insn.op {
        MirOp::ConstI32(v) => {
            stack.push(builder.ins().iconst(types::I32, i64::from(*v)));
        }
        MirOp::ConstNumeric { bits, .. } => {
            stack.push(builder.ins().iconst(types::I32, *bits as i32 as i64));
        }
        MirOp::ConstBool(v) => {
            stack.push(builder.ins().iconst(types::I32, if *v { 1 } else { 0 }));
        }
        MirOp::ConstNil => {
            stack.push(builder.ins().iconst(types::I32, 0));
        }
        MirOp::LoadLocal(i) => {
            stack.push(builder.use_var(Variable::from_u32(*i)));
        }
        MirOp::StoreLocal(i) => {
            let v = pop(builder, stack);
            builder.def_var(Variable::from_u32(*i), v);
        }
        MirOp::Add { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().iadd(a, b));
        }
        MirOp::Sub { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().isub(a, b));
        }
        MirOp::Mul { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().imul(a, b));
        }
        MirOp::Div { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().sdiv(a, b));
        }
        MirOp::Mod { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().srem(a, b));
        }
        MirOp::Neg { .. } => {
            let a = pop(builder, stack);
            stack.push(builder.ins().ineg(a));
        }
        MirOp::Not => {
            let a = pop(builder, stack);
            let z = builder.ins().iconst(types::I32, 0);
            let c = builder.ins().icmp(IntCC::Equal, a, z);
            stack.push(builder.ins().uextend(types::I32, c));
        }
        MirOp::Eq
        | MirOp::Ne
        | MirOp::Lt { .. }
        | MirOp::Le { .. }
        | MirOp::Gt { .. }
        | MirOp::Ge { .. } => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            let cc = match insn.op {
                MirOp::Eq => IntCC::Equal,
                MirOp::Ne => IntCC::NotEqual,
                MirOp::Lt { .. } => IntCC::SignedLessThan,
                MirOp::Le { .. } => IntCC::SignedLessThanOrEqual,
                MirOp::Gt { .. } => IntCC::SignedGreaterThan,
                MirOp::Ge { .. } => IntCC::SignedGreaterThanOrEqual,
                _ => unreachable!(),
            };
            let c = builder.ins().icmp(cc, a, b);
            stack.push(builder.ins().uextend(types::I32, c));
        }
        MirOp::And => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().band(a, b));
        }
        MirOp::Or => {
            let b = pop(builder, stack);
            let a = pop(builder, stack);
            stack.push(builder.ins().bor(a, b));
        }
        MirOp::Pop => {
            let _ = stack.pop();
        }
        MirOp::Dup => {
            if let Some(&v) = stack.last() {
                stack.push(v);
            }
        }
        MirOp::Jump(target) => {
            let dest = *clif_blocks.get(target).ok_or_else(|| {
                SyxDiagnostic::error(format!("JIT: unknown jump target {target}"))
            })?;
            builder.ins().jump(dest, &[]);
            return Ok(true);
        }
        MirOp::JumpIfFalse(target) => {
            let cond = pop(builder, stack);
            let false_dest = *clif_blocks.get(target).ok_or_else(|| {
                SyxDiagnostic::error(format!("JIT: unknown jump target {target}"))
            })?;
            let cont = builder.create_block();
            // cond != 0 → continue; cond == 0 → false_dest
            builder.ins().brif(cond, cont, &[], false_dest, &[]);
            builder.switch_to_block(cont);
            builder.seal_block(cont);
            return Ok(false);
        }
        MirOp::Call(name) => {
            let cfn = module
                .functions
                .iter()
                .find(|f| f.name == name.as_str())
                .ok_or_else(|| SyxDiagnostic::error(format!("JIT: unknown callee {name}")))?;
            let n = cfn.param_count as usize;
            let mut args = Vec::with_capacity(n);
            for _ in 0..n {
                args.push(pop(builder, stack));
            }
            args.reverse();
            let fid = *func_ids
                .get(name.as_str())
                .ok_or_else(|| SyxDiagnostic::error(format!("JIT: undeclared callee {name}")))?;
            let fref = jit_module.declare_func_in_func(fid, &mut builder.func);
            let call = builder.ins().call(fref, &args);
            let rets = builder.inst_results(call);
            let ret = rets
                .first()
                .copied()
                .unwrap_or_else(|| builder.ins().iconst(types::I32, 0));
            stack.push(ret);
        }
        MirOp::Return => {
            let v = pop(builder, stack);
            builder.ins().return_(&[v]);
            return Ok(true);
        }
        other => {
            return Err(SyxDiagnostic::error(
                unsupported_mir_op_reason(other)
                    .unwrap_or_else(|| format!("UnsupportedMirOp({other:?})")),
            ));
        }
    }
    Ok(false)
}

/// Call a finalized native entry with i32 args. Increments `jit_native_call_count`.
pub fn invoke_native(entry: &JitNativeEntry, args: &[i32]) -> SyxResult<i32> {
    if !entry.native || entry.code_ptr == 0 {
        return Err(SyxDiagnostic::error("JIT: entry not native"));
    }
    if args.len() != entry.param_count as usize {
        return Err(SyxDiagnostic::error(format!(
            "JIT: arity mismatch for {}: expected {}, got {}",
            entry.func_name,
            entry.param_count,
            args.len()
        )));
    }
    NATIVE_CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    let ptr = entry.code_ptr as *const u8;
    let result = unsafe {
        match entry.param_count {
            0 => {
                let f: unsafe extern "C" fn() -> i32 = std::mem::transmute(ptr);
                f()
            }
            1 => {
                let f: unsafe extern "C" fn(i32) -> i32 = std::mem::transmute(ptr);
                f(args[0])
            }
            2 => {
                let f: unsafe extern "C" fn(i32, i32) -> i32 = std::mem::transmute(ptr);
                f(args[0], args[1])
            }
            3 => {
                let f: unsafe extern "C" fn(i32, i32, i32) -> i32 = std::mem::transmute(ptr);
                f(args[0], args[1], args[2])
            }
            4 => {
                let f: unsafe extern "C" fn(i32, i32, i32, i32) -> i32 = std::mem::transmute(ptr);
                f(args[0], args[1], args[2], args[3])
            }
            n => {
                return Err(SyxDiagnostic::error(format!(
                    "JIT: unsupported arity {n} (max 4 in M1 W4)"
                )));
            }
        }
    };
    Ok(result)
}

/// Lookup cached entry by name (if still resident).
pub fn lookup_native_entry(func_name: &str) -> Option<JitNativeEntry> {
    let guard = jit_state_lock().lock().ok()?;
    guard.as_ref()?.entries.get(func_name).cloned()
}

/// Invalidate tier-2 cache (GC safepoint / endpoint change).
pub fn invalidate_jit_cache() {
    if let Ok(mut guard) = jit_state_lock().lock() {
        *guard = None;
    }
}

pub fn jit_cache_len() -> usize {
    jit_state_lock()
        .lock()
        .ok()
        .and_then(|g| g.as_ref().map(|s| s.entries.len()))
        .unwrap_or(0)
}

extern "C" fn syx_vm_deopt_trampoline() -> i32 {
    record_deopt_reason("DeoptTrampoline");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

    fn add_module() -> MirModule {
        MirModule {
            functions: vec![MirFunction {
                name: "add".into(),
                param_count: 2,
                local_count: 2,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        MirInstr {
                            dest: Some(0),
                            op: MirOp::LoadLocal(0),
                            span: None,
                        },
                        MirInstr {
                            dest: Some(1),
                            op: MirOp::LoadLocal(1),
                            span: None,
                        },
                        MirInstr {
                            dest: Some(0),
                            op: MirOp::Add {
                                kind: syx_stdlib::PLATFORM_INT.to_byte(),
                            },
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
        }
    }

    #[test]
    fn tier2_enabled_default_off() {
        std::env::remove_var("SYX_JIT_TIER2");
        assert!(!cranelift_jit_tier2_enabled());
    }

    #[test]
    fn tier2_opt_out_zero() {
        std::env::set_var("SYX_JIT_TIER2", "0");
        assert!(!cranelift_jit_tier2_enabled());
        std::env::remove_var("SYX_JIT_TIER2");
    }

    #[test]
    fn compile_and_invoke_add() {
        invalidate_jit_cache();
        let module = add_module();
        let entry = compile_hot_function(&module, "add").expect("jit compile");
        assert!(entry.native);
        assert_eq!(invoke_native(&entry, &[10, 5]).unwrap(), 15);
        assert!(jit_native_call_count() >= 1);
    }

    #[test]
    fn print_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::ConstI32(1),
                            span: None,
                        },
                        MirInstr {
                            dest: None,
                            op: MirOp::Print,
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
        let err = compile_hot_function(&module, "main").expect_err("print must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(Print)"),
            "{err}"
        );
    }

    #[test]
    fn callhost_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::CallHost(0),
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
        let err = compile_hot_function(&module, "main").expect_err("CallHost must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(CallHost)"),
            "{err}"
        );
    }

    #[test]
    fn conststr_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::ConstStr("hi".into()),
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
        let err = compile_hot_function(&module, "main").expect_err("ConstStr must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(ConstStr)"),
            "{err}"
        );
    }

    #[test]
    fn arraybuild_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::ArrayBuild(2),
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
        let err = compile_hot_function(&module, "main").expect_err("ArrayBuild must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(ArrayBuild"),
            "{err}"
        );
    }

    #[test]
    fn dictbuild_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::DictBuild(1),
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
        let err = compile_hot_function(&module, "main").expect_err("DictBuild must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(DictBuild"),
            "{err}"
        );
    }

    #[test]
    fn jump_flat_refuses_with_observable_reason() {
        invalidate_jit_cache();
        // Flat body (no blocks) with Jump — only reachable via hand-built MIR.
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(1),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(0),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let err = compile_hot_function(&module, "main").expect_err("Jump_flat must fail");
        assert!(
            err.to_string()
                .contains("UnsupportedMirOp(Jump_flat_without_blocks)"),
            "{err}"
        );
    }

    #[test]
    fn setbuild_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::SetBuild(2),
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
        let err = compile_hot_function(&module, "main").expect_err("SetBuild must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(SetBuild"),
            "{err}"
        );
    }

    #[test]
    fn loadglobal_refuses_with_observable_reason() {
        invalidate_jit_cache();
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
                            op: MirOp::LoadGlobal(0),
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
        let err = compile_hot_function(&module, "main").expect_err("LoadGlobal must fail");
        assert!(
            err.to_string().contains("UnsupportedMirOp(LoadGlobal"),
            "{err}"
        );
    }
}
