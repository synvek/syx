//! C-3q: MIR Array/Dict/SetBuild stack-height floor fail-closed (MIR017).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_build_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3q"),
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
fn c3q_rejects_array_build_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::ConstI32(1), MirOp::ArrayBuild(2), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_build_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR017"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("ArrayBuild") && text.contains("C-3q") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3q_array_build_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::ArrayBuild(2),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_build_stack_floor(&mir).expect("ok");
}

#[test]
fn c3q_dict_build_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::ConstI32(1), MirOp::DictBuild(1), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_build_stack_floor(&mir).expect_err("dict");
    assert_eq!(err.code.as_deref(), Some("MIR017"), "{err}");
}

#[test]
fn c3q_no_build_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_build_stack_floor(&mir).expect("no Build");
}

#[test]
fn c3q_empty_ok() {
    verify_mir_build_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3q_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::Dup, MirOp::ArrayBuild(9), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_build_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3q_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::ArrayBuild(1), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_build_stack_floor(&mir).expect("StoreLocal abandon ≠ MIR017");
}

#[test]
fn c3q_compile_from_mir_rejects_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ArrayBuild(1), MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR017"), "{err}");
}

#[test]
fn c3q_compile_from_mir_accepts_array_build() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstI32(1),
                MirOp::ArrayBuild(1),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3q_hooks_after_named_callee() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let callee_u = unify
        .find("verify_mir_named_callee_floor_functions")
        .expect("named-callee unify");
    let build_u = unify
        .find("verify_mir_build_stack_floor")
        .expect("build-stack unify");
    assert!(
        build_u > callee_u,
        "unify build-stack floor must follow named-callee floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let callee_c = body
        .find("verify_mir_named_callee_floor_functions")
        .expect("named-callee codegen");
    let build_c = body
        .find("verify_mir_build_stack_floor_functions")
        .expect("build-stack codegen");
    assert!(
        build_c > callee_c,
        "codegen build-stack floor must follow named-callee floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_build_stack_floor.rs"))
        .expect("mir_build_stack_floor.rs");
    assert!(
        api.contains("MIR017")
            && api.contains("C-3q")
            && api.contains("ArrayBuild")
            && api.contains("DictBuild")
            && api.contains("SetBuild")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_build_stack_floor")
    );
}
