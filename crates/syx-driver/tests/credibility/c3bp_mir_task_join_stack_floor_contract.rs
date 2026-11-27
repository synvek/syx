//! C-3bp: MIR TaskJoin stack-height floor fail-closed (MIR068).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_task_join_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3bp"),
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

fn await_suspend() -> MirOp {
    MirOp::AwaitSuspend {
        resume_fn: 0,
        state: 0,
    }
}

#[test]
fn c3bp_rejects_task_join_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::TaskJoin, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_task_join_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR068"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before TaskJoin") && text.contains("C-3bp") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3bp_task_join_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::TaskJoin, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_task_join_stack_floor(&mir).expect("ok");
}

#[test]
fn c3bp_no_task_join_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_task_join_stack_floor(&mir).expect("no TaskJoin");
}

#[test]
fn c3bp_empty_ok() {
    verify_mir_task_join_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3bp_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![await_suspend(), MirOp::TaskJoin, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_task_join_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3bp_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::TaskJoin, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_task_join_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3bp_compile_from_mir_rejects_task_join_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::TaskJoin, MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR068"), "{err}");
}

#[test]
fn c3bp_compile_from_mir_accepts_task_join() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::TaskJoin, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3bp_hooks_after_make_iterator_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev = unify
        .find("verify_mir_make_iterator_stack_floor")
        .expect("make-iterator-stack unify");
    let cur = unify
        .find("verify_mir_task_join_stack_floor")
        .expect("task-join-stack unify");
    assert!(
        cur > prev,
        "unify task-join-stack floor must follow make-iterator-stack floor"
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
        .find("verify_mir_make_iterator_stack_floor_functions")
        .expect("make-iterator-stack codegen");
    let cur_c = body
        .find("verify_mir_task_join_stack_floor_functions")
        .expect("task-join-stack codegen");
    assert!(
        cur_c > prev_c,
        "codegen task-join-stack floor must follow make-iterator-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_task_join_stack_floor.rs"))
        .expect("mir_task_join_stack_floor.rs");
    assert!(
        api.contains("MIR068")
            && api.contains("C-3bp")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_task_join_stack_floor")
            && api.contains("MirOp::TaskJoin")
            && api.contains("Some((1, 1))")
    );
}
