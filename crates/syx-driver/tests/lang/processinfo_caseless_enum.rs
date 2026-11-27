//! SA-2-03: caseless enum namespace — `enum ProcessInfo { static var processorCount }`.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Result<Vec<String>, String> {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn sa2_caseless_enum_static_var_check_run() {
    let src = r#"
enum ProcessInfo {
    static var processorCount: Int {
        return 8
    }

    static func doubled() -> Int {
        return ProcessInfo.processorCount * 2
    }
}

func main() {
    print(ProcessInfo.processorCount)
    print(ProcessInfo.doubled())
}
"#;
    let printed = run(src).expect("caseless enum ProcessInfo must check/run");
    assert_eq!(printed.len(), 2, "expected two prints, got {printed:?}");
    assert_eq!(printed[0].trim(), "8");
    assert_eq!(printed[1].trim(), "16");
}

#[test]
fn sa2_os_processinfo_surface_present() {
    let path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/os/mod.sy");
    let src = std::fs::read_to_string(&path).expect("read packages/os/mod.sy");
    assert!(
        src.contains("enum ProcessInfo") && src.contains("processorCount"),
        "Os package must expose caseless ProcessInfo.processorCount"
    );
}
