//! S2-I238 S2-05 syx-semantic feature_gates extract contracts.
//! Nails: feature_gates module + root pub use + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-feature-gates-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_semantic_feature_gates_extract_baseline() {
    let gates = repo_file("crates/syx-semantic/src/feature_gates.rs");
    assert!(
        gates.contains("pub fn global_actor_v4_cross_module_enabled")
            && gates.contains("pub fn typealias_prod_enabled")
            && gates.contains("pub fn borrow_regions_language_complete_enabled")
            && gates.contains("pub fn property_wrapper_mvp_enabled"),
        "feature_gates must hold actor/typealias/borrow/property gate surface"
    );
    assert!(
        gates.contains("pub fn reflect_type_lower")
            || gates.contains("fn reflect_type_lower"),
        "feature_gates must hold reflect_type_lower (scaffold neighbor)"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod feature_gates;"),
        "lib.rs must declare mod feature_gates"
    );
    assert!(
        lib.contains("pub use feature_gates::*") || lib.contains("pub use feature_gates::{"),
        "lib.rs must pub-use feature_gates"
    );
    assert!(
        !lib.contains("\npub fn global_actor_v4_cross_module_enabled(")
            && !lib.contains("\npub fn typealias_prod_enabled("),
        "gate definitions must leave lib.rs"
    );
    assert!(
        lib.contains("pub use semantic_analyzer::SemanticAnalyzer")
            || lib.contains("mod semantic_analyzer;"),
        "SemanticAnalyzer must be wired (I239–I240 landed)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-feature-gates-extract.md");
    assert!(adr.contains("S2-I238"), "extract ADR must name S2-I238");
    assert!(
        adr.contains("S2-I239") && adr.contains("SemanticAnalyzer"),
        "ADR must name S2-I239 SemanticAnalyzer ladder"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
