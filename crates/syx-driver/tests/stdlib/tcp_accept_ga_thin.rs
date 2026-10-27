//! TcpListen accept GA thin-open inventory
//! Locks inventory signed; after accept GA is met (narrow); bans TLS claim / B `[x]`.

#[test]
fn tcp_accept_ga_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tcp-accept-ga-vectors.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/residual-product-vectors.md",
        "docs/lang/hosted-site-exit.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "crates/syx-driver/tests/stdlib/leak_tcp_listener.rs",
        "crates/syx-vm/src/hosts.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "accept GA thin requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("host_tcp_listener_accept")
            || hosts.contains("TcpListenerAccept")
            || hosts.contains("accept GA frozen")
            || hosts.contains("Does **not** accept"),
        "hosts.rs must retain accept path honesty (W383 deepen or prior frozen)"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tcp.listener\"")
            && (caps.contains("ADR-726")
                || caps.contains("ADR-727")
                || caps.contains("ADR-728")
                || caps.contains("ADR-729")),
        "capabilities.toml must retain listener + accept inventory/exit cross-index"
    );
}

#[test]
fn tcp_accept_ga_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tcp-accept-ga-vectors.md");
    assert!(
        spec.contains("ADR-726")
            && (spec.contains("thin-open") || spec.contains("inventory signed")),
        "spec must cite ADR-726 accept GA thin-open inventory"
    );
    assert!(
        (spec.contains("Accept GA inventory thin") || spec.contains("inventory thin"))
            && (spec.contains("**met** (W382)") || spec.contains("ADR-726")),
        "spec must index accept inventory met"
    );
    assert!(
        spec.contains("TcpListen accept GA")
            && (spec.contains("**met** (W385")
                || spec.contains("met (narrow)")
                || spec.contains("ADR-729")),
        "spec must mark accept GA met (narrow) after W385"
    );
    assert!(
        (spec.contains("TLS") || spec.contains("tls.server")) && spec.contains("**unmet**"),
        "spec must index TLS as unmet/deferred"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("TLS") || spec.contains("B-04") || spec.contains("status")),
        "spec Frozen must keep TLS / B / status fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let residual = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        residual.contains("accept")
            && (residual.contains("**met** (W385")
                || residual.contains("met (narrow)")
                || residual.contains("ADR-729")),
        "residual must mark accept GA met (narrow) after W385"
    );
    assert!(
        residual.contains("ADR-726")
            || residual.contains("accept GA inventory")
            || residual.contains("W382")
            || residual.contains("ADR-729"),
        "residual must cross-index accept GA inventory/exit"
    );
}

#[test]
fn tcp_accept_ga_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W382")
            && (roadmap.contains("accept")
                || roadmap.contains("ADR-726")
                || roadmap.contains("inventory")),
        "roadmap must cite W382 accept GA thin inventory"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a10.contains("| [x] |"),
        "A-10 must remain [x]: {a10}"
    );
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}"
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
