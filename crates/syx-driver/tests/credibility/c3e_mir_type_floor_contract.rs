//! C-3e: MIR pilot type-tag floor fail-closed (MIR005).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{verify_mir_type_floor, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3e"),
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
        owner_module: None,
    }
}

#[test]
fn c3e_rejects_nil_add() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_nil",
            vec![
                MirOp::ConstNil,
                MirOp::ConstI32(1),
                MirOp::Add { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_type_floor(&mir).expect_err("Nil Add");
    assert_eq!(err.code.as_deref(), Some("MIR005"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("type tag") && text.contains("Add") && text.contains("C-3e"));
}

#[test]
fn c3e_rejects_str_sub() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_sub",
            vec![
                MirOp::ConstStr("a".into()),
                MirOp::ConstI32(1),
                MirOp::Sub { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_type_floor(&mir).expect_err("Str Sub");
    assert_eq!(err.code.as_deref(), Some("MIR005"), "{err}");
    assert!(format!("{err}").contains("Sub"));
}

#[test]
fn c3e_rejects_bool_mul() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_mul",
            vec![
                MirOp::ConstBool(true),
                MirOp::ConstI32(1),
                MirOp::Mul { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_type_floor(&mir).expect_err("Bool Mul");
    assert_eq!(err.code.as_deref(), Some("MIR005"), "{err}");
    assert!(format!("{err}").contains("Mul"));
}

#[test]
fn c3e_accepts_i32_add() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::Add { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_type_floor(&mir).expect("I32 Add");
}

#[test]
fn c3e_accepts_str_concat_add() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "cat",
            vec![
                MirOp::ConstStr("a".into()),
                MirOp::ConstStr("b".into()),
                MirOp::Add { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_type_floor(&mir).expect("Str concat Add");
}

#[test]
fn c3e_accepts_unknown_loadlocal_add() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "unk",
            vec![
                MirOp::LoadLocal(0),
                MirOp::ConstI32(1),
                MirOp::Add { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_type_floor(&mir).expect("Unknown Add");
}

#[test]
fn c3e_compile_from_mir_rejects_nil_add() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstNil,
                MirOp::ConstI32(1),
                MirOp::Add { kind: 2 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("Nil Add must MIR005 via compile_from_mir");
    assert_eq!(err.code.as_deref(), Some("MIR005"), "{err}");
}

#[test]
fn c3e_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_type_floor.rs"))
        .expect("mir_type_floor.rs");
    assert!(
        api.contains("verify_mir_type_floor") && api.contains("MIR005") && api.contains("C-3e"),
        "type floor API + MIR005"
    );

    let lib = std::fs::read_to_string(root.join("crates/syx-mir/src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("verify_mir_type_floor") && lib.contains("verify_mir_type_floor_functions"),
        "lib.rs must re-export type floor"
    );

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let stk = unify.find("verify_mir_stack_floor").expect("stack");
    let typ = unify.find("verify_mir_type_floor").expect("type");
    assert!(typ > stk, "type floor after stack floor in unify");

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
    let stk = body
        .find("verify_mir_stack_floor_functions")
        .expect("stack in body");
    let typ = body
        .find("verify_mir_type_floor_functions")
        .expect("type in body");
    assert!(typ > stk, "type floor after stack floor in compile_from_mir");
}
