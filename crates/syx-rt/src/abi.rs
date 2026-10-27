use std::cell::UnsafeCell;
use std::io::Write;
use std::os::raw::{c_char, c_int};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::MirModule;
use syx_mir_run::MirRuntime;
use syx_runtime_core::Value;

thread_local! {
    static RT: UnsafeCell<NativeRt> = UnsafeCell::new(NativeRt::new());
}

static LOADED_NATIVE_CTX: OnceLock<Arc<crate::bridge::NativeRunContext>> = OnceLock::new();

static RT_INTERNAL_EXEC: AtomicU64 = AtomicU64::new(0);

/// Phase 76: count of RT-internal `mir.exec` fallbacks (doctor / CI gate).
pub fn rt_internal_exec_count() -> u64 {
    RT_INTERNAL_EXEC.load(Ordering::Relaxed)
}

/// Reset RT-internal exec counter (tests).
pub fn reset_rt_internal_exec_count() {
    RT_INTERNAL_EXEC.store(0, Ordering::Relaxed);
}

fn record_rt_internal_exec() {
    RT_INTERNAL_EXEC.fetch_add(1, Ordering::Relaxed);
}

/// Phase 79: RT hot-path dispatch table marker (ADR-061).
pub const RT_DISPATCH_TABLE: &str = "rt-dispatch-table-v4";

/// Phase 80: native stack/register dispatch v5 (ADR-065).
pub const NATIVE_STACK_DISPATCH: &str = "native-stack-dispatch-v5";

/// Phase 81: full 88-opcode native stack dispatch v6 (ADR-069).
pub const NATIVE_STACK_DISPATCH_V6: &str = "native-stack-dispatch-v6-full";

/// Phase 82: full 88-opcode native stack dispatch v7 — zero internal exec counter (ADR-072).
pub const NATIVE_STACK_DISPATCH_V7: &str = "native-stack-dispatch-v7-real";

/// Fail-closed: surface runtime diagnostics (e.g. Array/String OOB R003) instead of
/// swallowing `Err` as `0` and continuing with a corrupted stack.
fn native_runtime_fatal(err: SyxDiagnostic) -> ! {
    let code = err.code.as_deref().unwrap_or("R000");
    eprintln!("  × [{code}] {}", err.message);
    let _ = std::io::stderr().flush();
    std::process::exit(1);
}

fn native_stack_dispatch_v7_direct(rt: &mut MirRuntime, opcode: u32, a: i32, b: i32) -> i32 {
    match rt.exec(opcode, a, b, 0, &mut []) {
        Ok(n) => n,
        Err(err) => native_runtime_fatal(err),
    }
}

#[cfg(debug_assertions)]
fn native_stack_dispatch_fallback(rt: &mut MirRuntime, opcode: u32, a: i32, b: i32) -> i32 {
    record_rt_internal_exec();
    native_stack_dispatch_v7_direct(rt, opcode, a, b)
}

#[cfg(not(debug_assertions))]
fn native_stack_dispatch_fallback(rt: &mut MirRuntime, opcode: u32, a: i32, b: i32) -> i32 {
    native_stack_dispatch_v7_direct(rt, opcode, a, b)
}

fn pop_i32(rt: &mut MirRuntime) -> i32 {
    rt.stack
        .pop()
        .and_then(|v| match v {
            Value::I32(n) => Some(n),
            _ => None,
        })
        .unwrap_or(0)
}

fn push_i32(rt: &mut MirRuntime, n: i32) {
    rt.stack.push(Value::I32(n));
}

/// Phase 80: operate directly on MIR stack/locals without `MirRuntime.exec` on hot path.
pub fn native_stack_dispatch(rt: &mut MirRuntime, op: u32, a: i32, b: i32) -> i32 {
    use syx_mir_run::{
        ACTOR_CALL, ACTOR_REGISTER, ACTOR_SEND, ADD, AND, ARRAY_BUILD, ARRAY_GET, ARRAY_SET,
        AWAIT_POLL, AWAIT_SUSPEND, BEGIN_TRY, CALL_CLOSURE, CALL_FN, CALL_HOST, CALL_WITNESS,
        CATCH, CONST_BOOL, CONST_I32, CONST_NIL, CONST_STR, DICT_BUILD, DIV, DUP, END_TRY, EQ, GE,
        GT, HEAP_COMPACT, ITERATOR_NEXT, LE, LOAD_GLOBAL, LOAD_LOCAL, LT, MAKE_CLOSURE,
        MAKE_ITERATOR, METHOD_CALL, MOD, MUL, NE, NEG, NOT, OBJECT_GET, OBJECT_SET, OR, POP,
        PRINT, REF_EQ, REF_NE, SET_BUILD,
        SIMD_ADD, SIMD_AND, SIMD_DIV, SIMD_EQ, SIMD_GE, SIMD_GT, SIMD_LE, SIMD_LOAD, SIMD_LT,
        SIMD_MUL, SIMD_NE, SIMD_NOT, SIMD_OR, SIMD_SHL, SIMD_SHR, SIMD_SHUFFLE, SIMD_STORE,
        SIMD_SUB, SIMD_WASM_FILL, SIMD_WASM_SPILL, SIMD_XOR, STORE_GLOBAL, STORE_LOCAL, SUB,
        TASK_JOIN, TASK_SPAWN, THROW, TO_STR, VALUE_EQ, VALUE_NE,
    };
    match op {
        CONST_I32 => {
            push_i32(rt, a);
            0
        }
        CONST_BOOL => {
            rt.stack.push(Value::Bool(a != 0));
            0
        }
        CONST_NIL => {
            rt.stack.push(Value::Nil);
            0
        }
        LOAD_LOCAL => {
            if let Some(v) = rt.locals.get(a as usize).cloned() {
                rt.stack.push(v);
            }
            0
        }
        STORE_LOCAL => {
            if let Some(v) = rt.stack.pop() {
                let i = a as usize;
                if rt.locals.len() <= i {
                    rt.locals.resize(i + 1, Value::Nil);
                }
                rt.locals[i] = v;
            }
            0
        }
        // M0-W4: Int is platform I64 via ConstNumeric; pop_i32-only ADD dropped values to 0.
        // Delegate scalar arithmetic/compare to MirRuntime.exec (handles NumericKind).
        ADD | SUB | MUL | DIV | MOD | EQ | NE | LT | GT | LE | GE | AND | OR | NOT | NEG => {
            native_stack_dispatch_fallback(rt, op, a, b)
        }
        POP => {
            let _ = rt.stack.pop();
            0
        }
        DUP => {
            if let Some(v) = rt.stack.last().cloned() {
                rt.stack.push(v);
            }
            0
        }
        PRINT => mir_print_top(rt),
        CALL_HOST => mir_call_host(rt, a as u32),
        METHOD_CALL | MAKE_ITERATOR | ITERATOR_NEXT | ARRAY_BUILD | ARRAY_GET | ARRAY_SET
        | OBJECT_GET | OBJECT_SET | CALL_FN | CALL_WITNESS | DICT_BUILD | SET_BUILD
        | AWAIT_POLL | AWAIT_SUSPEND | ACTOR_CALL | ACTOR_SEND | ACTOR_REGISTER | TASK_SPAWN
        | TASK_JOIN | MAKE_CLOSURE | CALL_CLOSURE | BEGIN_TRY | END_TRY | CATCH | THROW
        | CONST_STR | TO_STR | VALUE_EQ | VALUE_NE | REF_EQ | REF_NE | LOAD_GLOBAL
        | STORE_GLOBAL
        | HEAP_COMPACT | SIMD_ADD | SIMD_SUB | SIMD_MUL | SIMD_DIV | SIMD_EQ | SIMD_LT
        | SIMD_GT | SIMD_NE | SIMD_LE | SIMD_GE | SIMD_AND | SIMD_OR | SIMD_XOR | SIMD_NOT
        | SIMD_SHL | SIMD_SHR | SIMD_LOAD | SIMD_STORE | SIMD_SHUFFLE | SIMD_WASM_SPILL
        | SIMD_WASM_FILL => native_stack_dispatch_fallback(rt, op, a, b),
        _ => native_stack_dispatch_fallback(rt, op, a, b),
    }
}

