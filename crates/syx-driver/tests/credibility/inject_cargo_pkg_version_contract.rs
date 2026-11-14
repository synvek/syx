//! S2-I264 S2-07 inject CARGO_PKG_VERSION contracts.
//! Nails: build.rs inject + manifest 0.1.3 + Non-claim S2-07 [x].
//! See docs/adr/stage2-s2-07-inject-cargo-pkg-version.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_07_inject_cargo_pkg_version_locked() {
    let adr = repo_file("docs/adr/stage2-s2-07-inject-cargo-pkg-version.md");
    assert!(adr.contains("S2-I264"), "inject ADR must name S2-I264");
    assert!(
        adr.contains("CARGO_PKG_VERSION"),
        "ADR must decide CARGO_PKG_VERSION inject"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-07"),
        "ADR must Non-claim S2-07 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
    assert!(
        adr.contains("S2-I265") && (adr.contains("claim") || adr.contains("`[x]`")),
        "ADR must lock next = S2-I265 S2-07 claim"
    );

    let build_rs = repo_file("crates/syx-driver/build.rs");
    assert!(
        build_rs.contains("CARGO_PKG_VERSION"),
        "build.rs must inject CARGO_PKG_VERSION"
    );
    assert!(
        !build_rs.contains("\"version\": \"117.0\""),
        "build.rs must not hardcode marketing 117.0 version"
    );

    let manifest = repo_file("packages/internal/manifest.json");
    assert!(
        manifest.contains("\"version\": \"0.2.2\"") || manifest.contains("\"version\":\"0.2.2\""),
        "checked-in manifest version must be 0.2.2"
    );
    assert!(
        !manifest.contains("\"version\": \"117.0\""),
        "checked-in manifest must not advertise 117.0"
    );

    let caps = repo_file("packages/internal/capabilities.toml");
    assert!(
        caps.contains("syx_version = \"0.2.2\""),
        "capabilities syx_version must be 0.2.2"
    );

    let smoke = repo_file("crates/syx-driver/tests/evergreen/smoke.rs");
    assert!(
        smoke.contains("CARGO_PKG_VERSION"),
        "evergreen smoke must assert CARGO_PKG_VERSION"
    );

    let census = repo_file("docs/census/stage2-version-census.md");
    assert!(
        census.contains("S2-I264"),
        "version census Current must name S2-I264"
    );

    // Plan S2-07 status is owned by s2_07_claim_contract after S2-I265.
    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    assert!(
        plan02.contains("S2-I264") || plan02.contains("stage2-s2-07-inject-cargo-pkg-version"),
        "plan/02 must cite S2-I264 inject"
    );
}
