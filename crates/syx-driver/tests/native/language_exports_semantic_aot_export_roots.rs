//! Language exports semantic→aot_export_roots (W481 / ADR-825).
//! lower_module fills roots from Decl::Export; ≠ auto-discovery / ≠ exports GA / ≠ delete compile API.

#[test]
fn language_exports_semantic_aot_export_roots_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/lang/language-exports-semantic-aot-export-roots.md",
        "docs/lang/language-exports-decl-export-first-slice.md",
        "docs/lang/language-exports-aot-roots-thin.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "semantic→aot_export_roots requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsSemanticAotExportRootsInventory")
            && mir.contains("language_exports_semantic_aot_export_roots_inventory")
            && mir.contains("ADR-825")
            && (mir.contains("language_exports_semantic_aot_export_roots_signed: true")
                || mir.contains("language_exports_semantic_aot_export_roots_signed:true")),
        "syx-mir must expose LanguageExportsSemanticAotExportRootsInventory (W481 / ADR-825)"
    );
    assert!(
        mir.contains("semantic_fills_aot_export_roots: true")
            || mir.contains("semantic_fills_aot_export_roots:true"),
        "semantic inventory must set semantic_fills_aot_export_roots=true"
    );
    assert!(
        (mir.contains("language_exports_auto_discovery_enabled: false")
            || mir.contains("language_exports_auto_discovery_enabled:false"))
            && (mir.contains("exports_ga_claimed: false")
                || mir.contains("exports_ga_claimed:false")),
        "semantic inventory must keep auto/GA false"
    );
    assert!(
        mir.contains("is_export") && mir.contains("aot_export_roots.push") && mir.contains("W481"),
        "lower_module must fill aot_export_roots from Decl::Export (W481)"
    );
}

#[test]
fn language_exports_semantic_aot_export_roots_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-semantic-aot-export-roots.md");
    assert!(
        spec.contains("ADR-825")
            && (spec.contains("aot_export_roots") || spec.contains("semantic")),
        "spec must cite ADR-825 semantic→aot_export_roots"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-first"),
        
        "spec must cite W481 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsSemanticAotExportRootsInventory")
            && spec.contains("language_exports_semantic_aot_export_roots_inventory"),
        "spec must name semantic→roots inventory API"
    );
    assert!(
        spec.contains("language_exports_semantic_aot_export_roots_signed=true")
            || spec.contains("`language_exports_semantic_aot_export_roots_signed=true`"),
        "spec must cite language_exports_semantic_aot_export_roots_signed=true"
    );
    assert!(
        spec.contains("semantic_fills_aot_export_roots=true")
            || spec.contains("`semantic_fills_aot_export_roots=true`"),
        "spec must cite semantic_fills_aot_export_roots=true"
    );
    assert!(
        (spec.contains("language_exports_auto_discovery_enabled=false")
            || spec.contains("`language_exports_auto_discovery_enabled=false`"))
            && (spec.contains("exports_ga_claimed=false")
                || spec.contains("`exports_ga_claimed=false`")),
        "spec must cite auto/GA false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("auto-discovery")
                || spec.contains("compile")
                || spec.contains("B-09")),
        "spec Frozen must fence auto-discovery / delete API / B-09"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn language_exports_semantic_aot_export_roots_inventory_nails() {
    let inv = syx_mir::language_exports_semantic_aot_export_roots_inventory();
    assert!(inv.decl_export_first_slice_floor_ok);
    assert!(inv.semantic_fills_aot_export_roots);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(!inv.exports_ga_claimed);
    assert!(inv.language_exports_semantic_aot_export_roots_signed);
    // Honesty floor: W480 inventory historical semantic_fills stays false.
    let floor = syx_mir::language_exports_decl_export_first_slice_inventory();
    assert!(!floor.semantic_fills_aot_export_roots);
    assert!(floor.language_exports_decl_export_first_slice_signed);
    let thin = syx_mir::language_exports_aot_roots_thin_inventory();
    assert!(!thin.semantic_fills_aot_export_roots);
    assert!(thin.language_exports_thin_signed);
}

#[test]
fn language_exports_semantic_aot_export_roots_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W481")
            && (roadmap.contains("ADR-825")
                || roadmap.contains("aot_export_roots")
                || roadmap.contains("semantic")),
        "roadmap must cite W481 / ADR-825 semantic→aot_export_roots"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(
        b02.contains("| [x] |"),
        "B-02 must remain [x] after W481: {b02}"
    );
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("| [x] |") || b09.contains("| [~] |"),
        "B-09 must be [x] (W558+) or historical [~] after W481: {b09}"
    );
    for id in ["B-01", "B-03", "B-04", "B-05", "B-06", "B-07", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} must remain [x]: {row}");
    }
}
