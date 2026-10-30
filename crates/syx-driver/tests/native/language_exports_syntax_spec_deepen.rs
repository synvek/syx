//! Language exports syntax-spec deepen (W462 / ADR-806).
//! Proposed EBNF + roots mapping; honesty floor keeps inventory lexer/decl/parser=false.
//! Live frontend Export landed in W480 — this meta no longer nails live absence.

#[test]
fn language_exports_syntax_spec_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/lang/language-exports-syntax-spec-deepen.md",
        "docs/lang/language-exports-aot-roots-thin.md",
        "crates/syx-lexer/src/token.rs",
        "crates/syx-ast/src/lib.rs",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "language exports syntax-spec deepen requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("LanguageExportsSyntaxSpecDeepenInventory")
            && mir.contains("language_exports_syntax_spec_deepen_inventory")
            && mir.contains("ADR-806")
            && (mir.contains("language_exports_syntax_spec_signed: true")
                || mir.contains("language_exports_syntax_spec_signed:true")),
        "syx-mir must expose LanguageExportsSyntaxSpecDeepenInventory (W462 / ADR-806)"
    );
    assert!(
        mir.contains("proposed_syntax_documented: true")
            || mir.contains("proposed_syntax_documented:true"),
        "inventory must set proposed_syntax_documented=true"
    );
    // Honesty floor: W462 inventory historical fields stay false even after W480 live Export.
    assert!(
        (mir.contains("lexer_export_keyword_present: false")
            || mir.contains("lexer_export_keyword_present:false"))
            && (mir.contains("decl_export_variant_present: false")
                || mir.contains("decl_export_variant_present:false"))
            && (mir.contains("parser_export_enabled: false")
                || mir.contains("parser_export_enabled:false")),
        "W462 inventory must keep historical lexer/decl/parser export flags false"
    );
}

#[test]
fn language_exports_syntax_spec_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/language-exports-syntax-spec-deepen.md");
    assert!(
        spec.contains("ADR-806") && (spec.contains("syntax-spec") || spec.contains("syntax spec")),
        "spec must cite ADR-806 language exports syntax-spec deepen"
    );
    assert!(
        spec.contains("three-hundred-and-twelfth"),
        
        "spec must cite W462 fingerprint"
    );
    assert!(
        spec.contains("LanguageExportsSyntaxSpecDeepenInventory")
            && spec.contains("language_exports_syntax_spec_deepen_inventory"),
        "spec must name syntax-spec deepen inventory API"
    );
    assert!(
        spec.contains("language_exports_syntax_spec_signed=true")
            || spec.contains("`language_exports_syntax_spec_signed=true`"),
        "spec must cite language_exports_syntax_spec_signed=true"
    );
    assert!(
        spec.contains("proposed_syntax_documented=true")
            || spec.contains("`proposed_syntax_documented=true`"),
        "spec must cite proposed_syntax_documented=true"
    );
    assert!(
        spec.contains("export")
            && spec.contains("func")
            && (spec.contains("aot_export_roots") || spec.contains("`aot_export_roots`")),
        "spec proposed EBNF must mention export/func → aot_export_roots"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("semantic")
                || spec.contains("aot_export_roots")
                || spec.contains("auto-discovery")),
        "spec Frozen must fence semantic roots / auto-discovery"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    let thin = include_str!("../../../../docs/lang/language-exports-aot-roots-thin.md");
    assert!(
        !thin.contains("frontend_export_syntax_present=true"),
        "thin inventory floor must not claim frontend syntax present"
    );
    assert!(
        thin.contains("Frontend syntax") || thin.contains("Frontend `Module.exports`"),
        "thin Index should still track frontend syntax"
    );
}

#[test]
fn language_exports_syntax_spec_deepen_inventory_nails() {
    let inv = syx_mir::language_exports_syntax_spec_deepen_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.thin_open_floor_ok);
    assert!(inv.proposed_syntax_documented);
    assert!(!inv.lexer_export_keyword_present);
    assert!(!inv.decl_export_variant_present);
    assert!(!inv.parser_export_enabled);
    assert!(!inv.semantic_fills_aot_export_roots);
    assert!(!inv.language_exports_auto_discovery_enabled);
    assert!(inv.language_exports_syntax_spec_signed);
    // Thin floor still denies frontend syntax claim.
    let thin = syx_mir::language_exports_aot_roots_thin_inventory();
    assert!(!thin.frontend_export_syntax_present);
    assert!(thin.language_exports_thin_signed);
}

