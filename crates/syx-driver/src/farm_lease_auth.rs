//! Phase 104–109: cross-node farm worker lease authority (TCP grant/renew/query).
//!
//! Phase 107: dual-authority HA (shared Mutex — superseded).
//! Phase 108: process-isolated HA — independent `AuthorityState` per node + SYNC
//! replication so secondary can GRANT/validate after primary stop without sharing
//! one Mutex. (Same-process thread+SYNC — **not** HA v3 GA.)
//! Phase 109: OS-process HA v3 via `std::process::Command` + Raft quorum MVP.
//! Phase 110: Raft v2 replicated log + WAL fsync + membership change MVP.
//! Phase 111: configurable multihost members, tenant isolation MVP, WAL CRC.
//! Phase 112: runtime member config (env/file JSON), WAL snapshot checkpoint, tenant token MVP.
//! Phase 113: token default enforce (not RUNTIME_GA-only GA), WAL log compaction,
//! variable member list (`Vec`, len ≥ 3 — not fixed `[String;3]`).
//! Phase 114: token on all GRANT paths (HA + authority), reject legacy snap without token,
//! region-tagged multi-DC discovery probe + failover MVP.
//! Phase 115: multi-DC wired into runtime spawn, ban loopback GA fallback on runtime config,
//! member registry reload+probe MVP.
//! Phase 116: cross-host non-loopback member GA (≥2 distinct host keys), registry health-sync
//! (leader MEMBER push after HEALTH prune), legacy loopback fallback banned on all paths.
//! Phase 117: real-connect GA (≥2 non-loopback connect targets; ban 0.0.0.0→127 rewrite as GA),
//! registry consensus-sync (membership change via Raft log observable by follower), tenant token rotate MVP.
//! Phase 118: os-dual-process GA (≥2 independent OS child processes; ban TEST-NET hostkey-only),
//! registry follower-view (follower MEMBER set matches leader after consensus-sync),
//! durable token epoch file (rotate persists; old token invalid after restart).
//!
//! Gate fingerprints: `farm-lease-auth-v1:cross-node-ttl`,
//! `farm-lease-failclosed-v1:no-auth-is-err`, `farm-lease-ha-v2:process-isolated`,
//! `farm-lease-ha-v3:os-process`, `farm-lease-raft-v1:quorum-mvp`,
//! `farm-lease-raft-v2:log-wal`, `farm-lease-member-v1:dynamic-mvp`,
//! `farm-lease-multihost-v1:config-members`, `farm-lease-tenant-v1:isolation-mvp`,
//! `farm-lease-wal-crc-v1:reject-corrupt`, `farm-lease-runtime-v1:config-members`,
//! `farm-lease-tenant-v2:token-mvp`, `farm-lease-wal-snap-v1:checkpoint-replay`,
//! `farm-lease-tenant-v3:token-default`, `farm-lease-wal-compact-v1:snap-truncate-log`,
//! `farm-lease-members-v2:dyn-list`, `farm-lease-tenant-v4:token-all-paths-multidc`,
//! `farm-lease-multidc-v2:runtime-wired`, `farm-lease-registry-v1:reload-probe`,
//! `farm-lease-multidc-v3:cross-host`, `farm-lease-registry-v2:health-sync`,
//! `farm-lease-multidc-v4:real-connect`, `farm-lease-registry-v3:consensus-sync`,
//! `farm-lease-token-v1:rotate`, `farm-lease-multidc-v5:os-dual-process`,
//! `farm-lease-registry-v4:follower-view`, `farm-lease-token-v2:durable-epoch`,
//! `farm-lease-token-v3:hsm-ref-mvp`, `farm-lease-token-v4:kms-bridge-mvp`,
//! `farm-lease-token-v5:pkcs11-bridge-mvp`
//! `farm-lease-token-v6:pkcs11-so-adapter`
//! `farm-lease-token-v7:pkcs11-session-mvp`
//! `farm-lease-token-v8:session-pin-persist`
//! `farm-lease-token-v17:hsm-session-pin-mvp`
//! `farm-lease-token-v18:hsm-key-label-mvp`
//! `farm-lease-token-v19:hsm-key-id-mvp`
//! `farm-lease-token-v9:pkcs11-sign-mvp`
//! `farm-lease-token-v10:session-close-mvp`
//! `farm-lease-token-v11:hsm-ref-bound-mvp`

use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

static LEASE_AUTH_URL: Mutex<Option<String>> = Mutex::new(None);
static LEASE_AUTH_SECONDARY: Mutex<Option<String>> = Mutex::new(None);
static LEASE_AUTH_TERTIARY: Mutex<Option<String>> = Mutex::new(None);

/// Env: bind address for OS-child authority (`host:port` or `127.0.0.1:0`).
pub const ENV_OS_CHILD_BIND: &str = "SYX_FARM_LEASE_OS_CHILD_BIND";
/// Env: max accepts before child exits.
pub const ENV_OS_CHILD_MAX_CONNS: &str = "SYX_FARM_LEASE_OS_CHILD_MAX_CONNS";
/// Env: peer URL for HA SYNC replication (`tcp://host:port`).
pub const ENV_OS_CHILD_PEER: &str = "SYX_FARM_LEASE_OS_CHILD_PEER";
/// Env: path where child writes `host:port` once listening.
pub const ENV_OS_CHILD_READY_FILE: &str = "SYX_FARM_LEASE_OS_CHILD_READY_FILE";
/// Env: `ha` or `raft` child mode.
pub const ENV_OS_CHILD_MODE: &str = "SYX_FARM_LEASE_OS_CHILD_MODE";
/// Env: raft node id `0|1|2`.
pub const ENV_OS_CHILD_RAFT_ID: &str = "SYX_FARM_LEASE_OS_CHILD_RAFT_ID";
/// Env: comma-separated peer URLs for raft.
pub const ENV_OS_CHILD_RAFT_PEERS: &str = "SYX_FARM_LEASE_OS_CHILD_RAFT_PEERS";
/// Env: comma-separated full member URLs (`tcp://host:port`, ordered by node id).
pub const ENV_OS_CHILD_RAFT_MEMBER_URLS: &str = "SYX_FARM_LEASE_OS_CHILD_RAFT_MEMBER_URLS";
/// Env: cargo-test filter used when spawning OS children (default `p109_farm_os_child_entry`).
pub const ENV_OS_CHILD_FILTER: &str = "SYX_FARM_LEASE_OS_CHILD_FILTER";
/// Env: per-node WAL directory (created if missing; replayed on restart).
pub const ENV_OS_CHILD_WAL_DIR: &str = "SYX_FARM_LEASE_OS_CHILD_WAL_DIR";
/// Env: durable tenant token epoch file (defaults to `{WAL_DIR}/token-epoch.dat`).
pub const ENV_FARM_TOKEN_EPOCH_PATH: &str = "SYX_FARM_TOKEN_EPOCH_PATH";
/// Env: JSON array of `host:port` Raft member binds (Phase 112 runtime GA).
pub const ENV_FARM_LEASE_MEMBERS: &str = "SYX_FARM_LEASE_MEMBERS";
/// Env: path to JSON file containing a `host:port` array (Phase 112 runtime GA).
pub const ENV_FARM_LEASE_MEMBERS_FILE: &str = "SYX_FARM_LEASE_MEMBERS_FILE";
/// Env: when `1`, OS raft child marks runtime config GA (Phase 112 member JSON path).
/// Phase 113: **not** the token-security GA — tokens are default-enforced on all raft GRANT.
pub const ENV_RUNTIME_CONFIG_GA: &str = "SYX_FARM_LEASE_RUNTIME_GA";
/// Env: when `1`, OS raft child marks dyn-list members GA (Phase 113 variable `Vec` members).
pub const ENV_DYN_MEMBERS_GA: &str = "SYX_FARM_LEASE_DYN_MEMBERS_GA";

/// Fixed ports for Raft MVP (3-node quorum). Tests use `--test-threads=1`.
/// Phase 111: **not** multihost GA — use [`spawn_farm_lease_raft_multihost`].
pub const FARM_LEASE_RAFT_PORTS: [u16; 3] = [28_709, 28_710, 28_711];

/// Phase 111 compile-time binds (127.0.0.1 + ::1). Phase 112: **not** runtime GA —
/// load members via [`ENV_FARM_LEASE_MEMBERS`] / [`ENV_FARM_LEASE_MEMBERS_FILE`] and
/// [`spawn_farm_lease_raft_runtime`].
pub const FARM_LEASE_RAFT_MULTIHOST_BINDS: [&str; 3] =
    ["127.0.0.1:28720", "127.0.0.1:28721", "[::1]:28722"];

/// Leader-election probe token (not a tenant secret; cleared via RESET after discovery).
const RAFT_LEADER_PROBE_TOKEN: u32 = 0xA11C_E000;

/// Phase 114: default tenant token for [`farm_lease_acquire`] / [`farm_lease_acquire_tenant`].
pub const FARM_LEASE_DEFAULT_TOKEN: u32 = 0xFA11_4000;

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

fn deadline_from_ttl(ttl_secs: u32) -> u64 {
    let now = now_ms();
    if ttl_secs == 0 {
        now.saturating_sub(1)
    } else {
        now + (ttl_secs as u64) * 1000
    }
}

fn tcp_write_frame(stream: &mut TcpStream, payload: &[u8]) -> Result<(), String> {
    let len = u32::try_from(payload.len()).map_err(|_| "lease auth frame too large".to_string())?;
    stream
        .write_all(&len.to_be_bytes())
        .map_err(|e| format!("lease auth write len: {e}"))?;
    stream
        .write_all(payload)
        .map_err(|e| format!("lease auth write body: {e}"))?;
    Ok(())
}

fn tcp_read_frame(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut len_buf = [0u8; 4];
    stream
        .read_exact(&mut len_buf)
        .map_err(|e| format!("lease auth read len: {e}"))?;
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream
            .read_exact(&mut payload)
            .map_err(|e| format!("lease auth read body: {e}"))?;
    }
    Ok(payload)
}

fn encode_ok_deadline(deadline_ms: u64) -> Vec<u8> {
    let mut out = b"OK\0".to_vec();
    out.extend_from_slice(deadline_ms.to_string().as_bytes());
    out
}

fn encode_ok_none() -> Vec<u8> {
    b"OK\0none".to_vec()
}

fn encode_err(msg: &str) -> Vec<u8> {
    let mut out = b"ERR\0".to_vec();
    out.extend_from_slice(msg.as_bytes());
    out
}

fn parse_ok_deadline(payload: &[u8]) -> Result<Option<u64>, String> {
    let rest = payload.strip_prefix(b"OK\0").ok_or_else(|| {
        format!(
            "lease auth bad response: {:?}",
            String::from_utf8_lossy(payload)
        )
    })?;
    let s = std::str::from_utf8(rest).map_err(|e| format!("lease auth deadline utf8: {e}"))?;
    if s == "none" {
        return Ok(None);
    }
    s.parse::<u64>()
        .map(Some)
        .map_err(|e| format!("lease auth deadline parse: {e}"))
}

fn parse_tenant_suffix(rest: &[u8]) -> u32 {
    if rest.len() >= 4 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&rest[..4]);
        u32::from_le_bytes(arr)
    } else {
        0
    }
}

/// Phase 112 GA: `tenant_id + token + ttl` (12 bytes).
fn parse_grant_token_payload(rest: &[u8]) -> Result<(u32, u32, u32), String> {
    if rest.len() != 12 {
        return Err("grant missing tenant+token+ttl".into());
    }
    let tenant_id = u32::from_le_bytes(rest[0..4].try_into().unwrap());
    let token = u32::from_le_bytes(rest[4..8].try_into().unwrap());
    let ttl = u32::from_le_bytes(rest[8..12].try_into().unwrap());
    Ok((tenant_id, token, ttl))
}

fn parse_query_token_payload(rest: &[u8]) -> Result<(u32, u32), String> {
    match rest.len() {
        4 => {
            let tenant_id = parse_tenant_suffix(rest);
            Ok((tenant_id, 0))
        }
        8 => {
            let tenant_id = u32::from_le_bytes(rest[0..4].try_into().unwrap());
            let token = u32::from_le_bytes(rest[4..8].try_into().unwrap());
            Ok((tenant_id, token))
        }
        _ => Err("query missing tenant+token".into()),
    }
}

fn encode_query(tenant_id: u32) -> Vec<u8> {
    let mut out = b"QUERY\0".to_vec();
    out.extend_from_slice(&tenant_id.to_le_bytes());
    out
}

fn encode_query_token(tenant_id: u32, token: u32) -> Vec<u8> {
    let mut out = b"QUERY\0".to_vec();
    out.extend_from_slice(&tenant_id.to_le_bytes());
    out.extend_from_slice(&token.to_le_bytes());
    out
}

fn encode_grant(tenant_id: u32, ttl_secs: u32) -> Vec<u8> {
    let mut out = b"GRANT\0".to_vec();
    out.extend_from_slice(&tenant_id.to_le_bytes());
    out.extend_from_slice(&ttl_secs.to_le_bytes());
    out
}

fn encode_grant_token(tenant_id: u32, token: u32, ttl_secs: u32) -> Vec<u8> {
    let mut out = b"GRANT\0".to_vec();
    out.extend_from_slice(&tenant_id.to_le_bytes());
    out.extend_from_slice(&token.to_le_bytes());
    out.extend_from_slice(&ttl_secs.to_le_bytes());
    out
}

fn encode_leader_probe_grant(ttl_secs: u32) -> Vec<u8> {
    encode_grant_token(0, RAFT_LEADER_PROBE_TOKEN, ttl_secs)
}

/// Phase 113: raft GRANT always requires tenant token (default enforce — not RUNTIME_GA opt-in).
fn raft_token_default_enforced() -> bool {
    true
}

#[derive(Default, Clone)]
struct AuthorityState {
    tenant_deadlines: HashMap<u32, u64>,
    tenant_tokens: HashMap<u32, u32>,
}

impl AuthorityState {
    fn deadline_for(&self, tenant_id: u32) -> Option<u64> {
        self.tenant_deadlines.get(&tenant_id).copied()
    }

    fn set_deadline(&mut self, tenant_id: u32, deadline_ms: u64) {
        self.tenant_deadlines.insert(tenant_id, deadline_ms);
    }

    fn clear_tenant(&mut self, tenant_id: u32) {
        self.tenant_deadlines.remove(&tenant_id);
        self.tenant_tokens.remove(&tenant_id);
    }

    fn clear_all(&mut self) {
        self.tenant_deadlines.clear();
        self.tenant_tokens.clear();
    }

    fn token_for(&self, tenant_id: u32) -> Option<u32> {
        self.tenant_tokens.get(&tenant_id).copied()
    }

    fn set_token(&mut self, tenant_id: u32, token: u32) {
        self.tenant_tokens.insert(tenant_id, token);
    }

    fn check_token(&self, tenant_id: u32, token: u32) -> bool {
        self.token_for(tenant_id).is_some_and(|t| t == token)
    }

    /// Phase 117: GRANT must present the current key-table token when one is registered.
    fn accept_grant_token(&self, tenant_id: u32, token: u32) -> Result<(), String> {
        if let Some(existing) = self.token_for(tenant_id) {
            if existing != token {
                return Err("token mismatch — rotated or invalid".into());
            }
        }
        Ok(())
    }

    /// Phase 117: rotate tenant token in the in-process key table.
    fn rotate_token(
        &mut self,
        tenant_id: u32,
        old_token: u32,
        new_token: u32,
    ) -> Result<(), String> {
        match self.token_for(tenant_id) {
            Some(t) if t == old_token => {
                self.set_token(tenant_id, new_token);
                Ok(())
            }
            Some(_) => Err("token mismatch — rotate rejected".into()),
            None => Err("no token to rotate".into()),
        }
    }
}

/// Replication snapshot after a mutating authority op (GRANT/RENEW/RESET).
#[derive(Clone, Copy, Debug)]
struct AuthorityMutation {
    tenant_id: u32,
    deadline_ms: Option<u64>,
    token: Option<u32>,
}

/// Apply one authority request. Returns `(response, mutated_state_snapshot)`.
/// `mutated` is `Some` when local state changed and should be replicated.
fn handle_authority_request(
    state: &mut AuthorityState,
    req: &[u8],
) -> (Vec<u8>, Option<AuthorityMutation>) {
    if req == b"RESET\0" {
        state.clear_all();
        return (
            encode_ok_none(),
            Some(AuthorityMutation {
                tenant_id: 0,
                deadline_ms: None,
                token: None,
            }),
        );
    }
    if req == b"HEALTH\0" {
        return (encode_ok_none(), None);
    }
    // Replication from peer: apply deadline + token without re-replicating.
    if let Some(rest) = req.strip_prefix(b"SYNC\0") {
        if rest == b"none" {
            state.clear_all();
        } else if let Some(colon) = rest.iter().position(|&b| b == b':') {
            let tenant_s = &rest[..colon];
            let rest2 = &rest[colon + 1..];
            let tenant_id = std::str::from_utf8(tenant_s)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            if rest2 == b"none" {
                state.clear_tenant(tenant_id);
            } else if let Some(colon2) = rest2.iter().position(|&b| b == b':') {
                let deadline_s = &rest2[..colon2];
                let token_s = &rest2[colon2 + 1..];
                let s = std::str::from_utf8(deadline_s).unwrap_or("");
                let token_s = std::str::from_utf8(token_s).unwrap_or("");
                match (s.parse::<u64>(), token_s.parse::<u32>()) {
                    (Ok(d), Ok(token)) => {
                        state.set_deadline(tenant_id, d);
                        state.set_token(tenant_id, token);
                    }
                    _ => return (encode_err("sync bad deadline or token"), None),
                }
            } else {
                return (encode_err("sync missing token"), None);
            }
        } else {
            return (encode_err("sync bad payload"), None);
        }
        return (encode_ok_none(), None);
    }
    if let Some(rest) = req.strip_prefix(b"QUERY\0") {
        let tenant_id = parse_tenant_suffix(rest);
        let resp = match state.deadline_for(tenant_id) {
            Some(d) => encode_ok_deadline(d),
            None => encode_ok_none(),
        };
        return (resp, None);
    }
    if let Some(rest) = req
        .strip_prefix(b"GRANT\0")
        .or_else(|| req.strip_prefix(b"RENEW\0"))
    {
        // Phase 114: all authority GRANT paths require tenant+token+ttl (same as Raft).
        // Phase 117: when a token is registered, GRANT must present the current token.
        match parse_grant_token_payload(rest) {
            Ok((tenant_id, token, ttl)) => {
                if let Err(e) = state.accept_grant_token(tenant_id, token) {
                    return (encode_err(&e), None);
                }
                let deadline = deadline_from_ttl(ttl);
                state.set_deadline(tenant_id, deadline);
                state.set_token(tenant_id, token);
                return (
                    encode_ok_deadline(deadline),
                    Some(AuthorityMutation {
                        tenant_id,
                        deadline_ms: Some(deadline),
                        token: Some(token),
                    }),
                );
            }
            Err(e) => {
                if rest.len() == 4 || rest.len() == 8 {
                    return (encode_err("legacy grant rejected — token required"), None);
                }
                return (encode_err(&e), None);
            }
        }
    }
    if let Some(rest) = req.strip_prefix(b"ROTATE\0") {
        let (tenant_id, old_token, new_token) = match parse_rotate_hsm_payload(rest) {
            Ok(v) => v,
            Err(e) => return (encode_err(&e), None),
        };
        match state.rotate_token(tenant_id, old_token, new_token) {
            Ok(()) => {
                return (
                    encode_ok_none(),
                    Some(AuthorityMutation {
                        tenant_id,
                        deadline_ms: state.deadline_for(tenant_id),
                        token: Some(new_token),
                    }),
                );
            }
            Err(e) => return (encode_err(&e), None),
        }
    }
    (encode_err("unknown lease op"), None)
}

fn encode_sync_payload(tenant_id: u32, deadline_ms: Option<u64>, token: Option<u32>) -> Vec<u8> {
    let mut out = b"SYNC\0".to_vec();
    out.extend_from_slice(tenant_id.to_string().as_bytes());
    out.push(b':');
    match deadline_ms {
        Some(d) => {
            out.extend_from_slice(d.to_string().as_bytes());
            out.push(b':');
            out.extend_from_slice(token.unwrap_or(0).to_string().as_bytes());
        }
        None => out.extend_from_slice(b"none"),
    }
    out
}

fn sync_lease_to_peer(peer_url: &str, mutation: AuthorityMutation) {
    let _ = lease_auth_rpc_one(
        peer_url,
        &encode_sync_payload(mutation.tenant_id, mutation.deadline_ms, mutation.token),
    );
}

fn normalize_tcp_url(url: &str) -> String {
    if url.starts_with("tcp://") {
        url.to_string()
    } else {
        format!("tcp://{url}")
    }
}

/// True when an in-process authority URL or `SYX_FARM_LEASE_AUTH` is set.
pub fn farm_lease_auth_is_configured() -> bool {
    if std::env::var("SYX_FARM_LEASE_AUTH")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return true;
    }
    LEASE_AUTH_URL.lock().ok().and_then(|g| g.clone()).is_some()
}

/// Drop in-process authority URL (tests); does not clear `SYX_FARM_LEASE_AUTH`.
pub fn farm_lease_auth_clear_configured() {
    if let Ok(mut g) = LEASE_AUTH_URL.lock() {
        *g = None;
    }
    if let Ok(mut g) = LEASE_AUTH_SECONDARY.lock() {
        *g = None;
    }
    if let Ok(mut g) = LEASE_AUTH_TERTIARY.lock() {
        *g = None;
    }
}

/// Point lease clients (workers + cache nodes) at the shared authority (`tcp://host:port`).
pub fn farm_lease_auth_configure(url: &str) {
    let normalized = normalize_tcp_url(url);
    if let Ok(mut g) = LEASE_AUTH_URL.lock() {
        *g = Some(normalized);
    }
    if let Ok(mut g) = LEASE_AUTH_SECONDARY.lock() {
        *g = None;
    }
    if let Ok(mut g) = LEASE_AUTH_TERTIARY.lock() {
        *g = None;
    }
}

/// Phase 107+: configure primary + secondary lease authorities (HA failover).
pub fn farm_lease_auth_configure_ha(primary: &str, secondary: &str) {
    let p = normalize_tcp_url(primary);
    let s = normalize_tcp_url(secondary);
    if let Ok(mut g) = LEASE_AUTH_URL.lock() {
        *g = Some(p);
    }
    if let Ok(mut g) = LEASE_AUTH_SECONDARY.lock() {
        *g = Some(s);
    }
    if let Ok(mut g) = LEASE_AUTH_TERTIARY.lock() {
        *g = None;
    }
}

/// Phase 109: configure 3-node Raft lease authorities (client tries until leader/quorum).
pub fn farm_lease_auth_configure_raft(a: &str, b: &str, c: &str) {
    let a = normalize_tcp_url(a);
    let b = normalize_tcp_url(b);
    let c = normalize_tcp_url(c);
    if let Ok(mut g) = LEASE_AUTH_URL.lock() {
        *g = Some(a);
    }
    if let Ok(mut g) = LEASE_AUTH_SECONDARY.lock() {
        *g = Some(b);
    }
    if let Ok(mut g) = LEASE_AUTH_TERTIARY.lock() {
        *g = Some(c);
    }
}

fn authority_url_list() -> Result<Vec<String>, String> {
    let env_primary = std::env::var("SYX_FARM_LEASE_AUTH")
        .ok()
        .filter(|s| !s.is_empty());
    let env_secondary = std::env::var("SYX_FARM_LEASE_AUTH_SECONDARY")
        .ok()
        .filter(|s| !s.is_empty());
    let env_tertiary = std::env::var("SYX_FARM_LEASE_AUTH_TERTIARY")
        .ok()
        .filter(|s| !s.is_empty());
    let primary = env_primary
        .or_else(|| LEASE_AUTH_URL.lock().ok().and_then(|g| g.clone()))
        .ok_or_else(|| {
            "farm lease authority not configured (set SYX_FARM_LEASE_AUTH or farm_lease_auth_configure)"
                .to_string()
        })?;
    let mut urls = vec![primary];
    if let Some(s) =
        env_secondary.or_else(|| LEASE_AUTH_SECONDARY.lock().ok().and_then(|g| g.clone()))
    {
        urls.push(s);
    }
    if let Some(t) =
        env_tertiary.or_else(|| LEASE_AUTH_TERTIARY.lock().ok().and_then(|g| g.clone()))
    {
        urls.push(t);
    }
    Ok(urls)
}

fn host_port_of(url: &str) -> &str {
    url.strip_prefix("tcp://").unwrap_or(url)
}

fn lease_auth_rpc_one(url: &str, payload: &[u8]) -> Result<Vec<u8>, String> {
    let host_port = host_port_of(url);
    let addr: SocketAddr = host_port
        .parse()
        .map_err(|e| format!("lease auth bad addr {host_port}: {e}"))?;
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(200))
        .map_err(|e| format!("lease auth connect failed: {e}"))?;
    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
    tcp_write_frame(&mut stream, payload)?;
    tcp_read_frame(&mut stream)
}

fn lease_auth_rpc(payload: &[u8]) -> Result<Vec<u8>, String> {
    let urls = authority_url_list()?;
    let mut errors: Vec<String> = Vec::new();
    let is_mutate =
        payload.starts_with(b"GRANT\0") || payload.starts_with(b"RENEW\0") || payload == b"RESET\0";
    for url in urls {
        match lease_auth_rpc_one(&url, payload) {
            Ok(resp) => {
                if is_mutate {
                    if let Some(msg) = resp.strip_prefix(b"ERR\0") {
                        let m = String::from_utf8_lossy(msg);
                        if m.contains("not leader") || m.contains("no quorum") {
                            errors.push(format!("{url}: {m}"));
                            continue;
                        }
                    }
                }
                return Ok(resp);
            }
            Err(e) => errors.push(format!("{url}: {e}")),
        }
    }
    Err(format!(
        "lease auth all authorities failed ({})",
        errors.join("; ")
    ))
}

fn farm_lease_query_deadline() -> Result<Option<u64>, String> {
    farm_lease_query_tenant(0)
}

fn farm_lease_query_tenant(tenant_id: u32) -> Result<Option<u64>, String> {
    let payload = lease_auth_rpc(&encode_query(tenant_id))?;
    if let Some(msg) = payload.strip_prefix(b"ERR\0") {
        return Err(String::from_utf8_lossy(msg).into_owned());
    }
    parse_ok_deadline(&payload)
}

fn spawn_authority_listener(
    state: Arc<Mutex<AuthorityState>>,
    stop: Arc<AtomicBool>,
    max_conns: usize,
    accept_deadline: Duration,
    // When set, mutate ops (GRANT/RENEW/RESET) are replicated to this peer via SYNC.
    replicate_to: Option<String>,
) -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("farm lease auth bind");
    listener
        .set_nonblocking(true)
        .expect("farm lease auth nonblocking");
    let addr = listener.local_addr().expect("farm lease auth local_addr");
    let handle = std::thread::spawn(move || {
        let deadline = Instant::now() + accept_deadline;
        let mut accepted = 0usize;
        while accepted < max_conns && Instant::now() < deadline && !stop.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    accepted += 1;
                    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
                    let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
                    let _ = (|| -> Result<(), String> {
                        let req = tcp_read_frame(&mut stream)?;
                        let (resp, mutated) = {
                            let mut guard = state
                                .lock()
                                .map_err(|_| "lease auth state poisoned".to_string())?;
                            handle_authority_request(&mut guard, &req)
                        };
                        if let (Some(peer), Some(mutation)) = (replicate_to.as_ref(), mutated) {
                            sync_lease_to_peer(peer, mutation);
                        }
                        tcp_write_frame(&mut stream, &resp)
                    })();
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5));
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => {
                    std::thread::sleep(Duration::from_millis(5));
                }
            }
        }
    });
    (addr, handle)
}

/// Spawn a TCP lease authority; configure clients with [`farm_lease_auth_configure`].
///
/// Nonblocking accept + short deadline so `JoinHandle::join` cannot hang (P105/P106 lesson).
pub fn spawn_farm_lease_authority(max_conns: usize) -> (SocketAddr, JoinHandle<()>) {
    let state = Arc::new(Mutex::new(AuthorityState::default()));
    let stop = Arc::new(AtomicBool::new(false));
    spawn_authority_listener(state, stop, max_conns, Duration::from_secs(5), None)
}

/// Dual-node HA lease cluster: independent state per node + SYNC replication.
pub struct FarmLeaseHaCluster {
    primary_addr: SocketAddr,
    secondary_addr: SocketAddr,
    primary_stop: Arc<AtomicBool>,
    secondary_stop: Arc<AtomicBool>,
    primary_handle: Option<JoinHandle<()>>,
    secondary_handle: Option<JoinHandle<()>>,
}

impl FarmLeaseHaCluster {
    pub fn primary_addr(&self) -> SocketAddr {
        self.primary_addr
    }

    pub fn secondary_addr(&self) -> SocketAddr {
        self.secondary_addr
    }

    /// Stop primary accept loop (failover to secondary).
    pub fn stop_primary(&mut self) {
        self.primary_stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.primary_handle.take() {
            let _ = h.join();
        }
    }

