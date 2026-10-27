//! JIT Suite-1 differential vs interpreter (opt-in tier-2).
// was: jit_suite1_differential
//!
//! Requires `SYX_JIT_TIER2=1`. Pure arith / branch / nested call must match interpreter
//! exactly via native entry; fixtures with `print` / CallHost / ConstStr must deopt.
//! W363 / ADR-707: `jit_callhost_deopts` + `jit_conststr_deopts` deepen nails.
//! W364 / ADR-708: `jit_arraybuild_deopts` + `jit_dictbuild_deopts` deepen batch-2.
//! W365 / ADR-709: `jit_setbuild_deopts` + `jit_loadglobal_deopts` deepen batch-3.

use std::path::PathBuf;
use std::sync::Mutex;

use syx_driver::{compile_file_with_options, DriverOptions};
use syx_runtime_core::Value;
use syx_vm::WasmVM;

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/jit_suite1")
        .join(name)
}

fn compile_prog(name: &str) -> syx_codegen::CompiledProgram {
    let path = fixture(name);
    let unit = compile_file_with_options(&path, &DriverOptions::default())
        .unwrap_or_else(|e| panic!("compile {}: {e}", path.display()));
    assert!(
        unit.program.mir_module.is_some(),
        "expected mir_module for JIT"
    );
    unit.program
}

fn find_fn<'a>(
    program: &'a syx_codegen::CompiledProgram,
    name: &str,
) -> &'a syx_codegen::FunctionUnit {
    program
        .functions
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("missing function {name}"))
}

fn run_interp(program: &syx_codegen::CompiledProgram, name: &str, args: &[Value]) -> Value {
    let _g = env_lock();
    std::env::remove_var("SYX_JIT_TIER2");
    std::env::remove_var("SYX_JIT_FASTPATH");
    let mut vm = WasmVM::new();
    let func = find_fn(program, name);
    vm.run_function_with_args(func, program, args)
        .expect("interpreter run")
}

fn run_jit(program: &syx_codegen::CompiledProgram, name: &str, args: &[Value]) -> (Value, WasmVM) {
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    std::env::remove_var("SYX_JIT_FASTPATH");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(program, name);
    // Hot threshold is 1 under tier2 — first call compiles + invokes native.
    let v = vm
        .run_function_with_args(func, program, args)
        .expect("jit run");
    std::env::remove_var("SYX_JIT_TIER2");
    (v, vm)
}

fn assert_i32(v: &Value, expect: i32) {
    assert_eq!(v.as_i32(), expect, "value={v:?}");
}

#[test]
fn jit_arith_exact() {
    let program = compile_prog("arith.sy");
    let interp = run_interp(&program, "compute", &[]);
    let (jit, vm) = run_jit(&program, "compute", &[]);
    assert_i32(&interp, 15);
    assert_eq!(interp.as_i32(), jit.as_i32());
    assert!(vm.jit_native_ready("compute") || vm.jit_native_call_count() >= 1);
    assert!(vm.jit_native_call_count() >= 1, "expected native invoke");
}

#[test]
fn jit_branch_exact() {
    let program = compile_prog("branch.sy");
    let interp = run_interp(&program, "branch", &[]);
    let (jit, vm) = run_jit(&program, "branch", &[]);
    assert_i32(&interp, 1);
    assert_eq!(interp.as_i32(), jit.as_i32());
    assert!(vm.jit_native_call_count() >= 1);
}

#[test]
fn jit_call_nested_exact() {
    let program = compile_prog("call_nested.sy");
    let interp = run_interp(&program, "nest", &[Value::I32(1)]);
    let (jit, vm) = run_jit(&program, "nest", &[Value::I32(1)]);
    assert_i32(&interp, 3);
    assert_eq!(interp.as_i32(), jit.as_i32());
    assert!(vm.jit_native_call_count() >= 1);
}

