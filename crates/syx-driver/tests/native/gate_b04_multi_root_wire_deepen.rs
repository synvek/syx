//! Short multi-root production wire deepen
//! Wire API + honesty migration; B-04 remains [~]; ≠ cutover / size-CI / MIR-free.

#[test]
fn b04_multi_root_wire_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b04-multi-root-wire.md",
        "docs/gates/gate-b04-production-cutover-deepen.md",
        "docs/gates/gate-b04-production-reopen.md",
        "docs/gates/gate-b04-mir-tree-shake.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-04 multi-root wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("AotShortHorizonMultiRootWireInventory")
            && mir.contains("aot_short_horizon_multi_root_wire_inventory")
            && mir.contains("reachable_fn_names_from_roots")
            && mir.contains("llvm_emit_function_names_with_roots"),
        "syx-mir must expose multi-root wire APIs (W398)"
    );
}

#[test]
fn b04_multi_root_wire_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b04-multi-root-wire.md");
    assert!(
        spec.contains("ADR-742") && (spec.contains("multi-root") || spec.contains("Multi-root")),
        "spec must cite ADR-742 W398 multi-root wire"
    );
    assert!(
        spec.contains("AotShortHorizonMultiRootWireInventory")
            && spec.contains("llvm_emit_function_names_with_roots"),
        "spec must name wire inventory / emit API"
    );
    assert!(
        spec.contains("multi_root_wire_signed=true")
            || spec.contains("`multi_root_wire_signed=true`")
            || spec.contains("partial met"),
        "spec must cite wire signed / partial met"
    );
    assert!(
        spec.contains("cutover_enabled=false")
            || spec.contains("`cutover_enabled=false`")
            || (spec.contains("cutover") && spec.contains("unmet")),
        "spec must cite cutover still unmet"
    );
    assert!(
        spec.contains("main-only") || spec.contains("empty extras"),
        "spec must cite default main-only path"
    );
    for line in spec.lines() {
        if line.contains("MIR-free") || (line.contains("删除") && line.contains("blob")) {
            let denied = line.contains("out of scope")
                || line.contains("≠")
                || line.contains("禁止")
                || line.contains("Frozen")
                || line.contains("非")
                || line.contains("not this window");
            assert!(denied, "affirmative MIR-free forbidden: {line}");
        }
    }
}

