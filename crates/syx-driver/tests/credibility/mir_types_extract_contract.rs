//! S2-I166 S2-05 syx-mir mir_types first-extract contracts.
//! Nails: mir_types module + root pub use + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-types-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_types_extract_baseline() {
    let types = repo_file("crates/syx-mir/src/mir_types.rs");
    assert!(
        types.contains("pub enum MirOp") && types.contains("pub struct MirModule"),
        "mir_types must hold MirOp and MirModule"
    );
    assert!(
        types.contains("pub struct MirInstr")
            && types.contains("pub struct MirBlock")
            && types.contains("pub struct MirFunction"),
        "mir_types must hold MirInstr/MirBlock/MirFunction"
    );
    assert!(
        types.contains("is_suite1_pilot_mir_op")
            && types.contains("mir_module_aot_export_root_refs"),
        "mir_types must hold suite1 + aot export helpers"
    );
    assert!(
        !types.contains("LowerCtx") && !types.contains("ModuleIndex"),
        "mir_types must not pull lowering index / LowerCtx"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_types;"),
        "lib.rs must declare mod mir_types"
    );
    assert!(
        lib.contains("pub use mir_types::") && lib.contains("MirOp") && lib.contains("MirModule"),
        "lib.rs must pub-use mir_types IR surface"
    );
    assert!(
        !lib.contains("pub enum MirOp"),
        "MirOp definition must leave lib.rs"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-types-extract.md");
    assert!(adr.contains("S2-I166"), "extract ADR must name S2-I166");
    assert!(
        adr.contains("S2-I167") && adr.contains("codec"),
        "ADR must lock next knife = S2-I167 codec/blob"
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
