//! C-3f: MIR JumpIfFalse condition type-tag floor fail-closed (MIR006).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{verify_mir_branch_floor, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3f"),
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
fn c3f_rejects_i32_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_i32",
            vec![MirOp::ConstI32(1), MirOp::JumpIfFalse(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_branch_floor(&mir).expect_err("I32 cond");
    assert_eq!(err.code.as_deref(), Some("MIR006"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("branch condition tag")
            && text.contains("JumpIfFalse")
            && text.contains("C-3f")
    );
}

#[test]
fn c3f_rejects_str_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_str",
            vec![
                MirOp::ConstStr("x".into()),
                MirOp::JumpIfFalse(0),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_branch_floor(&mir).expect_err("Str cond");
    assert_eq!(err.code.as_deref(), Some("MIR006"), "{err}");
}

#[test]
fn c3f_rejects_nil_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_nil",
            vec![MirOp::ConstNil, MirOp::JumpIfFalse(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_branch_floor(&mir).expect_err("Nil cond");
    assert_eq!(err.code.as_deref(), Some("MIR006"), "{err}");
}

#[test]
fn c3f_accepts_bool_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstBool(true), MirOp::JumpIfFalse(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_branch_floor(&mir).expect("Bool cond");
}

#[test]
fn c3f_accepts_unknown_loadlocal_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::LoadLocal(0), MirOp::JumpIfFalse(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_branch_floor(&mir).expect("Unknown cond");
}

#[test]
fn c3f_accepts_lt_then_jump() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::Lt { kind: 2 },
                MirOp::JumpIfFalse(0),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_branch_floor(&mir).expect("Lt→Bool→JumpIfFalse");
}

#[test]
fn c3f_compile_from_mir_rejects_i32_cond() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstI32(1),
                MirOp::JumpIfFalse(0),
                MirOp::ConstNil,
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("I32 cond must MIR006 via compile_from_mir");
    assert_eq!(err.code.as_deref(), Some("MIR006"), "{err}");
}

#[test]
fn c3f_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_branch_floor.rs"))
        .expect("mir_branch_floor.rs");
    assert!(
        api.contains("verify_mir_branch_floor") && api.contains("MIR006") && api.contains("C-3f"),
        "branch floor API + MIR006"
    );

    let lib = std::fs::read_to_string(root.join("crates/syx-mir/src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("verify_mir_branch_floor") && lib.contains("verify_mir_branch_floor_functions"),
        "lib.rs must re-export branch floor"
    );

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let typ = unify.find("verify_mir_type_floor").expect("type");
    let br = unify.find("verify_mir_branch_floor").expect("branch");
    assert!(br > typ, "branch floor after type floor in unify");

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
    let typ = body
        .find("verify_mir_type_floor_functions")
        .expect("type in body");
    let br = body
        .find("verify_mir_branch_floor_functions")
        .expect("branch in body");
    assert!(br > typ, "branch floor after type floor in compile_from_mir");
}
