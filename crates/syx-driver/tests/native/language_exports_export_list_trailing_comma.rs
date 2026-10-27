//! Language exports `export {…}` trailing-comma (W486 / ADR-830).
//! Optional trailing comma in ExportList; ≠ scan-all auto / ≠ exports GA / ≠ B-09 [x].

#[test]
fn language_exports_export_list_trailing_comma_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-parser/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "docs/lang/language-exports-export-list-trailing-comma.md",
        "docs/lang/language-exports-export-list-first-slice.md",
        "docs/lang/language-exports-syntax-spec-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "export-list trailing-comma requires evidence file: {path}"
        );
    }
    let parser = include_str!("../../../syx-parser/src/lib.rs");
    assert!(
        parser.contains("parse_export_list")
            && parser.contains("ADR-830")
            && parser.contains("trailing comma"),
        "parser must cite ADR-830 trailing comma for export list"
    );
    assert!(
        !parser.contains("Trailing comma not allowed in export list"),
        "parser must no longer reject trailing comma with that error"
    );
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsExportListTrailingCommaInventory")
            && mir.contains("language_exports_export_list_trailing_comma_inventory")
            && (mir.contains("language_exports_export_list_trailing_comma_signed: true")
                || mir.contains("language_exports_export_list_trailing_comma_signed:true"))
            && (mir.contains("export_list_trailing_comma_enabled: true")
                || mir.contains("export_list_trailing_comma_enabled:true")),
        "syx-mir must expose trailing-comma inventory"
    );
    assert!(
        mir.contains("language_exports_auto_discovery_enabled: false")
            || mir.contains("language_exports_auto_discovery_enabled:false"),
        "trailing-comma inventory path must keep auto false"
    );
}

#[test]
fn language_exports_export_list_trailing_comma_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-export-list-trailing-comma.md");
    assert!(
        spec.contains("ADR-830") && (spec.contains("trailing") || spec.contains("尾逗号")),
        "spec must cite ADR-830 trailing-comma"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-sixth"),
        
        "spec must cite W486 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsExportListTrailingCommaInventory")
            && spec.contains("language_exports_export_list_trailing_comma_inventory"),
        "spec must name trailing-comma inventory API"
    );
    assert!(
        spec.contains("language_exports_export_list_trailing_comma_signed=true")
            || spec.contains("`language_exports_export_list_trailing_comma_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("export_list_trailing_comma_enabled=true")
            || spec.contains("`export_list_trailing_comma_enabled=true`"))
            && (spec.contains("language_exports_auto_discovery_enabled=false")
                || spec.contains("`language_exports_auto_discovery_enabled=false`")),
        "spec must cite trailing true and auto false"
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
fn language_exports_export_list_trailing_comma_parser_accepts() {
    let source = syx_diagnostics::SourceFile::new(
        "t.sy",
        "export { greet, }\nfunc greet() {}\nfunc main() { greet() }\n",
    );
    let module = syx_parser::Parser::parse(source).expect("trailing comma must parse");
    match &module.decls[0] {
        syx_ast::Decl::ExportList { names, .. } => {
            assert_eq!(
                names.iter().map(|n| n.as_str()).collect::<Vec<_>>(),
                vec!["greet"]
            );
        }
        other => panic!("expected ExportList, got {other:?}"),
    }
    let empty = syx_diagnostics::SourceFile::new("t.sy", "export { }\n");
    assert!(syx_parser::Parser::parse(empty).is_err());
}

#[test]
fn language_exports_export_list_trailing_comma_inventory_nails() {
    let inv = syx_mir::language_exports_export_list_trailing_comma_inventory();
    assert!(inv.export_list_first_slice_floor_ok);
    assert!(inv.export_list_trailing_comma_enabled);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(!inv.exports_ga_claimed);
    assert!(inv.language_exports_export_list_trailing_comma_signed);
    let floor = syx_mir::language_exports_export_list_first_slice_inventory();
    assert!(floor.export_list_syntax_enabled);
    assert!(!floor.language_exports_auto_discovery_enabled);
}

#[test]
fn language_exports_export_list_trailing_comma_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W486")
            && (roadmap.contains("ADR-830")
                || roadmap.contains("trailing")
                || roadmap.contains("尾逗号")
                || roadmap.contains("export")),
        "roadmap must cite W486 / ADR-830 export-list trailing-comma"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(b02.contains("[x]"), "roadmap B-02 must remain [x]: {b02}");
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("[x]") || b09.contains("[~]"),
        "roadmap B-09 must be [x] (W558+) or historical [~]: {b09}"
    );
}
