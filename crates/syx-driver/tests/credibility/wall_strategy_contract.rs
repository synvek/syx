//! S3-I69 S3-04 wall-strategy (interp ceiling + whitelist + dual-axis) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-wall-strategy.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_wall_strategy_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-wall-strategy.md");
    assert!(adr.contains("S3-I69"), "wall-strategy ADR must name S3-I69");
    assert!(
        adr.contains("fib30_interp_ceiling") || adr.contains("residual") || adr.contains("ceiling"),
        "ADR must publish interp residual ceiling"
    );
    assert!(
        adr.contains("JIT-off") || adr.contains("JIT off") || adr.contains("fib30_s"),
        "ADR must lock claim primary = JIT-off fib30_s"
    );
    assert!(
        adr.contains("SYX_JIT_TIER2") || adr.contains("fib30_tier2"),
        "ADR must whitelist Tier2 secondary axis"
    );
    assert!(
        adr.contains("S3-I70")
            && (adr.contains("residual-scope") || adr.contains("stage3-s3-04-residual-scope")),
        "ADR must cite landed S3-I70 residual-scope"
    );
    assert!(
        adr.contains("≠") && (adr.contains("[x]") || adr.contains("0.5") || adr.contains("Copy")),
        "ADR must Non-claim S3-04 [x] / 0.5"
    );

    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("wall_strategy_helper")
            && census.contains("fib30_interp_ceiling_s")
            && census.contains("fib30_tier2_s"),
        "census must publish I69 KPIs"
    );
    assert!(
        census.contains("S3-I72"),
        "census ladder must name S3-I72 next"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-claim-prep.md");
    assert!(
        prior.contains("S3-I69") || prior.contains("stage3-s3-04-wall-strategy"),
        "I68 ADR must cite landed I69"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_04 = plan02
        .lines()
        .find(|l| l.contains("| S3-04 |"))
        .expect("S3-04 row");
    assert!(
        s3_04.contains("| [x] |") || s3_04.contains("| `[x]` |"),
        "S3-04 must be [x] after I71 claim; got: {s3_04}"
    );
    assert!(
        plan02.contains("S3-I69") || plan02.contains("wall-strategy"),
        "plan/02 must cite S3-I69"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
}

#[test]
fn s3_04_wall_strategy_census_script_emits_hinge() {
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
        "wall_strategy_helper=",
        "claim_prep_helper=",
        "fib30_s=",
        "fib30_tier2_s=",
        "fib30_interp_ceiling_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let wall = stdout
        .lines()
        .find(|l| l.starts_with("wall_strategy_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        wall >= 1,
        "expected wall_strategy_helper>=1, got {wall}\n{stdout}"
    );
}
