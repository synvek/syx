//! MIR ConstFnIndex resolve floor fail-closed (MIR080).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    demote_unresolved_const_fn_index, verify_mir_const_fn_index_floor, MirFunction, MirInstr,
    MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("const_fn_index_floor"),
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
fn rejects_unknown_const_fn_index() {
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
    let err = verify_mir_const_fn_index_floor(&mir).expect_err("unknown");
    assert_eq!(err.code.as_deref(), Some("MIR080"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("Int8_equals")
            && text.contains("C-3cd")
            && text.contains("unknown ConstFnIndex"),
        "{text}"
    );
}

#[test]
fn known_const_fn_index_ok() {
    let mir = MirModule {
        functions: vec![
            flat_fn("helper", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::ConstFnIndex("helper".into()), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    verify_mir_const_fn_index_floor(&mir).expect("known");
}

#[test]
fn no_const_fn_index_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_const_fn_index_floor(&mir).expect("no op");
}

#[test]
fn empty_ok() {
    verify_mir_const_fn_index_floor(&MirModule::default()).expect("empty");
}

#[test]
fn demote_then_floor_ok() {
    let mut mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstFnIndex("Int8_equals".into()),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    demote_unresolved_const_fn_index(&mut mir);
    verify_mir_const_fn_index_floor(&mir).expect("demoted");
}

#[test]
fn compile_from_mir_demotes_missing_shell() {
    // Codegen demotes then floors — optional shell sentinel becomes ConstI32(-1).
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
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("demote path OK");
}

#[test]
fn compile_from_mir_accepts_known() {
    let mir = MirModule {
        functions: vec![
            flat_fn("helper", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::ConstFnIndex("helper".into()), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn hooks_after_call_closure_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev_u = unify
        .find("verify_mir_call_closure_stack_floor")
        .expect("call-closure unify");
    let cur_u = unify
        .find("verify_mir_const_fn_index_floor_functions")
        .expect("const-fn-index unify");
    assert!(
        cur_u > prev_u,
        "unify ConstFnIndex floor must follow CallClosure stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(8000);
    let body = &codegen[start..start + 1 + rel];
    let prev_c = body
        .find("verify_mir_call_closure_stack_floor_functions")
        .expect("call-closure-stack codegen");
    let cur_c = body
        .find("verify_mir_const_fn_index_floor_functions")
        .expect("const-fn-index codegen");
    assert!(
        cur_c > prev_c,
        "codegen ConstFnIndex floor must follow CallClosure stack floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_const_fn_index_floor.rs"))
            .expect("mir_const_fn_index_floor.rs");
    assert!(
        api.contains("MIR080")
            && api.contains("C-3cd")
            && api.contains("demote_unresolved_const_fn_index")
            && api.contains("verify_mir_const_fn_index_floor")
            && api.contains("ConstFnIndex")
    );
}
