//! Extension→MIR shell migrate
//! Extension methods via MIR; residual 13; ≠ B-01 [x] / ≠ Decl::Export / ≠ daemon GA.

#[test]
fn b01_extension_mir_shell_migrate_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b01-extension-mir-shell-migrate.md",
        "docs/gates/gate-b01-mir-consumer-deepen-thin.md",
        "docs/gates/gate-b01-varglobal-mir-shell-migrate.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-01 Extension MIR migrate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB01ExtensionMirShellMigrateInventory")
            && cg.contains("gate_b01_extension_mir_shell_migrate_inventory")
            && cg.contains("ADR-814")
            && (cg.contains("b01_extension_mir_shell_migrate_signed: true")
                || cg.contains("b01_extension_mir_shell_migrate_signed:true")),
        "syx-codegen must expose GateB01ExtensionMirShellMigrateInventory (W470 / ADR-814)"
    );
    assert!(
        cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT: usize = 11")
            || cg.contains("B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT = 11"),
        "residual cluster count must track live residual (11 after Actor migrate)"
    );
    assert!(
        cg.contains("compile_type_shells")
            && cg.contains("Decl::Extension")
            && cg.contains("extension_methods"),
        "compile_type_shells must skip Extension; extension_methods index must remain"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("Decl::Extension")
            && mir.contains("lower_type_members")
            && mir.contains("ADR-814"),
        "MIR must lower Extension via lower_type_members (W470 / ADR-814)"
    );
}

#[test]
fn b01_extension_mir_shell_migrate_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b01-extension-mir-shell-migrate.md");
    assert!(
        spec.contains("ADR-814") && spec.contains("Extension"),
        "spec must cite ADR-814 Extension migrate"
    );
    assert!(
        spec.contains("three-hundred-and-twentieth"),
        
        "spec must cite W470 fingerprint"
    );
    assert!(
        spec.contains("GateB01ExtensionMirShellMigrateInventory")
            && spec.contains("gate_b01_extension_mir_shell_migrate_inventory"),
        "spec must name Extension migrate inventory API"
    );
    assert!(
        spec.contains("b01_extension_mir_shell_migrate_signed=true")
            || spec.contains("`b01_extension_mir_shell_migrate_signed=true`"),
        "spec must cite b01_extension_mir_shell_migrate_signed=true"
    );
    assert!(
        spec.contains("residual_ast_shell_cluster_count=13")
            || spec.contains("residual_ast_shell_cluster_count=11")
            || spec.contains("`residual_ast_shell_cluster_count=13`")
            || spec.contains("`residual_ast_shell_cluster_count=11`")
            || spec.contains("**14→13**")
            || spec.contains("14→13"),
        "spec must cite residual cluster count (W470 signed 13; live may be 11)"
    );
    assert!(
        spec.contains("compile_type_shells")
            && spec.contains("lower_type_members")
            && spec.contains("extension_methods"),
        "spec must name AST skip + MIR lower + index preserve"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-01") || spec.contains("Export") || spec.contains("Class")),
        "spec Frozen must fence B-01 [x] / Class / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn gate_b01_extension_mir_shell_migrate_inventory_nails() {
    let inv = syx_codegen::gate_b01_extension_mir_shell_migrate_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.varglobal_migrate_floor_ok);
    assert!(inv.extension_mir_path_present);
    assert!(inv.extension_ast_shell_skipped);
    assert!(inv.extension_methods_index_preserved);
    assert_eq!(inv.residual_ast_shell_cluster_count, 11);
    assert_eq!(
        inv.residual_ast_shell_cluster_count,
        syx_codegen::B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT
    );
    assert!(!inv.ast_codegen_migration_complete);
    assert!(!inv.b01_axis_exit_enabled);
    assert!(!inv.type_shell_delete_enabled);
    assert!(inv.b01_extension_mir_shell_migrate_signed);
    let vg = syx_codegen::gate_b01_varglobal_mir_shell_migrate_inventory();
    assert!(vg.b01_varglobal_mir_shell_migrate_signed);
    assert_eq!(vg.residual_ast_shell_cluster_count, 11);
    let deepen = syx_codegen::gate_b01_mir_consumer_deepen_thin_inventory();
    assert!(deepen.b01_mir_consumer_deepen_thin_signed);
    assert_eq!(deepen.residual_ast_shell_cluster_count, 11);
    assert!(!deepen.b01_axis_exit_enabled);
}

#[test]
fn b01_extension_mir_shell_migrate_parity_compile() {
    // Instance (self), static (no self), builtin target — MIR path after AST skip.
    let src = r#"
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
extension Box {
    func doubled() -> Int { return self.n * 2 }
    static func zero() -> Int { return 0 }
}
extension Int {
    func plusOne() -> Int { return self + 1 }
}
func main() {
    let b = Box(n: 3)
    print(b.doubled())
    print(Box.zero())
    print(1.plusOne())
}
"#;
    let unit = syx_driver::compile_file_from_source("ext_w470.sy", src)
        .expect("Extension MIR migrate parity compile");
    let prog = &unit.program;
    let doubled = prog
        .functions
        .iter()
        .find(|f| f.name == "Box_doubled")
        .expect("Box_doubled present");
    assert_eq!(doubled.param_count, 1);
    assert!(
        !doubled.instructions.is_empty(),
        "Box_doubled must have MIR body"
    );
    let zero = prog
        .functions
        .iter()
        .find(|f| f.name == "Box_zero")
        .expect("Box_zero present");
    assert_eq!(zero.param_count, 0);
    assert!(!zero.instructions.is_empty(), "Box_zero must have MIR body");
    let plus = prog
        .functions
        .iter()
        .find(|f| f.name == "Int_plusOne")
        .expect("Int_plusOne present");
    assert_eq!(plus.param_count, 1);
    assert!(
        !plus.instructions.is_empty(),
        "Int_plusOne must have MIR body"
    );
}

#[test]
fn b01_extension_mir_shell_migrate_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W470")
            && (roadmap.contains("ADR-814")
                || roadmap.contains("Extension")
                || roadmap.contains("b01_extension_mir_shell")
                || roadmap.contains("B-01")),
        "roadmap must cite W470 / ADR-814 Extension MIR migrate"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(
        b01.contains("| [x] |") || b01.contains("| [~] |"),
        "B-01 must be [x] after W474 exit (or historical [~]) after W470 Extension migrate: {b01}"
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
