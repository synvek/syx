//! C-3bl: MIR ArrSorted stack-height floor fail-closed (MIR064).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_arr_sorted_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3bl"),
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
fn c3bl_rejects_arr_sorted_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::ArrSorted, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_arr_sorted_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR064"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before ArrSorted") && text.contains("C-3bl") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3bl_arr_sorted_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::ArrSorted, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_arr_sorted_stack_floor(&mir).expect("ok");
}

#[test]
fn c3bl_no_arr_sorted_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_arr_sorted_stack_floor(&mir).expect("no ArrSorted");
}

#[test]
fn c3bl_empty_ok() {
    verify_mir_arr_sorted_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3bl_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::CallIndirect(0), MirOp::ArrSorted, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_arr_sorted_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3bl_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::ArrSorted, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_arr_sorted_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3bl_compile_from_mir_rejects_arr_sorted_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::ArrSorted, MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR064"), "{err}");
}

#[test]
fn c3bl_compile_from_mir_accepts_arr_sorted() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::ArrSorted, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3bl_hooks_after_store_global_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let tostr_u = unify
        .find("verify_mir_store_global_stack_floor")
        .expect("store-global-stack unify");
    let not_u = unify
        .find("verify_mir_arr_sorted_stack_floor")
        .expect("not-stack unify");
    assert!(
        not_u > tostr_u,
        "unify not-stack floor must follow store-global-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let tostr_c = body
        .find("verify_mir_store_global_stack_floor_functions")
        .expect("store-global-stack codegen");
    let not_c = body
        .find("verify_mir_arr_sorted_stack_floor_functions")
        .expect("not-stack codegen");
    assert!(
        not_c > tostr_c,
        "codegen not-stack floor must follow store-global-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_arr_sorted_stack_floor.rs"))
        .expect("mir_arr_sorted_stack_floor.rs");
    assert!(
        api.contains("MIR064")
            && api.contains("C-3bl")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("verify_mir_arr_sorted_stack_floor")
            && api.contains("MirOp::ArrSorted")
    );
}
