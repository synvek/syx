//! TLS unbounded multi-conn first-slice
//! idle-timeout accept; live concurrent (historical ≠ concurrent); env ALPN .


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_unbounded_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-unbounded.md",
        "docs/lang/tls-server-multi-conn.md",
        "docs/lang/tls-server-hostfn-status.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS unbounded requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        !hosts.contains("for _ in 0..2"),
        "host_tls_serve must not use fixed N=2 loop"
    );
    assert!(
        hosts.contains("TLS_SERVE_IDLE")
            && hosts.contains("set_nonblocking")
            && hosts.contains("syx/1")
            && hosts.contains("HANDLED")
            && hosts.contains("ADR-786"),
        "host_tls_serve must be idle-timeout unbounded + ALPN syx/1 + HANDLED (ADR-786)"
    );
}

#[test]
fn tls_server_unbounded_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-unbounded.md");
    assert!(
        spec.contains("ADR-786")
            && (spec.contains("unbounded") || spec.contains("idle")),
        "spec must cite ADR-786 unbounded idle-exit"
    );
    assert!(
        spec.contains("two-hundred-ninety-second"),
        
        "spec must cite W442 fingerprint"
    );
    assert!(
        spec.contains("400") && (spec.contains("15") || spec.contains("8")),
        "spec must nail idle 400ms and overall first-accept budget"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("concurrent") || spec.contains("configurable") || spec.contains("ALPN")),
        "spec Frozen must fence concurrent / configurable ALPN"
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
fn tls_server_unbounded_caps_and_hostfn() {
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
        block.contains("ADR-786"),
        "TlsServe notes must cite ADR-786: {block}"
    );
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "TlsServe backends must remain llvm+wasm=real: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1400);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented: {slice}"
    );
    assert!(
        slice.contains("ADR-786") || slice.contains("unbounded"),
        "net.tls.server notes must cross ADR-786: {slice}"
    );
}

