//! Narrow RFC6455 client (W1144 / ADR-1490).
//! `ws://`: HTTP upgrade + masked text frames.
//! W1147 / ADR-1493: loopback `wss://` evidence via rustls + RFC6455 (test cert).
//! Opt-in via `SYX_WS_REAL=1` with mock disabled. Default path remains mock.
//! ≠ product WebSocket GA / ≠ Autobahn / ≠ external peer suite.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

struct Conn {
    stream: TcpStream,
}

static CONNS: OnceLock<Mutex<HashMap<i64, Conn>>> = OnceLock::new();
static NEXT_ID: AtomicI64 = AtomicI64::new(1);

fn conns() -> &'static Mutex<HashMap<i64, Conn>> {
    CONNS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Parse `ws://host[:port][/path]` into (host, port, path).
fn parse_ws_url(url: &str) -> Option<(String, u16, String)> {
    let rest = url.strip_prefix("ws://")?;
    let (authority, path) = match rest.split_once('/') {
        Some((a, p)) => (a, format!("/{p}")),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return None;
    }
    let (host, port) = if let Some((h, p)) = authority.rsplit_once(':') {
        // IPv6 in brackets: [::1]:port
        if h.starts_with('[') {
            let host = h.trim_start_matches('[').trim_end_matches(']').to_string();
            let port: u16 = p.parse().ok()?;
            (host, port)
        } else if h.contains(':') {
            // bare IPv6 without brackets — reject for this slice
            return None;
        } else {
            (h.to_string(), p.parse().ok()?)
        }
    } else {
        (authority.to_string(), 80)
    };
    Some((host, port, path))
}

fn b64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            T[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            T[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// Minimal SHA-1 (RFC 3174) for WebSocket accept key only.
fn sha1(message: &[u8]) -> [u8; 20] {
    fn rotl(x: u32, n: u32) -> u32 {
        x.rotate_left(n)
    }
    let bit_len = (message.len() as u64) * 8;
    let mut msg = message.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0: u32 = 0x6745_2301;
    let mut h1: u32 = 0xEFCD_AB89;
    let mut h2: u32 = 0x98BA_DCFE;
    let mut h3: u32 = 0x1032_5476;
    let mut h4: u32 = 0xC3D2_E1F0;

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = rotl(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        for i in 0..80 {
            let (f, k) = if i < 20 {
                ((b & c) | ((!b) & d), 0x5A82_7999)
            } else if i < 40 {
                (b ^ c ^ d, 0x6ED9_EBA1)
            } else if i < 60 {
                ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC)
            } else {
                (b ^ c ^ d, 0xCA62_C1D6)
            };
            let temp = rotl(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = rotl(b, 30);
            b = a;
            a = temp;
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}

/// RFC6455 §1.3 Accept = base64(SHA1(key + GUID)).
/// W1182 / ADR-1528: `Http.serve` upgradeTerminate first slice.
pub(crate) fn accept_key(client_key_b64: &str) -> String {
    let mut material = Vec::with_capacity(client_key_b64.len() + WS_GUID.len());
    material.extend_from_slice(client_key_b64.as_bytes());
    material.extend_from_slice(WS_GUID.as_bytes());
    b64_encode(&sha1(&material))
}

/// After 101: read one client-masked text frame, echo as unmasked server text.
/// W1182 first slice — ≠ full codec.
pub(crate) fn echo_one_client_text_frame(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_data_frame(stream, 0x1, 0x81)
}

/// W1193–W1196: fragmented text; cont loop (N≥2; nails N=4/N=5 + mid↔final ping).
/// Frame1: FIN=0 + opcode 0x1; optional ping(s) between conts; until FIN=1 → `0x81`.
/// ≠ unbounded-N matrix GA / ≠ extension / ≠ full codec.
pub(crate) fn echo_one_client_fragmented_text(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_fragmented_data(stream, 0x1, 0x81, true)
}

/// W1194–W1196: fragmented binary; cont loop (N≥2; nails N=4/N=5 + mid↔final ping).
/// Frame1: FIN=0 + opcode 0x2; optional ping(s) between conts; until FIN=1 → `0x82`.
/// ≠ unbounded-N matrix GA / ≠ extension / ≠ full codec.
pub(crate) fn echo_one_client_fragmented_binary(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_fragmented_data(stream, 0x2, 0x82, true)
}

/// W1191–W1196: first fragment + zero-or-more FIN=0 cont + final FIN=1 cont.
/// W1207: FIN=1 complete first frame accepted (single-frame message); text UTF-8 required.
/// W1208: reject unexpected continuation / unsolicited client pong on first frame.
/// When `allow_control_ping`, opcode `0x9` anywhere before final is answered with pong.
fn echo_one_client_fragmented_data(
    stream: &mut (impl Read + Write),
    first_opcode: u8,
    server_fin_opcode: u8,
    allow_control_ping: bool,
) -> std::io::Result<()> {
    let (fin1, op1, part1) = read_client_ws_frame(stream)?;
    // W1208: first-frame negative gates (before FIN=1 short-circuit / FIN=0 expect).
    reject_unexpected_continuation(op1)?;
    reject_unsolicited_client_pong(op1)?;
    // W1207: FIN=1 + matching opcode = complete single-frame message (RFC6455 §5.4).
    if fin1 && op1 == first_opcode {
        if first_opcode == 0x1 {
            require_text_payload_utf8(&part1)?;
        }
        return write_unmasked_server_frame(stream, server_fin_opcode, &part1);
    }
    if fin1 || op1 != first_opcode {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "expected FIN=0 first fragment opcode {first_opcode:#x}, got fin={fin1} opcode={op1:#x}"
            ),
        ));
    }
    let mut payload = part1;
    let mut got_continuation = false;
    loop {
        let (fin, op, part) = read_client_ws_frame(stream)?;
        if op == 0x9 {
            if !allow_control_ping {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unexpected ping during fragment reassembly",
                ));
            }
            require_control_fin(fin)?;
            write_unmasked_server_frame(stream, 0x8A, &part)?;
            continue;
        }
        if op != 0x0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("expected continuation opcode 0x0, got fin={fin} opcode={op:#x}"),
            ));
        }
        payload.extend_from_slice(&part);
        got_continuation = true;
        if fin {
            break;
        }
    }
    if !got_continuation {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "expected at least one continuation frame after first fragment",
        ));
    }
    // W1207 / ADR-1553: reassembled text must be UTF-8 before echo.
    if first_opcode == 0x1 {
        require_text_payload_utf8(&payload)?;
    }
    write_unmasked_server_frame(stream, server_fin_opcode, &payload)
}

