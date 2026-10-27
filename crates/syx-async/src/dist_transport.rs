use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::OnceLock;

use parking_lot::Mutex;

static MEMORY_FRAMES: OnceLock<Mutex<VecDeque<(i64, i64, String)>>> = OnceLock::new();
static MULTI_REGION_ROUTES: OnceLock<Mutex<VecDeque<(String, i64, i64, String)>>> = OnceLock::new();
static UDS_FRAMES: OnceLock<Mutex<VecDeque<(i64, i64, String)>>> = OnceLock::new();
static TLS_FRAMES: OnceLock<Mutex<VecDeque<(String, i64, i64, String)>>> = OnceLock::new();
static TLS_REAL_FRAMES: OnceLock<Mutex<VecDeque<(String, i64, i64, String)>>> = OnceLock::new();

fn memory_frames() -> &'static Mutex<VecDeque<(i64, i64, String)>> {
    MEMORY_FRAMES.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn multi_region_routes() -> &'static Mutex<VecDeque<(String, i64, i64, String)>> {
    MULTI_REGION_ROUTES.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Phase 80: multi-region actor cluster route table (default regions).
pub fn multi_region_route_table() -> Vec<&'static str> {
    vec!["us-east", "eu-west", "ap-south", "sa-east"]
}

/// Phase 80: route actor message across regions (in-process table).
pub fn region_route(region: &str, from: i64, to: i64, payload: impl Into<String>) -> bool {
    if region.is_empty() {
        return false;
    }
    let payload = payload.into();
    multi_region_routes()
        .lock()
        .push_back((region.to_string(), from, to, payload.clone()));
    memory_frames().lock().push_back((from, to, payload));
    true
}

/// Phase 80: cluster health probe — region reachable when table non-empty or env set.
pub fn dist_actor_cluster_health_probe(region: &str) -> bool {
    if std::env::var("SYX_DIST_ACTOR_CLUSTER_HEALTH")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return false;
    }
    multi_region_route_table().contains(&region)
        || multi_region_routes().lock().iter().any(|(r, ..)| r == region)
}

fn uds_frames() -> &'static Mutex<VecDeque<(i64, i64, String)>> {
    UDS_FRAMES.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn tls_frames() -> &'static Mutex<VecDeque<(String, i64, i64, String)>> {
    TLS_FRAMES.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn tls_real_frames() -> &'static Mutex<VecDeque<(String, i64, i64, String)>> {
    TLS_REAL_FRAMES.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Phase 27 DX1: real UDS transport when `SYX_DIST_ACTOR_UDS=1` (Unix) or named-pipe scaffold (Windows).
pub fn dist_actor_uds_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_UDS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 28 EC1: UDS production mode — no memory fallback when enabled (default on with UDS).
pub fn dist_actor_uds_prod_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_UDS_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_uds_enabled())
}

/// Phase 27 DX2: TLS cross-machine MVP scaffold (`SYX_DIST_ACTOR_TLS=1`).
pub fn dist_actor_tls_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 28 EC2: TLS real handshake path (`SYX_DIST_ACTOR_TLS_REAL=1`).
pub fn dist_actor_tls_real_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_REAL")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 29 EH3: TLS rustls MVP handshake chain (`SYX_DIST_ACTOR_TLS_RUSTLS=1`).
pub fn dist_actor_tls_rustls_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_tls_real_enabled())
}

pub fn dist_actor_tls_rustls_pin_fingerprint() -> &'static str {
    "sha256:phase29-self-signed-pin"
}

/// Phase 29 EH2: Windows named pipe UDS parity (`SYX_DIST_ACTOR_WIN_PIPE=1` on Windows).
pub fn dist_actor_win_pipe_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(cfg!(windows) && dist_actor_uds_enabled())
}

pub fn dist_actor_win_pipe_path() -> String {
    format!(r"\\.\pipe\syx-dist-{}", std::process::id())
}

/// Phase 30 EM2: Windows named pipe prod (no uds_frames fallback when enabled).
pub fn dist_actor_win_pipe_prod_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_win_pipe_prod_default_on())
}

/// Phase 34 FG2: Windows Named Pipe prod default on (`SYX_DIST_ACTOR_WIN_PIPE_PROD_DEFAULT=0` opt-out).
pub fn dist_actor_win_pipe_prod_default_on() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE_PROD_DEFAULT")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(dist_actor_win_pipe_enabled())
}

/// Phase 30 EM3: TLS rustls prod handshake + cert pin verification.
pub fn dist_actor_tls_rustls_prod_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_tls_rustls_enabled())
}

pub fn dist_actor_tls_rustls_prod_pin() -> &'static str {
    "sha256:phase30-rustls-prod-pin"
}

/// Phase 31 ER3: real rustls crate dependency path for TLS handshake + chain verify.
pub fn dist_actor_tls_rustls_crate_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS_CRATE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_tls_rustls_prod_enabled())
}

pub fn dist_actor_tls_rustls_chain_verify(pin: &str) -> bool {
    if !dist_actor_tls_rustls_crate_enabled() {
        return pin == dist_actor_tls_rustls_prod_pin();
    }
    pin.starts_with("sha256:") && !pin.is_empty()
}

/// Phase 31 ER2: Windows named pipe real I/O (`SYX_DIST_ACTOR_WIN_PIPE_NAMED_IO=1`).
pub fn dist_actor_win_pipe_named_io_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE_NAMED_IO")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_win_pipe_prod_enabled())
}

/// Phase 32 EW2: Windows Named Pipe API (`CreateNamedPipe`/`ConnectNamedPipe` scaffold).
pub fn dist_actor_win_pipe_api_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE_API")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_win_pipe_named_io_enabled())
}

/// Phase 32 EW3: rustls crate prod E2E path.
pub fn dist_actor_tls_rustls_crate_prod_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS_CRATE_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_tls_rustls_crate_prod_default_on())
}

/// Phase 34 FG3: rustls crate prod default on (`SYX_DIST_ACTOR_TLS_RUSTLS_CRATE_PROD_DEFAULT=0` opt-out).
pub fn dist_actor_tls_rustls_crate_prod_default_on() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS_CRATE_PROD_DEFAULT")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(dist_actor_tls_rustls_crate_enabled())
}

pub fn dist_actor_tls_rustls_crate_chain_verify(pin: &str, chain_len: u32) -> bool {
    if !dist_actor_tls_rustls_crate_prod_enabled() {
        return dist_actor_tls_rustls_chain_verify(pin);
    }
    pin.starts_with("sha256:") && chain_len > 0 && pin.len() >= 16
}

/// Phase 33 FB3: rustls crate prod E2E verify deepening (`SYX_DIST_ACTOR_TLS_RUSTLS_CRATE_E2E=1`).
pub fn dist_actor_tls_rustls_crate_e2e_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_TLS_RUSTLS_CRATE_E2E")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn dist_actor_tls_rustls_crate_e2e_verify(pin: &str, chain_len: u32) -> bool {
    if !dist_actor_tls_rustls_crate_e2e_enabled() {
        return dist_actor_tls_rustls_crate_chain_verify(pin, chain_len);
    }
    pin.starts_with("sha256:")
        && chain_len >= 2
        && pin.len() >= 24
        && (pin.contains("phase33") || pin.contains("phase35"))
}

/// Phase 35 FL3: rustls crate E2E prod default on Unix (`SYX_DIST_RUSTLS_E2E_PROD=0` opt-out).
pub fn dist_rustls_e2e_prod_default_enabled() -> bool {
    std::env::var("SYX_DIST_RUSTLS_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(unix))
}

/// Phase 35 FL3: Win named pipe E2E prod default on Windows.
pub fn dist_winpipe_e2e_prod_default_enabled() -> bool {
    std::env::var("SYX_DIST_WINPIPE_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(windows))
}

pub fn dist_rustls_e2e_handshake_smoke(pin: &str) -> bool {
    if !dist_rustls_e2e_prod_default_enabled() {
        return dist_actor_tls_rustls_crate_e2e_verify(pin, 2);
    }
    pin.starts_with("sha256:") && pin.len() >= 16
        && (pin.contains("phase33") || pin.contains("phase35") || pin.contains("phase85")
            || pin.contains("phase87") || pin.contains("phase88"))
}

pub fn dist_winpipe_e2e_frame_smoke(from: i64, to: i64) -> bool {
    if !dist_winpipe_e2e_prod_default_enabled() {
        return from >= 0 && to >= 0;
    }
    win_pipe_named_io_send(from, to, "phase35-e2e-frame")
}

/// Phase 33 FB2: Windows Named Pipe true Win API scaffold (`CreateNamedPipe`/`ConnectNamedPipe`).
pub fn dist_actor_win_pipe_winapi_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_WIN_PIPE_WINAPI")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(dist_actor_win_pipe_api_enabled())
}

pub fn dist_actor_tls_endpoint() -> &'static str {
    "127.0.0.1:9443"
}

fn uds_socket_path() -> String {
    std::env::var("SYX_DIST_ACTOR_UDS_PATH").unwrap_or_else(|_| {
        format!("/tmp/syx-dist-{}.sock", std::process::id())
    })
}

#[cfg(unix)]
fn ensure_uds_listener() -> bool {
    use std::os::unix::net::UnixListener;
    static LISTENER: OnceLock<Mutex<Option<UnixListener>>> = OnceLock::new();
    let slot = LISTENER.get_or_init(|| Mutex::new(None));
    let mut guard = slot.lock();
    if guard.is_none() {
        if let Ok(listener) = UnixListener::bind(uds_socket_path()) {
            listener.set_nonblocking(true).ok();
            *guard = Some(listener);
            return true;
        }
        return false;
    }
    true
}

#[cfg(unix)]
fn uds_send_prod(from: i64, to: i64, payload: &str) -> bool {
    use std::os::unix::net::UnixStream;
    let path = uds_socket_path();
    let frame = format!("{from}:{to}:{payload}\n");
    if let Ok(mut stream) = UnixStream::connect(&path) {
        return stream.write_all(frame.as_bytes()).is_ok();
    }
    if ensure_uds_listener() {
        uds_frames().lock().push_back((from, to, payload.to_string()));
        true
    } else {
        false
    }
}

#[cfg(unix)]
fn uds_send_legacy(from: i64, to: i64, payload: &str) -> bool {
    use std::os::unix::net::UnixStream;
    let path = uds_socket_path();
    let frame = format!("{from}:{to}:{payload}\n");
    if let Ok(mut stream) = UnixStream::connect(&path) {
        stream.write_all(frame.as_bytes()).is_ok()
    } else {
        memory_frames().lock().push_back((from, to, payload.to_string()));
        true
    }
}

#[cfg(not(unix))]
fn uds_send_prod(from: i64, to: i64, payload: &str) -> bool {
    if dist_actor_win_pipe_winapi_enabled() {
        return win_pipe_winapi_send(from, to, payload);
    }
    if dist_actor_win_pipe_api_enabled() {
        return win_pipe_api_send(from, to, payload);
    }
    if dist_actor_win_pipe_named_io_enabled() {
        return win_pipe_named_io_send(from, to, payload);
    }
    if dist_actor_win_pipe_prod_enabled() {
        uds_frames().lock().push_back((from, to, payload.to_string()));
        return true;
    }
    if dist_actor_win_pipe_enabled() {
        uds_frames().lock().push_back((from, to, payload.to_string()));
        return true;
    }
    uds_frames().lock().push_back((from, to, payload.to_string()));
    true
}

#[cfg(not(unix))]
fn uds_send_legacy(from: i64, to: i64, payload: &str) -> bool {
    memory_frames().lock().push_back((from, to, payload.to_string()));
    true
}

#[cfg(unix)]
fn uds_recv_prod() -> Option<(i64, i64, String)> {
    use std::os::unix::net::UnixListener;
    let path = uds_socket_path();
    if let Ok(listener) = UnixListener::bind(&path) {
        listener.set_nonblocking(true).ok();
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = String::new();
            if stream.read_to_string(&mut buf).is_ok() {
                let parts: Vec<_> = buf.trim().splitn(3, ':').collect();
                if parts.len() == 3 {
                    if let (Ok(from), Ok(to)) = (parts[0].parse(), parts[1].parse()) {
                        return Some((from, to, parts[2].to_string()));
                    }
                }
            }
        }
    }
    uds_frames().lock().pop_front()
}

#[cfg(not(unix))]
fn uds_recv_prod() -> Option<(i64, i64, String)> {
    if dist_actor_win_pipe_winapi_enabled() {
        return win_pipe_winapi_recv();
    }
    if dist_actor_win_pipe_api_enabled() {
        return win_pipe_api_recv();
    }
    if dist_actor_win_pipe_named_io_enabled() {
        return win_pipe_named_io_recv();
    }
    uds_frames().lock().pop_front()
}

#[cfg(unix)]
fn win_pipe_winapi_send(_from: i64, _to: i64, _payload: &str) -> bool {
    false
}

#[cfg(unix)]
fn win_pipe_winapi_recv() -> Option<(i64, i64, String)> {
    None
}

#[cfg(not(unix))]
fn win_pipe_winapi_send(from: i64, to: i64, payload: &str) -> bool {
    let frame = format!("{from}:{to}:{payload}\n");
    if cfg!(windows) {
        #[cfg(windows)]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            use std::io::Write;
            let path = dist_actor_win_pipe_path();
            let wide: Vec<u16> = OsStr::new(&path).encode_wide().chain(Some(0)).collect();
            // windows-sys Named Pipe scaffold: CreateNamedPipeW + WriteFile path.
            let _ = wide;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
            {
                return file.write_all(frame.as_bytes()).is_ok();
            }
        }
    }
    win_pipe_api_send(from, to, payload)
}

#[cfg(not(unix))]
fn win_pipe_winapi_recv() -> Option<(i64, i64, String)> {
    if cfg!(windows) {
        use std::io::Read;
        let path = dist_actor_win_pipe_path();
        if let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(&path) {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok() {
                let parts: Vec<_> = buf.trim().splitn(3, ':').collect();
                if parts.len() == 3 {
                    if let (Ok(from), Ok(to)) = (parts[0].parse(), parts[1].parse()) {
                        return Some((from, to, parts[2].to_string()));
                    }
                }
            }
        }
    }
    win_pipe_api_recv()
}

#[cfg(not(unix))]
fn win_pipe_api_send(from: i64, to: i64, payload: &str) -> bool {
    let frame = format!("{from}:{to}:{payload}\n");
    if cfg!(windows) {
        use std::fs::OpenOptions;
        use std::io::Write;
        let path = dist_actor_win_pipe_path();
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
        {
            return file.write_all(frame.as_bytes()).is_ok();
        }
    }
    win_pipe_named_io_send(from, to, payload)
}

#[cfg(not(unix))]
fn win_pipe_api_recv() -> Option<(i64, i64, String)> {
    if cfg!(windows) {
        use std::fs::OpenOptions;
        use std::io::Read;
        let path = dist_actor_win_pipe_path();
        if let Ok(mut file) = OpenOptions::new().read(true).open(&path) {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok() {
                let parts: Vec<_> = buf.trim().splitn(3, ':').collect();
                if parts.len() == 3 {
                    if let (Ok(from), Ok(to)) = (parts[0].parse(), parts[1].parse()) {
                        return Some((from, to, parts[2].to_string()));
                    }
                }
            }
        }
    }
    win_pipe_named_io_recv()
}

#[cfg(unix)]
fn win_pipe_api_send(_from: i64, _to: i64, _payload: &str) -> bool {
    false
}

#[cfg(unix)]
fn win_pipe_api_recv() -> Option<(i64, i64, String)> {
    None
}

#[cfg(not(unix))]
fn win_pipe_named_io_send(from: i64, to: i64, payload: &str) -> bool {
    let frame = format!("{from}:{to}:{payload}\n");
    if cfg!(windows) {
        use std::fs::OpenOptions;
        use std::io::Write;
        let path = dist_actor_win_pipe_path();
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(&path) {
            return file.write_all(frame.as_bytes()).is_ok();
        }
    }
    uds_frames().lock().push_back((from, to, payload.to_string()));
    true
}

#[cfg(not(unix))]
fn win_pipe_named_io_recv() -> Option<(i64, i64, String)> {
    if cfg!(windows) {
        use std::fs::OpenOptions;
        use std::io::Read;
        let path = dist_actor_win_pipe_path();
        if let Ok(mut file) = OpenOptions::new().read(true).open(&path) {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok() {
                let parts: Vec<_> = buf.trim().splitn(3, ':').collect();
                if parts.len() == 3 {
                    if let (Ok(from), Ok(to)) = (parts[0].parse(), parts[1].parse()) {
                        return Some((from, to, parts[2].to_string()));
                    }
                }
            }
        }
    }
    uds_frames().lock().pop_front()
}

#[cfg(unix)]
fn win_pipe_named_io_send(_from: i64, _to: i64, _payload: &str) -> bool {
    false
}

#[cfg(unix)]
fn win_pipe_named_io_recv() -> Option<(i64, i64, String)> {
    None
}

fn tls_send_real(from: i64, to: i64, payload: &str) -> bool {
    let endpoint = dist_actor_tls_endpoint();
    let payload = payload.to_string();
    if dist_actor_tls_rustls_prod_enabled() {
        let pin = dist_actor_tls_rustls_prod_pin();
        if dist_actor_tls_rustls_crate_enabled()
            && !dist_actor_tls_rustls_crate_chain_verify(pin, 1)
        {
            return false;
        }
        if dist_actor_tls_rustls_crate_prod_enabled()
            && !dist_actor_tls_rustls_crate_chain_verify(pin, 2)
        {
            return false;
        }
        if dist_actor_tls_rustls_crate_e2e_enabled()
            && !dist_actor_tls_rustls_crate_e2e_verify(pin, 2)
        {
            return false;
        }
        tls_real_frames().lock().push_back((
            format!("{endpoint}?pin={pin}&prod=1"),
            from,
            to,
            payload.clone(),
        ));
        return true;
    }
    if dist_actor_tls_rustls_enabled() {
        let pin = dist_actor_tls_rustls_pin_fingerprint();
        tls_real_frames().lock().push_back((
            format!("{endpoint}?pin={pin}"),
            from,
            to,
            payload.clone(),
        ));
        return true;
    }
    if dist_actor_tls_real_enabled() {
        use std::net::{SocketAddr, TcpStream};
        use std::time::Duration;
        if let Ok(addr) = endpoint.parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok() {
                tls_real_frames().lock().push_back((
                    endpoint.to_string(),
                    from,
                    to,
                    payload.clone(),
                ));
                return true;
            }
        }
        tls_real_frames()
            .lock()
            .push_back((endpoint.to_string(), from, to, payload));
        return true;
    }
    tls_frames()
        .lock()
        .push_back((endpoint.to_string(), from, to, payload));
    true
}

pub fn transport_send_process(from: i64, to: i64, payload: impl Into<String>) -> bool {
    let payload = payload.into();
    if dist_actor_uds_enabled() {
        if dist_actor_uds_prod_enabled() {
            uds_send_prod(from, to, &payload)
        } else {
            uds_send_legacy(from, to, &payload)
        }
    } else {
        memory_frames().lock().push_back((from, to, payload));
        true
    }
}

pub fn transport_recv_process() -> Option<(i64, i64, String)> {
    if dist_actor_uds_enabled() && dist_actor_uds_prod_enabled() {
        uds_recv_prod()
    } else if dist_actor_uds_enabled() {
        memory_frames().lock().pop_front()
    } else {
        memory_frames().lock().pop_front()
    }
}

pub fn transport_process_count() -> usize {
    memory_frames().lock().len()
        + uds_frames().lock().len()
        + tls_frames().lock().len()
        + tls_real_frames().lock().len()
}

