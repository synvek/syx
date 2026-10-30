//! TLS language ALPN parameter first-slice
//! 5th arg alpn; empty → env/default; ≠ Array alpn / ≠ daemon / ≠ B-04 [x].

fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}

#[test]
fn tls_server_alpn_lang_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-alpn-lang.md",
        "docs/lang/tls-server-concurrent.md",
        "docs/lang/tls-server-alpn-config.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS language ALPN requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("ADR-789")
            && hosts.contains("tls_serve_alpn_protocols")
            && hosts.contains("alpn_arg")
            && hosts.contains("SYX_TLS_SERVE_ALPN")
            && hosts.contains("HANDLED"),
        "host_tls_serve must take language alpn arg with env fallback (ADR-789)"
    );
    let facade = include_str!("../../../../packages/net/tls.sy");
    assert!(
        facade.contains("serveOnceAlpn")
            && facade.contains("__syx_tls_serve(host, port, certPemPath, keyPemPath, \"\", 0)"),
        "tls.sy must expose serveOnceAlpn and serveOnce empty-alpn path"
    );
}

#[test]
fn tls_server_alpn_lang_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-alpn-lang.md");
    assert!(
        spec.contains("ADR-789")
            && (spec.contains("language") || spec.contains("5th")),
        "spec must cite ADR-789 language ALPN"
    );
    assert!(
        spec.contains("two-hundred-ninety-fifth"),
        
        "spec must cite W445 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("daemon") || spec.contains("Array") || spec.contains("B-04")),
        "spec Frozen must fence daemon / Array alpn / B-04"
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
fn tls_server_alpn_lang_caps_and_hostfn() {
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
        block.contains("ADR-789")
            && (block.contains("language ALPN")
                || block.contains("5th arg")
                || block.contains("serveOnceAlpn")),
        "TlsServe notes must cite ADR-789 language ALPN: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 2000);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented: {slice}"
    );
    assert!(
        slice.contains("ADR-789") || slice.contains("language ALPN") || slice.contains("serveOnceAlpn"),
        "net.tls.server notes must cross ADR-789: {slice}"
    );
}

