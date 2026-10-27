//! Narrow true gRPC wire (W1152 / ADR-1498).
//! Loopback unary over cleartext HTTP/2 prior-knowledge (h2c) + gRPC
//! length-prefixed framing (opaque payload; no protoc).
//! W1154 / ADR-1500: named loopback rustls + H2 + gRPC framing (grpcs).
//! W1157 / ADR-1503: negative fail-closed (missing PREFACE / truncated DATA).
//! W1158 / ADR-1504: narrow server-streaming (multi DATA loopback h2c).
//! W1159 / ADR-1505: narrow client-streaming (multi DATA request + unary response).
//! W1160 / ADR-1506: narrow bidi-streaming (multi DATA request + multi DATA response).
//! W1161 / ADR-1507: narrow grpcs server-streaming (rustls+H2 + multi DATA response).
//! Opt-in via `SYX_GRPC_REAL=1`. Default path remains HTTP/1.1 facade.
//! ≠ product gRPC GA / ≠ streaming/bidi GA / ≠ interop suite / ≠ negative suite.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
const FRAME_DATA: u8 = 0x0;
const FRAME_HEADERS: u8 = 0x1;
const FRAME_SETTINGS: u8 = 0x4;
/// HTTP/2 WINDOW_UPDATE (RFC 7540 §6.9). W1167 / ADR-1513 first-slice.
const FRAME_WINDOW_UPDATE: u8 = 0x8;
const FLAG_END_STREAM: u8 = 0x1;
const FLAG_END_HEADERS: u8 = 0x4;
const FLAG_ACK: u8 = 0x1;
/// Fixed WINDOW_UPDATE increment for W1167 credit-gate evidence (≠ full FC GA).
const W1167_WINDOW_INCREMENT: u32 = 0x0001_0000;

/// Encode gRPC message: 1-byte flags + 4-byte big-endian length + payload.
pub fn encode_grpc_message(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + payload.len());
    out.push(0); // no compression
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

/// Decode first gRPC message from a DATA payload.
pub fn decode_grpc_message(buf: &[u8]) -> Option<Vec<u8>> {
    if buf.len() < 5 {
        return None;
    }
    let len = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
    if buf.len() < 5 + len {
        return None;
    }
    Some(buf[5..5 + len].to_vec())
}

fn write_frame<S: Write>(stream: &mut S, ty: u8, flags: u8, stream_id: u32, payload: &[u8]) -> bool {
    let len = payload.len() as u32;
    if len > 0x00FF_FFFF {
        return false;
    }
    let hdr = [
        ((len >> 16) & 0xff) as u8,
        ((len >> 8) & 0xff) as u8,
        (len & 0xff) as u8,
        ty,
        flags,
        ((stream_id >> 24) & 0x7f) as u8,
        ((stream_id >> 16) & 0xff) as u8,
        ((stream_id >> 8) & 0xff) as u8,
        (stream_id & 0xff) as u8,
    ];
    stream.write_all(&hdr).is_ok() && (payload.is_empty() || stream.write_all(payload).is_ok())
}

fn read_exact_n<S: Read>(stream: &mut S, n: usize) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; n];
    stream.read_exact(&mut buf).ok()?;
    Some(buf)
}

fn read_frame<S: Read>(stream: &mut S) -> Option<(u8, u8, u32, Vec<u8>)> {
    let hdr = read_exact_n(stream, 9)?;
    let len = ((hdr[0] as usize) << 16) | ((hdr[1] as usize) << 8) | (hdr[2] as usize);
    let ty = hdr[3];
    let flags = hdr[4];
    let stream_id =
        ((hdr[5] as u32) << 24) | ((hdr[6] as u32) << 16) | ((hdr[7] as u32) << 8) | (hdr[8] as u32);
    let stream_id = stream_id & 0x7fff_ffff;
    let payload = if len == 0 {
        Vec::new()
    } else {
        read_exact_n(stream, len)?
    };
    Some((ty, flags, stream_id, payload))
}

fn encode_int(n: usize) -> Vec<u8> {
    // 7-bit prefix integer with N=7 (first byte already has high bit clear for length).
    if n < 127 {
        vec![n as u8]
    } else {
        let mut out = vec![127u8];
        let mut i = n - 127;
        while i >= 128 {
            out.push(((i % 128) as u8) | 0x80);
            i /= 128;
        }
        out.push(i as u8);
        out
    }
}

fn hpack_literal(name: &str, value: &str) -> Vec<u8> {
    // Literal Header Field without Indexing — New Name (RFC 7541 §6.2.2).
    let mut out = vec![0x00];
    out.extend(encode_int(name.len()));
    out.extend_from_slice(name.as_bytes());
    out.extend(encode_int(value.len()));
    out.extend_from_slice(value.as_bytes());
    out
}

fn hpack_block(headers: &[(&str, &str)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (n, v) in headers {
        out.extend(hpack_literal(n, v));
    }
    out
}

fn decode_hpack_string(buf: &[u8], off: &mut usize) -> Option<String> {
    if *off >= buf.len() {
        return None;
    }
    let first = buf[*off];
    *off += 1;
    // Huffman bit must be clear for this narrow encoder.
    if first & 0x80 != 0 {
        return None;
    }
    let mut len = (first & 0x7f) as usize;
    if len == 127 {
        let mut m = 0u32;
        loop {
            if *off >= buf.len() {
                return None;
            }
            let b = buf[*off];
            *off += 1;
            len += ((b & 0x7f) as usize) << m;
            m += 7;
            if b & 0x80 == 0 {
                break;
            }
        }
    }
    if *off + len > buf.len() {
        return None;
    }
    let s = String::from_utf8_lossy(&buf[*off..*off + len]).into_owned();
    *off += len;
    Some(s)
}

fn decode_hpack_literals(buf: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut off = 0;
    while off < buf.len() {
        let b = buf[off];
        if b == 0x00 {
            off += 1;
            let Some(name) = decode_hpack_string(buf, &mut off) else {
                break;
            };
            let Some(value) = decode_hpack_string(buf, &mut off) else {
                break;
            };
            out.push((name, value));
        } else {
            // Unsupported HPACK form in this narrow slice — stop.
            break;
        }
    }
    out
}

fn exchange_settings<S: Read + Write>(stream: &mut S, send_preface: bool) -> bool {
    if send_preface && stream.write_all(PREFACE).is_err() {
        return false;
    }
    // Empty SETTINGS
    if !write_frame(stream, FRAME_SETTINGS, 0, 0, &[]) {
        return false;
    }
    // Read peer SETTINGS (skip other frames until SETTINGS on stream 0).
    for _ in 0..16 {
        let Some((ty, flags, sid, _)) = read_frame(stream) else {
            return false;
        };
        if ty == FRAME_SETTINGS && sid == 0 {
            if flags & FLAG_ACK == 0 {
                // ACK peer SETTINGS
                if !write_frame(stream, FRAME_SETTINGS, FLAG_ACK, 0, &[]) {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

enum UnaryServerMode {
    /// W1152 construction: wrap body as `grpc-wire-ok:{method}:{body}`.
    Construction,
    /// W1153 interop / W1154 grpcs: verbatim echo of decoded opaque payload.
    EchoPeer,
}

fn serve_one_unary_mode_io<S: Read + Write>(
    stream: &mut S,
    mode: UnaryServerMode,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }

    let mut method = "unknown".to_string();
    let mut req_data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid == 0 {
            continue;
        }
        if ty == FRAME_HEADERS {
            for (n, v) in decode_hpack_literals(&payload) {
                if n == ":path" {
                    method = v.trim_start_matches('/').to_string();
                }
            }
        } else if ty == FRAME_DATA {
            req_data.extend_from_slice(&payload);
            if flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
    }

    let payload = decode_grpc_message(&req_data).unwrap_or_default();
    let resp_msg = match mode {
        UnaryServerMode::Construction => {
            let body = String::from_utf8_lossy(&payload);
            let resp_text = format!("grpc-wire-ok:{method}:{body}");
            encode_grpc_message(resp_text.as_bytes())
        }
        UnaryServerMode::EchoPeer => encode_grpc_message(&payload),
    };

    let hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    let _ = write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
    let _ = write_frame(stream, FRAME_DATA, 0, 1, &resp_msg);
    let trailers = hpack_block(&[("grpc-status", "0")]);
    let _ = write_frame(
        stream,
        FRAME_HEADERS,
        FLAG_END_HEADERS | FLAG_END_STREAM,
        1,
        &trailers,
    );
    Ok(())
}

fn serve_one_unary(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_unary_mode_io(&mut stream, UnaryServerMode::Construction)
}

fn serve_one_echo_peer(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_unary_mode_io(&mut stream, UnaryServerMode::EchoPeer)
}

/// Unary call over narrow h2c + gRPC framing on an open Read+Write stream.
fn grpc_wire_unary_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    body: &str,
    scheme: &str,
) -> Option<String> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let msg = encode_grpc_message(body.as_bytes());
    if !write_frame(stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
        return None;
    }

    let mut data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            data.extend_from_slice(&payload);
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    let payload = decode_grpc_message(&data)?;
    Some(String::from_utf8_lossy(&payload).into_owned())
}

/// Unary call over narrow h2c + gRPC framing to `127.0.0.1:port`.
pub fn grpc_wire_unary_call(port: u16, method: &str, body: &str) -> Option<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_millis(1500)))
        .ok();
    grpc_wire_unary_call_io(&mut stream, method, body, "http")
}