/// W1209 / ADR-1555: RFC6455 §7.4.1 — protocol error → unmasked close(status=1002).
/// Called from terminate exchange outer exit on `InvalidData`; ≠ close-code GA / ≠ extension.
fn write_protocol_close_1002(stream: &mut (impl Read + Write)) -> std::io::Result<()> {
    write_unmasked_server_frame(stream, 0x88, &1002u16.to_be_bytes())
}

/// On protocol `InvalidData`, write close(1002) then re-raise (terminate product reply).
fn map_invalid_data_to_protocol_close_1002(
    stream: &mut (impl Read + Write),
    result: std::io::Result<()>,
) -> std::io::Result<()> {
    match result {
        Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
            let _ = write_protocol_close_1002(stream);
            Err(e)
        }
        other => other,
    }
}

/// W1198 / ADR-1544: RFC6455 RSV1–3 must be 0 unless an extension is negotiated.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ extension negotiation GA.
fn reject_nonzero_rsv(hdr0: u8) -> std::io::Result<()> {
    if hdr0 & 0x70 != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("nonzero RSV bits in WebSocket frame header {hdr0:#x}"),
        ));
    }
    Ok(())
}

/// W1200 / ADR-1546: RFC6455 §5.1 — client→server frames MUST be masked.
/// Fail-closed (`InvalidData` → terminate writes close(1002)).
fn require_client_mask(masked: bool) -> std::io::Result<()> {
    if !masked {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "client WebSocket frame must be masked",
        ));
    }
    Ok(())
}

/// W1201 / ADR-1547: RFC6455 §5.4 — control frames MUST NOT be fragmented (FIN=1).
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ extension.
fn require_control_fin(fin: bool) -> std::io::Result<()> {
    if !fin {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "control WebSocket frame must have FIN=1",
        ));
    }
    Ok(())
}

/// W1202 / ADR-1548: RFC6455 reserved control opcodes 0xB–0xF must be rejected.
/// W1203 / ADR-1549: also reject data-reserved opcodes 0x3–0x7.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ extension.
fn reject_reserved_opcode(opcode: u8) -> std::io::Result<()> {
    if (0x3..=0x7).contains(&opcode) || opcode >= 0xB {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("reserved WebSocket opcode {opcode:#x}"),
        ));
    }
    Ok(())
}

/// W1204 / ADR-1550: RFC6455 §5.5 — control frames MUST have payload length ≤ 125
/// (`len7` must not be 126/127). Call before parsing extended length.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ extension.
fn require_control_payload_len7(opcode: u8, len7: u8) -> std::io::Result<()> {
    if matches!(opcode, 0x8 | 0x9 | 0xA) && len7 > 125 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("control WebSocket frame payload len7={len7} exceeds 125"),
        ));
    }
    Ok(())
}

/// W1205 / ADR-1551: RFC6455 §7.4.1 — close status MUST NOT be 0–999 / 1004 / 1005 / 1006 / 1015.
/// W1207 / ADR-1553: also reject reserved status 1004.
/// Call when close body `len≥2`. Fail-closed (`InvalidData` → terminate writes close(1002)).
fn reject_illegal_close_status(payload: &[u8]) -> std::io::Result<()> {
    if payload.len() < 2 {
        return Ok(());
    }
    let status = u16::from_be_bytes([payload[0], payload[1]]);
    let illegal =
        (0..=999).contains(&status) || matches!(status, 1004 | 1005 | 1006 | 1015);
    if illegal {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("illegal WebSocket close status {status}"),
        ));
    }
    Ok(())
}

/// W1206 / ADR-1552: RFC6455 §5.5.1 — close reason (`payload[2..]`) MUST be UTF-8 when present.
/// `len≤2` (empty or status-only) → Ok. Fail-closed (`InvalidData` → terminate writes close(1002)).
fn require_close_reason_utf8(payload: &[u8]) -> std::io::Result<()> {
    if payload.len() <= 2 {
        return Ok(());
    }
    if std::str::from_utf8(&payload[2..]).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "close reason must be valid UTF-8",
        ));
    }
    Ok(())
}

/// W1207 / ADR-1553: RFC6455 §5.6 — text frame payload MUST be valid UTF-8.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ UTF-8 text product GA.
fn require_text_payload_utf8(payload: &[u8]) -> std::io::Result<()> {
    if std::str::from_utf8(payload).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "text WebSocket payload must be valid UTF-8",
        ));
    }
    Ok(())
}

