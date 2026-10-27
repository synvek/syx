//! `syx bench` allowlist behavioral floor.
//! ≠ coverage / ≠ fuzz / ≠ SLA.

use std::path::PathBuf;
use std::process::Command;

fn workspace_syx_bin() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_syx") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../"));
    for rel in ["target/debug/syx", "target/release/syx"] {
        let p = root.join(rel);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

#[test]
fn syx_bench_cli_list_allowlist() {
    let Some(syx) = workspace_syx_bin() else {
        // Binary may be absent when only driver crate is built; nail allowlist in source.
        let cli = include_str!("../../../syx-cli/src/main.rs");
        for name in [
            "path_join_criterion",
            "http_response_criterion",
            "sql_exec_criterion",
            "tcp_connect_criterion",
            "compile_bench",
        ] {
            assert!(
                cli.contains(name),
                "CLI source must list allowlisted bench {name}"
            );
        }
        return;
    };
    let out = Command::new(&syx)
        .args(["bench", "--list"])
        .output()
        .expect("syx bench --list");
    assert!(
        out.status.success(),
        "syx bench --list failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for name in [
        "path_join_criterion",
        "http_response_criterion",
        "sql_exec_criterion",
        "tcp_connect_criterion",
        "compile_bench",
    ] {
        assert!(
            stdout.lines().any(|l| l.trim() == name),
            "syx bench --list must include {name}; got:\n{stdout}"
        );
    }
    let bad = Command::new(&syx)
        .args(["bench", "not_a_real_bench"])
        .output()
        .expect("syx bench unknown");
    assert!(
        !bad.status.success(),
        "unknown bench name must exit non-zero"
    );
}
