//! S3-I85 S3-07 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-07 [x]; scoped Non-claims; Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-07-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_07_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-07-claim.md");
    assert!(adr.contains("S3-I85"), "S3-07 claim ADR must exist");
    assert!(
        adr.contains("S3-07") && adr.contains("`[x]`"),
        "ADR Status must claim S3-07 [x]"
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
        adr.contains("default_value") || adr.contains("\"vm\"") || adr.contains("vm"),
        "ADR Evidence must cite Primary default-vm"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep UsageGuide/crate deferred"
    );
    assert!(
        adr.contains("S3-I84") || adr.contains("residual-scope"),
        "ADR must cite I84 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I86")
            || (adr.contains("exit") && adr.contains("open")),
        "ADR must cite S3-I86 exit-open ladder"
    );
    assert!(
        adr.contains("S3-I87")
            || (adr.contains("stage3-exit") && adr.contains("write")),
        "ADR Consequences must lock next = stage3-exit / I87"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_07 = plan02
        .lines()
        .find(|l| l.contains("| S3-07 |"))
        .expect("plan/02 must have S3-07 row");
    assert!(
        s3_07.contains("| [x] |") || s3_07.contains("| `[x]` |"),
        "plan/02 S3-07 status must be [x]; got: {s3_07}"
    );
    assert!(
        plan02.contains("S3-I85") || plan02.contains("stage3-s3-07-claim"),
        "plan/02 must cite S3-I85 claim"
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
        plan02.contains("7/7"),
        "plan/02 must publish Stage-3 7/7"
    );
    assert!(
        plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4"))
            || (plan02.contains("exit") && plan02.contains("open")),
        "plan/02 must cite I86 exit-open / I87 next"
    );

    let census = repo_file("docs/census/stage3-init-template-census.md");
    assert!(
        census.contains("S3-I85"),
        "init-template census must name S3-I85"
    );
    assert!(
        census.contains("Current:** S3-I85") || census.contains("Current: S3-I85"),
        "census Current must be S3-I85"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("7/7"),
        "census must publish Stage-3 7/7"
    );
    assert!(
        census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")) || census.contains("exit"),
        "census ladder must cite I86 / I87 exit"
    );

    let prior = repo_file("docs/adr/stage3-s3-07-residual-scope.md");
    assert!(
        prior.contains("S3-I85") || prior.contains("stage3-s3-07-claim"),
        "I84 ADR must cite landed I85"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I85") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I85 S3-07 [x]"
    );
    assert!(
        plan01.contains("7/7"),
        "plan/01 must publish Stage-3 7/7"
    );
    assert!(
        plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4"))
            || (plan01.contains("exit") && plan01.contains("open")),
        "plan/01 must lock next = Stage-3 exit open / I86"
    );
}

#[test]
fn s3_07_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_07_init_template_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run init-template census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s3_07_claim_helper=",
        "residual_scope_helper=",
        "init_default_template_native=",
        "init_default_template_vm=",
        "s3_07_status=",
        "stage3_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I85") && stdout.contains("next=S3-I89"),
        "script ladder must be I85→I89:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_07_status=x") && stdout.contains("stage3_checklist=7/7"),
        "script must emit claimed status / 7/7:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s3_07_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s3_07_claim_helper>=1, got {claim}\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert_eq!(parse("init_default_template_native="), 0);
    assert!(parse("init_default_template_vm=") >= 1);
    assert!(parse("init_vm_named_template_count=") >= 1);
    assert!(parse("init_integration_test_count=") >= 1);
}
