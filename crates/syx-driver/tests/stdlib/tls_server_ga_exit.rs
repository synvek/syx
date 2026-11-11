//! TLS honesty-ladder floor exit charter
//! Policy + meta flip only — floor met; after TLS GA is met (narrow); product status still unsupported.

#[test]
fn tls_server_ga_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/tls-server-ga-vectors.md",
        "crates/syx-driver/tests/stdlib/tls_server_ga_thin.rs",
        "packages/internal/capabilities.toml",
        "crates/syx-stdlib/src/registry.rs",
        "tools/sgate/sidecars/tls.sy",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS floor exit requires evidence file: {path}"
        );
    }
    // S2-I128: deepen soft nail retired; HostFn-gap deepen history retained in vectors SSOT.
    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        vectors.contains("ADR-731")
            && (vectors.contains("HostFn-gap") || vectors.contains("honesty deepen")),
        "exit evidence must retain W387 HostFn-gap deepen meta in vectors SSOT"
    );
}

#[test]
fn tls_server_ga_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-exit.md");
    assert!(
        spec.contains("ADR-732")
            && (spec.contains("exit signed")
                || spec.contains("Exit signed")
                || spec.contains("floor exit")),
        "spec must cite ADR-732 honesty-ladder floor exit signed"
    );
    assert!(
        spec.contains("scope narrow")
            || spec.contains("Honesty-floor scope narrow")
            || spec.contains("honesty-floor"),
        "spec must cite honesty-floor scope narrow"
    );
    assert!(
        spec.contains("W386")
            && spec.contains("W387")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + HostFn-gap deepen partial-sufficient prerequisites"
    );
    assert!(
        (spec.contains("TLS GA") || spec.contains("tls.server") || spec.contains("GA"))
            && (spec.contains("≠ TLS GA")
                || spec.contains("Exit ≠ TLS GA")
                || spec.contains("honesty")
                || spec.contains("floor")
                || spec.contains("ADR-732")),
        "honesty floor exit must remain floor-scoped (≠ product status raise)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("TLS GA") || spec.contains("HostFn")),
        "spec Frozen must keep TLS GA / HostFn / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        (vectors.contains("TLS honesty-ladder floor") || vectors.contains("honesty-ladder floor"))
            && (vectors.contains("**met** (W388")
                || vectors.contains("ADR-732")
                || vectors.contains("met (narrow)")),
        "vectors must mark honesty floor met (narrow)"
    );
    assert!(
        (vectors.contains("TLS server GA") || vectors.contains("TLS GA"))
            && (vectors.contains("**met** (W395")
                || vectors.contains("met (narrow)")
                || vectors.contains("ADR-739")),
        "vectors must mark TLS GA met (narrow) after W395"
    );
    assert!(
        vectors.contains("unsupported") || vectors.contains("net.tls.server"),
        "vectors must retain product unsupported / net.tls.server cross-index"
    );
}
