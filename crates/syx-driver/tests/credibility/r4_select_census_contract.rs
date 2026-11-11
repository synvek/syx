//! R-4 Select public-surface census contract.
//! Nails: script green, Select methods off Object, Array<Channel> + Sendable.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r4_select_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r4_select_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r4_select_census_files_exist() {
    assert!(repo_path("scripts/r4_select_census.sh").is_file());
    assert!(repo_path("docs/census/r-4-select-census.md").is_file());
    let sync = std::fs::read_to_string(repo_path("packages/sync/mod.sy")).unwrap();
    assert!(
        sync.contains("func select(channels: Array<Channel>) -> Sendable"),
        "Select.select must be Array<Channel> -> Sendable"
    );
    assert!(
        sync.contains("func orDefault(channels: Array<Channel>, fallback: Sendable) -> Sendable"),
        "Select.orDefault must be Array<Channel> + Sendable"
    );
    assert!(
        sync.contains("func timeout(channels: Array<Channel>, ms: Int) -> Sendable"),
        "Select.timeout must be Array<Channel> -> Sendable"
    );
    // Select method lines must not spell Object (handle field may still).
    let in_select = sync
        .split("public class Select")
        .nth(1)
        .and_then(|rest| rest.split("public class").next())
        .unwrap_or("");
    assert!(
        !in_select.lines().any(|ln| {
            (ln.contains("func select(")
                || ln.contains("func orDefault(")
                || ln.contains("func timeout("))
                && ln.contains("Object")
        }),
        "Select methods must not spell Object"
    );
}

#[test]
fn r4_select_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r3_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "select_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "select_channels_array_channel_typed="), 1);
    assert_eq!(parse(&stdout, "select_return_sendable_typed="), 1);
    assert_eq!(parse(&stdout, "select_fallback_sendable_typed="), 1);
}
