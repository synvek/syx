//! TLS configurable ALPN first-slice
//! SYX_TLS_SERVE_ALPN env; default syx/1; live language alpn ; live concurrent (historical ≠ concurrent).


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_alpn_config_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-alpn-config.md",
        "docs/lang/tls-server-alpn.md",
        "docs/lang/tls-server-unbounded.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS ALPN config requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("SYX_TLS_SERVE_ALPN")
            && hosts.contains("tls_serve_alpn_protocols")
            && hosts.contains("ADR-787")
            && hosts.contains("syx/1"),
        "host_tls_serve must parse SYX_TLS_SERVE_ALPN (ADR-787)"
    );
    assert!(
        hosts.contains("TLS_SERVE_IDLE") && hosts.contains("HANDLED"),
        "idle + HANDLED behavior must remain"
    );
}

#[test]
fn tls_server_alpn_config_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-alpn-config.md");
    assert!(
        spec.contains("ADR-787")
            && spec.contains("SYX_TLS_SERVE_ALPN")
            && (spec.contains("configurable") || spec.contains("comma")),
        "spec must cite ADR-787 SYX_TLS_SERVE_ALPN"
    );
    assert!(
        spec.contains("two-hundred-ninety-third"),
        
        "spec must cite W443 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("Array")
                || spec.contains("daemon")
                || spec.contains("thread-pool")),
        "spec Frozen must fence Array alpn / daemon / thread-pool"
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
fn tls_server_alpn_config_caps_and_hostfn() {
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
        block.contains("ADR-787") && block.contains("SYX_TLS_SERVE_ALPN"),
        "TlsServe notes must cite ADR-787 env ALPN: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1600);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented: {slice}"
    );
    assert!(
        slice.contains("ADR-787") || slice.contains("SYX_TLS_SERVE_ALPN"),
        "net.tls.server notes must cross ADR-787: {slice}"
    );
}

#[test]
fn tls_server_alpn_config_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W443")
            && (roadmap.contains("ALPN")
                || roadmap.contains("ADR-787")
                || roadmap.contains("SYX_TLS_SERVE_ALPN")),
        "roadmap must cite W443 configurable ALPN"
    );
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
