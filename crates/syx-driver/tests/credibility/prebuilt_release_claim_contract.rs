//! S3-I107 S4-05 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S4-05 [x]; scoped Non-claims; Stage-4 [x] 退出 5/5.
//! See docs/adr/stage4-s4-05-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_05_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage4-s4-05-claim.md");
    assert!(adr.contains("S3-I107"), "S4-05 claim ADR must exist");
    assert!(
        adr.contains("S4-05") && adr.contains("`[x]`"),
        "ADR Status must claim S4-05 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );
    assert!(
        adr.contains("native_sdk_layout")
            || adr.contains("full-native")
            || adr.contains("doctor-native-sdk"),
        "ADR Evidence must cite Primary layout / full-native hinge"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep CLI/sysroot deferred"
    );
    assert!(
        adr.contains("S3-I106") || adr.contains("residual-scope"),
        "ADR must cite I106 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I108")
            || (adr.contains("exit-open") && adr.contains("Stage-4")),
        "ADR must cite S3-I108 Stage-4 exit-open"
    );
    assert!(
        adr.contains("M2-S4-05") || adr.contains("TLS"),
        "ADR must disambiguate from old M2-S4-05 TLS"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s405 = plan02
        .lines()
        .find(|l| l.contains("| S4-05 |"))
        .expect("plan/02 must have S4-05 row");
    assert!(
        s405.contains("| [x] |") || s405.contains("| `[x]` |"),
        "plan/02 S4-05 status must be [x]; got: {s405}"
    );
    assert!(
        plan02.contains("S3-I107") || plan02.contains("stage4-s4-05-claim"),
        "plan/02 must cite S3-I107 claim"
    );
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 must retain stage-4 summary row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        plan02.contains("5/5"),
        "plan/02 must publish Stage-4 5/5"
    );
    assert!(
        plan02.contains("S3-I108") || plan02.contains("exit-open"),
        "plan/02 must cite I108 / exit-open"
    );

    let census = repo_file("docs/census/stage4-prebuilt-release-census.md");
    assert!(
        census.contains("S3-I107"),
        "census must name S3-I107"
    );
    assert!(
        census.contains("Current:** S3-I107") || census.contains("Current: S3-I107"),
        "census Current must be S3-I107"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("5/5"),
        "census must publish Stage-4 5/5"
    );
    assert!(
        census.contains("S3-I108"),
        "census ladder must cite I108 next"
    );
    assert!(
        census.contains("s4_05_claim_helper"),
        "census must publish claim KPI"
    );

    let prior = repo_file("docs/adr/stage4-s4-05-residual-scope.md");
    assert!(
        prior.contains("S3-I107") || prior.contains("stage4-s4-05-claim"),
        "I106 ADR must cite landed I107"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I107") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I107 S4-05 [x]"
    );
    assert!(
        plan01.contains("5/5"),
        "plan/01 must publish Stage-4 5/5"
    );
    assert!(
        plan01.contains("S3-I108") || plan01.contains("exit-open"),
        "plan/01 must cite I108 / exit-open"
    );
}

#[test]
fn s4_05_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s4_05_prebuilt_release_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run s4_05 census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s4_05_claim_helper=",
        "residual_scope_helper=",
        "doctor_native_sdk_hinge_helper=",
        "full_native_build_helper=",
        "syx_sdk_packaging_helper=",
        "native_sdk_layout_helper=",
        "layout_report_unit_helper=",
        "s4_05_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I107") && stdout.contains("next=S3-I108"),
        "script ladder must be I107→I108:\n{stdout}"
    );
    assert!(
        stdout.contains("s4_05_status=x")
            && stdout.contains("stage4_status=x")
            && stdout.contains("stage4_checklist=5/5"),
        "script must emit claimed status / 5/5:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0)
    };
    assert!(parse("s4_05_claim_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("doctor_native_sdk_hinge_helper=") >= 1);
    assert!(parse("native_sdk_layout_helper=") >= 1);
    assert!(parse("layout_report_unit_helper=") >= 1);
    assert!(parse("full_native_build_helper=") >= 1);
    assert!(parse("syx_sdk_packaging_helper=") >= 1);

    let cli = repo_file("crates/syx-cli/src/main.rs");
    let doctor_block = cli
        .split("Doctor {")
        .nth(1)
        .and_then(|rest| rest.split("},").next())
        .expect("CLI Doctor variant");
    assert!(
        doctor_block.contains("native_sdk"),
        "Doctor must expose --native-sdk (release/docs smoke); got:\n{doctor_block}"
    );
}
