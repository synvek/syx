//! S3-I83 S3-07 default-vm construct contracts.
//! Hinge: Init default=vm · explicit vm/native arms · init→run nail.
//! post-I85: S3-07 [x] · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-07-default-vm.md · docs/stage3-init-template-census.md.

use std::path::Path;

use syx_driver::{run_project_with_options, DriverOptions};

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_07_default_vm_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-07-default-vm.md");
    assert!(adr.contains("S3-I83"), "construct ADR must name S3-I83");
    assert!(
        adr.contains("default_value") || adr.contains("\"vm\""),
        "ADR must name default vm hinge"
    );
    assert!(
        adr.contains("S3-I82") || adr.contains("stage3-s3-07-open"),
        "ADR must cite prior open"
    );
    assert!(
        adr.contains("S3-I84") || adr.contains("stage3-s3-07-residual-scope"),
        "ADR must cite landed I84"
    );
    assert!(
        adr.contains("S3-I85")
            || (adr.contains("claim") && adr.contains("S3-07")),
        "ADR must cite S3-I85 claim ladder"
    );
    assert!(
        adr.contains("S3-I86")
            || (adr.contains("exit") && adr.contains("open")),
        "ADR Consequences must lock next = Stage-3 exit open / I86"
    );
    assert!(
        adr.contains("≠ S3-07 `[x]`") || adr.contains("≠ S3-07 [x]"),
        "ADR must Non-claim S3-07 [x]"
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
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("init_default_template_vm")
            && census.contains("init_vm_named_template_count")
            && census.contains("S3-I84")
            && census.contains("S3-I85")
            && census.contains("S3-I86"),
        "census must publish I83 KPIs and I84…I86 ladder"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");

    let cli = repo_file("crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("default_value = \"vm\""),
        "Init default must be vm"
    );
    assert!(
        !cli.contains("default_value = \"native\""),
        "Init default must not remain native"
    );
    assert!(
        cli.contains("\"vm\"") && cli.contains("print(\"vm\")"),
        "Init must have explicit vm arm"
    );
    assert!(
        cli.contains("\"native\"") && cli.contains("print(\"native\")"),
        "Init must keep explicit native arm"
    );

    let prior = repo_file("docs/adr/stage3-s3-07-open.md");
    assert!(
        prior.contains("S3-I83") || prior.contains("stage3-s3-07-default-vm"),
        "open ADR must cite landed I83"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s307 = plan02
        .lines()
        .find(|l| l.contains("| S3-07 |"))
        .expect("S3-07 row");
    assert!(
        s307.contains("| [x] |") || s307.contains("| `[x]` |") || s307.contains("[x]"),
        "S3-07 must be [x] after I85; got: {s307}"
    );
    assert!(
        plan02.contains("S3-I83")
            && plan02.contains("S3-I84")
            && plan02.contains("S3-I85")
            && plan02.contains("S3-I86"),
        "plan/02 must cite I83…I86"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I83")
            && plan01.contains("S3-I84")
            && plan01.contains("S3-I85")
            && plan01.contains("7/7"),
        "plan/01 must cite I83…I85 / 7/7"
    );
}

#[test]
fn s3_07_default_vm_init_run_nail() {
    // Init-equivalent VM scaffold: three files matching `syx init --template vm`.
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::write(
        root.join("main.sy"),
        "func main() {\n    print(\"vm\")\n}\n",
    )
    .expect("main.sy");
    std::fs::write(
        root.join("build.syb"),
        "build {\n    compileOptions {\n        optLevel = release\n    }\n}\n",
    )
    .expect("build.syb");
    std::fs::write(
        root.join("syx.toml"),
        "name = \"demo\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\nbuild = \"build.syb\"\n",
    )
    .expect("syx.toml");

    run_project_with_options(root, &DriverOptions::default())
        .unwrap_or_else(|e| panic!("init-equivalent vm project must run under VM: {e}"));

    // Source hinge still present (CLI Init default).
    let cli = repo_file("crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("default_value = \"vm\"") && cli.contains("print(\"vm\")"),
        "CLI Init must keep vm default hinge"
    );
    let _ = Path::new(root); // silence unused if optimized
}

#[test]
fn s3_07_default_vm_census_script_cleared() {
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
        stdout.contains("current=S3-I85") && stdout.contains("next=S3-I86"),
        "script ladder must be I85→I86:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
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
