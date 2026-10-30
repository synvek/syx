//! TLS HostFn implement thin-open inventory
//! After : HostFn may exist; still locks inventory signed; bans TLS GA / status raise / B `[x]`.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_hostfn_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/marketing-deploy-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-stdlib/src/registry.rs",
        "crates/syx-vm/src/hosts.rs",
        "packages/net/tls.sy",
        "crates/syx-driver/tests/stdlib/tls_server_ga_thin.rs",
        "crates/syx-driver/tests/stdlib/tls_server_ga_deepen.rs",
        "crates/syx-driver/tests/stdlib/tls_server_ga_exit.rs",
        "crates/syx-driver/tests/stdlib/tls_server_hostfn_deepen.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS HostFn implement thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tls.server\"")
            && (caps.contains("ADR-779")
                || caps.contains("ADR-738")
                || caps.contains("ADR-737")
                || caps.contains("ADR-736")
                || caps.contains("ADR-732")
                || caps.contains("ADR-730")),
        "capabilities.toml must retain net.tls.server + implement/llvm/wasm/status-raise cross-index"
    );
}

#[test]
fn tls_server_hostfn_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        spec.contains("ADR-736")
            && (spec.contains("implement inventory")
                || spec.contains("HostFn implement inventory")
                || spec.contains("implement thin")),
        "spec must cite ADR-736 TLS HostFn implement thin inventory"
    );
    assert!(
        (spec.contains("TLS HostFn implement inventory thin")
            || spec.contains("implement inventory thin"))
            && (spec.contains("**met** (W392)") || spec.contains("ADR-736")),
        "spec must index TLS HostFn implement inventory thin met"
    );
    assert!(
        (spec.contains("TLS server GA") || spec.contains("TLS GA"))
            && (spec.contains("**met** (W395")
                || spec.contains("met (narrow)")
                || spec.contains("ADR-739")),
        "spec must mark TLS GA met (narrow) after W395"
    );
    assert!(
        spec.contains("TlsServe")
            && spec.contains("__syx_tls_serve")
            && (spec.contains("llvm-only")
                || spec.contains("llvm first-slice")
                || spec.contains("llvm")),
        "spec must bound candidate HostFn names + llvm first-slice"
    );
    assert!(
        (spec.contains("wasm") || spec.contains("Wasm"))
            && (spec.contains("**met** (W394)")
                || spec.contains("ADR-738")
                || spec.contains("wasm first-slice")
                || spec.contains("W394")),
        "spec must index wasm first-slice after W394"
    );
    assert!(
        spec.contains("noop") || spec.contains("disguise"),
        "spec must ban noop TlsServe disguise"
    );
    assert!(
        (spec.contains("tls.sy") || spec.contains("packages/net"))
            && (spec.contains("facade")
                || spec.contains("serveOnce")
                || spec.contains("wired")
                || spec.contains("first-slice")
                || spec.contains("W393")),
        "spec must cite packages/net/tls.sy facade after W393"
    );
    assert!(
        (spec.contains("honesty") || spec.contains("W386") || spec.contains("W388"))
            && (spec.contains("separate") || spec.contains("分轴") || spec.contains("≠ honesty")),
        "spec must separate implement inventory from honesty axis"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("TLS") || spec.contains("B-04") || spec.contains("HostFn")),
        "spec Frozen must keep TLS / B / HostFn fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let floor = include_str!("../../../../docs/lang/tls-server-ga-exit.md");
    assert!(
        floor.contains("ADR-732")
            && (floor.contains("exit signed") || floor.contains("honesty")),
        "honesty floor exit must remain signed after implement thin"
    );
}

#[test]
fn tls_server_hostfn_thin_product_hostfn_retained() {
    let registry = include_str!("../../../../crates/syx-stdlib/src/registry.rs");
    let hosts = include_str!("../../../../crates/syx-vm/src/hosts.rs");
    // W393 added TlsServe; W435 later raised product status — inventory thin still requires HostFn nails.
    assert!(
        registry.contains("TlsServe") && registry.contains("__syx_tls_serve"),
        "after W393 registry must contain TlsServe / __syx_tls_serve"
    );
    assert!(
        hosts.contains("TlsServe") && hosts.contains("host_tls_serve"),
        "after W393 hosts must implement TLS serve HostFn"
    );
    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        vectors.contains("ADR-736")
            && (vectors.contains("implement inventory") || vectors.contains("HostFn implement")),
        "vectors must retain W392 implement-inventory historical axis"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1600);
    assert!(
        slice.contains("status = \"implemented\"") && slice.contains("ADR-779"),
        "live product status raise is W435/ADR-779 (historical W392 defer remains in vectors): {slice}"
    );
}

