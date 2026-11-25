//! C-3o: MIR CallWitness / AwaitSuspend.resume_fn fn-lit OOB floor fail-closed (MIR015).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_fn_lit_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3o"),
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
fn c3o_rejects_call_witness_oob() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::CallWitness(0, 1), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_fn_lit_floor(&mir).expect_err("method OOB");
    assert_eq!(err.code.as_deref(), Some("MIR015"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("CallWitness") && text.contains("C-3o"),
        "{text}"
    );
}

#[test]
fn c3o_rejects_await_resume_oob() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![
                MirOp::AwaitSuspend {
                    resume_fn: 99,
                    state: 0,
                },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_fn_lit_floor(&mir).expect_err("resume OOB");
    assert_eq!(err.code.as_deref(), Some("MIR015"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("AwaitSuspend") && text.contains("resume_fn") && text.contains("C-3o"),
        "{text}"
    );
}

#[test]
fn c3o_call_witness_in_range_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::CallWitness(0, 0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_fn_lit_floor(&mir).expect("in range");
}

#[test]
fn c3o_await_resume_zero_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::AwaitSuspend {
                    resume_fn: 0,
                    state: 0,
                },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_fn_lit_floor(&mir).expect("resume 0");
}

#[test]
fn c3o_empty_ok() {
    verify_mir_fn_lit_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3o_compile_from_mir_rejects_await_oob() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::AwaitSuspend {
                    resume_fn: 5,
                    state: 0,
                },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("resume OOB via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR015"), "{err}");
}

#[test]
fn c3o_compile_from_mir_accepts_no_fn_lit() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3o_hooks_after_param_local() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let param_u = unify
        .find("verify_mir_param_local_floor")
        .expect("param-local unify");
    let lit_u = unify
        .find("verify_mir_fn_lit_floor")
        .expect("fn-lit unify");
    assert!(
        lit_u > param_u,
        "unify fn-lit floor must follow param-local floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let param_c = body
        .find("verify_mir_param_local_floor_functions")
        .expect("param-local codegen");
    let lit_c = body
        .find("verify_mir_fn_lit_floor_functions")
        .expect("fn-lit codegen");
    assert!(
        lit_c > param_c,
        "codegen fn-lit floor must follow param-local floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_fn_lit_floor.rs"))
        .expect("mir_fn_lit_floor.rs");
    assert!(
        api.contains("MIR015")
            && api.contains("C-3o")
            && api.contains("CallWitness")
            && api.contains("AwaitSuspend")
            && api.contains("Not ConstFnIndex")
            && api.contains("verify_mir_fn_lit_floor")
    );
}
