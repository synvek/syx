//! S3-I105 S4-05 doctor-native-sdk / full-native hinge contracts.
//! Post-I107: Stage-4 [x] 退出 · S4-05 [x] · 5/5 · Primary KPIs · next = S3-I108.
//! See docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_05_doctor_native_sdk_hinge_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md");
    assert!(adr.contains("S3-I105"), "hinge ADR must name S3-I105");
    assert!(
        adr.contains("native_sdk_layout")
            || adr.contains("full-native")
            || adr.contains("doctor-native-sdk"),
        "ADR must publish layout / full-native Primary"
    );
    assert!(
        adr.contains("S3-I106") || adr.contains("stage4-s4-05-residual-scope"),
        "ADR Consequences must cite landed I106"
    );
    assert!(
        adr.contains("S3-I107") || adr.contains("stage4-s4-05-claim"),
        "ADR Consequences must cite landed I107"
    );
    assert!(
        adr.contains("S3-I108") || adr.contains("exit-open"),
        "ADR must lock next = I108 exit-open"
    );
    assert!(
        adr.contains("M2-S4-05") || adr.contains("TLS"),
        "ADR must disambiguate from old M2-S4-05 TLS"
    );

    let charter = repo_file("docs/census/s4-05-doctor-native-sdk-v0.md");
    assert!(
        charter.contains("S3-I105")
            && (charter.contains("native_sdk_layout")
                || charter.contains("full-native")
                || charter.contains("Primary")),
        "doctor-native-sdk-v0 must exist"
    );

    let census = repo_file("docs/census/stage4-prebuilt-release-census.md");
    assert!(
        census.contains("Current:** S3-I107") || census.contains("Current: S3-I107"),
        "census Current must advance to S3-I107 after claim"
    );
    assert!(
        census.contains("doctor_native_sdk_hinge_helper")
            && census.contains("s4_05_claim_helper")
            && census.contains("S3-I108"),
        "census must publish I105/I107 KPIs / I108 next"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(census.contains("5/5"), "census must publish Stage-4 5/5");

    let prior = repo_file("docs/adr/stage4-s4-05-open.md");
    assert!(
        prior.contains("S3-I105") || prior.contains("stage4-s4-05-doctor-native-sdk-hinge"),
        "I104 ADR must cite landed I105"
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
        plan02.contains("S3-I105")
            && plan02.contains("S3-I106")
            && plan02.contains("S3-I107")
            && plan02.contains("5/5"),
        "plan/02 must cite I105…I107 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I105") && plan01.contains("5/5"),
        "plan/01 must cite I105 / 5/5"
    );
    assert!(
        plan01.contains("S3-I107") && (plan01.contains("S3-I108") || plan01.contains("exit-open")),
        "plan/01 must cite I107 / next I108"
    );
}

#[test]
fn s4_05_doctor_native_sdk_hinge_census_script_hinges() {
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
    assert!(parse("doctor_native_sdk_hinge_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("s4_05_claim_helper=") >= 1);
    assert!(parse("layout_report_unit_helper=") >= 1);
    assert!(parse("native_sdk_layout_helper=") >= 1);
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
        doctor_block.contains("native_sdk"),
        "Doctor must expose --native-sdk (release/docs smoke); got:\n{doctor_block}"
    );
}