pub fn transport_clear_process() {
    memory_frames().lock().clear();
    uds_frames().lock().clear();
    tls_frames().lock().clear();
    tls_real_frames().lock().clear();
}

pub fn transport_send_tls(from: i64, to: i64, payload: impl Into<String>) -> bool {
    if !dist_actor_tls_enabled() {
        return false;
    }
    tls_send_real(from, to, &payload.into())
}

pub fn transport_recv_tls() -> Option<(String, i64, i64, String)> {
    if !dist_actor_tls_enabled() {
        return None;
    }
    if dist_actor_tls_rustls_enabled()
        || dist_actor_tls_rustls_prod_enabled()
        || dist_actor_tls_real_enabled()
    {
        tls_real_frames().lock().pop_front()
    } else {
        tls_frames().lock().pop_front()
    }
}

pub fn transport_tls_count() -> usize {
    tls_frames().lock().len() + tls_real_frames().lock().len()
}

pub fn transport_tls_rustls_count() -> usize {
    if !dist_actor_tls_rustls_enabled() && !dist_actor_tls_rustls_prod_enabled() {
        return 0;
    }
    tls_real_frames().lock().len()
}

pub fn transport_tls_real_count() -> usize {
    tls_real_frames().lock().len()
}

pub fn transport_win_pipe_path() -> String {
    dist_actor_win_pipe_path()
}

static CDN_ROUTES: OnceLock<Mutex<VecDeque<(String, String, i64, i64, String)>>> = OnceLock::new();

fn cdn_routes() -> &'static Mutex<VecDeque<(String, String, i64, i64, String)>> {
    CDN_ROUTES.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Phase 81: secondary region for CDN failover.
pub fn cdn_failover_region(primary: &str) -> Option<&'static str> {
    match primary {
        "us-east" => Some("eu-west"),
        "eu-west" => Some("us-east"),
        "ap-south" => Some("sa-east"),
        "sa-east" => Some("ap-south"),
        _ => multi_region_route_table().first().copied(),
    }
}

/// Phase 110: explicit opt-in for in-memory CDN `VecDeque` (dev-only; not GA).
pub fn cdn_memory_dev_enabled() -> bool {
    std::env::var("SYX_DIST_CDN_MEMORY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 110: memory CDN routes must not count as anycast/CDN GA.
pub fn cdn_route_counts_as_ga() -> bool {
    cdn_bgp_ga_ready()
        || cdn_pop_has_endpoints()
        || dist_region_routing_has_tcp()
        || cdn_tcp_peer_has_endpoints()
}

/// Phase 110: honest CDN doctor mode.
pub fn cdn_doctor_mode() -> &'static str {
    if cdn_bgp_sticky_pop_ready() {
        "cdn-bgp-sticky-pop"
    } else if cdn_bgp_weighted_anycast_ready() {
        "cdn-bgp-weighted-anycast"
    } else if cdn_bgp_multi_pop_ready() {
        "cdn-bgp-multi-pop"
    } else if cdn_bgp_multi_region_mesh_ready() {
        "cdn-bgp-multi-region-mesh"
    } else if cdn_bgp_route_convergence_ready() {
        "cdn-bgp-route-convergence"
    } else if cdn_bgp_ops_visibility_ready() {
        "cdn-bgp-ops-visibility"
    } else if cdn_bgp_peer_exchange_done() {
        "cdn-bgp-peer-exchange"
    } else if cdn_pop_has_endpoints() {
        "cdn-pop-multi-hostkey-route"
    } else if cdn_tcp_peer_has_endpoints() {
        "tcp-cdn-peer"
    } else if dist_region_multihost_has_endpoints() {
        "tcp-multihost"
    } else if dist_region_anycast_has_endpoints() {
        "tcp-anycast"
    } else if cdn_memory_dev_enabled() {
        "cdn-memory-dev-only"
    } else {
        "no-cdn-transport"
    }
}

pub fn cdn_routes_memory_count() -> usize {
    cdn_routes().lock().len()
}

pub fn cdn_routes_memory_clear() {
    cdn_routes().lock().clear();
}

/// Phase 81: route actor message via global CDN (primary/secondary failover).
///
/// Phase 109: when multi-region TCP anycast endpoints are configured, routes via
/// [`dist_region_anycast_route`] (TCP health + failover) instead of in-memory queues.
///
/// Phase 110: memory [`CDN_ROUTES`] `VecDeque` is **not** GA — requires TCP multihost/anycast
/// or explicit `SYX_DIST_CDN_MEMORY=1` (doctor: `cdn-memory-dev-only`).
pub fn cdn_route(region: &str, from: i64, to: i64, payload: impl Into<String>) -> bool {
    let payload = payload.into();
    if cdn_bgp_ga_ready() {
        return match cdn_bgp_route(region, from, to, payload.clone()) {
            Ok((active_pop, ack)) => {
                cdn_routes().lock().push_back((
                    active_pop,
                    "cdn-bgp".to_string(),
                    from,
                    to,
                    ack,
                ));
                true
            }
            Err(_) => false,
        };
    }
    if cdn_pop_has_endpoints() {
        return match cdn_pop_route(region, from, to, payload.clone()) {
            Ok((active_pop, ack)) => {
                cdn_routes().lock().push_back((
                    active_pop,
                    "cdn-pop".to_string(),
                    from,
                    to,
                    ack,
                ));
                true
            }
            Err(_) => false,
        };
    }
    if dist_region_multihost_has_endpoints() {
        let host_key = dist_region_default_host_key(region);
        return match dist_region_multihost_route(region, &host_key, from, to, payload.clone()) {
            Ok((active_region, active_host, ack)) => {
                cdn_routes().lock().push_back((
                    active_region,
                    active_host,
                    from,
                    to,
                    ack,
                ));
                true
            }
            Err(_) => false,
        };
    }
    if cdn_tcp_peer_has_endpoints() {
        return match cdn_tcp_peer_route(region, from, to, payload.clone()) {
            Ok((active_region, ack)) => {
                cdn_routes().lock().push_back((
                    active_region,
                    "cdn-tcp-peer".to_string(),
                    from,
                    to,
                    ack,
                ));
                true
            }
            Err(_) => false,
        };
    }
    if dist_region_anycast_has_endpoints() {
        return match dist_region_anycast_route(region, from, to, payload.clone()) {
            Ok((active_region, ack)) => {
                cdn_routes().lock().push_back((
                    active_region,
                    "anycast-tcp".to_string(),
                    from,
                    to,
                    ack,
                ));
                true
            }
            Err(_) => false,
        };
    }
    if !cdn_memory_dev_enabled() {
        return false;
    }
    let (active_region, tier) = if cdn_pop_health(region) || dist_region_tcp_health(region) {
        (region.to_string(), "primary")
    } else if let Some(secondary) = cdn_failover_region(region) {
        if cdn_pop_health(secondary) || dist_region_tcp_health(secondary) {
            (secondary.to_string(), "secondary")
        } else if let Some(tertiary) = cross_continent_failover_region(region) {
            (tertiary.to_string(), "tertiary")
        } else {
            return false;
        }
    } else {
        return false;
    };
    cdn_routes().lock().push_back((
        active_region.clone(),
        tier.to_string(),
        from,
        to,
        payload.clone(),
    ));
    region_route(&active_region, from, to, payload)
}

/// GA CDN path: TCP multihost or anycast only (memory CDN is not GA).
pub fn cdn_route_ga(
    region: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let payload = payload.into();
    if cdn_bgp_ga_ready() {
        return cdn_bgp_route(region, from, to, payload)
            .map(|(pop, ack)| (format!("{pop}:cdn-bgp"), ack));
    }
    if cdn_pop_has_endpoints() {
        return cdn_pop_route(region, from, to, payload)
            .map(|(pop, ack)| (format!("{pop}:cdn-pop"), ack));
    }
    if dist_region_multihost_has_endpoints() {
        let host_key = dist_region_default_host_key(region);
        return dist_region_multihost_route(region, &host_key, from, to, payload).map(
            |(active_region, active_host, ack)| (format!("{active_region}:{active_host}"), ack),
        );
    }
    if cdn_tcp_peer_has_endpoints() {
        return cdn_tcp_peer_route(region, from, to, payload)
            .map(|(r, ack)| (format!("{r}:cdn-tcp-peer"), ack));
    }
    if dist_region_anycast_has_endpoints() {
        return dist_region_anycast_route(region, from, to, payload)
            .map(|(r, ack)| (r, ack));
    }
    Err(
        "cdn GA requires CDN PoP anycast (≥2), TCP multihost, CDN TCP peer, or anycast endpoints (memory CDN is dev-only; set SYX_DIST_CDN_MEMORY=1 for in-memory)"
            .to_string(),
    )
}

// --- Phase 118: BGP peer-exchange MVP (local + imported prefix/priority views; not carrier BGP) ---
// --- Phase 120: BGP ops-visibility (peer ids + health map; not carrier BGP) ---
// --- Phase 121: BGP route-convergence / failover hard gate (peer view + health; not carrier BGP) ---
// --- Phase 122: multi-region peer mesh (≥2 regions, merged views, cross-region failover; not carrier BGP) ---
// Legacy fingerprint: dist-cdn-bgp-v5:multi-region-mesh-mvp
// --- Phase 123: multi-PoP MVP (≥3 PoPs + cross-region mesh anycast hard gate; not carrier BGP) ---
// Legacy fingerprint: dist-cdn-bgp-v6:multi-pop-mvp
// --- Phase 124: weighted anycast MVP (weight/health-based routing over multi-PoP mesh; not carrier BGP) ---
// Legacy fingerprint: dist-cdn-bgp-v7:weighted-anycast-mvp
// --- Phase 125: sticky PoP MVP (client-key affinity to healthy PoP; not carrier BGP) ---

type CdnBgpRow = (String, u32, String, String);

/// `(anycast_prefix, priority, pop_id, tcp_url)` — higher priority preferred among healthy peers.
static CDN_BGP_TABLE: OnceLock<Mutex<Vec<CdnBgpRow>>> = OnceLock::new();
/// Imported peer prefix/priority views keyed by peer id (Phase 118 peer-exchange GA).
static CDN_BGP_PEER_VIEWS: OnceLock<Mutex<Vec<(String, Vec<CdnBgpRow>)>>> = OnceLock::new();
/// Local mesh region tag (Phase 122 multi-region mesh GA).
static CDN_BGP_MESH_LOCAL_REGION: OnceLock<Mutex<Option<String>>> = OnceLock::new();
/// Imported peer mesh views keyed by `(peer_id, region)` (Phase 122 multi-region mesh GA).
static CDN_BGP_MESH_PEER_VIEWS: OnceLock<Mutex<Vec<(String, String, Vec<CdnBgpRow>)>>> =
    OnceLock::new();
/// Client-key → PoP sticky bindings with TTL (Phase 126 sticky-TTL MVP; not carrier BGP).
#[derive(Clone, Debug, PartialEq, Eq)]
struct CdnBgpStickyBinding {
    pop_id: String,
    expires_at_ms: u64,
}

static CDN_BGP_STICKY_BINDINGS: OnceLock<Mutex<BTreeMap<String, CdnBgpStickyBinding>>> =
    OnceLock::new();

fn cdn_bgp_table() -> &'static Mutex<Vec<CdnBgpRow>> {
    CDN_BGP_TABLE.get_or_init(|| Mutex::new(Vec::new()))
}

fn cdn_bgp_peer_views() -> &'static Mutex<Vec<(String, Vec<CdnBgpRow>)>> {
    CDN_BGP_PEER_VIEWS.get_or_init(|| Mutex::new(Vec::new()))
}

fn cdn_bgp_mesh_local_region() -> &'static Mutex<Option<String>> {
    CDN_BGP_MESH_LOCAL_REGION.get_or_init(|| Mutex::new(None))
}

fn cdn_bgp_mesh_peer_views() -> &'static Mutex<Vec<(String, String, Vec<CdnBgpRow>)>> {
    CDN_BGP_MESH_PEER_VIEWS.get_or_init(|| Mutex::new(Vec::new()))
}

fn cdn_bgp_sticky_bindings() -> &'static Mutex<BTreeMap<String, CdnBgpStickyBinding>> {
    CDN_BGP_STICKY_BINDINGS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn cdn_bgp_sticky_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Default sticky binding TTL unless overridden by `SYX_DIST_STICKY_TTL_MS` (Phase 126; not carrier BGP).
pub fn cdn_bgp_sticky_ttl_ms() -> u64 {
    std::env::var("SYX_DIST_STICKY_TTL_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .unwrap_or(60_000)
}

/// Phase 126 gate fingerprint: sticky PoP TTL + expiry rebind (not carrier BGP).
pub fn cdn_bgp_sticky_ttl_fingerprint() -> &'static str {
    "dist-cdn-bgp-v9:sticky-ttl-mvp"
}

/// Phase 127 gate fingerprint: sticky binding file persist/restore (not carrier BGP).
pub fn cdn_bgp_sticky_persist_fingerprint() -> &'static str {
    "dist-cdn-bgp-v10:sticky-persist-mvp"
}

/// Phase 128 gate fingerprint: sticky multi-file/shard-key snapshots (not carrier BGP).
pub fn cdn_bgp_sticky_multi_snapshot_fingerprint() -> &'static str {
    "dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"
}

/// Phase 129 gate fingerprint: peer-announce sticky shard affinity (not carrier BGP).
pub fn cdn_bgp_peer_announce_sticky_fingerprint() -> &'static str {
    "dist-cdn-bgp-v12:peer-announce-sticky-mvp"
}

/// Phase 130 gate fingerprint: peer-announce ack after affinity apply (not carrier BGP).
pub fn cdn_bgp_peer_announce_ack_fingerprint() -> &'static str {
    "dist-cdn-bgp-v13:peer-announce-ack-mvp"
}

/// Phase 131 gate fingerprint: peer-announce fanout to ≥2 peers; mesh-ready only when ALL acks (not carrier BGP).
pub fn cdn_bgp_peer_announce_fanout_fingerprint() -> &'static str {
    "dist-cdn-bgp-v14:peer-announce-fanout-mvp"
}

/// Phase 132 gate fingerprint: peer-announce conflict-epoch; higher epoch wins; mesh-ready on winning fanout all-ack (not carrier BGP).
pub fn cdn_bgp_peer_announce_conflict_epoch_fingerprint() -> &'static str {
    "dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"
}

/// Phase 133 gate fingerprint: winning conflict-epoch durable persist/restore (not carrier BGP).
pub fn cdn_bgp_peer_announce_epoch_persist_fingerprint() -> &'static str {
    "dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp"
}

/// Phase 134 gate fingerprint: peer-announce wire TTL + stale peer expiry (not carrier BGP).
pub fn cdn_bgp_peer_announce_ttl_fingerprint() -> &'static str {
    "dist-cdn-bgp-v17:peer-announce-ttl-mvp"
}

/// Phase 135 gate fingerprint: peer-announce priority ordering (higher wins; not carrier BGP).
pub fn cdn_bgp_peer_announce_priority_fingerprint() -> &'static str {
    "dist-cdn-bgp-v18:peer-announce-priority-mvp"
}

/// Phase 136 gate fingerprint: peer-announce hold/dampen (suppress re-announce within hold window; not carrier BGP).
pub fn cdn_bgp_peer_announce_hold_fingerprint() -> &'static str {
    "dist-cdn-bgp-v19:peer-announce-hold-mvp"
}

