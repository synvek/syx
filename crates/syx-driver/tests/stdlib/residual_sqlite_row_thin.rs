//! Residual product data.sqlite.row raise
//! Locks only row dual-backend raise; pool raised ; listener raised .

#[test]
fn residual_sqlite_row_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/hostfn_sql.rs",
        "crates/syx-driver/tests/stdlib/sql_typed_row.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual sqlite row requires evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_sql.rs");
    assert!(
        nails.contains("fn hostfn_sql_row_accessors_native_llvm")
            && nails.contains("fn hostfn_sql_row_accessors_wasm")
            && nails.contains("fn hostfn_sql_prepare_bind_row_get_int_wasm"),
        "qs1_sql must retain row dual-backend nails"
    );
}

#[test]
fn residual_sqlite_row_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-715")
            && (spec.contains("data.sqlite.row") || spec.contains("product")),
        "spec must cite ADR-715 product row raise"
    );
    assert!(
        spec.contains("**`data.sqlite.row`**")
            && spec.contains("**met**")
            && spec.contains("net.tcp.listener")
            && (spec.contains("**met** (W375)") || spec.contains("ADR-719"))
            && (spec.contains("4/4") || spec.contains("**4/4")),
        "spec must index row met and listener raised (4/4)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("four-row")
                || spec.contains("listener")
                || spec.contains("website")
                || spec.contains("B-04")),
        "spec Frozen must keep four-row / website / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_sqlite_row_thin_caps_raised() {
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
        "data.sqlite.row must be dual-backend real + ADR-715: {block}"
    );
    assert!(
        !block.contains("native = \"partial\"") && !block.contains("wasm = \"unsupported\""),
        "data.sqlite.row must not remain partial/unsupported: {block}"
    );
    assert!(
        block.contains("qs1_sql_row_accessors_wasm")
            || block.contains("qs1_sql_row_accessors_native"),
        "tests must list qs1 accessors evidence: {block}"
    );
    for id in ["id = \"net.tcp.listener\""] {
        let i = caps.find(id).unwrap_or_else(|| panic!("missing {id}"));
        let rest = &caps[i..];
        let e = rest.find("\n[[capability]]").unwrap_or(rest.len());
        let b = &rest[..e];
        assert!(
            b.contains("status = \"implemented\"")
                && b.contains("native = \"real\"")
                && b.contains("wasm = \"real\"")
                && b.contains("ADR-719"),
            "{id} must be product-raised (W375): {b}"
        );
    }
}