/// W1208 / ADR-1554: RFC6455 §5.4 — continuation (0x0) MUST NOT start a message.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ extension.
fn reject_unexpected_continuation(opcode: u8) -> std::io::Result<()> {
    if opcode == 0x0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unexpected continuation WebSocket frame (no message in progress)",
        ));
    }
    Ok(())
}

/// W1208 / ADR-1554: RFC6455 §5.5.3 — unsolicited client pong (0xA) as message start → reject.
/// Fail-closed (`InvalidData` → terminate writes close(1002)); ≠ keepalive GA.
fn reject_unsolicited_client_pong(opcode: u8) -> std::io::Result<()> {
    if opcode == 0xA {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unsolicited client pong WebSocket frame",
        ));
    }
    Ok(())
}

fn read_client_ws_frame(
    stream: &mut (impl Read + Write),
) -> std::io::Result<(bool, u8, Vec<u8>)> {
    let mut hdr = [0u8; 2];
    read_exact_io(stream, &mut hdr)?;
    reject_nonzero_rsv(hdr[0])?;
    let fin = (hdr[0] & 0x80) != 0;
    let opcode = hdr[0] & 0x0F;
    reject_reserved_opcode(opcode)?;
    // W1208 / ADR-1554: centralize control FIN=1 for close/ping/pong on shared read path.
    if matches!(opcode, 0x8 | 0x9 | 0xA) {
        require_control_fin(fin)?;
    }
    let len7 = hdr[1] & 0x7F;
    require_control_payload_len7(opcode, len7)?;
    let mut len = len7 as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_io(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_io(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let masked = (hdr[1] & 0x80) != 0;
    require_client_mask(masked)?;
    let mut mask = [0u8; 4];
    if masked {
        read_exact_io(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_io(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    Ok((fin, opcode, payload))
}

fn write_unmasked_server_frame(
    stream: &mut (impl Read + Write),
    fin_opcode: u8,
    payload: &[u8],
) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(2 + payload.len() + 8);
    out.push(fin_opcode);
    if payload.len() < 126 {
        out.push(payload.len() as u8);
    } else if payload.len() <= 0xFFFF {
        out.push(126);
        out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        out.push(127);
        out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    }
    out.extend_from_slice(payload);
    stream.write_all(&out)?;
    stream.flush()?;
    Ok(())
}

/// After 101: read one client-masked binary frame, echo as unmasked server binary.
/// W1184 / ADR-1530 first slice — ≠ fragment / ≠ full codec.
pub(crate) fn echo_one_client_binary_frame(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_data_frame(stream, 0x2, 0x82)
}

/// W1185 / ADR-1531: read client ping (opcode 9) → reply unmasked pong (`0x8A` + same payload).
/// ≠ fragment / ≠ keepalive matrix / ≠ full codec.
pub(crate) fn reply_client_ping_with_pong(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    let mut hdr = [0u8; 2];
    read_exact_io(stream, &mut hdr)?;
    reject_nonzero_rsv(hdr[0])?;
    let fin = (hdr[0] & 0x80) != 0;
    require_control_fin(fin)?;
    let opcode = hdr[0] & 0x0F;
    reject_reserved_opcode(opcode)?;
    if opcode != 0x9 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("expected client ping opcode 0x9, got {opcode:#x}"),
        ));
    }
    let len7 = hdr[1] & 0x7F;
    require_control_payload_len7(opcode, len7)?;
    let mut len = len7 as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_io(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_io(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let masked = (hdr[1] & 0x80) != 0;
    require_client_mask(masked)?;
    let mut mask = [0u8; 4];
    if masked {
        read_exact_io(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_io(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    // Server pong: FIN + opcode 0xA, unmasked, same application data.
    let mut out = Vec::with_capacity(2 + payload.len() + 8);
    out.push(0x8A);
    if payload.len() < 126 {
        out.push(payload.len() as u8);
    } else if payload.len() <= 0xFFFF {
        out.push(126);
        out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        out.push(127);
        out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    }
    out.extend_from_slice(&payload);
    stream.write_all(&out)?;
    stream.flush()?;
    Ok(())
}

fn echo_one_client_data_frame(
    stream: &mut (impl Read + Write),
    expect_opcode: u8,
    server_fin_opcode: u8,
) -> std::io::Result<()> {
    let mut hdr = [0u8; 2];
    read_exact_io(stream, &mut hdr)?;
    reject_nonzero_rsv(hdr[0])?;
    let opcode = hdr[0] & 0x0F;
    reject_reserved_opcode(opcode)?;
    if opcode != expect_opcode {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("expected client opcode {expect_opcode:#x}, got {opcode:#x}"),
        ));
    }
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_io(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_io(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let masked = (hdr[1] & 0x80) != 0;
    require_client_mask(masked)?;
    let mut mask = [0u8; 4];
    if masked {
        read_exact_io(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_io(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    // W1207 / ADR-1553: single-frame text payload must be UTF-8 before echo.
    if expect_opcode == 0x1 {
        require_text_payload_utf8(&payload)?;
    }
    let mut out = Vec::with_capacity(2 + payload.len() + 8);
    out.push(server_fin_opcode);
    if payload.len() < 126 {
        out.push(payload.len() as u8);
    } else if payload.len() <= 0xFFFF {
        out.push(126);
        out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        out.push(127);
        out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    }
    out.extend_from_slice(&payload);
    stream.write_all(&out)?;
    stream.flush()?;
    Ok(())
}

/// W1183 close-path helper kept as alias for tests that only need text+close.
#[allow(dead_code)]
pub(crate) fn echo_text_then_close_exchange(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_text_frame(stream)?;
    read_client_close_then_reply(stream)
}

/// W1184 binary helper kept as alias — prefer ping lifecycle on serve terminate.
#[allow(dead_code)]
pub(crate) fn echo_text_binary_then_close_exchange(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_text_frame(stream)?;
    echo_one_client_binary_frame(stream)?;
    read_client_close_then_reply(stream)
}

/// W1185 ping helper kept as alias — prefer fragmented-text lifecycle on serve terminate.
#[allow(dead_code)]
pub(crate) fn echo_text_binary_ping_then_close_exchange(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_text_frame(stream)?;
    echo_one_client_binary_frame(stream)?;
    reply_client_ping_with_pong(stream)?;
    read_client_close_then_reply(stream)
}

/// W1186 helper kept as alias — prefer frag-binary lifecycle on serve terminate.
#[allow(dead_code)]
pub(crate) fn echo_fragmented_text_binary_ping_then_close_exchange(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    echo_one_client_fragmented_text(stream)?;
    echo_one_client_binary_frame(stream)?;
    reply_client_ping_with_pong(stream)?;
    read_client_close_then_reply(stream)
}

/// W1202–W1204 / ADR-1548–1550: fragmented text/binary → ping/pong → close; RSV/MASK/FIN/reserved-op/control-len.
/// W1209 / ADR-1555: protocol `InvalidData` → unmasked close(1002) then abort.
/// ≠ full codec / ≠ extension / ≠ close-code GA / ≠ unbounded-N matrix GA.
pub(crate) fn echo_fragmented_text_fragmented_binary_ping_then_close_exchange(
    stream: &mut (impl Read + Write),
) -> std::io::Result<()> {
    let result = (|| {
        echo_one_client_fragmented_text(stream)?;
        echo_one_client_fragmented_binary(stream)?;
        reply_client_ping_with_pong(stream)?;
        read_client_close_then_reply(stream)
    })();
    map_invalid_data_to_protocol_close_1002(stream, result)
}

/// W1197 / ADR-1543: client close → server unmasked close.
/// Empty payload → `[0x88,0x00]`; status+reason (`len≥2`) → echo same application data.
/// W1198: nonzero RSV on close header → fail-closed.
/// W1199 / ADR-1545: close body `len==1` → fail-closed (RFC6455 §5.5.1: 0 or ≥2 only).
/// W1200: client MASK required.
/// W1201: control FIN=1 required.
/// W1202/W1203: reserved opcodes 0x3–0x7 and 0xB–0xF rejected.
/// W1204: control payload len7 must be ≤125.
/// W1205: illegal close status (0–999 / 1005 / 1006 / 1015) rejected.
/// W1206: close reason UTF-8 required when `len>2`.
fn read_client_close_then_reply(stream: &mut (impl Read + Write)) -> std::io::Result<()> {
    let mut hdr = [0u8; 2];
    read_exact_io(stream, &mut hdr)?;
    reject_nonzero_rsv(hdr[0])?;
    let fin = (hdr[0] & 0x80) != 0;
    require_control_fin(fin)?;
    let opcode = hdr[0] & 0x0F;
    reject_reserved_opcode(opcode)?;
    let masked = (hdr[1] & 0x80) != 0;
    require_client_mask(masked)?;
    let len7 = hdr[1] & 0x7F;
    require_control_payload_len7(opcode, len7)?;
    let mut len = len7 as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_io(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_io(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask = [0u8; 4];
    if masked {
        read_exact_io(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_io(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    if opcode != 0x8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "expected client close after data echo",
        ));
    }
    if payload.len() == 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "close body length must be 0 or >= 2 (got 1)",
        ));
    }
    reject_illegal_close_status(&payload)?;
    require_close_reason_utf8(&payload)?;
    if payload.is_empty() {
        // Server close: FIN + opcode 8, unmasked empty (historical empty-close nails).
        stream.write_all(&[0x88, 0x00])?;
        stream.flush()?;
    } else {
        // Echo status (2 bytes) + reason as unmasked close payload.
        write_unmasked_server_frame(stream, 0x88, &payload)?;
    }
    Ok(())
}

fn random_key_b64() -> String {
    let mut raw = [0u8; 16];
    // Non-crypto PRNG is fine for masking/key material in this narrow slice.
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0xDEAD_BEEF);
    let mut x = t ^ 0x9E37_79B9_7F4A_7C15;
    for b in &mut raw {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        *b = (x >> 33) as u8;
    }
    b64_encode(&raw)
}

fn mask_key() -> [u8; 4] {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u32)
        .unwrap_or(0xA5A5_A5A5);
    t.to_le_bytes()
}

fn encode_client_text_frame(payload: &[u8]) -> Vec<u8> {
    let mask = mask_key();
    let mut frame = Vec::with_capacity(2 + 4 + payload.len() + 8);
    frame.push(0x81); // FIN + text
    let len = payload.len();
    if len < 126 {
        frame.push(0x80 | (len as u8));
    } else if len <= 0xFFFF {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(0x80 | 127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }
    frame.extend_from_slice(&mask);
    for (i, b) in payload.iter().enumerate() {
        frame.push(b ^ mask[i % 4]);
    }
    frame
}

fn read_exact_timeout(stream: &mut TcpStream, buf: &mut [u8]) -> std::io::Result<()> {
    let mut read = 0;
    while read < buf.len() {
        match stream.read(&mut buf[read..]) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "ws eof",
                ))
            }
            Ok(n) => read += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn decode_server_text_frame(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut hdr = [0u8; 2];
    read_exact_timeout(stream, &mut hdr)?;
    let opcode = hdr[0] & 0x0F;
    let masked = (hdr[1] & 0x80) != 0;
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_timeout(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_timeout(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask = [0u8; 4];
    if masked {
        read_exact_timeout(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_timeout(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    match opcode {
        0x1 => String::from_utf8(payload)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        0x8 => Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            "ws close frame",
        )),
        0x9 => {
            // ping — reply pong and recurse once
            let mut pong = encode_client_text_frame(&[]);
            // rebuild as pong opcode
            if !pong.is_empty() {
                pong[0] = 0x8A;
            }
            let _ = stream.write_all(&pong);
            decode_server_text_frame(stream)
        }
        other => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unsupported ws opcode {other}"),
        )),
    }
}

fn handshake(stream: &mut TcpStream, host: &str, path: &str) -> std::io::Result<()> {
    let key = random_key_b64();
    let expect = accept_key(&key);
    let req = format!(
        "GET {path} HTTP/1.1\r\n\
         Host: {host}\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Key: {key}\r\n\
         Sec-WebSocket-Version: 13\r\n\
         \r\n"
    );
    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut buf = Vec::with_capacity(1024);
    let mut tmp = [0u8; 256];
    loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
        if buf.len() > 16 * 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "handshake too large",
            ));
        }
    }
    let text = String::from_utf8_lossy(&buf);
    let status_ok = text.starts_with("HTTP/1.1 101") || text.starts_with("HTTP/1.0 101");
    if !status_ok {
        return Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "ws upgrade rejected",
        ));
    }
    let accept_ok = text
        .lines()
        .any(|l| l.to_ascii_lowercase().starts_with("sec-websocket-accept:") && {
            let v = l.split_once(':').map(|(_, v)| v.trim()).unwrap_or("");
            v.eq_ignore_ascii_case(&expect)
        });
    if !accept_ok {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad Sec-WebSocket-Accept",
        ));
    }
    Ok(())
}

