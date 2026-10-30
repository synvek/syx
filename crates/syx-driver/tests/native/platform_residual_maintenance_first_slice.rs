//! Platform residual maintenance first-slice (W952 / ADR-1296).
//! Rollup inventory nails W951 C-09 maintenance floor + Wave C/F honesty;
//! platform_residual_maintenance_ga=false; ≠ F-08 / ≠ Wave D/E / ≠ HostFn+1.

#[test]
fn platform_residual_maintenance_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-driver/src/lib.rs",
        ".github/workflows/ci.yml",
        "docs/lang/platform-residual-maintenance-first-slice.md",
        "docs/lang/c09-syx-residual-maintenance-first-slice.md",
        "plan/topics/syx-fuzz.md",
        "plan/topics/syx-property.md",
        "plan/topics/ga-evidence-audit.md",
        "docs/9_UsageGuide.md",
        "plan/02-improvement-roadmap.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "platform residual maintenance requires evidence file: {path}"
        );
    }
    let driver = include_str!("../../src/lib.rs");
    assert!(
        driver.contains("SyxPlatformResidualMaintenanceFirstSliceInventory")
            && driver.contains("syx_platform_residual_maintenance_first_slice_inventory")
            && driver.contains("ADR-1296")
            && driver.contains("wave_cf_floor_ok")
            && (driver.contains("syx_platform_residual_maintenance_first_slice_signed: true")
                || driver.contains("syx_platform_residual_maintenance_first_slice_signed:true")),
        "syx-driver must expose SyxPlatformResidualMaintenanceFirstSliceInventory (W952 / ADR-1296)"
    );
    assert!(
        (driver.contains("platform_residual_maintenance_ga: false")
            || driver.contains("platform_residual_maintenance_ga:false"))
            && driver.contains("W952"),
        "inventory must keep platform_residual_maintenance_ga false (W952)"
    );
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("property_smoke")
            && ci.contains("syx_smoke")
            && ci.contains("-runs=1"),
        "ci.yml must still nail property_smoke and bounded syx_smoke"
    );
}

#[test]
fn platform_residual_maintenance_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/lang/platform-residual-maintenance-first-slice.md");
    assert!(
        spec.contains("ADR-1296")
            && (spec.contains("platform residual") || spec.contains("Platform residual")),
        "spec must cite ADR-1296 platform residual maintenance"
    );
    assert!(
        spec.contains("eight-hundred-and-second"),
        
        "spec must cite W952 fingerprint"
    );
    assert!(
        spec.contains("SyxPlatformResidualMaintenanceFirstSliceInventory")
            && spec.contains("syx_platform_residual_maintenance_first_slice_inventory"),
        "spec must name platform maintenance inventory API"
    );
    assert!(
        spec.contains("syx_platform_residual_maintenance_first_slice_signed=true")
            || spec.contains("`syx_platform_residual_maintenance_first_slice_signed=true`"),
        "spec must cite platform signed=true"
    );
    assert!(
        spec.contains("platform_residual_maintenance_ga=false")
            || spec.contains("`platform_residual_maintenance_ga=false`"),
        "spec must cite platform_residual_maintenance_ga=false"
    );
    assert!(
        (spec.contains("[x]") || spec.contains("`[x]`"))
            && (spec.contains("Wave C") || spec.contains("C-09") || spec.contains("11/11")),
        "spec must cite Wave C / C-09 honesty"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("F-08")
                || spec.contains("Wave D")
                || spec.contains("HostFn")
                || spec.contains("*_ga")),
        "spec Frozen must fence F-08 / D/E / HostFn / *_ga"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn platform_residual_maintenance_first_slice_inventory_nails() {
    let inv = syx_driver::syx_platform_residual_maintenance_first_slice_inventory();
    assert!(inv.c09_maintenance_floor_ok);
    assert!(inv.wave_cf_floor_ok);
    assert!(inv.platform_entries_present);
    assert!(!inv.platform_residual_maintenance_ga);
    assert!(inv.syx_platform_residual_maintenance_first_slice_signed);
    let c09 = syx_driver::syx_c09_residual_maintenance_first_slice_inventory();
    assert!(c09.syx_c09_residual_maintenance_first_slice_signed);
    assert!(!c09.c09_residual_maintenance_ga);
}

