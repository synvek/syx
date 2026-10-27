//! TcpListen accept GA met exit charter
//! Policy + meta flip only — scope narrow; TLS remains unmet.

#[test]
fn tcp_accept_ga_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/tcp-accept-ga-vectors.md",
        "crates/syx-driver/tests/stdlib/tcp_accept_ga_thin.rs",
        "crates/syx-driver/tests/stdlib/tcp_accept_ga_deepen.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "packages/internal/host_fn_capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "accept exit requires evidence file: {path}"
        );
    }
    let qs1 = include_str!("hostfn_tcp.rs");
    assert!(
        qs1.contains("fn hostfn_tcp_listener_accept_native_llvm")
            && qs1.contains("fn hostfn_tcp_listener_accept_wasm"),
        "exit evidence must retain W383/W384 accept nails"
    );
}

#[test]
fn tcp_accept_ga_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tcp-accept-ga-exit.md");
    assert!(
        spec.contains("ADR-729")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-729 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("Accept scope narrow"),
        "spec must cite accept scope narrow"
    );
    assert!(
        spec.contains("W382")
            && spec.contains("W383")
            && spec.contains("W384")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + llvm partial-sufficient + wasm prerequisites"
    );
    assert!(
        (spec.contains("TLS") || spec.contains("tls.server"))
            && (spec.contains("unmet")
                || spec.contains("defer")
                || spec.contains("unsupported")
                || spec.contains("met (narrow)")
                || spec.contains("ADR-739")),
        "spec must retain TLS product defer / GA met-narrow cross-index"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("TLS") || spec.contains("status")),
        "spec Frozen must keep TLS / B / status fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let vectors = include_str!("../../../../docs/lang/tcp-accept-ga-vectors.md");
    assert!(
        vectors.contains("TcpListen accept GA")
            && (vectors.contains("**met** (W385") || vectors.contains("met (narrow)") || vectors.contains("ADR-729")),
        "vectors must mark accept GA met (narrow)"
    );
    assert!(
        (vectors.contains("TLS") || vectors.contains("tls.server"))
            && (vectors.contains("**unmet**")
                || vectors.contains("unsupported")
                || vectors.contains("met (narrow)")),
        "vectors must retain TLS product unmet or GA met-narrow cross-index"
    );
}

#[test]
fn tcp_accept_ga_exit_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W385")
            && (roadmap.contains("accept")
                || roadmap.contains("ADR-729")
                || roadmap.contains("exit")),
        "roadmap must cite W385 accept GA met exit"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(a10.contains("| [x] |"), "A-10 must remain [x]: {a10}");
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}");
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
