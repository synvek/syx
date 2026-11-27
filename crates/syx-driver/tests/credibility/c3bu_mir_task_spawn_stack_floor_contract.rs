//! C-3bu: MIR TaskSpawn stack-height floor fail-closed (MIR073).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_task_spawn_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3bu"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

fn flat_fn_params(name: &str, param_count: u32, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count,
        local_count: param_count,
        entry: 0,
        blocks: vec![],
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3bu_rejects_task_spawn_underflow() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "bad",
                vec![MirOp::TaskSpawn(SmolStr::new("f")), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    let err = verify_mir_task_spawn_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR073"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before TaskSpawn") && text.contains("C-3bu") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3bu_task_spawn_ok() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "ok",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::TaskSpawn(SmolStr::new("f")),
                    MirOp::Return,
                ],
            ),
        ],
        ..Default::default()
    };
    verify_mir_task_spawn_stack_floor(&mir).expect("ok");
}

#[test]
fn c3bu_no_task_spawn_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_task_spawn_stack_floor(&mir).expect("no TaskSpawn");
}

#[test]
fn c3bu_empty_ok() {
    verify_mir_task_spawn_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3bu_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "ok",
                vec![
                    MirOp::JumpIfNil(0),
                    MirOp::TaskSpawn(SmolStr::new("f")),
                    MirOp::Return,
                ],
            ),
        ],
        ..Default::default()
    };
    verify_mir_task_spawn_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3bu_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "ok",
                vec![
                    MirOp::StoreLocal(0),
                    MirOp::TaskSpawn(SmolStr::new("f")),
                    MirOp::Return,
                ],
            ),
        ],
        ..Default::default()
    };
    verify_mir_task_spawn_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3bu_compile_from_mir_rejects_task_spawn_underflow() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![MirOp::TaskSpawn(SmolStr::new("f")), MirOp::Return],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR073"), "{err}");
}

#[test]
fn c3bu_compile_from_mir_accepts_task_spawn() {
    let mir = MirModule {
        functions: vec![
            flat_fn_params("f", 1, vec![MirOp::Return]),
            flat_fn(
                "main",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::TaskSpawn(SmolStr::new("f")),
                    MirOp::Return,
                ],
            ),
        ],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3bu_hooks_after_actor_send_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev = unify
        .find("verify_mir_actor_send_stack_floor")
        .expect("actor-send-stack unify");
    let cur = unify
        .find("verify_mir_task_spawn_stack_floor")
        .expect("task-spawn-stack unify");
    assert!(
        cur > prev,
        "unify task-spawn-stack floor must follow actor-send-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let prev_c = body
        .find("verify_mir_actor_send_stack_floor_functions")
        .expect("actor-send-stack codegen");
    let cur_c = body
        .find("verify_mir_task_spawn_stack_floor_functions")
        .expect("task-spawn-stack codegen");
    assert!(
        cur_c > prev_c,
        "codegen task-spawn-stack floor must follow actor-send-stack floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_task_spawn_stack_floor.rs"))
            .expect("mir_task_spawn_stack_floor.rs");
    assert!(
        api.contains("MIR073")
            && api.contains("C-3bu")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_task_spawn_stack_floor")
            && api.contains("MirOp::TaskSpawn")
            && api.contains("callees.get(name)")
    );
}
