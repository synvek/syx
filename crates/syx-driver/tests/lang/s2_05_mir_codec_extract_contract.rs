//! S2-I167 S2-05 syx-mir mir_codec extract contracts.
//! Nails: mir_codec module + root pub use + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-codec-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_codec_extract_baseline() {
    let codec = repo_file("crates/syx-mir/src/mir_codec.rs");
    assert!(
        codec.contains("pub fn encode_module") && codec.contains("pub fn decode_module"),
        "mir_codec must hold encode_module/decode_module"
    );
    assert!(
        codec.contains("encode_module_dual_blob_package")
            && codec.contains("MirModuleDualBlobPackage")
            && codec.contains("MirModuleMetadataSidecarView"),
        "mir_codec must hold dual-blob + sidecar surface"
    );
    assert!(
        codec.contains("encode_module_aot_embed_blob")
            && codec.contains("mir_module_from_sidecar_fn_stubs"),
        "mir_codec must hold aot_embed framing + sidecar stubs"
    );
    assert!(
        !codec.contains("llvm_emit_function_names")
            && !codec.contains("shaken_module_for_aot_blob")
            && !codec.contains("module_with_native_instructions_omitted"),
        "mir_codec must not pull AOT llvm/shaken/native-omit path"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_codec;"),
        "lib.rs must declare mod mir_codec"
    );
    assert!(
        lib.contains("pub use mir_codec::") && lib.contains("encode_module"),
        "lib.rs must pub-use mir_codec encode surface"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("pub fn encode_module(module")),
        "encode_module definition must leave lib.rs"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-codec-extract.md");
    assert!(adr.contains("S2-I167"), "codec ADR must name S2-I167");
    assert!(
        adr.contains("S2-I168") && (adr.contains("shaken") || adr.contains("compact")),
        "ADR must lock next knife = S2-I168 AOT shaken/compact"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
