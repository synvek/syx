//! Language exports Decl::Export first-slice (W480 / ADR-824).
//! lexer+AST+parser met; honesty floor keeps semantic_fills=false (live fill landed W481).
//! ≠ auto-discovery / ≠ exports GA / ≠ delete compile API.

#[test]
fn language_exports_decl_export_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-lexer/src/token.rs",
        "crates/syx-ast/src/lib.rs",
        "crates/syx-parser/src/lib.rs",
        "docs/lang/language-exports-decl-export-first-slice.md",
        "docs/lang/language-exports-syntax-spec-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Decl::Export first-slice requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsDeclExportFirstSliceInventory")
            && mir.contains("language_exports_decl_export_first_slice_inventory")
            && mir.contains("ADR-824")
            && (mir.contains("language_exports_decl_export_first_slice_signed: true")
                || mir.contains("language_exports_decl_export_first_slice_signed:true")),
        "syx-mir must expose LanguageExportsDeclExportFirstSliceInventory (W480 / ADR-824)"
    );
    assert!(
        (mir.contains("lexer_export_keyword_present: true")
            || mir.contains("lexer_export_keyword_present:true"))
            && (mir.contains("decl_export_variant_present: true")
                || mir.contains("decl_export_variant_present:true"))
            && (mir.contains("parser_export_enabled: true")
                || mir.contains("parser_export_enabled:true")),
        "first-slice inventory must set lexer/decl/parser true"
    );
    assert!(
        (mir.contains("semantic_fills_aot_export_roots: false")
            || mir.contains("semantic_fills_aot_export_roots:false"))
            && (mir.contains("language_exports_auto_discovery_enabled: false")
                || mir.contains("language_exports_auto_discovery_enabled:false"))
            && (mir.contains("exports_ga_claimed: false")
                || mir.contains("exports_ga_claimed:false")),
        "first-slice inventory must keep semantic/auto/GA false"
    );
    let lexer = include_str!("../../../syx-lexer/src/token.rs");
    assert!(
        lexer.contains("Export")
            && lexer.contains("\"export\"")
            && lexer.contains("TokenKind::Export"),
        "lexer must expose TokenKind::Export + export keyword"
    );
    let ast = include_str!("../../../syx-ast/src/lib.rs");
    assert!(
        ast.contains("Export {")
            && (ast.contains("/// W480") || ast.contains("ADR-824") || ast.contains("export func")),
        "AST must expose Decl::Export variant"
    );
    let parser = include_str!("../../../syx-parser/src/lib.rs");
    assert!(
        parser.contains("parse_export_decl")
            && parser.contains("TokenKind::Export")
            && parser.contains("Decl::Export"),
        "parser must accept export → Decl::Export"
    );
}

#[test]
fn language_exports_decl_export_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-decl-export-first-slice.md");
    assert!(
        spec.contains("ADR-824") && (spec.contains("Decl::Export") || spec.contains("first-slice")),
        "spec must cite ADR-824 Decl::Export first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-thirtieth"),
        
        "spec must cite W480 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsDeclExportFirstSliceInventory")
            && spec.contains("language_exports_decl_export_first_slice_inventory"),
        "spec must name first-slice inventory API"
    );
    assert!(
        spec.contains("language_exports_decl_export_first_slice_signed=true")
            || spec.contains("`language_exports_decl_export_first_slice_signed=true`"),
        "spec must cite language_exports_decl_export_first_slice_signed=true"
    );
    assert!(
        (spec.contains("lexer_export_keyword_present=true")
            || spec.contains("`lexer_export_keyword_present=true`"))
            && (spec.contains("parser_export_enabled=true")
                || spec.contains("`parser_export_enabled=true`")),
        "spec must cite lexer/parser true"
    );
    assert!(
        (spec.contains("semantic_fills_aot_export_roots=false")
            || spec.contains("`semantic_fills_aot_export_roots=false`"))
            && (spec.contains("exports_ga_claimed=false")
                || spec.contains("`exports_ga_claimed=false`")),
        "spec must cite semantic/GA false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("semantic")
                || spec.contains("aot_export_roots")
                || spec.contains("compile")),
        "spec Frozen must fence semantic roots / delete API"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn language_exports_decl_export_first_slice_inventory_nails() {
    let inv = syx_mir::language_exports_decl_export_first_slice_inventory();
    assert!(inv.syntax_spec_floor_ok);
    assert!(inv.lexer_export_keyword_present);
    assert!(inv.decl_export_variant_present);
    assert!(inv.parser_export_enabled);
    assert!(!inv.semantic_fills_aot_export_roots);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(!inv.exports_ga_claimed);
    assert!(inv.language_exports_decl_export_first_slice_signed);
    // Honesty floor: W462 inventory historical flags stay false.
    let floor = syx_mir::language_exports_syntax_spec_deepen_inventory();
    assert!(!floor.lexer_export_keyword_present);
    assert!(!floor.decl_export_variant_present);
    assert!(!floor.parser_export_enabled);
    assert!(floor.language_exports_syntax_spec_signed);
}

