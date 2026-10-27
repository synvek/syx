//! Unified compile pipeline parity (Phase 63).
// was: unified_pipeline

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

#[test]
fn unified_mir_matches_run_output() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/types/greeter_extension.sy");
    let opts = DriverOptions::default();
    let unit = compile_file_with_options(&path, &opts).unwrap();
    assert!(unit.program.mir_module.is_some());
    let (_, lines) = run_file_printed(&path, &opts).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "Greeter"));
}