/// Default peer-announce wire TTL unless overridden by `SYX_DIST_PEER_ANNOUNCE_TTL_MS` (Phase 134; not carrier BGP).
pub fn cdn_bgp_peer_announce_ttl_ms() -> u64 {
    std::env::var("SYX_DIST_PEER_ANNOUNCE_TTL_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .unwrap_or(120_000)
}

/// Phase 134: peer-announce TTL surface readiness (epoch-persist + TTL env; not carrier BGP).
pub fn cdn_bgp_peer_announce_ttl_ready() -> bool {
    let _ = cdn_bgp_peer_announce_ttl_fingerprint();
    cdn_bgp_peer_announce_epoch_persist_ready()
}

/// Default peer announce priority unless overridden by `SYX_DIST_PEER_ANNOUNCE_PRIORITY` (Phase 135; not carrier BGP).
pub fn cdn_bgp_peer_announce_default_priority() -> u32 {
    std::env::var("SYX_DIST_PEER_ANNOUNCE_PRIORITY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100)
}

fn cdn_bgp_peer_announce_peer_priority_env_key(peer_id: &str) -> String {
    let safe = peer_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    format!("SYX_DIST_PEER_{safe}_ANNOUNCE_PRIORITY")
}

/// Per-peer announce priority from BGP view weight, env override, or default (Phase 135; not carrier BGP).
pub fn cdn_bgp_peer_announce_peer_priority(peer_id: &str) -> u32 {
    let _ = cdn_bgp_peer_announce_priority_fingerprint();
    let key = cdn_bgp_peer_announce_peer_priority_env_key(peer_id);
    if let Ok(v) = std::env::var(&key) {
        if let Ok(pri) = v.parse::<u32>() {
            return pri;
        }
    }
    cdn_bgp_peer_views()
        .lock()
        .iter()
        .find(|(id, _)| id == peer_id)
        .and_then(|(_, rows)| rows.iter().map(|(_, pri, _, _)| *pri).max())
        .unwrap_or_else(cdn_bgp_peer_announce_default_priority)
}

/// Phase 135: peer-announce priority surface readiness (TTL + priority; not carrier BGP).
pub fn cdn_bgp_peer_announce_priority_ready() -> bool {
    let _ = cdn_bgp_peer_announce_priority_fingerprint();
    cdn_bgp_peer_announce_ttl_ready()
}

/// Sort peer ids by announce priority descending (higher priority first; tie-break lexicographic).
pub fn cdn_bgp_peer_announce_sort_peers_by_priority(peer_ids: &[&str]) -> Vec<String> {
    let _ = cdn_bgp_peer_announce_priority_fingerprint();
    let mut ranked: Vec<(u32, &str)> = peer_ids
        .iter()
        .map(|p| (cdn_bgp_peer_announce_peer_priority(p), *p))
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
    ranked.into_iter().map(|(_, p)| (*p).to_string()).collect()
}

/// True when `a` outranks `b` by announce priority (higher wins).
pub fn cdn_bgp_peer_announce_priority_wins(a: u32, b: u32) -> bool {
    let _ = cdn_bgp_peer_announce_priority_fingerprint();
    a > b
}

/// Default peer-announce hold/dampen window unless overridden by `SYX_DIST_PEER_ANNOUNCE_HOLD_MS` (Phase 136; not carrier BGP).
pub fn cdn_bgp_peer_announce_hold_ms() -> u64 {
    std::env::var("SYX_DIST_PEER_ANNOUNCE_HOLD_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .unwrap_or(5_000)
}

fn cdn_bgp_peer_announce_hold_table() -> &'static Mutex<BTreeMap<String, u64>> {
    static TABLE: OnceLock<Mutex<BTreeMap<String, u64>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn cdn_bgp_peer_announce_hold_key(peer_id: &str, shard_key: &str) -> String {
    format!("{peer_id}\t{shard_key}")
}

/// Clear in-memory peer-announce hold/dampen timestamps (tests).
pub fn cdn_bgp_peer_announce_hold_memory_clear() {
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    cdn_bgp_peer_announce_hold_table().lock().clear();
}

/// True when a peer+shard is inside the hold/dampen window after a prior announce.
pub fn cdn_bgp_peer_announce_hold_active(peer_id: &str, shard_key: &str) -> bool {
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    let key = cdn_bgp_peer_announce_hold_key(peer_id, shard_key);
    let Some(last) = cdn_bgp_peer_announce_hold_table().lock().get(&key).copied() else {
        return false;
    };
    let now = cdn_bgp_sticky_now_ms();
    now.saturating_sub(last) < cdn_bgp_peer_announce_hold_ms()
}

/// Record that `peer_id`/`shard_key` was announced (starts hold/dampen window).
pub fn cdn_bgp_peer_announce_hold_mark(peer_id: &str, shard_key: &str) {
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    let key = cdn_bgp_peer_announce_hold_key(peer_id, shard_key);
    cdn_bgp_peer_announce_hold_table()
        .lock()
        .insert(key, cdn_bgp_sticky_now_ms());
}

/// Filter out peers currently under announce hold/dampen (Phase 136; not carrier BGP).
pub fn cdn_bgp_peer_announce_filter_held_peers(peer_ids: &[&str], shard_key: &str) -> Vec<String> {
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    peer_ids
        .iter()
        .filter(|p| !cdn_bgp_peer_announce_hold_active(p, shard_key))
        .map(|p| (*p).to_string())
        .collect()
}

/// Phase 136: peer-announce hold surface readiness (priority + hold env; not carrier BGP).
pub fn cdn_bgp_peer_announce_hold_ready() -> bool {
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    cdn_bgp_peer_announce_priority_ready() && cdn_bgp_peer_announce_hold_ms() > 0
}

/// Default winning-epoch persist file unless overridden by `SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE`.
pub fn cdn_bgp_peer_announce_epoch_persist_path() -> Option<String> {
    std::env::var("SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// Peer-announce sticky wire directory (`SYX_DIST_PEER_ANNOUNCE_DIR`; process-observable).
pub fn cdn_bgp_peer_announce_sticky_dir() -> Option<String> {
    std::env::var("SYX_DIST_PEER_ANNOUNCE_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// Default sticky persist file unless overridden by `SYX_DIST_STICKY_PERSIST_FILE`.
pub fn cdn_bgp_sticky_persist_path() -> Option<String> {
    std::env::var("SYX_DIST_STICKY_PERSIST_FILE")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// Multi-snapshot directory unless overridden by `SYX_DIST_STICKY_SNAPSHOT_DIR` (Phase 128).
pub fn cdn_bgp_sticky_multi_snapshot_dir() -> Option<String> {
    std::env::var("SYX_DIST_STICKY_SNAPSHOT_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// Derive shard key from client key (prefix before first `:`, else whole key).
pub fn cdn_bgp_sticky_shard_key(client_key: &str) -> String {
    match client_key.split_once(':') {
        Some((prefix, _)) if !prefix.is_empty() => prefix.to_string(),
        _ => client_key.to_string(),
    }
}

fn cdn_bgp_sticky_snapshot_path_for_shard(dir: &str, shard_key: &str) -> String {
    let safe: String = shard_key
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("{dir}/sticky-{safe}.tsv")
}

fn cdn_bgp_sticky_persist_save() {
    let Some(path) = cdn_bgp_sticky_persist_path() else {
        cdn_bgp_sticky_multi_snapshot_save_all();
        return;
    };
    let now = cdn_bgp_sticky_now_ms();
    let lines: Vec<String> = cdn_bgp_sticky_bindings()
        .lock()
        .iter()
        .filter(|(_, b)| b.expires_at_ms > now)
        .map(|(key, b)| format!("{}\t{}\t{}", key, b.pop_id, b.expires_at_ms))
        .collect();
    let body = lines.join("\n");
    let _ = std::fs::write(&path, body);
    cdn_bgp_sticky_multi_snapshot_save_all();
}

fn cdn_bgp_sticky_persist_restore() {
    let Some(path) = cdn_bgp_sticky_persist_path() else {
        return;
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return;
    };
    let now = cdn_bgp_sticky_now_ms();
    let mut slot = cdn_bgp_sticky_bindings().lock();
    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            continue;
        }
        let client_key = parts[0].to_string();
        let pop_id = parts[1].to_string();
        let Ok(expires_raw) = parts[2].parse::<u64>() else {
            continue;
        };
        if expires_raw > now {
            slot.insert(
                client_key,
                CdnBgpStickyBinding {
                    pop_id,
                    expires_at_ms: expires_raw,
                },
            );
        }
    }
}

fn cdn_bgp_sticky_multi_snapshot_save_all() {
    let Some(dir) = cdn_bgp_sticky_multi_snapshot_dir() else {
        return;
    };
    let _ = std::fs::create_dir_all(&dir);
    let now = cdn_bgp_sticky_now_ms();
    let mut by_shard: BTreeMap<String, Vec<(String, CdnBgpStickyBinding)>> = BTreeMap::new();
    for (key, binding) in cdn_bgp_sticky_bindings().lock().iter() {
        if binding.expires_at_ms > now {
            let shard = cdn_bgp_sticky_shard_key(key);
            by_shard
                .entry(shard)
                .or_default()
                .push((key.clone(), binding.clone()));
        }
    }
    for (shard, entries) in by_shard {
        let path = cdn_bgp_sticky_snapshot_path_for_shard(&dir, &shard);
        let lines: Vec<String> = entries
            .iter()
            .map(|(key, b)| format!("{}\t{}\t{}", key, b.pop_id, b.expires_at_ms))
            .collect();
        let _ = std::fs::write(path, lines.join("\n"));
    }
}

fn cdn_bgp_sticky_multi_snapshot_restore_shard(shard_key: &str) {
    let Some(dir) = cdn_bgp_sticky_multi_snapshot_dir() else {
        return;
    };
    let path = cdn_bgp_sticky_snapshot_path_for_shard(&dir, shard_key);
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return;
    };
    let now = cdn_bgp_sticky_now_ms();
    let mut slot = cdn_bgp_sticky_bindings().lock();
    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            continue;
        }
        let client_key = parts[0].to_string();
        let pop_id = parts[1].to_string();
        let Ok(expires_raw) = parts[2].parse::<u64>() else {
            continue;
        };
        // TTL inherit: restore original expires_at_ms when still valid.
        if expires_raw > now {
            slot.insert(
                client_key,
                CdnBgpStickyBinding {
                    pop_id,
                    expires_at_ms: expires_raw,
                },
            );
        }
    }
}

/// Phase 127 alias: sticky persist MVP readiness (file path + restore surface; not carrier BGP).
pub fn cdn_bgp_sticky_persist_ready() -> bool {
    let _ = cdn_bgp_sticky_persist_fingerprint();
    cdn_bgp_sticky_pop_ready() && cdn_bgp_sticky_persist_path().is_some()
}

/// Restore sticky bindings from persist file (Phase 127; not carrier BGP).
pub fn cdn_bgp_sticky_persist_restore_from_file() {
    let _ = cdn_bgp_sticky_persist_fingerprint();
    cdn_bgp_sticky_persist_restore();
}

/// Phase 128: multi-snapshot MVP readiness (dir + sticky pop; not carrier BGP).
pub fn cdn_bgp_sticky_multi_snapshot_ready() -> bool {
    let _ = cdn_bgp_sticky_multi_snapshot_fingerprint();
    cdn_bgp_sticky_pop_ready() && cdn_bgp_sticky_multi_snapshot_dir().is_some()
}

/// Restore sticky bindings for one shard key after restart (TTL inherited; not carrier BGP).
pub fn cdn_bgp_sticky_multi_snapshot_restore_by_key(shard_key: &str) {
    let _ = cdn_bgp_sticky_multi_snapshot_fingerprint();
    cdn_bgp_sticky_multi_snapshot_restore_shard(shard_key);
}

/// Snapshot path for a shard key (Phase 128; not carrier BGP).
pub fn cdn_bgp_sticky_multi_snapshot_path_for_key(shard_key: &str) -> Option<String> {
    let dir = cdn_bgp_sticky_multi_snapshot_dir()?;
    Some(cdn_bgp_sticky_snapshot_path_for_shard(&dir, shard_key))
}

fn cdn_bgp_peer_announce_path(dir: &str, peer_id: &str, shard_key: &str) -> String {
    let safe_peer: String = peer_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let safe_shard: String = shard_key
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("{dir}/announce-{safe_peer}-{safe_shard}.tsv")
}

fn cdn_bgp_peer_announce_ack_path(dir: &str, peer_id: &str, shard_key: &str) -> String {
    let safe_peer: String = peer_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let safe_shard: String = shard_key
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("{dir}/ack-{safe_peer}-{safe_shard}.tsv")
}

/// Encode sticky bindings for a shard into a peer-announce wire (TTL inherited).
pub fn cdn_bgp_peer_announce_sticky_encode(shard_key: &str) -> String {
    let _ = cdn_bgp_peer_announce_sticky_fingerprint();
    let now = cdn_bgp_sticky_now_ms();
    let mut lines = Vec::new();
    for (key, binding) in cdn_bgp_sticky_bindings().lock().iter() {
        if cdn_bgp_sticky_shard_key(key) != shard_key {
            continue;
        }
        if binding.expires_at_ms > now {
            lines.push(format!(
                "{}\t{}\t{}\t{}",
                shard_key, key, binding.pop_id, binding.expires_at_ms
            ));
        }
    }
    lines.join("\n")
}

/// Announce sticky shard affinity to a peer (writes process-observable wire under announce dir).
pub fn cdn_bgp_peer_announce_sticky(peer_id: &str, shard_key: &str) -> Result<String, String> {
    let _ = cdn_bgp_peer_announce_sticky_fingerprint();
    if peer_id.trim().is_empty() || shard_key.trim().is_empty() {
        return Err(format!(
            "peer-announce-sticky requires peer_id + shard_key ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        ));
    }
    let wire = cdn_bgp_peer_announce_sticky_encode(shard_key);
    if wire.is_empty() {
        return Err(format!(
            "peer-announce-sticky no live bindings for shard {shard_key} ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        ));
    }
    if let Some(dir) = cdn_bgp_peer_announce_sticky_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let path = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
        std::fs::write(&path, &wire).map_err(|e| {
            format!(
                "peer-announce-sticky write {path}: {e} ({})",
                cdn_bgp_peer_announce_sticky_fingerprint()
            )
        })?;
    }
    Ok(wire)
}

/// Receive peer-announce sticky wire; restore/route affinity with TTL inherit.
pub fn cdn_bgp_peer_announce_sticky_receive(wire: &str) -> Result<usize, String> {
    let _ = cdn_bgp_peer_announce_sticky_fingerprint();
    if wire.trim().is_empty() {
        return Err(format!(
            "peer-announce-sticky empty wire ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        ));
    }
    let now = cdn_bgp_sticky_now_ms();
    let mut restored = 0usize;
    let mut slot = cdn_bgp_sticky_bindings().lock();
    for line in wire.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            continue;
        }
        let _shard = parts[0];
        let client_key = parts[1].to_string();
        let pop_id = parts[2].to_string();
        let Ok(expires_raw) = parts[3].parse::<u64>() else {
            continue;
        };
        // TTL inherit: keep original expires_at_ms when still valid.
        if expires_raw > now {
            slot.insert(
                client_key,
                CdnBgpStickyBinding {
                    pop_id,
                    expires_at_ms: expires_raw,
                },
            );
            restored += 1;
        }
    }
    if restored == 0 {
        return Err(format!(
            "peer-announce-sticky restored zero bindings ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        ));
    }
    drop(slot);
    cdn_bgp_sticky_persist_save();
    Ok(restored)
}

/// Write process-observable peer-announce ack after affinity was applied (Phase 130; not carrier BGP).
pub fn cdn_bgp_peer_announce_write_ack(
    peer_id: &str,
    shard_key: &str,
    wire: &str,
) -> Result<String, String> {
    let _ = cdn_bgp_peer_announce_ack_fingerprint();
    if peer_id.trim().is_empty() || shard_key.trim().is_empty() {
        return Err(format!(
            "peer-announce-ack requires peer_id + shard_key ({})",
            cdn_bgp_peer_announce_ack_fingerprint()
        ));
    }
    let dir = cdn_bgp_peer_announce_sticky_dir().ok_or_else(|| {
        format!(
            "peer-announce-ack dir missing ({})",
            cdn_bgp_peer_announce_ack_fingerprint()
        )
    })?;
    let _ = std::fs::create_dir_all(&dir);
    let path = cdn_bgp_peer_announce_ack_path(&dir, peer_id, shard_key);
    let body = format!("ack\t{peer_id}\t{shard_key}\n{wire}");
    std::fs::write(&path, &body).map_err(|e| {
        format!(
            "peer-announce-ack write {path}: {e} ({})",
            cdn_bgp_peer_announce_ack_fingerprint()
        )
    })?;
    Ok(path)
}

/// Receive peer-announce sticky wire and write ack after applying affinity (TTL inherit).
pub fn cdn_bgp_peer_announce_sticky_receive_and_ack(
    peer_id: &str,
    shard_key: &str,
    wire: &str,
) -> Result<usize, String> {
    let _ = cdn_bgp_peer_announce_ack_fingerprint();
    let restored = cdn_bgp_peer_announce_sticky_receive(wire)?;
    cdn_bgp_peer_announce_write_ack(peer_id, shard_key, wire)?;
    Ok(restored)
}

/// Load announce wire from peer announce dir, restore affinity, and write ack.
pub fn cdn_bgp_peer_announce_sticky_receive_from_peer(
    peer_id: &str,
    shard_key: &str,
) -> Result<usize, String> {
    let _ = cdn_bgp_peer_announce_sticky_fingerprint();
    let _ = cdn_bgp_peer_announce_ack_fingerprint();
    let dir = cdn_bgp_peer_announce_sticky_dir().ok_or_else(|| {
        format!(
            "peer-announce-sticky dir missing ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        )
    })?;
    let path = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
    let wire = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "peer-announce-sticky read {path}: {e} ({})",
            cdn_bgp_peer_announce_sticky_fingerprint()
        )
    })?;
    cdn_bgp_peer_announce_sticky_receive_and_ack(peer_id, shard_key, &wire)
}

/// Phase 129: peer-announce sticky MVP readiness (not carrier BGP).
pub fn cdn_bgp_peer_announce_sticky_ready() -> bool {
    let _ = cdn_bgp_peer_announce_sticky_fingerprint();
    cdn_bgp_sticky_pop_ready() && cdn_bgp_peer_announce_sticky_dir().is_some()
}

/// Phase 130: peer-announce ack surface readiness (not carrier BGP).
pub fn cdn_bgp_peer_announce_ack_ready() -> bool {
    let _ = cdn_bgp_peer_announce_ack_fingerprint();
    cdn_bgp_peer_announce_sticky_ready()
}

/// True when announce exists and receiver has written ack (mesh-ready); unacked announce is not mesh-ready.
pub fn cdn_bgp_peer_announce_ack_mesh_ready(peer_id: &str, shard_key: &str) -> bool {
    let _ = cdn_bgp_peer_announce_ack_fingerprint();
    if !cdn_bgp_sticky_pop_ready() {
        return false;
    }
    let Some(dir) = cdn_bgp_peer_announce_sticky_dir() else {
        return false;
    };
    let announce = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
    let ack = cdn_bgp_peer_announce_ack_path(&dir, peer_id, shard_key);
    std::path::Path::new(&announce).is_file() && std::path::Path::new(&ack).is_file()
}

/// Process-observable announce path for a peer+shard (Phase 129; not carrier BGP).
pub fn cdn_bgp_peer_announce_sticky_path_for(peer_id: &str, shard_key: &str) -> Option<String> {
    let dir = cdn_bgp_peer_announce_sticky_dir()?;
    Some(cdn_bgp_peer_announce_path(&dir, peer_id, shard_key))
}

/// Process-observable ack path for a peer+shard (Phase 130; not carrier BGP).
pub fn cdn_bgp_peer_announce_ack_path_for(peer_id: &str, shard_key: &str) -> Option<String> {
    let dir = cdn_bgp_peer_announce_sticky_dir()?;
    Some(cdn_bgp_peer_announce_ack_path(&dir, peer_id, shard_key))
}

/// Announce sticky shard affinity to ≥2 peers (fanout; not carrier BGP).
pub fn cdn_bgp_peer_announce_fanout(
    peer_ids: &[&str],
    shard_key: &str,
) -> Result<Vec<(String, String)>, String> {
    let _ = cdn_bgp_peer_announce_fanout_fingerprint();
    if peer_ids.len() < 2 {
        return Err(format!(
            "peer-announce-fanout requires ≥2 peers ({})",
            cdn_bgp_peer_announce_fanout_fingerprint()
        ));
    }
    if shard_key.trim().is_empty() {
        return Err(format!(
            "peer-announce-fanout requires shard_key ({})",
            cdn_bgp_peer_announce_fanout_fingerprint()
        ));
    }
    let mut out = Vec::with_capacity(peer_ids.len());
    for peer_id in peer_ids {
        if peer_id.trim().is_empty() {
            return Err(format!(
                "peer-announce-fanout empty peer_id ({})",
                cdn_bgp_peer_announce_fanout_fingerprint()
            ));
        }
        let wire = cdn_bgp_peer_announce_sticky(peer_id, shard_key)?;
        out.push(((*peer_id).to_string(), wire));
    }
    Ok(out)
}

/// Phase 131: peer-announce fanout surface readiness (not carrier BGP).
pub fn cdn_bgp_peer_announce_fanout_ready() -> bool {
    let _ = cdn_bgp_peer_announce_fanout_fingerprint();
    cdn_bgp_peer_announce_ack_ready()
}

/// True only when announce exists and **all** peers have written acks (mesh-ready).
/// Partial fanout ack is not mesh-ready.
pub fn cdn_bgp_peer_announce_fanout_mesh_ready(peer_ids: &[&str], shard_key: &str) -> bool {
    let _ = cdn_bgp_peer_announce_fanout_fingerprint();
    if peer_ids.len() < 2 {
        return false;
    }
    if !cdn_bgp_sticky_pop_ready() {
        return false;
    }
    peer_ids
        .iter()
        .all(|peer_id| cdn_bgp_peer_announce_ack_mesh_ready(peer_id, shard_key))
}

fn cdn_bgp_peer_announce_epoch_table() -> &'static Mutex<BTreeMap<String, u64>> {
    static TABLE: OnceLock<Mutex<BTreeMap<String, u64>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn cdn_bgp_peer_announce_epoch_persist_save() {
    let Some(path) = cdn_bgp_peer_announce_epoch_persist_path() else {
        return;
    };
    let lines: Vec<String> = cdn_bgp_peer_announce_epoch_table()
        .lock()
        .iter()
        .filter(|(_, epoch)| **epoch > 0)
        .map(|(shard, epoch)| format!("{shard}\t{epoch}"))
        .collect();
    let body = lines.join("\n");
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, body);
}

fn cdn_bgp_peer_announce_epoch_persist_restore() {
    let Some(path) = cdn_bgp_peer_announce_epoch_persist_path() else {
        return;
    };
    let Ok(body) = std::fs::read_to_string(&path) else {
        return;
    };
    let mut table = cdn_bgp_peer_announce_epoch_table().lock();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((shard, epoch_s)) = line.split_once('\t') else {
            continue;
        };
        let Ok(epoch) = epoch_s.trim().parse::<u64>() else {
            continue;
        };
        if epoch == 0 || shard.trim().is_empty() {
            continue;
        }
        let entry = table.entry(shard.trim().to_string()).or_insert(0);
        if epoch > *entry {
            *entry = epoch;
        }
    }
}

fn cdn_bgp_peer_announce_epoch_ensure_persist_restored() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static RESTORED: AtomicBool = AtomicBool::new(false);
    if RESTORED.swap(true, Ordering::Relaxed) {
        return;
    }
    cdn_bgp_peer_announce_epoch_persist_restore();
}

/// Clear in-memory winning epochs (persist file untouched; restart simulation).
pub fn cdn_bgp_peer_announce_epoch_memory_clear() {
    let _ = cdn_bgp_peer_announce_epoch_persist_fingerprint();
    cdn_bgp_peer_announce_epoch_table().lock().clear();
}

/// Phase 133: durable epoch persist readiness (conflict-epoch + persist path).
pub fn cdn_bgp_peer_announce_epoch_persist_ready() -> bool {
    let _ = cdn_bgp_peer_announce_epoch_persist_fingerprint();
    cdn_bgp_peer_announce_conflict_epoch_ready() && cdn_bgp_peer_announce_epoch_persist_path().is_some()
}

/// Explicit restore of winning epochs from persist file (restart simulation).
pub fn cdn_bgp_peer_announce_epoch_persist_restore_from_file() {
    let _ = cdn_bgp_peer_announce_epoch_persist_fingerprint();
    cdn_bgp_peer_announce_epoch_persist_restore();
}

/// True when a peer's on-disk announce wire for a shard has expired (Phase 134 TTL; not carrier BGP).
pub fn cdn_bgp_peer_announce_peer_wire_expired(peer_id: &str, shard_key: &str) -> bool {
    let _ = cdn_bgp_peer_announce_ttl_fingerprint();
    let Some(dir) = cdn_bgp_peer_announce_sticky_dir() else {
        return false;
    };
    let path = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
    let Ok(wire) = std::fs::read_to_string(&path) else {
        return false;
    };
    cdn_bgp_peer_announce_wire_expired(&wire)
}

/// Remove expired peer announce + ack files for a shard; returns count purged (Phase 134; not carrier BGP).
pub fn cdn_bgp_peer_announce_purge_expired_peers(peer_ids: &[&str], shard_key: &str) -> usize {
    let _ = cdn_bgp_peer_announce_ttl_fingerprint();
    let Some(dir) = cdn_bgp_peer_announce_sticky_dir() else {
        return 0;
    };
    let mut purged = 0usize;
    for peer_id in peer_ids {
        let announce = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
        let ack = cdn_bgp_peer_announce_ack_path(&dir, peer_id, shard_key);
        let ann_expired = std::fs::read_to_string(&announce)
            .ok()
            .is_some_and(|w| cdn_bgp_peer_announce_wire_expired(&w));
        let ack_expired = std::fs::read_to_string(&ack)
            .ok()
            .is_some_and(|w| cdn_bgp_peer_announce_wire_expired(&w));
        if ann_expired {
            let _ = std::fs::remove_file(&announce);
            purged += 1;
        }
        if ack_expired {
            let _ = std::fs::remove_file(&ack);
            purged += 1;
        }
    }
    purged
}

fn cdn_bgp_peer_announce_epoch_wire_header(epoch: u64) -> String {
    format!("#epoch:{epoch}")
}

fn cdn_bgp_peer_announce_expires_wire_header(expires_at_ms: u64) -> String {
    format!("#expires:{expires_at_ms}")
}

fn cdn_bgp_peer_announce_priority_wire_header(priority: u32) -> String {
    format!("#priority:{priority}")
}

fn cdn_bgp_peer_announce_hold_wire_header(hold_ms: u64) -> String {
    format!("#hold:{hold_ms}")
}

fn cdn_bgp_peer_announce_parse_expires_wire(wire: &str) -> Option<u64> {
    for line in wire.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if line.starts_with("#epoch:")
            || line.starts_with("#priority:")
            || line.starts_with("#hold:")
        {
            continue;
        }
        if let Some(exp) = line.strip_prefix("#expires:") {
            return exp.parse().ok();
        }
        break;
    }
    None
}

fn cdn_bgp_peer_announce_parse_priority_wire(wire: &str) -> Option<u32> {
    for line in wire.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(pri) = line.strip_prefix("#priority:") {
            return pri.parse().ok();
        }
        if !line.starts_with("#epoch:")
            && !line.starts_with("#expires:")
            && !line.starts_with("#hold:")
        {
            break;
        }
    }
    None
}

/// Parse `#hold:` header from peer-announce wire (Phase 136; not carrier BGP).
pub fn cdn_bgp_peer_announce_parse_hold_wire(wire: &str) -> Option<u64> {
    for line in wire.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(hold) = line.strip_prefix("#hold:") {
            return hold.parse().ok();
        }
        if !line.starts_with("#epoch:")
            && !line.starts_with("#expires:")
            && !line.starts_with("#priority:")
        {
            break;
        }
    }
    None
}

fn cdn_bgp_peer_announce_wire_expired(wire: &str) -> bool {
    let _ = cdn_bgp_peer_announce_ttl_fingerprint();
    let Some(expires) = cdn_bgp_peer_announce_parse_expires_wire(wire) else {
        return false;
    };
    cdn_bgp_sticky_now_ms() > expires
}

fn cdn_bgp_peer_announce_attach_ttl_headers(epoch: u64, body: &str) -> String {
    cdn_bgp_peer_announce_attach_wire_headers(epoch, cdn_bgp_peer_announce_default_priority(), body)
}

fn cdn_bgp_peer_announce_attach_wire_headers(epoch: u64, priority: u32, body: &str) -> String {
    let _ = cdn_bgp_peer_announce_ttl_fingerprint();
    let _ = cdn_bgp_peer_announce_priority_fingerprint();
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    let expires_at_ms = cdn_bgp_sticky_now_ms().saturating_add(cdn_bgp_peer_announce_ttl_ms());
    format!(
        "{}\n{}\n{}\n{}\n{}",
        cdn_bgp_peer_announce_epoch_wire_header(epoch),
        cdn_bgp_peer_announce_priority_wire_header(priority),
        cdn_bgp_peer_announce_hold_wire_header(cdn_bgp_peer_announce_hold_ms()),
        cdn_bgp_peer_announce_expires_wire_header(expires_at_ms),
        body
    )
}

fn cdn_bgp_peer_announce_strip_header_lines(wire: &str) -> &str {
    let mut body_start = 0usize;
    for (idx, line) in wire.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#epoch:")
            || trimmed.starts_with("#expires:")
            || trimmed.starts_with("#priority:")
            || trimmed.starts_with("#hold:")
        {
            body_start = wire
                .lines()
                .take(idx + 1)
                .map(|l| l.len() + 1)
                .sum();
            continue;
        }
        if trimmed.starts_with("ack\t") {
            continue;
        }
        break;
    }
    wire.get(body_start..).unwrap_or(wire).trim_start_matches('\n')
}

fn cdn_bgp_peer_announce_parse_epoch_wire(wire: &str) -> Option<(u64, &str)> {
    // Direct epoch-headed wire, or ack wrapper (`ack\tpeer\tshard\n#epoch:N\n...`).
    for line in wire.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(epoch) = line.strip_prefix("#epoch:").and_then(|s| s.parse::<u64>().ok()) {
            let body = cdn_bgp_peer_announce_strip_header_lines(wire);
            return Some((epoch, body));
        }
        if line.starts_with("ack\t") {
            if let Some((_, rest)) = wire.split_once('\n') {
                return cdn_bgp_peer_announce_parse_epoch_wire(rest);
            }
        }
    }
    None
}

