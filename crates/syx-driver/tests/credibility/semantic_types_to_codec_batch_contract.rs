//! S2-I236–I237 S2-05 syx-semantic types→codec batch extract contracts.
//! Nails: 2 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-types-to-codec-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_semantic_types_to_codec_batch_baseline() {
    let types = repo_file("crates/syx-semantic/src/semantic_types.rs");
    assert!(
        types.contains("pub enum TypeId")
            && types.contains("pub struct Symbol")
            && types.contains("pub struct Scope"),
        "semantic_types must hold TypeId/Symbol/Scope"
    );

    let codec = repo_file("crates/syx-semantic/src/semantic_type_codec.rs");
    assert!(
        codec.contains("pub fn type_name_to_id")
            && codec.contains("pub fn type_expr_to_id")
            && codec.contains("pub fn type_id_to_name")
            && codec.contains("pub fn simd_lanes_from_type_name"),
        "semantic_type_codec must hold TypeId codec surface"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod semantic_types;") && lib.contains("mod semantic_type_codec;"),
        "lib.rs must declare semantic_types and semantic_type_codec"
    );
    assert!(
        lib.contains("pub use semantic_types::{")
            && lib.contains("TypeId")
            && lib.contains("pub use semantic_type_codec::{"),
        "lib.rs must pub-use semantic_types and semantic_type_codec"
    );
    assert!(
        !lib.contains("pub enum TypeId") && !lib.contains("\npub fn type_name_to_id("),
        "TypeId / type_name_to_id definitions must leave lib.rs"
    );
    assert!(
        lib.contains("pub use feature_gates::*") || lib.contains("mod feature_gates;"),
        "feature gates must be wired (I238 landed)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-types-to-codec-batch.md");
    assert!(
        adr.contains("S2-I236") && adr.contains("S2-I237"),
        "batch ADR must name I236–I237"
    );
    assert!(
        adr.contains("S2-I238") && adr.contains("feature_gates"),
        "ADR must name S2-I238 semantic feature_gates ladder"
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
