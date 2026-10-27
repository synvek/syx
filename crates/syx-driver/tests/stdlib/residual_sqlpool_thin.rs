//! Residual SqlPool* HostFn wasm first-slice
//! Locks Create/Acquire/Release/Close wasm raise; product pool raised ; listener raised .

#[test]
fn residual_sqlpool_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/host_fn_capabilities.toml",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/hostfn_sql.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual SqlPool requires evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_sql.rs");
    assert!(
        nails.contains("fn hostfn_sql_pool_lifecycle_native_llvm")
            && nails.contains("fn hostfn_sql_pool_lifecycle_wasm"),
        "qs1_sql must retain SqlPool dual-backend lifecycle nails"
    );
}

#[test]
fn residual_sqlpool_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-716")
            && (spec.contains("SqlPool") || spec.contains("HostFn wasm")),
        "spec must cite ADR-716 SqlPool HostFn first-slice"
    );
    for name in [
        "SqlPoolCreate",
        "SqlPoolAcquire",
        "SqlPoolRelease",
        "SqlPoolClose",
    ] {
        assert!(
            spec.contains(name),
            "spec must index {name} HostFn progress"
        );
    }
    assert!(
        spec.contains("data.sqlite.pool")
            && (spec.contains("**met** (W373)") || spec.contains("ADR-717"))
            && (spec.contains("**met** (W372)") || spec.contains("ADR-716")),
        "spec must index SqlPool HostFn met and product pool met after W373"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("product")
                || spec.contains("four-row")
                || spec.contains("TcpListen")
                || spec.contains("B-04")),
        "spec Frozen must keep product-listener / TcpListen / website / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_sqlpool_thin_hostfn_raised() {
    let host = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "SqlPoolCreate",
        "SqlPoolAcquire",
        "SqlPoolRelease",
        "SqlPoolClose",
    ] {
        let i = host
            .find(&format!("name = \"{name}\""))
            .unwrap_or_else(|| panic!("missing {name}"));
        let rest = &host[i..];
        let e = rest.find("\n[[host]]").unwrap_or(rest.len());
        let b = &rest[..e];
        assert!(
            b.contains("wasm = \"real\"")
                && b.contains("status = \"partial\"")
                && b.contains("ADR-716"),
            "{name} must be wasm=real + partial + ADR-716: {b}"
        );
        assert!(
            !b.contains("wasm = \"unsupported\""),
            "{name} must not remain wasm unsupported: {b}"
        );
    }
}

#[test]
fn residual_sqlpool_thin_product_pool_raised() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let start = caps
        .find("id = \"data.sqlite.pool\"")
        .expect("data.sqlite.pool");
    let rest = &caps[start..];
    let end = rest.find("\n[[capability]]").unwrap_or(rest.len());
    let block = &rest[..end];
    assert!(
        block.contains("status = \"implemented\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"real\"")
            && block.contains("ADR-717"),
        "product data.sqlite.pool must be raised with ADR-717: {block}"
    );
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
}

#[test]
fn residual_sqlpool_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W372")
            && (roadmap.contains("SqlPool")
                || roadmap.contains("ADR-716")
                || roadmap.contains("HostFn")),
        "roadmap must cite W372 SqlPool HostFn"
    );
    // Honesty: Wave B axes remain on the live table (may all be [x] after exit charters).
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [ ] |") || row.contains("| [~] |") || row.contains("| [x] |"),
            "{id} must remain on the Wave table after residual sqlpool: {row}"
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