/// Connect `ws://` with RFC6455 upgrade. Returns connection id on success.
pub fn rfc6455_connect(url: &str) -> Option<i64> {
    let (host, port, path) = parse_ws_url(url)?;
    let addr = format!("{host}:{port}");
    let mut addrs = addr.to_socket_addrs().ok()?;
    let sock_addr = addrs.next()?;
    let mut stream = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(3)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(3)))
        .ok()?;
    let host_header = if port == 80 {
        host.clone()
    } else {
        format!("{host}:{port}")
    };
    handshake(&mut stream, &host_header, &path).ok()?;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    conns().lock().ok()?.insert(id, Conn { stream });
    Some(id)
}

pub fn rfc6455_send(conn_id: i64, message: &str) -> bool {
    let Ok(mut guard) = conns().lock() else {
        return false;
    };
    let Some(conn) = guard.get_mut(&conn_id) else {
        return false;
    };
    let frame = encode_client_text_frame(message.as_bytes());
    conn.stream.write_all(&frame).is_ok() && conn.stream.flush().is_ok()
}

pub fn rfc6455_recv(conn_id: i64) -> Option<String> {
    let Ok(mut guard) = conns().lock() else {
        return None;
    };
    let conn = guard.get_mut(&conn_id)?;
    decode_server_text_frame(&mut conn.stream).ok()
}

