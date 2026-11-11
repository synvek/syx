//! R-6 Channel.handle census contract.
//! Nails: script green, handle Int, create→Int, id Int|bag dual-track.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r6_channel_handle_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r6_channel_handle_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r6_channel_handle_census_files_exist() {
    assert!(repo_path("scripts/r6_channel_handle_census.sh").is_file());
    assert!(repo_path("docs/census/r-6-channel-handle-census.md").is_file());
    let sync = std::fs::read_to_string(repo_path("packages/sync/mod.sy")).unwrap();
    assert!(
        sync.contains("var handle: Int"),
        "Channel.handle must be Int"
    );
    assert!(
        !sync.contains("var handle: Object"),
        "Channel.handle must not be Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func channel_create(n0: Int) -> Int"),
        "SyxHost.channel_create must return Int"
    );
}

#[test]
fn r6_channel_handle_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r5_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "channel_handle_object_residual="), 0);
    assert_eq!(parse(&stdout, "channel_create_returns_int="), 1);
    assert_eq!(parse(&stdout, "channel_id_accepts_int="), 1);
    assert_eq!(parse(&stdout, "channel_id_bag_dual_track="), 1);
}
