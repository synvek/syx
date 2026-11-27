//! C-3z: MIR non-empty block-instruction floor fail-closed (MIR026).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_block_instr_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3z"),
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
fn c3z_rejects_empty_instr_block() {
    let mir = MirModule {
        functions: vec![fn_with_blocks(
            "bad",
            vec![MirBlock {
                id: 0,
                instructions: vec![],
                successors: vec![],
            }],
        )],
        ..Default::default()
    };
    let err = verify_mir_block_instr_floor(&mir).expect_err("empty instr");
    assert_eq!(err.code.as_deref(), Some("MIR026"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("empty") && text.contains("C-3z") && text.contains("instructions"),
        "{text}"
    );
}

#[test]
fn c3z_non_empty_instr_ok() {
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
    verify_mir_block_instr_floor(&mir).expect("instr");
}

#[test]
fn c3z_empty_blocks_ok() {
    let mir = MirModule {
        functions: vec![fn_with_blocks("main", vec![])],
        ..Default::default()
    };
    verify_mir_block_instr_floor(&mir).expect("empty blocks");
}

#[test]
fn c3z_empty_module_ok() {
    verify_mir_block_instr_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3z_compile_from_mir_rejects_empty_instr_block() {
    // Keep flat↔blocks counts equal (MIR022) so MIR026 is the fail-closed hit.
    let mir = MirModule {
        functions: vec![MirFunction {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![
                MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![1],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![],
                    successors: vec![],
                },
            ],
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
        .expect_err("empty instr via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR026"), "{err}");
}

#[test]
fn c3z_compile_from_mir_accepts_instr() {
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
fn c3z_hooks_after_fn_name() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let name_u = unify
        .find("verify_mir_fn_name_floor")
        .expect("fn-name unify");
    let instr_u = unify
        .find("verify_mir_block_instr_floor")
        .expect("block-instr unify");
    assert!(
        instr_u > name_u,
        "unify block-instr floor must follow fn-name floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let name_c = body
        .find("verify_mir_fn_name_floor_functions")
        .expect("fn-name codegen");
    let instr_c = body
        .find("verify_mir_block_instr_floor_functions")
        .expect("block-instr codegen");
    assert!(
        instr_c > name_c,
        "codegen block-instr floor must follow fn-name floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_block_instr_floor.rs"))
        .expect("mir_block_instr_floor.rs");
    assert!(
        api.contains("MIR026")
            && api.contains("C-3z")
            && api.contains("instructions")
            && api.contains("verify_mir_block_instr_floor")
    );
}
