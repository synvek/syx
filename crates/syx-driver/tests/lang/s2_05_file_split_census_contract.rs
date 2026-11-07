//! S2-I165 S2-05 file-split census contracts (open baseline).
//! Nails: open ADR + census + script KPIs; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-open.md · docs/stage2-file-split-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_file_split_census_baseline() {
    let census = repo_file("docs/stage2-file-split-census.md");
    assert!(
        census.contains("S2-I165"),
        "file-split census must name S2-I165 baseline"
    );
    assert!(
        census.contains("13216") && census.contains("9117") && census.contains("8889"),
        "census must nail mir/driver/semantic lib.rs LOC baseline"
    );
    assert!(
        census.contains("≠") && census.contains("S2-05"),
        "census must keep Non-claim honesty for S2-05"
    );
    assert!(
        census.contains("syx-mir") && census.contains("first knife"),
        "census ladder must name syx-mir first knife"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-open.md");
    assert!(adr.contains("S2-I165"), "S2-05 open ADR must exist");
    assert!(
        adr.contains("Open S2-05") || adr.contains("**Open S2-05**"),
        "ADR must open S2-05 workstream"
    );
    assert!(
        adr.contains("S2-I166") && adr.contains("syx-mir"),
        "ADR must lock next knife = S2-I166 syx-mir extract"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-05") || adr.contains("[x]")),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );

    let script = repo_file("scripts/s2_05_file_split_census.sh");
    assert!(
        script.contains("syx_mir_lib_loc"),
        "census script must emit syx_mir_lib_loc"
    );
    assert!(
        script.contains("syx_driver_lib_loc") && script.contains("syx_semantic_lib_loc"),
        "census script must emit driver/semantic LOC KPIs"
    );

    // Honesty: giant faces still present while S2-05 remains [~] (floor tracks peel).
    let mir = repo_file("crates/syx-mir/src/lib.rs");
    let driver = repo_file("crates/syx-driver/src/lib.rs");
    let semantic = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        mir.lines().count() >= 2000,
        "syx-mir lib.rs must still be giant while S2-05 remains [~]"
    );
    assert!(
        driver.lines().count() >= 1100,
        "syx-driver lib.rs must still be large while S2-05 remains [~]"
    );
    assert!(
        semantic.lines().count() >= 8000,
        "syx-semantic lib.rs must still be large at open baseline"
    );
}
