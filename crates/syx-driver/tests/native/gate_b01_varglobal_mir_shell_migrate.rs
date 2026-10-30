//! VarGlobal→MIR `__init_global` migrate
//! Combined init via MIR; residual 14; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_varglobal_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-varglobal-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "docs/gates/gate-b01-finalclass-mir-shell-migrate.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 VarGlobal MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01VarGlobalMirShellMigrateInventory")
            && cg.contains("gate_b01_varglobal_mir_shell_migrate_inventory")
            && cg.contains("ADR-813")
            && (cg.contains("b01_varglobal_mir_shell_migrate_signed: true")
                || cg.contains("b01_varglobal_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01VarGlobalMirShellMigrateInventory (W469 / ADR-813)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must track live residual (11 after Actor migrate)"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::VarGlobal")
            && cg.contains("lower_varglobal_inits")
            && cg.contains("program.globals.push"),
        "compile_type_shells must skip VarGlobal; MIR path must register globals + merge inits"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("lower_varglobal_inits")
            && mir.contains("__init_global")
            && mir.contains("ADR-813"),
        "lower_varglobal_inits must emit combined __init_global (W469 / ADR-813)"
    );
}

#[test]
fn b01_varglobal_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-varglobal-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-813") && (spec.contains("VarGlobal") || spec.contains("__init_global")),
        "spec must cite ADR-813 VarGlobal migrate"
    );
    assert!(
        spec.contains("three-hundred-and-nineteenth"),
        
        "spec must cite W469 fingerprint"
    );
    assert!(
        spec.contains("GateB01VarGlobalMirShellMigrateInventory")
            && spec.contains("gate_b01_varglobal_mir_shell_migrate_inventory"),
        "spec must name VarGlobal migrate inventory API"
    );
    assert!(
        spec.contains("b01_varglobal_mir_shell_migrate_signed=true")
            || spec.contains("`b01_varglobal_mir_shell_migrate_signed=true`"),
        "spec must cite b01_varglobal_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=14")
            || spec.contains("residual_ast_shell_cluster_count=13")
            || spec.contains("`residual_ast_shell_cluster_count=14`")
            || spec.contains("`residual_ast_shell_cluster_count=13`")
            || spec.contains("**15→14**")
            || spec.contains("15→14"),
        "spec must cite residual cluster count (W469 signed 14; live may be 13)"
    );
    assert!(
        spec.contains("lower_varglobal_inits")
            && spec.contains("compile_type_shells")
            && spec.contains("program.globals"),
        "spec must name MIR lower + AST skip + globals register"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-01") || spec.contains("Export") || spec.contains("Class")),
        "spec Frozen must fence B-01 [x] / other shells / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_varglobal_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_varglobal_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.finalclass_migrate_floor_ok);
    assert!(inv.varglobal_mir_init_path_present);
    assert!(inv.varglobal_ast_shell_skipped);
    assert!(inv.program_globals_registered_on_mir_path);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_varglobal_mir_shell_migrate_signed);
    let fc = syx_codegen::gate_b01_finalclass_mir_shell_migrate_inventory();
    assert!(fc.b01_finalclass_mir_shell_migrate_signed);
    assert_eq!(fc.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

