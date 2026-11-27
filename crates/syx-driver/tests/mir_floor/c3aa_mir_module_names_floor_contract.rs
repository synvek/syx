//! C-3aa: MIR module_names empty-string floor fail-closed (MIR027).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_module_names_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3aa"),
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
fn c3aa_rejects_empty_entry() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        module_names: vec![SmolStr::new("a"), SmolStr::new("")],
        ..Default::default()
    };
    let err = verify_mir_module_names_floor(&mir).expect_err("empty entry");
    assert_eq!(err.code.as_deref(), Some("MIR027"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("empty") && text.contains("C-3aa") && text.contains("module_names"),
        "{text}"
    );
}

#[test]
fn c3aa_non_empty_names_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        module_names: vec![SmolStr::new("a"), SmolStr::new("b")],
        ..Default::default()
    };
    verify_mir_module_names_floor(&mir).expect("names");
}

#[test]
fn c3aa_empty_table_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        module_names: vec![],
        ..Default::default()
    };
    verify_mir_module_names_floor(&mir).expect("empty table");
}

#[test]
fn c3aa_empty_module_ok() {
    verify_mir_module_names_floor(&MirModule::default()).expect("empty module");
}

#[test]
fn c3aa_compile_from_mir_rejects_empty_entry() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        module_names: vec![SmolStr::new("")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("empty entry via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR027"), "{err}");
}

#[test]
fn c3aa_compile_from_mir_accepts_names() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        module_names: vec![SmolStr::new("main")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3aa_hooks_after_block_instr() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let instr_u = unify
        .find("verify_mir_block_instr_floor")
        .expect("block-instr unify");
    let names_u = unify
        .find("verify_mir_module_names_floor")
        .expect("module-names unify");
    assert!(
        names_u > instr_u,
        "unify module-names floor must follow block-instr floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let instr_c = body
        .find("verify_mir_block_instr_floor_functions")
        .expect("block-instr codegen");
    let names_c = body
        .find("verify_mir_module_names_floor")
        .expect("module-names codegen");
    assert!(
        names_c > instr_c,
        "codegen module-names floor must follow block-instr floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_module_names_floor.rs"))
        .expect("mir_module_names_floor.rs");
    assert!(
        api.contains("MIR027")
            && api.contains("C-3aa")
            && api.contains("module_names")
            && api.contains("verify_mir_module_names_floor")
    );
}
