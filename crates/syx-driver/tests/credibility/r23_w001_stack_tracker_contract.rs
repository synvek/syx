//! R-23 W001 StackTracker + Call-stub Drop contract.
//!
//! After four hung windows, census required emit-time stack observability before
//! another guessed patch. R-23 delivered `stack_tracker` and, from
//! `wasm_validate_probe` evidence on `print(1)`, a single targeted fix: drop the
//! guest i32 stub that `MirOp::Call` + `take_catch` left on the operand stack.

use std::path::PathBuf;
use syx_driver::{build_wasm_file_with_options, DriverOptions};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn r23_stack_tracker_module_exists() {
    let path = repo_root().join("crates/syx-wasm-emit/src/stack_tracker.rs");
    let src = std::fs::read_to_string(&path).expect("stack_tracker.rs");
    assert!(
        src.contains("pub struct StackTracker"),
        "StackTracker type must exist"
    );
    assert!(
        src.contains("W001 stack tracker"),
        "diagnostics must carry the stable keyword"
    );
    assert!(
        src.contains("values remaining on stack"),
        "End-of-block leak wording must stay"
    );
    assert!(
        src.contains("fn begin_function") && src.contains("fn track_instruction"),
        "emit wiring hooks must exist"
    );
}

#[test]
fn r23_stack_tracker_wired_into_mir_emit() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    let lib = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/lib.rs"))
        .expect("lib.rs");
    assert!(
        lib.contains("mod stack_tracker"),
        "stack_tracker must be a crate module"
    );
    assert!(
        emit.contains("stack_tracker::begin_function"),
        "function body must activate the tracker"
    );
    assert!(
        emit.contains("emit_wasm_instr"),
        "wasm instructions must go through the tracked helper"
    );
    assert!(
        emit.contains("stack_tracker::set_mir_context"),
        "MIR site context must be set before emit_mir_instr"
    );
}

#[test]
fn r23_call_take_catch_drops_guest_stub() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    assert!(
        emit.contains("fn leaves_guest_call_stub"),
        "Call/CallHost stub detector must exist"
    );
    assert!(
        emit.contains("leaves_guest_call_stub(op)") && emit.contains("next_is_pop"),
        "unused Call stubs must Drop unless the next op is Pop"
    );
    assert!(
        emit.contains("R-23: Call-stub cleanup")
            || emit.contains("R-23: guest i32 stub left on the stack"),
        "Call path must document the R-23 cleanup"
    );
}

#[test]
fn r23_intentional_leak_unit_test_present() {
    let src = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/stack_tracker.rs"))
        .expect("stack_tracker.rs");
    assert!(
        src.contains("r23_intentional_leak_diagnoses_mir_site"),
        "unit test for intentional leak must exist"
    );
}

#[test]
fn r23_minimal_print_program_emits_valid_wasm() {
    // Unique temp path avoids incremental-cache ghosts from earlier W001 probes.
    let dir = std::env::temp_dir().join(format!(
        "syx_r23_wasm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("main.sy");
    std::fs::write(&file, "func main() {\n    print(1)\n}\n").expect("write");
    let bytes = build_wasm_file_with_options(&file, &DriverOptions::default())
        .expect("minimal print(1) must emit valid wasm after R-23");
    assert!(!bytes.is_empty(), "wasm bytes must be non-empty");
    wasmparser::validate(&bytes).expect("emitted module must pass wasmparser::validate");
    let _ = std::fs::remove_dir_all(&dir);
}
