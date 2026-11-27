//! C-3ac: MIR aot_export_roots uniqueness floor fail-closed (MIR029).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_aot_export_unique_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3ac"),
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
fn c3ac_rejects_duplicate_roots() {
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("greet")],
        aot_export_roots: vec![SmolStr::new("greet"), SmolStr::new("greet")],
        ..Default::default()
    };
    let err = verify_mir_aot_export_unique_floor(&mir).expect_err("dup");
    assert_eq!(err.code.as_deref(), Some("MIR029"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("duplicate")
            && text.contains("C-3ac")
            && text.contains("aot_export_roots"),
        "{text}"
    );
}

#[test]
fn c3ac_unique_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("greet")],
        aot_export_roots: vec![SmolStr::new("greet"), SmolStr::new("main")],
        ..Default::default()
    };
    verify_mir_aot_export_unique_floor(&mir).expect("unique");
}

#[test]
fn c3ac_empty_roots_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main")],
        aot_export_roots: vec![],
        ..Default::default()
    };
    verify_mir_aot_export_unique_floor(&mir).expect("empty");
}

#[test]
fn c3ac_empty_module_ok() {
    verify_mir_aot_export_unique_floor(&MirModule::default()).expect("empty module");
}

#[test]
fn c3ac_compile_from_mir_rejects_dup() {
    // Roots must resolve (MIR019) so MIR029 is the fail-closed hit.
    let mir = MirModule {
        functions: vec![flat_fn("main"), flat_fn("greet")],
        aot_export_roots: vec![SmolStr::new("greet"), SmolStr::new("greet")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("dup via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR029"), "{err}");
}

#[test]
fn c3ac_compile_from_mir_accepts_unique() {
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
fn c3ac_hooks_after_succ_unique() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let uniq_u = unify
        .find("verify_mir_succ_unique_floor")
        .expect("succ-unique unify");
    let export_u = unify
        .find("verify_mir_aot_export_unique_floor")
        .expect("aot-export-unique unify");
    assert!(
        export_u > uniq_u,
        "unify aot-export-unique floor must follow succ-unique floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let uniq_c = body
        .find("verify_mir_succ_unique_floor_functions")
        .expect("succ-unique codegen");
    let export_c = body
        .find("verify_mir_aot_export_unique_floor")
        .expect("aot-export-unique codegen");
    assert!(
        export_c > uniq_c,
        "codegen aot-export-unique floor must follow succ-unique floor"
    );

    let api =
        std::fs::read_to_string(root.join("crates/syx-mir/src/mir_aot_export_unique_floor.rs"))
            .expect("mir_aot_export_unique_floor.rs");
    assert!(
        api.contains("MIR029")
            && api.contains("C-3ac")
            && api.contains("aot_export_roots")
            && api.contains("verify_mir_aot_export_unique_floor")
    );
}
