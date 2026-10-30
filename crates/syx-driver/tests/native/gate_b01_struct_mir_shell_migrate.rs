//! Struct→MIR type-shell migrate
//! Struct shells via MIR; residual 16; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_struct_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-struct-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 Struct MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01StructMirShellMigrateInventory")
            && cg.contains("gate_b01_struct_mir_shell_migrate_inventory")
            && cg.contains("ADR-811")
            && (cg.contains("b01_struct_mir_shell_migrate_signed: true")
                || cg.contains("b01_struct_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01StructMirShellMigrateInventory (W467 / ADR-811)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must track live residual (11 after Actor migrate)"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::Struct")
            && (cg.contains("Decl::DataClass") && cg.contains("Decl::Enum")),
        "compile_type_shells must skip Struct with DataClass/Enum"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("lower_dataclass_shells")
            && mir.contains("Decl::Struct")
            && mir.contains("ADR-811"),
        "lower_dataclass_shells must cover Decl::Struct (W467 / ADR-811)"
    );
}

#[test]
fn b01_struct_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-struct-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-811") && (spec.contains("Struct") || spec.contains("struct")),
        "spec must cite ADR-811 Struct migrate"
    );
    assert!(
        spec.contains("three-hundred-and-seventeenth"),
        
        "spec must cite W467 fingerprint"
    );
    assert!(
        spec.contains("GateB01StructMirShellMigrateInventory")
            && spec.contains("gate_b01_struct_mir_shell_migrate_inventory"),
        "spec must name Struct migrate inventory API"
    );
    assert!(
        spec.contains("b01_struct_mir_shell_migrate_signed=true")
            || spec.contains("`b01_struct_mir_shell_migrate_signed=true`"),
        "spec must cite b01_struct_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=15")
            || spec.contains("`residual_ast_shell_cluster_count=15`")
            || spec.contains("residual_ast_shell_cluster_count=16")
            || spec.contains("`residual_ast_shell_cluster_count=16`")
            || spec.contains("**17→16**")
            || spec.contains("17→16"),
        "spec must cite residual cluster count (W467 signed 16; live may be 15)"
    );
    assert!(
        spec.contains("lower_dataclass_shells") && spec.contains("compile_type_shells"),
        "spec must name MIR lower + AST skip paths"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-01") || spec.contains("Export") || spec.contains("Class")),
        "spec Frozen must fence B-01 [x] / other shells / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_struct_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_struct_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.deepen_floor_ok);
    assert!(inv.struct_mir_shell_path_present);
    assert!(inv.struct_ast_shell_skipped);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_struct_mir_shell_migrate_signed);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

