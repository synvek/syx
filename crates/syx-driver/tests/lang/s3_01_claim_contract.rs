//! S3-I41 S3-01 plan claim contracts.
//! Nails: claim ADR + plan/02 S3-01 [x]; scoped Non-claims; Stage-3 still [~].
//! See docs/adr/stage3-s3-01-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_01_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-01-claim.md");
    assert!(adr.contains("S3-I41"), "S3-01 claim ADR must exist");
    assert!(
        adr.contains("S3-01") && adr.contains("`[x]`"),
        "ADR Status must claim S3-01 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );
    assert!(
        adr.contains("positionless_helper_calls")
            || adr.contains("helper_calls")
            || adr.contains("miette"),
        "ADR Evidence must tie to claim axes ①②③"
    );
    assert!(
        adr.contains("S3-I42") && (adr.contains("S3-02") || adr.contains("open")),
        "ADR must cite S3-I42 S3-02 open"
    );
    assert!(
        adr.contains("S3-I43") && (adr.contains("demangle") || adr.contains("S3-02")),
        "ADR must cite S3-I43 demangle"
    );
    assert!(
        adr.contains("S3-I44") && (adr.contains("debug") || adr.contains("stack") || adr.contains("S3-02")),
        "ADR must cite S3-I44 debug stack"
    );
    assert!(
        adr.contains("S3-I45") && (adr.contains("runtime") || adr.contains("__type")),
        "ADR must cite S3-I45 runtime"
    );
    assert!(
        adr.contains("S3-I46") && (adr.contains("meta") || adr.contains("S3-02")),
        "ADR must cite S3-I46 meta"
    );
    assert!(
        adr.contains("S3-I47") && (adr.contains("claim") || adr.contains("S3-02")),
        "ADR must cite S3-I47 claim"
    );
    assert!(
        adr.contains("S3-03") && (adr.contains("open") || adr.contains("标准库")),
        "ADR must lock next = S3-03"
    );
    assert!(
        adr.contains("noise") || adr.contains("I40") || adr.contains("S3-I40"),
        "ADR Evidence must cite I40 noise/claim-prep"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_01_line = plan02
        .lines()
        .find(|l| l.contains("| S3-01 |"))
        .expect("plan/02 must have S3-01 row");
    assert!(
        s3_01_line.contains("| [x] |") || s3_01_line.contains("| `[x]` |"),
        "plan/02 S3-01 status must be [x]; got: {s3_01_line}"
    );
    assert!(
        plan02.contains("S3-I41") || plan02.contains("stage3-s3-01-claim"),
        "plan/02 must cite S3-I41 claim"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );

    let census = repo_file("docs/stage3-diagnostic-span-census.md");
    assert!(
        census.contains("S3-I41"),
        "span census must name S3-I41"
    );
    assert!(
        census.contains("Current:** S3-I47") || census.contains("Current: S3-I47"),
        "census Current must be S3-I47"
    );
    assert!(
        census.contains("S3-I42") && (census.contains("S3-02") || census.contains("open")),
        "census must name S3-I42 S3-02 open"
    );
    assert!(
        census.contains("S3-I43") && (census.contains("demangle") || census.contains("S3-02")),
        "census must name S3-I43 demangle"
    );
    assert!(
        census.contains("S3-I44") && (census.contains("debug") || census.contains("stack") || census.contains("S3-02")),
        "census must name S3-I44 debug stack"
    );
    assert!(
        census.contains("S3-I45") && (census.contains("runtime") || census.contains("__type")),
        "census must name S3-I45 runtime"
    );
    assert!(
        census.contains("S3-I46") && (census.contains("meta") || census.contains("S3-02")),
        "census must name S3-I46 meta"
    );
    assert!(
        census.contains("S3-I47") && census.contains("claim"),
        "census must name S3-I47 claim"
    );
    assert!(
        census.contains("S3-03"),
        "census ladder must next = S3-03"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I41") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I41 S3-01 [x]"
    );
    assert!(
        plan01.contains("S3-I42") || plan01.contains("S3-02"),
        "plan/01 must name S3-I42 S3-02"
    );
    assert!(
        plan01.contains("S3-I43") || plan01.contains("demangle"),
        "plan/01 must name S3-I43 demangle"
    );
    assert!(
        plan01.contains("S3-I44") || plan01.contains("debug"),
        "plan/01 must name S3-I44 debug"
    );
    assert!(
        plan01.contains("S3-I45") || plan01.contains("runtime"),
        "plan/01 must name S3-I45 runtime"
    );
    assert!(
        plan01.contains("S3-I46") || plan01.contains("meta"),
        "plan/01 must name S3-I46 meta"
    );
    assert!(
        plan01.contains("S3-I47") || plan01.contains("claim"),
        "plan/01 must name S3-I47 claim"
    );
    assert!(
        plan01.contains("S3-03") || plan01.contains("open"),
        "plan/01 must lock next = S3-03"
    );
}
