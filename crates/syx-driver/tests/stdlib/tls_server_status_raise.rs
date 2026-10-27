//! TLS product status raise first-slice
//! Caps `net.tls.server` → implemented; TlsServe HostFn still partial; ≠ handler deepen.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_status_raise_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-status-raise.md",
        "docs/lang/tls-server-ga-met-exit.md",
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "packages/internal/capabilities.toml",
        "packages/internal/host_fn_capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS status raise requires evidence file: {path}"
        );
    }
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("fn hostfn_tls_serve_native_llvm")
            && qs1.contains("fn hostfn_tls_serve_wasm"),
        "status raise evidence must retain W393/W394 TLS HostFn nails"
    );
}

#[test]
fn tls_server_status_raise_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-status-raise.md");
    assert!(
        spec.contains("ADR-779")
            && (spec.contains("status raise") || spec.contains("status=implemented")),
        "spec must cite ADR-779 TLS product status raise"
    );
    assert!(
        spec.contains("two-hundred-eighty-fifth"),
        
        "spec must cite W435 fingerprint"
    );
    assert!(
        (spec.contains("TlsServe") || spec.contains("HostFn"))
            && (spec.contains("partial")
                || spec.contains("still")
                || spec.contains("implemented")
                || spec.contains("W441")),
        "spec must cite TlsServe HostFn status honesty (W435 partial HostFn / W441 implemented)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("handler") || spec.contains("ALPN") || spec.contains("multi-conn")),
        "spec Frozen must fence handler deepen"
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
fn tls_server_status_raise_caps_implemented() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1600);
    assert!(
        slice.contains("status = \"implemented\""),
        "net.tls.server must be status=implemented after W435: {slice}"
    );
    assert!(
        slice.contains("ADR-779") && (slice.contains("W435") || slice.contains("status raise")),
        "net.tls.server notes must cite ADR-779 / W435: {slice}"
    );
    assert!(
        slice.contains("native = \"real\"") && slice.contains("wasm = \"real\""),
        "native/wasm backends must be real: {slice}"
    );
    assert!(
        slice.contains("vm = \"partial\"") && slice.contains("mir = \"partial\""),
        "vm/mir backends must stay partial honesty: {slice}"
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
            || (block.contains("status = \"partial\"") && block.contains("ADR-779")),
        "TlsServe HostFn live implemented (W441) or historical W435 partial: {block}"
    );
}

#[test]
fn tls_server_status_raise_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W435")
            && (roadmap.contains("TLS")
                || roadmap.contains("ADR-779")
                || roadmap.contains("status raise")
                || roadmap.contains("status=implemented")),
        "roadmap must cite W435 TLS product status raise"
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
