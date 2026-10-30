//! Residual deque first-slice
//! Locks collections.deque dual-backend raise; listener raised (row+pool raised).

#[test]
fn residual_deque_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "crates/syx-driver/tests/stdlib/edge_deinit_nil.rs",
        "crates/syx-driver/tests/stdlib/deque.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual deque requires evidence file: {path}"
        );
    }
    let nails = include_str!("edge_deinit_nil.rs");
    assert!(
        nails.contains("fn edge_deque_empty_pop_nil_native")
            && nails.contains("fn edge_deque_empty_pop_nil_wasm"),
        "a04 must retain deque dual-backend nails"
    );
}

#[test]
fn residual_deque_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/residual-product-vectors.md");
    assert!(
        spec.contains("ADR-712")
            && (spec.contains("deque first-slice") || spec.contains("first-slice signed")),
        "spec must cite ADR-712 deque first-slice"
    );
    assert!(
        spec.contains("collections.deque")
            && spec.contains("**met**")
            && spec.contains("net.tcp.listener")
            && (spec.contains("**met** (W375)") || spec.contains("ADR-719"))
            && (spec.contains("4/4") || spec.contains("**4/4")),
        "spec must index deque met and listener raised (4/4)"
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
fn residual_deque_thin_caps_raised() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    // Isolate collections.deque block between [[capability]] markers.
    let start = caps
        .find("id = \"collections.deque\"")
        .expect("collections.deque id");
    let rest = &caps[start..];
    let end = rest.find("\n[[capability]]").unwrap_or(rest.len());
    let block = &rest[..end];
    assert!(
        block.contains("status = \"implemented\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"real\"")
            && block.contains("ADR-712"),
        "collections.deque must be dual-backend real + ADR-712: {block}"
    );
    assert!(
        !block.contains("native = \"partial\"") && !block.contains("wasm = \"unsupported\""),
        "collections.deque must not remain partial/unsupported: {block}"
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

