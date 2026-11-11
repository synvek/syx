//! S3-I52 S3-03 warm residual contracts.
//! Hinge landed · ≠ S3-03 [x] · ≠ hello < 0.15 claim · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-warm-residual.md · docs/census/stage3-stdlib-cache-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_warm_residual_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-warm-residual.md");
    assert!(adr.contains("S3-I52"), "warm-residual ADR must name S3-I52");
    assert!(
        adr.contains("SYX_CACHE_VERSION_TAG") || adr.contains("bake"),
        "ADR must name bake tag hinge"
    );
    assert!(
        adr.contains("load_from_env") && (adr.contains("Once") || adr.contains("OnceLock")),
        "ADR must name plugin Once"
    );
    assert!(
        adr.contains("trim") || adr.contains("trim-skip") || adr.contains("secondary trim"),
        "ADR must name trim skip"
    );
    assert!(
        adr.contains("S3-I53") && (adr.contains("cache-roundtrip") || adr.contains("roundtrip")),
        "ADR must cite landed S3-I53 cache roundtrip"
    );
    assert!(
        adr.contains("S3-I54") && (adr.contains("prelude-embed") || adr.contains("embed")),
        "ADR must cite landed S3-I54 prelude embed"
    );
    assert!(
        adr.contains("S3-I55") && (adr.contains("stage3-s3-03-claim") || adr.contains("claim")),
        "ADR must cite landed S3-I55 claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/census/stage3-stdlib-cache-census.md");
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("cache_tag_bake")
            && census.contains("load_from_env_once")
            && census.contains("trim_skip_warm"),
        "census must publish I52 structural KPIs"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let build = repo_file("crates/syx-driver/build.rs");
    assert!(
        build.contains("SYX_CACHE_VERSION_TAG"),
        "build.rs must emit SYX_CACHE_VERSION_TAG"
    );
    let fp = repo_file("crates/syx-driver/src/cache_fingerprint.rs");
    assert!(
        fp.contains("SYX_CACHE_VERSION_TAG") && fp.contains("S3-I52"),
        "cache_version_tag must prefer bake"
    );
    let loader = repo_file("crates/syx-host-plugin/src/loader.rs");
    assert!(
        loader.contains("load_from_env_inner") && loader.contains("OnceLock"),
        "load_from_env must OnceLock"
    );
    let run = repo_file("crates/syx-driver/src/run_exec.rs");
    assert!(
        run.contains("S3-I52") && run.contains("skip secondary trim"),
        "run_exec must document trim skip"
    );
    assert!(
        !run.contains("trim_program_to_entry_reachability(&mut program)"),
        "run_compiled must not unconditionally re-trim"
    );

    let prior = repo_file("docs/adr/stage3-s3-03-warm-fixed-tax.md");
    assert!(
        prior.contains("S3-I52") || prior.contains("stage3-s3-03-warm-residual"),
        "warm-fixed-tax ADR must cite landed I52"
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
        plan02.contains("S3-I52") || plan02.contains("warm-residual"),
        "plan/02 must cite S3-I52"
    );
}

#[test]
fn s3_03_warm_residual_census_script_emits_helpers() {
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
        "cache_tag_bake=",
        "load_from_env_once=",
        "trim_skip_warm=",
        "hello_warm_s=",
        "hello_prelude_hit_s=",
        "hello_nostd_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    for (name, min) in [
        ("cache_tag_bake", 1usize),
        ("load_from_env_once", 1usize),
        ("trim_skip_warm", 1usize),
    ] {
        let n = stdout
            .lines()
            .find(|l| l.starts_with(&format!("{name}=")))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        assert!(n >= min, "expected {name}>={min}, got {n}\n{stdout}");
    }
}