    /// Join remaining authority threads (secondary + primary if still running).
    pub fn join(mut self) {
        self.primary_stop.store(true, Ordering::Relaxed);
        self.secondary_stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.primary_handle.take() {
            let _ = h.join();
        }
        if let Some(h) = self.secondary_handle.take() {
            let _ = h.join();
        }
    }
}

/// Spawn primary + secondary with **independent** `AuthorityState` (no shared Mutex).
///
/// Primary replicates GRANT/RENEW/RESET to secondary via `SYNC\0` so failover keeps
/// the lease epoch without sharing one process Mutex.
///
/// `max_conns` caps accepts per node; short deadlines avoid hung joins.
pub fn spawn_farm_lease_ha(max_conns: usize) -> FarmLeaseHaCluster {
    // Independent AuthorityState per node — GA forbids Arc<Mutex> sharing between nodes.
    let primary_state = Arc::new(Mutex::new(AuthorityState::default()));
    let secondary_state = Arc::new(Mutex::new(AuthorityState::default()));
    let primary_stop = Arc::new(AtomicBool::new(false));
    let secondary_stop = Arc::new(AtomicBool::new(false));

    // Secondary first so primary can replicate to a known peer URL.
    let (secondary_addr, secondary_handle) = spawn_authority_listener(
        secondary_state,
        secondary_stop.clone(),
        max_conns,
        Duration::from_secs(5),
        None,
    );
    let secondary_url = format!("tcp://{secondary_addr}");
    let (primary_addr, primary_handle) = spawn_authority_listener(
        primary_state,
        primary_stop.clone(),
        max_conns,
        Duration::from_secs(5),
        Some(secondary_url),
    );
    FarmLeaseHaCluster {
        primary_addr,
        secondary_addr,
        primary_stop,
        secondary_stop,
        primary_handle: Some(primary_handle),
        secondary_handle: Some(secondary_handle),
    }
}

/// Acquire or extend lease via authority GRANT/RENEW (TTL seconds).
pub fn farm_lease_acquire(ttl_secs: u32) -> Result<(), String> {
    farm_lease_acquire_tenant(0, ttl_secs)
}

/// Phase 111 / 114: acquire lease scoped to `tenant_id` (uses default token).
pub fn farm_lease_acquire_tenant(tenant_id: u32, ttl_secs: u32) -> Result<(), String> {
    farm_lease_acquire_tenant_token(tenant_id, FARM_LEASE_DEFAULT_TOKEN, ttl_secs)
}

/// Phase 114: anonymous legacy GRANT for negative tests (rejected on all GA paths).
pub fn farm_lease_legacy_grant_tenant(tenant_id: u32, ttl_secs: u32) -> Result<(), String> {
    let req = encode_grant(tenant_id, ttl_secs);
    let payload = lease_auth_rpc(&req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    parse_ok_deadline(&payload).map(|_| ())
}

/// Phase 105: TCP farm / remote cache require a live authority grant (fail-closed).
/// Phase 107+: queries failover to secondary when primary is unreachable.
pub fn farm_lease_require_active() -> Result<(), String> {
    match farm_lease_query_deadline() {
        Ok(Some(deadline)) if now_ms() <= deadline => Ok(()),
        Ok(Some(_)) => Err("farm lease expired".into()),
        Ok(None) => Err("farm lease not granted".into()),
        Err(e) => Err(e),
    }
}

/// True when TCP farm paths must hard-reject (missing auth, query failure, no grant, or past TTL).
pub fn farm_lease_expired() -> bool {
    farm_lease_require_active().is_err()
}

/// Clear authority lease (tests).
pub fn farm_lease_reset() {
    let _ = lease_auth_rpc(b"RESET\0");
}

/// Phase 104 gate: cross-node lease authority contract string.
pub fn farm_lease_auth_fingerprint() -> &'static str {
    "farm-lease-auth-v1:cross-node-ttl"
}

/// Phase 105 gate: missing authority or failed query is ERR (not fail-open).
pub fn farm_lease_failclosed_fingerprint() -> &'static str {
    "farm-lease-failclosed-v1:no-auth-is-err"
}

/// Phase 106 gate: farm TCP node GET/PUT enforce authority TTL (not client-only).
pub fn farm_lease_server_fingerprint() -> &'static str {
    "farm-lease-server-v1:node-enforces-ttl"
}

/// Phase 108 gate: process-isolated dual authority (independent state + SYNC).
/// Same-process thread+SYNC — **not** Phase 109 HA v3 GA (`farm-lease-ha-v3:os-process`).
pub fn farm_lease_ha_fingerprint() -> &'static str {
    "farm-lease-ha-v2:process-isolated"
}

/// Phase 109 gate: OS-process primary/secondary via `std::process::Command`.
pub fn farm_lease_ha_v3_fingerprint() -> &'static str {
    "farm-lease-ha-v3:os-process"
}

/// Phase 109 gate: 3-node Raft quorum MVP (GRANT needs majority).
pub fn farm_lease_raft_fingerprint() -> &'static str {
    "farm-lease-raft-v1:quorum-mvp"
}

/// Phase 110 gate: Raft v2 replicated log + WAL fsync + commitIndex catch-up.
pub fn farm_lease_raft_v2_fingerprint() -> &'static str {
    "farm-lease-raft-v2:log-wal"
}

/// Phase 110 gate: dynamic membership (AddLearner / Remove) with quorum updates.
pub fn farm_lease_member_fingerprint() -> &'static str {
    "farm-lease-member-v1:dynamic-mvp"
}

/// Phase 111 gate: configurable host:port Raft members (not hardcoded loopback ports GA).
pub fn farm_lease_multihost_fingerprint() -> &'static str {
    "farm-lease-multihost-v1:config-members"
}

/// Phase 111 gate: tenant-scoped GRANT/QUERY isolation MVP.
pub fn farm_lease_tenant_fingerprint() -> &'static str {
    "farm-lease-tenant-v1:isolation-mvp"
}

/// Phase 111 gate: WAL record CRC; corrupt replay rejected.
pub fn farm_lease_wal_crc_fingerprint() -> &'static str {
    "farm-lease-wal-crc-v1:reject-corrupt"
}

/// Phase 112 gate: runtime host:port member config (env/file JSON — not compile-time binds GA).
pub fn farm_lease_runtime_fingerprint() -> &'static str {
    "farm-lease-runtime-v1:config-members"
}

/// Phase 112 gate: tenant-scoped GRANT/QUERY require matching token on runtime GA path.
pub fn farm_lease_tenant_v2_fingerprint() -> &'static str {
    "farm-lease-tenant-v2:token-mvp"
}

/// Phase 112 gate: WAL snapshot checkpoint + incremental replay on restart.
pub fn farm_lease_wal_snap_fingerprint() -> &'static str {
    "farm-lease-wal-snap-v1:checkpoint-replay"
}

/// Phase 113 gate: production/default raft GRANT rejects legacy anonymous GRANT (token default).
pub fn farm_lease_tenant_v3_fingerprint() -> &'static str {
    "farm-lease-tenant-v3:token-default"
}

/// Phase 113 gate: checkpoint snapshot omits discardable log; restart = snap + incremental.
pub fn farm_lease_wal_compact_fingerprint() -> &'static str {
    "farm-lease-wal-compact-v1:snap-truncate-log"
}

/// Phase 113 gate: variable member list (`Vec`, len ≥ 3 — not fixed `[String;3]` GA).
pub fn farm_lease_members_v2_fingerprint() -> &'static str {
    "farm-lease-members-v2:dyn-list"
}

/// Phase 114 gate: all GRANT paths (HA + authority + Raft) require tenant token.
pub fn farm_lease_tenant_v4_fingerprint() -> &'static str {
    "farm-lease-tenant-v4:token-all-paths-multidc"
}

/// Phase 118 gate: multi-DC with ≥2 independent OS processes (not same-process multi-bind GA).
pub fn farm_lease_multidc_fingerprint() -> &'static str {
    "farm-lease-multidc-v5:os-dual-process"
}

/// Phase 119 gate: consensus-sync + ≥2 follower MEMBER views match leader.
pub fn farm_lease_registry_fingerprint() -> &'static str {
    "farm-lease-registry-v5:multi-follower-sync"
}

/// Phase 125 gate fingerprint: PKCS#11 session PIN/slot persist across rotate (stub path; not hardware GA).
pub fn farm_lease_pkcs11_bridge_fingerprint() -> &'static str {
    "farm-lease-token-v8:session-pin-persist"
}

/// Phase 125 gate alias: PKCS#11 session PIN persist via `.so`/`.dylib` adapter (not hardware GA).
pub fn farm_lease_pkcs11_so_fingerprint() -> &'static str {
    farm_lease_pkcs11_bridge_fingerprint()
}

/// Phase 125 session-path alias (OpenSession/Login + PIN persist stub; not hardware GA).
pub fn farm_lease_pkcs11_session_fingerprint() -> &'static str {
    farm_lease_pkcs11_bridge_fingerprint()
}

/// Phase 125 session PIN persist alias (reuses OpenSession/Login state across rotate; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_persist_fingerprint() -> &'static str {
    farm_lease_pkcs11_bridge_fingerprint()
}

/// Phase 126 gate fingerprint: PKCS#11 `C_Sign` on rotate (stub `.so`/mock; not hardware GA).
pub fn farm_lease_pkcs11_sign_mvp_fingerprint() -> &'static str {
    "farm-lease-token-v9:pkcs11-sign-mvp"
}

/// Phase 127 gate fingerprint: PKCS#11 Logout/CloseSession on explicit close / teardown (stub; not hardware GA).
pub fn farm_lease_pkcs11_session_close_fingerprint() -> &'static str {
    "farm-lease-token-v10:session-close-mvp"
}

/// Phase 128 gate fingerprint: PKCS#11 session HSM-ref bind via `C_GetAttributeValue` (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_ref_bound_fingerprint() -> &'static str {
    "farm-lease-token-v11:hsm-ref-bound-mvp"
}

/// Doctor/capability export for Phase 128 HSM-ref bind (not hardware GA).
pub fn farm_lease_hsm_ref_bound_doctor_export() -> &'static str {
    "hsm-ref-bound"
}

/// Phase 129 gate fingerprint: PKCS#11 `C_Sign` bound to validated hsm-ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_bound_fingerprint() -> &'static str {
    "farm-lease-token-v12:hsm-sign-bound-mvp"
}

/// Doctor/capability export for Phase 129 HSM-sign bind (not hardware GA).
pub fn farm_lease_hsm_sign_bound_doctor_export() -> &'static str {
    "hsm-sign-bound"
}

/// Phase 130 gate fingerprint: PKCS#11 `C_Verify` after bound `C_Sign` for same hsm-ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_verify_fingerprint() -> &'static str {
    "farm-lease-token-v13:hsm-sign-verify-mvp"
}

/// Doctor/capability export for Phase 130 HSM sign→verify (not hardware GA).
pub fn farm_lease_hsm_sign_verify_doctor_export() -> &'static str {
    "hsm-sign-verify"
}

/// Phase 131 gate fingerprint: PKCS#11 `C_Digest` bound to validated hsm-ref; Sign/Verify same digest+ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_bound_fingerprint() -> &'static str {
    "farm-lease-token-v14:hsm-digest-bound-mvp"
}

/// Doctor/capability export for Phase 131 HSM digest-bound (not hardware GA).
pub fn farm_lease_hsm_digest_bound_doctor_export() -> &'static str {
    "hsm-digest-bound"
}

/// Phase 132 gate fingerprint: PKCS#11 `C_DigestInit`/`Update`/`Final` streaming bound to hsm-ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_streaming_fingerprint() -> &'static str {
    "farm-lease-token-v15:hsm-digest-streaming-mvp"
}

/// Doctor/capability export for Phase 132 HSM digest streaming (not hardware GA).
pub fn farm_lease_hsm_digest_streaming_doctor_export() -> &'static str {
    "hsm-digest-streaming"
}

/// Phase 133 gate fingerprint: PKCS#11 multi-slot (≥2); rotate/digest/sign bound to chosen slot (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_multi_slot_fingerprint() -> &'static str {
    "farm-lease-token-v16:hsm-multi-slot-mvp"
}

/// Doctor/capability export for Phase 133 HSM multi-slot (not hardware GA).
pub fn farm_lease_hsm_multi_slot_doctor_export() -> &'static str {
    "hsm-multi-slot"
}

/// Phase 134 gate fingerprint: PKCS#11 session PIN persist/check on stub HSM path (not hardware GA).
pub fn farm_lease_pkcs11_hsm_session_pin_fingerprint() -> &'static str {
    "farm-lease-token-v17:hsm-session-pin-mvp"
}

/// Doctor/capability export for Phase 134 HSM session PIN (not hardware GA).
pub fn farm_lease_hsm_session_pin_doctor_export() -> &'static str {
    "hsm-session-pin"
}

/// Phase 135 gate fingerprint: PKCS#11 key label bind/check on stub HSM path (not hardware GA).
pub fn farm_lease_pkcs11_hsm_key_label_fingerprint() -> &'static str {
    "farm-lease-token-v18:hsm-key-label-mvp"
}

/// Doctor/capability export for Phase 135 HSM key label (not hardware GA).
pub fn farm_lease_hsm_key_label_doctor_export() -> &'static str {
    "hsm-key-label"
}

/// Phase 136 gate fingerprint: PKCS#11 CKA_ID-shaped key id bind/check on stub HSM path (not hardware GA).
pub fn farm_lease_pkcs11_hsm_key_id_fingerprint() -> &'static str {
    "farm-lease-token-v19:hsm-key-id-mvp"
}

/// Doctor/capability export for Phase 136 HSM key id (CKA_ID-shaped stub; not hardware GA).
pub fn farm_lease_hsm_key_id_doctor_export() -> &'static str {
    "hsm-key-id"
}

/// Phase 125 gate: tenant token rotate requires PKCS#11 session adapter sign ack (mock, `.so`, or file-slot; PIN/slot persist across rotate; not hardware GA).
pub fn farm_lease_token_fingerprint() -> &'static str {
    farm_lease_pkcs11_bridge_fingerprint()
}

const FARM_PKCS11_BRIDGE_MOCK_REF: &str = "pkcs11-bridge-mock-v1";
const FARM_PKCS11_HSM_REF_BOUND_MOCK: &str = "pkcs11-hsm-ref-bound-mock-v1";
const FARM_PKCS11_SESSION_MOCK_HANDLE: u64 = 0x7001_0001;
const ENV_FARM_PKCS11_BRIDGE: &str = "SYX_FARM_PKCS11_BRIDGE";
/// Path to a PKCS#11 shared library (`.so`/`.dylib` stub with symbol table; not hardware GA).
pub const ENV_FARM_PKCS11_SO: &str = "SYX_FARM_PKCS11_SO";
/// Path to a file containing the PKCS#11 slot label (file-slot adapter; not hardware GA).
pub const ENV_FARM_PKCS11_SLOT_FILE: &str = "SYX_FARM_PKCS11_SLOT_FILE";
/// Path to persisted session PIN for stub HSM check (Phase 134; not hardware GA).
pub const ENV_FARM_PKCS11_PIN_FILE: &str = "SYX_FARM_PKCS11_PIN_FILE";
/// Path to persisted PKCS#11 key label for stub HSM bind (Phase 135; not hardware GA).
pub const ENV_FARM_PKCS11_KEY_LABEL_FILE: &str = "SYX_FARM_PKCS11_KEY_LABEL_FILE";
/// Path to persisted PKCS#11 CKA_ID-shaped key id for stub HSM bind (Phase 136; not hardware GA).
pub const ENV_FARM_PKCS11_KEY_ID_FILE: &str = "SYX_FARM_PKCS11_KEY_ID_FILE";
const ENV_FARM_KMS_BRIDGE: &str = "SYX_FARM_KMS_BRIDGE";

/// Required PKCS#11 entry symbols for `.so` session MVP (OpenSession/Login stub path; not hardware GA).
pub const FARM_PKCS11_SO_SYMBOL_TABLE: &[&str] = &[
    "C_GetFunctionList",
    "C_Initialize",
    "C_Finalize",
    "C_GetSlotList",
    "C_OpenSession",
    "C_Login",
    "C_Logout",
    "C_CloseSession",
    "C_Sign",
    "C_Verify",
    "C_Digest",
    "C_DigestInit",
    "C_DigestUpdate",
    "C_DigestFinal",
    "C_GetAttributeValue",
    "syx_pkcs11_slot_label",
];

/// PKCS#11-style token bridge trait (mock + file-slot adapters; not hardware GA).
pub trait FarmPkcs11Bridge {
    fn adapter_name(&self) -> &'static str;
    fn sign_rotate(&self, tenant_id: u32, old_token: u32, new_token: u32)
        -> Result<String, String>;
    fn validate_wire(&self, wire_ref: &str) -> Result<(), String>;
}

struct FarmPkcs11MockBridge;

impl FarmPkcs11Bridge for FarmPkcs11MockBridge {
    fn adapter_name(&self) -> &'static str {
        "mock"
    }

    fn sign_rotate(
        &self,
        tenant_id: u32,
        old_token: u32,
        new_token: u32,
    ) -> Result<String, String> {
        farm_lease_pkcs11_session_pin_reuse_on_rotate()?;
        farm_lease_pkcs11_mock_sign_ack(tenant_id, old_token, new_token)?;
        Ok(FARM_PKCS11_BRIDGE_MOCK_REF.to_string())
    }

    fn validate_wire(&self, wire_ref: &str) -> Result<(), String> {
        if wire_ref == FARM_PKCS11_BRIDGE_MOCK_REF {
            Ok(())
        } else {
            Err(format!(
                "pkcs11-bridge mock adapter expected {FARM_PKCS11_BRIDGE_MOCK_REF}, got {wire_ref}"
            ))
        }
    }
}

struct FarmPkcs11FileSlotBridge {
    slot_file: String,
}

struct FarmPkcs11SoBridge {
    slot_label: String,
    session_handle: u64,
    slot_id: u64,
    logged_in: bool,
    pin_persisted: bool,
    _library: libloading::Library,
}

static FARM_PKCS11_SESSION_ROTATE_COUNT: AtomicU32 = AtomicU32::new(0);
static FARM_PKCS11_SESSION_ACTIVE: AtomicBool = AtomicBool::new(false);
static FARM_PKCS11_SESSION_CLOSE_COUNT: AtomicU32 = AtomicU32::new(0);
static FARM_PKCS11_PIN_PERSISTED: AtomicBool = AtomicBool::new(false);
static FARM_PKCS11_SESSION_PIN_VALUE: Mutex<Option<String>> = Mutex::new(None);
static FARM_PKCS11_KEY_LABEL_VALUE: Mutex<Option<String>> = Mutex::new(None);
static FARM_PKCS11_KEY_ID_VALUE: Mutex<Option<String>> = Mutex::new(None);
static FARM_PKCS11_SESSION_SLOT_ID: AtomicU32 = AtomicU32::new(0);
static FARM_PKCS11_HSM_REF_BOUND: Mutex<Option<String>> = Mutex::new(None);

/// Phase 133: active multi-slot selection (None = unbound / not selected).
static FARM_PKCS11_MULTI_SLOT_ACTIVE: Mutex<Option<u64>> = Mutex::new(None);
/// Phase 133: per-slot bound HSM-ref map (stub; not hardware GA).
static FARM_PKCS11_MULTI_SLOT_REFS: Mutex<Option<HashMap<u64, String>>> = Mutex::new(None);

fn farm_lease_pkcs11_multi_slot_refs(
) -> std::sync::MutexGuard<'static, Option<HashMap<u64, String>>> {
    FARM_PKCS11_MULTI_SLOT_REFS.lock().unwrap()
}

fn farm_lease_pkcs11_multi_slot_clear() {
    *FARM_PKCS11_MULTI_SLOT_ACTIVE.lock().unwrap() = None;
    *farm_lease_pkcs11_multi_slot_refs() = None;
}

fn farm_lease_pkcs11_multi_slot_ref_for(slot_id: u64) -> String {
    let label_suffix = FARM_PKCS11_KEY_LABEL_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .filter(|l| !l.is_empty())
        .map(|l| format!("-label-{l}"))
        .unwrap_or_default();
    let key_id_suffix = FARM_PKCS11_KEY_ID_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .filter(|l| !l.is_empty())
        .map(|l| format!("-cka-id-{l}"))
        .unwrap_or_default();
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        format!("pkcs11-hsm-multi-slot-{slot_id}{label_suffix}{key_id_suffix}")
    } else {
        format!("pkcs11-so-hsm-multi-slot-{slot_id}{label_suffix}{key_id_suffix}")
    }
}

/// Last bound `C_Digest` material for Phase 131 digest+ref joint Sign/Verify (stub; not hardware GA).
#[derive(Clone, Debug)]
struct FarmPkcs11LastDigest {
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    hsm_ref: String,
    digest: Vec<u8>,
    wire: String,
}

static FARM_PKCS11_LAST_DIGEST: Mutex<Option<FarmPkcs11LastDigest>> = Mutex::new(None);

/// Phase 132 streaming digest accumulator (Init/Update/Final; stub; not hardware GA).
#[derive(Clone, Debug, Default)]
struct FarmPkcs11DigestStream {
    active: bool,
    hsm_ref: String,
    chunks: Vec<u8>,
    forged: bool,
}

static FARM_PKCS11_DIGEST_STREAM: Mutex<FarmPkcs11DigestStream> =
    Mutex::new(FarmPkcs11DigestStream {
        active: false,
        hsm_ref: String::new(),
        chunks: Vec::new(),
        forged: false,
    });

fn farm_lease_pkcs11_clear_digest_stream() {
    let mut slot = FARM_PKCS11_DIGEST_STREAM.lock().unwrap();
    *slot = FarmPkcs11DigestStream {
        active: false,
        hsm_ref: String::new(),
        chunks: Vec::new(),
        forged: false,
    };
}

fn farm_lease_pkcs11_clear_last_digest() {
    *FARM_PKCS11_LAST_DIGEST.lock().unwrap() = None;
    farm_lease_pkcs11_clear_digest_stream();
}

fn farm_lease_pkcs11_store_last_digest(material: FarmPkcs11LastDigest) {
    *FARM_PKCS11_LAST_DIGEST.lock().unwrap() = Some(material);
}

fn farm_lease_pkcs11_digest_hex(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Last bound `C_Sign` material for Phase 130 `C_Verify` round-trip (stub; not hardware GA).
#[derive(Clone, Debug)]
struct FarmPkcs11LastSign {
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    hsm_ref: String,
    signature: Vec<u8>,
    wire: String,
    digest: Option<Vec<u8>>,
    digest_wire: Option<String>,
}

static FARM_PKCS11_LAST_SIGN: Mutex<Option<FarmPkcs11LastSign>> = Mutex::new(None);

fn farm_lease_pkcs11_clear_last_sign() {
    *FARM_PKCS11_LAST_SIGN.lock().unwrap() = None;
}

fn farm_lease_pkcs11_store_last_sign(material: FarmPkcs11LastSign) {
    *FARM_PKCS11_LAST_SIGN.lock().unwrap() = Some(material);
}

struct FarmPkcs11SoBridgeAdapter {
    slot_label: String,
    session_handle: u64,
}

static FARM_PKCS11_SO_LOADED: OnceLock<Result<FarmPkcs11SoBridge, String>> = OnceLock::new();

impl FarmPkcs11Bridge for FarmPkcs11FileSlotBridge {
    fn adapter_name(&self) -> &'static str {
        "file-slot"
    }

    fn sign_rotate(
        &self,
        tenant_id: u32,
        old_token: u32,
        new_token: u32,
    ) -> Result<String, String> {
        farm_lease_pkcs11_session_pin_reuse_on_rotate()?;
        farm_lease_pkcs11_mock_sign_ack(tenant_id, old_token, new_token)?;
        farm_lease_pkcs11_file_slot_read(&self.slot_file)
    }

    fn validate_wire(&self, wire_ref: &str) -> Result<(), String> {
        let slot = farm_lease_pkcs11_file_slot_read(&self.slot_file)?;
        if wire_ref == slot {
            Ok(())
        } else {
            Err(format!(
                "pkcs11-bridge file-slot mismatch: wire {wire_ref} != slot file {slot}"
            ))
        }
    }
}

impl FarmPkcs11Bridge for FarmPkcs11SoBridgeAdapter {
    fn adapter_name(&self) -> &'static str {
        "pkcs11-so"
    }

    fn sign_rotate(
        &self,
        tenant_id: u32,
        old_token: u32,
        new_token: u32,
    ) -> Result<String, String> {
        farm_lease_pkcs11_so_ensure_loaded()?;
        if self.session_handle == 0 {
            return Err(
                "pkcs11-so session handle missing after OpenSession (not hardware GA)".into(),
            );
        }
        farm_lease_pkcs11_session_pin_reuse_on_rotate()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        let (ack, _sig) = farm_lease_pkcs11_so_invoke_sign(
            &bridge._library,
            self.session_handle,
            tenant_id,
            old_token,
            new_token,
        )?;
        if ack == 0 {
            return Err(format!(
                "pkcs11-so C_Sign ack must be non-zero ({})",
                farm_lease_pkcs11_sign_mvp_fingerprint()
            ));
        }
        Ok(self.slot_label.clone())
    }

    fn validate_wire(&self, wire_ref: &str) -> Result<(), String> {
        if wire_ref == self.slot_label {
            Ok(())
        } else {
            Err(format!(
                "pkcs11-so adapter expected {}, got {wire_ref}",
                self.slot_label
            ))
        }
    }
}

