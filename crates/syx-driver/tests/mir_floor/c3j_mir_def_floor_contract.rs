//! C-3j: optional owner_module Def floor fail-closed (MIR010).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_def_floor, MirFunction, MirInstr, MirModule, MirOp, MirDefIndex,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3j"),
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
fn c3j_rejects_oob_owner() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", Some(0))],
        module_names: vec![],
        ..Default::default()
    };
    let err = verify_mir_def_floor(&mir).expect_err("OOB");
    assert_eq!(err.code.as_deref(), Some("MIR010"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("out of range") && text.contains("C-3j"), "{text}");
}

#[test]
fn c3j_rejects_dup_owner_name() {
    let mir = MirModule {
        functions: vec![flat_fn("dup", Some(0)), flat_fn("dup", Some(0))],
        module_names: vec![SmolStr::new("m")],
        ..Default::default()
    };
    let err = verify_mir_def_floor(&mir).expect_err("dup");
    assert_eq!(err.code.as_deref(), Some("MIR010"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("duplicate") && text.contains("C-3j"), "{text}");
}

#[test]
fn c3j_all_none_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("a", None), flat_fn("b", None)],
        module_names: vec![SmolStr::new("m")],
        ..Default::default()
    };
    verify_mir_def_floor(&mir).expect("all None");
    assert!(MirDefIndex::build(&mir).unwrap().is_empty());
}

#[test]
fn c3j_empty_ok() {
    verify_mir_def_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3j_compile_from_mir_rejects_oob_owner() {
    let mir = MirModule {
        functions: vec![flat_fn("main", Some(9))],
        module_names: vec![SmolStr::new("only")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("OOB via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR010"), "{err}");
}

#[test]
fn c3j_compile_from_mir_accepts_all_none() {
    let mir = MirModule {
        functions: vec![flat_fn("main", None)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect("all None OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3j_hooks_after_try_and_populate_user_owner() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let try_u = unify.find("verify_mir_try_floor").expect("try unify");
    let def_u = unify.find("verify_mir_def_floor").expect("def unify");
    assert!(def_u > try_u, "unify def floor must follow try floor");
    assert!(
        unify.contains("owner_module_for_path")
            && unify.contains("set_owner_module")
            && unify.contains("entry_owner"),
        "unify must populate owner_module on user/entry paths"
    );
    // stdlib path must not set owner (append_mir_functions stays clone-as-is / None)
    let stdlib_merge = unify
        .find("fn merge_stdlib_modules_into_mir")
        .expect("stdlib merge");
    let user_loop = unify
        .find("// User (non-stdlib) deps always need a lower")
        .expect("user deps");
    assert!(user_loop > stdlib_merge);
    let stdlib_chunk = &unify[stdlib_merge..user_loop];
    assert!(
        !stdlib_chunk.contains("set_owner_module"),
        "stdlib merge must not populate owner_module"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    assert!(body.contains("verify_mir_try_floor_functions"));
    assert!(body.contains("verify_mir_def_floor_functions"));
    let try_c = body.find("verify_mir_try_floor_functions").expect("try");
    let def_c = body.find("verify_mir_def_floor_functions").expect("def");
    assert!(def_c > try_c, "codegen def floor must follow try floor");

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_def_floor.rs"))
        .expect("mir_def_floor.rs");
    assert!(
        api.contains("MIR010")
            && api.contains("C-3j")
            && api.contains("Not a DefId graph")
            && api.contains("verify_mir_def_floor")
            && api.contains("MirDefIndex")
    );

    let types = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_types.rs"))
        .expect("mir_types");
    assert!(
        types.contains("owner_module") && types.contains("C-3j"),
        "MirFunction must carry owner_module"
    );
}
