//! R-0 residual census contract.
//! Nails: script exists, runs green, tcp readAsync Object slice cleared.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r0_residual_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r0_residual_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    stdout
}

fn parse(stdout: &str, key: &str) -> i64 {
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            return rest
                .parse()
                .unwrap_or_else(|_| panic!("bad int for {key}: {rest}"));
        }
    }
    panic!("missing key {key} in census output:\n{stdout}");
}

#[test]
fn r0_residual_census_files_exist() {
    assert!(repo_path("scripts/r0_residual_census.sh").is_file());
    assert!(repo_path("docs/census/r-0-residual-census.md").is_file());
    let plan = std::fs::read_to_string(repo_path("plan/02-improvement-roadmap.md")).unwrap();
    assert!(
        plan.contains("阶段 R · post-SA Residual"),
        "plan/02 must have R stage section"
    );
}

#[test]
fn r0_residual_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "sa6_hinge_still_green="), 1);
    assert_eq!(
        parse(&stdout, "networking_tcp_readasync_object_cleared="),
        1
    );
    assert!(
        stdout.contains("object_sig_residual_count="),
        "must roll up object inventory via sa_6"
    );
}
