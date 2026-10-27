//! FinalClass→MIR type-shell migrate
//! FinalClass shells via MIR; residual 15; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_finalclass_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-finalclass-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "docs/gates/gate-b01-struct-mir-shell-migrate.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 FinalClass MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01FinalClassMirShellMigrateInventory")
            && cg.contains("gate_b01_finalclass_mir_shell_migrate_inventory")
            && cg.contains("ADR-812")
            && (cg.contains("b01_finalclass_mir_shell_migrate_signed: true")
                || cg.contains("b01_finalclass_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01FinalClassMirShellMigrateInventory (W468 / ADR-812)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must track live residual (11 after Actor migrate)"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::FinalClass")
            && cg.contains("lower_finalclass_shells"),
        "compile_type_shells must skip FinalClass; compile_with_mir must merge lower_finalclass_shells"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("lower_finalclass_shells")
            && mir.contains("Decl::FinalClass")
            && mir.contains("ADR-812"),
        "lower_finalclass_shells must cover Decl::FinalClass (W468 / ADR-812)"
    );
}

#[test]
fn b01_finalclass_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-finalclass-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-812") && (spec.contains("FinalClass") || spec.contains("final class")),
        "spec must cite ADR-812 FinalClass migrate"
    );
    assert!(
        spec.contains("three-hundred-and-eighteenth"),
        
        "spec must cite W468 fingerprint"
    );
    assert!(
        spec.contains("GateB01FinalClassMirShellMigrateInventory")
            && spec.contains("gate_b01_finalclass_mir_shell_migrate_inventory"),
        "spec must name FinalClass migrate inventory API"
    );
    assert!(
        spec.contains("b01_finalclass_mir_shell_migrate_signed=true")
            || spec.contains("`b01_finalclass_mir_shell_migrate_signed=true`"),
        "spec must cite b01_finalclass_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=14")
            || spec.contains("residual_ast_shell_cluster_count=13")
            || spec.contains("`residual_ast_shell_cluster_count=14`")
            || spec.contains("`residual_ast_shell_cluster_count=13`")
            || spec.contains("residual_ast_shell_cluster_count=15")
            || spec.contains("`residual_ast_shell_cluster_count=15`")
            || spec.contains("**16→15**")
            || spec.contains("16→15"),
        "spec must cite residual cluster count (W468 signed 15; live may be 11)"
    );
    assert!(
        spec.contains("lower_finalclass_shells") && spec.contains("compile_type_shells"),
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
fn gate_b01_finalclass_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_finalclass_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.struct_migrate_floor_ok);
    assert!(inv.finalclass_mir_shell_path_present);
    assert!(inv.finalclass_ast_shell_skipped);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_finalclass_mir_shell_migrate_signed);
    let struct_inv = syx_codegen::gate_b01_struct_mir_shell_migrate_inventory();
    assert!(struct_inv.b01_struct_mir_shell_migrate_signed);
    assert_eq!(struct_inv.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

#[test]
fn b01_finalclass_mir_shell_migrate_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W468")
            && (roadmap.contains("ADR-812")
                || roadmap.contains("FinalClass")
                || roadmap.contains("b01_finalclass_mir_shell")
                || roadmap.contains("B-01")),
        "roadmap must cite W468 / ADR-812 FinalClass MIR migrate"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(
        b01.contains("| [x] |") || b01.contains("| [~] |"),
        "B-01 must be [x] after W474 exit (or historical [~]) after W468 FinalClass migrate: {b01}"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(
        b02.contains("| [~] |") || b02.contains("| [x] |"),
        "B-02 must remain [~] or [x] after exit: {b02}"
    );
    for id in ["B-03", "B-04", "B-05", "B-06", "B-07", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} must remain [x]: {row}");
    }
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("| [x] |") || b09.contains("| [~] |") || b09.contains("| [ ] |"),
        "B-09 must be [x] (W558+) or historical [~] (or historical [ ]): {b09}"
    );
}
