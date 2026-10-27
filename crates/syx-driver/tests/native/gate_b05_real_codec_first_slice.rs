//! Medium real codec first-slice wire
//! Sidecar encode/decode + inventory; B-05 historical [~] at slice (live [x] via); ≠ dual-blob / cutover / MIR-free.

#[test]
fn b05_real_codec_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b05-real-codec-first-slice.md",
        "docs/gates/gate-b05-production-cutover-deepen.md",
        "docs/gates/gate-b05-production-reopen.md",
        "docs/gates/gate-b05-mir-metadata-blob.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-05 real codec first-slice requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonRealCodecFirstSliceInventory")
            && mir.contains("mir_module_medium_horizon_real_codec_first_slice_inventory")
            && mir.contains("encode_module_metadata_sidecar")
            && mir.contains("decode_module_metadata_sidecar"),
        "syx-mir must expose real codec first-slice wire APIs (W403)"
    );
}

#[test]
fn b05_real_codec_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-real-codec-first-slice.md");
    assert!(
        spec.contains("ADR-747")
            && (spec.contains("real codec")
                || spec.contains("Real codec")
                || spec.contains("first-slice")),
        "spec must cite ADR-747 W403 B-05 real codec first-slice: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonRealCodecFirstSliceInventory")
            && spec.contains("encode_module_metadata_sidecar")
            && spec.contains("decode_module_metadata_sidecar"),
        "spec must name first-slice inventory / sidecar encode-decode APIs"
    );
    assert!(
        spec.contains("real_codec_first_slice_signed=true")
            || spec.contains("`real_codec_first_slice_signed=true`")
            || spec.contains("partial met"),
        "spec must cite first-slice signed / partial met"
    );
    assert!(
        spec.contains("real_codec_enabled=true") || spec.contains("`real_codec_enabled=true`"),
        "spec must cite real_codec_enabled=true"
    );
    assert!(
        spec.contains("dual_blob_reopen_gap_open=false")
            || spec.contains("`dual_blob_reopen_gap_open=false`")
            || spec.contains("b05-dual-blob-production-wire")
            || spec.contains("gate-b05-dual-blob-production-wire")
            || (spec.contains("dual-blob") && spec.contains("partial met")),
        "spec must cite dual-blob production wire closed/partial met (W404)"
    );
    assert!(
        spec.contains("cutover_enabled=false")
            || spec.contains("`cutover_enabled=false`")
            || spec.contains("cutover_enabled=true")
            || spec.contains("`cutover_enabled=true`")
            || (spec.contains("cutover") && spec.contains("unmet"))
            || (spec.contains("cutover") && (spec.contains("partial met") || spec.contains("≠"))),
        "spec must cite Medium cutover honesty (off/unmet at window, or W405 partial/true)"
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
fn b05_real_codec_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W403")
            && (roadmap.contains("real codec")
                || roadmap.contains("first-slice")
                || roadmap.contains("ADR-747")
                || roadmap.contains("real_codec")),
        "roadmap must cite W403 B-05 real codec first-slice"
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
        "Wave A exit met claim must remain after W403"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}
