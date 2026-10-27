//! Residual TcpListen* HostFn wasm first-slice
//! Locks TcpListen/TcpListenerClose wasm raise; product listener raised in .

#[test]
fn residual_tcplisten_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "crates/syx-driver/tests/stdlib/leak_tcp_listener.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual TcpListen requires evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_tcp.rs");
    assert!(
        nails.contains("fn hostfn_tcp_listen_close_wasm"),
        "qs1_tcp must retain TcpListen wasm nail"
    );
    let leak = include_str!("leak_tcp_listener.rs");
    assert!(
        leak.contains("fn m2_s4_tcp_listen_close_native_llvm"),
        "leak_tcp_listener must retain llvm listen/close nail"
    );
}

#[test]
fn residual_tcplisten_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-718")
            && (spec.contains("TcpListen") || spec.contains("HostFn wasm")),
        "spec must cite ADR-718 TcpListen HostFn first-slice"
    );
    assert!(
        spec.contains("TcpListen")
            && spec.contains("TcpListenerClose")
            && (spec.contains("**met** (W374)") || spec.contains("ADR-718")),
        "spec must index TcpListen* HostFn met"
    );
    assert!(
        spec.contains("net.tcp.listener")
            && (spec.contains("**met** (W375)") || spec.contains("ADR-719"))
            && (spec.contains("4/4") || spec.contains("**4/4")),
        "spec must index product listener raised and product 4/4"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("website") || spec.contains("B-04") || spec.contains("accept")),
        "spec Frozen must keep website / B / accept fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_tcplisten_thin_hostfn_raised() {
    let host = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["TcpListen", "TcpListenerClose"] {
        let i = host
            .find(&format!("name = \"{name}\""))
            .unwrap_or_else(|| panic!("missing {name}"));
        let rest = &host[i..];
        let e = rest.find("\n[[host]]").unwrap_or(rest.len());
        let b = &rest[..e];
        assert!(
            b.contains("wasm = \"real\"")
                && b.contains("status = \"partial\"")
                && b.contains("ADR-718"),
            "{name} must be wasm=real + partial + ADR-718: {b}"
        );
        assert!(
            !b.contains("wasm = \"unsupported\""),
            "{name} must not remain wasm unsupported: {b}"
        );
    }
}

#[test]
fn residual_tcplisten_thin_product_listener_raised() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let i = caps
        .find("id = \"net.tcp.listener\"")
        .expect("net.tcp.listener");
    let rest = &caps[i..];
    let e = rest.find("\n[[capability]]").unwrap_or(rest.len());
    let b = &rest[..e];
    assert!(
        b.contains("status = \"implemented\"")
            && b.contains("native = \"real\"")
            && b.contains("wasm = \"real\"")
            && b.contains("ADR-719"),
        "net.tcp.listener must be product-raised (W375): {b}"
    );
    assert!(
        !b.contains("native = \"partial\"") && !b.contains("wasm = \"unsupported\""),
        "net.tcp.listener must not remain unmet: {b}"
    );
}

#[test]
fn residual_tcplisten_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W374")
            && (roadmap.contains("TcpListen")
                || roadmap.contains("ADR-718")
                || roadmap.contains("HostFn")),
        "roadmap must cite W374 TcpListen HostFn"
    );
    // Honesty: Wave B axes remain on the live table (may all be [x] after exit charters).
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [ ] |") || row.contains("| [~] |") || row.contains("| [x] |"),
            "{id} must remain on the Wave table after residual tcplisten: {row}"
        );
    }
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(
        b08.contains("| [x] |"),
        "B-08 must remain [x]: {b08}"
    );
    // F-02 slim SSOT dropped `- [x] Q-L5` checklist lines; Q-L5 remains as B-08 source tag.
    assert!(
        roadmap.lines().any(|l| l.starts_with("- [x] Q-L5"))
            || (b08.contains("Q-L5") && b08.contains("| [x] |")),
        "Q-L5 must remain checked (checklist or B-08 [x] source tag)"
    );
}
