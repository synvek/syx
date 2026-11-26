//! C-3ae: MIR local_names empty-string-slot floor fail-closed (MIR031).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_local_names_empty_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ae"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, local_count: u32, local_names: Vec<SmolStr>) -> MirFunction {
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
        local_names,
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3ae_rejects_empty_slot() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            2,
            vec![SmolStr::new("local_0"), SmolStr::new("")],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_names_empty_floor(&mir).expect_err("empty slot");
    assert_eq!(err.code.as_deref(), Some("MIR031"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("empty") && text.contains("C-3ae") && text.contains("local_names"),
        "{text}"
    );
}

#[test]
fn c3ae_local_i_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            2,
            vec![SmolStr::new("local_0"), SmolStr::new("x")],
        )],
        ..Default::default()
    };
    verify_mir_local_names_empty_floor(&mir).expect("sentinels");
}

#[test]
fn c3ae_empty_vector_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", 2, vec![])],
        ..Default::default()
    };
    verify_mir_local_names_empty_floor(&mir).expect("stripped");
}

#[test]
fn c3ae_empty_module_ok() {
    verify_mir_local_names_empty_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ae_compile_from_mir_rejects_empty_slot() {
    // len == local_count so MIR018 passes; empty slot → MIR031.
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, vec![SmolStr::new("")])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("empty slot via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR031"), "{err}");
}

#[test]
fn c3ae_compile_from_mir_accepts_local_i() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, vec![SmolStr::new("local_0")])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3ae_hooks_after_local_types_empty() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let types_u = unify
        .find("verify_mir_local_types_empty_floor")
        .expect("local-types-empty unify");
    let names_u = unify
        .find("verify_mir_local_names_empty_floor")
        .expect("local-names-empty unify");
    assert!(
        names_u > types_u,
        "unify local-names-empty floor must follow local-types-empty floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let types_c = body
        .find("verify_mir_local_types_empty_floor_functions")
        .expect("local-types-empty codegen");
    let names_c = body
        .find("verify_mir_local_names_empty_floor_functions")
        .expect("local-names-empty codegen");
    assert!(
        names_c > types_c,
        "codegen local-names-empty floor must follow local-types-empty floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_local_names_empty_floor.rs"))
            .expect("mir_local_names_empty_floor.rs");
    assert!(
        api.contains("MIR031")
            && api.contains("C-3ae")
            && api.contains("local_names")
            && api.contains("verify_mir_local_names_empty_floor")
    );
}
