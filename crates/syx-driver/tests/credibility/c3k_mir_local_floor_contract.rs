//! C-3k: MIR locals slot OOB floor fail-closed (MIR011).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_local_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3k"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, local_count: u32, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count,
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
fn c3k_rejects_oob_load() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            1,
            vec![MirOp::LoadLocal(1), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_floor(&mir).expect_err("OOB Load");
    assert_eq!(err.code.as_deref(), Some("MIR011"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("LoadLocal") && text.contains("C-3k"), "{text}");
}

#[test]
fn c3k_rejects_oob_store() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            0,
            vec![MirOp::ConstI32(0), MirOp::StoreLocal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_floor(&mir).expect_err("OOB Store");
    assert_eq!(err.code.as_deref(), Some("MIR011"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("StoreLocal") && text.contains("C-3k"), "{text}");
}

#[test]
fn c3k_in_range_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            1,
            vec![
                MirOp::ConstI32(1),
                MirOp::StoreLocal(0),
                MirOp::LoadLocal(0),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_local_floor(&mir).expect("in range");
}

#[test]
fn c3k_empty_ok() {
    verify_mir_local_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3k_compile_from_mir_rejects_oob_load() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            0,
            vec![MirOp::LoadLocal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("OOB via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR011"), "{err}");
}

#[test]
fn c3k_compile_from_mir_accepts_in_range() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 0, vec![MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3k_hooks_after_def() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let def_u = unify.find("verify_mir_def_floor").expect("def unify");
    let loc_u = unify.find("verify_mir_local_floor").expect("local unify");
    assert!(loc_u > def_u, "unify local floor must follow def floor");

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let def_c = body.find("verify_mir_def_floor_functions").expect("def codegen");
    let loc_c = body
        .find("verify_mir_local_floor_functions")
        .expect("local codegen");
    assert!(loc_c > def_c, "codegen local floor must follow def floor");

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_local_floor.rs"))
        .expect("mir_local_floor.rs");
    assert!(
        api.contains("MIR011")
            && api.contains("C-3k")
            && api.contains("Not full locals type")
            && api.contains("verify_mir_local_floor")
    );
}