#[test]
fn jit_print_deopts() {
    let program = compile_prog("print_deopt.sy");
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let _ = vm
        .run_function_with_args(func, &program, &[])
        .expect("print runs on interpreter after deopt");
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for Print, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(Print)") || reason.contains("Print"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_callhost_deopts() {
    let program = compile_prog("callhost_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for CallHost nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_callhost = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::CallHost(_)));
    assert!(has_callhost, "callhost_deopt.sy main must contain CallHost");
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("CallHost runs on interpreter after deopt");
    assert!(
        matches!(v, Value::I32(_) | Value::I64(_)),
        "expected Int from time_now_ms, got {v:?}"
    );
    assert!(v.as_i32() > 0, "expected positive epoch ms, got {}", v.as_i32());
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for CallHost, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(CallHost)") || reason.contains("CallHost"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_conststr_deopts() {
    let program = compile_prog("conststr_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for ConstStr nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_conststr = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::ConstStr(_)));
    assert!(has_conststr, "conststr_deopt.sy main must contain ConstStr");
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("ConstStr runs on interpreter after deopt");
    assert_eq!(
        v.as_str(),
        "hi",
        "expected String \"hi\" from conststr fixture, got {v:?}"
    );
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for ConstStr, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(ConstStr)") || reason.contains("ConstStr"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_arraybuild_deopts() {
    let program = compile_prog("arraybuild_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for ArrayBuild nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_arraybuild = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::ArrayBuild(_)));
    assert!(
        has_arraybuild,
        "arraybuild_deopt.sy main must contain ArrayBuild"
    );
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("ArrayBuild runs on interpreter after deopt");
    let len = Value::materialize_array(v.clone())
        .map(|a| a.len())
        .unwrap_or(0);
    assert_eq!(len, 2, "expected array of length 2, got {v:?}");
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for ArrayBuild, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(ArrayBuild(2))") || reason.contains("ArrayBuild"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_dictbuild_deopts() {
    let program = compile_prog("dictbuild_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for DictBuild nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_dictbuild = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::DictBuild(_)));
    assert!(
        has_dictbuild,
        "dictbuild_deopt.sy main must contain DictBuild"
    );
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("DictBuild runs on interpreter after deopt");
    let n = Value::materialize_dict(v.clone())
        .map(|d| d.len())
        .unwrap_or(0);
    assert!(
        n >= 1,
        "expected dict/object with >=1 entry after DictBuild, got n={n} v={v:?}"
    );
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for DictBuild, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(DictBuild(1))") || reason.contains("DictBuild"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_setbuild_deopts() {
    let program = compile_prog("setbuild_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for SetBuild nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_setbuild = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::SetBuild(_)));
    assert!(
        has_setbuild,
        "setbuild_deopt.sy main must contain SetBuild"
    );
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("SetBuild runs on interpreter after deopt");
    // Set may be inline Set or HeapSlot; just require non-nil successful fallback.
    assert!(
        !matches!(v, Value::Nil),
        "expected Set value after SetBuild fallback, got {v:?}"
    );
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for SetBuild, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(SetBuild(2))") || reason.contains("SetBuild"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}

#[test]
fn jit_loadglobal_deopts() {
    let program = compile_prog("loadglobal_deopt.sy");
    let mir = program
        .mir_module
        .as_ref()
        .expect("mir_module for LoadGlobal nail");
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let has_loadglobal = main
        .blocks
        .iter()
        .flat_map(|b| b.instructions.iter())
        .chain(main.instructions.iter())
        .any(|i| matches!(i.op, syx_mir::MirOp::LoadGlobal(_)));
    assert!(
        has_loadglobal,
        "loadglobal_deopt.sy main must contain LoadGlobal"
    );
    let _g = env_lock();
    std::env::set_var("SYX_JIT_TIER2", "1");
    syx_cranelift_jit::invalidate_jit_cache();
    let mut vm = WasmVM::new();
    let func = find_fn(&program, "main");
    let v = vm
        .run_function_with_args(func, &program, &[])
        .expect("LoadGlobal runs on interpreter after deopt");
    // Isolated fn entry may not run module-level var init; value may be Nil.
    // Nail is deopt observability (LoadGlobal reason), not global-init GA.
    let _ = v;
    assert!(
        vm.jit_deopt_count() >= 1,
        "expected deopt for LoadGlobal, got reason={:?}",
        vm.jit_last_deopt_reason()
    );
    let reason = vm.jit_last_deopt_reason().unwrap_or("");
    assert!(
        reason.contains("UnsupportedMirOp(LoadGlobal(0))") || reason.contains("LoadGlobal"),
        "reason={reason}"
    );
    assert!(!vm.jit_native_ready("main"));
    std::env::remove_var("SYX_JIT_TIER2");
}
