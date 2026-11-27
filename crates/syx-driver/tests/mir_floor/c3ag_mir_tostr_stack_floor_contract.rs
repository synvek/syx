//! C-3ag: MIR ToStr stack-height floor fail-closed (MIR033).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_tostr_stack_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ag"),
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
fn c3ag_rejects_tostr_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::ToStr, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_tostr_stack_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR033"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("ToStr") && text.contains("C-3ag") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn c3ag_tostr_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::ConstI32(1), MirOp::ToStr, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_tostr_stack_floor(&mir).expect("ok");
}

#[test]
fn c3ag_no_tostr_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_tostr_stack_floor(&mir).expect("no ToStr");
}

#[test]
fn c3ag_empty_ok() {
    verify_mir_tostr_stack_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3ag_abandon_non_trackable() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::Dup, MirOp::ToStr, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_tostr_stack_floor(&mir).expect("abandon");
}

#[test]
fn c3ag_abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::StoreLocal(0), MirOp::ToStr, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_tostr_stack_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn c3ag_compile_from_mir_rejects_tostr_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::ToStr, MirOp::Return])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR033"), "{err}");
}

#[test]
fn c3ag_compile_from_mir_accepts_tostr() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::ConstI32(1), MirOp::ToStr, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3ag_hooks_after_dup_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let dup_u = unify
        .find("verify_mir_dup_stack_floor")
        .expect("dup-stack unify");
    let tostr_u = unify
        .find("verify_mir_tostr_stack_floor")
        .expect("tostr-stack unify");
    assert!(
        tostr_u > dup_u,
        "unify tostr-stack floor must follow dup-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let dup_c = body
        .find("verify_mir_dup_stack_floor_functions")
        .expect("dup-stack codegen");
    let tostr_c = body
        .find("verify_mir_tostr_stack_floor_functions")
        .expect("tostr-stack codegen");
    assert!(
        tostr_c > dup_c,
        "codegen tostr-stack floor must follow dup-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_tostr_stack_floor.rs"))
        .expect("mir_tostr_stack_floor.rs");
    assert!(
        api.contains("MIR033")
            && api.contains("C-3ag")
            && api.contains("Not a change to `is_suite1_pilot_mir_op`")
            && api.contains("ToStr")
            && api.contains("verify_mir_tostr_stack_floor")
    );
}
