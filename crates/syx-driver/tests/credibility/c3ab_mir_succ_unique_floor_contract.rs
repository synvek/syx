//! C-3ab: MIR MirBlock.successors uniqueness floor fail-closed (MIR028).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_succ_unique_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ab"),
        decls: vec![],
    }
}

fn ret() -> MirInstr {
    MirInstr {
        dest: None,
        op: MirOp::Return,
        span: None,
    }
}

fn fn_with_blocks(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks,
        instructions: vec![ret()],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3ab_rejects_duplicate_successors() {
    let mir = MirModule {
        functions: vec![fn_with_blocks(
            "bad",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![1, 1],
            }],
        )],
        ..Default::default()
    };
    let err = verify_mir_succ_unique_floor(&mir).expect_err("dup");
    assert_eq!(err.code.as_deref(), Some("MIR028"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("duplicate") && text.contains("C-3ab") && text.contains("successors"),
        "{text}"
    );
}

#[test]
fn c3ab_unique_ok() {
    let mir = MirModule {
        functions: vec![fn_with_blocks(
            "ok",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![1, 2],
            }],
        )],
        ..Default::default()
    };
    verify_mir_succ_unique_floor(&mir).expect("unique");
}

#[test]
fn c3ab_empty_successors_ok() {
    let mir = MirModule {
        functions: vec![fn_with_blocks(
            "ok",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![],
            }],
        )],
        ..Default::default()
    };
    verify_mir_succ_unique_floor(&mir).expect("empty succ");
}

#[test]
fn c3ab_empty_blocks_ok() {
    let mir = MirModule {
        functions: vec![fn_with_blocks("main", vec![])],
        ..Default::default()
    };
    verify_mir_succ_unique_floor(&mir).expect("empty blocks");
}

#[test]
fn c3ab_empty_module_ok() {
    verify_mir_succ_unique_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ab_compile_from_mir_rejects_dup() {
    // Return + extra edges OK for MIR023; dup edges → MIR028.
    // flat↔blocks counts match (MIR022).
    let mir = MirModule {
        functions: vec![MirFunction {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![0, 0],
            }],
            instructions: vec![ret()],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("dup via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR028"), "{err}");
}

#[test]
fn c3ab_compile_from_mir_accepts_unique() {
    let mir = MirModule {
        functions: vec![fn_with_blocks(
            "main",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![],
            }],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3ab_hooks_after_module_names() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let names_u = unify
        .find("verify_mir_module_names_floor")
        .expect("module-names unify");
    let uniq_u = unify
        .find("verify_mir_succ_unique_floor")
        .expect("succ-unique unify");
    assert!(
        uniq_u > names_u,
        "unify succ-unique floor must follow module-names floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let names_c = body
        .find("verify_mir_module_names_floor")
        .expect("module-names codegen");
    let uniq_c = body
        .find("verify_mir_succ_unique_floor_functions")
        .expect("succ-unique codegen");
    assert!(
        uniq_c > names_c,
        "codegen succ-unique floor must follow module-names floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_succ_unique_floor.rs"))
        .expect("mir_succ_unique_floor.rs");
    assert!(
        api.contains("MIR028")
            && api.contains("C-3ab")
            && api.contains("successors")
            && api.contains("verify_mir_succ_unique_floor")
    );
}
