//! Extension must not clobber class/dataclass `_init` (Gate0).
// was: extension_init

use syx_codegen::Instruction;
use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn greeter_extension_init_bytecode() {
    let path = fixture("types/greeter_extension.sy");
    let unit = compile_file_with_options(&path, &DriverOptions::default()).unwrap();
    let init = unit
        .program
        .functions
        .iter()
        .find(|f| f.name == "Greeter_init")
        .expect("Greeter_init");
    assert_eq!(init.param_count, 1, "{:?}", init.instructions);
    assert!(
        init.instructions
            .iter()
            .any(|i| matches!(i, Instruction::LoadLocal(0))),
        "{:?}",
        init.instructions
    );
}

#[test]
fn greeter_extension_run() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("types/greeter_extension.sy");
    let opts = DriverOptions::default();
    let (_, lines) = run_file_printed(&path, &opts).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "Greeter"));
    assert!(lines.iter().any(|l| l.trim() == "Hello, Greeter!"));
}

#[test]
fn pair_dataclass_init_run() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("macros/macro_matrix.sy");
    let opts = DriverOptions::default();
    let (_, lines) = run_file_printed(&path, &opts).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "7"), "got {lines:?}");
}
