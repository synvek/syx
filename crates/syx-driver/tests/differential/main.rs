//! M0-D01 / ADR-133/134: four-backend exact differential harness (hardened gate).
//!
//! Oracle format (sibling `*.oracle.json` next to each `.sy` fixture):
//! ```json
//! {
//!   "exit": 0,
//!   "stdout": ["15"],
//!   "stderr": [],
//!   "skip": { "wasm": true, "reason": { "wasm": "…" } }
//! }
//! ```
//!
//! Optional fields:
//! - `expect_error: true` — compile/run must fail (Suite-3 connect-refused).
//! - `http_loopback: true` — replace `SYX_DIFF_HTTP_PORT` with a tiny loopback server.
//! - Suite-2 paths: fixtures use `SYX_DIFF_TMP_PATH`; harness substitutes a per-run temp file.
//!
//! Backends: `vm` (embed), `mir`, `wasm`, `native` (SYX_NATIVE_STRICT=1 via Command env).
//! Assertions are exact line-by-line stdout + exit code (no `contains`).
//!
//! Env:
//! - `SYX_DIFF_TRACE=1` — print backend start/end
//! - `SYX_DIFF_BACKEND_TIMEOUT_SECS` — wall timeout for vm/mir/wasm/build (default 120)
//! - `SYX_DIFF_NATIVE_RUN_TIMEOUT_SECS` — native child kill timeout (default 60)

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

mod suite1;
mod suite2;
mod suite3;
mod suite4;

const DEFAULT_BACKEND_TIMEOUT_SECS: u64 = 120;
const DEFAULT_NATIVE_RUN_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Deserialize)]
struct Oracle {
    exit: i32,
    stdout: Vec<String>,
    #[serde(default)]
    stderr: Vec<String>,
    #[serde(default)]
    skip: SkipBackends,
    #[serde(default)]
    expect_error: bool,
    #[serde(default)]
    http_loopback: bool,
}

#[derive(Debug, Default, Deserialize)]
struct SkipBackends {
    #[serde(default)]
    vm: bool,
    #[serde(default)]
    mir: bool,
    #[serde(default)]
    wasm: bool,
    #[serde(default)]
    native: bool,
    #[serde(default)]
    reason: SkipReasons,
}

#[derive(Debug, Default, Deserialize)]
struct SkipReasons {
    #[serde(default)]
    vm: Option<String>,
    #[serde(default)]
    mir: Option<String>,
    #[serde(default)]
    wasm: Option<String>,
    #[serde(default)]
    native: Option<String>,
}

fn mir_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/differential")
}

fn workspace_target_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target")
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn backend_timeout() -> Duration {
    Duration::from_secs(env_u64(
        "SYX_DIFF_BACKEND_TIMEOUT_SECS",
        DEFAULT_BACKEND_TIMEOUT_SECS,
    ))
}

fn native_run_timeout() -> Duration {
    Duration::from_secs(env_u64(
        "SYX_DIFF_NATIVE_RUN_TIMEOUT_SECS",
        DEFAULT_NATIVE_RUN_TIMEOUT_SECS,
    ))
}

fn diff_trace(msg: &str) {
    if std::env::var("SYX_DIFF_TRACE").ok().as_deref() == Some("1") {
        eprintln!("[syx-diff] {msg}");
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }
}

fn validate_skip_reasons(oracle: &Oracle, path: &Path) {
    let s = &oracle.skip;
    let check = |flag: bool, reason: &Option<String>, backend: &str| {
        if flag && reason.as_ref().map(|r| r.trim().is_empty()).unwrap_or(true) {
            panic!(
                "oracle {}: skip.{backend}=true requires skip.reason.{backend}",
                path.display()
            );
        }
    };
    check(s.vm, &s.reason.vm, "vm");
    check(s.mir, &s.reason.mir, "mir");
    check(s.wasm, &s.reason.wasm, "wasm");
    check(s.native, &s.reason.native, "native");
}

