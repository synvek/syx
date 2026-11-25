//! C-3s: MIR aot_export_roots name-resolve floor fail-closed (MIR019).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_aot_export_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3s"),
        decls: vec![],
    }
}

fn flat_fn(name: &str) -> MirFunction {
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
        owner_module: None,
    }
}

#[test]
fn c3s_rejects_unknown_root() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        aot_export_roots: vec![SmolStr::new("missing")],
        ..Default::default()
    };
    let err = verify_mir_aot_export_floor(&mir).expect_err("unknown");
    assert_eq!(err.code.as_deref(), Some("MIR019"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("aot_export_roots")
            && text.contains("missing")
            && text.contains("C-3s")
            && text.contains("functions[].name"),
        "{text}"
    );
}

#[test]
fn c3s_resolvable_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("greet")],
        aot_export_roots: vec![SmolStr::new("greet")],
        ..Default::default()
    };
    verify_mir_aot_export_floor(&mir).expect("resolvable");
}

#[test]
fn c3s_empty_roots_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        aot_export_roots: vec![],
        ..Default::default()
    };
    verify_mir_aot_export_floor(&mir).expect("empty");
}

#[test]
fn c3s_empty_module_ok() {
    verify_mir_aot_export_floor(&MirModule::default()).expect("empty module");
}

#[test]
fn c3s_compile_from_mir_rejects_unknown() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        aot_export_roots: vec![SmolStr::new("ghost")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("unknown via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR019"), "{err}");
}

#[test]
fn c3s_compile_from_mir_accepts_resolvable() {
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("greet")],
        aot_export_roots: vec![SmolStr::new("greet")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3s_hooks_after_local_debug() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let debug_u = unify
        .find("verify_mir_local_debug_floor")
        .expect("local-debug unify");
    let export_u = unify
        .find("verify_mir_aot_export_floor")
        .expect("aot-export unify");
    assert!(
        export_u > debug_u,
        "unify aot-export floor must follow local-debug floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let debug_c = body
        .find("verify_mir_local_debug_floor_functions")
        .expect("local-debug codegen");
    let export_c = body
        .find("verify_mir_aot_export_floor")
        .expect("aot-export codegen");
    assert!(
        export_c > debug_c,
        "codegen aot-export floor must follow local-debug floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_aot_export_floor.rs"))
        .expect("mir_aot_export_floor.rs");
    assert!(
        api.contains("MIR019")
            && api.contains("C-3s")
            && api.contains("aot_export_roots")
            && api.contains("Not AOT shake rewrite")
            && api.contains("verify_mir_aot_export_floor")
    );
}
