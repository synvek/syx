//! TcpListen accept HostFn llvm deepen + wasm first-slice (/ · /).
//! Locks TcpListenerAccept llvm+wasm nails; after accept GA is met (narrow); bans TLS / B `[x]`.

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

#[test]
fn tcp_accept_ga_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/tcp-accept-ga-vectors.md",
        "docs/lang/tcp-accept-ga-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/net/tcp.sy",
        "crates/syx-vm/src/hosts.rs",
        "crates/syx-stdlib/src/registry.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "accept deepen requires evidence file: {path}"
        );
    }
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("host_tcp_listener_accept")
            && hosts.contains("TcpListenerAccept")
            && hosts.contains("ADR-727"),
        "hosts.rs must implement TcpListenerAccept (ADR-727)"
    );
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        registry.contains("TcpListenerAccept")
            && registry.contains("__syx_tcp_listener_accept"),
        "registry must append TcpListenerAccept HostFn"
    );
    let facade = include_str!("../../../../packages/net/tcp.sy");
    assert!(
        facade.contains("func accept") && facade.contains("__syx_tcp_listener_accept"),
        "tcp.sy must expose accept facade"
    );
    let qs1 = include_str!("hostfn_tcp.rs");
    assert!(
        qs1.contains("fn hostfn_tcp_listener_accept_native_llvm")
            && qs1.contains("fn hostfn_tcp_listener_accept_wasm"),
        "qs1_tcp must retain accept llvm + wasm nails"
    );
}

#[test]
fn tcp_accept_ga_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/tcp-accept-ga-vectors.md");
    assert!(
        spec.contains("ADR-728")
            && (spec.contains("wasm first-slice")
                || spec.contains("wasm=real")
                || spec.contains("W384")),
        "spec must cite ADR-728 accept wasm first-slice"
    );
    assert!(
        (spec.contains("Accept HostFn wasm") || spec.contains("wasm first-slice"))
            && (spec.contains("**met** (W384)") || spec.contains("ADR-728")),
        "spec must index accept HostFn wasm first-slice met"
    );
    assert!(
        (spec.contains("Accept HostFn llvm") || spec.contains("llvm first-slice"))
            && (spec.contains("**partial met**")
                || spec.contains("partial met")
                || spec.contains("ADR-727")),
        "spec must retain accept HostFn llvm first-slice"
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
        "spec must keep TLS unmet"
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
}

#[test]
fn tcp_accept_ga_deepen_caps_honesty() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "TcpListenerAccept");
    assert!(
        block.contains("llvm = \"real\"")
            && block.contains("wasm = \"real\"")
            && block.contains("status = \"partial\"")
            && (block.contains("ADR-728") || block.contains("ADR-729")),
        "TcpListenerAccept must be llvm+wasm=real / partial + ADR-728/729: {block}"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tcp.listener\"")
            && (caps.contains("ADR-729") || caps.contains("ADR-728") || caps.contains("ADR-727"))
            && caps.contains("status = \"implemented\""),
        "product listener must cross-index ADR-729 without status rollback"
    );
}

#[test]
fn tcp_accept_ga_deepen_roadmap_honesty() {
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
