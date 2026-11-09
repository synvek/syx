//! S3-I102 S4-04 residual-scope contracts.
//! Post-I103: S4-04 [x] · Stage-4 [x] 退出 4/5 · residual hinge stays green.
//! See docs/adr/stage4-s4-04-residual-scope.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_04_residual_scope_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-04-residual-scope.md");
    assert!(adr.contains("S3-I102"), "residual-scope ADR must name S3-I102");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer makeIterator/map·filter/Index / supersede-only"
    );
    assert!(
        adr.contains("S3-I103")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite landed I103 scoped claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );

    let scope = repo_file("docs/s4-04-sequence-residual-scope-v0.md");
    assert!(
        scope.contains("S3-I102")
            && scope.contains("Deferred")
            && (scope.contains("supersede") || scope.contains("superseding")),
        "scope-v0 must name I102 / deferred supersede"
    );

    let census = repo_file("docs/stage4-sequence-collection-census.md");
    assert!(
        census.contains("Current:** S3-I103") || census.contains("Current: S3-I103"),
        "census Current must advance to S3-I103"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I102 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I104") && census.contains("4/5"),
        "census must name I104 next and Stage-4 4/5"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-04-iterator-forin-hinge.md");
    assert!(
        prior.contains("S3-I102") || prior.contains("stage4-s4-04-residual-scope"),
        "I101 ADR must cite landed I102"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s404 = plan02
        .lines()
        .find(|l| l.contains("| S4-04 |"))
        .expect("S4-04 row");
    assert!(
        s404.contains("| [x] |") || s404.contains("| `[x]` |") || s404.contains("[x]"),
        "S4-04 must be [x] after I103; got: {s404}"
    );
    assert!(
        s404.contains("scoped")
            || s404.contains("residual")
            || s404.contains("deferred")
            || s404.contains("Deferred"),
        "S4-04 row must publish scoped/deferred; got: {s404}"
    );
    assert!(
        plan02.contains("S3-I102")
            && plan02.contains("S3-I103")
            && plan02.contains("S3-I104")
            && plan02.contains("5/5"),
        "plan/02 must cite I102…I104 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I102")
            && plan01.contains("S3-I103")
            && plan01.contains("5/5"),
        "plan/01 must cite I102 / I103 / 5/5"
    );
}

#[test]
fn s4_04_residual_scope_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_04_sequence_collection_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_04 census");
    assert!(
        out.status.success(),
        "census failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I103") && stdout.contains("next=S3-I104"),
        "ladder I103→I104:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_04_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "status/checklist:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("custom_iterator_forin_hinge_helper=") >= 1);
    assert!(parse("custom_iterator_parity_fixture_helper=") >= 1);
    assert!(parse("iterator_forin_test_helper=") >= 1);
    assert!(parse("collections_protocol_helper=") >= 1);
}
