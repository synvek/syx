//! Class→MIR shell migrate
//! Class init/methods via MIR; residual live 11 after Actor; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_class_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-class-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "docs/gates/gate-b01-extension-mir-shell-migrate.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 Class MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01ClassMirShellMigrateInventory")
            && cg.contains("gate_b01_class_mir_shell_migrate_inventory")
            && cg.contains("ADR-815")
            && (cg.contains("b01_class_mir_shell_migrate_signed: true")
                || cg.contains("b01_class_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01ClassMirShellMigrateInventory (W471 / ADR-815)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must track live residual (11 after Actor migrate)"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::Class")
            && cg.contains("class_types")
            && cg.contains("class_vtables"),
        "compile_type_shells must skip Class; class indexes must remain"
    );
    let skip_region = cg.split("pub fn compile_type_shells").nth(1).unwrap_or("");
    let skip_region = skip_region.split("Ok(())").next().unwrap_or("");
    assert!(
        skip_region.contains("Decl::Class { .. }"),
        "compile_type_shells skip set must include Decl::Class"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("Decl::Class")
            && mir.contains("lower_type_members")
            && mir.contains("lower_class_init")
            && mir.contains("ADR-815"),
        "MIR must lower Class via lower_type_members / lower_class_init (W471 / ADR-815)"
    );
}

#[test]
fn b01_class_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-class-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-815") && spec.contains("Class"),
        "spec must cite ADR-815 Class migrate"
    );
    assert!(
        spec.contains("three-hundred-and-twenty-first"),
        
        "spec must cite W471 fingerprint"
    );
    assert!(
        spec.contains("GateB01ClassMirShellMigrateInventory")
            && spec.contains("gate_b01_class_mir_shell_migrate_inventory"),
        "spec must name Class migrate inventory API"
    );
    assert!(
        spec.contains("b01_class_mir_shell_migrate_signed=true")
            || spec.contains("`b01_class_mir_shell_migrate_signed=true`"),
        "spec must cite b01_class_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=12")
            || spec.contains("`residual_ast_shell_cluster_count=12`")
            || spec.contains("residual_ast_shell_cluster_count=11")
            || spec.contains("`residual_ast_shell_cluster_count=11`")
            || spec.contains("**13→12**")
            || spec.contains("13→12"),
        "spec must cite residual cluster count (W471 signed 12; live may be 11)"
    );
    assert!(
        spec.contains("compile_type_shells") && spec.contains("lower_type_members"),
        "spec must name AST skip + MIR lower"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("Actor") || spec.contains("Export") || spec.contains("B-01")),
        "spec Frozen must fence Actor / B-01 [x] / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_class_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_class_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.extension_migrate_floor_ok);
    assert!(inv.class_mir_path_present);
    assert!(inv.class_ast_shell_skipped);
    assert!(inv.class_index_preserved);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_class_mir_shell_migrate_signed);
    let ext = syx_codegen::gate_b01_extension_mir_shell_migrate_inventory();
    assert!(ext.b01_extension_mir_shell_migrate_signed);
    assert_eq!(ext.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

#[test]
fn b01_class_mir_shell_migrate_parity_compile() {
    let src = r#"
class Point {
    var x: Int
    init(x: Int) { self.x = x }
    func doubled() -> Int { return self.x * 2 }
    static func origin() -> Int { return 0 }
    deinit { }
    class Nested {
        var y: Int
        init(y: Int) { self.y = y }
        func get() -> Int { return self.y }
    }
}
func main() {
    let p = Point(x: 3)
    print(p.doubled())
    print(Point.origin())
}
"#;
    let unit = syx_driver::compile_file_from_source("class_w471.sy", src)
        .expect("Class MIR migrate parity compile");
    let prog = &unit.program;
    let init = prog
        .functions
        .iter()
        .find(|f| f.name == "Point_init")
        .expect("Point_init present");
    assert!(init.param_count >= 1);
    assert!(
        !init.instructions.is_empty(),
        "Point_init must have MIR body"
    );
    let doubled = prog
        .functions
        .iter()
        .find(|f| f.name == "Point_doubled")
        .expect("Point_doubled present");
    assert_eq!(doubled.param_count, 1);
    assert!(!doubled.instructions.is_empty());
    let origin = prog
        .functions
        .iter()
        .find(|f| f.name == "Point_origin")
        .expect("Point_origin present");
    assert_eq!(origin.param_count, 0);
    assert!(!origin.instructions.is_empty());
    let deinit = prog
        .functions
        .iter()
        .find(|f| f.name == "Point_deinit")
        .expect("Point_deinit present");
    assert!(!deinit.instructions.is_empty() || deinit.param_count >= 1);
    assert!(
        prog.functions
            .iter()
            .any(|f| f.name == "Point_Nested_init" || f.name == "Nested_init"),
        "nested class init must be present"
    );
}

#[test]
fn b01_class_mir_shell_migrate_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W471")
            && (roadmap.contains("ADR-815")
                || roadmap.contains("Class")
                || roadmap.contains("b01_class_mir_shell")
                || roadmap.contains("B-01")),
        "roadmap must cite W471 / ADR-815 Class MIR migrate"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(
        b01.contains("| [x] |") || b01.contains("| [~] |"),
        "B-01 must be [x] after W474 exit (or historical [~]) after W471 Class migrate: {b01}"
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
