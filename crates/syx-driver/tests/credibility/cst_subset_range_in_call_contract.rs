//! S2-I157 S2-04 CST subset range-in-call-simple contracts.
//! Nails: ADR + take_integer_range_simple + range MVP + control-flow mixed.sy;
//! ≠ RangeDyn / ≠ optional-chain / ≠ Frontend / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-cst-subset-range-in-call.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
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
fn s2_04_cst_subset_range_in_call_adr_and_grammar() {
    let adr = repo_file("docs/adr/stage2-s2-04-cst-subset-range-in-call.md");
    assert!(adr.contains("S2-I157"), "range-in-call ADR must exist");
    assert!(
        adr.contains("take_integer_range_simple"),
        "ADR must name take_integer_range_simple"
    );
    assert!(
        adr.contains("≠") && adr.contains("RangeDyn"),
        "ADR must Non-claim RangeDyn"
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
        cst.contains("fn take_integer_range_simple("),
        "take_integer_range_simple must exist"
    );
    assert!(
        cst.contains("take_integer_range_simple(body, 2)")
            || cst.contains("take_integer_range_simple(tokens, j)")
            || cst.contains("take_integer_range_simple(body, 1)"),
        "take_integer_range_simple must be wired into print/call/return"
    );
}

#[test]
fn s2_04_cst_subset_range_mvp_fixtures() {
    for name in ["literal.sy", "call_arg.sy", "two_ranges.sy", "mixed.sy"] {
        let src = fixture("closed_range_mvp", name);
        assert_cst_ok(&format!("closed_range_mvp/{name}"), &src);
    }
    for name in ["literal.sy", "call_arg.sy", "two_ranges.sy"] {
        let src = fixture("half_open_range_mvp", name);
        assert_cst_ok(&format!("half_open_range_mvp/{name}"), &src);
    }
}

#[test]
fn s2_04_cst_subset_control_flow_mixed_with_range() {
    for (dir, name) in [
        ("if_let_mvp", "mixed.sy"),
        ("guard_let_mvp", "mixed.sy"),
        ("if_case_mvp", "mixed.sy"),
    ] {
        let src = fixture(dir, name);
        assert_cst_ok(&format!("{dir}/{name}"), &src);
    }
}

#[test]
fn s2_04_cst_subset_half_open_mixed_with_optional_ok() {
    // S2-I158: half_open mixed (range + Ident?.Ident) now CST Ok.
    let src = fixture("half_open_range_mvp", "mixed.sy");
    assert_cst_ok("half_open_range_mvp/mixed.sy", &src);
}
