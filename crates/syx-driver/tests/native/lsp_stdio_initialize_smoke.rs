//! LSP stdio Content-Length `initialize` behavioral floor.
//! ≠ marketplace / ≠ GUI E2E.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

fn workspace_syx_bin() -> PathBuf {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_syx") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return path;
        }
    }
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../"));
    let debug = root.join("target/debug/syx");
    if debug.is_file() {
        return debug;
    }
    let release = root.join("target/release/syx");
    if release.is_file() {
        return release;
    }
    which_syx().unwrap_or_else(|| {
        panic!(
            "syx binary not found; build with `cargo build -p syx-cli` \
             (looked under target/debug|release and PATH)"
        )
    })
}

fn which_syx() -> Option<PathBuf> {
    let out = Command::new("which").arg("syx").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        return None;
    }
    let p = PathBuf::from(s);
    p.is_file().then_some(p)
}

fn lsp_frame(body: &str) -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes()
}

fn read_lsp_message(reader: &mut impl BufRead) -> Result<serde_json::Value, String> {
    let mut headers = String::new();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => return Err("EOF before Content-Length headers".into()),
            Ok(_) => {}
            Err(e) => return Err(format!("read header: {e}")),
        }
        if line == "\r\n" || line == "\n" {
            break;
        }
        headers.push_str(&line);
    }
    let content_length = headers
        .lines()
        .find_map(|l| l.strip_prefix("Content-Length:"))
        .and_then(|n| n.trim().parse::<usize>().ok())
        .ok_or_else(|| format!("missing Content-Length in headers: {headers:?}"))?;
    let mut body = vec![0u8; content_length];
    std::io::Read::read_exact(reader, &mut body).map_err(|e| format!("read body: {e}"))?;
    serde_json::from_slice(&body).map_err(|e| format!("json: {e}"))
}

#[test]
fn lsp_stdio_initialize_smoke() {
    let syx = workspace_syx_bin();
    let mut child = Command::new(&syx)
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn `{} lsp`: {e}", syx.display()));

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);

    let child_id = child.id();
    let (done_tx, done_rx) = mpsc::channel::<()>();
    std::thread::spawn(move || {
        if done_rx.recv_timeout(Duration::from_secs(15)).is_err() {
            let _ = Command::new("kill")
                .args(["-9", &child_id.to_string()])
                .status();
        }
    });

    let init_body = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{},"rootUri":null}}"#;
    stdin
        .write_all(&lsp_frame(init_body))
        .expect("write initialize");
    stdin.flush().expect("flush initialize");

    let msg = read_lsp_message(&mut reader).unwrap_or_else(|e| {
        let _ = child.kill();
        let _ = done_tx.send(());
        panic!("LSP initialize read failed: {e}");
    });

    assert_eq!(msg.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"));
    assert_eq!(msg.get("id").and_then(|v| v.as_i64()), Some(1));
    let caps = msg
        .pointer("/result/capabilities")
        .unwrap_or_else(|| panic!("InitializeResult missing capabilities: {msg}"));
    assert!(
        caps.is_object() && !caps.as_object().unwrap().is_empty(),
        "capabilities must be a non-empty object: {caps}"
    );
    assert!(
        caps.get("hoverProvider").is_some() || caps.get("textDocumentSync").is_some(),
        "expected hoverProvider or textDocumentSync in capabilities: {caps}"
    );

    let shutdown = r#"{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}"#;
    stdin
        .write_all(&lsp_frame(shutdown))
        .expect("write shutdown");
    stdin.flush().expect("flush shutdown");
    let _shutdown_resp = read_lsp_message(&mut reader).unwrap_or_else(|e| {
        let _ = child.kill();
        let _ = done_tx.send(());
        panic!("LSP shutdown read failed: {e}");
    });

    let exit = r#"{"jsonrpc":"2.0","method":"exit"}"#;
    stdin.write_all(&lsp_frame(exit)).expect("write exit");
    drop(stdin);

    let status = child.wait().unwrap_or_else(|e| panic!("wait lsp: {e}"));
    let _ = done_tx.send(());
    assert!(
        status.success() || status.code() == Some(0),
        "syx lsp should exit cleanly after shutdown/exit: {status:?}"
    );
}
