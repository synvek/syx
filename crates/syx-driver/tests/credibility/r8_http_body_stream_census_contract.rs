//! R-8 HttpBodyStream census contract.
//! Nails: script green, open* → Dictionary, chunk/close Int.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r8_http_body_stream_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r8_http_body_stream_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r8_http_body_stream_census_files_exist() {
    assert!(repo_path("scripts/r8_http_body_stream_census.sh").is_file());
    assert!(repo_path("docs/census/r-8-http-body-stream-census.md").is_file());
    let src = std::fs::read_to_string(repo_path("packages/net/http_body_stream.sy")).unwrap();
    assert!(
        src.contains("func open(url: String) -> Dictionary"),
        "HttpBodyStream.open must return Dictionary"
    );
    assert!(
        src.contains("func openWithOptions(url: String) -> Dictionary"),
        "HttpBodyStream.openWithOptions must return Dictionary"
    );
    assert!(
        !src.lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "HttpBodyStream methods must not spell Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func http_body_stream_open(url: String) -> Dictionary"),
        "SyxHost.http_body_stream_open must return Dictionary"
    );
    assert!(
        host.contains(
            "func http_body_stream_open_opts(url: String, options: Dictionary) -> Dictionary"
        ),
        "SyxHost.http_body_stream_open_opts must be Dictionary → Dictionary"
    );
}

#[test]
fn r8_http_body_stream_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r7_hinge_still_green="), 1);
    assert_eq!(
        parse(&stdout, "http_body_stream_public_object_residual="),
        0
    );
    assert_eq!(
        parse(&stdout, "http_body_stream_open_returns_dictionary="),
        1
    );
    assert_eq!(
        parse(
            &stdout,
            "http_body_stream_open_with_options_returns_dictionary="
        ),
        1
    );
    assert_eq!(parse(&stdout, "http_body_stream_chunk_close_int_typed="), 1);
}
