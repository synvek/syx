//! S3-I86 Stage-3 exit-open contracts (post-I87).
//! Hinge landed · Stage-3 [x] 退出 · exit census exited · next Stage-4.
//! See docs/adr/stage3-exit-open.md.

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
fn s3_exit_open_adr_and_plan_locked() {
    let adr = repo_file("docs/adr/stage3-exit-open.md");
    assert!(adr.contains("S3-I86"), "exit-open ADR must name S3-I86");
    assert!(
        adr.contains("exit-open") || adr.contains("exit open"),
        "ADR must publish exit-open status"
    );
    assert!(
        adr.contains("Stage-3") && (adr.contains("`[~]`") || adr.contains("[~]")),
        "ADR Status must keep historical Stage-3 [~] for open window"
    );
    assert!(
        adr.contains("7/7"),
        "ADR must publish Stage-3 7/7"
    );
    assert!(
        adr.contains("S3-I85") || adr.contains("stage3-s3-07-claim"),
        "ADR must cite prior S3-I85 claim"
    );
    assert!(
        adr.contains("S3-I87") || adr.contains("stage3-exit"),
        "ADR Consequences must cite landed I87 / stage3-exit"
    );
    assert!(
        adr.contains("≠")
            && (adr.contains("stage3-exit") || adr.contains("`stage3-exit`")),
        "ADR Decision must Non-claim write stage3-exit that window"
    );
    assert!(
        adr.contains("Residual") || adr.contains("deferred"),
        "ADR/Gap must surface Residual draft"
    );
    assert!(
        repo_path("docs/adr/stage3-exit.md").exists(),
        "stage3-exit.md must exist after I87"
    );

    let census = repo_file("docs/census/stage3-exit-census.md");
    assert!(
        census.contains("S3-I86") && census.contains("S3-I87")
            && (census.contains("S3-I88") && (census.contains("S3-I89") || census.contains("Stage-4"))),
        "exit census must name I86 open / I87 exit"
    );
    assert!(
        census.contains("Current:** S3-I87") || census.contains("Current: S3-I87"),
        "census Current must advance to S3-I87"
    );
    assert!(
        census.contains("exit_status") && census.contains("exited"),
        "census must mark exit_status exited"
    );
    assert!(
        census.contains("7/7"),
        "census must publish Stage-3 7/7"
    );
    assert!(
        census.contains("S3-I88") && (census.contains("S3-I89") || census.contains("Stage-4")),
        "census ladder must next = Stage-4 / I88"
    );
    for claim in [
        "S3-I41", "S3-I47", "S3-I55", "S3-I71", "S3-I77", "S3-I81", "S3-I85",
    ] {
        assert!(
            census.contains(claim),
            "exit census Evidence must cite {claim}"
        );
    }
    assert!(
        census.contains("fib") || census.contains("0.5"),
        "Residual draft must cite S3-04 fib"
    );
    assert!(
        census.contains("hybrid") || census.contains("L010"),
        "Residual draft must cite S3-05 hybrid"
    );
    assert!(
        census.contains("rename") || census.contains("fence"),
        "Residual draft must cite S3-06 rename/fence"
    );
    assert!(
        census.contains("UsageGuide") || census.contains("crate"),
        "Residual draft must cite S3-07 UsageGuide/crate"
    );

    let prior = repo_file("docs/adr/stage3-s3-07-claim.md");
    assert!(
        prior.contains("S3-I86") || prior.contains("stage3-exit-open"),
        "I85 ADR must cite landed I86"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出 after I87; got: {stage3}"
    );
    assert!(
        plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") && (plan02.contains("S3-I89") || plan02.contains("Stage-4"))),
        "plan/02 must cite I86…I87 and Stage-4 next"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") && (plan01.contains("S3-I89") || plan01.contains("Stage-4"))),
        "plan/01 must cite I86…I87 and Stage-4"
    );
    assert!(
        plan01.contains("退出"),
        "plan/01 must publish Stage-3 exit"
    );
}

#[test]
fn s3_exit_open_census_script_hinges() {
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
        stdout.contains("current=S3-I87") && stdout.contains("next=S3-I89"),
        "script ladder must be I87→I89:\n{stdout}"
    );
    assert!(
        stdout.contains("stage3_status=x")
            && stdout.contains("stage3_checklist=7/7")
            && stdout.contains("exit_status=exited"),
        "script must emit exited / 7/7:\n{stdout}"
    );
    assert!(
        stdout.contains("stage3_exit_adr_present=1"),
        "stage3-exit.md must be present:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0)
    };
    assert!(
        parse("stage3_exit_open_helper=") >= 1,
        "expected stage3_exit_open_helper>=1\n{stdout}"
    );
    assert!(
        parse("stage3_claim_chain_helper=") >= 7,
        "expected stage3_claim_chain_helper>=7\n{stdout}"
    );
}
