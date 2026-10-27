//! Actor→MIR shell migrate
//! Actor init+ActorRegister/methods via MIR; residual 11; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_actor_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-actor-mir-shell-migrate.md",
        "docs/gates/gate-b01-class-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 Actor MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01ActorMirShellMigrateInventory")
            && cg.contains("gate_b01_actor_mir_shell_migrate_inventory")
            && cg.contains("ADR-816")
            && (cg.contains("b01_actor_mir_shell_migrate_signed: true")
                || cg.contains("b01_actor_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01ActorMirShellMigrateInventory (W472 / ADR-816)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must be 11 after Actor migrate"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::Actor")
            && cg.contains("actor_types")
            && cg.contains("class_types"),
        "compile_type_shells must skip Actor; actor/class indexes must remain"
    );
    let skip_region = cg.split("pub fn compile_type_shells").nth(1).unwrap_or("");
    let skip_region = skip_region.split("Ok(())").next().unwrap_or("");
    assert!(
        skip_region.contains("Decl::Actor { .. }"),
        "compile_type_shells skip set must include Decl::Actor"
    );
    assert!(
        !skip_region.contains("Decl::GlobalActor"),
        "compile_type_shells must NOT skip Decl::GlobalActor this window"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("Decl::Actor")
            && mir.contains("lower_type_members")
            && mir.contains("lower_class_init")
            && mir.contains("ActorRegister")
            && mir.contains("ADR-816"),
        "MIR must lower Actor via lower_type_members / lower_class_init / ActorRegister (W472 / ADR-816)"
    );
}

#[test]
fn b01_actor_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-actor-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-816") && spec.contains("Actor"),
        "spec must cite ADR-816 Actor migrate"
    );
    assert!(
        spec.contains("three-hundred-and-twenty-second"),
        
        "spec must cite W472 fingerprint"
    );
    assert!(
        spec.contains("GateB01ActorMirShellMigrateInventory")
            && spec.contains("gate_b01_actor_mir_shell_migrate_inventory"),
        "spec must name Actor migrate inventory API"
    );
    assert!(
        spec.contains("b01_actor_mir_shell_migrate_signed=true")
            || spec.contains("`b01_actor_mir_shell_migrate_signed=true`"),
        "spec must cite b01_actor_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=11")
            || spec.contains("`residual_ast_shell_cluster_count=11`")
            || spec.contains("**12→11**")
            || spec.contains("12→11"),
        "spec must cite residual cluster count 11"
    );
    assert!(
        spec.contains("compile_type_shells")
            && spec.contains("lower_type_members")
            && spec.contains("ActorRegister"),
        "spec must name AST skip + MIR lower + ActorRegister"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-01") || spec.contains("Export") || spec.contains("GlobalActor")),
        "spec Frozen must fence B-01 [x] / Export / GlobalActor"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_actor_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_actor_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.class_migrate_floor_ok);
    assert!(inv.actor_mir_path_present);
    assert!(inv.actor_ast_shell_skipped);
    assert!(inv.actor_index_preserved);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_actor_mir_shell_migrate_signed);
    let class = syx_codegen::gate_b01_class_mir_shell_migrate_inventory();
    assert!(class.b01_class_mir_shell_migrate_signed);
    assert_eq!(class.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

#[test]
fn b01_actor_mir_shell_migrate_parity_compile() {
    let src = r#"
actor Counter {
    var n: Int
    init(n: Int) { self.n = n }
    func bump() -> Int { return self.n + 1 }
    static func zero() -> Int { return 0 }
    deinit { }
}
func main() {
    let c = Counter(n: 3)
    print(c.bump())
}
"#;
    let unit = syx_driver::compile_file_from_source("actor_w472.sy", src)
        .expect("Actor MIR migrate parity compile");
    let prog = &unit.program;
    let init = prog
        .functions
        .iter()
        .find(|f| f.name == "Counter_init")
        .expect("Counter_init present");
    assert!(init.param_count >= 1);
    assert!(
        !init.instructions.is_empty(),
        "Counter_init must have MIR body"
    );
    assert!(
        init.instructions
            .iter()
            .any(|i| matches!(i, syx_codegen::Instruction::ActorRegister)),
        "Counter_init must contain ActorRegister bytecode"
    );
    let bump = prog
        .functions
        .iter()
        .find(|f| f.name == "Counter_bump")
        .expect("Counter_bump present");
    assert_eq!(bump.param_count, 1);
    assert!(!bump.instructions.is_empty());
    let zero = prog
        .functions
        .iter()
        .find(|f| f.name == "Counter_zero")
        .expect("Counter_zero present");
    assert_eq!(zero.param_count, 0);
    assert!(!zero.instructions.is_empty());
    let deinit = prog
        .functions
        .iter()
        .find(|f| f.name == "Counter_deinit")
        .expect("Counter_deinit present");
    assert!(!deinit.instructions.is_empty() || deinit.param_count >= 1);
}

#[test]
fn b01_actor_mir_shell_migrate_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W472")
            && (roadmap.contains("ADR-816")
                || roadmap.contains("Actor")
                || roadmap.contains("b01_actor_mir_shell")
                || roadmap.contains("B-01")),
        "roadmap must cite W472 / ADR-816 Actor MIR migrate"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(
        b01.contains("| [x] |") || b01.contains("| [~] |"),
        "B-01 must be [x] after W474 exit (or historical [~]) after W472 Actor migrate: {b01}"
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