fn encode_client_close_frame() -> Vec<u8> {
    let mask = mask_key();
    let mut frame = vec![0x88, 0x80];
    frame.extend_from_slice(&mask);
    frame
}

fn read_server_close_frame(stream: &mut TcpStream) -> bool {
    let mut hdr = [0u8; 2];
    if read_exact_timeout(stream, &mut hdr).is_err() {
        return false;
    }
    let opcode = hdr[0] & 0x0F;
    let masked = (hdr[1] & 0x80) != 0;
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        if read_exact_timeout(stream, &mut ext).is_err() {
            return false;
        }
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        if read_exact_timeout(stream, &mut ext).is_err() {
            return false;
        }
        len = u64::from_be_bytes(ext) as usize;
    }
    if masked {
        let mut mask = [0u8; 4];
        if read_exact_timeout(stream, &mut mask).is_err() {
            return false;
        }
    }
    if len > 0 {
        let mut payload = vec![0u8; len];
        if read_exact_timeout(stream, &mut payload).is_err() {
            return false;
        }
    }
    opcode == 0x8
}

/// Send close and attempt to observe peer close (W1145 close-path deepen).
/// Returns whether the close handshake completed honestly.
pub fn rfc6455_close_exchange(conn_id: i64) -> bool {
    let Ok(mut guard) = conns().lock() else {
        return false;
    };
    let Some(mut conn) = guard.remove(&conn_id) else {
        return false;
    };
    let frame = encode_client_close_frame();
    if conn.stream.write_all(&frame).is_err() || conn.stream.flush().is_err() {
        let _ = conn.stream.shutdown(std::net::Shutdown::Both);
        return false;
    }
    let peer_closed = read_server_close_frame(&mut conn.stream);
    let _ = conn.stream.shutdown(std::net::Shutdown::Both);
    peer_closed
}

