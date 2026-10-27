//! TLS server thin-open inventory
//! Locks inventory signed; after honesty floor is met; bans TLS GA / HostFn / B `[x]`.

#[test]
fn tls_server_ga_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tls-server-ga-vectors.md",
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/residual-product-vectors.md",
        "plan/archive/gates/gate-a06-qs2-exit.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/tests/stdlib/tls_server_ga_deepen.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "TLS thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tls.server\"")
            && (caps.contains("ADR-779")
                || caps.contains("ADR-730")
                || caps.contains("ADR-731")
                || caps.contains("ADR-732")
                || caps.contains("ADR-237")),
        "capabilities.toml must retain net.tls.server + inventory/exit/status-raise cross-index"
    );
    assert!(
        caps.contains("id = \"net.tls.client\"") && caps.contains("status = \"implemented\""),
        "capabilities.toml must retain net.tls.client implemented"
    );
    let vectors = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        vectors.contains("ADR-730")
            && (vectors.contains("thin-open") || vectors.contains("inventory thin")),
        "vectors must retain W386 thin-open historical axis (status raise is later W435)"
    );
}

#[test]
fn tls_server_ga_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tls-server-ga-vectors.md");
    assert!(
        spec.contains("ADR-730")
            && (spec.contains("thin-open") || spec.contains("inventory thin")),
        "spec must cite ADR-730 TLS server thin-open inventory"
    );
    assert!(
        (spec.contains("TLS server inventory thin") || spec.contains("inventory thin"))
            && (spec.contains("**met** (W386)") || spec.contains("ADR-730")),
        "spec must index TLS inventory thin met"
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
        (spec.contains("net.tls.client") || spec.contains("`net.tls.client`"))
            && (spec.contains("cross-met") || spec.contains("**cross-met**")),
        "spec must index tls.client as cross-met"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("TLS") || spec.contains("B-04") || spec.contains("HostFn")),
        "spec Frozen must keep TLS / B / HostFn fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let accept = include_str!("../../../../docs/lang/tcp-accept-ga-exit.md");
    assert!(
        accept.contains("ADR-729")
            && (accept.contains("met") || accept.contains("exit signed")),
        "accept exit must remain signed after TLS floor exit"
    );
}

#[test]
fn tls_server_ga_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W386")
            && (roadmap.contains("TLS")
                || roadmap.contains("ADR-730")
                || roadmap.contains("tls")),
        "roadmap must cite W386 TLS server thin inventory"
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
