//! Short absolute-threshold first-slice
//! absolute_threshold_count=3; ceilings met; default emit main-only; B-04 still [~].

#[test]
fn b04_absolute_thresholds_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/gates/gate-b04-absolute-thresholds.md",
        "docs/gates/gate-b04-size-ci-gate.md",
        "docs/gates/gate-b04-production-cutover.md",
        "docs/gates/gate-b04-multi-root-wire.md",
        "crates/syx-mir/src/lib.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-04 absolute thresholds require evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("AotShortHorizonAbsoluteThresholdInventory")
            && mir.contains("aot_short_horizon_absolute_threshold_inventory")
            && mir.contains("AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT")
            && mir.contains("ADR-783"),
        "syx-mir must expose absolute-threshold inventory (W439)"
    );
    assert!(
        mir.contains("AOT_ABS_THRESH_ORPHAN_64_PRODUCTION_BYTES")
            && mir.contains("AOT_ABS_THRESH_CALL_INDIRECT_PRODUCTION_BYTES"),
        "syx-mir must publish absolute ceiling constants"
    );
}

#[test]
fn b04_absolute_thresholds_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b04-absolute-thresholds.md");
    assert!(
        spec.contains("ADR-783")
            && (spec.contains("absolute-threshold")
                || spec.contains("absolute threshold")
                || spec.contains("absolute_threshold")),
        "spec must cite ADR-783 absolute thresholds"
    );
    assert!(
        spec.contains("two-hundred-eighty-ninth"),
        
        "spec must cite W439 fingerprint"
    );
    assert!(
        spec.contains("absolute_threshold_count=3")
            || spec.contains("`absolute_threshold_count=3`")
            || spec.contains("count=3"),
        "spec must cite count=3"
    );
    assert!(
        spec.contains("## Frozen") && (spec.contains("multi-root") || spec.contains("B-04")),
        "spec Frozen must fence default multi-root / B-04 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
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
            && !line.contains("exit charter")
        {
            panic!("affirmative B-04 [x] claim forbidden: {line}");
        }
    }
}

#[test]
fn b04_absolute_thresholds_live_inventory() {
    use syx_mir::{
        aot_reachability_fixture_main_and_export_orphan,
        aot_short_horizon_absolute_threshold_inventory, llvm_emit_function_names,
        AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT,
    };
    let module = aot_reachability_fixture_main_and_export_orphan();
    let inv = aot_short_horizon_absolute_threshold_inventory(&module);
    assert!(inv.absolute_threshold_signed);
    assert_eq!(
        inv.absolute_threshold_count,
        AOT_ABSOLUTE_BYTE_THRESHOLD_COUNT
    );
    assert!(inv.relational_absolute_ceiling_ok, "{inv:?}");
    assert!(
        inv.relational_default_main_only_ok,
        "empty-roots path must stay main-only: {inv:?}"
    );
    assert!(
        llvm_emit_function_names(&module).contains("export_lib"),
        "W440: fixture default emit includes export_lib (historical W439 asserted absent)"
    );
}

#[test]
fn b04_absolute_thresholds_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W439")
            && (roadmap.contains("absolute")
                || roadmap.contains("ADR-783")
                || roadmap.contains("阈值")
                || roadmap.contains("threshold")),
        "roadmap must cite W439 Short absolute thresholds"
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
