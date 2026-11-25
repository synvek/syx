//! C-3n: MIR param_count ≤ local_count metadata floor fail-closed (MIR014).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_param_local_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3n"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, param_count: u32, local_count: u32) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count,
        local_count,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3n_rejects_param_gt_local() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", 2, 1)],
        ..Default::default()
    };
    let err = verify_mir_param_local_floor(&mir).expect_err("param > local");
    assert_eq!(err.code.as_deref(), Some("MIR014"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("param_count") && text.contains("local_count") && text.contains("C-3n"),
        "{text}"
    );
}

#[test]
fn c3n_param_le_local_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", 2, 2), flat_fn("ok2", 1, 3)],
        ..Default::default()
    };
    verify_mir_param_local_floor(&mir).expect("param <= local");
}

#[test]
fn c3n_zero_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", 0, 0)],
        ..Default::default()
    };
    verify_mir_param_local_floor(&mir).expect("zero");
}

#[test]
fn c3n_empty_ok() {
    verify_mir_param_local_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3n_compile_from_mir_rejects_param_gt_local() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 3, 1)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("param > local via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR014"), "{err}");
}

#[test]
fn c3n_compile_from_mir_accepts_param_le_local() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, 2)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3n_hooks_after_call_host() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let host_u = unify
        .find("verify_mir_call_host_floor")
        .expect("call-host unify");
    let param_u = unify
        .find("verify_mir_param_local_floor")
        .expect("param-local unify");
    assert!(
        param_u > host_u,
        "unify param-local floor must follow call-host floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let host_c = body
        .find("verify_mir_call_host_floor_functions")
        .expect("call-host codegen");
    let param_c = body
        .find("verify_mir_param_local_floor_functions")
        .expect("param-local codegen");
    assert!(
        param_c > host_c,
        "codegen param-local floor must follow call-host floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_param_local_floor.rs"))
        .expect("mir_param_local_floor.rs");
    assert!(
        api.contains("MIR014")
            && api.contains("C-3n")
            && api.contains("param_count")
            && api.contains("local_count")
            && api.contains("verify_mir_param_local_floor")
    );
}
