//! S2-I261 S2-06 exclude syx-gui contracts.
//! Nails: root exclude + de-inherited gui toml + census KPI + Non-claim S2-06 [x].
//! See docs/adr/stage2-s2-06-exclude-syx-gui.md · docs/stage2-workspace-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn s2_06_exclude_syx_gui_locked() {
    let adr = repo_file("docs/adr/stage2-s2-06-exclude-syx-gui.md");
    assert!(adr.contains("S2-I261"), "exclude ADR must name S2-I261");
    assert!(
        adr.contains("exclude") && adr.contains("syx-gui"),
        "ADR must decide exclude syx-gui"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-06"),
        "ADR must Non-claim S2-06 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
    assert!(
        adr.contains("S2-I262") && (adr.contains("claim") || adr.contains("`[x]`")),
        "ADR must lock next = S2-I262 S2-06 claim"
    );

    let root_toml = repo_file("Cargo.toml");
    assert!(
        root_toml.contains("members = [\"crates/*\"]"),
        "root must keep crates/* members"
    );
    assert!(
        root_toml.contains("exclude = [\"crates/syx-gui\"]")
            || root_toml.contains("exclude = [\"crates/syx-gui\","),
        "root must exclude crates/syx-gui"
    );

    let gui_toml = repo_file("crates/syx-gui/Cargo.toml");
    assert!(
        !gui_toml.contains(".workspace = true") && !gui_toml.contains("{ workspace = true }"),
        "excluded syx-gui must not inherit workspace fields"
    );
    assert!(
        gui_toml.contains("path = \"../syx-diagnostics\"")
            && gui_toml.contains("path = \"../syx-runtime-core\""),
        "syx-gui must use path deps to diagnostics/runtime-core"
    );
    assert!(
        gui_toml.contains("version = \"0.1.3\""),
        "syx-gui must pin version 0.1.3"
    );

    assert!(
        repo_path("crates/syx-gui").is_dir(),
        "syx-gui directory must remain on disk"
    );

    let vm_toml = repo_file("crates/syx-vm/Cargo.toml");
    assert!(
        vm_toml.contains("syx-gui") && vm_toml.contains("path = \"../syx-gui\""),
        "syx-vm must keep path opt-in to syx-gui"
    );
    assert!(
        vm_toml.contains("gui-syx-isolated"),
        "syx-vm must keep gui-syx-isolated feature"
    );

    let census = repo_file("docs/stage2-workspace-census.md");
    assert!(
        census.contains("S2-I261"),
        "workspace census Current must name S2-I261"
    );
    assert!(
        census.contains("exclude") && census.contains("syx-gui"),
        "census must record syx-gui exclude"
    );
    assert!(
        census.contains("≠") && census.contains("S2-06"),
        "census must keep Non-claim honesty for S2-06"
    );

    let script = repo_file("scripts/s2_06_workspace_census.sh");
    assert!(
        script.contains("syx_gui_workspace_excluded"),
        "census script must emit syx_gui_workspace_excluded"
    );

    // Plan S2-06 status is owned by s2_06_claim_contract after S2-I262.
    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    assert!(
        plan02.contains("S2-I261") || plan02.contains("stage2-s2-06-exclude-syx-gui"),
        "plan/02 must cite S2-I261 exclude"
    );
}
