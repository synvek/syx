//! R-7 UdpSocket census contract.
//! Nails: script green, UdpSocket off Object, Int/Dictionary typed.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r7_udp_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r7_udp_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r7_udp_census_files_exist() {
    assert!(repo_path("scripts/r7_udp_census.sh").is_file());
    assert!(repo_path("docs/census/r-7-udp-census.md").is_file());
    let udp = std::fs::read_to_string(repo_path("packages/net/udp.sy")).unwrap();
    assert!(
        udp.contains("func bind(host: String, port: Int) -> Int"),
        "UdpSocket.bind must return Int"
    );
    assert!(
        udp.contains("func sendTo(socket: Int,"),
        "UdpSocket.sendTo socket must be Int"
    );
    assert!(
        udp.contains("func recvFrom(socket: Int, maxBytes: Int) -> Dictionary"),
        "UdpSocket.recvFrom must return Dictionary"
    );
    assert!(
        !udp.lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "UdpSocket methods must not spell Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func udp_recv_from(socket: Int, maxBytes: Int) -> Dictionary"),
        "SyxHost.udp_recv_from must be Int → Dictionary"
    );
}

#[test]
fn r7_udp_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r6_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "udp_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "udp_bind_returns_int="), 1);
    assert_eq!(parse(&stdout, "udp_socket_params_int_typed="), 1);
    assert_eq!(parse(&stdout, "udp_recv_from_dictionary_typed="), 1);
}
