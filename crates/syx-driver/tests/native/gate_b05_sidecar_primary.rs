//! Medium sidecar-primary deepen
//! Execution string-table strip + RT sidecar-primary; B-05 historical [~] at slice (live [x] via); ≠ MIR-free / blob delete.

#[test]
fn b05_sidecar_primary_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/gates/gate-b05-sidecar-primary.md",
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
            "B-05 Medium sidecar-primary requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonSidecarPrimaryInventory")
            && mir.contains("mir_module_medium_horizon_sidecar_primary_inventory")
            && mir.contains("module_with_execution_string_table_stripped")
            && mir.contains("sidecar_primary_signed"),
        "syx-mir must expose Medium sidecar-primary inventory + string-table strip (W408)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("sidecar_string_pool_primary")
            && abi.contains("reset_string_pool_for_sidecar_primary"),
        "syx_rt_load_mir_blob must reset pool and mark sidecar-primary (W408)"
    );
}

#[test]
fn b05_sidecar_primary_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-sidecar-primary.md");
    assert!(
        spec.contains("ADR-752")
            && (spec.contains("sidecar-primary")
                || spec.contains("Sidecar-primary")
                || spec.contains("sidecar_string_pool_primary")),
        "spec must cite ADR-752 W408 B-05 Medium sidecar-primary: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonSidecarPrimaryInventory")
            && (spec.contains("sidecar_primary_signed=true")
                || spec.contains("`sidecar_primary_signed=true`")),
        "spec must name sidecar-primary inventory / sidecar_primary_signed"
    );
    assert!(
        spec.contains("sidecar_string_pool_primary=true")
            || spec.contains("`sidecar_string_pool_primary=true`")
            || spec.contains("partial met"),
        "spec must cite sidecar_string_pool_primary=true / partial met"
    );
    assert!(
        spec.contains("execution_string_table_stripped=true")
            || spec.contains("`execution_string_table_stripped=true`")
            || spec.contains("string-table"),
        "spec must cite execution string-table strip"
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
fn b05_sidecar_primary_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W408")
            && (roadmap.contains("sidecar-primary")
                || roadmap.contains("sidecar_primary")
                || roadmap.contains("ADR-752")
                || roadmap.contains("string-table strip")
                || roadmap.contains("string table strip")),
        "roadmap must cite W408 B-05 Medium sidecar-primary"
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
        "Wave A exit met claim must remain after W408"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}
