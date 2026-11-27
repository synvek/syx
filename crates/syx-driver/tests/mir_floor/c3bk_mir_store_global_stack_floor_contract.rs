//! C-3bk: MIR Pop stack-height floor fail-closed (MIR063).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_store_global_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3bk"),
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
fn c3bk_rejects_store_global_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::StoreGlobal(0), MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_store_global_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR063"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("StoreGlobal") && text.contains("C-3bk") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3bk_store_global_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::StoreGlobal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_store_global_stack_floor(&mir).expect("ok");
}

#[test]
fn c3bk_no_store_global_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_store_global_stack_floor(&mir).expect("no StoreGlobal");
}

#[test]
fn c3bk_empty_ok() {
    verify_mir_store_global_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3bk_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ArrSorted, MirOp::StoreGlobal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_store_global_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3bk_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::StoreGlobal(0), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_store_global_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3bk_compile_from_mir_rejects_store_global_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::StoreGlobal(0), MirOp::Return])],
        global_count: 1, // MIR012 slot OK; MIR063 stack floor fires
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR063"), "{err}");
}

#[test]
fn c3bk_compile_from_mir_accepts_store_global() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::StoreGlobal(0), MirOp::Return],
        )],
        global_count: 1, // MIR012 slot OK; stack height OK → MIR063 pass
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3bk_hooks_after_opaque_value_eq_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let succ_u = unify
        .find("verify_mir_opaque_value_eq_stack_floor")
        .expect("opaque-value-eq-stack unify");
    let pop_u = unify
        .find("verify_mir_store_global_stack_floor")
        .expect("pop-stack unify");
    assert!(
        pop_u > succ_u,
        "unify pop-stack floor must follow opaque-value-eq-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let succ_c = body
        .find("verify_mir_opaque_value_eq_stack_floor_functions")
        .expect("opaque-value-eq-stack codegen");
    let pop_c = body
        .find("verify_mir_store_global_stack_floor_functions")
        .expect("pop-stack codegen");
    assert!(
        pop_c > succ_c,
        "codegen pop-stack floor must follow opaque-value-eq-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_store_global_stack_floor.rs"))
        .expect("mir_store_global_stack_floor.rs");
    assert!(
        api.contains("MIR063")
            && api.contains("C-3bk")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("StoreGlobal")
            && api.contains("verify_mir_store_global_stack_floor")
            && api.contains("Some((1, 0))")
    );
}