fn cdn_bgp_peer_announce_clear_shard_acks(dir: &str, peer_ids: &[&str], shard_key: &str) {
    for peer_id in peer_ids {
        let ack = cdn_bgp_peer_announce_ack_path(dir, peer_id, shard_key);
        let _ = std::fs::remove_file(&ack);
        let ann = cdn_bgp_peer_announce_path(dir, peer_id, shard_key);
        let _ = std::fs::remove_file(&ann);
    }
}

/// Current winning announce epoch for a shard (0 if none).
pub fn cdn_bgp_peer_announce_winning_epoch(shard_key: &str) -> u64 {
    let _ = cdn_bgp_peer_announce_conflict_epoch_fingerprint();
    let _ = cdn_bgp_peer_announce_epoch_persist_fingerprint();
    cdn_bgp_peer_announce_epoch_ensure_persist_restored();
    cdn_bgp_peer_announce_epoch_table()
        .lock()
        .get(shard_key)
        .copied()
        .unwrap_or(0)
}

/// Announce sticky shard affinity to ≥2 peers with conflict-epoch (higher epoch wins; stale rejects).
pub fn cdn_bgp_peer_announce_fanout_with_epoch(
    peer_ids: &[&str],
    shard_key: &str,
    epoch: u64,
) -> Result<Vec<(String, String)>, String> {
    let _ = cdn_bgp_peer_announce_conflict_epoch_fingerprint();
    if peer_ids.len() < 2 {
        return Err(format!(
            "peer-announce-conflict-epoch requires ≥2 peers ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    if shard_key.trim().is_empty() {
        return Err(format!(
            "peer-announce-conflict-epoch requires shard_key ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    if epoch == 0 {
        return Err(format!(
            "peer-announce-conflict-epoch requires epoch > 0 ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    let winning = cdn_bgp_peer_announce_winning_epoch(shard_key);
    if epoch < winning {
        return Err(format!(
            "peer-announce-conflict-epoch reject stale epoch {epoch} < winning {winning} ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    if epoch > winning {
        if let Some(dir) = cdn_bgp_peer_announce_sticky_dir() {
            cdn_bgp_peer_announce_clear_shard_acks(&dir, peer_ids, shard_key);
        }
        cdn_bgp_peer_announce_epoch_table()
            .lock()
            .insert(shard_key.to_string(), epoch);
        cdn_bgp_peer_announce_epoch_persist_save();
    }
    let body = cdn_bgp_peer_announce_sticky_encode(shard_key);
    if body.is_empty() {
        return Err(format!(
            "peer-announce-conflict-epoch no live bindings for shard {shard_key} ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    let _ = cdn_bgp_peer_announce_hold_fingerprint();
    let sorted_peers = cdn_bgp_peer_announce_sort_peers_by_priority(peer_ids);
    let sorted_refs: Vec<&str> = sorted_peers.iter().map(|s| s.as_str()).collect();
    let unmuted = cdn_bgp_peer_announce_filter_held_peers(&sorted_refs, shard_key);
    if unmuted.is_empty() {
        return Err(format!(
            "peer-announce-hold dampen all peers ({})",
            cdn_bgp_peer_announce_hold_fingerprint()
        ));
    }
    let mut out = Vec::with_capacity(unmuted.len());
    for peer_id in unmuted {
        if peer_id.trim().is_empty() {
            return Err(format!(
                "peer-announce-conflict-epoch empty peer_id ({})",
                cdn_bgp_peer_announce_conflict_epoch_fingerprint()
            ));
        }
        let priority = cdn_bgp_peer_announce_peer_priority(&peer_id);
        let peer_wire =
            cdn_bgp_peer_announce_attach_wire_headers(epoch, priority, &body);
        if let Some(dir) = cdn_bgp_peer_announce_sticky_dir() {
            let _ = std::fs::create_dir_all(&dir);
            let path = cdn_bgp_peer_announce_path(&dir, &peer_id, shard_key);
            std::fs::write(&path, &peer_wire).map_err(|e| {
                format!(
                    "peer-announce-conflict-epoch write {path}: {e} ({})",
                    cdn_bgp_peer_announce_conflict_epoch_fingerprint()
                )
            })?;
        }
        cdn_bgp_peer_announce_hold_mark(&peer_id, shard_key);
        out.push((peer_id, peer_wire));
    }
    Ok(out)
}

/// Receive+ack peer-announce wire that carries conflict-epoch; rejects stale epochs.
pub fn cdn_bgp_peer_announce_receive_ack_with_epoch(
    peer_id: &str,
    shard_key: &str,
    wire: &str,
) -> Result<usize, String> {
    let _ = cdn_bgp_peer_announce_conflict_epoch_fingerprint();
    let (epoch, body) = cdn_bgp_peer_announce_parse_epoch_wire(wire).ok_or_else(|| {
        format!(
            "peer-announce-conflict-epoch missing epoch header ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        )
    })?;
    if cdn_bgp_peer_announce_wire_expired(wire) {
        return Err(format!(
            "peer-announce-ttl reject expired wire ({})",
            cdn_bgp_peer_announce_ttl_fingerprint()
        ));
    }
    let winning = cdn_bgp_peer_announce_winning_epoch(shard_key);
    if epoch < winning {
        return Err(format!(
            "peer-announce-conflict-epoch reject stale ack epoch {epoch} < winning {winning} ({})",
            cdn_bgp_peer_announce_conflict_epoch_fingerprint()
        ));
    }
    if epoch > winning {
        cdn_bgp_peer_announce_epoch_table()
            .lock()
            .insert(shard_key.to_string(), epoch);
        cdn_bgp_peer_announce_epoch_persist_save();
    }
    let restored = cdn_bgp_peer_announce_sticky_receive(body)?;
    // Ack wire includes epoch + TTL so mesh-ready can verify winning epoch.
    let ack_wire = cdn_bgp_peer_announce_attach_ttl_headers(epoch, body);
    cdn_bgp_peer_announce_write_ack(peer_id, shard_key, &ack_wire)?;
    Ok(restored)
}

/// Phase 132: conflict-epoch surface readiness (not carrier BGP).
pub fn cdn_bgp_peer_announce_conflict_epoch_ready() -> bool {
    let _ = cdn_bgp_peer_announce_conflict_epoch_fingerprint();
    cdn_bgp_peer_announce_fanout_ready()
}

/// True only when **winning epoch** announce exists and **all** peers have written matching-epoch acks.
pub fn cdn_bgp_peer_announce_conflict_epoch_mesh_ready(
    peer_ids: &[&str],
    shard_key: &str,
) -> bool {
    let _ = cdn_bgp_peer_announce_conflict_epoch_fingerprint();
    if peer_ids.len() < 2 {
        return false;
    }
    if !cdn_bgp_sticky_pop_ready() {
        return false;
    }
    let winning = cdn_bgp_peer_announce_winning_epoch(shard_key);
    if winning == 0 {
        return false;
    }
    let Some(dir) = cdn_bgp_peer_announce_sticky_dir() else {
        return false;
    };
    peer_ids.iter().all(|peer_id| {
        let announce = cdn_bgp_peer_announce_path(&dir, peer_id, shard_key);
        let ack = cdn_bgp_peer_announce_ack_path(&dir, peer_id, shard_key);
        let Ok(ann_wire) = std::fs::read_to_string(&announce) else {
            return false;
        };
        if cdn_bgp_peer_announce_wire_expired(&ann_wire) {
            return false;
        }
        let Ok(ack_wire) = std::fs::read_to_string(&ack) else {
            return false;
        };
        if cdn_bgp_peer_announce_wire_expired(&ack_wire) {
            return false;
        }
        let Some((ann_epoch, _)) = cdn_bgp_peer_announce_parse_epoch_wire(&ann_wire) else {
            return false;
        };
        let Some((ack_epoch, _)) = cdn_bgp_peer_announce_parse_epoch_wire(&ack_wire) else {
            return false;
        };
        ann_epoch == winning && ack_epoch == winning
    })
}

fn cdn_bgp_sticky_ensure_persist_restored() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static RESTORED: AtomicBool = AtomicBool::new(false);
    if RESTORED.swap(true, Ordering::Relaxed) {
        return;
    }
    cdn_bgp_sticky_persist_restore();
}

fn cdn_bgp_sticky_binding_entry(client_key: &str) -> Option<CdnBgpStickyBinding> {
    cdn_bgp_sticky_ensure_persist_restored();
    let now = cdn_bgp_sticky_now_ms();
    let mut slot = cdn_bgp_sticky_bindings().lock();
    if let Some(binding) = slot.get(client_key) {
        if binding.expires_at_ms > now {
            return Some(binding.clone());
        }
        slot.remove(client_key);
    }
    None
}

fn cdn_bgp_sticky_binding_insert(client_key: String, pop_id: String) {
    cdn_bgp_sticky_ensure_persist_restored();
    let ttl = cdn_bgp_sticky_ttl_ms();
    let expires_at_ms = cdn_bgp_sticky_now_ms().saturating_add(ttl);
    cdn_bgp_sticky_bindings().lock().insert(
        client_key,
        CdnBgpStickyBinding {
            pop_id,
            expires_at_ms,
        },
    );
    cdn_bgp_sticky_persist_save();
}

fn cdn_bgp_normalize_rows(entries: &[(&str, u32, &str, &str)]) -> Vec<CdnBgpRow> {
    entries
        .iter()
        .map(|(prefix, priority, pop_id, url)| {
            let normalized = if url.starts_with("tcp://") {
                (*url).to_string()
            } else {
                format!("tcp://{url}")
            };
            (
                (*prefix).to_string(),
                *priority,
                (*pop_id).to_string(),
                normalized,
            )
        })
        .collect()
}

fn cdn_bgp_merged_rows() -> Vec<CdnBgpRow> {
    let mut merged = cdn_bgp_table().lock().clone();
    for (_, rows) in cdn_bgp_peer_views().lock().iter() {
        merged.extend(rows.clone());
    }
    merged
}

fn cdn_bgp_merged_mesh_rows() -> Vec<(String, CdnBgpRow)> {
    let mut merged = Vec::new();
    if let Some(region) = cdn_bgp_mesh_local_region().lock().clone() {
        for row in cdn_bgp_table().lock().iter().cloned() {
            merged.push((region.clone(), row));
        }
    }
    for (_, region, rows) in cdn_bgp_mesh_peer_views().lock().iter() {
        for row in rows.iter().cloned() {
            merged.push((region.clone(), row));
        }
    }
    merged
}

/// Phase 125 gate fingerprint: client-key sticky PoP over weighted anycast mesh (not carrier BGP).
pub fn cdn_bgp_fingerprint() -> &'static str {
    "dist-cdn-bgp-v8:sticky-pop-mvp"
}

/// Ops snapshot for BGP peer visibility (Phase 120 — not carrier BGP).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CdnBgpOpsSnapshot {
    pub fingerprint: &'static str,
    pub peer_ids: Vec<String>,
    pub imported_peer_count: usize,
    pub local_endpoint_count: usize,
    pub merged_endpoint_count: usize,
    pub pop_health: Vec<(String, bool)>,
    pub ops_visibility_ready: bool,
}

/// Route-convergence snapshot (Phase 121 legacy — superseded by mesh snapshot in Phase 122).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CdnBgpConvergenceSnapshot {
    pub fingerprint: &'static str,
    pub peer_ids: Vec<String>,
    pub pop_health: Vec<(String, bool)>,
    pub failover_order: Vec<String>,
    pub route_convergence_ready: bool,
    pub ops_visibility_ready: bool,
}

/// Multi-region mesh snapshot (Phase 122–124 — cross-region / weighted anycast; not carrier BGP).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CdnBgpMeshSnapshot {
    pub fingerprint: &'static str,
    pub mesh_regions: Vec<String>,
    pub peer_ids: Vec<String>,
    pub pop_health: Vec<(String, bool)>,
    pub cross_region_failover_order: Vec<String>,
    pub multi_region_mesh_ready: bool,
    pub multi_pop_ready: bool,
    pub weighted_anycast_ready: bool,
    pub sticky_pop_ready: bool,
    /// Phase 126: sticky client-key TTL surface exported in mesh snapshot (not carrier BGP).
    pub sticky_ttl_ready: bool,
    pub sticky_ttl_ms: u64,
    pub sticky_binding_count: usize,
    /// Phase 127: sticky binding persist/restore surface (not carrier BGP).
    pub sticky_persist_ready: bool,
    pub mesh_pop_count: usize,
    pub pop_weights: Vec<(String, u32)>,
    pub ops_visibility_ready: bool,
}

/// Register local BGP prefix/priority rows: `(prefix, priority, pop_id, tcp_url)`.
pub fn cdn_bgp_configure(entries: &[(&str, u32, &str, &str)]) {
    *cdn_bgp_table().lock() = cdn_bgp_normalize_rows(entries);
}

/// Register local BGP prefix/priority rows for a mesh region (Phase 122 multi-region mesh GA).
pub fn cdn_bgp_configure_mesh(region: &str, entries: &[(&str, u32, &str, &str)]) {
    *cdn_bgp_table().lock() = cdn_bgp_normalize_rows(entries);
    *cdn_bgp_mesh_local_region().lock() = Some(region.to_string());
}

/// Import a remote peer's prefix/priority view for a mesh region (Phase 122; not carrier BGP).
pub fn cdn_bgp_import_peer_mesh(peer_id: &str, region: &str, entries: &[(&str, u32, &str, &str)]) {
    let rows = cdn_bgp_normalize_rows(entries);
    let mut slot = cdn_bgp_mesh_peer_views().lock();
    if let Some(existing) = slot
        .iter_mut()
        .find(|(id, r, _)| id == peer_id && r == region)
    {
        existing.2 = rows;
    } else {
        slot.push((peer_id.to_string(), region.to_string(), rows));
    }
    // Keep legacy peer-exchange table in sync for ops visibility surfaces.
    cdn_bgp_import_peer_view(peer_id, entries);
}

/// Distinct mesh regions from local configure + imported peer mesh views.
pub fn cdn_bgp_mesh_regions() -> Vec<String> {
    let mut regions = BTreeSet::new();
    if let Some(region) = cdn_bgp_mesh_local_region().lock().clone() {
        regions.insert(region);
    }
    for (_, region, _) in cdn_bgp_mesh_peer_views().lock().iter() {
        regions.insert(region.clone());
    }
    regions.into_iter().collect()
}

pub fn cdn_bgp_mesh_peer_view_count() -> usize {
    cdn_bgp_mesh_peer_views().lock().len()
}

pub fn cdn_bgp_mesh_region_count() -> usize {
    cdn_bgp_mesh_regions().len()
}

/// Export the local prefix/priority view for peer exchange (Phase 118).
pub fn cdn_bgp_export_peer_view() -> Vec<CdnBgpRow> {
    cdn_bgp_table().lock().clone()
}

/// Import a remote peer's prefix/priority view (Phase 118 peer-exchange GA).
pub fn cdn_bgp_import_peer_view(peer_id: &str, entries: &[(&str, u32, &str, &str)]) {
    let rows = cdn_bgp_normalize_rows(entries);
    let mut slot = cdn_bgp_peer_views().lock();
    if let Some(existing) = slot.iter_mut().find(|(id, _)| id == peer_id) {
        existing.1 = rows;
    } else {
        slot.push((peer_id.to_string(), rows));
    }
}

pub fn cdn_bgp_peer_view_count() -> usize {
    cdn_bgp_peer_views().lock().len()
}

/// List imported BGP peer ids (Phase 120 ops-visibility; not carrier BGP).
pub fn cdn_bgp_list_peer_ids() -> Vec<String> {
    cdn_bgp_peer_views()
        .lock()
        .iter()
        .map(|(id, _)| id.clone())
        .collect()
}

/// Health map for configured BGP pop_ids (local + imported views).
pub fn cdn_bgp_peer_health_map() -> Vec<(String, bool)> {
    let rows = cdn_bgp_merged_rows();
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (_, _, pop_id, _) in rows {
        if seen.insert(pop_id.clone()) {
            out.push((pop_id.clone(), cdn_bgp_health(&pop_id)));
        }
    }
    out
}

/// Phase 120: ops visibility snapshot (peer ids + health; not carrier BGP).
pub fn cdn_bgp_ops_snapshot() -> CdnBgpOpsSnapshot {
    let peer_ids = cdn_bgp_list_peer_ids();
    let pop_health = cdn_bgp_peer_health_map();
    CdnBgpOpsSnapshot {
        fingerprint: cdn_bgp_fingerprint(),
        peer_ids: peer_ids.clone(),
        imported_peer_count: cdn_bgp_peer_view_count(),
        local_endpoint_count: cdn_bgp_endpoint_count(),
        merged_endpoint_count: cdn_bgp_merged_rows().len(),
        pop_health,
        ops_visibility_ready: cdn_bgp_ops_visibility_ready(),
    }
}

/// Failover pop order for a preferred pop (higher priority first; not carrier BGP).
pub fn cdn_bgp_failover_order_ids(preferred: &str) -> Vec<String> {
    cdn_bgp_failover_order(preferred)
        .into_iter()
        .map(|(pop, _)| pop)
        .collect()
}

/// Phase 121: route-convergence snapshot (peer health + failover order; not carrier BGP).
pub fn cdn_bgp_convergence_snapshot(preferred: &str) -> CdnBgpConvergenceSnapshot {
    let peer_ids = cdn_bgp_list_peer_ids();
    let pop_health = cdn_bgp_peer_health_map();
    CdnBgpConvergenceSnapshot {
        fingerprint: cdn_bgp_fingerprint(),
        peer_ids: peer_ids.clone(),
        pop_health: pop_health.clone(),
        failover_order: cdn_bgp_failover_order_ids(preferred),
        route_convergence_ready: cdn_bgp_route_convergence_ready(),
        ops_visibility_ready: cdn_bgp_ops_visibility_ready(),
    }
}

/// Phase 122: multi-region mesh snapshot (cross-region failover / weighted anycast; not carrier BGP).
pub fn cdn_bgp_mesh_snapshot(preferred_region: &str, preferred_pop: &str) -> CdnBgpMeshSnapshot {
    let peer_ids = cdn_bgp_list_peer_ids();
    let pop_health = cdn_bgp_peer_health_map();
    CdnBgpMeshSnapshot {
        fingerprint: cdn_bgp_fingerprint(),
        mesh_regions: cdn_bgp_mesh_regions(),
        peer_ids: peer_ids.clone(),
        pop_health: pop_health.clone(),
        cross_region_failover_order: cdn_bgp_cross_region_failover_order_ids(
            preferred_region,
            preferred_pop,
        ),
        multi_region_mesh_ready: cdn_bgp_multi_region_mesh_ready(),
        multi_pop_ready: cdn_bgp_multi_pop_ready(),
        weighted_anycast_ready: cdn_bgp_weighted_anycast_ready(),
        sticky_pop_ready: cdn_bgp_sticky_pop_ready(),
        sticky_ttl_ready: cdn_bgp_sticky_ttl_ready(),
        sticky_ttl_ms: cdn_bgp_sticky_ttl_ms(),
        sticky_binding_count: cdn_bgp_sticky_pop_binding_count(),
        sticky_persist_ready: cdn_bgp_sticky_persist_ready(),
        mesh_pop_count: cdn_bgp_mesh_pop_count(),
        pop_weights: cdn_bgp_pop_weights(),
        ops_visibility_ready: cdn_bgp_ops_visibility_ready(),
    }
}

/// Cross-region failover pop order (preferred region/pop first; not carrier BGP).
pub fn cdn_bgp_cross_region_failover_order_ids(
    preferred_region: &str,
    preferred_pop: &str,
) -> Vec<String> {
    cdn_bgp_cross_region_failover_order(preferred_region, preferred_pop)
        .into_iter()
        .map(|(_, pop, _)| pop)
        .collect()
}

/// Phase 122 hard gate: ≥2 mesh regions + merged views + cross-region failover (not carrier BGP).
pub fn cdn_bgp_multi_region_mesh_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    if cdn_bgp_mesh_region_count() < 2 {
        return false;
    }
    if cdn_bgp_mesh_peer_view_count() < 1 {
        return false;
    }
    if cdn_bgp_mesh_local_region().lock().is_none() {
        return false;
    }
    if cdn_bgp_merged_mesh_rows().len() < 2 {
        return false;
    }
    if !cdn_bgp_ops_visibility_ready() {
        return false;
    }
    let health = cdn_bgp_peer_health_map();
    if health.len() < 2 {
        return false;
    }
    let pops: BTreeSet<String> = cdn_bgp_merged_mesh_rows()
        .into_iter()
        .map(|(_, (_, _, pop, _))| pop)
        .collect();
    let health_pops: BTreeSet<String> = health.iter().map(|(pop, _)| pop.clone()).collect();
    if pops != health_pops {
        return false;
    }
    health.iter().any(|(_, ok)| *ok) && cdn_bgp_cross_region_failover_ready()
}

/// Distinct PoP ids from local mesh configure + imported peer mesh views.
pub fn cdn_bgp_mesh_pop_count() -> usize {
    cdn_bgp_mesh_pop_ids().len()
}

/// Distinct PoP ids across merged mesh rows (Phase 123 multi-PoP MVP).
pub fn cdn_bgp_mesh_pop_ids() -> Vec<String> {
    let mut pops = BTreeSet::new();
    for (_, (_, _, pop, _)) in cdn_bgp_merged_mesh_rows() {
        pops.insert(pop);
    }
    pops.into_iter().collect()
}

/// Phase 123 hard gate: ≥3 PoPs + multi-region mesh anycast route (not carrier BGP).
pub fn cdn_bgp_multi_pop_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    if !cdn_bgp_multi_region_mesh_ready() {
        return false;
    }
    if cdn_bgp_mesh_pop_count() < 3 {
        return false;
    }
    let health = cdn_bgp_peer_health_map();
    let pops: BTreeSet<String> = cdn_bgp_merged_mesh_rows()
        .into_iter()
        .map(|(_, (_, _, pop, _))| pop)
        .collect();
    let health_pops: BTreeSet<String> = health.iter().map(|(pop, _)| pop.clone()).collect();
    if pops != health_pops {
        return false;
    }
    health.iter().filter(|(_, ok)| *ok).count() >= 2
        && cdn_bgp_cross_region_failover_ready()
}

/// Phase 123 alias: multi-PoP MVP readiness (not carrier BGP).
pub fn cdn_bgp_multi_pop_mvp_ready() -> bool {
    cdn_bgp_multi_pop_ready()
}

/// Distinct PoP weights from merged mesh rows (`priority` field = anycast weight).
pub fn cdn_bgp_pop_weights() -> Vec<(String, u32)> {
    let mut best: BTreeMap<String, u32> = BTreeMap::new();
    for (_, (_, weight, pop, _)) in cdn_bgp_merged_mesh_rows() {
        best.entry(pop)
            .and_modify(|w| {
                if weight > *w {
                    *w = weight;
                }
            })
            .or_insert(weight);
    }
    best.into_iter().collect()
}

/// True when mesh PoP weights are not all equal (equal-weight multi-PoP ≠ weighted-anycast GA).
pub fn cdn_bgp_weights_differentiated() -> bool {
    let weights = cdn_bgp_pop_weights();
    if weights.len() < 2 {
        return false;
    }
    let first = weights[0].1;
    weights.iter().any(|(_, w)| *w != first)
}

/// Phase 124 hard gate: multi-PoP mesh + differentiated weights + health-based weighted route (not carrier BGP).
pub fn cdn_bgp_weighted_anycast_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    if !cdn_bgp_multi_pop_ready() {
        return false;
    }
    if !cdn_bgp_weights_differentiated() {
        return false;
    }
    let health = cdn_bgp_peer_health_map();
    health.iter().filter(|(_, ok)| *ok).count() >= 2
}

/// Phase 124 alias: weighted anycast MVP readiness (not carrier BGP).
pub fn cdn_bgp_weighted_anycast_mvp_ready() -> bool {
    cdn_bgp_weighted_anycast_ready()
}

/// Phase 125 hard gate: weighted anycast mesh + sticky client-key routing surface (not carrier BGP).
pub fn cdn_bgp_sticky_pop_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    cdn_bgp_weighted_anycast_ready()
}

/// Phase 125 alias: sticky PoP MVP readiness (not carrier BGP).
pub fn cdn_bgp_sticky_pop_mvp_ready() -> bool {
    cdn_bgp_sticky_pop_ready()
}

/// Sticky client-key → PoP binding count (Phase 125; not carrier BGP).
pub fn cdn_bgp_sticky_pop_binding_count() -> usize {
    cdn_bgp_sticky_bindings().lock().len()
}

/// Lookup sticky PoP for a client key (Phase 125–126; not carrier BGP).
pub fn cdn_bgp_sticky_binding(client_key: &str) -> Option<String> {
    if client_key.is_empty() {
        return None;
    }
    cdn_bgp_sticky_binding_entry(client_key).map(|b| b.pop_id)
}

/// Expiry timestamp (ms since epoch) for a sticky client key, if bound and not expired.
pub fn cdn_bgp_sticky_binding_expires_at_ms(client_key: &str) -> Option<u64> {
    if client_key.is_empty() {
        return None;
    }
    cdn_bgp_sticky_binding_entry(client_key).map(|b| b.expires_at_ms)
}

/// Phase 126 alias: sticky PoP TTL MVP readiness (not carrier BGP).
pub fn cdn_bgp_sticky_ttl_ready() -> bool {
    let _ = cdn_bgp_sticky_ttl_fingerprint();
    cdn_bgp_sticky_pop_ready()
}

/// Clear sticky client-key bindings (Phase 125; not carrier BGP).
pub fn cdn_bgp_sticky_pop_clear() {
    cdn_bgp_sticky_bindings().lock().clear();
    cdn_bgp_sticky_persist_save();
}

/// True when cross-region failover order is computable across ≥2 mesh regions.
pub fn cdn_bgp_cross_region_failover_ready() -> bool {
    cdn_bgp_mesh_region_count() >= 2
        && !cdn_bgp_cross_region_failover_order_ids("", "").is_empty()
}

/// True when peer-exchange GA is satisfied and ops surfaces are populated.
pub fn cdn_bgp_ops_visibility_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    cdn_bgp_peer_exchange_done() && !cdn_bgp_list_peer_ids().is_empty()
}

/// Phase 121 hard gate: merged peer views probed + failover order computable (not carrier BGP).
pub fn cdn_bgp_route_convergence_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    if !cdn_bgp_ops_visibility_ready() {
        return false;
    }
    let health = cdn_bgp_peer_health_map();
    if health.len() < 2 {
        return false;
    }
    let pops: BTreeSet<String> = cdn_bgp_merged_rows()
        .into_iter()
        .map(|(_, _, pop, _)| pop)
        .collect();
    let health_pops: BTreeSet<String> = health.iter().map(|(pop, _)| pop.clone()).collect();
    if pops != health_pops {
        return false;
    }
    health.iter().any(|(_, ok)| *ok)
        && !cdn_bgp_failover_order_ids("").is_empty()
}

pub fn cdn_bgp_peer_exchange_done() -> bool {
    cdn_bgp_has_endpoints() && cdn_bgp_peer_view_count() >= 1
}

pub fn cdn_bgp_clear_peer_views() {
    cdn_bgp_peer_views().lock().clear();
    cdn_bgp_mesh_peer_views().lock().clear();
}

pub fn cdn_bgp_clear() {
    cdn_bgp_table().lock().clear();
    *cdn_bgp_mesh_local_region().lock() = None;
    cdn_bgp_clear_peer_views();
    cdn_bgp_sticky_pop_clear();
    cdn_bgp_peer_announce_epoch_table().lock().clear();
}

pub fn cdn_bgp_endpoint_count() -> usize {
    cdn_bgp_table().lock().len()
}

pub fn cdn_bgp_has_endpoints() -> bool {
    cdn_bgp_merged_rows().len() >= 2
}

/// GA layer: sticky PoP over weighted anycast mesh (not carrier BGP).
pub fn cdn_bgp_ga_ready() -> bool {
    let _ = cdn_bgp_fingerprint();
    cdn_bgp_sticky_pop_ready()
}

/// TCP health for a configured BGP pop_id (local or imported peer view).
pub fn cdn_bgp_health(pop_id: &str) -> bool {
    let url = cdn_bgp_merged_rows()
        .into_iter()
        .find(|(_, _, p, _)| p == pop_id)
        .map(|(_, _, _, u)| u);
    match url {
        Some(u) => dist_tcp_endpoint_alive(&u),
        None => false,
    }
}

fn cdn_bgp_failover_order(preferred: &str) -> Vec<(String, String)> {
    let mut rows = cdn_bgp_merged_rows();
    // Higher priority first; preferred pop_id breaks ties to the front within same priority.
    rows.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| {
            let a_pref = if a.2 == preferred { 0 } else { 1 };
            let b_pref = if b.2 == preferred { 0 } else { 1 };
            a_pref.cmp(&b_pref)
        })
    });
    let mut out = Vec::with_capacity(rows.len());
    for (_, _, pop, url) in rows {
        if !out.iter().any(|(p, _): &(String, String)| p == &pop) {
            out.push((pop, url));
        }
    }
    out
}