pub fn rfc6455_close(conn_id: i64) {
    // Fire-and-forget close (host path). Exchange evidence uses `rfc6455_close_exchange`.
    if let Ok(mut guard) = conns().lock() {
        if let Some(mut conn) = guard.remove(&conn_id) {
            let _ = conn.stream.write_all(&encode_client_close_frame());
            let _ = conn.stream.shutdown(std::net::Shutdown::Both);
        }
    }
}

/// Server-side accept key helper for loopback fixture.
fn server_accept_response(client_key: &str) -> String {
    let accept = accept_key(client_key);
    format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {accept}\r\n\
         \r\n"
    )
}

fn read_exact_io(stream: &mut impl Read, buf: &mut [u8]) -> std::io::Result<()> {
    let mut read = 0;
    while read < buf.len() {
        match stream.read(&mut buf[read..]) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "ws eof",
                ))
            }
            Ok(n) => read += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn serve_one_echo_io(stream: &mut (impl Read + Write)) -> std::io::Result<()> {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 256];
    loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "client gone",
            ));
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let req = String::from_utf8_lossy(&buf);
    let key = req
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("sec-websocket-key:"))
        .and_then(|l| l.split_once(':').map(|(_, v)| v.trim().to_string()))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "no key"))?;
    stream.write_all(server_accept_response(&key).as_bytes())?;
    stream.flush()?;
    echo_one_client_text_frame(stream)
}

fn serve_one_echo(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_echo_io(&mut stream)
}

/// Loopback evidence: ephemeral echo server + client text round-trip.
/// Used by W1144 native meta; does not require `SYX_WS_REAL`.
pub fn e01_rfc6455_loopback_roundtrip_ok() -> bool {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let _ = serve_one_echo(stream);
        }
    });
    // brief settle for accept thread
    thread::sleep(Duration::from_millis(20));
    let url = format!("ws://127.0.0.1:{port}/");
    let Some(id) = rfc6455_connect(&url) else {
        return false;
    };
    let ok_send = rfc6455_send(id, "syx-e01-w1144");
    let got = rfc6455_recv(id);
    // fire-and-forget close after echo (server may already be gone)
    if let Ok(mut guard) = conns().lock() {
        if let Some(mut conn) = guard.remove(&id) {
            let _ = conn.stream.write_all(&encode_client_close_frame());
            let _ = conn.stream.shutdown(std::net::Shutdown::Both);
        }
    }
    ok_send && got.as_deref() == Some("syx-e01-w1144")
}

#[derive(Clone, Copy)]
enum NegHandshakeKind {
    BadAccept,
    Non101,
}

fn serve_negative_handshake(mut stream: TcpStream, kind: NegHandshakeKind) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 256];
    loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let resp = match kind {
        NegHandshakeKind::BadAccept => {
            "HTTP/1.1 101 Switching Protocols\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Accept: AAAAAAAAAAAAAAAAAAAAAAAAAAA=\r\n\
             \r\n"
                .to_string()
        }
        NegHandshakeKind::Non101 => {
            "HTTP/1.1 400 Bad Request\r\n\
             Content-Length: 0\r\n\
             Connection: close\r\n\
             \r\n"
                .to_string()
        }
    };
    stream.write_all(resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn loopback_connect_against(kind: NegHandshakeKind) -> Option<i64> {
    let listener = TcpListener::bind("127.0.0.1:0").ok()?;
    let port = listener.local_addr().ok()?.port();
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let _ = serve_negative_handshake(stream, kind);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let url = format!("ws://127.0.0.1:{port}/");
    rfc6455_connect(&url)
}

/// W1145: handshake negatives fail-closed (bad Accept + non-101).
/// Connect must return `None` — no silent mock fallback on this path.
pub fn e01_rfc6455_negative_handshake_fail_closed_ok() -> bool {
    let bad_accept_fail = loopback_connect_against(NegHandshakeKind::BadAccept).is_none();
    let non_101_fail = loopback_connect_against(NegHandshakeKind::Non101).is_none();
    bad_accept_fail && non_101_fail
}

fn serve_close_exchange(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 256];
    loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "client gone",
            ));
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let req = String::from_utf8_lossy(&buf);
    let key = req
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("sec-websocket-key:"))
        .and_then(|l| l.split_once(':').map(|(_, v)| v.trim().to_string()))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "no key"))?;
    stream.write_all(server_accept_response(&key).as_bytes())?;
    stream.flush()?;

    // Wait for client close (opcode 8), reply with unmasked close.
    let mut hdr = [0u8; 2];
    read_exact_timeout(&mut stream, &mut hdr)?;
    let opcode = hdr[0] & 0x0F;
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_timeout(&mut stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    }
    let mut mask = [0u8; 4];
    read_exact_timeout(&mut stream, &mut mask)?;
    if len > 0 {
        let mut payload = vec![0u8; len];
        read_exact_timeout(&mut stream, &mut payload)?;
    }
    if opcode != 0x8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "expected client close",
        ));
    }
    // server close: FIN + opcode 8, unmasked empty
    stream.write_all(&[0x88, 0x00])?;
    stream.flush()?;
    Ok(())
}

