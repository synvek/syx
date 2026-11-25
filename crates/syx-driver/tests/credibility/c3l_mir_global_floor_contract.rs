//! C-3l: MIR globals slot OOB floor fail-closed (MIR012).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_global_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3l"),
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
fn c3l_rejects_oob_load() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::LoadGlobal(0), MirOp::Return],
        )],
        global_count: 0,
        ..Default::default()
    };
    let err = verify_mir_global_floor(&mir).expect_err("OOB Load");
    assert_eq!(err.code.as_deref(), Some("MIR012"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("LoadGlobal") && text.contains("C-3l"), "{text}");
}

#[test]
fn c3l_rejects_oob_store() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::ConstI32(0), MirOp::StoreGlobal(1), MirOp::Return],
        )],
        global_count: 1,
        ..Default::default()
    };
    let err = verify_mir_global_floor(&mir).expect_err("OOB Store");
    assert_eq!(err.code.as_deref(), Some("MIR012"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("StoreGlobal") && text.contains("C-3l"), "{text}");
}

#[test]
fn c3l_in_range_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::StoreGlobal(0),
                MirOp::LoadGlobal(0),
                MirOp::Return,
            ],
        )],
        global_count: 1,
        ..Default::default()
    };
    verify_mir_global_floor(&mir).expect("in range");
}

#[test]
fn c3l_empty_ok() {
    verify_mir_global_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3l_compile_from_mir_rejects_oob_load() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::LoadGlobal(0), MirOp::Return],
        )],
        global_count: 0,
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("OOB via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR012"), "{err}");
}

#[test]
fn c3l_compile_from_mir_accepts_in_range() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        global_count: 0,
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3l_hooks_after_local_and_schema() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let loc_u = unify.find("verify_mir_local_floor").expect("local unify");
    let glob_u = unify.find("verify_mir_global_floor").expect("global unify");
    assert!(glob_u > loc_u, "unify global floor must follow local floor");
    assert!(
        unify.contains("absorb_fragment_global_count")
            && unify.contains("absorb_mir_global_count"),
        "unify must populate/absorb global_count"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let loc_c = body
        .find("verify_mir_local_floor_functions")
        .expect("local codegen");
    let glob_c = body
        .find("verify_mir_global_floor_functions")
        .expect("global codegen");
    assert!(glob_c > loc_c, "codegen global floor must follow local floor");

    let types = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_types.rs"))
        .expect("mir_types");
    assert!(
        types.contains("global_count") && types.contains("C-3l"),
        "MirModule must carry global_count"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_global_floor.rs"))
        .expect("mir_global_floor.rs");
    assert!(
        api.contains("MIR012")
            && api.contains("C-3l")
            && api.contains("Not global name table")
            && api.contains("verify_mir_global_floor")
    );

    let lower = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_lower_module.rs"))
        .expect("lower");
    assert!(
        lower.contains("global_count") && lower.contains("C-3l"),
        "lower_module must populate global_count"
    );
}