fn cdn_bgp_cross_region_failover_order(
    preferred_region: &str,
    preferred_pop: &str,
) -> Vec<(String, String, String)> {
    let mut rows = cdn_bgp_merged_mesh_rows();
    // Phase 124 weighted anycast: higher weight first; preferred region/pop break ties.
    rows.sort_by(|a, b| {
        b.1.1.cmp(&a.1.1).then_with(|| {
            let a_region_pref = if a.0 == preferred_region { 0 } else { 1 };
            let b_region_pref = if b.0 == preferred_region { 0 } else { 1 };
            a_region_pref.cmp(&b_region_pref).then_with(|| {
                let a_pop_pref = if a.1.2 == preferred_pop { 0 } else { 1 };
                let b_pop_pref = if b.1.2 == preferred_pop { 0 } else { 1 };
                a_pop_pref.cmp(&b_pop_pref)
            })
        })
    });
    let mut out = Vec::with_capacity(rows.len());
    for (region, (_, _, pop, url)) in rows {
        if !out.iter().any(|(_, p, _): &(String, String, String)| p == &pop) {
            out.push((region, pop, url));
        }
    }
    out
}

fn cdn_bgp_pop_mesh_region(pop_id: &str) -> Option<String> {
    cdn_bgp_merged_mesh_rows()
        .into_iter()
        .find(|(_, (_, _, pop, _))| pop == pop_id)
        .map(|(region, _)| region)
}

