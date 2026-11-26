//! C-3ad: MIR local_types empty-string-slot floor fail-closed (MIR030).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_local_types_empty_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ad"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, local_count: u32, local_types: Vec<SmolStr>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names: vec![],
        local_types,
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3ad_rejects_empty_slot() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            2,
            vec![SmolStr::new("Unknown"), SmolStr::new("")],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_types_empty_floor(&mir).expect_err("empty slot");
    assert_eq!(err.code.as_deref(), Some("MIR030"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("empty") && text.contains("C-3ad") && text.contains("local_types"),
        "{text}"
    );
}

#[test]
fn c3ad_unknown_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            2,
            vec![SmolStr::new("Unknown"), SmolStr::new("Int")],
        )],
        ..Default::default()
    };
    verify_mir_local_types_empty_floor(&mir).expect("sentinels");
}

#[test]
fn c3ad_empty_vector_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", 2, vec![])],
        ..Default::default()
    };
    verify_mir_local_types_empty_floor(&mir).expect("stripped");
}

#[test]
fn c3ad_empty_module_ok() {
    verify_mir_local_types_empty_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ad_compile_from_mir_rejects_empty_slot() {
    // len == local_count so MIR018 passes; empty slot → MIR030.
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            1,
            vec![SmolStr::new("")],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("empty slot via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR030"), "{err}");
}

#[test]
fn c3ad_compile_from_mir_accepts_unknown() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, vec![SmolStr::new("Unknown")])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3ad_hooks_after_aot_export_unique() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let export_u = unify
        .find("verify_mir_aot_export_unique_floor")
        .expect("aot-export-unique unify");
    let types_u = unify
        .find("verify_mir_local_types_empty_floor")
        .expect("local-types-empty unify");
    assert!(
        types_u > export_u,
        "unify local-types-empty floor must follow aot-export-unique floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let export_c = body
        .find("verify_mir_aot_export_unique_floor")
        .expect("aot-export-unique codegen");
    let types_c = body
        .find("verify_mir_local_types_empty_floor_functions")
        .expect("local-types-empty codegen");
    assert!(
        types_c > export_c,
        "codegen local-types-empty floor must follow aot-export-unique floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_local_types_empty_floor.rs"))
            .expect("mir_local_types_empty_floor.rs");
    assert!(
        api.contains("MIR030")
            && api.contains("C-3ad")
            && api.contains("local_types")
            && api.contains("verify_mir_local_types_empty_floor")
    );
}
