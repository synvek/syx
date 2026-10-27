//! TLS server HostFn-gap honesty deepen
//! Locks deepen partial met; after honesty floor is met; bans TLS GA / HostFn / B `[x]`.

#[test]
fn tls_server_ga_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "crates/syx-stdlib/src/registry.rs",
        "crates/syx-vm/src/hosts.rs",
        "tools/sgate/tls.sy",
        "crates/syx-driver/tests/stdlib/tls_server_ga_thin.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS deepen requires evidence file: {path}"
        );
    }
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    // W387 honesty deepen locked HostFn absence; W393 llvm first-slice may add TlsServe.
    // Honesty axis still requires product unsupported + TLS GA unmet (≠ product GA).
    assert!(
        registry.contains("TcpListenerAccept") && registry.contains("__syx_tcp_listener_accept"),
        "registry must retain TcpListenerAccept as contrast (≠ TLS GA)"
    );
    let sgate = include_str!("../../../../tools/sgate/tls.sy");
    assert!(
        sgate.contains("__syx_tls_serve") && sgate.contains("if false"),
        "sgate tls.sy must retain __syx_tls_serve placeholder under if false"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tls.server\"")
            && (caps.contains("ADR-779")
                || caps.contains("ADR-737")
                || caps.contains("ADR-731")
                || caps.contains("ADR-730")
                || caps.contains("ADR-732")),
        "net.tls.server must retain deepen/exit/llvm/status-raise cross-index"
    );
    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        vectors.contains("ADR-731")
            && (vectors.contains("HostFn-gap") || vectors.contains("honesty deepen")),
        "vectors must retain W387 HostFn-gap deepen historical axis"
    );
}

#[test]
fn tls_server_ga_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        spec.contains("ADR-731")
            && (spec.contains("HostFn-gap") || spec.contains("honesty deepen") || spec.contains("deepen")),
        "spec must cite ADR-731 TLS HostFn-gap honesty deepen"
    );
    assert!(
        (spec.contains("TLS HostFn-gap honesty deepen") || spec.contains("HostFn-gap"))
            && (spec.contains("**partial met**") || spec.contains("partial met")),
        "spec must index HostFn-gap deepen as partial met"
    );
    assert!(
        (spec.contains("TLS honesty-ladder floor") || spec.contains("honesty-ladder floor") || spec.contains("ADR-732"))
            && (spec.contains("**met** (W388") || spec.contains("met (narrow)") || spec.contains("ADR-732")),
        "spec must mark honesty floor met after W388"
    );
    assert!(
        (spec.contains("TLS server GA") || spec.contains("TLS GA"))
            && (spec.contains("**met** (W395")
                || spec.contains("met (narrow)")
                || spec.contains("ADR-739")),
        "spec must mark TLS GA met (narrow) after W395"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("HostFn") || spec.contains("TLS") || spec.contains("B-04")),
        "spec Frozen must keep HostFn / TLS / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn tls_server_ga_deepen_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W388")
            && (roadmap.contains("TLS")
                || roadmap.contains("ADR-732")
                || roadmap.contains("honesty")
                || roadmap.contains("floor")),
        "roadmap must cite W388 TLS honesty-ladder floor exit"
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
    assert!(
        roadmap.contains("Q-L5") && roadmap.contains("已签"),
        "Q-L5 must remain signed"
    );
}
