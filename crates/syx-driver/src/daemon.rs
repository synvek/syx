//! Local SYX daemon — keeps hydrated prelude warm across `syx run` clients.
//! Protocol: length-prefixed JSON (u32 LE + UTF-8 body) over a Unix domain socket.

use crate::rkyv_image::{store_hydrated_graph, take_hydrated_graph};
use crate::{ensure_prelude_modules_in_graph, run_file_printed, DriverOptions, ModuleGraph};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use syx_pkg::SyxManifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum DaemonRequest {
    Ping,
    Run {
        path: String,
        #[serde(default)]
        with_cache: bool,
    },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonResponse {
    pub ok: bool,
    #[serde(default)]
    pub exit_code: i32,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
}

fn read_msg(stream: &mut UnixStream) -> std::io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > 16 * 1024 * 1024 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "daemon message too large",
        ));
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    Ok(buf)
}

fn write_msg(stream: &mut UnixStream, bytes: &[u8]) -> std::io::Result<()> {
    let len = (bytes.len() as u32).to_le_bytes();
    stream.write_all(&len)?;
    stream.write_all(bytes)?;
    stream.flush()
}

/// Default socket path: `$SYX_DAEMON_SOCK` or `$TMPDIR/syx-daemon-$USER.sock`.
pub fn default_daemon_sock() -> PathBuf {
    if let Ok(p) = std::env::var("SYX_DAEMON_SOCK") {
        return PathBuf::from(p);
    }
    let tmp = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into());
    let user = std::env::var("USER").unwrap_or_else(|_| "syx".into());
    PathBuf::from(tmp).join(format!("syx-daemon-{user}.sock"))
}

fn warm_prelude() {
    if take_hydrated_graph().is_some() {
        return;
    }
    let mut graph = ModuleGraph::default();
    let opts = DriverOptions {
        stdlib_mir: false,
        incremental_cache: false,
        ..DriverOptions::default()
    };
    let root = std::env::temp_dir();
    let _ = ensure_prelude_modules_in_graph(&mut graph, &SyxManifest::default(), &opts, &root);
    if !graph.modules.is_empty() {
        store_hydrated_graph(graph);
    }
}

/// Serve until Shutdown or listener error. Removes the socket file on exit.
pub fn run_daemon(sock: &Path) -> std::io::Result<()> {
    if sock.exists() {
        let _ = std::fs::remove_file(sock);
    }
    if let Some(parent) = sock.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let listener = UnixListener::bind(sock)?;
    eprintln!("syx daemon listening on {}", sock.display());
    warm_prelude();
    eprintln!("syx daemon prelude warm");

    let sock_path = sock.to_path_buf();
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("daemon accept error: {e}");
                continue;
            }
        };
        let body = match read_msg(&mut stream) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("daemon read error: {e}");
                continue;
            }
        };
        let req: DaemonRequest = match serde_json::from_slice(&body) {
            Ok(r) => r,
            Err(e) => {
                let resp = DaemonResponse {
                    ok: false,
                    exit_code: 2,
                    stdout: String::new(),
                    stderr: format!("bad request: {e}"),
                };
                let _ = write_msg(&mut stream, &serde_json::to_vec(&resp).unwrap_or_default());
                continue;
            }
        };
        match req {
            DaemonRequest::Ping => {
                let resp = DaemonResponse {
                    ok: true,
                    exit_code: 0,
                    stdout: "pong".into(),
                    stderr: String::new(),
                };
                let _ = write_msg(&mut stream, &serde_json::to_vec(&resp).unwrap_or_default());
            }
            DaemonRequest::Shutdown => {
                let resp = DaemonResponse {
                    ok: true,
                    exit_code: 0,
                    stdout: "bye".into(),
                    stderr: String::new(),
                };
                let _ = write_msg(&mut stream, &serde_json::to_vec(&resp).unwrap_or_default());
                break;
            }
            DaemonRequest::Run { path, with_cache } => {
                let resp = handle_run(&path, with_cache);
                let _ = write_msg(&mut stream, &serde_json::to_vec(&resp).unwrap_or_default());
            }
        }
    }
    let _ = std::fs::remove_file(&sock_path);
    Ok(())
}

fn handle_run(path: &str, with_cache: bool) -> DaemonResponse {
    let path = PathBuf::from(path);
    let options = DriverOptions {
        incremental_cache: with_cache,
        stdlib_mir: false,
        ..DriverOptions::default()
    };
    match run_file_printed(&path, &options) {
        Ok((code, printed)) => DaemonResponse {
            ok: true,
            exit_code: code,
            stdout: printed.join("\n") + if printed.is_empty() { "" } else { "\n" },
            stderr: String::new(),
        },
        Err(e) => DaemonResponse {
            ok: false,
            exit_code: 1,
            stdout: String::new(),
            stderr: format!("{e:?}"),
        },
    }
}

/// Client: send one request and return the response.
pub fn daemon_request(sock: &Path, req: &DaemonRequest) -> Result<DaemonResponse, String> {
    let mut stream =
        UnixStream::connect(sock).map_err(|e| format!("connect {}: {e}", sock.display()))?;
    let body = serde_json::to_vec(req).map_err(|e| e.to_string())?;
    write_msg(&mut stream, &body).map_err(|e| e.to_string())?;
    let resp_bytes = read_msg(&mut stream).map_err(|e| e.to_string())?;
    serde_json::from_slice(&resp_bytes).map_err(|e| e.to_string())
}

/// True when `SYX_DAEMON=1` or socket exists at default / env path.
pub fn daemon_available(sock: &Path) -> bool {
    sock.exists()
}
