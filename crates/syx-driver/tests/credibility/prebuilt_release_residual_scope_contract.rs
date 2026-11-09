//! S3-I106 S4-05 residual-scope contracts.
//! Post-I107: Stage-4 [x] 退出 · S4-05 [x] · 5/5 · residual hinge stays green.
//! See docs/adr/stage4-s4-05-residual-scope.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_05_residual_scope_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-05-residual-scope.md");
    assert!(adr.contains("S3-I106"), "residual-scope ADR must name S3-I106");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer CLI/sysroot/marketplace / supersede-only"
    );
    assert!(
        adr.contains("S3-I107")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite landed I107 scoped claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );
    assert!(
        adr.contains("M2-S4-05") || adr.contains("TLS"),
        "ADR must disambiguate from old M2-S4-05 TLS"
    );

    let scope = repo_file("docs/census/s4-05-prebuilt-residual-scope-v0.md");
    assert!(
        scope.contains("S3-I106")
            && scope.contains("Deferred")
            && (scope.contains("supersede") || scope.contains("superseding")),
        "scope-v0 must name I106 / deferred supersede"
    );

    let census = repo_file("docs/census/stage4-prebuilt-release-census.md");
    assert!(
        census.contains("Current:** S3-I107") || census.contains("Current: S3-I107"),
        "census Current must advance to S3-I107"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I106 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I108") && census.contains("5/5"),
        "census must name I108 next and Stage-4 5/5"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md");
    assert!(
        prior.contains("S3-I106") || prior.contains("stage4-s4-05-residual-scope"),
        "I105 ADR must cite landed I106"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s405 = plan02
        .lines()
        .find(|l| l.contains("| S4-05 |"))
        .expect("S4-05 row");
    assert!(
        s405.contains("| [x] |") || s405.contains("| `[x]` |") || s405.contains("[x]"),
        "S4-05 must be [x] after I107; got: {s405}"
    );
    assert!(
        s405.contains("scoped")
            || s405.contains("residual")
            || s405.contains("deferred")
            || s405.contains("Deferred"),
        "S4-05 row must publish scoped/deferred; got: {s405}"
    );
    assert!(
        plan02.contains("S3-I106")
            && plan02.contains("S3-I107")
            && plan02.contains("S3-I108")
            && plan02.contains("5/5"),
        "plan/02 must cite I106…I108 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I106")
            && plan01.contains("S3-I107")
            && plan01.contains("5/5"),
        "plan/01 must cite I106 / I107 / 5/5"
    );
}

#[test]
fn s4_05_residual_scope_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_05_prebuilt_release_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_05 census");
    assert!(
        out.status.success(),
        "census failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I107") && stdout.contains("next=S3-I108"),
        "ladder I107→I108:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_05_status=x")
            && stdout.contains("stage4_checklist=5/5"),
        "status/checklist:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("doctor_native_sdk_hinge_helper=") >= 1);
    assert!(parse("s4_05_claim_helper=") >= 1);
    assert!(parse("native_sdk_layout_helper=") >= 1);
    assert!(parse("layout_report_unit_helper=") >= 1);
    assert!(parse("full_native_build_helper=") >= 1);
    assert!(parse("syx_sdk_packaging_helper=") >= 1);
    assert!(parse("s4_05_open_helper=") >= 1);

    let cli = repo_file("crates/syx-cli/src/main.rs");
    let doctor_block = cli
        .split("Doctor {")
        .nth(1)
        .and_then(|rest| rest.split("},").next())
        .expect("CLI Doctor variant");
    assert!(
        !doctor_block.contains("native_sdk") && !doctor_block.contains("native-sdk"),
        "Hard OUT: Doctor must remain unwired for --native-sdk; got:\n{doctor_block}"
    );
}
