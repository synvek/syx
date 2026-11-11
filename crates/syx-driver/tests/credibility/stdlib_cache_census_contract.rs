//! S3-I48…I55 S3-03 stdlib-cache census contracts.
//! I48 open · I49–I54 construct · I55 scoped claim.
//! S3-03 [x] · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-claim.md · docs/census/stage3-stdlib-cache-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_stdlib_cache_census_baseline() {
    let census = repo_file("docs/census/stage3-stdlib-cache-census.md");
    assert!(census.contains("S3-I48"), "census must name S3-I48 open");
    assert!(
        census.contains("S3-I49"),
        "census must name S3-I49 prelude snapshot"
    );
    assert!(
        census.contains("S3-I50"),
        "census must name S3-I50 warm trim"
    );
    assert!(
        census.contains("S3-I51"),
        "census must name S3-I51 warm fixed tax"
    );
    assert!(
        census.contains("S3-I52"),
        "census must name S3-I52 warm residual"
    );
    assert!(
        census.contains("S3-I53"),
        "census must name S3-I53 cache roundtrip"
    );
    assert!(
        census.contains("S3-I54"),
        "census must name S3-I54 prelude embed"
    );
    assert!(census.contains("S3-I55"), "census must name S3-I55 claim");
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("Claimed") || census.contains("claim"),
        "census must reflect claim"
    );
    assert!(
        census.contains("hello_cold_s")
            && census.contains("hello_warm_s")
            && census.contains("hello_nostd_s"),
        "census must publish hello cold/warm/nostd KPIs"
    );
    assert!(
        census.contains("dedicated_prelude_cache_hit"),
        "census must publish dedicated prelude cache KPI"
    );
    assert!(
        census.contains("hello_target_s") && census.contains("0.15"),
        "census must name 0.15 target"
    );
    assert!(
        census.contains("embedded_std_include_str")
            && census.contains("ensure_prelude_fn")
            && census.contains("cached_module_unit"),
        "census must publish structural cache/prelude axes"
    );
    assert!(
        census.contains("S3-04"),
        "census ladder must name S3-04 next"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );

    let open_adr = repo_file("docs/adr/stage3-s3-03-open.md");
    assert!(open_adr.contains("S3-I48"));
    assert!(
        open_adr.contains("S3-I55") && open_adr.contains("stage3-s3-03-claim"),
        "open ADR must cite landed S3-I55 claim"
    );
    assert!(
        open_adr.contains("S3-04"),
        "open ADR must lock next = S3-04"
    );
    assert!(
        open_adr.contains("S3-I49") && open_adr.contains("stage3-s3-03-prelude-snapshot"),
        "open ADR must cite landed S3-I49 prelude snapshot"
    );
    assert!(
        open_adr.contains("S3-I50") && open_adr.contains("stage3-s3-03-warm-program-trim"),
        "open ADR must cite landed S3-I50 warm trim"
    );
    assert!(
        open_adr.contains("S3-I51") && open_adr.contains("stage3-s3-03-warm-fixed-tax"),
        "open ADR must cite landed S3-I51 warm fixed tax"
    );
    assert!(
        open_adr.contains("S3-I52") && open_adr.contains("stage3-s3-03-warm-residual"),
        "open ADR must cite landed S3-I52 warm residual"
    );
    assert!(
        open_adr.contains("S3-I53") && open_adr.contains("stage3-s3-03-cache-roundtrip"),
        "open ADR must cite landed S3-I53 cache roundtrip"
    );
    assert!(
        open_adr.contains("S3-I54") && open_adr.contains("stage3-s3-03-prelude-embed"),
        "open ADR must cite landed S3-I54 prelude embed"
    );

    let claim_adr = repo_file("docs/adr/stage3-s3-03-claim.md");
    assert!(claim_adr.contains("S3-I55"), "claim ADR must exist");
    assert!(
        claim_adr.contains("S3-03") && claim_adr.contains("`[x]`"),
        "claim ADR must Status S3-03 [x]"
    );

    let script = repo_file("scripts/s3_03_stdlib_cache_census.sh");
    assert!(
        script.contains("hello_cold_s") && script.contains("hello_warm_s"),
        "census script must emit hello timings"
    );
    assert!(
        script.contains("embedded_std_include_str")
            && script.contains("ensure_prelude_fn")
            && script.contains("cached_module_unit"),
        "census script must emit structural KPIs"
    );
    assert!(
        script.contains("hello_target_s=0.15"),
        "census script must emit target KPI"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_03 = plan02
        .lines()
        .find(|l| l.contains("| S3-03 |"))
        .expect("S3-03 row");
    assert!(
        s3_03.contains("| [x] |") || s3_03.contains("| `[x]` |"),
        "S3-03 must be [x]; got: {s3_03}"
    );
    assert!(
        plan02.contains("S3-I55") || plan02.contains("stage3-s3-03-claim"),
        "plan/02 must cite S3-I55 claim"
    );
    assert!(plan02.contains("S3-04"), "plan/02 must lock next = S3-04");
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("stage-3 row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I55") && plan01.contains("S3-03"),
        "plan/01 must lock current toward S3-I55 claim"
    );

    let claim_s302 = repo_file("docs/adr/stage3-s3-02-claim.md");
    assert!(
        claim_s302.contains("S3-I48") || claim_s302.contains("stage3-s3-03-open"),
        "S3-02 claim ADR must cite landed S3-03 open"
    );
}

#[test]
fn s3_03_stdlib_cache_census_script_runs() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_03_stdlib_cache_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run stdlib-cache census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "embedded_std_include_str=",
        "ensure_prelude_fn=",
        "cached_module_unit=",
        "incremental_cache_path_fn=",
        "dedicated_prelude_cache_hit=",
        "hello_cold_s=",
        "hello_prelude_hit_s=",
        "hello_warm_s=",
        "hello_nostd_s=",
        "hello_target_s=0.15",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("I55 claim")
            || stdout.contains("claim [x]")
            || stdout.contains("≠ Stage-3"),
        "script must note I55 claim / Stage-3 OUT, got:\n{stdout}"
    );
}
