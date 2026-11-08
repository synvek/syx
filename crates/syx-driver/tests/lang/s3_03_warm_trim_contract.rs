//! S3-I50 S3-03 warm program trim contracts.
//! Hinge landed · ≠ S3-03 [x] · ≠ hello < 0.15 claim · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-warm-program-trim.md · docs/stage3-stdlib-cache-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_warm_trim_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-warm-program-trim.md");
    assert!(adr.contains("S3-I50"), "warm-trim ADR must name S3-I50");
    assert!(
        adr.contains("trim_program_to_entry_reachability"),
        "ADR must name trim hinge"
    );
    assert!(
        adr.contains("S3-I51")
            && (adr.contains("warm-fixed-tax") || adr.contains("fixed tax")),
        "ADR must cite landed S3-I51 warm fixed tax"
    );
    assert!(
        adr.contains("S3-I52")
            && (adr.contains("warm-residual") || adr.contains("residual")),
        "ADR must cite landed S3-I52 warm residual"
    );
    assert!(
        adr.contains("S3-I53")
            && (adr.contains("roundtrip") || adr.contains("cache-roundtrip")),
        "ADR must cite landed S3-I53 cache roundtrip"
    );
    assert!(
        adr.contains("S3-I54")
            && (adr.contains("embed") || adr.contains("prelude-embed")),
        "ADR must cite landed S3-I54 prelude embed"
    );
    assert!(
        adr.contains("S3-I55")
            && (adr.contains("stage3-s3-03-claim") || adr.contains("claim")),
        "ADR must cite landed S3-I55 claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/stage3-stdlib-cache-census.md");
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("entry_cache_bytes") && census.contains("program_trim_helper"),
        "census must publish trim KPIs"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let trim_src = repo_file("crates/syx-codegen/src/program_trim.rs");
    assert!(
        trim_src.contains("pub fn trim_program_to_entry_reachability"),
        "codegen must define trim"
    );
    let linked = repo_file("crates/syx-driver/src/compile_linked.rs");
    assert!(
        linked.contains("trim_program_to_entry_reachability")
            && linked.contains("maybe_refresh_service_di_from_source"),
        "compile_linked must mount trim + DI skip"
    );
    let run = repo_file("crates/syx-driver/src/run_exec.rs");
    assert!(
        run.contains("trim_program_to_entry_reachability"),
        "run_exec must mount trim"
    );

    let prelude_adr = repo_file("docs/adr/stage3-s3-03-prelude-snapshot.md");
    assert!(
        prelude_adr.contains("S3-I50")
            || prelude_adr.contains("stage3-s3-03-warm-program-trim"),
        "prelude ADR must cite landed I50"
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
        plan02.contains("S3-I50") || plan02.contains("warm-program-trim"),
        "plan/02 must cite S3-I50"
    );
}

#[test]
fn s3_03_warm_trim_census_script_emits_trim_kpi() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_03_stdlib_cache_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run stdlib-cache census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trim = stdout
        .lines()
        .find(|l| l.starts_with("program_trim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        trim >= 1,
        "expected program_trim_helper>=1, got {trim}\n{stdout}"
    );
    assert!(
        stdout.contains("entry_cache_bytes="),
        "script must emit entry_cache_bytes, got:\n{stdout}"
    );
}
