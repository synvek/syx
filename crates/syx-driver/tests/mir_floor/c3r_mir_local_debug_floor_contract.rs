//! C-3r: MIR local debug metadata floor fail-closed (MIR018).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_local_debug_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3r"),
        decls: vec![],
    }
}

fn flat_fn(
    name: &str,
    local_count: u32,
    local_names: Vec<SmolStr>,
    local_types: Vec<SmolStr>,
) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names,
        local_types,
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3r_rejects_names_mismatch() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            2,
            vec![SmolStr::new("only")],
            vec![],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_debug_floor(&mir).expect_err("names");
    assert_eq!(err.code.as_deref(), Some("MIR018"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("local_names") && text.contains("C-3r") && text.contains("local_count"),
        "{text}"
    );
}

#[test]
fn c3r_rejects_types_mismatch() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            1,
            vec![],
            vec![SmolStr::new("Int"), SmolStr::new("extra")],
        )],
        ..Default::default()
    };
    let err = verify_mir_local_debug_floor(&mir).expect_err("types");
    assert_eq!(err.code.as_deref(), Some("MIR018"), "{err}");
    assert!(format!("{err}").contains("local_types"));
}

#[test]
fn c3r_equal_len_ok() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            2,
            vec![SmolStr::new("a"), SmolStr::new("b")],
            vec![SmolStr::new("Int"), SmolStr::new("Int")],
        )],
        ..Default::default()
    };
    verify_mir_local_debug_floor(&mir).expect("equal");
}

#[test]
fn c3r_empty_vectors_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("ok", 3, vec![], vec![])],
        ..Default::default()
    };
    verify_mir_local_debug_floor(&mir).expect("stripped");
}

#[test]
fn c3r_empty_module_ok() {
    verify_mir_local_debug_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3r_compile_from_mir_rejects_mismatch() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            2,
            vec![SmolStr::new("x")],
            vec![],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("mismatch via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR018"), "{err}");
}

#[test]
fn c3r_compile_from_mir_accepts_empty_debug() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, vec![], vec![])],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3r_hooks_after_build_stack() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let build_u = unify
        .find("verify_mir_build_stack_floor")
        .expect("build-stack unify");
    let debug_u = unify
        .find("verify_mir_local_debug_floor")
        .expect("local-debug unify");
    assert!(
        debug_u > build_u,
        "unify local-debug floor must follow build-stack floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let build_c = body
        .find("verify_mir_build_stack_floor_functions")
        .expect("build-stack codegen");
    let debug_c = body
        .find("verify_mir_local_debug_floor_functions")
        .expect("local-debug codegen");
    assert!(
        debug_c > build_c,
        "codegen local-debug floor must follow build-stack floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_local_debug_floor.rs"))
        .expect("mir_local_debug_floor.rs");
    assert!(
        api.contains("MIR018")
            && api.contains("C-3r")
            && api.contains("local_names")
            && api.contains("local_types")
            && api.contains("Not locals type semantics")
            && api.contains("verify_mir_local_debug_floor")
    );
}
