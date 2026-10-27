//! compile API delete
//! Delete public compile/compile_into*; retired=true; ≠ auto-discovery / ≠ B-09 [x] / ≠ private helper purge.

#[test]
fn b02_compile_api_delete_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "docs/gates/gate-b02-compile-api-delete.md",
        "plan/archive/gates/gate-b02-dual-track-exit.md",
        "docs/gates/gate-b02-deprecate-gate-strict.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-02 compile API delete requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB02CompileApiDeleteInventory")
            && cg.contains("gate_b02_compile_api_delete_inventory")
            && cg.contains("ADR-826")
            && (cg.contains("b02_compile_api_delete_signed: true")
                || cg.contains("b02_compile_api_delete_signed:true")),
        "syx-codegen must expose GateB02CompileApiDeleteInventory (W482 / ADR-826)"
    );
    let banned_compile = format!("{}{}", "pub fn compile", "(module: &Module)");
    let banned_into = format!("{}{}", "pub fn compile_into", "(module: &Module");
    let banned_sem = format!("{}{}", "pub fn compile_into_with_semantic", "(");
    assert!(
        cg.contains("fn compile_with_mir")
            && !cg.contains(&banned_compile)
            && !cg.contains(&banned_into)
            && !cg.contains(&banned_sem),
        "public compile/compile_into* must be deleted; compile_with_mir retained"
    );
    assert!(
        (cg.contains("api_delete_enabled: true") || cg.contains("api_delete_enabled:true"))
            && (cg.contains("compile_into_api_present: false")
                || cg.contains("compile_into_api_present:false"))
            && (cg.contains("b02_dual_track_retired: true")
                || cg.contains("b02_dual_track_retired:true")),
        "delete inventory must set api_delete true / api_present false / retired true"
    );
    // Honesty floor: W479 exit inventory historical values remain in source.
    assert!(
        (cg.contains("api_delete_enabled: false") || cg.contains("api_delete_enabled:false"))
            && (cg.contains("b02_dual_track_exit_signed: true")
                || cg.contains("b02_dual_track_exit_signed:true")),
        "W479 exit inventory historical api_delete=false floor must remain"
    );
}

#[test]
fn b02_compile_api_delete_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b02-compile-api-delete.md");
    assert!(
        spec.contains("ADR-826") && (spec.contains("delete") || spec.contains("api_delete")),
        "spec must cite ADR-826 B-02 compile API delete"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-second"),
        
        "spec must cite W482 fingerprint"
    );
    assert!(
        spec.contains("GateB02CompileApiDeleteInventory")
            && spec.contains("gate_b02_compile_api_delete_inventory"),
        "spec must name delete inventory API"
    );
    assert!(
        spec.contains("b02_compile_api_delete_signed=true")
            || spec.contains("`b02_compile_api_delete_signed=true`"),
        "spec must cite b02_compile_api_delete_signed=true"
    );
    assert!(
        (spec.contains("api_delete_enabled=true") || spec.contains("`api_delete_enabled=true`"))
            && (spec.contains("compile_into_api_present=false")
                || spec.contains("`compile_into_api_present=false`"))
            && (spec.contains("b02_dual_track_retired=true")
                || spec.contains("`b02_dual_track_retired=true`")),
        "spec must cite delete/retired true and api_present false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("auto-discovery")
                || spec.contains("B-09")
                || spec.contains("helper")),
        "spec Frozen must fence auto-discovery / B-09 / private helper purge"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b02_compile_api_delete_inventory_nails() {
    let inv = syx_codegen::gate_b02_compile_api_delete_inventory();
    assert!(inv.dual_track_exit_floor_ok);
    assert!(!inv.compile_into_api_present);
    assert!(inv.api_delete_enabled);
    assert!(inv.b02_dual_track_retired);
    assert!(inv.compile_with_mir_present);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(inv.b02_compile_api_delete_signed);
    // Honesty floors: W479 / W478 / W477 historical values.
    let exit = syx_codegen::gate_b02_dual_track_horizon_exit_inventory();
    assert!(exit.compile_into_api_present);
    assert!(!exit.api_delete_enabled);
    assert!(!exit.b02_dual_track_retired);
    assert!(exit.b02_dual_track_exit_signed);
    let gate = syx_codegen::gate_b02_deprecate_gate_strict_inventory();
    assert!(gate.compile_into_api_present);
    assert!(!gate.b02_dual_track_retired);
    let first = syx_codegen::gate_b02_compile_into_first_slice_inventory();
    assert!(first.compile_into_api_present);
    assert!(!first.b02_dual_track_retired);
}

#[test]
fn b02_compile_api_delete_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W482")
            && (roadmap.contains("ADR-826")
                || roadmap.contains("api_delete")
                || roadmap.contains("compile_into")
                || roadmap.contains("删")),
        "roadmap must cite W482 / ADR-826 B-02 compile API delete"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(
        b02.contains("| [x] |"),
        "B-02 must remain [x] after W482: {b02}"
    );
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("| [x] |") || b09.contains("| [~] |") || b09.contains("| [ ] |"),
        "B-09 must be [x] (W558+) or historical [~] (or historical [ ]): {b09}"
    );
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-01 |"))
        .expect("B-01 row");
    assert!(b01.contains("| [x] |"), "B-01 must remain [x]: {b01}");
    for id in ["B-03", "B-04", "B-05", "B-06", "B-07", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} must remain [x]: {row}");
    }
}
