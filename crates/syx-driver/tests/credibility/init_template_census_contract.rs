//! S3-I82…I83 S3-07 init-template census contracts.
//! I82 open · I83–I85 ladder · S3-07 [x] · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-07-open.md · docs/census/stage3-init-template-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_07_init_template_census_baseline() {
    let census = repo_file("docs/census/stage3-init-template-census.md");
    assert!(census.contains("S3-I82"), "census must name S3-I82 open");
    assert!(census.contains("S3-I83"), "census must name S3-I83");
    assert!(census.contains("S3-I84"), "census must name S3-I84");
    assert!(census.contains("S3-I85"), "census must name S3-I85");
    assert!(
        census.contains("S3-I86") && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census must name I86 landed / I87 next"
    );
    assert!(
        census.contains("Current:** S3-I85") || census.contains("Current: S3-I85"),
        "census Current must be S3-I85"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("stage3_init_census_helper")
            && census.contains("init_default_template_native")
            && census.contains("init_default_template_vm")
            && census.contains("init_vm_named_template_count")
            && census.contains("init_integration_test_count"),
        "census must publish I83 KPIs"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");

    let open_adr = repo_file("docs/adr/stage3-s3-07-open.md");
    assert!(open_adr.contains("S3-I82"), "open ADR must name S3-I82");
    assert!(
        open_adr.contains("S3-07 `[~]`"),
        "open ADR Status must keep historical S3-07 [~]"
    );
    assert!(
        open_adr.contains("≠ S3-07 `[x]`") || open_adr.contains("≠ S3-07 [x]"),
        "open ADR must Non-claim S3-07 [x] for that window"
    );
    assert!(
        open_adr.contains("S3-I81") || open_adr.contains("stage3-s3-06-claim"),
        "open ADR must cite prior S3-I81 claim"
    );
    assert!(
        open_adr.contains("S3-I83") || open_adr.contains("stage3-s3-07-default-vm"),
        "open ADR must cite landed S3-I83"
    );
    assert!(
        open_adr.contains("native") && open_adr.contains("Init"),
        "open ADR Gap must cite native Init default (historical)"
    );

    let script = repo_file("scripts/s3_07_init_template_census.sh");
    assert!(
        script.contains("stage3_init_census_helper")
            && script.contains("init_default_template_native")
            && script.contains("init_default_template_vm")
            && script.contains("init_vm_named_template_count")
            && script.contains("init_integration_test_count")
            && script.contains("residual_scope_helper"),
        "census script must emit I83/I84 KPIs"
    );
    assert!(
        script.contains("current=S3-I85") && script.contains("next=S3-I89"),
        "census script must emit ladder current/next"
    );
    assert!(
        script.contains("s3_07_claim_helper") && script.contains("s3_07_status=x"),
        "census script must emit I85 claim hinge"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s307_line = plan02
        .lines()
        .find(|l| l.contains("| S3-07 |"))
        .expect("plan/02 must have S3-07 table row");
    assert!(
        s307_line.contains("| [x] |")
            || s307_line.contains("| `[x]` |")
            || s307_line.contains("[x]"),
        "plan/02 S3-07 must be [x], got: {s307_line}"
    );
    assert!(
        plan02.contains("S3-I82") && plan02.contains("7/7"),
        "plan/02 must cite S3-I82 and Stage-3 7/7"
    );
    assert!(
        plan02.contains("S3-I83")
            && plan02.contains("S3-I84")
            && plan02.contains("S3-I85")
            && plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4")),
        "plan/02 must cite I83…I86"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I82") && plan01.contains("7/7"),
        "plan/01 must cite S3-I82 and Stage-3 7/7"
    );
    assert!(
        plan01.contains("S3-I83")
            && plan01.contains("S3-I84")
            && plan01.contains("S3-I85")
            && plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4")),
        "plan/01 must cite I83…I86"
    );

    let prior = repo_file("docs/adr/stage3-s3-06-claim.md");
    assert!(
        prior.contains("S3-I82") || prior.contains("stage3-s3-07-open"),
        "I81 claim ADR must cite landed I82"
    );
}

#[test]
fn s3_07_init_template_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = root.join("scripts/s3_07_init_template_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
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
        stdout.contains("stage3_init_census_helper=")
            && stdout.contains("init_default_template_native=")
            && stdout.contains("init_default_template_vm=")
            && stdout.contains("init_vm_named_template_count=")
            && stdout.contains("init_integration_test_count=")
            && stdout.contains("init_commands_helper=")
            && stdout.contains("residual_scope_helper="),
        "script stdout missing KPIs:\n{stdout}"
    );
    assert!(
        stdout.contains("current=S3-I85") && stdout.contains("next=S3-I89"),
        "script must emit ladder:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_07_status=x") && stdout.contains("stage3_checklist=7/7"),
        "script must emit claimed status / 7/7:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_07_claim_helper="),
        "script must emit I85 claim hinge:\n{stdout}"
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
        parse("stage3_init_census_helper=") >= 1,
        "stage3_init_census_helper must be >=1\n{stdout}"
    );
    assert_eq!(parse("init_default_template_native="), 0);
    assert!(
        parse("init_default_template_vm=") >= 1,
        "init_default_template_vm must be >=1\n{stdout}"
    );
    assert!(
        parse("init_vm_named_template_count=") >= 1,
        "init_vm_named_template_count must be >=1\n{stdout}"
    );
    assert!(
        parse("init_integration_test_count=") >= 1,
        "init_integration_test_count must be >=1\n{stdout}"
    );
}

#[test]
fn s3_07_open_does_not_claim_or_exit() {
    let open_adr = repo_file("docs/adr/stage3-s3-07-open.md");
    assert!(
        open_adr.contains("S3-07 `[~]`"),
        "open ADR Status must keep historical S3-07 [~]"
    );
    assert!(
        open_adr.contains("≠ S3-07 `[x]`"),
        "open ADR must Non-claim S3-07 [x] for that window"
    );
    let census = repo_file("docs/census/stage3-init-template-census.md");
    assert!(
        census.contains("7/7"),
        "census must publish Stage-3 checklist 7/7"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    let construct = repo_file("docs/adr/stage3-s3-07-default-vm.md");
    assert!(
        construct.contains("S3-07 `[~]`") && construct.contains("≠ S3-07 `[x]`"),
        "construct ADR must keep S3-07 [~] and Non-claim [x]"
    );
}
