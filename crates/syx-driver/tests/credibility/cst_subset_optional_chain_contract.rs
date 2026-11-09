//! S2-I158 S2-04 CST subset optional-chain-simple contracts.
//! Nails: ADR + take_optional_chain_simple + W716 fixtures + half_open mixed;
//! ≠ multi-hop / ≠ force-unwrap / ≠ Frontend / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-cst-subset-optional-chain.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn fixture(dir: &str, name: &str) -> String {
    repo_file(&format!(
        "crates/syx-driver/tests/fixtures/format/{dir}/{name}"
    ))
}

fn assert_cst_ok(label: &str, src: &str) {
    let file = syx_diagnostics::SourceFile::new(label, src);
    let root = syx_cst::parse_cst_subset(&file)
        .unwrap_or_else(|e| panic!("{label}: parse_cst_subset must Ok, got {e:?}"));
    assert_eq!(
        syx_cst::format_cst(&root),
        src,
        "{label}: CST roundtrip must be byte-identical"
    );
    let formatted = syx_fmt::format_source_auto(&file)
        .unwrap_or_else(|e| panic!("{label}: format_source_auto must Ok, got {e:?}"));
    assert!(!formatted.is_empty(), "{label}: format_source_auto empty");
}

#[test]
fn s2_04_cst_subset_optional_chain_adr_and_grammar() {
    let adr = repo_file("docs/adr/stage2-s2-04-cst-subset-optional-chain.md");
    assert!(adr.contains("S2-I158"), "optional-chain ADR must exist");
    assert!(
        adr.contains("take_optional_chain_simple"),
        "ADR must name take_optional_chain_simple"
    );
    assert!(
        adr.contains("≠") && (adr.contains("multi-hop") || adr.contains("force-unwrap")),
        "ADR must Non-claim multi-hop / force-unwrap"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Frontend"),
        "ADR must Non-claim FrontendPipeline"
    );

    let cst = repo_file("crates/syx-cst/src/lib.rs");
    assert!(
        cst.contains("fn take_optional_chain_simple("),
        "take_optional_chain_simple must exist"
    );
    assert!(
        cst.contains("take_optional_chain_simple(body, 2)")
            || cst.contains("take_optional_chain_simple(tokens, j)")
            || cst.contains("take_optional_chain_simple(body, 1)"),
        "take_optional_chain_simple must be wired"
    );
}

#[test]
fn s2_04_cst_subset_optional_chain_mvp_fixtures() {
    for name in ["ident.sy", "call_arg.sy", "two_chains.sy", "nested.sy"] {
        let src = fixture("optional_chain_mvp", name);
        assert_cst_ok(&format!("optional_chain_mvp/{name}"), &src);
    }
}

#[test]
fn s2_04_cst_subset_half_open_mixed_unlocked() {
    let src = fixture("half_open_range_mvp", "mixed.sy");
    assert_cst_ok("half_open_range_mvp/mixed.sy", &src);
}

#[test]
fn s2_04_cst_subset_optional_chain_mixed_force_unwrap_still_cst001() {
    // mixed.sy includes print(1!) — force-unwrap remains out of subset.
    let src = fixture("optional_chain_mvp", "mixed.sy");
    let file = syx_diagnostics::SourceFile::new("opt_mixed.sy", src.as_str());
    let err = syx_cst::parse_cst_subset(&file).expect_err("force unwrap must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}