fn farm_lease_pkcs11_so_path() -> Option<String> {
    std::env::var(ENV_FARM_PKCS11_SO)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

fn farm_lease_pkcs11_so_probe_symbols(library: &libloading::Library) -> Result<(), String> {
    for sym in FARM_PKCS11_SO_SYMBOL_TABLE {
        unsafe {
            library
                .get::<*const ()>(sym.as_bytes())
                .map_err(|e| format!("pkcs11-so symbol table missing {sym}: {e}"))?;
        }
    }
    Ok(())
}

fn farm_lease_pkcs11_so_open_session_login(
    library: &libloading::Library,
) -> Result<(u64, bool, u64), String> {
    // PKCS#11-shaped stub: C_OpenSession(slot, flags, app, notify, &session) / C_Login(session, userType, pin, pinLen).
    type OpenSessionFn = unsafe extern "C" fn(
        u64,
        u64,
        *mut std::ffi::c_void,
        *mut std::ffi::c_void,
        *mut u64,
    ) -> u32;
    type LoginFn = unsafe extern "C" fn(u64, u64, *const u8, u64) -> u32;
    let open_fn: libloading::Symbol<OpenSessionFn> = unsafe {
        library
            .get(b"C_OpenSession")
            .map_err(|e| format!("pkcs11-so C_OpenSession: {e}"))?
    };
    let login_fn: libloading::Symbol<LoginFn> = unsafe {
        library
            .get(b"C_Login")
            .map_err(|e| format!("pkcs11-so C_Login: {e}"))?
    };
    let slot_id: u64 = 0;
    let mut session: u64 = 0;
    let open_rc = unsafe {
        open_fn(
            slot_id,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut session,
        )
    };
    if open_rc != 0 {
        return Err(format!(
            "pkcs11-so C_OpenSession failed rc={open_rc} (not hardware GA)"
        ));
    }
    if session == 0 {
        return Err("pkcs11-so C_OpenSession returned null session (not hardware GA)".into());
    }
    let pin = b"syx-pkcs11-session-mvp";
    let login_rc = unsafe { login_fn(session, 1, pin.as_ptr(), pin.len() as u64) };
    if login_rc != 0 {
        return Err(format!(
            "pkcs11-so C_Login failed rc={login_rc} (not hardware GA)"
        ));
    }
    Ok((session, true, slot_id))
}

fn farm_lease_pkcs11_so_close_session(
    library: &libloading::Library,
    session: u64,
) -> Result<(), String> {
    type LogoutFn = unsafe extern "C" fn(u64) -> u32;
    type CloseSessionFn = unsafe extern "C" fn(u64) -> u32;
    let logout_fn: libloading::Symbol<LogoutFn> = unsafe {
        library
            .get(b"C_Logout")
            .map_err(|e| format!("pkcs11-so C_Logout: {e}"))?
    };
    let close_fn: libloading::Symbol<CloseSessionFn> = unsafe {
        library
            .get(b"C_CloseSession")
            .map_err(|e| format!("pkcs11-so C_CloseSession: {e}"))?
    };
    let logout_rc = unsafe { logout_fn(session) };
    if logout_rc != 0 {
        return Err(format!(
            "pkcs11-so C_Logout failed rc={logout_rc} ({})",
            farm_lease_pkcs11_session_close_fingerprint()
        ));
    }
    let close_rc = unsafe { close_fn(session) };
    if close_rc != 0 {
        return Err(format!(
            "pkcs11-so C_CloseSession failed rc={close_rc} ({})",
            farm_lease_pkcs11_session_close_fingerprint()
        ));
    }
    Ok(())
}

#[repr(C)]
struct FarmPkcs11Attr {
    attr_type: u64,
    value: *mut u8,
    value_len: u64,
}

fn farm_lease_pkcs11_so_get_attribute_hsm_ref(
    library: &libloading::Library,
    session: u64,
) -> Result<String, String> {
    type GetAttrFn = unsafe extern "C" fn(u64, u64, *mut FarmPkcs11Attr, u64) -> u32;
    let get_fn: libloading::Symbol<GetAttrFn> = unsafe {
        library
            .get(b"C_GetAttributeValue")
            .map_err(|e| format!("pkcs11-so C_GetAttributeValue: {e}"))?
    };
    let mut buf = [0u8; 128];
    let mut attrs = [FarmPkcs11Attr {
        attr_type: 0x0000_0003, // CKA_LABEL-shaped
        value: buf.as_mut_ptr(),
        value_len: buf.len() as u64,
    }];
    let rc = unsafe { get_fn(session, 1, attrs.as_mut_ptr(), 1) };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_GetAttributeValue failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    let len = attrs[0].value_len as usize;
    if len == 0 || len > buf.len() {
        return Err(format!(
            "pkcs11-so C_GetAttributeValue returned invalid hsm-ref len ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    let s = std::str::from_utf8(&buf[..len])
        .map_err(|_| {
            format!(
                "pkcs11-so C_GetAttributeValue hsm-ref not utf-8 ({})",
                farm_lease_pkcs11_hsm_ref_bound_fingerprint()
            )
        })?
        .trim()
        .to_string();
    if s.is_empty() {
        return Err(format!(
            "pkcs11-so C_GetAttributeValue empty hsm-ref ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    Ok(s)
}

fn farm_lease_pkcs11_store_bound_hsm_ref(hsm_ref: String) {
    *FARM_PKCS11_HSM_REF_BOUND.lock().unwrap() = Some(hsm_ref);
}

fn farm_lease_pkcs11_clear_bound_hsm_ref() {
    *FARM_PKCS11_HSM_REF_BOUND.lock().unwrap() = None;
    farm_lease_pkcs11_clear_last_sign();
    farm_lease_pkcs11_clear_last_digest();
    farm_lease_pkcs11_multi_slot_clear();
}

fn farm_lease_pkcs11_rotate_payload(tenant_id: u32, old_token: u32, new_token: u32) -> [u8; 12] {
    let mut payload = [0u8; 12];
    payload[0..4].copy_from_slice(&tenant_id.to_le_bytes());
    payload[4..8].copy_from_slice(&old_token.to_le_bytes());
    payload[8..12].copy_from_slice(&new_token.to_le_bytes());
    payload
}

fn farm_lease_pkcs11_so_invoke_sign(
    library: &libloading::Library,
    session: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<(u64, Vec<u8>), String> {
    type SignFn = unsafe extern "C" fn(u64, *mut u8, u64, *mut u8, *mut u64) -> u32;
    let sign_fn: libloading::Symbol<SignFn> = unsafe {
        library
            .get(b"C_Sign")
            .map_err(|e| format!("pkcs11-so C_Sign: {e}"))?
    };
    let mut payload = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
    let mut signature = [0u8; 64];
    let mut signature_len: u64 = signature.len() as u64;
    let rc = unsafe {
        sign_fn(
            session,
            payload.as_mut_ptr(),
            payload.len() as u64,
            signature.as_mut_ptr(),
            &mut signature_len,
        )
    };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_Sign failed rc={rc} ({})",
            farm_lease_pkcs11_sign_mvp_fingerprint()
        ));
    }
    if signature_len == 0 {
        return Err(format!(
            "pkcs11-so C_Sign returned zero-length ack ({})",
            farm_lease_pkcs11_sign_mvp_fingerprint()
        ));
    }
    let len = signature_len as usize;
    Ok((signature_len, signature[..len].to_vec()))
}

fn farm_lease_pkcs11_so_invoke_verify(
    library: &libloading::Library,
    session: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    signature: &[u8],
) -> Result<(), String> {
    type VerifyFn = unsafe extern "C" fn(u64, *mut u8, u64, *mut u8, u64) -> u32;
    let verify_fn: libloading::Symbol<VerifyFn> = unsafe {
        library
            .get(b"C_Verify")
            .map_err(|e| format!("pkcs11-so C_Verify: {e}"))?
    };
    let mut payload = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
    let mut sig_buf = signature.to_vec();
    let rc = unsafe {
        verify_fn(
            session,
            payload.as_mut_ptr(),
            payload.len() as u64,
            sig_buf.as_mut_ptr(),
            sig_buf.len() as u64,
        )
    };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_Verify failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    Ok(())
}

fn farm_lease_pkcs11_mock_sign_ack(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<(u64, Vec<u8>), String> {
    let payload = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
    if payload.iter().all(|b| *b == 0) {
        return Err("pkcs11 mock C_Sign payload must be non-zero".into());
    }
    // Deterministic mock signature bytes for C_Verify round-trip (not hardware GA).
    let mut sig = Vec::with_capacity(13);
    sig.push(0xAB);
    sig.extend_from_slice(&payload);
    Ok((sig.len() as u64, sig))
}

fn farm_lease_pkcs11_mock_verify_ack(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    signature: &[u8],
) -> Result<(), String> {
    let (_, expected) = farm_lease_pkcs11_mock_sign_ack(tenant_id, old_token, new_token)?;
    if signature != expected.as_slice() {
        return Err(format!(
            "hsm-sign-verify forged signature ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    Ok(())
}

fn farm_lease_pkcs11_so_load(path: &str) -> Result<FarmPkcs11SoBridge, String> {
    if path.trim().is_empty() {
        return Err("pkcs11-so path must be non-empty".into());
    }
    let library = unsafe {
        libloading::Library::new(path)
            .map_err(|e| format!("pkcs11-so dlopen {path}: {e} (not hardware GA)"))?
    };
    farm_lease_pkcs11_so_probe_symbols(&library)?;
    type SlotLabelFn = unsafe extern "C" fn() -> *const std::ffi::c_char;
    let label_fn: libloading::Symbol<SlotLabelFn> = unsafe {
        library
            .get(b"syx_pkcs11_slot_label")
            .map_err(|e| format!("pkcs11-so syx_pkcs11_slot_label: {e}"))?
    };
    let label_ptr = unsafe { label_fn() };
    if label_ptr.is_null() {
        return Err("pkcs11-so syx_pkcs11_slot_label returned null".into());
    }
    let slot_label = unsafe { std::ffi::CStr::from_ptr(label_ptr) }
        .to_string_lossy()
        .trim()
        .to_string();
    if slot_label.is_empty() {
        return Err("pkcs11-so slot label must be non-empty".into());
    }
    let (session_handle, logged_in, slot_id) = farm_lease_pkcs11_so_open_session_login(&library)?;
    let bound_ref = farm_lease_pkcs11_so_get_attribute_hsm_ref(&library, session_handle)?;
    farm_lease_pkcs11_store_bound_hsm_ref(bound_ref);
    FARM_PKCS11_PIN_PERSISTED.store(true, Ordering::Relaxed);
    FARM_PKCS11_SESSION_SLOT_ID.store(
        u32::try_from(slot_id).unwrap_or(u32::MAX),
        Ordering::Relaxed,
    );
    FARM_PKCS11_SESSION_ACTIVE.store(true, Ordering::Relaxed);
    Ok(FarmPkcs11SoBridge {
        slot_label,
        session_handle,
        slot_id,
        logged_in,
        pin_persisted: true,
        _library: library,
    })
}

fn farm_lease_pkcs11_so_ensure_loaded() -> Result<(), String> {
    let path = farm_lease_pkcs11_so_path().ok_or_else(|| {
        "pkcs11-so adapter path missing (SYX_FARM_PKCS11_SO; not hardware GA)".to_string()
    })?;
    FARM_PKCS11_SO_LOADED
        .get_or_init(|| -> Result<FarmPkcs11SoBridge, String> { farm_lease_pkcs11_so_load(&path) })
        .as_ref()
        .map(|_| ())
        .map_err(|e| e.clone())
}

fn farm_lease_pkcs11_so_adapter() -> Result<FarmPkcs11SoBridgeAdapter, String> {
    farm_lease_pkcs11_so_ensure_loaded()?;
    let bridge = FARM_PKCS11_SO_LOADED
        .get()
        .and_then(|r| r.as_ref().ok())
        .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
    Ok(FarmPkcs11SoBridgeAdapter {
        slot_label: bridge.slot_label.clone(),
        session_handle: bridge.session_handle,
    })
}

/// PKCS#11 `.so`/`.dylib` symbol table contract (Phase 124 session MVP; not hardware GA).
pub fn farm_lease_pkcs11_so_symbol_table() -> &'static [&'static str] {
    FARM_PKCS11_SO_SYMBOL_TABLE
}

/// True when a PKCS#11 shared library is configured and dlopen + symbol table probe succeeded.
pub fn farm_lease_pkcs11_so_adapter_loaded() -> bool {
    farm_lease_pkcs11_so_path().is_some() && farm_lease_pkcs11_so_ensure_loaded().is_ok()
}

/// Active PKCS#11 session handle after OpenSession stub (`.so` or mock; not hardware GA).
pub fn farm_lease_pkcs11_session_handle() -> Result<u64, String> {
    let _ = farm_lease_pkcs11_session_fingerprint();
    if farm_lease_pkcs11_use_mock() {
        if !FARM_PKCS11_PIN_PERSISTED.load(Ordering::Relaxed) {
            FARM_PKCS11_PIN_PERSISTED.store(true, Ordering::Relaxed);
        }
        if !FARM_PKCS11_SESSION_ACTIVE.load(Ordering::Relaxed) {
            FARM_PKCS11_SESSION_ACTIVE.store(true, Ordering::Relaxed);
        }
        return Ok(FARM_PKCS11_SESSION_MOCK_HANDLE);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        if !FARM_PKCS11_SESSION_ACTIVE.load(Ordering::Relaxed) {
            return Err(format!(
                "pkcs11-so session closed ({})",
                farm_lease_pkcs11_session_close_fingerprint()
            ));
        }
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so session not loaded".to_string())?;
        if bridge.session_handle == 0 || !bridge.logged_in {
            return Err("pkcs11-so OpenSession/Login incomplete (not hardware GA)".into());
        }
        return Ok(bridge.session_handle);
    }
    if farm_lease_pkcs11_file_slot_path().is_some() {
        // File-slot adapter inherits session MVP via bridge readiness (no native session handle).
        return Ok(FARM_PKCS11_SESSION_MOCK_HANDLE);
    }
    Err("pkcs11 session MVP requires mock, shared library, or file-slot (not hardware GA)".into())
}

/// True when PKCS#11 OpenSession/Login session path MVP is satisfied (mock still ok; not hardware GA).
pub fn farm_lease_pkcs11_session_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_session_fingerprint();
    farm_lease_pkcs11_bridge_mvp_ready() && farm_lease_pkcs11_session_handle().is_ok()
}

/// True when PKCS#11 PIN/slot state is persisted after OpenSession/Login (reused across rotate; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_persisted() -> bool {
    let _ = farm_lease_pkcs11_session_pin_persist_fingerprint();
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        return farm_lease_pkcs11_session_mvp_ready()
            || FARM_PKCS11_PIN_PERSISTED.load(Ordering::Relaxed);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        return FARM_PKCS11_PIN_PERSISTED.load(Ordering::Relaxed)
            && farm_lease_pkcs11_so_ensure_loaded().is_ok();
    }
    false
}

/// True when session PIN persist MVP gate is satisfied (mock, `.so`, or file-slot; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_persist_ready() -> bool {
    farm_lease_pkcs11_session_mvp_ready() && farm_lease_pkcs11_session_pin_persisted()
}

/// Default session PIN persist file unless overridden by `SYX_FARM_PKCS11_PIN_FILE` (Phase 134; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_path() -> Option<String> {
    std::env::var(ENV_FARM_PKCS11_PIN_FILE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

fn farm_lease_pkcs11_session_pin_load_from_file() -> Result<(), String> {
    if FARM_PKCS11_SESSION_PIN_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|p| !p.is_empty())
    {
        return Ok(());
    }
    let Some(path) = farm_lease_pkcs11_session_pin_path() else {
        return Ok(());
    };
    let pin = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "hsm-session-pin load {path}: {e} ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        )
    })?;
    let pin = pin.trim().to_string();
    if pin.is_empty() {
        return Err(format!(
            "hsm-session-pin file {path} is empty ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    *FARM_PKCS11_SESSION_PIN_VALUE.lock().unwrap() = Some(pin);
    FARM_PKCS11_PIN_PERSISTED.store(true, Ordering::Relaxed);
    Ok(())
}

/// Persist session PIN to file + in-memory (stub HSM; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_persist(pin: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    let _ = farm_lease_hsm_session_pin_doctor_export();
    if pin.trim().is_empty() {
        return Err(format!(
            "hsm-session-pin reject empty PIN ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-session-pin requires active session MVP ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    *FARM_PKCS11_SESSION_PIN_VALUE.lock().unwrap() = Some(pin.trim().to_string());
    FARM_PKCS11_PIN_PERSISTED.store(true, Ordering::Relaxed);
    if let Some(path) = farm_lease_pkcs11_session_pin_path() {
        std::fs::write(&path, pin.trim()).map_err(|e| {
            format!(
                "hsm-session-pin persist write {path}: {e} ({})",
                farm_lease_pkcs11_hsm_session_pin_fingerprint()
            )
        })?;
    }
    Ok(())
}

/// Check session PIN against persisted value (stub; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_verify(pin: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    if pin.trim().is_empty() {
        return Err(format!(
            "hsm-session-pin reject empty PIN verify ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    farm_lease_pkcs11_session_pin_load_from_file()?;
    let guard = FARM_PKCS11_SESSION_PIN_VALUE.lock().unwrap();
    let stored = guard.as_deref().ok_or_else(|| {
        format!(
            "hsm-session-pin reject unbound PIN ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        )
    })?;
    if stored != pin.trim() {
        return Err(format!(
            "hsm-session-pin mismatch ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    Ok(())
}

/// True when a session PIN is loaded from memory or persist file (stub; not hardware GA).
pub fn farm_lease_pkcs11_session_pin_loaded() -> bool {
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    let _ = farm_lease_pkcs11_session_pin_load_from_file();
    FARM_PKCS11_SESSION_PIN_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|p| !p.is_empty())
}

fn farm_lease_pkcs11_session_pin_require_for_multi_slot() -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    // Phase 134: enforce only when PIN file env is configured or v17 persist already loaded.
    if farm_lease_pkcs11_session_pin_path().is_none() && !farm_lease_pkcs11_session_pin_loaded() {
        return Ok(());
    }
    farm_lease_pkcs11_session_pin_load_from_file()?;
    if farm_lease_pkcs11_session_pin_path().is_some() && !farm_lease_pkcs11_session_pin_loaded() {
        return Err(format!(
            "hsm-session-pin required but not loaded ({})",
            farm_lease_pkcs11_hsm_session_pin_fingerprint()
        ));
    }
    if farm_lease_pkcs11_session_pin_loaded() {
        let pin = FARM_PKCS11_SESSION_PIN_VALUE
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default();
        return farm_lease_pkcs11_session_pin_verify(&pin);
    }
    if farm_lease_pkcs11_session_pin_persisted() {
        return Ok(());
    }
    Err(format!(
        "hsm-session-pin not persisted for multi-slot op ({})",
        farm_lease_pkcs11_hsm_session_pin_fingerprint()
    ))
}

/// Phase 134 gate: HSM session PIN MVP readiness (multi-slot + persisted PIN; not hardware GA).
pub fn farm_lease_pkcs11_hsm_session_pin_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    let _ = farm_lease_hsm_session_pin_doctor_export();
    farm_lease_pkcs11_hsm_multi_slot_mvp_ready() && farm_lease_pkcs11_session_pin_loaded()
}

/// Default key label persist file unless overridden by `SYX_FARM_PKCS11_KEY_LABEL_FILE` (Phase 135; not hardware GA).
pub fn farm_lease_pkcs11_key_label_path() -> Option<String> {
    std::env::var(ENV_FARM_PKCS11_KEY_LABEL_FILE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

fn farm_lease_pkcs11_key_label_load_from_file() -> Result<(), String> {
    if FARM_PKCS11_KEY_LABEL_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|l| !l.is_empty())
    {
        return Ok(());
    }
    let Some(path) = farm_lease_pkcs11_key_label_path() else {
        return Ok(());
    };
    let label = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "hsm-key-label load {path}: {e} ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        )
    })?;
    let label = label.trim().to_string();
    if label.is_empty() {
        return Err(format!(
            "hsm-key-label file {path} is empty ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    *FARM_PKCS11_KEY_LABEL_VALUE.lock().unwrap() = Some(label);
    Ok(())
}

/// Persist PKCS#11 key label to file + in-memory (stub HSM; not hardware GA).
pub fn farm_lease_pkcs11_key_label_persist(label: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_label_fingerprint();
    let _ = farm_lease_hsm_key_label_doctor_export();
    if label.trim().is_empty() {
        return Err(format!(
            "hsm-key-label reject empty label ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-key-label requires active session MVP ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    *FARM_PKCS11_KEY_LABEL_VALUE.lock().unwrap() = Some(label.trim().to_string());
    if let Some(path) = farm_lease_pkcs11_key_label_path() {
        std::fs::write(&path, label.trim()).map_err(|e| {
            format!(
                "hsm-key-label persist write {path}: {e} ({})",
                farm_lease_pkcs11_hsm_key_label_fingerprint()
            )
        })?;
    }
    Ok(())
}

/// Check key label against persisted value (stub; not hardware GA).
pub fn farm_lease_pkcs11_key_label_verify(label: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_label_fingerprint();
    if label.trim().is_empty() {
        return Err(format!(
            "hsm-key-label reject empty label verify ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    farm_lease_pkcs11_key_label_load_from_file()?;
    let guard = FARM_PKCS11_KEY_LABEL_VALUE.lock().unwrap();
    let stored = guard.as_deref().ok_or_else(|| {
        format!(
            "hsm-key-label reject unbound label ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        )
    })?;
    if stored != label.trim() {
        return Err(format!(
            "hsm-key-label mismatch ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    Ok(())
}

/// True when a key label is loaded from memory or persist file (stub; not hardware GA).
pub fn farm_lease_pkcs11_key_label_loaded() -> bool {
    let _ = farm_lease_pkcs11_hsm_key_label_fingerprint();
    let _ = farm_lease_pkcs11_key_label_load_from_file();
    FARM_PKCS11_KEY_LABEL_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|l| !l.is_empty())
}

fn farm_lease_pkcs11_key_label_require_for_multi_slot() -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_label_fingerprint();
    if farm_lease_pkcs11_key_label_path().is_none() && !farm_lease_pkcs11_key_label_loaded() {
        return Ok(());
    }
    farm_lease_pkcs11_key_label_load_from_file()?;
    if farm_lease_pkcs11_key_label_path().is_some() && !farm_lease_pkcs11_key_label_loaded() {
        return Err(format!(
            "hsm-key-label required but not loaded ({})",
            farm_lease_pkcs11_hsm_key_label_fingerprint()
        ));
    }
    if farm_lease_pkcs11_key_label_loaded() {
        let label = FARM_PKCS11_KEY_LABEL_VALUE
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default();
        return farm_lease_pkcs11_key_label_verify(&label);
    }
    Err(format!(
        "hsm-key-label not persisted for multi-slot op ({})",
        farm_lease_pkcs11_hsm_key_label_fingerprint()
    ))
}

/// Phase 135 gate: HSM key label MVP readiness (multi-slot + persisted label; not hardware GA).
pub fn farm_lease_pkcs11_hsm_key_label_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_key_label_fingerprint();
    let _ = farm_lease_hsm_key_label_doctor_export();
    farm_lease_pkcs11_hsm_multi_slot_mvp_ready() && farm_lease_pkcs11_key_label_loaded()
}

/// Normalize CKA_ID-shaped stub: hex digits optionally colon-separated (not hardware GA).
fn farm_lease_pkcs11_normalize_cka_id(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "hsm-key-id reject empty CKA_ID ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        ));
    }
    let hex: String = trimmed
        .chars()
        .filter(|c| *c != ':')
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if hex.is_empty() || hex.len() % 2 != 0 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "hsm-key-id reject non-CKA_ID shape ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        ));
    }
    Ok(hex)
}

/// Default key id persist file unless overridden by `SYX_FARM_PKCS11_KEY_ID_FILE` (Phase 136; not hardware GA).
pub fn farm_lease_pkcs11_key_id_path() -> Option<String> {
    std::env::var(ENV_FARM_PKCS11_KEY_ID_FILE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

fn farm_lease_pkcs11_key_id_load_from_file() -> Result<(), String> {
    if FARM_PKCS11_KEY_ID_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|l| !l.is_empty())
    {
        return Ok(());
    }
    let Some(path) = farm_lease_pkcs11_key_id_path() else {
        return Ok(());
    };
    let raw = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "hsm-key-id load {path}: {e} ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        )
    })?;
    let id = farm_lease_pkcs11_normalize_cka_id(&raw)?;
    *FARM_PKCS11_KEY_ID_VALUE.lock().unwrap() = Some(id);
    Ok(())
}

/// Persist PKCS#11 CKA_ID-shaped key id to file + in-memory (stub HSM; not hardware GA).
pub fn farm_lease_pkcs11_key_id_persist(key_id: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_id_fingerprint();
    let _ = farm_lease_hsm_key_id_doctor_export();
    let id = farm_lease_pkcs11_normalize_cka_id(key_id)?;
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-key-id requires active session MVP ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        ));
    }
    *FARM_PKCS11_KEY_ID_VALUE.lock().unwrap() = Some(id.clone());
    if let Some(path) = farm_lease_pkcs11_key_id_path() {
        std::fs::write(&path, &id).map_err(|e| {
            format!(
                "hsm-key-id persist write {path}: {e} ({})",
                farm_lease_pkcs11_hsm_key_id_fingerprint()
            )
        })?;
    }
    Ok(())
}

/// Check key id against persisted CKA_ID-shaped value (stub; not hardware GA).
pub fn farm_lease_pkcs11_key_id_verify(key_id: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_id_fingerprint();
    let want = farm_lease_pkcs11_normalize_cka_id(key_id)?;
    farm_lease_pkcs11_key_id_load_from_file()?;
    let guard = FARM_PKCS11_KEY_ID_VALUE.lock().unwrap();
    let stored = guard.as_deref().ok_or_else(|| {
        format!(
            "hsm-key-id reject unbound CKA_ID ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        )
    })?;
    if stored != want {
        return Err(format!(
            "hsm-key-id mismatch ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        ));
    }
    Ok(())
}

/// True when a CKA_ID-shaped key id is loaded from memory or persist file (stub; not hardware GA).
pub fn farm_lease_pkcs11_key_id_loaded() -> bool {
    let _ = farm_lease_pkcs11_hsm_key_id_fingerprint();
    let _ = farm_lease_pkcs11_key_id_load_from_file();
    FARM_PKCS11_KEY_ID_VALUE
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|l| !l.is_empty())
}

fn farm_lease_pkcs11_key_id_require_for_multi_slot() -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_key_id_fingerprint();
    if farm_lease_pkcs11_key_id_path().is_none() && !farm_lease_pkcs11_key_id_loaded() {
        return Ok(());
    }
    farm_lease_pkcs11_key_id_load_from_file()?;
    if farm_lease_pkcs11_key_id_path().is_some() && !farm_lease_pkcs11_key_id_loaded() {
        return Err(format!(
            "hsm-key-id required but not loaded ({})",
            farm_lease_pkcs11_hsm_key_id_fingerprint()
        ));
    }
    if farm_lease_pkcs11_key_id_loaded() {
        let id = FARM_PKCS11_KEY_ID_VALUE
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default();
        return farm_lease_pkcs11_key_id_verify(&id);
    }
    Err(format!(
        "hsm-key-id not persisted for multi-slot op ({})",
        farm_lease_pkcs11_hsm_key_id_fingerprint()
    ))
}

/// Phase 136 gate: HSM key id MVP readiness (key-label + persisted CKA_ID; not hardware GA).
pub fn farm_lease_pkcs11_hsm_key_id_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_key_id_fingerprint();
    let _ = farm_lease_hsm_key_id_doctor_export();
    farm_lease_pkcs11_hsm_key_label_mvp_ready() && farm_lease_pkcs11_key_id_loaded()
}

/// PKCS#11 rotate sign count without re-OpenSession/re-Login (Phase 125 PIN persist; not hardware GA).
pub fn farm_lease_pkcs11_session_rotate_count() -> u32 {
    FARM_PKCS11_SESSION_ROTATE_COUNT.load(Ordering::Relaxed)
}

/// Active PKCS#11 slot id after OpenSession stub (`.so` path; mock/file-slot return 0).
pub fn farm_lease_pkcs11_session_slot_id() -> u64 {
    u64::from(FARM_PKCS11_SESSION_SLOT_ID.load(Ordering::Relaxed))
}

/// Explicit PKCS#11 session teardown: `C_Logout` + `C_CloseSession` (stub `.so`/mock; not hardware GA).
pub fn farm_lease_pkcs11_session_close() -> Result<(), String> {
    let _ = farm_lease_pkcs11_session_close_fingerprint();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err("pkcs11 session close requires active session MVP (not hardware GA)".into());
    }
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        FARM_PKCS11_SESSION_ACTIVE.store(false, Ordering::Relaxed);
        FARM_PKCS11_SESSION_CLOSE_COUNT.fetch_add(1, Ordering::Relaxed);
        farm_lease_pkcs11_clear_bound_hsm_ref();
        return Ok(());
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so session not loaded".to_string())?;
        if bridge.session_handle == 0 {
            return Err("pkcs11-so session handle missing (not hardware GA)".into());
        }
        farm_lease_pkcs11_so_close_session(&bridge._library, bridge.session_handle)?;
        FARM_PKCS11_SESSION_ACTIVE.store(false, Ordering::Relaxed);
        FARM_PKCS11_SESSION_CLOSE_COUNT.fetch_add(1, Ordering::Relaxed);
        farm_lease_pkcs11_clear_bound_hsm_ref();
        return Ok(());
    }
    Err("pkcs11 session close requires mock, shared library, or file-slot (not hardware GA)".into())
}

/// Phase 127 gate: session-close MVP readiness (Logout/CloseSession stub path; not hardware GA).
pub fn farm_lease_pkcs11_session_close_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_session_close_fingerprint();
    farm_lease_pkcs11_session_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_Logout")
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_CloseSession")
}

/// Count of successful session close teardowns (stub path; not hardware GA).
pub fn farm_lease_pkcs11_session_close_count() -> u32 {
    FARM_PKCS11_SESSION_CLOSE_COUNT.load(Ordering::Relaxed)
}

/// Bind HSM-ref to the active PKCS#11 session via `C_GetAttributeValue` (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_ref_bind() -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_ref_bound_fingerprint();
    let _ = farm_lease_hsm_ref_bound_doctor_export();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-ref-bound requires active PKCS#11 session ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        let bound = if farm_lease_pkcs11_file_slot_path().is_some() {
            farm_lease_pkcs11_file_slot_read(
                &farm_lease_pkcs11_file_slot_path().unwrap_or_default(),
            )?
        } else {
            FARM_PKCS11_HSM_REF_BOUND_MOCK.to_string()
        };
        farm_lease_pkcs11_store_bound_hsm_ref(bound.clone());
        return Ok(bound);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        if let Some(existing) = FARM_PKCS11_HSM_REF_BOUND.lock().unwrap().clone() {
            return Ok(existing);
        }
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so session not loaded".to_string())?;
        let bound =
            farm_lease_pkcs11_so_get_attribute_hsm_ref(&bridge._library, bridge.session_handle)?;
        farm_lease_pkcs11_store_bound_hsm_ref(bound.clone());
        return Ok(bound);
    }
    Err(format!(
        "hsm-ref-bound requires mock, shared library, or file-slot ({})",
        farm_lease_pkcs11_hsm_ref_bound_fingerprint()
    ))
}

/// Currently bound HSM-ref after session bind (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_ref_bound() -> Option<String> {
    let _ = farm_lease_pkcs11_hsm_ref_bound_fingerprint();
    FARM_PKCS11_HSM_REF_BOUND.lock().unwrap().clone()
}

/// Validate a wire/lease HSM-ref against the session-bound attribute; reject forged refs.
pub fn farm_lease_pkcs11_hsm_ref_bound_validate(wire_ref: &str) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_ref_bound_fingerprint();
    if wire_ref.trim().is_empty() {
        return Err(format!(
            "hsm-ref-bound reject empty wire ref ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound() {
        Some(b) => b,
        None => farm_lease_pkcs11_hsm_ref_bind()?,
    };
    if wire_ref != bound {
        return Err(format!(
            "hsm-ref-bound forged/mismatch: wire {wire_ref} != bound {bound} ({})",
            farm_lease_pkcs11_hsm_ref_bound_fingerprint()
        ));
    }
    Ok(())
}

/// Phase 128 gate: HSM-ref bind MVP readiness (`C_GetAttributeValue` + reject forged; not hardware GA).
pub fn farm_lease_pkcs11_hsm_ref_bound_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_ref_bound_fingerprint();
    let _ = farm_lease_hsm_ref_bound_doctor_export();
    farm_lease_pkcs11_session_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_GetAttributeValue")
        && farm_lease_pkcs11_hsm_ref_bind().is_ok()
}

/// Phase 129: `C_Sign` only succeeds when session has a validated bound hsm-ref.
/// Forged wire refs and unbound sessions are rejected (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_bound(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_sign_bound_fingerprint();
    let _ = farm_lease_hsm_sign_bound_doctor_export();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-sign-bound reject unbound session ({})",
            farm_lease_pkcs11_hsm_sign_bound_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound() {
        Some(b) if !b.is_empty() => b,
        _ => {
            return Err(format!(
                "hsm-sign-bound reject unbound hsm-ref ({})",
                farm_lease_pkcs11_hsm_sign_bound_fingerprint()
            ));
        }
    };
    if let Some(claimed) = claimed_hsm_ref {
        if claimed.trim().is_empty() || claimed != bound {
            return Err(format!(
                "hsm-sign-bound forged/mismatch: claimed {claimed} != bound {bound} ({})",
                farm_lease_pkcs11_hsm_sign_bound_fingerprint()
            ));
        }
    }
    // Joint verify: C_Sign path must re-validate bound ref before producing ack.
    farm_lease_pkcs11_hsm_ref_bound_validate(&bound).map_err(|e| {
        format!(
            "hsm-sign-bound validate failed: {e} ({})",
            farm_lease_pkcs11_hsm_sign_bound_fingerprint()
        )
    })?;
    let signature = farm_lease_pkcs11_produce_signature(tenant_id, old_token, new_token)?;
    let ack_ref = farm_lease_pkcs11_bridge_sign_rotate(tenant_id, old_token, new_token)?;
    // Signature ack is bound to the validated hsm-ref (not a free-floating sign).
    // Phase 131: when a prior C_Digest exists for same payload+ref, embed digest in the wire.
    let last_digest = FARM_PKCS11_LAST_DIGEST.lock().unwrap().clone();
    let (wire, digest, digest_wire) = if let Some(d) = last_digest.as_ref() {
        if d.hsm_ref != bound
            || d.tenant_id != tenant_id
            || d.old_token != old_token
            || d.new_token != new_token
        {
            return Err(format!(
                "hsm-sign-bound digest mismatch/forge ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
        let digest_hex = farm_lease_pkcs11_digest_hex(&d.digest);
        (
            format!("{ack_ref}|hsm-sign-bound|{bound}|digest:{digest_hex}"),
            Some(d.digest.clone()),
            Some(d.wire.clone()),
        )
    } else {
        (format!("{ack_ref}|hsm-sign-bound|{bound}"), None, None)
    };
    farm_lease_pkcs11_store_last_sign(FarmPkcs11LastSign {
        tenant_id,
        old_token,
        new_token,
        hsm_ref: bound.clone(),
        signature,
        wire: wire.clone(),
        digest,
        digest_wire,
    });
    Ok(wire)
}

fn farm_lease_pkcs11_produce_signature(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<Vec<u8>, String> {
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        let (_, sig) = farm_lease_pkcs11_mock_sign_ack(tenant_id, old_token, new_token)?;
        return Ok(sig);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        if bridge.session_handle == 0 {
            return Err("pkcs11-so session handle missing (not hardware GA)".into());
        }
        let (_, sig) = farm_lease_pkcs11_so_invoke_sign(
            &bridge._library,
            bridge.session_handle,
            tenant_id,
            old_token,
            new_token,
        )?;
        return Ok(sig);
    }
    Err("pkcs11 produce signature requires mock, shared library, or file-slot".into())
}

/// Phase 129 gate: HSM-sign-bound MVP readiness (`C_Sign` + validated hsm-ref; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_bound_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_sign_bound_fingerprint();
    let _ = farm_lease_hsm_sign_bound_doctor_export();
    farm_lease_pkcs11_hsm_ref_bound_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_Sign")
        && farm_lease_pkcs11_hsm_ref_bound().is_some()
}

/// Phase 130: after bound `C_Sign`, `C_Verify` must succeed for the same hsm-ref.
/// Forged signatures, wrong refs, and unbound sessions are rejected (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_verify(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    signature_wire: &str,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_sign_verify_fingerprint();
    let _ = farm_lease_hsm_sign_verify_doctor_export();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-sign-verify reject unbound session ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound() {
        Some(b) if !b.is_empty() => b,
        _ => {
            return Err(format!(
                "hsm-sign-verify reject unbound hsm-ref ({})",
                farm_lease_pkcs11_hsm_sign_verify_fingerprint()
            ));
        }
    };
    if let Some(claimed) = claimed_hsm_ref {
        if claimed.trim().is_empty() || claimed != bound {
            return Err(format!(
                "hsm-sign-verify forged/mismatch: claimed {claimed} != bound {bound} ({})",
                farm_lease_pkcs11_hsm_sign_verify_fingerprint()
            ));
        }
    }
    let last = FARM_PKCS11_LAST_SIGN.lock().unwrap().clone();
    let Some(last) = last else {
        return Err(format!(
            "hsm-sign-verify reject: no prior bound C_Sign ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    };
    if last.hsm_ref != bound {
        return Err(format!(
            "hsm-sign-verify wrong ref: last {} != bound {bound} ({})",
            last.hsm_ref,
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    if signature_wire != last.wire
        || last.tenant_id != tenant_id
        || last.old_token != old_token
        || last.new_token != new_token
    {
        return Err(format!(
            "hsm-sign-verify forged signature wire ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    if !signature_wire.contains(&bound) || !signature_wire.contains("hsm-sign-bound") {
        return Err(format!(
            "hsm-sign-verify forged/mismatch wire missing bound ref ({})",
            farm_lease_pkcs11_hsm_sign_verify_fingerprint()
        ));
    }
    // Phase 131: when prior C_Digest was bound into the sign, verify must carry same digest+ref.
    if let Some(ref digest) = last.digest {
        let digest_hex = farm_lease_pkcs11_digest_hex(digest);
        let digest_marker = format!("digest:{digest_hex}");
        if !signature_wire.contains(&digest_marker) {
            return Err(format!(
                "hsm-sign-verify forged digest mismatch ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
        let Some(ref dw) = last.digest_wire else {
            return Err(format!(
                "hsm-sign-verify forged digest unbound ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        };
        if !dw.contains(&bound) || !dw.contains("hsm-digest-bound") {
            return Err(format!(
                "hsm-sign-verify forged digest wire ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
    }
    farm_lease_pkcs11_invoke_verify(tenant_id, old_token, new_token, &last.signature)?;
    Ok(format!(
        "{}|hsm-sign-verify|{bound}",
        farm_lease_hsm_sign_verify_doctor_export()
    ))
}

fn farm_lease_pkcs11_invoke_verify(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    signature: &[u8],
) -> Result<(), String> {
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        return farm_lease_pkcs11_mock_verify_ack(tenant_id, old_token, new_token, signature);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        if bridge.session_handle == 0 {
            return Err("pkcs11-so session handle missing (not hardware GA)".into());
        }
        return farm_lease_pkcs11_so_invoke_verify(
            &bridge._library,
            bridge.session_handle,
            tenant_id,
            old_token,
            new_token,
            signature,
        );
    }
    Err(format!(
        "hsm-sign-verify requires mock, shared library, or file-slot ({})",
        farm_lease_pkcs11_hsm_sign_verify_fingerprint()
    ))
}

/// Phase 130 gate: HSM sign→verify MVP readiness (`C_Sign`+`C_Verify`+hsm-ref; not hardware GA).
pub fn farm_lease_pkcs11_hsm_sign_verify_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_sign_verify_fingerprint();
    let _ = farm_lease_hsm_sign_verify_doctor_export();
    farm_lease_pkcs11_hsm_sign_bound_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_Verify")
        && farm_lease_pkcs11_hsm_ref_bound().is_some()
}

fn farm_lease_pkcs11_produce_digest(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<Vec<u8>, String> {
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        let payload = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
        let mut digest = Vec::with_capacity(1 + payload.len());
        digest.push(0xD1);
        digest.extend_from_slice(&payload);
        return Ok(digest);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        if bridge.session_handle == 0 {
            return Err("pkcs11-so session handle missing (not hardware GA)".into());
        }
        return farm_lease_pkcs11_so_invoke_digest(
            &bridge._library,
            bridge.session_handle,
            tenant_id,
            old_token,
            new_token,
        );
    }
    Err(format!(
        "hsm-digest-bound requires mock, shared library, or file-slot ({})",
        farm_lease_pkcs11_hsm_digest_bound_fingerprint()
    ))
}

fn farm_lease_pkcs11_so_invoke_digest(
    library: &libloading::Library,
    session: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<Vec<u8>, String> {
    type DigestFn = unsafe extern "C" fn(u64, *mut u8, u64, *mut u8, *mut u64) -> u32;
    let digest_fn: libloading::Symbol<DigestFn> = unsafe {
        library
            .get(b"C_Digest")
            .map_err(|e| format!("pkcs11-so C_Digest: {e}"))?
    };
    let mut payload = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
    let mut digest = [0u8; 64];
    let mut digest_len: u64 = digest.len() as u64;
    let rc = unsafe {
        digest_fn(
            session,
            payload.as_mut_ptr(),
            payload.len() as u64,
            digest.as_mut_ptr(),
            &mut digest_len,
        )
    };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_Digest failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    }
    Ok(digest[..digest_len as usize].to_vec())
}

/// Phase 131: `C_Digest` only succeeds when session has a validated bound hsm-ref.
/// Forged claimed refs and unbound sessions are rejected (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_bound(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_bound_fingerprint();
    let _ = farm_lease_hsm_digest_bound_doctor_export();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-digest-bound reject unbound session ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound() {
        Some(b) if !b.is_empty() => b,
        _ => {
            return Err(format!(
                "hsm-digest-bound reject unbound hsm-ref ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
    };
    if let Some(claimed) = claimed_hsm_ref {
        if claimed.trim().is_empty() || claimed != bound {
            return Err(format!(
                "hsm-digest-bound forged/mismatch: claimed {claimed} != bound {bound} ({})",
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
    }
    farm_lease_pkcs11_hsm_ref_bound_validate(&bound).map_err(|e| {
        format!(
            "hsm-digest-bound validate failed: {e} ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        )
    })?;
    let digest = farm_lease_pkcs11_produce_digest(tenant_id, old_token, new_token)?;
    let digest_hex = farm_lease_pkcs11_digest_hex(&digest);
    let wire = format!("{digest_hex}|hsm-digest-bound|{bound}");
    farm_lease_pkcs11_store_last_digest(FarmPkcs11LastDigest {
        tenant_id,
        old_token,
        new_token,
        hsm_ref: bound.clone(),
        digest,
        wire: wire.clone(),
    });
    Ok(wire)
}

/// Phase 131: after `C_Digest`, `C_Sign` must target the same digest+ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_sign_bound(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_bound_fingerprint();
    let last = FARM_PKCS11_LAST_DIGEST.lock().unwrap().clone();
    let Some(last) = last else {
        return Err(format!(
            "hsm-digest-bound reject: no prior C_Digest ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    };
    if last.tenant_id != tenant_id || last.old_token != old_token || last.new_token != new_token {
        return Err(format!(
            "hsm-digest-bound forged digest payload mismatch ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    }
    if let Some(claimed) = claimed_hsm_ref {
        if claimed != last.hsm_ref {
            return Err(format!(
                "hsm-digest-bound forged/mismatch: claimed {claimed} != digest-ref {} ({})",
                last.hsm_ref,
                farm_lease_pkcs11_hsm_digest_bound_fingerprint()
            ));
        }
    }
    farm_lease_pkcs11_hsm_sign_bound(tenant_id, old_token, new_token, claimed_hsm_ref)
}

/// Phase 131: `C_Verify` after digest-bound `C_Sign` for the same digest+ref.
pub fn farm_lease_pkcs11_hsm_digest_sign_verify(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    signature_wire: &str,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_bound_fingerprint();
    let last_digest = FARM_PKCS11_LAST_DIGEST.lock().unwrap().clone();
    let Some(last_digest) = last_digest else {
        return Err(format!(
            "hsm-digest-bound reject: no prior C_Digest for verify ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    };
    let digest_hex = farm_lease_pkcs11_digest_hex(&last_digest.digest);
    if !signature_wire.contains(&format!("digest:{digest_hex}")) {
        return Err(format!(
            "hsm-digest-bound forged digest on verify ({})",
            farm_lease_pkcs11_hsm_digest_bound_fingerprint()
        ));
    }
    let verified = farm_lease_pkcs11_hsm_sign_verify(
        tenant_id,
        old_token,
        new_token,
        signature_wire,
        claimed_hsm_ref,
    )?;
    Ok(format!(
        "{verified}|{}|{digest_hex}",
        farm_lease_hsm_digest_bound_doctor_export()
    ))
}

/// Phase 131 gate: HSM digest-bound MVP readiness (`C_Digest`+Sign/Verify+hsm-ref; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_bound_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_digest_bound_fingerprint();
    let _ = farm_lease_hsm_digest_bound_doctor_export();
    farm_lease_pkcs11_hsm_sign_verify_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_Digest")
        && farm_lease_pkcs11_hsm_ref_bound().is_some()
}

fn farm_lease_pkcs11_require_bound_ref_for_stream(
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-digest-streaming reject unbound session ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound() {
        Some(b) if !b.is_empty() => b,
        _ => {
            return Err(format!(
                "hsm-digest-streaming reject unbound hsm-ref ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
    };
    if let Some(claimed) = claimed_hsm_ref {
        if claimed.trim().is_empty() || claimed != bound {
            return Err(format!(
                "hsm-digest-streaming forged/mismatch: claimed {claimed} != bound {bound} ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
    }
    farm_lease_pkcs11_hsm_ref_bound_validate(&bound).map_err(|e| {
        format!(
            "hsm-digest-streaming validate failed: {e} ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        )
    })?;
    Ok(bound)
}

fn farm_lease_pkcs11_so_digest_init(
    library: &libloading::Library,
    session: u64,
) -> Result<(), String> {
    type DigestInitFn = unsafe extern "C" fn(u64, *mut std::ffi::c_void) -> u32;
    let init_fn: libloading::Symbol<DigestInitFn> = unsafe {
        library
            .get(b"C_DigestInit")
            .map_err(|e| format!("pkcs11-so C_DigestInit: {e}"))?
    };
    let rc = unsafe { init_fn(session, std::ptr::null_mut()) };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_DigestInit failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    Ok(())
}

fn farm_lease_pkcs11_so_digest_update(
    library: &libloading::Library,
    session: u64,
    part: &[u8],
) -> Result<(), String> {
    type DigestUpdateFn = unsafe extern "C" fn(u64, *mut u8, u64) -> u32;
    let update_fn: libloading::Symbol<DigestUpdateFn> = unsafe {
        library
            .get(b"C_DigestUpdate")
            .map_err(|e| format!("pkcs11-so C_DigestUpdate: {e}"))?
    };
    let mut buf = part.to_vec();
    let rc = unsafe { update_fn(session, buf.as_mut_ptr(), buf.len() as u64) };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_DigestUpdate failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    Ok(())
}

fn farm_lease_pkcs11_so_digest_final(
    library: &libloading::Library,
    session: u64,
) -> Result<Vec<u8>, String> {
    type DigestFinalFn = unsafe extern "C" fn(u64, *mut u8, *mut u64) -> u32;
    let final_fn: libloading::Symbol<DigestFinalFn> = unsafe {
        library
            .get(b"C_DigestFinal")
            .map_err(|e| format!("pkcs11-so C_DigestFinal: {e}"))?
    };
    let mut digest = [0u8; 64];
    let mut digest_len: u64 = digest.len() as u64;
    let rc = unsafe { final_fn(session, digest.as_mut_ptr(), &mut digest_len) };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_DigestFinal failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    Ok(digest[..digest_len as usize].to_vec())
}

/// Phase 132: `C_DigestInit` bound to validated hsm-ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_init(claimed_hsm_ref: Option<&str>) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    let _ = farm_lease_hsm_digest_streaming_doctor_export();
    let bound = farm_lease_pkcs11_require_bound_ref_for_stream(claimed_hsm_ref)?;
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        let mut slot = FARM_PKCS11_DIGEST_STREAM.lock().unwrap();
        *slot = FarmPkcs11DigestStream {
            active: true,
            hsm_ref: bound,
            chunks: Vec::new(),
            forged: false,
        };
        return Ok(());
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        if bridge.session_handle == 0 {
            return Err("pkcs11-so session handle missing (not hardware GA)".into());
        }
        farm_lease_pkcs11_so_digest_init(&bridge._library, bridge.session_handle)?;
        let mut slot = FARM_PKCS11_DIGEST_STREAM.lock().unwrap();
        *slot = FarmPkcs11DigestStream {
            active: true,
            hsm_ref: bound,
            chunks: Vec::new(),
            forged: false,
        };
        return Ok(());
    }
    Err(format!(
        "hsm-digest-streaming requires mock, shared library, or file-slot ({})",
        farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
    ))
}

/// Phase 132: `C_DigestUpdate` mid-stream; forged wrong-ref rejects (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_update(
    part: &[u8],
    claimed_hsm_ref: Option<&str>,
) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    if part.is_empty() {
        return Err(format!(
            "hsm-digest-streaming reject empty update ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    let bound = farm_lease_pkcs11_require_bound_ref_for_stream(claimed_hsm_ref)?;
    {
        let mut slot = FARM_PKCS11_DIGEST_STREAM.lock().unwrap();
        if !slot.active {
            return Err(format!(
                "hsm-digest-streaming reject unbound stream (no DigestInit) ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
        if slot.hsm_ref != bound {
            slot.forged = true;
            return Err(format!(
                "hsm-digest-streaming forged mid-stream: claimed {bound} != stream-ref {} ({})",
                slot.hsm_ref,
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
        if let Some(claimed) = claimed_hsm_ref {
            if claimed != slot.hsm_ref {
                slot.forged = true;
                return Err(format!(
                    "hsm-digest-streaming forged mid-stream wrong ref ({})",
                    farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
                ));
            }
        }
        slot.chunks.extend_from_slice(part);
    }
    if farm_lease_pkcs11_so_path().is_some()
        && !farm_lease_pkcs11_use_mock()
        && farm_lease_pkcs11_file_slot_path().is_none()
    {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        farm_lease_pkcs11_so_digest_update(&bridge._library, bridge.session_handle, part)?;
    }
    Ok(())
}

/// Phase 132: `C_DigestFinal`; multi-chunk digest must equal one-shot `C_Digest` (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_final(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    let _ = farm_lease_hsm_digest_streaming_doctor_export();
    let bound = farm_lease_pkcs11_require_bound_ref_for_stream(claimed_hsm_ref)?;
    let (chunks, stream_ref, forged) = {
        let slot = FARM_PKCS11_DIGEST_STREAM.lock().unwrap();
        if !slot.active {
            return Err(format!(
                "hsm-digest-streaming reject unbound stream (no DigestInit) ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
        if slot.forged {
            return Err(format!(
                "hsm-digest-streaming forged mid-stream reject Final ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
        if slot.hsm_ref != bound {
            return Err(format!(
                "hsm-digest-streaming forged/mismatch Final ({})",
                farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
            ));
        }
        (slot.chunks.clone(), slot.hsm_ref.clone(), slot.forged)
    };
    let _ = (stream_ref, forged);
    let expected = farm_lease_pkcs11_rotate_payload(tenant_id, old_token, new_token);
    if chunks != expected {
        return Err(format!(
            "hsm-digest-streaming forged payload mismatch vs rotate ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    let digest = if farm_lease_pkcs11_so_path().is_some()
        && !farm_lease_pkcs11_use_mock()
        && farm_lease_pkcs11_file_slot_path().is_none()
    {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        farm_lease_pkcs11_so_digest_final(&bridge._library, bridge.session_handle)?
    } else {
        let mut digest = Vec::with_capacity(1 + chunks.len());
        digest.push(0xD1);
        digest.extend_from_slice(&chunks);
        digest
    };
    // Multi-chunk must equal one-shot C_Digest for same payload.
    let oneshot = farm_lease_pkcs11_produce_digest(tenant_id, old_token, new_token)?;
    if digest != oneshot {
        return Err(format!(
            "hsm-digest-streaming multi-chunk != one-shot C_Digest ({})",
            farm_lease_pkcs11_hsm_digest_streaming_fingerprint()
        ));
    }
    let digest_hex = farm_lease_pkcs11_digest_hex(&digest);
    let wire = format!(
        "{digest_hex}|{}|{bound}",
        farm_lease_hsm_digest_streaming_doctor_export()
    );
    farm_lease_pkcs11_store_last_digest(FarmPkcs11LastDigest {
        tenant_id,
        old_token,
        new_token,
        hsm_ref: bound,
        digest,
        wire: wire.clone(),
    });
    farm_lease_pkcs11_clear_digest_stream();
    Ok(wire)
}

/// Phase 132 convenience: Init + multi-chunk Update + Final; equals one-shot digest (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_streaming(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    chunks: &[&[u8]],
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    farm_lease_pkcs11_hsm_digest_init(claimed_hsm_ref)?;
    for part in chunks {
        farm_lease_pkcs11_hsm_digest_update(part, claimed_hsm_ref)?;
    }
    farm_lease_pkcs11_hsm_digest_final(tenant_id, old_token, new_token, claimed_hsm_ref)
}

/// Phase 132 gate: HSM digest-streaming MVP readiness (Init/Update/Final + hsm-ref; not hardware GA).
pub fn farm_lease_pkcs11_hsm_digest_streaming_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_digest_streaming_fingerprint();
    let _ = farm_lease_hsm_digest_streaming_doctor_export();
    farm_lease_pkcs11_hsm_digest_bound_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_DigestInit")
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_DigestUpdate")
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_DigestFinal")
        && farm_lease_pkcs11_hsm_ref_bound().is_some()
}

fn farm_lease_pkcs11_so_get_slot_list(library: &libloading::Library) -> Result<Vec<u64>, String> {
    // PKCS#11: C_GetSlotList(tokenPresent, pSlotList, pulCount)
    type GetSlotListFn = unsafe extern "C" fn(u8, *mut u64, *mut u64) -> u32;
    let get_fn: libloading::Symbol<GetSlotListFn> = unsafe {
        library
            .get(b"C_GetSlotList")
            .map_err(|e| format!("pkcs11-so C_GetSlotList: {e}"))?
    };
    let mut count: u64 = 0;
    let rc = unsafe { get_fn(1, std::ptr::null_mut(), &mut count) };
    if rc != 0 {
        return Err(format!(
            "pkcs11-so C_GetSlotList count failed rc={rc} ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    if count < 2 {
        return Err(format!(
            "pkcs11-so C_GetSlotList requires ≥2 slots, got {count} ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    let mut slots = vec![0u64; count as usize];
    let mut count2 = count;
    let rc2 = unsafe { get_fn(1, slots.as_mut_ptr(), &mut count2) };
    if rc2 != 0 {
        return Err(format!(
            "pkcs11-so C_GetSlotList list failed rc={rc2} ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    slots.truncate(count2 as usize);
    Ok(slots)
}

/// Enumerate PKCS#11 slots (`C_GetSlotList`); mock/file-slot expose ≥2 stub slots (not hardware GA).
pub fn farm_lease_pkcs11_slot_list() -> Result<Vec<u64>, String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_hsm_multi_slot_doctor_export();
    if farm_lease_pkcs11_use_mock() || farm_lease_pkcs11_file_slot_path().is_some() {
        return Ok(vec![0, 1]);
    }
    if farm_lease_pkcs11_so_path().is_some() {
        farm_lease_pkcs11_so_ensure_loaded()?;
        let bridge = FARM_PKCS11_SO_LOADED
            .get()
            .and_then(|r| r.as_ref().ok())
            .ok_or_else(|| "pkcs11-so adapter not loaded".to_string())?;
        return farm_lease_pkcs11_so_get_slot_list(&bridge._library);
    }
    // Default mock multi-slot surface when bridge env unset (tests set mock explicitly).
    Err(format!(
        "hsm-multi-slot requires mock, shared library, or file-slot ({})",
        farm_lease_pkcs11_hsm_multi_slot_fingerprint()
    ))
}

/// Select active PKCS#11 slot for rotate/digest/sign binding (stub; not hardware GA).
pub fn farm_lease_pkcs11_slot_select(slot_id: u64) -> Result<(), String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-multi-slot reject unbound session ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    let slots = farm_lease_pkcs11_slot_list()?;
    if !slots.contains(&slot_id) {
        return Err(format!(
            "hsm-multi-slot wrong-slot: {slot_id} not in {slots:?} ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    *FARM_PKCS11_MULTI_SLOT_ACTIVE.lock().unwrap() = Some(slot_id);
    FARM_PKCS11_SESSION_SLOT_ID.store(slot_id as u32, Ordering::Relaxed);
    Ok(())
}

/// Currently selected multi-slot id (None if unbound/unselected).
pub fn farm_lease_pkcs11_active_slot_id() -> Option<u64> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    *FARM_PKCS11_MULTI_SLOT_ACTIVE.lock().unwrap()
}

/// Bind HSM-ref for a chosen slot; selects the slot and stores per-slot ref (stub; not hardware GA).
pub fn farm_lease_pkcs11_hsm_multi_slot_bind(slot_id: u64) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_hsm_multi_slot_doctor_export();
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    farm_lease_pkcs11_slot_select(slot_id)?;
    farm_lease_pkcs11_session_pin_require_for_multi_slot()?;
    farm_lease_pkcs11_key_label_require_for_multi_slot()?;
    farm_lease_pkcs11_key_id_require_for_multi_slot()?;
    let bound = farm_lease_pkcs11_multi_slot_ref_for(slot_id);
    {
        let mut guard = farm_lease_pkcs11_multi_slot_refs();
        let map = guard.get_or_insert_with(HashMap::new);
        map.insert(slot_id, bound.clone());
    }
    farm_lease_pkcs11_store_bound_hsm_ref(bound.clone());
    Ok(bound)
}

/// Bound HSM-ref for a specific slot, if previously multi-slot-bound.
pub fn farm_lease_pkcs11_hsm_ref_bound_for_slot(slot_id: u64) -> Option<String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    farm_lease_pkcs11_multi_slot_refs()
        .as_ref()
        .and_then(|m| m.get(&slot_id).cloned())
}

fn farm_lease_pkcs11_require_multi_slot(
    slot_id: u64,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(format!(
            "hsm-multi-slot reject unbound session ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    farm_lease_pkcs11_session_pin_require_for_multi_slot()?;
    farm_lease_pkcs11_key_label_require_for_multi_slot()?;
    farm_lease_pkcs11_key_id_require_for_multi_slot()?;
    let active = farm_lease_pkcs11_active_slot_id().ok_or_else(|| {
        format!(
            "hsm-multi-slot reject unbound slot (no select) ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        )
    })?;
    if slot_id != active {
        return Err(format!(
            "hsm-multi-slot wrong-slot: claimed {slot_id} != active {active} ({})",
            farm_lease_pkcs11_hsm_multi_slot_fingerprint()
        ));
    }
    let bound = match farm_lease_pkcs11_hsm_ref_bound_for_slot(slot_id) {
        Some(b) if !b.is_empty() => b,
        _ => {
            return Err(format!(
                "hsm-multi-slot reject unbound hsm-ref for slot {slot_id} ({})",
                farm_lease_pkcs11_hsm_multi_slot_fingerprint()
            ));
        }
    };
    // Cross-slot forge: claimed ref belongs to a different slot.
    if let Some(claimed) = claimed_hsm_ref {
        if claimed.trim().is_empty() || claimed != bound {
            let cross = farm_lease_pkcs11_multi_slot_refs()
                .as_ref()
                .map(|m| m.iter().any(|(sid, r)| *sid != slot_id && r == claimed))
                .unwrap_or(false);
            if cross {
                return Err(format!(
                    "hsm-multi-slot cross-slot forge: claimed {claimed} for slot {slot_id} ({})",
                    farm_lease_pkcs11_hsm_multi_slot_fingerprint()
                ));
            }
            return Err(format!(
                "hsm-multi-slot forged/mismatch: claimed {claimed} != bound {bound} ({})",
                farm_lease_pkcs11_hsm_multi_slot_fingerprint()
            ));
        }
    }
    farm_lease_pkcs11_store_bound_hsm_ref(bound.clone());
    Ok(bound)
}

/// Phase 133: rotate/sign bound to chosen slot; wrong-slot / unbound / cross-slot forge reject.
pub fn farm_lease_pkcs11_hsm_multi_slot_sign_bound(
    slot_id: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_hsm_multi_slot_doctor_export();
    let bound = farm_lease_pkcs11_require_multi_slot(slot_id, claimed_hsm_ref)?;
    let wire = farm_lease_pkcs11_hsm_sign_bound(tenant_id, old_token, new_token, Some(&bound))?;
    Ok(format!(
        "{wire}|{}|slot:{slot_id}|{bound}",
        farm_lease_hsm_multi_slot_doctor_export()
    ))
}

/// Phase 133: digest streaming bound to chosen slot; wrong-slot / unbound / cross-slot forge reject.
pub fn farm_lease_pkcs11_hsm_multi_slot_digest_streaming(
    slot_id: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    chunks: &[&[u8]],
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_hsm_multi_slot_doctor_export();
    let bound = farm_lease_pkcs11_require_multi_slot(slot_id, claimed_hsm_ref)?;
    let wire = farm_lease_pkcs11_hsm_digest_streaming(
        tenant_id,
        old_token,
        new_token,
        chunks,
        Some(&bound),
    )?;
    Ok(format!(
        "{wire}|{}|slot:{slot_id}|{bound}",
        farm_lease_hsm_multi_slot_doctor_export()
    ))
}

/// Phase 133: rotate via multi-slot sign binding (chosen slot).
pub fn farm_lease_pkcs11_hsm_multi_slot_rotate(
    slot_id: u64,
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
    claimed_hsm_ref: Option<&str>,
) -> Result<String, String> {
    farm_lease_pkcs11_hsm_multi_slot_sign_bound(
        slot_id,
        tenant_id,
        old_token,
        new_token,
        claimed_hsm_ref,
    )
}

/// Phase 133 gate: HSM multi-slot MVP readiness (≥2 slots + select/bind; not hardware GA).
pub fn farm_lease_pkcs11_hsm_multi_slot_mvp_ready() -> bool {
    let _ = farm_lease_pkcs11_hsm_multi_slot_fingerprint();
    let _ = farm_lease_hsm_multi_slot_doctor_export();
    farm_lease_pkcs11_hsm_digest_streaming_mvp_ready()
        && FARM_PKCS11_SO_SYMBOL_TABLE.contains(&"C_GetSlotList")
        && farm_lease_pkcs11_slot_list()
            .map(|s| s.len() >= 2)
            .unwrap_or(false)
        && farm_lease_pkcs11_active_slot_id().is_some()
        && farm_lease_pkcs11_hsm_ref_bound().is_some()
}

fn farm_lease_pkcs11_session_pin_reuse_on_rotate() -> Result<(), String> {
    let _ = farm_lease_pkcs11_session_pin_persist_fingerprint();
    let _ = farm_lease_pkcs11_hsm_session_pin_fingerprint();
    if !farm_lease_pkcs11_session_mvp_ready() {
        return Err(
            "pkcs11 session MVP required before rotate sign (farm-lease-token-v8:session-pin-persist; not hardware GA)".into(),
        );
    }
    FARM_PKCS11_PIN_PERSISTED.store(true, Ordering::Relaxed);
    FARM_PKCS11_SESSION_ROTATE_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

fn farm_lease_pkcs11_file_slot_read(path: &str) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("pkcs11-bridge file-slot path must be non-empty".into());
    }
    let slot = std::fs::read_to_string(path)
        .map_err(|e| format!("pkcs11-bridge file-slot read {path}: {e}"))?
        .trim()
        .to_string();
    if slot.is_empty() {
        return Err(format!("pkcs11-bridge file-slot {path} is empty"));
    }
    Ok(slot)
}

fn farm_lease_pkcs11_use_mock() -> bool {
    std::env::var(ENV_FARM_PKCS11_BRIDGE)
        .map(|v| v == "mock" || v.eq_ignore_ascii_case("mock"))
        .unwrap_or(false)
        || farm_lease_kms_bridge_use_mock()
}

fn farm_lease_pkcs11_file_slot_path() -> Option<String> {
    std::env::var(ENV_FARM_PKCS11_SLOT_FILE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

fn farm_lease_pkcs11_bridge_impl() -> Result<Box<dyn FarmPkcs11Bridge>, String> {
    let _ = farm_lease_pkcs11_bridge_fingerprint();
    if farm_lease_pkcs11_use_mock() {
        return Ok(Box::new(FarmPkcs11MockBridge));
    }
    if farm_lease_pkcs11_so_path().is_some() {
        return Ok(Box::new(farm_lease_pkcs11_so_adapter()?));
    }
    if let Some(path) = farm_lease_pkcs11_file_slot_path() {
        farm_lease_pkcs11_file_slot_read(&path)?;
        return Ok(Box::new(FarmPkcs11FileSlotBridge { slot_file: path }));
    }
    Err(
        "pkcs11-session MVP requires mock, shared library, or file-slot (SYX_FARM_PKCS11_BRIDGE=mock, SYX_FARM_PKCS11_SO, or SYX_FARM_PKCS11_SLOT_FILE; OpenSession/Login stub; not hardware GA)".into(),
    )
}

/// Active PKCS#11 bridge adapter name (`mock`, `pkcs11-so`, or `file-slot`; not hardware GA).
pub fn farm_lease_pkcs11_bridge_adapter_name() -> Result<&'static str, String> {
    Ok(farm_lease_pkcs11_bridge_impl()?.adapter_name())
}

/// True when PKCS#11 adapter MVP gate is satisfied (mock, `.so`, or file-slot; not hardware GA).
pub fn farm_lease_pkcs11_bridge_mvp_ready() -> bool {
    farm_lease_pkcs11_bridge_impl().is_ok()
}

/// Resolve rotate wire ref via PKCS#11 bridge (mock or file-slot; not hardware GA).
pub fn farm_lease_pkcs11_bridge_sign_rotate(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<String, String> {
    let bridge = farm_lease_pkcs11_bridge_impl()?;
    bridge.sign_rotate(tenant_id, old_token, new_token)
}

fn farm_lease_pkcs11_validate_wire(wire_ref: &str) -> Result<(), String> {
    if wire_ref.is_empty() {
        return Err("pkcs11-bridge ref must be non-empty".into());
    }
    farm_lease_pkcs11_bridge_impl()?.validate_wire(wire_ref)
}

fn farm_lease_kms_bridge_use_mock() -> bool {
    std::env::var(ENV_FARM_KMS_BRIDGE)
        .map(|v| v == "mock" || v.eq_ignore_ascii_case("mock"))
        .unwrap_or(false)
}

/// Legacy KMS bridge adapter name (delegates to PKCS#11 bridge; not hardware GA).
pub fn farm_lease_kms_bridge_adapter_name() -> &'static str {
    farm_lease_pkcs11_bridge_adapter_name().unwrap_or("unconfigured")
}

/// Legacy KMS bridge MVP gate (delegates to PKCS#11 bridge).
pub fn farm_lease_kms_bridge_mvp_ready() -> bool {
    farm_lease_pkcs11_bridge_mvp_ready()
}

/// Legacy KMS bridge rotate sign (delegates to PKCS#11 bridge).
pub fn farm_lease_kms_bridge_sign_rotate(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<String, String> {
    farm_lease_pkcs11_bridge_sign_rotate(tenant_id, old_token, new_token)
}

fn farm_lease_kms_bridge_validate_wire(kms_ref: &str) -> Result<(), String> {
    farm_lease_pkcs11_validate_wire(kms_ref)
}

/// Resolve HSM key ref from `SYX_HSM_KEY_REF` (env adapter input; not hardware GA).
pub fn farm_lease_token_hsm_ref_resolve() -> Option<String> {
    std::env::var("SYX_HSM_KEY_REF")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// True when env HSM ref ack is configured (legacy env adapter; not PKCS#11 v5 GA).
pub fn farm_lease_token_hsm_ref_mvp_ready() -> bool {
    farm_lease_token_hsm_ref_resolve().is_some()
}

/// Validate lockfile `hsm_key_ref` against env (same contract as `syx_pkg::enforce_hsm_key_ref`).
pub fn farm_lease_token_hsm_ref_accept_lock(lock_hsm: Option<&str>) -> Result<(), String> {
    let _ = farm_lease_token_fingerprint();
    let Some(lock_ref) = lock_hsm.filter(|s| !s.is_empty()) else {
        return Err("hsm-ref MVP requires non-empty lockfile hsm_key_ref".into());
    };
    let env = std::env::var("SYX_HSM_KEY_REF").unwrap_or_default();
    if env.is_empty() {
        return Err(format!(
            "hsm-ref MVP requires SYX_HSM_KEY_REF env ack for lock ref {lock_ref}"
        ));
    }
    if env != lock_ref {
        return Err(format!(
            "hsm-ref stale: lock requires {lock_ref} but env has {env}"
        ));
    }
    Ok(())
}

fn farm_lease_token_hsm_ref_validate_wire(hsm_ref: &str) -> Result<(), String> {
    if hsm_ref.is_empty() {
        return Err("hsm-ref must be non-empty".into());
    }
    let env = std::env::var("SYX_HSM_KEY_REF").unwrap_or_default();
    if env.is_empty() {
        return Err(format!(
            "hsm-ref MVP requires SYX_HSM_KEY_REF env ack (wire ref {hsm_ref})"
        ));
    }
    if env != hsm_ref {
        return Err(format!("hsm-ref mismatch: wire {hsm_ref} != env {env}"));
    }
    Ok(())
}

fn farm_lease_token_hsm_ref_gate() -> Result<String, String> {
    farm_lease_token_hsm_ref_resolve()
        .ok_or_else(|| "hsm-ref MVP requires non-empty SYX_HSM_KEY_REF (not hardware GA)".into())
}

fn parse_rotate_hsm_payload(rest: &[u8]) -> Result<(u32, u32, u32), String> {
    if rest.len() < 12 {
        return Err("rotate bad payload".into());
    }
    let tenant_id = u32::from_le_bytes(rest[0..4].try_into().unwrap());
    let old_token = u32::from_le_bytes(rest[4..8].try_into().unwrap());
    let new_token = u32::from_le_bytes(rest[8..12].try_into().unwrap());
    if rest.len() == 12 {
        return Err("pkcs11-so adapter ref required for rotate (farm-lease-token-v8:session-pin-persist; inherits farm-lease-token-v7:pkcs11-session-mvp and farm-lease-token-v6:pkcs11-so-adapter)".into());
    }
    if rest.len() < 16 {
        return Err("rotate hsm-ref length missing".into());
    }
    let ref_len = u32::from_le_bytes(rest[12..16].try_into().unwrap()) as usize;
    if rest.len() != 16 + ref_len {
        return Err("rotate hsm-ref truncated".into());
    }
    let hsm_ref = std::str::from_utf8(&rest[16..16 + ref_len])
        .map_err(|_| "rotate hsm-ref not utf-8".to_string())?;
    farm_lease_kms_bridge_validate_wire(hsm_ref)?;
    Ok((tenant_id, old_token, new_token))
}

/// Default HEALTH probe timeout for multi-DC discovery / registry reload.
pub const FARM_LEASE_MULTIDC_PROBE_TIMEOUT_MS: u64 = 150;

/// Probe timeout used by runtime spawn and registry reload.
pub fn farm_lease_multidc_probe_timeout() -> Duration {
    Duration::from_millis(FARM_LEASE_MULTIDC_PROBE_TIMEOUT_MS)
}

/// Region-tagged farm lease member (Phase 114 multi-DC MVP).
///
/// Phase 116+: `host_key` is the logical cross-host identity; `connect` is the HEALTH/RPC probe
/// target. Phase 117 real-connect GA requires ≥2 non-loopback `connect` targets — the legacy
/// `0.0.0.0`→`127.0.0.1` rewrite must **not** satisfy GA (concrete local IP is used instead).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FarmMemberRegion {
    pub bind: String,
    pub region: String,
    pub host_key: String,
    pub connect: String,
}

impl FarmMemberRegion {
    pub fn from_bind_region(bind: impl Into<String>, region: impl Into<String>) -> Self {
        let bind = bind.into();
        let region = region.into();
        let host_key = bind.clone();
        let connect = default_member_connect(&bind, &host_key);
        Self {
            bind,
            region,
            host_key,
            connect,
        }
    }
}

fn host_of_host_port(host_port: &str) -> &str {
    let s = host_port.trim();
    if s.starts_with('[') {
        s.split("]:")
            .next()
            .map(|part| part.trim_start_matches('['))
            .unwrap_or(s)
    } else {
        s.split(':').next().unwrap_or(s)
    }
}

fn port_of_host_port(host_port: &str) -> Option<&str> {
    let s = host_port.trim();
    if s.starts_with('[') {
        s.split("]:").nth(1)
    } else {
        s.rsplit(':').next()
    }
}

/// Resolve a concrete non-loopback local IPv4 (for `0.0.0.0` connect defaults).
/// Never treat `127.0.0.1` rewrite as real-connect GA proof.
pub fn farm_lease_concrete_local_ipv4() -> Option<Ipv4Addr> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    let local = sock.local_addr().ok()?;
    match local.ip() {
        IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_unspecified() => Some(v4),
        _ => None,
    }
}

fn default_member_connect(bind: &str, host_key: &str) -> String {
    let bind_host = host_of_host_port(bind);
    if (bind_host == "0.0.0.0" || bind_host == "::")
        && !farm_lease_bind_host_is_loopback(host_of_host_port(host_key))
    {
        if let Some(port) = port_of_host_port(bind) {
            // Phase 117: concrete local IP when available. The 127 fallback is explicit non-GA.
            if let Some(ip) = farm_lease_concrete_local_ipv4() {
                return format!("{ip}:{port}");
            }
            return format!("127.0.0.1:{port}");
        }
    }
    bind.to_string()
}

/// True when the host portion is loopback (`127.0.0.1`, `::1`, or `localhost`).
pub fn farm_lease_bind_host_is_loopback(host_port: &str) -> bool {
    matches!(
        host_of_host_port(host_port),
        "127.0.0.1" | "::1" | "localhost"
    )
}

/// True when the host portion is an RFC5737 documentation / TEST-NET address.
pub fn farm_lease_host_is_documentation(host_port: &str) -> bool {
    let host = host_of_host_port(host_port);
    let Ok(ip) = host.parse::<IpAddr>() else {
        return false;
    };
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            (o[0] == 192 && o[1] == 0 && o[2] == 2)
                || (o[0] == 198 && o[1] == 51 && o[2] == 100)
                || (o[0] == 203 && o[1] == 0 && o[2] == 113)
        }
        IpAddr::V6(_) => false,
    }
}

/// Phase 116 GA: at least two distinct non-loopback host keys in the member list.
pub fn farm_lease_member_regions_cross_host_ga(members: &[FarmMemberRegion]) -> bool {
    let distinct: HashSet<String> = members
        .iter()
        .filter(|m| !farm_lease_bind_host_is_loopback(&m.host_key))
        .map(|m| host_of_host_port(&m.host_key).to_string())
        .collect();
    distinct.len() >= 2
}

/// Phase 117 GA: ≥2 members whose **connect** targets are non-loopback.
/// TEST-NET hostkey + 127 connect, and `0.0.0.0`→`127.0.0.1` rewrite, must not pass.
pub fn farm_lease_member_regions_real_connect_ga(members: &[FarmMemberRegion]) -> bool {
    members
        .iter()
        .filter(|m| !farm_lease_bind_host_is_loopback(&m.connect))
        .count()
        >= 2
}

/// Phase 118 GA: real-connect plus ban TEST-NET hostkey-only paper proof (doc host_key + loopback connect).
pub fn farm_lease_member_regions_os_dual_process_ga(members: &[FarmMemberRegion]) -> bool {
    if !farm_lease_member_regions_real_connect_ga(members) {
        return false;
    }
    let testnet_hostkey_loopback_connect = members
        .iter()
        .filter(|m| {
            farm_lease_host_is_documentation(&m.host_key)
                && farm_lease_bind_host_is_loopback(&m.connect)
        })
        .count();
    if testnet_hostkey_loopback_connect >= 2 {
        return false;
    }
    let documentation_connect = members
        .iter()
        .filter(|m| farm_lease_host_is_documentation(&m.connect))
        .count();
    if documentation_connect >= 2 {
        return false;
    }
    true
}

/// Phase 118 GA: runtime cluster has ≥2 live OS child processes with distinct PIDs.
pub fn farm_lease_cluster_os_dual_process_ga(cluster: &FarmLeaseRaftCluster) -> bool {
    if !farm_lease_member_regions_os_dual_process_ga(cluster.member_regions()) {
        return false;
    }
    let mut distinct_pids = HashSet::new();
    for child in &cluster.children {
        if let Some(c) = child {
            distinct_pids.insert(c.id());
        }
    }
    distinct_pids.len() >= 2
}

fn finalize_member_region(
    bind: String,
    region: String,
    host_key: Option<String>,
    connect: Option<String>,
) -> FarmMemberRegion {
    let host_key = host_key.unwrap_or_else(|| bind.clone());
    let connect = connect.unwrap_or_else(|| default_member_connect(&bind, &host_key));
    FarmMemberRegion {
        bind,
        region,
        host_key,
        connect,
    }
}

fn strip_region_suffix(host_port: &str) -> (String, Option<String>) {
    if let Some(at) = host_port.rfind('@') {
        let bind = host_port[..at].trim().to_string();
        let region = host_port[at + 1..].trim().to_string();
        if bind.is_empty() || region.is_empty() {
            return (host_port.trim().to_string(), None);
        }
        return (bind, Some(region));
    }
    (host_port.trim().to_string(), None)
}

fn parse_json_object_member_region(raw: &str) -> Result<Option<FarmMemberRegion>, String> {
    let s = raw.trim();
    if !s.starts_with('{') || !s.ends_with('}') {
        return Ok(None);
    }
    let inner = &s[1..s.len().saturating_sub(1)];
    let mut bind: Option<String> = None;
    let mut region: Option<String> = None;
    let mut host_key: Option<String> = None;
    let mut connect: Option<String> = None;
    for part in inner.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let Some((key, val)) = part.split_once(':') else {
            continue;
        };
        let key = key.trim().trim_matches('"');
        let val = val.trim().trim_matches('"');
        match key {
            "bind" | "host_port" | "addr" => bind = Some(val.to_string()),
            "host" => {
                if bind.is_none() {
                    bind = Some(val.to_string());
                } else {
                    host_key = Some(val.to_string());
                }
            }
            "host_key" | "identity" => host_key = Some(val.to_string()),
            "connect" | "probe" => connect = Some(val.to_string()),
            "region" | "dc" => region = Some(val.to_string()),
            _ => {}
        }
    }
    match (bind, region) {
        (Some(b), Some(r)) if !b.is_empty() && !r.is_empty() => {
            Ok(Some(finalize_member_region(b, r, host_key, connect)))
        }
        _ => Err("farm lease member object requires bind/host and region fields".into()),
    }
}

/// Parse region-tagged members from JSON: `["host:port@dc1", ...]` or
/// `[{"bind":"host:port","region":"dc1"}, ...]`.
pub fn parse_region_tagged_members(raw: &str) -> Result<Vec<FarmMemberRegion>, String> {
    let s = raw.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err("farm lease members must be JSON array".into());
    }
    let inner = s[1..s.len().saturating_sub(1)].trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    let mut i = 0;
    let bytes = inner.as_bytes();
    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if bytes[i] == b'{' {
            let start = i;
            let mut depth = 0usize;
            while i < bytes.len() {
                if bytes[i] == b'{' {
                    depth += 1;
                } else if bytes[i] == b'}' {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            let obj = std::str::from_utf8(&bytes[start..i])
                .map_err(|e| format!("farm lease members utf8: {e}"))?;
            if let Some(member) = parse_json_object_member_region(obj)? {
                out.push(member);
            }
        } else if bytes[i] == b'"' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            if i >= bytes.len() {
                return Err("farm lease members JSON unterminated string".into());
            }
            let host_port = std::str::from_utf8(&bytes[start..i])
                .map_err(|e| format!("farm lease members utf8: {e}"))?
                .trim()
                .to_string();
            if host_port.is_empty() {
                return Err("farm lease members empty host:port".into());
            }
            let (bind, region) = strip_region_suffix(&host_port);
            let region = region.unwrap_or_else(|| "default".to_string());
            out.push(finalize_member_region(bind, region, None, None));
            i += 1;
        } else {
            return Err("farm lease members JSON must use quoted strings or objects".into());
        }
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < bytes.len() {
            if bytes[i] != b',' {
                return Err("farm lease members JSON expected comma".into());
            }
            i += 1;
        }
    }
    Ok(out)
}

pub fn farm_lease_probe_member_health(bind: &str, timeout: Duration) -> bool {
    farm_lease_probe_member_connect(bind, timeout)
}

fn farm_lease_probe_member_connect(connect: &str, timeout: Duration) -> bool {
    let normalized = normalize_tcp_url(connect);
    let host_port = host_port_of(&normalized);
    let Ok(addr) = host_port.parse::<SocketAddr>() else {
        return false;
    };
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));
    tcp_write_frame(&mut stream, b"HEALTH\0").is_ok()
        && tcp_read_frame(&mut stream)
            .map(|resp| resp.starts_with(b"OK\0"))
            .unwrap_or(false)
}

fn farm_lease_probe_member_region_health(member: &FarmMemberRegion, timeout: Duration) -> bool {
    farm_lease_probe_member_connect(&member.connect, timeout)
}

/// Multi-DC discovery: local HEALTH on each member, then at least one cross-region probe.
/// Members that fail probe are removed from the active set.
pub fn farm_lease_multidc_discover_active(
    members: &[FarmMemberRegion],
    probe_timeout: Duration,
) -> Vec<FarmMemberRegion> {
    let mut active: HashSet<usize> = HashSet::new();
    for (i, m) in members.iter().enumerate() {
        if farm_lease_probe_member_region_health(m, probe_timeout) {
            active.insert(i);
        }
    }

    let mut by_region: HashMap<String, Vec<usize>> = HashMap::new();
    for &i in &active {
        by_region
            .entry(members[i].region.clone())
            .or_default()
            .push(i);
    }
    let regions: Vec<String> = by_region.keys().cloned().collect();
    if regions.len() >= 2 {
        let r0 = &regions[0];
        let r1 = &regions[1];
        if let (Some(&i0), Some(&i1)) = (by_region[r0].first(), by_region[r1].first()) {
            if !farm_lease_probe_member_region_health(&members[i1], probe_timeout) {
                active.remove(&i1);
            }
            if !farm_lease_probe_member_region_health(&members[i0], probe_timeout) {
                active.remove(&i0);
            }
        }
    }

    let mut out: Vec<FarmMemberRegion> = active.into_iter().map(|i| members[i].clone()).collect();
    out.sort_by(|a, b| a.bind.cmp(&b.bind));
    out
}

/// Phase 114: validate snapshot authority fields (tests — deadlines require tokens).
pub fn farm_lease_validate_snap_authority_fields(
    tenant_deadlines: &HashMap<u32, u64>,
    tenant_tokens: &HashMap<u32, u32>,
) -> Result<(), String> {
    validate_snap_authority(&WalSnapshotData {
        commit_index: 0,
        last_applied: 0,
        log: Vec::new(),
        tenant_deadlines: tenant_deadlines.clone(),
        tenant_tokens: tenant_tokens.clone(),
        members: Vec::new(),
    })
}

fn validate_snap_authority(data: &WalSnapshotData) -> Result<(), String> {
    if !data.log.is_empty() {
        return Err("legacy embedded-log snap rejected".into());
    }
    for tenant_id in data.tenant_deadlines.keys() {
        if !data.tenant_tokens.contains_key(tenant_id) {
            return Err(format!(
                "snap tenant {tenant_id} missing token (legacy snap rejected)"
            ));
        }
    }
    Ok(())
}

fn validate_cross_host_member_regions(members: &[FarmMemberRegion]) -> Result<(), String> {
    if !farm_lease_member_regions_cross_host_ga(members) {
        return Err(
            "farm lease cross-host GA requires ≥2 distinct non-loopback host keys (not all 127.0.0.1/::1)"
                .into(),
        );
    }
    Ok(())
}

fn validate_real_connect_member_regions(members: &[FarmMemberRegion]) -> Result<(), String> {
    if !farm_lease_member_regions_real_connect_ga(members) {
        return Err(
            "farm lease real-connect GA requires ≥2 members with non-loopback connect targets (127 rewrite / TEST-NET+127 is not GA)"
                .into(),
        );
    }
    Ok(())
}

fn validate_os_dual_process_member_regions(members: &[FarmMemberRegion]) -> Result<(), String> {
    if !farm_lease_member_regions_os_dual_process_ga(members) {
        return Err(
            "farm lease os-dual-process GA requires ≥2 non-loopback connect targets and bans TEST-NET hostkey-only paper proof"
                .into(),
        );
    }
    Ok(())
}

fn validate_runtime_member_regions(members: &[FarmMemberRegion]) -> Result<(), String> {
    if members.len() < 3 {
        return Err(format!(
            "farm lease runtime members require at least 3 host:port entries, got {}",
            members.len()
        ));
    }
    validate_cross_host_member_regions(members)?;
    validate_real_connect_member_regions(members)?;
    validate_os_dual_process_member_regions(members)
}

fn load_runtime_member_regions_raw() -> Result<Vec<FarmMemberRegion>, String> {
    if let Ok(raw) = std::env::var(ENV_FARM_LEASE_MEMBERS) {
        if !raw.trim().is_empty() {
            let members = parse_region_tagged_members(&raw)?;
            validate_runtime_member_regions(&members)?;
            return Ok(members);
        }
    }
    if let Ok(path) = std::env::var(ENV_FARM_LEASE_MEMBERS_FILE) {
        if !path.trim().is_empty() {
            let raw = fs::read_to_string(&path)
                .map_err(|e| format!("farm lease members file {}: {e}", path))?;
            let members = parse_region_tagged_members(&raw)?;
            validate_runtime_member_regions(&members)?;
            return Ok(members);
        }
    }
    Err(format!(
        "farm lease runtime members not configured (set {} or {})",
        ENV_FARM_LEASE_MEMBERS, ENV_FARM_LEASE_MEMBERS_FILE
    ))
}

/// Load region-tagged Raft members from [`ENV_FARM_LEASE_MEMBERS`] or
/// [`ENV_FARM_LEASE_MEMBERS_FILE`] (Phase 115 — retains `region`, not bind-only).
pub fn load_runtime_member_regions() -> Result<Vec<FarmMemberRegion>, String> {
    load_runtime_member_regions_raw()
}

/// Load Raft member binds from [`ENV_FARM_LEASE_MEMBERS`] or [`ENV_FARM_LEASE_MEMBERS_FILE`].
///
/// Phase 113: returns a dynamic `Vec` (len ≥ 3). Fixed `[String;3]` compile-time binds are
/// **not** dyn-list GA — use [`spawn_farm_lease_raft_runtime`].
pub fn load_runtime_member_binds() -> Result<Vec<String>, String> {
    load_runtime_member_regions().map(|members| members.into_iter().map(|m| m.bind).collect())
}

/// Phase 115: reload env/file member list and re-probe active set (registry reload leg).
pub fn farm_lease_registry_reload_probe(
    probe_timeout: Duration,
) -> Result<Vec<FarmMemberRegion>, String> {
    let members = load_runtime_member_regions()?;
    let active = farm_lease_multidc_discover_active(&members, probe_timeout);
    if active.len() < 3 {
        return Err(format!(
            "farm lease registry reload-probe: need at least 3 active members, got {}",
            active.len()
        ));
    }
    Ok(active)
}

fn farm_lease_raft_member_remove_at(leader_url: &str, id: u32) -> Result<(), String> {
    let mut req = b"MEMBER\0".to_vec();
    req.push(2);
    req.extend_from_slice(&id.to_le_bytes());
    let payload = lease_auth_rpc_one(leader_url, &req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

fn farm_lease_registry_sync_health_to_cluster(
    cluster: &mut FarmLeaseRaftCluster,
    members: &[FarmMemberRegion],
    active: &[FarmMemberRegion],
) -> Result<(), String> {
    if !cluster.runtime_config_ga {
        return Err("registry health-sync requires runtime config GA cluster".into());
    }
    let active_connects: HashSet<&str> = active.iter().map(|m| m.connect.as_str()).collect();
    let leader_url = format!("tcp://{}", cluster.leader_addr());
    for (id, member) in members.iter().enumerate() {
        if !active_connects.contains(member.connect.as_str()) {
            farm_lease_raft_member_remove_at(&leader_url, id as u32)?;
        }
    }
    Ok(())
}

/// Phase 116: HEALTH prune + leader MEMBER push to sync cluster membership view.
pub fn farm_lease_registry_health_sync(
    cluster: &mut FarmLeaseRaftCluster,
    probe_timeout: Duration,
) -> Result<Vec<FarmMemberRegion>, String> {
    if !cluster.runtime_config_ga {
        return Err("registry health-sync requires runtime config GA cluster".into());
    }
    let members = load_runtime_member_regions()?;
    let active = farm_lease_multidc_discover_active(&members, probe_timeout);
    if active.len() < 3 {
        return Err(format!(
            "farm lease registry health-sync: need at least 3 active members, got {}",
            active.len()
        ));
    }
    let _ = cluster.refresh_leader(30);
    farm_lease_registry_sync_health_to_cluster(cluster, &members, &active)?;
    cluster.member_regions = members;
    cluster.active_member_regions = active.clone();
    Ok(active)
}

/// Query Raft membership ids from any node (leader or follower).
pub fn farm_lease_raft_members_at(url: &str) -> Result<Vec<u32>, String> {
    let payload = lease_auth_rpc_one(url, b"MEMBERS\0")?;
    if let Some(msg) = payload.strip_prefix(b"ERR\0") {
        return Err(String::from_utf8_lossy(msg).into_owned());
    }
    let body = payload
        .strip_prefix(b"OK\0")
        .ok_or_else(|| "members: bad OK".to_string())?;
    if body.len() < 4 {
        return Err("members: truncated count".into());
    }
    let n = u32::from_le_bytes(body[0..4].try_into().unwrap()) as usize;
    if body.len() < 4 + n * 4 {
        return Err("members: truncated ids".into());
    }
    let mut ids = Vec::with_capacity(n);
    for i in 0..n {
        let off = 4 + i * 4;
        ids.push(u32::from_le_bytes(body[off..off + 4].try_into().unwrap()));
    }
    Ok(ids)
}

/// Phase 117: HEALTH prune + Raft consensus membership change, verified on a follower.
///
/// Beyond Phase 116 leader-only MEMBER remove / health-sync: after removing inactive members
/// via the replicated log, a live follower must observe the updated membership set.
pub fn farm_lease_registry_consensus_sync(
    cluster: &mut FarmLeaseRaftCluster,
    probe_timeout: Duration,
) -> Result<Vec<FarmMemberRegion>, String> {
    if !cluster.runtime_config_ga {
        return Err("registry consensus-sync requires runtime config GA cluster".into());
    }
    let members = load_runtime_member_regions()?;
    let active = farm_lease_multidc_discover_active(&members, probe_timeout);
    if active.len() < 3 {
        return Err(format!(
            "farm lease registry consensus-sync: need at least 3 active members, got {}",
            active.len()
        ));
    }
    let _ = cluster.refresh_leader(30);
    let active_connects: HashSet<&str> = active.iter().map(|m| m.connect.as_str()).collect();
    let mut removed_ids: Vec<u32> = Vec::new();
    let leader_url = format!("tcp://{}", cluster.leader_addr());
    for (id, member) in members.iter().enumerate() {
        if !active_connects.contains(member.connect.as_str()) {
            let id = id as u32;
            farm_lease_raft_member_remove_at(&leader_url, id)?;
            removed_ids.push(id);
        }
    }
    let mut added_learner: Option<u32> = None;
    if removed_ids.is_empty() {
        // When all members are healthy, still exercise a consensus config change.
        const LEARNER_ID: u32 = 9001;
        farm_lease_raft_member_add_learner_at(&leader_url, LEARNER_ID)?;
        added_learner = Some(LEARNER_ID);
    }

    let leader_idx = cluster.leader_idx();
    let probe = farm_lease_multidc_probe_timeout();
    let follower_idx = (0..cluster.addrs().len())
        .find(|&i| {
            if i == leader_idx {
                return false;
            }
            let addr = cluster.addrs()[i].to_string();
            farm_lease_probe_member_health(&addr, probe)
        })
        .ok_or_else(|| "consensus-sync: no live follower to observe membership".to_string())?;
    let follower_url = format!("tcp://{}", cluster.addrs()[follower_idx]);
    std::thread::sleep(Duration::from_millis(40));
    let follower_members = farm_lease_raft_members_at(&follower_url)?;
    if let Some(learner_id) = added_learner {
        if !follower_members.contains(&learner_id) {
            return Err(format!(
                "consensus-sync: follower did not observe AddLearner {learner_id}, got {follower_members:?}"
            ));
        }
    } else {
        for id in &removed_ids {
            if follower_members.contains(id) {
                return Err(format!(
                    "consensus-sync: follower still lists removed member {id}, got {follower_members:?}"
                ));
            }
        }
    }

    cluster.member_regions = members;
    cluster.active_member_regions = active.clone();
    farm_lease_registry_follower_view(cluster)?;
    Ok(active)
}

/// Phase 118: follower MEMBER ids must exactly match leader after consensus-sync.
pub fn farm_lease_registry_follower_view(cluster: &FarmLeaseRaftCluster) -> Result<(), String> {
    let leader_url = format!("tcp://{}", cluster.leader_addr());
    let leader_members = farm_lease_raft_members_at(&leader_url)?;
    let probe = farm_lease_multidc_probe_timeout();
    let leader_idx = cluster.leader_idx();
    let mut matched = 0usize;
    for (i, addr) in cluster.addrs().iter().enumerate() {
        if i == leader_idx {
            continue;
        }
        if cluster.children.get(i).and_then(|c| c.as_ref()).is_none() {
            continue;
        }
        if !farm_lease_probe_member_health(&addr.to_string(), probe) {
            continue;
        }
        let follower_url = format!("tcp://{addr}");
        std::thread::sleep(Duration::from_millis(40));
        let follower_members = farm_lease_raft_members_at(&follower_url)?;
        if leader_members != follower_members {
            return Err(format!(
                "follower-view: leader {leader_members:?} != follower {follower_members:?}"
            ));
        }
        matched += 1;
    }
    if matched == 0 {
        return Err("follower-view: no live follower to observe MEMBER view".to_string());
    }
    Ok(())
}

/// Phase 119: ≥2 live followers must observe the same MEMBER set as leader after consensus-sync.
pub fn farm_lease_registry_multi_follower_sync(
    cluster: &FarmLeaseRaftCluster,
    timeout: Duration,
) -> Result<(), String> {
    let _ = timeout;
    let leader_url = format!("tcp://{}", cluster.leader_addr());
    let leader_members = farm_lease_raft_members_at(&leader_url)?;
    let probe = farm_lease_multidc_probe_timeout();
    let leader_idx = cluster.leader_idx();
    let mut matched = 0usize;
    for (i, addr) in cluster.addrs().iter().enumerate() {
        if i == leader_idx {
            continue;
        }
        if cluster.children.get(i).and_then(|c| c.as_ref()).is_none() {
            continue;
        }
        if !farm_lease_probe_member_health(&addr.to_string(), probe) {
            continue;
        }
        let follower_url = format!("tcp://{addr}");
        std::thread::sleep(Duration::from_millis(40));
        let follower_members = farm_lease_raft_members_at(&follower_url)?;
        if leader_members != follower_members {
            return Err(format!(
                "multi-follower-sync: leader {leader_members:?} != follower {follower_members:?}"
            ));
        }
        matched += 1;
    }
    if matched < 2 {
        return Err(format!(
            "multi-follower-sync: need ≥2 live followers observing MEMBER set, found {matched}"
        ));
    }
    Ok(())
}

fn farm_lease_raft_member_add_learner_at(leader_url: &str, id: u32) -> Result<(), String> {
    let mut req = b"MEMBER\0".to_vec();
    req.push(1);
    req.extend_from_slice(&id.to_le_bytes());
    let payload = lease_auth_rpc_one(leader_url, &req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

/// Explicit non-GA legacy loopback member URLs (Phase 116: must not be used as silent GA fallback).
pub fn farm_lease_legacy_loopback_member_urls_non_ga() -> Vec<String> {
    legacy_loopback_member_urls()
}

// ----- Phase 109: OS-process HA v3 + Raft quorum MVP -----

fn os_child_filter() -> String {
    std::env::var(ENV_OS_CHILD_FILTER)
        .unwrap_or_else(|_| "build_farm_lease_ha_v3::p109_farm_os_child_entry".to_string())
}

fn os_child_exe() -> PathBuf {
    std::env::current_exe().expect("current_exe for farm lease OS child")
}

fn write_ready_file(path: &Path, addr: SocketAddr) {
    let _ = std::fs::write(path, addr.to_string());
}

fn spawn_os_authority_child(
    mode: &str,
    bind: &str,
    max_conns: usize,
    ready_file: &Path,
    peer: Option<&str>,
    raft_id: Option<u32>,
    raft_peers: Option<&str>,
    raft_member_urls: Option<&str>,
    wal_dir: Option<&Path>,
    runtime_config_ga: bool,
    dyn_members_ga: bool,
) -> Child {
    let mut cmd = Command::new(os_child_exe());
    cmd.args([os_child_filter(), "--exact".into(), "--nocapture".into()]);
    cmd.env(ENV_OS_CHILD_MODE, mode);
    cmd.env(ENV_OS_CHILD_BIND, bind);
    cmd.env(ENV_OS_CHILD_MAX_CONNS, max_conns.to_string());
    cmd.env(ENV_OS_CHILD_READY_FILE, ready_file);
    cmd.env("RUST_TEST_THREADS", "1");
    // Avoid inheriting parent lease client config into the authority child.
    cmd.env_remove("SYX_FARM_LEASE_AUTH");
    cmd.env_remove("SYX_FARM_LEASE_AUTH_SECONDARY");
    cmd.env_remove("SYX_FARM_LEASE_AUTH_TERTIARY");
    if let Some(p) = peer {
        cmd.env(ENV_OS_CHILD_PEER, p);
    }
    if let Some(id) = raft_id {
        cmd.env(ENV_OS_CHILD_RAFT_ID, id.to_string());
    }
    if let Some(peers) = raft_peers {
        cmd.env(ENV_OS_CHILD_RAFT_PEERS, peers);
    }
    if let Some(members) = raft_member_urls {
        cmd.env(ENV_OS_CHILD_RAFT_MEMBER_URLS, members);
    }
    if let Some(dir) = wal_dir {
        cmd.env(ENV_OS_CHILD_WAL_DIR, dir);
        cmd.env(
            ENV_FARM_TOKEN_EPOCH_PATH,
            farm_lease_token_epoch_path(Some(dir)),
        );
    }
    if runtime_config_ga {
        cmd.env(ENV_RUNTIME_CONFIG_GA, "1");
    }
    if dyn_members_ga {
        cmd.env(ENV_DYN_MEMBERS_GA, "1");
    }
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.spawn()
        .unwrap_or_else(|e| panic!("farm lease OS child spawn ({mode}): {e}"))
}

fn wait_ready_file(path: &Path, deadline: Duration) -> Result<SocketAddr, String> {
    let start = Instant::now();
    while start.elapsed() < deadline {
        if let Ok(s) = std::fs::read_to_string(path) {
            let s = s.trim();
            if !s.is_empty() {
                return s
                    .parse()
                    .map_err(|e| format!("farm lease OS child ready parse {s}: {e}"));
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    Err(format!(
        "farm lease OS child ready timeout ({})",
        path.display()
    ))
}

fn wait_child_ready(child: &mut Child, path: &Path, deadline: Duration) -> SocketAddr {
    match wait_ready_file(path, deadline) {
        Ok(addr) => addr,
        Err(e) => {
            let status = child.try_wait().ok().flatten();
            kill_child_deadline(child, Duration::from_millis(200));
            panic!("{e}; child_status={status:?} filter={}", os_child_filter());
        }
    }
}

fn kill_child_deadline(child: &mut Child, join_deadline: Duration) {
    let _ = child.kill();
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) if start.elapsed() < join_deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            _ => return,
        }
    }
}

fn serve_ha_os_child(bind: &str, max_conns: usize, peer: Option<String>, ready_file: &Path) {
    let listener = TcpListener::bind(bind).unwrap_or_else(|e| {
        panic!("farm lease HA v3 child bind {bind}: {e}");
    });
    listener
        .set_nonblocking(true)
        .expect("farm lease HA v3 nonblocking");
    let addr = listener.local_addr().expect("farm lease HA v3 local_addr");
    write_ready_file(ready_file, addr);

    // Independent AuthorityState in this OS process (not shared with parent/peer process).
    let mut state = AuthorityState::default();
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut accepted = 0usize;
    while accepted < max_conns && Instant::now() < deadline {
        match listener.accept() {
            Ok((mut stream, _)) => {
                accepted += 1;
                let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
                let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
                let _ = (|| -> Result<(), String> {
                    let req = tcp_read_frame(&mut stream)?;
                    let (resp, mutated) = handle_authority_request(&mut state, &req);
                    if let (Some(peer_url), Some(mutation)) = (peer.as_ref(), mutated) {
                        sync_lease_to_peer(peer_url, mutation);
                    }
                    tcp_write_frame(&mut stream, &resp)
                })();
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => std::thread::sleep(Duration::from_millis(5)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MemberRole {
    Voter,
    Learner,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RaftMember {
    id: u32,
    url: String,
    role: MemberRole,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum RaftCommand {
    SetDeadline {
        tenant_id: u32,
        deadline_ms: u64,
        token: u32,
    },
    ClearDeadline,
    AddLearner {
        id: u32,
        url: String,
    },
    RemoveMember {
        id: u32,
    },
    /// Phase 117: rotate tenant token in the replicated key table.
    RotateToken {
        tenant_id: u32,
        token: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LogEntry {
    term: u64,
    index: u64,
    command: RaftCommand,
}

struct NodeWal {
    path: PathBuf,
    file: File,
}

const WAL_MAGIC: &[u8; 8] = b"SYXWALv3";
const SNAP_MAGIC: &[u8; 8] = b"SYXSNAPv";
const TOKEN_EPOCH_MAGIC: &[u8; 8] = b"SYXTOKv1";

/// Resolve durable tenant token epoch path (`SYX_FARM_TOKEN_EPOCH_PATH` or next to WAL).
pub fn farm_lease_token_epoch_path(wal_dir: Option<&Path>) -> PathBuf {
    if let Ok(raw) = std::env::var(ENV_FARM_TOKEN_EPOCH_PATH) {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Some(dir) = wal_dir {
        return dir.join("token-epoch.dat");
    }
    std::env::temp_dir().join("syx-farm-token-epoch.dat")
}

fn write_token_epoch_file(path: &Path, tokens: &HashMap<u32, u32>) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)
            .map_err(|e| format!("token epoch mkdir {}: {e}", parent.display()))?;
    }
    let mut body = Vec::new();
    body.extend_from_slice(TOKEN_EPOCH_MAGIC);
    body.extend_from_slice(&(tokens.len() as u32).to_le_bytes());
    for (tenant_id, token) in tokens {
        body.extend_from_slice(&tenant_id.to_le_bytes());
        body.extend_from_slice(&token.to_le_bytes());
    }
    let crc = wal_crc32(&body);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("token epoch open {}: {e}", path.display()))?;
    file.write_all(&body)
        .map_err(|e| format!("token epoch body write: {e}"))?;
    file.write_all(&crc.to_le_bytes())
        .map_err(|e| format!("token epoch crc write: {e}"))?;
    file.sync_all()
        .map_err(|e| format!("token epoch fsync {}: {e}", path.display()))?;
    Ok(())
}

/// Read durable token epoch entries (empty map when file missing).
pub fn farm_lease_read_token_epoch_file(path: &Path) -> Result<HashMap<u32, u32>, String> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let mut file =
        File::open(path).map_err(|e| format!("token epoch open {}: {e}", path.display()))?;
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)
        .map_err(|e| format!("token epoch magic read: {e}"))?;
    if magic != *TOKEN_EPOCH_MAGIC {
        return Err(format!("token epoch bad magic at {}", path.display()));
    }
    let mut count_buf = [0u8; 4];
    file.read_exact(&mut count_buf)
        .map_err(|e| format!("token epoch count read: {e}"))?;
    let count = u32::from_le_bytes(count_buf) as usize;
    let mut body = Vec::new();
    body.extend_from_slice(&magic);
    body.extend_from_slice(&count_buf);
    let mut tokens = HashMap::with_capacity(count);
    for _ in 0..count {
        let mut tenant_buf = [0u8; 4];
        file.read_exact(&mut tenant_buf)
            .map_err(|e| format!("token epoch tenant read: {e}"))?;
        let mut token_buf = [0u8; 4];
        file.read_exact(&mut token_buf)
            .map_err(|e| format!("token epoch token read: {e}"))?;
        body.extend_from_slice(&tenant_buf);
        body.extend_from_slice(&token_buf);
        tokens.insert(
            u32::from_le_bytes(tenant_buf),
            u32::from_le_bytes(token_buf),
        );
    }
    let mut crc_buf = [0u8; 4];
    file.read_exact(&mut crc_buf)
        .map_err(|e| format!("token epoch crc read: {e}"))?;
    let stored_crc = u32::from_le_bytes(crc_buf);
    if wal_crc32(&body) != stored_crc {
        return Err(format!("token epoch crc mismatch at {}", path.display()));
    }
    Ok(tokens)
}

fn persist_token_epoch_for_wal(wal: &NodeWal, tokens: &HashMap<u32, u32>) {
    if let Some(dir) = wal.path.parent() {
        let path = farm_lease_token_epoch_path(Some(dir));
        let _ = write_token_epoch_file(&path, tokens);
    }
}

fn load_token_epoch_into_auth(auth: &mut AuthorityState, wal_dir: Option<&Path>) {
    let path = farm_lease_token_epoch_path(wal_dir);
    if let Ok(tokens) = farm_lease_read_token_epoch_file(&path) {
        for (tenant_id, token) in tokens {
            auth.set_token(tenant_id, token);
        }
    }
}

struct WalSnapshotData {
    commit_index: u64,
    last_applied: u64,
    log: Vec<LogEntry>,
    tenant_deadlines: HashMap<u32, u64>,
    tenant_tokens: HashMap<u32, u32>,
    members: Vec<RaftMember>,
}

fn wal_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                0xEDB8_8320 ^ (crc >> 1)
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

impl NodeWal {
    fn open(dir: &Path, node_id: u32) -> Result<Self, String> {
        fs::create_dir_all(dir).map_err(|e| format!("wal mkdir {}: {e}", dir.display()))?;
        let path = dir.join(format!("node-{node_id}.wal"));
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)
            .map_err(|e| format!("wal open {}: {e}", path.display()))?;
        let meta = file.metadata().map_err(|e| format!("wal metadata: {e}"))?;
        if meta.len() == 0 {
            file.write_all(WAL_MAGIC)
                .map_err(|e| format!("wal magic write: {e}"))?;
            file.sync_all()
                .map_err(|e| format!("wal magic fsync: {e}"))?;
        }
        file.seek(SeekFrom::End(0))
            .map_err(|e| format!("wal seek end: {e}"))?;
        Ok(Self { path, file })
    }

    fn append_entry(&mut self, entry: &LogEntry) -> Result<(), String> {
        let (cmd_tag, payload) = encode_command(&entry.command);
        let mut record = Vec::new();
        record.extend_from_slice(&entry.term.to_le_bytes());
        record.extend_from_slice(&entry.index.to_le_bytes());
        record.push(cmd_tag);
        record.extend_from_slice(&payload);
        let crc = wal_crc32(&record);
        self.file
            .write_all(&record)
            .map_err(|e| format!("wal record: {e}"))?;
        self.file
            .write_all(&crc.to_le_bytes())
            .map_err(|e| format!("wal crc: {e}"))?;
        self.file
            .sync_all()
            .map_err(|e| format!("wal fsync {}: {e}", self.path.display()))?;
        Ok(())
    }

    fn read_record_body(file: &mut File) -> Result<Option<(u64, u64, u8, Vec<u8>)>, String> {
        let mut term_buf = [0u8; 8];
        match file.read_exact(&mut term_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(format!("wal replay term: {e}")),
        }
        let term = u64::from_le_bytes(term_buf);
        let mut index_buf = [0u8; 8];
        file.read_exact(&mut index_buf)
            .map_err(|e| format!("wal replay index: {e}"))?;
        let index = u64::from_le_bytes(index_buf);
        let mut cmd_tag = [0u8; 1];
        file.read_exact(&mut cmd_tag)
            .map_err(|e| format!("wal replay cmd tag: {e}"))?;
        let payload = decode_command_payload(cmd_tag[0], file)?;
        let mut crc_buf = [0u8; 4];
        file.read_exact(&mut crc_buf)
            .map_err(|e| format!("wal replay crc: {e}"))?;
        let stored_crc = u32::from_le_bytes(crc_buf);
        let mut record = Vec::new();
        record.extend_from_slice(&term_buf);
        record.extend_from_slice(&index_buf);
        record.push(cmd_tag[0]);
        record.extend_from_slice(&payload);
        let computed = wal_crc32(&record);
        if computed != stored_crc {
            return Err(format!(
                "wal crc mismatch at {} index={index} (corrupt record rejected)",
                file.metadata().map(|m| m.len()).unwrap_or(0)
            ));
        }
        Ok(Some((term, index, cmd_tag[0], payload)))
    }

    fn replay(dir: &Path, node_id: u32) -> Result<Vec<LogEntry>, String> {
        let path = dir.join(format!("node-{node_id}.wal"));
        if !path.exists() {
            return Ok(Vec::new());
        }
        let mut file = File::open(&path).map_err(|e| format!("wal replay open: {e}"))?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)
            .map_err(|e| format!("wal replay magic: {e}"))?;
        if magic != *WAL_MAGIC {
            return Err(format!("wal bad magic at {}", path.display()));
        }
        let mut out = Vec::new();
        loop {
            let Some((term, index, cmd_tag, payload)) = Self::read_record_body(&mut file)? else {
                break;
            };
            let command = command_from_parts(cmd_tag, &payload)?;
            out.push(LogEntry {
                term,
                index,
                command,
            });
        }
        Ok(out)
    }

    fn snapshot_path(dir: &Path, node_id: u32) -> PathBuf {
        dir.join(format!("node-{node_id}.snap"))
    }

    fn write_snapshot(dir: &Path, node_id: u32, data: &WalSnapshotData) -> Result<(), String> {
        let path = Self::snapshot_path(dir, node_id);
        let mut body = Vec::new();
        body.extend_from_slice(&data.commit_index.to_le_bytes());
        body.extend_from_slice(&data.last_applied.to_le_bytes());
        body.extend_from_slice(&(data.log.len() as u32).to_le_bytes());
        for entry in &data.log {
            body.extend_from_slice(&entry.term.to_le_bytes());
            body.extend_from_slice(&entry.index.to_le_bytes());
            let (tag, payload) = encode_command(&entry.command);
            body.push(tag);
            body.extend_from_slice(&payload);
        }
        body.extend_from_slice(&(data.tenant_deadlines.len() as u32).to_le_bytes());
        for (tenant_id, deadline) in &data.tenant_deadlines {
            body.extend_from_slice(&tenant_id.to_le_bytes());
            body.extend_from_slice(&deadline.to_le_bytes());
        }
        body.extend_from_slice(&(data.tenant_tokens.len() as u32).to_le_bytes());
        for (tenant_id, token) in &data.tenant_tokens {
            body.extend_from_slice(&tenant_id.to_le_bytes());
            body.extend_from_slice(&token.to_le_bytes());
        }
        body.extend_from_slice(&(data.members.len() as u32).to_le_bytes());
        for member in &data.members {
            body.extend_from_slice(&member.id.to_le_bytes());
            body.push(match member.role {
                MemberRole::Voter => 0,
                MemberRole::Learner => 1,
            });
            let url_bytes = member.url.as_bytes();
            body.extend_from_slice(&(url_bytes.len() as u32).to_le_bytes());
            body.extend_from_slice(url_bytes);
        }
        let crc = wal_crc32(&body);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| format!("snap open {}: {e}", path.display()))?;
        file.write_all(SNAP_MAGIC)
            .map_err(|e| format!("snap magic write: {e}"))?;
        file.write_all(&crc.to_le_bytes())
            .map_err(|e| format!("snap crc write: {e}"))?;
        file.write_all(&body)
            .map_err(|e| format!("snap body write: {e}"))?;
        file.sync_all()
            .map_err(|e| format!("snap fsync {}: {e}", path.display()))?;
        Ok(())
    }

    fn read_snapshot(dir: &Path, node_id: u32) -> Result<Option<WalSnapshotData>, String> {
        let path = Self::snapshot_path(dir, node_id);
        if !path.exists() {
            return Ok(None);
        }
        let mut file =
            File::open(&path).map_err(|e| format!("snap open {}: {e}", path.display()))?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)
            .map_err(|e| format!("snap magic read: {e}"))?;
        if magic != *SNAP_MAGIC {
            return Err(format!("snap bad magic at {}", path.display()));
        }
        let mut crc_buf = [0u8; 4];
        file.read_exact(&mut crc_buf)
            .map_err(|e| format!("snap crc read: {e}"))?;
        let stored_crc = u32::from_le_bytes(crc_buf);
        let mut body = Vec::new();
        file.read_to_end(&mut body)
            .map_err(|e| format!("snap body read: {e}"))?;
        let computed = wal_crc32(&body);
        if computed != stored_crc {
            return Err(format!(
                "snap crc mismatch at {} (corrupt snapshot rejected)",
                path.display()
            ));
        }
        let mut off = 0usize;
        let need = |body: &[u8], off: usize, n: usize| -> Result<(), String> {
            if off + n > body.len() {
                Err("snap truncated body".into())
            } else {
                Ok(())
            }
        };
        need(&body, off, 16)?;
        let commit_index = u64::from_le_bytes(body[off..off + 8].try_into().unwrap());
        off += 8;
        let last_applied = u64::from_le_bytes(body[off..off + 8].try_into().unwrap());
        off += 8;
        need(&body, off, 4)?;
        let n_log = u32::from_le_bytes(body[off..off + 4].try_into().unwrap()) as usize;
        off += 4;
        let mut log = Vec::with_capacity(n_log);
        for _ in 0..n_log {
            need(&body, off, 17)?;
            let term = u64::from_le_bytes(body[off..off + 8].try_into().unwrap());
            off += 8;
            let index = u64::from_le_bytes(body[off..off + 8].try_into().unwrap());
            off += 8;
            let tag = body[off];
            off += 1;
            let payload = decode_command_payload_from_bytes(tag, &body, &mut off)?;
            let command = command_from_parts(tag, &payload)?;
            log.push(LogEntry {
                term,
                index,
                command,
            });
        }
        need(&body, off, 4)?;
        let n_deadlines = u32::from_le_bytes(body[off..off + 4].try_into().unwrap()) as usize;
        off += 4;
        let mut tenant_deadlines = HashMap::new();
        for _ in 0..n_deadlines {
            need(&body, off, 12)?;
            let tenant_id = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
            off += 4;
            let deadline = u64::from_le_bytes(body[off..off + 8].try_into().unwrap());
            off += 8;
            tenant_deadlines.insert(tenant_id, deadline);
        }
        need(&body, off, 4)?;
        let n_tokens = u32::from_le_bytes(body[off..off + 4].try_into().unwrap()) as usize;
        off += 4;
        let mut tenant_tokens = HashMap::new();
        for _ in 0..n_tokens {
            need(&body, off, 8)?;
            let tenant_id = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
            off += 4;
            let token = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
            off += 4;
            tenant_tokens.insert(tenant_id, token);
        }
        need(&body, off, 4)?;
        let n_members = u32::from_le_bytes(body[off..off + 4].try_into().unwrap()) as usize;
        off += 4;
        let mut members = Vec::with_capacity(n_members);
        for _ in 0..n_members {
            need(&body, off, 9)?;
            let id = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
            off += 4;
            let role = match body[off] {
                0 => MemberRole::Voter,
                1 => MemberRole::Learner,
                other => return Err(format!("snap bad member role {other}")),
            };
            off += 1;
            let url_len = u32::from_le_bytes(body[off..off + 4].try_into().unwrap()) as usize;
            off += 4;
            need(&body, off, url_len)?;
            let url = std::str::from_utf8(&body[off..off + url_len])
                .map_err(|e| format!("snap member url utf8: {e}"))?
                .to_string();
            off += url_len;
            members.push(RaftMember { id, url, role });
        }
        Ok(Some(WalSnapshotData {
            commit_index,
            last_applied,
            log,
            tenant_deadlines,
            tenant_tokens,
            members,
        }))
    }

    /// Phase 113 compaction: snapshot embeds applied state only — **not** discardable old log.
    /// WAL is truncated; restart = snap state + incremental WAL.
    fn checkpoint_from_state(
        dir: &Path,
        node_id: u32,
        state: &mut RaftNodeState,
    ) -> Result<(), String> {
        let data = WalSnapshotData {
            commit_index: state.commit_index,
            last_applied: state.last_applied,
            // Compact: omit discardable log entries (authority is deadlines/tokens/members).
            log: Vec::new(),
            tenant_deadlines: state.auth.tenant_deadlines.clone(),
            tenant_tokens: state.auth.tenant_tokens.clone(),
            members: state.members.clone(),
        };
        Self::write_snapshot(dir, node_id, &data)?;
        let wal_path = dir.join(format!("node-{node_id}.wal"));
        if wal_path.exists() {
            let file = OpenOptions::new()
                .write(true)
                .open(&wal_path)
                .map_err(|e| format!("wal truncate open: {e}"))?;
            file.set_len(WAL_MAGIC.len() as u64)
                .map_err(|e| format!("wal truncate: {e}"))?;
            file.sync_all()
                .map_err(|e| format!("wal truncate fsync: {e}"))?;
        }
        // In-memory compact: keep a single base entry at last_applied for raft indices.
        state.log = vec![compacted_base_entry(state.last_applied, state.current_term)];
        if let Some(wal) = state.wal.as_mut() {
            wal.file
                .seek(SeekFrom::End(0))
                .map_err(|e| format!("wal seek after truncate: {e}"))?;
        }
        Ok(())
    }
}

fn compacted_base_entry(index: u64, term: u64) -> LogEntry {
    LogEntry {
        term,
        index,
        // Placeholder — applied state is restored from snapshot fields, not this command.
        command: RaftCommand::ClearDeadline,
    }
}

fn encode_command(cmd: &RaftCommand) -> (u8, Vec<u8>) {
    match cmd {
        RaftCommand::SetDeadline {
            tenant_id,
            deadline_ms,
            token,
        } => {
            let mut payload = tenant_id.to_le_bytes().to_vec();
            payload.extend_from_slice(&deadline_ms.to_le_bytes());
            payload.extend_from_slice(&token.to_le_bytes());
            (1, payload)
        }
        RaftCommand::ClearDeadline => (2, Vec::new()),
        RaftCommand::AddLearner { id, url } => {
            let mut payload = id.to_le_bytes().to_vec();
            let url_bytes = url.as_bytes();
            payload.extend_from_slice(&(url_bytes.len() as u32).to_le_bytes());
            payload.extend_from_slice(url_bytes);
            (3, payload)
        }
        RaftCommand::RemoveMember { id } => (4, id.to_le_bytes().to_vec()),
        RaftCommand::RotateToken { tenant_id, token } => {
            let mut payload = tenant_id.to_le_bytes().to_vec();
            payload.extend_from_slice(&token.to_le_bytes());
            (5, payload)
        }
    }
}

fn decode_command_payload_from_bytes(
    tag: u8,
    body: &[u8],
    off: &mut usize,
) -> Result<Vec<u8>, String> {
    let payload = match tag {
        1 => {
            if *off + 16 > body.len() {
                return Err("snap decode deadline truncated".into());
            }
            let slice = body[*off..*off + 16].to_vec();
            *off += 16;
            slice
        }
        2 => Vec::new(),
        3 => {
            if *off + 8 > body.len() {
                return Err("snap decode add learner truncated".into());
            }
            let id_buf = body[*off..*off + 4].to_vec();
            *off += 4;
            let len = u32::from_le_bytes(body[*off..*off + 4].try_into().unwrap()) as usize;
            *off += 4;
            if *off + len > body.len() {
                return Err("snap decode add learner url truncated".into());
            }
            let mut payload = id_buf;
            payload.extend_from_slice(&(len as u32).to_le_bytes());
            payload.extend_from_slice(&body[*off..*off + len]);
            *off += len;
            payload
        }
        4 => {
            if *off + 4 > body.len() {
                return Err("snap decode remove truncated".into());
            }
            let slice = body[*off..*off + 4].to_vec();
            *off += 4;
            slice
        }
        5 => {
            if *off + 8 > body.len() {
                return Err("snap decode rotate truncated".into());
            }
            let slice = body[*off..*off + 8].to_vec();
            *off += 8;
            slice
        }
        other => return Err(format!("snap unknown cmd tag {other}")),
    };
    Ok(payload)
}

fn decode_command_payload(tag: u8, file: &mut File) -> Result<Vec<u8>, String> {
    let payload = match tag {
        1 => {
            let mut buf = [0u8; 16];
            file.read_exact(&mut buf)
                .map_err(|e| format!("wal decode deadline: {e}"))?;
            buf.to_vec()
        }
        2 => Vec::new(),
        3 => {
            let mut id_buf = [0u8; 4];
            file.read_exact(&mut id_buf)
                .map_err(|e| format!("wal decode add learner id: {e}"))?;
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf)
                .map_err(|e| format!("wal decode add learner len: {e}"))?;
            let len = u32::from_le_bytes(len_buf) as usize;
            let mut url_buf = vec![0u8; len];
            file.read_exact(&mut url_buf)
                .map_err(|e| format!("wal decode add learner url: {e}"))?;
            let mut payload = id_buf.to_vec();
            payload.extend_from_slice(&len_buf);
            payload.extend_from_slice(&url_buf);
            payload
        }
        4 => {
            let mut buf = [0u8; 4];
            file.read_exact(&mut buf)
                .map_err(|e| format!("wal decode remove: {e}"))?;
            buf.to_vec()
        }
        5 => {
            let mut buf = [0u8; 8];
            file.read_exact(&mut buf)
                .map_err(|e| format!("wal decode rotate: {e}"))?;
            buf.to_vec()
        }
        other => return Err(format!("wal unknown cmd tag {other}")),
    };
    Ok(payload)
}

fn command_from_parts(tag: u8, payload: &[u8]) -> Result<RaftCommand, String> {
    match tag {
        1 => {
            if payload.len() != 16 {
                return Err("wal decode deadline payload".into());
            }
            let tenant_id = u32::from_le_bytes(payload[0..4].try_into().unwrap());
            let deadline_ms = u64::from_le_bytes(payload[4..12].try_into().unwrap());
            let token = u32::from_le_bytes(payload[12..16].try_into().unwrap());
            Ok(RaftCommand::SetDeadline {
                tenant_id,
                deadline_ms,
                token,
            })
        }
        2 => Ok(RaftCommand::ClearDeadline),
        3 => {
            if payload.len() < 8 {
                return Err("wal decode add learner payload".into());
            }
            let id = u32::from_le_bytes(payload[0..4].try_into().unwrap());
            let len = u32::from_le_bytes(payload[4..8].try_into().unwrap()) as usize;
            if payload.len() < 8 + len {
                return Err("wal decode add learner url payload".into());
            }
            let url = std::str::from_utf8(&payload[8..8 + len])
                .map_err(|e| format!("wal decode add learner utf8: {e}"))?
                .to_string();
            Ok(RaftCommand::AddLearner { id, url })
        }
        4 => {
            if payload.len() != 4 {
                return Err("wal decode remove payload".into());
            }
            Ok(RaftCommand::RemoveMember {
                id: u32::from_le_bytes(payload.try_into().unwrap()),
            })
        }
        5 => {
            if payload.len() != 8 {
                return Err("wal decode rotate payload".into());
            }
            let tenant_id = u32::from_le_bytes(payload[0..4].try_into().unwrap());
            let token = u32::from_le_bytes(payload[4..8].try_into().unwrap());
            Ok(RaftCommand::RotateToken { tenant_id, token })
        }
        other => Err(format!("wal unknown cmd tag {other}")),
    }
}

fn sentinel_log() -> LogEntry {
    LogEntry {
        term: 0,
        index: 0,
        command: RaftCommand::ClearDeadline,
    }
}

struct RaftNodeState {
    auth: AuthorityState,
    current_term: u64,
    voted_for: Option<u32>,
    role: RaftRole,
    election_deadline: Instant,
    node_id: u32,
    members: Vec<RaftMember>,
    log: Vec<LogEntry>,
    commit_index: u64,
    last_applied: u64,
    next_index: HashMap<u32, u64>,
    match_index: HashMap<u32, u64>,
    wal: Option<NodeWal>,
    last_heartbeat: Instant,
}

impl RaftNodeState {
    fn reset_election_timeout(&mut self) {
        let base_ms = 40u64 + (self.node_id as u64) * 25;
        self.election_deadline = Instant::now() + Duration::from_millis(base_ms);
    }

    fn voter_urls(&self) -> Vec<String> {
        self.members
            .iter()
            .filter(|m| m.role == MemberRole::Voter && m.id != self.node_id)
            .map(|m| m.url.clone())
            .collect()
    }

    fn voter_count(&self) -> u32 {
        self.members
            .iter()
            .filter(|m| m.role == MemberRole::Voter)
            .count() as u32
    }

    fn quorum(&self) -> u32 {
        self.voter_count() / 2 + 1
    }

    fn last_log_index(&self) -> u64 {
        self.log.last().map_or(0, |e| e.index)
    }

    fn last_log_term(&self) -> u64 {
        self.log.last().map_or(0, |e| e.term)
    }

    fn term_at(&self, index: u64) -> u64 {
        self.log
            .iter()
            .find(|e| e.index == index)
            .map_or(0, |e| e.term)
    }

    fn append_local(&mut self, term: u64, command: RaftCommand) -> Result<u64, String> {
        let index = self.last_log_index() + 1;
        let entry = LogEntry {
            term,
            index,
            command,
        };
        if let Some(wal) = self.wal.as_mut() {
            wal.append_entry(&entry)?;
        }
        self.log.push(entry);
        Ok(index)
    }

    fn apply_entry(&mut self, entry: &LogEntry) {
        match &entry.command {
            RaftCommand::SetDeadline {
                tenant_id,
                deadline_ms,
                token,
            } => {
                self.auth.set_deadline(*tenant_id, *deadline_ms);
                self.auth.set_token(*tenant_id, *token);
            }
            RaftCommand::ClearDeadline => {
                self.auth.clear_all();
            }
            RaftCommand::AddLearner { id, url } => {
                if !self.members.iter().any(|m| m.id == *id) {
                    self.members.push(RaftMember {
                        id: *id,
                        url: normalize_tcp_url(url),
                        role: MemberRole::Learner,
                    });
                }
            }
            RaftCommand::RemoveMember { id } => {
                self.members.retain(|m| m.id != *id);
                self.next_index.remove(id);
                self.match_index.remove(id);
            }
            RaftCommand::RotateToken { tenant_id, token } => {
                self.auth.set_token(*tenant_id, *token);
                if let Some(wal) = self.wal.as_ref() {
                    persist_token_epoch_for_wal(wal, &self.auth.tenant_tokens);
                }
            }
        }
    }

    fn apply_through(&mut self, target: u64) {
        while self.last_applied < target {
            let next = self.last_applied + 1;
            if let Some(entry) = self.log.iter().find(|e| e.index == next).cloned() {
                self.apply_entry(&entry);
                self.last_applied = next;
            } else {
                break;
            }
        }
    }

    fn advance_commit_leader(&mut self) {
        for n_idx in (1..=self.last_log_index()).rev() {
            let entry_term = self.term_at(n_idx);
            if entry_term != self.current_term {
                continue;
            }
            let mut matches = 1u32;
            for m in self.members.iter().filter(|m| m.role == MemberRole::Voter) {
                if m.id == self.node_id {
                    continue;
                }
                if self.match_index.get(&m.id).copied().unwrap_or(0) >= n_idx {
                    matches += 1;
                }
            }
            if matches >= self.quorum() {
                self.commit_index = n_idx;
                break;
            }
        }
        self.apply_through(self.commit_index);
    }
}

fn encode_vote_req(
    term: u64,
    candidate_id: u32,
    last_log_index: u64,
    last_log_term: u64,
) -> Vec<u8> {
    let mut out = b"VOTE\0".to_vec();
    out.extend_from_slice(&term.to_le_bytes());
    out.extend_from_slice(&candidate_id.to_le_bytes());
    out.extend_from_slice(&last_log_index.to_le_bytes());
    out.extend_from_slice(&last_log_term.to_le_bytes());
    out
}

fn encode_vote_ok(term: u64, granted: bool) -> Vec<u8> {
    let mut out = if granted {
        b"VOTEOK\0".to_vec()
    } else {
        b"VOTENO\0".to_vec()
    };
    out.extend_from_slice(&term.to_le_bytes());
    out
}

/// Phase 109 toy heartbeat — **not** Raft v2 GA (no log/WAL).
#[allow(dead_code)]
fn encode_append_toy_v1(term: u64, leader_id: u32, deadline_ms: Option<u64>) -> Vec<u8> {
    let mut out = b"APPEND\0".to_vec();
    out.extend_from_slice(&term.to_le_bytes());
    out.extend_from_slice(&leader_id.to_le_bytes());
    match deadline_ms {
        Some(d) => {
            out.push(1);
            out.extend_from_slice(&d.to_le_bytes());
        }
        None => out.push(0),
    }
    out
}

fn encode_append_entries_v2(
    term: u64,
    leader_id: u32,
    prev_log_index: u64,
    prev_log_term: u64,
    leader_commit: u64,
    entries: &[LogEntry],
) -> Vec<u8> {
    let mut out = b"APPEND2\0".to_vec();
    out.extend_from_slice(&term.to_le_bytes());
    out.extend_from_slice(&leader_id.to_le_bytes());
    out.extend_from_slice(&prev_log_index.to_le_bytes());
    out.extend_from_slice(&prev_log_term.to_le_bytes());
    out.extend_from_slice(&leader_commit.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for entry in entries {
        out.extend_from_slice(&entry.term.to_le_bytes());
        out.extend_from_slice(&entry.index.to_le_bytes());
        let (tag, payload) = encode_command(&entry.command);
        out.push(tag);
        out.extend_from_slice(&payload);
    }
    out
}

fn encode_append_ok_v2(term: u64, success: bool, match_index: u64) -> Vec<u8> {
    let mut out = b"APPENDOK2\0".to_vec();
    out.extend_from_slice(&term.to_le_bytes());
    out.push(u8::from(success));
    out.extend_from_slice(&match_index.to_le_bytes());
    out
}

#[allow(dead_code)]
fn encode_append_ok(term: u64) -> Vec<u8> {
    let mut out = b"APPENDOK\0".to_vec();
    out.extend_from_slice(&term.to_le_bytes());
    out
}

fn raft_rpc(url: &str, payload: &[u8]) -> Result<Vec<u8>, String> {
    lease_auth_rpc_one(url, payload)
}

fn raft_send_append_v2(state: &mut RaftNodeState, peer_id: u32, peer_url: &str) -> bool {
    let next = state.next_index.get(&peer_id).copied().unwrap_or(1);
    let prev_index = next.saturating_sub(1);
    let prev_term = state.term_at(prev_index);
    let entries: Vec<LogEntry> = state
        .log
        .iter()
        .filter(|e| e.index >= next)
        .cloned()
        .collect();
    let msg = encode_append_entries_v2(
        state.current_term,
        state.node_id,
        prev_index,
        prev_term,
        state.commit_index,
        &entries,
    );
    match raft_rpc(peer_url, &msg) {
        Ok(resp) if resp.starts_with(b"APPENDOK2\0") => {
            let rest = &resp[b"APPENDOK2\0".len()..];
            if rest.len() >= 17 {
                let mut tbuf = [0u8; 8];
                tbuf.copy_from_slice(&rest[0..8]);
                let peer_term = u64::from_le_bytes(tbuf);
                if peer_term > state.current_term {
                    state.current_term = peer_term;
                    state.role = RaftRole::Follower;
                    state.voted_for = None;
                    state.reset_election_timeout();
                    return false;
                }
                let success = rest[8] != 0;
                let mut mbuf = [0u8; 8];
                mbuf.copy_from_slice(&rest[9..17]);
                let match_idx = u64::from_le_bytes(mbuf);
                if success {
                    state.next_index.insert(peer_id, match_idx + 1);
                    state.match_index.insert(peer_id, match_idx);
                    true
                } else {
                    let dec = state.next_index.get(&peer_id).copied().unwrap_or(1);
                    state
                        .next_index
                        .insert(peer_id, dec.saturating_sub(1).max(1));
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

fn raft_replicate_log_majority(state: &mut RaftNodeState) -> bool {
    if state.role != RaftRole::Leader {
        return false;
    }
    state
        .match_index
        .insert(state.node_id, state.last_log_index());
    let peers: Vec<(u32, String)> = state
        .members
        .iter()
        .filter(|m| m.role == MemberRole::Voter && m.id != state.node_id)
        .map(|m| (m.id, m.url.clone()))
        .collect();
    for (pid, url) in &peers {
        let _ = raft_send_append_v2(state, *pid, url);
    }
    state.advance_commit_leader();
    let last_idx = state.last_log_index();
    let mut acks = 1u32;
    for (pid, _) in &peers {
        if state.match_index.get(pid).copied().unwrap_or(0) >= last_idx {
            acks += 1;
        }
    }
    acks >= state.quorum() && state.commit_index >= last_idx
}

fn raft_start_election(state: &mut RaftNodeState) {
    state.current_term = state.current_term.saturating_add(1);
    state.role = RaftRole::Candidate;
    state.voted_for = Some(state.node_id);
    state.reset_election_timeout();
    let term = state.current_term;
    let id = state.node_id;
    let last_idx = state.last_log_index();
    let last_term = state.last_log_term();
    let peers = state.voter_urls();
    let mut votes = 1u32;
    let majority = state.quorum();
    for peer in &peers {
        if let Ok(resp) = raft_rpc(peer, &encode_vote_req(term, id, last_idx, last_term)) {
            if let Some(rest) = resp.strip_prefix(b"VOTEOK\0") {
                if rest.len() >= 8 {
                    let mut tbuf = [0u8; 8];
                    tbuf.copy_from_slice(&rest[..8]);
                    let peer_term = u64::from_le_bytes(tbuf);
                    if peer_term > state.current_term {
                        state.current_term = peer_term;
                        state.role = RaftRole::Follower;
                        state.voted_for = None;
                        state.reset_election_timeout();
                        return;
                    }
                    votes += 1;
                }
            } else if let Some(rest) = resp.strip_prefix(b"VOTENO\0") {
                if rest.len() >= 8 {
                    let mut tbuf = [0u8; 8];
                    tbuf.copy_from_slice(&rest[..8]);
                    let peer_term = u64::from_le_bytes(tbuf);
                    if peer_term > state.current_term {
                        state.current_term = peer_term;
                        state.role = RaftRole::Follower;
                        state.voted_for = None;
                        state.reset_election_timeout();
                        return;
                    }
                }
            }
        }
    }
    if votes >= majority && state.role == RaftRole::Candidate {
        state.role = RaftRole::Leader;
        state.last_heartbeat = Instant::now();
        for m in state.members.iter().filter(|m| m.role == MemberRole::Voter) {
            state.next_index.insert(m.id, state.last_log_index() + 1);
            state.match_index.insert(m.id, 0);
        }
        state
            .match_index
            .insert(state.node_id, state.last_log_index());
        for (pid, url) in state
            .members
            .iter()
            .filter(|m| m.role == MemberRole::Voter && m.id != state.node_id)
            .map(|m| (m.id, m.url.clone()))
            .collect::<Vec<_>>()
        {
            let _ = raft_send_append_v2(state, pid, &url);
        }
    }
}

fn handle_append_entries_v2(state: &mut RaftNodeState, rest: &[u8]) -> Vec<u8> {
    if rest.len() < 36 {
        return encode_err("append2 bad payload");
    }
    let term = u64::from_le_bytes(rest[0..8].try_into().unwrap());
    let leader_id = u32::from_le_bytes(rest[8..12].try_into().unwrap());
    let prev_log_index = u64::from_le_bytes(rest[12..20].try_into().unwrap());
    let prev_log_term = u64::from_le_bytes(rest[20..28].try_into().unwrap());
    let leader_commit = u64::from_le_bytes(rest[28..36].try_into().unwrap());
    let n_entries = u32::from_le_bytes(rest[36..40].try_into().unwrap()) as usize;
    let mut off = 40usize;
    if term < state.current_term {
        return encode_append_ok_v2(state.current_term, false, state.last_log_index());
    }
    if term > state.current_term {
        state.current_term = term;
        state.voted_for = None;
    }
    state.role = RaftRole::Follower;
    state.reset_election_timeout();

    if prev_log_index > 0 {
        let local_term = state.term_at(prev_log_index);
        if local_term != prev_log_term {
            return encode_append_ok_v2(state.current_term, false, state.last_log_index());
        }
    }

    let mut entries = Vec::with_capacity(n_entries);
    for _ in 0..n_entries {
        if off + 17 > rest.len() {
            return encode_err("append2 truncated entry");
        }
        let eterm = u64::from_le_bytes(rest[off..off + 8].try_into().unwrap());
        off += 8;
        let eindex = u64::from_le_bytes(rest[off..off + 8].try_into().unwrap());
        off += 8;
        let tag = rest[off];
        off += 1;
        let command = match tag {
            1 => {
                if off + 16 > rest.len() {
                    return encode_err("append2 deadline truncated");
                }
                let tenant_id = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                let d = u64::from_le_bytes(rest[off..off + 8].try_into().unwrap());
                off += 8;
                let token = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                RaftCommand::SetDeadline {
                    tenant_id,
                    deadline_ms: d,
                    token,
                }
            }
            2 => RaftCommand::ClearDeadline,
            3 => {
                if off + 8 > rest.len() {
                    return encode_err("append2 learner truncated");
                }
                let id = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                let url_len = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap()) as usize;
                off += 4;
                if off + url_len > rest.len() {
                    return encode_err("append2 learner url truncated");
                }
                let url = std::str::from_utf8(&rest[off..off + url_len])
                    .unwrap_or("")
                    .to_string();
                off += url_len;
                RaftCommand::AddLearner { id, url }
            }
            4 => {
                if off + 4 > rest.len() {
                    return encode_err("append2 remove truncated");
                }
                let id = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                RaftCommand::RemoveMember { id }
            }
            5 => {
                if off + 8 > rest.len() {
                    return encode_err("append2 rotate truncated");
                }
                let tenant_id = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                let token = u32::from_le_bytes(rest[off..off + 4].try_into().unwrap());
                off += 4;
                RaftCommand::RotateToken { tenant_id, token }
            }
            _ => return encode_err("append2 bad cmd"),
        };
        entries.push(LogEntry {
            term: eterm,
            index: eindex,
            command,
        });
    }

    if !entries.is_empty() {
        let first_new = entries[0].index;
        state.log.retain(|e| e.index < first_new);
        for entry in entries {
            if let Some(wal) = state.wal.as_mut() {
                let _ = wal.append_entry(&entry);
            }
            state.log.push(entry);
        }
    }

    if leader_commit > state.commit_index {
        state.commit_index = leader_commit.min(state.last_log_index());
        state.apply_through(state.commit_index);
    }

    let _ = leader_id;
    encode_append_ok_v2(state.current_term, true, state.last_log_index())
}

fn handle_raft_request(state: &mut RaftNodeState, req: &[u8]) -> Vec<u8> {
    if let Some(rest) = req.strip_prefix(b"VOTE\0") {
        if rest.len() < 28 {
            return encode_err("vote bad payload");
        }
        let term = u64::from_le_bytes(rest[0..8].try_into().unwrap());
        let candidate_id = u32::from_le_bytes(rest[8..12].try_into().unwrap());
        let last_log_index = u64::from_le_bytes(rest[12..20].try_into().unwrap());
        let last_log_term = u64::from_le_bytes(rest[20..28].try_into().unwrap());
        if term < state.current_term {
            return encode_vote_ok(state.current_term, false);
        }
        if term > state.current_term {
            state.current_term = term;
            state.voted_for = None;
            state.role = RaftRole::Follower;
        }
        let up_to_date = last_log_term > state.last_log_term()
            || (last_log_term == state.last_log_term() && last_log_index >= state.last_log_index());
        let can_vote =
            (state.voted_for.is_none() || state.voted_for == Some(candidate_id)) && up_to_date;
        if can_vote {
            state.voted_for = Some(candidate_id);
            state.role = RaftRole::Follower;
            state.reset_election_timeout();
            return encode_vote_ok(state.current_term, true);
        }
        return encode_vote_ok(state.current_term, false);
    }
    if let Some(rest) = req.strip_prefix(b"APPEND2\0") {
        return handle_append_entries_v2(state, rest);
    }
    // Toy v1 append — rejected for v2 nodes (log/WAL GA).
    if req.starts_with(b"APPEND\0") {
        return encode_err("append v1 rejected — use APPEND2 log path");
    }

    if req == b"HEALTH\0" {
        return encode_ok_none();
    }
    if req == b"MEMBERS\0" {
        // Any role may answer — followers observe consensus membership changes.
        let mut out = b"OK\0".to_vec();
        out.extend_from_slice(&(state.members.len() as u32).to_le_bytes());
        for m in &state.members {
            out.extend_from_slice(&m.id.to_le_bytes());
        }
        return out;
    }
    if req == b"RESET\0" {
        if state.role != RaftRole::Leader {
            return encode_err("not leader");
        }
        let term = state.current_term;
        let _ = state.append_local(term, RaftCommand::ClearDeadline);
        if !raft_replicate_log_majority(state) {
            return encode_err("no quorum");
        }
        if let Some(wal) = state.wal.as_ref() {
            if let Some(dir) = wal.path.parent() {
                load_token_epoch_into_auth(&mut state.auth, Some(dir));
            }
        }
        return encode_ok_none();
    }
    if let Some(rest) = req.strip_prefix(b"QUERY\0") {
        match parse_query_token_payload(rest) {
            Ok((tenant_id, token)) => {
                // Token mismatch rejects forged QUERY; 4-byte tenant-only QUERY still reads
                // deadline (token-default GA is GRANT-side — Phase 113).
                if rest.len() == 8 && !state.auth.check_token(tenant_id, token) {
                    return encode_err("token mismatch");
                }
                return match state.auth.deadline_for(tenant_id) {
                    Some(d) => encode_ok_deadline(d),
                    None => encode_ok_none(),
                };
            }
            Err(e) => return encode_err(&e),
        }
    }
    if req == b"CHECKPOINT\0" {
        if state.role != RaftRole::Leader {
            return encode_err("not leader");
        }
        let Some(wal) = state.wal.as_ref() else {
            return encode_err("wal not configured");
        };
        let Some(dir) = wal.path.parent().map(|p| p.to_path_buf()) else {
            return encode_err("wal path parent");
        };
        let node_id = state.node_id;
        if let Err(e) = NodeWal::checkpoint_from_state(&dir, node_id, state) {
            return encode_err(&e);
        }
        return encode_ok_none();
    }
    if let Some(rest) = req.strip_prefix(b"MEMBER\0") {
        if state.role != RaftRole::Leader {
            return encode_err("not leader");
        }
        if rest.len() < 5 {
            return encode_err("member bad payload");
        }
        let op = rest[0];
        let target_id = u32::from_le_bytes(rest[1..5].try_into().unwrap());
        let cmd = match op {
            1 => {
                let url = state
                    .members
                    .iter()
                    .find(|m| m.id == target_id)
                    .map(|m| m.url.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "tcp://127.0.0.1:{}",
                            FARM_LEASE_RAFT_PORTS
                                .get(target_id as usize)
                                .copied()
                                .unwrap_or(0)
                        )
                    });
                RaftCommand::AddLearner { id: target_id, url }
            }
            2 => RaftCommand::RemoveMember { id: target_id },
            _ => return encode_err("member unknown op"),
        };
        let term = state.current_term;
        let _ = state.append_local(term, cmd);
        if !raft_replicate_log_majority(state) {
            return encode_err("no quorum");
        }
        return encode_ok_none();
    }
    if let Some(rest) = req
        .strip_prefix(b"GRANT\0")
        .or_else(|| req.strip_prefix(b"RENEW\0"))
    {
        if state.role != RaftRole::Leader {
            return encode_err("not leader");
        }
        // Phase 113 token-default: always require tenant+token+ttl (abolish RUNTIME_GA-only GA).
        // Phase 117: registered token must match (rotation invalidates old GRANT).
        debug_assert!(raft_token_default_enforced());
        match parse_grant_token_payload(rest) {
            Ok((tenant_id, token, ttl)) => {
                if let Err(e) = state.auth.accept_grant_token(tenant_id, token) {
                    return encode_err(&e);
                }
                let deadline = deadline_from_ttl(ttl);
                let term = state.current_term;
                let _ = state.append_local(
                    term,
                    RaftCommand::SetDeadline {
                        tenant_id,
                        deadline_ms: deadline,
                        token,
                    },
                );
                if !raft_replicate_log_majority(state) {
                    return encode_err("no quorum");
                }
                return encode_ok_deadline(deadline);
            }
            Err(e) => {
                if rest.len() == 4 || rest.len() == 8 {
                    return encode_err("legacy grant rejected — token required");
                }
                return encode_err(&e);
            }
        }
    }
    if let Some(rest) = req.strip_prefix(b"ROTATE\0") {
        if state.role != RaftRole::Leader {
            return encode_err("not leader");
        }
        let (tenant_id, old_token, new_token) = match parse_rotate_hsm_payload(rest) {
            Ok(v) => v,
            Err(e) => return encode_err(&e),
        };
        match state.auth.token_for(tenant_id) {
            Some(t) if t == old_token => {}
            Some(_) => return encode_err("token mismatch — rotate rejected"),
            None => return encode_err("no token to rotate"),
        }
        let term = state.current_term;
        let _ = state.append_local(
            term,
            RaftCommand::RotateToken {
                tenant_id,
                token: new_token,
            },
        );
        if !raft_replicate_log_majority(state) {
            return encode_err("no quorum");
        }
        state.auth.set_token(tenant_id, new_token);
        if let Some(wal) = state.wal.as_ref() {
            persist_token_epoch_for_wal(wal, &state.auth.tenant_tokens);
        }
        return encode_ok_none();
    }
    encode_err("unknown lease op")
}

fn legacy_loopback_member_urls() -> Vec<String> {
    (0..3u32)
        .map(|id| format!("tcp://127.0.0.1:{}", FARM_LEASE_RAFT_PORTS[id as usize]))
        .collect()
}

fn initial_raft_members(
    member_urls: &[String],
    _runtime_config_ga: bool,
) -> Result<Vec<RaftMember>, String> {
    if member_urls.len() < 3 {
        return Err(format!(
            "farm lease requires at least 3 member urls, got {} (legacy loopback fallback banned)",
            member_urls.len()
        ));
    }
    Ok(member_urls
        .iter()
        .enumerate()
        .map(|(id, url)| RaftMember {
            id: id as u32,
            url: normalize_tcp_url(url),
            role: MemberRole::Voter,
        })
        .collect())
}

fn apply_replayed_log_entry(
    auth: &mut AuthorityState,
    members: &mut Vec<RaftMember>,
    entry: &LogEntry,
) {
    match &entry.command {
        RaftCommand::SetDeadline {
            tenant_id,
            deadline_ms,
            token,
        } => {
            auth.set_deadline(*tenant_id, *deadline_ms);
            auth.set_token(*tenant_id, *token);
        }
        RaftCommand::ClearDeadline => auth.clear_all(),
        RaftCommand::AddLearner { id, url } => {
            if !members.iter().any(|m| m.id == *id) {
                members.push(RaftMember {
                    id: *id,
                    url: normalize_tcp_url(url),
                    role: MemberRole::Learner,
                });
            }
        }
        RaftCommand::RemoveMember { id } => {
            members.retain(|m| m.id != *id);
        }
        RaftCommand::RotateToken { tenant_id, token } => {
            auth.set_token(*tenant_id, *token);
        }
    }
}

fn build_raft_state_from_wal(
    node_id: u32,
    member_urls: Vec<String>,
    wal_dir: Option<&Path>,
) -> RaftNodeState {
    let runtime_config_ga = std::env::var(ENV_RUNTIME_CONFIG_GA).ok().as_deref() == Some("1");
    let mut log = vec![sentinel_log()];
    let mut commit_index = 0u64;
    let mut last_applied = 0u64;
    let mut auth = AuthorityState::default();
    let mut members =
        initial_raft_members(&member_urls, runtime_config_ga).unwrap_or_else(|e| panic!("{e}"));
    let wal = wal_dir.map(|d| NodeWal::open(d, node_id).expect("raft wal open"));

    if let Some(dir) = wal_dir {
        if NodeWal::snapshot_path(dir, node_id).exists() {
            let snap = NodeWal::read_snapshot(dir, node_id)
                .expect("snap load")
                .expect("snap present");
            validate_snap_authority(&snap).expect("snap authority validation");
            commit_index = snap.commit_index;
            last_applied = snap.last_applied;
            auth.tenant_deadlines = snap.tenant_deadlines;
            auth.tenant_tokens = snap.tenant_tokens;
            members = snap.members;
            log = vec![compacted_base_entry(last_applied, 0)];
        } else {
            let replayed = NodeWal::replay(dir, node_id).expect("wal replay");
            for entry in &replayed {
                log.push(entry.clone());
            }
            if !replayed.is_empty() {
                commit_index = log.last().map_or(0, |e| e.index);
            }
            auth = AuthorityState::default();
            members = initial_raft_members(&member_urls, runtime_config_ga)
                .unwrap_or_else(|e| panic!("{e}"));
            for entry in log.iter().skip(1) {
                apply_replayed_log_entry(&mut auth, &mut members, entry);
                last_applied = entry.index;
            }
        }

        // Incremental only: entries with index > last_applied (old compacted log is not authority).
        let incremental = NodeWal::replay(dir, node_id).expect("wal incremental replay");
        for entry in &incremental {
            if entry.index > last_applied {
                log.push(entry.clone());
                apply_replayed_log_entry(&mut auth, &mut members, entry);
                last_applied = entry.index;
                commit_index = commit_index.max(entry.index);
            }
        }
        load_token_epoch_into_auth(&mut auth, wal_dir);
    }

    RaftNodeState {
        auth,
        current_term: 0,
        voted_for: None,
        role: RaftRole::Follower,
        election_deadline: Instant::now(),
        node_id,
        members,
        log,
        commit_index,
        last_applied,
        next_index: HashMap::new(),
        match_index: HashMap::new(),
        wal,
        last_heartbeat: Instant::now(),
    }
}

fn serve_raft_os_child(
    bind: &str,
    max_conns: usize,
    node_id: u32,
    member_urls: Vec<String>,
    ready_file: &Path,
    wal_dir: Option<PathBuf>,
) {
    let listener = {
        let mut last_err = None;
        let mut listener = None;
        for _ in 0..20 {
            match TcpListener::bind(bind) {
                Ok(l) => {
                    listener = Some(l);
                    break;
                }
                Err(e) => {
                    last_err = Some(e);
                    std::thread::sleep(Duration::from_millis(25));
                }
            }
        }
        listener.unwrap_or_else(|| {
            panic!(
                "farm lease raft child bind {bind}: {}",
                last_err
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "unknown".into())
            )
        })
    };
    listener
        .set_nonblocking(true)
        .expect("farm lease raft nonblocking");
    let addr = listener.local_addr().expect("farm lease raft local_addr");
    write_ready_file(ready_file, addr);

    let mut state = build_raft_state_from_wal(node_id, member_urls, wal_dir.as_deref());
    state.reset_election_timeout();

    let accept_deadline = Instant::now() + Duration::from_secs(5);
    let mut accepted = 0usize;
    while accepted < max_conns && Instant::now() < accept_deadline {
        if Instant::now() >= state.election_deadline && state.role != RaftRole::Leader {
            raft_start_election(&mut state);
        }
        if state.role == RaftRole::Leader
            && state.last_heartbeat.elapsed() >= Duration::from_millis(15)
        {
            for (pid, url) in state
                .members
                .iter()
                .filter(|m| m.role == MemberRole::Voter && m.id != state.node_id)
                .map(|m| (m.id, m.url.clone()))
                .collect::<Vec<_>>()
            {
                let _ = raft_send_append_v2(&mut state, pid, &url);
            }
            state.advance_commit_leader();
            state.last_heartbeat = Instant::now();
        }
        match listener.accept() {
            Ok((mut stream, _)) => {
                let _ = stream.set_read_timeout(Some(Duration::from_millis(300)));
                let _ = stream.set_write_timeout(Some(Duration::from_millis(300)));
                match tcp_read_frame(&mut stream) {
                    Ok(req) => {
                        let is_peer = req.starts_with(b"VOTE\0")
                            || req.starts_with(b"APPEND2\0")
                            || req.starts_with(b"APPEND\0");
                        if !is_peer {
                            accepted += 1;
                        }
                        let resp = handle_raft_request(&mut state, &req);
                        let _ = tcp_write_frame(&mut stream, &resp);
                    }
                    Err(_) => {}
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(2));
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => std::thread::sleep(Duration::from_millis(2)),
        }
    }
}

/// If this process was spawned as an OS lease-auth child, serve then return `true`.
/// Integration tests call this from `p109_farm_os_child_entry`.
pub fn farm_lease_os_child_try_serve() -> bool {
    let Ok(bind) = std::env::var(ENV_OS_CHILD_BIND) else {
        return false;
    };
    let max_conns: usize = std::env::var(ENV_OS_CHILD_MAX_CONNS)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let ready_path = std::env::var(ENV_OS_CHILD_READY_FILE)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::temp_dir().join(format!("syx-farm-lease-ready-{}", std::process::id()))
        });
    let mode = std::env::var(ENV_OS_CHILD_MODE).unwrap_or_else(|_| "ha".to_string());
    if mode == "raft" {
        let node_id: u32 = std::env::var(ENV_OS_CHILD_RAFT_ID)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let member_urls: Vec<String> = std::env::var(ENV_OS_CHILD_RAFT_MEMBER_URLS)
            .ok()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .filter(|p| !p.is_empty())
                    .map(|p| normalize_tcp_url(p))
                    .collect()
            })
            .unwrap_or_else(|| {
                std::env::var(ENV_OS_CHILD_RAFT_PEERS)
                    .unwrap_or_default()
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| normalize_tcp_url(s))
                    .collect()
            });
        let wal_dir = std::env::var(ENV_OS_CHILD_WAL_DIR).ok().map(PathBuf::from);
        serve_raft_os_child(&bind, max_conns, node_id, member_urls, &ready_path, wal_dir);
    } else {
        let peer = std::env::var(ENV_OS_CHILD_PEER)
            .ok()
            .filter(|s| !s.is_empty());
        serve_ha_os_child(&bind, max_conns, peer, &ready_path);
    }
    true
}

/// Dual-node HA lease cluster using **OS processes** (`std::process::Command`).
pub struct FarmLeaseHaV3Cluster {
    primary_addr: SocketAddr,
    secondary_addr: SocketAddr,
    primary_child: Option<Child>,
    secondary_child: Option<Child>,
}

impl FarmLeaseHaV3Cluster {
    pub fn primary_addr(&self) -> SocketAddr {
        self.primary_addr
    }

    pub fn secondary_addr(&self) -> SocketAddr {
        self.secondary_addr
    }

    /// Kill primary OS process (failover to secondary).
    pub fn stop_primary(&mut self) {
        if let Some(mut c) = self.primary_child.take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
    }

    /// Kill remaining children with a hard join deadline (no hung tests).
    pub fn join(mut self) {
        if let Some(mut c) = self.primary_child.take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
        if let Some(mut c) = self.secondary_child.take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
    }
}

/// Spawn primary + secondary as **OS processes**, each with independent `AuthorityState`.
///
/// Fingerprint: `farm-lease-ha-v3:os-process`. Same-process `spawn_farm_lease_ha` (threads+SYNC)
/// must not be treated as this GA.
pub fn spawn_farm_lease_ha_v3(max_conns: usize) -> FarmLeaseHaV3Cluster {
    let tmp = std::env::temp_dir();
    let sec_ready = tmp.join(format!("syx-farm-ha-v3-sec-{}.ready", std::process::id()));
    let pri_ready = tmp.join(format!("syx-farm-ha-v3-pri-{}.ready", std::process::id()));
    let _ = std::fs::remove_file(&sec_ready);
    let _ = std::fs::remove_file(&pri_ready);

    // Secondary first so primary can SYNC to a known peer.
    let mut secondary_child = spawn_os_authority_child(
        "ha",
        "127.0.0.1:0",
        max_conns,
        &sec_ready,
        None,
        None,
        None,
        None,
        None,
        false,
        false,
    );
    let secondary_addr = wait_child_ready(&mut secondary_child, &sec_ready, Duration::from_secs(3));
    let secondary_url = format!("tcp://{secondary_addr}");

    let mut primary_child = spawn_os_authority_child(
        "ha",
        "127.0.0.1:0",
        max_conns,
        &pri_ready,
        Some(&secondary_url),
        None,
        None,
        None,
        None,
        false,
        false,
    );
    let primary_addr = wait_child_ready(&mut primary_child, &pri_ready, Duration::from_secs(3));

    // Smoke: if child exited early with client capacity remaining, surface it.
    if max_conns > 0 {
        if let Ok(Some(status)) = secondary_child.try_wait() {
            panic!("HA v3 secondary exited early: {status}");
        }
        if let Ok(Some(status)) = primary_child.try_wait() {
            panic!("HA v3 primary exited early: {status}");
        }
    }

    FarmLeaseHaV3Cluster {
        primary_addr,
        secondary_addr,
        primary_child: Some(primary_child),
        secondary_child: Some(secondary_child),
    }
}

impl Drop for FarmLeaseHaV3Cluster {
    fn drop(&mut self) {
        if let Some(mut c) = self.primary_child.take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
        if let Some(mut c) = self.secondary_child.take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
    }
}

/// N-node Raft lease cluster (OS processes, short election timeouts; N ≥ 3).
pub struct FarmLeaseRaftCluster {
    addrs: Vec<SocketAddr>,
    children: Vec<Option<Child>>,
    leader_idx: usize,
    wal_dir: PathBuf,
    ready_paths: Vec<PathBuf>,
    member_binds: Vec<String>,
    member_urls: Vec<String>,
    /// Phase 115: configured region-tagged members (runtime spawn only).
    member_regions: Vec<FarmMemberRegion>,
    /// Phase 115: active members after multidc HEALTH probe (runtime spawn only).
    active_member_regions: Vec<FarmMemberRegion>,
    /// Phase 111 multihost GA (compile-time host:port members — not Phase 112 runtime GA).
    multihost_ga: bool,
    /// Phase 112 runtime config GA ([`ENV_FARM_LEASE_MEMBERS`] / file JSON).
    runtime_config_ga: bool,
    /// Phase 113 dyn-list GA (variable `Vec` members — not fixed `[String;3]` constants).
    dyn_members_ga: bool,
}

impl FarmLeaseRaftCluster {
    pub fn addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }

    pub fn wal_dir(&self) -> &Path {
        &self.wal_dir
    }

    pub fn leader_addr(&self) -> SocketAddr {
        self.addrs[self.leader_idx]
    }

    pub fn leader_idx(&self) -> usize {
        self.leader_idx
    }

    pub fn member_count(&self) -> usize {
        self.addrs.len()
    }

    /// True when spawned via [`spawn_farm_lease_raft_multihost`] (compile-time — not runtime GA).
    pub fn is_multihost_ga(&self) -> bool {
        self.multihost_ga
    }

    /// True when spawned via [`spawn_farm_lease_raft_runtime`] (env/file JSON members).
    pub fn is_runtime_config_ga(&self) -> bool {
        self.runtime_config_ga
    }

    /// True when spawned via dyn-list runtime members (`Vec`, len ≥ 3 — not fixed ports GA).
    pub fn is_dyn_members_ga(&self) -> bool {
        self.dyn_members_ga
    }

    /// Phase 118: true when ≥2 live OS child processes with distinct PIDs satisfy os-dual-process GA.
    pub fn is_os_dual_process_ga(&self) -> bool {
        farm_lease_cluster_os_dual_process_ga(self)
    }

    /// Issue WAL snapshot checkpoint on the current leader (Phase 112/113 compaction).
    pub fn wal_checkpoint(&self) -> Result<(), String> {
        let url = format!("tcp://{}", self.leader_addr());
        let payload = lease_auth_rpc_one(&url, b"CHECKPOINT\0")?;
        if payload.starts_with(b"ERR\0") {
            return Err(String::from_utf8_lossy(
                payload.strip_prefix(b"ERR\0").unwrap_or(&payload),
            )
            .into_owned());
        }
        Ok(())
    }

    pub fn member_urls(&self) -> &[String] {
        &self.member_urls
    }

    /// Configured region-tagged members (runtime GA only).
    pub fn member_regions(&self) -> &[FarmMemberRegion] {
        &self.member_regions
    }

    /// Active members after multidc discovery probe (runtime GA only).
    pub fn active_member_regions(&self) -> &[FarmMemberRegion] {
        &self.active_member_regions
    }

    /// Phase 117: reload env/file members, HEALTH prune, consensus MEMBER sync + follower observe.
    pub fn reload_active_members(
        &mut self,
        probe_timeout: Duration,
    ) -> Result<Vec<FarmMemberRegion>, String> {
        farm_lease_registry_consensus_sync(self, probe_timeout)
    }

    /// Kill the current leader OS process; remaining nodes elect a new leader.
    pub fn kill_leader(&mut self) {
        if let Some(mut c) = self.children[self.leader_idx].take() {
            kill_child_deadline(&mut c, Duration::from_secs(1));
        }
    }

    /// Kill node by index. Used to force quorum loss in tests.
    pub fn kill_node(&mut self, idx: usize) {
        if idx < self.children.len() {
            if let Some(mut c) = self.children[idx].take() {
                kill_child_deadline(&mut c, Duration::from_secs(1));
            }
        }
    }

    /// Restart a killed node from its on-disk WAL/snapshot (Phase 110/113 catch-up gate).
    pub fn restart_node(&mut self, idx: usize, max_conns: usize) -> Result<SocketAddr, String> {
        if idx >= self.children.len() {
            return Err("restart_node: bad index".into());
        }
        if self.children[idx].is_some() {
            return Ok(self.addrs[idx]);
        }
        let n = self.member_binds.len();
        let bind = self.member_binds[idx].clone();
        let peers: Vec<String> = (0..n)
            .filter(|&j| j != idx)
            .map(|j| self.member_urls[j].clone())
            .collect();
        let peers_joined = peers.join(",");
        let members_joined = self.member_urls.join(",");
        let ready = self.ready_paths[idx].clone();
        let _ = fs::remove_file(&ready);
        let mut child = spawn_os_authority_child(
            "raft",
            &bind,
            max_conns,
            &ready,
            None,
            Some(idx as u32),
            Some(&peers_joined),
            Some(&members_joined),
            Some(&self.wal_dir),
            self.runtime_config_ga,
            self.dyn_members_ga,
        );
        let addr = wait_child_ready(&mut child, &ready, Duration::from_secs(3));
        self.addrs[idx] = addr;
        self.children[idx] = Some(child);
        Ok(addr)
    }

    /// Count of still-running OS node processes.
    pub fn live_count(&self) -> usize {
        self.children.iter().filter(|c| c.is_some()).count()
    }

    /// Probe which node accepts GRANT (is leader). Updates `leader_idx`.
    /// Phase 113: always uses token probe (legacy anonymous GRANT rejected).
    pub fn refresh_leader(&mut self, ttl_secs: u32) -> Result<SocketAddr, String> {
        let deadline = Instant::now() + Duration::from_millis(1500);
        let req = encode_leader_probe_grant(ttl_secs);
        while Instant::now() < deadline {
            for (i, addr) in self.addrs.iter().enumerate() {
                if self.children[i].is_none() {
                    continue;
                }
                let url = format!("tcp://{addr}");
                match lease_auth_rpc_one(&url, &req) {
                    Ok(resp) if resp.starts_with(b"OK\0") => {
                        self.leader_idx = i;
                        // Reset so callers can GRANT cleanly after discovery.
                        let _ = lease_auth_rpc_one(&url, b"RESET\0");
                        return Ok(*addr);
                    }
                    _ => {}
                }
            }
            std::thread::sleep(Duration::from_millis(25));
        }
        Err("raft: no leader elected within deadline".into())
    }

    pub fn join(mut self) {
        for slot in self.children.iter_mut() {
            if let Some(mut c) = slot.take() {
                kill_child_deadline(&mut c, Duration::from_secs(1));
            }
        }
    }
}

/// Spawn 3 OS-process Raft nodes on [`FARM_LEASE_RAFT_PORTS`].
///
/// Phase 110: replicated log + WAL fsync + commitIndex. **Not** Phase 111 multihost GA
/// (hardcoded loopback ports — use [`spawn_farm_lease_raft_multihost`]).
/// Phase 113: **not** dyn-list GA (fixed three-port constants).
pub fn spawn_farm_lease_raft(max_conns: usize) -> FarmLeaseRaftCluster {
    let binds: Vec<String> = FARM_LEASE_RAFT_PORTS
        .iter()
        .map(|port| format!("127.0.0.1:{port}"))
        .collect();
    let member_urls: Vec<String> = binds.iter().map(|b| format!("tcp://{b}")).collect();
    spawn_farm_lease_raft_internal(
        &binds,
        &member_urls,
        max_conns,
        false,
        false,
        false,
        Vec::new(),
    )
}

/// Phase 111: spawn 3-node Raft with compile-time host:port members (not Phase 112 runtime GA).
/// Phase 113: **not** dyn-list GA (fixed `[String;3]` / const binds).
pub fn spawn_farm_lease_raft_multihost(max_conns: usize) -> FarmLeaseRaftCluster {
    let binds: Vec<String> = FARM_LEASE_RAFT_MULTIHOST_BINDS
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let member_urls: Vec<String> = binds.iter().map(|b| format!("tcp://{b}")).collect();
    spawn_farm_lease_raft_internal(
        &binds,
        &member_urls,
        max_conns,
        true,
        false,
        false,
        Vec::new(),
    )
}

/// Phase 112/113/115/116/117/118: spawn Raft from runtime env/file JSON member list (`Vec`, len ≥ 3).
/// Phase 118: requires os-dual-process GA (≥2 non-loopback connect + ban TEST-NET hostkey-only);
/// cluster nodes are independent OS processes (not same-process multi-bind).
pub fn spawn_farm_lease_raft_runtime(max_conns: usize) -> FarmLeaseRaftCluster {
    let member_regions = load_runtime_member_regions().unwrap_or_else(|e| panic!("{e}"));
    let binds: Vec<String> = member_regions.iter().map(|m| m.bind.clone()).collect();
    let member_urls: Vec<String> = binds.iter().map(|b| format!("tcp://{b}")).collect();
    let mut cluster = spawn_farm_lease_raft_internal(
        &binds,
        &member_urls,
        max_conns,
        false,
        true,
        true,
        member_regions.clone(),
    );
    let probe_timeout = farm_lease_multidc_probe_timeout();
    let active = farm_lease_multidc_discover_active(&member_regions, probe_timeout);
    if active.len() < 3 {
        cluster.join();
        panic!(
            "farm lease runtime multidc discover: need at least 3 active members, got {}",
            active.len()
        );
    }
    cluster.member_regions = member_regions;
    cluster.active_member_regions = active;
    cluster
}

fn spawn_farm_lease_raft_internal(
    member_binds: &[String],
    member_urls: &[String],
    max_conns: usize,
    multihost_ga: bool,
    runtime_config_ga: bool,
    dyn_members_ga: bool,
    member_regions: Vec<FarmMemberRegion>,
) -> FarmLeaseRaftCluster {
    assert!(
        member_binds.len() >= 3 && member_binds.len() == member_urls.len(),
        "raft cluster requires ≥3 matching member binds/urls"
    );
    let n = member_binds.len();
    let tmp = std::env::temp_dir();
    let pid = std::process::id();
    let tag = if dyn_members_ga {
        "dyn"
    } else if runtime_config_ga {
        "runtime"
    } else if multihost_ga {
        "multihost"
    } else {
        "loopback"
    };
    let wal_dir = tmp.join(format!("syx-farm-raft-wal-{tag}-{pid}"));
    let _ = fs::remove_dir_all(&wal_dir);
    fs::create_dir_all(&wal_dir).expect("raft wal dir");
    let mut addrs = vec![SocketAddr::from(([127, 0, 0, 1], 0)); n];
    let mut children: Vec<Option<Child>> = (0..n).map(|_| None).collect();
    let mut ready_paths = vec![PathBuf::new(); n];
    let members_joined = member_urls.join(",");

    for id in 0..n {
        let bind = &member_binds[id];
        let ready = tmp.join(format!("syx-farm-raft-{tag}-{pid}-{id}.ready"));
        ready_paths[id] = ready.clone();
        let _ = fs::remove_file(&ready);
        let peers: Vec<String> = (0..n)
            .filter(|&j| j != id)
            .map(|j| member_urls[j].clone())
            .collect();
        let peers_joined = peers.join(",");
        let mut child = spawn_os_authority_child(
            "raft",
            bind,
            max_conns,
            &ready,
            None,
            Some(id as u32),
            Some(&peers_joined),
            Some(&members_joined),
            Some(&wal_dir),
            runtime_config_ga,
            dyn_members_ga,
        );
        let addr = wait_child_ready(&mut child, &ready, Duration::from_secs(3));
        addrs[id] = addr;
        children[id] = Some(child);
    }

    let resolved_urls: Vec<String> = addrs.iter().map(|a| format!("tcp://{a}")).collect();

    let mut cluster = FarmLeaseRaftCluster {
        addrs,
        children,
        leader_idx: 0,
        wal_dir,
        ready_paths,
        member_binds: member_binds.to_vec(),
        member_urls: resolved_urls,
        member_regions,
        active_member_regions: Vec::new(),
        multihost_ga,
        runtime_config_ga,
        dyn_members_ga,
    };
    if max_conns > 0 {
        if let Err(e) = cluster.refresh_leader(30) {
            cluster.join();
            panic!("raft initial leader election: {e}");
        }
    }
    cluster
}

impl Drop for FarmLeaseRaftCluster {
    fn drop(&mut self) {
        for slot in self.children.iter_mut() {
            if let Some(mut c) = slot.take() {
                kill_child_deadline(&mut c, Duration::from_secs(1));
            }
        }
    }
}

static TEST_AUTH_GUARD: OnceLock<Mutex<()>> = OnceLock::new();

/// Query lease deadline at a specific authority URL (tests / post-restart validation).
pub fn farm_lease_query_at(url: &str) -> Result<Option<u64>, String> {
    farm_lease_query_at_tenant(url, 0)
}

/// Phase 111: tenant-scoped QUERY at a specific authority URL.
pub fn farm_lease_query_at_tenant(url: &str, tenant_id: u32) -> Result<Option<u64>, String> {
    let payload = lease_auth_rpc_one(url, &encode_query(tenant_id))?;
    if let Some(msg) = payload.strip_prefix(b"ERR\0") {
        return Err(String::from_utf8_lossy(msg).into_owned());
    }
    parse_ok_deadline(&payload)
}

/// Phase 111: replay node WAL (tests — corrupt records must ERR).
pub fn farm_lease_wal_replay(dir: &Path, node_id: u32) -> Result<usize, String> {
    NodeWal::replay(dir, node_id).map(|entries| entries.len())
}

/// Phase 112: load snapshot (corrupt snapshot must ERR).
pub fn farm_lease_wal_read_snapshot(dir: &Path, node_id: u32) -> Result<(), String> {
    match NodeWal::read_snapshot(dir, node_id)? {
        Some(snap) => {
            validate_snap_authority(&snap)?;
            Ok(())
        }
        None => Err("snapshot missing".into()),
    }
}

/// Phase 114: reject snapshots with tenant deadlines but missing tokens.
pub fn farm_lease_wal_validate_snapshot_authority(dir: &Path, node_id: u32) -> Result<(), String> {
    match NodeWal::read_snapshot(dir, node_id)? {
        Some(snap) => validate_snap_authority(&snap),
        None => Err("snapshot missing".into()),
    }
}

/// Phase 113: embedded log entry count in snapshot (compact GA ⇒ 0 discardable entries).
pub fn farm_lease_wal_snapshot_log_len(dir: &Path, node_id: u32) -> Result<usize, String> {
    match NodeWal::read_snapshot(dir, node_id)? {
        Some(snap) => Ok(snap.log.len()),
        None => Err("snapshot missing".into()),
    }
}

fn encode_rotate_token(tenant_id: u32, old_token: u32, new_token: u32, hsm_ref: &str) -> Vec<u8> {
    let mut out = b"ROTATE\0".to_vec();
    out.extend_from_slice(&tenant_id.to_le_bytes());
    out.extend_from_slice(&old_token.to_le_bytes());
    out.extend_from_slice(&new_token.to_le_bytes());
    let ref_bytes = hsm_ref.as_bytes();
    let ref_len = u32::try_from(ref_bytes.len()).unwrap_or(u32::MAX);
    out.extend_from_slice(&ref_len.to_le_bytes());
    out.extend_from_slice(ref_bytes);
    out
}

/// Phase 117: rotate tenant token (old GRANT fails after rotate).
pub fn farm_lease_rotate_tenant_token(
    tenant_id: u32,
    old_token: u32,
    new_token: u32,
) -> Result<(), String> {
    let _ = farm_lease_token_fingerprint();
    let pkcs11_ref = farm_lease_pkcs11_bridge_sign_rotate(tenant_id, old_token, new_token)?;
    let req = encode_rotate_token(tenant_id, old_token, new_token, &pkcs11_ref);
    let payload = lease_auth_rpc(&req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

/// Phase 112: acquire lease with tenant token (runtime GA path).
pub fn farm_lease_acquire_tenant_token(
    tenant_id: u32,
    token: u32,
    ttl_secs: u32,
) -> Result<(), String> {
    let req = encode_grant_token(tenant_id, token, ttl_secs);
    let payload = lease_auth_rpc(&req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    parse_ok_deadline(&payload).map(|_| ())
}

/// Phase 112: tenant+token QUERY at a specific authority URL.
pub fn farm_lease_query_at_tenant_token(
    url: &str,
    tenant_id: u32,
    token: u32,
) -> Result<Option<u64>, String> {
    let payload = lease_auth_rpc_one(url, &encode_query_token(tenant_id, token))?;
    if let Some(msg) = payload.strip_prefix(b"ERR\0") {
        return Err(String::from_utf8_lossy(msg).into_owned());
    }
    parse_ok_deadline(&payload)
}

/// Phase 112: WAL snapshot checkpoint via leader CHECKPOINT RPC.
pub fn farm_lease_wal_checkpoint(url: &str) -> Result<(), String> {
    let payload = lease_auth_rpc_one(url, b"CHECKPOINT\0")?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

/// Phase 110: append AddLearner membership change via Raft leader.
pub fn farm_lease_raft_member_add_learner(id: u32) -> Result<(), String> {
    let mut req = b"MEMBER\0".to_vec();
    req.push(1);
    req.extend_from_slice(&id.to_le_bytes());
    let payload = lease_auth_rpc(&req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

/// Phase 110: append Remove membership change via Raft leader.
pub fn farm_lease_raft_member_remove(id: u32) -> Result<(), String> {
    let mut req = b"MEMBER\0".to_vec();
    req.push(2);
    req.extend_from_slice(&id.to_le_bytes());
    let payload = lease_auth_rpc(&req)?;
    if payload.starts_with(b"ERR\0") {
        return Err(
            String::from_utf8_lossy(payload.strip_prefix(b"ERR\0").unwrap_or(&payload))
                .into_owned(),
        );
    }
    Ok(())
}

fn farm_lease_pkcs11_test_reset_session_state() {
    FARM_PKCS11_SESSION_ROTATE_COUNT.store(0, Ordering::Relaxed);
    FARM_PKCS11_PIN_PERSISTED.store(false, Ordering::Relaxed);
    FARM_PKCS11_SESSION_SLOT_ID.store(0, Ordering::Relaxed);
    farm_lease_pkcs11_multi_slot_clear();
    *FARM_PKCS11_HSM_REF_BOUND.lock().unwrap() = None;
    farm_lease_pkcs11_clear_last_sign();
    farm_lease_pkcs11_clear_last_digest();
}

/// Serialize tests that bind lease authority + farm TCP nodes.
pub fn farm_lease_test_lock() -> std::sync::MutexGuard<'static, ()> {
    let guard = TEST_AUTH_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    farm_lease_pkcs11_test_reset_session_state();
    guard
}
