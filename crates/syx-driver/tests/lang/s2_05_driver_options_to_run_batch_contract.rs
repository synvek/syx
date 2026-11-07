//! S2-I224–I226 S2-05 syx-driver options→check→run batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-driver-options-to-run-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_options_to_run_batch_baseline() {
    let mods = [
        ("crates/syx-driver/src/driver_options.rs", "DriverOptions"),
        ("crates/syx-driver/src/check_project.rs", "check_project_with_options"),
        ("crates/syx-driver/src/run_exec.rs", "run_project_with_options"),
    ];
    for (path, needle) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(needle)
                || src.contains(&format!("fn {needle}"))
                || src.contains(&format!("pub fn {needle}"))
                || src.contains(&format!("struct {needle}")),
            "{path} must hold {needle}"
        );
    }

    let lib = repo_file("crates/syx-driver/src/lib.rs");
    for mod_name in ["driver_options", "check_project", "run_exec"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub use driver_options::{")
            && lib.contains("pub use check_project::{")
            && lib.contains("pub use run_exec::{"),
        "lib.rs must wire driver_options/check_project/run_exec pub use"
    );
    assert!(
        !lib.contains("\npub struct DriverOptions {")
            && !lib.contains("\npub fn check_project_with_options(")
            && !lib.contains("\npub fn run_project_with_options("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("struct CachedModuleUnit")
            || lib.contains("pub(crate) struct CachedModuleUnit")
            || lib.contains("pub(crate) use compile_cache::{")
            || lib.contains("mod compile_cache;"),
        "CachedModuleUnit must remain wired (lib def or I228+ compile_cache re-export; not folded into options hub)"
    );
    let hub = repo_file("crates/syx-driver/src/driver_options.rs");
    assert!(
        !hub.contains("use crate::compile_")
            && !hub.contains("use crate::native_build")
            && !hub.contains("use crate::cache_fingerprint"),
        "driver_options hub must not reverse-depend on compile/native/fingerprint"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-options-to-run-batch.md");
    assert!(
        adr.contains("S2-I224") && adr.contains("S2-I225") && adr.contains("S2-I226"),
        "batch ADR must name I224–I226"
    );
    assert!(
        adr.contains("S2-I227") && adr.contains("format_lint"),
        "ADR must lock next knife = S2-I227 format_lint"
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
