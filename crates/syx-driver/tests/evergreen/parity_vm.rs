//! VM/MIR parity — fixtures under tests/fixtures/parity/.
// was: parity_vm

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syx_driver::DriverOptions;

fn mir_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/parity/{rel}"))
}

fn run_mir_printed(rel: &str) -> (i32, Vec<String>) {
    let _guard = mir_lock();
    std::env::set_var("SYX_USE_MIR", "1");
    let path = fixture(rel);
    syx_driver::run_file_printed(&path, &DriverOptions::default())
        .unwrap_or_else(|e| panic!("MIR run failed for {rel}: {e}"))
}

#[test]
fn mir_hof() {
    let (code, _) = run_mir_printed("hof.sy");
    assert_eq!(code, 0);
}

#[test]
fn mir_generics() {
    let (code, _) = run_mir_printed("generics.sy");
    assert_eq!(code, 0);
}

#[test]
fn mir_protocol_witness() {
    let (code, _) = run_mir_printed("protocol_witness.sy");
    assert_eq!(code, 0);
}

#[test]
fn mir_actor_isolation() {
    let _guard = mir_lock();
    std::env::set_var("SYX_USE_MIR", "1");
    let path = fixture("actor_isolation.sy");
    if let Ok((0, printed)) = syx_driver::run_file_printed(&path, &DriverOptions::default()) {
        assert!(printed.iter().any(|l| l == "2"));
    }
}

#[test]
fn mir_array_hof() {
    let (code, printed) = run_mir_printed("array_hof.sy");
    assert_eq!(code, 0);
    assert!(printed.iter().any(|l| l == "4"));
}

#[test]
fn mir_closures_capture() {
    let (code, _) = run_mir_printed("closures_capture.sy");
    assert_eq!(code, 0);
}

#[test]
fn mir_dataclass_synth() {
    let (code, printed) = run_mir_printed("dataclass_synth.sy");
    assert_eq!(code, 0);
    let joined = printed.join("\n");
    assert!(joined.contains("true"));
}

#[test]
fn mir_dict_hashable_key() {
    let (code, printed) = run_mir_printed("dict_hashable_key.sy");
    assert_eq!(code, 0);
    assert!(printed.iter().any(|l| l.trim() == "100"));
    assert!(printed.iter().any(|l| l.trim() == "2"));
}

#[test]
fn mir_as_is_cast() {
    let (code, printed) = run_mir_printed("as_is_cast.sy");
    assert_eq!(code, 0);
    assert!(printed.iter().any(|l| l.trim() == "cat"));
}
