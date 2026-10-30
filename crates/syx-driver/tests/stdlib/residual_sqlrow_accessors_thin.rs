//! Residual SqlRow* HostFn wasm batch-5
//! Locks GetString/Float/Blob/IsNull/ColumnNames wasm raise; product row raised .

#[test]
fn residual_sqlrow_accessors_thin_evidence_files_exist() {
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
            "residual SqlRow accessors require evidence file: {path}"
        );
    }
    let nails = include_str!("hostfn_sql.rs");
    assert!(
        nails.contains("fn hostfn_sql_row_accessors_native_llvm")
            && nails.contains("fn hostfn_sql_row_accessors_wasm"),
        "qs1_sql must retain accessors native+wasm nails"
    );
}

#[test]
fn residual_sqlrow_accessors_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-714")
            && (spec.contains("batch-5") || spec.contains("HostFn wasm batch")),
        "spec must cite ADR-714 SqlRow HostFn batch-5"
    );
    for name in [
        "SqlRowGetString",
        "SqlRowGetFloat",
        "SqlRowGetBlob",
        "SqlRowIsNull",
        "SqlRowColumnNames",
    ] {
        assert!(
            spec.contains(name),
            "spec must index {name} HostFn progress"
        );
    }
    assert!(
        spec.contains("data.sqlite.row")
            && (spec.contains("**met** (W371)") || spec.contains("ADR-715")),
        "spec must index product row met after W371"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("product") || spec.contains("four-row") || spec.contains("B-04")),
        "spec Frozen must keep product-row / website / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn residual_sqlrow_accessors_thin_hostfn_raised() {
    let host = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "SqlRowGetString",
        "SqlRowGetFloat",
        "SqlRowGetBlob",
        "SqlRowIsNull",
        "SqlRowColumnNames",
        "SqlRowGetInt",
    ] {
        let i = host
            .find(&format!("name = \"{name}\""))
            .unwrap_or_else(|| panic!("missing {name}"));
        let rest = &host[i..];
        let e = rest.find("\n[[host]]").unwrap_or(rest.len());
        let b = &rest[..e];
        assert!(
            b.contains("wasm = \"real\"") && b.contains("status = \"partial\""),
            "{name} must be wasm=real + partial: {b}"
        );
        if name != "SqlRowGetInt" {
            assert!(
                b.contains("ADR-714"),
                "{name} notes must cite ADR-714: {b}"
            );
        }
    }
}

#[test]
fn residual_sqlrow_accessors_thin_product_row_raised() {
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

