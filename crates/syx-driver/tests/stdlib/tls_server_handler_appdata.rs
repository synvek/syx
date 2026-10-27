//! TLS handler application-data first-slice
//! host_tls_serve: handshake + one app-data read/write HANDLED; TlsServe still partial; ≠ ALPN/multi-conn.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_handler_appdata_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-handler-appdata.md",
        "docs/lang/tls-server-status-raise.md",
        "docs/lang/tls-server-ga-vectors.md",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "crates/syx-vm/src/hosts.rs",
        "packages/net/tls.sy",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS handler app-data requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("HANDLED\\n")
            || hosts.contains("b\"HANDLED\\n\"")
            || hosts.contains("HANDLED"),
        "host_tls_serve must write HANDLED application-data (ADR-780)"
    );
    assert!(
        hosts.contains("ADR-780")
            || hosts.contains("application-data first-slice")
            || hosts.contains("W436"),
        "host_tls_serve must cite W436 / ADR-780 app-data"
    );
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("HANDLED")
            && qs1.contains("fn hostfn_tls_serve_native_llvm")
            && qs1.contains("fn hostfn_tls_serve_wasm"),
        "qs1_tls must nail HANDLED app-data on llvm+wasm"
    );
}

#[test]
fn tls_server_handler_appdata_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-handler-appdata.md");
    assert!(
        spec.contains("ADR-780")
            && (spec.contains("application-data") || spec.contains("app-data") || spec.contains("HANDLED")),
        "spec must cite ADR-780 TLS handler application-data"
    );
    assert!(
        spec.contains("two-hundred-eighty-sixth"),
        
        "spec must cite W436 fingerprint"
    );
    assert!(
        (spec.contains("TlsServe") || spec.contains("HostFn"))
            && (spec.contains("partial")
                || spec.contains("still")
                || spec.contains("implemented")
                || spec.contains("W441")),
        "spec must cite TlsServe HostFn status honesty (W436 partial / W441 implemented)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("ALPN") || spec.contains("multi-conn")),
        "spec Frozen must fence ALPN / multi-conn"
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
fn tls_server_handler_appdata_caps_and_hostfn() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 700);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must remain implemented after W436: {slice}"
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
            || (block.contains("status = \"partial\"") && block.contains("ADR-780")),
        "TlsServe HostFn live implemented (W441) or historical W436 partial: {block}"
    );
    assert!(
        block.contains("ADR-780"),
        "TlsServe notes must cite ADR-780: {block}"
    );
}

#[test]
fn tls_server_handler_appdata_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W436")
            && (roadmap.contains("TLS")
                || roadmap.contains("ADR-780")
                || roadmap.contains("application-data")
                || roadmap.contains("app-data")
                || roadmap.contains("HANDLED")),
        "roadmap must cite W436 TLS handler application-data"
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