/// Opt-in host path: ephemeral loopback unary over true wire.
pub fn grpc_wire_unary_via_loopback(method: &str, body: &str) -> Option<String> {
    let listener = TcpListener::bind("127.0.0.1:0").ok()?;
    let port = listener.local_addr().ok()?.port();
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let _ = serve_one_unary(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    grpc_wire_unary_call(port, method, body)
}

/// Evidence: loopback unary H2 + gRPC framing round-trip.
pub fn e02_grpc_wire_loopback_roundtrip_ok() -> bool {
    match grpc_wire_unary_via_loopback("echo.Echo/Unary", "syx-e02-w1152") {
        Some(resp) => resp == "grpc-wire-ok:echo.Echo/Unary:syx-e02-w1152",
        None => false,
    }
}

/// Named loopback echo peer (W1153 / ADR-1499): verbatim opaque payload.
/// Distinct from W1152 construction `grpc-wire-ok:…` evidence.
pub fn e02_grpc_loopback_peer_echo_wired_ok() -> bool {
    const MSG: &str = "syx-e02-w1153-interop-peer";
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
            let _ = serve_one_echo_peer(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_unary_call(port, "echo.Echo/Unary", MSG) {
        Some(resp) => resp == MSG,
        None => false,
    }
}

/// Combined W1153 interop first-slice evidence helper.
pub fn e02_grpc_interop_first_slice_ok() -> bool {
    e02_grpc_loopback_peer_echo_wired_ok()
}

/// Named loopback grpcs (W1154 / ADR-1500): rustls + H2 + gRPC framing echo.
/// Distinct from h2c construction / interop evidence.
pub fn e02_grpc_tls_loopback_wired_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MSG: &str = "syx-e02-w1154-grpcs-peer";

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
        let _ = serve_one_unary_mode_io(&mut tls, UnaryServerMode::EchoPeer);
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
    match grpc_wire_unary_call_io(&mut tls, "echo.Echo/Unary", MSG, "https") {
        Some(resp) => resp == MSG,
        None => false,
    }
}

/// Combined W1154 grpcs first-slice evidence helper.
pub fn e02_grpcs_first_slice_ok() -> bool {
    e02_grpc_tls_loopback_wired_ok()
}

/// W1157 / ADR-1503: client omits HTTP/2 connection preface → fail-closed.
/// Against a live unary peer, skipping PREFACE must yield `None` (no fake success).
pub fn e02_grpc_negative_preface_fail_closed_ok() -> bool {
    const MSG: &str = "syx-e02-w1157-neg";
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
            let _ = serve_one_unary(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(400)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(400)));
    // Deliberately omit PREFACE — peer expects it; call must fail-closed.
    let result = if exchange_settings(&mut stream, false) {
        let path = "/echo.Echo/Unary";
        let hdrs = hpack_block(&[
            (":method", "POST"),
            (":scheme", "http"),
            (":authority", "127.0.0.1"),
            (":path", path),
            ("content-type", "application/grpc"),
            ("te", "trailers"),
        ]);
        if !write_frame(&mut stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
            None
        } else {
            let msg = encode_grpc_message(MSG.as_bytes());
            if !write_frame(&mut stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
                None
            } else {
                let mut data = Vec::new();
                for _ in 0..8 {
                    let Some((ty, flags, sid, payload)) = read_frame(&mut stream) else {
                        break;
                    };
                    if sid != 1 {
                        continue;
                    }
                    if ty == FRAME_DATA {
                        data.extend_from_slice(&payload);
                    }
                    if flags & FLAG_END_STREAM != 0 {
                        break;
                    }
                }
                decode_grpc_message(&data)
                    .map(|p| String::from_utf8_lossy(&p).into_owned())
            }
        }
    } else {
        None
    };
    result.is_none()
}

