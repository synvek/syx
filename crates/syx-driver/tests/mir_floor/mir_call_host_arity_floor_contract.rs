//! MIR CallHost pilot-arity stack-height floor fail-closed (MIR081).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_call_host_arity_floor, MirFunction, MirInstr, MirModule, MirOp,
};
use syx_stdlib::{host_fn_arity, host_fn_index, HostFnId};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("call_host_arity_floor"),
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

fn fmt_print_idx() -> u32 {
    host_fn_index(HostFnId::FmtPrint)
}

#[test]
fn rejects_known_arity_call_host_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::CallHost(fmt_print_idx()), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_call_host_arity_floor(&mir).expect_err("underflow");
    assert_eq!(err.code.as_deref(), Some("MIR081"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("before CallHost") && text.contains("C-3ce") && text.contains("underflow"),
        "{text}"
    );
}

#[test]
fn known_arity_call_host_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::CallHost(fmt_print_idx()),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("ok");
}

#[test]
fn no_call_host_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", vec![MirOp::Return])],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("no CallHost");
}

#[test]
fn empty_ok() {
    verify_mir_call_host_arity_floor(&MirModule::default()).expect("empty");
}

#[test]
fn unknown_arity_abandons() {
    assert_eq!(host_fn_arity(u32::MAX), None);
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::CallHost(u32::MAX), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("unknown arity abandon");
}

#[test]
fn abandon_store_local_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::StoreLocal(0),
                MirOp::CallHost(fmt_print_idx()),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("StoreLocal abandon");
}

#[test]
fn compile_from_mir_rejects_underflow() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::CallHost(fmt_print_idx()), MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("underflow via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR081"), "{err}");
}

#[test]
fn compile_from_mir_accepts_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![
                MirOp::ConstI32(1),
                MirOp::CallHost(fmt_print_idx()),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn hooks_after_const_fn_index() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev_u = unify
        .find("verify_mir_const_fn_index_floor_functions")
        .expect("const-fn-index unify");
    let cur_u = unify
        .find("verify_mir_call_host_arity_floor")
        .expect("call-host-arity unify");
    assert!(
        cur_u > prev_u,
        "unify CallHost arity floor must follow ConstFnIndex floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(8000);
    let body = &codegen[start..start + 1 + rel];
    let prev_c = body
        .find("verify_mir_const_fn_index_floor_functions")
        .expect("const-fn-index codegen");
    let cur_c = body
        .find("verify_mir_call_host_arity_floor_functions")
        .expect("call-host-arity codegen");
    assert!(
        cur_c > prev_c,
        "codegen CallHost arity floor must follow ConstFnIndex floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_call_host_arity_floor.rs"))
            .expect("mir_call_host_arity_floor.rs");
    assert!(
        api.contains("MIR081")
            && api.contains("C-3ce")
            && api.contains("host_fn_arity")
            && api.contains("verify_mir_call_host_arity_floor")
            && api.contains("CallHost")
    );
}
