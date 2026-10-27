//! Residual SqlRowGetInt HostFn wasm first-slice
//! Locks only GetInt HostFn wasm raise; product row raised ; pool/listener raised (/).

#[test]
fn residual_sqlrow_getint_thin_evidence_files_exist() {
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
            "residual SqlRowGetInt requires evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_sql.rs");
    assert!(
        nails.contains("fn hostfn_sql_prepare_bind_row_get_int_wasm")
            && nails.contains("fn hostfn_sql_prepare_bind_row_get_int_native_llvm"),
        "qs1_sql must retain GetInt dual-backend nails"
    );
}

#[test]
fn residual_sqlrow_getint_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-713")
            && (spec.contains("SqlRowGetInt") || spec.contains("HostFn wasm")),
        "spec must cite ADR-713 SqlRowGetInt HostFn first-slice"
    );
    assert!(
        spec.contains("SqlRowGetInt")
            && spec.contains("**met**")
            && (spec.contains("ADR-715") || spec.contains("data.sqlite.row")),
        "spec must index GetInt HostFn met and acknowledge product row path"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("four-row")
                || spec.contains("product")
                || spec.contains("website")
                || spec.contains("B-04")),
        "spec Frozen must keep product-row / website / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_sqlrow_getint_thin_hostfn_raised() {
    let host = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let start = host
        .find("name = \"SqlRowGetInt\"")
        .expect("SqlRowGetInt");
    let rest = &host[start..];
    let end = rest.find("\n[[host]]").unwrap_or(rest.len());
    let block = &rest[..end];
    assert!(
        block.contains("wasm = \"real\"")
            && block.contains("status = \"partial\"")
            && (block.contains("ADR-713") || block.contains("ADR-714")),
        "SqlRowGetInt must be wasm=real + partial + ADR-713: {block}"
    );
    assert!(
        !block.contains("wasm = \"unsupported\""),
        "SqlRowGetInt must not remain wasm unsupported: {block}"
    );
}

#[test]
fn residual_sqlrow_getint_thin_product_row_raised() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let start = caps
        .find("id = \"data.sqlite.row\"")
        .expect("data.sqlite.row");
    let rest = &caps[start..];
    let end = rest.find("\n[[capability]]").unwrap_or(rest.len());
    let block = &rest[..end];
    assert!(
        block.contains("status = \"implemented\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"real\"")
            && block.contains("ADR-715"),
        "product data.sqlite.row must be raised with ADR-715: {block}"
    );
}

#[test]
fn residual_sqlrow_getint_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W369")
            && (roadmap.contains("SqlRowGetInt")
                || roadmap.contains("ADR-713")
                || roadmap.contains("HostFn")),
        "roadmap must cite W369 SqlRowGetInt HostFn"
    );
    // Honesty: Wave B axes remain on the live table (may all be [x] after exit charters).
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [ ] |") || row.contains("| [~] |") || row.contains("| [x] |"),
            "{id} must remain on the Wave table after residual sqlrow getint: {row}"
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
