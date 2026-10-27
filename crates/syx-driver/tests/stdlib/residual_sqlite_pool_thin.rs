//! Residual product data.sqlite.pool raise
//! Locks only pool dual-backend raise; listener raised .

#[test]
fn residual_sqlite_pool_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/hostfn_sql.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual sqlite pool requires evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_sql.rs");
    assert!(
        nails.contains("fn hostfn_sql_pool_lifecycle_native_llvm")
            && nails.contains("fn hostfn_sql_pool_lifecycle_wasm"),
        "qs1_sql must retain pool dual-backend lifecycle nails"
    );
}

#[test]
fn residual_sqlite_pool_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-717")
            && (spec.contains("data.sqlite.pool") || spec.contains("product")),
        "spec must cite ADR-717 product pool raise"
    );
    assert!(
        spec.contains("**`data.sqlite.pool`**")
            && spec.contains("**met**")
            && spec.contains("net.tcp.listener")
            && (spec.contains("**met** (W375)") || spec.contains("ADR-719"))
            && (spec.contains("4/4") || spec.contains("**4/4")),
        "spec must index pool met and listener raised (4/4)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("four-row")
                || spec.contains("listener")
                || spec.contains("TcpListen")
                || spec.contains("website")
                || spec.contains("B-04")),
        "spec Frozen must keep four-row / listener / website / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_sqlite_pool_thin_caps_raised() {
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
        "data.sqlite.pool must be dual-backend real + ADR-717: {block}"
    );
    assert!(
        !block.contains("native = \"partial\"") && !block.contains("wasm = \"unsupported\""),
        "data.sqlite.pool must not remain partial/unsupported: {block}"
    );
    assert!(
        block.contains("qs1_sql_pool_lifecycle_wasm")
            || block.contains("qs1_sql_pool_lifecycle_native"),
        "tests must list qs1 pool lifecycle evidence: {block}"
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
fn residual_sqlite_pool_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W373")
            && (roadmap.contains("data.sqlite.pool")
                || roadmap.contains("ADR-717")
                || roadmap.contains("sqlite.pool")
                || roadmap.contains("product pool")),
        "roadmap must cite W373 product pool raise"
    );
    // Honesty: Wave B axes remain on the live table (may all be [x] after exit charters).
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [ ] |") || row.contains("| [~] |") || row.contains("| [x] |"),
            "{id} must remain on the Wave table after residual sqlite pool: {row}"
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
