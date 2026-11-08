//! S3-I84 S3-07 residual-scope (scoped acceptance + deferred UsageGuide/crate) contracts.
//! Hinge landed · post-I85: S3-07 [x] · Stage-3 still [~] 7/7 · ≠ Init product knife.
//! See docs/adr/stage3-s3-07-residual-scope.md · docs/stage3-init-template-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_07_residual_scope_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-07-residual-scope.md");
    assert!(adr.contains("S3-I84"), "residual-scope ADR must name S3-I84");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer UsageGuide/crate / supersede-only"
    );
    assert!(
        adr.contains("UsageGuide")
            || adr.contains("crate")
            || adr.contains("subprocess")
            || adr.contains("_"),
        "ADR must name deferred axes"
    );
    assert!(
        adr.contains("S3-I85")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must lock next = S3-I85 scoped claim"
    );
    assert!(
        adr.contains("≠ S3-07 `[x]`") || adr.contains("≠ S3-07 [x]"),
        "ADR must Non-claim S3-07 [x] this window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/stage3-init-template-census.md");
    assert!(
        census.contains("Current:** S3-I85") || census.contains("Current: S3-I85"),
        "census Current must advance to S3-I85"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I84 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I85") && census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census ladder must name I85 claimed / I86 next"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage3-s3-07-default-vm.md");
    assert!(
        prior.contains("S3-I84") || prior.contains("stage3-s3-07-residual-scope"),
        "I83 ADR must cite landed I84"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s307 = plan02
        .lines()
        .find(|l| l.contains("| S3-07 |"))
        .expect("S3-07 row");
    assert!(
        s307.contains("| [x] |") || s307.contains("| `[x]` |") || s307.contains("[x]"),
        "S3-07 must be [x] after I85 claim; got: {s307}"
    );
    assert!(
        s307.contains("scoped")
            || s307.contains("residual-scope")
            || s307.contains("deferred")
            || s307.contains("Deferred"),
        "S3-07 row must publish scoped acceptance; got: {s307}"
    );
    assert!(
        plan02.contains("S3-I84")
            && plan02.contains("S3-I85")
            && plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4")),
        "plan/02 must cite I84…I86"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I84")
            && plan01.contains("S3-I85")
            && plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4"))
            && plan01.contains("7/7"),
        "plan/01 must cite I84…I86 / 7/7"
    );
}

#[test]
fn s3_07_residual_scope_census_script_emits_hinge() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s3_07_init_template_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s3_07_init_template_census.sh");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I85") && stdout.contains("next=S3-I88"),
        "script ladder must be I85→I88:\n{stdout}"
    );
    assert!(
        stdout.contains("residual_scope_helper="),
        "script must emit I84 hinge:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0)
    };
    assert!(
        parse("residual_scope_helper=") >= 1,
        "residual_scope_helper must be >=1\n{stdout}"
    );
    assert_eq!(parse("init_default_template_native="), 0);
    assert!(parse("init_default_template_vm=") >= 1);
    assert!(parse("init_vm_named_template_count=") >= 1);
    assert!(parse("init_integration_test_count=") >= 1);
}
