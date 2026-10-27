//! no-op residual honesty
//! active=0 + residual 11 all compile_decl no-op; ≠ B-01 [x] / ≠ count→0 / ≠ Decl::Export.

#[test]
fn b01_noop_residual_honesty_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "docs/gates/gate-b01-noop-residual-honesty.md",
        "docs/gates/gate-b01-actor-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 no-op residual honesty requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01NoopResidualHonestyInventory")
            && cg.contains("gate_b01_noop_residual_honesty_inventory")
            && cg.contains("ADR-817")
            && (cg.contains("b01_noop_residual_honesty_signed: true")
                || cg.contains("b01_noop_residual_honesty_signed:true")),
        "syx-codegen must expose GateB01NoopResidualHonestyInventory (W473 / ADR-817)"
    );
    let residual_const = cg
        .lines()
        .find(|l| {
            let t = l.trim_start();
            t.starts_with("pub const B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT")
        })
        .expect("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT const present");
    assert!(
        residual_const.contains("= 11"),
        "residual cluster count const must be 11, got: {residual_const}"
    );
    assert!(
        cg.contains("fn compile_decl") && cg.contains("_ => {}"),
        "compile_decl must retain _ => {{}} no-op residual arms"
    );
    let skip_region = cg.split("pub fn compile_type_shells").nth(1).unwrap_or("");
    let skip_region = skip_region.split("Ok(())").next().unwrap_or("");
    assert!(
        skip_region.contains("Decl::Actor { .. }"),
        "Actor must remain skipped (active shells cleared)"
    );
}

#[test]
fn b01_noop_residual_honesty_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-noop-residual-honesty.md");
    assert!(
        spec.contains("ADR-817")
            && (spec.contains("no-op") || spec.contains("noop") || spec.contains("No-op")),
        "spec must cite ADR-817 no-op residual honesty"
    );
    assert!(
        spec.contains("three-hundred-and-twenty-third"),
        
        "spec must cite W473 fingerprint"
    );
    assert!(
        spec.contains("GateB01NoopResidualHonestyInventory")
            && spec.contains("gate_b01_noop_residual_honesty_inventory"),
        "spec must name no-op honesty inventory API"
    );
    assert!(
        spec.contains("b01_noop_residual_honesty_signed=true")
            || spec.contains("`b01_noop_residual_honesty_signed=true`"),
        "spec must cite b01_noop_residual_honesty_signed=true"
    );
    assert!(
        spec.contains("active_ast_type_shell_count=0")
            || spec.contains("`active_ast_type_shell_count=0`")
            || spec.contains("active=0"),
        "spec must cite active_ast_type_shell_count=0"
    );
    assert!(
        spec.contains("all_residual_compile_decl_noop=true")
            || spec.contains("`all_residual_compile_decl_noop=true`"),
        "spec must cite all_residual_compile_decl_noop=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=11")
            || spec.contains("`residual_ast_shell_cluster_count=11`"),
        "spec must cite residual cluster count 11"
    );
    for cluster in [
        "Protocol",
        "GlobalActor",
        "TypeAlias",
        "OperatorFunc",
        "Import",
        "MacroDecl",
        "MacroSiteInvoke",
        "TopLevelStmt",
        "Init",
        "Deinit",
        "Subscript",
    ] {
        assert!(
            spec.contains(cluster),
            "spec residual no-op table must name cluster: {cluster}"
        );
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-01") || spec.contains("Export") || spec.contains("假迁")),
        "spec Frozen must fence B-01 [x] / fake migrate / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_noop_residual_honesty_inventory_nails() {
    let inv = syx_codegen::gate_b01_noop_residual_honesty_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.actor_migrate_floor_ok);
    assert_eq!(inv.active_ast_type_shell_count, 0);
    assert!(inv.all_residual_compile_decl_noop);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(inv.noop_cluster_names_documented);
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_noop_residual_honesty_signed);
    let actor = syx_codegen::gate_b01_actor_mir_shell_migrate_inventory();
    assert!(actor.b01_actor_mir_shell_migrate_signed);
    assert_eq!(actor.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

#[test]
fn b01_noop_residual_honesty_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W473")
            && (roadmap.contains("ADR-817")
                || roadmap.contains("no-op")
                || roadmap.contains("noop")
                || roadmap.contains("b01_noop_residual")
                || roadmap.contains("B-01")),
        "roadmap must cite W473 / ADR-817 no-op residual honesty"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(
        b01.contains("| [x] |"),
        "B-01 must be [x] after W474 narrow exit (honesty floor still holds): {b01}"
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
