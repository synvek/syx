//! S2-I141 S2-03 differential floor — Suite-1 VM ⟷ MIR ⟷ oracle.
//! See docs/adr/stage2-s2-03-differential-deepen.md.
//! Wasm/native are honesty OUT (Darwin suite1 wasm fail-closed pre-existing;
//! ≠ flip suite1 skip.wasm; ≠ S2-03 [x]).

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use syx_driver::{compile_and_run_embedded, run_native_mir_file_printed, DriverOptions};

const SUITE1: &[&str] = &[
    "arith_add.sy",
    "branch_if.sy",
    "call_nested.sy",
    "string_len.sy",
    "string_concat.sy",
    "array_index.sy",
    "array_for_in.sy",
    "dict_subscript.sy",
];

#[derive(Debug, Deserialize)]
struct Oracle {
    exit: i32,
    stdout: Vec<String>,
}

fn suite1_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/differential/suite1")
}

fn load_oracle(syx: &Path) -> Oracle {
    let path = syx.with_extension("oracle.json");
    let text =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("oracle {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("bad oracle {}: {e}", path.display()))
}

fn assert_match(backend: &str, name: &str, exit: i32, lines: &[String], oracle: &Oracle) {
    assert_eq!(
        exit, oracle.exit,
        "{backend} {name}: exit mismatch; stdout={lines:?}"
    );
    assert_eq!(
        lines,
        oracle.stdout.as_slice(),
        "{backend} {name}: stdout exact mismatch"
    );
}

#[test]
fn s2_03_suite1_vm_mir_floor() {
    syx_host_plugin::register_echo_plugin_in_process();
    let opts = DriverOptions::default();
    let root = suite1_dir();

    for name in SUITE1 {
        let syx = root.join(name);
        let src =
            fs::read_to_string(&syx).unwrap_or_else(|e| panic!("read {}: {e}", syx.display()));
        let oracle = load_oracle(&syx);

        let vm = compile_and_run_embedded(&src, &opts).unwrap_or_else(|e| panic!("vm {name}: {e}"));
        assert_match("vm", name, vm.exit_code, &vm.printed, &oracle);

        let mir =
            run_native_mir_file_printed(&syx, &opts).unwrap_or_else(|e| panic!("mir {name}: {e}"));
        assert_match("mir", name, mir.0, &mir.1, &oracle);
    }
}