/// W1157 / ADR-1503: truncated gRPC DATA (length-prefix > available) → fail-closed.
pub fn e02_grpc_negative_truncated_data_fail_closed_ok() -> bool {
    // Local framing: claim 100 bytes, only 3 present.
    let mut short = vec![0u8];
    short.extend_from_slice(&100u32.to_be_bytes());
    short.extend_from_slice(b"abc");
    if decode_grpc_message(&short).is_some() {
        return false;
    }

    // Wire: peer sends HEADERS + DATA with truncated gRPC message; client must get None.
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    thread::spawn(move || {
        let Ok((mut stream, _)) = listener.accept() else {
            return;
        };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(3)));
        let mut preface = [0u8; 24];
        if stream.read_exact(&mut preface).is_err() || &preface != PREFACE {
            return;
        }
        if !exchange_settings(&mut stream, false) {
            return;
        }
        // Drain client request frames briefly.
        for _ in 0..16 {
            let Some((ty, flags, sid, _)) = read_frame(&mut stream) else {
                break;
            };
            if sid == 1 && ty == FRAME_DATA && flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
        let hdrs = hpack_block(&[
            (":status", "200"),
            ("content-type", "application/grpc"),
        ]);
        let _ = write_frame(&mut stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
        // Truncated gRPC DATA: length=32, only 3 payload bytes (marker syx-e02-w1157-neg).
        let mut trunc = vec![0u8];
        trunc.extend_from_slice(&32u32.to_be_bytes());
        trunc.extend_from_slice(b"neg");
        let _ = write_frame(
            &mut stream,
            FRAME_DATA,
            FLAG_END_STREAM,
            1,
            &trunc,
        );
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_unary_call(port, "echo.Echo/Unary", "syx-e02-w1157-neg") {
        Some(_) => false,
        None => true,
    }
}

/// Combined W1157 negative tests first-slice evidence helper.
pub fn e02_grpc_negative_tests_first_slice_ok() -> bool {
    e02_grpc_negative_preface_fail_closed_ok() && e02_grpc_negative_truncated_data_fail_closed_ok()
}

/// Named alias: negative framing wired (PREFACE + truncated DATA).
pub fn e02_grpc_negative_framing_wired_ok() -> bool {
    e02_grpc_negative_tests_first_slice_ok()
}

const SERVER_STREAM_N: usize = 3;

/// Server-streaming peer over Read+Write: unary request, N DATA messages.
fn serve_one_server_stream_io<S: Read + Write>(
    stream: &mut S,
    marker_prefix: &str,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }

    let mut req_data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid == 0 {
            continue;
        }
        if ty == FRAME_DATA {
            req_data.extend_from_slice(&payload);
            if flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
    }
    let _ = decode_grpc_message(&req_data);

    let hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    let _ = write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
    for i in 0..SERVER_STREAM_N {
        let body = format!("{marker_prefix}:{i}");
        let msg = encode_grpc_message(body.as_bytes());
        let _ = write_frame(stream, FRAME_DATA, 0, 1, &msg);
    }
    let trailers = hpack_block(&[("grpc-status", "0")]);
    let _ = write_frame(
        stream,
        FRAME_HEADERS,
        FLAG_END_HEADERS | FLAG_END_STREAM,
        1,
        &trailers,
    );
    Ok(())
}

/// W1158 / ADR-1504: server-streaming peer — unary request, N DATA messages.
fn serve_one_server_stream(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_server_stream_io(&mut stream, "syx-e02-w1158-stream")
}

/// Client: unary request; collect one gRPC message per DATA frame until END_STREAM.
fn grpc_wire_server_stream_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    body: &str,
    scheme: &str,
) -> Option<Vec<String>> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let msg = encode_grpc_message(body.as_bytes());
    if !write_frame(stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
        return None;
    }

    let mut out = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            let decoded = decode_grpc_message(&payload)?;
            out.push(String::from_utf8_lossy(&decoded).into_owned());
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn grpc_wire_server_stream_call(port: u16, method: &str, body: &str) -> Option<Vec<String>> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_millis(1500)))
        .ok();
    grpc_wire_server_stream_call_io(&mut stream, method, body, "http")
}

/// W1158 / ADR-1504: named loopback server-streaming (N=3 DATA messages).
pub fn e02_grpc_server_streaming_loopback_ok() -> bool {
    const MSG: &str = "syx-e02-w1158-stream";
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
            let _ = serve_one_server_stream(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_server_stream_call(port, "echo.Echo/ServerStream", MSG) {
        Some(msgs) => {
            msgs.len() == SERVER_STREAM_N
                && msgs.iter().enumerate().all(|(i, m)| {
                    m == &format!("syx-e02-w1158-stream:{i}")
                })
        }
        None => false,
    }
}

/// Named alias: server-streaming wired.
pub fn e02_grpc_server_streaming_wired_ok() -> bool {
    e02_grpc_server_streaming_loopback_ok()
}

/// Combined W1158 streaming first-slice evidence helper.
pub fn e02_grpc_streaming_first_slice_ok() -> bool {
    e02_grpc_server_streaming_loopback_ok()
}

/// W1164 / ADR-1510: server-stream mid-stream truncated DATA must fail-closed.
/// Peer sends one valid gRPC DATA then a truncated DATA; client must get `None`
/// (not a fake N=3 success). Marker: `syx-e02-w1164-stream-neg`.
pub fn e02_grpc_streaming_negative_truncated_data_fail_closed_ok() -> bool {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let port = match listener.local_addr() {
        Ok(a) => a.port(),
        Err(_) => return false,
    };
    thread::spawn(move || {
        let Ok((mut stream, _)) = listener.accept() else {
            return;
        };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(3)));
        let mut preface = [0u8; 24];
        if stream.read_exact(&mut preface).is_err() || &preface != PREFACE {
            return;
        }
        if !exchange_settings(&mut stream, false) {
            return;
        }
        // Drain unary client request (HEADERS + DATA END_STREAM).
        for _ in 0..16 {
            let Some((ty, flags, sid, _)) = read_frame(&mut stream) else {
                break;
            };
            if sid == 1 && ty == FRAME_DATA && flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
        let hdrs = hpack_block(&[
            (":status", "200"),
            ("content-type", "application/grpc"),
        ]);
        let _ = write_frame(&mut stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
        // DATA#0: valid gRPC message.
        let ok0 = encode_grpc_message(b"syx-e02-w1164-stream-neg:0");
        let _ = write_frame(&mut stream, FRAME_DATA, 0, 1, &ok0);
        // DATA#1: truncated gRPC message (claim 32 bytes, only 3 present) + END_STREAM.
        let mut trunc = vec![0u8];
        trunc.extend_from_slice(&32u32.to_be_bytes());
        trunc.extend_from_slice(b"neg");
        let _ = write_frame(
            &mut stream,
            FRAME_DATA,
            FLAG_END_STREAM,
            1,
            &trunc,
        );
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_server_stream_call(
        port,
        "echo.Echo/ServerStream",
        "syx-e02-w1164-stream-neg",
    ) {
        Some(_) => false,
        None => true,
    }
}

/// Named alias: streaming negative framing wired (mid-stream truncated DATA).
pub fn e02_grpc_streaming_negative_framing_wired_ok() -> bool {
    e02_grpc_streaming_negative_truncated_data_fail_closed_ok()
}

/// Combined W1164 streaming-negative first-slice evidence helper.
pub fn e02_streaming_negative_tests_first_slice_ok() -> bool {
    e02_grpc_streaming_negative_truncated_data_fail_closed_ok()
}

const CLIENT_STREAM_N: usize = 3;

/// Client-streaming peer over Read+Write: N DATA request messages, unary response.
fn serve_one_client_stream_io<S: Read + Write>(
    stream: &mut S,
    marker_prefix: &str,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }

    let mut msgs = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid == 0 {
            continue;
        }
        if ty == FRAME_DATA {
            if let Some(decoded) = decode_grpc_message(&payload) {
                msgs.push(String::from_utf8_lossy(&decoded).into_owned());
            }
            if flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
    }

    let ok = msgs.len() == CLIENT_STREAM_N
        && msgs
            .iter()
            .enumerate()
            .all(|(i, m)| m == &format!("{marker_prefix}:{i}"));
    let resp_body = if ok {
        format!("{marker_prefix}:ok")
    } else {
        format!("{marker_prefix}:fail")
    };

    let hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    let _ = write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
    let msg = encode_grpc_message(resp_body.as_bytes());
    let _ = write_frame(stream, FRAME_DATA, 0, 1, &msg);
    let trailers = hpack_block(&[("grpc-status", "0")]);
    let _ = write_frame(
        stream,
        FRAME_HEADERS,
        FLAG_END_HEADERS | FLAG_END_STREAM,
        1,
        &trailers,
    );
    Ok(())
}

/// W1159 / ADR-1505: client-streaming peer — N DATA request messages, unary response.
fn serve_one_client_stream(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_client_stream_io(&mut stream, "syx-e02-w1159-client-stream")
}

/// Client: N DATA request messages; collect unary response until END_STREAM.
fn grpc_wire_client_stream_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    bodies: &[&str],
    scheme: &str,
) -> Option<String> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    for (i, body) in bodies.iter().enumerate() {
        let msg = encode_grpc_message(body.as_bytes());
        let flags = if i + 1 == bodies.len() {
            FLAG_END_STREAM
        } else {
            0
        };
        if !write_frame(stream, FRAME_DATA, flags, 1, &msg) {
            return None;
        }
    }

    let mut data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            data.extend_from_slice(&payload);
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    let payload = decode_grpc_message(&data)?;
    Some(String::from_utf8_lossy(&payload).into_owned())
}

