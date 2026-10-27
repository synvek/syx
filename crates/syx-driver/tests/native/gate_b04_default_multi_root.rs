//! Short default emit multi-root first-slice
//! aot_export_roots drives default emit; empty roots main-only; B-04 still [~].

#[test]
fn b04_default_multi_root_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/gates/gate-b04-default-multi-root.md",
        "docs/gates/gate-b04-absolute-thresholds.md",
        "docs/gates/gate-b04-multi-root-wire.md",
        "crates/syx-mir/src/lib.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-04 default multi-root requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("AotShortHorizonDefaultMultiRootInventory")
            && mir.contains("aot_short_horizon_default_multi_root_inventory")
            && mir.contains("aot_export_roots")
            && mir.contains("ADR-784"),
        "syx-mir must expose default multi-root inventory (W440)"
    );
}

#[test]
fn b04_default_multi_root_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b04-default-multi-root.md");
    assert!(
        spec.contains("ADR-784")
            && (spec.contains("default emit multi-root")
                || spec.contains("aot_export_roots")
                || spec.contains("default multi-root")),
        "spec must cite ADR-784 default multi-root"
    );
    assert!(
        spec.contains("two-hundred-ninetieth"),
        
        "spec must cite W440 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("Module.exports")
                || spec.contains("exports")
                || spec.contains("B-04")),
        "spec Frozen must fence language exports / B-04 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    // Historical W440 doc may still fence affirmative B-04 [x] without exit/ADR-790 context.
    for line in spec.lines() {
        if line.contains("B-04")
            && line.contains("[x]")
            && !line.contains("≠")
            && !line.contains("仍")
            && !line.contains("Frozen")
            && !line.contains("still")
            && !line.contains("Forbidden")
            && !line.contains("Claiming")
            && !line.contains("unmet")
            && !line.contains("exit")
            && !line.contains("ADR-790")
            && !line.contains("已交")
        {
            panic!("affirmative B-04 [x] without exit context forbidden: {line}");
        }
    }
}

#[test]
fn b04_default_multi_root_live_inventory() {
    use syx_mir::{
        aot_reachability_fixture_main_and_export_orphan,
        aot_short_horizon_default_multi_root_inventory, compact_module_for_aot_blob,
        llvm_emit_function_names, AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
    };
    let module = aot_reachability_fixture_main_and_export_orphan();
    let inv = aot_short_horizon_default_multi_root_inventory(&module);
    assert!(inv.default_multi_root_signed);
    assert!(inv.production_multi_root_enabled);
    assert_eq!(
        inv.absolute_threshold_count,
        AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT
    );
    assert!(inv.relational_empty_roots_main_only_ok, "{inv:?}");
    assert!(
        inv.relational_fixture_default_emit_includes_export_ok,
        "{inv:?}"
    );
    assert!(inv.relational_with_roots_api_still_aligns_ok, "{inv:?}");
    let emit = llvm_emit_function_names(&module);
    assert!(emit.contains("export_lib") && emit.contains("callee_b"));
    let compact = compact_module_for_aot_blob(&module);
    assert!(compact.functions.iter().any(|f| f.name == "export_lib"));
}

#[test]
fn b04_default_multi_root_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W440")
            && (roadmap.contains("multi-root")
                || roadmap.contains("ADR-784")
                || roadmap.contains("多根")
                || roadmap.contains("aot_export_roots")),
        "roadmap must cite W440 Short default multi-root"
    );
    let b05 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-05 |"))
        .expect("B-05 row");
    assert!(
        b05.contains("| [x] |"),
        "B-05 must be [x] after W447 exit: {b05}"
    );
    let b06 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-06 |"))
        .expect("B-06 row");
    assert!(
        b06.contains("| [x] |"),
        "B-06 must be [x] after W448 exit: {b06}"
    );
    let b04 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-04 |"))
        .expect("B-04 row");
    assert!(
        b04.contains("| [x] |"),
        "B-04 must be [x] after W446 exit: {b04}"
    );
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(b08.contains("| [x] |"), "B-08 must remain [x]: {b08}");
}
