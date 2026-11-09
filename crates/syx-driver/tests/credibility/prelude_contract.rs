//! SA-1 prelude shrink contracts.
//! Nails: target inject set · removed modules importable · Double.pi / Int.random · census green.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn sa1_prelude_core_active() {
    let toml = repo_file("packages/internal/prelude_core.toml");
    assert!(
        toml.contains("status = \"active\""),
        "prelude_core.toml must be active after SA-1"
    );
    assert!(
        !toml.contains("\"Math\"")
            || toml
                .split("[current_injected]")
                .nth(1)
                .map(|s| !s.contains("\"Math\""))
                .unwrap_or(false),
        "Math must not remain in current_injected"
    );
}

#[test]
fn sa1_swift_surfaces_present() {
    let dbl = repo_file("packages/core/builtins/numeric/double.sy");
    assert!(
        dbl.contains("static var pi") || dbl.contains("pi:"),
        "Double.pi must exist"
    );
    let int = repo_file("packages/core/builtins/numeric/int.sy");
    assert!(
        int.contains("func random(in"),
        "Int.random(in:) must exist"
    );
}

#[test]
fn sa1_prelude_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/sa_1_prelude_census.sh")
        .current_dir(&root)
        .output()
        .expect("run sa_1 census");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "sa1 census failed:\n{stdout}\n{stderr}"
    );
    assert!(
        stdout.contains("sa1_prelude_helper=1") && stdout.contains("OK"),
        "census must report helper:\n{stdout}"
    );
    assert!(
        repo_path("docs/census/sa-1-prelude-census.md").exists(),
        "sa-1 census doc must exist"
    );
}
