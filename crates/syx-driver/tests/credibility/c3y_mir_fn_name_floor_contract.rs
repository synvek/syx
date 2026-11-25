//! C-3y: MIR function-name floor fail-closed (MIR025).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_fn_name_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3y"),
        decls: vec![],
    }
}

fn flat_fn(name: &str) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
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
fn c3y_rejects_empty_name() {
    let mir = MirModule {
        functions: vec![flat_fn("")],
        ..Default::default()
    };
    let err = verify_mir_fn_name_floor(&mir).expect_err("empty name");
    assert_eq!(err.code.as_deref(), Some("MIR025"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("empty") && text.contains("C-3y") && text.contains("name"),
        "{text}"
    );
}

#[test]
fn c3y_non_empty_name_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("g")],
        ..Default::default()
    };
    verify_mir_fn_name_floor(&mir).expect("names");
}

#[test]
fn c3y_empty_module_ok() {
    verify_mir_fn_name_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3y_compile_from_mir_rejects_empty_name() {
    let mir = MirModule {
        functions: vec![flat_fn("")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("empty name via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR025"), "{err}");
}

#[test]
fn c3y_compile_from_mir_accepts_name() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3y_hooks_after_pop_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let pop_u = unify
        .find("verify_mir_pop_stack_floor")
        .expect("pop-stack unify");
    let name_u = unify
        .find("verify_mir_fn_name_floor")
        .expect("fn-name unify");
    assert!(
        name_u > pop_u,
        "unify fn-name floor must follow pop-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let pop_c = body
        .find("verify_mir_pop_stack_floor_functions")
        .expect("pop-stack codegen");
    let name_c = body
        .find("verify_mir_fn_name_floor_functions")
        .expect("fn-name codegen");
    assert!(
        name_c > pop_c,
        "codegen fn-name floor must follow pop-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_fn_name_floor.rs"))
        .expect("mir_fn_name_floor.rs");
    assert!(
        api.contains("MIR025")
            && api.contains("C-3y")
            && api.contains("name")
            && api.contains("verify_mir_fn_name_floor")
    );
}
