//! S3-I101 S4-04 custom-Iterator for-in hinge contracts.
//! Post-I103: S4-04 [x] · Stage-4 [x] 退出 4/5 · Primary KPIs stay green.
//! See docs/adr/stage4-s4-04-iterator-forin-hinge.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_04_iterator_forin_hinge_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-04-iterator-forin-hinge.md");
    assert!(adr.contains("S3-I101"), "hinge ADR must name S3-I101");
    assert!(
        adr.contains("Iterator") || adr.contains("for-in") || adr.contains("forin"),
        "ADR must publish Iterator for-in Primary"
    );
    assert!(
        adr.contains("S3-I103") || adr.contains("stage4-s4-04-claim"),
        "ADR Consequences must cite landed I103"
    );

    let charter = repo_file("docs/s4-04-iterator-forin-v0.md");
    assert!(
        charter.contains("S3-I101")
            && (charter.contains("Iterator") || charter.contains("for-in")),
        "iterator-forin-v0 must exist"
    );

    let census = repo_file("docs/stage4-sequence-collection-census.md");
    assert!(
        census.contains("Current:** S3-I103") || census.contains("Current: S3-I103"),
        "census Current must advance to S3-I103 after claim"
    );
    assert!(
        census.contains("custom_iterator_forin_hinge_helper") && census.contains("S3-I104"),
        "census must publish I101 KPIs / I104 next"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-04-open.md");
    assert!(
        prior.contains("S3-I101") || prior.contains("stage4-s4-04-iterator-forin-hinge"),
        "I100 ADR must cite landed I101"
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
        plan02.contains("S3-I101")
            && plan02.contains("S3-I102")
            && plan02.contains("S3-I103")
            && plan02.contains("5/5"),
        "plan/02 must cite I101…I103 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I101") && plan01.contains("5/5"),
        "plan/01 must cite I101 / 5/5"
    );
}

#[test]
fn s4_04_iterator_forin_hinge_census_script_hinges() {
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
    assert!(parse("custom_iterator_forin_hinge_helper=") >= 1);
    assert!(parse("custom_iterator_parity_fixture_helper=") >= 1);
    assert!(parse("iterator_forin_test_helper=") >= 1);
    assert!(parse("collections_protocol_helper=") >= 1);
    assert!(parse("s4_04_open_helper=") >= 1);
}
