//! Language exports → aot_export_roots thin-open (W460 / ADR-804).
//! MIR field + W440 wire honesty; inventory semantic_fills remains false (honesty floor).
//! ≠ auto-discovery / ≠ Module.exports frontend crate wiring / ≠ daemon GA.

#[test]
fn language_exports_aot_roots_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/lang/language-exports-aot-roots-thin.md",
        "docs/gates/gate-b04-default-multi-root.md",
        "plan/archive/gates/gate-b04-short-exit.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "language exports thin-open requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsAotRootsThinInventory")
            && mir.contains("language_exports_aot_roots_thin_inventory")
            && mir.contains("ADR-804")
            && mir.contains("aot_export_roots")
            && (mir.contains("language_exports_thin_signed: true")
                || mir.contains("language_exports_thin_signed:true")),
        "syx-mir must expose LanguageExportsAotRootsThinInventory (W460 / ADR-804)"
    );
    assert!(
        mir.contains("language_exports_auto_discovery_enabled: false")
            || mir.contains("language_exports_auto_discovery_enabled:false"),
        "inventory must keep auto-discovery false"
    );
    // Honesty floor: W460 inventory keeps semantic_fills/frontend flags false
    // (live Decl::Export fill landed W481 in syx-mir lower_module; not frontend crate).
    assert!(
        mir.contains("semantic_fills_aot_export_roots: false")
            || mir.contains("semantic_fills_aot_export_roots:false"),
        "W460 thin inventory must keep historical semantic_fills false"
    );
    let frontend_root = format!("{root}crates/syx-frontend/src");
    let mut found_module_exports = false;
    if let Ok(entries) = std::fs::read_dir(&frontend_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                if text.contains("Module.exports") {
                    found_module_exports = true;
                    break;
                }
            }
        }
    }
    assert!(
        !found_module_exports,
        "frontend crate must not wire Module.exports (W460 honesty)"
    );
}

#[test]
fn language_exports_aot_roots_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-aot-roots-thin.md");
    assert!(
        spec.contains("ADR-804") && (spec.contains("thin-open") || spec.contains("thin open")),
        "spec must cite ADR-804 language exports thin-open"
    );
    assert!(
        spec.contains("three-hundred-and-tenth"),
        
        "spec must cite W460 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsAotRootsThinInventory")
            && spec.contains("language_exports_aot_roots_thin_inventory"),
        "spec must name language exports thin inventory API"
    );
    assert!(
        spec.contains("language_exports_thin_signed=true")
            || spec.contains("`language_exports_thin_signed=true`"),
        "spec must cite language_exports_thin_signed=true"
    );
    assert!(
        spec.contains("language_exports_auto_discovery_enabled=false")
            || spec.contains("`language_exports_auto_discovery_enabled=false`"),
        "spec must cite language_exports_auto_discovery_enabled=false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("auto-discovery")
                || spec.contains("Module.exports")
                || spec.contains("frontend")),
        "spec Frozen must fence auto-discovery / frontend"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn language_exports_aot_roots_thin_inventory_nails() {
    let inv = syx_mir::language_exports_aot_roots_thin_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.relational_mir_field_and_multi_root_ok);
    assert!(inv.mir_aot_export_roots_field_present);
    assert!(inv.default_multi_root_emit_wired);
    assert!(!inv.frontend_export_syntax_present);
    assert!(!inv.semantic_fills_aot_export_roots);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(inv.language_exports_thin_signed);
}

