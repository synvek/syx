//! S3-I87 Stage-3 exit contracts.
//! Nails: stage3-exit ADR + plan Stage-3 [x] 退出; Residual; next Stage-4 open.
//! See docs/adr/stage3-exit.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn s3_exit_adr_and_plan_locked() {
    assert!(
        repo_path("docs/adr/stage3-exit.md").exists(),
        "stage3-exit.md must exist"
    );
    let adr = repo_file("docs/adr/stage3-exit.md");
    assert!(adr.contains("S3-I87"), "exit ADR must name S3-I87");
    assert!(
        adr.contains("Stage-3 exits") || adr.contains("Stage-3 exit"),
        "ADR must declare Stage-3 exits"
    );
    assert!(
        adr.contains("S3-I41")
            && adr.contains("S3-I47")
            && adr.contains("S3-I55")
            && adr.contains("S3-I71")
            && adr.contains("S3-I77")
            && adr.contains("S3-I81")
            && adr.contains("S3-I85"),
        "ADR Evidence must cite S3-01…07 claim windows"
    );
    assert!(
        adr.contains("S3-I86") || adr.contains("stage3-exit-open"),
        "ADR must cite exit-open prior"
    );
    assert!(
        adr.contains("Residual") || adr.contains("post-exit"),
        "ADR must publish post-exit Residual"
    );
    assert!(
        (adr.contains("fib") || adr.contains("0.5"))
            && (adr.contains("hybrid") || adr.contains("L010"))
            && (adr.contains("rename") || adr.contains("fence"))
            && (adr.contains("UsageGuide") || adr.contains("crate")),
        "ADR Residual must cover S3-04…07 deferred axes"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 construction"
    );
    assert!(
        adr.contains("Stage-4")
            && (adr.contains("open") || adr.contains("S3-I88")),
        "ADR Consequences must lock next = Stage-4 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 stage-3 row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "plan/02 Stage-3 must be [x] 退出; got: {stage3}"
    );
    assert!(
        plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4")) || plan02.contains("stage3-exit"),
        "plan/02 must cite S3-I87 exit"
    );
    assert!(
        plan02.contains("Stage-4") || plan02.contains("S3-I88"),
        "plan/02 must name Stage-4 open next"
    );
    assert!(
        plan02.contains("7/7") || plan02.contains("S3-01"),
        "plan/02 must retain S3 checklist honesty"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4"))
            && (plan01.contains("[x]") && plan01.contains("退出")),
        "plan/01 must reflect Stage-3 [x] 退出 / S3-I87"
    );
    assert!(
        plan01.contains("Stage-4") || plan01.contains("S3-I88"),
        "plan/01 must lock next = Stage-4 open"
    );

    let prior = repo_file("docs/adr/stage3-exit-open.md");
    assert!(
        prior.contains("S3-I87") || prior.contains("stage3-exit"),
        "I86 ADR must cite landed I87"
    );

    let census = repo_file("docs/stage3-exit-census.md");
    assert!(
        census.contains("Current:** S3-I87") || census.contains("Current: S3-I87"),
        "census Current must be S3-I87"
    );
    assert!(
        census.contains("exit_status") && census.contains("exited"),
        "census must mark exit_status exited"
    );
    assert!(
        census.contains("S3-I88") || census.contains("Stage-4"),
        "census ladder must next = Stage-4 / I88"
    );
}

#[test]
fn s3_exit_census_script_hinges() {
    let root = repo_path(".");
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_exit_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s3_exit_census.sh");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "stage3_exit_helper=",
        "stage3_exit_open_helper=",
        "stage3_claim_chain_helper=",
        "stage3_exit_adr_present=",
        "stage3_status=",
        "stage3_checklist=",
        "exit_status=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I87") && stdout.contains("next=S3-I88"),
        "script ladder must be I87→I88:\n{stdout}"
    );
    assert!(
        stdout.contains("stage3_status=x")
            && stdout.contains("stage3_checklist=7/7")
            && stdout.contains("exit_status=exited")
            && stdout.contains("stage3_exit_adr_present=1"),
        "script must emit exited status:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0)
    };
    assert!(parse("stage3_exit_helper=") >= 1, "exit helper:\n{stdout}");
    assert!(
        parse("stage3_exit_open_helper=") >= 1,
        "exit-open helper:\n{stdout}"
    );
    assert!(
        parse("stage3_claim_chain_helper=") >= 7,
        "claim chain:\n{stdout}"
    );
}