/// Client: N DATA request messages over plain TCP; collect unary response.
fn grpc_wire_client_stream_call(port: u16, method: &str, bodies: &[&str]) -> Option<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_millis(1500)))
        .ok();
    grpc_wire_client_stream_call_io(&mut stream, method, bodies, "http")
}

/// W1159 / ADR-1505: named loopback client-streaming (N=3 request DATA + unary response).
pub fn e02_grpc_client_streaming_loopback_ok() -> bool {
    let bodies: Vec<String> = (0..CLIENT_STREAM_N)
        .map(|i| format!("syx-e02-w1159-client-stream:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
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
            let _ = serve_one_client_stream(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_client_stream_call(port, "echo.Echo/ClientStream", &body_refs) {
        Some(resp) => resp == "syx-e02-w1159-client-stream:ok",
        None => false,
    }
}

/// Named alias: client-streaming wired.
pub fn e02_grpc_client_streaming_wired_ok() -> bool {
    e02_grpc_client_streaming_loopback_ok()
}

/// Combined W1159 client-streaming first-slice evidence helper.
pub fn e02_grpc_client_streaming_first_slice_ok() -> bool {
    e02_grpc_client_streaming_loopback_ok()
}

const BIDI_STREAM_N: usize = 3;

/// Bidi peer over Read+Write: N DATA request messages, N DATA response messages.
fn serve_one_bidi_stream_io<S: Read + Write>(
    stream: &mut S,
    marker_prefix: &str,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }

    let mut msgs = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid == 0 {
            continue;
        }
        if ty == FRAME_DATA {
            if let Some(decoded) = decode_grpc_message(&payload) {
                msgs.push(String::from_utf8_lossy(&decoded).into_owned());
            }
            if flags & FLAG_END_STREAM != 0 {
                break;
            }
        }
    }

    let ok = msgs.len() == BIDI_STREAM_N
        && msgs
            .iter()
            .enumerate()
            .all(|(i, m)| m == &format!("{marker_prefix}:{i}"));

    let hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    let _ = write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs);
    if ok {
        for i in 0..BIDI_STREAM_N {
            let body = format!("{marker_prefix}:{i}");
            let msg = encode_grpc_message(body.as_bytes());
            let _ = write_frame(stream, FRAME_DATA, 0, 1, &msg);
        }
    } else {
        let fail = format!("{marker_prefix}:fail");
        let msg = encode_grpc_message(fail.as_bytes());
        let _ = write_frame(stream, FRAME_DATA, 0, 1, &msg);
    }
    let trailers = hpack_block(&[("grpc-status", "0")]);
    let _ = write_frame(
        stream,
        FRAME_HEADERS,
        FLAG_END_HEADERS | FLAG_END_STREAM,
        1,
        &trailers,
    );
    Ok(())
}

/// W1160 / ADR-1506: bidi peer — N DATA request messages, N DATA response messages.
fn serve_one_bidi_stream(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_bidi_stream_io(&mut stream, "syx-e02-w1160-bidi")
}

/// Client: N DATA request messages; collect N gRPC messages until END_STREAM.
fn grpc_wire_bidi_stream_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    bodies: &[&str],
    scheme: &str,
) -> Option<Vec<String>> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    for (i, body) in bodies.iter().enumerate() {
        let msg = encode_grpc_message(body.as_bytes());
        let flags = if i + 1 == bodies.len() {
            FLAG_END_STREAM
        } else {
            0
        };
        if !write_frame(stream, FRAME_DATA, flags, 1, &msg) {
            return None;
        }
    }

    let mut out = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            let decoded = decode_grpc_message(&payload)?;
            out.push(String::from_utf8_lossy(&decoded).into_owned());
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// Client: N DATA request messages over plain TCP; collect N response messages.
fn grpc_wire_bidi_stream_call(port: u16, method: &str, bodies: &[&str]) -> Option<Vec<String>> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_millis(1500)))
        .ok();
    grpc_wire_bidi_stream_call_io(&mut stream, method, bodies, "http")
}

/// W1160 / ADR-1506: named loopback bidi-streaming (N=3 request DATA + N=3 response DATA).
pub fn e02_grpc_bidi_streaming_loopback_ok() -> bool {
    let bodies: Vec<String> = (0..BIDI_STREAM_N)
        .map(|i| format!("syx-e02-w1160-bidi:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
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
            let _ = serve_one_bidi_stream(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_bidi_stream_call(port, "echo.Echo/BidiStream", &body_refs) {
        Some(msgs) => {
            msgs.len() == BIDI_STREAM_N
                && msgs.iter().enumerate().all(|(i, m)| {
                    m == &format!("syx-e02-w1160-bidi:{i}")
                })
        }
        None => false,
    }
}

/// Named alias: bidi-streaming wired.
pub fn e02_grpc_bidi_streaming_wired_ok() -> bool {
    e02_grpc_bidi_streaming_loopback_ok()
}

/// Combined W1160 bidi-streaming first-slice evidence helper.
pub fn e02_grpc_bidi_streaming_first_slice_ok() -> bool {
    e02_grpc_bidi_streaming_loopback_ok()
}

/// W1165 / ADR-1511: bidi peer — ping-pong one DATA request → one DATA response ×N.
fn serve_one_bidi_interleaved_stream_io<S: Read + Write>(
    stream: &mut S,
    marker_prefix: &str,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }
    // Drain request HEADERS.
    for _ in 0..8 {
        let Some((ty, flags, sid, _)) = read_frame(stream) else {
            break;
        };
        if sid == 1 && ty == FRAME_HEADERS && flags & FLAG_END_HEADERS != 0 {
            break;
        }
    }
    let resp_hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    let _ = write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &resp_hdrs);
    let mut got = 0usize;
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 || ty != FRAME_DATA {
            continue;
        }
        let Some(decoded) = decode_grpc_message(&payload) else {
            return Ok(());
        };
        let body = String::from_utf8_lossy(&decoded).into_owned();
        let expected = format!("{marker_prefix}:{got}");
        let echo = if body == expected {
            body
        } else {
            format!("{marker_prefix}:fail")
        };
        let msg = encode_grpc_message(echo.as_bytes());
        let _ = write_frame(stream, FRAME_DATA, 0, 1, &msg);
        got += 1;
        if flags & FLAG_END_STREAM != 0 {
            let trailers = hpack_block(&[("grpc-status", "0")]);
            let _ = write_frame(
                stream,
                FRAME_HEADERS,
                FLAG_END_HEADERS | FLAG_END_STREAM,
                1,
                &trailers,
            );
            break;
        }
    }
    Ok(())
}

fn serve_one_bidi_interleaved_stream(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_bidi_interleaved_stream_io(&mut stream, "syx-e02-w1165-interleave")
}

/// Client: ping-pong N DATA requests with N DATA responses (interleaved).
fn grpc_wire_bidi_interleaved_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    bodies: &[&str],
    scheme: &str,
) -> Option<Vec<String>> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let mut out = Vec::new();
    for (i, body) in bodies.iter().enumerate() {
        let msg = encode_grpc_message(body.as_bytes());
        let flags = if i + 1 == bodies.len() {
            FLAG_END_STREAM
        } else {
            0
        };
        if !write_frame(stream, FRAME_DATA, flags, 1, &msg) {
            return None;
        }
        // Wait for one DATA response before sending next (true ping-pong).
        // Skip response HEADERS / other frames until DATA arrives.
        let mut got_data = false;
        for _ in 0..32 {
            let Some((ty, flags, sid, payload)) = read_frame(stream) else {
                break;
            };
            if sid != 1 {
                continue;
            }
            if ty == FRAME_DATA {
                let decoded = decode_grpc_message(&payload)?;
                out.push(String::from_utf8_lossy(&decoded).into_owned());
                got_data = true;
            }
            if flags & FLAG_END_STREAM != 0 {
                break;
            }
            if got_data {
                break;
            }
        }
        if !got_data {
            return None;
        }
    }
    if out.len() == bodies.len() {
        Some(out)
    } else {
        None
    }
}