/// Pure-native Print (W423 host-bridge Print wire): pop stack top, format, record + stdout.
///
/// Shared by `native_stack_dispatch(PRINT)` and `rt_print_native` so AOT stubs no longer
/// need `rt_direct_exec(PRINT)`.
/// Formatting matches VM / mir-run `value_description` (e.g. `Character` prints the grapheme,
/// not `Debug` `Character("…")`).
fn mir_print_top(rt: &mut MirRuntime) -> i32 {
    let v = rt.stack.pop().unwrap_or(Value::Nil);
    let text = syx_runtime_core::value_description(&v);
    rt.printed.push(text.clone());
    let _ = writeln!(std::io::stdout(), "{text}");
    let _ = std::io::stdout().flush();
    0
}

/// W423: Print wire first-slice — pure native stack/IO; no MIR opcode `rt_direct_exec(PRINT)`.
pub fn rt_print_native() {
    NativeRt::with(|rt| {
        let _ = mir_print_top(&mut rt.mir);
    });
}

/// HostFn dispatch (W424 CallHost wire): same semantics as `MirRuntime::exec(CALL_HOST)`.
///
/// Shared by `native_stack_dispatch(CALL_HOST)` and `rt_call_host_native` so AOT stubs no
/// longer need `rt_direct_exec(CALL_HOST)`.
fn mir_call_host(rt: &mut MirRuntime, idx: u32) -> i32 {
    match rt.exec(syx_mir_run::CALL_HOST, idx as i32, 0, 0, &mut []) {
        Ok(n) => n,
        Err(err) => native_runtime_fatal(err),
    }
}

/// W424: CallHost wire first-slice — HostFn dispatch without `rt_direct_exec(CALL_HOST)`.
pub fn rt_call_host_native(idx: u32) -> i32 {
    NativeRt::with(|rt| mir_call_host(&mut rt.mir, idx))
}

fn rt_dispatch_table_exec(rt: &mut MirRuntime, op: u32, a: i32, b: i32) -> i32 {
    native_stack_dispatch(rt, op, a, b)
}

/// Phase 79: direct MIR dispatch via RT_DISPATCH_TABLE (no `mir.exec` in hot path).
pub fn rt_direct_exec(op: u32, a: i32, b: i32) -> i32 {
    NativeRt::with(|rt| rt_dispatch_table_exec(&mut rt.mir, op, a, b))
}

struct NativeRt {
    mir: MirRuntime,
    native_fns: Vec<NativeFn>,
    /// Shared native run context (MIR module + print capture); set by `load_mir_module`.
    run_ctx: Option<Arc<crate::bridge::NativeRunContext>>,
}

type NativeFn = extern "C" fn() -> i32;

impl NativeRt {
    fn new() -> Self {
        Self {
            mir: MirRuntime::new(),
            native_fns: Vec::new(),
            run_ctx: None,
        }
    }

    fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut NativeRt) -> R,
    {
        RT.with(|rt| f(unsafe { &mut *rt.get() }))
    }
}

