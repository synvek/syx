//! C-3m: MIR CallHost absolute-index floor fail-closed (MIR013).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_call_host_floor, MirFunction, MirInstr, MirModule, MirOp,
};
use syx_stdlib::{host_fn_index, HostFnId};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3m"),
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
fn c3m_rejects_unresolved_call_host() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::CallHost(u32::MAX), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_call_host_floor(&mir).expect_err("unresolved");
    assert_eq!(err.code.as_deref(), Some("MIR013"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("CallHost") && text.contains("C-3m") && text.contains("unresolved"),
        "{text}"
    );
}

#[test]
fn c3m_known_index_ok() {
    let idx = host_fn_index(HostFnId::FmtPrint);
    let mir = MirModule {
        functions: vec![flat_fn("ok", vec![MirOp::CallHost(idx), MirOp::Return])],
        ..Default::default()
    };
    verify_mir_call_host_floor(&mir).expect("known");
}

#[test]
fn c3m_no_call_host_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_call_host_floor(&mir).expect("no CallHost");
}

#[test]
fn c3m_empty_ok() {
    verify_mir_call_host_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3m_compile_from_mir_rejects_unresolved() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::CallHost(u32::MAX), MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("unresolved via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR013"), "{err}");
}

#[test]
fn c3m_compile_from_mir_accepts_no_call_host() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3m_hooks_after_global() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let glob_u = unify.find("verify_mir_global_floor").expect("global unify");
    let host_u = unify
        .find("verify_mir_call_host_floor")
        .expect("call-host unify");
    assert!(host_u > glob_u, "unify call-host floor must follow global floor");

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let glob_c = body
        .find("verify_mir_global_floor_functions")
        .expect("global codegen");
    let host_c = body
        .find("verify_mir_call_host_floor_functions")
        .expect("call-host codegen");
    assert!(
        host_c > glob_c,
        "codegen call-host floor must follow global floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_call_host_floor.rs"))
        .expect("mir_call_host_floor.rs");
    assert!(
        api.contains("MIR013")
            && api.contains("C-3m")
            && api.contains("host_fn_name_for_index")
            && api.contains("Not bytecode trim")
            && api.contains("verify_mir_call_host_floor")
    );
}
