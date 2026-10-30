//! Medium AOT sidecar embed first-slice
//! Embed package + peel load; B-05 historical [~] at slice (live [x] via); ≠ dual-load runtime / MIR-free / blob delete.

#[test]
fn b05_aot_sidecar_embed_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
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
            "B-05 Medium AOT sidecar embed requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonAotSidecarEmbedInventory")
            && mir.contains("mir_module_medium_horizon_aot_sidecar_embed_inventory")
            && mir.contains("encode_module_aot_embed_blob")
            && mir.contains("aot_sidecar_embed_signed"),
        "syx-mir must expose Medium AOT sidecar embed inventory + encode helper (W406)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        llvm.contains("encode_module_aot_embed_blob"),
        "syx-llvm must embed AOT sidecar package via encode_module_aot_embed_blob"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("is_dual_blob_package_bytes")
            && abi.contains("peel_dual_blob_package_execution"),
        "syx_rt_load_mir_blob must peel SYXD packages (W406)"
    );
}

#[test]
fn b05_aot_sidecar_embed_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-aot-sidecar-embed.md");
    assert!(
        spec.contains("ADR-750")
            && (spec.contains("sidecar embed")
                || spec.contains("AOT sidecar")
                || spec.contains("aot_embeds_sidecar")),
        "spec must cite ADR-750 W406 B-05 Medium AOT sidecar embed: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonAotSidecarEmbedInventory")
            && (spec.contains("aot_sidecar_embed_signed=true")
                || spec.contains("`aot_sidecar_embed_signed=true`")),
        "spec must name sidecar embed inventory / aot_sidecar_embed_signed"
    );
    assert!(
        spec.contains("aot_embeds_sidecar=true")
            || spec.contains("`aot_embeds_sidecar=true`")
            || spec.contains("partial met"),
        "spec must cite aot_embeds_sidecar=true / partial met"
    );
    assert!(
        spec.contains("load_path_is_whole_module=false")
            || spec.contains("`load_path_is_whole_module=false`")
            || spec.contains("load_path_is_whole_module=true")
            || spec.contains("`load_path_is_whole_module=true`")
            || (spec.contains("peel") && spec.contains("execution"))
            || spec.contains("b05-dual-load-runtime"),
        "spec must cite load honesty (whole-module / dual-load / W407 cross)"
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