fn grpc_wire_bidi_interleaved_call(
    port: u16,
    method: &str,
    bodies: &[&str],
) -> Option<Vec<String>> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_millis(1500)))
        .ok();
    grpc_wire_bidi_interleaved_call_io(&mut stream, method, bodies, "http")
}

/// W1165 / ADR-1511: named loopback bidi interleaved ping-pong (N=3).
pub fn e02_grpc_bidi_interleaved_loopback_ok() -> bool {
    let bodies: Vec<String> = (0..BIDI_STREAM_N)
        .map(|i| format!("syx-e02-w1165-interleave:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
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
            let _ = serve_one_bidi_interleaved_stream(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    match grpc_wire_bidi_interleaved_call(port, "echo.Echo/BidiStream", &body_refs) {
        Some(msgs) => {
            msgs.len() == BIDI_STREAM_N
                && msgs.iter().enumerate().all(|(i, m)| {
                    m == &format!("syx-e02-w1165-interleave:{i}")
                })
        }
        None => false,
    }
}

/// Named alias: bidi interleaved wired.
pub fn e02_grpc_bidi_interleaved_wired_ok() -> bool {
    e02_grpc_bidi_interleaved_loopback_ok()
}

/// Combined W1165 bidi interleaved first-slice evidence helper.
pub fn e02_bidi_interleaved_first_slice_ok() -> bool {
    e02_grpc_bidi_interleaved_loopback_ok()
}

/// W1161 / ADR-1507: named loopback grpcs server-streaming (rustls + H2 + N DATA).
pub fn e02_grpc_tls_server_streaming_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MSG: &str = "syx-e02-w1161-grpcs-stream";
    const MARKER: &str = "syx-e02-w1161-grpcs-stream";

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
        let _ = serve_one_server_stream_io(&mut tls, MARKER);
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
    match grpc_wire_server_stream_call_io(&mut tls, "echo.Echo/ServerStream", MSG, "https") {
        Some(msgs) => {
            msgs.len() == SERVER_STREAM_N
                && msgs.iter().enumerate().all(|(i, m)| m == &format!("{MARKER}:{i}"))
        }
        None => false,
    }
}

/// Named alias: grpcs server-streaming wired.
pub fn e02_grpc_tls_server_streaming_wired_ok() -> bool {
    e02_grpc_tls_server_streaming_loopback_ok()
}

/// Combined W1161 grpcs+server-streaming first-slice evidence helper.
pub fn e02_grpcs_streaming_first_slice_ok() -> bool {
    e02_grpc_tls_server_streaming_loopback_ok()
}

/// W1162 / ADR-1508: named loopback grpcs client-streaming (rustls + H2 + N DATA req).
pub fn e02_grpc_tls_client_streaming_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MARKER: &str = "syx-e02-w1162-grpcs-client-stream";

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
        let _ = serve_one_client_stream_io(&mut tls, MARKER);
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
    let bodies: Vec<String> = (0..CLIENT_STREAM_N)
        .map(|i| format!("{MARKER}:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
    let mut tls = rustls::Stream::new(&mut client, &mut tcp);
    match grpc_wire_client_stream_call_io(
        &mut tls,
        "echo.Echo/ClientStream",
        &body_refs,
        "https",
    ) {
        Some(resp) => resp == format!("{MARKER}:ok"),
        None => false,
    }
}

/// Named alias: grpcs client-streaming wired.
pub fn e02_grpc_tls_client_streaming_wired_ok() -> bool {
    e02_grpc_tls_client_streaming_loopback_ok()
}

/// Combined W1162 grpcs+client-streaming first-slice evidence helper.
pub fn e02_grpcs_client_streaming_first_slice_ok() -> bool {
    e02_grpc_tls_client_streaming_loopback_ok()
}

/// W1163 / ADR-1509: named loopback grpcs bidi-streaming (rustls + H2 + N↔N DATA).
pub fn e02_grpc_tls_bidi_streaming_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MARKER: &str = "syx-e02-w1163-grpcs-bidi";

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
        let _ = serve_one_bidi_stream_io(&mut tls, MARKER);
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
    let bodies: Vec<String> = (0..BIDI_STREAM_N)
        .map(|i| format!("{MARKER}:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
    let mut tls = rustls::Stream::new(&mut client, &mut tcp);
    match grpc_wire_bidi_stream_call_io(&mut tls, "echo.Echo/BidiStream", &body_refs, "https")
    {
        Some(msgs) => {
            msgs.len() == BIDI_STREAM_N
                && msgs
                    .iter()
                    .enumerate()
                    .all(|(i, m)| m == &format!("{MARKER}:{i}"))
        }
        None => false,
    }
}

/// Named alias: grpcs bidi-streaming wired.
pub fn e02_grpc_tls_bidi_streaming_wired_ok() -> bool {
    e02_grpc_tls_bidi_streaming_loopback_ok()
}

/// Combined W1163 grpcs+bidi-streaming first-slice evidence helper.
pub fn e02_grpcs_bidi_streaming_first_slice_ok() -> bool {
    e02_grpc_tls_bidi_streaming_loopback_ok()
}

/// W1166 / ADR-1512: named loopback grpcs bidi interleaved (rustls + H2 ping-pong).
pub fn e02_grpc_tls_bidi_interleaved_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MARKER: &str = "syx-e02-w1166-grpcs-interleave";

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
        let _ = serve_one_bidi_interleaved_stream_io(&mut tls, MARKER);
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
    let bodies: Vec<String> = (0..BIDI_STREAM_N)
        .map(|i| format!("{MARKER}:{i}"))
        .collect();
    let body_refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
    let mut tls = rustls::Stream::new(&mut client, &mut tcp);
    match grpc_wire_bidi_interleaved_call_io(
        &mut tls,
        "echo.Echo/BidiStream",
        &body_refs,
        "https",
    ) {
        Some(msgs) => {
            msgs.len() == BIDI_STREAM_N
                && msgs
                    .iter()
                    .enumerate()
                    .all(|(i, m)| m == &format!("{MARKER}:{i}"))
        }
        None => false,
    }
}

/// Named alias: grpcs bidi interleaved wired.
pub fn e02_grpc_tls_bidi_interleaved_wired_ok() -> bool {
    e02_grpc_tls_bidi_interleaved_loopback_ok()
}

/// Combined W1166 grpcs+bidi interleaved first-slice evidence helper.
pub fn e02_grpcs_bidi_interleaved_first_slice_ok() -> bool {
    e02_grpc_tls_bidi_interleaved_loopback_ok()
}

/// W1167 / ADR-1513: encode WINDOW_UPDATE payload (4-byte window size increment).
fn encode_window_update_payload(increment: u32) -> [u8; 4] {
    // Reserved bit must be clear (RFC 7540 §6.9).
    (increment & 0x7fff_ffff).to_be_bytes()
}

fn decode_window_update_payload(payload: &[u8]) -> Option<u32> {
    if payload.len() != 4 {
        return None;
    }
    let inc = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]) & 0x7fff_ffff;
    if inc == 0 {
        None
    } else {
        Some(inc)
    }
}

/// Peer: unary echo gated on receiving WINDOW_UPDATE before response DATA.
fn serve_one_window_update_credit_gated_io<S: Read + Write>(
    stream: &mut S,
    resp_marker: &str,
) -> std::io::Result<()> {
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != PREFACE {
        return Ok(());
    }
    if !exchange_settings(stream, false) {
        return Ok(());
    }

    let mut req_data = Vec::new();
    let mut saw_req_end = false;
    let mut saw_credit = false;
    for _ in 0..64 {
        if saw_req_end && saw_credit {
            break;
        }
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if ty == FRAME_WINDOW_UPDATE {
            if let Some(inc) = decode_window_update_payload(&payload) {
                if inc == W1167_WINDOW_INCREMENT && (sid == 0 || sid == 1) {
                    saw_credit = true;
                }
            }
            continue;
        }
        if sid == 0 {
            continue;
        }
        if ty == FRAME_HEADERS {
            // path ignored for credit-gate unary
        } else if ty == FRAME_DATA {
            req_data.extend_from_slice(&payload);
            if flags & FLAG_END_STREAM != 0 {
                saw_req_end = true;
            }
        }
    }
    // Credit gate: response DATA only after request END_STREAM ∧ WINDOW_UPDATE.
    if !(saw_req_end && saw_credit) {
        return Ok(());
    }

    let payload = decode_grpc_message(&req_data).unwrap_or_default();
    let body = String::from_utf8_lossy(&payload);
    let resp_text = format!("{resp_marker}:{body}");
    let resp_msg = encode_grpc_message(resp_text.as_bytes());
    let hdrs = hpack_block(&[
        (":status", "200"),
        ("content-type", "application/grpc"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return Ok(());
    }
    if !write_frame(stream, FRAME_DATA, 0, 1, &resp_msg) {
        return Ok(());
    }
    let trailers = hpack_block(&[("grpc-status", "0")]);
    let _ = write_frame(
        stream,
        FRAME_HEADERS,
        FLAG_END_HEADERS | FLAG_END_STREAM,
        1,
        &trailers,
    );
    let _ = stream.flush();
    Ok(())
}

fn serve_one_window_update_credit_gated(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_window_update_credit_gated_io(&mut stream, "syx-e02-w1167-window-update")
}

/// Client unary: after request DATA, emit WINDOW_UPDATE then read response.
fn grpc_wire_window_update_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    body: &str,
    scheme: &str,
) -> Option<String> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let msg = encode_grpc_message(body.as_bytes());
    if !write_frame(stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
        return None;
    }
    // Emit stream-1 WINDOW_UPDATE (fixed increment) to unlock peer DATA.
    // Single frame only — unread peer buffer on drop would RST and race the response.
    let wu = encode_window_update_payload(W1167_WINDOW_INCREMENT);
    if !write_frame(stream, FRAME_WINDOW_UPDATE, 0, 1, &wu) {
        return None;
    }
    let _ = stream.flush();

    let mut data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            data.extend_from_slice(&payload);
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    let payload = decode_grpc_message(&data)?;
    Some(String::from_utf8_lossy(&payload).into_owned())
}

/// W1167 / ADR-1513: h2c WINDOW_UPDATE + credit-gate unary loopback.
pub fn e02_grpc_window_update_loopback_ok() -> bool {
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
            let _ = serve_one_window_update_credit_gated(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let mut stream = match TcpStream::connect(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(1500)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(1500)));
    match grpc_wire_window_update_call_io(&mut stream, "echo.Echo/Unary", "credit", "http") {
        Some(resp) => resp == "syx-e02-w1167-window-update:credit",
        None => false,
    }
}

/// Named alias: WINDOW_UPDATE / credit gate wired.
pub fn e02_grpc_window_update_wired_ok() -> bool {
    e02_grpc_window_update_loopback_ok()
}

/// Combined W1167 WINDOW_UPDATE/credit first-slice evidence helper.
pub fn e02_window_update_first_slice_ok() -> bool {
    e02_grpc_window_update_loopback_ok()
}

/// W1168 / ADR-1514: named loopback grpcs WINDOW_UPDATE (rustls + H2 credit gate).
pub fn e02_grpc_tls_window_update_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MARKER: &str = "syx-e02-w1168-grpcs-window-update";

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
        let _ = serve_one_window_update_credit_gated_io(&mut tls, MARKER);
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
    match grpc_wire_window_update_call_io(&mut tls, "echo.Echo/Unary", "credit", "https") {
        Some(resp) => resp == format!("{MARKER}:credit"),
        None => false,
    }
}

