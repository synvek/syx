//! SYX driver farm TCP protocol + remote cache IO.
//! Extracted in S2-I216 (zero semantics).

use crate::cache_fingerprint::{hash_source, module_hash};
use crate::{cache_entry_fresh, project_cache_dir, CachedModuleUnit};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static FARM_TCP_FETCH_MUTEX: Mutex<()> = Mutex::new(());
static LAST_CACHE_REMOTE_PUSH: Mutex<Option<String>> = Mutex::new(None);
static LAST_CACHE_REMOTE_FETCH: Mutex<Option<String>> = Mutex::new(None);

/// Phase 94: default farm worker cache URL (`tcp://` tenant path; override with `SYX_FARM_CACHE_URL`).
pub fn default_farm_cache_url() -> String {
    std::env::var("SYX_FARM_CACHE_URL").unwrap_or_else(|_| {
        if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
            if let Ok(addr) = listener.local_addr() {
                return format!("tcp://127.0.0.1:{}/tenant/default", addr.port());
            }
        }
        "tcp://127.0.0.1:0/tenant/default".into()
    })
}

/// Phase 94: TCP cache fetch with mutex to reduce concurrent flake in farm 3-node gates.
pub fn fetch_cache_remote_hardened(url: &str) -> Result<Vec<u8>, String> {
    if url.starts_with("tcp://") {
        let _guard = FARM_TCP_FETCH_MUTEX
            .lock()
            .map_err(|e| format!("farm tcp mutex poisoned: {e}"))?;
        let mut last_err = None;
        for attempt in 0..3 {
            match fetch_cache_remote(url) {
                Ok(b) => return Ok(b),
                Err(e) => {
                    last_err = Some(e);
                    if attempt < 2 {
                        std::thread::sleep(std::time::Duration::from_millis(
                            10 * (attempt + 1) as u64,
                        ));
                    }
                }
            }
        }
        return Err(last_err.unwrap_or_else(|| "tcp cache fetch failed".into()));
    }
    fetch_cache_remote(url)
}

/// Phase 100: farm TCP frame status (`OK` payload / `MISS` / `ERR`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FarmTcpResponse {
    Ok(Vec<u8>),
    Miss,
    Err(String),
}

pub(crate) fn farm_tcp_url_parts(rest: &str) -> (&str, &str) {
    let (host_port, path) = rest.split_once('/').unwrap_or((rest, ""));
    (host_port, path.trim_start_matches('/'))
}

pub(crate) fn farm_tcp_write_frame(
    stream: &mut impl std::io::Write,
    payload: &[u8],
) -> Result<(), String> {
    use std::io::Write;
    let len = u32::try_from(payload.len()).map_err(|_| "farm tcp frame too large".to_string())?;
    stream
        .write_all(&len.to_be_bytes())
        .map_err(|e| format!("farm tcp frame write len: {e}"))?;
    stream
        .write_all(payload)
        .map_err(|e| format!("farm tcp frame write body: {e}"))?;
    Ok(())
}

pub(crate) fn farm_tcp_read_frame(stream: &mut impl std::io::Read) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut len_buf = [0u8; 4];
    stream
        .read_exact(&mut len_buf)
        .map_err(|e| format!("farm tcp frame read len: {e}"))?;
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream
            .read_exact(&mut payload)
            .map_err(|e| format!("farm tcp frame read body: {e}"))?;
    }
    Ok(payload)
}

/// Encode a length-prefixed farm TCP frame payload (without the length prefix).
pub fn farm_tcp_encode_get(key: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + key.len());
    out.extend_from_slice(b"GET\0");
    out.extend_from_slice(key.as_bytes());
    out
}

pub fn farm_tcp_encode_put(key: &str, value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + key.len() + value.len());
    out.extend_from_slice(b"PUT\0");
    out.extend_from_slice(key.as_bytes());
    out.push(0);
    out.extend_from_slice(value);
    out
}

pub fn farm_tcp_encode_ok(value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + value.len());
    out.extend_from_slice(b"OK");
    out.extend_from_slice(value);
    out
}

pub fn farm_tcp_encode_miss() -> Vec<u8> {
    b"MISS".to_vec()
}

pub fn farm_tcp_encode_err(msg: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(3 + msg.len());
    out.extend_from_slice(b"ERR");
    out.extend_from_slice(msg.as_bytes());
    out
}

pub fn farm_tcp_parse_response(payload: &[u8]) -> Result<FarmTcpResponse, String> {
    if payload == b"MISS" {
        return Ok(FarmTcpResponse::Miss);
    }
    if let Some(msg) = payload.strip_prefix(b"ERR") {
        return Ok(FarmTcpResponse::Err(
            String::from_utf8_lossy(msg).into_owned(),
        ));
    }
    if let Some(value) = payload.strip_prefix(b"OK") {
        return Ok(FarmTcpResponse::Ok(value.to_vec()));
    }
    Err(format!(
        "farm tcp bad response: {:?}",
        String::from_utf8_lossy(payload)
    ))
}

