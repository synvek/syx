//! R-10 WebSocket census contract.
//! Nails: script green, connect/send/recv/close → Dictionary.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r10_websocket_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r10_websocket_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r10_websocket_census_files_exist() {
    assert!(repo_path("scripts/r10_websocket_census.sh").is_file());
    assert!(repo_path("docs/census/r-10-websocket-census.md").is_file());
    let ws = std::fs::read_to_string(repo_path("packages/net/websocket.sy")).unwrap();
    assert!(
        ws.contains("func connect(url: String) -> Dictionary"),
        "WebSocket.connect must return Dictionary"
    );
    assert!(
        ws.contains("func send(socket: Dictionary,"),
        "WebSocket.send socket must be Dictionary"
    );
    assert!(
        !ws
            .lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "WebSocket methods must not spell Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func ws_connect(url: String) -> Dictionary"),
        "SyxHost.ws_connect must return Dictionary"
    );
    assert!(
        host.contains("func ws_send(socket: Dictionary,"),
        "SyxHost.ws_send socket must be Dictionary"
    );
}

#[test]
fn r10_websocket_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r9_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "websocket_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "websocket_connect_returns_dictionary="), 1);
    assert_eq!(parse(&stdout, "websocket_socket_params_dictionary="), 1);
    assert_eq!(parse(&stdout, "websocket_syxhost_dictionary="), 1);
    assert_eq!(parse(&stdout, "networking_websocket_mirror_dictionary="), 1);
}