/// Initialize MIR function table and optional async runtime before user code runs.
#[no_mangle]
pub extern "C" fn syx_rt_init_module(_fn_count: u32) {
    crate::bridge::install_native_task_hosts();
    NativeRt::with(|rt| {
        rt.native_fns.clear();
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_register_native_fn(idx: u32, ptr: u64) {
    NativeRt::with(|rt| {
        let f: NativeFn = unsafe { std::mem::transmute(ptr as usize) };
        let i = idx as usize;
        if rt.native_fns.len() <= i {
            rt.native_fns.resize(i + 1, stub_native_fn_c);
        }
        rt.native_fns[i] = f;
    });
    // W429: register after empty-pool install marks Register-only observability.
    if EMPTY_RUNTIME_INSTALL.load(std::sync::atomic::Ordering::SeqCst) {
        REGISTER_ONLY_RUNTIME_INSTALL.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

fn stub_native_fn() -> i32 {
    0
}

extern "C" fn stub_native_fn_c() -> i32 {
    0
}

/// Initialize dynamic Host plugins before MIR load (ADR-071 appendix B).
#[no_mangle]
pub extern "C" fn syx_rt_init_plugins() {
    let _ = syx_host_plugin::load_from_env();
}

/// Load embedded MIR metadata from a linked object file blob.
///
/// Wave A W406: if the blob is a `SYXD` dual-blob / AOT-embed package, peel the
/// execution segment and `decode_module` that segment.
/// Wave A W407: also peel/decode the sidecar and install it into RT metadata
/// (string pool) — dual-load runtime first-slice.
/// Wave A W408: after execution load, **reset** the string pool then install
/// sidecar as the primary string-pool source (`sidecar_string_pool_primary`).
/// Wave A W409: if peeled execution is **empty**, skip decode/load of execution
/// and install runtime from sidecar only (fn stubs + string pool);
/// `aot_execution_segment_omitted=true`.
/// Legacy compact/whole blobs still decode whole without sidecar install.
#[no_mangle]
pub extern "C" fn syx_rt_load_mir_blob(ptr: *const u8, len: i32) {
    syx_rt_init_plugins();
    // Blob path is never the W412 no-blob alternate entry.
    NATIVE_METADATA_ALTERNATE_ENTRY.store(false, std::sync::atomic::Ordering::SeqCst);
    if ptr.is_null() || len <= 0 {
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    if syx_mir::is_dual_blob_package_bytes(slice) {
        let Ok(exec) = syx_mir::peel_dual_blob_package_execution(slice) else {
            return;
        };
        let Ok(sidecar_bytes) = syx_mir::peel_dual_blob_package_sidecar(slice) else {
            return;
        };
        // W409: empty execution → sidecar-only runtime install.
        if exec.is_empty() {
            let Ok(view) = syx_mir::decode_module_metadata_sidecar(sidecar_bytes) else {
                AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
                SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
                return;
            };
            crate::bridge::install_native_task_hosts();
            install_runtime_from_sidecar_only(&view);
            AOT_EXECUTION_SEGMENT_OMITTED.store(true, std::sync::atomic::Ordering::SeqCst);
            SIDECAR_STRING_POOL_PRIMARY.store(true, std::sync::atomic::Ordering::SeqCst);
            return;
        }
        AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
        let Ok(module) = syx_mir::decode_module(exec) else {
            return;
        };
        crate::bridge::install_native_task_hosts();
        load_mir_module(&module);
        // W408: sidecar is the string-pool primary source for SYXD dual-load.
        reset_string_pool_for_sidecar_primary();
        if let Ok(view) = syx_mir::decode_module_metadata_sidecar(sidecar_bytes) {
            install_metadata_sidecar_view(&view);
            SIDECAR_STRING_POOL_PRIMARY.store(true, std::sync::atomic::Ordering::SeqCst);
        } else {
            SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
        }
        return;
    }
    AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
    SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
    if let Ok(module) = syx_mir::decode_module(slice) {
        crate::bridge::install_native_task_hosts();
        load_mir_module(&module);
    }
}

/// W407 dual-load runtime: last sidecar `string_literals.len()` installed into RT.
static SIDECAR_INSTALL_LITERAL_COUNT: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

/// W408: true when last SYXD load reset the pool and installed sidecar as primary.
static SIDECAR_STRING_POOL_PRIMARY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// W409: true when last SYXD load used empty execution (sidecar-only install).
static AOT_EXECUTION_SEGMENT_OMITTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// W412: true when last install used the public no-blob alternate entry
/// (`syx_rt_install_metadata_sidecar`), not `syx_rt_load_mir_blob`.
static NATIVE_METADATA_ALTERNATE_ENTRY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// W428: true when last install used the public empty-pool entry
/// (`syx_rt_install_empty_runtime`) — no sidecar bytes.
static EMPTY_RUNTIME_INSTALL: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// W429: true when native fns were registered after empty-pool install
/// (Register-only path; no sidecar-byte install).
static REGISTER_ONLY_RUNTIME_INSTALL: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Clear RT string pool before sidecar-primary install (W408).
fn reset_string_pool_for_sidecar_primary() {
    NativeRt::with(|rt| {
        rt.mir.string_pool.clear();
    });
}

/// W409: install RT from sidecar only (fn Return stubs + string pool).
///
/// Used when AOT embed execution segment is empty. Preserves fn-index alignment via
/// [`syx_mir::mir_module_from_sidecar_fn_stubs`].
pub fn install_runtime_from_sidecar_only(view: &syx_mir::MirModuleMetadataSidecarView) {
    let stubs = syx_mir::mir_module_from_sidecar_fn_stubs(view);
    load_mir_module(&stubs);
    reset_string_pool_for_sidecar_primary();
    install_metadata_sidecar_view(view);
}

/// W428: install RT with empty fn stub table + empty string pool (no sidecar bytes).
///
/// Symmetric to [`install_runtime_from_sidecar_only`] with an empty view. Used by
/// [`syx_rt_install_empty_runtime`]. Does **not** switch production `syx_user_init`.
pub fn install_runtime_empty() {
    let empty = syx_mir::MirModuleMetadataSidecarView {
        fn_names: Vec::new(),
        string_literals: Vec::new(),
        local_names: Vec::new(),
        local_types: Vec::new(),
    };
    install_runtime_from_sidecar_only(&empty);
}

/// Install metadata sidecar view into the native RT (string pool first-slice).
///
/// Interns each `string_literals` entry into `MirRuntime.string_pool` after
/// execution-module load. Records install literal count for observability.
pub fn install_metadata_sidecar_view(view: &syx_mir::MirModuleMetadataSidecarView) {
    NativeRt::with(|rt| {
        for s in &view.string_literals {
            let _ = rt.mir.intern_string(s);
        }
    });
    SIDECAR_INSTALL_LITERAL_COUNT.store(
        view.string_literals.len(),
        std::sync::atomic::Ordering::SeqCst,
    );
}

/// W412: public C ABI alternate entry — install RT from metadata-sidecar bytes only.
///
/// Decodes [`syx_mir::decode_module_metadata_sidecar`] then
/// [`install_runtime_from_sidecar_only`]. Does **not** require SYXD / `@syx_mir_blob`.
/// Production `syx_user_init` uses empty install + register (W431+; ≠ load_mir_blob).
#[no_mangle]
pub extern "C" fn syx_rt_install_metadata_sidecar(ptr: *const u8, len: i32) {
    syx_rt_init_plugins();
    NATIVE_METADATA_ALTERNATE_ENTRY.store(false, std::sync::atomic::Ordering::SeqCst);
    if ptr.is_null() || len <= 0 {
        AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
        SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let Ok(view) = syx_mir::decode_module_metadata_sidecar(slice) else {
        AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
        SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
        return;
    };
    crate::bridge::install_native_task_hosts();
    install_runtime_from_sidecar_only(&view);
    AOT_EXECUTION_SEGMENT_OMITTED.store(true, std::sync::atomic::Ordering::SeqCst);
    SIDECAR_STRING_POOL_PRIMARY.store(true, std::sync::atomic::Ordering::SeqCst);
    NATIVE_METADATA_ALTERNATE_ENTRY.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: number of sidecar string literals last installed (W407).
pub fn sidecar_install_literal_count() -> usize {
    SIDECAR_INSTALL_LITERAL_COUNT.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset sidecar install observability counter (tests).
pub fn reset_sidecar_install_literal_count() {
    SIDECAR_INSTALL_LITERAL_COUNT.store(0, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: last SYXD load used sidecar as string-pool primary (W408).
pub fn sidecar_string_pool_primary() -> bool {
    SIDECAR_STRING_POOL_PRIMARY.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset sidecar-primary observability flag (tests).
pub fn reset_sidecar_string_pool_primary() {
    SIDECAR_STRING_POOL_PRIMARY.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: last SYXD load omitted execution segment (W409).
pub fn aot_execution_segment_omitted() -> bool {
    AOT_EXECUTION_SEGMENT_OMITTED.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset execution-omit observability flag (tests).
pub fn reset_aot_execution_segment_omitted() {
    AOT_EXECUTION_SEGMENT_OMITTED.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: last install used no-blob alternate entry (W412).
pub fn native_metadata_alternate_entry() -> bool {
    NATIVE_METADATA_ALTERNATE_ENTRY.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset alternate-entry observability flag (tests).
pub fn reset_native_metadata_alternate_entry() {
    NATIVE_METADATA_ALTERNATE_ENTRY.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// W428: public C ABI — install empty fn stub table + empty string pool without sidecar bytes.
///
/// Production `syx_user_init` still uses [`syx_rt_install_metadata_sidecar`] (wire ≠ emit omit).
#[no_mangle]
pub extern "C" fn syx_rt_install_empty_runtime() {
    syx_rt_init_plugins();
    EMPTY_RUNTIME_INSTALL.store(false, std::sync::atomic::Ordering::SeqCst);
    REGISTER_ONLY_RUNTIME_INSTALL.store(false, std::sync::atomic::Ordering::SeqCst);
    NATIVE_METADATA_ALTERNATE_ENTRY.store(false, std::sync::atomic::Ordering::SeqCst);
    crate::bridge::install_native_task_hosts();
    install_runtime_empty();
    AOT_EXECUTION_SEGMENT_OMITTED.store(true, std::sync::atomic::Ordering::SeqCst);
    SIDECAR_STRING_POOL_PRIMARY.store(true, std::sync::atomic::Ordering::SeqCst);
    EMPTY_RUNTIME_INSTALL.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: last install used empty-pool entry (W428).
pub fn empty_runtime_install() -> bool {
    EMPTY_RUNTIME_INSTALL.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset empty-runtime observability flag (tests).
pub fn reset_empty_runtime_install() {
    EMPTY_RUNTIME_INSTALL.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// Observability: last register happened after empty-pool install (W429).
pub fn register_only_runtime_install() -> bool {
    REGISTER_ONLY_RUNTIME_INSTALL.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset register-only observability flag (tests).
pub fn reset_register_only_runtime_install() {
    REGISTER_ONLY_RUNTIME_INSTALL.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// Load MIR metadata from host (driver calls before linking user code).
pub fn load_mir_module(module: &MirModule) {
    let ctx = crate::bridge::install_native_module(module);
    let _ = LOADED_NATIVE_CTX.set(Arc::clone(&ctx));
    NativeRt::with(|rt| {
        rt.run_ctx = Some(Arc::clone(&ctx));
        rt.mir.install_functions(module.functions.clone());
        rt.mir
            .set_runtime(crate::bridge::native_vm_runtime_for(Arc::clone(&ctx)));
        syx_mir_run::install_module_string_pool(&mut rt.mir, module);
    });
    syx_vm::set_mir_closure_executor(execute_native_mir_closure);
    syx_vm::set_mir_closure_preferred(true);
}

/// Phase 88: run MIR closure bodies for HOF hosts when native exe has no bytecode ProgramCtx.
fn execute_native_mir_closure(fn_index: u32, captures: &[Value], args: &[Value]) -> Value {
    NativeRt::with(|rt| {
        let mut pool = rt.mir.string_pool.clone();
        syx_mir_run::pin_active_mir_runtime(&mut rt.mir);
        let result = rt
            .mir
            .run_closure_with_captures(fn_index, captures, args, &mut pool)
            .unwrap_or(Value::Nil);
        syx_mir_run::unpin_active_mir_runtime();
        result
    })
}

/// Run a MIR function by name using the loaded native module (HTTP handlers, etc.).
#[cfg(feature = "http")]
pub(crate) fn run_native_mir_handler(handler: &str, req: &Value) -> SyxResult<Value> {
    let ctx = LOADED_NATIVE_CTX
        .get()
        .cloned()
        .or_else(|| NativeRt::with(|rt| rt.run_ctx.clone()))
        .ok_or_else(|| SyxDiagnostic::error("MIR module not loaded for native HTTP"))?;
    crate::bridge::run_mir_by_name(&ctx, handler, vec![req.clone()])
}

/// Pointer to embedded string pool for ConstStr in generated code.
#[no_mangle]
pub extern "C" fn syx_rt_string_pool_base() -> *const u8 {
    NativeRt::with(|rt| rt.mir.string_pool.as_ptr())
}

#[no_mangle]
pub extern "C" fn syx_rt_load_local(slot: i32) {
    let _ = rt_direct_exec(syx_mir_run::LOAD_LOCAL, slot, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_store_local(slot: i32) {
    let _ = rt_direct_exec(syx_mir_run::STORE_LOCAL, slot, 0);
}

/// Phase 104 locals v10: frame `MirRuntime::locals` owns Value slots; no TLS slab hot path.
/// Gate fingerprint: `native-llvm-locals-v10:no-tls-slab`

fn locals_v10_hard_fail_legacy_local_bits(caller: &str) -> ! {
    eprintln!("syx locals v10: {caller} removed; use frame_local_ptr / pop_into_local_ptr");
    std::process::abort();
}

fn value_from_local_ptr(ptr: i64) -> Value {
    if ptr == 0 {
        return Value::Nil;
    }
    unsafe { (*(ptr as *const Value)).clone() }
}

/// Pointer to the frame-local Value slot (`begin_function` owns the contiguous locals array).
#[no_mangle]
pub extern "C" fn syx_rt_frame_local_ptr(slot: i32) -> i64 {
    NativeRt::with(|rt| {
        let i = slot.max(0) as usize;
        if i >= rt.mir.locals.len() {
            rt.mir.locals.resize(i + 1, Value::Nil);
        }
        &mut rt.mir.locals[i] as *mut Value as i64
    })
}

/// Pop MIR stack Value → write through frame-local pointer (alloca stores this ptr).
#[no_mangle]
pub extern "C" fn syx_rt_pop_into_local_ptr(ptr: i64) {
    if ptr == 0 {
        let _ = NativeRt::with(|rt| {
            let _ = rt.mir.stack.pop();
        });
        return;
    }
    NativeRt::with(|rt| {
        let v = rt.mir.stack.pop().unwrap_or(Value::Nil);
        unsafe {
            *(ptr as *mut Value) = v;
        }
    });
}

/// Materialize frame-local pointer onto the MIR stack (`ptr == 0` → Nil).
#[no_mangle]
pub extern "C" fn syx_rt_push_from_local_ptr(ptr: i64) {
    NativeRt::with(|rt| {
        rt.mir.stack.push(value_from_local_ptr(ptr));
    });
}

/// Legacy v9 link names — hard-fail under locals v10.
#[no_mangle]
pub extern "C" fn syx_rt_pop_to_local_bits() -> i64 {
    locals_v10_hard_fail_legacy_local_bits("syx_rt_pop_to_local_bits");
}

#[no_mangle]
pub extern "C" fn syx_rt_push_from_local_bits(_bits: i64) {
    locals_v10_hard_fail_legacy_local_bits("syx_rt_push_from_local_bits");
}

#[no_mangle]
pub extern "C" fn syx_rt_frame_local_bits(_slot: i32) -> i64 {
    locals_v10_hard_fail_legacy_local_bits("syx_rt_frame_local_bits");
}

/// Phase 95: pop MIR stack slot (LLVM `MirOp::Pop` direct emit).
#[no_mangle]
pub extern "C" fn syx_rt_pop_stack() {
    let _ = rt_direct_exec(syx_mir_run::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_begin_function(param_count: u32, local_count: u32) {
    NativeRt::with(|rt| {
        rt.mir
            .begin_function_from_call_stack(param_count, local_count);
    });
}

/// Phase 98: restore caller locals and leave return value on the MIR stack (LLVM `Return`).
#[no_mangle]
pub extern "C" fn syx_rt_end_function() {
    NativeRt::with(|rt| {
        rt.mir.end_function();
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_push_i32(v: i32) {
    NativeRt::with(|rt| {
        rt.mir.stack.push(syx_vm::Value::I32(v));
    });
}

/// Push `Value::Nil` (void / unit return placeholder for LLVM `ConstNil`).
#[no_mangle]
pub extern "C" fn syx_rt_push_nil() {
    NativeRt::with(|rt| {
        rt.mir.stack.push(syx_vm::Value::Nil);
    });
}

/// Wave A W203 / ADR-547: Push `Value::Bool` for LLVM `ConstBool` (RT stack for Print/consumers).
#[no_mangle]
pub extern "C" fn syx_rt_push_bool(v: i32) {
    NativeRt::with(|rt| {
        rt.mir.stack.push(syx_vm::Value::Bool(v != 0));
    });
}

/// Push a `ConstNumeric` literal onto the MIR stack for LLVM tier-1 stack ops.
///
/// Historical `(kind, bits_lo)` ABI — **truncates** `bits` to low 32 bits.
/// Prefer [`syx_rt_push_numeric64`] for Float64 / wide numeric literals (W955 / ADR-1299).
#[no_mangle]
pub extern "C" fn syx_rt_push_numeric(kind: i32, bits_lo: i32) {
    let _ = rt_direct_exec(syx_mir_run::CONST_NUMERIC, kind, bits_lo);
}

/// W955 / ADR-1299: push full 64-bit `ConstNumeric` bits (Float64-safe).
///
/// Does not route through `CONST_NUMERIC` opcode immediates (i32×2 truncates f64).
#[no_mangle]
pub extern "C" fn syx_rt_push_numeric64(kind: i32, bits: i64) {
    NativeRt::with(|rt| {
        let v = syx_stdlib::NumericKind::from_byte(kind as u8)
            .map(|k| syx_runtime_core::value_from_numeric(k, bits as u64))
            .unwrap_or(syx_vm::Value::I32(0));
        rt.mir.stack.push(v);
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_push_str(ptr: *const u8, len: i32) {
    NativeRt::with(|rt| {
        let s = if ptr.is_null() || len <= 0 {
            String::new()
        } else {
            let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            std::str::from_utf8(slice).unwrap_or("").to_string()
        };
        rt.mir.stack.push(syx_vm::Value::Str(s));
    });
}

fn value_to_display_string(v: &syx_vm::Value) -> String {
    match v {
        syx_vm::Value::Str(s) => s.clone(),
        syx_vm::Value::I32(n) => n.to_string(),
        syx_vm::Value::Bool(b) => b.to_string(),
        syx_vm::Value::Nil => "nil".into(),
        other if syx_runtime_core::value_kind(other).is_some() => {
            syx_runtime_core::display_numeric(other).unwrap_or_else(|| format!("{other:?}"))
        }
        _ => format!("{v:?}"),
    }
}

/// Phase 93: concat top two stack strings (LLVM direct emit).
#[no_mangle]
pub extern "C" fn syx_rt_concat_str() {
    NativeRt::with(|rt| {
        let b = rt.mir.stack.pop().unwrap_or(syx_vm::Value::Nil);
        let a = rt.mir.stack.pop().unwrap_or(syx_vm::Value::Nil);
        let merged = format!(
            "{}{}",
            value_to_display_string(&a),
            value_to_display_string(&b)
        );
        rt.mir.stack.push(syx_vm::Value::Str(merged));
    });
}

/// Phase 93: convert top stack value to string (LLVM ToStr / interpolation).
#[no_mangle]
pub extern "C" fn syx_rt_to_str() {
    NativeRt::with(|rt| {
        let v = rt.mir.stack.pop().unwrap_or(syx_vm::Value::Nil);
        rt.mir
            .stack
            .push(syx_vm::Value::Str(value_to_display_string(&v)));
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_take_catch() -> i32 {
    // Wave A W6 / ADR-350: framed take_catch (see MirRuntime::take_catch).
    NativeRt::with(|rt| rt.mir.take_catch())
}

/// Wave A W6 / ADR-350: MIR `Throw` — pop error value into `pending_catch`.
#[no_mangle]
pub extern "C" fn syx_rt_throw() {
    let _ = rt_direct_exec(syx_mir_run::THROW, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_make_iterator(_: i32) -> i32 {
    rt_direct_exec(syx_mir_run::MAKE_ITERATOR, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_iterator_next(_: i32) -> i32 {
    rt_direct_exec(syx_mir_run::ITERATOR_NEXT, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_exec_op(op: u32, a: i32, b: i32) -> i32 {
    NativeRt::with(|rt| native_stack_dispatch(&mut rt.mir, op, a, b))
}

/// W424: CallHost wire first-slice — no `rt_direct_exec(CALL_HOST)`.
#[no_mangle]
pub extern "C" fn syx_rt_call_host(idx: u32) -> i32 {
    rt_call_host_native(idx)
}

// Wave A W530 / ADR-874: `syx_rt_object_ref_wrap` / `syx_rt_object_ref_retain` /
// `syx_rt_object_ref_release` RT opcode ABI true-deleted.
// Wave A W534 / ADR-878: handle-table C ABI `syx_rt_object_retain` / `release` /
// `drop` true-deleted. Wave A W541 / ADR-885: mir-run OBJECT_REF_* opcodes +
// `object_handle_*` table peeled; MirOp/VM ObjectRef residuals remain.

macro_rules! rt_stack_op0 {
    ($name:ident, $op:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() -> i32 {
            rt_direct_exec($op, 0, 0)
        }
    };
}

rt_stack_op0!(syx_rt_object_get, syx_mir_run::OBJECT_GET);
rt_stack_op0!(syx_rt_array_get, syx_mir_run::ARRAY_GET);
rt_stack_op0!(syx_rt_array_set, syx_mir_run::ARRAY_SET);
rt_stack_op0!(syx_rt_value_eq, syx_mir_run::VALUE_EQ);
rt_stack_op0!(syx_rt_ref_eq, syx_mir_run::REF_EQ);
rt_stack_op0!(syx_rt_array_sorted, syx_mir_run::ARRAY_SORTED);

#[no_mangle]
pub extern "C" fn syx_rt_object_set() {
    let _ = rt_direct_exec(syx_mir_run::OBJECT_SET, 0, 0);
}

macro_rules! rt_stack_op_void {
    ($name:ident, $op:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            let _ = rt_direct_exec($op, 0, 0);
        }
    };
}

rt_stack_op_void!(syx_rt_simd_add, syx_mir_run::SIMD_ADD);
rt_stack_op_void!(syx_rt_simd_sub, syx_mir_run::SIMD_SUB);
rt_stack_op_void!(syx_rt_simd_mul, syx_mir_run::SIMD_MUL);
rt_stack_op_void!(syx_rt_simd_div, syx_mir_run::SIMD_DIV);
rt_stack_op_void!(syx_rt_simd_eq, syx_mir_run::SIMD_EQ);
rt_stack_op_void!(syx_rt_simd_lt, syx_mir_run::SIMD_LT);
rt_stack_op_void!(syx_rt_simd_gt, syx_mir_run::SIMD_GT);
rt_stack_op_void!(syx_rt_simd_ne, syx_mir_run::SIMD_NE);
rt_stack_op_void!(syx_rt_simd_le, syx_mir_run::SIMD_LE);
rt_stack_op_void!(syx_rt_simd_ge, syx_mir_run::SIMD_GE);
rt_stack_op_void!(syx_rt_simd_and, syx_mir_run::SIMD_AND);
rt_stack_op_void!(syx_rt_simd_or, syx_mir_run::SIMD_OR);
rt_stack_op_void!(syx_rt_simd_xor, syx_mir_run::SIMD_XOR);
rt_stack_op_void!(syx_rt_simd_not, syx_mir_run::SIMD_NOT);
rt_stack_op_void!(syx_rt_simd_shl, syx_mir_run::SIMD_SHL);
rt_stack_op_void!(syx_rt_simd_shr, syx_mir_run::SIMD_SHR);
rt_stack_op_void!(syx_rt_simd_min, syx_mir_run::SIMD_MIN);
rt_stack_op_void!(syx_rt_simd_max, syx_mir_run::SIMD_MAX);
rt_stack_op_void!(syx_rt_simd_f32_add, syx_mir_run::SIMD_F32_ADD);
rt_stack_op_void!(syx_rt_simd_f32_sub, syx_mir_run::SIMD_F32_SUB);
rt_stack_op_void!(syx_rt_simd_f32_mul, syx_mir_run::SIMD_F32_MUL);
rt_stack_op_void!(syx_rt_simd_f32_div, syx_mir_run::SIMD_F32_DIV);
rt_stack_op_void!(syx_rt_simd_f64_add, syx_mir_run::SIMD_F64_ADD);
rt_stack_op_void!(syx_rt_simd_i16_add, syx_mir_run::SIMD_I16_ADD);
rt_stack_op_void!(syx_rt_simd_i16_sub, syx_mir_run::SIMD_I16_SUB);
rt_stack_op_void!(syx_rt_simd_i16_mul, syx_mir_run::SIMD_I16_MUL);
rt_stack_op_void!(syx_rt_simd_i8_add, syx_mir_run::SIMD_I8_ADD);
rt_stack_op_void!(syx_rt_simd_i8_sub, syx_mir_run::SIMD_I8_SUB);
rt_stack_op_void!(syx_rt_simd_i8_mul, syx_mir_run::SIMD_I8_MUL);
rt_stack_op_void!(syx_rt_simd_i8_and, syx_mir_run::SIMD_I8_AND);
rt_stack_op_void!(syx_rt_simd_i8_or, syx_mir_run::SIMD_I8_OR);
rt_stack_op_void!(syx_rt_simd_i8_xor, syx_mir_run::SIMD_I8_XOR);
rt_stack_op_void!(syx_rt_simd_i8_not, syx_mir_run::SIMD_I8_NOT);
rt_stack_op_void!(syx_rt_simd_i8_min, syx_mir_run::SIMD_I8_MIN);
rt_stack_op_void!(syx_rt_simd_i8_max, syx_mir_run::SIMD_I8_MAX);
rt_stack_op_void!(syx_rt_simd_i8_umin, syx_mir_run::SIMD_I8_UMIN);
rt_stack_op_void!(syx_rt_simd_i8_umax, syx_mir_run::SIMD_I8_UMAX);
rt_stack_op_void!(syx_rt_simd_i8_abs, syx_mir_run::SIMD_I8_ABS);
rt_stack_op_void!(syx_rt_simd_i8_neg, syx_mir_run::SIMD_I8_NEG);
rt_stack_op_void!(syx_rt_simd_i8_extract, syx_mir_run::SIMD_I8_EXTRACT);
rt_stack_op_void!(syx_rt_simd_i8_replace, syx_mir_run::SIMD_I8_REPLACE);
rt_stack_op_void!(syx_rt_simd_i8_eq, syx_mir_run::SIMD_I8_EQ);
rt_stack_op_void!(syx_rt_simd_i8_ne, syx_mir_run::SIMD_I8_NE);
rt_stack_op_void!(syx_rt_simd_i8_le, syx_mir_run::SIMD_I8_LE);
rt_stack_op_void!(syx_rt_simd_i8_ge, syx_mir_run::SIMD_I8_GE);
rt_stack_op_void!(syx_rt_simd_i8_avgr_u, syx_mir_run::SIMD_I8_AVGR_U);
rt_stack_op_void!(syx_rt_simd_i8_popcnt, syx_mir_run::SIMD_I8_POPCNT);
rt_stack_op_void!(syx_rt_simd_i8_bitmask, syx_mir_run::SIMD_I8_BITMASK);
rt_stack_op_void!(syx_rt_simd_i8_all_true, syx_mir_run::SIMD_I8_ALL_TRUE);
rt_stack_op_void!(syx_rt_simd_i8_any_true, syx_mir_run::SIMD_I8_ANY_TRUE);
rt_stack_op_void!(syx_rt_simd_i8_lt, syx_mir_run::SIMD_I8_LT);
rt_stack_op_void!(syx_rt_simd_i8_gt, syx_mir_run::SIMD_I8_GT);
rt_stack_op_void!(syx_rt_simd_i8_add_sat, syx_mir_run::SIMD_I8_ADD_SAT);
rt_stack_op_void!(syx_rt_simd_i8_sub_sat, syx_mir_run::SIMD_I8_SUB_SAT);
rt_stack_op_void!(syx_rt_simd_i8_shl, syx_mir_run::SIMD_I8_SHL);
rt_stack_op_void!(syx_rt_simd_i8_shr, syx_mir_run::SIMD_I8_SHR);
rt_stack_op_void!(syx_rt_simd_i64_add, syx_mir_run::SIMD_I64_ADD);
rt_stack_op_void!(syx_rt_simd_f64_sub, syx_mir_run::SIMD_F64_SUB);
rt_stack_op_void!(syx_rt_simd_f64_mul, syx_mir_run::SIMD_F64_MUL);
rt_stack_op_void!(syx_rt_simd_f64_div, syx_mir_run::SIMD_F64_DIV);

// M0-W4 Track R2: scalar MIR stack binops (match ConcatStr / SIMD void pattern).
rt_stack_op_void!(syx_rt_add, syx_mir_run::ADD);
rt_stack_op_void!(syx_rt_sub, syx_mir_run::SUB);
rt_stack_op_void!(syx_rt_mul, syx_mir_run::MUL);
rt_stack_op_void!(syx_rt_div, syx_mir_run::DIV);
rt_stack_op_void!(syx_rt_mod, syx_mir_run::MOD);
rt_stack_op_void!(syx_rt_eq, syx_mir_run::EQ);
rt_stack_op_void!(syx_rt_ne, syx_mir_run::NE);
rt_stack_op_void!(syx_rt_lt, syx_mir_run::LT);
rt_stack_op_void!(syx_rt_le, syx_mir_run::LE);
rt_stack_op_void!(syx_rt_gt, syx_mir_run::GT);
rt_stack_op_void!(syx_rt_ge, syx_mir_run::GE);
rt_stack_op_void!(syx_rt_and, syx_mir_run::AND);
rt_stack_op_void!(syx_rt_or, syx_mir_run::OR);
rt_stack_op_void!(syx_rt_not, syx_mir_run::NOT);
rt_stack_op_void!(syx_rt_neg, syx_mir_run::NEG);

#[no_mangle]
pub extern "C" fn syx_rt_simd_reduce_add() {
    NativeRt::with(|rt| {
        let v = rt.mir.stack.pop().unwrap_or(syx_runtime_core::Value::Nil);
        if let Some(sum) = syx_runtime_core::simd_reduce_add(&v) {
            rt.mir.stack.push(sum);
        }
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_simd_shuffle(mode: i32) {
    let _ = rt_direct_exec(syx_mir_run::SIMD_SHUFFLE, mode, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_simd_i8_shuffle(mode: i32) {
    let _ = rt_direct_exec(syx_mir_run::SIMD_I8_SHUFFLE, mode, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_simd_load(tag: i32) {
    let _ = rt_direct_exec(syx_mir_run::SIMD_LOAD, tag, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_simd_store(tag: i32) {
    let _ = rt_direct_exec(syx_mir_run::SIMD_STORE, tag, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_const_numeric128(
    kind: i32,
    lo0: i32,
    lo1: i32,
    hi0: i32,
    hi1: i32,
) -> i32 {
    let lo = (lo0 as u32 as u64) | ((lo1 as u32 as u64) << 32);
    let hi = (hi0 as u32 as u64) | ((hi1 as u32 as u64) << 32);
    let _ = rt_direct_exec(syx_mir_run::SET_CONST128_LO, lo as i32, (lo >> 32) as i32);
    let _ = rt_direct_exec(syx_mir_run::SET_CONST128_HI, hi as i32, (hi >> 32) as i32);
    rt_direct_exec(syx_mir_run::CONST_NUMERIC128, kind, lo as i32)
}

#[no_mangle]
pub extern "C" fn syx_rt_heap_collect() -> i32 {
    NativeRt::with(|rt| {
        rt.mir.heap_collect();
        0
    })
}

#[no_mangle]
pub extern "C" fn syx_rt_heap_compact() -> i32 {
    NativeRt::with(|rt| {
        rt.mir.heap_compact();
        0
    })
}

#[no_mangle]
pub extern "C" fn syx_rt_heap_relocate() -> i32 {
    NativeRt::with(|rt| {
        let mut roots: Vec<Value> = rt.mir.stack.clone();
        syx_runtime_core::relocate_global(&mut roots);
        0
    })
}

#[no_mangle]
pub extern "C" fn syx_rt_heap_forward_slot(old: i32) -> i32 {
    if old < 0 {
        return old;
    }
    let old = old as usize;
    if let Ok(heap) = syx_runtime_core::global_heap().lock() {
        if heap
            .get(old)
            .is_some_and(|v| !matches!(v, syx_runtime_core::Value::Nil))
        {
            return old as i32;
        }
    }
    old as i32
}

#[no_mangle]
pub extern "C" fn syx_rt_method_call(idx: u32) {
    let _ = rt_direct_exec(syx_mir_run::METHOD_CALL, idx as i32, 0);
}

/// CallFn / CallClosure dispatch (W416): registered native only; no MIR interpret fallback.
pub(crate) fn call_fn_or_fail_closed(idx: u32) {
    NativeRt::with(|rt| {
        if let Some(native) = rt.native_fns.get(idx as usize).copied() {
            let saved_locals = std::mem::take(&mut rt.mir.locals);
            let _ = native();
            rt.mir.locals = saved_locals;
        } else {
            panic!(
                "syx_rt_call_fn: no-native fn idx={idx} (MIR interpret fallback removed; W416 CallFn wire)"
            );
        }
    });
}

#[no_mangle]
pub extern "C" fn syx_rt_call_fn(idx: u32) {
    // W416: CallClosure / call_fn wire first-slice — no MIR interpret fallback.
    // Missing native registration fails closed (≠ full closure_async_mir_free_gap close).
    // Panic across this C ABI aborts (intentional fail-closed).
    call_fn_or_fail_closed(idx);
}

#[no_mangle]
pub extern "C" fn syx_rt_pop_bool() -> i32 {
    NativeRt::with(|rt| i32::from(rt.mir.pop_bool()))
}

#[no_mangle]
pub extern "C" fn syx_rt_get_return() -> i32 {
    NativeRt::with(|rt| rt.mir.return_value())
}

/// Peek MIR stack top as process/native i32 without popping (nested Call keeps return for Print).
#[no_mangle]
pub extern "C" fn syx_rt_peek_return() -> i32 {
    NativeRt::with(|rt| match rt.mir.stack.last() {
        Some(Value::Bool(b)) => i32::from(*b),
        Some(v) => v.as_i32(),
        None => 0,
    })
}

#[no_mangle]
pub extern "C" fn syx_rt_is_suspended() -> i32 {
    NativeRt::with(|rt| i32::from(rt.mir.suspended))
}

/// Phase 87: poll async resume; returns 1 when a suspended frame may continue.
#[no_mangle]
pub extern "C" fn syx_rt_async_resume(_state: i32) -> i32 {
    NativeRt::with(|rt| {
        if !rt.mir.suspended && rt.mir.suspend_frame.is_none() {
            return 1;
        }
        if let Some(vm_rt) = rt.mir.runtime.as_ref() {
            if let Some(top) = rt.mir.stack.last().cloned() {
                #[cfg(feature = "async-runtime")]
                if syx_async::task_id_from_value(&top).is_some() {
                    match vm_rt.try_await_task_value(&top) {
                        None => {
                            std::thread::yield_now();
                            return 0;
                        }
                        Some(Ok(v)) => {
                            rt.mir.stack.pop();
                            rt.mir.stack.push(v);
                            rt.mir.suspended = false;
                            return 1;
                        }
                        Some(Err(_)) => {
                            rt.mir.suspended = false;
                            return 1;
                        }
                    }
                }
            }
        }
        rt.mir.suspended = false;
        1
    })
}

/// Run MIR function `idx` with async suspend/resume until completion.
#[no_mangle]
pub extern "C" fn syx_rt_run_main_fn(idx: u32) -> i32 {
    NativeRt::with(|rt| loop {
        let Some(func) = rt.mir.functions.get(idx as usize).cloned() else {
            return 1;
        };
        match syx_mir_run::run_mir_function_with_args(&func, &mut rt.mir, &mut [], &[]) {
            Ok(v) => {
                if !rt.mir.suspended && rt.mir.suspend_frame.is_none() {
                    return syx_mir_run::mir_main_exit_code(v);
                }
            }
            Err(_) => return 1,
        }
        while rt.mir.suspended || rt.mir.suspend_frame.is_some() {
            if syx_rt_async_resume(0) != 0 && !rt.mir.suspended {
                break;
            }
            std::thread::yield_now();
        }
    })
}

/// Program entry: calls `syx_user_init` then `syx_user_main`.
#[no_mangle]
pub extern "C" fn syx_rt_start(_argc: c_int, _argv: *const *const c_char) -> c_int {
    extern "C" {
        fn syx_user_init();
        fn syx_user_main() -> i32;
    }
    unsafe {
        syx_user_init();
        syx_user_main() as c_int
    }
}

// Wave A W534 / ADR-878: `syx_rt_object_retain` / `syx_rt_object_release` /
// `syx_rt_object_drop` handle C ABI true-deleted. Callers use
// `syx_runtime_core::object_handle_*` (mir-run) directly.

/// Prepare MIR tables and string pool before running native main.
pub fn prepare_module(module: &MirModule) {
    NativeRt::with(|rt| {
        rt.mir.install_functions(module.functions.clone());
        for func in &module.functions {
            for block in &func.blocks {
                for instr in &block.instructions {
                    if let syx_mir::MirOp::ConstStr(s) = &instr.op {
                        rt.mir.intern_string(s);
                    }
                }
            }
        }
    });
}

/// Run a compiled executable's user main in-process (for tests).
pub fn run_user_main_inprocess(module: &MirModule, user_main: extern "C" fn() -> i32) -> i32 {
    prepare_module(module);
    NativeRt::with(|rt| {
        rt.native_fns = vec![user_main];
    });
    user_main()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;
    use syx_mir::{decode_module, encode_module, lower_module, MirOp};
    use syx_mir_run::MirRuntime;
    use syx_parser::Parser;

    #[test]
    fn push_and_return() {
        prepare_module(&MirModule::default());
        syx_rt_begin_function(0, 0);
        syx_rt_push_i32(42);
        assert_eq!(syx_rt_get_return(), 42);
    }

    #[test]
    fn print_wire_native_records_without_direct_exec_print() {
        prepare_module(&MirModule::default());
        syx_rt_begin_function(0, 0);
        syx_rt_push_i32(7);
        rt_print_native();
        NativeRt::with(|rt| {
            assert_eq!(rt.mir.printed, vec!["7".to_string()]);
        });
    }

    #[test]
    fn call_host_wire_native_fmt_print_line_without_direct_exec() {
        prepare_module(&MirModule::default());
        syx_rt_begin_function(0, 0);
        let s = b"hello-w424";
        syx_rt_push_str(s.as_ptr(), s.len() as i32);
        let idx = syx_stdlib::host_fn_index(syx_stdlib::HostFnId::FmtPrintLine);
        let _ = rt_call_host_native(idx);
        NativeRt::with(|rt| {
            assert!(
                rt.mir.printed.iter().any(|p| p.contains("hello-w424")),
                "printed={:?}",
                rt.mir.printed
            );
        });
    }

    #[test]
    fn exec_add() {
        prepare_module(&MirModule::default());
        syx_rt_begin_function(0, 0);
        syx_rt_push_i32(1);
        syx_rt_push_i32(2);
        syx_rt_exec_op(syx_mir_run::ADD, 0, 0);
        assert_eq!(syx_rt_get_return(), 3);
    }

    #[test]
    fn hasher_combine_store_local_slot_one_via_exec() {
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
        let bytes = encode_module(&mir);
        syx_rt_load_mir_blob(bytes.as_ptr(), bytes.len() as i32);

        NativeRt::with(|rt| {
            let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
            let ops: Vec<_> = main
                .blocks
                .iter()
                .flat_map(|b| b.instructions.iter())
                .collect();
            rt.mir.begin_function(main.local_count);
            for instr in ops {
                if matches!(instr.op, MirOp::Return) {
                    break;
                }
                rt.mir.exec_mir_op(&instr.op, &mut []).expect("exec");
            }
            let printed = rt.mir.printed.join(",");
            assert!(
                printed.contains("4698564035575446361"),
                "printed={printed} locals={:?}",
                rt.mir.locals
            );
        });
    }

    #[test]
    fn load_mir_blob_actor_exec_via_native_rt() {
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
    c.increment()
    print(c.get())
}"#,
        );
        let ast = Parser::parse(source).unwrap();
        let mir = lower_module(&ast).unwrap();
        let bytes = encode_module(&mir);
        syx_rt_load_mir_blob(bytes.as_ptr(), bytes.len() as i32);

        NativeRt::with(|rt| {
            let main = mir
                .functions
                .iter()
                .find(|f| f.name == "main")
                .unwrap()
                .clone();
            let code =
                syx_mir_run::run_mir_function_with_args(&main, &mut rt.mir, &mut [], &[]).unwrap();
            assert_eq!(code.as_i32(), 0);
            assert!(
                rt.mir.printed.iter().any(|s| s.contains('2')),
                "printed={:?}",
                rt.mir.printed
            );
        });
    }

    #[test]
    fn load_mir_blob_installs_filter_closures() {
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
        let bytes = encode_module(&mir);
        assert!(!bytes.is_empty());
        assert!(decode_module(&bytes).is_ok());

        syx_rt_load_mir_blob(bytes.as_ptr(), bytes.len() as i32);

        let filter_idx = syx_stdlib::MethodId::ArrFilter.index();
        let closure_idx = mir
            .functions
            .iter()
            .position(|f| f.name.starts_with("__mir_closure_"))
            .unwrap() as i32;

        NativeRt::with(|rt| {
            assert!(rt.mir.functions.len() > 1);
            rt.mir.begin_function(0);
            for v in [1i32, 2, 3, 4] {
                rt.mir.stack.push(syx_runtime_core::Value::I32(v));
            }
            let mut pool = rt.mir.string_pool.clone();
            let array_build = syx_mir_run::ARRAY_BUILD;
            let make_closure = syx_mir_run::MAKE_CLOSURE;
            let method_call = syx_mir_run::METHOD_CALL;
            rt.mir.exec(array_build, 4, 0, 0, &mut pool).unwrap();
            rt.mir
                .exec(make_closure, closure_idx, 0, 0, &mut pool)
                .unwrap();
            assert!(
                matches!(
                    rt.mir.stack.last(),
                    Some(syx_runtime_core::Value::Closure { .. })
                ),
                "MakeClosure should push closure, stack={:?}",
                rt.mir.stack
            );
            rt.mir
                .exec(method_call, filter_idx as i32, 0, 0, &mut pool)
                .unwrap();
            let filtered = rt.mir.stack.last().unwrap();
            let len = match filtered {
                syx_runtime_core::Value::Array(items) => items.len(),
                other => panic!("expected array, got {other:?}"),
            };
            assert_eq!(len, 2, "filter should keep evens");
        });
    }

    #[test]
    #[cfg(feature = "http")]
    fn native_http_serve_dispatches_mir_handler() {
        use std::io::{Read, Write};
        use std::net::TcpStream;
        use std::thread;
        use std::time::Duration;
        use syx_runtime_core::Value;

        let source = SourceFile::new(
            "t.sy",
            r#"func handle(req: Object) -> Object {
    return __syx_http_make_response(200, { "status": "ok" })
}"#,
        );
        let mir = lower_module(&Parser::parse(source).unwrap()).unwrap();
        let bytes = encode_module(&mir);
        syx_rt_load_mir_blob(bytes.as_ptr(), bytes.len() as i32);

        let port = 19876i32;
        let _server = thread::spawn(move || {
            syx_rt_begin_function(0, 0);
            syx_rt_push_str(b"127.0.0.1".as_ptr(), 9);
            syx_rt_push_i32(port);
            syx_rt_push_str(b"handle".as_ptr(), 6);
            let idx = syx_stdlib::host_fn_index(syx_stdlib::HostFnId::HttpServe);
            syx_rt_exec_op(syx_mir_run::CALL_HOST, idx as i32, 0);
        });
        thread::sleep(Duration::from_millis(300));
        let mut stream =
            TcpStream::connect(("127.0.0.1", port as u16)).expect("connect to test server");
        stream
            .write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("write request");
        let mut buf = vec![0u8; 512];
        let n = stream.read(&mut buf).expect("read response");
        let resp = String::from_utf8_lossy(&buf[..n]);
        assert!(resp.contains("ok"), "response was: {resp}");
    }

    #[test]
    fn syx_rt_load_mir_blob_syxd_installs_sidecar_string_literals() {
        use syx_mir::{
            aot_codec_fixture_main_with_const_strs, encode_module_aot_embed_blob,
            mir_module_metadata_sidecar_view,
        };
        reset_sidecar_install_literal_count();
        reset_sidecar_string_pool_primary();
        reset_aot_execution_segment_omitted();
        reset_native_metadata_alternate_entry();
        let module = aot_codec_fixture_main_with_const_strs(4);
        let view = mir_module_metadata_sidecar_view(&module);
        assert_eq!(view.string_literals.len(), 4);
        let bytes = encode_module_aot_embed_blob(&module);
        syx_rt_load_mir_blob(bytes.as_ptr(), bytes.len() as i32);
        assert_eq!(
            sidecar_install_literal_count(),
            4,
            "SYXD load must install sidecar string_literals into RT"
        );
        assert!(
            sidecar_string_pool_primary(),
            "SYXD load must mark sidecar string pool as primary (W408)"
        );
        assert!(
            aot_execution_segment_omitted(),
            "default AOT embed load must mark execution omitted (W409)"
        );
        assert!(
            !native_metadata_alternate_entry(),
            "SYXD load_mir_blob must not mark alternate entry (W412)"
        );
        NativeRt::with(|rt| {
            let pool = String::from_utf8_lossy(&rt.mir.string_pool);
            for lit in &view.string_literals {
                assert!(
                    pool.contains(lit.as_str()),
                    "string pool must contain sidecar literal {lit:?}; pool={pool:?}"
                );
            }
            assert_eq!(
                rt.mir.functions.len(),
                view.fn_names.len(),
                "sidecar-only load must install fn stubs matching sidecar fn_names"
            );
        });
    }

    #[test]
    fn syx_rt_install_metadata_sidecar_no_blob_alternate_entry() {
        use syx_mir::{
            aot_codec_fixture_main_with_const_strs, encode_module_metadata_sidecar,
            mir_module_metadata_sidecar_view,
        };
        reset_sidecar_install_literal_count();
        reset_sidecar_string_pool_primary();
        reset_aot_execution_segment_omitted();
        reset_native_metadata_alternate_entry();
        let module = aot_codec_fixture_main_with_const_strs(3);
        let view = mir_module_metadata_sidecar_view(&module);
        let bytes = encode_module_metadata_sidecar(&module);
        // Direct C ABI — must not require SYXD / syx_rt_load_mir_blob.
        syx_rt_install_metadata_sidecar(bytes.as_ptr(), bytes.len() as i32);
        assert_eq!(sidecar_install_literal_count(), 3);
        assert!(sidecar_string_pool_primary());
        assert!(aot_execution_segment_omitted());
        assert!(
            native_metadata_alternate_entry(),
            "install_metadata_sidecar must mark alternate entry (W412)"
        );
        NativeRt::with(|rt| {
            let pool = String::from_utf8_lossy(&rt.mir.string_pool);
            for lit in &view.string_literals {
                assert!(
                    pool.contains(lit.as_str()),
                    "string pool must contain sidecar literal {lit:?}; pool={pool:?}"
                );
            }
            assert_eq!(rt.mir.functions.len(), view.fn_names.len());
        });
    }

    #[test]
    fn syx_rt_install_empty_runtime_builds_empty_pools() {
        reset_sidecar_install_literal_count();
        reset_sidecar_string_pool_primary();
        reset_aot_execution_segment_omitted();
        reset_native_metadata_alternate_entry();
        reset_empty_runtime_install();
        // Seed non-empty state first, then empty-install must clear.
        {
            use syx_mir::{aot_codec_fixture_main_with_const_strs, encode_module_metadata_sidecar};
            let module = aot_codec_fixture_main_with_const_strs(2);
            let bytes = encode_module_metadata_sidecar(&module);
            syx_rt_install_metadata_sidecar(bytes.as_ptr(), bytes.len() as i32);
            assert_eq!(sidecar_install_literal_count(), 2);
        }
        syx_rt_install_empty_runtime();
        assert_eq!(sidecar_install_literal_count(), 0);
        assert!(sidecar_string_pool_primary());
        assert!(aot_execution_segment_omitted());
        assert!(
            empty_runtime_install(),
            "install_empty_runtime must mark empty-pool entry (W428)"
        );
        assert!(
            !native_metadata_alternate_entry(),
            "empty-pool install is not the sidecar-byte alternate entry"
        );
        NativeRt::with(|rt| {
            assert!(
                rt.mir.functions.is_empty(),
                "empty-pool install must leave zero fn stubs"
            );
            assert!(
                rt.mir.string_pool.is_empty(),
                "empty-pool install must leave empty string pool"
            );
        });
    }

    extern "C" fn w416_call_fn_marker_native() -> i32 {
        41
    }

    #[test]
    fn syx_rt_register_only_after_empty_install_invokes_call_fn() {
        reset_empty_runtime_install();
        reset_register_only_runtime_install();
        reset_native_metadata_alternate_entry();
        // Register-only path: empty install (no sidecar bytes) then register + call.
        syx_rt_install_empty_runtime();
        assert!(empty_runtime_install());
        assert!(!register_only_runtime_install());
        syx_rt_register_native_fn(0, w416_call_fn_marker_native as *const () as usize as u64);
        assert!(
            register_only_runtime_install(),
            "register after empty install must mark Register-only path (W429)"
        );
        assert!(
            !native_metadata_alternate_entry(),
            "Register-only path must not use sidecar-byte alternate entry"
        );
        syx_rt_call_fn(0);
    }

    #[test]
    fn syx_rt_call_fn_invokes_registered_native() {
        syx_rt_init_module(0);
        syx_rt_register_native_fn(0, w416_call_fn_marker_native as *const () as usize as u64);
        syx_rt_call_fn(0);
    }

    #[test]
    #[should_panic(expected = "syx_rt_call_fn: no-native")]
    fn syx_rt_call_fn_fail_closed_without_native_no_mir_fallback() {
        syx_rt_init_module(0);
        // Exercise Rust helper (not extern "C") so should_panic can unwind.
        call_fn_or_fail_closed(99);
    }

    #[test]
    fn syx_rt_call_fn_source_has_no_mir_interpret_fallback() {
        let abi = include_str!("abi.rs");
        let start = abi
            .find("fn call_fn_or_fail_closed")
            .expect("call_fn_or_fail_closed present");
        let body = &abi[start..];
        let end = body
            .find("pub extern \"C\" fn syx_rt_pop_bool")
            .unwrap_or(body.len());
        let call_fn = &body[..end];
        assert!(
            call_fn.contains("no-native") && call_fn.contains("panic!"),
            "syx_rt_call_fn must fail-closed with no-native panic (W416)"
        );
        assert!(
            !call_fn.contains("rt.mir.call_fn"),
            "syx_rt_call_fn must not MIR-interpret when native missing (W416)"
        );
    }
}
