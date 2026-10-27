//! Long MIR-free readiness thin-open inventory (W410 / ADR-754).
//! Public inventory API + policy/meta; MIR-free remains off; ≠ delete `@syx_mir_blob`.

#[test]
fn long_mir_free_readiness_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-readiness.md",
        "docs/gates/gate-b05-execution-omit.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long MIR-free readiness thin requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonMirFreeReadinessThinInventory")
            && mir.contains("mir_module_long_horizon_mir_free_readiness_thin_inventory"),
        "syx-mir must expose MirModuleLongHorizonMirFreeReadinessThinInventory (W410)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("long_mir_free_readiness_thin_signed")
            && wave_b.contains("mir_free_enabled"),
        "wave_b_horizon must expose Long MIR-free readiness flags (W410)"
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
fn long_mir_free_readiness_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-readiness.md");
    assert!(
        spec.contains("ADR-754")
            && (spec.contains("MIR-free readiness")
                || spec.contains("Long MIR-free")
                || spec.contains("readiness thin")),
        "spec must cite ADR-754 W410 Long MIR-free readiness thin: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonMirFreeReadinessThinInventory")
            && spec.contains("mir_module_long_horizon_mir_free_readiness_thin_inventory"),
        "spec must name public Long-horizon MIR-free readiness thin inventory API"
    );
    assert!(
        spec.contains("long_mir_free_readiness_thin_signed=true")
            || spec.contains("`long_mir_free_readiness_thin_signed=true`"),
        "spec must cite long_mir_free_readiness_thin_signed=true"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    assert!(
        spec.contains("blob_still_embedded=true")
            || spec.contains("`blob_still_embedded=true`")
            || (spec.contains("still embedded") && spec.contains("syx_mir_blob")),
        "spec must cite blob still embedded honesty"
    );
    assert!(
        spec.contains("execution_omit_signed=true")
            || spec.contains("`execution_omit_signed=true`")
            || spec.contains("W409"),
        "spec must index W409 execution-omit prerequisite"
    );
    for line in spec.lines() {
        let claims_mir_free_done = (line.contains("MIR-free") || line.contains("mir_free"))
            && (line.contains("met") || line.contains("enabled=true") || line.contains("已交"))
            && !line.contains("readiness")
            && !line.contains("thin")
            && !line.contains("≠")
            && !line.contains("out of scope")
            && !line.contains("forbidden")
            && !line.contains("Frozen")
            && !line.contains("false")
            && !line.contains("unmet")
            && !line.contains("仍")
            && !line.contains("not this window");
        if claims_mir_free_done && line.contains("MIR-free AOT") {
            panic!("affirmative MIR-free delivery forbidden: {line}");
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
                || line.contains("Not this window")
                || line.contains("仍未")
                || line.contains("ahead")
                || line.contains("unmet");
            assert!(
                denied,
                "affirmative MIR-free / blob delete forbidden: {line}"
            );
        }
    }
}

#[test]
fn long_mir_free_readiness_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W410")
            && (roadmap.contains("MIR-free readiness")
                || roadmap.contains("Long MIR-free")
                || roadmap.contains("ADR-754")
                || roadmap.contains("long-mir-free")),
        "roadmap must cite W410 Long MIR-free readiness thin"
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
        "Wave A exit met claim must remain after W410"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}