/// Handle one farm TCP request against an in-memory store; returns response payload (no length prefix).
///
/// Gate fingerprint: `farm-lease-server-v1:node-enforces-ttl` — GET/PUT require
/// [`farm_lease_require_active`] (authority grant + TTL); raw TCP without lease → `ERR`.
pub fn farm_tcp_handle_request(
    store: &mut std::collections::HashMap<String, Vec<u8>>,
    req: &[u8],
) -> Vec<u8> {
    if let Err(msg) = crate::farm_lease_auth::farm_lease_require_active() {
        return farm_tcp_encode_err(&msg);
    }
    if let Some(key) = req.strip_prefix(b"GET\0") {
        let key = String::from_utf8_lossy(key).into_owned();
        return match store.get(&key) {
            Some(v) => farm_tcp_encode_ok(v),
            None => farm_tcp_encode_miss(),
        };
    }
    if let Some(rest) = req.strip_prefix(b"PUT\0") {
        let Some(sep) = rest.iter().position(|&b| b == 0) else {
            return farm_tcp_encode_err("PUT missing value separator");
        };
        let key = String::from_utf8_lossy(&rest[..sep]).into_owned();
        let value = rest[sep + 1..].to_vec();
        store.insert(key, value.clone());
        return farm_tcp_encode_ok(&value);
    }
    farm_tcp_encode_err("unknown op")
}

/// One length-prefixed farm TCP round-trip (tests / raw clients bypassing cache helpers).
#[doc(hidden)]
pub fn farm_tcp_roundtrip(host_port: &str, req: &[u8]) -> Result<Vec<u8>, String> {
    let mut stream =
        std::net::TcpStream::connect(host_port).map_err(|e| format!("farm tcp connect: {e}"))?;
    farm_tcp_write_frame(&mut stream, req)?;
    farm_tcp_read_frame(&mut stream)
}

/// Spawn a farm TCP node that speaks GET/PUT frames for `max_conns` accepts.
///
/// Callers that fail-closed without connecting must pass `max_conns = 0` so
/// `JoinHandle::join` returns immediately (Phase 105).
pub fn spawn_farm_tcp_node(
    initial: std::collections::HashMap<String, Vec<u8>>,
    max_conns: usize,
) -> (std::net::SocketAddr, std::thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("farm tcp bind");
    listener
        .set_nonblocking(true)
        .expect("farm tcp listener nonblocking");
    let addr = listener.local_addr().expect("farm tcp local_addr");
    let handle = std::thread::spawn(move || {
        let mut store = initial;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        let mut accepted = 0usize;
        while accepted < max_conns && std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    accepted += 1;
                    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));
                    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(2)));
                    if let Ok(req) = farm_tcp_read_frame(&mut stream) {
                        let resp = farm_tcp_handle_request(&mut store, &req);
                        let _ = farm_tcp_write_frame(&mut stream, &resp);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(_) => {
                    // Transient accept errors must not tear down the node early —
                    // expired-lease clients still need to connect for ERR responses.
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
    });
    (addr, handle)
}

/// Phase 70: single-node cache mirror fetch (`SYX_CACHE_REMOTE`, `file://`, `http(s)://`, or `tcp://`).
/// Phase 100: `tcp://` uses length-prefixed GET/PUT frames (`OK|MISS|ERR`).
pub fn fetch_cache_remote(url: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = url.strip_prefix("file://") {
        return std::fs::read(path).map_err(|e| e.to_string());
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        return reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .bytes()
            .map_err(|e| e.to_string())
            .map(|b| b.to_vec());
    }
    if let Some(rest) = url.strip_prefix("tcp://") {
        // Phase 105 fail-closed: missing authority OR inactive/expired lease → ERR
        // without opening TCP (avoids farm-node accept/join hangs when clients stop early).
        if !crate::farm_lease_auth::farm_lease_auth_is_configured() {
            return Err("tcp cache lease: farm lease authority not configured".into());
        }
        crate::farm_lease_auth::farm_lease_require_active()
            .map_err(|e| format!("tcp cache lease: {e}"))?;

        let (host_port, key) = farm_tcp_url_parts(rest);
        let mut stream = std::net::TcpStream::connect(host_port)
            .map_err(|e| format!("tcp cache connect failed: {e}"))?;
        farm_tcp_write_frame(&mut stream, &farm_tcp_encode_get(key))?;
        let payload = farm_tcp_read_frame(&mut stream)?;
        record_cache_remote_fetch(url);
        match farm_tcp_parse_response(&payload)? {
            FarmTcpResponse::Ok(bytes) => return Ok(bytes),
            FarmTcpResponse::Miss => return Err(format!("tcp cache miss: {key}")),
            FarmTcpResponse::Err(msg) => return Err(format!("tcp cache err: {msg}")),
        }
    }
    Err(format!("unsupported SYX_CACHE_REMOTE URL: {url}"))
}

