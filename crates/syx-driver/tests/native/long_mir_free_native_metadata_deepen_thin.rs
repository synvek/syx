//! Long native-metadata-without-blob deepen thin-open inventory (W411 / ADR-755).
//! Public inventory API + policy/meta; gap remains open; ≠ MIR-free / ≠ delete blob.

#[test]
fn long_mir_free_native_metadata_deepen_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-native-metadata-deepen.md",
        "docs/lang/long-mir-free-readiness.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long native-metadata deepen thin requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonNativeMetadataDeepenThinInventory")
            && mir.contains("mir_module_long_horizon_native_metadata_deepen_thin_inventory"),
        "syx-mir must expose MirModuleLongHorizonNativeMetadataDeepenThinInventory (W411)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("native_metadata_deepen_thin_signed")
            && wave_b.contains("native_metadata_without_blob_gap_open"),
        "wave_b_horizon must expose native-metadata deepen flags (W411)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit omit (W413): must use sidecar install; no @syx_mir_blob"
    );
}

#[test]
fn long_mir_free_native_metadata_deepen_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-native-metadata-deepen.md");
    assert!(
        spec.contains("ADR-755")
            && (spec.contains("native-metadata")
                || spec.contains("native metadata")
                || spec.contains("deepen thin")),
        "spec must cite ADR-755 W411 Long native-metadata deepen thin: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonNativeMetadataDeepenThinInventory")
            && spec.contains("mir_module_long_horizon_native_metadata_deepen_thin_inventory"),
        "spec must name public Long native-metadata deepen thin inventory API"
    );
    assert!(
        spec.contains("native_metadata_deepen_thin_signed=true")
            || spec.contains("`native_metadata_deepen_thin_signed=true`"),
        "spec must cite native_metadata_deepen_thin_signed=true"
    );
    assert!(
        spec.contains("native_metadata_without_blob_gap_open=true")
            || spec.contains("`native_metadata_without_blob_gap_open=true`")
            || (spec.contains("still open") && spec.contains("native_metadata")),
        "spec must cite native_metadata_without_blob_gap_open still open"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    assert!(
        spec.contains("long_mir_free_readiness_thin_signed=true")
            || spec.contains("`long_mir_free_readiness_thin_signed=true`")
            || spec.contains("W410"),
        "spec must index W410 readiness thin"
    );
    for line in spec.lines() {
        if line.contains("gap")
            && (line.contains("closed") || line.contains("close"))
            && !line.contains("≠")
            && !line.contains("not")
            && !line.contains("NOT")
            && !line.contains("仍")
            && !line.contains("still")
            && !line.contains("forbidden")
            && !line.contains("Frozen")
            && !line.contains("unmet")
            && !line.contains("open")
        {
            panic!("affirmative gap close forbidden: {line}");
        }
        if line.contains("删除") && line.contains("blob") {
            let denied = line.contains("out of scope")
                || line.contains("≠")
                || line.contains("禁止")
                || line.contains("Forbidden")
                || line.contains("forbidden")
                || line.contains("Frozen")
                || line.contains("非")
                || line.contains("not this window")
                || line.contains("unmet")
                || line.contains("仍");
            assert!(
                denied,
                "affirmative MIR-free / blob delete forbidden: {line}"
            );
        }
    }
}

#[test]
fn long_mir_free_native_metadata_deepen_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W411")
            && (roadmap.contains("native-metadata")
                || roadmap.contains("native metadata")
                || roadmap.contains("ADR-755")
                || roadmap.contains("Long MIR-free")),
        "roadmap must cite W411 Long native-metadata deepen thin"
    );
    for id in ["B-04", "B-05"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} must be [x] after exit charters (ADR-790/ADR-791): {row}"
        );
    }
    let b06 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-06 |"))
        .expect("B-06 row");
    assert!(
        b06.contains("| [x] |"),
        "B-06 must be [x] after W448 exit: {b06}"
    );
    assert!(
        roadmap.contains("Wave A 退出条件已满足")
            || roadmap.contains("Wave A exit met")
            || roadmap.contains("Wave A 退出已满足"),
        "Wave A exit met claim must remain after W411"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}
