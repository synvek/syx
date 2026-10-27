//! Math residual maintenance first-slice (W953 / ADR-1297).
//! Rollup inventory nails W952 platform floor + Math W902–W922 closed;
//! math_residual_maintenance_ga=false; ≠ llvm Math GA / ≠ F-08 / ≠ HostFn+1.

#[test]
fn math_residual_maintenance_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-driver/src/lib.rs",
        "packages/core/math.sy",
        "docs/lang/math-residual-maintenance-first-slice.md",
        "docs/lang/platform-residual-maintenance-first-slice.md",
        "docs/lang/math-f32-methods.md",
        "docs/lang/math-f64-methods.md",
        "plan/topics/ga-evidence-audit.md",
        "docs/9_UsageGuide.md",
        "plan/02-improvement-roadmap.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Math residual maintenance requires evidence file: {path}"
        );
    }
    let driver = include_str!("../../src/lib.rs");
    assert!(
        driver.contains("SyxMathResidualMaintenanceFirstSliceInventory")
            && driver.contains("syx_math_residual_maintenance_first_slice_inventory")
            && driver.contains("ADR-1297")
            && driver.contains("math_track_closed")
            && (driver.contains("syx_math_residual_maintenance_first_slice_signed: true")
                || driver.contains("syx_math_residual_maintenance_first_slice_signed:true")),
        "syx-driver must expose SyxMathResidualMaintenanceFirstSliceInventory (W953 / ADR-1297)"
    );
    assert!(
        (driver.contains("math_residual_maintenance_ga: false")
            || driver.contains("math_residual_maintenance_ga:false"))
            && driver.contains("W953"),
        "inventory must keep math_residual_maintenance_ga false (W953)"
    );
}

#[test]
fn math_residual_maintenance_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/lang/math-residual-maintenance-first-slice.md");
    assert!(
        spec.contains("ADR-1297")
            && (spec.contains("Math residual") || spec.contains("math residual")),
        "spec must cite ADR-1297 Math residual maintenance"
    );
    assert!(
        spec.contains("eight-hundred-and-third"),
        
        "spec must cite W953 fingerprint"
    );
    assert!(
        spec.contains("SyxMathResidualMaintenanceFirstSliceInventory")
            && spec.contains("syx_math_residual_maintenance_first_slice_inventory"),
        "spec must name Math maintenance inventory API"
    );
    assert!(
        spec.contains("syx_math_residual_maintenance_first_slice_signed=true")
            || spec.contains("`syx_math_residual_maintenance_first_slice_signed=true`"),
        "spec must cite Math signed=true"
    );
    assert!(
        spec.contains("math_residual_maintenance_ga=false")
            || spec.contains("`math_residual_maintenance_ga=false`"),
        "spec must cite math_residual_maintenance_ga=false"
    );
    assert!(
        spec.contains("W902") && spec.contains("W922"),
        "spec must cite Math track W902–W922"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("llvm")
                || spec.contains("F-08")
                || spec.contains("HostFn")
                || spec.contains("QS1")),
        "spec Frozen must fence llvm Math GA / F-08 / HostFn / QS1"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn math_residual_maintenance_first_slice_inventory_nails() {
    let inv = syx_driver::syx_math_residual_maintenance_first_slice_inventory();
    assert!(inv.platform_maintenance_floor_ok);
    assert!(inv.math_track_closed);
    assert!(inv.math_entries_present);
    assert!(!inv.math_residual_maintenance_ga);
    assert!(inv.syx_math_residual_maintenance_first_slice_signed);
    let platform = syx_driver::syx_platform_residual_maintenance_first_slice_inventory();
    assert!(platform.syx_platform_residual_maintenance_first_slice_signed);
    assert!(!platform.platform_residual_maintenance_ga);
}

#[test]
fn math_residual_maintenance_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W953")
            && (roadmap.contains("ADR-1297")
                || roadmap.contains("Math residual")
                || roadmap.contains("Math 维护")),
        "roadmap must cite W953 / ADR-1297 Math residual maintenance"
    );
    for id in ["C-06", "C-07", "C-08", "C-09", "C-11", "F-07"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("[x]"), "roadmap {id} must remain [x]: {row}");
    }
    let f08 = roadmap
        .lines()
        .find(|l| l.starts_with("| F-08 |"))
        .expect("F-08 row");
    assert!(
        f08.contains("[ ]"),
        "roadmap F-08 must remain frozen [ ]: {f08}"
    );
    assert!(
        !f08.contains("[x]"),
        "roadmap F-08 must not flip to [x]: {f08}"
    );
}
