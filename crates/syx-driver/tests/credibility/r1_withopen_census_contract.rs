//! R-1 withOpen census contract.
//! Nails: script green, public withOpen off Object, Function-typed.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r1_withopen_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r1_withopen_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r1_withopen_census_files_exist() {
    assert!(repo_path("scripts/r1_withopen_census.sh").is_file());
    assert!(repo_path("docs/census/r-1-withopen-census.md").is_file());
    let fh = std::fs::read_to_string(repo_path("packages/io/file_handle.sy")).unwrap();
    assert!(
        fh.contains("body: Function"),
        "public withOpen must be Function-typed"
    );
    assert!(
        !fh.lines()
            .any(|ln| ln.contains("func withOpen") && ln.contains("Object")),
        "public withOpen must not spell Object"
    );
}

#[test]
fn r1_withopen_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r0_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "filehandle_withopen_object_residual="), 0);
    assert_eq!(parse(&stdout, "filehandle_withopen_object_cleared="), 1);
    assert_eq!(parse(&stdout, "filehandle_withopen_function_typed="), 1);
}