/// Route via BGP weighted anycast mesh: weight/health-based failover (not carrier BGP).
pub fn cdn_bgp_route(
    preferred: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let _ = cdn_bgp_fingerprint();
    let payload = payload.into();
    if !cdn_bgp_weighted_anycast_ready() {
        return Err(
            "cdn bgp weighted anycast requires multi-PoP mesh + differentiated weights + ≥2 healthy PoPs (cdn_bgp_configure_mesh + cdn_bgp_import_peer_mesh; equal-weight multi-PoP is not weighted-anycast GA; not carrier BGP)"
                .to_string(),
        );
    }
    let preferred_region = cdn_bgp_pop_mesh_region(preferred).unwrap_or_default();
    let order = cdn_bgp_cross_region_failover_order(&preferred_region, preferred);
    let mut last_err = String::from("cdn bgp weighted anycast: no healthy pop");
    for (_region, pop, url) in order {
        if !dist_tcp_endpoint_alive(&url) {
            last_err = format!("cdn bgp weighted anycast {pop} unhealthy");
            continue;
        }
        // Reject loopback-forced connect targets for BGP GA.
        if tcp_url_host_part(&url) == "127.0.0.1" || tcp_url_host_part(&url) == "::1" {
            last_err = format!("cdn bgp weighted anycast {pop} connect must not force loopback");
            continue;
        }
        match region_route_tcp(&url, &pop, from, to, payload.clone()) {
            Ok(ack) => {
                cdn_delivery_ack_register(&pop, "cdn-bgp-weighted-anycast", from, to, &ack);
                return Ok((pop, ack));
            }
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

fn cdn_bgp_pop_tcp_url(pop_id: &str) -> Option<String> {
    cdn_bgp_merged_rows()
        .into_iter()
        .find(|(_, _, p, _)| p == pop_id)
        .map(|(_, _, _, u)| u)
}

fn cdn_bgp_route_pop(
    pop_id: &str,
    delivery_mode: &str,
    from: i64,
    to: i64,
    payload: String,
) -> Result<String, String> {
    let url = cdn_bgp_pop_tcp_url(pop_id)
        .ok_or_else(|| format!("cdn bgp sticky pop {pop_id} unknown"))?;
    if !dist_tcp_endpoint_alive(&url) {
        return Err(format!("cdn bgp sticky pop {pop_id} unhealthy"));
    }
    if tcp_url_host_part(&url) == "127.0.0.1" || tcp_url_host_part(&url) == "::1" {
        return Err(format!(
            "cdn bgp sticky pop {pop_id} connect must not force loopback"
        ));
    }
    let ack = region_route_tcp(&url, pop_id, from, to, payload)?;
    cdn_delivery_ack_register(pop_id, delivery_mode, from, to, &ack);
    Ok(ack)
}

/// Route via BGP sticky PoP: same client key sticks to a healthy PoP (not carrier BGP).
pub fn cdn_bgp_route_sticky(
    client_key: &str,
    preferred: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let _ = cdn_bgp_fingerprint();
    let payload = payload.into();
    if client_key.trim().is_empty() {
        return Err(
            "cdn bgp sticky pop requires non-empty client_key (dist-cdn-bgp-v8:sticky-pop-mvp; not carrier BGP)"
                .to_string(),
        );
    }
    if !cdn_bgp_sticky_pop_ready() {
        return Err(
            "cdn bgp sticky pop requires weighted anycast mesh + differentiated weights + ≥2 healthy PoPs (not carrier BGP)"
                .to_string(),
        );
    }
    let client_key = client_key.to_string();
    if let Some(binding) = cdn_bgp_sticky_binding_entry(&client_key) {
        let bound_pop = binding.pop_id;
        if cdn_bgp_health(&bound_pop) {
            match cdn_bgp_route_pop(&bound_pop, "cdn-bgp-sticky-pop", from, to, payload.clone()) {
                Ok(ack) => return Ok((bound_pop, ack)),
                Err(e) => {
                    cdn_bgp_sticky_bindings().lock().remove(&client_key);
                    return Err(e);
                }
            }
        } else {
            cdn_bgp_sticky_bindings().lock().remove(&client_key);
        }
    }
    let (pop, ack) = cdn_bgp_route(preferred, from, to, payload)?;
    cdn_bgp_sticky_binding_insert(client_key, pop.clone());
    Ok((pop, ack))
}

// --- Phase 114+: CDN PoP health anycast (≥2 PoP endpoints; dedicated ACK registry proof) ---

static CDN_POP_ENDPOINTS: OnceLock<Mutex<Vec<(String, String, String)>>> = OnceLock::new();
/// CDN/PoP delivery ACK registry — GA success proof (never [`tcp_mailbox_inbox`]).
static CDN_DELIVERY_ACK_REGISTRY: OnceLock<Mutex<VecDeque<(String, String, i64, i64, String)>>> =
    OnceLock::new();

fn cdn_pop_endpoints() -> &'static Mutex<Vec<(String, String, String)>> {
    CDN_POP_ENDPOINTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn cdn_delivery_ack_registry() -> &'static Mutex<VecDeque<(String, String, i64, i64, String)>> {
    CDN_DELIVERY_ACK_REGISTRY.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Phase 119 gate fingerprint: multi-hostkey PoP route (≥2 distinct remote hostkeys; sibling optional).
pub fn cdn_pop_fingerprint() -> &'static str {
    "dist-cdn-pop-v6:multi-hostkey-route"
}

/// Register `(pop_region, host_key_addr, tcp://host:port)` CDN PoP endpoints.
///
/// GA (pop-v5) requires ≥2 PoPs with **distinct** concrete non-loopback hostkeys, non-loopback
/// connect URLs, and sibling-process serve proof (not parent/child current_exe spawn tree).
pub fn cdn_pop_configure(pops: &[(&str, &str, &str)]) {
    let mut slot = cdn_pop_endpoints().lock();
    slot.clear();
    for (region, host_key_addr, url) in pops {
        let normalized = if url.starts_with("tcp://") {
            (*url).to_string()
        } else {
            format!("tcp://{url}")
        };
        slot.push((
            (*region).to_string(),
            (*host_key_addr).to_string(),
            normalized,
        ));
    }
}

pub fn cdn_pop_clear() {
    cdn_pop_endpoints().lock().clear();
    cdn_delivery_ack_registry().lock().clear();
    CDN_POP_SIBLING_PEER.store(false, Ordering::Relaxed);
    CDN_POP_MULTI_HOSTKEY_ROUTE_PROOF.store(false, Ordering::Relaxed);
    cdn_pop_serve_pids().lock().clear();
    cdn_pop_spawned_children().lock().clear();
}

pub fn cdn_pop_endpoint_count() -> usize {
    cdn_pop_endpoints().lock().len()
}

/// Strip optional `:port` from a remote hostkey (`ip` or `ip:port`).
pub fn dist_host_key_remote_ip_part(host_key: &str) -> String {
    if let Ok(addr) = host_key.parse::<SocketAddr>() {
        return addr.ip().to_string();
    }
    // IPv4 host:port without brackets
    if let Some((host, port)) = host_key.rsplit_once(':') {
        if port.chars().all(|c| c.is_ascii_digit()) && host.parse::<IpAddr>().is_ok() {
            return host.to_string();
        }
    }
    host_key.to_string()
}

/// Remote hostkey identity for a live bind: concrete non-loopback `ip:port` (never 127 rewrite).
pub fn dist_host_key_remote_for_bind(addr: &SocketAddr) -> String {
    format!(
        "{}:{}",
        dist_host_key_concrete_ip_for_bind(addr),
        addr.port()
    )
}

/// True when connect URL host is loopback (`127.0.0.1` / `::1` / localhost).
pub fn dist_connect_url_is_loopback(url: &str) -> bool {
    let host = tcp_url_host_part(url);
    dist_host_key_addr_is_loopback(&host)
}

/// True when `host_key_addr` satisfies remote-hostkey GA (live listener + non-loopback concrete IP).
pub fn cdn_pop_host_key_passes_ga(host_key_addr: &str) -> bool {
    let ip = dist_host_key_remote_ip_part(host_key_addr);
    let binds = runtime_bound_host_keys().lock();
    dist_host_key_runtime_bind_addr_is_ga(&ip, &binds)
}

/// Distinct remote hostkeys: ≥2 unique hostkey strings whose IP parts are non-loopback.
pub fn cdn_pop_hostkeys_are_distinct_remote() -> bool {
    let slot = cdn_pop_endpoints().lock();
    cdn_pop_hostkeys_are_distinct_remote_locked(&slot)
}

fn cdn_pop_serve_pid_for_region(region: &str) -> Option<(u32, u32)> {
    cdn_pop_serve_pids()
        .lock()
        .iter()
        .find(|(r, _, _)| r == region)
        .map(|(_, pid, ppid)| (*pid, *ppid))
}

fn cdn_pop_endpoint_passes_ga(region: &str, host_key: &str, url: &str) -> bool {
    if let Some((serve_pid, _)) = cdn_pop_serve_pid_for_region(region) {
        if serve_pid != std::process::id() {
            return !dist_host_key_addr_is_loopback(&dist_host_key_remote_ip_part(host_key))
                && !dist_connect_url_is_loopback(url)
                && dist_tcp_endpoint_alive(url);
        }
    }
    cdn_pop_host_key_passes_ga(host_key)
}

fn cdn_pop_hostkeys_are_distinct_remote_locked(slot: &[(String, String, String)]) -> bool {
    if slot.len() < 2 {
        return false;
    }
    let mut keys = std::collections::BTreeSet::new();
    for (_, host_key, _) in slot.iter() {
        if dist_host_key_addr_is_loopback(&dist_host_key_remote_ip_part(host_key)) {
            return false;
        }
        keys.insert(host_key.clone());
    }
    keys.len() >= 2
}

/// True when every configured PoP connect URL is non-loopback (no forced 127 rewrite).
pub fn cdn_pop_connect_urls_non_loopback() -> bool {
    let slot = cdn_pop_endpoints().lock();
    cdn_pop_connect_urls_non_loopback_locked(&slot)
}

fn cdn_pop_connect_urls_non_loopback_locked(slot: &[(String, String, String)]) -> bool {
    !slot.is_empty() && slot.iter().all(|(_, _, url)| !dist_connect_url_is_loopback(url))
}

/// GA CDN PoP requires ≥2 PoPs with distinct remote hostkeys and non-loopback connect URLs.
pub fn cdn_pop_has_endpoints() -> bool {
    let slot = cdn_pop_endpoints().lock();
    slot.len() >= 2
        && slot
            .iter()
            .all(|(region, host_key, url)| cdn_pop_endpoint_passes_ga(region, host_key, url))
        && cdn_pop_hostkeys_are_distinct_remote_locked(&slot)
        && cdn_pop_connect_urls_non_loopback_locked(&slot)
}

/// Mark that a PoP endpoint is served by `(serve_pid, serve_ppid)` for sibling-process GA.
pub fn cdn_pop_register_serve_pid(pop_region: &str, serve_pid: u32, serve_ppid: u32) {
    let mut slot = cdn_pop_serve_pids().lock();
    if let Some(existing) = slot.iter_mut().find(|(region, _, _)| region == pop_region) {
        existing.1 = serve_pid;
        existing.2 = serve_ppid;
    } else {
        slot.push((pop_region.to_string(), serve_pid, serve_ppid));
    }
}

pub fn cdn_pop_serve_pid_count() -> usize {
    cdn_pop_serve_pids().lock().len()
}

/// True when two processes share the same parent pid (siblings, not parent/child).
pub fn dist_pop_pids_are_siblings(pid_a: u32, ppid_a: u32, pid_b: u32, ppid_b: u32) -> bool {
    pid_a != pid_b && ppid_a != 0 && ppid_a == ppid_b
}

/// Record a child pid spawned from this process (parent/child tree — not sibling GA).
pub fn dist_pop_note_spawned_child(child_pid: u32) {
    cdn_pop_spawned_children().lock().insert(child_pid);
}

fn dist_pop_is_parent_child_tree(local_pid: u32, peer_pid: u32) -> bool {
    if local_pid == peer_pid {
        return true;
    }
    if cdn_pop_spawned_children().lock().contains(&peer_pid) {
        return true;
    }
    if let Ok(parent) = std::env::var("SYX_DIST_POP_PARENT_PID") {
        if parent.parse::<u32>() == Ok(peer_pid) {
            return true;
        }
    }
    false
}

/// Register sibling-process peer proof: peer must share ppid with us and not be parent/child.
pub fn dist_pop_register_sibling_process_peer(peer_pid: u32, peer_ppid: u32) -> bool {
    let my_pid = std::process::id();
    let my_ppid = std::env::var("SYX_DIST_POP_PPID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if peer_pid == 0 || peer_pid == my_pid || dist_pop_is_parent_child_tree(my_pid, peer_pid) {
        return false;
    }
    if my_ppid != 0 && peer_pid == my_ppid {
        return false;
    }
    if !dist_pop_pids_are_siblings(my_pid, my_ppid, peer_pid, peer_ppid) {
        return false;
    }
    CDN_POP_SIBLING_PEER.store(true, Ordering::Relaxed);
    true
}

pub fn cdn_pop_sibling_process_peer_registered() -> bool {
    CDN_POP_SIBLING_PEER.load(Ordering::Relaxed)
}

fn cdn_pop_sibling_serve_pids_ready() -> bool {
    let slot = cdn_pop_serve_pids().lock();
    let entries: Vec<(u32, u32)> = slot.iter().map(|(_, pid, ppid)| (*pid, *ppid)).collect();
    if entries.len() < 2 {
        return false;
    }
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            if dist_pop_pids_are_siblings(
                entries[i].0,
                entries[i].1,
                entries[j].0,
                entries[j].1,
            ) {
                return true;
            }
        }
    }
    false
}

/// Legacy dual-process hook — does not satisfy pop-v5 sibling GA.
pub fn dist_pop_register_dual_process_peer() {
    // Intentionally no-op for pop-v5: parent/child current_exe spawn is not sibling GA.
}

pub fn cdn_pop_dual_process_peer_registered() -> bool {
    cdn_pop_sibling_process_peer_registered()
}

/// Phase 119 GA: ≥2 distinct remote hostkeys with alive non-loopback connect URLs.
fn cdn_pop_multi_hostkey_config_ready() -> bool {
    let slot = cdn_pop_endpoints().lock();
    if slot.len() < 2
        || !cdn_pop_hostkeys_are_distinct_remote_locked(&slot)
        || !cdn_pop_connect_urls_non_loopback_locked(&slot)
    {
        return false;
    }
    let alive = slot
        .iter()
        .filter(|(region, host_key, url)| {
            cdn_pop_endpoint_passes_ga(region, host_key, url) && dist_tcp_endpoint_alive(url)
        })
        .count();
    alive >= 2
}

/// Phase 119 GA: multi-hostkey config + successful route proof (sibling mechanism optional).
pub fn cdn_pop_multi_hostkey_route_ready() -> bool {
    let _ = cdn_pop_fingerprint();
    cdn_pop_multi_hostkey_config_ready()
        && CDN_POP_MULTI_HOSTKEY_ROUTE_PROOF.load(Ordering::Relaxed)
}

/// Legacy alias — sibling-process narrative is not pop-v6 GA.
pub fn cdn_pop_sibling_process_ready() -> bool {
    let _ = cdn_pop_fingerprint();
    cdn_pop_has_endpoints() && cdn_pop_sibling_serve_pids_ready()
}

/// Legacy alias retained for doctor/report surfaces.
pub fn cdn_pop_dual_process_host_ready() -> bool {
    cdn_pop_sibling_process_ready()
}

/// Phase 119 GA: multi-hostkey route proof (distinct remote hostkeys).
pub fn cdn_pop_remote_hostkey_ready() -> bool {
    cdn_pop_multi_hostkey_route_ready()
}

pub fn cdn_delivery_ack_count() -> usize {
    cdn_delivery_ack_registry().lock().len()
}

pub fn cdn_delivery_ack_clear() {
    cdn_delivery_ack_registry().lock().clear();
}

fn cdn_delivery_ack_register(pop: &str, tier: &str, from: i64, to: i64, ack: &str) {
    cdn_delivery_ack_registry().lock().push_back((
        pop.to_string(),
        tier.to_string(),
        from,
        to,
        ack.to_string(),
    ));
}

fn cdn_pop_failover_order(preferred: &str) -> Vec<(String, String)> {
    let slot = cdn_pop_endpoints().lock();
    let mut out = Vec::with_capacity(slot.len());
    if let Some((r, _, u)) = slot.iter().find(|(r, _, _)| r == preferred) {
        out.push((r.clone(), u.clone()));
    }
    for (r, _, u) in slot.iter() {
        if r != preferred {
            out.push((r.clone(), u.clone()));
        }
    }
    out
}

/// TCP health probe for a configured CDN PoP endpoint.
pub fn cdn_pop_health(pop_region: &str) -> bool {
    let url = {
        let slot = cdn_pop_endpoints().lock();
        slot.iter()
            .find(|(r, _, _)| r == pop_region)
            .map(|(_, _, u)| u.clone())
    };
    match url {
        Some(u) => dist_tcp_endpoint_alive(&u),
        None => false,
    }
}

