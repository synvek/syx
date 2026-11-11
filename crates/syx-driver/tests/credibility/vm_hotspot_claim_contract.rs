//! S3-I71 S3-04 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-04 [x]; scoped Non-claims; Stage-3 still [~].
//! See docs/adr/stage3-s3-04-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-04-claim.md");
    assert!(adr.contains("S3-I71"), "S3-04 claim ADR must exist");
    assert!(
        adr.contains("S3-04") && adr.contains("`[x]`"),
        "ADR Status must claim S3-04 [x]"
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
        adr.contains("1.5")
            && adr.contains("1.8")
            && (adr.contains("fib30_interp_ceiling") || adr.contains("ceiling")),
        "ADR Evidence must cite interp ceiling"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep literal <0.5 deferred"
    );
    assert!(
        adr.contains("S3-I70") || adr.contains("residual-scope"),
        "ADR must cite I70 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I72") || (adr.contains("S3-05") && adr.contains("open")),
        "ADR must lock next = S3-05 open / I72"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_04 = plan02
        .lines()
        .find(|l| l.contains("| S3-04 |"))
        .expect("plan/02 must have S3-04 row");
    assert!(
        s3_04.contains("| [x] |") || s3_04.contains("| `[x]` |"),
        "plan/02 S3-04 status must be [x]; got: {s3_04}"
    );
    assert!(
        plan02.contains("S3-I71") || plan02.contains("stage3-s3-04-claim"),
        "plan/02 must cite S3-I71 claim"
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
        plan02.contains("S3-I72") || (plan02.contains("S3-05") && plan02.contains("open")),
        "plan/02 must lock next = S3-05 open / I72"
    );

    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("S3-I71"),
        "vm-hotspot census must name S3-I71"
    );
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("S3-I72") || census.contains("S3-05"),
        "census ladder must next = S3-05 / I72"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-residual-scope.md");
    assert!(
        prior.contains("S3-I71") || prior.contains("stage3-s3-04-claim"),
        "I70 ADR must cite landed I71"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I71") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I71 S3-04 [x]"
    );
    assert!(
        plan01.contains("S3-I72") || (plan01.contains("S3-05") && plan01.contains("open")),
        "plan/01 must lock next = S3-05 open / I72"
    );
}

#[test]
fn s3_04_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_04_vm_hotspot_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run vm-hotspot census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s3_04_claim_helper=",
        "residual_scope_helper=",
        "examples_fib=",
        "fib_criterion_bench=",
        "fib30_s=",
        "fib30_interp_ceiling_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s3_04_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s3_04_claim_helper>=1, got {claim}\n{stdout}"
    );
}