fn load_oracle(syx: &Path) -> Oracle {
    let oracle_path = syx.with_extension("oracle.json");
    let text = fs::read_to_string(&oracle_path)
        .unwrap_or_else(|e| panic!("missing oracle {}: {e}", oracle_path.display()));
    let oracle: Oracle = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("bad oracle {}: {e}", oracle_path.display()));
    validate_skip_reasons(&oracle, &oracle_path);
    oracle
}

fn temp_project(tag: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_diff_{tag}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"diff\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    dir
}

fn failure_dir(rel: &str, backend: &str) -> PathBuf {
    let slug = rel.replace('/', "_").replace(".sy", "");
    workspace_target_dir()
        .join("differential-failures")
        .join(slug)
        .join(backend)
}

fn retain_dir_on_failure(rel: &str, backend: &str, src_dir: &Path) -> PathBuf {
    let dest = failure_dir(rel, backend);
    let _ = fs::remove_dir_all(&dest);
    if let Some(parent) = dest.parent() {
        let _ = fs::create_dir_all(parent);
    }
    copy_dir_recursive(src_dir, &dest);
    dest
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    let _ = fs::create_dir_all(dst);
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    for entry in entries.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to);
        } else {
            let _ = fs::copy(&from, &to);
        }
    }
}

fn finish_temp(rel: &str, backend: &str, dir: PathBuf, result: &Result<(i32, Vec<String>), String>) {
    if result.is_err() {
        let kept = retain_dir_on_failure(rel, backend, &dir);
        diff_trace(&format!(
            "{rel} {backend} retained failure artifacts at {}",
            kept.display()
        ));
    }
    let _ = fs::remove_dir_all(&dir);
}

fn cleanup_or_retain_after_assert(
    rel: &str,
    backend: &str,
    work_dir: Option<PathBuf>,
    assert_ok: bool,
) {
    let Some(dir) = work_dir else {
        return;
    };
    if !assert_ok {
        let kept = retain_dir_on_failure(rel, backend, &dir);
        eprintln!(
            "[syx-diff] {rel} {backend} assert failed; artifacts at {}",
            kept.display()
        );
    }
    let _ = fs::remove_dir_all(&dir);
}

/// Serve HTTP/1.1 GET with body `ok` on an ephemeral port (short accept window).
fn spawn_loopback_http() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    listener
        .set_nonblocking(true)
        .expect("nonblocking loopback");
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(30);
        while Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut buf = [0u8; 1024];
                    let _ = stream.read(&mut buf);
                    let resp =
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok";
                    let _ = stream.write_all(resp);
                    let _ = stream.flush();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });
    thread::sleep(Duration::from_millis(20));
    port
}

fn prepare_src(src: &str, oracle: &Oracle, tmp_file: Option<&Path>) -> String {
    let mut out = src.to_string();
    if let Some(path) = tmp_file {
        out = out.replace("SYX_DIFF_TMP_PATH", &path.to_string_lossy());
    }
    if oracle.http_loopback {
        let port = spawn_loopback_http();
        out = out.replace("SYX_DIFF_HTTP_PORT", &port.to_string());
    }
    out
}

fn before_run(rel: &str) {
    if rel.starts_with("suite4/") {
        syx_vm::reset_rusqlite_store();
    }
}

fn run_with_wall_timeout<F>(label: &str, timeout: Duration, f: F) -> Result<(i32, Vec<String>), String>
where
    F: FnOnce() -> Result<(i32, Vec<String>), String> + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(timeout) {
        Ok(r) => r,
        Err(mpsc::RecvTimeoutError::Timeout) => Err(format!(
            "{label}: wall timeout after {}s",
            timeout.as_secs()
        )),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(format!("{label}: worker disconnected before result"))
        }
    }
}

fn run_vm(src: String) -> Result<(i32, Vec<String>), String> {
    run_with_wall_timeout("vm", backend_timeout(), move || {
        syx_host_plugin::register_echo_plugin_in_process();
        match compile_and_run_embedded(&src, &opts()) {
            Ok(result) => Ok((result.exit_code, result.printed)),
            Err(e) => Err(e.to_string()),
        }
    })
}

