//! S3-I100…I103 Stage-4 / S4-04 Sequence/Collection census contracts.
//! Open + hinge + residual + claim · Stage-4 [x] 退出 · S4-04 [x] · 4/5.
//! See docs/adr/stage4-s4-04-open.md · docs/stage4-sequence-collection-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_04_sequence_collection_census_baseline() {
    let census = repo_file("docs/stage4-sequence-collection-census.md");
    assert!(census.contains("S3-I100"), "census must name S3-I100");
    assert!(census.contains("S3-I101"), "census must name S3-I101");
    assert!(census.contains("S3-I102"), "census must name S3-I102");
    assert!(census.contains("S3-I103"), "census must name S3-I103");
    assert!(census.contains("S3-I104"), "census must name S3-I104 next");
    assert!(
        census.contains("Current:** S3-I103") || census.contains("Current: S3-I103"),
        "census Current must advance to S3-I103"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("4/5"),
        "census must publish Stage-4 5/5"
    );
    assert!(
        census.contains("Iterator")
            || census.contains("Sequence")
            || census.contains("Collection")
            || census.contains("for-in"),
        "census must cover Sequence/Collection/Iterator axes"
    );
    assert!(
        census.contains("s4_04_claim_helper")
            || census.contains("custom_iterator_forin_hinge_helper")
            || census.contains("residual_scope_helper"),
        "census must publish claim / hinge / residual KPIs"
    );

    let adr = repo_file("docs/adr/stage4-s4-04-open.md");
    assert!(adr.contains("S3-I100"), "open ADR must name S3-I100");
    assert!(
        adr.contains("S3-I103") || adr.contains("stage4-s4-04-claim"),
        "open ADR Consequences must cite landed I103"
    );
    assert!(
        adr.contains("S3-I104") || adr.contains("S4-05"),
        "open ADR Consequences must lock next = I104"
    );
    assert!(
        adr.contains("S3-I99") || adr.contains("stage4-s4-03-claim"),
        "ADR must cite prior S4-03 claim"
    );
    assert!(
        adr.contains("makeIterator") || adr.contains("MakeIterator"),
        "ADR Gap must name makeIterator axis"
    );
    assert!(
        adr.contains("M2-S4-04") || adr.contains("HTTP"),
        "ADR must disambiguate from old M2-S4-04 HTTP id"
    );

    let script = repo_file("scripts/s4_04_sequence_collection_census.sh");
    assert!(
        script.contains("s4_04_open_helper")
            && script.contains("custom_iterator_forin_hinge_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s4_04_claim_helper")
            && script.contains("iterator_forin_test_helper")
            && script.contains("collections_protocol_helper")
            && script.contains("current=S3-I103")
            && script.contains("next=S3-I104"),
        "census script must emit I103 ladder / KPIs"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 stage-4 row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        stage4.contains("5/5"),
        "Stage-4 checklist must be 5/5; got: {stage4}"
    );
    let s404 = plan02
        .lines()
        .find(|l| l.contains("| S4-04 |"))
        .expect("plan/02 S4-04 row");
    assert!(
        s404.contains("| [x] |") || s404.contains("| `[x]` |") || s404.contains("[x]"),
        "S4-04 must be [x] after I103; got: {s404}"
    );
    assert!(
        plan02.contains("S3-I100")
            && plan02.contains("S3-I101")
            && plan02.contains("S3-I102")
            && plan02.contains("S3-I103")
            && plan02.contains("S3-I104"),
        "plan/02 must cite I100…I104"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I103") && plan01.contains("5/5"),
        "plan/01 must cite I103 / 5/5"
    );
    assert!(
        plan01.contains("S4-04") && plan01.contains("[x]"),
        "plan/01 must reflect S4-04 [x]"
    );
    assert!(
        plan01.contains("S3-I104") || plan01.contains("S4-05"),
        "plan/01 must lock next = I104"
    );

    let prior = repo_file("docs/adr/stage4-s4-03-claim.md");
    assert!(
        prior.contains("S3-I100") || prior.contains("stage4-s4-04-open"),
        "I99 ADR must cite landed I100"
    );
}

#[test]
fn s4_04_sequence_collection_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_04_sequence_collection_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_04 census");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for key in [
        "s4_04_open_helper=",
        "s4_04_census_helper=",
        "custom_iterator_forin_hinge_helper=",
        "residual_scope_helper=",
        "s4_04_claim_helper=",
        "custom_iterator_parity_fixture_helper=",
        "iterator_forin_test_helper=",
        "collections_protocol_helper=",
        "make_iterator_surface_helper=",
        "collection_host_map_filter_helper=",
        "stage4_status=",
        "s4_04_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected {key} in:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I103") && stdout.contains("next=S3-I104"),
        "ladder I103→I104:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_01_status=x")
            && stdout.contains("s4_02_status=x")
            && stdout.contains("s4_03_status=x")
            && stdout.contains("s4_04_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "status / 4/5:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("s4_04_open_helper=") >= 1);
    assert!(parse("s4_04_census_helper=") >= 1);
    assert!(parse("custom_iterator_forin_hinge_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("s4_04_claim_helper=") >= 1);
    assert!(parse("custom_iterator_parity_fixture_helper=") >= 1);
    assert!(parse("iterator_forin_test_helper=") >= 1);
    assert!(parse("collections_protocol_helper=") >= 1);
    assert!(parse("make_iterator_surface_helper=") >= 1);
    assert!(parse("collection_host_map_filter_helper=") >= 1);
}

#[test]
fn s4_04_open_does_not_exit_stage4() {
    let claim = repo_file("docs/adr/stage4-s4-04-claim.md");
    assert!(
        claim.contains("S4-04") && claim.contains("`[x]`"),
        "claim ADR Status must keep S4-04 [x]"
    );
    assert!(
        claim.contains("≠") && claim.contains("Stage-4"),
        "claim ADR must Non-claim Stage-4 exit"
    );
    let census = repo_file("docs/stage4-sequence-collection-census.md");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("4/5"),
        "census must keep Stage-4 4/5"
    );
    let residual = repo_file("docs/adr/stage4-s4-04-residual-scope.md");
    assert!(
        residual.contains("S3-I103") || residual.contains("stage4-s4-04-claim"),
        "residual ADR must cite landed I103 claim"
    );
}
