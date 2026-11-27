//! C-3bn: MIR ActorRegister stack-height floor fail-closed (MIR066).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_actor_register_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3bn"),
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

#[test]
fn c3bn_rejects_actor_register_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::ActorRegister, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_actor_register_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR066"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before ActorRegister") && text.contains("C-3bn") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3bn_actor_register_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::ActorRegister, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_actor_register_stack_floor(&mir).expect("ok");
}

#[test]
fn c3bn_no_actor_register_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_actor_register_stack_floor(&mir).expect("no ActorRegister");
}

#[test]
fn c3bn_empty_ok() {
    verify_mir_actor_register_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3bn_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::MakeIterator, MirOp::ActorRegister, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_actor_register_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3bn_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::ActorRegister, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_actor_register_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3bn_compile_from_mir_rejects_actor_register_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::ActorRegister, MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR066"), "{err}");
}

#[test]
fn c3bn_compile_from_mir_accepts_actor_register() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::ActorRegister, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3bn_hooks_after_call_indirect_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev = unify
        .find("verify_mir_call_indirect_stack_floor")
        .expect("call-indirect-stack unify");
    let cur = unify
        .find("verify_mir_actor_register_stack_floor")
        .expect("actor-register-stack unify");
    assert!(
        cur > prev,
        "unify actor-register-stack floor must follow call-indirect-stack floor"
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
        .find("verify_mir_call_indirect_stack_floor_functions")
        .expect("call-indirect-stack codegen");
    let cur_c = body
        .find("verify_mir_actor_register_stack_floor_functions")
        .expect("actor-register-stack codegen");
    assert!(
        cur_c > prev_c,
        "codegen actor-register-stack floor must follow call-indirect-stack floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_actor_register_stack_floor.rs"))
            .expect("mir_actor_register_stack_floor.rs");
    assert!(
        api.contains("MIR066")
            && api.contains("C-3bn")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_actor_register_stack_floor")
            && api.contains("MirOp::ActorRegister")
            && api.contains("Some((1, 1))")
    );
}
