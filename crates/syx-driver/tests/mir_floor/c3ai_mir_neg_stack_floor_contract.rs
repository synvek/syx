//! C-3ai: MIR Neg stack-height floor fail-closed (MIR035).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_neg_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ai"),
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
fn c3ai_rejects_neg_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_neg_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR035"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before Neg") && text.contains("C-3ai") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3ai_neg_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_neg_stack_floor(&mir).expect("ok");
}

#[test]
fn c3ai_no_neg_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_neg_stack_floor(&mir).expect("no Neg");
}

#[test]
fn c3ai_empty_ok() {
    verify_mir_neg_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ai_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::Not, MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_neg_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3ai_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_neg_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3ai_compile_from_mir_rejects_neg_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR035"), "{err}");
}

#[test]
fn c3ai_compile_from_mir_accepts_neg() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::Neg { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3ai_hooks_after_not_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let not_u = unify
        .find("verify_mir_not_stack_floor")
        .expect("not-stack unify");
    let neg_u = unify
        .find("verify_mir_neg_stack_floor")
        .expect("neg-stack unify");
    assert!(
        neg_u > not_u,
        "unify neg-stack floor must follow not-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let not_c = body
        .find("verify_mir_not_stack_floor_functions")
        .expect("not-stack codegen");
    let neg_c = body
        .find("verify_mir_neg_stack_floor_functions")
        .expect("neg-stack codegen");
    assert!(
        neg_c > not_c,
        "codegen neg-stack floor must follow not-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_neg_stack_floor.rs"))
        .expect("mir_neg_stack_floor.rs");
    assert!(
        api.contains("MIR035")
            && api.contains("C-3ai")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_neg_stack_floor")
            && api.contains("MirOp::Neg")
    );
}
