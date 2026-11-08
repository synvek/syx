//! S3-I55 S3-03 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-03 [x]; scoped Non-claims; Stage-3 still [~].
//! See docs/adr/stage3-s3-03-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-03-claim.md");
    assert!(adr.contains("S3-I55"), "S3-03 claim ADR must exist");
    assert!(
        adr.contains("S3-03") && adr.contains("`[x]`"),
        "ADR Status must claim S3-03 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );
    assert!(
        adr.contains("0.15")
            && (adr.contains("hello_cold_s")
                || adr.contains("hello_warm_s")
                || adr.contains("hello_prelude_hit_s")),
        "ADR Evidence must cite hello startup KPIs"
    );
    assert!(
        adr.contains("S3-I54") || adr.contains("prelude-embed"),
        "ADR must cite I54 embed prior"
    );
    assert!(
        adr.contains("S3-04") && adr.contains("open"),
        "ADR must lock next = S3-04 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_03 = plan02
        .lines()
        .find(|l| l.contains("| S3-03 |"))
        .expect("plan/02 must have S3-03 row");
    assert!(
        s3_03.contains("| [x] |") || s3_03.contains("| `[x]` |"),
        "plan/02 S3-03 status must be [x]; got: {s3_03}"
    );
    assert!(
        plan02.contains("S3-I55") || plan02.contains("stage3-s3-03-claim"),
        "plan/02 must cite S3-I55 claim"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[~]") || stage3.contains("`[~]`"),
        "Stage-3 must stay [~]; got: {stage3}"
    );
    assert!(
        plan02.contains("S3-04")
            && (plan02.contains("open") || plan02.contains("热点") || plan02.contains("fib")),
        "plan/02 must lock next = S3-04"
    );

    let census = repo_file("docs/stage3-stdlib-cache-census.md");
    assert!(
        census.contains("S3-I55"),
        "stdlib census must name S3-I55"
    );
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("S3-04"),
        "census ladder must next = S3-04"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I55") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I55 S3-03 [x]"
    );
    assert!(
        plan01.contains("S3-04") || plan01.contains("open"),
        "plan/01 must lock next = S3-04"
    );
}

#[test]
fn s3_03_claim_census_script_hinges() {
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
        "prelude_embed_bytes=",
        "prelude_embed_hit=",
        "cache_roundtrip_helper=",
        "hello_cold_s=",
        "hello_warm_s=",
        "hello_prelude_hit_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let embed = stdout
        .lines()
        .find(|l| l.starts_with("prelude_embed_bytes="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(embed >= 1, "expected prelude_embed_bytes>=1, got {embed}");
}
