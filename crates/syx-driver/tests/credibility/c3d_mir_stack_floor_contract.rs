//! C-3d: MIR pilot value-stack height floor fail-closed (MIR004).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_stack_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3d"),
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

fn flat_fn(name: &str, param_count: u32, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count,
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

fn block_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![MirBlock {
            id: 0,
            instructions: ops.into_iter().map(instr).collect(),
            successors: vec![],
        }],
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

#[test]
fn c3d_rejects_add_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_add",
            0,
            vec![MirOp::ConstI32(1), MirOp::Add { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_stack_floor(&mir).expect_err("Add underflow");
    assert_eq!(err.code.as_deref(), Some("MIR004"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("stack underflow") && text.contains("Add") && text.contains("C-3d"));
}

#[test]
fn c3d_rejects_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad_store",
            0,
            vec![MirOp::StoreLocal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_stack_floor(&mir).expect_err("StoreLocal underflow");
    assert_eq!(err.code.as_deref(), Some("MIR004"), "{err}");
    assert!(format!("{err}").contains("StoreLocal"));
}

#[test]
fn c3d_rejects_call_arity_underflow() {
    let callee = flat_fn("needs2", 2, vec![MirOp::ConstNil, MirOp::Return]);
    let caller = flat_fn(
        "main",
        0,
        vec![
            MirOp::Call(SmolStr::new("needs2")),
            MirOp::Return,
        ],
    );
    let mir = MirModule {
        functions: vec![caller, callee],
        ..Default::default()
    };
    let err = verify_mir_stack_floor(&mir).expect_err("Call underflow");
    assert_eq!(err.code.as_deref(), Some("MIR004"), "{err}");
    assert!(format!("{err}").contains("Call"));
}

#[test]
fn c3d_accepts_const_add_return() {
    let mir = MirModule {
        functions: vec![block_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::Add { kind: 0 },
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_stack_floor(&mir).expect("valid Const+Add+Return");
}

#[test]
fn c3d_accepts_known_call_arity() {
    let callee = flat_fn("needs1", 1, vec![MirOp::ConstNil, MirOp::Return]);
    let caller = flat_fn(
        "main",
        0,
        vec![
            MirOp::ConstI32(1),
            MirOp::Call(SmolStr::new("needs1")),
            MirOp::Return,
        ],
    );
    let mir = MirModule {
        functions: vec![caller, callee],
        ..Default::default()
    };
    verify_mir_stack_floor(&mir).expect("known Call arity");
}

#[test]
fn c3d_compile_from_mir_rejects_add_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            0,
            vec![MirOp::ConstI32(1), MirOp::Add { kind: 0 }, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("Add underflow must MIR004 via compile_from_mir");
    assert_eq!(err.code.as_deref(), Some("MIR004"), "{err}");
}

#[test]
fn c3d_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_stack_floor.rs"))
        .expect("mir_stack_floor.rs");
    assert!(
        api.contains("verify_mir_stack_floor") && api.contains("MIR004") && api.contains("C-3d"),
        "stack floor API + MIR004"
    );

    let lib = std::fs::read_to_string(root.join("crates/syx-mir/src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("verify_mir_stack_floor") && lib.contains("verify_mir_stack_floor_functions"),
        "lib.rs must re-export stack floor"
    );

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let cfg = unify.find("verify_mir_cfg_floor").expect("cfg");
    let fnx = unify.find("MirFnIndex::build").expect("fn index");
    let stk = unify.find("verify_mir_stack_floor").expect("stack");
    assert!(fnx > cfg, "FnIndex after CFG");
    assert!(stk > fnx, "stack floor after FnIndex in unify");

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
    let cfg = body.find("verify_mir_cfg_floor").expect("cfg in body");
    let fnx = body
        .find("MirFnIndex::build_from_functions")
        .expect("fn index in body");
    let stk = body
        .find("verify_mir_stack_floor_functions")
        .expect("stack in body");
    assert!(fnx > cfg, "FnIndex after CFG in compile_from_mir");
    assert!(stk > fnx, "stack floor after FnIndex in compile_from_mir");
}
