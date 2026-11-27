//! C-3x: MIR Pop stack-height floor fail-closed (MIR024).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_pop_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3x"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3x_rejects_pop_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::Pop, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_pop_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR024"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("Pop") && text.contains("C-3x") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3x_pop_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::Pop, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_pop_stack_floor(&mir).expect("ok");
}

#[test]
fn c3x_no_pop_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_pop_stack_floor(&mir).expect("no Pop");
}

#[test]
fn c3x_empty_ok() {
    verify_mir_pop_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3x_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::Dup, MirOp::Pop, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_pop_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3x_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::Pop, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_pop_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3x_compile_from_mir_rejects_pop_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Pop, MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR024"), "{err}");
}

#[test]
fn c3x_compile_from_mir_accepts_pop() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::Pop, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3x_hooks_after_succ_term() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let succ_u = unify
        .find("verify_mir_succ_term_floor")
        .expect("succ-term unify");
    let pop_u = unify
        .find("verify_mir_pop_stack_floor")
        .expect("pop-stack unify");
    assert!(
        pop_u > succ_u,
        "unify pop-stack floor must follow succ-term floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let succ_c = body
        .find("verify_mir_succ_term_floor_functions")
        .expect("succ-term codegen");
    let pop_c = body
        .find("verify_mir_pop_stack_floor_functions")
        .expect("pop-stack codegen");
    assert!(
        pop_c > succ_c,
        "codegen pop-stack floor must follow succ-term floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_pop_stack_floor.rs"))
        .expect("mir_pop_stack_floor.rs");
    assert!(
        api.contains("MIR024")
            && api.contains("C-3x")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("Pop")
            && api.contains("verify_mir_pop_stack_floor")
    );
}