/// Phase 76: push compiled cache artifact to remote mirror (`file://` or `http(s)://` PUT).
/// Phase 100: `tcp://` uses length-prefixed PUT frames; expects `OK`.
pub fn push_cache_remote(url: &str, bytes: &[u8]) -> Result<(), String> {
    record_cache_remote_push(url);
    if let Some(path) = url.strip_prefix("file://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        return std::fs::write(path, bytes)
            .map_err(|e| e.to_string())
            .map(|_| ());
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        return reqwest::blocking::Client::new()
            .put(url)
            .body(bytes.to_vec())
            .send()
            .map_err(|e| e.to_string())
            .map(|_| ());
    }
    if let Some(rest) = url.strip_prefix("tcp://") {
        if !crate::farm_lease_auth::farm_lease_auth_is_configured() {
            return Err("tcp cache lease: farm lease authority not configured".into());
        }
        crate::farm_lease_auth::farm_lease_require_active()
            .map_err(|e| format!("tcp cache lease: {e}"))?;

        let (host_port, key) = farm_tcp_url_parts(rest);
        let mut stream = std::net::TcpStream::connect(host_port)
            .map_err(|e| format!("tcp cache push connect failed: {e}"))?;
        farm_tcp_write_frame(&mut stream, &farm_tcp_encode_put(key, bytes))?;
        let payload = farm_tcp_read_frame(&mut stream)?;
        match farm_tcp_parse_response(&payload)? {
            FarmTcpResponse::Ok(_) => return Ok(()),
            FarmTcpResponse::Miss => return Err("tcp cache put unexpected MISS".into()),
            FarmTcpResponse::Err(msg) => return Err(format!("tcp cache put err: {msg}")),
        }
    }
    Err(format!("unsupported cache push URL: {url}"))
}

/// Phase 76: broadcast cache artifact to `SYX_CACHE_REMOTE_NODES` (comma-separated).
pub fn push_cache_remote_nodes(base: &str, key: &str, bytes: &[u8]) {
    let nodes: Vec<String> = std::env::var("SYX_CACHE_REMOTE_NODES")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty())
                .collect()
        })
        .filter(|v: &Vec<String>| !v.is_empty())
        .unwrap_or_else(|| vec![base.to_string()]);
    for node in nodes {
        let url = format!("{}/{}", node.trim_end_matches('/'), key);
        let _ = push_cache_remote(&url, bytes);
    }
}

pub(crate) fn cache_remote_push_enabled() -> bool {
    std::env::var("SYX_CACHE_REMOTE_PUSH")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

pub(crate) fn record_cache_remote_push(url: &str) {
    if let Ok(mut g) = LAST_CACHE_REMOTE_PUSH.lock() {
        *g = Some(url.to_string());
    }
}

pub(crate) fn record_cache_remote_fetch(url: &str) {
    if let Ok(mut g) = LAST_CACHE_REMOTE_FETCH.lock() {
        *g = Some(url.to_string());
    }
}

/// Phase 76: last cache remote push/fetch URLs for doctor report.
pub fn cache_remote_report() -> (Option<String>, Option<String>) {
    let push = LAST_CACHE_REMOTE_PUSH.lock().ok().and_then(|g| g.clone());
    let fetch = LAST_CACHE_REMOTE_FETCH.lock().ok().and_then(|g| g.clone());
    (push, fetch)
}

/// Remove cache entries whose recorded dependency snapshots no longer match disk.
pub fn purge_stale_incremental_caches(root: &Path) {
    let cache_dir = project_cache_dir(root);
    let Ok(entries) = std::fs::read_dir(&cache_dir) else {
        // One-time migration: drop legacy `.sy-cache` at project root.
        let legacy = root.join(".sy-cache");
        if legacy.is_dir() {
            let _ = std::fs::remove_dir_all(&legacy);
        }
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        // S3-I49: dedicated prelude snapshots use a different bincode shape.
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("prelude_"))
        {
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) else {
            let _ = std::fs::remove_file(&path);
            continue;
        };
        let current = std::fs::read_to_string(&cached.path).unwrap_or_default();
        if cached.source_hash != hash_source(&current) || !cache_entry_fresh(&cached, &current) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

pub fn incremental_cache_path(root: &Path, module: &Path) -> PathBuf {
    let cache_dir = project_cache_dir(root);
    let _ = std::fs::create_dir_all(&cache_dir);
    cache_dir.join(format!("{}.bin", module_hash(module, root, &[])))
}
