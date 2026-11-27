//! S2-I136…I141 S2-03 MIR consumer contracts
//! (open → hollow → delete-fn → compile_module delete → compile_decl delete
//! → differential deepen).
//! Proof lives in tests/ only (≠ restore GateB01/B02 src inventory).
//! See docs/adr/stage2-s2-03-open.md · stage2-s2-03-type-shell-hollow.md ·
//! stage2-s2-03-type-shell-delete.md · stage2-s2-03-compile-module-delete.md ·
//! stage2-s2-03-compile-decl-delete.md · stage2-s2-03-differential-deepen.md.
//! Suite-1 VM⟷MIR floor: `s2_03_differential_floor.rs`.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn greeter_fixture() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/types/greeter_extension.sy")
}

fn codegen_lib_rs() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../syx-codegen/src/lib.rs"
    ))
}

#[test]
fn s2_03_production_compile_attaches_mir_module() {
    let path = greeter_fixture();
    let unit = compile_file_with_options(&path, &DriverOptions::default())
        .expect("compile greeter_extension.sy");
    assert!(
        unit.program.mir_module.is_some(),
        "S2-03 floor: production compile must attach mir_module (compile_with_mir path)"
    );
}

#[test]
fn s2_03_public_ast_compile_api_absent() {
    // W482 / ADR-826 deleted public AST-only entry points. Honesty fence without
    // restoring GateB02 inventory into syx-codegen/src.
    let cg = codegen_lib_rs();
    assert!(
        cg.contains("W482 / ADR-826: public AST-only `compile`"),
        "codegen must keep W482 delete charter comment"
    );
    assert!(
        cg.contains("pub fn compile_with_mir("),
        "MIR primary path must remain public"
    );
    assert!(
        !cg.contains("pub fn compile("),
        "public AST-only CodeGenerator::compile must stay deleted"
    );
    assert!(
        !cg.contains("pub fn compile_into("),
        "public AST-only CodeGenerator::compile_into must stay deleted"
    );
    assert!(
        !cg.contains("pub fn compile_into_with_semantic("),
        "public AST-only CodeGenerator::compile_into_with_semantic must stay deleted"
    );
}

#[test]
fn s2_03_mir_path_run_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = greeter_fixture();
    let opts = DriverOptions::default();
    let unit = compile_file_with_options(&path, &opts).expect("compile");
    assert!(unit.program.mir_module.is_some());
    let (_, lines) = run_file_printed(&path, &opts).expect("run");
    assert!(
        lines.iter().any(|l| l.trim() == "Greeter"),
        "MIR-lowered bytecode VM path must print Greeter; got {lines:?}"
    );
}

#[test]
fn s2_03_compile_type_shells_absent_on_mir_path() {
    // S2-I138: compile_type_shells call + method deleted after I137 hollow.
    let cg = codegen_lib_rs();
    assert!(
        cg.contains("S2-I138 / ADR stage2-s2-03-type-shell-delete"),
        "codegen must keep S2-I138 delete charter at former call site"
    );
    assert!(
        !cg.contains("pub fn compile_type_shells("),
        "compile_type_shells method must be deleted"
    );
    assert!(
        !cg.contains("gen.compile_type_shells("),
        "compile_with_mir must not call compile_type_shells"
    );
}

#[test]
fn s2_03_compile_module_absent_on_mir_path() {
    // S2-I139: dead AST module entry trio deleted.
    // S2-I140: compile_decl corpse also deleted (asserted separately).
    let cg = codegen_lib_rs();
    assert!(
        cg.contains("S2-I139 / ADR stage2-s2-03-compile-module-delete"),
        "codegen must keep S2-I139 compile_module delete charter"
    );
    assert!(
        !cg.contains("fn compile_module("),
        "compile_module must be deleted"
    );
    assert!(
        !cg.contains("fn preregister_decl_functions("),
        "preregister_decl_functions must be deleted"
    );
    assert!(
        !cg.contains("fn ensure_function_slot("),
        "ensure_function_slot must be deleted"
    );
    assert!(
        cg.contains("fn compile_function_decl("),
        "live compile_function_decl (closures) must remain"
    );
}

#[test]
fn s2_03_compile_decl_absent_on_mir_path() {
    // S2-I140: compile_decl corpse cluster deleted; live closures keep compile_function_decl.
    let cg = codegen_lib_rs();
    assert!(
        cg.contains("S2-I140 / ADR stage2-s2-03-compile-decl-delete"),
        "codegen must keep S2-I140 compile_decl delete charter"
    );
    assert!(
        !cg.contains("fn compile_decl("),
        "compile_decl must be deleted"
    );
    assert!(
        !cg.contains("fn compile_dataclass_init("),
        "compile_dataclass_init must be deleted"
    );
    assert!(
        !cg.contains("fn compile_type_members("),
        "compile_type_members must be deleted"
    );
    assert!(
        !cg.contains("fn compile_class_init("),
        "compile_class_init must be deleted"
    );
    assert!(
        !cg.contains("fn compile_enum_case_init("),
        "compile_enum_case_init must be deleted"
    );
    assert!(
        !cg.contains("fn merged_class_fields("),
        "merged_class_fields must be deleted"
    );
    assert!(
        cg.contains("fn compile_function_decl("),
        "live compile_function_decl (closures) must remain"
    );
}
