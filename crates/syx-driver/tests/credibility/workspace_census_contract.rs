//! S2-I260 S2-06 workspace slim census contracts (open baseline).
//! Nails: open ADR + census + script KPIs; ≠ S2-06 [x] / ≠ Stage-2 exit.
//! Honesty after S2-I261: gui may be workspace-excluded while directory remains.
//! See docs/adr/stage2-s2-06-open.md · docs/census/stage2-workspace-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_06_workspace_census_baseline() {
    let census = repo_file("docs/census/stage2-workspace-census.md");
    assert!(
        census.contains("S2-I260"),
        "workspace census must name S2-I260 baseline"
    );
    assert!(
        census.contains("37") && census.contains("syx-gui"),
        "census must nail directory count 37 and syx-gui"
    );
    assert!(
        census.contains("crates/*") || census.contains("`[\"crates/*\"]`"),
        "census must nail members glob crates/*"
    );
    assert!(
        census.contains("≠") && census.contains("S2-06"),
        "census must keep Non-claim honesty for S2-06"
    );
    assert!(
        census.contains("S2-I261") && census.contains("syx-gui"),
        "census ladder / Current must name S2-I261 exclude syx-gui"
    );

    let adr = repo_file("docs/adr/stage2-s2-06-open.md");
    assert!(adr.contains("S2-I260"), "S2-06 open ADR must exist");
    assert!(
        adr.contains("Open S2-06") || adr.contains("**Open S2-06**"),
        "ADR must open S2-06 workstream"
    );
    assert!(
        adr.contains("S2-I261") && adr.contains("syx-gui"),
        "ADR must lock next knife = S2-I261 exclude syx-gui"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-06") || adr.contains("[x]")),
        "ADR must Non-claim S2-06 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );

    let script = repo_file("scripts/s2_06_workspace_census.sh");
    assert!(
        script.contains("workspace_member_count"),
        "census script must emit workspace_member_count"
    );
    assert!(
        script.contains("has_syx_gui") && script.contains("members_glob_crates_star"),
        "census script must emit gui/members KPIs"
    );
    assert!(
        script.contains("syx_gui_workspace_excluded"),
        "census script must emit syx_gui_workspace_excluded (I261)"
    );

    // Honesty: syx-gui directory remains while S2-06 stays [~]; membership exclude is I261.
    let gui = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("crates/syx-gui");
    assert!(
        gui.is_dir(),
        "syx-gui must still exist under crates/ while S2-06 remains [~]"
    );

    let root_toml = repo_file("Cargo.toml");
    assert!(
        root_toml.contains("members = [\"crates/*\"]"),
        "root Cargo.toml must still use crates/* members"
    );
}
