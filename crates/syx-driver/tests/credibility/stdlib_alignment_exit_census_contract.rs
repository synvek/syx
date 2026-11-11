//! SA-6-01 exit census contract.
//! Nails: script exists, runs green, Object slice cleared, residual inventory printed.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/sa_6_exit_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "sa_6_exit_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn sa6_exit_census_files_exist() {
    assert!(repo_path("scripts/sa_6_exit_census.sh").is_file());
    assert!(repo_path("docs/census/sa-6-exit-census.md").is_file());
    assert!(repo_path("docs/adr/sa-exit.md").is_file());
    let adr = std::fs::read_to_string(repo_path("docs/adr/sa-exit.md")).unwrap();
    assert!(adr.contains("post-SA Residual"));
    assert!(adr.contains("Accepted"));
}

#[test]
fn sa6_exit_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "prelude_match="), 1);
    assert_eq!(parse(&stdout, "importable_match="), 1);
    assert_eq!(parse(&stdout, "old_import_lines="), 0);
    assert_eq!(parse(&stdout, "public_syx_hostfn_spellings="), 0);
    assert_eq!(parse(&stdout, "system_io_object_slice_cleared="), 1);
    // Residual inventory must be reported (may be >0; ≠ literal full clear).
    assert!(
        stdout.contains("object_sig_residual_count="),
        "must print object inventory"
    );
}
