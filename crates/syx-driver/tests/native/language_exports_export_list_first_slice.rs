//! Language exports `export {…}` list first-slice (W483 / ADR-827).
//! Decl::ExportList + lower → aot_export_roots; ≠ scan-all auto / ≠ exports GA / ≠ B-09 [x].

#[test]
fn language_exports_export_list_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-ast/src/lib.rs",
        "crates/syx-parser/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/lang/language-exports-export-list-first-slice.md",
        "docs/lang/language-exports-semantic-aot-export-roots.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "export-list first-slice requires evidence file: {path}"
        );
    }
    let ast = include_str!("../../../syx-ast/src/lib.rs");
    assert!(
        ast.contains("ExportList") && ast.contains("ADR-827"),
        "AST must expose Decl::ExportList (W483 / ADR-827)"
    );
    let parser = include_str!("../../../syx-parser/src/lib.rs");
    assert!(
        parser.contains("parse_export_list")
            && parser.contains("Decl::ExportList")
            && parser.contains("ADR-827"),
        "parser must accept export {{…}} → Decl::ExportList"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsExportListFirstSliceInventory")
            && mir.contains("language_exports_export_list_first_slice_inventory")
            && (mir.contains("language_exports_export_list_first_slice_signed: true")
                || mir.contains("language_exports_export_list_first_slice_signed:true"))
            && mir.contains("ExportList"),
        "syx-mir must expose export-list inventory and lower ExportList"
    );
    assert!(
        (mir.contains("language_exports_auto_discovery_enabled: false")
            || mir.contains("language_exports_auto_discovery_enabled:false"))
            && (mir.contains("export_list_syntax_enabled: true")
                || mir.contains("export_list_syntax_enabled:true")),
        "export-list inventory must keep auto false and list syntax true"
    );
}

#[test]
fn language_exports_export_list_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-export-list-first-slice.md");
    assert!(
        spec.contains("ADR-827") && (spec.contains("ExportList") || spec.contains("export {")),
        "spec must cite ADR-827 export-list first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-third"),
        
        "spec must cite W483 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsExportListFirstSliceInventory")
            && spec.contains("language_exports_export_list_first_slice_inventory"),
        "spec must name export-list inventory API"
    );
    assert!(
        spec.contains("language_exports_export_list_first_slice_signed=true")
            || spec.contains("`language_exports_export_list_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("export_list_syntax_enabled=true")
            || spec.contains("`export_list_syntax_enabled=true`"))
            && (spec.contains("language_exports_auto_discovery_enabled=false")
                || spec.contains("`language_exports_auto_discovery_enabled=false`")),
        "spec must cite list true and auto false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("auto-discovery")
                || spec.contains("auto_discovery")
                || spec.contains("B-09")),
        "spec Frozen must fence scan-all auto / B-09"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn language_exports_export_list_first_slice_inventory_nails() {
    let inv = syx_mir::language_exports_export_list_first_slice_inventory();
    assert!(inv.semantic_roots_floor_ok);
    assert!(inv.export_list_syntax_enabled);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(!inv.exports_ga_claimed);
    assert!(inv.language_exports_export_list_first_slice_signed);
    let sem = syx_mir::language_exports_semantic_aot_export_roots_inventory();
    assert!(!sem.language_exports_auto_discovery_enabled);
    assert!(sem.language_exports_semantic_aot_export_roots_signed);
    let floor = syx_mir::language_exports_decl_export_first_slice_inventory();
    assert!(!floor.language_exports_auto_discovery_enabled);
}

