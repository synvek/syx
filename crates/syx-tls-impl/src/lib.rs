//! rustls `Tls.serve` host (moved out of `syx-vm`).

use std::net::{TcpListener, TcpStream};

use syx_diagnostics::SyxResult;
use syx_runtime_core::{platform_int_value, Value};

pub fn host_tls_serve(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::{ErrorKind, Read, Write};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
    use rustls::ServerConfig;

    /// W442 / ADR-786: continuous accept-idle before success exit (after ≥1 handled).
    const TLS_SERVE_IDLE: Duration = Duration::from_millis(400);
    /// W442 / ADR-786: fail if no connection handled before this overall deadline.
    const TLS_SERVE_OVERALL: Duration = Duration::from_secs(15);
    const TLS_SERVE_SPIN: Duration = Duration::from_millis(10);

    /// Parse comma-separated ALPN list; empty → default `[syx/1]`.
    fn tls_serve_parse_alpn_list(raw: &str) -> Vec<Vec<u8>> {
        let list: Vec<Vec<u8>> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.as_bytes().to_vec())
            .collect();
        if list.is_empty() {
            vec![b"syx/1".to_vec()]
        } else {
            list
        }
    }

    /// W445 / ADR-789: arg > env `SYX_TLS_SERVE_ALPN` > default `[syx/1]`.
    fn tls_serve_alpn_protocols(alpn_arg: &str) -> Vec<Vec<u8>> {
        let trimmed = alpn_arg.trim();
        if !trimmed.is_empty() {
            return tls_serve_parse_alpn_list(trimmed);
        }
        match std::env::var("SYX_TLS_SERVE_ALPN") {
            Ok(raw) => tls_serve_parse_alpn_list(&raw),
            Err(_) => vec![b"syx/1".to_vec()],
        }
    }

    /// W457 / ADR-801: non-empty `SYX_TLS_SERVE_NO_IDLE` → skip idle break.
    fn tls_serve_no_idle_env() -> bool {
        match std::env::var("SYX_TLS_SERVE_NO_IDLE") {
            Ok(raw) => !raw.trim().is_empty(),
            Err(_) => false,
        }
    }

    /// W457 / ADR-801: optional positive `SYX_TLS_SERVE_MAX_HANDLED` bound.
    fn tls_serve_max_handled_env() -> Option<u32> {
        match std::env::var("SYX_TLS_SERVE_MAX_HANDLED") {
            Ok(raw) => raw.trim().parse::<u32>().ok().filter(|&n| n > 0),
            Err(_) => None,
        }
    }

    /// W444 / ADR-788: per-connection handshake + HANDLED on a worker thread.
    fn tls_serve_handle_conn(mut stream: TcpStream, config: Arc<ServerConfig>) -> bool {
        if stream.set_nonblocking(false).is_err() {
            return false;
        }
        let mut conn = match rustls::ServerConnection::new(config) {
            Ok(c) => c,
            Err(_) => return false,
        };
        loop {
            if !conn.is_handshaking() {
                break;
            }
            match conn.complete_io(&mut stream) {
                Ok((0, 0)) => break,
                Ok(_) => continue,
                Err(_) => return false,
            }
        }
        if conn.is_handshaking() {
            return false;
        }
        {
            let mut tls = rustls::Stream::new(&mut conn, &mut stream);
            let mut buf = [0u8; 256];
            match tls.read(&mut buf) {
                Ok(n) if n > 0 => {}
                _ => return false,
            }
            if tls.write_all(b"HANDLED\n").is_err() {
                return false;
            }
            let _ = tls.flush();
        }
        let _ = stream.shutdown(std::net::Shutdown::Both);
        true
    }

    // W458 / ADR-802: pop order no_idle, alpn, key, cert, port, host.
    let no_idle_arg = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let alpn_arg = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let key_path = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let cert_path = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let port = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let host = stack.pop().map(|v| v.as_str()).unwrap_or_default();

    let fail = |stack: &mut Vec<Value>| -> SyxResult<()> {
        stack.push(platform_int_value(-1));
        Ok(())
    };

    if host.is_empty() || port <= 0 || cert_path.is_empty() || key_path.is_empty() {
        return fail(stack);
    }

    let certs = match CertificateDer::pem_file_iter(std::path::Path::new(cert_path.as_str())) {
        Ok(iter) => match iter.collect::<Result<Vec<_>, _>>() {
            Ok(c) if !c.is_empty() => c,
            _ => return fail(stack),
        },
        Err(_) => return fail(stack),
    };
    let key = match PrivateKeyDer::from_pem_file(std::path::Path::new(key_path.as_str())) {
        Ok(k) => k,
        Err(_) => return fail(stack),
    };
    let config = match ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
    {
        Ok(mut c) => {
            // W445 / ADR-789: language alpn arg > SYX_TLS_SERVE_ALPN > default syx/1.
            c.alpn_protocols = tls_serve_alpn_protocols(alpn_arg.as_str());
            Arc::new(c)
        }
        Err(_) => return fail(stack),
    };

    let addr = format!("{host}:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(_) => return fail(stack),
    };
    if listener.set_nonblocking(true).is_err() {
        return fail(stack);
    }

    // W444 / ADR-788: concurrent thread-per-conn + idle drain.
    // W457 / ADR-801: optional no-idle + max-handled env (default idle unchanged).
    // W458 / ADR-802: language noIdle≠0 wins; else env; else idle.
    let no_idle = no_idle_arg != 0 || tls_serve_no_idle_env();
    let max_handled = tls_serve_max_handled_env();
    let overall_deadline = Instant::now() + TLS_SERVE_OVERALL;
    let handled = Arc::new(AtomicU32::new(0));
    let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut idle_since: Option<Instant> = None;

    let reap_finished = |workers: &mut Vec<thread::JoinHandle<()>>| {
        let mut still = Vec::with_capacity(workers.len());
        for h in workers.drain(..) {
            if h.is_finished() {
                let _ = h.join();
            } else {
                still.push(h);
            }
        }
        *workers = still;
    };

    loop {
        reap_finished(&mut workers);
        match listener.accept() {
            Ok((stream, _)) => {
                idle_since = None;
                let cfg = Arc::clone(&config);
                let handled_c = Arc::clone(&handled);
                workers.push(thread::spawn(move || {
                    if tls_serve_handle_conn(stream, cfg) {
                        handled_c.fetch_add(1, Ordering::SeqCst);
                    }
                }));
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                reap_finished(&mut workers);
                let h = handled.load(Ordering::SeqCst);
                if h >= 1 && workers.is_empty() {
                    // W457: optional max-handled success exit (test/ops bound).
                    if let Some(max) = max_handled {
                        if h >= max {
                            break;
                        }
                    }
                    // W457: NO_IDLE skips 400ms idle break; default path unchanged.
                    if !no_idle {
                        let since = *idle_since.get_or_insert_with(Instant::now);
                        if since.elapsed() >= TLS_SERVE_IDLE {
                            break;
                        }
                    }
                } else {
                    idle_since = None;
                    if h == 0 && Instant::now() >= overall_deadline && workers.is_empty() {
                        return fail(stack);
                    }
                }
                thread::sleep(TLS_SERVE_SPIN);
            }
            Err(_) => {
                for h in workers {
                    let _ = h.join();
                }
                return fail(stack);
            }
        }
    }
    for h in workers {
        let _ = h.join();
    }
    stack.push(platform_int_value(0));
    Ok(())
}
