//! TLS concurrent serve first-slice
//! thread-per-conn + idle drain; live language alpn ; ≠ daemon / ≠ Array alpn / ≠ B-04 [x].

fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}

#[test]
fn tls_server_concurrent_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-concurrent.md",
        "docs/lang/tls-server-alpn-config.md",
        "docs/lang/tls-server-unbounded.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS concurrent requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("ADR-788")
            && hosts.contains("tls_serve_handle_conn")
            && hosts.contains("thread::spawn")
            && hosts.contains("TLS_SERVE_IDLE")
            && hosts.contains("HANDLED")
            && hosts.contains("SYX_TLS_SERVE_ALPN"),
        "host_tls_serve must be thread-per-conn + idle + ALPN env (ADR-788)"
    );
}

#[test]
fn tls_server_concurrent_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-concurrent.md");
    assert!(
        spec.contains("ADR-788")
            && (spec.contains("concurrent") || spec.contains("thread-per-conn")),
        "spec must cite ADR-788 concurrent"
    );
    assert!(
        spec.contains("two-hundred-ninety-fourth"),
        
        "spec must cite W444 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("daemon")
                || spec.contains("thread-pool")
                || spec.contains("Array")
                || spec.contains("alpn")),
        "spec Frozen must fence daemon / thread-pool / Array alpn"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    for line in spec.lines() {
        if line.contains("B-04")
            && line.contains("[x]")
            && !line.contains("≠")
            && !line.contains("仍")
            && !line.contains("Frozen")
            && !line.contains("still")
            && !line.contains("Forbidden")
            && !line.contains("Claiming")
            && !line.contains("unmet")
            && !line.contains("exit charter")
            && !line.contains("narrow")
            && !line.contains("ADR-790")
            && !line.contains("W446")
            && !line.contains("**met**")
        {
            panic!("affirmative B-04 [x] claim forbidden: {line}");
        }
    }
}

#[test]
fn tls_server_concurrent_caps_and_hostfn() {
    let hosts = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let needle = "name = \"TlsServe\"";
    let rest = hosts
        .split(needle)
        .nth(1)
        .expect("TlsServe host row");
    let block = rest.split("[[host]]").next().unwrap_or(rest);
    assert!(
        block.contains("status = \"implemented\""),
        "TlsServe HostFn must remain implemented: {block}"
    );
    assert!(
        block.contains("ADR-788")
            && (block.contains("concurrent") || block.contains("thread-per-conn")),
        "TlsServe notes must cite ADR-788 concurrent: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1800);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented: {slice}"
    );
    assert!(
        slice.contains("ADR-788") || slice.contains("concurrent"),
        "net.tls.server notes must cross ADR-788: {slice}"
    );
}

