//! MIR Def↔FnIndex consistency floor fail-closed (MIR082).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_def_fn_consistency_floor, verify_mir_def_fn_consistency_floor_functions,
    MirFnIndex, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("def_fn_consistency_floor"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, owner: Option<u32>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: owner,
    }
}

#[test]
fn rejects_stale_fn_index_missing() {
    let mir = MirModule {
        functions: vec![flat_fn("owned", Some(0))],
        module_names: vec![SmolStr::new("app")],
        ..Default::default()
    };
    let stale = MirFnIndex::build_from_names([SmolStr::new("other")].iter()).expect("stale");
    let err = verify_mir_def_fn_consistency_floor_functions(
        &stale,
        mir.module_names.len(),
        mir.functions.iter(),
    )
    .expect_err("missing");
    assert_eq!(err.code.as_deref(), Some("MIR082"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("missing from MirFnIndex") && text.contains("C-3cf"),
        "{text}"
    );
}

#[test]
fn rejects_stale_fn_index_wrong_slot() {
    let funcs = vec![flat_fn("a", Some(0)), flat_fn("b", Some(0))];
    let swapped =
        MirFnIndex::build_from_names([SmolStr::new("b"), SmolStr::new("a")].iter()).expect("swap");
    let err = verify_mir_def_fn_consistency_floor_functions(&swapped, 1, funcs.iter())
        .expect_err("slot");
    assert_eq!(err.code.as_deref(), Some("MIR082"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("MirFnIndex resolves to FuncId") && text.contains("C-3cf"),
        "{text}"
    );
}

#[test]
fn owned_matching_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", Some(0)), flat_fn("helper", Some(0))],
        module_names: vec![SmolStr::new("app")],
        ..Default::default()
    };
    verify_mir_def_fn_consistency_floor(&mir).expect("owned ok");
}

#[test]
fn all_none_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("a", None), flat_fn("b", None)],
        module_names: vec![SmolStr::new("m")],
        ..Default::default()
    };
    verify_mir_def_fn_consistency_floor(&mir).expect("all None");
}

#[test]
fn empty_ok() {
    verify_mir_def_fn_consistency_floor(&MirModule::default()).expect("empty");
}

#[test]
fn compile_from_mir_accepts_owned() {
    let mir = MirModule {
        functions: vec![flat_fn("main", Some(0))],
        module_names: vec![SmolStr::new("app")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("owned OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn compile_from_mir_accepts_all_none() {
    let mir = MirModule {
        functions: vec![flat_fn("main", None)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("None OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn hooks_after_call_host_arity() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let prev_u = unify
        .find("verify_mir_call_host_arity_floor")
        .expect("call-host-arity unify");
    let cur_u = unify
        .find("verify_mir_def_fn_consistency_floor_functions")
        .expect("def-fn consistency unify");
    assert!(
        cur_u > prev_u,
        "unify Def↔Fn floor must follow CallHost arity floor"
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
        .find("verify_mir_call_host_arity_floor_functions")
        .expect("call-host-arity codegen");
    let cur_c = body
        .find("verify_mir_def_fn_consistency_floor_functions")
        .expect("def-fn consistency codegen");
    assert!(
        cur_c > prev_c,
        "codegen Def↔Fn floor must follow CallHost arity floor"
    );

    let api = std::fs::read_to_string(
        root.join("crates/syx-mir/src/mir_def_fn_consistency_floor.rs"),
    )
    .expect("mir_def_fn_consistency_floor.rs");
    assert!(
        api.contains("MIR082")
            && api.contains("C-3cf")
            && api.contains("MirDefIndex")
            && api.contains("MirFnIndex")
            && api.contains("verify_mir_def_fn_consistency_floor")
    );
}
