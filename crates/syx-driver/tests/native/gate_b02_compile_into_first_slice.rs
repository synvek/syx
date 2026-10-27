//! compile_into first-slice
//! Driver prod compile_into* → MIR; ≠ retired / ≠ B-02 [x] / ≠ delete APIs.

#[test]
fn b02_compile_into_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-driver/src/lib.rs",
        "docs/gates/gate-b02-compile-into-first-slice.md",
        "docs/gates/gate-b02-retire-readiness-thin.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-02 compile_into first-slice requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB02CompileIntoFirstSliceInventory")
            && cg.contains("gate_b02_compile_into_first_slice_inventory")
            && cg.contains("ADR-821")
            && (cg.contains("b02_compile_into_first_slice_signed: true")
                || cg.contains("b02_compile_into_first_slice_signed:true")),
        "syx-codegen must expose GateB02CompileIntoFirstSliceInventory (W477 / ADR-821)"
    );
    // W482 honesty: live public APIs may be deleted; nail historical first-slice inventory only.
    assert!(
        cg.contains("fn compile_with_mir")
            && (cg.contains("compile_into_api_present: true")
                || cg.contains("compile_into_api_present:true")),
        "first-slice inventory historical floor: compile_into_api_present true"
    );
    assert!(
        (cg.contains("b02_dual_track_retired: false")
            || cg.contains("b02_dual_track_retired:false"))
            && (cg.contains("b02_axis_exit_enabled: false")
                || cg.contains("b02_axis_exit_enabled:false"))
            && (cg.contains("deprecate_gate_strict_enabled: false")
                || cg.contains("deprecate_gate_strict_enabled:false")),
        "inventory must keep retire/exit/gate flags false"
    );
    let driver = include_str!("../../src/lib.rs");
    assert!(
        driver.contains("compile_with_mir") && driver.contains("ADR-821"),
        "driver must cite ADR-821 MIR path"
    );
    assert!(
        !driver.contains("CodeGenerator::compile_into(")
            && !driver.contains("CodeGenerator::compile_into_with_semantic("),
        "driver production must not call compile_into*"
    );
}

#[test]
fn b02_compile_into_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b02-compile-into-first-slice.md");
    assert!(
        spec.contains("ADR-821") && (spec.contains("first-slice") || spec.contains("compile_into")),
        "spec must cite ADR-821 B-02 compile_into first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-twenty-seventh"),
        
        "spec must cite W477 fingerprint"
    );
    assert!(
        spec.contains("GateB02CompileIntoFirstSliceInventory")
            && spec.contains("gate_b02_compile_into_first_slice_inventory"),
        "spec must name first-slice inventory API"
    );
    assert!(
        spec.contains("b02_compile_into_first_slice_signed=true")
            || spec.contains("`b02_compile_into_first_slice_signed=true`"),
        "spec must cite b02_compile_into_first_slice_signed=true"
    );
    assert!(
        spec.contains("compile_into_production_call_sites_zero=true")
            || spec.contains("`compile_into_production_call_sites_zero=true`"),
        "spec must cite compile_into_production_call_sites_zero=true"
    );
    assert!(
        spec.contains("b02_dual_track_retired=false")
            || spec.contains("`b02_dual_track_retired=false`"),
        "spec must cite b02_dual_track_retired=false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-02") || spec.contains("compile") || spec.contains("Export")),
        "spec Frozen must fence B-02 [x] / delete compile / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b02_compile_into_first_slice_inventory_nails() {
    let inv = syx_codegen::gate_b02_compile_into_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.retire_readiness_floor_ok);
    assert!(inv.stdlib_mono_compile_into_migrated);
    assert!(inv.compile_to_program_ast_else_cleared);
    assert!(inv.compile_into_production_call_sites_zero);
    assert!(inv.compile_into_api_present);
    assert!(!inv.b02_dual_track_retired);
    assert!(!inv.b02_axis_exit_enabled);
    assert!(!inv.deprecate_gate_strict_enabled);
    assert!(inv.b02_compile_into_first_slice_signed);
    let readiness = syx_codegen::gate_b02_retire_readiness_thin_inventory();
    assert!(readiness.b02_retire_readiness_signed);
    assert!(!readiness.b02_dual_track_retired);
}

#[test]
fn b02_compile_into_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W477")
            && (roadmap.contains("ADR-821")
                || roadmap.contains("compile_into")
                || roadmap.contains("first-slice")
                || roadmap.contains("first_slice")),
        "roadmap must cite W477 / ADR-821 B-02 compile_into first-slice"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(
        b02.contains("| [~] |") || b02.contains("| [x] |"),
        "B-02 must remain [~] or [x] after exit: {b02}"
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