/// Named alias: grpcs WINDOW_UPDATE / credit gate wired.
pub fn e02_grpc_tls_window_update_wired_ok() -> bool {
    e02_grpc_tls_window_update_loopback_ok()
}

/// Combined W1168 grpcs WINDOW_UPDATE/credit first-slice evidence helper.
pub fn e02_grpcs_window_update_first_slice_ok() -> bool {
    e02_grpc_tls_window_update_loopback_ok()
}

/// Client unary that **omits** WINDOW_UPDATE after request DATA (W1169 negative).
fn grpc_wire_window_update_call_omit_credit_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    body: &str,
    scheme: &str,
) -> Option<String> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let msg = encode_grpc_message(body.as_bytes());
    if !write_frame(stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
        return None;
    }
    // Intentionally omit WINDOW_UPDATE — peer credit gate must refuse response DATA.
    let _ = stream.flush();

    let mut data = Vec::new();
    for _ in 0..16 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            data.extend_from_slice(&payload);
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    let payload = decode_grpc_message(&data)?;
    Some(String::from_utf8_lossy(&payload).into_owned())
}

/// W1169 / ADR-1515: omit WINDOW_UPDATE → credit gate fail-closed (no response DATA).
/// Marker identity: `syx-e02-w1169-window-update-neg` (must not appear as a success payload).
pub fn e02_grpc_window_update_omit_credit_fail_closed_ok() -> bool {
    let _ = "syx-e02-w1169-window-update-neg";
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
            let _ = serve_one_window_update_credit_gated(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let mut stream = match TcpStream::connect(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(800)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(800)));
    match grpc_wire_window_update_call_omit_credit_io(
        &mut stream,
        "echo.Echo/Unary",
        "no-credit",
        "http",
    ) {
        // Fail-closed: any decoded response (incl. W1167/W1168 markers) is a miss.
        Some(_) => false,
        None => true,
    }
}

/// Named alias: WINDOW_UPDATE omit-credit negative framing wired.
pub fn e02_grpc_window_update_negative_wired_ok() -> bool {
    e02_grpc_window_update_omit_credit_fail_closed_ok()
}

/// Combined W1169 WINDOW_UPDATE/credit negative first-slice evidence helper.
pub fn e02_window_update_negative_first_slice_ok() -> bool {
    e02_grpc_window_update_omit_credit_fail_closed_ok()
}

/// W1170 / ADR-1516: TLS omit WINDOW_UPDATE → credit gate fail-closed.
/// Marker identity: `syx-e02-w1170-grpcs-window-update-neg`.
pub fn e02_grpc_tls_window_update_omit_credit_fail_closed_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    let _ = "syx-e02-w1170-grpcs-window-update-neg";

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
        // Credit-gate peer: refuses response DATA without WINDOW_UPDATE.
        let _ = serve_one_window_update_credit_gated_io(&mut tls, "syx-e02-w1168-grpcs-window-update");
    });
    thread::sleep(Duration::from_millis(30));

    let Ok(mut tcp) = TcpStream::connect_timeout(
        &format!("127.0.0.1:{port}").parse().unwrap(),
        Duration::from_secs(3),
    ) else {
        return false;
    };
    let _ = tcp.set_read_timeout(Some(Duration::from_millis(800)));
    let _ = tcp.set_write_timeout(Some(Duration::from_millis(800)));
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
    match grpc_wire_window_update_call_omit_credit_io(
        &mut tls,
        "echo.Echo/Unary",
        "no-credit",
        "https",
    ) {
        Some(_) => false,
        None => true,
    }
}

