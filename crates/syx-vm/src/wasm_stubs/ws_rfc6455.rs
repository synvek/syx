//! wasm32 stub for `ws_rfc6455.rs` — native C/TLS/sqlite hosts unavailable.
#![allow(unused_variables)]

pub fn rfc6455_connect(url: &str) -> Option<i64> {
    None
}

pub fn rfc6455_send(conn_id: i64, message: &str) -> bool {
    false
}

pub fn rfc6455_recv(conn_id: i64) -> Option<String> {
    None
}

pub fn rfc6455_close_exchange(conn_id: i64) -> bool {
    false
}

pub fn rfc6455_close(conn_id: i64) {
}
pub fn e01_rfc6455_loopback_roundtrip_ok() -> bool {
    false
}

pub fn e01_rfc6455_negative_handshake_fail_closed_ok() -> bool {
    false
}

pub fn e01_rfc6455_close_path_ok() -> bool {
    false
}

pub fn e01_rfc6455_negative_tests_first_slice_ok() -> bool {
    false
}

pub fn e01_rfc6455_loopback_peer_echo_wired_ok() -> bool {
    false
}

pub fn e01_rfc6455_interop_first_slice_ok() -> bool {
    false
}

pub fn e01_rfc6455_wss_loopback_wired_ok() -> bool {
    false
}

pub fn e01_rfc6455_wss_first_slice_ok() -> bool {
    false
}

pub fn rfc6455_connect_wss(url: &str) -> Option<i64> {
    None
}

pub fn rfc6455_send_any(conn_id: i64, message: &str) -> bool {
    false
}

