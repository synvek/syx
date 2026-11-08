//! S3-I42…I47 S3-02 mono-display census contracts.
//! I42–I46 construct · I47 scoped claim.
//! S3-02 [x] · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-02-claim.md · docs/stage3-mono-display-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_02_mono_display_census_baseline() {
    let census = repo_file("docs/stage3-mono-display-census.md");
    assert!(census.contains("S3-I42"), "census must name S3-I42 open");
    assert!(census.contains("S3-I43"), "census must name S3-I43 demangle");
    assert!(census.contains("S3-I44"), "census must name S3-I44 debug stack");
    assert!(census.contains("S3-I45"), "census must name S3-I45 runtime");
    assert!(census.contains("S3-I46"), "census must name S3-I46 meta");
    assert!(census.contains("S3-I47"), "census must name S3-I47 claim");
    assert!(
        census.contains("Current:** S3-I47") || census.contains("Current: S3-I47"),
        "census Current must be S3-I47"
    );
    assert!(
        census.contains("user_facing_meta_type_name_helper"),
        "census must publish meta helper KPI"
    );
    assert!(
        census.contains("Claimed") || census.contains("claim"),
        "census must reflect claim"
    );
    assert!(
        census.contains("S3-03"),
        "census ladder must name S3-03 next"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );

    let open_adr = repo_file("docs/adr/stage3-s3-02-open.md");
    assert!(open_adr.contains("S3-I42"));
    assert!(
        open_adr.contains("S3-I47")
            && open_adr.contains("stage3-s3-02-claim"),
        "open ADR must cite landed S3-I47 claim"
    );
    assert!(
        open_adr.contains("S3-03"),
        "open ADR must lock next = S3-03"
    );

    let meta_adr = repo_file("docs/adr/stage3-s3-02-meta-type-display.md");
    assert!(meta_adr.contains("S3-I46"), "meta-type ADR must exist");
    assert!(
        meta_adr.contains("S3-I47")
            && meta_adr.contains("stage3-s3-02-claim"),
        "meta ADR must cite landed S3-I47"
    );

    let claim_adr = repo_file("docs/adr/stage3-s3-02-claim.md");
    assert!(claim_adr.contains("S3-I47"), "claim ADR must exist");
    assert!(
        claim_adr.contains("S3-02") && claim_adr.contains("`[x]`"),
        "claim ADR must Status S3-02 [x]"
    );

    let script = repo_file("scripts/s3_02_mono_display_census.sh");
    assert!(
        script.contains("user_facing_meta_type_name_helper"),
        "census script must emit meta helper KPI"
    );
    assert!(
        script.contains("user_facing_runtime_type_name_helper")
            && script.contains("object_ref_type_name"),
        "census script must keep prior KPIs"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_02 = plan02
        .lines()
        .find(|l| l.contains("| S3-02 |"))
        .expect("S3-02 row");
    assert!(
        s3_02.contains("| [x] |") || s3_02.contains("| `[x]` |"),
        "S3-02 must be [x]; got: {s3_02}"
    );
    assert!(
        plan02.contains("S3-I47")
            || plan02.contains("stage3-s3-02-claim")
    );
    assert!(
        plan02.contains("S3-03"),
        "plan/02 must lock next = S3-03"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("stage-3 row");
    assert!(stage3.contains("[~]") || stage3.contains("`[~]`"));
}

#[test]
fn s3_02_mono_display_census_script_runs() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_02_mono_display_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run mono-display census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("type_id_to_name_named_passthrough=0"),
        "named passthrough must stay 0, got:\n{stdout}"
    );
    for key in [
        "user_facing_type_name_helper=",
        "user_facing_mir_fn_name_helper=",
        "user_facing_runtime_type_name_helper=",
        "user_facing_meta_type_name_helper=",
    ] {
        let v = stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        assert!(v >= 1, "expected {key}>=1, got {v}\n{stdout}");
    }
    assert!(
        stdout.contains("mangle_name_mono_def=1"),
        "expected mangle_name_mono_def=1, got:\n{stdout}"
    );
}