/// W1145: close-path deepen — client close + peer close observed.
pub fn e01_rfc6455_close_path_ok() -> bool {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let _ = serve_close_exchange(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let url = format!("ws://127.0.0.1:{port}/");
    let Some(id) = rfc6455_connect(&url) else {
        return false;
    };
    rfc6455_close_exchange(id)
}

/// Combined W1145 negative-tests evidence helper.
pub fn e01_rfc6455_negative_tests_first_slice_ok() -> bool {
    e01_rfc6455_negative_handshake_fail_closed_ok() && e01_rfc6455_close_path_ok()
}

/// W1146 interop SSOT: named loopback RFC6455 **echo peer** (client ↔ peer).
/// Distinct from W1144 construction round-trip helper naming; same transport floor.
/// ≠ external peer suite / ≠ wss / ≠ product interop GA.
pub fn e01_rfc6455_loopback_peer_echo_wired_ok() -> bool {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let _ = serve_one_echo(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let url = format!("ws://127.0.0.1:{port}/");
    let Some(id) = rfc6455_connect(&url) else {
        return false;
    };
    // Interop-named payload — peer must echo verbatim.
    const MSG: &str = "syx-e01-w1146-interop-peer";
    let ok_send = rfc6455_send(id, MSG);
    let got = rfc6455_recv(id);
    if let Ok(mut guard) = conns().lock() {
        if let Some(mut conn) = guard.remove(&id) {
            let _ = conn.stream.write_all(&encode_client_close_frame());
            let _ = conn.stream.shutdown(std::net::Shutdown::Both);
        }
    }
    ok_send && got.as_deref() == Some(MSG)
}

/// Combined W1146 interop first-slice evidence helper.
pub fn e01_rfc6455_interop_first_slice_ok() -> bool {
    e01_rfc6455_loopback_peer_echo_wired_ok()
}

fn parse_wss_url(url: &str) -> Option<(String, u16, String)> {
    let rest = url.strip_prefix("wss://")?;
    let (authority, path) = match rest.split_once('/') {
        Some((a, p)) => (a, format!("/{p}")),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return None;
    }
    let (host, port) = if let Some((h, p)) = authority.rsplit_once(':') {
        if h.starts_with('[') {
            let host = h.trim_start_matches('[').trim_end_matches(']').to_string();
            let port: u16 = p.parse().ok()?;
            (host, port)
        } else if h.contains(':') {
            return None;
        } else {
            (h.to_string(), p.parse().ok()?)
        }
    } else {
        (authority.to_string(), 443)
    };
    Some((host, port, path))
}

fn handshake_io(stream: &mut (impl Read + Write), host: &str, path: &str) -> std::io::Result<()> {
    let key = random_key_b64();
    let expect = accept_key(&key);
    let req = format!(
        "GET {path} HTTP/1.1\r\n\
         Host: {host}\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Key: {key}\r\n\
         Sec-WebSocket-Version: 13\r\n\
         \r\n"
    );
    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut buf = Vec::with_capacity(1024);
    let mut tmp = [0u8; 256];
    loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
        if buf.len() > 16 * 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "handshake too large",
            ));
        }
    }
    let text = String::from_utf8_lossy(&buf);
    let status_ok = text.starts_with("HTTP/1.1 101") || text.starts_with("HTTP/1.0 101");
    if !status_ok {
        return Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "ws upgrade rejected",
        ));
    }
    let accept_ok = text
        .lines()
        .any(|l| l.to_ascii_lowercase().starts_with("sec-websocket-accept:") && {
            let v = l.split_once(':').map(|(_, v)| v.trim()).unwrap_or("");
            v.eq_ignore_ascii_case(&expect)
        });
    if !accept_ok {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad Sec-WebSocket-Accept",
        ));
    }
    Ok(())
}

fn decode_server_text_frame_io(stream: &mut impl Read) -> std::io::Result<String> {
    let mut hdr = [0u8; 2];
    read_exact_io(stream, &mut hdr)?;
    let opcode = hdr[0] & 0x0F;
    let masked = (hdr[1] & 0x80) != 0;
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        read_exact_io(stream, &mut ext)?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        read_exact_io(stream, &mut ext)?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask = [0u8; 4];
    if masked {
        read_exact_io(stream, &mut mask)?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        read_exact_io(stream, &mut payload)?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    match opcode {
        0x1 => String::from_utf8(payload)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        0x8 => Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            "ws close frame",
        )),
        other => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unsupported ws opcode {other}"),
        )),
    }
}

