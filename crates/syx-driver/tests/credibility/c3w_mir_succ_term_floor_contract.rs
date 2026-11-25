//! C-3w: MIR successor⊇terminator jump-target floor fail-closed (MIR023).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_succ_term_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3w"),
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

fn jump(t: u32) -> MirInstr {
    MirInstr {
        dest: None,
        op: MirOp::Jump(t),
        span: None,
    }
}

fn mk(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks,
        // empty flat → MIR022 OK when blocks non-empty
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3w_rejects_jump_missing_succ() {
    let mir = MirModule {
        functions: vec![mk(
            "bad",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![jump(1)],
                    successors: vec![],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_succ_term_floor(&mir).expect_err("missing");
    assert_eq!(err.code.as_deref(), Some("MIR023"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("terminator jump target")
            && text.contains("successors")
            && text.contains("C-3w"),
        "{text}"
    );
}

#[test]
fn c3w_jump_linked_ok() {
    let mir = MirModule {
        functions: vec![mk(
            "ok",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![jump(1)],
                    successors: vec![1],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    verify_mir_succ_term_floor(&mir).expect("linked");
}

#[test]
fn c3w_return_extra_succ_ok() {
    let mir = MirModule {
        functions: vec![mk(
            "ok",
            vec![MirBlock {
                id: 0,
                instructions: vec![ret()],
                successors: vec![9],
            }],
        )],
        ..Default::default()
    };
    verify_mir_succ_term_floor(&mir).expect("extra");
}

#[test]
fn c3w_empty_blocks_ok() {
    let mir = MirModule {
        functions: vec![mk("main", vec![])],
        ..Default::default()
    };
    verify_mir_succ_term_floor(&mir).expect("empty blocks");
}

#[test]
fn c3w_empty_module_ok() {
    verify_mir_succ_term_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3w_compile_from_mir_rejects_missing_succ() {
    // Jump target exists as block id (MIR002 OK) but not in successors → MIR023.
    let mir = MirModule {
        functions: vec![mk(
            "main",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![jump(1)],
                    successors: vec![],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("missing via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR023"), "{err}");
}

#[test]
fn c3w_compile_from_mir_accepts_linked() {
    let mir = MirModule {
        functions: vec![mk(
            "main",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![jump(1)],
                    successors: vec![1],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![ret()],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3w_hooks_after_flat_blocks() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let flat_u = unify
        .find("verify_mir_flat_blocks_floor")
        .expect("flat-blocks unify");
    let succ_u = unify
        .find("verify_mir_succ_term_floor")
        .expect("succ-term unify");
    assert!(
        succ_u > flat_u,
        "unify succ-term floor must follow flat-blocks floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let flat_c = body
        .find("verify_mir_flat_blocks_floor_functions")
        .expect("flat-blocks codegen");
    let succ_c = body
        .find("verify_mir_succ_term_floor_functions")
        .expect("succ-term codegen");
    assert!(
        succ_c > flat_c,
        "codegen succ-term floor must follow flat-blocks floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_succ_term_floor.rs"))
        .expect("mir_succ_term_floor.rs");
    assert!(
        api.contains("MIR023")
            && api.contains("C-3w")
            && api.contains("Not succ==term")
            && api.contains("JumpIfFalse")
            && api.contains("verify_mir_succ_term_floor")
    );
}
