//! R-2 Channel send/trySend census contract.
//! Nails: script green, public send/trySend off Object, Sendable-typed.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r2_channel_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r2_channel_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r2_channel_census_files_exist() {
    assert!(repo_path("scripts/r2_channel_census.sh").is_file());
    assert!(repo_path("docs/census/r-2-channel-census.md").is_file());
    let sync = std::fs::read_to_string(repo_path("packages/sync/mod.sy")).unwrap();
    assert!(
        sync.contains("func send(_ value: Sendable)"),
        "public send must be Sendable-typed"
    );
    assert!(
        sync.contains("func trySend(_ value: Sendable)"),
        "public trySend must be Sendable-typed"
    );
    assert!(
        !sync.lines().any(|ln| {
            (ln.contains("func send(") || ln.contains("func trySend(")) && ln.contains("Object")
        }),
        "public send/trySend must not spell Object"
    );
}

#[test]
fn r2_channel_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r1_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "channel_send_trysend_object_residual="), 0);
    assert_eq!(parse(&stdout, "channel_send_trysend_sendable_typed="), 1);
}
