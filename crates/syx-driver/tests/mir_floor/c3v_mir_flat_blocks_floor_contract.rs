//! C-3v: MIR flat↔blocks instruction-count floor fail-closed (MIR022).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_flat_blocks_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3v"),
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

fn mk(name: &str, blocks: Vec<MirBlock>, instructions: Vec<MirInstr>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks,
        instructions,
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3v_rejects_count_mismatch() {
    let mir = MirModule {
        functions: vec![mk(
            "bad",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
            vec![ret()],
        )],
        ..Default::default()
    };
    let err = verify_mir_flat_blocks_floor(&mir).expect_err("mismatch");
    assert_eq!(err.code.as_deref(), Some("MIR022"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("sum(block.instructions)")
            && text.contains("instructions.len()")
            && text.contains("C-3v"),
        "{text}"
    );
}

#[test]
fn c3v_equal_len_ok() {
    let mir = MirModule {
        functions: vec![mk(
            "ok",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
            vec![ret(), ret()],
        )],
        ..Default::default()
    };
    verify_mir_flat_blocks_floor(&mir).expect("equal");
}

#[test]
fn c3v_empty_blocks_ok() {
    let mir = MirModule {
        functions: vec![mk("main", vec![], vec![ret()])],
        ..Default::default()
    };
    verify_mir_flat_blocks_floor(&mir).expect("empty blocks");
}

#[test]
fn c3v_empty_flat_ok() {
    let mir = MirModule {
        functions: vec![mk(
            "main",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![],
            }],
            vec![],
        )],
        ..Default::default()
    };
    verify_mir_flat_blocks_floor(&mir).expect("empty flat");
}

#[test]
fn c3v_empty_module_ok() {
    verify_mir_flat_blocks_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3v_compile_from_mir_rejects_mismatch() {
    let mir = MirModule {
        functions: vec![mk(
            "main",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
            vec![ret()],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("mismatch via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR022"), "{err}");
}

#[test]
fn c3v_compile_from_mir_accepts_empty_blocks() {
    let mir = MirModule {
        functions: vec![mk("main", vec![], vec![ret()])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3v_hooks_after_block_id() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let block_u = unify
        .find("verify_mir_block_id_floor")
        .expect("block-id unify");
    let flat_u = unify
        .find("verify_mir_flat_blocks_floor")
        .expect("flat-blocks unify");
    assert!(
        flat_u > block_u,
        "unify flat-blocks floor must follow block-id floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let block_c = body
        .find("verify_mir_block_id_floor_functions")
        .expect("block-id codegen");
    let flat_c = body
        .find("verify_mir_flat_blocks_floor_functions")
        .expect("flat-blocks codegen");
    assert!(
        flat_c > block_c,
        "codegen flat-blocks floor must follow block-id floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_flat_blocks_floor.rs"))
        .expect("mir_flat_blocks_floor.rs");
    assert!(
        api.contains("MIR022")
            && api.contains("C-3v")
            && api.contains("sum(block.instructions.len())")
            && api.contains("Not instruction/PC semantic equality")
            && api.contains("verify_mir_flat_blocks_floor")
    );
}
