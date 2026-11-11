//! S2-I168 S2-05 syx-mir mir_aot extract contracts.
//! Nails: mir_aot module + root pub use + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-aot-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_aot_extract_baseline() {
    let aot = repo_file("crates/syx-mir/src/mir_aot.rs");
    assert!(
        aot.contains("shaken_module_for_aot_blob")
            && aot.contains("encode_module_compact_for_aot")
            && aot.contains("module_with_native_instructions_omitted"),
        "mir_aot must hold shaken/compact/native-omit surface"
    );
    assert!(
        aot.contains("reachable_fn_names")
            && aot.contains("llvm_emit_function_names")
            && aot.contains("AotBlobSizeReport"),
        "mir_aot must hold reachable/llvm_emit/size report"
    );
    assert!(
        aot.contains("module_with_debug_locals_stripped")
            && aot.contains("aot_codec_fixture_main_with_const_strs"),
        "mir_aot must include strip helpers + codec fixtures (slice B)"
    );
    assert!(
        !aot.contains("pub fn dead_code_elim") && !aot.contains("pub fn optimize("),
        "mir_aot must not pull MIR opt surface"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_aot;"),
        "lib.rs must declare mod mir_aot"
    );
    assert!(
        lib.contains("pub use mir_aot::") && lib.contains("shaken_module_for_aot_blob"),
        "lib.rs must pub-use mir_aot surface"
    );
    assert!(
        !lib.lines()
            .any(|l| l.starts_with("pub fn shaken_module_for_aot_blob")),
        "shaken_module_for_aot_blob definition must leave lib.rs"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("pub fn dead_code_elim")),
        "dead_code_elim left lib.rs at S2-I169 (now mir_opt)"
    );
    assert!(
        repo_file("crates/syx-mir/src/mir_opt.rs").contains("pub fn dead_code_elim"),
        "dead_code_elim must live in mir_opt after I169"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-aot-extract.md");
    assert!(adr.contains("S2-I168"), "aot ADR must name S2-I168");
    assert!(
        adr.contains("S2-I169") && adr.contains("mir_opt"),
        "ADR must lock next knife = S2-I169 mir_opt"
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
