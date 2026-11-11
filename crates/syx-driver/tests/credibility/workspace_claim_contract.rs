//! S2-I262 S2-06 plan claim contracts.
//! Nails: claim ADR + plan/02 S2-06 [x]; scoped Non-claims; Stage-2 still [~].
//! See docs/adr/stage2-s2-06-claim.md.

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
fn s2_06_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage2-s2-06-claim.md");
    assert!(adr.contains("S2-I262"), "S2-06 claim ADR must exist");
    assert!(
        adr.contains("S2-06") && adr.contains("`[x]`"),
        "ADR Status must claim S2-06 [x]"
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
        adr.contains("≠") && adr.contains("S2-07"),
        "ADR must Non-claim open/complete S2-07 this window"
    );
    assert!(
        adr.contains("S2-07") && (adr.contains("open") || adr.contains("S2-I263")),
        "ADR must lock next = S2-07 open"
    );
    assert!(
        adr.contains("exclude") || adr.contains("syx-gui") || adr.contains("census"),
        "ADR Evidence must tie to exclude / census"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s2_06_line = plan02
        .lines()
        .find(|l| l.contains("| S2-06 |"))
        .expect("plan/02 must have S2-06 row");
    assert!(
        s2_06_line.contains("| [x] |") || s2_06_line.contains("| `[x]` |"),
        "plan/02 S2-06 status must be [x]; got: {s2_06_line}"
    );
    assert!(
        plan02.contains("S2-I262") || plan02.contains("stage2-s2-06-claim"),
        "plan/02 must cite S2-I262 claim"
    );
    assert!(
        plan02.contains("| **2** |") && plan02.contains("`[~]`"),
        "plan/02 stage-2 summary must remain [~]"
    );

    let census = repo_file("docs/census/stage2-workspace-census.md");
    assert!(
        census.contains("S2-I262"),
        "workspace census must name S2-I262"
    );

    let root_toml = repo_file("Cargo.toml");
    assert!(
        root_toml.contains("exclude = [\"crates/syx-gui\"]")
            || root_toml.contains("exclude = [\"crates/syx-gui\","),
        "claim evidence: root must still exclude crates/syx-gui"
    );
    assert!(
        repo_path("crates/syx-gui").is_dir(),
        "scoped residual: syx-gui directory may remain"
    );

    let vm_toml = repo_file("crates/syx-vm/Cargo.toml");
    assert!(
        vm_toml.contains("gui-syx-isolated") && vm_toml.contains("path = \"../syx-gui\""),
        "scoped residual: vm path opt-in must remain"
    );

    let ci = repo_file(".github/workflows/ci.yml");
    assert!(
        ci.contains("--manifest-path crates/syx-gui/Cargo.toml"),
        "CI must build syx-gui via manifest-path after exclude"
    );
}
