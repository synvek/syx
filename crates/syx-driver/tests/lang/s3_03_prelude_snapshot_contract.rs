//! S3-I49 S3-03 prelude disk snapshot contracts.
//! Hinge landed · ≠ S3-03 [x] · ≠ hello < 0.15 claim · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-prelude-snapshot.md · docs/stage3-stdlib-cache-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_prelude_snapshot_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-prelude-snapshot.md");
    assert!(adr.contains("S3-I49"), "prelude-snapshot ADR must name S3-I49");
    assert!(
        adr.contains("load_prelude_snapshot") || adr.contains("PreludeSnapshot"),
        "ADR must name load/store hinge"
    );
    assert!(
        adr.contains("S3-I50")
            && (adr.contains("warm") || adr.contains("stage3-s3-03-warm-program-trim")),
        "ADR must cite landed S3-I50 warm trim"
    );
    assert!(
        adr.contains("S3-I51")
            && (adr.contains("fixed tax") || adr.contains("warm-fixed-tax")),
        "ADR must cite landed S3-I51 warm fixed tax"
    );
    assert!(
        adr.contains("S3-I52")
            && (adr.contains("residual") || adr.contains("warm-residual")),
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
        census.contains("dedicated_prelude_cache_hit") && census.contains("≥1"),
        "census must show dedicated_prelude_cache_hit≥1"
    );
    assert!(
        census.contains("hello_prelude_hit_s"),
        "census must publish hello_prelude_hit_s"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let open_adr = repo_file("docs/adr/stage3-s3-03-open.md");
    assert!(
        open_adr.contains("S3-I49")
            && open_adr.contains("stage3-s3-03-prelude-snapshot"),
        "open ADR must cite landed S3-I49"
    );

    let src = repo_file("crates/syx-driver/src/compile_cache.rs");
    assert!(
        src.contains("fn load_prelude_snapshot")
            && src.contains("fn store_prelude_snapshot")
            && src.contains("PRELUDE_SNAPSHOT_HIT")
            && src.contains("struct PreludeSnapshot"),
        "compile_cache must define prelude snapshot hinge"
    );
    let glue = repo_file("crates/syx-driver/src/prelude_glue.rs");
    assert!(
        glue.contains("load_prelude_snapshot") && glue.contains("store_prelude_snapshot"),
        "ensure_prelude must call load/store"
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
        plan02.contains("S3-I49") || plan02.contains("prelude-snapshot"),
        "plan/02 must cite S3-I49"
    );
}

#[test]
fn s3_03_prelude_snapshot_census_script_emits_hit_kpi() {
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
    let hit = stdout
        .lines()
        .find(|l| l.starts_with("dedicated_prelude_cache_hit="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        hit >= 1,
        "expected dedicated_prelude_cache_hit>=1, got {hit}\n{stdout}"
    );
    assert!(
        stdout.contains("hello_prelude_hit_s="),
        "script must emit hello_prelude_hit_s, got:\n{stdout}"
    );
    assert!(
        stdout.contains("I55 claim")
            || stdout.contains("claim [x]")
            || stdout.contains("≠ Stage-3"),
        "script must note I55 claim / Stage-3 OUT, got:\n{stdout}"
    );
}
