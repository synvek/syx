//! C-3al: MIR MakeClosure stack-height floor fail-closed (MIR038).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_make_closure_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3al"),
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
fn c3al_rejects_make_closure_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::MakeClosure(SmolStr::new("f"), 1), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_make_closure_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR038"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before MakeClosure")
            && text.contains("C-3al")
            && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3al_make_closure_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::MakeClosure(SmolStr::new("f"), 1),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_make_closure_stack_floor(&mir).expect("ok");
}

#[test]
fn c3al_make_closure_cap0_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::MakeClosure(SmolStr::new("f"), 0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_make_closure_stack_floor(&mir).expect("cap=0");
}

#[test]
fn c3al_no_make_closure_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_make_closure_stack_floor(&mir).expect("no MakeClosure");
}

#[test]
fn c3al_empty_ok() {
    verify_mir_make_closure_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3al_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::BitNot { kind: 0 },
                MirOp::MakeClosure(SmolStr::new("f"), 1),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_make_closure_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3al_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::StoreLocal(0),
                MirOp::MakeClosure(SmolStr::new("f"), 1),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_make_closure_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3al_compile_from_mir_rejects_make_closure_underflow() {
    // Stub callee so MIR016 (named-callee) passes; this nail only asserts MIR038 stack.
    let mir = MirModule {
        functions: vec![
            flat_fn("f", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::MakeClosure(SmolStr::new("f"), 1), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR038"), "{err}");
}

#[test]
fn c3al_compile_from_mir_accepts_make_closure() {
    let mir = MirModule {
        functions: vec![
            flat_fn("f", vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::MakeClosure(SmolStr::new("f"), 1),
                    MirOp::Return,
                ],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3al_hooks_after_numeric_cast_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let ncast_u = unify
        .find("verify_mir_numeric_cast_stack_floor")
        .expect("numeric-cast-stack unify");
    let mclos_u = unify
        .find("verify_mir_make_closure_stack_floor")
        .expect("make-closure-stack unify");
    assert!(
        mclos_u > ncast_u,
        "unify make-closure-stack floor must follow numeric-cast-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let ncast_c = body
        .find("verify_mir_numeric_cast_stack_floor_functions")
        .expect("numeric-cast-stack codegen");
    let mclos_c = body
        .find("verify_mir_make_closure_stack_floor_functions")
        .expect("make-closure-stack codegen");
    assert!(
        mclos_c > ncast_c,
        "codegen make-closure-stack floor must follow numeric-cast-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_make_closure_stack_floor.rs"))
        .expect("mir_make_closure_stack_floor.rs");
    assert!(
        api.contains("MIR038")
            && api.contains("C-3al")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_make_closure_stack_floor")
            && api.contains("MirOp::MakeClosure")
    );
}
