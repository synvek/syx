//! S2-I263 S2-07 version census contracts (open baseline).
//! Nails: open ADR + census + script KPIs; ≠ S2-07 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-07-open.md · docs/census/stage2-version-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_07_version_census_baseline() {
    let census = repo_file("docs/census/stage2-version-census.md");
    assert!(
        census.contains("S2-I263"),
        "version census must name S2-I263 baseline"
    );
    assert!(
        census.contains("0.1.3") && census.contains("117.0"),
        "census must nail dual-track 0.1.3 vs 117.0"
    );
    assert!(
        census.contains("≠") && census.contains("S2-07"),
        "census must keep Non-claim honesty for S2-07"
    );
    assert!(
        census.contains("S2-I264") && census.contains("CARGO_PKG_VERSION"),
        "census ladder must name S2-I264 inject CARGO_PKG_VERSION"
    );

    let adr = repo_file("docs/adr/stage2-s2-07-open.md");
    assert!(adr.contains("S2-I263"), "S2-07 open ADR must exist");
    assert!(
        adr.contains("Open S2-07") || adr.contains("**Open S2-07**"),
        "ADR must open S2-07 workstream"
    );
    assert!(
        adr.contains("S2-I264") && adr.contains("CARGO_PKG_VERSION"),
        "ADR must lock next knife = S2-I264 inject CARGO_PKG_VERSION"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-07") || adr.contains("[x]")),
        "ADR must Non-claim S2-07 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );

    let script = repo_file("scripts/s2_07_version_census.sh");
    assert!(
        script.contains("workspace_semver"),
        "census script must emit workspace_semver"
    );
    assert!(
        script.contains("manifest_has_117") && script.contains("evergreen_nails_117"),
        "census script must emit manifest/evergreen KPIs"
    );
    assert!(
        script.contains("build_rs_has_117") && script.contains("manifest_phase_139"),
        "census script must emit build.rs / phase KPIs"
    );

    // Honesty: workspace semver remains 0.1.3; public inject is owned by I264 contract.
    let root_toml = repo_file("Cargo.toml");
    assert!(
        root_toml.contains("version = \"0.1.3\""),
        "workspace must still advertise crate semver 0.1.3"
    );

    // Plan S2-07 status is owned by s2_07_claim_contract after S2-I265.
    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    assert!(
        plan02.contains("S2-I263") || plan02.contains("stage2-s2-07-open"),
        "plan/02 must cite S2-I263 open"
    );
}
