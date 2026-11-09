//! S3-I108 Stage-4 exit-open contracts (post-I109).
//! Hinge landed · Stage-4 [x] 退出 · exit census exited · next post-exit residual.
//! See docs/adr/stage4-exit-open.md.

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
fn s4_exit_open_adr_and_plan_locked() {
    let adr = repo_file("docs/adr/stage4-exit-open.md");
    assert!(adr.contains("S3-I108"), "exit-open ADR must name S3-I108");
    assert!(
        adr.contains("exit-open") || adr.contains("exit open"),
        "ADR must publish exit-open status"
    );
    assert!(
        adr.contains("Stage-4") && (adr.contains("`[~]`") || adr.contains("[~]")),
        "ADR Status must keep historical Stage-4 [~] for open window"
    );
    assert!(
        adr.contains("5/5"),
        "ADR must publish Stage-4 5/5"
    );
    assert!(
        adr.contains("S3-I107") || adr.contains("stage4-s4-05-claim"),
        "ADR must cite prior S3-I107 claim"
    );
    assert!(
        adr.contains("S3-I109") || adr.contains("stage4-exit"),
        "ADR Consequences must cite landed I109 / stage4-exit"
    );
    assert!(
        adr.contains("≠")
            && (adr.contains("stage4-exit") || adr.contains("`stage4-exit`")),
        "ADR Decision must Non-claim write stage4-exit that window"
    );
    assert!(
        adr.contains("Residual") || adr.contains("deferred"),
        "ADR/Gap must surface Residual draft"
    );
    assert!(
        repo_path("docs/adr/stage4-exit.md").exists(),
        "stage4-exit.md must exist after I109"
    );

    let census = repo_file("docs/stage4-exit-census.md");
    assert!(
        census.contains("S3-I108") && census.contains("S3-I109"),
        "exit census must name I108 open / I109 exit"
    );
    assert!(
        census.contains("Current:** S3-I109") || census.contains("Current: S3-I109"),
        "census Current must advance to S3-I109"
    );
    assert!(
        census.contains("exit_status") && census.contains("exited"),
        "census must mark exit_status exited"
    );
    assert!(
        census.contains("5/5"),
        "census must publish Stage-4 5/5"
    );
    assert!(
        census.contains("Residual")
            && (census.contains("维护") || census.contains("post-exit")),
        "census ladder must next = post-exit Residual"
    );
    for claim in [
        "S3-I91", "S3-I95", "S3-I99", "S3-I103", "S3-I107",
    ] {
        assert!(
            census.contains(claim),
            "exit census Evidence must cite {claim}"
        );
    }
    assert!(
        census.contains("native-sdk")
            || census.contains("sysroot")
            || census.contains("marketplace"),
        "Residual draft must cite S4-05 deferred axes"
    );
    assert!(
        census.contains("makeIterator") || census.contains("map"),
        "Residual draft must cite S4-04 deferred"
    );

    let prior = repo_file("docs/adr/stage4-s4-05-claim.md");
    assert!(
        prior.contains("S3-I108") || prior.contains("stage4-exit-open"),
        "I107 ADR must cite landed I108"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 must retain stage-4 summary row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出 after I109; got: {stage4}"
    );
    assert!(
        plan02.contains("S3-I108")
            && plan02.contains("S3-I109")
            && (plan02.contains("exit-open") || plan02.contains("stage4-exit")),
        "plan/02 must cite I108…I109"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I108")
            && plan01.contains("S3-I109")
            && plan01.contains("退出"),
        "plan/01 must cite I108…I109 and Stage-4 exit"
    );
}

#[test]
fn s4_exit_open_census_script_hinges() {
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
            && stdout.contains("exit_status=exited"),
        "script must emit exited / 5/5:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_exit_adr_present=1"),
        "stage4-exit.md must be present:\n{stdout}"
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
        parse("stage4_exit_open_helper=") >= 1,
        "expected stage4_exit_open_helper>=1\n{stdout}"
    );
    assert!(
        parse("stage4_claim_chain_helper=") >= 5,
        "expected stage4_claim_chain_helper>=5\n{stdout}"
    );
}
