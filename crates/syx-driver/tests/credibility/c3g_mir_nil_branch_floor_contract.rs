//! C-3g: MIR JumpIfNil condition type-tag floor fail-closed (MIR007).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{verify_mir_nil_branch_floor, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3g"),
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

fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
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
    }
}

#[test]
fn c3g_rejects_i32_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_i32",
            vec![MirOp::ConstI32(1), MirOp::JumpIfNil(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_nil_branch_floor(&mir).expect_err("I32");
    assert_eq!(err.code.as_deref(), Some("MIR007"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("nil-branch condition tag")
            && text.contains("JumpIfNil")
            && text.contains("C-3g")
    );
}

#[test]
fn c3g_rejects_str_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_str",
            vec![
                MirOp::ConstStr("x".into()),
                MirOp::JumpIfNil(0),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_nil_branch_floor(&mir).expect_err("Str");
    assert_eq!(err.code.as_deref(), Some("MIR007"), "{err}");
}

#[test]
fn c3g_rejects_bool_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_bool",
            vec![MirOp::ConstBool(true), MirOp::JumpIfNil(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_nil_branch_floor(&mir).expect_err("Bool");
    assert_eq!(err.code.as_deref(), Some("MIR007"), "{err}");
}

#[test]
fn c3g_accepts_nil_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstNil, MirOp::JumpIfNil(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_nil_branch_floor(&mir).expect("Nil cond");
}

#[test]
fn c3g_accepts_unknown_then_store() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::LoadLocal(0),
                MirOp::JumpIfNil(0),
                MirOp::StoreLocal(1),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_nil_branch_floor(&mir).expect("Unknown leave + StoreLocal");
}

#[test]
fn c3g_compile_from_mir_rejects_i32_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstI32(1),
                MirOp::JumpIfNil(0),
                MirOp::ConstNil,
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("I32 JumpIfNil must MIR007");
    assert_eq!(err.code.as_deref(), Some("MIR007"), "{err}");
}

#[test]
fn c3g_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_nil_branch_floor.rs"))
        .expect("mir_nil_branch_floor.rs");
    assert!(
        api.contains("verify_mir_nil_branch_floor")
            && api.contains("MIR007")
            && api.contains("C-3g"),
        "nil-branch floor API + MIR007"
    );

    let lib = std::fs::read_to_string(root.join("crates/syx-mir/src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("verify_mir_nil_branch_floor")
            && lib.contains("verify_mir_nil_branch_floor_functions"),
        "lib.rs must re-export nil-branch floor"
    );

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let br = unify.find("verify_mir_branch_floor").expect("branch");
    let nil = unify.find("verify_mir_nil_branch_floor").expect("nil");
    assert!(nil > br, "nil-branch floor after branch floor in unify");

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let nxt = after
        .find("\n    pub fn ")
        .map(|i| start + 1 + i)
        .unwrap_or(start + 5000);
    let body = &codegen[start..nxt];
    let br = body
        .find("verify_mir_branch_floor_functions")
        .expect("branch in body");
    let nil = body
        .find("verify_mir_nil_branch_floor_functions")
        .expect("nil in body");
    assert!(nil > br, "nil-branch after branch in compile_from_mir");
}
