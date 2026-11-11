//! R-12 Fmt + Testing census contract.
//! Nails: script green, public Object → Sendable / Array<Sendable> / Function.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r12_text_testing_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r12_text_testing_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r12_text_testing_census_files_exist() {
    assert!(repo_path("scripts/r12_text_testing_census.sh").is_file());
    assert!(repo_path("docs/census/r-12-text-testing-census.md").is_file());
    let fmt = std::fs::read_to_string(repo_path("packages/text/fmt.sy")).unwrap();
    assert!(
        fmt.contains("func print(value: Sendable)"),
        "Fmt.print value must be Sendable"
    );
    assert!(
        fmt.contains("func format(pattern: String, args: Array<Sendable>)"),
        "Fmt.format args must be Array<Sendable>"
    );
    assert!(
        !fmt.lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "Fmt methods must not spell Object"
    );
    let testing = std::fs::read_to_string(repo_path("packages/testing/mod.sy")).unwrap();
    assert!(
        testing.contains("func assertEq(actual: Sendable, expected: Sendable)"),
        "assertEq must take Sendable"
    );
    assert!(
        testing.contains("func test(name: String, fn: Function)"),
        "test fn must be Function"
    );
    assert!(
        !testing
            .lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "Testing funcs must not spell Object"
    );
}

#[test]
fn r12_text_testing_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r11_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "fmt_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "fmt_format_args_array_sendable="), 1);
    assert_eq!(parse(&stdout, "testing_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "testing_test_fn_function="), 1);
    // Monotonic guard: absolute values are pinned by the newest window's contract.
    assert!(parse(&stdout, "object_sig_residual_count=") <= 217);
    assert!(parse(&stdout, "object_sig_residual_files=") <= 17);
}
