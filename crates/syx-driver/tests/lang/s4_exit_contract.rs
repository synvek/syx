//! S3-I109 Stage-4 exit contracts.
//! Nails: stage4-exit ADR + plan Stage-4 [x] 退出; Residual; next post-exit residual.
//! See docs/adr/stage4-exit.md.

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
fn s4_exit_adr_and_plan_locked() {
    assert!(
        repo_path("docs/adr/stage4-exit.md").exists(),
        "stage4-exit.md must exist"
    );
    let adr = repo_file("docs/adr/stage4-exit.md");
    assert!(adr.contains("S3-I109"), "exit ADR must name S3-I109");
    assert!(
        adr.contains("Stage-4 exits") || adr.contains("Stage-4 exit"),
        "ADR must declare Stage-4 exits"
    );
    assert!(
        adr.contains("S3-I91")
            && adr.contains("S3-I95")
            && adr.contains("S3-I99")
            && adr.contains("S3-I103")
            && adr.contains("S3-I107"),
        "ADR Evidence must cite S4-01…05 claim windows"
    );
    assert!(
        adr.contains("S3-I108") || adr.contains("stage4-exit-open"),
        "ADR must cite exit-open prior"
    );
    assert!(
        adr.contains("Residual") || adr.contains("post-exit"),
        "ADR must publish post-exit Residual"
    );
    assert!(
        (adr.contains("manifest") || adr.contains("doctest"))
            && (adr.contains("registry") || adr.contains("packages.sy"))
            && (adr.contains("Sendable") || adr.contains("withTaskGroup") || adr.contains("TaskGroup"))
            && (adr.contains("makeIterator") || adr.contains("map"))
            && (adr.contains("native-sdk") || adr.contains("sysroot")),
        "ADR Residual must cover S4-01…05 deferred axes"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-5"),
        "ADR must Non-claim Stage-5"
    );
    assert!(
        adr.contains("Residual")
            && (adr.contains("维护") || adr.contains("maintenance") || adr.contains("post-exit")),
        "ADR Consequences must cite post-exit Residual / 维护"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 stage-4 row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "plan/02 Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        plan02.contains("S3-I109")
            && (plan02.contains("stage4-exit") || plan02.contains("退出")),
        "plan/02 must cite S3-I109 exit"
    );
    assert!(
        plan02.contains("5/5") || plan02.contains("S4-01"),
        "plan/02 must retain S4 checklist honesty"
    );
    assert!(
        plan02.contains("Residual") || plan02.contains("维护"),
        "plan/02 must tip = post-exit Residual / 维护"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I109")
            && (plan01.contains("[x]") && plan01.contains("退出")),
        "plan/01 must reflect Stage-4 [x] 退出 / S3-I109"
    );
    assert!(
        plan01.contains("Residual") || plan01.contains("维护"),
        "plan/01 must cite post-exit Residual / 维护"
    );

    let prior = repo_file("docs/adr/stage4-exit-open.md");
    assert!(
        prior.contains("S3-I109") || prior.contains("stage4-exit"),
        "I108 ADR must cite landed I109"
    );

    let census = repo_file("docs/stage4-exit-census.md");
    assert!(
        census.contains("Current:** S3-I109") || census.contains("Current: S3-I109"),
        "census Current must be S3-I109"
    );
    assert!(
        census.contains("exit_status") && census.contains("exited"),
        "census must mark exit_status exited"
    );
    assert!(
        census.contains("Residual")
            && (census.contains("维护") || census.contains("post-exit")),
        "census ladder must next = post-exit Residual"
    );
}

#[test]
fn s4_exit_census_script_hinges() {
    let root = repo_path(".");
    let output = std::process::Command::new("bash")
        .arg("scripts/s4_exit_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_exit_census.sh");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "stage4_exit_helper=",
        "stage4_exit_open_helper=",
        "stage4_claim_chain_helper=",
        "stage4_exit_adr_present=",
        "stage4_status=",
        "stage4_checklist=",
        "exit_status=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I109")
            && (stdout.contains("next=post-exit-residual")
                || stdout.contains("next=residual")),
        "script ladder must be I109→post-exit residual:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("stage4_checklist=5/5")
            && stdout.contains("exit_status=exited")
            && stdout.contains("stage4_exit_adr_present=1"),
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
    assert!(parse("stage4_exit_helper=") >= 1, "exit helper:\n{stdout}");
    assert!(
        parse("stage4_exit_open_helper=") >= 1,
        "exit-open helper:\n{stdout}"
    );
    assert!(
        parse("stage4_claim_chain_helper=") >= 5,
        "claim chain:\n{stdout}"
    );
}
