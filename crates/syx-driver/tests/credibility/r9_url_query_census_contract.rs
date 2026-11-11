//! R-9 Url query census contract.
//! Nails: script green, encodeQuery/build query → Dictionary.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r9_url_query_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r9_url_query_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r9_url_query_census_files_exist() {
    assert!(repo_path("scripts/r9_url_query_census.sh").is_file());
    assert!(repo_path("docs/census/r-9-url-query-census.md").is_file());
    let url = std::fs::read_to_string(repo_path("packages/net/url.sy")).unwrap();
    assert!(
        url.contains("func encodeQuery(query: Dictionary) -> String"),
        "Url.encodeQuery query must be Dictionary"
    );
    assert!(
        url.contains("func build(base: String, path: String, query: Dictionary) -> String"),
        "Url.build query must be Dictionary"
    );
    assert!(
        !url
            .lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "Url methods must not spell Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func url_encode_query(query: Dictionary) -> String"),
        "SyxHost.url_encode_query must take Dictionary"
    );
    assert!(
        host.contains("func url_build(base: String, path: String, query: Dictionary) -> String"),
        "SyxHost.url_build must be base:String + query:Dictionary"
    );
}

#[test]
fn r9_url_query_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r8_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "url_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "url_encode_query_param_dictionary="), 1);
    assert_eq!(parse(&stdout, "url_build_query_param_dictionary="), 1);
    assert_eq!(parse(&stdout, "url_syxhost_query_dictionary="), 1);
    assert_eq!(parse(&stdout, "networking_url_mirror_dictionary="), 1);
}
