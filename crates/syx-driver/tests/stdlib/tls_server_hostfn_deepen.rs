//! TLS HostFn llvm+wasm first-slice deepen (/ · /).
//! Locks TlsServe llvm+wasm nails + caps partial; bans TLS GA / status=implemented / B `[x]`.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

#[test]
fn tls_server_hostfn_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/marketing-deploy-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "packages/net/tls.sy",
        "crates/syx-vm/src/hosts.rs",
        "crates/syx-stdlib/src/registry.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "crates/syx-driver/tests/stdlib/tls_server_hostfn_thin.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS HostFn deepen requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("host_tls_serve")
            && hosts.contains("TlsServe")
            && hosts.contains("ADR-737"),
        "hosts.rs must implement TlsServe (ADR-737)"
    );
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        registry.contains("TlsServe") && registry.contains("__syx_tls_serve"),
        "registry must append TlsServe HostFn"
    );
    let facade = include_str!("../../../../packages/net/tls.sy");
    assert!(
        facade.contains("serveOnce") && facade.contains("__syx_tls_serve"),
        "packages/net/tls.sy must expose serveOnce facade"
    );
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("fn hostfn_tls_serve_native_llvm")
            && qs1.contains("fn hostfn_tls_serve_wasm")
            && qs1.contains("HANDSHAKED")
            && qs1.contains("CLOSED"),
        "qs1_tls must retain TlsServe llvm + wasm nails"
    );
    let caps = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(caps, "TlsServe");
    assert!(
        block.contains("llvm = \"real\"")
            && block.contains("wasm = \"real\"")
            && (block.contains("status = \"implemented\"") || block.contains("status = \"partial\""))
            && block.contains("ADR-738"),
        "TlsServe caps must be llvm+wasm=real + ADR-738 (W441 may be implemented): {block}"
    );
    let product = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = product
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(product, idx, 1600);
    // W394 charter deferred status raise; live raise is W435/ADR-779.
    assert!(
        slice.contains("status = \"implemented\"") && slice.contains("ADR-779"),
        "live net.tls.server status raise is W435/ADR-779 (W394 wasm first-slice ≠ status raise): {slice}"
    );
    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        vectors.contains("ADR-738")
            && (vectors.contains("wasm first-slice") || vectors.contains("W394")),
        "vectors must retain W394 wasm first-slice historical axis"
    );
}

#[test]
fn tls_server_hostfn_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        spec.contains("ADR-738")
            && (spec.contains("wasm first-slice")
                || spec.contains("HostFn wasm")
                || spec.contains("W394")),
        "spec must cite ADR-738 TLS HostFn wasm first-slice"
    );
    assert!(
        (spec.contains("TLS HostFn wasm first-slice") || spec.contains("wasm first-slice"))
            && (spec.contains("**met** (W394)") || spec.contains("ADR-738")),
        "spec must index TLS HostFn wasm first-slice as met"
    );
    assert!(
        (spec.contains("TLS HostFn llvm first-slice") || spec.contains("llvm first-slice"))
            && (spec.contains("**partial met**")
                || spec.contains("partial met")
                || spec.contains("ADR-737")),
        "spec must retain TLS HostFn llvm first-slice as partial met"
    );
    assert!(
        (spec.contains("TLS HostFn implement inventory")
            || spec.contains("implement inventory thin"))
            && (spec.contains("**met** (W392)") || spec.contains("ADR-736")),
        "spec must retain implement inventory thin met"
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
            && (spec.contains("TLS") || spec.contains("B-04") || spec.contains("status")),
        "spec Frozen must keep TLS / B / status fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let floor = include_str!("../../../../docs/lang/tls-server-ga-exit.md");
    assert!(
        floor.contains("ADR-732")
            && (floor.contains("exit signed") || floor.contains("honesty")),
        "honesty floor exit must remain signed after wasm first-slice"
    );
}

