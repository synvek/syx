//! R-24 W001 residual contract: hello fixture retarget + Drop/Pop + string stubs.
//!
//! R-23 made minimal `print(1)` validate. R-24 clears D001 hello false-negatives,
//! stops Drop/Pop double-cleanup, and (tracker evidence) pushes guest placeholders
//! for `ToStr` / `ConcatStr` so greeter/dataclass shells emit.

use std::path::PathBuf;
use syx_driver::{build_wasm_file_with_options, DriverOptions};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn r24_hello_fixture_points_at_examples() {
    let full = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_full_program_aot.rs"),
    )
    .expect("wasm_full_program_aot.rs");
    let hybrid = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_hybrid_aot.rs"),
    )
    .expect("wasm_hybrid_aot.rs");
    assert!(
        full.contains("examples/hello.sy") && hybrid.contains("examples/hello.sy"),
        "full/hybrid AOT tests must retarget hello to examples/hello.sy"
    );
    assert!(
        repo_root().join("examples/hello.sy").is_file(),
        "examples/hello.sy must exist as the single source"
    );
}

#[test]
fn r24_drop_pop_skips_when_next_is_pop() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    assert!(
        emit.contains("fn next_is_pop"),
        "next_is_pop helper must remain"
    );
    assert!(
        emit.contains("if next_is_pop(ops, idx)") && emit.contains("return;"),
        "drop_unused_guest_cond must defer cleanup when next MirOp is Pop"
    );
    assert!(
        emit.contains("MirOp::Print") && emit.contains("leaves_guest_call_stub"),
        "Print must participate in stub cleanup (not unconditional Drop)"
    );
}

#[test]
fn r24_simd_i8_extract_host_stub_dropped() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    let tracker =
        std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/stack_tracker.rs"))
            .expect("stack_tracker.rs");
    assert!(
        emit.contains("R-24-03 (tracker: SyxHost_simd_i8_extract leftover height=1)"),
        "SimdI8Extract host-path Drop must cite tracker evidence"
    );
    assert!(
        emit.contains("drop_host_stub"),
        "host CallHost path for SimdI8Extract must Drop the guest stub"
    );
    assert!(
        tracker.contains("fn block_result_arity")
            && tracker.contains("R-24-03: `Block(Result(I32))`"),
        "tracker must honor block result arity (SimdI8Extract pilot false positive)"
    );
    assert!(
        tracker.contains("r24_block_result_i32_keeps_arity"),
        "unit test for Block(Result(i32)) must exist"
    );
}

#[test]
fn r24_tostr_concat_push_guest_placeholders() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    assert!(
        emit.contains("R-24-03 (tracker: Greeter_greet ToStr underflow")
            || emit.contains("R-24-03 (tracker: Point_describe ConcatStr underflow"),
        "ToStr/ConcatStr placeholder fix must cite tracker evidence"
    );
    // Both imports need dummy i32 args before Call (host uses RT Values).
    let to_str_arm = emit
        .split("if matches!(instr.op, MirOp::ToStr)")
        .nth(1)
        .expect("ToStr AOT arm");
    let to_str_body = to_str_arm.split("return;").next().expect("ToStr body");
    assert!(
        to_str_body.contains("I32Const(0)"),
        "ToStr must push a guest placeholder before Call"
    );
    let concat_arm = emit
        .split("if matches!(instr.op, MirOp::ConcatStr)")
        .nth(1)
        .expect("ConcatStr AOT arm");
    let concat_body = concat_arm.split("return;").next().expect("ConcatStr body");
    assert!(
        concat_body.matches("I32Const(0)").count() >= 2,
        "ConcatStr must push two guest placeholders before Call (got {})",
        concat_body.matches("I32Const(0)").count()
    );
}

#[test]
fn r24_minimal_print_program_still_validates() {
    let dir = std::env::temp_dir().join(format!(
        "syx_r24_wasm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("main.sy");
    std::fs::write(&file, "func main() {\n    print(1)\n}\n").expect("write");
    let bytes = build_wasm_file_with_options(&file, &DriverOptions::default())
        .expect("minimal print(1) must still emit after R-24");
    assert!(!bytes.is_empty());
    wasmparser::validate(&bytes).expect("emitted module must pass wasmparser::validate");
    let _ = std::fs::remove_dir_all(&dir);
}
