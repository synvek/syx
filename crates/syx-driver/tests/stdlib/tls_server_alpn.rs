//! TLS ALPN first-slice
//! host_tls_serve: fixed syx/1 ALPN + N=2 HANDLED; TlsServe still partial; ≠ configurable ALPN/unbounded.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_alpn_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-alpn.md",
        "docs/lang/tls-server-multi-conn.md",
        "docs/lang/tls-server-handler-appdata.md",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "crates/syx-vm/src/hosts.rs",
        "packages/net/tls.sy",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS ALPN requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("syx/1")
            && (hosts.contains("alpn_protocols") || hosts.contains("ALPN")),
        "host_tls_serve must set ALPN syx/1 (ADR-782)"
    );
    assert!(
        hosts.contains("ADR-782") || hosts.contains("W438"),
        "host_tls_serve must cite W438 / ADR-782 ALPN"
    );
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("syx/1") && qs1.contains("alpn_protocol"),
        "qs1_tls must nail ALPN syx/1 on llvm+wasm"
    );
}

#[test]
fn tls_server_alpn_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-alpn.md");
    assert!(
        spec.contains("ADR-782")
            && (spec.contains("ALPN") || spec.contains("syx/1")),
        "spec must cite ADR-782 TLS ALPN"
    );
    assert!(
        spec.contains("two-hundred-eighty-eighth"),
        
        "spec must cite W438 fingerprint"
    );
    assert!(
        (spec.contains("TlsServe") || spec.contains("HostFn"))
            && (spec.contains("partial")
                || spec.contains("still")
                || spec.contains("implemented")
                || spec.contains("W441")),
        "spec must cite TlsServe HostFn status honesty (W438 partial / W441 implemented)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("configurable") || spec.contains("unbounded") || spec.contains("Short")),
        "spec Frozen must fence configurable ALPN / unbounded / Short"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    for line in spec.lines() {
        if line.contains("B-05")
            && line.contains("[x]")
            && !line.contains("≠")
            && !line.contains("仍")
            && !line.contains("Frozen")
            && !line.contains("still")
            && !line.contains("Forbidden")
            && !line.contains("Claiming")
        {
            panic!("affirmative B-05 [x] claim forbidden: {line}");
        }
    }
}

#[test]
fn tls_server_alpn_caps_and_hostfn() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1600);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented after W438: {slice}"
    );
    assert!(
        slice.contains("ADR-782") || slice.contains("ALPN") || slice.contains("syx/1"),
        "net.tls.server notes must cite ADR-782 ALPN: {slice}"
    );
    let hosts = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let needle = "name = \"TlsServe\"";
    let rest = hosts
        .split(needle)
        .nth(1)
        .expect("TlsServe host row");
    let block = rest.split("[[host]]").next().unwrap_or(rest);
    assert!(
        block.contains("status = \"implemented\"")
            || (block.contains("status = \"partial\"") && block.contains("ADR-782")),
        "TlsServe HostFn live implemented (W441) or historical W438 partial: {block}"
    );
    assert!(
        block.contains("ADR-782"),
        "TlsServe notes must cite ADR-782: {block}"
    );
}