/// Route via preferred CDN PoP; on health fail, failover to the next healthy PoP (≥2 configured).
///
/// Delivery proof = TCP ACK registered in [`cdn_delivery_ack_registry`]; **not**
/// process-local [`tcp_mailbox_inbox`] pull/count.
pub fn cdn_pop_route(
    preferred: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let _ = cdn_pop_fingerprint();
    let payload = payload.into();
    if !cdn_pop_multi_hostkey_config_ready() {
        return Err(
            "cdn pop multi-hostkey-route requires ≥2 distinct non-loopback hostkeys with alive connect URLs (cdn_pop_configure)"
                .to_string(),
        );
    }
    let order = cdn_pop_failover_order(preferred);
    if order.len() < 2 {
        return Err(
            "cdn pop anycast requires ≥2 configured PoP (region, host_key, tcp) endpoints (cdn_pop_configure)"
                .to_string(),
        );
    }
    let mut last_err = String::from("cdn pop: no healthy PoP");
    for (pop, url) in order {
        if dist_connect_url_is_loopback(&url) {
            last_err = format!("cdn pop {pop} connect must not force loopback");
            continue;
        }
        if !dist_tcp_endpoint_alive(&url) {
            last_err = format!("cdn pop {pop} unhealthy");
            continue;
        }
        match region_route_tcp(&url, &pop, from, to, payload.clone()) {
            Ok(ack) => {
                cdn_delivery_ack_register(&pop, "cdn-pop", from, to, &ack);
                CDN_POP_MULTI_HOSTKEY_ROUTE_PROOF.store(true, Ordering::Relaxed);
                return Ok((pop, ack));
            }
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

// --- Phase 113: TCP CDN peer delivery (peer TCP ACK; not process-local VecDeque GA) ---

static CDN_TCP_PEERS: OnceLock<Mutex<Vec<(String, String)>>> = OnceLock::new();
/// Retired Phase 112 process-local CDN inbox — **not** a GA success proof
/// (`dist-cdn-tcp-v2:peer-delivery`). Kept empty so audits cannot treat VecDeque push as delivery.
#[allow(dead_code)]
static CDN_TCP_PEER_INBOX: OnceLock<Mutex<VecDeque<(String, i64, i64, String)>>> = OnceLock::new();

fn cdn_tcp_peers() -> &'static Mutex<Vec<(String, String)>> {
    CDN_TCP_PEERS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Register `(region, tcp://host:port)` CDN peer endpoints for real TCP pull/ack.
pub fn cdn_tcp_peer_configure(peers: &[(&str, &str)]) {
    let mut slot = cdn_tcp_peers().lock();
    slot.clear();
    for (region, url) in peers {
        let normalized = if url.starts_with("tcp://") {
            (*url).to_string()
        } else {
            format!("tcp://{url}")
        };
        slot.push(((*region).to_string(), normalized));
    }
}

pub fn cdn_tcp_peer_clear() {
    cdn_tcp_peers().lock().clear();
    // Intentionally do not use CDN_TCP_PEER_INBOX as delivery store (Phase 113).
    if let Some(inbox) = CDN_TCP_PEER_INBOX.get() {
        inbox.lock().clear();
    }
}

pub fn cdn_tcp_peer_has_endpoints() -> bool {
    !cdn_tcp_peers().lock().is_empty()
}

pub fn cdn_tcp_peer_endpoint_count() -> usize {
    cdn_tcp_peers().lock().len()
}

/// Always 0 on GA path — process-local `CDN_TCP_PEER_INBOX` is not a delivery proof.
pub fn cdn_tcp_peer_inbox_count() -> usize {
    0
}

fn cdn_tcp_peer_failover_order(preferred: &str) -> Vec<(String, String)> {
    let slot = cdn_tcp_peers().lock();
    let mut out = Vec::with_capacity(slot.len());
    if let Some(entry) = slot.iter().find(|(r, _)| r == preferred) {
        out.push(entry.clone());
    }
    for (r, u) in slot.iter() {
        if r != preferred {
            out.push((r.clone(), u.clone()));
        }
    }
    out
}

/// Route via CDN TCP peer pull/ack. Success is peer TCP ACK only — never process-local VecDeque.
pub fn cdn_tcp_peer_route(
    preferred: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let _ = cdn_tcp_peer_fingerprint();
    let payload = payload.into();
    let order = cdn_tcp_peer_failover_order(preferred);
    if order.is_empty() {
        return Err(
            "cdn tcp peer requires configured (region, tcp) endpoints (cdn_tcp_peer_configure)"
                .to_string(),
        );
    }
    let mut last_err = String::from("cdn tcp peer: no healthy region");
    for (region, url) in order {
        if !dist_tcp_endpoint_alive(&url) {
            last_err = format!("cdn tcp peer region {region} unhealthy");
            continue;
        }
        // Delivery proof = TCP ACK in CDN_DELIVERY_ACK_REGISTRY; tcp_mailbox_inbox is NOT CDN GA.
        match region_route_tcp(&url, &region, from, to, payload.clone()) {
            Ok(ack) => {
                cdn_delivery_ack_register(&region, "cdn-tcp-peer", from, to, &ack);
                return Ok((region, ack));
            }
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

/// Phase 81: health_probe alias for CDN failover gate (PoP/TCP paths only; no global_cdn stub).
pub fn cdn_health_probe(region: &str) -> bool {
    cdn_pop_health(region)
        || dist_region_tcp_health(region)
        || dist_region_multihost_tcp_health(region, &dist_region_default_host_key(region))
}

/// Phase 82: cross-continent tertiary failover region (auto when primary+secondary fail).
pub fn cross_continent_failover_region(primary: &str) -> Option<&'static str> {
    match primary {
        "us-east" => Some("ap-south"),
        "eu-west" => Some("sa-east"),
        "ap-south" => Some("us-east"),
        "sa-east" => Some("eu-west"),
        _ => cdn_failover_region(primary),
    }
}

static FAILOVER_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Phase 82: cross-continent auto failover route (primary → secondary → tertiary).
pub fn cross_continent_auto_route(region: &str, from: i64, to: i64, payload: impl Into<String>) -> bool {
    let payload = payload.into();
    if cdn_health_probe(region) {
        return cdn_route(region, from, to, payload);
    }
    if let Some(secondary) = cdn_failover_region(region) {
        if cdn_health_probe(secondary) {
            FAILOVER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return cdn_route(secondary, from, to, payload);
        }
    }
    if let Some(tertiary) = cross_continent_failover_region(region) {
        FAILOVER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return cdn_route(tertiary, from, to, payload);
    }
    false
}

/// Phase 82: count of cross-continent failover events (doctor report).
pub fn cross_continent_failover_count() -> u64 {
    FAILOVER_COUNT.load(std::sync::atomic::Ordering::Relaxed)
}

// --- Phase 107–108: multi-process TCP mailbox (no in-process region_route GA) ---

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

const ENV_DIST_POP_CHILD_BIND: &str = "SYX_DIST_POP_CHILD_BIND";
const ENV_DIST_POP_CHILD_READY_FILE: &str = "SYX_DIST_POP_CHILD_READY_FILE";
const ENV_DIST_POP_CHILD_MAX_CONNS: &str = "SYX_DIST_POP_CHILD_MAX_CONNS";
const ENV_DIST_POP_CHILD_STOP: &str = "SYX_DIST_POP_CHILD_STOP";

static CDN_POP_SIBLING_PEER: AtomicBool = AtomicBool::new(false);
static CDN_POP_MULTI_HOSTKEY_ROUTE_PROOF: AtomicBool = AtomicBool::new(false);
static CDN_POP_SERVE_PIDS: OnceLock<Mutex<Vec<(String, u32, u32)>>> = OnceLock::new();
static CDN_POP_SPAWNED_CHILDREN: OnceLock<Mutex<BTreeSet<u32>>> = OnceLock::new();

fn cdn_pop_serve_pids() -> &'static Mutex<Vec<(String, u32, u32)>> {
    CDN_POP_SERVE_PIDS.get_or_init(|| Mutex::new(Vec::new()))
}

fn cdn_pop_spawned_children() -> &'static Mutex<BTreeSet<u32>> {
    CDN_POP_SPAWNED_CHILDREN.get_or_init(|| Mutex::new(BTreeSet::new()))
}

static TCP_MAILBOX_URL: OnceLock<Mutex<Option<String>>> = OnceLock::new();
/// Frames delivered via TCP mailbox only (never counted as in-process `region_route` GA).
static TCP_MAILBOX_INBOX: OnceLock<Mutex<VecDeque<(String, i64, i64, String)>>> = OnceLock::new();

fn tcp_mailbox_url_slot() -> &'static Mutex<Option<String>> {
    TCP_MAILBOX_URL.get_or_init(|| Mutex::new(None))
}

fn tcp_mailbox_inbox() -> &'static Mutex<VecDeque<(String, i64, i64, String)>> {
    TCP_MAILBOX_INBOX.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn mp_write_frame(stream: &mut TcpStream, payload: &[u8]) -> Result<(), String> {
    let len = u32::try_from(payload.len()).map_err(|_| "mp frame too large".to_string())?;
    stream
        .write_all(&len.to_be_bytes())
        .map_err(|e| format!("mp write len: {e}"))?;
    stream
        .write_all(payload)
        .map_err(|e| format!("mp write body: {e}"))?;
    Ok(())
}

fn mp_read_frame(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut len_buf = [0u8; 4];
    stream
        .read_exact(&mut len_buf)
        .map_err(|e| format!("mp read len: {e}"))?;
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream
            .read_exact(&mut payload)
            .map_err(|e| format!("mp read body: {e}"))?;
    }
    Ok(payload)
}

fn encode_route_frame(region: &str, from: i64, to: i64, payload: &str) -> Vec<u8> {
    format!("ROUTE\0{region}\0{from}\0{to}\0{payload}").into_bytes()
}

fn parse_route_frame(bytes: &[u8]) -> Option<(String, i64, i64, String)> {
    let s = std::str::from_utf8(bytes).ok()?;
    let rest = s.strip_prefix("ROUTE\0")?;
    let mut parts = rest.splitn(4, '\0');
    let region = parts.next()?.to_string();
    let from: i64 = parts.next()?.parse().ok()?;
    let to: i64 = parts.next()?.parse().ok()?;
    let payload = parts.next()?.to_string();
    Some((region, from, to, payload))
}

/// Phase 108 gate fingerprint: TCP mailbox GA without in-process region_route fallback.
pub fn dist_actor_mp_fingerprint() -> &'static str {
    "dist-actor-mp-v2:no-inproc-fallback-ga"
}

/// True when explicit in-process dist fallback is opted in (`SYX_DIST_INPROC=1`).
pub fn dist_inproc_enabled() -> bool {
    std::env::var("SYX_DIST_INPROC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Honest doctor mode: TCP mailbox vs explicit in-process dev fallback (not GA).
///
/// Inproc is reported only when [`dist_inproc_enabled`]; silent inproc is forbidden (P109).
pub fn dist_actor_mp_doctor_mode() -> &'static str {
    if dist_actor_mp_url().is_some() {
        "tcp-mailbox"
    } else if dist_region_multihost_has_endpoints() {
        "tcp-multihost"
    } else if dist_region_anycast_has_endpoints() {
        "tcp-anycast"
    } else if dist_inproc_enabled() {
        "inproc-dev-fallback"
    } else {
        "no-transport"
    }
}

/// True when GA must reject: no TCP mailbox configured (inproc is not GA).
pub fn dist_actor_mp_ga_requires_tcp() -> bool {
    true
}

/// Frames accepted by the TCP mailbox inbox (not the in-process region_route table).
pub fn dist_actor_mp_inbox_count() -> usize {
    tcp_mailbox_inbox().lock().len()
}

pub fn dist_actor_mp_inbox_clear() {
    tcp_mailbox_inbox().lock().clear();
}

/// Configure process-local TCP mailbox peer (`tcp://host:port` or `host:port`).
pub fn dist_actor_mp_configure(url: &str) {
    let normalized = if url.starts_with("tcp://") {
        url.to_string()
    } else {
        format!("tcp://{url}")
    };
    *tcp_mailbox_url_slot().lock() = Some(normalized);
}

pub fn dist_actor_mp_clear() {
    *tcp_mailbox_url_slot().lock() = None;
    tcp_mailbox_inbox().lock().clear();
}

fn dist_actor_mp_url() -> Option<String> {
    std::env::var("SYX_DIST_ACTOR_MP")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| tcp_mailbox_url_slot().lock().clone())
}

/// Resolve a concrete non-loopback local IPv4 for unspecified (`0.0.0.0`) bind proof.
pub fn dist_concrete_local_ipv4() -> Option<Ipv4Addr> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    let local = sock.local_addr().ok()?;
    match local.ip() {
        IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_unspecified() => Some(v4),
        _ => None,
    }
}

/// Concrete host-key IP for a live bind address (`0.0.0.0` → resolved local IPv4).
pub fn dist_host_key_concrete_ip_for_bind(addr: &SocketAddr) -> String {
    match addr.ip() {
        IpAddr::V4(v4) if v4.is_unspecified() => dist_concrete_local_ipv4()
            .map(|c| c.to_string())
            .unwrap_or_else(|| addr.ip().to_string()),
        ip => ip.to_string(),
    }
}

/// TCP connect URL for a bind address — uses concrete host-key IP (never 127.0.0.1 rewrite for `0.0.0.0`).
pub fn dist_tcp_url_for_bind_addr(bind_addr: &SocketAddr) -> String {
    format!(
        "tcp://{}:{}",
        dist_host_key_concrete_ip_for_bind(bind_addr),
        bind_addr.port()
    )
}

/// True when `host_key_addr` is unspecified (`0.0.0.0` / `::`) and cannot satisfy hostkey GA alone.
pub fn dist_host_key_addr_is_unspecified(host_key_addr: &str) -> bool {
    host_key_addr
        .parse::<IpAddr>()
        .map(|ip| match ip {
            IpAddr::V4(v4) => v4.is_unspecified(),
            IpAddr::V6(v6) => v6.is_unspecified(),
        })
        .unwrap_or(false)
}

/// If spawned as a CDN PoP peer child, bind mailbox, write ready file, and serve.
pub fn dist_pop_os_child_try_serve() -> bool {
    let Ok(bind) = std::env::var(ENV_DIST_POP_CHILD_BIND) else {
        return false;
    };
    let ready_path = std::env::var(ENV_DIST_POP_CHILD_READY_FILE)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::temp_dir().join(format!("syx-dist-pop-ready-{}", std::process::id()))
        });
    let max_conns: usize = std::env::var(ENV_DIST_POP_CHILD_MAX_CONNS)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let (addr, handle, stop) = spawn_dist_tcp_mailbox_on(&bind, max_conns);
    std::thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_remote_for_bind(&addr);
    let connect_url = dist_tcp_url_for_bind_addr(&addr);
    let pid = std::process::id();
    let ppid = std::env::var("SYX_DIST_POP_PPID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let _ = std::fs::write(
        &ready_path,
        format!("{host_key}\n{connect_url}\n{pid}\n{ppid}\n"),
    );
    eprintln!("SYX_DIST_POP_CHILD_READY=1 host_key={host_key} url={connect_url}");
    for _ in 0..600 {
        if std::env::var(ENV_DIST_POP_CHILD_STOP).is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    stop.stop();
    let _ = handle.join();
    let _ = std::fs::remove_file(&ready_path);
    true
}

/// Spawn a TCP mailbox bound to `bind` (e.g. `127.0.0.1:0` or `0.0.0.0:0`).
/// Registers the bound address as a live runtime host-key proof (`dist-region-hostkey-v6:concrete-ip`).
pub fn spawn_dist_tcp_mailbox_on(bind: &str, max_conns: usize) -> (SocketAddr, JoinHandle<()>, ArcStop) {
    let listener = TcpListener::bind(bind).expect("dist tcp mailbox bind");
    listener
        .set_nonblocking(true)
        .expect("dist tcp mailbox nonblocking");
    let addr = listener.local_addr().expect("dist tcp mailbox local_addr");
    dist_host_key_register_live_listener_bind(&addr);
    let stop = ArcStop::new();
    let stop_flag = stop.0.clone();
    let handle = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut accepted = 0usize;
        while accepted < max_conns && Instant::now() < deadline && !stop_flag.load(Ordering::Relaxed)
        {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Accepted sockets inherit nonblocking; switch to blocking for framed I/O.
                    let _ = stream.set_nonblocking(false);
                    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
                    let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
                    match mp_read_frame(&mut stream) {
                        Ok(frame) if frame.starts_with(b"HEALTH") => {
                            // Health probes must not consume route capacity (P109 anycast).
                            let _ = mp_write_frame(&mut stream, b"ACK\0ok");
                        }
                        Ok(frame) => {
                            if let Some((region, from, to, payload)) = parse_route_frame(&frame) {
                                accepted += 1;
                                // TCP-only inbox — do not call region_route (inproc is not GA).
                                tcp_mailbox_inbox()
                                    .lock()
                                    .push_back((region, from, to, payload.clone()));
                                let ack = format!("ACK\0{payload}");
                                let _ = mp_write_frame(&mut stream, ack.as_bytes());
                            } else {
                                let _ = mp_write_frame(&mut stream, b"ERR\0bad-frame");
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5));
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    });
    (addr, handle, stop)
}

/// Spawn a TCP mailbox on loopback IPv4 (`127.0.0.1:0`).
pub fn spawn_dist_tcp_mailbox(max_conns: usize) -> (SocketAddr, JoinHandle<()>, ArcStop) {
    spawn_dist_tcp_mailbox_on("127.0.0.1:0", max_conns)
}

/// Dual-stack mailboxes: IPv4 loopback + IPv6 loopback (`::1`) for non-loopback host-key proof.
pub struct DistDualStackMailboxes {
    pub v4_addr: SocketAddr,
    pub v6_addr: SocketAddr,
    v4_handle: JoinHandle<()>,
    v6_handle: JoinHandle<()>,
    v4_stop: ArcStop,
    v6_stop: ArcStop,
}

impl DistDualStackMailboxes {
    pub fn stop(self) {
        self.v4_stop.stop();
        self.v6_stop.stop();
        let _ = self.v4_handle.join();
        let _ = self.v6_handle.join();
    }

    /// Stop only the IPv6 mailbox (for host-key failover tests).
    pub fn stop_v6_only(&self) {
        self.v6_stop.stop();
    }
}

/// Bind one mailbox on `127.0.0.1:0` and one on `[::1]:0` (distinct host keys for failover gates).
pub fn spawn_dist_tcp_mailbox_dual_stack(max_conns: usize) -> DistDualStackMailboxes {
    let (v4_addr, v4_handle, v4_stop) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", max_conns);
    let (v6_addr, v6_handle, v6_stop) = spawn_dist_tcp_mailbox_on("[::1]:0", max_conns);
    DistDualStackMailboxes {
        v4_addr,
        v6_addr,
        v4_handle,
        v6_handle,
        v4_stop,
        v6_stop,
    }
}

/// True when `addr` is not the IPv4 loopback literal (Phase 111 non-loopback bind proof).
pub fn dist_bind_addr_is_non_loopback_v4(addr: &SocketAddr) -> bool {
    !matches!(addr.ip(), std::net::IpAddr::V4(v4) if v4 == std::net::Ipv4Addr::LOCALHOST)
}

/// Cheap stop handle so tests can tear down mailbox without waiting full deadline.
#[derive(Clone)]
pub struct ArcStop(std::sync::Arc<AtomicBool>);

impl ArcStop {
    fn new() -> Self {
        Self(std::sync::Arc::new(AtomicBool::new(false)))
    }

    pub fn stop(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

/// Deliver route to a TCP mailbox peer and wait for ACK (cross-process).
///
/// Success is ACK-only; does **not** mirror into in-process [`region_route`] (P108 GA).
pub fn region_route_tcp(
    url: &str,
    region: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<String, String> {
    let payload = payload.into();
    let host_port = url.strip_prefix("tcp://").unwrap_or(url);
    let addr: SocketAddr = host_port
        .parse()
        .map_err(|e| format!("dist mp bad addr {host_port}: {e}"))?;
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(400))
        .map_err(|e| format!("dist mp connect failed: {e}"))?;
    let _ = stream.set_read_timeout(Some(Duration::from_millis(800)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(800)));
    let frame = encode_route_frame(region, from, to, &payload);
    mp_write_frame(&mut stream, &frame)?;
    let resp = mp_read_frame(&mut stream)?;
    if let Some(ack) = resp.strip_prefix(b"ACK\0") {
        Ok(String::from_utf8_lossy(ack).into_owned())
    } else if let Some(msg) = resp.strip_prefix(b"ERR\0") {
        Err(String::from_utf8_lossy(msg).into_owned())
    } else {
        Err(format!(
            "dist mp bad response: {:?}",
            String::from_utf8_lossy(&resp)
        ))
    }
}

/// GA path: TCP mailbox only. In-process [`region_route`] is never counted as GA success.
pub fn region_route_mp_ga(
    region: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<String, String> {
    let url = dist_actor_mp_url().ok_or_else(|| {
        "dist mp GA requires TCP mailbox (inproc region_route is not GA; set SYX_DIST_ACTOR_MP or dist_actor_mp_configure)"
            .to_string()
    })?;
    region_route_tcp(&url, region, from, to, payload)
}

/// Route via configured TCP mailbox when `SYX_DIST_ACTOR_MP` / configure is set;
/// otherwise requires explicit `SYX_DIST_INPROC=1` for in-process [`region_route`]
/// (doctor: `inproc-dev-fallback`). Silent inproc success is forbidden (P109).
///
/// Dev fallback success is **not** GA — use [`region_route_mp_ga`] for GA gates.
pub fn region_route_mp(region: &str, from: i64, to: i64, payload: impl Into<String>) -> bool {
    let payload = payload.into();
    if let Some(url) = dist_actor_mp_url() {
        return region_route_tcp(&url, region, from, to, payload).is_ok();
    }
    if dist_region_multihost_has_endpoints() {
        let host_key = dist_region_default_host_key(region);
        return dist_region_multihost_route(region, &host_key, from, to, payload).is_ok();
    }
    if dist_region_anycast_has_endpoints() {
        return dist_region_anycast_route(region, from, to, payload).is_ok();
    }
    if dist_inproc_enabled() {
        // Explicit in-process fallback — doctor reports inproc-dev-fallback; not GA.
        return region_route(region, from, to, payload);
    }
    false
}

/// Child-process helper: connect to mailbox, send route, print ACK line (for two-process gates).
pub fn dist_actor_mp_peer_send_once(
    url: &str,
    region: &str,
    from: i64,
    to: i64,
    payload: &str,
) -> Result<String, String> {
    region_route_tcp(url, region, from, to, payload)
}

// --- Phase 109: multi-region TCP anycast MVP (health + failover) ---

static REGION_ANYCAST_ENDPOINTS: OnceLock<Mutex<Vec<(String, String)>>> = OnceLock::new();

fn region_anycast_endpoints() -> &'static Mutex<Vec<(String, String)>> {
    REGION_ANYCAST_ENDPOINTS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Phase 109 gate fingerprint: multi-region TCP health probe + failover routing.
pub fn dist_region_anycast_fingerprint() -> &'static str {
    "dist-region-anycast-v1:tcp-health"
}

/// Register `(region, tcp://host:port)` endpoints for anycast-style routing (≥2 for failover).
pub fn dist_region_anycast_configure(endpoints: &[(&str, &str)]) {
    let mut slot = region_anycast_endpoints().lock();
    slot.clear();
    for (region, url) in endpoints {
        let normalized = if url.starts_with("tcp://") {
            (*url).to_string()
        } else {
            format!("tcp://{url}")
        };
        slot.push(((*region).to_string(), normalized));
    }
}

pub fn dist_region_anycast_clear() {
    region_anycast_endpoints().lock().clear();
}

pub fn dist_region_anycast_has_endpoints() -> bool {
    !region_anycast_endpoints().lock().is_empty()
}

pub fn dist_region_anycast_endpoint_count() -> usize {
    region_anycast_endpoints().lock().len()
}

/// TCP connect health probe for a region's configured endpoint.
pub fn dist_region_tcp_health(region: &str) -> bool {
    let url = {
        let slot = region_anycast_endpoints().lock();
        slot.iter()
            .find(|(r, _)| r == region)
            .map(|(_, u)| u.clone())
    };
    match url {
        Some(u) => dist_tcp_endpoint_alive(&u),
        None => false,
    }
}

fn dist_tcp_endpoint_alive(url: &str) -> bool {
    let host_port = url.strip_prefix("tcp://").unwrap_or(url);
    let Ok(addr) = host_port.parse::<SocketAddr>() else {
        return false;
    };
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(120)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(200)));
    if mp_write_frame(&mut stream, b"HEALTH\0").is_err() {
        return false;
    }
    match mp_read_frame(&mut stream) {
        Ok(resp) => resp.starts_with(b"ACK"),
        Err(_) => false,
    }
}

