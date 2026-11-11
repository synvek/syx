//! R-13 Concurrency census contract.
//! Nails: script green, task bags → Dictionary, any-value slots → Sendable.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r13_concurrency_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r13_concurrency_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r13_concurrency_census_files_exist() {
    assert!(repo_path("scripts/r13_concurrency_census.sh").is_file());
    assert!(repo_path("docs/census/r-13-concurrency-census.md").is_file());
    let concurrency =
        std::fs::read_to_string(repo_path("packages/core/concurrency.sy")).unwrap();
    assert!(
        concurrency.contains("func detach(_ name: String) -> Dictionary"),
        "detach must return Dictionary"
    );
    assert!(
        concurrency.contains("func join(_ task: Dictionary) -> Dictionary"),
        "join must take/return Dictionary"
    );
    assert!(
        concurrency.contains("func taskGroupCreate() -> Dictionary"),
        "taskGroupCreate must return Dictionary"
    );
    assert!(
        concurrency.contains("func awaitValue(_ task: Dictionary) -> Sendable"),
        "awaitValue must return Sendable"
    );
    assert!(
        concurrency.contains("func taskLocalGet(_ key: String) -> Sendable"),
        "taskLocalGet must return Sendable"
    );
    assert!(
        concurrency.contains("args: Array<Sendable>) -> Dictionary"),
        "detach args must be Array<Sendable>"
    );
    assert!(
        !concurrency
            .lines()
            .filter(|ln| !ln.trim_start().starts_with("//"))
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "Concurrency methods must not spell Object"
    );
}

#[test]
fn r13_concurrency_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r12_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "concurrency_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "concurrency_task_handle_dictionary="), 1);
    assert_eq!(parse(&stdout, "concurrency_any_value_sendable="), 1);
    // Monotonic guard: absolute values are pinned by the newest window's contract (R-27).
    assert!(parse(&stdout, "object_sig_residual_count=") <= 198);
    assert!(parse(&stdout, "object_sig_residual_files=") <= 14);
}
