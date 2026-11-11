//! S3-I70 S3-04 residual-scope (scoped acceptance + deferred <0.5) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-residual-scope.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_residual_scope_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-residual-scope.md");
    assert!(
        adr.contains("S3-I70"),
        "residual-scope ADR must name S3-I70"
    );
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must defer literal <0.5"
    );
    assert!(
        adr.contains("1.5") && adr.contains("1.8"),
        "ADR must cite interp ceiling 1.5–1.8"
    );
    assert!(
        adr.contains("S3-I71")
            && (adr.contains("claim") || adr.contains("stage3-s3-04-claim") || adr.contains("[x]")),
        "ADR must cite landed S3-I71 claim"
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
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I70 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I72"),
        "census ladder must name S3-I72 next"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-wall-strategy.md");
    assert!(
        prior.contains("S3-I70") || prior.contains("stage3-s3-04-residual-scope"),
        "I69 ADR must cite landed I70"
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
        s3_04.contains("scoped") || s3_04.contains("residual-scope") || s3_04.contains("deferred"),
        "S3-04 row must publish scoped acceptance; got: {s3_04}"
    );
    assert!(
        plan02.contains("S3-I70") || plan02.contains("residual-scope"),
        "plan/02 must cite S3-I70"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
}

#[test]
fn s3_04_residual_scope_census_script_emits_hinge() {
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
        "residual_scope_helper=",
        "wall_strategy_helper=",
        "fib30_s=",
        "fib30_interp_ceiling_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let residual = stdout
        .lines()
        .find(|l| l.starts_with("residual_scope_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        residual >= 1,
        "expected residual_scope_helper>=1, got {residual}\n{stdout}"
    );
}