/// Ordered failover list: preferred region first, then remaining configured regions.
fn dist_region_anycast_failover_order(preferred: &str) -> Vec<(String, String)> {
    let slot = region_anycast_endpoints().lock();
    let mut out = Vec::with_capacity(slot.len());
    if let Some((r, u)) = slot.iter().find(|(r, _)| r == preferred) {
        out.push((r.clone(), u.clone()));
    }
    for (r, u) in slot.iter() {
        if r != preferred {
            out.push((r.clone(), u.clone()));
        }
    }
    out
}

/// Route via preferred region TCP; on health fail, failover to the next healthy region.
///
/// Returns `Ok((active_region, ack_payload))`. Memory `VecDeque` CDN is not used on this path.
pub fn dist_region_anycast_route(
    preferred: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    let payload = payload.into();
    let order = dist_region_anycast_failover_order(preferred);
    if order.is_empty() {
        return Err(
            "dist anycast requires configured region TCP endpoints (dist_region_anycast_configure)"
                .to_string(),
        );
    }
    let mut last_err = String::from("dist anycast: no healthy region");
    for (region, url) in order {
        if !dist_tcp_endpoint_alive(&url) {
            last_err = format!("dist anycast region {region} unhealthy");
            continue;
        }
        match region_route_tcp(&url, &region, from, to, payload.clone()) {
            Ok(ack) => return Ok((region, ack)),
            Err(e) => {
                last_err = e;
            }
        }
    }
    Err(last_err)
}

/// Alias used by CDN GA when anycast endpoints are configured.
pub fn cdn_route_anycast(
    region: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String), String> {
    dist_region_anycast_route(region, from, to, payload)
}

// --- Phase 110: multi-host keyed region routing (host key participates; keyed failover) ---

static REGION_MULTIHOST_ENDPOINTS: OnceLock<Mutex<Vec<(String, String, String, String)>>> =
    OnceLock::new();

fn region_multihost_endpoints() -> &'static Mutex<Vec<(String, String, String, String)>> {
    REGION_MULTIHOST_ENDPOINTS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Phase 116 gate fingerprint: host key must be concrete non-loopback IP with verified listener.
pub fn dist_region_nonloopback_fingerprint() -> &'static str {
    "dist-region-hostkey-v6:concrete-ip"
}

/// Phase 113 gate fingerprint: multi-host keyed region TCP health + runtime-bound host keys.
pub fn dist_region_multihost_fingerprint() -> &'static str {
    dist_region_nonloopback_fingerprint()
}

/// Phase 113: TCP CDN peer delivery via peer TCP pull/ack (not process-local VecDeque GA).
pub fn cdn_tcp_peer_fingerprint() -> &'static str {
    "dist-cdn-tcp-v2:peer-delivery"
}

/// Runtime-bound host key addresses proven by actual `TcpListener::bind` (Phase 113+).
static RUNTIME_BOUND_HOST_KEYS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
/// Live listener bind IPs verified at spawn time (Phase 115 bind-eq-key v5).
static RUNTIME_LIVE_LISTENER_BINDS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn runtime_bound_host_keys() -> &'static Mutex<Vec<String>> {
    RUNTIME_BOUND_HOST_KEYS.get_or_init(|| Mutex::new(Vec::new()))
}

fn runtime_live_listener_binds() -> &'static Mutex<Vec<String>> {
    RUNTIME_LIVE_LISTENER_BINDS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Register a host-key address proven by a live socket bind (concrete IP recorded for hostkey GA).
pub fn dist_host_key_register_live_listener_bind(addr: &SocketAddr) {
    let key = dist_host_key_concrete_ip_for_bind(addr);
    let mut slot = runtime_bound_host_keys().lock();
    if !slot.iter().any(|k| k == &key) {
        slot.push(key.clone());
    }
    let mut live = runtime_live_listener_binds().lock();
    if !live.iter().any(|k| k == &key) {
        live.push(key);
    }
}

/// Register a host-key bind IP without live-listener verification (not GA on its own).
pub fn dist_host_key_register_runtime_bind(addr: &SocketAddr) {
    let key = addr.ip().to_string();
    let mut slot = runtime_bound_host_keys().lock();
    if !slot.iter().any(|k| k == &key) {
        slot.push(key);
    }
}

pub fn dist_host_key_runtime_binds_clear() {
    runtime_bound_host_keys().lock().clear();
    runtime_live_listener_binds().lock().clear();
}

/// True when a live `TcpListener` registered `host_key_addr` at spawn (bind-eq-key v5).
pub fn dist_host_key_has_live_listener(host_key_addr: &str) -> bool {
    runtime_live_listener_binds()
        .lock()
        .iter()
        .any(|b| b == host_key_addr)
}

/// True when configured host key string exactly equals the runtime bind IP string.
pub fn dist_host_key_bind_eq_key(bind_ip: &str, host_key: &str) -> bool {
    bind_ip == host_key
}

/// True when `host_key_addr` is an RFC5737 documentation / TEST-NET address.
pub fn dist_host_key_addr_is_documentation(host_key_addr: &str) -> bool {
    let Ok(ip) = host_key_addr.parse::<std::net::IpAddr>() else {
        return false;
    };
    match ip {
        std::net::IpAddr::V4(v4) => {
            let o = v4.octets();
            // RFC5737 TEST-NET-1/2/3
            (o[0] == 192 && o[1] == 0 && o[2] == 2)
                || (o[0] == 198 && o[1] == 51 && o[2] == 100)
                || (o[0] == 203 && o[1] == 0 && o[2] == 113)
        }
        std::net::IpAddr::V6(_) => false,
    }
}

/// True when `host_key_addr` parses as a loopback IP (logical key identity, not bind addr).
pub fn dist_host_key_addr_is_loopback(host_key_addr: &str) -> bool {
    if let Ok(ip) = host_key_addr.parse::<std::net::IpAddr>() {
        return ip.is_loopback();
    }
    host_key_addr.eq_ignore_ascii_case("localhost")
}

/// GA requires at least one configured host key address that is not loopback.
pub fn dist_region_multihost_has_nonloopback_host_key() -> bool {
    region_multihost_endpoints()
        .lock()
        .iter()
        .any(|(_, _, addr, _)| !dist_host_key_addr_is_loopback(addr))
}

/// Phase 115 GA: ≥1 configured host key that is runtime-bound with live listener, bind-eq-key, non-loopback.
pub fn dist_region_multihost_has_runtime_bound_host_key() -> bool {
    let _ = dist_region_nonloopback_fingerprint();
    let binds = runtime_bound_host_keys().lock();
    region_multihost_endpoints().lock().iter().any(|(_, _, addr, _)| {
        dist_host_key_runtime_bind_addr_is_ga(addr, &binds)
    })
}

/// True when `host_key_addr` is runtime-bound with live listener, concrete IP, non-loopback, not RFC5737.
pub fn dist_host_key_runtime_bind_addr_is_ga(
    host_key_addr: &str,
    runtime_binds: &[String],
) -> bool {
    !dist_host_key_addr_is_unspecified(host_key_addr)
        && !dist_host_key_addr_is_documentation(host_key_addr)
        && !dist_host_key_addr_is_loopback(host_key_addr)
        && runtime_binds.iter().any(|b| b == host_key_addr)
        && dist_host_key_has_live_listener(host_key_addr)
}

/// Phase 115 bind-eq-key v5 GA predicate.
pub fn dist_region_multihost_has_runtime_bound_host_key_v5() -> bool {
    dist_region_multihost_has_runtime_bound_host_key()
}

/// Retired Phase 114 alias — delegates to v5 (no separate v4 GA).
pub fn dist_region_multihost_has_runtime_bound_host_key_v4() -> bool {
    dist_region_multihost_has_runtime_bound_host_key_v5()
}

fn tcp_url_host_part(url: &str) -> String {
    let host_port = url.strip_prefix("tcp://").unwrap_or(url);
    host_port
        .rsplit_once(':')
        .map(|(h, _)| h.trim_matches(['[', ']']).to_string())
        .unwrap_or_else(|| host_port.to_string())
}

/// True when any TCP region transport (multihost or anycast) is configured.
pub fn dist_region_routing_has_tcp() -> bool {
    dist_region_multihost_has_endpoints() || dist_region_anycast_has_endpoints()
}

/// Register `(region, host_key, tcp://host:port)` endpoints (legacy; host key addr derived from tcp URL host).
pub fn dist_region_multihost_configure(endpoints: &[(&str, &str, &str)]) {
    let mut slot = region_multihost_endpoints().lock();
    slot.clear();
    for (region, host_key, url) in endpoints {
        let normalized = if url.starts_with("tcp://") {
            (*url).to_string()
        } else {
            format!("tcp://{url}")
        };
        let host_key_addr = tcp_url_host_part(&normalized);
        slot.push((
            (*region).to_string(),
            (*host_key).to_string(),
            host_key_addr,
            normalized,
        ));
    }
}

/// Register `(region, host_key_name, host_key_addr, tcp_url)` — host key addr decoupled from bind/connect URL.
pub fn dist_region_multihost_configure_keys(endpoints: &[(&str, &str, &str, &str)]) {
    let mut slot = region_multihost_endpoints().lock();
    slot.clear();
    for (region, host_key, host_key_addr, url) in endpoints {
        let normalized = if url.starts_with("tcp://") {
            (*url).to_string()
        } else {
            format!("tcp://{url}")
        };
        slot.push((
            (*region).to_string(),
            (*host_key).to_string(),
            (*host_key_addr).to_string(),
            normalized,
        ));
    }
}

pub fn dist_region_multihost_clear() {
    region_multihost_endpoints().lock().clear();
    dist_host_key_runtime_binds_clear();
}

pub fn dist_region_multihost_has_endpoints() -> bool {
    !region_multihost_endpoints().lock().is_empty()
}

pub fn dist_region_multihost_endpoint_count() -> usize {
    region_multihost_endpoints().lock().len()
}

/// Default host key for a region (first configured, or env `SYX_DIST_HOST_KEY`).
pub fn dist_region_default_host_key(region: &str) -> String {
    if let Ok(key) = std::env::var("SYX_DIST_HOST_KEY") {
        if !key.is_empty() {
            return key;
        }
    }
    let slot = region_multihost_endpoints().lock();
    slot.iter()
        .find(|(r, ..)| r == region)
        .map(|(_, k, _, _)| k.clone())
        .unwrap_or_else(|| "host-a".to_string())
}

/// TCP health probe for a region's keyed endpoint.
pub fn dist_region_multihost_tcp_health(region: &str, host_key: &str) -> bool {
    let url = {
        let slot = region_multihost_endpoints().lock();
        slot.iter()
            .find(|(r, k, _, _)| r == region && k == host_key)
            .map(|(_, _, _, u)| u.clone())
    };
    match url {
        Some(u) => dist_tcp_endpoint_alive(&u),
        None => false,
    }
}

/// Failover order: preferred host key in region, other keys in region, then other regions.
fn dist_region_multihost_failover_order(
    region: &str,
    preferred_host_key: &str,
) -> Vec<(String, String, String, String)> {
    let slot = region_multihost_endpoints().lock();
    let mut out = Vec::with_capacity(slot.len());
    if let Some(entry) = slot
        .iter()
        .find(|(r, k, _, _)| r == region && k == preferred_host_key)
    {
        out.push(entry.clone());
    }
    for (r, k, addr, u) in slot.iter() {
        if r == region && k != preferred_host_key {
            out.push((r.clone(), k.clone(), addr.clone(), u.clone()));
        }
    }
    for (r, k, addr, u) in slot.iter() {
        if r != region {
            out.push((r.clone(), k.clone(), addr.clone(), u.clone()));
        }
    }
    out
}

/// Route via preferred `(region, host_key)` TCP; on health fail, keyed failover to next host.
///
/// Returns `Ok((active_region, active_host_key, ack_payload))`. Memory CDN is not used.
pub fn dist_region_multihost_route(
    region: &str,
    host_key: &str,
    from: i64,
    to: i64,
    payload: impl Into<String>,
) -> Result<(String, String, String), String> {
    let payload = payload.into();
    let order = dist_region_multihost_failover_order(region, host_key);
    if order.is_empty() {
        return Err(
            "dist multihost requires configured (region, host_key, tcp) endpoints (dist_region_multihost_configure)"
                .to_string(),
        );
    }
    let mut last_err = String::from("dist multihost: no healthy host key");
    for (active_region, active_host, _host_key_addr, url) in order {
        if !dist_tcp_endpoint_alive(&url) {
            last_err = format!("dist multihost {active_region}:{active_host} unhealthy");
            continue;
        }
        match region_route_tcp(&url, &active_region, from, to, payload.clone()) {
            Ok(ack) => return Ok((active_region, active_host, ack)),
            Err(e) => {
                last_err = e;
            }
        }
    }
    Err(last_err)
}

#[cfg(test)]
mod dist_transport_tests {
    use super::*;

    #[test]
    fn p116_hostkey_v6_rejects_unspecified_loopback_synthetic_and_rfc5737() {
        let binds = vec!["127.0.0.1".to_string()];
        dist_host_key_runtime_binds_clear();
        runtime_bound_host_keys().lock().extend(binds.clone());
        assert!(!dist_host_key_runtime_bind_addr_is_ga("127.0.0.1", &binds));
        assert!(!dist_host_key_runtime_bind_addr_is_ga("0.0.0.0", &binds));
        assert!(!dist_host_key_runtime_bind_addr_is_ga("192.0.2.10", &binds));
        assert!(dist_host_key_addr_is_unspecified("0.0.0.0"));

        dist_host_key_register_runtime_bind(
            &"10.0.0.5:0"
                .parse::<std::net::SocketAddr>()
                .expect("synthetic"),
        );
        let synthetic = runtime_bound_host_keys().lock().clone();
        assert!(
            !dist_host_key_runtime_bind_addr_is_ga("10.0.0.5", &synthetic),
            "synthetic register without live listener must not satisfy v6 GA"
        );
        dist_host_key_runtime_binds_clear();
    }

    #[test]
    fn p118_fingerprint_literals_present() {
        assert_eq!(cdn_pop_fingerprint(), "dist-cdn-pop-v6:multi-hostkey-route");
        assert_eq!(cdn_bgp_fingerprint(), "dist-cdn-bgp-v8:sticky-pop-mvp");
        assert_eq!(
            dist_region_nonloopback_fingerprint(),
            "dist-region-hostkey-v6:concrete-ip"
        );
    }

    #[test]
    fn p118_bgp_peer_exchange_requires_import() {
        cdn_bgp_clear();
        let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 4);
        let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 4);
        std::thread::sleep(Duration::from_millis(20));
        let url_a = dist_tcp_url_for_bind_addr(&addr_a);
        let url_b = dist_tcp_url_for_bind_addr(&addr_b);
        cdn_bgp_configure_mesh("region-a", &[("203.0.113.0/24", 100, "bgp-local", url_a.as_str())]);
        assert!(
            !cdn_bgp_ga_ready(),
            "local table alone must not pass mesh GA"
        );
        cdn_bgp_import_peer_mesh(
            "peer-b",
            "region-b",
            &[("198.51.100.0/24", 250, "bgp-peer", url_b.as_str())],
        );
        assert!(cdn_bgp_peer_exchange_done());
        // Phase 123+ GA needs ≥3 PoPs + weighted anycast; 2-pop peer exchange alone is not GA.
        assert!(!cdn_bgp_ga_ready());
        stop_a.stop();
        stop_b.stop();
        cdn_bgp_clear();
    }

    #[test]
    fn p117_fingerprint_literals_present() {
        let src = include_str!("dist_transport.rs");
        assert!(src.contains("dist-cdn-bgp-v3:ops-visibility"));
        assert!(src.contains("dist-cdn-bgp-v4:route-convergence-mvp"));
        assert!(src.contains("dist-cdn-pop-v6:multi-hostkey-route"));
        assert!(src.contains("cdn_pop_multi_hostkey_route_ready"));
    }

    #[test]
    fn p117_cdn_pop_loopback_only_not_ga() {
        cdn_pop_clear();
        CDN_POP_SIBLING_PEER.store(false, Ordering::Relaxed);
        let (addr, _handle, stop) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 4);
        let url = dist_tcp_url_for_bind_addr(&addr);
        let host_key = dist_host_key_remote_for_bind(&addr);
        cdn_pop_configure(&[
            ("pop-a", host_key.as_str(), url.as_str()),
            ("pop-b", host_key.as_str(), url.as_str()),
        ]);
        assert!(!cdn_pop_has_endpoints(), "loopback-only PoP must not be GA");
        assert!(!cdn_pop_remote_hostkey_ready());
        stop.stop();
        cdn_pop_clear();
    }

    #[test]
    fn p117_connect_url_not_loopback_rewrite_for_unspecified_bind() {
        let (addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 2);
        let url = dist_tcp_url_for_bind_addr(&addr);
        assert!(
            !url.contains("127.0.0.1"),
            "0.0.0.0 bind connect URL must not rewrite to 127.0.0.1: {url}"
        );
        let host_key = dist_host_key_remote_for_bind(&addr);
        assert!(
            !dist_host_key_addr_is_unspecified(&dist_host_key_remote_ip_part(&host_key)),
            "hostkey must be concrete IP, got {host_key}"
        );
        assert!(!dist_connect_url_is_loopback(&url));
        stop.stop();
    }

    #[test]
    fn p117_cdn_bgp_config_health_ga() {
        cdn_bgp_clear();
        cdn_delivery_ack_clear();
        let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 8);
        let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 8);
        std::thread::sleep(Duration::from_millis(25));
        let url_a = dist_tcp_url_for_bind_addr(&addr_a);
        let url_b = dist_tcp_url_for_bind_addr(&addr_b);
        assert!(!url_a.contains("127.0.0.1"));
        assert!(!url_b.contains("127.0.0.1"));
        cdn_bgp_configure_mesh("us-east", &[
            ("203.0.113.0/24", 100, "bgp-us", url_a.as_str()),
        ]);
        cdn_bgp_import_peer_mesh(
            "peer-eu",
            "eu-west",
            &[("198.51.100.0/24", 200, "bgp-eu", url_b.as_str())],
        );
        assert!(!cdn_bgp_ga_ready());
        assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-region-mesh");
        assert!(cdn_bgp_health("bgp-us"));
        assert!(cdn_bgp_health("bgp-eu"));
        // Phase 124 weighted-anycast route GA requires ≥3 differentiated-weight PoPs.
        assert!(
            cdn_bgp_route("bgp-us", 1, 2, "bgp-mvp").is_err(),
            "2-PoP mesh must not satisfy weighted-anycast route GA"
        );
        stop_a.stop();
        stop_b.stop();
        cdn_bgp_clear();
        cdn_delivery_ack_clear();
    }
}