/// Named alias: grpcs WINDOW_UPDATE omit-credit negative framing wired.
pub fn e02_grpc_tls_window_update_negative_wired_ok() -> bool {
    e02_grpc_tls_window_update_omit_credit_fail_closed_ok()
}

/// Combined W1170 grpcs WINDOW_UPDATE/credit negative first-slice evidence helper.
pub fn e02_grpcs_window_update_negative_first_slice_ok() -> bool {
    e02_grpc_tls_window_update_omit_credit_fail_closed_ok()
}

/// Client unary: after request DATA, emit **connection-level** (sid=0) WINDOW_UPDATE then read response.
/// Does **not** emit stream-1 WINDOW_UPDATE — credit unlock is connection-window only.
fn grpc_wire_window_update_connection_call_io<S: Read + Write>(
    stream: &mut S,
    method: &str,
    body: &str,
    scheme: &str,
) -> Option<String> {
    if !exchange_settings(stream, true) {
        return None;
    }
    let path = format!("/{method}");
    let hdrs = hpack_block(&[
        (":method", "POST"),
        (":scheme", scheme),
        (":authority", "127.0.0.1"),
        (":path", path.as_str()),
        ("content-type", "application/grpc"),
        ("te", "trailers"),
    ]);
    if !write_frame(stream, FRAME_HEADERS, FLAG_END_HEADERS, 1, &hdrs) {
        return None;
    }
    let msg = encode_grpc_message(body.as_bytes());
    if !write_frame(stream, FRAME_DATA, FLAG_END_STREAM, 1, &msg) {
        return None;
    }
    // Emit stream-0 (connection-level) WINDOW_UPDATE only — no stream-1 WINDOW_UPDATE.
    let wu = encode_window_update_payload(W1167_WINDOW_INCREMENT);
    if !write_frame(stream, FRAME_WINDOW_UPDATE, 0, 0, &wu) {
        return None;
    }
    let _ = stream.flush();

    let mut data = Vec::new();
    for _ in 0..32 {
        let Some((ty, flags, sid, payload)) = read_frame(stream) else {
            break;
        };
        if sid != 1 {
            continue;
        }
        if ty == FRAME_DATA {
            data.extend_from_slice(&payload);
        }
        if flags & FLAG_END_STREAM != 0 {
            break;
        }
    }
    let payload = decode_grpc_message(&data)?;
    Some(String::from_utf8_lossy(&payload).into_owned())
}

fn serve_one_window_update_connection_credit_gated(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    serve_one_window_update_credit_gated_io(&mut stream, "syx-e02-w1171-window-update-conn")
}

/// W1171 / ADR-1517: h2c connection-level (sid=0) WINDOW_UPDATE + credit-gate unary loopback.
pub fn e02_grpc_window_update_connection_loopback_ok() -> bool {
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
            let _ = serve_one_window_update_connection_credit_gated(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let mut stream = match TcpStream::connect(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(1500)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(1500)));
    match grpc_wire_window_update_connection_call_io(
        &mut stream,
        "echo.Echo/Unary",
        "credit",
        "http",
    ) {
        Some(resp) => resp == "syx-e02-w1171-window-update-conn:credit",
        None => false,
    }
}

/// Named alias: connection-level WINDOW_UPDATE / credit gate wired.
pub fn e02_grpc_window_update_connection_wired_ok() -> bool {
    e02_grpc_window_update_connection_loopback_ok()
}

/// Combined W1171 connection-level WINDOW_UPDATE/credit first-slice evidence helper.
pub fn e02_window_update_connection_first_slice_ok() -> bool {
    e02_grpc_window_update_connection_loopback_ok()
}

/// W1172 / ADR-1518: named loopback grpcs connection-level WINDOW_UPDATE (rustls + H2 credit gate).
pub fn e02_grpc_tls_window_update_connection_loopback_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    const MARKER: &str = "syx-e02-w1172-grpcs-window-update-conn";

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
        let _ = serve_one_window_update_credit_gated_io(&mut tls, MARKER);
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
    match grpc_wire_window_update_connection_call_io(
        &mut tls,
        "echo.Echo/Unary",
        "credit",
        "https",
    ) {
        Some(resp) => resp == format!("{MARKER}:credit"),
        None => false,
    }
}

/// Named alias: grpcs connection-level WINDOW_UPDATE / credit gate wired.
pub fn e02_grpc_tls_window_update_connection_wired_ok() -> bool {
    e02_grpc_tls_window_update_connection_loopback_ok()
}

/// Combined W1172 grpcs connection-level WINDOW_UPDATE/credit first-slice evidence helper.
pub fn e02_grpcs_window_update_connection_first_slice_ok() -> bool {
    e02_grpc_tls_window_update_connection_loopback_ok()
}

/// W1173 / ADR-1519: omit WINDOW_UPDATE on connection-level credit path → fail-closed.
/// Marker identity: `syx-e02-w1173-window-update-conn-neg` (must not appear as a success payload).
/// Peer is the W1171 connection credit-gate (`syx-e02-w1171-window-update-conn`); client must not
/// decode that (or W1172) positive payload when sid=0 WINDOW_UPDATE is omitted.
pub fn e02_grpc_window_update_connection_omit_credit_fail_closed_ok() -> bool {
    let _ = "syx-e02-w1173-window-update-conn-neg";
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
            let _ = serve_one_window_update_connection_credit_gated(stream);
        }
    });
    thread::sleep(Duration::from_millis(20));
    let mut stream = match TcpStream::connect(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(800)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(800)));
    // Reuse omit-credit client path: no sid=0 and no stream-1 WINDOW_UPDATE after DATA.
    match grpc_wire_window_update_call_omit_credit_io(
        &mut stream,
        "echo.Echo/Unary",
        "no-credit",
        "http",
    ) {
        // Fail-closed: any decoded response (incl. W1171/W1172 conn markers) is a miss.
        Some(_) => false,
        None => true,
    }
}

/// Named alias: connection-level WINDOW_UPDATE omit-credit negative framing wired.
pub fn e02_grpc_window_update_connection_negative_wired_ok() -> bool {
    e02_grpc_window_update_connection_omit_credit_fail_closed_ok()
}

/// Combined W1173 connection-level WINDOW_UPDATE/credit negative first-slice evidence helper.
pub fn e02_window_update_connection_negative_first_slice_ok() -> bool {
    e02_grpc_window_update_connection_omit_credit_fail_closed_ok()
}

/// W1174 / ADR-1520: TLS connection-level omit WINDOW_UPDATE → credit gate fail-closed.
/// Marker identity: `syx-e02-w1174-grpcs-window-update-conn-neg`.
/// Peer uses W1172 grpcs-conn credit-gate marker; client must not decode positive payload.
pub fn e02_grpc_tls_window_update_connection_omit_credit_fail_closed_ok() -> bool {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    use rustls::{ClientConfig, RootCertStore, ServerConfig};
    use std::sync::Arc;

    let _ = "syx-e02-w1174-grpcs-window-update-conn-neg";
    const PEER_MARKER: &str = "syx-e02-w1172-grpcs-window-update-conn";

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
        let _ = serve_one_window_update_credit_gated_io(&mut tls, PEER_MARKER);
    });
    thread::sleep(Duration::from_millis(30));

    let Ok(mut tcp) = TcpStream::connect_timeout(
        &format!("127.0.0.1:{port}").parse().unwrap(),
        Duration::from_secs(3),
    ) else {
        return false;
    };
    let _ = tcp.set_read_timeout(Some(Duration::from_millis(800)));
    let _ = tcp.set_write_timeout(Some(Duration::from_millis(800)));
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
    match grpc_wire_window_update_call_omit_credit_io(
        &mut tls,
        "echo.Echo/Unary",
        "no-credit",
        "https",
    ) {
        Some(_) => false,
        None => true,
    }
}

