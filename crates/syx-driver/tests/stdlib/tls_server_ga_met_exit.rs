//! TLS GA met exit charter
//! Policy + meta flip only — scope narrow; product status remains unsupported.


fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}
#[test]
fn tls_server_ga_met_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-met-exit.md",
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "crates/syx-driver/tests/stdlib/tls_server_hostfn_thin.rs",
        "crates/syx-driver/tests/stdlib/tls_server_hostfn_deepen.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tls.rs",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS GA met exit requires evidence file: {path}"
        );
    }
    let qs1 = include_str!("hostfn_tls.rs");
    assert!(
        qs1.contains("fn hostfn_tls_serve_native_llvm")
            && qs1.contains("fn hostfn_tls_serve_wasm"),
        "exit evidence must retain W393/W394 TLS HostFn nails"
    );
    let floor = include_str!("../../../../docs/lang/tls-server-ga-exit.md");
    assert!(
        floor.contains("ADR-732")
            && (floor.contains("honesty") || floor.contains("floor")),
        "honesty floor exit must remain present (分轴 from GA met exit)"
    );
}

#[test]
fn tls_server_ga_met_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-met-exit.md");
    assert!(
        spec.contains("ADR-739")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-739 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("TLS GA scope narrow"),
        "spec must cite TLS GA scope narrow"
    );
    assert!(
        spec.contains("W392")
            && spec.contains("W393")
            && spec.contains("W394")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + llvm partial-sufficient + wasm prerequisites"
    );
    assert!(
        (spec.contains("status=implemented") || spec.contains("unsupported") || spec.contains("product"))
            && (spec.contains("defer") || spec.contains("≠") || spec.contains("Not")),
        "spec must defer product status=implemented raise"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("status") || spec.contains("handler")),
        "spec Frozen must keep B / status / handler fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        (vectors.contains("TLS server GA") || vectors.contains("TLS GA"))
            && (vectors.contains("**met** (W395")
                || vectors.contains("met (narrow)")
                || vectors.contains("ADR-739")),
        "vectors must mark TLS GA met (narrow)"
    );
    // W395 charter deferred product status raise; live raise is W435/ADR-779.
    assert!(
        spec.contains("product defer")
            || spec.contains("stays **unsupported**")
            || (spec.contains("unsupported") && spec.contains("defer")),
        "GA met-exit charter must retain historical product-status defer honesty"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let idx = caps
        .find("id = \"net.tls.server\"")
        .expect("net.tls.server row");
    let slice = utf8_prefix_slice(caps, idx, 1100);
    assert!(
        slice.contains("status = \"implemented\"") && slice.contains("ADR-779"),
        "live net.tls.server is W435 status raise (ADR-779); historical W395 defer remains in charter: {slice}"
    );
}

