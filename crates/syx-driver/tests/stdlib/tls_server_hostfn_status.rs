//! TLS TlsServe HostFn status→implemented first-slice
//! status=implemented; HANDLED+ALPN retained; charter N=2; live may be unbounded.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_hostfn_status_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-hostfn-status.md",
        "docs/lang/tls-server-alpn.md",
        "docs/lang/tls-server-status-raise.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS HostFn status raise requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    // W441 charter retained N=2; live W442 may use idle unbounded — both require ALPN+HANDLED.
    assert!(
        (hosts.contains("for _ in 0..2") || hosts.contains("TLS_SERVE_IDLE") || hosts.contains("ADR-786"))
            && hosts.contains("syx/1")
            && hosts.contains("HANDLED"),
        "host_tls_serve must retain ALPN syx/1 + HANDLED (N=2 historical or idle unbounded)"
    );
}

#[test]
fn tls_server_hostfn_status_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-hostfn-status.md");
    assert!(
        spec.contains("ADR-785")
            && (spec.contains("implemented") || spec.contains("HostFn status")),
        "spec must cite ADR-785 HostFn status raise"
    );
    assert!(
        spec.contains("two-hundred-ninety-first"),
        
        "spec must cite W441 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unbounded") || spec.contains("configurable") || spec.contains("ALPN")),
        "spec Frozen must fence unbounded / configurable ALPN"
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
fn tls_server_hostfn_status_caps_and_hostfn() {
    let hosts = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let needle = "name = \"TlsServe\"";
    let rest = hosts
        .split(needle)
        .nth(1)
        .expect("TlsServe host row");
    let block = rest.split("[[host]]").next().unwrap_or(rest);
    assert!(
        block.contains("status = \"implemented\""),
        "TlsServe HostFn must be implemented (ADR-785): {block}"
    );
    assert!(
        block.contains("ADR-785"),
        "TlsServe notes must cite ADR-785: {block}"
    );
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "TlsServe backends must remain llvm+wasm=real: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1200);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented: {slice}"
    );
    assert!(
        slice.contains("ADR-785") || slice.contains("HostFn"),
        "net.tls.server notes must cross ADR-785: {slice}"
    );
}

#[test]
fn tls_server_hostfn_status_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W441")
            && (roadmap.contains("TlsServe")
                || roadmap.contains("ADR-785")
                || roadmap.contains("HostFn")
                || roadmap.contains("implemented")),
        "roadmap must cite W441 TlsServe HostFn status"
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
