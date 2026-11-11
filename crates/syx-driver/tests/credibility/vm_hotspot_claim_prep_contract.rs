//! S3-I68 S3-04 claim-prep (examples/fib + fib criterion) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-claim-prep.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn s3_04_claim_prep_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-claim-prep.md");
    assert!(adr.contains("S3-I68"), "claim-prep ADR must name S3-I68");
    assert!(
        adr.contains("examples/fib") || adr.contains("fib_criterion") || adr.contains("claim"),
        "ADR must name claim-prep hinge"
    );
    assert!(
        adr.contains("0.5")
            && (adr.contains("判据") || adr.contains("criteria") || adr.contains("draft")),
        "ADR must publish claim criteria draft"
    );
    assert!(
        adr.contains("S3-I69")
            && (adr.contains("wall")
                || adr.contains("strategy")
                || adr.contains("stage3-s3-04-wall-strategy")),
        "ADR must cite landed S3-I69 wall-strategy"
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
        census.contains("claim_prep_helper")
            && census.contains("examples_fib")
            && census.contains("fib_criterion_bench"),
        "census must publish I68 KPIs"
    );
    assert!(
        census.contains("S3-I72"),
        "census ladder must name S3-I72 next"
    );

    assert!(
        repo_path("examples/fib.sy").is_file(),
        "examples/fib.sy must exist"
    );
    assert!(
        repo_path("examples/fib20.sy").is_file(),
        "examples/fib20.sy must exist"
    );
    assert!(
        repo_path("crates/syx-driver/benches/fib_criterion.rs").is_file(),
        "fib_criterion bench must exist"
    );
    let cargo = repo_file("crates/syx-driver/Cargo.toml");
    assert!(
        cargo.contains("fib_criterion"),
        "Cargo.toml must register fib_criterion bench"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-can-stack-bind-cache.md");
    assert!(
        prior.contains("S3-I68") || prior.contains("stage3-s3-04-claim-prep"),
        "I67 ADR must cite landed I68"
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
        plan02.contains("S3-I68") || plan02.contains("claim-prep"),
        "plan/02 must cite S3-I68"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
}

#[test]
fn s3_04_claim_prep_census_script_emits_hinge() {
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
        "claim_prep_helper=",
        "examples_fib=",
        "fib_criterion_bench=",
        "can_stack_bind_cache_helper=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("claim_prep_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected claim_prep_helper>=1, got {claim}\n{stdout}"
    );
    let examples = stdout
        .lines()
        .find(|l| l.starts_with("examples_fib="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        examples >= 1,
        "expected examples_fib>=1, got {examples}\n{stdout}"
    );
    let bench = stdout
        .lines()
        .find(|l| l.starts_with("fib_criterion_bench="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        bench >= 1,
        "expected fib_criterion_bench>=1, got {bench}\n{stdout}"
    );
}
