//! TLS multi-conn first-slice
//! host_tls_serve: exactly 2 sequential accepts + HANDLED each; TlsServe still partial; ≠ ALPN/unbounded.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_multi_conn_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-multi-conn.md",
        "docs/lang/tls-server-handler-appdata.md",
        "docs/lang/tls-server-status-raise.md",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "crates/syx-vm/src/hosts.rs",
        "packages/net/tls.sy",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS multi-conn requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    // W437 historical N=2 OR live W442 idle-timeout unbounded (ADR-786).
    assert!(
        hosts.contains("for _ in 0..2")
            || hosts.contains("TLS_SERVE_IDLE")
            || hosts.contains("ADR-786"),
        "host_tls_serve must show W437 N=2 historical or W442 idle unbounded"
    );
    assert!(
        hosts.contains("ADR-781")
            || hosts.contains("multi-conn first-slice")
            || hosts.contains("W437")
            || hosts.contains("ADR-786"),
        "host_tls_serve must cite W437 / ADR-781 multi-conn or W442 / ADR-786"
    );
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("handled1") && qs1.contains("handled2"),
        "qs1_tls must nail two-client HANDLED on llvm+wasm"
    );
}

#[test]
fn tls_server_multi_conn_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-multi-conn.md");
    assert!(
        spec.contains("ADR-781")
            && (spec.contains("multi-conn") || spec.contains("N=2") || spec.contains("2")),
        "spec must cite ADR-781 TLS multi-conn"
    );
    assert!(
        spec.contains("two-hundred-eighty-seventh"),
        
        "spec must cite W437 fingerprint"
    );
    assert!(
        (spec.contains("TlsServe") || spec.contains("HostFn"))
            && (spec.contains("partial")
                || spec.contains("still")
                || spec.contains("implemented")
                || spec.contains("W441")),
        "spec must cite TlsServe HostFn status honesty (W437 partial / W441 implemented)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("ALPN") || spec.contains("unbounded")),
        "spec Frozen must fence ALPN / unbounded multi-conn"
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
fn tls_server_multi_conn_caps_and_hostfn() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1600);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented after W437: {slice}"
    );
    assert!(
        slice.contains("ADR-781") || slice.contains("multi-conn"),
        "net.tls.server notes must cite ADR-781 multi-conn: {slice}"
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
            || (block.contains("status = \"partial\"") && block.contains("ADR-781")),
        "TlsServe HostFn live implemented (W441) or historical W437 partial: {block}"
    );
    assert!(
        block.contains("ADR-781"),
        "TlsServe notes must cite ADR-781: {block}"
    );
}

#[test]
fn tls_server_multi_conn_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W437")
            && (roadmap.contains("TLS")
                || roadmap.contains("ADR-781")
                || roadmap.contains("multi-conn")),
        "roadmap must cite W437 TLS multi-conn"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(a10.contains("| [x] |"), "A-10 must remain [x]: {a10}");
    for id in ["B-04", "B-05", "B-06", "B-07", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} must remain [x] after W456/W457: {row}"
        );
    }
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(b08.contains("| [x] |"), "B-08 must remain [x]: {b08}");
}
