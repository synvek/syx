//! S3-I51 S3-03 warm fixed tax contracts.
//! Hinge landed · ≠ S3-03 [x] · ≠ hello < 0.15 claim · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-warm-fixed-tax.md · docs/census/stage3-stdlib-cache-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_warm_fixed_tax_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-warm-fixed-tax.md");
    assert!(adr.contains("S3-I51"), "warm-fixed-tax ADR must name S3-I51");
    assert!(
        adr.contains("cache-first") || adr.contains("cache_first") || adr.contains("try_incremental_cache_hit"),
        "ADR must name cache-first hinge"
    );
    assert!(
        adr.contains("OnceLock") || adr.contains("SYX_RUSTC_VERSION"),
        "ADR must name fingerprint OnceLock / compile-time rustc"
    );
    assert!(
        adr.contains("lazy") || adr.contains("ensure_host_fn"),
        "ADR must name lazy hosts"
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

    let census = repo_file("docs/census/stage3-stdlib-cache-census.md");
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("cache_first_helper")
            && census.contains("fingerprint_once_lock")
            && census.contains("lazy_host_helper"),
        "census must publish I51 structural KPIs"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let linked = repo_file("crates/syx-driver/src/compile_linked.rs");
    assert!(
        linked.contains("try_incremental_cache_hit") && linked.contains("S3-I51"),
        "compile_linked must mount cache-first"
    );
    let fp = repo_file("crates/syx-driver/src/cache_fingerprint.rs");
    assert!(
        fp.contains("OnceLock")
            && fp.contains("witness_table_digest")
            && fp.contains("SYX_RUSTC_VERSION"),
        "cache_fingerprint must OnceLock digests + compile-time rustc"
    );
    let build = repo_file("crates/syx-driver/build.rs");
    assert!(
        build.contains("SYX_RUSTC_VERSION"),
        "build.rs must emit SYX_RUSTC_VERSION"
    );
    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("ensure_host_fn") && vm.contains("S3-I51"),
        "VM must mount lazy host"
    );

    let trim_adr = repo_file("docs/adr/stage3-s3-03-warm-program-trim.md");
    assert!(
        trim_adr.contains("S3-I51")
            || trim_adr.contains("stage3-s3-03-warm-fixed-tax"),
        "warm-trim ADR must cite landed I51"
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
        plan02.contains("S3-I51") || plan02.contains("warm-fixed-tax"),
        "plan/02 must cite S3-I51"
    );
}

#[test]
fn s3_03_warm_fixed_tax_census_script_emits_helpers() {
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
    for key in [
        "cache_first_helper=",
        "fingerprint_once_lock=",
        "lazy_host_helper=",
        "hello_warm_s=",
        "hello_prelude_hit_s=",
        "hello_nostd_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let cache_first = stdout
        .lines()
        .find(|l| l.starts_with("cache_first_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        cache_first >= 1,
        "expected cache_first_helper>=1, got {cache_first}\n{stdout}"
    );
    let once = stdout
        .lines()
        .find(|l| l.starts_with("fingerprint_once_lock="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        once >= 1,
        "expected fingerprint_once_lock>=1, got {once}\n{stdout}"
    );
    let lazy = stdout
        .lines()
        .find(|l| l.starts_with("lazy_host_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        lazy >= 1,
        "expected lazy_host_helper>=1, got {lazy}\n{stdout}"
    );
}
