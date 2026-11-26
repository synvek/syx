//! C-3ax: MIR Ge stack-height floor fail-closed (MIR050).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_ge_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ax"),
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
fn c3ax_rejects_ge_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::Ge { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_ge_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR050"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before Ge") && text.contains("C-3ax") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3ax_rejects_ge_single_value() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::ConstI32(1), MirOp::Ge { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_ge_stack_floor(&mir).expect_err("single");
    assert_eq!(err.code.as_deref(), Some("MIR050"), "{err}");
}

#[test]
fn c3ax_ge_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::Ge { kind: 0 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_ge_stack_floor(&mir).expect("ok");
}

#[test]
fn c3ax_no_ge_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_ge_stack_floor(&mir).expect("no Ge");
}

#[test]
fn c3ax_empty_ok() {
    verify_mir_ge_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ax_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::And,
                MirOp::Ge { kind: 0 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_ge_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3ax_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::StoreLocal(0),
                MirOp::Ge { kind: 0 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_ge_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3ax_compile_from_mir_rejects_ge_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::Ge { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR050"), "{err}");
}

#[test]
fn c3ax_compile_from_mir_accepts_ge() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::Ge { kind: 0 },
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
fn c3ax_hooks_after_gt_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let gt_u = unify
        .find("verify_mir_gt_stack_floor")
        .expect("gt-stack unify");
    let ge_u = unify
        .find("verify_mir_ge_stack_floor")
        .expect("ge-stack unify");
    assert!(
        ge_u > gt_u,
        "unify ge-stack floor must follow gt-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let gt_c = body
        .find("verify_mir_gt_stack_floor_functions")
        .expect("gt-stack codegen");
    let ge_c = body
        .find("verify_mir_ge_stack_floor_functions")
        .expect("ge-stack codegen");
    assert!(
        ge_c > gt_c,
        "codegen ge-stack floor must follow gt-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_ge_stack_floor.rs"))
        .expect("mir_ge_stack_floor.rs");
    assert!(
        api.contains("MIR050")
            && api.contains("C-3ax")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_ge_stack_floor")
            && api.contains("MirOp::Ge")
            && api.contains("Some((2, 1))")
    );
}