/// W1147 wss SSOT: named loopback **rustls + RFC6455** echo (client `wss://` ↔ peer).
/// Uses ephemeral self-signed test cert; CI-safe (no external network).
/// ≠ product WebSocket GA / ≠ Autobahn / ≠ skip-verify prod path.
pub fn e01_rfc6455_wss_loopback_wired_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    let cert = match rcgen::generate_simple_self_signed(vec![
        "127.0.0.1".into(),
        "localhost".into(),
    ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der()));
    let cert_der = CertificateDer::from(cert.cert);
    let server_cfg = match ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der.clone()], key_der)
    {
        Ok(c) => Arc::new(c),
        Err(_) => return false,
    };
    let mut roots = RootCertStore::empty();
    if roots.add(cert_der).is_err() {
        return false;
    }
    let client_cfg = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    );

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    let server_cfg_bg = Arc::clone(&server_cfg);
    thread::spawn(move || {
        let Ok((mut tcp, _)) = listener.accept() else {
            return;
        };
        let _ = tcp.set_read_timeout(Some(Duration::from_secs(3)));
        let _ = tcp.set_write_timeout(Some(Duration::from_secs(3)));
        let Ok(mut conn) = rustls::ServerConnection::new(server_cfg_bg) else {
            return;
        };
        while conn.is_handshaking() {
            if conn.complete_io(&mut tcp).is_err() {
                return;
            }
        }
        let mut tls = rustls::Stream::new(&mut conn, &mut tcp);
        let _ = serve_one_echo_io(&mut tls);
    });
    thread::sleep(Duration::from_millis(30));

    let Ok(mut tcp) = TcpStream::connect_timeout(
        &format!("127.0.0.1:{port}").parse().unwrap(),
        Duration::from_secs(3),
    ) else {
        return false;
    };
    let _ = tcp.set_read_timeout(Some(Duration::from_secs(3)));
    let _ = tcp.set_write_timeout(Some(Duration::from_secs(3)));
    let Ok(name) = ServerName::try_from("localhost") else {
        return false;
    };
    let Ok(mut client) = rustls::ClientConnection::new(client_cfg, name) else {
        return false;
    };
    while client.is_handshaking() {
        if client.complete_io(&mut tcp).is_err() {
            return false;
        }
    }
    let mut tls = rustls::Stream::new(&mut client, &mut tcp);
    let host_header = format!("127.0.0.1:{port}");
    if handshake_io(&mut tls, &host_header, "/").is_err() {
        return false;
    }
    const MSG: &str = "syx-e01-w1147-wss-loopback";
    let frame = encode_client_text_frame(MSG.as_bytes());
    if tls.write_all(&frame).is_err() || tls.flush().is_err() {
        return false;
    }
    let got = match decode_server_text_frame_io(&mut tls) {
        Ok(s) => s,
        Err(_) => return false,
    };
    got == MSG
}

/// Combined W1147 wss first-slice evidence helper.
pub fn e01_rfc6455_wss_first_slice_ok() -> bool {
    e01_rfc6455_wss_loopback_wired_ok()
}

/// Opt-in `wss://` RFC6455+TLS connect for loopback hosts (127.0.0.1 / localhost).
/// Requires a peer speaking RFC6455 over TLS with a cert trusted via
/// `SYX_WS_WSS_TEST_ROOT` PEM path, or returns `None` (fail-closed).
/// ≠ product GA / ≠ danger_accept_invalid_certs for arbitrary hosts.
pub fn rfc6455_connect_wss(url: &str) -> Option<i64> {
    use rustls::pki_types::{CertificateDer, ServerName, pem::PemObject};
    use rustls::{ClientConfig, RootCertStore};
    use std::sync::Arc;

    let (host, port, path) = parse_wss_url(url)?;
    let loopback = host == "127.0.0.1" || host == "localhost" || host == "::1";
    if !loopback {
        return None;
    }
    let root_path = std::env::var("SYX_WS_WSS_TEST_ROOT").ok()?;
    let certs: Vec<CertificateDer<'static>> =
        CertificateDer::pem_file_iter(std::path::Path::new(&root_path))
            .ok()?
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
    if certs.is_empty() {
        return None;
    }
    let mut roots = RootCertStore::empty();
    for c in certs {
        roots.add(c).ok()?;
    }
    let client_cfg = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    );
    let addr = format!("{host}:{port}");
    let mut addrs = addr.to_socket_addrs().ok()?;
    let sock_addr = addrs.next()?;
    let mut tcp = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(3)).ok()?;
    tcp.set_read_timeout(Some(Duration::from_secs(3))).ok()?;
    tcp.set_write_timeout(Some(Duration::from_secs(3))).ok()?;
    let name = ServerName::try_from(host.as_str()).ok()?.to_owned();
    let client = rustls::ClientConnection::new(client_cfg, name).ok()?;
    let mut tls = rustls::StreamOwned::new(client, tcp);
    while tls.conn.is_handshaking() {
        tls.conn.complete_io(&mut tls.sock).ok()?;
    }
    let host_header = if port == 443 {
        host.clone()
    } else {
        format!("{host}:{port}")
    };
    handshake_io(&mut tls, &host_header, &path).ok()?;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    wss_conns().lock().ok()?.insert(id, tls);
    Some(id)
}

type WssConn = rustls::StreamOwned<rustls::ClientConnection, TcpStream>;

static WSS_CONNS: OnceLock<Mutex<HashMap<i64, WssConn>>> = OnceLock::new();

fn wss_conns() -> &'static Mutex<HashMap<i64, WssConn>> {
    WSS_CONNS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Send on either cleartext or wss conn table.
pub fn rfc6455_send_any(conn_id: i64, message: &str) -> bool {
    if rfc6455_send(conn_id, message) {
        return true;
    }
    let Ok(mut guard) = wss_conns().lock() else {
        return false;
    };
    let Some(stream) = guard.get_mut(&conn_id) else {
        return false;
    };
    let frame = encode_client_text_frame(message.as_bytes());
    stream.write_all(&frame).is_ok() && stream.flush().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_key_rfc_example() {
        // RFC6455 §1.3 example
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        assert_eq!(accept_key(key), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }

    #[test]
    fn loopback_roundtrip() {
        assert!(e01_rfc6455_loopback_roundtrip_ok());
    }

    #[test]
    fn negative_handshake_fail_closed() {
        assert!(e01_rfc6455_negative_handshake_fail_closed_ok());
    }

    #[test]
    fn close_path_exchange() {
        assert!(e01_rfc6455_close_path_ok());
    }

    #[test]
    fn loopback_peer_echo_interop() {
        assert!(e01_rfc6455_loopback_peer_echo_wired_ok());
    }

    #[test]
    fn wss_loopback_rustls_rfc6455() {
        assert!(e01_rfc6455_wss_loopback_wired_ok());
    }
}