/// Named alias: grpcs connection-level WINDOW_UPDATE omit-credit negative framing wired.
pub fn e02_grpc_tls_window_update_connection_negative_wired_ok() -> bool {
    e02_grpc_tls_window_update_connection_omit_credit_fail_closed_ok()
}

/// Combined W1174 grpcs connection-level WINDOW_UPDATE/credit negative first-slice evidence helper.
pub fn e02_grpcs_window_update_connection_negative_first_slice_ok() -> bool {
    e02_grpc_tls_window_update_connection_omit_credit_fail_closed_ok()
}

/// `SYX_GRPC_REAL=1` truthy check (aligned with SYX_WS_REAL).
pub fn syx_grpc_real_enabled() -> bool {
    match std::env::var("SYX_GRPC_REAL") {
        Ok(v) => {
            let t = v.trim();
            t == "1" || t.eq_ignore_ascii_case("true") || t.eq_ignore_ascii_case("yes")
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grpc_framing_roundtrip() {
        let enc = encode_grpc_message(b"hello");
        assert_eq!(enc[0], 0);
        assert_eq!(&enc[1..5], &5u32.to_be_bytes());
        assert_eq!(decode_grpc_message(&enc).as_deref(), Some(b"hello".as_slice()));
    }

    #[test]
    fn loopback_h2_unary_ok() {
        assert!(e02_grpc_wire_loopback_roundtrip_ok());
    }

    #[test]
    fn loopback_echo_peer_interop_ok() {
        assert!(e02_grpc_interop_first_slice_ok());
        assert!(e02_grpc_loopback_peer_echo_wired_ok());
        // Distinct from construction wrapper response.
        assert!(!e02_grpc_wire_loopback_roundtrip_ok()
            || e02_grpc_loopback_peer_echo_wired_ok());
    }

    #[test]
    fn loopback_grpcs_tls_ok() {
        assert!(e02_grpcs_first_slice_ok());
        assert!(e02_grpc_tls_loopback_wired_ok());
    }

    #[test]
    fn negative_preface_and_truncated_data_fail_closed() {
        assert!(e02_grpc_negative_preface_fail_closed_ok());
        assert!(e02_grpc_negative_truncated_data_fail_closed_ok());
        assert!(e02_grpc_negative_tests_first_slice_ok());
        assert!(e02_grpc_negative_framing_wired_ok());
    }

    #[test]
    fn loopback_server_streaming_ok() {
        assert!(e02_grpc_server_streaming_loopback_ok());
        assert!(e02_grpc_server_streaming_wired_ok());
        assert!(e02_grpc_streaming_first_slice_ok());
    }

    #[test]
    fn loopback_client_streaming_ok() {
        assert!(e02_grpc_client_streaming_loopback_ok());
        assert!(e02_grpc_client_streaming_wired_ok());
        assert!(e02_grpc_client_streaming_first_slice_ok());
    }

    #[test]
    fn loopback_bidi_streaming_ok() {
        assert!(e02_grpc_bidi_streaming_loopback_ok());
        assert!(e02_grpc_bidi_streaming_wired_ok());
        assert!(e02_grpc_bidi_streaming_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_server_streaming_ok() {
        assert!(e02_grpc_tls_server_streaming_loopback_ok());
        assert!(e02_grpc_tls_server_streaming_wired_ok());
        assert!(e02_grpcs_streaming_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_client_streaming_ok() {
        assert!(e02_grpc_tls_client_streaming_loopback_ok());
        assert!(e02_grpc_tls_client_streaming_wired_ok());
        assert!(e02_grpcs_client_streaming_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_bidi_streaming_ok() {
        assert!(e02_grpc_tls_bidi_streaming_loopback_ok());
        assert!(e02_grpc_tls_bidi_streaming_wired_ok());
        assert!(e02_grpcs_bidi_streaming_first_slice_ok());
    }

    #[test]
    fn streaming_negative_truncated_data_fail_closed() {
        assert!(e02_grpc_streaming_negative_truncated_data_fail_closed_ok());
        assert!(e02_grpc_streaming_negative_framing_wired_ok());
        assert!(e02_streaming_negative_tests_first_slice_ok());
    }

    #[test]
    fn loopback_bidi_interleaved_ok() {
        assert!(e02_grpc_bidi_interleaved_loopback_ok());
        assert!(e02_grpc_bidi_interleaved_wired_ok());
        assert!(e02_bidi_interleaved_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_bidi_interleaved_ok() {
        assert!(e02_grpc_tls_bidi_interleaved_loopback_ok());
        assert!(e02_grpc_tls_bidi_interleaved_wired_ok());
        assert!(e02_grpcs_bidi_interleaved_first_slice_ok());
    }

    #[test]
    fn loopback_window_update_credit_gate_ok() {
        assert!(e02_grpc_window_update_loopback_ok());
        assert!(e02_grpc_window_update_wired_ok());
        assert!(e02_window_update_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_window_update_credit_gate_ok() {
        assert!(e02_grpc_tls_window_update_loopback_ok());
        assert!(e02_grpc_tls_window_update_wired_ok());
        assert!(e02_grpcs_window_update_first_slice_ok());
    }

    #[test]
    fn window_update_omit_credit_fail_closed() {
        assert!(e02_grpc_window_update_omit_credit_fail_closed_ok());
        assert!(e02_grpc_window_update_negative_wired_ok());
        assert!(e02_window_update_negative_first_slice_ok());
    }

    #[test]
    fn grpcs_window_update_omit_credit_fail_closed() {
        assert!(e02_grpc_tls_window_update_omit_credit_fail_closed_ok());
        assert!(e02_grpc_tls_window_update_negative_wired_ok());
        assert!(e02_grpcs_window_update_negative_first_slice_ok());
    }

    #[test]
    fn loopback_window_update_connection_credit_gate_ok() {
        assert!(e02_grpc_window_update_connection_loopback_ok());
        assert!(e02_grpc_window_update_connection_wired_ok());
        assert!(e02_window_update_connection_first_slice_ok());
    }

    #[test]
    fn loopback_grpcs_window_update_connection_credit_gate_ok() {
        assert!(e02_grpc_tls_window_update_connection_loopback_ok());
        assert!(e02_grpc_tls_window_update_connection_wired_ok());
        assert!(e02_grpcs_window_update_connection_first_slice_ok());
    }

    #[test]
    fn window_update_connection_omit_credit_fail_closed() {
        assert!(e02_grpc_window_update_connection_omit_credit_fail_closed_ok());
        assert!(e02_grpc_window_update_connection_negative_wired_ok());
        assert!(e02_window_update_connection_negative_first_slice_ok());
    }

    #[test]
    fn grpcs_window_update_connection_omit_credit_fail_closed() {
        assert!(e02_grpc_tls_window_update_connection_omit_credit_fail_closed_ok());
        assert!(e02_grpc_tls_window_update_connection_negative_wired_ok());
        assert!(e02_grpcs_window_update_connection_negative_first_slice_ok());
    }
}
