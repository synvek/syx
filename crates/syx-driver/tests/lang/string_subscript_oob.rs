//! String grapheme subscript out-of-bounds → R003 (Array OOB parity).

use std::fs;
use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_str_oob_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

#[test]
fn string_subscript_oob_read_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    let s = "ab"
    print(s[2])
}
"#,
        &opts(),
    )
    .expect_err("oob read");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of bounds"),
        "message={}",
        err.message
    );
}

#[test]
fn string_subscript_negative_index_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    let s = "ab"
    print(s[-1])
}
"#,
        &opts(),
    )
    .expect_err("negative index");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of bounds"),
        "message={}",
        err.message
    );
}

#[test]
fn string_subscript_in_range_still_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let s = "ab"
    print(s[0])
    print(s[1])
}
"#,
        &opts(),
    )
    .expect("run");
    let joined = result.printed.join("\n");
    assert!(
        joined.contains('a') && joined.contains('b'),
        "printed={:?}",
        result.printed
    );
    assert!(
        !joined.contains("Character("),
        "VM print must not use Debug Character(...): {joined}"
    );
}

#[test]
fn string_subscript_native_print_matches_vm() {
    // Native AOT Print must use value_description (grapheme), not Debug Character("…").
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"
func main() {
    let s = "Hello"
    print(s[2])
}
"#;
    let vm = compile_and_run_embedded(src, &opts()).expect("vm");
    let path = write_src_tmp("print_parity", src);
    let (code, lines) = run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    let vm_line = vm.printed.iter().map(|l| l.trim()).collect::<Vec<_>>().join("\n");
    let native_line = lines.iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");
    assert_eq!(
        native_line, vm_line,
        "native={native_line:?} vm={vm_line:?}"
    );
    assert_eq!(native_line, "l");
    assert!(!native_line.contains("Character("));
}
