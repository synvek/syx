//! S3-I47 S3-02 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-02 [x]; scoped Non-claims; Stage-3 still [~].
//! See docs/adr/stage3-s3-02-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_02_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-02-claim.md");
    assert!(adr.contains("S3-I47"), "S3-02 claim ADR must exist");
    assert!(
        adr.contains("S3-02") && adr.contains("`[x]`"),
        "ADR Status must claim S3-02 [x]"
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
        adr.contains("named_passthrough")
            || adr.contains("user_facing_type_name"),
        "ADR Evidence must cite axis ①"
    );
    assert!(
        adr.contains("user_facing_mir_fn_name"),
        "ADR Evidence must cite axis ③ debug"
    );
    assert!(
        adr.contains("user_facing_runtime_type_name"),
        "ADR Evidence must cite axis ④ runtime"
    );
    assert!(
        adr.contains("user_facing_meta_type_name"),
        "ADR Evidence must cite axis ⑤ meta"
    );
    assert!(
        adr.contains("debug_mangled_fn_nail")
            || adr.contains("breakpoint")
            || adr.contains("断点")
            || adr.contains("identity"),
        "ADR must document intentional OUT"
    );
    assert!(
        adr.contains("S3-03") && (adr.contains("open") || adr.contains("标准库")),
        "ADR must lock next = S3-03 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_02 = plan02
        .lines()
        .find(|l| l.contains("| S3-02 |"))
        .expect("plan/02 must have S3-02 row");
    assert!(
        s3_02.contains("| [x] |") || s3_02.contains("| `[x]` |"),
        "plan/02 S3-02 status must be [x]; got: {s3_02}"
    );
    assert!(
        plan02.contains("S3-I47") || plan02.contains("stage3-s3-02-claim"),
        "plan/02 must cite S3-I47 claim"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );
    assert!(
        plan02.contains("S3-03")
            && (plan02.contains("open") || plan02.contains("标准库") || plan02.contains("预编译")),
        "plan/02 must lock next = S3-03"
    );

    let census = repo_file("docs/stage3-mono-display-census.md");
    assert!(
        census.contains("S3-I47"),
        "mono census must name S3-I47"
    );
    assert!(
        census.contains("Current:** S3-I47") || census.contains("Current: S3-I47"),
        "census Current must be S3-I47"
    );
    assert!(
        census.contains("Claimed") || census.contains("`[x]`"),
        "census must mark Claimed"
    );
    assert!(
        census.contains("S3-03"),
        "census ladder must next = S3-03"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I47") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I47 S3-02 [x]"
    );
    assert!(
        plan01.contains("S3-03") || plan01.contains("open"),
        "plan/01 must lock next = S3-03"
    );
}
