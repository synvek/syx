//! S3-I54 S3-03 prelude embed contracts.
//! Hinge landed · S3-I55 claimed S3-03 [x] · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-prelude-embed.md · docs/census/stage3-stdlib-cache-census.md.

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
fn s3_03_prelude_embed_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-prelude-embed.md");
    assert!(adr.contains("S3-I54"), "prelude-embed ADR must name S3-I54");
    assert!(
        adr.contains("include_bytes") || adr.contains("EMBEDDED_PRELUDE"),
        "ADR must name include_bytes hinge"
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
        census.contains("prelude_embed_bytes") && census.contains("prelude_embed_hit"),
        "census must publish embed KPIs"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let cache = repo_file("crates/syx-driver/src/compile_cache.rs");
    assert!(
        cache.contains("EMBEDDED_PRELUDE_SNAPSHOT")
            && cache.contains("PRELUDE_EMBED_HIT")
            && cache.contains("S3-I54"),
        "compile_cache must mount embed fallback"
    );
    let asset = repo_path("crates/syx-driver/assets/prelude_snapshot.bin");
    let meta = std::fs::metadata(&asset).expect("prelude_snapshot.bin must exist");
    assert!(
        meta.len() > 1024,
        "embedded asset must be non-trivial, got {} bytes",
        meta.len()
    );
    let gen = repo_file("scripts/gen_prelude_snapshot.sh");
    assert!(
        gen.contains("prelude_snapshot.bin") && gen.contains("prelude_vm_image.rkyv"),
        "gen script must write bincode + rkyv assets"
    );
    let gen_ps1 = repo_file("scripts/gen_prelude_snapshot.ps1");
    assert!(
        gen_ps1.contains("prelude_snapshot.bin")
            && gen_ps1.contains("prelude_vm_image.rkyv")
            && gen_ps1.contains("check_prelude_embed.ps1"),
        "Windows gen_prelude_snapshot.ps1 must mirror sh (bin + rkyv + embed check)"
    );
    let check_ps1 = repo_path("scripts/check_prelude_embed.ps1");
    assert!(
        check_ps1.is_file(),
        "scripts/check_prelude_embed.ps1 must exist for Windows regen verify"
    );
    let rkyv = repo_path("crates/syx-driver/assets/prelude_vm_image.rkyv");
    let rkyv_meta = std::fs::metadata(&rkyv).expect("prelude_vm_image.rkyv must exist");
    assert!(
        rkyv_meta.len() > 1024,
        "rkyv VM image must be non-trivial, got {} bytes",
        rkyv_meta.len()
    );

    let prior = repo_file("docs/adr/stage3-s3-03-cache-roundtrip.md");
    assert!(
        prior.contains("S3-I54") || prior.contains("stage3-s3-03-prelude-embed"),
        "cache-roundtrip ADR must cite landed I54"
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
        plan02.contains("S3-I54") || plan02.contains("prelude-embed"),
        "plan/02 must cite S3-I54"
    );
}

#[test]
fn s3_03_prelude_embed_census_script_emits_helpers() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_03_stdlib_cache_census.sh")
        .current_dir(repo_path("."))
        .output()
        .expect("run stdlib-cache census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "prelude_embed_bytes=",
        "prelude_embed_hit=",
        "prelude_embed_asset_bytes=",
        "hello_cold_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let asset_bytes = stdout
        .lines()
        .find(|l| l.starts_with("prelude_embed_asset_bytes="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        asset_bytes > 1024,
        "expected prelude_embed_asset_bytes>1024, got {asset_bytes}\n{stdout}"
    );
}
