//! S2-I170 S2-05 syx-mir mir_module_index extract contracts.
//! Nails: module_index module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-module-index-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_module_index_extract_baseline() {
    let idx = repo_file("crates/syx-mir/src/mir_module_index.rs");
    assert!(
        idx.contains("struct ModuleIndex") && idx.contains("mir_type_expr_name"),
        "mir_module_index must hold ModuleIndex and mir_type_expr_name"
    );
    assert!(
        idx.contains("from_module") && idx.contains("remap_platform_int_extension_call"),
        "mir_module_index must hold from_module + platform int remap helper"
    );
    assert!(
        !idx.contains("struct LowerCtx") && !idx.contains("pub fn lower_module"),
        "mir_module_index must not pull LowerCtx / lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_module_index;"),
        "lib.rs must declare mod mir_module_index"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("struct ModuleIndex")
            || l.starts_with("pub(crate) struct ModuleIndex")),
        "ModuleIndex definition must leave lib.rs"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("struct LowerCtx")
            || l.starts_with("pub(crate) struct LowerCtx")),
        "LowerCtx left lib.rs in I171 (see mir_lower_ctx)"
    );
    assert!(
        lib.contains("mod mir_lower_ctx;"),
        "post-I171: LowerCtx lives behind mir_lower_ctx"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-module-index-extract.md");
    assert!(adr.contains("S2-I170"), "index ADR must name S2-I170");
    assert!(
        adr.contains("S2-I171") && adr.contains("LowerCtx"),
        "ADR must lock next knife = S2-I171 LowerCtx"
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
