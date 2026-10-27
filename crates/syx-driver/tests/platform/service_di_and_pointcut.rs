//! Service DI `within(T) && args(U)` pointcut compose MVP.

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn run_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    let (code, printed) = run_file_printed(&path, &options).expect("run fixture");
    assert_eq!(code, 0, "{printed:?}");
    printed
}

#[test]
fn service_di_and_pointcut_hits_within_and_args() {
    let printed = run_fixture("tests/fixtures/macros/and_pointcut.sy");
    assert!(
        printed.iter().any(|l| l.contains("__before__:handleInt")),
        "before handleInt: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("__around__:handleInt")),
        "around handleInt: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("__after__:handleInt")),
        "after handleInt: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("and-int-ok")),
        "handleInt result: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("__before__:addOne")),
        "before addOne: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "4"),
        "addOne(3) => 4: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("no-advice")),
        "handleStr without advice: {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l.contains("__before__:otherType")),
        "otherType should not match and-compose: {printed:?}"
    );
}

#[test]
fn service_di_and_v16_fingerprint_strings() {
    let service_rs = include_str!("../../../syx-macro/src/service.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(service_rs.contains("macro-service-v16:pointcut-and-compose-mvp"));
    assert!(lib.contains("macro-service-v16:pointcut-and-compose-mvp"));
}
