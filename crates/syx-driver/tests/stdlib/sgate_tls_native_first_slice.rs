//! sgate TLS native first slice.
//! Locks product wiring (tls_native + main) and live HTTPS /health via syx.
//! ≠ net.tls.server HostFn GA; ≠ websocat ws_routes; ≠ Playwright; ≠ todo 8 overall closeout.

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn find_syx(root: &std::path::Path) -> Option<PathBuf> {
    for rel in ["target/release/syx", "target/debug/syx"] {
        let p = root.join(rel);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn wait_tcp(addr: &str, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if TcpStream::connect(addr).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

#[test]
fn sgate_tls_native_first_slice_source_wiring() {
    let tls_native = include_str!("../../../../tools/sgate/tls_native.sy");
    assert!(
        tls_native.contains("SYX_HTTP_SERVE_TLS_CERT")
            && tls_native.contains("SYX_HTTP_SERVE_TLS_KEY")
            && tls_native.contains("Env.set")
            && tls_native.contains("tlsPrepareNativeFromConfig"),
        "tls_native.sy must Env.set SYX_HTTP_SERVE_TLS_* for Http.serve"
    );
    let main = include_str!("../../../../tools/sgate/main.sy");
    assert!(
        main.contains("tls_native")
            && main.contains("tlsPrepareNativeFromConfig")
            && main.contains("SYX_SGATE_CONFIG")
            && main.contains("mode == \"native\""),
        "main.sy must wire native TLS prepare + SYX_SGATE_CONFIG"
    );
    let tls = include_str!("../../../../tools/sgate/tls.sy");
    assert!(
        tls.contains("__syx_tls_serve") && tls.contains("if false"),
        "tls.sy must retain __syx_tls_serve honesty under if false (≠ product native path)"
    );
    let readme = include_str!("../../../../tools/sgate/README.md");
    assert!(
        readme.contains("W1249")
            && readme.contains("tls_native.sy")
            && readme.contains("mode: \"native\""),
        "sgate README must document W1249 native TLS product path"
    );
}

#[test]
fn sgate_tls_native_first_slice_https_health_live() {
    let root = repo_root();
    let Some(syx) = find_syx(&root) else {
        eprintln!("skip live nail: no syx binary under target/{{release,debug}}");
        return;
    };

    let dir = root.join("tools/sgate/.tmp_w1249_native_nail");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir nail dir");

    // Self-signed PEM via openssl (same shape as test.sh).
    let cert = dir.join("server.pem");
    let key = dir.join("server.key");
    let openssl_ok = Command::new("openssl")
        .args([
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-nodes",
            "-keyout",
            key.to_str().unwrap(),
            "-out",
            cert.to_str().unwrap(),
            "-days",
            "1",
            "-subj",
            "/CN=localhost",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !openssl_ok {
        eprintln!("skip live nail: openssl cert generation failed");
        return;
    }

    let port: u16 = 18447;
    let cfg_path = dir.join("native.json");
    let cfg = format!(
        r#"{{
  "server": {{
    "listen": "127.0.0.1",
    "port": 18087,
    "access_log": false,
    "worker_name": "sgate-w1249-nail",
    "tls": {{
      "enabled": true,
      "port": {port},
      "cert": "{cert}",
      "key": "{key}",
      "backend_port": 18087,
      "mode": "native"
    }},
    "hot_reload": {{ "enabled": false, "interval_ms": 1000 }}
  }},
  "upstreams": [],
  "locations": [
    {{
      "name": "health",
      "exact": "/health",
      "status": 200,
      "body": "ok",
      "x_sgate_handler": "health"
    }}
  ],
  "ws_routes": [],
  "error_pages": {{
    "not_found": "Not Found",
    "bad_gateway": "Bad Gateway",
    "gateway_timeout": "Gateway Timeout"
  }}
}}"#,
        port = port,
        cert = cert.display(),
        key = key.display()
    );
    std::fs::write(&cfg_path, cfg).expect("write native.json");

    let sgate_dir = root.join("tools/sgate");
    let log_path = dir.join("sgate.log");
    let log_file = std::fs::File::create(&log_path).expect("sgate log");
    let mut child = Command::new(&syx)
        .current_dir(&root)
        .env("SYX_SGATE_CONFIG", cfg_path.as_os_str())
        .args(["run", sgate_dir.to_str().unwrap()])
        .stdout(Stdio::from(log_file.try_clone().unwrap()))
        .stderr(Stdio::from(log_file))
        .spawn()
        .expect("spawn sgate");

    let addr = format!("127.0.0.1:{port}");
    let up = wait_tcp(&addr, Duration::from_secs(30));
    if !up {
        let _ = child.kill();
        let _ = child.wait();
        let log = std::fs::read_to_string(&log_path).unwrap_or_default();
        panic!("sgate native did not listen on {addr}\nlog:\n{log}");
    }

    let curl = Command::new("curl")
        .args([
            "-sk",
            "--max-time",
            "5",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            &format!("https://{addr}/health"),
        ])
        .output();

    let _ = child.kill();
    let _ = child.wait();

    let code = match curl {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Ok(o) => {
            let log = std::fs::read_to_string(&log_path).unwrap_or_default();
            panic!(
                "curl failed status={:?} stderr={} log=\n{log}",
                o.status,
                String::from_utf8_lossy(&o.stderr)
            );
        }
        Err(e) => {
            eprintln!("skip live assert detail: curl missing ({e}); listen-up already checked");
            return;
        }
    };
    assert_eq!(
        code,
        "200",
        "native HTTPS /health want 200, got {code}\n--- sgate log ---\n{}",
        std::fs::read_to_string(&log_path).unwrap_or_default()
    );
}
