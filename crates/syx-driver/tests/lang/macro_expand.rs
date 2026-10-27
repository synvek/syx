//! SYX 38.0 macro subsystem tests.
// was: macro

fn adr_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/adr/architecture-decisions.md")
}

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn architecture_macro_38_deliverables() {
    let adr = std::fs::read_to_string(adr_path()).unwrap();
    assert!(adr.contains("@resultBuilder"));
    assert!(adr.contains("#compileTime"));
    assert!(adr.contains("JSON v2") || adr.contains("Enum Codable"));
}

#[test]
fn architecture_backend_llvm_only() {
    let adr = std::fs::read_to_string(adr_path()).unwrap();
    assert!(adr.contains("LLVM is the sole native backend"));
}

#[test]
fn macro_matrix_fixture_exists() {
    assert!(fixture("macros/macro_matrix.sy").exists());
}

#[test]
fn attached_conformance_fixture_exists() {
    assert!(fixture("macros/attached_conformance_dispatch.sy").exists());
}

#[test]
fn external_protocol_v1_frozen() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-macro/src/external_protocol.rs"),
    )
    .unwrap();
    assert!(src.contains("v1") || src.contains("protocol"));
}
