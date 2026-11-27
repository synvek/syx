//! MIR BindResult mid-block successor floor fail-closed (MIR083).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_bind_result_succ_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("bind_result_succ_floor"),
        decls: vec![],
    }
}

fn instr(op: MirOp) -> MirInstr {
    MirInstr {
        dest: None,
        op,
        span: None,
    }
}

fn blocked(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks,
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

fn flat(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops.into_iter().map(instr).collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn rejects_bind_result_not_in_successors() {
    // Target block 2 exists (C-3b OK) but is absent from successors → MIR083.
    let mir = MirModule {
        functions: vec![blocked(
            "bad",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::BindResult(2)), instr(MirOp::Jump(1))],
                    successors: vec![1],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
                MirBlock {
                    id: 2,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_bind_result_succ_floor(&mir).expect_err("missing succ");
    assert_eq!(err.code.as_deref(), Some("MIR083"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("BindResult(2)") && text.contains("C-3ch") && text.contains("successors"),
        "{text}"
    );
}

#[test]
fn bind_result_in_successors_ok() {
    let mir = MirModule {
        functions: vec![blocked(
            "ok",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::BindResult(1)), instr(MirOp::Jump(2))],
                    successors: vec![1, 2],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
                MirBlock {
                    id: 2,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    verify_mir_bind_result_succ_floor(&mir).expect("ok");
}

#[test]
fn no_bind_result_ok() {
    let mir = MirModule {
        functions: vec![blocked(
            "ok",
            vec![MirBlock {
                id: 0,
                instructions: vec![instr(MirOp::Return)],
                successors: vec![],
            }],
        )],
        ..Default::default()
    };
    verify_mir_bind_result_succ_floor(&mir).expect("no BindResult");
}

#[test]
fn empty_ok() {
    verify_mir_bind_result_succ_floor(&MirModule::default()).expect("empty");
}

#[test]
fn flat_instructions_abandon_ok() {
    let mir = MirModule {
        functions: vec![flat(
            "main",
            vec![MirOp::BindResult(9), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_bind_result_succ_floor(&mir).expect("flat abandon");
}

#[test]
fn compile_from_mir_rejects_missing_succ() {
    let mir = MirModule {
        functions: vec![blocked(
            "main",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::BindResult(2)), instr(MirOp::Jump(1))],
                    successors: vec![1],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
                MirBlock {
                    id: 2,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("MIR083 via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR083"), "{err}");
}

#[test]
fn compile_from_mir_accepts_linked() {
    let mir = MirModule {
        functions: vec![blocked(
            "main",
            vec![
                MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::BindResult(1)), instr(MirOp::Jump(2))],
                    successors: vec![1, 2],
                },
                MirBlock {
                    id: 1,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                },
                MirBlock {
                    id: 2,
                    instructions: vec![instr(MirOp::Return)],
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
fn hooks_after_def_fn_consistency() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev_u = unify
        .find("verify_mir_def_fn_consistency_floor_functions")
        .expect("def-fn unify");
    let cur_u = unify
        .find("verify_mir_bind_result_succ_floor")
        .expect("bind-result-succ unify");
    assert!(
        cur_u > prev_u,
        "unify BindResult succ floor must follow Def↔Fn floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(8000);
    let body = &codegen[start..start + 1 + rel];
    let prev_c = body
        .find("verify_mir_def_fn_consistency_floor_functions")
        .expect("def-fn codegen");
    let cur_c = body
        .find("verify_mir_bind_result_succ_floor_functions")
        .expect("bind-result-succ codegen");
    assert!(
        cur_c > prev_c,
        "codegen BindResult succ floor must follow Def↔Fn floor"
    );

    let api = std::fs::read_to_string(
        root.join("crates/syx-mir/src/mir_bind_result_succ_floor.rs"),
    )
    .expect("mir_bind_result_succ_floor.rs");
    assert!(
        api.contains("MIR083")
            && api.contains("C-3ch")
            && api.contains("BindResult")
            && api.contains("verify_mir_bind_result_succ_floor")
            && api.contains("successors")
    );
}
