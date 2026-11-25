//! C-3p: MIR named-callee resolve floor fail-closed (MIR016).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_named_callee_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3p"),
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
fn c3p_rejects_unknown_call() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::Call(SmolStr::new("missing")), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_named_callee_floor(&mir).expect_err("unknown");
    assert_eq!(err.code.as_deref(), Some("MIR016"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("missing") && text.contains("C-3p") && text.contains("unknown callee"),
        "{text}"
    );
}

#[test]
fn c3p_known_call_ok() {
    let mir = MirModule {
        functions: vec![
            flat_fn("helper", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::Call(SmolStr::new("helper")), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    verify_mir_named_callee_floor(&mir).expect("known");
}

#[test]
fn c3p_no_named_callee_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_named_callee_floor(&mir).expect("no callee");
}

#[test]
fn c3p_empty_ok() {
    verify_mir_named_callee_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3p_const_fn_index_not_scanned() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstFnIndex("Int8_equals".into()),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_named_callee_floor(&mir).expect("ConstFnIndex R-33 path OK");
}

#[test]
fn c3p_compile_from_mir_rejects_unknown_call() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::Call(SmolStr::new("nope")), MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("unknown via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR016"), "{err}");
}

#[test]
fn c3p_compile_from_mir_accepts_known_call() {
    let mir = MirModule {
        functions: vec![
            flat_fn("helper", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::Call(SmolStr::new("helper")), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3p_hooks_after_fn_lit() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let lit_u = unify
        .find("verify_mir_fn_lit_floor")
        .expect("fn-lit unify");
    let callee_u = unify
        .find("verify_mir_named_callee_floor_functions")
        .expect("named-callee unify");
    assert!(
        callee_u > lit_u,
        "unify named-callee floor must follow fn-lit floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let lit_c = body
        .find("verify_mir_fn_lit_floor_functions")
        .expect("fn-lit codegen");
    let callee_c = body
        .find("verify_mir_named_callee_floor_functions")
        .expect("named-callee codegen");
    assert!(
        callee_c > lit_c,
        "codegen named-callee floor must follow fn-lit floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_named_callee_floor.rs"))
        .expect("mir_named_callee_floor.rs");
    assert!(
        api.contains("MIR016")
            && api.contains("C-3p")
            && api.contains("ActorSend")
            && api.contains("ConstFnIndex")
            && api.contains("Not ConstFnIndex")
            && api.contains("verify_mir_named_callee_floor")
    );
}
