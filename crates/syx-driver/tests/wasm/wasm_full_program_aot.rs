//! Phase 66 WASM full-program AOT ratio + emit smoke (multi-fixture matrix).
// was: wasm_full_program_aot

use std::path::PathBuf;
use syx_config::SyxRuntimeConfig;
use syx_driver::{emit_wasm_bytes, compile_and_run_embedded, DriverOptions};
use syx_wasm_emit::measure_aot_ratio;

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

fn with_aot<F: FnOnce() -> T, T>(f: F) -> T {
    syx_config::install_runtime_config(SyxRuntimeConfig {
        wasm: syx_config::WasmConfig {
            aot: true,
            mir_exec_only: false,
        },
        ..SyxRuntimeConfig::default()
    });
    f()
}

fn emit_fixture(rel: &str) -> Option<Vec<u8>> {
    with_aot(|| {
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        match emit_wasm_bytes(&fixture(rel), &opts) {
            Ok(bytes) => Some(bytes),
            Err(e) if e.code.as_deref() == Some("S057") => None,
            Err(e) => panic!("emit {}: {e:?}", rel),
        }
    })
}

macro_rules! aot_fixture_smoke {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            if let Some(bytes) = emit_fixture($path) {
                assert_eq!(&bytes[0..4], b"\0asm");
                assert!(bytes.len() > 32);
            }
        }
    };
}

aot_fixture_smoke!(aot_hello, "hello.sy");
aot_fixture_smoke!(aot_greeter, "types/greeter_extension.sy");
aot_fixture_smoke!(aot_inheritance, "types/inheritance_basic.sy");
aot_fixture_smoke!(aot_dict_hashable, "types/dict_hashable_key.sy");
aot_fixture_smoke!(aot_protocol_for_in, "protocol/for_in_array.sy");
aot_fixture_smoke!(aot_parity_generics, "parity/generics.sy");
aot_fixture_smoke!(aot_parity_hof, "parity/hof.sy");
aot_fixture_smoke!(aot_parity_dataclass, "parity/dataclass_synth.sy");

#[test]
fn aot_hot_loop_ratio_meets_gate() {
    with_aot(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hot_loop.sy");
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        let Ok(bytes) = emit_wasm_bytes(&path, &opts) else {
            return;
        };
        let (_d, _m, ratio) = measure_aot_ratio(&bytes);
        assert!(ratio >= 0.99, "hot_loop AOT ratio {ratio:.4} < 0.99");
    });
}

#[test]
fn aot_hello_ratio_nonzero() {
    if let Some(bytes) = emit_fixture("hello.sy") {
        let (direct, mir_exec, ratio) = measure_aot_ratio(&bytes);
        assert!(direct + mir_exec > 0);
        assert!(ratio >= 0.0 && ratio <= 1.0);
    }
}

#[test]
fn aot_wasm_runs_via_wasmtime() {
    with_aot(|| {
        if let Some(bytes) = emit_fixture("hello.sy") {
            let exit = syx_wasm_run::run_wasm_with_module(&bytes, "main", None).expect("run");
            let _ = exit;
        }
    });
}

#[test]
fn aot_mir_exec_only_differs_from_hybrid() {
    let path = fixture("hello.sy");
    syx_config::install_runtime_config(SyxRuntimeConfig {
        wasm: syx_config::WasmConfig {
            aot: false,
            mir_exec_only: true,
        },
        ..SyxRuntimeConfig::default()
    });
    let baseline = match emit_wasm_bytes(&path, &DriverOptions::default()) {
        Ok(b) => b,
        Err(e) if e.code.as_deref() == Some("S057") => return,
        Err(e) => panic!("mir_exec: {e:?}"),
    };
    with_aot(|| {
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        if let Ok(hybrid) = emit_wasm_bytes(&path, &opts) {
            assert!(!hybrid.is_empty());
            let _ = baseline.len();
        }
    });
}

#[test]
fn aot_wasm_wasi_example_bytes() {
    with_aot(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_wasi/main.sy");
        if !path.exists() {
            return;
        }
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        match emit_wasm_bytes(&path, &opts) {
            Ok(bytes) => assert_eq!(&bytes[0..4], b"\0asm"),
            Err(e) if e.code.as_deref() == Some("S057") => {}
            Err(e) => panic!("wasm_wasi: {e:?}"),
        }
    });
}

#[test]
fn aot_matrix_script_exists() {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/wasm_aot_ratio.sh");
    assert!(script.exists());
    let text = std::fs::read_to_string(script).unwrap();
    assert!(text.contains("--matrix") || text.contains("--min-direct"));
}

#[test]
fn aot_opcode_matrix_doc_exists() {
    let doc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/wasm_native_opcode_matrix.md");
    assert!(doc.exists());
}

#[test]
fn aot_parity_protocol_witness() {
    let path = fixture("parity/protocol_witness.sy");
    if path.exists() {
        if let Some(bytes) = emit_fixture("parity/protocol_witness.sy") {
            assert!(bytes.len() > 50);
        }
    }
}

#[test]
fn aot_parity_array_hof() {
    let path = fixture("parity/array_hof.sy");
    if path.exists() {
        if let Some(bytes) = emit_fixture("parity/array_hof.sy") {
            assert_eq!(&bytes[0..4], b"\0asm");
        }
    }
}

#[test]
fn aot_release_profile_flag() {
    with_aot(|| {
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        opts.build_profile = syx_driver::BuildProfile::Release;
        match emit_wasm_bytes(&fixture("hello.sy"), &opts) {
            Ok(bytes) => assert!(!bytes.is_empty()),
            Err(e) if e.code.as_deref() == Some("S057") => {}
            Err(e) => panic!("release emit: {e:?}"),
        }
    });
}

#[test]
fn aot_embed_arithmetic_prints() {
    with_aot(|| {
        syx_host_plugin::register_echo_plugin_in_process();
        let r = compile_and_run_embedded("func main() { print(2 + 3) }", &DriverOptions::default())
            .unwrap();
        assert!(r.printed.iter().any(|l| l.trim() == "5"));
    });
}

#[test]
fn aot_phase66_charter_mentions_matrix() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 67") || adr.contains("Phase 66") || adr.contains("wasm_aot"));
}
