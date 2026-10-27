//! Residual product net.tcp.listener raise
//! Locks only product listener dual-backend raise; bans website / B `[x]`.

#[test]
fn residual_tcp_listener_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "crates/syx-driver/tests/stdlib/leak_tcp_listener.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual tcp listener requires evidence file: {path}"
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
fn residual_tcp_listener_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-719")
            && (spec.contains("net.tcp.listener") || spec.contains("product")),
        "spec must cite ADR-719 product listener raise"
    );
    assert!(
        spec.contains("**`net.tcp.listener`**")
            && spec.contains("**met** (W375)")
            && (spec.contains("4/4") || spec.contains("**4/4")),
        "spec must index listener met and product 4/4"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("website") || spec.contains("B-04") || spec.contains("accept")),
        "spec Frozen must keep website / B / accept fences"
    );
    assert!(
        (spec.contains("Accept GA inventory") || spec.contains("ADR-726") || spec.contains("W382"))
            && (spec.contains("**met** (W382)") || spec.contains("ADR-726")),
        "spec must index accept GA inventory thin met (W382)"
    );
    assert!(
        spec.contains("TcpListen accept GA") && spec.contains("**unmet**"),
        "spec must keep accept GA unmet after inventory"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_tcp_listener_thin_caps_raised() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let start = caps
        .find("id = \"net.tcp.listener\"")
        .expect("net.tcp.listener");
    let rest = &caps[start..];
    let end = rest.find("\n[[capability]]").unwrap_or(rest.len());
    let block = &rest[..end];
    assert!(
        block.contains("status = \"implemented\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"real\"")
            && block.contains("ADR-719"),
        "net.tcp.listener must be dual-backend real + ADR-719: {block}"
    );
    assert!(
        !block.contains("native = \"partial\"") && !block.contains("wasm = \"unsupported\""),
        "net.tcp.listener must not remain partial/unsupported: {block}"
    );
    assert!(
        block.contains("qs1_tcp_listen_close_wasm")
            || block.contains("m2_s4_tcp_listen_close_native_llvm"),
        "tests must list listen/close evidence: {block}"
    );
}

#[test]
fn residual_tcp_listener_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W375")
            && (roadmap.contains("net.tcp.listener")
                || roadmap.contains("ADR-719")
                || roadmap.contains("listener")),
        "roadmap must cite W375 product listener raise"
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
    assert!(
        b08.contains("| [x] |"),
        "B-08 must remain [x]: {b08}"
    );
    assert!(
        roadmap.contains("Q-L5") && roadmap.contains("已签"),
        "Q-L5 must remain signed"
    );
}
