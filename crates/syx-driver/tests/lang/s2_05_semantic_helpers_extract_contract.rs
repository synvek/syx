//! S2-I249 S2-05 syx-semantic free-helpers extract contracts.
//! Nails: semantic_helpers sidecar + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-helpers-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_semantic_helpers_extract_baseline() {
    let helpers = repo_file("crates/syx-semantic/src/semantic_helpers.rs");
    assert!(
        helpers.contains("fn has_attr(") || helpers.contains("pub(crate) fn has_attr("),
        "semantic_helpers must hold has_attr"
    );
    assert!(
        helpers.contains("fn expr_span(") || helpers.contains("pub(crate) fn expr_span("),
        "semantic_helpers must hold expr_span"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod semantic_helpers"),
        "lib.rs must declare mod semantic_helpers"
    );
    assert!(
        lib.contains("#[cfg(test)]"),
        "#[cfg(test)] must remain in lib"
    );
    assert!(
        !lib.contains("pub(crate) fn has_attr(") && !lib.contains("\nfn has_attr("),
        "has_attr definition must leave lib.rs"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-helpers-extract.md");
    assert!(adr.contains("S2-I249"), "extract ADR must name S2-I249");
    assert!(
        adr.contains("S2-I250") && adr.contains("claim"),
        "ADR must lock next = S2-I250 S2-05 claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x] this window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
