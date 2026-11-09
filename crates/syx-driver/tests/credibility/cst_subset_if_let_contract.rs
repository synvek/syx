//! S2-I154 S2-04 CST subset if-let-simple contracts.
//! Nails: ADR + take_if_let_simple + if_let_mvp fixtures; if case still CST001;
//! ≠ FrontendPipeline / ≠ S2-04 [x] / ≠ delete syx-cst.
//! See docs/adr/stage2-s2-04-cst-subset-if-let.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn fixture(name: &str) -> String {
    repo_file(&format!(
        "crates/syx-driver/tests/fixtures/format/if_let_mvp/{name}"
    ))
}

#[test]
fn s2_04_cst_subset_if_let_adr_and_grammar() {
    let adr = repo_file("docs/adr/stage2-s2-04-cst-subset-if-let.md");
    assert!(adr.contains("S2-I154"), "if-let ADR must exist");
    assert!(
        adr.contains("take_if_let_simple"),
        "ADR must name take_if_let_simple"
    );
    assert!(
        adr.contains("≠") && adr.contains("if case"),
        "ADR must Non-claim if case"
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
        cst.contains("fn take_if_let_simple("),
        "take_if_let_simple must exist"
    );
    assert!(
        cst.contains("take_if_let_simple(rest)") || cst.contains("take_if_let_simple(&"),
        "take_if_let_simple must be wired"
    );
}

#[test]
fn s2_04_cst_subset_if_let_mvp_fixtures() {
    for name in ["basic.sy", "with_else.sy", "two_ifs.sy", "call_rhs.sy"] {
        let src = fixture(name);
        let file = syx_diagnostics::SourceFile::new(name, src.as_str());
        let root = syx_cst::parse_cst_subset(&file)
            .unwrap_or_else(|e| panic!("{name}: parse_cst_subset must Ok, got {e:?}"));
        assert_eq!(
            syx_cst::format_cst(&root),
            src,
            "{name}: CST roundtrip must be byte-identical"
        );
        // format_source_auto must succeed on CST path (not forced AST-only failure).
        let formatted = syx_fmt::format_source_auto(&file)
            .unwrap_or_else(|e| panic!("{name}: format_source_auto must Ok, got {e:?}"));
        assert!(
            !formatted.is_empty(),
            "{name}: format_source_auto produced empty"
        );
    }
}

#[test]
fn s2_04_cst_subset_if_case_still_cst001() {
    let src = "func main() {\n  if case \\Foo.bar = x {\n    print(1)\n  }\n}\n";
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let err = syx_cst::parse_cst_subset(&file).expect_err("if case must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}
