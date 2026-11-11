//! S3-I104…I107 Stage-4 / S4-05 prebuilt-release census contracts.
//! Open→claim · Stage-4 [x] 退出 · S4-05 [x] · 5/5 · next = S3-I108.
//! See docs/adr/stage4-s4-05-open.md · docs/census/stage4-prebuilt-release-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_05_prebuilt_release_census_baseline() {
    let census = repo_file("docs/census/stage4-prebuilt-release-census.md");
    assert!(census.contains("S3-I104"), "census must name S3-I104");
    assert!(census.contains("S3-I105"), "census must name S3-I105");
    assert!(census.contains("S3-I106"), "census must name S3-I106");
    assert!(census.contains("S3-I107"), "census must name S3-I107");
    assert!(census.contains("S3-I108"), "census must name S3-I108 next");
    assert!(
        census.contains("Current:** S3-I107") || census.contains("Current: S3-I107"),
        "census Current must advance to S3-I107"
    );
    assert!(
        census.contains("doctor_native_sdk_hinge_helper")
            && census.contains("residual_scope_helper")
            && census.contains("s4_05_claim_helper"),
        "census must publish hinge + residual + claim KPIs"
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
        census.contains("s4_05_status") && (census.contains("`x`") || census.contains("x")),
        "census must publish s4_05_status x"
    );
    assert!(
        census.contains("full-native")
            || census.contains("syx-sdk")
            || census.contains("native_sdk_layout")
            || census.contains("doctor"),
        "census must cover full-native / syx-sdk / layout / doctor axes"
    );

    let adr = repo_file("docs/adr/stage4-s4-05-open.md");
    assert!(adr.contains("S3-I104"), "open ADR must name S3-I104");
    assert!(
        adr.contains("S3-I103") || adr.contains("stage4-s4-04-claim"),
        "ADR must cite prior S4-04 claim"
    );
    assert!(
        adr.contains("S3-I107") || adr.contains("stage4-s4-05-claim"),
        "open ADR Consequences must cite landed I107"
    );
    assert!(
        adr.contains("S3-I108") || adr.contains("exit-open"),
        "open ADR Consequences must lock next = I108"
    );
    assert!(
        adr.contains("M2-S4-05") || adr.contains("TLS"),
        "ADR must disambiguate from old M2-S4-05 TLS id"
    );

    let script = repo_file("scripts/s4_05_prebuilt_release_census.sh");
    assert!(
        script.contains("s4_05_open_helper")
            && script.contains("s4_05_census_helper")
            && script.contains("full_native_build_helper")
            && script.contains("syx_sdk_packaging_helper")
            && script.contains("native_sdk_layout_helper")
            && script.contains("doctor_cli_helper")
            && script.contains("doctor_native_sdk_flag_helper")
            && script.contains("doctor_native_sdk_hinge_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s4_05_claim_helper")
            && script.contains("S3-I107")
            && script.contains("S3-I108")
            && (script.contains("emit current") || script.contains("current=S3-I107")),
        "census script must emit full ladder / KPIs"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 stage-4 row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        stage4.contains("5/5"),
        "Stage-4 checklist must be 5/5; got: {stage4}"
    );
    let s405 = plan02
        .lines()
        .find(|l| l.contains("| S4-05 |"))
        .expect("plan/02 S4-05 row");
    assert!(
        s405.contains("| [x] |") || s405.contains("| `[x]` |") || s405.contains("[x]"),
        "S4-05 must be [x] after I107; got: {s405}"
    );
    assert!(
        plan02.contains("S3-I104")
            && plan02.contains("S3-I105")
            && plan02.contains("S3-I106")
            && plan02.contains("S3-I107")
            && plan02.contains("S3-I108"),
        "plan/02 must cite I104…I108"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I107") && plan01.contains("5/5"),
        "plan/01 must cite I107 / 5/5"
    );
    assert!(
        plan01.contains("S4-05") && plan01.contains("[x]"),
        "plan/01 must reflect S4-05 [x]"
    );
    assert!(
        plan01.contains("S3-I108") || plan01.contains("exit-open"),
        "plan/01 must lock next = I108"
    );

    let prior = repo_file("docs/adr/stage4-s4-04-claim.md");
    assert!(
        prior.contains("S3-I104") || prior.contains("stage4-s4-05-open"),
        "I103 Consequences must cite landed I104"
    );
}

#[test]
fn s4_05_prebuilt_release_census_script_hinges() {
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
        "s4_05_open_helper=",
        "s4_05_census_helper=",
        "full_native_build_helper=",
        "syx_sdk_packaging_helper=",
        "native_sdk_layout_helper=",
        "doctor_cli_helper=",
        "doctor_native_sdk_flag_helper=",
        "s4_05_claim_helper=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0)
    };
    assert!(parse("s4_05_open_helper=") >= 1);
    assert!(parse("s4_05_census_helper=") >= 1);
    assert!(parse("full_native_build_helper=") >= 1);
    assert!(parse("syx_sdk_packaging_helper=") >= 1);
    assert!(parse("native_sdk_layout_helper=") >= 1);
    assert!(parse("doctor_cli_helper=") >= 1);
    assert!(parse("doctor_native_sdk_flag_helper=") >= 1);
    assert!(parse("s4_05_claim_helper=") >= 1);
    assert!(
        stdout.contains("current=S3-I107") && stdout.contains("next=S3-I108"),
        "ladder I107→I108:\n{stdout}"
    );
    assert!(
        stdout.contains("s4_05_status=x") && stdout.contains("stage4_checklist=5/5"),
        "claimed status:\n{stdout}"
    );

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
