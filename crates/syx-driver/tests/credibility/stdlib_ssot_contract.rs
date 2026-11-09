//! SA-0-03 stdlib SSOT contracts.
//! Nails: ADR + prelude_core + rename map + census script green.
//! ≠ SA-1 prelude shrink · ≠ SA-3 rename landing.

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
fn sa0_charter_and_ssot_files() {
    assert!(
        repo_path("docs/adr/stdlib-swift-alignment.md").exists(),
        "stdlib-swift-alignment ADR must exist"
    );
    let adr = repo_file("docs/adr/stdlib-swift-alignment.md");
    assert!(adr.contains("SA-0-01"), "ADR must name SA-0-01");
    assert!(
        adr.contains("Foundation")
            && (adr.contains("Never") || adr.contains("never") || adr.contains("禁止")),
        "ADR must forbid Foundation naming"
    );
    assert!(
        adr.contains("do") && adr.contains("catch") && adr.contains("throws -> T"),
        "ADR must lock do/catch + throws -> T"
    );
    assert!(
        repo_path("packages/internal/prelude_core.toml").exists(),
        "prelude_core.toml must exist"
    );
    assert!(
        repo_path("packages/internal/module_rename_map.toml").exists(),
        "module_rename_map.toml must exist"
    );
    let rename = repo_file("packages/internal/module_rename_map.toml");
    assert!(
        rename.contains("Os") && rename.contains("System"),
        "rename map must include Os→System"
    );
    assert!(
        rename.contains("Networking") && rename.contains("Synchronization"),
        "rename map must include Networking / Synchronization"
    );
}

#[test]
fn sa0_manifest_honesty() {
    let manifest: serde_json::Value =
        serde_json::from_str(&repo_file("packages/internal/manifest.json"))
            .expect("manifest.json");
    let prelude = manifest["prelude_modules"]
        .as_array()
        .expect("prelude_modules");
    let importable = manifest["importable_modules"]
        .as_array()
        .expect("importable_modules");
    let retired = manifest["retired_modules"]
        .as_array()
        .expect("retired_modules");
    let prelude: Vec<&str> = prelude.iter().filter_map(|v| v.as_str()).collect();
    let importable: Vec<&str> = importable.iter().filter_map(|v| v.as_str()).collect();
    let retired: Vec<&str> = retired.iter().filter_map(|v| v.as_str()).collect();

    // SA-1: Fmt left prelude — must remain importable.
    assert!(
        !prelude.contains(&"Fmt"),
        "Fmt must not remain in prelude after SA-1"
    );
    assert!(
        importable.contains(&"Fmt"),
        "Fmt must be importable after leaving prelude"
    );
    for name in ["Tracing", "Metrics", "Profiling", "JSON", "Math", "SIMD4"] {
        assert!(
            importable.contains(&name),
            "{name} must be importable_modules"
        );
    }
    assert!(
        !retired.contains(&"Text"),
        "Text must not remain in retired_modules"
    );
    assert!(
        importable.contains(&"Text"),
        "Text must remain importable"
    );
}

#[test]
fn sa0_ssot_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/sa_0_ssot_census.sh")
        .current_dir(&root)
        .output()
        .expect("run sa_0 census");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "census failed:\n{stdout}\n{stderr}"
    );
    assert!(
        stdout.contains("prelude_match=1") && stdout.contains("importable_match=1"),
        "census must report prelude/importable match:\n{stdout}"
    );
    assert!(
        stdout.contains("text_dual_cleared=1") && stdout.contains("sa0_ssot_helper=1"),
        "census must clear Text dual + helper:\n{stdout}"
    );
}
