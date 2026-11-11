//! SA-3 module remap closeout contracts.

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
fn sa3_system_networking_surfaces() {
    assert!(repo_path("packages/system/mod.sy").is_file());
    assert!(repo_path("packages/networking/mod.sy").is_file());
    let emb = repo_file("crates/syx-driver/src/embedded_std.rs");
    assert!(emb.contains("\"System\"") && emb.contains("\"Networking\""));
    assert!(!emb.contains("(\"Os\"") && !emb.contains("(\"Net\","));
}

#[test]
fn sa3_module_remap_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/sa_3_module_remap_census.sh")
        .current_dir(&root)
        .output()
        .expect("run sa_3 census");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "sa3 census failed:\n{stdout}\n{stderr}"
    );
    assert!(
        stdout.contains("sa3_module_remap_helper=1") && stdout.contains("OK"),
        "census must report helper:\n{stdout}"
    );
    assert!(
        repo_path("docs/census/sa-3-module-remap.md").exists(),
        "sa-3 census doc must exist"
    );
}
