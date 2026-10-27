//! Medium dual-load runtime first-slice
//! Sidecar→RT install + inventory; B-05 historical [~] at slice (live [x] via); ≠ MIR-free / blob delete / execution-omit.

#[test]
fn b05_dual_load_runtime_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/gates/gate-b05-dual-load-runtime.md",
        "docs/gates/gate-b05-aot-sidecar-embed.md",
        "docs/gates/gate-b05-production-cutover.md",
        "docs/gates/gate-b05-dual-blob-production-wire.md",
        "docs/gates/gate-b05-real-codec-first-slice.md",
        "docs/gates/gate-b05-production-cutover-deepen.md",
        "docs/gates/gate-b05-production-reopen.md",
        "docs/gates/gate-b05-mir-metadata-blob.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-05 Medium dual-load runtime requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonDualLoadRuntimeInventory")
            && mir.contains("mir_module_medium_horizon_dual_load_runtime_inventory")
            && mir.contains("peel_dual_blob_package_sidecar")
            && mir.contains("dual_load_runtime_signed"),
        "syx-mir must expose Medium dual-load runtime inventory + peel sidecar (W407)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("install_metadata_sidecar_view")
            && abi.contains("peel_dual_blob_package_sidecar"),
        "syx_rt_load_mir_blob must install sidecar via dual-load runtime (W407)"
    );
}

#[test]
fn b05_dual_load_runtime_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-dual-load-runtime.md");
    assert!(
        spec.contains("ADR-751")
            && (spec.contains("dual-load")
                || spec.contains("Dual-load")
                || spec.contains("dual_load_runtime")),
        "spec must cite ADR-751 W407 B-05 Medium dual-load runtime: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonDualLoadRuntimeInventory")
            && (spec.contains("dual_load_runtime_signed=true")
                || spec.contains("`dual_load_runtime_signed=true`")),
        "spec must name dual-load runtime inventory / dual_load_runtime_signed"
    );
    assert!(
        spec.contains("dual_load_runtime_enabled=true")
            || spec.contains("`dual_load_runtime_enabled=true`")
            || spec.contains("partial met"),
        "spec must cite dual_load_runtime_enabled=true / partial met"
    );
    assert!(
        spec.contains("load_path_is_whole_module=false")
            || spec.contains("`load_path_is_whole_module=false`")
            || (spec.contains("install") && spec.contains("sidecar")),
        "spec must cite load dual-path / load_path_is_whole_module=false"
    );
    for line in spec.lines() {
        if line.contains("MIR-free") || (line.contains("删除") && line.contains("blob")) {
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
                || line.contains("ahead");
            assert!(
                denied,
                "affirmative MIR-free / blob delete forbidden: {line}"
            );
        }
    }
}

#[test]
fn b05_dual_load_runtime_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W407")
            && (roadmap.contains("dual-load")
                || roadmap.contains("dual_load")
                || roadmap.contains("ADR-751")
                || roadmap.contains("sidecar→RT")
                || roadmap.contains("sidecar->RT")),
        "roadmap must cite W407 B-05 Medium dual-load runtime"
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
        "Wave A exit met claim must remain after W407"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}
