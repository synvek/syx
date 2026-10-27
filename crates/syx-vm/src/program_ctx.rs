use std::cell::RefCell;
use std::sync::Arc;

use syx_codegen::CompiledProgram;

use crate::VmRuntime;

thread_local! {
    static PROGRAM_CTX: RefCell<Option<ProgramCtx>> = const { RefCell::new(None) };
    static ACTIVE_VM: RefCell<Option<*mut crate::VmInstance>> = const { RefCell::new(None) };
}

/// Native/MIR closure executor registered by `syx-rt` when bytecode `ProgramCtx` is absent.
type MirClosureExecutor = fn(u32, &[crate::Value], &[crate::Value]) -> crate::Value;

thread_local! {
    static MIR_CLOSURE_EXECUTOR: RefCell<Option<MirClosureExecutor>> = const { RefCell::new(None) };
    static MIR_CLOSURE_PREFERRED: RefCell<bool> = const { RefCell::new(false) };
}

#[derive(Clone)]
pub struct ProgramCtx {
    pub program: Arc<CompiledProgram>,
    pub runtime: Option<VmRuntime>,
}

pub fn set_program_ctx(ctx: ProgramCtx) {
    PROGRAM_CTX.with(|c| *c.borrow_mut() = Some(ctx));
}

pub fn clear_program_ctx() {
    PROGRAM_CTX.with(|c| *c.borrow_mut() = None);
}

/// Phase 121: active interpreter VM for host-initiated user fn calls (service ctor).
pub fn set_active_vm(vm: &mut crate::VmInstance) {
    ACTIVE_VM.with(|c| *c.borrow_mut() = Some(vm as *mut crate::VmInstance));
}

pub fn clear_active_vm() {
    ACTIVE_VM.with(|c| *c.borrow_mut() = None);
}

pub fn with_active_vm<R>(f: impl FnOnce(&mut crate::VmInstance) -> R) -> Option<R> {
    // Copy the raw pointer out before calling `f` so nested host → user-fn
    // invocations (e.g. Di.withRequest → handler → resolve → ctor) share the
    // same VM (globals / stack) without re-borrowing ACTIVE_VM's RefCell.
    let ptr = ACTIVE_VM.with(|c| *c.borrow());
    ptr.map(|ptr| unsafe { f(&mut *ptr) })
}

/// Phase 88: register MIR-native closure runner (HOF parity on native strict).
pub fn set_mir_closure_executor(exec: MirClosureExecutor) {
    MIR_CLOSURE_EXECUTOR.with(|c| *c.borrow_mut() = Some(exec));
}

/// Prefer MIR closure executor over bytecode VM (MIR/WASM driver paths).
pub fn set_mir_closure_preferred(preferred: bool) {
    MIR_CLOSURE_PREFERRED.with(|c| *c.borrow_mut() = preferred);
}

pub fn clear_mir_closure_executor() {
    MIR_CLOSURE_EXECUTOR.with(|c| *c.borrow_mut() = None);
    MIR_CLOSURE_PREFERRED.with(|c| *c.borrow_mut() = false);
}

pub fn with_program_ctx<R>(f: impl FnOnce(&ProgramCtx) -> R) -> Option<R> {
    PROGRAM_CTX.with(|c| c.borrow().as_ref().map(f))
}

/// Run a bytecode closure when a program context is active (HOF host parity).
#[cfg(feature = "interpreter")]
pub fn execute_closure(
    fn_index: u32,
    captures: &[crate::Value],
    args: &[crate::Value],
) -> crate::Value {
    use crate::{host_fn_for, WasmVM};
    use syx_stdlib::HostFnId;

    if MIR_CLOSURE_PREFERRED.with(|c| *c.borrow()) {
        return MIR_CLOSURE_EXECUTOR.with(|c| {
            c.borrow()
                .map(|exec| exec(fn_index, captures, args))
                .unwrap_or(crate::Value::Nil)
        });
    }

    if let Some(ctx) = with_program_ctx(|ctx| ctx.clone()) {
        if let Some(func) = ctx.program.functions.get(fn_index as usize) {
            let mut vm = WasmVM::default();
            vm.runtime = ctx.runtime.clone();
            for id in HostFnId::all() {
                if let Some(f) = host_fn_for(*id) {
                    vm.host_fns.insert(id.name().into(), f);
                }
            }
            let v = vm
                .run_closure_with_captures(func, &ctx.program, captures, args)
                .unwrap_or(crate::Value::Nil);
            if !matches!(v, crate::Value::Nil) {
                return v;
            }
        }
    }
    MIR_CLOSURE_EXECUTOR.with(|c| {
        c.borrow()
            .map(|exec| exec(fn_index, captures, args))
            .unwrap_or(crate::Value::Nil)
    })
}