fn run_mir(rel: &str, src: String) -> (Result<(i32, Vec<String>), String>, Option<PathBuf>) {
    let rel = rel.to_string();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _guard = mir_lock();
        let dir = temp_project("mir", &src);
        let path = dir.join("main.sy");
        syx_host_plugin::register_echo_plugin_in_process();
        let out = run_native_mir_file_printed(&path, &opts()).map_err(|e| e.to_string());
        let _ = tx.send((out, dir));
    });
    match rx.recv_timeout(backend_timeout()) {
        Ok((out, dir)) => {
            if out.is_err() {
                finish_temp(&rel, "mir", dir, &out);
                (out, None)
            } else {
                (out, Some(dir))
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => (
            Err(format!(
                "mir: wall timeout after {}s",
                backend_timeout().as_secs()
            )),
            None,
        ),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            (Err("mir: worker disconnected before result".into()), None)
        }
    }
}

fn run_wasm(rel: &str, src: String) -> (Result<(i32, Vec<String>), String>, Option<PathBuf>) {
    let rel = rel.to_string();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let dir = temp_project("wasm", &src);
        let path = dir.join("main.sy");
        syx_host_plugin::register_echo_plugin_in_process();
        let out = run_wasm_source_printed(&path, "main", &opts()).map_err(|e| e.to_string());
        let _ = tx.send((out, dir));
    });
    match rx.recv_timeout(backend_timeout()) {
        Ok((out, dir)) => {
            if out.is_err() {
                finish_temp(&rel, "wasm", dir, &out);
                (out, None)
            } else {
                (out, Some(dir))
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => (
            Err(format!(
                "wasm: wall timeout after {}s",
                backend_timeout().as_secs()
            )),
            None,
        ),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            (Err("wasm: worker disconnected before result".into()), None)
        }
    }
}

fn wait_child_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<std::process::Output, String> {
    let mut stdout_pipe = child
        .stdout
        .take()
        .ok_or_else(|| "native: missing stdout pipe".to_string())?;
    let mut stderr_pipe = child
        .stderr
        .take()
        .ok_or_else(|| "native: missing stderr pipe".to_string())?;
    let stdout_h = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let stderr_h = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        buf
    });

    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = stdout_h.join();
                    let _ = stderr_h.join();
                    return Err(format!(
                        "native: child timeout after {}s (killed)",
                        timeout.as_secs()
                    ));
                }
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                let _ = child.kill();
                let _ = stdout_h.join();
                let _ = stderr_h.join();
                return Err(format!("try_wait: {e}"));
            }
        }
    };

    let stdout = stdout_h.join().unwrap_or_default();
    let stderr = stderr_h.join().unwrap_or_default();
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

