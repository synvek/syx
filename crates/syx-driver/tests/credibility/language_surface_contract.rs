//! SA-2 language-surface closeout contracts.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn sa2_do_catch_adr_present() {
    let adr = repo_file("docs/adr/sa-2-01-do-catch.md");
    assert!(
        adr.contains("do") && adr.contains("catch"),
        "SA-2-01 ADR must document do/catch"
    );
}

#[test]
fn sa2_public_data_class_rejected() {
    let parser = repo_file("crates/syx-parser/src/lib.rs");
    assert!(
        parser.contains("public data class is removed (SA-2-04)"),
        "parser must reject public data class"
    );
}

#[test]
fn sa2_language_surface_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/sa_2_language_surface_census.sh")
        .current_dir(&root)
        .output()
        .expect("run sa_2 census");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "sa2 census failed:\n{stdout}\n{stderr}"
    );
    assert!(
        stdout.contains("sa2_language_surface_helper=1") && stdout.contains("OK"),
        "census must report helper:\n{stdout}"
    );
    assert!(
        repo_path("docs/census/sa-2-language-surface.md").exists(),
        "sa-2 census doc must exist"
    );
}
