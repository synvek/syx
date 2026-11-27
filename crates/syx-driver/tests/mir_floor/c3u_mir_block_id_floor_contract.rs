//! C-3u: MIR MirBlock.id uniqueness floor fail-closed (MIR021).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_block_id_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3u"),
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

fn flat_fn(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
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
fn c3u_rejects_duplicate_id() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![
                MirBlock {
                    id: 3,
                    instructions: vec![ret()],
                    successors: vec![],
                },
                MirBlock {
                    id: 3,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_block_id_floor(&mir).expect_err("dup");
    assert_eq!(err.code.as_deref(), Some("MIR021"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("MirBlock.id") && text.contains("C-3u") && text.contains("duplicate"),
        "{text}"
    );
}

#[test]
fn c3u_unique_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                },
                MirBlock {
                    id: 2,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    verify_mir_block_id_floor(&mir).expect("unique");
}

#[test]
fn c3u_empty_blocks_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![])],
        ..Default::default()
    };
    verify_mir_block_id_floor(&mir).expect("empty blocks");
}

#[test]
fn c3u_empty_module_ok() {
    verify_mir_block_id_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3u_compile_from_mir_rejects_dup() {
    // entry must be a present id so C-3b MIR002 does not fire before MIR021.
    let mut func = flat_fn(
        "main",
        vec![
            MirBlock {
                id: 1,
                instructions: vec![ret()],
                successors: vec![],
            },
            MirBlock {
                id: 1,
                instructions: vec![ret()],
                successors: vec![],
            },
        ],
    );
    func.entry = 1;
    let mir = MirModule {
        functions: vec![func],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("dup via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR021"), "{err}");
}

#[test]
fn c3u_compile_from_mir_accepts_empty_blocks() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3u_hooks_after_init_pc() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let init_u = unify
        .find("verify_mir_init_pc_floor")
        .expect("init-pc unify");
    let block_u = unify
        .find("verify_mir_block_id_floor")
        .expect("block-id unify");
    assert!(
        block_u > init_u,
        "unify block-id floor must follow init-pc floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let init_c = body
        .find("verify_mir_init_pc_floor_functions")
        .expect("init-pc codegen");
    let block_c = body
        .find("verify_mir_block_id_floor_functions")
        .expect("block-id codegen");
    assert!(
        block_c > init_c,
        "codegen block-id floor must follow init-pc floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_block_id_floor.rs"))
        .expect("mir_block_id_floor.rs");
    assert!(
        api.contains("MIR021")
            && api.contains("C-3u")
            && api.contains("MirBlock.id")
            && api.contains("Not dense")
            && api.contains("verify_mir_block_id_floor")
    );
}
