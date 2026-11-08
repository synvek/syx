//! S2-I265 S2-07 plan claim contracts.
//! Nails: claim ADR + plan/02 S2-07 [x]; scoped Non-claims; Stage-2 still [~].
//! See docs/adr/stage2-s2-07-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_07_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage2-s2-07-claim.md");
    assert!(adr.contains("S2-I265"), "S2-07 claim ADR must exist");
    assert!(
        adr.contains("S2-07") && adr.contains("`[x]`"),
        "ADR Status must claim S2-07 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
    assert!(
        adr.contains("CARGO_PKG_VERSION") || adr.contains("0.1.3"),
        "ADR Evidence must tie to semver inject"
    );
    assert!(
        adr.contains("S2-I266") || adr.contains("Stage-2 exit"),
        "ADR must lock next = Stage-2 exit"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s2_07_line = plan02
        .lines()
        .find(|l| l.contains("| S2-07 |"))
        .expect("plan/02 must have S2-07 row");
    assert!(
        s2_07_line.contains("| [x] |") || s2_07_line.contains("| `[x]` |"),
        "plan/02 S2-07 status must be [x]; got: {s2_07_line}"
    );
    assert!(
        plan02.contains("S2-I265") || plan02.contains("stage2-s2-07-claim"),
        "plan/02 must cite S2-I265 claim"
    );
    // After S2-I266, Stage-2 summary is [x] 退出; residual [~] rows (e.g. S1-04) may remain.
    assert!(
        plan02.contains("| **2** |"),
        "plan/02 must retain stage-2 summary row"
    );

    let census = repo_file("docs/stage2-version-census.md");
    assert!(
        census.contains("S2-I265"),
        "version census must name S2-I265"
    );

    let build_rs = repo_file("crates/syx-driver/build.rs");
    assert!(
        build_rs.contains("CARGO_PKG_VERSION"),
        "claim evidence: build.rs must still inject CARGO_PKG_VERSION"
    );

    let manifest = repo_file("packages/internal/manifest.json");
    assert!(
        manifest.contains("\"version\": \"0.1.3\"")
            || manifest.contains("\"version\":\"0.1.3\""),
        "claim evidence: manifest version must remain 0.1.3"
    );
}