fn run_native(rel: &str, src: String) -> (Result<(i32, Vec<String>), String>, Option<PathBuf>) {
    let rel = rel.to_string();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        syx_host_plugin::register_echo_plugin_in_process();
        let dir = temp_project("native", &src);
        let exe = dir.join("app");
        if let Err(e) = build_native_executable(&dir, &opts(), &exe) {
            let _ = tx.send((Err(e.to_string()), dir));
            return;
        }
        let child = match Command::new(&exe)
            .env("SYX_NATIVE_STRICT", "1")
            .env_remove("SYX_NATIVE_C_STUB")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send((Err(e.to_string()), dir));
                return;
            }
        };
        let output = match wait_child_with_timeout(child, native_run_timeout()) {
            Ok(o) => o,
            Err(e) => {
                let _ = tx.send((Err(e), dir));
                return;
            }
        };
        let code = output.status.code().unwrap_or(1);
        let lines = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(|s| s.to_string())
            .collect();
        let _ = tx.send((Ok((code, lines)), dir));
    });
    match rx.recv_timeout(backend_timeout()) {
        Ok((out, dir)) => {
            if out.is_err() {
                finish_temp(&rel, "native", dir, &out);
                (out, None)
            } else {
                (out, Some(dir))
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => (
            Err(format!(
                "native-build: wall timeout after {}s",
                backend_timeout().as_secs()
            )),
            None,
        ),
        Err(mpsc::RecvTimeoutError::Disconnected) => (
            Err("native: worker disconnected before result".into()),
            None,
        ),
    }
}

fn assert_exact(backend: &str, got_exit: i32, got_out: &[String], oracle: &Oracle, dump: &str) {
    assert_eq!(
        got_exit, oracle.exit,
        "{backend}: exit mismatch; stdout={got_out:?}; dump={dump}"
    );
    assert_eq!(
        got_out,
        oracle.stdout.as_slice(),
        "{backend}: stdout exact mismatch; dump={dump}"
    );
    let _ = &oracle.stderr; // reserved
}

fn assert_backend(
    backend: &str,
    rel: &str,
    result: Result<(i32, Vec<String>), String>,
    work_dir: Option<PathBuf>,
    oracle: &Oracle,
) {
    let dump = failure_dir(rel, backend).display().to_string();
    if oracle.expect_error {
        let ok = result.is_err() || result.as_ref().is_ok_and(|(c, _)| *c != 0);
        if !ok {
            cleanup_or_retain_after_assert(rel, backend, work_dir, false);
        } else {
            cleanup_or_retain_after_assert(rel, backend, work_dir, true);
        }
        assert!(
            ok,
            "{backend}: expected error/non-zero exit, got {result:?}; dump={dump}"
        );
        return;
    }
    match result {
        Ok((code, lines)) => {
            let ok = code == oracle.exit && lines.as_slice() == oracle.stdout.as_slice();
            cleanup_or_retain_after_assert(rel, backend, work_dir, ok);
            assert_exact(backend, code, &lines, oracle, &dump);
        }
        Err(e) => {
            cleanup_or_retain_after_assert(rel, backend, work_dir, false);
            panic!("{backend}: {e}; dump={dump}");
        }
    }
}

/// Run one fixture against all non-skipped backends with exact oracle compare.
pub fn diff_exact(rel: &str) {
    let path = fixtures_root().join(rel);
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let oracle = load_oracle(&path);

    let suite2_tmp = if rel.starts_with("suite2/") {
        let dir = std::env::temp_dir().join(format!(
            "syx_diff_suite2_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        Some(dir.join("fixture.bin"))
    } else {
        None
    };
    let src = prepare_src(&raw, &oracle, suite2_tmp.as_deref());

    if !oracle.skip.vm {
        before_run(rel);
        diff_trace(&format!("{rel} start vm"));
        let result = run_vm(src.clone());
        diff_trace(&format!("{rel} end vm ok={}", result.is_ok()));
        assert_backend("vm", rel, result, None, &oracle);
    }
    if !oracle.skip.mir {
        before_run(rel);
        diff_trace(&format!("{rel} start mir"));
        let (result, dir) = run_mir(rel, src.clone());
        diff_trace(&format!("{rel} end mir ok={}", result.is_ok()));
        assert_backend("mir", rel, result, dir, &oracle);
    }
    if !oracle.skip.wasm {
        before_run(rel);
        diff_trace(&format!("{rel} start wasm"));
        let (result, dir) = run_wasm(rel, src.clone());
        diff_trace(&format!("{rel} end wasm ok={}", result.is_ok()));
        assert_backend("wasm", rel, result, dir, &oracle);
    }
    if !oracle.skip.native {
        before_run(rel);
        diff_trace(&format!("{rel} start native"));
        let (result, dir) = run_native(rel, src);
        diff_trace(&format!("{rel} end native ok={}", result.is_ok()));
        assert_backend("native", rel, result, dir, &oracle);
    }

    if let Some(path) = suite2_tmp {
        let _ = fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}
